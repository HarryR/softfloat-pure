use super::{
    defaultNaNF64UI, expF64UI, float64_t, fracF64UI, packToF64, signF64UI, softfloat_flag_invalid,
    softfloat_mul64To128, softfloat_normSubnormalF64Sig, softfloat_propagateNaNF64,
    softfloat_roundPackToF64,
};

#[must_use]
pub const fn f64_mul(
    a: float64_t,
    b: float64_t,
    roundingMode: u8,
    detectTininess: u8,
) -> (float64_t, u8) {
    let signA = signF64UI(a.v);
    let mut expA = expF64UI(a.v);
    let mut sigA = fracF64UI(a.v);

    let signB = signF64UI(b.v);
    let mut expB = expF64UI(b.v);
    let mut sigB = fracF64UI(b.v);

    let signZ = signA ^ signB;

    if expA == 0x7FF {
        if sigA != 0 || ((expB == 0x7FF) && sigB != 0) {
            return softfloat_propagateNaNF64(a.v, b.v);
        }
        let magBits = (expB as u64) | sigB;
        if magBits == 0 {
            return (float64_t { v: defaultNaNF64UI }, softfloat_flag_invalid);
        }
        return (packToF64(signZ, 0x7FF, 0), 0);
    }

    if expB == 0x7FF {
        if sigB != 0 {
            return softfloat_propagateNaNF64(a.v, b.v);
        }
        let magBits = (expA as u64) | sigA;
        if magBits == 0 {
            return (float64_t { v: defaultNaNF64UI }, softfloat_flag_invalid);
        }
        return (packToF64(signZ, 0x7FF, 0), 0);
    }

    if expA == 0 {
        if sigA == 0 {
            return (packToF64(signZ, 0, 0), 0);
        }
        let normExpSig = softfloat_normSubnormalF64Sig(sigA);
        expA = normExpSig.exp;
        sigA = normExpSig.sig;
    }

    if expB == 0 {
        if sigB == 0 {
            return (packToF64(signZ, 0, 0), 0);
        }
        let normExpSig = softfloat_normSubnormalF64Sig(sigB);
        expB = normExpSig.exp;
        sigB = normExpSig.sig;
    }

    let mut expZ = expA.wrapping_add(expB).wrapping_sub(0x3FF);
    sigA = (sigA | 0x0010_0000_0000_0000) << 10;
    sigB = (sigB | 0x0010_0000_0000_0000) << 11;

    let sig128Z = softfloat_mul64To128(sigA, sigB);
    let mut sigZ = sig128Z.v64 | ((sig128Z.v0 != 0) as u64);

    if sigZ < 0x4000_0000_0000_0000 {
        expZ = expZ.wrapping_sub(1);
        sigZ <<= 1;
    }

    return softfloat_roundPackToF64(signZ, expZ, sigZ, roundingMode, detectTininess);
}
