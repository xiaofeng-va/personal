use core::fmt;

use serde::{de, ser, Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Error {
    // TODO(xguo): Reorganize errors.
    BufferOverflow = 1,
    DeviceError,
    EchoMismatch,
    FlushError,
    InvalidResponse,
    ReadError,
    WriteError,

    WriteErrorInTryOnce = 0x11,
    WriteErrorInWriteLine,
    WriteErrorInCtl200Query,
    FormatErrorInWriteResponse,
    FormatErrorInWriteError,

    BytesToUTF8Error = 0x1000,
    InvalidBoolean,
    ParseIntError,
    ParseFloatError,

    // Used by application
    InvalidFirmwareVersion = 0x2000,

    // (De)Serialization errors
    EndOfFile,
    Utf8Error,
    ParseI8Error,
    UnexpectedToken,

    // Ferox Request related
    InvalidRequest,
    InvalidRequestForDeserialize,
    InvalidRequestForSerialize,
    NotSupportedInSerializing,
    Ctl200RequestSerializeError,
    SmcRequestSerializeError,

    UartRequestTimeout,

    // There should be no errors after PlaceHolder.
    PlaceHolder = 0xFFFF,
}

#[cfg(not(feature = "full-display"))]
impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let error_number = *self as u16;
        write!(f, "0x{:04X}", error_number)
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
            Error::EndOfFile => write!(f, "End of file"),
            Error::Utf8Error => write!(f, "UTF-8 error"),
            Error::ParseI8Error => write!(f, "Parse i8 error"),
            Error::UnexpectedToken => write!(f, "Unexpected token"),
            Error::InvalidRequest => write!(f, "Invalid request"),
            Error::PlaceHolder => write!(f, "Placeholder error"),
            Error::InvalidRequestForDeserialize => write!(f, "Invalid request for deserialize"),
            Error::UartRequestTimeout => write!(f, "UART request timeout"),
            Error::InvalidRequestForSerialize => write!(f, "Invalid request for serialize"),
            Error::NotSupportedInSerializing => write!(f, "Not supported in serializing"),
            Error::WriteErrorInTryOnce => write!(f, "Write error in try_once operation"),
            Error::WriteErrorInWriteLine => write!(f, "Write error in write_line operation"),
            Error::WriteErrorInCtl200Query => write!(f, "Write error in CTL200 query"),
            Error::FormatErrorInWriteResponse => write!(f, "Format error in write response"),
            Error::FormatErrorInWriteError => write!(f, "Format error in write error"),
            Error::Ctl200RequestSerializeError => write!(f, "CTL200 request serialization error"),
            Error::SmcRequestSerializeError => write!(f, "SMC request serialization error"),
        }
    }
}

impl core::error::Error for Error {}

impl ser::Error for Error {
    fn custom<T>(_msg: T) -> Self
    where
        T: fmt::Display,
    {
        Error::InvalidRequestForSerialize
    }
}

impl de::Error for Error {
    fn custom<T>(_msg: T) -> Self
    where
        T: fmt::Display,
    {
        Error::InvalidRequestForDeserialize
    }
}
