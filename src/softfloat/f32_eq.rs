use super::{float32_t, isNaNF32UI, softfloat_flag_invalid, softfloat_isSigNaNF32UI};

#[must_use]
pub const fn f32_eq(a: float32_t, b: float32_t) -> (bool, u8) {
    if isNaNF32UI(a.v) || isNaNF32UI(b.v) {
        if softfloat_isSigNaNF32UI(a.v) || softfloat_isSigNaNF32UI(b.v) {
            return (false, softfloat_flag_invalid);
        }
        return (false, 0);
    }
    return (a.v == b.v || ((a.v | b.v) << 1) == 0, 0);
}
