use core::{fmt::Display, str::FromStr};

use embedded_io_async::{Read, Write};
use heapless::String;

use crate::{debug, info};

const MAX_STRING_SIZE: usize = 64;
pub const FIRMWARE_VERSION: &str = "V0.17";

/// A fixed-size string backed by `heapless::String<N>`.
/// This type provides a `FromStr` implementation and a displayable error type.
#[derive(Debug, PartialEq)]
pub struct FixedSizeString(String<MAX_STRING_SIZE>);

#[cfg(feature = "defmt")]
impl defmt::Format for FixedSizeString {
    fn format(&self, fmt: defmt::Formatter) {
        defmt::write!(fmt, "{}", self.as_str());
    }
}

impl FromStr for FixedSizeString {
    type Err = Error;

    fn from_str(s: &str) -> core::result::Result<Self, Self::Err> {
        let mut result: String<MAX_STRING_SIZE> = String::new();
        // `push_str` returns Err if the input does not fit into the fixed-size buffer.
        if result.push_str(s).is_err() {
            return Err(Error::StringTooLongError);
        }
        Ok(FixedSizeString(result))
    }
}

impl FixedSizeString {
    /// Returns the inner string slice.
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

/// Implementing `Deref` allows treating `FixedSizeString` as a `&str`.
impl core::ops::Deref for FixedSizeString {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        self.0.as_str()
    }
}

/// Driver for CTL200 Laser Controller
///
/// See <https://www.koheron.com/support/user-guides/ctl200/>.
pub struct Ctl200<U>
where
    U: Read + Write + 'static,
{
    _uart: U,
}

const CRLF: &[u8] = b"\r\n";
const CRLF_PROMPT: &[u8] = b"\r\n>>";

impl<U> Ctl200<U>
where
    U: Read + Write + 'static,
{
    pub fn new(uart: U) -> Self {
        Ctl200 { _uart: uart }
    }

    /// Returns the firmware version.
    pub async fn version(&mut self) -> Result<FixedSizeString> {
        let resp = self.get::<FixedSizeString>("version").await?;
        debug!("version: {:?}", resp.as_str());
        Ok(resp)
    }

    async fn query(&mut self, tx: &str) -> Result<FixedSizeString> {
        debug!("Sending command: '{}'", tx);
        self._uart.write_all(tx.as_bytes()).await.map_err(|_| {
            debug!("Failed to write command");
            Error::WriteError
        })?;
        self._uart.write_all(CRLF).await.map_err(|_| {
            debug!("Failed to write CRLF");
            Error::WriteError
        })?;
        self._uart.flush().await.map_err(|_| {
            debug!("Failed to flush UART");
            Error::FlushError
        })?;

        debug!("Waiting for echo...");
        let echo = self.wait_for_expected_str(CRLF).await?;
        let response = self.wait_for_expected_str(CRLF_PROMPT).await?;
        debug!("Received echo: '{}' and response: '{}'", echo, response);

        if echo.as_str() != tx {
            info!("Echo mismatch: expected {}, got {}", tx, echo.as_str());
            return Err(Error::EchoMismatch);
        }

        Ok(response)
    }

    async fn wait_for_expected_str(&mut self, expected_str: &[u8]) -> Result<FixedSizeString> {
        let mut buffer = String::new();
        let mut byte = [0u8; 1];

        loop {
            self._uart
                .read(&mut byte)
                .await
                .map_err(|_| Error::ReadError)?;
            buffer
                .push(byte[0] as char)
                .map_err(|_| Error::BufferOverflow)?;

            if buffer.as_bytes().ends_with(expected_str) {
                buffer.truncate(buffer.len() - expected_str.len());
                return Ok(FixedSizeString(buffer));
            }
        }
    }

    async fn get<F>(&mut self, param: &str) -> Result<F>
    where
        F: FromStr,
        F::Err: Display,
    {
        let rx = self.query(param).await?.parse::<F>().map_err(|_| {
            // TODO(xguo): fix this.
            // info!("Failed to parse response: {:?}", Debug2Format(&e));
            Error::InvalidResponse
        })?;
        Ok(rx)
    }

    #[allow(dead_code)]
    async fn set(&mut self, _param: &[u8], _value: Value) -> Result<()> {
        use core::fmt::Write;
        let mut s: String<MAX_STRING_SIZE> = String::new();
        write!(&mut s, "{:?} {:?}", _param, _value).unwrap();
        let _ = self.query(&s).await?;
        Ok(())
    }
}

#[derive(Debug)]
pub enum Value {
    Bool(bool),
    Int(i32),
    Float(f32),
    String(FixedSizeString),
    None,
}

impl FromStr for Value {
    type Err = Error;

    fn from_str(s: &str) -> core::result::Result<Self, Self::Err> {
        if let Ok(b) = s.parse::<bool>() {
            Ok(Value::Bool(b))
        } else if let Ok(i) = s.parse::<i32>() {
            Ok(Value::Int(i))
        } else if let Ok(f) = s.parse::<f32>() {
            Ok(Value::Float(f))
        } else {
            Ok(Value::String(FixedSizeString::from_str(s)?))
        }
    }
}

#[derive(Debug)]
pub enum Error {
    BufferOverflow,
    EchoMismatch,
    FlushError,
    InvalidFirmwareVersion,
    InvalidResponse,
    ReadError,
    StringTooLongError,
    WriteError,
}

#[cfg(feature = "defmt")]
impl defmt::Format for Error {
    fn format(&self, fmt: defmt::Formatter) {
        match self {
            Error::BufferOverflow => defmt::write!(fmt, "Buffer overflow"),
            Error::EchoMismatch => defmt::write!(fmt, "Echo mismatch"),
            Error::FlushError => defmt::write!(fmt, "Flush error"),
            Error::InvalidResponse => defmt::write!(fmt, "Invalid response"),
            Error::ReadError => defmt::write!(fmt, "Read error"),
            Error::WriteError => defmt::write!(fmt, "Write error"),
            Error::StringTooLongError => defmt::write!(fmt, "String too long error"),
            Error::InvalidFirmwareVersion => defmt::write!(fmt, "Invalid firmware version"),
        }
    }
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Error::BufferOverflow => write!(f, "Buffer overflow"),
            Error::EchoMismatch => write!(f, "Echo mismatch"),
            Error::FlushError => write!(f, "Flush error"),
            Error::InvalidResponse => write!(f, "Invalid response"),
            Error::ReadError => write!(f, "Read error"),
            Error::WriteError => write!(f, "Write error"),
            Error::StringTooLongError => write!(f, "String too long error"),
            Error::InvalidFirmwareVersion => write!(f, "Invalid firmware version"),
        }
    }
}

impl core::error::Error for Error {}

pub type Result<T> = core::result::Result<T, Error>;
