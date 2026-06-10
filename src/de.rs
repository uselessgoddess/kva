use alloc::{
    borrow::{Cow, ToOwned},
    format,
    string::{String, ToString},
};
use core::slice;

use serde::de::{
    self, DeserializeSeed, Deserializer as _, EnumAccess, IntoDeserializer, MapAccess, SeqAccess,
    VariantAccess, Visitor,
};

use crate::{Error, KvData, KvEntry, Result};

macro_rules! delegate_to_value_deserializer {
    ($human:expr) => {
        fn deserialize_any<V>(self, visitor: V) -> $crate::Result<V::Value>
        where
            V: ::serde::de::Visitor<'de>,
        {
            ::serde::de::Deserializer::deserialize_any(
                $crate::de::ValueDeserializer::new(&self.root.data, $human),
                visitor,
            )
        }

        fn deserialize_bool<V>(self, visitor: V) -> $crate::Result<V::Value>
        where
            V: ::serde::de::Visitor<'de>,
        {
            ::serde::de::Deserializer::deserialize_bool(
                $crate::de::ValueDeserializer::new(&self.root.data, $human),
                visitor,
            )
        }

        fn deserialize_i8<V>(self, visitor: V) -> $crate::Result<V::Value>
        where
            V: ::serde::de::Visitor<'de>,
        {
            ::serde::de::Deserializer::deserialize_i8(
                $crate::de::ValueDeserializer::new(&self.root.data, $human),
                visitor,
            )
        }

        fn deserialize_i16<V>(self, visitor: V) -> $crate::Result<V::Value>
        where
            V: ::serde::de::Visitor<'de>,
        {
            ::serde::de::Deserializer::deserialize_i16(
                $crate::de::ValueDeserializer::new(&self.root.data, $human),
                visitor,
            )
        }

        fn deserialize_i32<V>(self, visitor: V) -> $crate::Result<V::Value>
        where
            V: ::serde::de::Visitor<'de>,
        {
            ::serde::de::Deserializer::deserialize_i32(
                $crate::de::ValueDeserializer::new(&self.root.data, $human),
                visitor,
            )
        }

        fn deserialize_i64<V>(self, visitor: V) -> $crate::Result<V::Value>
        where
            V: ::serde::de::Visitor<'de>,
        {
            ::serde::de::Deserializer::deserialize_i64(
                $crate::de::ValueDeserializer::new(&self.root.data, $human),
                visitor,
            )
        }

        fn deserialize_u8<V>(self, visitor: V) -> $crate::Result<V::Value>
        where
            V: ::serde::de::Visitor<'de>,
        {
            ::serde::de::Deserializer::deserialize_u8(
                $crate::de::ValueDeserializer::new(&self.root.data, $human),
                visitor,
            )
        }

        fn deserialize_u16<V>(self, visitor: V) -> $crate::Result<V::Value>
        where
            V: ::serde::de::Visitor<'de>,
        {
            ::serde::de::Deserializer::deserialize_u16(
                $crate::de::ValueDeserializer::new(&self.root.data, $human),
                visitor,
            )
        }

        fn deserialize_u32<V>(self, visitor: V) -> $crate::Result<V::Value>
        where
            V: ::serde::de::Visitor<'de>,
        {
            ::serde::de::Deserializer::deserialize_u32(
                $crate::de::ValueDeserializer::new(&self.root.data, $human),
                visitor,
            )
        }

        fn deserialize_u64<V>(self, visitor: V) -> $crate::Result<V::Value>
        where
            V: ::serde::de::Visitor<'de>,
        {
            ::serde::de::Deserializer::deserialize_u64(
                $crate::de::ValueDeserializer::new(&self.root.data, $human),
                visitor,
            )
        }

        fn deserialize_f32<V>(self, visitor: V) -> $crate::Result<V::Value>
        where
            V: ::serde::de::Visitor<'de>,
        {
            ::serde::de::Deserializer::deserialize_f32(
                $crate::de::ValueDeserializer::new(&self.root.data, $human),
                visitor,
            )
        }

        fn deserialize_f64<V>(self, visitor: V) -> $crate::Result<V::Value>
        where
            V: ::serde::de::Visitor<'de>,
        {
            ::serde::de::Deserializer::deserialize_f64(
                $crate::de::ValueDeserializer::new(&self.root.data, $human),
                visitor,
            )
        }

        fn deserialize_char<V>(self, visitor: V) -> $crate::Result<V::Value>
        where
            V: ::serde::de::Visitor<'de>,
        {
            ::serde::de::Deserializer::deserialize_char(
                $crate::de::ValueDeserializer::new(&self.root.data, $human),
                visitor,
            )
        }

        fn deserialize_str<V>(self, visitor: V) -> $crate::Result<V::Value>
        where
            V: ::serde::de::Visitor<'de>,
        {
            ::serde::de::Deserializer::deserialize_str(
                $crate::de::ValueDeserializer::new(&self.root.data, $human),
                visitor,
            )
        }

        fn deserialize_string<V>(self, visitor: V) -> $crate::Result<V::Value>
        where
            V: ::serde::de::Visitor<'de>,
        {
            ::serde::de::Deserializer::deserialize_string(
                $crate::de::ValueDeserializer::new(&self.root.data, $human),
                visitor,
            )
        }

        fn deserialize_bytes<V>(self, visitor: V) -> $crate::Result<V::Value>
        where
            V: ::serde::de::Visitor<'de>,
        {
            ::serde::de::Deserializer::deserialize_bytes(
                $crate::de::ValueDeserializer::new(&self.root.data, $human),
                visitor,
            )
        }

        fn deserialize_byte_buf<V>(self, visitor: V) -> $crate::Result<V::Value>
        where
            V: ::serde::de::Visitor<'de>,
        {
            ::serde::de::Deserializer::deserialize_byte_buf(
                $crate::de::ValueDeserializer::new(&self.root.data, $human),
                visitor,
            )
        }

        fn deserialize_option<V>(self, visitor: V) -> $crate::Result<V::Value>
        where
            V: ::serde::de::Visitor<'de>,
        {
            ::serde::de::Deserializer::deserialize_option(
                $crate::de::ValueDeserializer::new(&self.root.data, $human),
                visitor,
            )
        }

        fn deserialize_unit<V>(self, visitor: V) -> $crate::Result<V::Value>
        where
            V: ::serde::de::Visitor<'de>,
        {
            ::serde::de::Deserializer::deserialize_unit(
                $crate::de::ValueDeserializer::new(&self.root.data, $human),
                visitor,
            )
        }

        fn deserialize_unit_struct<V>(
            self,
            name: &'static str,
            visitor: V,
        ) -> $crate::Result<V::Value>
        where
            V: ::serde::de::Visitor<'de>,
        {
            ::serde::de::Deserializer::deserialize_unit_struct(
                $crate::de::ValueDeserializer::new(&self.root.data, $human),
                name,
                visitor,
            )
        }

        fn deserialize_newtype_struct<V>(
            self,
            name: &'static str,
            visitor: V,
        ) -> $crate::Result<V::Value>
        where
            V: ::serde::de::Visitor<'de>,
        {
            ::serde::de::Deserializer::deserialize_newtype_struct(
                $crate::de::ValueDeserializer::new(&self.root.data, $human),
                name,
                visitor,
            )
        }

        fn deserialize_seq<V>(self, visitor: V) -> $crate::Result<V::Value>
        where
            V: ::serde::de::Visitor<'de>,
        {
            ::serde::de::Deserializer::deserialize_seq(
                $crate::de::ValueDeserializer::new(&self.root.data, $human),
                visitor,
            )
        }

        fn deserialize_tuple<V>(self, len: usize, visitor: V) -> $crate::Result<V::Value>
        where
            V: ::serde::de::Visitor<'de>,
        {
            ::serde::de::Deserializer::deserialize_tuple(
                $crate::de::ValueDeserializer::new(&self.root.data, $human),
                len,
                visitor,
            )
        }

        fn deserialize_tuple_struct<V>(
            self,
            name: &'static str,
            len: usize,
            visitor: V,
        ) -> $crate::Result<V::Value>
        where
            V: ::serde::de::Visitor<'de>,
        {
            ::serde::de::Deserializer::deserialize_tuple_struct(
                $crate::de::ValueDeserializer::new(&self.root.data, $human),
                name,
                len,
                visitor,
            )
        }

        fn deserialize_map<V>(self, visitor: V) -> $crate::Result<V::Value>
        where
            V: ::serde::de::Visitor<'de>,
        {
            ::serde::de::Deserializer::deserialize_map(
                $crate::de::ValueDeserializer::new(&self.root.data, $human),
                visitor,
            )
        }

        fn deserialize_struct<V>(
            self,
            name: &'static str,
            fields: &'static [&'static str],
            visitor: V,
        ) -> $crate::Result<V::Value>
        where
            V: ::serde::de::Visitor<'de>,
        {
            ::serde::de::Deserializer::deserialize_struct(
                $crate::de::ValueDeserializer::new(&self.root.data, $human),
                name,
                fields,
                visitor,
            )
        }

        fn deserialize_enum<V>(
            self,
            name: &'static str,
            variants: &'static [&'static str],
            visitor: V,
        ) -> $crate::Result<V::Value>
        where
            V: ::serde::de::Visitor<'de>,
        {
            ::serde::de::Deserializer::deserialize_enum(
                $crate::de::ValueDeserializer::new(&self.root.data, $human),
                name,
                variants,
                visitor,
            )
        }

        fn deserialize_identifier<V>(self, visitor: V) -> $crate::Result<V::Value>
        where
            V: ::serde::de::Visitor<'de>,
        {
            ::serde::de::Deserializer::deserialize_identifier(
                $crate::de::ValueDeserializer::new(&self.root.data, $human),
                visitor,
            )
        }

        fn deserialize_ignored_any<V>(self, visitor: V) -> $crate::Result<V::Value>
        where
            V: ::serde::de::Visitor<'de>,
        {
            ::serde::de::Deserializer::deserialize_ignored_any(
                $crate::de::ValueDeserializer::new(&self.root.data, $human),
                visitor,
            )
        }
    };
}

