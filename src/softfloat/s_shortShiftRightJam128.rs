use super::uint128;

#[inline]
#[must_use]
pub const fn softfloat_shortShiftRightJam128(a64: u64, a0: u64, dist: u8) -> uint128 {
    let uNegDist = dist.wrapping_neg() as u8;
    let v64 = a64 >> dist;
    let v0 = a64 << (uNegDist & 63)
        | a0 >> dist as i32
        | (a0 << (uNegDist as i32 & 63 as i32) != 0 as i32 as u64) as i32 as u64;
    uint128 { v0, v64 }
}
