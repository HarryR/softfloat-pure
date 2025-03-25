use super::{
    expF32UI, float32_t, fracF32UI, packToF32UI, signF32UI, softfloat_flag_inexact,
    softfloat_propagateNaNF32, softfloat_round_max, softfloat_round_min, softfloat_round_near_even,
    softfloat_round_near_maxMag, softfloat_round_odd,
};

#[must_use]
pub const fn f32_roundToInt(a: float32_t, roundingMode: u8, exact: bool) -> (float32_t, u8) {
    let exp = expF32UI(a.v);
    let mut flags: u8 = 0;
    if exp <= 0x7e {
        if (a.v << 1) == 0 {
            return (a, flags);
        }
        if exact {
            flags |= softfloat_flag_inexact;
        }
        let mut uiZ = a.v & packToF32UI(true, 0, 0);
        match roundingMode {
            softfloat_round_near_even => {
                if fracF32UI(a.v) != 0 && exp == 0x7E {
                    uiZ |= packToF32UI(false, 0x7F, 0);
                }
            }
            softfloat_round_near_maxMag => {
                if exp == 0x7E {
                    uiZ |= packToF32UI(false, 0x7F, 0);
                }
            }
            softfloat_round_min => {
                if uiZ != 0 {
                    uiZ = packToF32UI(true, 0x7F, 0);
                }
            }
            softfloat_round_max => {
                if uiZ == 0 {
                    uiZ = packToF32UI(false, 0x7F, 0);
                }
            }
            softfloat_round_odd => {
                uiZ |= packToF32UI(false, 0x7F, 0);
            }
            _ => { /* do nothing... */ }
        }
        return (float32_t { v: uiZ }, flags);
    }

    if 0x96 <= exp {
        if exp == 0xFF && fracF32UI(a.v) != 0 {
            let (ret, new_flags) = softfloat_propagateNaNF32(a.v, 0);
            return (ret, flags | new_flags);
        }
        return (a, flags);
    }

    let mut uiZ = a.v;
    let lastBitMask = (1 as u32) << (0x96 as i16).wrapping_sub(exp);
    let roundBitsMask = lastBitMask.wrapping_sub(1);
    if roundingMode == softfloat_round_near_maxMag {
        uiZ = uiZ.wrapping_add(lastBitMask >> 1);
    } else if roundingMode == softfloat_round_near_even {
        uiZ = uiZ.wrapping_add(lastBitMask >> 1);
        if (uiZ & roundBitsMask) == 0 {
            uiZ &= !lastBitMask;
        }
    } else if roundingMode
        == (if signF32UI(uiZ) {
            softfloat_round_min
        } else {
            softfloat_round_max
        })
    {
        uiZ = uiZ.wrapping_add(roundBitsMask);
    }

    uiZ &= !roundBitsMask;

    if uiZ != a.v {
        if roundingMode == softfloat_round_odd {
            uiZ |= lastBitMask;
        }
        if exact {
            flags |= softfloat_flag_inexact;
        }
    }

    return (float32_t { v: uiZ }, flags);
}