pub(crate) use delegate_to_value_deserializer;

#[derive(Clone, Copy)]
pub(crate) struct ValueDeserializer<'a, 'de> {
    data: &'a KvData<'de>,
    human: bool,
}

impl<'a, 'de> ValueDeserializer<'a, 'de> {
    pub(crate) fn new(data: &'a KvData<'de>, human: bool) -> Self {
        Self { data, human }
    }

    fn string_value(self) -> Result<Cow<'de, str>> {
        match self.data {
            KvData::String(value) => Ok(value.clone()),
            KvData::WideString(value) => String::from_utf16(value)
                .map(Cow::Owned)
                .map_err(|_| Error::InvalidUtf16),
            KvData::Int(value) => Ok(Cow::Owned(value.to_string())),
            KvData::Int64(value) => Ok(Cow::Owned(value.to_string())),
            KvData::UInt64(value) => Ok(Cow::Owned(value.to_string())),
            KvData::Float(value) => Ok(Cow::Owned(value.to_string())),
            KvData::Pointer(value) => Ok(Cow::Owned(value.to_string())),
            KvData::Color(value) => Ok(Cow::Owned(value.to_string())),
            KvData::BinaryString(_) | KvData::Compound(_) => Err(Error::UnexpectedToken),
        }
    }

    fn parse_bool(self) -> Result<bool> {
        match self.data {
            KvData::String(value) => match value.as_ref() {
                "1" | "true" | "True" | "TRUE" => Ok(true),
                "0" | "false" | "False" | "FALSE" | "" => Ok(false),
                _ => Err(Error::Message(format!("invalid bool value {value:?}"))),
            },
            KvData::Int(value) => Ok(*value != 0),
            KvData::Int64(value) => Ok(*value != 0),
            KvData::UInt64(value) => Ok(*value != 0),
            KvData::Float(value) => Ok(*value != 0.0),
            _ => Err(Error::UnexpectedToken),
        }
    }

    fn parse_i64(self) -> Result<i64> {
        match self.data {
            KvData::String(value) => parse_i64(value),
            KvData::Int(value) => Ok((*value).into()),
            KvData::Int64(value) => Ok(*value),
            KvData::UInt64(value) => i64::try_from(*value)
                .map_err(|_| Error::Message("u64 does not fit in i64".to_owned())),
            KvData::Float(value) => Ok(*value as i64),
            KvData::Pointer(value) => i64::try_from(*value)
                .map_err(|_| Error::Message("usize does not fit in i64".to_owned())),
            KvData::Color(value) => Ok((*value).into()),
            _ => Err(Error::UnexpectedToken),
        }
    }

    fn parse_u64(self) -> Result<u64> {
        match self.data {
            KvData::String(value) => parse_u64(value),
            KvData::Int(value) => u64::try_from(*value)
                .map_err(|_| Error::Message("negative integer does not fit in u64".to_owned())),
            KvData::Int64(value) => u64::try_from(*value)
                .map_err(|_| Error::Message("negative integer does not fit in u64".to_owned())),
            KvData::UInt64(value) => Ok(*value),
            KvData::Float(value) => Ok(*value as u64),
            KvData::Pointer(value) => Ok(*value as u64),
            KvData::Color(value) => Ok((*value).into()),
            _ => Err(Error::UnexpectedToken),
        }
    }

    fn parse_f64(self) -> Result<f64> {
        match self.data {
            KvData::String(value) => value
                .parse()
                .map_err(|_| Error::Message(format!("invalid float value {value:?}"))),
            KvData::Int(value) => Ok((*value).into()),
            KvData::Int64(value) => Ok(*value as f64),
            KvData::UInt64(value) => Ok(*value as f64),
            KvData::Float(value) => Ok((*value).into()),
            KvData::Pointer(value) => Ok(*value as f64),
            KvData::Color(value) => Ok((*value).into()),
            _ => Err(Error::UnexpectedToken),
        }
    }
}

