use alloc::{borrow::Cow, string::String, vec::Vec};

use crate::{Dialect, Error, KvData, Result, types::KvEntry};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Tag {
    Compound,
    String,
    Int,
    Float,
    Pointer,
    WideString,
    Color,
    UInt64,
    BinaryString,
    Int64,
    Byte,
    Zero,
    One,
    End,
}

impl Dialect {
    fn tag(self, byte: u8) -> Result<Tag> {
        Ok(match (self, byte) {
            (_, 0) => Tag::Compound,
            (_, 1) => Tag::String,
            (_, 2) => Tag::Int,
            (_, 3) => Tag::Float,
            (_, 4) => Tag::Pointer,
            (_, 5) => Tag::WideString,
            (_, 6) => Tag::Color,
            (_, 7) => Tag::UInt64,
            (Self::Vdf, 8) => Tag::End,
            (Self::Vdf, 9) => Tag::BinaryString,
            (Self::Vdf, 10) => Tag::Int64,
            (Self::Source, 8) => Tag::Byte,
            (Self::Source, 9) => Tag::Zero,
            (Self::Source, 10) => Tag::One,
            (Self::Source, 11) => Tag::End,
            _ => return Err(Error::InvalidType(byte)),
        })
    }
}

pub struct Parser<'a> {
    buf: &'a [u8],
    pos: usize,
    dialect: Dialect,
}

impl<'a> Parser<'a> {
    pub fn new(buf: &'a [u8]) -> Self {
        Self {
            buf,
            pos: 0,
            dialect: Dialect::Vdf,
        }
    }

    pub fn source(buf: &'a [u8]) -> Self {
        Self::new(buf).dialect(Dialect::Source)
    }

    pub fn dialect(mut self, dialect: Dialect) -> Self {
        self.dialect = dialect;
        self
    }

    pub fn parse(&mut self) -> Result<Option<KvEntry<'a>>> {
        self.parse_entry()
    }

    fn parse_data(&mut self, tag: Tag) -> Result<KvData<'a>> {
        match tag {
            Tag::Compound => Ok(KvData::Compound(self.parse_compound()?)),
            Tag::String => Ok(KvData::String(self.read_cow()?)),
            Tag::WideString => Ok(KvData::WideString(self.read_wide_string()?)),
            Tag::BinaryString => Ok(KvData::BinaryString(self.read_binary_string()?)),
            Tag::Float => Ok(KvData::Float(self.read_f32_le()?)),
            Tag::Color => Ok(KvData::Color(self.read_u32_le()?)),
            Tag::Int => Ok(KvData::Int(self.read_i32_le()?)),
            Tag::UInt64 => Ok(KvData::UInt64(self.read_u64_le()?)),
            Tag::Int64 => Ok(KvData::Int64(self.read_i64_le()?)),
            Tag::Pointer => Ok(KvData::Pointer(self.read_u32_le()? as usize)),
            // 8/9/10 encode an int, they are not types — valve's reader retags them TYPE_INT too
            Tag::Byte => Ok(KvData::Int(self.read_u8()?.into())),
            Tag::Zero => Ok(KvData::Int(0)),
            Tag::One => Ok(KvData::Int(1)),
            Tag::End => Err(Error::UnexpectedEnd),
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
        let tag = self.dialect.tag(self.read_u8()?)?;
        if matches!(tag, Tag::End) {
            return Ok(None);
        }

        let name = self.read_cow()?;
        let data = self.parse_data(tag)?;
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
        let (words, _) = bytes.as_chunks::<2>();
        let words = words.iter().copied().map(u16::from_le_bytes).collect();
        Ok(Cow::Owned(words))
    }
}
