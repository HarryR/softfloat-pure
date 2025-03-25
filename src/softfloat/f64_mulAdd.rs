use super::{float64_t, softfloat_mulAddF64};

#[inline]
#[must_use]
pub const fn f64_mulAdd(
    a: float64_t,
    b: float64_t,
    c: float64_t,
    roundingMode: u8,
    detectTininess: u8,
) -> (float64_t, u8) {
    return softfloat_mulAddF64(a.v, b.v, c.v, 0, roundingMode, detectTininess);
}
