use core::fmt;

use serde::de::{
    self, Deserialize, DeserializeSeed, Deserializer, EnumAccess, IntoDeserializer, Unexpected,
    VariantAccess, Visitor,
};

////////////////////////////////////////////////////////////////////////////////
// Custom error type (no_std friendly)
////////////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
pub struct AsciiError {
    msg: &'static str,
}

impl AsciiError {
    pub fn new(msg: &'static str) -> Self {
        AsciiError { msg }
    }
}

impl fmt::Display for AsciiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "AsciiError: {}", self.msg)
    }
}

// In no_std environment, we cannot implement std::error::Error; only satisfy serde::de::Error.
impl de::Error for AsciiError {
    fn custom<T: fmt::Display>(_: T) -> Self {
        // In no_std mode, we can't easily convert any fmt::Display into String.
        // We choose to write a fixed message or do more complicated handling.
        AsciiError::new("custom error")
    }
}

////////////////////////////////////////////////////////////////////////////////
// Deserialization entry point
////////////////////////////////////////////////////////////////////////////////

pub fn from_bytes<'de, T>(bytes: &'de [u8]) -> Result<T, AsciiError>
where
    T: Deserialize<'de>,
{
    let mut de = AsciiDeserializer::new(bytes);
    let t = T::deserialize(&mut de)?;
    Ok(t)
}

////////////////////////////////////////////////////////////////////////////////
// Custom Deserializer
////////////////////////////////////////////////////////////////////////////////

/// A deserializer that uses an index to traverse `input`
pub struct AsciiDeserializer<'de> {
    input: &'de [u8],
    index: usize,
}

impl<'de> AsciiDeserializer<'de> {
    pub fn new(input: &'de [u8]) -> Self {
        AsciiDeserializer { input, index: 0 }
    }

    /// Skip all whitespace characters
    fn skip_whitespace(&mut self) {
        while self.index < self.input.len() && is_whitespace(self.input[self.index]) {
            self.index += 1;
        }
    }
    /// Consume all remaining bytes and return them as a slice
    fn take_remaining(&mut self) -> &'de [u8] {
        let remaining = &self.input[self.index..];
        self.index = self.input.len(); // Move index to the end
        remaining
    }

    /// Read the next token (separated by whitespace). Returns `Some(&[u8])` slice; if no more data, returns `None`
    fn next_token(&mut self) -> Option<&'de [u8]> {
        self.skip_whitespace();
        if self.index >= self.input.len() {
            return None;
        }
        let start = self.index;

        // Find whitespace or end
        while self.index < self.input.len() && !is_whitespace(self.input[self.index]) {
            self.index += 1;
        }
        let end = self.index; // end points to whitespace or EOF

        Some(&self.input[start..end])
    }

    /// Method for peeking the next token (without consuming it)
    fn peek_token(&mut self) -> Option<&'de [u8]> {
        let saved = self.index;
        let t = self.next_token();
        // revert index after reading
        self.index = saved;
        t
    }
}

/// Helper function: determine if `b` is a whitespace character
fn is_whitespace(b: u8) -> bool {
    matches!(b, b' ' | b'\t' | b'\r' | b'\n')
}

////////////////////////////////////////////////////////////////////////////////
// Implement the `serde::Deserializer` trait
////////////////////////////////////////////////////////////////////////////////

