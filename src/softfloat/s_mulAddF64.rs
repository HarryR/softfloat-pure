use super::{
    defaultNaNF64UI, expF64UI, float64_t, fracF64UI, packToF64, packToF64UI, signF64UI,
    softfloat_add128, softfloat_countLeadingZeros64, softfloat_flag_invalid, softfloat_mul64To128,
    softfloat_normSubnormalF64Sig, softfloat_propagateNaNF64, softfloat_propagateNaNF64UI,
    softfloat_roundPackToF64, softfloat_round_min, softfloat_shiftRightJam128,
    softfloat_shiftRightJam64, softfloat_shortShiftLeft128, softfloat_shortShiftRightJam128,
    softfloat_shortShiftRightJam64, softfloat_sub128, uint128,
};

pub const softfloat_mulAdd_subProd: u8 = 2;
pub const softfloat_mulAdd_subC: u8 = 1;

#[inline]
const fn propagateNaN_ZC(uiZ: u64, uiC: u64) -> (float64_t, u8) {
    return softfloat_propagateNaNF64(uiZ, uiC);
}

#[inline]
const fn propagateNaN_ABC(uiA: u64, uiB: u64, uiC: u64) -> (float64_t, u8) {
    let (uiZ, flags) = softfloat_propagateNaNF64UI(uiA, uiB);
    let (res, new_flags) = propagateNaN_ZC(uiZ, uiC);
    return (res, flags | new_flags);
}

#[inline]
const fn infProdArg(
    magBits: u64,
    signZ: bool,
    expC: i16,
    sigC: u64,
    signC: bool,
    uiC: u64,
) -> (float64_t, u8) {
    if (magBits != 0) {
        let uiZ = packToF64UI(signZ, 0x7FF, 0);
        if expC != 0x7FF {
            return (float64_t { v: uiZ }, 0);
        }
        if sigC != 0 {
            return propagateNaN_ZC(uiZ, uiC);
        }
        if (signZ == signC) {
            return (float64_t { v: uiZ }, 0);
        }
    }
    let (res, flags) = propagateNaN_ZC(defaultNaNF64UI, uiC);
    return (res, flags | softfloat_flag_invalid);
}

#[inline]
const fn completeCancellation(roundingMode: u8) -> (float64_t, u8) {
    (packToF64((roundingMode == softfloat_round_min), 0, 0), 0)
}

#[inline]
const fn zeroProd(
    uiC: u64,
    expC: i16,
    sigC: u64,
    signZ: bool,
    signC: bool,
    roundingMode: u8,
) -> (float64_t, u8) {
    if (0 == ((expC as u64) | sigC) && (signZ != signC)) {
        return completeCancellation(roundingMode);
    }
    return (float64_t { v: uiC }, 0);
}

