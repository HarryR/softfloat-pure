use super::{float64_t, packToF64, softfloat_countLeadingZeros64, softfloat_roundPackToF64};

#[must_use]
pub const fn softfloat_normRoundPackToF64(
    sign: bool,
    mut exp: i16,
    sig: u64,
    roundingMode: u8,
    detectTininess: u8,
) -> (float64_t, u8) {
    let shiftDist = (softfloat_countLeadingZeros64(sig) as i16).wrapping_sub(1);
    exp = exp.wrapping_sub(shiftDist);
    if (10 <= shiftDist) && (exp as u32) < 0x7fd {
        return (
            packToF64(
                sign,
                if sig != 0 { exp } else { 0 },
                sig << shiftDist.wrapping_sub(10),
            ),
            0,
        );
    }
    return softfloat_roundPackToF64(sign, exp, sig << shiftDist, roundingMode, detectTininess);
}

#[cfg(test)]
mod tests {
    use super::super::deconstruct_f64UI;
    use super::*;

    #[test]
    fn test_normRoundPackToF64() {
        struct softfloat_normRoundPackToF64_TestCase {
            sign: bool,
            exp: i16,
            sig: u64,
            result: u64,
            flags: u8,
            roundingMode: u8,
            detectTininess: u8,
        }

        let cases = [
            softfloat_normRoundPackToF64_TestCase {
                sign: false,
                exp: 1149,
                sig: 9223366539296634879,
                result: 5183643165734731774,
                flags: 1,
                roundingMode: 0,
                detectTininess: 0,
            },
            softfloat_normRoundPackToF64_TestCase {
                sign: false,
                exp: 1113,
                sig: 4647714802559343616,
                result: 5017045169250236407,
                flags: 0,
                roundingMode: 0,
                detectTininess: 0,
            },
            softfloat_normRoundPackToF64_TestCase {
                sign: true,
                exp: 1021,
                sig: 7732052489556386815,
                result: 13829098088909384633,
                flags: 1,
                roundingMode: 0,
                detectTininess: 0,
            },
            softfloat_normRoundPackToF64_TestCase {
                sign: true,
                exp: 1019,
                sig: 9223372036838026239,
                result: 13821547256400035867,
                flags: 1,
                roundingMode: 0,
                detectTininess: 0,
            },
            softfloat_normRoundPackToF64_TestCase {
                sign: true,
                exp: 1026,
                sig: 2305842906537129984,
                result: 13844065254135824376,
                flags: 0,
                roundingMode: 0,
                detectTininess: 0,
            },
            softfloat_normRoundPackToF64_TestCase {
                sign: false,
                exp: 1086,
                sig: 9223363240895969279,
                result: 4899916385989296126,
                flags: 1,
                roundingMode: 0,
                detectTininess: 0,
            },
            softfloat_normRoundPackToF64_TestCase {
                sign: false,
                exp: 1090,
                sig: 8935141660707258365,
                result: 4917649318111875072,
                flags: 1,
                roundingMode: 0,
                detectTininess: 0,
            },
            softfloat_normRoundPackToF64_TestCase {
                sign: false,
                exp: 1023,
                sig: 3613698337959330304,
                result: 4609736823738973729,
                flags: 0,
                roundingMode: 0,
                detectTininess: 0,
            },
            softfloat_normRoundPackToF64_TestCase {
                sign: true,
                exp: 1023,
                sig: 4629700417473740799,
                result: 13835075647468732416,
                flags: 1,
                roundingMode: 0,
                detectTininess: 0,
            },
            softfloat_normRoundPackToF64_TestCase {
                sign: false,
                exp: 1607,
                sig: 4611690417010769919,
                result: 7241788205107249152,
                flags: 1,
                roundingMode: 0,
                detectTininess: 0,
            },
            softfloat_normRoundPackToF64_TestCase {
                sign: true,
                exp: 1052,
                sig: 4611686017388248064,
                result: 13965662444473878526,
                flags: 0,
                roundingMode: 0,
                detectTininess: 0,
            },
            softfloat_normRoundPackToF64_TestCase {
                sign: false,
                exp: 1032,
                sig: 4573406521039257632,
                result: 4652143650430386176,
                flags: 1,
                roundingMode: 0,
                detectTininess: 0,
            },
            softfloat_normRoundPackToF64_TestCase {
                sign: true,
                exp: 2045,
                sig: 4611756387171434495,
                result: 18437736943174287232,
                flags: 1,
                roundingMode: 0,
                detectTininess: 0,
            },
            softfloat_normRoundPackToF64_TestCase {
                sign: true,
                exp: 1025,
                sig: 1835008,
                result: 13658291769907871744,
                flags: 0,
                roundingMode: 0,
                detectTininess: 0,
            },
            softfloat_normRoundPackToF64_TestCase {
                sign: false,
                exp: 1020,
                sig: 9223336835168599039,
                result: 4602678784796000255,
                flags: 1,
                roundingMode: 0,
                detectTininess: 0,
            },
        ];

        for (i, c) in cases.iter().enumerate() {
            let (res, flags) = softfloat_normRoundPackToF64(
                c.sign,
                c.exp,
                c.sig,
                c.roundingMode,
                c.detectTininess,
            );
            assert_eq!(
                (i, deconstruct_f64UI(res.v), flags),
                (i, deconstruct_f64UI(c.result), c.flags)
            );
        }
    }
}
