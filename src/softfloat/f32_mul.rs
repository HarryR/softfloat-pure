use super::{
    defaultNaNF32UI, exp16_sig32, expF32UI, float32_t, fracF32UI, packToF32, signF32UI,
    softfloat_flag_invalid, softfloat_normSubnormalF32Sig, softfloat_propagateNaNF32,
    softfloat_roundPackToF32, softfloat_shortShiftRightJam64,
};

#[must_use]
pub const fn f32_mul(
    a: float32_t,
    b: float32_t,
    roundingMode: u8,
    detectTininess: u8,
) -> (float32_t, u8) {
    let signA = signF32UI(a.v);
    let mut expA = expF32UI(a.v);
    let mut sigA = fracF32UI(a.v);

    let signB = signF32UI(b.v);
    let mut expB = expF32UI(b.v);
    let mut sigB = fracF32UI(b.v);

    let signZ = signA ^ signB;

    if expA == 0xFF {
        if sigA != 0 || ((expB == 0xFF) && sigB != 0) {
            return softfloat_propagateNaNF32(a.v, b.v);
        }
        let magBits = (expB as u32) | sigB;
        if magBits == 0 {
            return (float32_t { v: defaultNaNF32UI }, softfloat_flag_invalid);
        }
        return (packToF32(signZ, 0xFF, 0), 0);
    }

    let mut normExpSig = exp16_sig32 { exp: 0, sig: 0 };

    if expB == 0xFF {
        if sigB != 0 {
            return softfloat_propagateNaNF32(a.v, b.v);
        }
        let magBits = (expA as u32) | sigA;
        if magBits == 0 {
            return (float32_t { v: defaultNaNF32UI }, softfloat_flag_invalid);
        }
        return (packToF32(signZ, 0xFF, 0), 0);
    }

    if expA == 0 {
        if sigA == 0 {
            return (packToF32(signZ, 0, 0), 0);
        }
        normExpSig = softfloat_normSubnormalF32Sig(sigA);
        expA = normExpSig.exp;
        sigA = normExpSig.sig;
    }

    if expB == 0 {
        if sigB == 0 {
            return (packToF32(signZ, 0, 0), 0);
        }
        normExpSig = softfloat_normSubnormalF32Sig(sigB);
        expB = normExpSig.exp;
        sigB = normExpSig.sig;
    }

    let mut expZ = expA.wrapping_add(expB).wrapping_sub(0x7F);
    sigA = (sigA | 0x0080_0000) << 7;
    sigB = (sigB | 0x0080_0000) << 8;
    let mut sigZ: u32 =
        softfloat_shortShiftRightJam64((sigA as u64).wrapping_mul(sigB as u64), 32) as u32;
    if sigZ < 0x4000_0000 {
        expZ -= 1;
        sigZ <<= 1;
    }

    return softfloat_roundPackToF32(signZ, expZ, sigZ, roundingMode, detectTininess);
}
