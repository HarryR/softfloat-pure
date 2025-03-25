use super::{
    i32_fromNegOverflow, i32_fromPosOverflow, softfloat_flag_inexact, softfloat_flag_invalid,
    softfloat_round_max, softfloat_round_min, softfloat_round_near_even,
    softfloat_round_near_maxMag, softfloat_round_odd,
};

#[must_use]
pub const fn softfloat_roundToI32(
    sign: bool,
    mut sig: u64,
    roundingMode: u8,
    exact: bool,
) -> (i32, u8) {
    let mut roundIncrement: u16 = 0x800;

    if roundingMode != softfloat_round_near_maxMag && roundingMode != softfloat_round_near_even {
        roundIncrement = 0;

        let x = if sign {
            roundingMode == softfloat_round_min || roundingMode == softfloat_round_odd
        } else {
            roundingMode == softfloat_round_max
        };

        if x {
            roundIncrement = 0xFFF;
        }
    }

    let roundBits = sig & 0xFFF;

    sig += roundIncrement as u64;

    if 0 != (sig & 0xFFFF_F000_0000_0000) {
        return (
            if sign {
                i32_fromNegOverflow
            } else {
                i32_fromPosOverflow
            },
            softfloat_flag_invalid,
        );
    }

    let mut sig32 = (sig >> 12) as u32;

    if roundBits == 0x800 && roundingMode == softfloat_round_near_even {
        sig32 &= !(1 as u32);
    }

    let mut z: i32 = if sign {
        sig32.wrapping_neg() as i32
    } else {
        sig32 as i32
    };

    if 0 != z && ((z < 0) ^ sign) {
        return (
            if sign {
                i32_fromNegOverflow
            } else {
                i32_fromPosOverflow
            },
            softfloat_flag_invalid,
        );
    }

    let mut flags: u8 = 0;

    if 0 != roundBits {
        if softfloat_round_odd == roundingMode {
            z |= 1;
        }

        if exact {
            flags = softfloat_flag_inexact;
        }
    }

    return (z, flags);
}
