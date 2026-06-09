#![doc = include_str!("../README.md")]
#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

mod error;
mod types;

pub use error::Error;
pub use types::{KvData, KvEntry};

#[cfg(any(feature = "text", feature = "binary", feature = "serde"))]
pub(crate) type Result<T> = core::result::Result<T, Error>;

#[cfg(feature = "binary")]
pub mod binary;

#[cfg(feature = "text")]
pub mod text;

#[cfg(all(feature = "serde", any(feature = "text", feature = "binary")))]
mod write;

#[cfg(any(
    all(feature = "serde", feature = "text"),
    all(feature = "serde", feature = "binary")
))]
mod de;

#[cfg(feature = "serde")]
pub mod ser;
