use crate::format::error;
use crate::format::error::Error;
use crate::pd;
use serde::de;
use serde::de::{DeserializeSeed, IntoDeserializer, Visitor};
use std::collections::VecDeque;

pub struct Deserializer<'input> {
    input: &'input pd::PersistentDataRecord,
    token_offset: usize,
    arg_offset: usize,
}

impl<'input> Deserializer<'input> {
    pub fn from_pdr(input: &'input pd::PersistentDataRecord) -> Self {
        Deserializer {
            input,
            token_offset: 0,
            arg_offset: 0,
        }
    }

    pub fn peek_token(&self) -> error::Result<&'input pd::Tokens> {
        if self.token_offset < self.input.tokens.len() {
            Ok(&self.input.tokens[self.token_offset])
        } else {
            Err(Error::NoMoreTokens)
        }
    }

    pub fn has_token(&self, name: &str) -> error::Result<bool> {
        Ok(self.peek_token()?.value() == name)
    }

    pub fn pop_token(&mut self) -> error::Result<&'input pd::Tokens> {
        let value = self.peek_token()?;

        self.token_offset += 1;

        Ok(value)
    }

    pub fn pop_arg(&mut self) -> error::Result<pd::Arg> {
        if self.arg_offset >= self.input.args.len() {
            return Err(Error::NoMoreArgs);
        }
        let arg = self.input.args[self.arg_offset];

        self.arg_offset += 1;

        Ok(arg)
    }

    pub fn parse_string(&mut self) -> error::Result<&'input str> {
        if let pd::Tokens::STRING_TOKEN(_) = *self.pop_token()? {
            let arg = self.pop_arg()?;
            Ok(&self.input.strings[arg as usize])
        } else {
            Err(Error::ExpectedString)
        }
    }
}

