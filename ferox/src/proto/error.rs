use core::fmt;
use serde::{de, ser, Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize, thiserror::Error)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Error {
    #[error("Buffer overflow")]
    BufferOverflow = 1,
    #[error("Device error")]
    DeviceError,
    #[error("Echo mismatch")]
    EchoMismatch,
    #[error("Flush error")]
    FlushError,
    #[error("Invalid response")]
    InvalidResponse,
    #[error("Read error")]
    ReadError,
    #[error("Write error")]
    WriteError,

    #[error("Write error in try_once operation")]
    WriteErrorInTryOnce = 0x11,
    #[error("Write error in write_line operation")]
    WriteErrorInWriteLine,
    #[error("Write error in CTL200 query")]
    WriteErrorInCtl200Query,
    #[error("Format error in write response")]
    FormatErrorInWriteResponse,
    #[error("Format error in write error")]
    FormatErrorInWriteError,

    #[error("Bytes to UTF-8 error")]
    BytesToUTF8Error = 0x1000,
    #[error("Invalid boolean")]
    InvalidBoolean,
    #[error("Parse int error")]
    ParseIntError,
    #[error("Parse float error")]
    ParseFloatError,

    #[error("Invalid firmware version")]
    InvalidFirmwareVersion = 0x2000,

    #[error("End of file")]
    EndOfFile,
    #[error("UTF-8 error")]
    Utf8Error,
    #[error("Parse i8 error")]
    ParseI8Error,
    #[error("Unexpected token")]
    UnexpectedToken,

    #[error("Invalid request")]
    InvalidRequest,
    #[error("Invalid request for deserialize")]
    InvalidRequestForDeserialize,
    #[error("Invalid request for serialize")]
    InvalidRequestForSerialize,
    #[error("Not supported in serializing")]
    NotSupportedInSerializing,
    #[error("CTL200 request serialization error")]
    Ctl200RequestSerializeError,
    #[error("SMC request serialization error")]
    SmcRequestSerializeError,

    #[error("UART request timeout")]
    UartRequestTimeout,

    #[error("Placeholder error")]
    PlaceHolder = 0xFFFF,
}

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
