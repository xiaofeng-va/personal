use heapless::{Vec};
use serde::{Deserialize, Serialize};

use crate::{MAX_STRING_SIZE, proto::error::Error as FeroxError};
use crate::proto::Result as Result;

pub mod deser;
pub mod ser;
pub mod vec;

pub fn to_bytes<T>(value: &T) -> Result<Vec<u8, MAX_STRING_SIZE>>
where
    T: Serialize,
{
    let mut serializer = ser::AsciiSerializer::new(vec::FeroxVec::<MAX_STRING_SIZE>::new());
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
