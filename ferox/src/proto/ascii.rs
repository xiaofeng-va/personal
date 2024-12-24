use deser::AsciiDeserializer;
use heapless::Vec;
use ser::AsciiSerializer;
use serde::{Deserialize, Serialize};

use crate::{common::MAX_STRING_SIZE, proto::error::Error as FeroxError};

pub mod deser;
pub mod ser;
pub mod vec;

pub fn to_string<T>(value: &T) -> Result<Vec<u8, MAX_STRING_SIZE>, FeroxError>
where
    T: Serialize,
{
    let mut serializer = AsciiSerializer::new(vec::FeroxVec::<MAX_STRING_SIZE>::new());
    value.serialize(&mut serializer)?;
    let t = serializer
        .finalize()
        .release()
        .map_err(|_| FeroxError::BufferOverflow)?;
    Ok(t)
}

pub fn from_str<'a, T>(input: &'a str) -> Result<T, serde::de::value::Error>
where
    T: Deserialize<'a>,
{
    let deserializer = AsciiDeserializer::new(input);
    T::deserialize(deserializer)
}
