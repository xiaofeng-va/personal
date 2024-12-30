use core::fmt::Write;

use defmt_or_log::info;
use postcard::ser_flavors::Flavor;
use serde::{
    ser::{
        SerializeMap, SerializeSeq, SerializeStruct, SerializeStructVariant, SerializeTuple,
        SerializeTupleStruct, SerializeTupleVariant,
    },
    Serialize, Serializer,
};

// use postcard::{Error as PostcardError, Result as PostCardResult};
use crate::proto::error::Error as FeroxError;

pub struct AsciiSerializer<F: Flavor> {
    buffer: F,
}

impl<F: Flavor> AsciiSerializer<F> {
    pub fn new(buffer: F) -> Self {
        Self { buffer }
    }

    fn try_extend(&mut self, data: &[u8]) -> Result<(), FeroxError> {
        self.buffer
            .try_extend(data)
            .map_err(|_| FeroxError::BufferOverflow)
    }

    fn try_push(&mut self, data: u8) -> Result<(), FeroxError> {
        self.buffer
            .try_push(data)
            .map_err(|_| FeroxError::BufferOverflow)
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
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl<F: Flavor> SerializeTuple for &mut AsciiSerializer<F> {
    type Ok = ();
    type Error = FeroxError;

    fn serialize_element<T>(&mut self, _value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        Err(Self::Error::NotSupportedInSerializing)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Err(Self::Error::NotSupportedInSerializing)
    }
}

impl<F: Flavor> SerializeTupleStruct for &mut AsciiSerializer<F> {
    type Ok = ();
    type Error = FeroxError;

    fn serialize_field<T>(&mut self, _value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        Err(Self::Error::NotSupportedInSerializing)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Err(Self::Error::NotSupportedInSerializing)
    }
}
impl<F: Flavor> SerializeTupleVariant for &mut AsciiSerializer<F> {
    type Ok = ();
    type Error = FeroxError;

    fn serialize_field<T>(&mut self, _value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        Err(Self::Error::NotSupportedInSerializing)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Err(Self::Error::NotSupportedInSerializing)
    }
}

impl<F: Flavor> SerializeMap for &mut AsciiSerializer<F> {
    type Ok = ();
    type Error = FeroxError;

    fn serialize_key<T>(&mut self, _key: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        Err(Self::Error::NotSupportedInSerializing)
    }

    fn serialize_value<T>(&mut self, _value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        Err(Self::Error::NotSupportedInSerializing)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Err(Self::Error::NotSupportedInSerializing)
    }
}

impl<F: Flavor> SerializeStruct for &mut AsciiSerializer<F> {
    type Ok = ();
    type Error = FeroxError;

    fn serialize_field<T>(&mut self, _key: &'static str, _value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        Err(Self::Error::NotSupportedInSerializing)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Err(Self::Error::NotSupportedInSerializing)
    }
}

impl<F: Flavor> SerializeStructVariant for &mut AsciiSerializer<F> {
    type Ok = ();
    type Error = FeroxError;

    fn serialize_field<T>(&mut self, _key: &'static str, _value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        Err(Self::Error::NotSupportedInSerializing)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Err(Self::Error::NotSupportedInSerializing)
    }
}

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
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        info!(
            "Serializing unit variant: name = {}, variant_index = {}, variant = {}",
            _name, variant_index, variant
        );

        self.serialize_str(variant)
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
        value.serialize(self)
    }

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        if v {
            self.serialize_char('1')
        } else {
            self.serialize_char('0')
        }
    }

    fn serialize_i8(self, _v: i8) -> Result<Self::Ok, Self::Error> {
        Err(Self::Error::NotSupportedInSerializing)
    }

    fn serialize_i16(self, _v: i16) -> Result<Self::Ok, Self::Error> {
        Err(Self::Error::NotSupportedInSerializing)
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        let mut s = heapless::String::<10>::new();
        write!(s, "{}", v).map_err(|_| FeroxError::BufferOverflow)?;
        self.try_extend(s.as_bytes())
    }

    fn serialize_i64(self, _v: i64) -> Result<Self::Ok, Self::Error> {
        Err(Self::Error::NotSupportedInSerializing)
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        self.serialize_char(v as char)
    }

    fn serialize_u16(self, _v: u16) -> Result<Self::Ok, Self::Error> {
        Err(Self::Error::NotSupportedInSerializing)
    }

    fn serialize_u32(self, _v: u32) -> Result<Self::Ok, Self::Error> {
        Err(Self::Error::NotSupportedInSerializing)
    }

    fn serialize_u64(self, _v: u64) -> Result<Self::Ok, Self::Error> {
        Err(Self::Error::NotSupportedInSerializing)
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        let mut s = heapless::String::<10>::new();
        write!(s, "{}", v).map_err(|_| FeroxError::BufferOverflow)?;
        self.try_extend(s.as_bytes())
    }

    fn serialize_f64(self, _v: f64) -> Result<Self::Ok, Self::Error> {
        Err(Self::Error::NotSupportedInSerializing)
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        self.try_push(v as u8)
    }

    fn serialize_bytes(self, _v: &[u8]) -> Result<Self::Ok, Self::Error> {
        Err(Self::Error::NotSupportedInSerializing)
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        self.serialize_char('?')
    }

    fn serialize_some<T>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        self.serialize_char(' ')?;
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Err(Self::Error::NotSupportedInSerializing)
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        Err(Self::Error::NotSupportedInSerializing)
    }

    // do nothing
    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Ok(self)
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Err(Self::Error::NotSupportedInSerializing)
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Err(Self::Error::NotSupportedInSerializing)
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Err(Self::Error::NotSupportedInSerializing)
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Err(Self::Error::NotSupportedInSerializing)
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        Err(Self::Error::NotSupportedInSerializing)
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

    fn collect_str<T>(self, _value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + core::fmt::Display,
    {
        Err(Self::Error::NotSupportedInSerializing)
    }
}

