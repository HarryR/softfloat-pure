use super::{
    defaultNaNF64UI, expF64UI, float64_t, fracF64UI, packToF64, signF64UI,
    softfloat_approxRecip32_1, softfloat_flag_infinite, softfloat_flag_invalid,
    softfloat_normSubnormalF64Sig, softfloat_propagateNaNF64, softfloat_roundPackToF64,
};

#[must_use]
pub const fn f64_div(
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
    // ------------------------------------------------------------------------
    if expA == 0x7FF {
        if sigA != 0 {
            return softfloat_propagateNaNF64(a.v, b.v);
        }
        if (expB == 0x7FF) {
            if sigB != 0 {
                return softfloat_propagateNaNF64(a.v, b.v);
            }
            // invalid
            return (float64_t { v: defaultNaNF64UI }, softfloat_flag_invalid);
        }
        // infinity
        return (packToF64(signZ, 0x7FF, 0), 0);
    }
    if (expB == 0x7FF) {
        if sigB != 0 {
            return softfloat_propagateNaNF64(a.v, b.v);
        }
        // zero
        return (packToF64(signZ, 0, 0), 0);
    }
    // ------------------------------------------------------------------------
    if 0 == expB {
        if 0 == sigB {
            if 0 == ((expA as u64) | sigA) {
                // invalid
                return (float64_t { v: defaultNaNF64UI }, softfloat_flag_invalid);
            }
            // infinity
            return (packToF64(signZ, 0x7FF, 0), softfloat_flag_infinite);
        }
        let normExpSig = softfloat_normSubnormalF64Sig(sigB);
        expB = normExpSig.exp;
        sigB = normExpSig.sig;
    }
    if 0 == expA {
        if 0 == sigA {
            // zero
            return (packToF64(signZ, 0, 0), 0);
        }
        let normExpSig = softfloat_normSubnormalF64Sig(sigA);
        expA = normExpSig.exp;
        sigA = normExpSig.sig;
    }
    // ------------------------------------------------------------------------
    let mut expZ = expA.wrapping_sub(expB).wrapping_add(0x3FE);
    sigA |= 0x0010_0000_0000_0000;
    sigB |= 0x0010_0000_0000_0000;
    if sigA < sigB {
        expZ = expZ.wrapping_sub(1);
        sigA <<= 11;
    } else {
        sigA <<= 10;
    }
    sigB <<= 11;
    let recip32 = softfloat_approxRecip32_1((sigB >> 32) as u32).wrapping_sub(2);
    let sig32Z = (((sigA >> 32).wrapping_mul(recip32 as u64) >> 32) as u32);
    let mut doubleTerm = sig32Z.wrapping_shl(1);
    let mut rem = ((sigA.wrapping_sub((doubleTerm as u64).wrapping_mul(sigB >> 32)))
        .wrapping_shl(28))
    .wrapping_sub((doubleTerm as u64).wrapping_mul((sigB >> 4) & 0x0FFF_FFFF));
    let mut q: u32 = (((rem >> 32).wrapping_mul(recip32 as u64) >> 32) as u32).wrapping_add(4);
    let mut sigZ = ((sig32Z as u64).wrapping_shl(32)).wrapping_add((q as u64).wrapping_shl(4));
    // ------------------------------------------------------------------------
    if (sigZ & 0x1FF) < (4 << 4) {
        q &= !(7 as u32);
        sigZ &= !(0x7F as u64);
        doubleTerm = q << 1;
        rem = (rem.wrapping_sub((doubleTerm as u64).wrapping_mul((sigB >> 32) as u32 as u64))
            << 28)
            .wrapping_sub((doubleTerm as u64).wrapping_mul((sigB as u32 >> 4) as u64));
        if (rem & 0x8000_0000_0000_0000) != 0 {
            sigZ = sigZ.wrapping_sub(1 << 7);
        } else if rem != 0 {
            sigZ |= 1;
        }
    }
    return softfloat_roundPackToF64(signZ, expZ, sigZ, roundingMode, detectTininess);
}

#[cfg(test)]
mod tests {
    use super::super::{softfloat_round_near_even, softfloat_tininess_beforeRounding};
    use super::*;

    #[test]
    fn test_f64_div() {
        let (res, flags) = f64_div(
            float64_t {
                v: 0x50E0100000001000,
            },
            float64_t {
                v: 0x3CA0000000000000,
            },
            softfloat_round_near_even,
            softfloat_tininess_beforeRounding,
        );
        assert_eq!(res.v, 0x5430100000001000);
        assert_eq!(flags, 0);
    }
}
