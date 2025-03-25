use super::{
    defaultNaNF32UI, expF32UI, float32_t, fracF32UI, packToF32, packToF32UI, signF32UI,
    softfloat_countLeadingZeros64, softfloat_flag_invalid, softfloat_normSubnormalF32Sig,
    softfloat_propagateNaNF32, softfloat_propagateNaNF32UI, softfloat_roundPackToF32,
    softfloat_round_min, softfloat_shiftRightJam64, softfloat_shortShiftRightJam64,
};

pub const softfloat_mulAdd_subProd: u8 = 2;
pub const softfloat_mulAdd_subC: u8 = 1;

#[inline]
const fn propagateNaN_ZC(uiZ: u32, uiC: u32) -> (float32_t, u8) {
    softfloat_propagateNaNF32(uiZ, uiC)
}

#[inline]
const fn propagateNaN_ABC(uiA: u32, uiB: u32, uiC: u32) -> (float32_t, u8) {
    let (uiZ, flags) = softfloat_propagateNaNF32UI(uiA, uiB);
    let (res, new_flags) = propagateNaN_ZC(uiZ, uiC);
    return (res, flags | new_flags);
}

#[inline]
const fn infProdArg(
    magBits: u32,
    signProd: bool,
    expC: i16,
    sigC: u32,
    signC: bool,
    uiC: u32,
) -> (float32_t, u8) {
    if magBits != 0 {
        let uiZ = packToF32UI(signProd, 0xFF, 0);
        if expC != 0xFF {
            return (float32_t { v: uiZ }, 0);
        }
        if sigC != 0 {
            return propagateNaN_ZC(uiZ, uiC);
        }
        if (signProd == signC) {
            return (float32_t { v: uiZ }, 0);
        }
    }
    let (res, flags) = propagateNaN_ZC(defaultNaNF32UI, uiC);
    return (res, flags | softfloat_flag_invalid);
}

#[inline]
const fn completeCancellation(roundingMode: u8) -> float32_t {
    packToF32((roundingMode == softfloat_round_min), 0, 0)
}

#[inline]
const fn zeroProd(
    uiC: u32,
    expC: i16,
    sigC: u32,
    signProd: bool,
    signC: bool,
    roundingMode: u8,
) -> float32_t {
    if 0 == ((expC as u32) | sigC) && (signProd != signC) {
        return completeCancellation(roundingMode);
    }
    return float32_t { v: uiC };
}

