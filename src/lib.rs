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
type Hex = [char; 2];

/// Encode str or bytes into hex iterator
pub fn encode<'x, T>(target: &T) -> Option<impl From<SliceVec<'x, char>>>
where
    T: ToHex,
    <T as encode::ToHex>::Hex: core::convert::From<SliceVec<'x, char>>,
{
    target.hex()?.into()
}
