use super::{
    expF64UI, float64_t, fracF64UI, signF64UI, softfloat_flag_invalid, softfloat_roundToUI64,
    softfloat_shiftRightJam64Extra, ui64_fromNaN, ui64_fromNegOverflow, ui64_fromPosOverflow,
    uint64_extra,
};

#[must_use]
pub const fn f64_to_ui64(a: float64_t, roundingMode: u8, exact: bool) -> (u64, u8) {
    let sign = signF64UI(a.v);
    let exp = expF64UI(a.v);
    let mut sig = fracF64UI(a.v);

    if exp != 0 {
        sig |= 0x0010_0000_0000_0000;
    }
    let shiftDist: i16 = 0x433 - exp;

    let mut sigExtra: uint64_extra = uint64_extra { extra: 0, v: 0 };

    if shiftDist <= 0 {
        if shiftDist < -11 {
            return (
                if exp == 0x7FF && fracF64UI(a.v) != 0 {
                    ui64_fromNaN
                } else if sign {
                    ui64_fromNegOverflow
                } else {
                    ui64_fromPosOverflow
                },
                softfloat_flag_invalid,
            );
        }
        sigExtra.v = sig << -shiftDist;
        sigExtra.extra = 0;
    } else {
        sigExtra = softfloat_shiftRightJam64Extra(sig, 0, shiftDist as u16 as u32);
    }

    return softfloat_roundToUI64(sign, sigExtra.v, sigExtra.extra, roundingMode, exact);
}
