use super::{float64_t, packToF64, softfloat_countLeadingZeros32};

#[must_use]
pub const fn i32_to_f64(a: i32) -> float64_t {
    if a == 0 {
        return float64_t { v: 0 };
    }

    let sign = a < 0;
    let absA = if sign {
        (a as u32).wrapping_neg()
    } else {
        a as u32
    };
    let shiftDist = softfloat_countLeadingZeros32(absA).wrapping_add(21) as i8;
    return packToF64(
        sign,
        (0x432 as i16).wrapping_sub(shiftDist as i16),
        ((absA as u64) << shiftDist),
    );
}
