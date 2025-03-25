use super::{
    expF64UI, float32_t, float64_t, fracF64UI, packToF32, signF64UI, softfloat_commonNaNToF32UI,
    softfloat_f64UIToCommonNaN, softfloat_roundPackToF32, softfloat_shortShiftRightJam64,
};

#[must_use]
pub const fn f64_to_f32(a: float64_t, roundingMode: u8, detectTininess: u8) -> (float32_t, u8) {
    let sign = signF64UI(a.v);
    let exp = expF64UI(a.v);
    let frac = fracF64UI(a.v);

    if exp == 0x7FF {
        if frac != 0 {
            let (cn, flags) = softfloat_f64UIToCommonNaN(a.v);
            return (
                float32_t {
                    v: softfloat_commonNaNToF32UI(&cn),
                },
                flags,
            );
        }
        return (packToF32(sign, 0xFF, 0), 0);
    }

    let frac32 = softfloat_shortShiftRightJam64(frac, 22) as u32;
    if ((exp as u32) | frac32) == 0 {
        return (packToF32(sign, 0, 0), 0);
    }

    return softfloat_roundPackToF32(
        sign,
        exp.wrapping_sub(0x381),
        frac32 | 0x4000_0000,
        roundingMode,
        detectTininess,
    );
}