impl<'de, 'a> Deserializer<'de> for &'a mut AsciiDeserializer<'de> {
    type Error = AsciiError;

    fn deserialize_any<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        // Not needed in this example
        Err(de::Error::custom("deserialize_any not supported"))
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        // Read one token; if "1" => true, if "0" => false, otherwise error
        let token = self.next_token().ok_or_else(|| AsciiError::new("EOF"))?;
        match token {
            b"1" => visitor.visit_bool(true),
            b"0" => visitor.visit_bool(false),
            _ => Err(de::Error::invalid_value(
                Unexpected::Bytes(token),
                &"expected '0' or '1'",
            )),
        }
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let token = self.next_token().ok_or_else(|| AsciiError::new("EOF"))?;
        let s = core::str::from_utf8(token).map_err(|_| AsciiError::new("UTF-8 error"))?;
        let parsed = s
            .parse::<i8>()
            .map_err(|_| AsciiError::new("parse i8 error"))?;
        visitor.visit_i8(parsed)
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let token = self.next_token().ok_or_else(|| AsciiError::new("EOF"))?;
        let s = core::str::from_utf8(token).map_err(|_| AsciiError::new("UTF-8 error"))?;
        let parsed = s
            .parse::<i16>()
            .map_err(|_| AsciiError::new("parse i16 error"))?;
        visitor.visit_i16(parsed)
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let token = self.next_token().ok_or_else(|| AsciiError::new("EOF"))?;
        let s = core::str::from_utf8(token).map_err(|_| AsciiError::new("UTF-8 error"))?;
        let parsed = s
            .parse::<i32>()
            .map_err(|_| AsciiError::new("parse i32 error"))?;
        visitor.visit_i32(parsed)
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let token = self.next_token().ok_or_else(|| AsciiError::new("EOF"))?;
        let s = core::str::from_utf8(token).map_err(|_| AsciiError::new("UTF-8 error"))?;
        let parsed = s
            .parse::<i64>()
            .map_err(|_| AsciiError::new("parse i64 error"))?;
        visitor.visit_i64(parsed)
    }

    fn deserialize_u8<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Err(de::Error::custom("u8 not supported"))
    }
    fn deserialize_u16<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Err(de::Error::custom("u16 not supported"))
    }
    fn deserialize_u32<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Err(de::Error::custom("u32 not supported"))
    }
    fn deserialize_u64<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Err(de::Error::custom("u64 not supported"))
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let token = self.next_token().ok_or_else(|| AsciiError::new("EOF"))?;
        let s = core::str::from_utf8(token).map_err(|_| AsciiError::new("UTF-8 error"))?;
        let parsed = s
            .parse::<f32>()
            .map_err(|_| AsciiError::new("parse f32 error"))?;
        visitor.visit_f32(parsed)
    }

    fn deserialize_f64<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Err(de::Error::custom("f64 not supported"))
    }

    fn deserialize_char<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Err(de::Error::custom("char not supported"))
    }

    fn deserialize_str<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_string<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        // Consume all remaining bytes and return them as a slice
        let remaining = self.take_remaining();
        visitor.visit_borrowed_bytes(remaining)
    }

    fn deserialize_byte_buf<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        // If the next token is "?", return None; otherwise Some(...)
        if let Some(token) = self.peek_token() {
            if token == b"?" {
                // consume it
                self.next_token();
                return visitor.visit_none();
            }
        }
        visitor.visit_some(self)
    }

    fn deserialize_unit<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Err(de::Error::custom("unit not supported"))
    }

    fn deserialize_unit_struct<V>(
        self,
        _name: &'static str,
        _visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Err(de::Error::custom("deserialize_unit_struct not supported"))
    }

    fn deserialize_newtype_struct<V>(
        self,
        _name: &'static str,
        _visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Err(de::Error::custom(
            "deserialize_newtype_struct not supported",
        ))
    }

    fn deserialize_seq<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Err(de::Error::custom("deserialize_seq not supported"))
    }

    fn deserialize_tuple<V>(self, _len: usize, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Err(de::Error::custom("deserialize_tuple not supported"))
    }

    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        _len: usize,
        _visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Err(de::Error::custom("deserialize_tuple_struct not supported"))
    }

    fn deserialize_map<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Err(de::Error::custom("deserialize_map not supported"))
    }

    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        _visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        // Not used in this example
        Err(de::Error::custom("deserialize_struct not supported"))
    }

    fn deserialize_identifier<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Err(de::Error::custom("deserialize_identifier not supported"))
    }

    /// Key point: parse enums (e.g. `varint 42` or `varint?`).
    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        // Take the next token as the "variant name"
        let variant_name = self.next_token().ok_or_else(|| AsciiError::new("EOF"))?;
        visitor.visit_enum(EnumRef {
            de: self,
            variant_name,
        })
    }

    fn deserialize_ignored_any<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Err(de::Error::custom("deserialize_ignored_any not supported"))
    }
}

////////////////////////////////////////////////////////////////////////////////
// EnumAccess / VariantAccess implementations
////////////////////////////////////////////////////////////////////////////////

/// Temporary structure for `visit_enum`
struct EnumRef<'a, 'de: 'a> {
    de: &'a mut AsciiDeserializer<'de>,
    variant_name: &'de [u8],
}

