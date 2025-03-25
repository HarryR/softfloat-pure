use super::{expF64UI, float64_t, fracF64UI, isNaNF64UI, signF64UI, softfloat_isSigNaNF64UI};

#[must_use]
pub const fn f64_classify(a: float64_t) -> u16 {
    let infOrNaN = expF64UI(a.v) == 0xFF;
    let subnormalOrZero = expF64UI(a.v) == 0;
    let sign = signF64UI(a.v);
    let fracZero = fracF64UI(a.v) == 0;
    let isNaN = isNaNF64UI(a.v);
    let isSNaN = softfloat_isSigNaNF64UI(a.v);

    return ((sign && infOrNaN && fracZero) as u16)
        | ((sign && !infOrNaN && !subnormalOrZero) as u16) << 1
        | ((sign && subnormalOrZero && !fracZero) as u16) << 2
        | ((sign && subnormalOrZero && fracZero) as u16) << 3
        | ((!sign && infOrNaN && fracZero) as u16) << 7
        | ((!sign && !infOrNaN && !subnormalOrZero) as u16) << 6
        | ((!sign && subnormalOrZero && !fracZero) as u16) << 5
        | ((!sign && subnormalOrZero && fracZero) as u16) << 4
        | ((isNaN && isSNaN) as u16) << 8
        | ((isNaN && !isSNaN) as u16) << 9;
}
