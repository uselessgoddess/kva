mod parser;

pub use parser::{Options, Parser};

#[cfg(feature = "serde")]
mod serde;

#[cfg(feature = "serde")]
pub use serde::{Deserializer, Serializer, from_str, from_str_options, to_string};
