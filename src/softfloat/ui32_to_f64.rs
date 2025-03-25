use super::{float64_t, packToF64, softfloat_countLeadingZeros32};

#[must_use]
pub const fn ui32_to_f64(a: u32) -> float64_t {
    if a == 0 {
        float64_t { v: 0 }
    } else {
        let shiftDist = softfloat_countLeadingZeros32(a).wrapping_add(21) as i8;
        packToF64(
            false,
            (0x432 as i16).wrapping_sub(shiftDist as i16),
            (a as u64) << shiftDist,
        )
    }
}
