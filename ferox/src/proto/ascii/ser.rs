use defmt_or_log::info;
use heapless::String;
use serde::{ser::{SerializeMap, SerializeSeq, SerializeStruct, SerializeStructVariant, SerializeTuple, SerializeTupleStruct, SerializeTupleVariant}, Serialize, Serializer};

use crate::{common::MAX_STRING_SIZE, proto::error::Error as FeroxError};

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
    _buffer: &'a mut AsciiSerializer,
}

impl<'a> SerializeSeq for ObjSerializer<'a> {
    type Ok = ();
    type Error = FeroxError;

    fn serialize_element<T>(&mut self, _value: &T) -> Result<(), Self::Error>
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

    fn serialize_element<T>(&mut self, _value: &T) -> Result<(), Self::Error>
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
    
    fn serialize_field<T>(&mut self, _value: &T) -> Result<(), Self::Error>
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

    fn serialize_field<T>(&mut self, _value: &T) -> Result<(), Self::Error>
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

    fn serialize_key<T>(&mut self, _key: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize {
        todo!()
    }

    fn serialize_value<T>(&mut self, _value: &T) -> Result<(), Self::Error>
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

    fn serialize_field<T>(&mut self, _key: &'static str, _value: &T) -> Result<(), Self::Error>
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

    fn serialize_field<T>(&mut self, _key: &'static str, _value: &T) -> Result<(), Self::Error>
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
        self.buffer.push_str(value).map_err(|_| FeroxError::BufferOverflow)
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