impl<'de> de::Deserializer<'de> for ValueDeserializer<'_, 'de> {
    type Error = Error;

    fn is_human_readable(&self) -> bool {
        self.human
    }

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        match self.data {
            KvData::String(value) => visit_str(value, visitor),
            KvData::WideString(value) => {
                visitor.visit_string(String::from_utf16(value).map_err(|_| Error::InvalidUtf16)?)
            }
            KvData::BinaryString(value) => visit_bytes(value, visitor),
            // behave consistently regardless of `numeric_inference` in human readable
            KvData::Int(_)
            | KvData::Int64(_)
            | KvData::UInt64(_)
            | KvData::Float(_)
            | KvData::Pointer(_)
            | KvData::Color(_)
                if self.human =>
            {
                let value = self.string_value()?;
                visit_str(&value, visitor)
            }
            KvData::Int(value) => visitor.visit_i32(*value),
            KvData::Int64(value) => visitor.visit_i64(*value),
            KvData::UInt64(value) => visitor.visit_u64(*value),
            KvData::Float(value) => visitor.visit_f32(*value),
            KvData::Pointer(value) => visitor.visit_u64(*value as u64),
            KvData::Color(value) => visitor.visit_u32(*value),
            KvData::Compound(entries) => {
                visitor.visit_map(CompoundAccess::new(entries, self.human))
            }
        }
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
        visitor.visit_i8(
            i8::try_from(self.parse_i64()?)
                .map_err(|_| Error::Message("integer does not fit in i8".to_owned()))?,
        )
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i16(
            i16::try_from(self.parse_i64()?)
                .map_err(|_| Error::Message("integer does not fit in i16".to_owned()))?,
        )
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i32(
            i32::try_from(self.parse_i64()?)
                .map_err(|_| Error::Message("integer does not fit in i32".to_owned()))?,
        )
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i64(self.parse_i64()?)
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u8(
            u8::try_from(self.parse_u64()?)
                .map_err(|_| Error::Message("integer does not fit in u8".to_owned()))?,
        )
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u16(
            u16::try_from(self.parse_u64()?)
                .map_err(|_| Error::Message("integer does not fit in u16".to_owned()))?,
        )
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u32(
            u32::try_from(self.parse_u64()?)
                .map_err(|_| Error::Message("integer does not fit in u32".to_owned()))?,
        )
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u64(self.parse_u64()?)
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_f32(self.parse_f64()? as f32)
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_f64(self.parse_f64()?)
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let value = self.string_value()?;
        let mut chars = value.chars();
        let Some(ch) = chars.next() else {
            return Err(Error::Message(
                "expected char, found empty string".to_owned(),
            ));
        };
        if chars.next().is_some() {
            return Err(Error::Message("expected one character".to_owned()));
        }
        visitor.visit_char(ch)
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let value = self.string_value()?;
        visit_str(&value, visitor)
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let value = self.string_value()?;
        match value {
            Cow::Borrowed(value) => visitor.visit_borrowed_str(value),
            Cow::Owned(value) => visitor.visit_string(value),
        }
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        match self.data {
            KvData::BinaryString(value) => visit_bytes(value, visitor),
            KvData::String(Cow::Borrowed(value)) => visitor.visit_borrowed_bytes(value.as_bytes()),
            KvData::String(Cow::Owned(value)) => visitor.visit_bytes(value.as_bytes()),
            _ => Err(Error::UnexpectedToken),
        }
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        match self.data {
            KvData::BinaryString(value) => match value {
                Cow::Borrowed(value) => visitor.visit_borrowed_bytes(value),
                Cow::Owned(value) => visitor.visit_byte_buf(value.clone()),
            },
            KvData::String(value) => visitor.visit_byte_buf(value.as_bytes().to_vec()),
            _ => Err(Error::UnexpectedToken),
        }
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        if matches!(self.data, KvData::String(value) if value.is_empty()) {
            visitor.visit_none()
        } else {
            visitor.visit_some(self)
        }
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        match self.data {
            KvData::String(value) if value.is_empty() => visitor.visit_unit(),
            KvData::Compound(entries) if entries.is_empty() => visitor.visit_unit(),
            _ => Err(Error::UnexpectedToken),
        }
    }

    fn deserialize_unit_struct<V>(self, _name: &'static str, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_unit(visitor)
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
        match self.data {
            KvData::Compound(entries) => visitor.visit_seq(CompoundSeqAccess {
                iter: entries.iter(),
                human: self.human,
            }),
            KvData::WideString(value) => visitor.visit_seq(WordsAccess { iter: value.iter() }),
            KvData::BinaryString(value) => visitor.visit_seq(BytesAccess { iter: value.iter() }),
            _ => Err(Error::UnexpectedToken),
        }
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
        match self.data {
            KvData::Compound(entries) => {
                visitor.visit_map(CompoundAccess::new(entries, self.human))
            }
            _ => Err(Error::UnexpectedToken),
        }
    }

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
        match self.data {
            KvData::String(variant) => visitor.visit_enum(KvEnumAccess {
                variant,
                value: None,
                human: self.human,
            }),
            KvData::Compound(entries) if entries.len() == 1 => {
                let entry = &entries[0];
                visitor.visit_enum(KvEnumAccess {
                    variant: &entry.name,
                    value: Some(&entry.data),
                    human: self.human,
                })
            }
            _ => Err(Error::UnexpectedToken),
        }
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_str(visitor)
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_unit()
    }
}

