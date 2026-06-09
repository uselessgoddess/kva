use alloc::{
    borrow::{Cow, ToOwned},
    string::{String, ToString},
    vec,
    vec::Vec,
};
use core::marker::PhantomData;

use serde::{Serialize, ser};

use crate::{Error, KvData, KvEntry, Result};

pub struct Text;
pub struct Binary;

pub struct Serializer<'a, F> {
    name: &'a str,
    format: PhantomData<F>,
}

impl<'a, F> Serializer<'a, F> {
    pub fn new(name: &'a str) -> Self {
        Self {
            name,
            format: PhantomData,
        }
    }
}

pub trait Output {
    type Ok;

    fn is_human_readable() -> bool;

    fn finish(name: &str, data: KvData<'static>) -> Result<Self::Ok>;
}

#[cfg(feature = "text")]
impl Output for Text {
    type Ok = String;

    fn is_human_readable() -> bool {
        true
    }

    fn finish(name: &str, data: KvData<'static>) -> Result<Self::Ok> {
        crate::write::text_to_string(name, data)
    }
}

#[cfg(feature = "binary")]
impl Output for Binary {
    type Ok = Vec<u8>;

    fn is_human_readable() -> bool {
        false
    }

    fn finish(name: &str, data: KvData<'static>) -> Result<Self::Ok> {
        crate::write::binary_to_vec(name, data)
    }
}

pub(crate) fn to_data<T, F>(value: &T) -> Result<KvData<'static>>
where
    T: Serialize + ?Sized,
    F: Output,
{
    value.serialize(ValueSerializer::<F>(PhantomData))
}

impl<'a, F> ser::Serializer for Serializer<'a, F>
where
    F: Output,
{
    type Ok = F::Ok;
    type Error = Error;
    type SerializeSeq = RootCompoundSerializer<'a, F>;
    type SerializeTuple = RootCompoundSerializer<'a, F>;
    type SerializeTupleStruct = RootCompoundSerializer<'a, F>;
    type SerializeTupleVariant = RootVariantSerializer<'a, F>;
    type SerializeMap = RootMapSerializer<'a, F>;
    type SerializeStruct = RootMapSerializer<'a, F>;
    type SerializeStructVariant = RootVariantSerializer<'a, F>;

    fn is_human_readable(&self) -> bool {
        F::is_human_readable()
    }

    fn serialize_bool(self, value: bool) -> Result<Self::Ok> {
        F::finish(self.name, bool_data(value))
    }

    fn serialize_i8(self, value: i8) -> Result<Self::Ok> {
        F::finish(self.name, KvData::Int(value.into()))
    }

    fn serialize_i16(self, value: i16) -> Result<Self::Ok> {
        F::finish(self.name, KvData::Int(value.into()))
    }

    fn serialize_i32(self, value: i32) -> Result<Self::Ok> {
        F::finish(self.name, KvData::Int(value))
    }

    fn serialize_i64(self, value: i64) -> Result<Self::Ok> {
        F::finish(self.name, i64_data(value))
    }

    fn serialize_u8(self, value: u8) -> Result<Self::Ok> {
        F::finish(self.name, KvData::Int(value.into()))
    }

    fn serialize_u16(self, value: u16) -> Result<Self::Ok> {
        F::finish(self.name, KvData::Int(value.into()))
    }

    fn serialize_u32(self, value: u32) -> Result<Self::Ok> {
        F::finish(self.name, u64_data(value.into()))
    }

    fn serialize_u64(self, value: u64) -> Result<Self::Ok> {
        F::finish(self.name, KvData::UInt64(value))
    }

    fn serialize_f32(self, value: f32) -> Result<Self::Ok> {
        F::finish(self.name, KvData::Float(value))
    }

    fn serialize_f64(self, value: f64) -> Result<Self::Ok> {
        F::finish(self.name, string_data(value.to_string()))
    }

    fn serialize_char(self, value: char) -> Result<Self::Ok> {
        F::finish(self.name, string_data(value.to_string()))
    }

    fn serialize_str(self, value: &str) -> Result<Self::Ok> {
        F::finish(self.name, string_data(value.to_owned()))
    }

    fn serialize_bytes(self, value: &[u8]) -> Result<Self::Ok> {
        F::finish(self.name, KvData::BinaryString(Cow::Owned(value.to_vec())))
    }

    fn serialize_none(self) -> Result<Self::Ok> {
        F::finish(self.name, string_data(String::new()))
    }

    fn serialize_some<T>(self, value: &T) -> Result<Self::Ok>
    where
        T: Serialize + ?Sized,
    {
        F::finish(self.name, to_data::<_, F>(value)?)
    }

    fn serialize_unit(self) -> Result<Self::Ok> {
        F::finish(self.name, string_data(String::new()))
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok> {
        self.serialize_unit()
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok> {
        F::finish(self.name, string_data(variant.to_owned()))
    }

    fn serialize_newtype_struct<T>(self, _name: &'static str, value: &T) -> Result<Self::Ok>
    where
        T: Serialize + ?Sized,
    {
        self.serialize_some(value)
    }

    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok>
    where
        T: Serialize + ?Sized,
    {
        F::finish(self.name, variant_data(variant, to_data::<_, F>(value)?))
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq> {
        Ok(RootCompoundSerializer::new(self.name))
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple> {
        Ok(RootCompoundSerializer::new(self.name))
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        Ok(RootCompoundSerializer::new(self.name))
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        Ok(RootVariantSerializer::new(self.name, variant))
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
        Ok(RootMapSerializer::new(self.name))
    }

    fn serialize_struct(self, _name: &'static str, _len: usize) -> Result<Self::SerializeStruct> {
        Ok(RootMapSerializer::new(self.name))
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        Ok(RootVariantSerializer::new(self.name, variant))
    }
}

pub struct RootCompoundSerializer<'a, F> {
    name: &'a str,
    inner: CompoundSerializer<F>,
    format: PhantomData<F>,
}

impl<'a, F> RootCompoundSerializer<'a, F> {
    fn new(name: &'a str) -> Self {
        Self {
            name,
            inner: CompoundSerializer::<F>::new(),
            format: PhantomData,
        }
    }
}

impl<F> ser::SerializeSeq for RootCompoundSerializer<'_, F>
where
    F: Output,
{
    type Ok = F::Ok;
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: Serialize + ?Sized,
    {
        self.inner.serialize_element(value)
    }

    fn end(self) -> Result<Self::Ok> {
        F::finish(self.name, self.inner.end()?)
    }
}

impl<F> ser::SerializeTuple for RootCompoundSerializer<'_, F>
where
    F: Output,
{
    type Ok = F::Ok;
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: Serialize + ?Sized,
    {
        self.inner.serialize_element(value)
    }

    fn end(self) -> Result<Self::Ok> {
        F::finish(self.name, self.inner.end()?)
    }
}

impl<F> ser::SerializeTupleStruct for RootCompoundSerializer<'_, F>
where
    F: Output,
{
    type Ok = F::Ok;
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<()>
    where
        T: Serialize + ?Sized,
    {
        self.inner.serialize_element(value)
    }

    fn end(self) -> Result<Self::Ok> {
        F::finish(self.name, self.inner.end()?)
    }
}

pub struct RootMapSerializer<'a, F> {
    name: &'a str,
    inner: MapSerializer<F>,
}

impl<'a, F> RootMapSerializer<'a, F> {
    fn new(name: &'a str) -> Self {
        Self {
            name,
            inner: MapSerializer::new(),
        }
    }
}

impl<F> ser::SerializeMap for RootMapSerializer<'_, F>
where
    F: Output,
{
    type Ok = F::Ok;
    type Error = Error;

    fn serialize_key<T>(&mut self, key: &T) -> Result<()>
    where
        T: Serialize + ?Sized,
    {
        self.inner.serialize_key(key)
    }

    fn serialize_value<T>(&mut self, value: &T) -> Result<()>
    where
        T: Serialize + ?Sized,
    {
        self.inner.serialize_value(value)
    }

    fn end(self) -> Result<Self::Ok> {
        F::finish(self.name, self.inner.end()?)
    }
}

impl<F> ser::SerializeStruct for RootMapSerializer<'_, F>
where
    F: Output,
{
    type Ok = F::Ok;
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
    where
        T: Serialize + ?Sized,
    {
        self.inner.serialize_field(key, value)
    }

    fn end(self) -> Result<Self::Ok> {
        F::finish(self.name, self.inner.end()?)
    }
}

pub struct RootVariantSerializer<'a, F> {
    name: &'a str,
    variant: &'static str,
    inner: CompoundSerializer<F>,
}

impl<'a, F> RootVariantSerializer<'a, F> {
    fn new(name: &'a str, variant: &'static str) -> Self {
        Self {
            name,
            variant,
            inner: CompoundSerializer::new(),
        }
    }
}

impl<F> ser::SerializeTupleVariant for RootVariantSerializer<'_, F>
where
    F: Output,
{
    type Ok = F::Ok;
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<()>
    where
        T: Serialize + ?Sized,
    {
        self.inner.serialize_element(value)
    }

    fn end(self) -> Result<Self::Ok> {
        F::finish(self.name, variant_data(self.variant, self.inner.end()?))
    }
}

impl<F> ser::SerializeStructVariant for RootVariantSerializer<'_, F>
where
    F: Output,
{
    type Ok = F::Ok;
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
    where
        T: Serialize + ?Sized,
    {
        self.inner.push(key.to_owned(), to_data::<_, F>(value)?);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok> {
        F::finish(self.name, variant_data(self.variant, self.inner.end()?))
    }
}

struct ValueSerializer<F>(PhantomData<F>);

impl<F> ser::Serializer for ValueSerializer<F>
where
    F: Output,
{
    type Ok = KvData<'static>;
    type Error = Error;
    type SerializeSeq = CompoundSerializer<F>;
    type SerializeTuple = CompoundSerializer<F>;
    type SerializeTupleStruct = CompoundSerializer<F>;
    type SerializeTupleVariant = VariantSerializer<F>;
    type SerializeMap = MapSerializer<F>;
    type SerializeStruct = MapSerializer<F>;
    type SerializeStructVariant = VariantSerializer<F>;

    fn is_human_readable(&self) -> bool {
        F::is_human_readable()
    }

    fn serialize_bool(self, value: bool) -> Result<Self::Ok> {
        Ok(bool_data(value))
    }

    fn serialize_i8(self, value: i8) -> Result<Self::Ok> {
        Ok(KvData::Int(value.into()))
    }

    fn serialize_i16(self, value: i16) -> Result<Self::Ok> {
        Ok(KvData::Int(value.into()))
    }

    fn serialize_i32(self, value: i32) -> Result<Self::Ok> {
        Ok(KvData::Int(value))
    }

    fn serialize_i64(self, value: i64) -> Result<Self::Ok> {
        Ok(i64_data(value))
    }

    fn serialize_u8(self, value: u8) -> Result<Self::Ok> {
        Ok(KvData::Int(value.into()))
    }

    fn serialize_u16(self, value: u16) -> Result<Self::Ok> {
        Ok(KvData::Int(value.into()))
    }

    fn serialize_u32(self, value: u32) -> Result<Self::Ok> {
        Ok(u64_data(value.into()))
    }

    fn serialize_u64(self, value: u64) -> Result<Self::Ok> {
        Ok(KvData::UInt64(value))
    }

    fn serialize_f32(self, value: f32) -> Result<Self::Ok> {
        Ok(KvData::Float(value))
    }

    fn serialize_f64(self, value: f64) -> Result<Self::Ok> {
        Ok(string_data(value.to_string()))
    }

    fn serialize_char(self, value: char) -> Result<Self::Ok> {
        Ok(string_data(value.to_string()))
    }

    fn serialize_str(self, value: &str) -> Result<Self::Ok> {
        Ok(string_data(value.to_owned()))
    }

    fn serialize_bytes(self, value: &[u8]) -> Result<Self::Ok> {
        Ok(KvData::BinaryString(Cow::Owned(value.to_vec())))
    }

    fn serialize_none(self) -> Result<Self::Ok> {
        Ok(string_data(String::new()))
    }

    fn serialize_some<T>(self, value: &T) -> Result<Self::Ok>
    where
        T: Serialize + ?Sized,
    {
        to_data::<_, F>(value)
    }

    fn serialize_unit(self) -> Result<Self::Ok> {
        Ok(string_data(String::new()))
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok> {
        self.serialize_unit()
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok> {
        Ok(string_data(variant.to_owned()))
    }

    fn serialize_newtype_struct<T>(self, _name: &'static str, value: &T) -> Result<Self::Ok>
    where
        T: Serialize + ?Sized,
    {
        to_data::<_, F>(value)
    }

    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok>
    where
        T: Serialize + ?Sized,
    {
        Ok(variant_data(variant, to_data::<_, F>(value)?))
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq> {
        Ok(CompoundSerializer::new())
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple> {
        Ok(CompoundSerializer::new())
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        Ok(CompoundSerializer::new())
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        Ok(VariantSerializer::new(variant, false))
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
        Ok(MapSerializer::new())
    }

    fn serialize_struct(self, _name: &'static str, _len: usize) -> Result<Self::SerializeStruct> {
        Ok(MapSerializer::new())
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        Ok(VariantSerializer::new(variant, true))
    }
}

pub struct CompoundSerializer<F> {
    entries: Vec<KvEntry<'static>>,
    next_index: usize,
    _marker: PhantomData<F>,
}

impl<F> CompoundSerializer<F> {
    fn new() -> Self {
        Self {
            entries: Vec::new(),
            next_index: 0,
            _marker: PhantomData,
        }
    }

    fn push(&mut self, key: String, data: KvData<'static>) {
        self.entries.push(KvEntry {
            name: Cow::Owned(key),
            data,
        });
    }

    fn end(self) -> Result<KvData<'static>> {
        Ok(KvData::Compound(self.entries))
    }
}

impl<F: Output> CompoundSerializer<F> {
    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: Serialize + ?Sized,
    {
        let key = self.next_index.to_string();
        self.next_index += 1;
        self.push(key, to_data::<_, F>(value)?);
        Ok(())
    }
}

impl<F: Output> ser::SerializeSeq for CompoundSerializer<F> {
    type Ok = KvData<'static>;
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: Serialize + ?Sized,
    {
        self.serialize_element(value)
    }

    fn end(self) -> Result<Self::Ok> {
        self.end()
    }
}

impl<F: Output> ser::SerializeTuple for CompoundSerializer<F> {
    type Ok = KvData<'static>;
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: Serialize + ?Sized,
    {
        self.serialize_element(value)
    }

    fn end(self) -> Result<Self::Ok> {
        self.end()
    }
}

impl<F: Output> ser::SerializeTupleStruct for CompoundSerializer<F> {
    type Ok = KvData<'static>;
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<()>
    where
        T: Serialize + ?Sized,
    {
        self.serialize_element(value)
    }

    fn end(self) -> Result<Self::Ok> {
        self.end()
    }
}

pub struct MapSerializer<F> {
    entries: Vec<KvEntry<'static>>,
    next_key: Option<String>,
    _marker: PhantomData<F>,
}

impl<F> MapSerializer<F> {
    fn new() -> Self {
        Self {
            entries: Vec::new(),
            next_key: None,
            _marker: PhantomData,
        }
    }

    fn push(&mut self, key: String, data: KvData<'static>) {
        self.entries.push(KvEntry {
            name: Cow::Owned(key),
            data,
        });
    }

    fn serialize_key<T>(&mut self, key: &T) -> Result<()>
    where
        T: Serialize + ?Sized,
    {
        if self.next_key.is_some() {
            return Err(Error::Message("serialize_value was not called".to_owned()));
        }

        self.next_key = Some(key.serialize(KeySerializer)?);
        Ok(())
    }

    fn end(self) -> Result<KvData<'static>> {
        if self.next_key.is_some() {
            return Err(Error::Message("serialize_value was not called".to_owned()));
        }

        Ok(KvData::Compound(self.entries))
    }
}

impl<F: Output> MapSerializer<F> {
    fn serialize_value<T>(&mut self, value: &T) -> Result<()>
    where
        T: Serialize + ?Sized,
    {
        let key = self
            .next_key
            .take()
            .ok_or_else(|| Error::Message("serialize_key was not called".to_owned()))?;
        self.push(key, to_data::<_, F>(value)?);
        Ok(())
    }

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
    where
        T: Serialize + ?Sized,
    {
        self.push(key.to_owned(), to_data::<_, F>(value)?);
        Ok(())
    }
}

impl<F: Output> ser::SerializeMap for MapSerializer<F> {
    type Ok = KvData<'static>;
    type Error = Error;

    fn serialize_key<T>(&mut self, key: &T) -> Result<()>
    where
        T: Serialize + ?Sized,
    {
        self.serialize_key(key)
    }

    fn serialize_value<T>(&mut self, value: &T) -> Result<()>
    where
        T: Serialize + ?Sized,
    {
        self.serialize_value(value)
    }

    fn end(self) -> Result<Self::Ok> {
        self.end()
    }
}

impl<F: Output> ser::SerializeStruct for MapSerializer<F> {
    type Ok = KvData<'static>;
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
    where
        T: Serialize + ?Sized,
    {
        self.serialize_field(key, value)
    }

    fn end(self) -> Result<Self::Ok> {
        self.end()
    }
}

pub struct VariantSerializer<F> {
    variant: &'static str,
    inner: CompoundSerializer<F>,
    fields_are_named: bool,
}

impl<F> VariantSerializer<F> {
    fn new(variant: &'static str, fields_are_named: bool) -> Self {
        Self {
            variant,
            inner: CompoundSerializer::new(),
            fields_are_named,
        }
    }
}

impl<F: Output> ser::SerializeTupleVariant for VariantSerializer<F> {
    type Ok = KvData<'static>;
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<()>
    where
        T: Serialize + ?Sized,
    {
        self.inner.serialize_element(value)
    }

    fn end(self) -> Result<Self::Ok> {
        Ok(variant_data(self.variant, self.inner.end()?))
    }
}

impl<F: Output> ser::SerializeStructVariant for VariantSerializer<F> {
    type Ok = KvData<'static>;
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
    where
        T: Serialize + ?Sized,
    {
        if !self.fields_are_named {
            return Err(Error::Unsupported("unnamed struct variant field"));
        }

        self.inner.push(key.to_owned(), to_data::<_, F>(value)?);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok> {
        Ok(variant_data(self.variant, self.inner.end()?))
    }
}

struct KeySerializer;

impl ser::Serializer for KeySerializer {
    type Ok = String;
    type Error = Error;
    type SerializeSeq = ser::Impossible<String, Error>;
    type SerializeTuple = ser::Impossible<String, Error>;
    type SerializeTupleStruct = ser::Impossible<String, Error>;
    type SerializeTupleVariant = ser::Impossible<String, Error>;
    type SerializeMap = ser::Impossible<String, Error>;
    type SerializeStruct = ser::Impossible<String, Error>;
    type SerializeStructVariant = ser::Impossible<String, Error>;

    fn serialize_bool(self, value: bool) -> Result<Self::Ok> {
        Ok(if value { "true" } else { "false" }.to_owned())
    }

    fn serialize_i8(self, value: i8) -> Result<Self::Ok> {
        Ok(value.to_string())
    }

    fn serialize_i16(self, value: i16) -> Result<Self::Ok> {
        Ok(value.to_string())
    }

    fn serialize_i32(self, value: i32) -> Result<Self::Ok> {
        Ok(value.to_string())
    }

    fn serialize_i64(self, value: i64) -> Result<Self::Ok> {
        Ok(value.to_string())
    }

    fn serialize_u8(self, value: u8) -> Result<Self::Ok> {
        Ok(value.to_string())
    }

    fn serialize_u16(self, value: u16) -> Result<Self::Ok> {
        Ok(value.to_string())
    }

    fn serialize_u32(self, value: u32) -> Result<Self::Ok> {
        Ok(value.to_string())
    }

    fn serialize_u64(self, value: u64) -> Result<Self::Ok> {
        Ok(value.to_string())
    }

    fn serialize_f32(self, value: f32) -> Result<Self::Ok> {
        Ok(value.to_string())
    }

    fn serialize_f64(self, value: f64) -> Result<Self::Ok> {
        Ok(value.to_string())
    }

    fn serialize_char(self, value: char) -> Result<Self::Ok> {
        Ok(value.to_string())
    }

    fn serialize_str(self, value: &str) -> Result<Self::Ok> {
        Ok(value.to_owned())
    }

    fn serialize_bytes(self, _value: &[u8]) -> Result<Self::Ok> {
        Err(Error::Unsupported("bytes map key"))
    }

    fn serialize_none(self) -> Result<Self::Ok> {
        Ok(String::new())
    }

    fn serialize_some<T>(self, value: &T) -> Result<Self::Ok>
    where
        T: Serialize + ?Sized,
    {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self::Ok> {
        Ok(String::new())
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok> {
        self.serialize_unit()
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok> {
        Ok(variant.to_owned())
    }

    fn serialize_newtype_struct<T>(self, _name: &'static str, value: &T) -> Result<Self::Ok>
    where
        T: Serialize + ?Sized,
    {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _value: &T,
    ) -> Result<Self::Ok>
    where
        T: Serialize + ?Sized,
    {
        Err(Error::Unsupported("newtype variant map key"))
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq> {
        Err(Error::Unsupported("sequence map key"))
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple> {
        Err(Error::Unsupported("tuple map key"))
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        Err(Error::Unsupported("tuple struct map key"))
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        Err(Error::Unsupported("tuple variant map key"))
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
        Err(Error::Unsupported("map key"))
    }

    fn serialize_struct(self, _name: &'static str, _len: usize) -> Result<Self::SerializeStruct> {
        Err(Error::Unsupported("struct map key"))
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        Err(Error::Unsupported("struct variant map key"))
    }
}

fn bool_data(value: bool) -> KvData<'static> {
    KvData::Int(i32::from(value))
}

fn i64_data(value: i64) -> KvData<'static> {
    match i32::try_from(value) {
        Ok(value) => KvData::Int(value),
        Err(_) => KvData::Int64(value),
    }
}

fn u64_data(value: u64) -> KvData<'static> {
    match i32::try_from(value) {
        Ok(value) => KvData::Int(value),
        Err(_) => KvData::UInt64(value),
    }
}

fn string_data(value: String) -> KvData<'static> {
    KvData::String(Cow::Owned(value))
}

fn variant_data(variant: &'static str, data: KvData<'static>) -> KvData<'static> {
    KvData::Compound(vec![KvEntry {
        name: Cow::Borrowed(variant),
        data,
    }])
}
