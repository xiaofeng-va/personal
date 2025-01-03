use core::fmt;
use serde::{de, ser, Deserialize, Serialize};

#[repr(u16)]
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, thiserror::Error)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Error {
    #[error("Buffer overflow")]
    BufferOverflow = 1,
    #[error("Device error")]
    DeviceError,
    #[error("Echo mismatch")]
    EchoMismatch,
    #[error("Invalid response")]
    InvalidResponse,
    #[error("Write error")]
    WriteError,

    #[error("Format error in write response")]
    FormatErrorInWriteResponse,
    #[error("Format error in write error")]
    FormatErrorInWriteError,

    #[error("Invalid firmware version")]
    InvalidFirmwareVersion = 0x2000,

    #[error("End of file")]
    EndOfFile,
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

    // Errors from Postcard
    #[error("Postcard serialization error")]
    PostcardSerError,
    #[error("Other Postcard error")]
    PostcardOtherError,

    // Placeholder, to be replaced by the actual error
    #[error("Placeholder error")]
    PlaceHolder,

    // TODO(xguo): Move errors below to the correct place
    // Serde errors.
    #[error("Invalid response for deserialize")]
    SerdeBufferFull,
    #[error("Serde UTF-8 error")]
    SerdeUtf8Error,
    #[error("Serde Parse int error")]
    SerdeParseIntError,
    #[error("Serde Parse float error")]
    SerdeParseFloatError,
    #[error("Serde Parse i8 error")]
    SerdeParseI8Error,
    #[error("Serde Invalid boolean")]
    SerdeInvalidBoolean,

    // FromBytes errors.
    #[error("UTF-8 error in from_bytes")]
    FromBytesUTF8Error,
    #[error("Parse int error in from_bytes")]
    FromBytesParseIntError,
    #[error("Parse float error in from_bytes")]
    FromBytesParseFloatError,
    #[error("Invalid boolean in from_bytes")]
    FromBytesInvalidBoolean,

    //
    #[error("UTF-8 error in from_utf8")]
    FromUTF8Error,

    // Errors in debug!()
    // TODO(xguo): Add #[cfg] for defmt errors or wrap them.
    #[error("UTF-8 error in debug!()")]
    DefmtUTF8Error,

    // CTL200 errors
    #[error("Write error in CTL200 execution")]
    Ctl200WriteError,
    #[error("Flush error in CTL200 execution")]
    Ctl200FlushError,
    #[error("Read error in CTL200 execution")]
    Ctl200ReadError,

    // Generic Uart errors
    #[error("Uart Read error")]
    UartReadError,
    #[error("Uart Write error in try_once operation")]
    UartWriteErrorInTryOnce,
    #[error("Uart Flush error")]
    UartFlushError,
    #[error("UART request timeout")]
    UartRequestTimeout,
    #[error("Uart Write error in write_line operation")]
    UartWriteErrorInWriteLine,
}

impl From<postcard::Error> for Error {
    fn from(e: postcard::Error) -> Self {
        match e {
            postcard::Error::SerializeBufferFull => Error::SerdeBufferFull,
            _ => Error::PostcardOtherError,
        }
    }
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
