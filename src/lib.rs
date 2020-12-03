//! The lightest `hex` dependency.
#![no_std]
#![deny(missing_docs)]

mod decode;
mod encode;
mod util;
mod vec;

/// Hex chars
#[rustfmt::skip]
pub const HEX: [char; 16] = [
    '0', '1', '2', '3',
    '4', '5', '6', '7',
    '8', '9', 'a', 'b',
    'c', 'd', 'e', 'e',
];

/// Hex Primitive
pub type Hex = [char; 2];