struct CompoundAccess<'a, 'de> {
    iter: slice::Iter<'a, KvEntry<'de>>,
    value: Option<&'a KvData<'de>>,
    human: bool,
}

impl<'a, 'de> CompoundAccess<'a, 'de> {
    fn new(entries: &'a [KvEntry<'de>], human: bool) -> Self {
        Self {
            iter: entries.iter(),
            value: None,
            human,
        }
    }
}

impl<'de> MapAccess<'de> for CompoundAccess<'_, 'de> {
    type Error = Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>>
    where
        K: DeserializeSeed<'de>,
    {
        let Some(entry) = self.iter.next() else {
            return Ok(None);
        };

        self.value = Some(&entry.data);
        seed.deserialize(KeyDeserializer {
            key: &entry.name,
            human: self.human,
        })
        .map(Some)
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value>
    where
        V: DeserializeSeed<'de>,
    {
        let value = self
            .value
            .take()
            .ok_or_else(|| Error::Message("missing map value".to_owned()))?;
        seed.deserialize(ValueDeserializer::new(value, self.human))
    }
}

struct CompoundSeqAccess<'a, 'de> {
    iter: slice::Iter<'a, KvEntry<'de>>,
    human: bool,
}

impl<'de> SeqAccess<'de> for CompoundSeqAccess<'_, 'de> {
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>>
    where
        T: DeserializeSeed<'de>,
    {
        let Some(entry) = self.iter.next() else {
            return Ok(None);
        };

        seed.deserialize(ValueDeserializer::new(&entry.data, self.human))
            .map(Some)
    }
}

struct WordsAccess<'a> {
    iter: slice::Iter<'a, u16>,
}

