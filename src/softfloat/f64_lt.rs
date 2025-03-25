use super::{float64_t, isNaNF64UI, signF64UI, softfloat_flag_invalid};

#[must_use]
pub const fn f64_lt(a: float64_t, b: float64_t) -> (bool, u8) {
    if isNaNF64UI(a.v) || isNaNF64UI(b.v) {
        return (false, softfloat_flag_invalid);
    }
    let signA = signF64UI(a.v);
    let signB = signF64UI(b.v);
    let result = if signA != signB {
        (signA && (a.v | b.v) & 0x7fff_ffff_ffff_ffff != 0)
    } else {
        (a.v != b.v && signA ^ (a.v < b.v))
    };
    return (result, 0);
}
