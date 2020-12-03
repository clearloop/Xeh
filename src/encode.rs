//! Encode to hex
/// To hex
pub trait ToHex {
    /// Target chars
    type Hex;
    /// To chars
    fn hex(&self) -> Option<Self::Hex>;
}

impl ToHex for &u8 {
    type Hex = Hex;
    fn hex(&self) -> Option<Self::Hex> {
        Some([HEX[(*self >> 4) as usize], HEX[(*self & 0xf) as usize]])
    }
}

impl ToHex for &char {
    type Hex = [Option<Hex>; 4];
    fn hex(&self) -> Option<Self::Hex> {
        let mut res = Self::Hex::default();
        let mut raw = self.to_digit(36)?;

        for mut i in 0..8 {
            i = 7 - i;
            if i % 2 == 0 {
                if let Some(inner) = res[i / 2].as_mut() {
                    inner[0] = HEX[(raw & 0xf) as usize];
                }
            } else {
                if raw == 0 {
                    break;
                }
                res[i / 2] = Some([HEX[0], HEX[(raw & 0xf) as usize]]);
            }
            raw >>= 4;
        }

        Some(res)
    }
}
