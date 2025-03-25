use super::{
    i64_fromNegOverflow, i64_fromPosOverflow, softfloat_flag_inexact, softfloat_flag_invalid,
    softfloat_round_max, softfloat_round_min, softfloat_round_near_even,
    softfloat_round_near_maxMag, softfloat_round_odd,
};

#[must_use]
pub const fn softfloat_roundToI64(
    sign: bool,
    mut sig: u64,
    sigExtra: u64,
    roundingMode: u8,
    exact: bool,
) -> (i64, u8) {
    if roundingMode == softfloat_round_near_maxMag || roundingMode == softfloat_round_near_even {
        if 0x8000_0000_0000_0000 <= sigExtra {
            // increment {
            sig = sig.wrapping_add(1);
            if 0 == sig {
                return (
                    if sign {
                        i64_fromNegOverflow
                    } else {
                        i64_fromPosOverflow
                    },
                    softfloat_flag_invalid,
                );
            }
            if sigExtra == 0x8000_0000_0000_0000 && roundingMode == softfloat_round_near_even {
                sig &= !(1 as u64);
            }
            //}
        }
    } else {
        let x = if sign {
            roundingMode == softfloat_round_min || roundingMode == softfloat_round_odd
        } else {
            roundingMode == softfloat_round_max
        };

        if sigExtra != 0 && x {
            // increment {
            sig = sig.wrapping_add(1);
            if 0 == sig {
                return (
                    if sign {
                        i64_fromNegOverflow
                    } else {
                        i64_fromPosOverflow
                    },
                    softfloat_flag_invalid,
                );
            }
            if sigExtra == 0x8000_0000_0000_0000 && roundingMode == softfloat_round_near_even {
                sig &= !(1 as u64);
            }
            //}
        }
    }

    let mut z = (if sign { sig.wrapping_neg() } else { sig }) as i64;
    if z != 0 && ((z < 0) ^ sign) {
        return (
            if sign {
                i64_fromNegOverflow
            } else {
                i64_fromPosOverflow
            },
            softfloat_flag_invalid,
        );
    }

    let mut flags: u8 = 0;

    if sigExtra != 0 {
        if roundingMode == softfloat_round_odd {
            z |= 1;
        }
        if exact {
            flags |= softfloat_flag_inexact;
        }
    }

    (z, flags)
}
