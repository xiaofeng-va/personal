use core::fmt;

use serde::{de, ser, Deserialize, Serialize};

#[repr(u16)]
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, thiserror::Error)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Error {
    // Ser/De errors inherited from other crates.
    #[error("Invalid response for deserialize")]
    SerdeBufferFull = 0x0100,
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

    // Deserialization errors.
    #[error("Deserialize Unexpected token")]
    DeserializeUnexpectedToken = 0x0110,
    #[error("Deserialize End of file")]
    DeserializeEndOfFile,
    #[error("Deserialize Invalid request")]
    DeserializeInvalidRequest,

    // Serialization errors.
    #[error("Serialize Not supported")]
    SerializeNotSupported = 0x0120,
    #[error("Invalid request for serialize")]
    SerializeInvalidRequest,

    // FromBytes errors.
    #[error("UTF-8 error in from_bytes")]
    FromBytesUtf8Error = 0x0200,
    #[error("Parse int error in from_bytes")]
    FromBytesParseIntError,
    #[error("Parse float error in from_bytes")]
    FromBytesParseFloatError,
    #[error("Invalid boolean in from_bytes")]
    FromBytesInvalidBoolean,

    // CTL200 errors
    #[error("Write error in CTL200 execution")]
    Ctl200WriteError = 0x0300,
    #[error("Flush error in CTL200 execution")]
    Ctl200FlushError,
    #[error("Read error in CTL200 execution")]
    Ctl200ReadError,
    #[error("User data error in CTL200 execution")]
    Ctl200UserDataError,
    #[error("Invalid response in CTL200 execution")]
    Ctl200InvalidResponse,
    #[error("Response buffer overflow in CTL200 execution")]
    Ctl200ResponseBufferOverflow,
    #[error("Echo mismatch in CTL200 execution")]
    Ctl200EchoMismatch,
    #[error("From UTF-8 error in CTL200 execution")]
    Ctl200FromUtf8Error,

    // Generic Uart errors
    #[error("Uart Read error")]
    UartReadError = 0x0400,
    #[error("Uart Write error in try_once operation")]
    UartWriteErrorInTryOnce,
    #[error("Uart Flush error")]
    UartFlushError,
    #[error("UART request timeout")]
    UartRequestTimeout,
    #[error("Uart Write error in write_line operation")]
    UartWriteErrorInWriteLine,
    #[error("Uart Read buffer overflow")]
    UartReadBufferOverflow,

    #[error("Ferox server Format error in write response")]
    FeroxServerFormatErrorInWriteResponse = 0x1000,

    // Other errors
    #[error("Other errors in postcard")]
    PostcardOtherError = 0xF000,
    // Errors in debug!()
    #[error("UTF-8 error in defmt")]
    Utf8ErrorInDefmt,
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
        Error::SerializeInvalidRequest
    }
}

impl de::Error for Error {
    fn custom<T>(_msg: T) -> Self
    where
        T: fmt::Display,
    {
        Error::DeserializeInvalidRequest
    }
}
