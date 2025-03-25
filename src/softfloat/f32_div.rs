use super::{
    defaultNaNF32UI, expF32UI, float32_t, fracF32UI, packToF32, signF32UI, softfloat_flag_infinite,
    softfloat_flag_invalid, softfloat_normSubnormalF32Sig, softfloat_propagateNaNF32,
    softfloat_roundPackToF32,
};

#[must_use]
pub const fn f32_div(
    a: float32_t,
    b: float32_t,
    roundingMode: u8,
    detectTininess: u8,
) -> (float32_t, u8) {
    // ------------------------------------------------------------------------
    let signA = signF32UI(a.v);
    let mut expA = expF32UI(a.v);
    let mut sigA = fracF32UI(a.v);
    let signB = signF32UI(b.v);
    let mut expB = expF32UI(b.v);
    let mut sigB = fracF32UI(b.v);
    let signZ = signA ^ signB;
    // ------------------------------------------------------------------------
    if expA == 0xFF {
        if sigA != 0 {
            return softfloat_propagateNaNF32(a.v, b.v);
        }
        if expB == 0xFF {
            if sigB != 0 {
                return softfloat_propagateNaNF32(a.v, b.v);
            }
            // invalid
            return (float32_t { v: defaultNaNF32UI }, softfloat_flag_invalid);
        }
        // infinity
        return (packToF32(signZ, 0xFF, 0), 0);
    }
    if (expB == 0xFF) {
        if sigB != 0 {
            return softfloat_propagateNaNF32(a.v, b.v);
        }
        // zero
        return (packToF32(signZ, 0, 0), 0);
    }
    // ------------------------------------------------------------------------
    if 0 == expB {
        if 0 == sigB {
            if 0 == ((expA as u32) | sigA) {
                // invalid
                return (float32_t { v: defaultNaNF32UI }, softfloat_flag_invalid);
            }
            // infinity
            return (packToF32(signZ, 0xFF, 0), softfloat_flag_infinite);
        }
        let normExpSig = softfloat_normSubnormalF32Sig(sigB);
        expB = normExpSig.exp;
        sigB = normExpSig.sig;
    }
    if 0 == expA {
        if 0 == sigA {
            // zero
            return (packToF32(signZ, 0, 0), 0);
        }
        let normExpSig = softfloat_normSubnormalF32Sig(sigA);
        expA = normExpSig.exp;
        sigA = normExpSig.sig;
    }
    // ------------------------------------------------------------------------
    let mut expZ = expA.wrapping_sub(expB).wrapping_add(0x7E);
    sigA |= 0x0080_0000;
    sigB |= 0x0080_0000;
    let sig64A = if sigA < sigB {
        expZ = expZ.wrapping_sub(1);
        (sigA as u64) << 31
    } else {
        (sigA as u64) << 30
    };
    let mut sigZ = sig64A / (sigB as u64);
    if 0 == (sigZ & 0x3F) {
        sigZ |= ((sigB as u64) * sigZ != sig64A) as u64;
    }
    return softfloat_roundPackToF32(signZ, expZ, sigZ as u32, roundingMode, detectTininess);
}
