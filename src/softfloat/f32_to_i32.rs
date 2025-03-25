use super::{
    expF32UI, float32_t, fracF32UI, i32_fromNaN, i32_fromNegOverflow, i32_fromPosOverflow,
    signF32UI, softfloat_flag_invalid, softfloat_roundToI32, softfloat_shiftRightJam64,
};

#[must_use]
pub const fn f32_to_i32(a: float32_t, roundingMode: u8, exact: bool) -> (i32, u8) {
    let uiA = a.v;
    let mut sign = signF32UI(uiA);
    let exp = expF32UI(uiA);
    let mut sig = fracF32UI(uiA);

    if (i32_fromNaN != i32_fromPosOverflow || i32_fromNaN != i32_fromNegOverflow)
        && exp == 0xFF
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
        sig |= 0x0080_0000;
    }

    let mut sig64 = (sig as u64) << 32;
    let shiftDist = 0xaa - exp;
    if 0 < shiftDist {
        sig64 = softfloat_shiftRightJam64(sig64, shiftDist as u32);
    }
    return softfloat_roundToI32(sign, sig64, roundingMode, exact);
}
