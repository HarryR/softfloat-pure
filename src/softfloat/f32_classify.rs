use super::{expF32UI, float32_t, fracF32UI, isNaNF32UI, signF32UI, softfloat_isSigNaNF32UI};

#[must_use]
pub const fn f32_classify(a: float32_t) -> u16 {
    let infOrNaN = expF32UI(a.v) == 0xFF;
    let subnormalOrZero = expF32UI(a.v) == 0;
    let sign = signF32UI(a.v);
    let fracZero = fracF32UI(a.v) == 0;
    let isNaN = isNaNF32UI(a.v);
    let isSNaN = softfloat_isSigNaNF32UI(a.v);

    return ((sign && infOrNaN && fracZero) as u16)
        | (((sign && !infOrNaN && !subnormalOrZero) as u16) << 1)
        | (((sign && subnormalOrZero && !fracZero) as u16) << 2)
        | (((sign && subnormalOrZero && fracZero) as u16) << 3)
        | (((!sign && infOrNaN && fracZero) as u16) << 7)
        | (((!sign && !infOrNaN && !subnormalOrZero) as u16) << 6)
        | (((!sign && subnormalOrZero && !fracZero) as u16) << 5)
        | (((!sign && subnormalOrZero && fracZero) as u16) << 4)
        | (((isNaN && isSNaN) as u16) << 8)
        | (((isNaN && !isSNaN) as u16) << 9);
}
