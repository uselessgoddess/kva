use alloc::string::String;
#[cfg(feature = "serde")]
use alloc::string::ToString;
use core::fmt;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Error {
    UnexpectedEof,
    UnexpectedToken,
    UnexpectedEnd,
    UnterminatedCString,
    InvalidUtf8,
    InvalidUtf16,
    InvalidType(u8),
    Message(String),
    Unsupported(&'static str),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnexpectedEof => f.write_str("unexpected EOF"),
            Self::UnexpectedToken => f.write_str("unexpected token"),
            Self::UnexpectedEnd => f.write_str("unexpected end marker"),
            Self::UnterminatedCString => f.write_str("cstring not terminated"),
            Self::InvalidUtf8 => f.write_str("invalid UTF-8 string"),
            Self::InvalidUtf16 => f.write_str("invalid UTF-16 string"),
            Self::InvalidType(ty) => write!(f, "invalid binary keyvalues type {ty}"),
            Self::Message(msg) => f.write_str(msg),
            Self::Unsupported(ty) => write!(f, "unsupported serde value {ty}"),
        }
    }
}

impl core::error::Error for Error {}

#[cfg(feature = "serde")]
impl serde::de::Error for Error {
    fn custom<T>(msg: T) -> Self
    where
        T: fmt::Display,
    {
        Self::Message(msg.to_string())
    }
}

#[cfg(feature = "serde")]
impl serde::ser::Error for Error {
    fn custom<T>(msg: T) -> Self
    where
        T: fmt::Display,
    {
        Self::Message(msg.to_string())
    }
}
