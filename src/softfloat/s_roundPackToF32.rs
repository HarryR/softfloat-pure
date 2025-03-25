use super::{
    float32_t, packToF32, packToF32UI, softfloat_flag_inexact, softfloat_flag_overflow,
    softfloat_flag_underflow, softfloat_round_max, softfloat_round_min, softfloat_round_near_even,
    softfloat_round_near_maxMag, softfloat_round_odd, softfloat_shiftRightJam32,
    softfloat_tininess_beforeRounding,
};

#[must_use]
pub const fn softfloat_roundPackToF32(
    sign: bool,
    mut exp: i16,
    mut sig: u32,
    roundingMode: u8,
    detectTininess: u8,
) -> (float32_t, u8) {
    let mut flags: u8 = 0;
    let roundNearEven = roundingMode == softfloat_round_near_even;
    let mut roundIncrement: u8 = 0x40;

    if !roundNearEven && roundingMode != softfloat_round_near_maxMag {
        let x = if sign {
            softfloat_round_min
        } else {
            softfloat_round_max
        };

        roundIncrement = if roundingMode == x { 0x7F } else { 0 };
    }
    let mut roundBits = (sig & 0x7F) as u8;
    // ------------------------------------------------------------------------
    if 0xFD <= (exp as u32) {
        if exp < 0 {
            // ----------------------------------------------------------------
            let isTiny = (detectTininess == softfloat_tininess_beforeRounding)
                || (exp < -1)
                || (sig.wrapping_add(roundIncrement as u32) < 0x8000_0000);

            sig = softfloat_shiftRightJam32(sig, exp.wrapping_neg() as u16);
            exp = 0;
            roundBits = (sig & 0x7F) as u8;

            if isTiny && roundBits != 0 {
                flags |= softfloat_flag_underflow;
            }
        } else if (0xFD < exp) || (0x8000_0000 <= sig.wrapping_add(roundIncrement as u32)) {
            // ----------------------------------------------------------------
            flags |= (softfloat_flag_overflow | softfloat_flag_inexact);
            return (
                float32_t {
                    v: packToF32UI(sign, 0xFF, 0).wrapping_sub((roundIncrement == 0) as u32),
                },
                flags,
            );
        }
    }
    // ------------------------------------------------------------------------
    sig = sig.wrapping_add(roundIncrement as u32) >> 7;
    if roundBits != 0 {
        flags |= softfloat_flag_inexact;
        if roundingMode == softfloat_round_odd {
            sig |= 1;
            return (packToF32(sign, exp, sig), flags);
        }
    }
    sig &= !(((roundBits ^ 0x40) == 0) as u32 & (roundNearEven as u32));
    if sig == 0 {
        exp = 0;
    }
    // ----------------------------------------------------------------
    return (packToF32(sign, exp, sig), flags);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_softfloat_roundPackToF32() {
        struct softfloat_roundPackToF32_TestCase {
            roundingMode: u8,
            detectTininess: u8,
            sign: bool,
            exp: i16,
            sig: u32,
            ret: u32,
            flags: u8,
        }

        let cases = [
            softfloat_roundPackToF32_TestCase {
                roundingMode: 0,
                detectTininess: 1,
                sign: false,
                exp: 126,
                sig: 1074003968,
                ret: 1065355264,
                flags: 0,
            },
            softfloat_roundPackToF32_TestCase {
                roundingMode: 0,
                detectTininess: 1,
                sign: true,
                exp: 127,
                sig: 2013267894,
                ret: 3228565519,
                flags: 1,
            },
            softfloat_roundPackToF32_TestCase {
                roundingMode: 0,
                detectTininess: 1,
                sign: false,
                exp: 189,
                sig: 1237375556,
                ret: 1595113909,
                flags: 1,
            },
            softfloat_roundPackToF32_TestCase {
                roundingMode: 0,
                detectTininess: 1,
                sign: false,
                exp: 158,
                sig: 2139097087,
                ret: 1342111760,
                flags: 1,
            },
            softfloat_roundPackToF32_TestCase {
                roundingMode: 0,
                detectTininess: 0,
                sign: true,
                exp: 122,
                sig: 1593836544,
                ret: 3183345672,
                flags: 0,
            },
        ];

        for (i, case) in cases.iter().enumerate() {
            let (result, flags) = softfloat_roundPackToF32(
                case.sign,
                case.exp,
                case.sig,
                case.roundingMode,
                case.detectTininess,
            );
            assert_eq!((i, result.to_bits(), flags), (i, case.ret, case.flags));
        }
    }
}
