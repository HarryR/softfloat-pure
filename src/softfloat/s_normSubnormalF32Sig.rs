use super::{exp16_sig32, softfloat_countLeadingZeros32};

#[inline]
#[must_use]
pub const fn softfloat_normSubnormalF32Sig(sig: u32) -> exp16_sig32 {
    let shiftDist = softfloat_countLeadingZeros32(sig).wrapping_sub(8) as i8;
    exp16_sig32 {
        exp: (1 as i16).wrapping_sub(shiftDist as i16),
        sig: sig << shiftDist,
    }
}
