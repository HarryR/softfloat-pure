use super::{
    defaultNaNF32UI, expF32UI, float32_t, fracF32UI, signF32UI, softfloat_approxRecip32_1,
    softfloat_flag_invalid, softfloat_normRoundPackToF32, softfloat_normSubnormalF32Sig,
    softfloat_propagateNaNF32,
};

#[must_use]
pub const fn f32_rem(
    a: float32_t,
    b: float32_t,
    roundingMode: u8,
    detectTininess: u8,
) -> (float32_t, u8) {
    let signA = signF32UI(a.v);
    let mut expA = expF32UI(a.v);
    let mut sigA = fracF32UI(a.v);

    let mut expB = expF32UI(b.v);
    let mut sigB = fracF32UI(b.v);

    // ------------------------------------------------------------------------

    if expA == 0xFF {
        if (sigA != 0) || ((expB == 0xFF) && (sigB != 0)) {
            return softfloat_propagateNaNF32(a.v, b.v);
        }
        // invalid
        return (float32_t { v: defaultNaNF32UI }, softfloat_flag_invalid);
    }
    if expB == 0xFF {
        if sigB != 0 {
            return softfloat_propagateNaNF32(a.v, b.v);
        }
        return (a, 0);
    }

    // ------------------------------------------------------------------------

    if expB == 0 {
        if sigB == 0 {
            // invalid
            return (float32_t { v: defaultNaNF32UI }, softfloat_flag_invalid);
        }
        let normExpSig = softfloat_normSubnormalF32Sig(sigB);
        expB = normExpSig.exp;
        sigB = normExpSig.sig;
    }
    if expA == 0 {
        if sigA == 0 {
            return (a, 0);
        }
        let normExpSig = softfloat_normSubnormalF32Sig(sigA);
        expA = normExpSig.exp;
        sigA = normExpSig.sig;
    }

    // ------------------------------------------------------------------------

    let mut rem = sigA | 0x0080_0000;
    let mut q: u32;
    sigB |= 0x0080_0000;
    let mut expDiff = expA.wrapping_sub(expB);
    if expDiff < 1 {
        if expDiff < -1 {
            return (a, 0);
        }
        sigB <<= 6;
        if expDiff != 0 {
            rem <<= 5;
            q = 0;
        } else {
            rem <<= 6;
            q = (sigB <= rem) as u32;
            if q != 0 {
                rem = rem.wrapping_sub(sigB);
            }
        }
    } else {
        let recip32 = softfloat_approxRecip32_1(sigB << 8);
        // Changing the shift of `rem' here requires also changing the initial
        // subtraction from `expDiff'.
        rem <<= 7;
        expDiff = expDiff.wrapping_sub(31);
        // The scale of `sigB' affects how many bits are obtained during each
        // cycle of the loop.  Currently this is 29 bits per loop iteration,
        // which is believed to be the maximum possible.
        sigB <<= 6;
        loop {
            q = ((rem as u64).wrapping_mul(recip32 as u64) >> 32) as u32;
            if expDiff < 0 {
                break;
            }
            rem = q.wrapping_mul(sigB).wrapping_neg();
            expDiff = expDiff.wrapping_sub(29);
        }
        // (`expDiff' cannot be less than -30 here.)
        q >>= (!(expDiff as u16)) & 31;
        rem = (rem.wrapping_shl((expDiff as u32).wrapping_add(30)))
            .wrapping_sub(q.wrapping_mul(sigB));
    }

    // ------------------------------------------------------------------------

    let mut altRem: u32;
    loop {
        altRem = rem;
        q = q.wrapping_add(1);
        rem = rem.wrapping_sub(sigB);

        if ((rem & 0x8000_0000) != 0) {
            break;
        }
    }
    let meanRem = rem.wrapping_add(altRem);
    if (meanRem & 0x8000_0000) != 0 || (meanRem == 0 && ((q & 1) != 0)) {
        rem = altRem;
    }
    let mut signRem = signA;
    if 0x8000_0000 <= rem {
        signRem = !signRem;
        rem = rem.wrapping_neg();
    }

    return softfloat_normRoundPackToF32(signRem, expB, rem, roundingMode, detectTininess);
}
