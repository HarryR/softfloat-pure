#[inline]
#[must_use]
pub const fn softfloat_countLeadingZeros64(mut a: u64) -> u8 {
    (if a != 0 { a.leading_zeros() } else { 64 }) as u8
}
