mod parser;

pub use parser::Parser;

#[cfg(feature = "serde")]
mod serde;

#[cfg(feature = "serde")]
pub use serde::{Deserializer, Serializer, from_slice, to_vec};
