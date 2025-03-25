use super::*;

#[inline]
#[must_use]
pub const fn softfloat_approxRecip32_1(a: u32) -> u32 {
    (0x7FFF_FFFF_FFFF_FFFF as u64).wrapping_div(a as u64) as u32
}
