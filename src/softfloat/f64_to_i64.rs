use super::{
    expF64UI, float64_t, fracF64UI, i64_fromNaN, i64_fromNegOverflow, i64_fromPosOverflow,
    signF64UI, softfloat_flag_invalid, softfloat_roundToI64, softfloat_shiftRightJam64Extra,
    uint64_extra,
};

#[must_use]
pub const fn f64_to_i64(a: float64_t, roundingMode: u8, exact: bool) -> (i64, u8) {
    let sign = signF64UI(a.v);
    let exp = expF64UI(a.v);
    let mut sig = fracF64UI(a.v);

    if exp != 0 {
        sig |= 0x0010_0000_0000_0000;
    }
    let shiftDist: i16 = (0x433 as i16).wrapping_sub(exp);

    let mut sigExtra = uint64_extra { extra: 0, v: 0 };

    if shiftDist <= 0 {
        if shiftDist < -11 {
            return (
                if exp == 0x7FF && fracF64UI(a.v) != 0 {
                    i64_fromNaN
                } else if sign {
                    i64_fromNegOverflow
                } else {
                    i64_fromPosOverflow
                },
                softfloat_flag_invalid,
            );
        }
        sigExtra.v = sig << shiftDist.wrapping_neg();
        sigExtra.extra = 0;
    } else {
        sigExtra = softfloat_shiftRightJam64Extra(sig, 0, shiftDist as u16 as u32);
    }

    return softfloat_roundToI64(sign, sigExtra.v, sigExtra.extra, roundingMode, exact);
}
