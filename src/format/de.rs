use std::ops::{AddAssign, MulAssign, Neg};

use crate::format::error::{Error, Result};
use crate::pd;
use serde::de::{
    self, DeserializeSeed, EnumAccess, IntoDeserializer, SeqAccess, VariantAccess, Visitor,
};
use serde::Deserialize;

pub struct Deserializer<'de> {
    input: &'de pd::PersistentDataRecord,
    token_offset: usize,
    arg_offset: usize,
}

impl<'de> Deserializer<'de> {
    pub fn from_pdr(input: &'de pd::PersistentDataRecord) -> Self {
        Deserializer {
            input,
            token_offset: 0,
            arg_offset: 0,
        }
    }
}

pub fn from_pdr<'a, T>(s: &'a pd::PersistentDataRecord) -> Result<T>
where
    T: Deserialize<'a>,
{
    let mut deserializer = Deserializer::from_pdr(s);
    let t = T::deserialize(&mut deserializer)?;

    Ok(t)
}

impl<'de> Deserializer<'de> {
    fn peek_token(&self) -> Result<&'de pd::Tokens> {
        if self.token_offset < self.input.tokens.len() {
            Ok(&self.input.tokens[self.token_offset])
        } else {
            Err(Error::NoMoreTokens)
        }
    }

    fn has_token(&self) -> Result<bool> {
        Ok(self.token_offset < self.input.tokens.len())
    }

    fn pop_token(&mut self) -> Result<&'de pd::Tokens> {
        let value = self.peek_token()?;

        self.token_offset += 1;

        Ok(value)
    }

    fn pop_arg(&mut self) -> Result<pd::Arg> {
        if self.arg_offset >= self.input.args.len() {
            return Err(Error::NoMoreArgs);
        }
        let arg = self.input.args[self.arg_offset];

        self.arg_offset += 1;

        Ok(arg)
    }

    fn parse_bool(&mut self) -> Result<bool> {
        if let pd::Tokens::SINT_TOKEN(_) = *self.pop_token()? {
            Ok(self.pop_arg()? != 0)
        } else {
            Err(Error::Message("expected bool".to_string()))
        }
    }

    fn parse_unsigned<T>(&mut self) -> Result<T>
    where
        T: AddAssign<T> + MulAssign<T> + From<u8>,
    {
        unimplemented!()
    }

    // Parse a possible minus sign followed by a group of decimal digits as a
    // signed integer of type T.
    fn parse_signed<T>(&mut self) -> Result<T>
    where
        T: Neg<Output = T> + AddAssign<T> + MulAssign<T> + From<i8>,
    {
        unimplemented!()
    }

    fn parse_string(&mut self) -> Result<&'de str> {
        if let pd::Tokens::STRING_TOKEN(_) = *self.pop_token()? {
            let arg = self.pop_arg()?;
            Ok(&self.input.strings[arg as usize])
        } else {
            Err(Error::ExpectedString)
        }
    }
}

