use alloc::{borrow::Cow, string::String, vec::Vec};

use crate::{Error, KvData, Result, types::KvEntry};

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum KvBinaryType {
    Compound = 0,
    String = 1,
    Int = 2,
    Float = 3,
    Pointer = 4,
    WideString = 5,
    Color = 6,
    UInt64 = 7,
    End = 8,
    BinaryString = 9,
    Int64 = 10,
}

impl TryFrom<u8> for KvBinaryType {
    type Error = Error;

    fn try_from(value: u8) -> core::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Compound),
            1 => Ok(Self::String),
            2 => Ok(Self::Int),
            3 => Ok(Self::Float),
            4 => Ok(Self::Pointer),
            5 => Ok(Self::WideString),
            6 => Ok(Self::Color),
            7 => Ok(Self::UInt64),
            8 => Ok(Self::End),
            9 => Ok(Self::BinaryString),
            10 => Ok(Self::Int64),
            ty => Err(Error::InvalidType(ty)),
        }
    }
}

pub struct Parser<'a> {
    buf: &'a [u8],
    pos: usize,
}

impl<'a> Parser<'a> {
    pub fn new(buf: &'a [u8]) -> Self {
        Self { buf, pos: 0 }
    }

    pub fn parse(&mut self) -> Result<Option<KvEntry<'a>>> {
        self.parse_entry()
    }

    fn parse_data(&mut self, ty: KvBinaryType) -> Result<KvData<'a>> {
        match ty {
            KvBinaryType::Compound => Ok(KvData::Compound(self.parse_compound()?)),
            KvBinaryType::String => Ok(KvData::String(self.read_cow()?)),
            KvBinaryType::WideString => Ok(KvData::WideString(self.read_wide_string()?)),
            KvBinaryType::BinaryString => Ok(KvData::BinaryString(self.read_binary_string()?)),
            KvBinaryType::Float => Ok(KvData::Float(self.read_f32_le()?)),
            KvBinaryType::Color => Ok(KvData::Color(self.read_u32_le()?)),
            KvBinaryType::Int => Ok(KvData::Int(self.read_i32_le()?)),
            KvBinaryType::UInt64 => Ok(KvData::UInt64(self.read_u64_le()?)),
            KvBinaryType::Int64 => Ok(KvData::Int64(self.read_i64_le()?)),
            KvBinaryType::Pointer => Ok(KvData::Pointer(self.read_u32_le()? as usize)),
            KvBinaryType::End => Err(Error::UnexpectedEnd),
        }
    }

    fn parse_compound(&mut self) -> Result<Vec<KvEntry<'a>>> {
        let mut entries = Vec::new();

        while let Some(entry) = self.parse_entry()? {
            entries.push(entry);
        }

        Ok(entries)
    }

    fn parse_entry(&mut self) -> Result<Option<KvEntry<'a>>> {
        let ty = KvBinaryType::try_from(self.read_u8()?)?;
        if matches!(ty, KvBinaryType::End) {
            return Ok(None);
        }

        let name = self.read_cow()?;
        let data = self.parse_data(ty)?;
        Ok(Some(KvEntry { name, data }))
    }

    fn take(&mut self, len: usize) -> Result<&'a [u8]> {
        let end = self.pos.checked_add(len).ok_or(Error::UnexpectedEof)?;
        let slice = self.buf.get(self.pos..end).ok_or(Error::UnexpectedEof)?;
        self.pos = end;
        Ok(slice)
    }

    fn read_u8(&mut self) -> Result<u8> {
        Ok(self.take(1)?[0])
    }

    fn read_i32_le(&mut self) -> Result<i32> {
        let bytes = self.take(4)?.try_into().expect("slice length checked");
        Ok(i32::from_le_bytes(bytes))
    }

    fn read_u16_le(&mut self) -> Result<u16> {
        let bytes = self.take(2)?.try_into().expect("slice length checked");
        Ok(u16::from_le_bytes(bytes))
    }

    fn read_u32_le(&mut self) -> Result<u32> {
        let bytes = self.take(4)?.try_into().expect("slice length checked");
        Ok(u32::from_le_bytes(bytes))
    }

    fn read_u64_le(&mut self) -> Result<u64> {
        let bytes = self.take(8)?.try_into().expect("slice length checked");
        Ok(u64::from_le_bytes(bytes))
    }

    fn read_i64_le(&mut self) -> Result<i64> {
        let bytes = self.take(8)?.try_into().expect("slice length checked");
        Ok(i64::from_le_bytes(bytes))
    }

    fn read_f32_le(&mut self) -> Result<f32> {
        Ok(f32::from_bits(self.read_u32_le()?))
    }

    fn read_str(&mut self) -> Result<&'a [u8]> {
        let rest = &self.buf[self.pos..];
        let end = rest
            .iter()
            .position(|&byte| byte == 0)
            .ok_or(Error::UnterminatedCString)?;
        let value = &rest[..end];
        self.pos += end + 1;
        Ok(value)
    }

    fn read_cow(&mut self) -> Result<Cow<'a, str>> {
        let slice = self.read_str()?;
        match core::str::from_utf8(slice) {
            Ok(value) => Ok(Cow::Borrowed(value)),
            Err(_) => Ok(Cow::Owned(String::from_utf8_lossy(slice).into_owned())),
        }
    }

    fn read_binary_string(&mut self) -> Result<Cow<'a, [u8]>> {
        let len = self.read_u32_le()? as usize;
        Ok(Cow::Borrowed(self.take(len)?))
    }

    fn read_wide_string(&mut self) -> Result<Cow<'a, [u16]>> {
        let len = self.read_u16_le()? as usize;
        if len == 0 {
            return Ok(Cow::Borrowed(&[]));
        }

        let bytes = self.take(len.checked_mul(2).ok_or(Error::UnexpectedEof)?)?;
        let words = bytes
            .chunks_exact(2)
            .map(|chunk| u16::from_le_bytes([chunk[0], chunk[1]]))
            .collect();
        Ok(Cow::Owned(words))
    }
}
