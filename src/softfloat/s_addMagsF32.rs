use super::{
    expF32UI, float32_t, fracF32UI, packToF32, signF32UI, softfloat_propagateNaNF32,
    softfloat_roundPackToF32, softfloat_shiftRightJam32,
};

#[must_use]
pub const fn softfloat_addMagsF32(
    uiA: u32,
    uiB: u32,
    roundingMode: u8,
    detectTininess: u8,
) -> (float32_t, u8) {
    // ------------------------------------------------------------------------
    let expA = expF32UI(uiA);
    let sigA = fracF32UI(uiA);
    let expB = expF32UI(uiB);
    let sigB = fracF32UI(uiB);
    // ------------------------------------------------------------------------
    let expDiff = expA.wrapping_sub(expB);
    if expDiff == 0 {
        // ------------------------------------------------------------------------
        if expA == 0 {
            return (
                float32_t {
                    v: uiA.wrapping_add(sigB),
                },
                0,
            );
        }
        if expA == 0xFF {
            if (sigA | sigB) != 0 {
                return softfloat_propagateNaNF32(uiA, uiB);
            }
            return (float32_t { v: uiA }, 0);
        }
        let signZ = signF32UI(uiA);
        let expZ = expA;
        let mut sigZ = (0x0100_0000 as u32).wrapping_add(sigA).wrapping_add(sigB);
        if 0 == (sigZ & 1) && (expZ < 0xFE) {
            return (packToF32(signZ, expZ, sigZ >> 1), 0);
        }
        sigZ <<= 6;
        return softfloat_roundPackToF32(signZ, expZ, sigZ, roundingMode, detectTininess);
    }
    let signZ = signF32UI(uiA);
    let mut sigA = sigA << 6;
    let mut sigB = sigB << 6;
    let mut expZ;
    if expDiff < 0 {
        if expB == 0xFF {
            if sigB != 0 {
                return softfloat_propagateNaNF32(uiA, uiB);
            }
            return (packToF32(signZ, 0xFF, 0), 0);
        }
        expZ = expB;
        sigA = sigA.wrapping_add(if expA != 0 { 0x2000_0000 } else { sigA });
        sigA = softfloat_shiftRightJam32(sigA, expDiff.wrapping_neg() as u16);
    } else {
        if expA == 0xFF {
            if sigA != 0 {
                return softfloat_propagateNaNF32(uiA, uiB);
            }
            return (float32_t { v: uiA }, 0);
        }
        expZ = expA;
        sigB = sigB.wrapping_add(if expB != 0 { 0x2000_0000 } else { sigB });
        sigB = softfloat_shiftRightJam32(sigB, expDiff as u16);
    }
    let mut sigZ = (0x2000_0000 as u32).wrapping_add(sigA).wrapping_add(sigB);
    if sigZ < 0x4000_0000 {
        expZ = expZ.wrapping_sub(1);
        sigZ <<= 1;
    }
    return softfloat_roundPackToF32(signZ, expZ, sigZ, roundingMode, detectTininess);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_addMagsF32() {
        struct softfloat_addMagsF32_TestCase {
            uiA: u32,
            uiB: u32,
            result: u32,
            flags: u8,
            roundingMode: u8,
            detectTininess: u8,
        }

        let cases = [
            softfloat_addMagsF32_TestCase {
                uiA: 0xF53CBE,
                uiB: 0xD3CB86,
                result: 0x1648422,
                flags: 0,
                roundingMode: 0,
                detectTininess: 1,
            },
            softfloat_addMagsF32_TestCase {
                uiA: 0,
                uiB: 1065357312,
                result: 1065357312,
                flags: 0,
                roundingMode: 0,
                detectTininess: 1,
            },
            softfloat_addMagsF32_TestCase {
                uiA: 1317003260,
                uiB: 977338367,
                result: 1317003260,
                flags: 1,
                roundingMode: 0,
                detectTininess: 1,
            },
            softfloat_addMagsF32_TestCase {
                uiA: 0,
                uiB: 1065361408,
                result: 1065361408,
                flags: 0,
                roundingMode: 0,
                detectTininess: 1,
            },
            softfloat_addMagsF32_TestCase {
                uiA: 0,
                uiB: 1065369600,
                result: 1065369600,
                flags: 0,
                roundingMode: 0,
                detectTininess: 1,
            },
            softfloat_addMagsF32_TestCase {
                uiA: 3221094403,
                uiB: 3061645311,
                result: 3221094435,
                flags: 1,
                roundingMode: 0,
                detectTininess: 1,
            },
            softfloat_addMagsF32_TestCase {
                uiA: 0,
                uiB: 1065385984,
                result: 1065385984,
                flags: 0,
                roundingMode: 0,
                detectTininess: 1,
            },
            softfloat_addMagsF32_TestCase {
                uiA: 3221225215,
                uiB: 3272867838,
                result: 3272998908,
                flags: 1,
                roundingMode: 0,
                detectTininess: 1,
            },
            softfloat_addMagsF32_TestCase {
                uiA: 0,
                uiB: 1065418752,
                result: 1065418752,
                flags: 0,
                roundingMode: 0,
                detectTininess: 1,
            },
            softfloat_addMagsF32_TestCase {
                uiA: 0,
                uiB: 1065484288,
                result: 1065484288,
                flags: 0,
                roundingMode: 0,
                detectTininess: 1,
            },
            softfloat_addMagsF32_TestCase {
                uiA: 3164476669,
                uiB: 3422519424,
                result: 3422519424,
                flags: 1,
                roundingMode: 0,
                detectTininess: 1,
            },
            softfloat_addMagsF32_TestCase {
                uiA: 0,
                uiB: 1065615360,
                result: 1065615360,
                flags: 0,
                roundingMode: 0,
                detectTininess: 1,
            },
            softfloat_addMagsF32_TestCase {
                uiA: 3195799551,
                uiB: 4286582272,
                result: 2143289344,
                flags: 16,
                roundingMode: 0,
                detectTininess: 1,
            },
            softfloat_addMagsF32_TestCase {
                uiA: 1275068319,
                uiB: 1602224416,
                result: 1602224416,
                flags: 1,
                roundingMode: 0,
                detectTininess: 0,
            },
            softfloat_addMagsF32_TestCase {
                uiA: 3237986815,
                uiB: 3245473792,
                result: 3250122880,
                flags: 1,
                roundingMode: 0,
                detectTininess: 0,
            },
            softfloat_addMagsF32_TestCase {
                uiA: 8388608,
                uiB: 1037683764,
                result: 1037683764,
                flags: 1,
                roundingMode: 0,
                detectTininess: 0,
            },
            softfloat_addMagsF32_TestCase {
                uiA: 3244955497,
                uiB: 3085163116,
                result: 3244955525,
                flags: 1,
                roundingMode: 0,
                detectTininess: 0,
            },
            softfloat_addMagsF32_TestCase {
                uiA: 3481288687,
                uiB: 3238002687,
                result: 3481288687,
                flags: 1,
                roundingMode: 0,
                detectTininess: 0,
            },
            softfloat_addMagsF32_TestCase {
                uiA: 3207429484,
                uiB: 3732963391,
                result: 3732963391,
                flags: 1,
                roundingMode: 0,
                detectTininess: 0,
            },
            softfloat_addMagsF32_TestCase {
                uiA: 3732895742,
                uiB: 3091185663,
                result: 3732895742,
                flags: 1,
                roundingMode: 0,
                detectTininess: 0,
            },
            softfloat_addMagsF32_TestCase {
                uiA: 8388608,
                uiB: 293603327,
                result: 293603327,
                flags: 1,
                roundingMode: 0,
                detectTininess: 0,
            },
            softfloat_addMagsF32_TestCase {
                uiA: 3193700618,
                uiB: 3238002686,
                result: 3238227971,
                flags: 1,
                roundingMode: 0,
                detectTininess: 0,
            },
            softfloat_addMagsF32_TestCase {
                uiA: 3178578052,
                uiB: 3323531915,
                result: 3323531976,
                flags: 1,
                roundingMode: 0,
                detectTininess: 0,
            },
            softfloat_addMagsF32_TestCase {
                uiA: 1308752895,
                uiB: 1308225179,
                result: 1316942668,
                flags: 1,
                roundingMode: 0,
                detectTininess: 0,
            },
            softfloat_addMagsF32_TestCase {
                uiA: 3143352398,
                uiB: 3414163456,
                result: 3414163456,
                flags: 1,
                roundingMode: 0,
                detectTininess: 0,
            },
            softfloat_addMagsF32_TestCase {
                uiA: 1587491890,
                uiB: 568311808,
                result: 1587491890,
                flags: 1,
                roundingMode: 0,
                detectTininess: 0,
            },
            softfloat_addMagsF32_TestCase {
                uiA: 8388608,
                uiB: 1488646091,
                result: 1488646091,
                flags: 1,
                roundingMode: 0,
                detectTininess: 0,
            },
            softfloat_addMagsF32_TestCase {
                uiA: 2712490840,
                uiB: 2255487231,
                result: 2712490840,
                flags: 1,
                roundingMode: 0,
                detectTininess: 0,
            },
            softfloat_addMagsF32_TestCase {
                uiA: 3472753664,
                uiB: 3414163457,
                result: 3472884224,
                flags: 1,
                roundingMode: 0,
                detectTininess: 0,
            },
            softfloat_addMagsF32_TestCase {
                uiA: 4286562310,
                uiB: 3244294655,
                result: 4286562310,
                flags: 1,
                roundingMode: 0,
                detectTininess: 0,
            },
            softfloat_addMagsF32_TestCase {
                uiA: 2533355552,
                uiB: 3208675327,
                result: 3208675327,
                flags: 1,
                roundingMode: 0,
                detectTininess: 0,
            },
            softfloat_addMagsF32_TestCase {
                uiA: 2715988417,
                uiB: 3422552063,
                result: 3422552063,
                flags: 1,
                roundingMode: 0,
                detectTininess: 0,
            },
            softfloat_addMagsF32_TestCase {
                uiA: 3204452360,
                uiB: 3422552062,
                result: 3422552062,
                flags: 1,
                roundingMode: 0,
                detectTininess: 0,
            },
            softfloat_addMagsF32_TestCase {
                uiA: 3244419195,
                uiB: 3771634588,
                result: 3771634588,
                flags: 1,
                roundingMode: 0,
                detectTininess: 0,
            },
        ];

        for (i, c) in cases.iter().enumerate() {
            let (res, flags) = softfloat_addMagsF32(c.uiA, c.uiB, c.roundingMode, c.detectTininess);
            assert_eq!((i, res.v, flags), (i, c.result, c.flags));
        }
    }
}
