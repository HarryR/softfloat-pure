use super::{
    defaultNaNF32UI, expF32UI, float32_t, fracF32UI, packToF32, signF32UI,
    softfloat_countLeadingZeros32, softfloat_flag_invalid, softfloat_normRoundPackToF32,
    softfloat_propagateNaNF32, softfloat_round_min, softfloat_shiftRightJam32,
};

#[must_use]
pub const fn softfloat_subMagsF32(
    uiA: u32,
    uiB: u32,
    roundingMode: u8,
    detectTininess: u8,
) -> (float32_t, u8) {
    let mut expA = expF32UI(uiA);
    let mut sigA = fracF32UI(uiA);
    let mut expB = expF32UI(uiB);
    let mut sigB = fracF32UI(uiB);
    // ------------------------------------------------------------------------
    let mut expDiff = expA.wrapping_sub(expB);
    if expDiff == 0 {
        // --------------------------------------------------------------------
        if expA == 0xff {
            if (sigA | sigB) != 0 {
                return softfloat_propagateNaNF32(uiA, uiB);
            }
            return (float32_t { v: defaultNaNF32UI }, softfloat_flag_invalid);
        }
        let mut sigDiff = sigA.wrapping_sub(sigB) as i32;
        if sigDiff == 0 {
            return (packToF32(roundingMode == softfloat_round_min, 0, 0), 0);
        }
        if expA != 0 {
            expA = expA.wrapping_sub(1);
        }
        let mut signZ = signF32UI(uiA);
        if sigDiff < 0 {
            signZ = !signZ;
            sigDiff = sigDiff.wrapping_neg();
        }
        let mut shiftDist = (softfloat_countLeadingZeros32(sigDiff as u32) as i8).wrapping_sub(8);
        let mut expZ = expA.wrapping_sub(shiftDist as i16);
        if expZ < 0 {
            shiftDist = expA as i8;
            expZ = 0;
        }
        return (packToF32(signZ, expZ, (sigDiff << shiftDist) as u32), 0);
    }
    // --------------------------------------------------------------------
    let mut sigX;
    let mut sigY;
    let mut expZ;
    let mut signZ = signF32UI(uiA);
    sigA <<= 7;
    sigB <<= 7;
    if expDiff < 0 {
        // --------------------------------------------------------------------
        signZ = !signZ;
        if expB == 0xff {
            if sigB != 0 {
                return softfloat_propagateNaNF32(uiA, uiB);
            }
            return (packToF32(signZ, 0xFF, 0), 0);
        }
        expZ = expB.wrapping_sub(1);
        sigX = sigB | 0x4000_0000;
        sigY = sigA.wrapping_add(if expA != 0 { 0x4000_0000 } else { sigA });
        expDiff = expDiff.wrapping_neg();
    } else {
        // --------------------------------------------------------------------
        if expA == 0xFF {
            if sigA != 0 {
                return softfloat_propagateNaNF32(uiA, uiB);
            }
            return (float32_t { v: uiA }, 0);
        }
        expZ = expA.wrapping_sub(1);
        sigX = sigA | 0x4000_0000;
        sigY = sigB.wrapping_add(if expB != 0 { 0x4000_0000 } else { sigB });
    }

    return softfloat_normRoundPackToF32(
        signZ,
        expZ,
        sigX.wrapping_sub(softfloat_shiftRightJam32(sigY, expDiff as u16)),
        roundingMode,
        detectTininess,
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_subMagsF32() {
        struct softfloat_subMagsF32_TestCase {
            uiA: u32,
            uiB: u32,
            result: u32,
            flags: u8,
            roundingMode: u8,
            detectTininess: u8,
        }

        let cases = [
            softfloat_subMagsF32_TestCase {
                uiA: 2155888703,
                uiB: 864026752,
                result: 864026752,
                flags: 1,
                roundingMode: 0,
                detectTininess: 1,
            },
            softfloat_subMagsF32_TestCase {
                uiA: 2164392191,
                uiB: 1333858303,
                result: 1333858303,
                flags: 1,
                roundingMode: 0,
                detectTininess: 1,
            },
            softfloat_subMagsF32_TestCase {
                uiA: 2510770523,
                uiB: 1325691792,
                result: 1325691792,
                flags: 1,
                roundingMode: 0,
                detectTininess: 1,
            },
            softfloat_subMagsF32_TestCase {
                uiA: 2152980832,
                uiB: 1090585087,
                result: 1090585087,
                flags: 1,
                roundingMode: 0,
                detectTininess: 1,
            },
            softfloat_subMagsF32_TestCase {
                uiA: 2189422589,
                uiB: 872284671,
                result: 872284671,
                flags: 1,
                roundingMode: 0,
                detectTininess: 1,
            },
            softfloat_subMagsF32_TestCase {
                uiA: 2169983265,
                uiB: 1035993095,
                result: 1035993095,
                flags: 1,
                roundingMode: 0,
                detectTininess: 1,
            },
            softfloat_subMagsF32_TestCase {
                uiA: 2095865596,
                uiB: 3220963199,
                result: 2095865596,
                flags: 1,
                roundingMode: 0,
                detectTininess: 1,
            },
            softfloat_subMagsF32_TestCase {
                uiA: 1040187551,
                uiB: 3196076160,
                result: 3187703905,
                flags: 0,
                roundingMode: 0,
                detectTininess: 1,
            },
            softfloat_subMagsF32_TestCase {
                uiA: 3481550847,
                uiB: 1274937279,
                result: 3481485823,
                flags: 1,
                roundingMode: 0,
                detectTininess: 1,
            },
            softfloat_subMagsF32_TestCase {
                uiA: 3481142272,
                uiB: 1085485534,
                result: 3481142272,
                flags: 1,
                roundingMode: 0,
                detectTininess: 1,
            },
            softfloat_subMagsF32_TestCase {
                uiA: 1042350078,
                uiB: 3196059649,
                result: 3183345672,
                flags: 0,
                roundingMode: 0,
                detectTininess: 0,
            },
            softfloat_subMagsF32_TestCase {
                uiA: 16777214,
                uiB: 3229614081,
                result: 3229614081,
                flags: 1,
                roundingMode: 0,
                detectTininess: 0,
            },
            softfloat_subMagsF32_TestCase {
                uiA: 8388608,
                uiB: 3229614081,
                result: 3229614081,
                flags: 1,
                roundingMode: 0,
                detectTininess: 0,
            },
            softfloat_subMagsF32_TestCase {
                uiA: 8388608,
                uiB: 3238002687,
                result: 3238002687,
                flags: 1,
                roundingMode: 0,
                detectTininess: 0,
            },
            softfloat_subMagsF32_TestCase {
                uiA: 3472875523,
                uiB: 1040809734,
                result: 3472875523,
                flags: 1,
                roundingMode: 0,
                detectTininess: 0,
            },
            softfloat_subMagsF32_TestCase {
                uiA: 8388608,
                uiB: 3238002686,
                result: 3238002686,
                flags: 1,
                roundingMode: 0,
                detectTininess: 0,
            },
            softfloat_subMagsF32_TestCase {
                uiA: 1107362046,
                uiB: 3456200771,
                result: 3456200770,
                flags: 1,
                roundingMode: 0,
                detectTininess: 0,
            },
            softfloat_subMagsF32_TestCase {
                uiA: 8388608,
                uiB: 3489628159,
                result: 3489628159,
                flags: 1,
                roundingMode: 0,
                detectTininess: 0,
            },
            softfloat_subMagsF32_TestCase {
                uiA: 3211967580,
                uiB: 130023426,
                result: 3211967580,
                flags: 1,
                roundingMode: 0,
                detectTininess: 0,
            },
            softfloat_subMagsF32_TestCase {
                uiA: 8388608,
                uiB: 3414163456,
                result: 3414163456,
                flags: 1,
                roundingMode: 0,
                detectTininess: 0,
            },
            softfloat_subMagsF32_TestCase {
                uiA: 8388608,
                uiB: 3414163457,
                result: 3414163457,
                flags: 1,
                roundingMode: 0,
                detectTininess: 0,
            },
            softfloat_subMagsF32_TestCase {
                uiA: 3481276407,
                uiB: 1132064867,
                result: 3481276407,
                flags: 1,
                roundingMode: 0,
                detectTininess: 0,
            },
            softfloat_subMagsF32_TestCase {
                uiA: 8388608,
                uiB: 3189586526,
                result: 3189586526,
                flags: 1,
                roundingMode: 0,
                detectTininess: 0,
            },
            softfloat_subMagsF32_TestCase {
                uiA: 1082122231,
                uiB: 3750435462,
                result: 3750435462,
                flags: 1,
                roundingMode: 0,
                detectTininess: 0,
            },
            softfloat_subMagsF32_TestCase {
                uiA: 8388608,
                uiB: 3422552063,
                result: 3422552063,
                flags: 1,
                roundingMode: 0,
                detectTininess: 0,
            },
            softfloat_subMagsF32_TestCase {
                uiA: 3735010656,
                uiB: 1329594360,
                result: 3735010656,
                flags: 1,
                roundingMode: 0,
                detectTininess: 0,
            },
            softfloat_subMagsF32_TestCase {
                uiA: 8388608,
                uiB: 3186382848,
                result: 3186382848,
                flags: 1,
                roundingMode: 0,
                detectTininess: 0,
            },
            softfloat_subMagsF32_TestCase {
                uiA: 1694892030,
                uiB: 3170500608,
                result: 1694892030,
                flags: 1,
                roundingMode: 0,
                detectTininess: 0,
            },
            softfloat_subMagsF32_TestCase {
                uiA: 8388608,
                uiB: 3422552062,
                result: 3422552062,
                flags: 1,
                roundingMode: 0,
                detectTininess: 0,
            },
            softfloat_subMagsF32_TestCase {
                uiA: 2122842144,
                uiB: 3188193279,
                result: 2122842144,
                flags: 1,
                roundingMode: 0,
                detectTininess: 0,
            },
            softfloat_subMagsF32_TestCase {
                uiA: 8388608,
                uiB: 3422519326,
                result: 3422519326,
                flags: 1,
                roundingMode: 0,
                detectTininess: 0,
            },
        ];

        for (i, c) in cases.iter().enumerate() {
            let (res, flags) = softfloat_subMagsF32(c.uiA, c.uiB, c.roundingMode, c.detectTininess);
            assert_eq!((i, res.v, flags), (i, c.result, c.flags));
        }
    }
}
