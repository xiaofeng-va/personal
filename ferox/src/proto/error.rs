use core::fmt;

use serde::{ser, Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Error {
    BufferOverflow = 1,
    DeviceError,
    EchoMismatch,
    FlushError,
    InvalidResponse,
    ReadError,
    WriteError,

    BytesToUTF8Error = 0x1000,
    InvalidBoolean,
    ParseIntError,
    ParseFloatError,

    // Used by application
    InvalidFirmwareVersion = 0x2000,
}

#[cfg(not(feature = "full-display"))]
impl core::fmt::Display for Error {
    fn fmt(&self, _f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        // do nothing if the feature is not enabled
        Ok(())
    }
}

#[cfg(feature = "full-display")]
impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Error::BufferOverflow => write!(f, "Buffer overflow"),
            Error::EchoMismatch => write!(f, "Echo mismatch"),
            Error::FlushError => write!(f, "Flush error"),
            Error::InvalidFirmwareVersion => write!(f, "Invalid firmware version"),
            Error::InvalidResponse => write!(f, "Invalid response"),
            Error::ReadError => write!(f, "Read error"),
            Error::WriteError => write!(f, "Write error"),
            Error::DeviceError => write!(f, "Device error"),
            Error::InvalidBoolean => write!(f, "Invalid boolean"),
            Error::BytesToUTF8Error => write!(f, "Bytes to UTF-8 error"),
            Error::ParseIntError => write!(f, "Parse int error"),
            Error::ParseFloatError => write!(f, "Parse float error"),
        }
    }
}

impl core::error::Error for Error {}

impl ser::Error for crate::proto::error::Error {
    fn custom<T>(_msg: T) -> Self
    where
        T: fmt::Display,
    {
        // TODO(xguo): Define the error code for custom error.
        // crate::proto::error::Error::new(msg.to_string())
        todo!()
    }
}
