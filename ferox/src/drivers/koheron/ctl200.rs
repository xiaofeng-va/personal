use core::str::FromStr;

use embedded_io_async::{Read, Write};
use heapless::String;

use crate::{debug, info};

const MAX_STRING_SIZE: usize = 64;

// Trait definition with lifetime parameter
trait FromBytes<'a> {
    fn from_bytes(bytes: &'a [u8]) -> Result<Self>
    where
        Self: Sized;
}

// Implementation for &[u8]
impl<'a> FromBytes<'a> for &'a [u8] {
    fn from_bytes(bytes: &'a [u8]) -> Result<Self> {
        Ok(bytes)
    }
}

// Implementation for bool
impl<'a> FromBytes<'a> for bool {
    fn from_bytes(bytes: &'a [u8]) -> Result<Self> {
        match bytes {
            b"0" => Ok(false),
            b"1" => Ok(true),
            _ => Err(Error::InvalidResponse),
        }
    }
}

// Implementation for i32
impl<'a> FromBytes<'a> for i32 {
    fn from_bytes(bytes: &'a [u8]) -> Result<Self> {
        core::str::from_utf8(bytes)
            .map_err(|_| Error::InvalidResponse)?
            .parse()
            .map_err(|_| Error::InvalidResponse)
    }
}

// Implementation for f32
impl<'a> FromBytes<'a> for f32 {
    fn from_bytes(bytes: &'a [u8]) -> Result<Self> {
        core::str::from_utf8(bytes)
            .map_err(|_| Error::InvalidResponse)?
            .parse()
            .map_err(|_| Error::InvalidResponse)
    }
}

/// Driver for CTL200 Laser Controller
///
/// See <https://www.koheron.com/support/user-guides/ctl200/>.
pub struct Ctl200<'a, U>
where
    U: Read + Write + 'static,
{
    _uart: U,
    _buf: [u8; MAX_STRING_SIZE],
    _buf_pos: usize,
    _echo_buf: [u8; MAX_STRING_SIZE],
    _echo_len: usize,
    _value: Value<'a>,
}

const CRLF: &[u8] = b"\r\n";
const CRLF_PROMPT: &[u8] = b"\r\n>>";

impl<'a, U> Ctl200<'a, U>
where
    U: Read + Write + 'static,
{
    pub fn new(uart: U) -> Self {
        Ctl200 {
            _uart: uart,
            _buf: [0; MAX_STRING_SIZE],
            _value: Value::None,
            _buf_pos: 0,
            _echo_buf: [0; MAX_STRING_SIZE],
            _echo_len: 0,
        }
    }

    /// Returns the firmware version.
    pub async fn version(&mut self) -> Result<&[u8]> {
        debug!("Ctl200::version() 0");
        let resp: &[u8] = self.get::<&[u8]>("version").await?;
        let t = core::str::from_utf8(resp).map_err(|_| Error::InvalidResponse)?;
        debug!("version: {:?}", t);
        Ok(resp)
    }

    async fn query(&mut self, request: &str) -> Result<&'_ [u8]> {
        debug!("Sending command: '{}'", request);
        self._uart
            .write_all(request.as_bytes())
            .await
            .map_err(|_| {
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

        debug!("Waiting for response...");
        let full_response = self.read_until(CRLF_PROMPT).await?;
        debug!("Got response: {:?}", full_response);

        let mut echo_end = None;
        let mut response_start = None;
        let len = full_response.len();

        for i in 0..len - 2 {
            if full_response[i] == b'\r' && full_response[i + 1] == b'\n' {
                if echo_end.is_none() {
                    echo_end = Some(i);
                    response_start = Some(i + 2);
                } else {
                    return Err(Error::InvalidResponse);
                }
            }
        }

        let (echo, response) = match (echo_end, response_start) {
            (Some(e_end), Some(r_start)) => {
                let echo = &full_response[..e_end];
                let response = &full_response[r_start..];
                (echo, response)
            }
            _ => return Err(Error::InvalidResponse),
        };
        debug!("Got echo: {}, response: {}", echo, response);

        if echo != request.as_bytes() {
            info!(
                "Echo mismatch: expected {}, got {}",
                request,
                core::str::from_utf8(echo).unwrap_or("<invalid>")
            );
            return Err(Error::EchoMismatch);
        }

        Ok(response)
    }

    async fn read_until(&mut self, expected_str: &[u8]) -> Result<&'_ [u8]> {
        let mut byte = [0u8; 1];

        while self._buf_pos + expected_str.len() <= MAX_STRING_SIZE {
            // Read one byte
            self._uart
                .read(&mut byte)
                .await
                .map_err(|_| Error::ReadError)?;

            // Add to buffer
            self._buf[self._buf_pos] = byte[0];
            self._buf_pos += 1;

            // Check if we have enough bytes to match expected_str
            if self._buf_pos >= expected_str.len() {
                // Check if buffer ends with expected_str
                let start_idx = self._buf_pos - expected_str.len();
                if self._buf[start_idx..self._buf_pos] == expected_str[..] {
                    // Found match, prepare result
                    let result = &self._buf[..start_idx];

                    // Reset buffer position to allow future reads
                    self._buf_pos = 0;

                    return Ok(result);
                }
            }
        }

        // Buffer overflow
        Err(Error::BufferOverflow)
    }

    async fn get<'b, T>(&'b mut self, param: &str) -> Result<T>
    where
        T: FromBytes<'b>,
    {
        let response = self.query(param).await?;
        T::from_bytes(response)
    }

    #[allow(dead_code)]
    async fn set(&mut self, param: &[u8], value: &str) -> Result<()> {
        use core::fmt::Write;
        let mut s: String<MAX_STRING_SIZE> = String::new();
        write!(&mut s, "{:?} {:?}", param, value).unwrap();
        let _ = self.query(&s).await?;
        Ok(())
    }
}

pub enum ParseError {
    ParseError,
}

pub struct NumericBool(pub bool);

impl FromStr for NumericBool {
    type Err = ParseError;

    fn from_str(s: &str) -> core::result::Result<Self, Self::Err> {
        match s {
            "0" => Ok(NumericBool(false)),
            "1" => Ok(NumericBool(true)),
            _ => Err(ParseError::ParseError),
        }
    }
}

#[derive(Debug)]
pub enum Value<'a> {
    Bool(bool),
    Int(i32),
    Float(f32),
    String(&'a [u8]),
    None,
}

#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Error {
    BufferOverflow = 1,
    EchoMismatch,
    FlushError,
    InvalidFirmwareVersion,
    InvalidResponse,
    ReadError,
    StringTooLongError,
    WriteError,
}

#[cfg(feature = "full-error-messages")]
impl core::fmt::Display for Error {
    fn fmt(&self, _f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        // do nothing if the feature is not enabled
        Ok(())
    }
}

#[cfg(not(feature = "full-error-messages"))]
impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Error::BufferOverflow => write!(f, "Buffer overflow"),
            Error::EchoMismatch => write!(f, "Echo mismatch"),
            Error::FlushError => write!(f, "Flush error"),
            Error::InvalidFirmwareVersion => write!(f, "Invalid firmware version"),
            Error::InvalidResponse => write!(f, "Invalid response"),
            Error::ReadError => write!(f, "Read error"),
            Error::StringTooLongError => write!(f, "String too long error"),
            Error::WriteError => write!(f, "Write error"),
        }
    }
}

impl core::error::Error for Error {}

pub type Result<T> = core::result::Result<T, Error>;
