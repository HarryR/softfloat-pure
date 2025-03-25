use super::uint128;

#[inline]
#[must_use]
pub const fn softfloat_add128(a64: u64, a0: u64, b64: u64, b0: u64) -> uint128 {
    let v0 = a0.wrapping_add(b0);
    uint128 {
        v0,
        v64: a64.wrapping_add(b64).wrapping_add((v0 < a0) as i32 as u64),
    }
}
