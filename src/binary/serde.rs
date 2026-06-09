use alloc::vec::Vec;

use serde::{Deserialize, Serialize};

use crate::{Error, KvEntry, Result, binary::Parser, de, ser};

pub type Serializer<'a> = ser::Serializer<'a, ser::Binary>;

pub fn to_vec<T>(name: &str, value: &T) -> Result<Vec<u8>>
where
    T: Serialize + ?Sized,
{
    value.serialize(Serializer::new(name))
}

pub fn from_slice<'de, T>(input: &'de [u8]) -> Result<T>
where
    T: Deserialize<'de>,
{
    let mut deserializer = Deserializer::from_slice(input)?;
    T::deserialize(&mut deserializer)
}

pub struct Deserializer<'de> {
    root: KvEntry<'de>,
}

impl<'de> Deserializer<'de> {
    pub fn from_slice(input: &'de [u8]) -> Result<Self> {
        let mut parser = Parser::new(input);
        let root = parser.parse()?.ok_or(Error::UnexpectedEnd)?;
        Ok(Self { root })
    }
}

impl<'de> serde::de::Deserializer<'de> for &mut Deserializer<'de> {
    type Error = Error;

    de::delegate_to_value_deserializer!(false);
}
