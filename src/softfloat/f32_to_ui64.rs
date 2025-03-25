use super::{
    expF32UI, float32_t, fracF32UI, signF32UI, softfloat_flag_invalid, softfloat_roundToUI64,
    softfloat_shiftRightJam64Extra, ui64_fromNaN, ui64_fromNegOverflow, ui64_fromPosOverflow,
};

#[must_use]
pub const fn f32_to_ui64(a: float32_t, roundingMode: u8, exact: bool) -> (u64, u8) {
    let sign = signF32UI(a.v);
    let exp = expF32UI(a.v);
    let mut sig = fracF32UI(a.v);
    let mut flags: u8 = 0;

    let shiftDist: i16 = 0xBE - exp;
    if shiftDist < 0 {
        flags |= softfloat_flag_invalid;
        return (
            (if exp == 0xFF && sig != 0 {
                ui64_fromNaN
            } else if sign {
                ui64_fromNegOverflow
            } else {
                ui64_fromPosOverflow
            }),
            flags,
        );
    }

    if exp != 0 {
        sig |= 0x0080_0000;
    }

    let mut sig64 = (sig as u64) << 40;
    let mut extra: u64 = 0;

    if shiftDist != 0 {
        let sig64Extra = softfloat_shiftRightJam64Extra(sig64, 0, shiftDist as u16 as u32);
        sig64 = sig64Extra.v;
        extra = sig64Extra.extra;
    }

    return softfloat_roundToUI64(sign, sig64, extra, roundingMode, exact);
}
