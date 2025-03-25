use super::{exp16_sig64, softfloat_countLeadingZeros64};

#[inline]
#[must_use]
pub const fn softfloat_normSubnormalF64Sig(sig: u64) -> exp16_sig64 {
    let shiftDist = softfloat_countLeadingZeros64(sig).wrapping_sub(11) as i8;
    exp16_sig64 {
        exp: (1 as i16).wrapping_sub(shiftDist as i16),
        sig: sig << shiftDist,
    }
}
