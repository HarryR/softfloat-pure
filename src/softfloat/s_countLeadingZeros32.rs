#[inline]
#[must_use]
pub const fn softfloat_countLeadingZeros32(a: u32) -> u8 {
    (if a != 0 { a.leading_zeros() } else { 32 }) as u8
}