#[must_use]
pub const fn softfloat_mulAddF64(
    uiA: u64,
    uiB: u64,
    uiC: u64,
    op: u8,
    roundingMode: u8,
    detectTininess: u8,
) -> (float64_t, u8) {
    let signA = signF64UI(uiA);
    let mut expA = expF64UI(uiA);
    let mut sigA = fracF64UI(uiA);
    let signB = signF64UI(uiB);
    let mut expB = expF64UI(uiB);
    let mut sigB = fracF64UI(uiB);
    let signC = signF64UI(uiC) ^ (op == softfloat_mulAdd_subC);
    let mut expC = expF64UI(uiC);
    let mut sigC = fracF64UI(uiC);
    let mut signZ = signA ^ signB ^ (op == softfloat_mulAdd_subProd);
    // ------------------------------------------------------------------------
    if expA == 0x7FF {
        if (sigA != 0) || ((expB == 0x7FF) && (sigB != 0)) {
            return propagateNaN_ABC(uiA, uiB, uiC);
        }
        return infProdArg((expB as u64) | sigB, signZ, expC, sigC, signC, uiC);
    }
    if expB == 0x7FF {
        if sigB != 0 {
            return propagateNaN_ABC(uiA, uiB, uiC);
        }
        return infProdArg((expA as u64) | sigA, signZ, expC, sigC, signC, uiC);
    }
    if expC == 0x7FF {
        if sigC != 0 {
            return propagateNaN_ZC(0, uiC);
        }
        return (float64_t { v: uiC }, 0);
    }
    // ------------------------------------------------------------------------
    if 0 == expA {
        if 0 == sigA {
            return zeroProd(uiC, expC, sigC, signZ, signC, roundingMode);
        }
        let normExpSig = softfloat_normSubnormalF64Sig(sigA);
        expA = normExpSig.exp;
        sigA = normExpSig.sig;
    }
    if 0 == expB {
        if 0 == sigB {
            return zeroProd(uiC, expC, sigC, signZ, signC, roundingMode);
        }
        let normExpSig = softfloat_normSubnormalF64Sig(sigB);
        expB = normExpSig.exp;
        sigB = normExpSig.sig;
    }
    // ------------------------------------------------------------------------
    let mut expZ = expA.wrapping_add(expB).wrapping_sub(0x3FE);
    sigA = (sigA | 0x0010_0000_0000_0000) << 10;
    sigB = (sigB | 0x0010_0000_0000_0000) << 10;
    let mut sig128Z = softfloat_mul64To128(sigA, sigB);
    if sig128Z.v64 < 0x2000_0000_0000_0000 {
        expZ = expZ.wrapping_sub(1);
        sig128Z = softfloat_add128(sig128Z.v64, sig128Z.v0, sig128Z.v64, sig128Z.v0);
    }
    if 0 == expC {
        if 0 == sigC {
            expZ = expZ.wrapping_sub(1);
            let sigZ = (sig128Z.v64 << 1) | ((sig128Z.v0 != 0) as u64);
            return softfloat_roundPackToF64(signZ, expZ, sigZ, roundingMode, detectTininess);
        }
        let normExpSig = softfloat_normSubnormalF64Sig(sigC);
        expC = normExpSig.exp;
        sigC = normExpSig.sig;
    }
    sigC = (sigC | 0x0010_0000_0000_0000) << 9;
    // ------------------------------------------------------------------------
    let expDiff = expZ.wrapping_sub(expC);
    let mut sig128C = uint128 { v64: 0, v0: 0 };
    let mut sigZ;

    if expDiff < 0 {
        expZ = expC;
        if signZ == signC || expDiff < -1 {
            sig128Z.v64 = softfloat_shiftRightJam64(sig128Z.v64, expDiff.wrapping_neg() as u32);
        } else {
            sig128Z = softfloat_shortShiftRightJam128(sig128Z.v64, sig128Z.v0, 1);
        }
    } else if expDiff != 0 {
        sig128C = softfloat_shiftRightJam128(sigC, 0, expDiff as u32);
    }
    // ------------------------------------------------------------------------
    if signZ == signC {
        // --------------------------------------------------------------------
        if expDiff <= 0 {
            sigZ = sigC.wrapping_add(sig128Z.v64) | ((sig128Z.v0 != 0) as u64);
        } else {
            sig128Z = softfloat_add128(sig128Z.v64, sig128Z.v0, sig128C.v64, sig128C.v0);
            sigZ = sig128Z.v64 | ((sig128Z.v0 != 0) as u64);
        }
        if sigZ < 0x4000_0000_0000_0000 {
            expZ = expZ.wrapping_sub(1);
            sigZ <<= 1;
        }
    } else {
        // --------------------------------------------------------------------
        if expDiff < 0 {
            signZ = signC;
            sig128Z = softfloat_sub128(sigC, 0, sig128Z.v64, sig128Z.v0);
        } else if 0 == expDiff {
            sig128Z.v64 = sig128Z.v64.wrapping_sub(sigC);
            if 0 == (sig128Z.v64 | sig128Z.v0) {
                return completeCancellation(roundingMode);
            }
            if (sig128Z.v64 & 0x8000_0000_0000_0000) != 0 {
                signZ = !signZ;
                sig128Z = softfloat_sub128(0, 0, sig128Z.v64, sig128Z.v0);
            }
        } else {
            sig128Z = softfloat_sub128(sig128Z.v64, sig128Z.v0, sig128C.v64, sig128C.v0);
        }
        // --------------------------------------------------------------------
        if 0 == sig128Z.v64 {
            expZ = expZ.wrapping_sub(64);
            sig128Z.v64 = sig128Z.v0;
            sig128Z.v0 = 0;
        }
        let shiftDist = softfloat_countLeadingZeros64(sig128Z.v64).wrapping_sub(1) as i8;
        expZ = expZ.wrapping_sub(shiftDist as i16);
        if shiftDist < 0 {
            sigZ = softfloat_shortShiftRightJam64(sig128Z.v64, shiftDist.wrapping_neg() as u8);
        } else {
            sig128Z = softfloat_shortShiftLeft128(sig128Z.v64, sig128Z.v0, shiftDist as u8);
            sigZ = sig128Z.v64;
        }
        sigZ |= (sig128Z.v0 != 0) as u64;
    }
    return softfloat_roundPackToF64(signZ, expZ, sigZ, roundingMode, detectTininess);
}

#[cfg(test)]
mod tests {
    use super::super::{softfloat_round_near_even, softfloat_tininess_beforeRounding};
    use super::*;

    #[test]
    fn test_mulAddF64() {
        let a: u64 = 0x41E00003FFFBFFFF;
        let b: u64 = 0xBFDFFFFFFFEFFFFF;
        let c: u64 = 0x80251295103185AE;
        let (res, flags) = softfloat_mulAddF64(
            a,
            b,
            c,
            0, /* mulAdd */
            softfloat_round_near_even,
            softfloat_tininess_beforeRounding,
        );
        assert_eq!(flags, 1);
        assert_eq!(res.v, 0xC1D00003FFF3FFFD);
    }
}
