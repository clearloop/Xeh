//! The lightest `hex` dependency.
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
pub fn encode<T>(target: &T) -> Option<impl From<SliceVec<char>>>
where
    T: ToHex,
{
    target.hex()?.into()
}