impl<'de> SeqAccess<'de> for WordsAccess<'_> {
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>>
    where
        T: DeserializeSeed<'de>,
    {
        let Some(word) = self.iter.next() else {
            return Ok(None);
        };

        seed.deserialize((*word).into_deserializer()).map(Some)
    }
}

struct BytesAccess<'a> {
    iter: slice::Iter<'a, u8>,
}

impl<'de> SeqAccess<'de> for BytesAccess<'_> {
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>>
    where
        T: DeserializeSeed<'de>,
    {
        let Some(byte) = self.iter.next() else {
            return Ok(None);
        };

        seed.deserialize((*byte).into_deserializer()).map(Some)
    }
}

struct KeyDeserializer<'a, 'de> {
    key: &'a Cow<'de, str>,
    human: bool,
}

impl<'de> de::Deserializer<'de> for KeyDeserializer<'_, 'de> {
    type Error = Error;

    fn is_human_readable(&self) -> bool {
        self.human
    }

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_str(visitor)
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        match self.key.as_ref() {
            "true" | "1" => visitor.visit_bool(true),
            "false" | "0" | "" => visitor.visit_bool(false),
            value => Err(Error::Message(format!("invalid bool key {value:?}"))),
        }
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i8(parse_key(self.key)?)
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i16(parse_key(self.key)?)
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i32(parse_key(self.key)?)
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i64(parse_key(self.key)?)
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u8(parse_key(self.key)?)
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u16(parse_key(self.key)?)
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u32(parse_key(self.key)?)
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u64(parse_key(self.key)?)
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_f32(parse_key(self.key)?)
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_f64(parse_key(self.key)?)
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let mut chars = self.key.chars();
        let Some(ch) = chars.next() else {
            return Err(Error::Message("expected char key".to_owned()));
        };
        if chars.next().is_some() {
            return Err(Error::Message("expected one-character key".to_owned()));
        }
        visitor.visit_char(ch)
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visit_str(self.key, visitor)
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        match self.key {
            Cow::Borrowed(value) => visitor.visit_borrowed_str(value),
            Cow::Owned(value) => visitor.visit_str(value),
        }
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_bytes(self.key.as_bytes())
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_byte_buf(self.key.as_bytes().to_vec())
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        if self.key.is_empty() {
            visitor.visit_none()
        } else {
            visitor.visit_some(self)
        }
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        if self.key.is_empty() {
            visitor.visit_unit()
        } else {
            Err(Error::UnexpectedToken)
        }
    }

    fn deserialize_unit_struct<V>(self, _name: &'static str, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_unit(visitor)
    }

    fn deserialize_newtype_struct<V>(self, _name: &'static str, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_newtype_struct(self)
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
        visitor.visit_enum(KvEnumAccess {
            variant: self.key,
            value: None,
            human: self.human,
        })
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_str(visitor)
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_unit()
    }

    serde::forward_to_deserialize_any! {
        seq tuple tuple_struct map struct
    }
}

