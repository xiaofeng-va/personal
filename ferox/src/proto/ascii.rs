use core::str;
use defmt_or_log::{error, info};
use heapless::String;
use serde::{de::{self, Deserialize, Deserializer, Visitor}, ser::{self, SerializeMap, SerializeSeq, SerializeStruct, SerializeStructVariant, SerializeTuple, SerializeTupleStruct, SerializeTupleVariant}, Serialize, Serializer};
use crate::proto::{error::Error as FeroxError, ferox::FeroxRequest};

const MAX_STRING_SIZE: usize = 128;

pub struct AsciiSerializer {
    buffer: String<MAX_STRING_SIZE>,
}

impl AsciiSerializer {
    pub fn new() -> Self {
        Self {
            buffer: String::new(),
        }
    }

    pub fn into_inner(self) -> String<MAX_STRING_SIZE> {
        self.buffer
    }
}

pub struct ObjSerializer<'a> {
    buffer: &'a mut AsciiSerializer,
}

impl<'a> SerializeSeq for ObjSerializer<'a> {
    type Ok = ();
    type Error = FeroxError;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize {
        todo!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }
}

impl<'a> SerializeTuple for ObjSerializer<'a> {
    type Ok = ();
    type Error = FeroxError;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize {
        todo!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }
}

impl<'a> SerializeTupleStruct for ObjSerializer<'a> {
    type Ok = ();
    type Error = FeroxError;
    
    fn serialize_field<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize {
        todo!()
    }
    
    fn end(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }
}

impl<'a> SerializeTupleVariant for ObjSerializer<'a> {
    type Ok = ();
    type Error = FeroxError;

    fn serialize_field<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize {
        todo!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }
}

impl<'a> SerializeMap for ObjSerializer<'a> {
    type Ok = ();
    type Error = FeroxError;

    fn serialize_key<T>(&mut self, key: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize {
        todo!()
    }

    fn serialize_value<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize {
        todo!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }
}

impl<'a> SerializeStruct for ObjSerializer<'a> {
    type Ok = ();
    type Error = FeroxError;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize {
        todo!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }
}

impl<'a> SerializeStructVariant for ObjSerializer<'a> {
    type Ok = ();
    type Error = FeroxError;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize {
        todo!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }
}

// TODO(xguo): Remove the allow below.
#[allow(unused_variables)]
impl<'a> Serializer for &'a mut AsciiSerializer {
    type Ok = ();
    type Error = FeroxError;

    type SerializeSeq = ObjSerializer<'a>;
    type SerializeTuple = ObjSerializer<'a>;
    type SerializeTupleStruct = ObjSerializer<'a>;
    type SerializeTupleVariant = ObjSerializer<'a>;
    type SerializeMap = ObjSerializer<'a>;
    type SerializeStruct = ObjSerializer<'a>;
    type SerializeStructVariant = ObjSerializer<'a>;

    fn serialize_str(self, value: &str) -> Result<Self::Ok, Self::Error> {
        info!("Serializing string: {}", value);
        self.buffer.push_str(value).map_err(|_| ser::Error::custom("Buffer overflow"))
    }

    fn serialize_newtype_struct<T>(self, _: &'static str, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        info!("Serializing newtype struct");
        value.serialize(self)
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        variant_index: u32,
        _variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        info!("Serializing unit variant: name = {}, variant_index = {}, variant = {}", _name, variant_index, _variant);

        self.serialize_str(&_variant)
    }
    
