use super::{
    float32_t, packToF32UI, softfloat_countLeadingZeros64, softfloat_roundPackToF32,
    softfloat_shortShiftRightJam64,
};

#[must_use]
pub const fn i64_to_f32(a: i64, roundingMode: u8, detectTininess: u8) -> (float32_t, u8) {
    let sign = a < 0;
    let absA = if sign {
        (a as u64).wrapping_neg()
    } else {
        a as u64
    };
    let mut shiftDist = softfloat_countLeadingZeros64(absA).wrapping_sub(40) as i8;
    if 0 <= shiftDist {
        return (
            float32_t {
                v: if a != 0 {
                    packToF32UI(
                        sign,
                        (0x95 as i16).wrapping_sub(shiftDist as i16),
                        (absA << shiftDist) as u32,
                    )
                } else {
                    0
                },
            },
            0,
        );
    }
    shiftDist = shiftDist.wrapping_add(7);
    let sig: u32 = if shiftDist < 0 {
        softfloat_shortShiftRightJam64(absA, shiftDist.wrapping_neg() as u8) as u32
    } else {
        (absA << shiftDist) as u32
    };
    return softfloat_roundPackToF32(
        sign,
        (0x9C as i16).wrapping_sub(shiftDist as i16),
        sig,
        roundingMode,
        detectTininess,
    );
}
