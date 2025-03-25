use super::{float64_t, isNaNF64UI, signF64UI, softfloat_flag_invalid, softfloat_isSigNaNF64UI};

#[must_use]
pub const fn f64_le_quiet(a: float64_t, b: float64_t) -> (bool, u8) {
    if isNaNF64UI(a.v) || isNaNF64UI(b.v) {
        if softfloat_isSigNaNF64UI(a.v) || softfloat_isSigNaNF64UI(b.v) {
            return (false, softfloat_flag_invalid);
        }
        return (false, 0);
    }
    let signA = signF64UI(a.v);
    let signB = signF64UI(b.v);
    let result = if signA != signB {
        (signA || (a.v | b.v).trailing_zeros() >= 63)
    } else {
        (a.v == b.v || signA ^ (a.v < b.v))
    };
    return (result, 0);
}
