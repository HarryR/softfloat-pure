use super::{
    expF64UI, float64_t, fracF64UI, packToF64UI, signF64UI, softfloat_flag_inexact,
    softfloat_propagateNaNF64, softfloat_round_max, softfloat_round_min, softfloat_round_near_even,
    softfloat_round_near_maxMag, softfloat_round_odd,
};

#[must_use]
pub const fn f64_roundToInt(a: float64_t, roundingMode: u8, exact: bool) -> (float64_t, u8) {
    let exp = expF64UI(a.v);

    if exp <= 0x3FE {
        let mut flags: u8 = 0;

        if a.v.trailing_zeros() >= 63 {
            return (a, 0);
        }
        if exact {
            flags |= softfloat_flag_inexact;
        }
        let mut uiZ = a.v & packToF64UI(true, 0, 0);
        match roundingMode {
            softfloat_round_near_even => {
                if fracF64UI(a.v) != 0 && exp == 0x3FE {
                    uiZ |= packToF64UI(false, 0x3FF, 0);
                }
            }
            softfloat_round_near_maxMag => {
                if exp == 0x3FE {
                    uiZ |= packToF64UI(false, 0x3FF, 0);
                }
            }
            softfloat_round_min => {
                if uiZ != 0 {
                    uiZ = packToF64UI(true, 0x3FF, 0);
                }
            }
            softfloat_round_max => {
                if uiZ == 0 {
                    uiZ = packToF64UI(false, 0x3FF, 0);
                }
            }
            softfloat_round_odd => {
                uiZ |= packToF64UI(false, 0x3FF, 0);
            }
            _ => { /* do nothing... */ }
        }
        return (float64_t { v: uiZ }, flags);
    }

    if 0x433 <= exp {
        if exp == 0x7FF && fracF64UI(a.v) != 0 {
            return softfloat_propagateNaNF64(a.v, 0);
        }
        return (a, 0);
    }

    let mut uiZ = a.v;
    let lastBitMask = (1 as u64) << (0x433 as i16).wrapping_sub(exp);
    let roundBitsMask = lastBitMask.wrapping_sub(1);
    if roundingMode == softfloat_round_near_maxMag {
        uiZ = uiZ.wrapping_add(lastBitMask >> 1);
    } else if roundingMode == softfloat_round_near_even {
        uiZ = uiZ.wrapping_add(lastBitMask >> 1);
        if (uiZ & roundBitsMask) == 0 {
            uiZ &= !lastBitMask;
        }
    } else if roundingMode
        == (if signF64UI(uiZ) {
            softfloat_round_min
        } else {
            softfloat_round_max
        })
    {
        uiZ = uiZ.wrapping_add(roundBitsMask);
    }

    uiZ &= !roundBitsMask;

    let mut flags: u8 = 0;
    if uiZ != a.v {
        if roundingMode == softfloat_round_odd {
            uiZ |= lastBitMask;
        }
        if exact {
            flags |= softfloat_flag_inexact;
        }
    }

    return (float64_t { v: uiZ }, flags);
}
