use alloc::string::String;

use serde::{Deserialize, Serialize};

use crate::{
    Error, KvEntry, Result, de, ser,
    text::{Options, Parser},
};

pub type Serializer<'a> = ser::Serializer<'a, ser::Text>;

pub fn to_string<T>(name: &str, value: &T) -> Result<String>
where
    T: Serialize + ?Sized,
{
    value.serialize(Serializer::new(name))
}

pub fn from_str<'de, T>(input: &'de str) -> Result<T>
where
    T: Deserialize<'de>,
{
    let parsed = Parser::new(input).parse()?;
    T::deserialize(&mut Deserializer::new(parsed))
}

pub fn from_str_options<'de, T>(input: &'de str, options: Options) -> Result<T>
where
    T: Deserialize<'de>,
{
    let parsed = Parser::new(input).options(options).parse()?;
    T::deserialize(&mut Deserializer::new(parsed))
}

pub struct Deserializer<'de> {
    root: KvEntry<'de>,
}

impl<'de> Deserializer<'de> {
    pub fn new(root: KvEntry<'de>) -> Self {
        Self { root }
    }
}

impl<'de> serde::de::Deserializer<'de> for &mut Deserializer<'de> {
    type Error = Error;

    de::delegate_to_value_deserializer!(true);
}