    fn serialize_newtype_variant<T>(
        self,
        _: &'static str,
        _: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        info!("Serializing newtype variant: {}", variant);
        self.serialize_str(variant)?;
        self.serialize_str(" ")?;
        value.serialize(self)
    }

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        todo!()
    }
    
    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        todo!()
    }
    
    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        todo!()
    }
    
    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        todo!()
    }
    
    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        todo!()
    }
    
    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        todo!()
    }
    
    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        todo!()
    }
    
    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        todo!()
    }
    
    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        todo!()
    }
    
    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        todo!()
    }
    
    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        todo!()
    }
    
    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        todo!()
    }
    
    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        todo!()
    }
    
    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }
    
    fn serialize_some<T>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize {
        todo!()
    }
    
    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }
    
    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok, Self::Error> {
        todo!()
    }
    
    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        todo!()
    }
    
    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        todo!()
    }
    
    fn serialize_tuple_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        todo!()
    }
    
    fn serialize_tuple_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        todo!()
    }
    
    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        todo!()
    }
    
    fn serialize_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        todo!()
    }
    
    fn serialize_struct_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        info!("Serializing struct variant: name = {}, variant_index = {}, variant = {}, len = {}", name, variant_index, variant, len);
        // TODO(xguo): Implement this.
        Err(FeroxError::PlaceHolder)
    }
    
    fn collect_str<T>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + core::fmt::Display {
        todo!()
    }
}

pub struct AsciiDeserializer<'a> {
    input: &'a str,
}

impl<'a> AsciiDeserializer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self { input }
    }
}

impl<'de> Deserializer<'de> for AsciiDeserializer<'de> {
    type Error = de::value::Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_str(self.input)
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_any(visitor)
    }

    fn deserialize_enum<V>(
        self,
        _: &'static str,
        _: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_str(self.input)
    }

    fn deserialize_i128<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let _ = visitor;
        Err(de::Error::custom("i128 is not supported"))
    }
    
    fn deserialize_u128<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let _ = visitor;
        Err(de::Error::custom("u128 is not supported"))
    }
    
    fn is_human_readable(&self) -> bool {
        true
    }
    
    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de> {
        todo!()
    }
    
    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de> {
        todo!()
    }
    
    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de> {
        todo!()
    }
    
    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de> {
        todo!()
    }
    
    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de> {
        todo!()
    }
    
    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de> {
        todo!()
    }
    
    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de> {
        todo!()
    }
    
    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de> {
        todo!()
    }
    
    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de> {
        todo!()
    }
    
    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de> {
        todo!()
    }
    
    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de> {
        todo!()
    }
    
    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de> {
        todo!()
    }
    
    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de> {
        todo!()
    }
    
    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de> {
        todo!()
    }
    
    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de> {
        todo!()
    }
    
    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de> {
        todo!()
    }
    
    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de> {
        todo!()
    }
    
    fn deserialize_unit_struct<V>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de> {
        todo!()
    }
    
    fn deserialize_newtype_struct<V>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de> {
        todo!()
    }
    
    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de> {
        todo!()
    }
    
    fn deserialize_tuple<V>(self, len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de> {
        todo!()
    }
    
    fn deserialize_tuple_struct<V>(
        self,
        name: &'static str,
        len: usize,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de> {
        todo!()
    }
    
    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de> {
        todo!()
    }
    
    fn deserialize_struct<V>(
        self,
        name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de> {
        todo!()
    }
    
    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de> {
        todo!()
    }
    
    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de> {
        todo!()
    }
    
    // Implement other methods as needed (e.g., deserialize_struct)
}

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

    // #[test]
    // fn test_another_serialize_version() {
    //     env_logger::builder().is_test(true).try_init().unwrap();
    //     info!("test_another_serialize_version: INFO");
    //     let request = FeroxRequest::Another(AnotherEnum::Version);
    //     let serialized = to_string(&request).unwrap();
    //     assert_eq!(serialized, "another ver");
    // }

    #[test]
    fn test_deserialize_version() {
        let input = "Version";
        let deserialized: FeroxRequest = from_str(input).unwrap();
        assert_eq!(deserialized, FeroxRequest::Version);
    }

    #[test]
    fn test_serialize_forward() {
        env_logger::builder().is_test(true).try_init().unwrap();
        info!("test_serialize_forward: INFO");
        let request = FeroxRequest::SmcForward { data: b"bia?" };
        let serialized = to_string(&request).unwrap();
        assert_eq!(serialized, "smc bia?");
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
