//! Xeh Util

/// Convert `char` to `[u8; 4]`
pub fn char2u8(src: &char) -> Option<[u8; 4]> {
    let mut dest = [0, 0, 0, 0];
    let mut raw = src.to_digit(36)?;
    for i in 0..4 {
        dest[3 - i] = (raw & 0xff) as u8;
        raw >>= 8;
    }

    Some(dest)
}
