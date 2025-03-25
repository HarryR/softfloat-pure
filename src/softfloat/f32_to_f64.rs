use super::{
    expF32UI, float32_t, float64_t, fracF32UI, packToF64, signF32UI, softfloat_commonNaNToF64UI,
    softfloat_f32UIToCommonNaN, softfloat_normSubnormalF32Sig,
};

#[must_use]
pub const fn f32_to_f64(a: float32_t) -> (float64_t, u8) {
    let sign = signF32UI(a.v);
    let mut exp = expF32UI(a.v);
    let mut frac = fracF32UI(a.v);

    if exp == 0xFF {
        if frac != 0 {
            let (cn, flags) = softfloat_f32UIToCommonNaN(a.v);
            return (
                float64_t {
                    v: softfloat_commonNaNToF64UI(&cn),
                },
                flags,
            );
        }
        return (packToF64(sign, 0x7FF, 0), 0);
    }

    if exp == 0 {
        if frac == 0 {
            return (packToF64(sign, 0, 0), 0);
        }
        let normExpSig = softfloat_normSubnormalF32Sig(frac);
        exp = normExpSig.exp.wrapping_sub(1);
        frac = normExpSig.sig;
    }

    return (
        packToF64(sign, exp.wrapping_add(0x380), (frac as u64) << 29),
        0,
    );
}
