
pub mod deser;
pub mod ser;

use core::str;
use deser::AsciiDeserializer;
use heapless::String;
use ser::AsciiSerializer;
use serde::{Deserialize, Serialize};
use crate::proto::error::Error as FeroxError;

const MAX_STRING_SIZE: usize = 128;
pub fn to_string<T>(value: &T) -> Result<String<MAX_STRING_SIZE>, FeroxError>
where
    T: Serialize,
{
    let mut serializer = AsciiSerializer::new();
    value.serialize(&mut serializer)?;
    Ok(serializer.into_inner())
}

pub fn from_str<'a, T>(input: &'a str) -> Result<T, serde::de::value::Error>
where
    T: Deserialize<'a>,
{
    let deserializer = AsciiDeserializer::new(input);
    T::deserialize(deserializer)
}

#[cfg(test)]
mod tests {
    use defmt_or_log::info;

    use crate::proto::ferox::{AnotherEnum, FeroxRequest};
    use super::*;

    #[test]
    fn test_serialize_version() {
        env_logger::builder().is_test(true).try_init().unwrap();
        info!("test_serialize_version: INFO");
        let request = FeroxRequest::Version;
        let serialized = to_string(&request).unwrap();
        assert_eq!(serialized, "ver");
    }

    #[test]
    fn test_another_serialize_version() {
        env_logger::builder().is_test(true).try_init().unwrap();
        info!("test_another_serialize_version: INFO");
        let request = FeroxRequest::Another(AnotherEnum::Version);
        let serialized = to_string(&request).unwrap();
        assert_eq!(serialized, "ver another version");
    }

    #[test]
    fn test_deserialize_version() {
        let input = "Version";
        let deserialized: FeroxRequest = from_str(input).unwrap();
        assert_eq!(deserialized, FeroxRequest::Version);
    }

    #[test]
    fn test_serialize_forward() {
        let request = FeroxRequest::SmcForward { data: b"example" };
        let serialized = to_string(&request).unwrap();
        assert_eq!(serialized, "SmcForward example");
    }

    #[test]
    fn test_deserialize_forward() {
        let input = "SmcForward example";
        let deserialized: FeroxRequest = from_str(input).unwrap();
        assert_eq!(
            deserialized,
            FeroxRequest::SmcForward { data: b"example" }
        );
    }
}
