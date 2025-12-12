use crate::format::de::deserializer::Deserializer;
use crate::format::error;
use crate::format::error::Error;
use crate::pd;
use serde::de::{DeserializeSeed, SeqAccess, Visitor};

pub struct ChildDeserializer<'child, 'input: 'child> {
    de: &'child mut Deserializer<'input>,
    field: &'static str,
}

impl<'child, 'input> ChildDeserializer<'child, 'input> {
    fn new(de: &'child mut Deserializer<'input>, field: &'static str) -> Self {
        Self { de, field }
    }

    fn peek_token(&self) -> error::Result<&'input pd::Tokens> {
        self.de.peek_token()
    }
}

impl<'a, 'child, 'input> serde::Deserializer<'input> for &'a mut ChildDeserializer<'child, 'input> {
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
        if let pd::Tokens::SINT_TOKEN(_) = *self.de.pop_token()? {
            let arg = self.de.pop_arg()?;
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
        if let pd::Tokens::SINT_TOKEN(_) = *self.de.pop_token()? {
            let arg = self.de.pop_arg()?;
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
        if let pd::Tokens::UINT_TOKEN(_) = *self.de.pop_token()? {
            visitor.visit_u32(self.de.pop_arg()?)
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

    // Float parsing is stupidly hard.
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
        visitor.visit_borrowed_str(self.de.parse_string()?)
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
        let value = visitor.visit_seq(SameField::new(self, self.field))?;

        Ok(value)
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
        unimplemented!()
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

    fn deserialize_identifier<V>(self, visitor: V) -> error::Result<V::Value>
    where
        V: Visitor<'input>,
    {
        let token = self.de.peek_token()?;
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

struct SameField<'a, 'child, 'de: 'a> {
    de: &'a mut ChildDeserializer<'child, 'de>,
    field: &'static str,
}

impl<'a, 'child, 'de> SameField<'a, 'child, 'de> {
    fn new(de: &'a mut ChildDeserializer<'child, 'de>, field: &'static str) -> Self {
        Self { de, field }
    }
}

impl<'a, 'child, 'de> SeqAccess<'de> for SameField<'a, 'child, 'de> {
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> error::Result<Option<T::Value>>
    where
        T: DeserializeSeed<'de>,
    {
        if self.de.peek_token()?.value() != self.field {
            Ok(None)
        } else {
            seed.deserialize(&mut *self.de).map(Some)
        }
    }
}
