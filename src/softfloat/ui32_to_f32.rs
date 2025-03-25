use super::{
    float32_t, softfloat_flag_inexact, softfloat_normRoundPackToF32, softfloat_roundPackToF32,
};

#[must_use]
pub const fn ui32_to_f32(a: u32, roundingMode: u8, detectTininess: u8) -> (float32_t, u8) {
    if a == 0 {
        (float32_t { v: 0 }, 0)
    } else if (a & 0x8000_0000) != 0 {
        softfloat_roundPackToF32(false, 0x9d, (a >> 1 | a & 1), roundingMode, detectTininess)
    } else {
        softfloat_normRoundPackToF32(false, 0x9c, a, roundingMode, detectTininess)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[allow(static_mut_refs)]
    #[test]
    fn test_ui32_to_f32_inexact_flag() {
        struct TestCase {
            input: u32,
            result: u32,
            flags: u8,
        }
        for test in [
            TestCase {
                input: 0xFF7FFF02,
                result: 0x4F7F7FFF,
                flags: softfloat_flag_inexact,
            },
            TestCase {
                input: 0x03FE007F,
                result: 0x4C7F8020,
                flags: softfloat_flag_inexact,
            },
        ]
        .iter()
        {
            let (result, flags) = ui32_to_f32(test.input, 0, 0);
            assert_eq!(result.v, test.result);
            assert_eq!(flags, test.flags);
        }
    }
}
