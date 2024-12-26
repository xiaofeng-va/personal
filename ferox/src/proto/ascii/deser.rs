use defmt_or_log::{debug, Display2Format};
use serde::de::{
    DeserializeSeed, Deserializer, EnumAccess, IntoDeserializer, VariantAccess, Visitor,
};

use crate::proto::{error::Error as FeroxError, Result};

pub struct AsciiDeserializer<'de> {
    input: &'de [u8],
    index: usize,
}

impl<'de> AsciiDeserializer<'de> {
    pub fn new(input: &'de [u8]) -> Self {
        AsciiDeserializer { input, index: 0 }
    }

    fn skip_whitespace(&mut self) {
        while self.index < self.input.len() && is_whitespace(self.input[self.index]) {
            self.index += 1;
        }
    }

    fn take_remaining(&mut self) -> Option<&'de [u8]> {
        self.skip_whitespace();
        if self.index >= self.input.len() {
            return None;
        }
        let remaining = &self.input[self.index..];
        self.index = self.input.len();
        Some(remaining)
    }

    fn next_token(&mut self) -> Option<&'de [u8]> {
        self.skip_whitespace();
        if self.index >= self.input.len() {
            return None;
        }

        let start = self.index;

        while self.index < self.input.len() {
            let current = self.input[self.index];

            if is_whitespace(current) || current == b'?' {
                break;
            }

            self.index += 1;
        }

        if self.index < self.input.len() && self.input[self.index] == b'?' {
            if start == self.index {
                self.index += 1;
                return Some(&self.input[start..self.index]);
            } else {
                return Some(&self.input[start..self.index]);
            }
        }

        Some(&self.input[start..self.index])
    }

    fn peek_token(&mut self) -> Option<&'de [u8]> {
        let saved = self.index;
        let t = self.next_token();
        self.index = saved;
        t
    }
}

fn is_whitespace(b: u8) -> bool {
    matches!(b, b' ' | b'\t' | b'\r' | b'\n')
}

impl<'de, 'a> Deserializer<'de> for &'a mut AsciiDeserializer<'de> {
    type Error = FeroxError;

    fn deserialize_any<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        Err(FeroxError::UnexpectedToken)
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let token = self.next_token().ok_or(FeroxError::EndOfFile)?;
        match token {
            b"1" => visitor.visit_bool(true),
            b"0" => visitor.visit_bool(false),
            _ => Err(FeroxError::InvalidBoolean),
        }
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        Err(FeroxError::UnexpectedToken)
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        Err(FeroxError::UnexpectedToken)
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let token = self.next_token().ok_or(FeroxError::EndOfFile)?;
        let s = core::str::from_utf8(token).map_err(|_| FeroxError::Utf8Error)?;
        let parsed = s.parse::<i32>().map_err(|_| FeroxError::ParseIntError)?;
        visitor.visit_i32(parsed)
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        Err(FeroxError::UnexpectedToken)
    }

    fn deserialize_u8<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        Err(FeroxError::UnexpectedToken)
    }

    fn deserialize_u16<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        Err(FeroxError::UnexpectedToken)
    }

    fn deserialize_u32<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        Err(FeroxError::UnexpectedToken)
    }

    fn deserialize_u64<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        Err(FeroxError::UnexpectedToken)
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let token = self.next_token().ok_or(FeroxError::EndOfFile)?;
        let s = core::str::from_utf8(token).map_err(|_| FeroxError::Utf8Error)?;
        let parsed = s.parse::<f32>().map_err(|_| FeroxError::ParseFloatError)?;
        visitor.visit_f32(parsed)
    }

    fn deserialize_f64<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        Err(FeroxError::UnexpectedToken)
    }

    fn deserialize_char<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        Err(FeroxError::UnexpectedToken)
    }

    fn deserialize_str<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        Err(FeroxError::UnexpectedToken)
    }

    fn deserialize_string<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        Err(FeroxError::UnexpectedToken)
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_borrowed_bytes(self.take_remaining().ok_or(FeroxError::EndOfFile)?)
    }

    fn deserialize_byte_buf<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        Err(FeroxError::UnexpectedToken)
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        if let Some(token) = self.peek_token() {
            if token == b"?" {
                self.next_token(); // consume it
                return visitor.visit_none();
            }
        }
        visitor.visit_some(&mut *self)
    }

    fn deserialize_unit<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        Err(FeroxError::UnexpectedToken)
    }

    fn deserialize_unit_struct<V>(self, _name: &'static str, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        Err(FeroxError::UnexpectedToken)
    }

    fn deserialize_newtype_struct<V>(self, _name: &'static str, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        Err(FeroxError::UnexpectedToken)
    }

    fn deserialize_seq<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        Err(FeroxError::UnexpectedToken)
    }

    fn deserialize_tuple<V>(self, _len: usize, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        Err(FeroxError::UnexpectedToken)
    }

    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        _len: usize,
        _visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        Err(FeroxError::UnexpectedToken)
    }

    fn deserialize_map<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        Err(FeroxError::UnexpectedToken)
    }

    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        _visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        Err(FeroxError::UnexpectedToken)
    }

    fn deserialize_identifier<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        Err(FeroxError::UnexpectedToken)
    }

    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let variant_name = self.next_token().ok_or(FeroxError::EndOfFile)?;
        debug!(
            "Deserializing enum variant: {:?}",
            core::str::from_utf8(variant_name).unwrap_or("<invalid utf8>")
        );
        visitor.visit_enum(EnumRef {
            de: self,
            variant_name,
        })
    }

    fn deserialize_ignored_any<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        Err(FeroxError::UnexpectedToken)
    }
}


