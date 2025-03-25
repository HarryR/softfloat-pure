use super::uint128;

#[inline]
#[must_use]
pub const fn softfloat_sub128(a64: u64, a0: u64, b64: u64, b0: u64) -> uint128 {
    uint128 {
        v0: a0.wrapping_sub(b0),
        v64: a64.wrapping_sub(b64).wrapping_sub((a0 < b0) as u64),
    }
}