#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};

    use crate::{proto::ascii::to_bytes, testing::helpers::init_logger};

    #[derive(Serialize, Deserialize, Debug)]
    enum TestReq<'a> {
        #[serde(rename = "varint")]
        VarInt(Option<i32>),

        #[serde(rename = "varfloat")]
        VarFloat(Option<f32>),

        #[serde(rename = "varbool")]
        VarBool(Option<bool>),

        #[serde(rename = "varbytes")]
        VarBytes(Option<&'a [u8]>),

        #[serde(rename = "varbytes2")]
        VarBytes2,
    }

    #[test]
    fn test_serialize_varint_some() {
        init_logger();
        assert_eq!(to_bytes(&TestReq::VarInt(Some(42))).unwrap(), b"varint 42");
    }

    #[test]
    fn test_serialize_varbytes2() {
        init_logger();
        assert_eq!(to_bytes(&TestReq::VarBytes2).unwrap(), b"varbytes2");
    }

    #[test]
    fn test_serialize_varint_none() {
        init_logger();
        assert_eq!(to_bytes(&TestReq::VarInt(None)).unwrap(), b"varint?");
    }

    #[test]
    fn test_serialize_varfloat_some() {
        init_logger();
        assert_eq!(
            to_bytes(&TestReq::VarFloat(Some(3.14f32))).unwrap(),
            b"varfloat 3.14"
        );
    }

    #[test]
    fn test_serialize_varfloat_none() {
        init_logger();
        assert_eq!(to_bytes(&TestReq::VarFloat(None)).unwrap(), b"varfloat?");
    }

    #[test]
    fn test_serialize_varbool_some() {
        init_logger();
        assert_eq!(
            to_bytes(&TestReq::VarBool(Some(true))).unwrap(),
            b"varbool 1"
        );
        assert_eq!(
            to_bytes(&TestReq::VarBool(Some(false))).unwrap(),
            b"varbool 0"
        );
    }

    #[test]
    fn test_serialize_varbool_none() {
        init_logger();
        assert_eq!(to_bytes(&TestReq::VarBool(None)).unwrap(), b"varbool?");
    }

    #[test]
    fn test_serialize_varbytes_some() {
        init_logger();
        assert_eq!(
            to_bytes(&TestReq::VarBytes(Some(b"hello"))).unwrap(),
            b"varbytes hello"
        );
    }

    #[test]
    fn test_serialize_varbytes_none() {
        init_logger();
        assert_eq!(to_bytes(&TestReq::VarBytes(None)).unwrap(), b"varbytes?");
    }
}
