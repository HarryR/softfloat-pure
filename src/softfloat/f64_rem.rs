use super::{
    defaultNaNF64UI, expF64UI, float64_t, fracF64UI, signF64UI, softfloat_approxRecip32_1,
    softfloat_flag_invalid, softfloat_normRoundPackToF64, softfloat_normSubnormalF64Sig,
    softfloat_propagateNaNF64,
};

const fn selectRem(
    mut rem: u64,
    altRem: u64,
    signA: bool,
    expB: i16,
    q: u32,
    roundingMode: u8,
    detectTininess: u8,
) -> (float64_t, u8) {
    let meanRem = rem.wrapping_add(altRem);
    if ((meanRem & 0x8000_0000_0000_0000) != 0 || (meanRem == 0 && (q & 1) != 0)) {
        rem = altRem;
    }
    let mut signRem = signA;
    if (rem & 0x8000_0000_0000_0000) != 0 {
        signRem = !signRem;
        rem = rem.wrapping_neg();
    }
    return softfloat_normRoundPackToF64(signRem, expB, rem, roundingMode, detectTininess);
}

#[must_use]
pub const fn f64_rem(
    a: float64_t,
    b: float64_t,
    roundingMode: u8,
    detectTininess: u8,
) -> (float64_t, u8) {
    let signA = signF64UI(a.v);
    let mut expA = expF64UI(a.v);
    let mut sigA = fracF64UI(a.v);
    let mut expB = expF64UI(b.v);
    let mut sigB = fracF64UI(b.v);
    // ------------------------------------------------------------------------
    if expA == 0x7FF {
        if sigA != 0 || ((expB == 0x7FF) && sigB != 0) {
            return softfloat_propagateNaNF64(a.v, b.v);
        }
        // invalid
        return (float64_t { v: defaultNaNF64UI }, softfloat_flag_invalid);
    }
    if expB == 0x7FF {
        if sigB != 0 {
            return softfloat_propagateNaNF64(a.v, b.v);
        }
        return (a, 0);
    }
    // ------------------------------------------------------------------------
    if expA < expB.wrapping_sub(1) {
        return (a, 0);
    }
    // ------------------------------------------------------------------------
    if 0 == expB {
        if 0 == sigB {
            // invalid
            return (float64_t { v: defaultNaNF64UI }, softfloat_flag_invalid);
        }
        let normExpSig = softfloat_normSubnormalF64Sig(sigB);
        expB = normExpSig.exp;
        sigB = normExpSig.sig;
    }
    if 0 == expA {
        if 0 == sigA {
            return (a, 0);
        }
        let normExpSig = softfloat_normSubnormalF64Sig(sigA);
        expA = normExpSig.exp;
        sigA = normExpSig.sig;
    }
    // ------------------------------------------------------------------------
    let mut rem = sigA | 0x0010_0000_0000_0000;
    sigB |= 0x0010_0000_0000_0000;
    let mut q: u32;
    let mut expDiff = expA.wrapping_sub(expB);
    let mut altRem;
    if expDiff < 1 {
        if expDiff < -1 {
            return (a, 0);
        }
        sigB <<= 9;
        if expDiff != 0 {
            rem <<= 8;
            q = 0;
        } else {
            rem <<= 9;
            q = (sigB <= rem) as u32;
            if q != 0 {
                rem = rem.wrapping_sub(sigB);
            }
        }
    } else {
        let recip32 = softfloat_approxRecip32_1((sigB >> 21) as u32);
        // Changing the shift of `rem' here requires also changing the initial
        // subtraction from `expDiff'.
        rem <<= 9;
        expDiff = expDiff.wrapping_sub(30);
        // The scale of `sigB' affects how many bits are obtained during each
        // cycle of the loop.  Currently this is 29 bits per loop iteration,
        // the maximum possible.
        sigB <<= 9;
        let mut q64;
        loop {
            q64 = ((rem >> 32) as u32 as u64).wrapping_mul(recip32 as u64);
            if expDiff < 0 {
                break;
            }
            q = (q64.wrapping_add(0x8000_0000) >> 32) as u32;
            rem <<= 29;
            rem = rem.wrapping_sub((q as u64).wrapping_mul(sigB));
            if (rem & 0x8000_0000_0000_0000) != 0 {
                rem = rem.wrapping_add(sigB);
            }
            expDiff = expDiff.wrapping_sub(29);
        }
        // (`expDiff' cannot be less than -29 here.)
        q = ((q64 >> 32) as u32) >> (((!expDiff as i32) & 31) as u32);
        rem = (rem << (expDiff.wrapping_add(30))).wrapping_sub((q as u64).wrapping_mul(sigB));
        if (rem & 0x8000_0000_0000_0000) != 0 {
            altRem = rem.wrapping_add(sigB);
            return selectRem(rem, altRem, signA, expB, q, roundingMode, detectTininess);
        }
    }
    // ------------------------------------------------------------------------
    loop {
        altRem = rem;
        q = q.wrapping_add(1);
        rem = rem.wrapping_sub(sigB);

        if (rem & 0x8000_0000_0000_0000) != 0 {
            break;
        }
    }

    return selectRem(rem, altRem, signA, expB, q, roundingMode, detectTininess);
}
