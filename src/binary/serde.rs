use alloc::vec::Vec;

use serde::{Deserialize, Serialize};

use crate::{
    Error, KvEntry, Result,
    binary::{Dialect, Parser},
    de, ser,
};

pub type Serializer<'a> = ser::Serializer<'a, ser::Binary>;

pub fn to_vec<T>(name: &str, value: &T) -> Result<Vec<u8>>
where
    T: Serialize + ?Sized,
{
    to_vec_dialect(name, value, Dialect::default())
}

pub fn to_vec_dialect<T>(name: &str, value: &T, dialect: Dialect) -> Result<Vec<u8>>
where
    T: Serialize + ?Sized,
{
    value.serialize(Serializer::with_dialect(name, dialect))
}

pub fn from_slice<'de, T>(input: &'de [u8]) -> Result<T>
where
    T: Deserialize<'de>,
{
    from_slice_dialect(input, Dialect::default())
}

pub fn from_slice_dialect<'de, T>(input: &'de [u8], dialect: Dialect) -> Result<T>
where
    T: Deserialize<'de>,
{
    let mut deserializer = Deserializer::from_slice_dialect(input, dialect)?;
    T::deserialize(&mut deserializer)
}

pub struct Deserializer<'de> {
    root: KvEntry<'de>,
}

impl<'de> Deserializer<'de> {
    pub fn from_slice(input: &'de [u8]) -> Result<Self> {
        Self::from_slice_dialect(input, Dialect::default())
    }

    pub fn from_slice_dialect(input: &'de [u8], dialect: Dialect) -> Result<Self> {
        let root = Parser::new(input)
            .dialect(dialect)
            .parse()?
            .ok_or(Error::UnexpectedEnd)?;
        Ok(Self { root })
    }
}

impl<'de> serde::de::Deserializer<'de> for &mut Deserializer<'de> {
    type Error = Error;

    de::delegate_to_value_deserializer!(false);
}
