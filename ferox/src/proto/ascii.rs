use heapless::String;
use serde::{Deserialize, Serialize};

use crate::{common::MAX_STRING_SIZE, proto::error::Error as FeroxError};
use crate::proto::Result as Result;

pub mod deser;
pub mod ser;
pub mod str;
pub mod vec;

pub fn to_string<T>(value: &T) -> Result<String<MAX_STRING_SIZE>>
where
    T: Serialize,
{
    let mut serializer = ser::AsciiSerializer::new(str::FeroxString::<MAX_STRING_SIZE>::new());
    value.serialize(&mut serializer)?;
    let t = serializer
        .finalize()
        .release()
        .map_err(|_| FeroxError::BufferOverflow)?;
    Ok(t)
}

pub fn from_bytes<'de, T>(bytes: &'de [u8]) -> Result<T>
where
    T: Deserialize<'de>,
{
    let mut de = deser::AsciiDeserializer::new(bytes);
    let t = T::deserialize(&mut de)?;
    Ok(t)
}
