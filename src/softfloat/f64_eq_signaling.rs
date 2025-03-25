use super::{float64_t, isNaNF64UI, softfloat_flag_invalid};

#[must_use]
pub const fn f64_eq_signaling(a: float64_t, b: float64_t) -> (bool, u8) {
    if isNaNF64UI(a.v) || isNaNF64UI(b.v) {
        return (false, softfloat_flag_invalid);
    }
    let result = (a.v == b.v) || (a.v | b.v).trailing_zeros() >= 63;
    return (result, 0);
}
