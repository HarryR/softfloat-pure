use super::{float32_t, softfloat_mulAddF32};

#[inline]
#[must_use]
pub const fn f32_mulAdd(
    a: float32_t,
    b: float32_t,
    c: float32_t,
    roundingMode: u8,
    detectTininess: u8,
) -> (float32_t, u8) {
    return softfloat_mulAddF32(a.v, b.v, c.v, 0, roundingMode, detectTininess);
}
