use super::{float32_t, isNaNF32UI, signF32UI, softfloat_flag_invalid, softfloat_isSigNaNF32UI};

#[must_use]
pub const fn f32_lt_quiet(a: float32_t, b: float32_t) -> (bool, u8) {
    if isNaNF32UI(a.v) || isNaNF32UI(b.v) {
        if softfloat_isSigNaNF32UI(a.v) || softfloat_isSigNaNF32UI(b.v) {
            return (false, softfloat_flag_invalid);
        }
        return (false, 0);
    }
    let signA = signF32UI(a.v);
    let signB = signF32UI(b.v);
    return (
        if signA != signB {
            signA && ((a.v | b.v) << 1) != 0
        } else {
            a.v != b.v && signA ^ (a.v < b.v)
        },
        0,
    );
}
