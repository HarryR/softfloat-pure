use super::{
    expF64UI, float64_t, fracF64UI, i32_fromNaN, i32_fromNegOverflow, i32_fromPosOverflow,
    signF64UI, softfloat_flag_invalid, softfloat_roundToI32, softfloat_shiftRightJam64,
};

#[must_use]
pub const fn f64_to_i32(a: float64_t, roundingMode: u8, exact: bool) -> (i32, u8) {
    let mut sign = signF64UI(a.v);
    let exp = expF64UI(a.v);
    let mut sig = fracF64UI(a.v);

    if (i32_fromNaN != i32_fromPosOverflow || i32_fromNaN != i32_fromNegOverflow)
        && exp == 0x7FF
        && sig != 0
    {
        if i32_fromNaN == i32_fromPosOverflow {
            sign = false;
        } else if i32_fromNaN == i32_fromNegOverflow {
            sign = true;
        } else {
            return (i32_fromNaN, softfloat_flag_invalid);
        }
    }

    if exp != 0 {
        sig |= 0x0010_0000_0000_0000;
    }

    let shiftDist: i16 = 0x427 - exp;

    if 0 < shiftDist {
        sig = softfloat_shiftRightJam64(sig, shiftDist as u32);
    }

    return softfloat_roundToI32(sign, sig, roundingMode, exact);
}
