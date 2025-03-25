use super::{float32_t, packToF32UI, softfloat_normRoundPackToF32};

#[must_use]
pub const fn i32_to_f32(a: i32, roundingMode: u8, detectTininess: u8) -> (float32_t, u8) {
    let sign = a < 0;

    if a.trailing_zeros() >= 31 {
        return (
            float32_t {
                v: if sign { packToF32UI(true, 0x9E, 0) } else { 0 },
            },
            0,
        );
    }
    let absA = if sign {
        (a as u32).wrapping_neg()
    } else {
        a as u32
    };
    return softfloat_normRoundPackToF32(sign, 0x9C, absA, roundingMode, detectTininess);
}
