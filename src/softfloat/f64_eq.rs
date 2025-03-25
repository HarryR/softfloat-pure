use super::{float64_t, isNaNF64UI, softfloat_flag_invalid, softfloat_isSigNaNF64UI};

#[must_use]
pub const fn f64_eq(mut a: float64_t, mut b: float64_t) -> (bool, u8) {
    if isNaNF64UI(a.v) || isNaNF64UI(b.v) {
        if softfloat_isSigNaNF64UI(a.v) || softfloat_isSigNaNF64UI(b.v) {
            return (false, softfloat_flag_invalid);
        }
        return (false, 0);
    }
    return (a.v == b.v || (a.v | b.v).trailing_zeros() >= 63, 0);
}
