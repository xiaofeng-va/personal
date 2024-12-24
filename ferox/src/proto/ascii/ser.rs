use defmt_or_log::info;
use postcard::ser_flavors::Flavor;
use serde::{
    ser::{
        SerializeMap, SerializeSeq, SerializeStruct, SerializeStructVariant, SerializeTuple,
        SerializeTupleStruct, SerializeTupleVariant,
    },
    Serialize, Serializer,
};

use crate::{proto::error::Error as FeroxError};

pub struct AsciiSerializer<F: Flavor> {
    buffer: F,
}

impl<F: Flavor> AsciiSerializer<F> {
    pub fn new(buffer: F) -> Self {
        Self {
            buffer,
        }
    }

    fn try_extend(&mut self, data: &[u8]) -> Result<(), FeroxError> {
        self.buffer.try_extend(data).map_err(|_| FeroxError::BufferOverflow)
    }

    pub fn finalize(self) -> F {
        self.buffer
    }
}

impl<F: Flavor> SerializeSeq for &mut AsciiSerializer<F> {
    type Ok = ();
    type Error = FeroxError;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        todo!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }
}

impl<F: Flavor> SerializeTuple for &mut AsciiSerializer<F> {
    type Ok = ();
    type Error = FeroxError;

    fn serialize_element<T>(&mut self, _value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        todo!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }
}

impl<F: Flavor> SerializeTupleStruct for &mut AsciiSerializer<F> {
    type Ok = ();
    type Error = FeroxError;

    fn serialize_field<T>(&mut self, _value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        todo!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }
}
impl<F: Flavor> SerializeTupleVariant for &mut AsciiSerializer<F> {
    type Ok = ();
    type Error = FeroxError;

    fn serialize_field<T>(&mut self, _value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        todo!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }
}

impl<F: Flavor> SerializeMap for &mut AsciiSerializer<F> {
    type Ok = ();
    type Error = FeroxError;

    fn serialize_key<T>(&mut self, _key: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        todo!()
    }

    fn serialize_value<T>(&mut self, _value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        todo!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }
}

impl<F: Flavor> SerializeStruct for &mut AsciiSerializer<F> {
    type Ok = ();
    type Error = FeroxError;

    fn serialize_field<T>(&mut self, _key: &'static str, _value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        todo!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }
}

impl<F: Flavor> SerializeStructVariant for &mut AsciiSerializer<F> {
    type Ok = ();
    type Error = FeroxError;

    fn serialize_field<T>(&mut self, _key: &'static str, _value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        todo!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }
}

// TODO(xguo): Remove the allow below.
#[allow(unused_variables)]
impl<F: Flavor> Serializer for &mut AsciiSerializer<F> {
    type Ok = ();
    type Error = FeroxError;

    type SerializeSeq = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;
    type SerializeMap = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Self;

    fn serialize_str(self, value: &str) -> Result<Self::Ok, Self::Error> {
        info!("Serializing string: {}", value);
        self.try_extend(value.as_bytes())
    }

