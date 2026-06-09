use alloc::{borrow::Cow, vec::Vec};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub enum KvData<'a> {
    String(#[cfg_attr(feature = "serde", serde(borrow))] Cow<'a, str>),
    WideString(#[cfg_attr(feature = "serde", serde(borrow))] Cow<'a, [u16]>),
    BinaryString(#[cfg_attr(feature = "serde", serde(borrow))] Cow<'a, [u8]>),
    Int(i32),
    Int64(i64),
    UInt64(u64),
    Float(f32),
    Pointer(usize),
    Color(u32),
    Compound(Vec<KvEntry<'a>>),
}

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct KvEntry<'a> {
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub name: Cow<'a, str>,
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub data: KvData<'a>,
}

impl<'a> KvEntry<'a> {
    pub fn into_owned(self) -> KvEntry<'static> {
        KvEntry {
            name: Cow::Owned(self.name.into_owned()),
            data: self.data.into_owned(),
        }
    }
}

impl<'a> KvData<'a> {
    pub fn into_owned(self) -> KvData<'static> {
        match self {
            KvData::String(s) => KvData::String(Cow::Owned(s.into_owned())),
            KvData::WideString(s) => KvData::WideString(Cow::Owned(s.into_owned())),
            KvData::BinaryString(b) => KvData::BinaryString(Cow::Owned(b.into_owned())),
            KvData::Compound(v) => {
                KvData::Compound(v.into_iter().map(KvEntry::into_owned).collect())
            }
            KvData::Int(v) => KvData::Int(v),
            KvData::Int64(v) => KvData::Int64(v),
            KvData::UInt64(v) => KvData::UInt64(v),
            KvData::Float(v) => KvData::Float(v),
            KvData::Pointer(v) => KvData::Pointer(v),
            KvData::Color(v) => KvData::Color(v),
        }
    }

    pub fn is_string(&self) -> bool {
        matches!(self, Self::String(_))
    }
    pub fn is_wide_string(&self) -> bool {
        matches!(self, Self::WideString(_))
    }
    pub fn is_binary(&self) -> bool {
        matches!(self, Self::BinaryString(_))
    }
    pub fn is_int(&self) -> bool {
        matches!(self, Self::Int(_))
    }
    pub fn is_int64(&self) -> bool {
        matches!(self, Self::Int64(_))
    }
    pub fn is_uint64(&self) -> bool {
        matches!(self, Self::UInt64(_))
    }
    pub fn is_float(&self) -> bool {
        matches!(self, Self::Float(_))
    }
    pub fn is_pointer(&self) -> bool {
        matches!(self, Self::Pointer(_))
    }
    pub fn is_color(&self) -> bool {
        matches!(self, Self::Color(_))
    }
    pub fn is_compound(&self) -> bool {
        matches!(self, Self::Compound(_))
    }

    pub fn as_str(&self) -> Option<&str> {
        if let Self::String(v) = self {
            Some(v)
        } else {
            None
        }
    }
    pub fn as_wide_string(&self) -> Option<&[u16]> {
        if let Self::WideString(v) = self {
            Some(v)
        } else {
            None
        }
    }
    pub fn as_binary(&self) -> Option<&[u8]> {
        if let Self::BinaryString(v) = self {
            Some(v)
        } else {
            None
        }
    }
    pub fn as_int(&self) -> Option<i32> {
        if let Self::Int(v) = self {
            Some(*v)
        } else {
            None
        }
    }
    pub fn as_int64(&self) -> Option<i64> {
        if let Self::Int64(v) = self {
            Some(*v)
        } else {
            None
        }
    }
    pub fn as_uint64(&self) -> Option<u64> {
        if let Self::UInt64(v) = self {
            Some(*v)
        } else {
            None
        }
    }
    pub fn as_float(&self) -> Option<f32> {
        if let Self::Float(v) = self {
            Some(*v)
        } else {
            None
        }
    }
    pub fn as_pointer(&self) -> Option<usize> {
        if let Self::Pointer(v) = self {
            Some(*v)
        } else {
            None
        }
    }
    pub fn as_color(&self) -> Option<u32> {
        if let Self::Color(v) = self {
            Some(*v)
        } else {
            None
        }
    }
    pub fn as_compound(&self) -> Option<&[KvEntry<'a>]> {
        if let Self::Compound(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn to_int(&self) -> Option<i32> {
        match self {
            Self::Int(v) => Some(*v),
            Self::Int64(v) => Some(*v as i32),
            Self::UInt64(v) => Some(*v as i32),
            Self::Float(v) => Some(*v as i32),
            Self::String(s) => s.parse().ok(),
            _ => None,
        }
    }

    pub fn to_float(&self) -> Option<f32> {
        match self {
            Self::Float(v) => Some(*v),
            Self::Int(v) => Some(*v as f32),
            Self::Int64(v) => Some(*v as f32),
            Self::UInt64(v) => Some(*v as f32),
            Self::String(s) => s.parse().ok(),
            _ => None,
        }
    }

    pub fn to_uint64(&self) -> Option<u64> {
        match self {
            Self::UInt64(v) => Some(*v),
            Self::Int(v) => Some(*v as u64),
            Self::Int64(v) => Some(*v as u64),
            Self::Float(v) => Some(*v as u64),
            Self::String(s) => s.parse().ok(),
            _ => None,
        }
    }
}

impl<'a> KvEntry<'a> {
    fn children(&self) -> &[KvEntry<'a>] {
        self.data.as_compound().unwrap_or(&[])
    }

    pub fn iter(&self) -> impl Iterator<Item = &KvEntry<'a>> {
        self.children().iter()
    }

    pub fn iter_compounds(&self) -> impl Iterator<Item = &KvEntry<'a>> {
        self.iter().filter(|e| e.data.is_compound())
    }

    pub fn iter_values(&self) -> impl Iterator<Item = &KvEntry<'a>> {
        self.iter().filter(|e| !e.data.is_compound())
    }

    pub fn find_all_keys<'s>(&'s self, name: &str) -> impl Iterator<Item = &'s KvEntry<'a>> {
        self.iter().filter(move |e| e.name == name)
    }

    pub fn iter_all_in<'s>(&'s self, name: &str) -> impl Iterator<Item = &'s KvEntry<'a>> {
        self.find_all_keys(name)
            .filter_map(|e| e.data.as_compound())
            .flatten()
    }

    pub fn get(&self, name: &str) -> Option<&KvEntry<'a>> {
        self.iter().find(|e| e.name == name)
    }

    pub fn get_str(&self, name: &str) -> Option<&str> {
        self.get(name)?.data.as_str()
    }

    pub fn get_int(&self, name: &str) -> Option<i32> {
        self.get(name)?.data.to_int()
    }

    pub fn get_float(&self, name: &str) -> Option<f32> {
        self.get(name)?.data.to_float()
    }

    pub fn get_uint64(&self, name: &str) -> Option<u64> {
        self.get(name)?.data.to_uint64()
    }

    pub fn get_all(&self, name: &'a str) -> KvEntry<'a> {
        let all = self.iter_all_in(name).cloned().collect();
        KvEntry {
            name: Cow::Borrowed(name),
            data: KvData::Compound(all),
        }
    }

    pub fn get_path(&self, path: &str) -> Option<&KvEntry<'a>> {
        let mut current = self;
        for segment in path.split('/') {
            current = current.get(segment)?;
        }
        Some(current)
    }

    pub fn is_empty(&self) -> bool {
        self.children().is_empty()
    }
}
