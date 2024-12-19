use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
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

    InvalidCommand,
    PostcardDeserializeError,
    PostcardSerializeError,
    UnexpectedFeroxRequest,

    // Used by application
    InvalidFirmwareVersion = 0x2000,
}

#[cfg(not(feature = "full-display"))]
impl core::fmt::Display for Error {
    fn fmt(&self, _f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(_f, "{:?}", self)
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
            Error::InvalidCommand => write!(f, "Invalid command"),
            Error::PostcardDeserializeError => write!(f, "Postcard deserialize error"),
            Error::UnexpectedFeroxRequest => write!(f, "Unexpected Ferox request"),
            Error::PostcardSerializeError => write!(f, "Postcard serialize error"),
        }
    }
}

impl core::error::Error for Error {}

pub type Result<T> = core::result::Result<T, Error>;
