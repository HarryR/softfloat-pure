#[inline]
#[must_use]
pub const fn softfloat_shiftRightJam32(a: u32, dist: u16) -> u32 {
    if dist < 31 {
        (a >> dist) | (a << (dist.wrapping_neg() & 31) != 0) as u32
    } else {
        (a != 0) as u32
    }
}
