use super::uint128;

#[inline]
#[must_use]
pub const fn softfloat_shortShiftLeft128(a64: u64, a0: u64, dist: u8) -> uint128 {
    uint128 {
        v0: a0 << dist,
        v64: (a64 << dist) | (a0 >> (dist.wrapping_neg() & 63)),
    }
}