impl<'de, 'a> de::Deserializer<'de> for &'a mut Deserializer<'de> {
    type Error = Error;

    fn deserialize_any<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        /*
        match self.peek_char()? {
            'n' => self.deserialize_unit(visitor),
            't' | 'f' => self.deserialize_bool(visitor),
            '"' => self.deserialize_str(visitor),
            '0'..='9' => self.deserialize_u64(visitor),
            '-' => self.deserialize_i64(visitor),
            '[' => self.deserialize_seq(visitor),
            '{' => self.deserialize_map(visitor),
            _ => Err(Error::InvalidFormat),
        }
         */
        unimplemented!()
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_bool(self.parse_bool()?)
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i8(self.parse_signed()?)
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i16(self.parse_signed()?)
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        if let pd::Tokens::SINT_TOKEN(_) = *self.pop_token()? {
            let arg = self.pop_arg()?;
            visitor.visit_i32(arg as i32)
        } else {
            Err(Error::ExpectedSintToken)
        }
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i64(self.parse_signed()?)
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u8(self.parse_unsigned()?)
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u16(self.parse_unsigned()?)
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        if let pd::Tokens::UINT_TOKEN(_) = *self.pop_token()? {
            visitor.visit_u32(self.pop_arg()?)
        } else {
            Err(Error::ExpectedUintToken)
        }
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u64(self.parse_unsigned()?)
    }

    // Float parsing is stupidly hard.
    fn deserialize_f32<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_f64<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_char<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_borrowed_str(self.parse_string()?)
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_str(visitor)
    }

    fn deserialize_bytes<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_byte_buf<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_option<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_unit<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    // Unit struct means a named value containing no data.
    fn deserialize_unit_struct<V>(self, _name: &'static str, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_newtype_struct<V>(self, _name: &'static str, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        /*
        let value = visitor.visit_seq(SameName::new(self))?;
        Ok(value)
         */
        unimplemented!()
    }

    fn deserialize_tuple<V>(self, _len: usize, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_seq(visitor)
    }

    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        _len: usize,
        visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_seq(visitor)
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let value = visitor.visit_map(MapAccess::new(self))?;
        Ok(value)
    }

    // Structs look just like maps in JSON.
    //
    // Notice the `fields` parameter - a "struct" in the Serde data model means
    // that the `Deserialize` implementation is required to know what the fields
    // are before even looking at the input data. Any key-value pairing in which
    // the fields cannot be known ahead of time is probably a map.
    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_map(visitor)
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
        if let pd::Tokens::STRING_TOKEN(_) = self.peek_token()? {
            // Visit a unit variant.
            visitor.visit_enum(self.parse_string()?.into_deserializer())
        } else if let pd::Tokens::BEGIN_TOKEN(_) = self.peek_token()? {
            // Visit a newtype variant, tuple variant, or struct variant.
            let value = visitor.visit_enum(Enum::new(self))?;
            // Parse the matching close brace.
            if let pd::Tokens::END_TOKEN(_) = self.peek_token()? {
                Ok(value)
            } else {
                Err(Error::ExpectedEndToken)
            }
        } else {
            Err(Error::ExpectedEnum)
        }
    }

    // An identifier in Serde is the type that identifies a field of a struct or
    // the variant of an enum. In JSON, struct fields and enum variants are
    // represented as strings. In other formats they may be represented as
    // numeric indices.
    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let token = self.peek_token()?;
        let value = visitor.visit_borrowed_str(token.value())?;
        Ok(value)
    }

    // Like `deserialize_any` but indicates to the `Deserializer` that it makes
    // no difference which `Visitor` method is called because the data is
    // ignored.
    //
    // Some deserializers are able to implement this more efficiently than
    // `deserialize_any`, for example by rapidly skipping over matched
    // delimiters without paying close attention to the data in between.
    //
    // Some formats are not able to implement this at all. Formats that can
    // implement `deserialize_any` and `deserialize_ignored_any` are known as
    // self-describing.
    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_any(visitor)
    }
}

// In order to handle commas correctly when deserializing a JSON array or map,
// we need to track whether we are on the first element or past the first
// element.
struct SameName<'a, 'de: 'a> {
    de: &'a mut Deserializer<'de>,
    name: &'a str,
}

impl<'a, 'de> SameName<'a, 'de> {
    fn new(de: &'a mut Deserializer<'de>, name: &'a str) -> Self {
        SameName { de, name }
    }
}

impl<'de, 'a> SeqAccess<'de> for SameName<'a, 'de> {
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>>
    where
        T: DeserializeSeed<'de>,
    {
        if self.de.peek_token()?.value() == self.name {
            return Ok(None);
        }

        seed.deserialize(&mut *self.de).map(Some)
    }
}

struct MapAccess<'a, 'de: 'a> {
    de: &'a mut Deserializer<'de>,
}

impl<'a, 'de> MapAccess<'a, 'de> {
    fn new(de: &'a mut Deserializer<'de>) -> Self {
        MapAccess { de }
    }
}

impl<'de, 'a> de::MapAccess<'de> for MapAccess<'a, 'de> {
    type Error = Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>>
    where
        K: DeserializeSeed<'de>,
    {
        if self.de.has_token()? {
            seed.deserialize(&mut *self.de).map(Some)
        } else {
            Ok(None)
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value>
    where
        V: DeserializeSeed<'de>,
    {
        seed.deserialize(&mut *self.de)
    }
}

struct Enum<'a, 'de: 'a> {
    de: &'a mut Deserializer<'de>,
}