impl<'de, 'a> EnumAccess<'de> for EnumRef<'a, 'de> {
    type Error = AsciiError;
    type Variant = VariantRef<'a, 'de>;

    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant), Self::Error>
    where
        V: DeserializeSeed<'de>,
    {
        // Convert variant_name into &str for deserialization
        let s = core::str::from_utf8(self.variant_name)
            .map_err(|_| AsciiError::new("UTF-8 error in variant"))?;
        let v = seed.deserialize(s.into_deserializer())?;
        Ok((v, VariantRef { de: self.de }))
    }
}

struct VariantRef<'a, 'de: 'a> {
    de: &'a mut AsciiDeserializer<'de>,
}

impl<'de, 'a> VariantAccess<'de> for VariantRef<'a, 'de> {
    type Error = AsciiError;

    fn unit_variant(self) -> Result<(), Self::Error> {
        Err(de::Error::custom("unit_variant not supported"))
    }

    fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value, Self::Error>
    where
        T: DeserializeSeed<'de>,
    {
        // For example, `varint Some(42)` will parse an Option and then an i32
        seed.deserialize(&mut *self.de)
    }

    fn tuple_variant<V>(self, _len: usize, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Err(de::Error::custom("tuple_variant not supported"))
    }

    fn struct_variant<V>(
        self,
        _fields: &'static [&'static str],
        _visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Err(de::Error::custom("struct_variant not supported"))
    }
}

#[cfg(test)]
mod tests {
    use defmt_or_log::info;
    use serde::{Deserialize, Serialize};

    use super::*;
    use crate::testing::helpers::init_logger;

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
    }

    #[test]
    fn test_deserialize_varint_some() {
        init_logger();
        info!("test_deserialize_varint_some: INFO");

        let deserialized: TestReq = from_bytes(b"varint 42").unwrap();
        assert_eq!(deserialized, TestReq::VarInt(Some(42_i32)));
    }

    #[test]
    fn test_deserialize_varint_none() {
        init_logger();
        info!("test_deserialize_varint_none: INFO");

        let deserialized: TestReq = from_bytes(b"varint?").unwrap();
        assert_eq!(deserialized, TestReq::VarInt(None));
    }

    #[test]
    fn test_deserialize_varfloat_some() {
        init_logger();
        info!("test_deserialize_varfloat_some: INFO");

        let deserialized: TestReq = from_bytes(b"varfloat 3.14").unwrap();
        assert_eq!(deserialized, TestReq::VarFloat(Some(3.14_f32)));
    }

    #[test]
    fn test_deserialize_varfloat_none() {
        init_logger();
        info!("test_deserialize_varfloat_none: INFO");

        let deserialized: TestReq = from_bytes(b"varfloat?").unwrap();
        assert_eq!(deserialized, TestReq::VarFloat(None));
    }

    #[test]
    fn test_deserialize_varbool_some() {
        init_logger();
        info!("test_deserialize_varbool_some: INFO");

        let deserialized_true: TestReq = from_bytes(b"varbool 1").unwrap();
        assert_eq!(deserialized_true, TestReq::VarBool(Some(true)));

        let deserialized_false: TestReq = from_bytes(b"varbool 0").unwrap();
        assert_eq!(deserialized_false, TestReq::VarBool(Some(false)));
    }

    #[test]
    fn test_deserialize_varbool_none() {
        init_logger();
        info!("test_deserialize_varbool_none: INFO");

        let deserialized: TestReq = from_bytes(b"varbool?").unwrap();
        assert_eq!(deserialized, TestReq::VarBool(None));
    }

    #[test]
    fn test_deserialize_varbytes_some() {
        init_logger();
        info!("test_deserialize_varbytes_some: INFO");

        let deserialized: TestReq = from_bytes(b"varbytes hello").unwrap();
        assert_eq!(deserialized, TestReq::VarBytes(Some(b"hello")));
    }

    #[test]
    fn test_deserialize_varbytes_none() {
        init_logger();
        info!("test_deserialize_varbytes_none: INFO");

        let deserialized: TestReq = from_bytes(b"varbytes?").unwrap();
        assert_eq!(deserialized, TestReq::VarBytes(None));
    }
}
