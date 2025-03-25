use super::{
    defaultNaNF64UI, expF64UI, float64_t, fracF64UI, packToF64, softfloat_countLeadingZeros64,
    softfloat_flag_invalid, softfloat_normRoundPackToF64, softfloat_propagateNaNF64,
    softfloat_round_min, softfloat_shiftRightJam64,
};

#[must_use]
pub const fn softfloat_subMagsF64(
    uiA: u64,
    uiB: u64,
    mut signZ: bool,
    roundingMode: u8,
    detectTininess: u8,
) -> (float64_t, u8) {
    // ------------------------------------------------------------------------
    let mut expA = expF64UI(uiA);
    let mut sigA = fracF64UI(uiA);
    let expB = expF64UI(uiB);
    let mut sigB = fracF64UI(uiB);
    // ------------------------------------------------------------------------
    let expDiff = expA.wrapping_sub(expB);
    if expDiff == 0 {
        // --------------------------------------------------------------------
        if expA == 0x7FF {
            if (sigA | sigB) != 0 {
                return softfloat_propagateNaNF64(uiA, uiB);
            }
            return (float64_t { v: defaultNaNF64UI }, softfloat_flag_invalid);
        }
        let mut sigDiff = sigA.wrapping_sub(sigB) as i64;
        if sigDiff == 0 {
            return (packToF64(roundingMode == softfloat_round_min, 0, 0), 0);
        }
        if expA != 0 {
            expA = expA.wrapping_sub(1);
        }
        if sigDiff < 0 {
            signZ = !signZ;
            sigDiff = sigDiff.wrapping_neg();
        }
        let mut shiftDist = softfloat_countLeadingZeros64(sigDiff as u64).wrapping_sub(11) as i8;
        let mut expZ = expA.wrapping_sub(shiftDist as i16);
        if expZ < 0 {
            shiftDist = expA as i8;
            expZ = 0;
        }
        return (packToF64(signZ, expZ, (sigDiff << shiftDist) as u64), 0);
    }
    // --------------------------------------------------------------------
    let expZ;
    let sigZ;
    sigA <<= 10;
    sigB <<= 10;
    if expDiff < 0 {
        // ----------------------------------------------------------------
        signZ = !signZ;
        if expB == 0x7FF {
            if sigB != 0 {
                return softfloat_propagateNaNF64(uiA, uiB);
            }
            return (packToF64(signZ, 0x7FF, 0), 0);
        }
        sigA = sigA.wrapping_add(if expA != 0 {
            0x4000_0000_0000_0000
        } else {
            sigA
        });
        sigA = softfloat_shiftRightJam64(sigA, expDiff.wrapping_neg() as u16 as u32);
        sigB |= 0x4000_0000_0000_0000;
        expZ = expB;
        sigZ = sigB.wrapping_sub(sigA);
    } else {
        // ----------------------------------------------------------------
        if expA == 0x7FF {
            if sigA != 0 {
                return softfloat_propagateNaNF64(uiA, uiB);
            }
            return (float64_t { v: uiA }, 0);
        }
        sigB = sigB.wrapping_add(if expB != 0 {
            0x4000_0000_0000_0000
        } else {
            sigB
        });
        sigB = softfloat_shiftRightJam64(sigB, expDiff as u32);
        sigA |= 0x4000_0000_0000_0000;
        expZ = expA;
        sigZ = sigA.wrapping_sub(sigB);
    }

    return softfloat_normRoundPackToF64(
        signZ,
        expZ.wrapping_sub(1),
        sigZ,
        roundingMode,
        detectTininess,
    );
}