#[must_use]
pub const fn softfloat_mulAddF32(
    uiA: u32,
    uiB: u32,
    uiC: u32,
    op: u8,
    roundingMode: u8,
    detectTininess: u8,
) -> (float32_t, u8) {
    let signA = signF32UI(uiA);
    let mut expA = expF32UI(uiA);
    let mut sigA = fracF32UI(uiA);
    let signB = signF32UI(uiB);
    let mut expB = expF32UI(uiB);
    let mut sigB = fracF32UI(uiB);
    let signC = signF32UI(uiC) ^ (op == softfloat_mulAdd_subC);
    let mut expC = expF32UI(uiC);
    let mut sigC = fracF32UI(uiC);
    let signProd = signA ^ signB ^ (op == softfloat_mulAdd_subProd);

    // ------------------------------------------------------------------------

    if expA == 0xFF {
        if sigA != 0 || ((expB == 0xFF) && (sigB != 0)) {
            return propagateNaN_ABC(uiA, uiB, uiC);
        }
        return infProdArg((expB as u32) | sigB, signProd, expC, sigC, signC, uiC);
    }
    if expB == 0xFF {
        if sigB != 0 {
            return propagateNaN_ABC(uiA, uiB, uiC);
        }
        return infProdArg((expA as u32) | sigA, signProd, expC, sigC, signC, uiC);
    }
    if expC == 0xFF {
        if sigC != 0 {
            return propagateNaN_ZC(0, uiC);
        }
        return (float32_t { v: uiC }, 0);
    }

    // ------------------------------------------------------------------------

    if expA == 0 {
        if sigA == 0 {
            return (zeroProd(uiC, expC, sigC, signProd, signC, roundingMode), 0);
        }
        let normExpSig = softfloat_normSubnormalF32Sig(sigA);
        expA = normExpSig.exp;
        sigA = normExpSig.sig;
    }
    if expB == 0 {
        if sigB == 0 {
            return (zeroProd(uiC, expC, sigC, signProd, signC, roundingMode), 0);
        }
        let normExpSig = softfloat_normSubnormalF32Sig(sigB);
        expB = normExpSig.exp;
        sigB = normExpSig.sig;
    }

    // ------------------------------------------------------------------------

    let mut expProd = expA.wrapping_add(expB).wrapping_sub(0x7E);
    sigA = (sigA | 0x0080_0000) << 7;
    sigB = (sigB | 0x0080_0000) << 7;
    let mut sigProd = (sigA as u64).wrapping_mul(sigB as u64);
    if sigProd < 0x2000_0000_0000_0000 {
        expProd = expProd.wrapping_sub(1);
        sigProd <<= 1;
    }

    if expC == 0 {
        if sigC == 0 {
            return softfloat_roundPackToF32(
                signProd,
                expProd.wrapping_sub(1),
                softfloat_shortShiftRightJam64(sigProd, 31) as u32,
                roundingMode,
                detectTininess,
            );
        }
        let normExpSig = softfloat_normSubnormalF32Sig(sigC);
        expC = normExpSig.exp;
        sigC = normExpSig.sig;
    }
    sigC = (sigC | 0x0080_0000) << 6;

    // ------------------------------------------------------------------------

    let expDiff = expProd - expC;
    let mut sig64Z: u64;
    let mut sigZ: u32;
    let mut expZ: i16;
    let mut signZ = signProd;
    if signProd == signC {
        // ------------------------------------------------------------------------
        if expDiff <= 0 {
            expZ = expC;
            sigZ = (sigC as u64).wrapping_add(softfloat_shiftRightJam64(
                sigProd,
                (32 as i16).wrapping_sub(expDiff) as u32,
            )) as u32;
        } else {
            expZ = expProd;
            sig64Z = sigProd.wrapping_add(softfloat_shiftRightJam64(
                (sigC as u64) << 32,
                expDiff as u32,
            ));
            sigZ = softfloat_shortShiftRightJam64(sig64Z, 32) as u32;
        }
        if sigZ < 0x4000_0000 {
            expZ = expZ.wrapping_sub(1);
            sigZ <<= 1;
        }
    } else {
        // ------------------------------------------------------------------------
        let sig64C = (sigC as u64) << 32;
        if (expDiff < 0) {
            signZ = signC;
            expZ = expC;
            sig64Z = sig64C.wrapping_sub(softfloat_shiftRightJam64(
                sigProd,
                expDiff.wrapping_neg() as u32,
            ));
        } else if expDiff == 0 {
            expZ = expProd;
            sig64Z = sigProd.wrapping_sub(sig64C);
            if 0 == sig64Z {
                return (packToF32((roundingMode == softfloat_round_min), 0, 0), 0);
            }
            if (sig64Z & 0x8000_0000_0000_0000) != 0 {
                signZ = !signZ;
                sig64Z = sig64Z.wrapping_neg();
            }
        } else {
            expZ = expProd;
            sig64Z = sigProd.wrapping_sub(softfloat_shiftRightJam64(sig64C, expDiff as u32));
        }
        let mut shiftDist: i8 = softfloat_countLeadingZeros64(sig64Z).wrapping_sub(1) as i8;
        expZ = expZ.wrapping_sub(shiftDist as i16);
        shiftDist = shiftDist.wrapping_sub(32);
        if (shiftDist < 0) {
            sigZ = softfloat_shortShiftRightJam64(sig64Z, shiftDist.wrapping_neg() as u8) as u32;
        } else {
            sigZ = (sig64Z as u32) << shiftDist;
        }
    }

    return softfloat_roundPackToF32(signZ, expZ, sigZ, roundingMode, detectTininess);
}
