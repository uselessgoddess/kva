mod parser;

pub use parser::Parser;

pub use crate::Dialect;

#[cfg(feature = "serde")]
mod serde;

#[cfg(feature = "serde")]
pub use serde::{Deserializer, Serializer, from_slice, from_slice_dialect, to_vec, to_vec_dialect};
