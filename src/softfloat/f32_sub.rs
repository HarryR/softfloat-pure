use super::{float32_t, signF32UI, softfloat_addMagsF32, softfloat_subMagsF32};

#[must_use]
pub const fn f32_sub(
    a: float32_t,
    b: float32_t,
    roundingMode: u8,
    detectTininess: u8,
) -> (float32_t, u8) {
    if signF32UI(a.v ^ b.v) {
        return softfloat_addMagsF32(a.v, b.v, roundingMode, detectTininess);
    }
    return softfloat_subMagsF32(a.v, b.v, roundingMode, detectTininess);
}
