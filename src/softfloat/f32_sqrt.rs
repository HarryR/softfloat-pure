use super::{
    defaultNaNF32UI, expF32UI, float32_t, fracF32UI, signF32UI, softfloat_approxRecipSqrt32_1,
    softfloat_flag_invalid, softfloat_normSubnormalF32Sig, softfloat_propagateNaNF32,
    softfloat_roundPackToF32,
};

#[must_use]
pub const fn f32_sqrt(a: float32_t, roundingMode: u8, detectTininess: u8) -> (float32_t, u8) {
    // ------------------------------------------------------------------------
    let signA = signF32UI(a.v);
    let mut expA = expF32UI(a.v);
    let mut sigA = fracF32UI(a.v);
    // ------------------------------------------------------------------------
    if expA == 0xFF {
        if sigA != 0 {
            return softfloat_propagateNaNF32(a.v, 0);
        }
        if !signA {
            return (a, 0);
        }
        // invalid
        return (float32_t { v: defaultNaNF32UI }, softfloat_flag_invalid);
    }
    // ------------------------------------------------------------------------
    if signA {
        if 0 == ((expA as u32) | sigA) {
            return (a, 0);
        };
        // invalid
        return (float32_t { v: defaultNaNF32UI }, softfloat_flag_invalid);
    }
    // ------------------------------------------------------------------------
    if 0 == expA {
        if 0 == sigA {
            return (a, 0);
        }
        let normExpSig = softfloat_normSubnormalF32Sig(sigA);
        expA = normExpSig.exp;
        sigA = normExpSig.sig;
    }
    // ------------------------------------------------------------------------
    let expZ = ((expA.wrapping_sub(0x7F)) >> 1).wrapping_add(0x7E);
    expA &= 1;
    sigA = (sigA | 0x0080_0000) << 8;
    let mut sigZ = ((sigA as u64)
        .wrapping_mul(softfloat_approxRecipSqrt32_1(expA as u32, sigA) as u64)
        >> 32) as u32;
    if expA != 0 {
        sigZ >>= 1;
    };
    // ------------------------------------------------------------------------
    sigZ = sigZ.wrapping_add(2);
    if ((sigZ & 0x3F) < 2) {
        let shiftedSigZ = sigZ >> 2;
        let negRem = shiftedSigZ.wrapping_mul(shiftedSigZ);
        sigZ &= !(3 as u32);
        if (negRem & 0x8000_0000) != 0 {
            sigZ |= 1;
        } else if negRem != 0 {
            sigZ = sigZ.wrapping_sub(1);
        }
    }
    return softfloat_roundPackToF32(false, expZ, sigZ, roundingMode, detectTininess);
}
