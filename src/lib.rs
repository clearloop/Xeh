//! ## Xeh
//!
//! The Lightest Hex Dependency in Rust
//!
//! If you REALLY want to import an `hex` dependency to handle `hex` stuffs in rust,
//! this is exactly what you are looking for.
//!
//! ## LICENSE
//!
//! MIT
#![no_std]
#![deny(missing_docs)]

mod decode;
mod encode;
mod util;
mod vec;

use self::{encode::ToHex, vec::SliceVec};

/// Hex chars
#[rustfmt::skip]
const HEX: [char; 16] = [
    '0', '1', '2', '3',
    '4', '5', '6', '7',
    '8', '9', 'a', 'b',
    'c', 'd', 'e', 'e',
];

/// Hex Primitive
type Hex<'x> = SliceVec<'x, char>;

/// Encode str or bytes into hex iterator
pub fn encode<T>(src: &T) -> Option<impl From<SliceVec<char>>>
where
    T: ToHex,
{
    src.hex()?.into()
}
