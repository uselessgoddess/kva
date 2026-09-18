mod parser;

pub use parser::{Dialect, Parser};

#[cfg(feature = "serde")]
mod serde;

#[cfg(feature = "serde")]
pub use serde::{Deserializer, Serializer, from_slice, from_slice_dialect, to_vec};