    fn serialize_newtype_struct<T>(
        self,
        _: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
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
        info!(
            "Serializing unit variant: name = {}, variant_index = {}, variant = {}",
            _name, variant_index, _variant
        );

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
        T: ?Sized + Serialize,
    {
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
        info!(
            "Serializing struct variant: name = {}, variant_index = {}, variant = {}, len = {}",
            name, variant_index, variant, len
        );
        // TODO(xguo): Implement this.
        Err(FeroxError::PlaceHolder)
    }

    fn collect_str<T>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + core::fmt::Display,
    {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use env_logger;
    use heapless::String;
    use serde::{Deserialize, Serialize};

    use super::*;
    use crate::proto::ascii::to_string;

    #[derive(Serialize, Deserialize, Debug)]
    enum TestReq<'a> {
        #[serde(rename = "ver")]
        Version,

        #[serde(rename = "varint")]
        VarInt(Option<i32>),

        #[serde(rename = "varfloat")]
        VarFloat(Option<f32>),

        #[serde(rename = "varbool")]
        VarBool(Option<bool>),

        #[serde(rename = "varbytes")]
        VarBytes(Option<&'a [u8]>),

        #[serde(rename = "smc")]
        SmcForward { request: &'a [u8] },

        #[serde(rename = "recur")]
        Recursive(RecursiveEnum),
    }

    #[derive(Serialize, Deserialize, Debug)]
    enum RecursiveEnum {
        #[serde(rename = "ver")]
        Version,
    }

    #[test]
    fn test_serialize_version() {
        env_logger::builder().is_test(true).try_init().unwrap();
        info!("test_serialize_failure: INFO");

        let s = String::<1>::new();

        assert_eq!(to_string(&TestReq::Version).unwrap(), "ver");
    }
    #[test]
    fn test_serialize_varint_some() {
        env_logger::builder().is_test(true).try_init().unwrap();
        info!("test_serialize_varint_some: INFO");

        assert_eq!(to_string(&TestReq::VarInt(Some(42))).unwrap(), "varint 42");
    }

    #[test]
    fn test_serialize_varint_none() {
        env_logger::builder().is_test(true).try_init().unwrap();
        info!("test_serialize_varint_none: INFO");

        assert_eq!(to_string(&TestReq::VarInt(None)).unwrap(), "varint?");
    }

    #[test]
    fn test_serialize_varfloat_some() {
        env_logger::builder().is_test(true).try_init().unwrap();
        info!("test_serialize_varfloat_some: INFO");

        assert_eq!(
            to_string(&TestReq::VarFloat(Some(3.14))).unwrap(),
            "varfloat 3.14"
        );
    }

    #[test]
    fn test_serialize_varfloat_none() {
        env_logger::builder().is_test(true).try_init().unwrap();
        info!("test_serialize_varfloat_none: INFO");

        assert_eq!(to_string(&TestReq::VarFloat(None)).unwrap(), "varfloat?");
    }

    #[test]
    fn test_serialize_varbool_some() {
        env_logger::builder().is_test(true).try_init().unwrap();
        info!("test_serialize_varbool_some: INFO");

        assert_eq!(
            to_string(&TestReq::VarBool(Some(true))).unwrap(),
            "varbool true"
        );
    }

    #[test]
    fn test_serialize_varbool_none() {
        env_logger::builder().is_test(true).try_init().unwrap();
        info!("test_serialize_varbool_none: INFO");

        assert_eq!(to_string(&TestReq::VarBool(None)).unwrap(), "varbool?");
    }

    #[test]
    fn test_serialize_varbytes_some() {
        env_logger::builder().is_test(true).try_init().unwrap();
        info!("test_serialize_varbytes_some: INFO");

        assert_eq!(
            to_string(&TestReq::VarBytes(Some(b"hello"))).unwrap(),
            "varbytes hello"
        );
    }

    #[test]
    fn test_serialize_varbytes_none() {
        env_logger::builder().is_test(true).try_init().unwrap();
        info!("test_serialize_varbytes_none: INFO");

        assert_eq!(to_string(&TestReq::VarBytes(None)).unwrap(), "varbytes?");
    }

    #[test]
    fn test_serialize_smcforward() {
        env_logger::builder().is_test(true).try_init().unwrap();
        info!("test_serialize_smcforward: INFO");

        assert_eq!(
            to_string(&TestReq::SmcForward { request: b"bia?" }).unwrap(),
            "smc bia?"
        );
    }

    #[test]
    fn test_serialize_recursive() {
        env_logger::builder().is_test(true).try_init().unwrap();
        info!("test_serialize_recursive: INFO");

        assert_eq!(
            to_string(&TestReq::Recursive(RecursiveEnum::Version)).unwrap(),
            "recur ver"
        );
    }
}