impl<'input, 'a> serde::Deserializer<'input> for &'a mut Deserializer<'input> {
    type Error = Error;

    fn deserialize_any<V>(self, _visitor: V) -> error::Result<V::Value>
    where
        V: Visitor<'input>,
    {
        unimplemented!()
    }

    fn deserialize_bool<V>(self, visitor: V) -> error::Result<V::Value>
    where
        V: Visitor<'input>,
    {
        if let pd::Tokens::SINT_TOKEN(_) = *self.pop_token()? {
            let arg = self.pop_arg()?;
            visitor.visit_bool(arg != 0)
        } else {
            Err(Error::ExpectedSintToken)
        }
    }

    fn deserialize_i8<V>(self, visitor: V) -> error::Result<V::Value>
    where
        V: Visitor<'input>,
    {
        unimplemented!()
    }

    fn deserialize_i16<V>(self, visitor: V) -> error::Result<V::Value>
    where
        V: Visitor<'input>,
    {
        unimplemented!()
    }

    fn deserialize_i32<V>(self, visitor: V) -> error::Result<V::Value>
    where
        V: Visitor<'input>,
    {
        if let pd::Tokens::SINT_TOKEN(_) = *self.pop_token()? {
            let arg = self.pop_arg()?;
            visitor.visit_i32(arg as i32)
        } else {
            Err(Error::ExpectedSintToken)
        }
    }

    fn deserialize_i64<V>(self, visitor: V) -> error::Result<V::Value>
    where
        V: Visitor<'input>,
    {
        unimplemented!()
    }

    fn deserialize_u8<V>(self, visitor: V) -> error::Result<V::Value>
    where
        V: Visitor<'input>,
    {
        unimplemented!()
    }

    fn deserialize_u16<V>(self, visitor: V) -> error::Result<V::Value>
    where
        V: Visitor<'input>,
    {
        unimplemented!()
    }

    fn deserialize_u32<V>(self, visitor: V) -> error::Result<V::Value>
    where
        V: Visitor<'input>,
    {
        if let pd::Tokens::UINT_TOKEN(_) = *self.pop_token()? {
            visitor.visit_u32(self.pop_arg()?)
        } else {
            Err(Error::ExpectedUintToken)
        }
    }

    fn deserialize_u64<V>(self, visitor: V) -> error::Result<V::Value>
    where
        V: Visitor<'input>,
    {
        unimplemented!()
    }

    fn deserialize_f32<V>(self, _visitor: V) -> error::Result<V::Value>
    where
        V: Visitor<'input>,
    {
        unimplemented!()
    }

    fn deserialize_f64<V>(self, _visitor: V) -> error::Result<V::Value>
    where
        V: Visitor<'input>,
    {
        unimplemented!()
    }

    fn deserialize_char<V>(self, _visitor: V) -> error::Result<V::Value>
    where
        V: Visitor<'input>,
    {
        unimplemented!()
    }

    fn deserialize_str<V>(self, visitor: V) -> error::Result<V::Value>
    where
        V: Visitor<'input>,
    {
        visitor.visit_borrowed_str(self.parse_string()?)
    }

    fn deserialize_string<V>(self, visitor: V) -> error::Result<V::Value>
    where
        V: Visitor<'input>,
    {
        self.deserialize_str(visitor)
    }

    fn deserialize_bytes<V>(self, _visitor: V) -> error::Result<V::Value>
    where
        V: Visitor<'input>,
    {
        unimplemented!()
    }

    fn deserialize_byte_buf<V>(self, _visitor: V) -> error::Result<V::Value>
    where
        V: Visitor<'input>,
    {
        unimplemented!()
    }

    fn deserialize_option<V>(self, _visitor: V) -> error::Result<V::Value>
    where
        V: Visitor<'input>,
    {
        unimplemented!()
    }

    fn deserialize_unit<V>(self, _visitor: V) -> error::Result<V::Value>
    where
        V: Visitor<'input>,
    {
        unimplemented!()
    }

    // Unit struct means a named value containing no data.
    fn deserialize_unit_struct<V>(self, _name: &'static str, _visitor: V) -> error::Result<V::Value>
    where
        V: Visitor<'input>,
    {
        unimplemented!()
    }

    fn deserialize_newtype_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> error::Result<V::Value>
    where
        V: Visitor<'input>,
    {
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_seq<V>(self, visitor: V) -> error::Result<V::Value>
    where
        V: Visitor<'input>,
    {
        unimplemented!()
    }

    fn deserialize_tuple<V>(self, _len: usize, visitor: V) -> error::Result<V::Value>
    where
        V: Visitor<'input>,
    {
        self.deserialize_seq(visitor)
    }

    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        _len: usize,
        visitor: V,
    ) -> error::Result<V::Value>
    where
        V: Visitor<'input>,
    {
        self.deserialize_seq(visitor)
    }

    fn deserialize_map<V>(self, _visitor: V) -> error::Result<V::Value>
    where
        V: Visitor<'input>,
    {
        unimplemented!()
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
        fields: &'static [&'static str],
        visitor: V,
    ) -> error::Result<V::Value>
    where
        V: Visitor<'input>,
    {
        let value = visitor.visit_map(StructAccess::new(self, fields))?;
        Ok(value)
    }

    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        visitor: V,
    ) -> error::Result<V::Value>
    where
        V: Visitor<'input>,
    {
        unimplemented!()
    }

    // An identifier in Serde is the type that identifies a field of a struct or
    // the variant of an enum. In JSON, struct fields and enum variants are
    // represented as strings. In other formats they may be represented as
    // numeric indices.
    fn deserialize_identifier<V>(self, visitor: V) -> error::Result<V::Value>
    where
        V: Visitor<'input>,
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
    fn deserialize_ignored_any<V>(self, visitor: V) -> error::Result<V::Value>
    where
        V: Visitor<'input>,
    {
        self.deserialize_any(visitor)
    }
}

struct StructAccess<'a, 'de: 'a> {
    de: &'a mut Deserializer<'de>,
    fields: VecDeque<&'static str>,
}

impl<'a, 'de> StructAccess<'a, 'de> {
    fn new(de: &'a mut Deserializer<'de>, fields: &'static [&'static str]) -> Self {
        Self {
            de,
            fields: fields.iter().cloned().collect(),
        }
    }
}

impl<'de, 'a> de::MapAccess<'de> for StructAccess<'a, 'de> {
    type Error = Error;

    fn next_key_seed<K>(&mut self, seed: K) -> error::Result<Option<K::Value>>
    where
        K: DeserializeSeed<'de>,
    {
        let field = self.fields.pop_front();
        if let Some(name) = field {
            if self.de.has_token(name)? {
                seed.deserialize(&mut *self.de).map(Some)
            } else {
                Ok(None)
            }
        } else {
            Ok(None)
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> error::Result<V::Value>
    where
        V: DeserializeSeed<'de>,
    {
        let token = self.de.peek_token()?;
        seed.deserialize(&mut *self.de)
    }
}
