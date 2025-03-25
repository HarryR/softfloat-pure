use super::{
    defaultNaNF64UI, expF64UI, float64_t, fracF64UI, signF64UI, softfloat_approxRecipSqrt32_1,
    softfloat_flag_invalid, softfloat_normSubnormalF64Sig, softfloat_propagateNaNF64,
    softfloat_roundPackToF64,
};

#[must_use]
pub const fn f64_sqrt(a: float64_t, roundingMode: u8, detectTininess: u8) -> (float64_t, u8) {
    let signA = signF64UI(a.v);
    let mut expA = expF64UI(a.v);
    let mut sigA = fracF64UI(a.v);
    // ------------------------------------------------------------------------
    if expA == 0x7FF {
        if sigA != 0 {
            return softfloat_propagateNaNF64(a.v, 0);
        }
        if !signA {
            return (a, 0);
        }
        // invalid
        return (float64_t { v: defaultNaNF64UI }, softfloat_flag_invalid);
    }
    // ------------------------------------------------------------------------
    if signA {
        if 0 == ((expA as u64) | sigA) {
            return (a, 0);
        }
        // invalid
        return (float64_t { v: defaultNaNF64UI }, softfloat_flag_invalid);
    }
    // ------------------------------------------------------------------------
    if 0 == expA {
        if 0 == sigA {
            return (a, 0);
        }
        let normExpSig = softfloat_normSubnormalF64Sig(sigA);
        expA = normExpSig.exp;
        sigA = normExpSig.sig;
    }
    // `sig32Z' is guaranteed to be a lower bound on the square root of `sig32A',
    // which makes `sig32Z' also a lower bound on the square root of `sigA'.
    let expZ = ((expA.wrapping_sub(0x3FF)) >> 1).wrapping_add(0x3FE);
    expA &= 1;
    sigA |= 0x0010_0000_0000_0000;
    let sig32A = (sigA >> 21) as u32;
    let recipSqrt32 = softfloat_approxRecipSqrt32_1(expA as u32, sig32A);
    let mut sig32Z = ((sig32A as u64).wrapping_mul(recipSqrt32 as u64) >> 32) as u32;
    if expA != 0 {
        sigA <<= 8;
        sig32Z >>= 1;
    } else {
        sigA <<= 9;
    }
    let mut rem = sigA.wrapping_sub((sig32Z as u64).wrapping_mul(sig32Z as u64));
    let q = (((rem >> 2) as u64).wrapping_mul(recipSqrt32 as u64) >> 32) as u32;
    let mut sigZ = ((sig32Z as u64) << 32 | 1u64 << 5).wrapping_add((q as u64) << 3);
    // ------------------------------------------------------------------------
    if ((sigZ & 0x1FF) < 0x22) {
        sigZ &= !(0x3F as u64);
        let shiftedSigZ = sigZ >> 6;
        rem = (sigA << 52).wrapping_sub(shiftedSigZ.wrapping_mul(shiftedSigZ));
        if (rem & 0x8000_0000_0000_0000) != 0 {
            sigZ = sigZ.wrapping_sub(1);
        } else if rem != 0 {
            sigZ |= 1;
        }
    }
    return softfloat_roundPackToF64(false, expZ, sigZ, roundingMode, detectTininess);
}
