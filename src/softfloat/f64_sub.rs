use super::{float64_t, signF64UI, softfloat_addMagsF64, softfloat_subMagsF64};

#[inline]
#[must_use]
pub const fn f64_sub(
    a: float64_t,
    b: float64_t,
    roundingMode: u8,
    detectTininess: u8,
) -> (float64_t, u8) {
    let signA = signF64UI(a.v);
    let signB = signF64UI(b.v);
    if signA == signB {
        return softfloat_subMagsF64(a.v, b.v, signA, roundingMode, detectTininess);
    }
    return softfloat_addMagsF64(a.v, b.v, signA, roundingMode, detectTininess);
}
