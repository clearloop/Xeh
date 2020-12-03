//! Encode to hex
use super::{util::char2u8, vec::SliceVec, Hex, HEX};

/// Primitives To hex
pub trait ToHex {
    /// To chars
    fn hex(&self) -> Option<Hex>;
}

impl<'x> ToHex for &u8 {
    fn hex(&self) -> Option<Hex> {
        let mut dest = Hex::default();
        dest.extend_from_slice(&SliceVec::from(&mut [
            HEX[(*self >> 4) as usize],
            HEX[(*self & 0xf) as usize],
        ]));

        Some(dest)
    }
}

impl<'x> ToHex for &'x char {
    fn hex(&self) -> Option<Hex> {
        let src = char2u8(&self)?;
        let mut dest = Hex::default();

        for (p, q) in src.iter().enumerate() {
            if src[3 - p] == 0 {
                break;
            }

            dest.extend_from_slice(&(&q).hex()?);
        }

        Some(dest)
    }
}

impl<'x> ToHex for &'x [u8] {
    fn hex(&self) -> Option<Hex> {
        let mut dest = SliceVec::default();
        for i in self.iter() {
            dest.extend_from_slice(&i.hex()?);
        }

        Some(dest)
    }
}

impl<'x> ToHex for &'x [char] {
    fn hex(&self) -> Option<Hex> {
        let mut dest: Hex = SliceVec::default();
        for i in self.iter() {
            dest.extend_from_slice(&[*i]);
        }

        Some(dest)
    }
}
