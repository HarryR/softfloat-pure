use super::{float32_t, signF32UI, softfloat_addMagsF32, softfloat_subMagsF32};

#[inline]
#[must_use]
pub const fn f32_add(
    a: float32_t,
    b: float32_t,
    roundingMode: u8,
    detectTininess: u8,
) -> (float32_t, u8) {
    if signF32UI(a.v ^ b.v) {
        softfloat_subMagsF32(a.v, b.v, roundingMode, detectTininess)
    } else {
        softfloat_addMagsF32(a.v, b.v, roundingMode, detectTininess)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_f32_add() {
        struct softfloat_f32_add_TestCase {
            a: u32,
            b: u32,
            result: u32,
            flags: u8,
            roundingMode: u8,
            detectTininess: u8,
        }

        let cases = [
            softfloat_f32_add_TestCase {
                a: 0xF53CBE,
                b: 0xD3CB86,
                result: 0x1648422,
                flags: 0,
                roundingMode: 0,
                detectTininess: 1,
            },
            softfloat_f32_add_TestCase {
                a: 0,
                b: 1065353248,
                result: 1065353248,
                flags: 0,
                roundingMode: 0,
                detectTininess: 1,
            },
            softfloat_f32_add_TestCase {
                a: 3438280704,
                b: 3254763487,
                result: 3438280708,
                flags: 1,
                roundingMode: 0,
                detectTininess: 1,
            },
            softfloat_f32_add_TestCase {
                a: 0,
                b: 1065353280,
                result: 1065353280,
                flags: 0,
                roundingMode: 0,
                detectTininess: 1,
            },
            softfloat_f32_add_TestCase {
                a: 973073408,
                b: 1040187391,
                result: 1040220150,
                flags: 1,
                roundingMode: 0,
                detectTininess: 1,
            },
            softfloat_f32_add_TestCase {
                a: 0,
                b: 1065353344,
                result: 1065353344,
                flags: 0,
                roundingMode: 0,
                detectTininess: 1,
            },
            softfloat_f32_add_TestCase {
                a: 2189422589,
                b: 872284671,
                result: 872284671,
                flags: 1,
                roundingMode: 0,
                detectTininess: 1,
            },
            softfloat_f32_add_TestCase {
                a: 0,
                b: 1065353472,
                result: 1065353472,
                flags: 0,
                roundingMode: 0,
                detectTininess: 1,
            },
            softfloat_f32_add_TestCase {
                a: 2169983265,
                b: 1035993095,
                result: 1035993095,
                flags: 1,
                roundingMode: 0,
                detectTininess: 1,
            },
            softfloat_f32_add_TestCase {
                a: 0,
                b: 1065353728,
                result: 1065353728,
                flags: 0,
                roundingMode: 0,
                detectTininess: 1,
            },
            softfloat_f32_add_TestCase {
                a: 2095865596,
                b: 3220963199,
                result: 2095865596,
                flags: 1,
                roundingMode: 0,
                detectTininess: 1,
            },
            softfloat_f32_add_TestCase {
                a: 0,
                b: 1065354240,
                result: 1065354240,
                flags: 0,
                roundingMode: 0,
                detectTininess: 1,
            },
            softfloat_f32_add_TestCase {
                a: 1040187551,
                b: 3196076160,
                result: 3187703905,
                flags: 0,
                roundingMode: 0,
                detectTininess: 1,
            },
            softfloat_f32_add_TestCase {
                a: 0,
                b: 1065355264,
                result: 1065355264,
                flags: 0,
                roundingMode: 0,
                detectTininess: 1,
            },
            softfloat_f32_add_TestCase {
                a: 1549822242,
                b: 1594884112,
                result: 1595113909,
                flags: 1,
                roundingMode: 0,
                detectTininess: 1,
            },
            softfloat_f32_add_TestCase {
                a: 0,
                b: 1065357312,
                result: 1065357312,
                flags: 0,
                roundingMode: 0,
                detectTininess: 1,
            },
            softfloat_f32_add_TestCase {
                a: 1317003260,
                b: 977338367,
                result: 1317003260,
                flags: 1,
                roundingMode: 0,
                detectTininess: 1,
            },
            softfloat_f32_add_TestCase {
                a: 0,
                b: 1065361408,
                result: 1065361408,
                flags: 0,
                roundingMode: 0,
                detectTininess: 1,
            },
            softfloat_f32_add_TestCase {
                a: 3481550847,
                b: 1274937279,
                result: 3481485823,
                flags: 1,
                roundingMode: 0,
                detectTininess: 1,
            },
            softfloat_f32_add_TestCase {
                a: 0,
                b: 1065369600,
                result: 1065369600,
                flags: 0,
                roundingMode: 0,
                detectTininess: 1,
            },
            softfloat_f32_add_TestCase {
                a: 3221094403,
                b: 3061645311,
                result: 3221094435,
                flags: 1,
                roundingMode: 0,
                detectTininess: 1,
            },
            softfloat_f32_add_TestCase {
                a: 0,
                b: 1065385984,
                result: 1065385984,
                flags: 0,
                roundingMode: 0,
                detectTininess: 1,
            },
            softfloat_f32_add_TestCase {
                a: 3221225215,
                b: 3272867838,
                result: 3272998908,
                flags: 1,
                roundingMode: 0,
                detectTininess: 1,
            },
            softfloat_f32_add_TestCase {
                a: 0,
                b: 1065418752,
                result: 1065418752,
                flags: 0,
                roundingMode: 0,
                detectTininess: 1,
            },
            softfloat_f32_add_TestCase {
                a: 3481142272,
                b: 1085485534,
                result: 3481142272,
                flags: 1,
                roundingMode: 0,
                detectTininess: 1,
            },
            softfloat_f32_add_TestCase {
                a: 0,
                b: 1065484288,
                result: 1065484288,
                flags: 0,
                roundingMode: 0,
                detectTininess: 1,
            },
            softfloat_f32_add_TestCase {
                a: 3164476669,
                b: 3422519424,
                result: 3422519424,
                flags: 1,
                roundingMode: 0,
                detectTininess: 1,
            },
            softfloat_f32_add_TestCase {
                a: 0,
                b: 1065615360,
                result: 1065615360,
                flags: 0,
                roundingMode: 0,
                detectTininess: 1,
            },
            softfloat_f32_add_TestCase {
                a: 3195799551,
                b: 4286582272,
                result: 2143289344,
                flags: 16,
                roundingMode: 0,
                detectTininess: 1,
            },
        ];

        for (i, c) in cases.iter().enumerate() {
            let (res, flags) = f32_add(
                float32_t { v: c.a },
                float32_t { v: c.b },
                c.roundingMode,
                c.detectTininess,
            );
            assert_eq!((i, res.v, flags), (i, c.result, c.flags));
        }
    }
}
