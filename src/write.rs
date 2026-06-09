use alloc::borrow::Cow;
#[cfg(feature = "text")]
use alloc::string::String;
#[cfg(feature = "binary")]
use alloc::{borrow::ToOwned, vec::Vec};
#[cfg(feature = "text")]
use core::fmt::Write as _;

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
pub(crate) fn binary_to_vec(name: &str, data: KvData<'_>) -> Result<Vec<u8>> {
    let entry = KvEntry {
        name: Cow::Borrowed(name),
        data,
    };
    let mut out = Vec::new();
    write_binary_entry(&mut out, &entry)?;
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
fn write_binary_entry(out: &mut Vec<u8>, entry: &KvEntry<'_>) -> Result<()> {
    out.push(binary_type(&entry.data)?);
    write_cstring(out, &entry.name)?;
    write_binary_data(out, &entry.data)
}

#[cfg(feature = "binary")]
fn binary_type(data: &KvData<'_>) -> Result<u8> {
    match data {
        KvData::Compound(_) => Ok(0),
        KvData::String(_) => Ok(1),
        KvData::Int(_) => Ok(2),
        KvData::Float(_) => Ok(3),
        KvData::Pointer(_) => Ok(4),
        KvData::WideString(_) => Ok(5),
        KvData::Color(_) => Ok(6),
        KvData::UInt64(_) => Ok(7),
        KvData::BinaryString(_) => Ok(9),
        KvData::Int64(_) => Ok(10),
    }
}

#[cfg(feature = "binary")]
fn write_binary_data(out: &mut Vec<u8>, data: &KvData<'_>) -> Result<()> {
    match data {
        KvData::Compound(entries) => {
            for entry in entries {
                write_binary_entry(out, entry)?;
            }
            out.push(8);
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
