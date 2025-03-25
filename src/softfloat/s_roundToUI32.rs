use super::{
    softfloat_flag_inexact, softfloat_flag_invalid, softfloat_round_max, softfloat_round_min,
    softfloat_round_near_even, softfloat_round_near_maxMag, softfloat_round_odd,
    ui32_fromNegOverflow, ui32_fromPosOverflow,
};

#[must_use]
pub const fn softfloat_roundToUI32(
    sign: bool,
    mut sig: u64,
    roundingMode: u8,
    exact: bool,
) -> (u32, u8) {
    let mut flags: u8 = 0;

    let mut roundIncrement: u16 = 0x800;
    if roundingMode != softfloat_round_near_maxMag && roundingMode != softfloat_round_near_even {
        roundIncrement = 0;
        if sign {
            if sig == 0 {
                return (0, flags);
            }

            if roundingMode == softfloat_round_min || roundingMode == softfloat_round_odd {
                flags |= softfloat_flag_invalid;
                return (
                    if sign {
                        ui32_fromNegOverflow
                    } else {
                        ui32_fromPosOverflow
                    },
                    flags,
                );
            }
        } else if roundingMode == softfloat_round_max {
            roundIncrement = 0xFFF;
        }
    }

    let roundBits: u16 = (sig & 0xFFF) as u16;

    sig = sig.wrapping_add(roundIncrement as u64);

    if (sig & 0xFFFF_F000_0000_0000) != 0 {
        flags |= softfloat_flag_invalid;
        return (
            if sign {
                ui32_fromNegOverflow
            } else {
                ui32_fromPosOverflow
            },
            flags,
        );
    }

    let mut z: u32 = (sig >> 12) as u32;

    if roundBits == 0x800 && roundingMode == softfloat_round_near_even {
        z &= !(1 as u32);
    }

    if sign && z != 0 {
        flags |= softfloat_flag_invalid;
        return (
            if sign {
                ui32_fromNegOverflow
            } else {
                ui32_fromPosOverflow
            },
            flags,
        );
    }

    if roundBits != 0 {
        if roundingMode == softfloat_round_odd {
            z |= 1;
        }
        if exact {
            flags |= softfloat_flag_inexact;
        }
    }

    return (z, flags);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_softfloat_roundToUI32() {
        struct softfloat_roundToUI32_TestCase {
            sign: bool,
            sig: u64,
            roundingMode: u8,
            exact: bool,
            ret: u32,
            flags: u8,
        }

        let cases = [
            softfloat_roundToUI32_TestCase {
                sign: false,
                sig: 36028801313931264,
                roundingMode: 0,
                exact: true,
                ret: 4294967295,
                flags: 16,
            },
            softfloat_roundToUI32_TestCase {
                sign: true,
                sig: 4289509195776,
                roundingMode: 0,
                exact: true,
                ret: 0,
                flags: 16,
            },
            softfloat_roundToUI32_TestCase {
                sign: true,
                sig: 4095,
                roundingMode: 0,
                exact: true,
                ret: 0,
                flags: 16,
            },
            softfloat_roundToUI32_TestCase {
                sign: false,
                sig: 72057589742960640,
                roundingMode: 0,
                exact: true,
                ret: 4294967295,
                flags: 16,
            },
            softfloat_roundToUI32_TestCase {
                sign: false,
                sig: 4398049132544,
                roundingMode: 0,
                exact: true,
                ret: 1073742464,
                flags: 0,
            },
            softfloat_roundToUI32_TestCase {
                sign: false,
                sig: 141836731547648,
                roundingMode: 0,
                exact: true,
                ret: 4294967295,
                flags: 16,
            },
            softfloat_roundToUI32_TestCase {
                sign: false,
                sig: 72057585447993344,
                roundingMode: 0,
                exact: true,
                ret: 4294967295,
                flags: 16,
            },
        ];

        for (i, case) in cases.iter().enumerate() {
            let (result, flags) =
                softfloat_roundToUI32(case.sign, case.sig, case.roundingMode, case.exact);
            assert_eq!((i, result, flags), (i, case.ret, case.flags));
        }
    }
}
