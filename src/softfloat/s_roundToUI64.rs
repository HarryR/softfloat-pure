use super::{
    softfloat_flag_inexact, softfloat_flag_invalid, softfloat_round_max, softfloat_round_min,
    softfloat_round_near_even, softfloat_round_near_maxMag, softfloat_round_odd,
    ui64_fromNegOverflow, ui64_fromPosOverflow,
};

#[must_use]
pub const fn softfloat_roundToUI64(
    sign: bool,
    mut sig: u64,
    sigExtra: u64,
    roundingMode: u8,
    exact: bool,
) -> (u64, u8) {
    let mut flags: u8 = 0;

    if roundingMode == softfloat_round_near_maxMag || roundingMode == softfloat_round_near_even {
        if 0x8000_0000_0000_0000 <= sigExtra {
            sig = sig.wrapping_add(1);
            if sig == 0 {
                flags |= softfloat_flag_invalid;
                return (
                    (if sign {
                        ui64_fromNegOverflow
                    } else {
                        ui64_fromPosOverflow
                    }),
                    flags,
                );
            }
            if sigExtra == 0x8000_0000_0000_0000 && roundingMode == softfloat_round_near_even {
                sig &= !(1 as u64);
            }
        }
    } else if sign {
        if 0 == (sig | sigExtra) {
            return (0, flags);
        }
        if roundingMode == softfloat_round_min || roundingMode == softfloat_round_odd {
            flags |= softfloat_flag_invalid;
            return (
                (if sign {
                    ui64_fromNegOverflow
                } else {
                    ui64_fromPosOverflow
                }),
                flags,
            );
        }
    } else if roundingMode == softfloat_round_max && sigExtra != 0 {
        sig = sig.wrapping_add(1);
        if sig == 0 {
            flags |= softfloat_flag_invalid;
            return (
                (if sign {
                    ui64_fromNegOverflow
                } else {
                    ui64_fromPosOverflow
                }),
                flags,
            );
        }
        if sigExtra == 0x8000_0000_0000_0000 && roundingMode == softfloat_round_near_even {
            sig &= !(1 as u64);
        }
    }

    if sign && sig != 0 {
        flags |= softfloat_flag_invalid;
        return (
            (if sign {
                ui64_fromNegOverflow
            } else {
                ui64_fromPosOverflow
            }),
            flags,
        );
    }

    if sigExtra != 0 {
        if roundingMode == softfloat_round_odd {
            sig |= 1;
        }

        if exact {
            flags |= softfloat_flag_inexact;
        }
    }

    return (sig, flags);
}