struct EnumRef<'a, 'de: 'a> {
    de: &'a mut AsciiDeserializer<'de>,
    variant_name: &'de [u8],
}

impl<'de, 'a> EnumAccess<'de> for EnumRef<'a, 'de> {
    type Error = FeroxError;
    type Variant = VariantRef<'a, 'de>;

    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant)>
    where
        V: DeserializeSeed<'de>,
    {
        let s = core::str::from_utf8(self.variant_name).map_err(|_| FeroxError::Utf8Error)?;
        debug!("Deserializing variant name: {:?}", s);
        let v = seed.deserialize(s.into_deserializer())?;
        // 检查是否还有更多的输入
        let has_value = self.de.peek_token().is_some();
        Ok((v, VariantRef { 
            de: self.de,
            has_value 
        }))
    }
}

struct VariantRef<'a, 'de: 'a> {
    de: &'a mut AsciiDeserializer<'de>,
    has_value: bool,
}

impl<'de, 'a> VariantAccess<'de> for VariantRef<'a, 'de> {
    type Error = FeroxError;

    fn unit_variant(self) -> Result<()> {
        if self.has_value {
            return Err(FeroxError::UnexpectedToken);
        }
        Ok(())
    }

    fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value>
    where
        T: DeserializeSeed<'de>,
    {
        if !self.has_value {
            return Err(FeroxError::UnexpectedToken);
        }
        seed.deserialize(&mut *self.de)
    }

    fn tuple_variant<V>(self, _len: usize, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        Err(FeroxError::UnexpectedToken)
    }

    fn struct_variant<V>(self, _fields: &'static [&'static str], _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        Err(FeroxError::UnexpectedToken)
    }
}

#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};

    use super::*;
    use crate::{proto::ascii::from_bytes, testing::helpers::init_logger};
    use crate::proto::error::Error;

    #[derive(Serialize, Deserialize, Debug, PartialEq)]
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
    fn test_deserialize_unknown_command() {
        init_logger();
        assert_eq!(Error::InvalidRequestForDeserialize, from_bytes::<TestReq>(b"abc").unwrap_err());
    }

    #[test]
    fn test_deserialize_varbytes2() {
        init_logger();
        let deserialized: TestReq = from_bytes(b"varbytes2").unwrap();
        assert_eq!(deserialized, TestReq::VarBytes2);
    }

    #[test]
    fn test_deserialize_varint_some() {
        init_logger();
        let deserialized: TestReq = from_bytes(b"varint 42").unwrap();
        assert_eq!(deserialized, TestReq::VarInt(Some(42_i32)));
    }

    #[test]
    fn test_deserialize_varint_none() {
        init_logger();
        let deserialized: TestReq = from_bytes(b"varint?").unwrap();
        assert_eq!(deserialized, TestReq::VarInt(None));
    }

    #[test]
    fn test_deserialize_varfloat_some() {
        init_logger();
        let deserialized: TestReq = from_bytes(b"varfloat 3.14").unwrap();
        assert_eq!(deserialized, TestReq::VarFloat(Some(3.14_f32)));
    }

    #[test]
    fn test_deserialize_varfloat_none() {
        init_logger();
        let deserialized: TestReq = from_bytes(b"varfloat?").unwrap();
        assert_eq!(deserialized, TestReq::VarFloat(None));
    }

    #[test]
    fn test_deserialize_varbool_some() {
        init_logger();
        let deserialized_true: TestReq = from_bytes(b"varbool 1").unwrap();
        assert_eq!(deserialized_true, TestReq::VarBool(Some(true)));

        let deserialized_false: TestReq = from_bytes(b"varbool 0").unwrap();
        assert_eq!(deserialized_false, TestReq::VarBool(Some(false)));
    }

    #[test]
    fn test_deserialize_varbool_none() {
        init_logger();
        let deserialized: TestReq = from_bytes(b"varbool?").unwrap();
        assert_eq!(deserialized, TestReq::VarBool(None));
    }

    #[test]
    fn test_deserialize_varbytes_some() {
        init_logger();
        let deserialized: TestReq = from_bytes(b"varbytes hello").unwrap();
        assert_eq!(deserialized, TestReq::VarBytes(Some(b"hello")));
    }

    #[test]
    fn test_deserialize_varbytes_none() {
        init_logger();
        let deserialized: TestReq = from_bytes(b"varbytes?").unwrap();
        assert_eq!(deserialized, TestReq::VarBytes(None));
    }

    #[test]
    fn test_next_token() {
        let input = b"varbool? varint 42 varfloat?";
        let mut deserializer = AsciiDeserializer::new(input);

        assert_eq!(deserializer.next_token(), Some(&b"varbool"[..])); // First token
        assert_eq!(deserializer.next_token(), Some(&b"?"[..])); // Special character `?`
        assert_eq!(deserializer.next_token(), Some(&b"varint"[..])); // Next token
        assert_eq!(deserializer.next_token(), Some(&b"42"[..])); // Next token
        assert_eq!(deserializer.next_token(), Some(&b"varfloat"[..])); // Next token
        assert_eq!(deserializer.next_token(), Some(&b"?"[..])); // Special character `?`
        assert_eq!(deserializer.next_token(), None); // End of input
    }
}
