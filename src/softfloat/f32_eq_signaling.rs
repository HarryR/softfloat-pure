use super::{float32_t, isNaNF32UI, softfloat_flag_invalid};

#[must_use]
pub const fn f32_eq_signaling(a: float32_t, b: float32_t) -> (bool, u8) {
    if isNaNF32UI(a.v) || isNaNF32UI(b.v) {
        return (false, softfloat_flag_invalid);
    }
    return ((a.v == b.v) || ((a.v | b.v) << 1) == 0, 0);
}
