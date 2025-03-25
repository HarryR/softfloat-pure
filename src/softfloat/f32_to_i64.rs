use super::{
    expF32UI, float32_t, fracF32UI, i64_fromNaN, i64_fromNegOverflow, i64_fromPosOverflow,
    signF32UI, softfloat_flag_invalid, softfloat_roundToI64, softfloat_shiftRightJam64Extra,
};

#[must_use]
pub const fn f32_to_i64(a: float32_t, roundingMode: u8, exact: bool) -> (i64, u8) {
    let sign = signF32UI(a.v);
    let exp = expF32UI(a.v);
    let mut sig = fracF32UI(a.v);

    let shiftDist: i16 = (0xBE as i16).wrapping_sub(exp);
    if shiftDist < 0 {
        return (
            if exp == 0xFF && sig != 0 {
                i64_fromNaN
            } else if sign {
                i64_fromNegOverflow
            } else {
                i64_fromPosOverflow
            },
            softfloat_flag_invalid,
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

    return softfloat_roundToI64(sign, sig64, extra, roundingMode, exact);
}
