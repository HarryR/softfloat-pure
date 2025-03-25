use super::{
    expF32UI, float32_t, fracF32UI, signF32UI, softfloat_flag_invalid, softfloat_roundToUI32,
    softfloat_shiftRightJam64, ui32_fromNaN, ui32_fromNegOverflow, ui32_fromPosOverflow,
};

#[must_use]
pub const fn f32_to_ui32(a: float32_t, roundingMode: u8, exact: bool) -> (u32, u8) {
    let mut sign = signF32UI(a.v);
    let exp = expF32UI(a.v);
    let mut sig = fracF32UI(a.v);

    if (ui32_fromNaN != ui32_fromPosOverflow || ui32_fromNaN != ui32_fromNegOverflow)
        && (exp == 0xFF)
        && sig != 0
    {
        if ui32_fromNaN == ui32_fromPosOverflow {
            sign = false;
        } else if ui32_fromNaN == ui32_fromNegOverflow {
            sign = true;
        } else {
            return (ui32_fromNaN, softfloat_flag_invalid);
        }
    }
    if exp != 0 {
        sig |= 0x0080_0000;
    }
    let mut sig64 = (sig as u64) << 32;
    let shiftDist = (0xAA as i16).wrapping_sub(exp);
    if 0 < shiftDist {
        sig64 = softfloat_shiftRightJam64(sig64, shiftDist as u16 as u32);
    }
    return softfloat_roundToUI32(sign, sig64, roundingMode, exact);
}
