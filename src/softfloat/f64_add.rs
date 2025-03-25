use super::{float64_t, signF64UI, softfloat_addMagsF64, softfloat_subMagsF64};

#[must_use]
pub const fn f64_add(
    a: float64_t,
    b: float64_t,
    roundingMode: u8,
    detectTininess: u8,
) -> (float64_t, u8) {
    let signA = signF64UI(a.v);
    if signA == signF64UI(b.v) {
        return softfloat_addMagsF64(a.v, b.v, signA, roundingMode, detectTininess);
    }
    return softfloat_subMagsF64(a.v, b.v, signA, roundingMode, detectTininess);
}

#[cfg(test)]
mod tests {
    use super::super::{softfloat_round_near_even, softfloat_tininess_beforeRounding};
    use super::*;

    #[test]
    fn test_f64_add() {
        let (res, flags) = f64_add(
            float64_t { v: 0x1 },
            float64_t {
                v: 0x8010000000000000,
            },
            softfloat_round_near_even,
            softfloat_tininess_beforeRounding,
        );
        assert_eq!(flags, 0);
        assert_eq!(res.v, 0x800FFFFFFFFFFFFF);
    }
}
