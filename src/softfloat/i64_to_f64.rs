use super::{float64_t, packToF64, softfloat_normRoundPackToF64};

#[must_use]
pub const fn i64_to_f64(a: i64, roundingMode: u8, detectTininess: u8) -> (float64_t, u8) {
    let sign = a < 0;
    if (a as u64).trailing_zeros() >= 63 {
        return (
            if sign {
                packToF64(true, 0x43E, 0)
            } else {
                float64_t { v: 0 }
            },
            0,
        );
    }
    let absA = if sign as i32 != 0 {
        (a as u64).wrapping_neg()
    } else {
        a as u64
    };
    return softfloat_normRoundPackToF64(
        sign,
        0x43c as i32 as i16,
        absA,
        roundingMode,
        detectTininess,
    );
}
