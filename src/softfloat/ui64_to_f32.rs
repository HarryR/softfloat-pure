use super::{
    float32_t, packToF32UI, softfloat_countLeadingZeros64, softfloat_roundPackToF32,
    softfloat_shortShiftRightJam64,
};

#[must_use]
pub const fn ui64_to_f32(a: u64, roundingMode: u8, detectTininess: u8) -> (float32_t, u8) {
    let mut shiftDist = (softfloat_countLeadingZeros64(a) as i8).wrapping_sub(40);
    if 0 <= shiftDist {
        (
            float32_t {
                v: if a != 0 {
                    packToF32UI(
                        false,
                        (0x95 as i16).wrapping_sub(shiftDist as i16),
                        (a << shiftDist) as u32,
                    )
                } else {
                    0
                },
            },
            0,
        )
    } else {
        shiftDist = shiftDist.wrapping_add(7);
        let sig = if shiftDist < 0 {
            softfloat_shortShiftRightJam64(a, shiftDist.wrapping_neg() as u8)
        } else {
            (a << shiftDist) as u64
        };
        softfloat_roundPackToF32(
            false,
            (0x9c as i16).wrapping_sub(shiftDist as i16),
            sig as u32,
            roundingMode,
            detectTininess,
        )
    }
}