impl<'a, 'de> Enum<'a, 'de> {
    fn new(de: &'a mut Deserializer<'de>) -> Self {
        Enum { de }
    }
}

impl<'de, 'a> EnumAccess<'de> for Enum<'a, 'de> {
    type Error = Error;
    type Variant = Self;

    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant)>
    where
        V: DeserializeSeed<'de>,
    {
        let val = seed.deserialize(&mut *self.de)?;

        Ok((val, self))
    }
}

// `VariantAccess` is provided to the `Visitor` to give it the ability to see
// the content of the single variant that it decided to deserialize.
impl<'de, 'a> VariantAccess<'de> for Enum<'a, 'de> {
    type Error = Error;

    // If the `Visitor` expected this variant to be a unit variant, the input
    // should have been the plain string case handled in `deserialize_enum`.
    fn unit_variant(self) -> Result<()> {
        Err(Error::ExpectedString)
    }

    // Newtype variants are represented in JSON as `{ NAME: VALUE }` so
    // deserialize the value here.
    fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value>
    where
        T: DeserializeSeed<'de>,
    {
        seed.deserialize(self.de)
    }

    // Tuple variants are represented in JSON as `{ NAME: [DATA...] }` so
    // deserialize the sequence of data here.
    fn tuple_variant<V>(self, _len: usize, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        de::Deserializer::deserialize_seq(self.de, visitor)
    }

    // Struct variants are represented in JSON as `{ NAME: { K: V, ... } }` so
    // deserialize the inner map here.
    fn struct_variant<V>(self, _fields: &'static [&'static str], visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        de::Deserializer::deserialize_map(self.de, visitor)
    }
}

////////////////////////////////////////////////////////////////////////////////
mod tests {
    use super::*;
    use crate::pd::{PersistentDataRecord, Tokens};
    #[test]
    fn test_simple_values() {
        #[derive(serde::Deserialize, PartialEq, Debug)]
        struct Test {
            int: u32,
            a: String,
            b: String,
        }

        let j = PersistentDataRecord {
            _TokenOffset: 0,
            _ArgOffset: 0,
            tokens: vec![
                Tokens::UINT_TOKEN("int".to_string()),
                Tokens::STRING_TOKEN("a".to_string()),
                Tokens::STRING_TOKEN("b".to_string()),
            ],
            args: vec![1, 1, 2],
            strings: vec!["int".to_string(), "a".to_string(), "b".to_string()],
        };
        let expected = Test {
            int: 1,
            a: "a".to_owned(),
            b: "b".to_owned(),
        };
        assert_eq!(expected, from_pdr(&j).unwrap());
    }

    #[test]
    fn test_struct() {
        #[derive(serde::Deserialize, PartialEq, Debug)]
        struct Test {
            int: u32,
            seq: Vec<String>,
        }

        let j = PersistentDataRecord {
            _TokenOffset: 0,
            _ArgOffset: 0,
            tokens: vec![
                Tokens::UINT_TOKEN("int".to_string()),
                Tokens::STRING_TOKEN("seq".to_string()),
                Tokens::STRING_TOKEN("seq".to_string()),
            ],
            args: vec![1, 2, 3],
            strings: vec![
                "int".to_string(),
                "seq".to_string(),
                "a".to_string(),
                "b".to_string(),
            ],
        };
        let expected = Test {
            int: 1,
            seq: vec!["a".to_owned(), "b".to_owned()],
        };
        assert_eq!(expected, from_pdr(&j).unwrap());
    }

    /*
    #[test]
    fn test_enum() {
        #[derive(Deserialize, PartialEq, Debug)]
        enum E {
            Unit,
            Newtype(u32),
            Tuple(u32, u32),
            Struct { a: u32 },
        }

        let j = r#""Unit""#;
        let expected = E::Unit;
        assert_eq!(expected, from_str(j).unwrap());

        let j = r#"{"Newtype":1}"#;
        let expected = E::Newtype(1);
        assert_eq!(expected, from_str(j).unwrap());

        let j = r#"{"Tuple":[1,2]}"#;
        let expected = E::Tuple(1, 2);
        assert_eq!(expected, from_str(j).unwrap());

        let j = r#"{"Struct":{"a":1}}"#;
        let expected = E::Struct { a: 1 };
        assert_eq!(expected, from_str(j).unwrap());
    }

     */
}
