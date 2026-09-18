use alloc::borrow::Cow;
#[cfg(feature = "text")]
use alloc::string::String;
#[cfg(feature = "binary")]
use alloc::{borrow::ToOwned, vec::Vec};
#[cfg(feature = "text")]
use core::fmt::Write as _;

#[cfg(feature = "binary")]
use crate::Dialect;
use crate::{Error, KvData, KvEntry, Result};

#[cfg(feature = "text")]
pub(crate) fn text_to_string(name: &str, data: KvData<'_>) -> Result<String> {
    let entry = KvEntry {
        name: Cow::Borrowed(name),
        data,
    };
    let mut out = String::new();
    write_text_entry(&mut out, &entry, 0)?;
    Ok(out)
}

#[cfg(feature = "binary")]
pub(crate) fn binary_to_vec(name: &str, data: KvData<'_>, dialect: Dialect) -> Result<Vec<u8>> {
    let entry = KvEntry {
        name: Cow::Borrowed(name),
        data,
    };
    let mut out = Vec::new();
    write_binary_entry(&mut out, &entry, dialect)?;
    Ok(out)
}

#[cfg(feature = "text")]
fn write_text_entry(out: &mut String, entry: &KvEntry<'_>, depth: usize) -> Result<()> {
    write_indent(out, depth);
    write_quoted(out, &entry.name);

    match &entry.data {
        KvData::Compound(entries) => {
            out.push('\n');
            write_indent(out, depth);
            out.push_str("{\n");

            for entry in entries {
                write_text_entry(out, entry, depth + 1)?;
            }

            write_indent(out, depth);
            out.push_str("}\n");
        }
        data => {
            out.push('\t');
            write_text_value(out, data)?;
            out.push('\n');
        }
    }

    Ok(())
}

#[cfg(feature = "text")]
fn write_text_value(out: &mut String, data: &KvData<'_>) -> Result<()> {
    match data {
        KvData::String(value) => write_quoted(out, value),
        KvData::WideString(value) => {
            let value = String::from_utf16(value).map_err(|_| Error::InvalidUtf16)?;
            write_quoted(out, &value);
        }
        KvData::BinaryString(_) => return Err(Error::Unsupported("binary string in text output")),
        KvData::Int(value) => write!(out, "\"{value}\"").expect("writing to String cannot fail"),
        KvData::Int64(value) => write!(out, "\"{value}\"").expect("writing to String cannot fail"),
        KvData::UInt64(value) => {
            write!(out, "\"0x{value:016X}\"").expect("writing to String cannot fail")
        }
        KvData::Float(value) => write!(out, "\"{value}\"").expect("writing to String cannot fail"),
        KvData::Pointer(value) => {
            write!(out, "\"{value}\"").expect("writing to String cannot fail")
        }
        KvData::Color(value) => write!(out, "\"{value}\"").expect("writing to String cannot fail"),
        KvData::Compound(_) => return Err(Error::UnexpectedToken),
    }

    Ok(())
}

#[cfg(feature = "text")]
fn write_indent(out: &mut String, depth: usize) {
    for _ in 0..depth {
        out.push('\t');
    }
}

#[cfg(feature = "text")]
fn write_quoted(out: &mut String, value: &str) {
    out.push('"');
    for ch in value.chars() {
        match ch {
            '"' => out.push_str("\\\""),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            '\\' => out.push_str("\\\\"),
            ch => out.push(ch),
        }
    }
    out.push('"');
}

#[cfg(feature = "binary")]
fn write_binary_entry(out: &mut Vec<u8>, entry: &KvEntry<'_>, dialect: Dialect) -> Result<()> {
    if let (KvData::Int(value), Dialect::Source) = (&entry.data, dialect) {
        return write_source_int(out, &entry.name, *value);
    }

    out.push(binary_tag(&entry.data, dialect)?);
    write_cstring(out, &entry.name)?;
    write_binary_data(out, &entry.data, dialect)
}

// source folds a small int into the tag, keeping only the low bytes that still matter
#[cfg(feature = "binary")]
fn write_source_int(out: &mut Vec<u8>, name: &str, value: i32) -> Result<()> {
    let bytes = value.to_le_bytes();
    let (tag, payload) = match value {
        0 => (9, &bytes[..0]),
        1 => (10, &bytes[..0]),
        2..=255 => (8, &bytes[..1]),
        _ => (2, &bytes[..]),
    };

    out.push(tag);
    write_cstring(out, name)?;
    out.extend_from_slice(payload);
    Ok(())
}

#[cfg(feature = "binary")]
fn binary_tag(data: &KvData<'_>, dialect: Dialect) -> Result<u8> {
    Ok(match data {
        KvData::Compound(_) => 0,
        KvData::String(_) => 1,
        KvData::Int(_) => 2,
        KvData::Float(_) => 3,
        KvData::Pointer(_) => 4,
        KvData::WideString(_) => 5,
        KvData::Color(_) => 6,
        KvData::UInt64(_) => 7,
        KvData::BinaryString(_) if dialect == Dialect::Vdf => 9,
        KvData::Int64(_) if dialect == Dialect::Vdf => 10,
        KvData::BinaryString(_) => return Err(Error::Unsupported("binary string in source kv")),
        KvData::Int64(_) => return Err(Error::Unsupported("int64 in source kv")),
    })
}

#[cfg(feature = "binary")]
fn write_binary_data(out: &mut Vec<u8>, data: &KvData<'_>, dialect: Dialect) -> Result<()> {
    match data {
        KvData::Compound(entries) => {
            for entry in entries {
                write_binary_entry(out, entry, dialect)?;
            }
            out.push(match dialect {
                Dialect::Vdf => 8,
                Dialect::Source => 11,
            });
        }
        KvData::String(value) => write_cstring(out, value)?,
        KvData::WideString(value) => {
            let len = u16::try_from(value.len())
                .map_err(|_| Error::Message("wide string is too long".to_owned()))?;
            out.extend_from_slice(&len.to_le_bytes());
            for word in value.as_ref() {
                out.extend_from_slice(&word.to_le_bytes());
            }
        }
        KvData::BinaryString(value) => {
            let len = u32::try_from(value.len())
                .map_err(|_| Error::Message("binary string is too long".to_owned()))?;
            out.extend_from_slice(&len.to_le_bytes());
            out.extend_from_slice(value);
        }
        KvData::Int(value) => out.extend_from_slice(&value.to_le_bytes()),
        KvData::Int64(value) => out.extend_from_slice(&value.to_le_bytes()),
        KvData::UInt64(value) => out.extend_from_slice(&value.to_le_bytes()),
        KvData::Float(value) => out.extend_from_slice(&value.to_le_bytes()),
        KvData::Pointer(value) => {
            let value = u32::try_from(*value)
                .map_err(|_| Error::Message("pointer value does not fit in u32".to_owned()))?;
            out.extend_from_slice(&value.to_le_bytes());
        }
        KvData::Color(value) => out.extend_from_slice(&value.to_le_bytes()),
    }

    Ok(())
}

#[cfg(feature = "binary")]
fn write_cstring(out: &mut Vec<u8>, value: &str) -> Result<()> {
    if value.as_bytes().contains(&0) {
        return Err(Error::Message(
            "strings cannot contain NUL bytes".to_owned(),
        ));
    }

    out.extend_from_slice(value.as_bytes());
    out.push(0);
    Ok(())
}
