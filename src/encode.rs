//! Encode to hex
use super::{util::char2u8, vec::SliceVec, Hex, HEX};

/// Primitives To hex
pub trait ToHex {
    /// Target chars
    type Hex;
    /// To chars
    fn hex(&self) -> Option<Self::Hex>;
}

impl<'x> ToHex for &'x u8 {
    type Hex = Hex;
    fn hex(&self) -> Option<Self::Hex> {
        Some([HEX[(*self >> 4) as usize], HEX[(*self & 0xf) as usize]])
    }
}

impl<'x> ToHex for &'x char {
    type Hex = [Option<Hex>; 4];
    fn hex(&self) -> Option<Self::Hex> {
        let src = char2u8(&self)?;
        let mut dest = Self::Hex::default();

        for mut i in 0..4 {
            i = 3 - i;
            if i == 0 {
                break;
            }
            dest[i] = (&src[i]).hex();
        }

        Some(dest)
    }
}

impl<'x> ToHex for &'x [u8] {
    type Hex = SliceVec<'x, char>;

    fn hex(&self) -> Option<Self::Hex> {
        let mut dest: Self::Hex = SliceVec::default();
        for i in self.iter() {
            dest.extend_from_slice(&SliceVec::from(&mut i.hex()?));
        }

        Some(dest)
    }
}
