use super::uint128;

#[inline]
#[must_use]
pub const fn softfloat_shiftRightJam128(a64: u64, a0: u64, dist: u32) -> uint128 {
    if dist < 64 {
        let u8NegDist = dist.wrapping_neg() as u8;
        uint128 {
            v64: a64 >> dist,
            v0: a64 << (u8NegDist & 63) | a0 >> dist | ((a0 << (u8NegDist & 63)) != 0) as u64,
        }
    } else {
        uint128 {
            v64: 0,
            v0: if dist < 127 {
                (a64 >> (dist & 63))
                    | (((a64 & (((1 as u64) << (dist & 63)) - 1)) | a0) != 0) as u64
            } else {
                ((a64 | a0) != 0) as u64
            },
        }
    }
}