struct KvEnumAccess<'a, 'de> {
    variant: &'a Cow<'de, str>,
    value: Option<&'a KvData<'de>>,
    human: bool,
}

impl<'a, 'de> EnumAccess<'de> for KvEnumAccess<'a, 'de> {
    type Error = Error;
    type Variant = KvVariantAccess<'a, 'de>;

    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant)>
    where
        V: DeserializeSeed<'de>,
    {
        let value = seed.deserialize(KeyDeserializer {
            key: self.variant,
            human: self.human,
        })?;
        Ok((
            value,
            KvVariantAccess {
                value: self.value,
                human: self.human,
            },
        ))
    }
}

struct KvVariantAccess<'a, 'de> {
    value: Option<&'a KvData<'de>>,
    human: bool,
}

impl<'de> VariantAccess<'de> for KvVariantAccess<'_, 'de> {
    type Error = Error;

    fn unit_variant(self) -> Result<()> {
        match self.value {
            None => Ok(()),
            Some(KvData::String(value)) if value.is_empty() => Ok(()),
            Some(KvData::Compound(entries)) if entries.is_empty() => Ok(()),
            _ => Err(Error::UnexpectedToken),
        }
    }

    fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value>
    where
        T: DeserializeSeed<'de>,
    {
        let value = self
            .value
            .ok_or_else(|| Error::Message("missing enum value".to_owned()))?;
        seed.deserialize(ValueDeserializer::new(value, self.human))
    }

