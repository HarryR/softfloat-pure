use super::{float32_t, isNaNF32UI, signF32UI, softfloat_flag_invalid};

#[must_use]
pub const fn f32_le(a: float32_t, b: float32_t) -> (bool, u8) {
    if isNaNF32UI(a.v) || isNaNF32UI(b.v) {
        return (false, softfloat_flag_invalid);
    }
    let signA = signF32UI(a.v);
    let signB = signF32UI(b.v);
    (
        if signA != signB {
            signA || ((a.v | b.v) << 1) == 0
        } else {
            a.v == b.v || signA ^ (a.v < b.v)
        },
        0,
    )
}
