#[inline]
#[must_use]
pub const fn softfloat_shortShiftRightJam64(a: u64, dist: u8) -> u64 {
    return (a >> dist) | (a & ((1 as u64) << dist).wrapping_sub(1) != 0 as u64) as u64;
}
