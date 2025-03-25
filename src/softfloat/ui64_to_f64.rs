use super::{
    float64_t, softfloat_normRoundPackToF64, softfloat_roundPackToF64,
    softfloat_shortShiftRightJam64,
};

#[must_use]
pub const fn ui64_to_f64(a: u64, roundingMode: u8, detectTininess: u8) -> (float64_t, u8) {
    if a == 0 {
        return (float64_t { v: 0 }, 0);
    }
    if (a & 0x8000_0000_0000_0000) != 0 {
        return softfloat_roundPackToF64(
            0 as i32 != 0,
            0x43d,
            softfloat_shortShiftRightJam64(a, 1),
            roundingMode,
            detectTininess,
        );
    }
    softfloat_normRoundPackToF64(0 as i32 != 0, 0x43c, a, roundingMode, detectTininess)
}