    fn tuple_variant<V>(self, len: usize, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let value = self
            .value
            .ok_or_else(|| Error::Message("missing enum value".to_owned()))?;
        ValueDeserializer::new(value, self.human).deserialize_tuple(len, visitor)
    }

    fn struct_variant<V>(self, fields: &'static [&'static str], visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let value = self
            .value
            .ok_or_else(|| Error::Message("missing enum value".to_owned()))?;
        ValueDeserializer::new(value, self.human).deserialize_struct("", fields, visitor)
    }
}

fn visit_str<'de, V>(value: &Cow<'de, str>, visitor: V) -> Result<V::Value>
where
    V: Visitor<'de>,
{
    match value {
        Cow::Borrowed(value) => visitor.visit_borrowed_str(value),
        Cow::Owned(value) => visitor.visit_str(value),
    }
}

fn visit_bytes<'de, V>(value: &Cow<'de, [u8]>, visitor: V) -> Result<V::Value>
where
    V: Visitor<'de>,
{
    match value {
        Cow::Borrowed(value) => visitor.visit_borrowed_bytes(value),
        Cow::Owned(value) => visitor.visit_bytes(value),
    }
}

fn parse_i64(value: &str) -> Result<i64> {
    value
        .parse()
        .map_err(|_| Error::Message(format!("invalid integer value {value:?}")))
}

fn parse_u64(value: &str) -> Result<u64> {
    if let Some(value) = value
        .strip_prefix("0x")
        .or_else(|| value.strip_prefix("0X"))
    {
        return u64::from_str_radix(value, 16)
            .map_err(|_| Error::Message(format!("invalid unsigned integer value {value:?}")));
    }

    value
        .parse()
        .map_err(|_| Error::Message(format!("invalid unsigned integer value {value:?}")))
}

fn parse_key<T>(value: &str) -> Result<T>
where
    T: core::str::FromStr,
{
    value
        .parse()
        .map_err(|_| Error::Message(format!("invalid map key {value:?}")))
}
