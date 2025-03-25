use super::{
    float64_t, packToF64, packToF64UI, softfloat_flag_inexact, softfloat_flag_overflow,
    softfloat_flag_underflow, softfloat_round_max, softfloat_round_min, softfloat_round_near_even,
    softfloat_round_near_maxMag, softfloat_round_odd, softfloat_shiftRightJam64,
    softfloat_tininess_beforeRounding,
};

#[must_use]
pub const fn softfloat_roundPackToF64(
    sign: bool,
    mut exp: i16,
    mut sig: u64,
    roundingMode: u8,
    detectTininess: u8,
) -> (float64_t, u8) {
    let mut flags: u8 = 0;
    let roundNearEven = roundingMode == softfloat_round_near_even;
    let mut roundIncrement: u16 = 0x200;

    if !roundNearEven && (roundingMode != softfloat_round_near_maxMag) {
        let x = if sign {
            softfloat_round_min
        } else {
            softfloat_round_max
        };
        roundIncrement = if roundingMode == x { 0x3FF } else { 0 };
    }
    let mut roundBits: u16 = (sig & 0x3FF) as u16;
    // ------------------------------------------------------------------------
    if 0x7FD <= (exp as u16) {
        if exp < 0 {
            // ----------------------------------------------------------------
            let isTiny = (detectTininess == softfloat_tininess_beforeRounding)
                || (exp < -1)
                || (sig.wrapping_add(roundIncrement as u64) < 0x8000_0000_0000_0000);
            sig = softfloat_shiftRightJam64(sig, exp.wrapping_neg() as u32);
            exp = 0;
            roundBits = (sig & 0x3FF) as u16;
            if isTiny && roundBits != 0 {
                flags |= softfloat_flag_underflow;
            }
        } else if (0x7FD < exp)
            || (0x8000_0000_0000_0000 <= sig.wrapping_add(roundIncrement as u64))
        {
            // ----------------------------------------------------------------
            flags |= softfloat_flag_overflow | softfloat_flag_inexact;
            return (
                float64_t {
                    v: packToF64UI(sign, 0x7FF, 0).wrapping_sub((roundIncrement == 0) as u64),
                },
                flags,
            );
        }
    }
    // ------------------------------------------------------------------------
    sig = sig.wrapping_add(roundIncrement as u64) >> 10;
    if roundBits != 0 {
        flags |= softfloat_flag_inexact;
        if roundingMode == softfloat_round_odd {
            sig |= 1;
            return (packToF64(sign, exp, sig), flags);
        }
    }
    sig &= !(((roundBits ^ 0x200) == 0) as u64 & (roundNearEven as u64));
    if sig == 0 {
        exp = 0;
    }
    // ------------------------------------------------------------------------
    return (packToF64(sign, exp, sig), flags);
}
