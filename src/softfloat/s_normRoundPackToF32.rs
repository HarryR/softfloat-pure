use super::{float32_t, packToF32, softfloat_countLeadingZeros32, softfloat_roundPackToF32};

#[must_use]
pub const fn softfloat_normRoundPackToF32(
    sign: bool,
    exp: i16,
    sig: u32,
    roundingMode: u8,
    detectTininess: u8,
) -> (float32_t, u8) {
    let shiftDist = softfloat_countLeadingZeros32(sig).wrapping_sub(1) as i8;
    let exp = exp.wrapping_sub(shiftDist as i16);
    if 7 <= shiftDist && (exp as u32) < 0xFD {
        return (
            packToF32(
                sign,
                if sig != 0 { exp } else { 0 },
                sig << shiftDist.wrapping_sub(7),
            ),
            0,
        );
    }
    return softfloat_roundPackToF32(sign, exp, sig << shiftDist, roundingMode, detectTininess);
}
