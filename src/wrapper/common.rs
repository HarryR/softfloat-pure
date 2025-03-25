// SPDX-License-Identifier: MIT OR Apache-2.0

use core::borrow::Borrow;
use core::cmp::Ordering;
use core::fmt::{LowerHex, UpperHex};
use num_traits::{
    identities::{One, Zero},
    PrimInt,
};

use super::super::{
    float32_t, float64_t,
    softfloat::{
        softfloat_flag_inexact, softfloat_flag_infinite, softfloat_flag_invalid,
        softfloat_flag_overflow, softfloat_flag_underflow, softfloat_round_max,
        softfloat_round_min, softfloat_round_minMag, softfloat_round_near_even,
        softfloat_round_near_maxMag, softfloat_round_odd, softfloat_tininess_afterRounding,
        softfloat_tininess_beforeRounding,
    },
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TininessMode {
    Before,
    After,
}

impl TininessMode {
    #[inline]
    #[must_use]
    pub const fn to_softfloat(&self) -> u8 {
        match self {
            Self::Before => softfloat_tininess_beforeRounding,
            Self::After => softfloat_tininess_afterRounding,
        }
    }
}

/// floating-point rounding mode defined by standard
#[derive(Copy, Clone, Debug)]
pub enum RoundingMode {
    /// to nearest, ties to even
    RneTiesToEven,
    /// toward 0
    RtzTowardZero,
    /// toward −∞
    RdnTowardNegative,
    /// toward +∞
    RupTowardPositive,
    /// to nearest, ties away from zero
    RmmTiesToAway,
    // odd (jamming) For rounding to an integer value, rounds to minimum magnitude instead.
    Rodd,
}

impl RoundingMode {
    #[inline]
    #[must_use]
    pub const fn to_softfloat(&self) -> u8 {
        match self {
            Self::RneTiesToEven => softfloat_round_near_even,
            Self::RtzTowardZero => softfloat_round_minMag,
            Self::RdnTowardNegative => softfloat_round_min,
            Self::RupTowardPositive => softfloat_round_max,
            Self::RmmTiesToAway => softfloat_round_near_maxMag,
            Self::Rodd => softfloat_round_odd,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct ExceptionFlags(u8);

impl ExceptionFlags {
    const FLAG_INEXACT: u8 = softfloat_flag_inexact;
    const FLAG_UNDERFLOW: u8 = softfloat_flag_underflow;
    const FLAG_OVERFLOW: u8 = softfloat_flag_overflow;
    const FLAG_INFINITE: u8 = softfloat_flag_infinite;
    const FLAG_INVALID: u8 = softfloat_flag_invalid;

    #[inline]
    #[must_use]
    pub const fn from_bits(x: u8) -> Self {
        Self(x)
    }

    #[inline]
    #[must_use]
    pub const fn to_bits(&self) -> u8 {
        self.0
    }

    #[inline]
    #[must_use]
    pub const fn is_inexact(&self) -> bool {
        self.0 & Self::FLAG_INEXACT != 0
    }

    #[inline]
    #[must_use]
    pub const fn is_infinite(&self) -> bool {
        self.0 & Self::FLAG_INFINITE != 0
    }

    #[inline]
    #[must_use]
    pub const fn is_invalid(&self) -> bool {
        self.0 & Self::FLAG_INVALID != 0
    }

    #[inline]
    #[must_use]
    pub const fn is_overflow(&self) -> bool {
        self.0 & Self::FLAG_OVERFLOW != 0
    }

    #[inline]
    #[must_use]
    pub const fn is_underflow(&self) -> bool {
        self.0 & Self::FLAG_UNDERFLOW != 0
    }

    #[inline]
    pub const fn reset(&mut self) {
        self.0 = 0;
    }

    #[inline]
    pub const fn merge(&mut self, flags: u8) {
        self.0 |= flags;
    }
}

impl Default for ExceptionFlags {
    #[inline]
    fn default() -> Self {
        Self(0)
    }
}

pub trait Float: Sized {
    type Payload: PrimInt + UpperHex + LowerHex;

    const EXPONENT_BIT: Self::Payload;
    const FRACTION_BIT: Self::Payload;
    const SIGN_POS: usize;
    const EXPONENT_POS: usize;

    fn set_payload(&mut self, x: Self::Payload);

    fn from_bits(v: Self::Payload) -> Self;

    fn to_bits(&self) -> Self::Payload;

    fn add<T: Borrow<Self>>(&self, x: T, rnd: RoundingMode, detect_tininess: u8) -> (Self, u8);

    fn sub<T: Borrow<Self>>(&self, x: T, rnd: RoundingMode, detect_tininess: u8) -> (Self, u8);

    fn mul<T: Borrow<Self>>(&self, x: T, rnd: RoundingMode, detect_tininess: u8) -> (Self, u8);

    fn fused_mul_add<T: Borrow<Self>>(
        &self,
        x: T,
        y: T,
        rnd: RoundingMode,
        detect_tininess: u8,
    ) -> (Self, u8);

    fn div<T: Borrow<Self>>(&self, x: T, rnd: RoundingMode, detect_tininess: u8) -> (Self, u8);

    fn rem<T: Borrow<Self>>(&self, x: T, rnd: RoundingMode, detect_tininess: u8) -> (Self, u8);

    fn sqrt(&self, rnd: RoundingMode, detect_tininess: u8) -> (Self, u8);

    fn eq<T: Borrow<Self>>(&self, x: T) -> (bool, u8);

    fn lt<T: Borrow<Self>>(&self, x: T) -> (bool, u8);

    fn le<T: Borrow<Self>>(&self, x: T) -> (bool, u8);

    fn lt_quiet<T: Borrow<Self>>(&self, x: T) -> (bool, u8);

    fn le_quiet<T: Borrow<Self>>(&self, x: T) -> (bool, u8);

    fn eq_signaling<T: Borrow<Self>>(&self, x: T) -> (bool, u8);

    fn is_signaling_nan(&self) -> bool;

    fn from_u32(x: u32, rnd: RoundingMode, detect_tininess: u8) -> (Self, u8);

    fn from_u64(x: u64, rnd: RoundingMode, detect_tininess: u8) -> (Self, u8);

    fn from_i32(x: i32, rnd: RoundingMode, detect_tininess: u8) -> (Self, u8);

    fn from_i64(x: i64, rnd: RoundingMode, detect_tininess: u8) -> (Self, u8);

    fn to_u32(&self, rnd: RoundingMode, exact: bool) -> (u32, u8);

    fn to_u64(&self, rnd: RoundingMode, exact: bool) -> (u64, u8);

    fn to_i32(&self, rnd: RoundingMode, exact: bool) -> (i32, u8);

    fn to_i64(&self, rnd: RoundingMode, exact: bool) -> (i64, u8);

    fn to_f32(&self, rnd: RoundingMode, detect_tininess: u8) -> (float32_t, u8);

    fn to_f64(&self, rnd: RoundingMode, detect_tininess: u8) -> (float64_t, u8);

    fn round_to_integral(&self, rnd: RoundingMode, exact: bool) -> (Self, u8);

    #[inline]
    fn compare<T: Borrow<Self>>(&self, x: T) -> Option<Ordering> {
        let (eq, _) = self.eq(x.borrow());
        let (lt, _) = self.lt(x.borrow());
        if self.is_nan() || x.borrow().is_nan() {
            None
        } else if eq {
            Some(Ordering::Equal)
        } else if lt {
            Some(Ordering::Less)
        } else {
            Some(Ordering::Greater)
        }
    }

    #[inline]
    #[must_use]
    fn neg(&self) -> Self {
        let mut ret = Self::from_bits(self.to_bits());
        ret.set_sign(!self.sign());
        ret
    }

    #[inline]
    #[must_use]
    fn abs(&self) -> Self {
        let mut ret = Self::from_bits(self.to_bits());
        ret.set_sign(Self::Payload::zero());
        ret
    }

    #[inline]
    #[must_use]
    fn sign(&self) -> Self::Payload {
        (self.to_bits() >> Self::SIGN_POS) & Self::Payload::one()
    }

    #[inline]
    #[must_use]
    fn exponent(&self) -> Self::Payload {
        (self.to_bits() >> Self::EXPONENT_POS) & Self::EXPONENT_BIT
    }

    #[inline]
    #[must_use]
    fn fraction(&self) -> Self::Payload {
        self.to_bits() & Self::FRACTION_BIT
    }

    #[inline]
    #[must_use]
    fn is_positive(&self) -> bool {
        self.sign() == Self::Payload::zero()
    }

    #[inline]
    #[must_use]
    fn is_positive_zero(&self) -> bool {
        self.is_positive()
            && self.exponent() == Self::Payload::zero()
            && self.fraction() == Self::Payload::zero()
    }

    #[inline]
    #[must_use]
    fn is_positive_subnormal(&self) -> bool {
        self.is_positive()
            && self.exponent() == Self::Payload::zero()
            && self.fraction() != Self::Payload::zero()
    }

    #[inline]
    #[must_use]
    fn is_positive_normal(&self) -> bool {
        self.is_positive()
            && self.exponent() != Self::Payload::zero()
            && self.exponent() != Self::EXPONENT_BIT
    }

    #[inline]
    #[must_use]
    fn is_positive_infinity(&self) -> bool {
        self.is_positive()
            && self.exponent() == Self::EXPONENT_BIT
            && self.fraction() == Self::Payload::zero()
    }

    #[inline]
    #[must_use]
    fn is_negative(&self) -> bool {
        self.sign() == Self::Payload::one()
    }

    #[inline]
    #[must_use]
    fn is_negative_zero(&self) -> bool {
        self.is_negative()
            && self.exponent() == Self::Payload::zero()
            && self.fraction() == Self::Payload::zero()
    }

    #[inline]
    #[must_use]
    fn is_negative_subnormal(&self) -> bool {
        self.is_negative()
            && self.exponent() == Self::Payload::zero()
            && self.fraction() != Self::Payload::zero()
    }

    #[inline]
    #[must_use]
    fn is_negative_normal(&self) -> bool {
        self.is_negative()
            && self.exponent() != Self::Payload::zero()
            && self.exponent() != Self::EXPONENT_BIT
    }

    #[inline]
    #[must_use]
    fn is_negative_infinity(&self) -> bool {
        self.is_negative()
            && self.exponent() == Self::EXPONENT_BIT
            && self.fraction() == Self::Payload::zero()
    }

    #[inline]
    #[must_use]
    fn is_nan(&self) -> bool {
        self.exponent() == Self::EXPONENT_BIT && self.fraction() != Self::Payload::zero()
    }

    #[inline]
    #[must_use]
    fn is_zero(&self) -> bool {
        self.is_positive_zero() || self.is_negative_zero()
    }

    #[inline]
    #[must_use]
    fn is_subnormal(&self) -> bool {
        self.exponent() == Self::Payload::zero()
    }

    #[inline]
    fn set_sign(&mut self, x: Self::Payload) {
        self.set_payload(
            (self.to_bits() & !(Self::Payload::one() << Self::SIGN_POS))
                | ((x & Self::Payload::one()) << Self::SIGN_POS),
        );
    }

    #[inline]
    fn set_exponent(&mut self, x: Self::Payload) {
        self.set_payload(
            (self.to_bits() & !(Self::EXPONENT_BIT << Self::EXPONENT_POS))
                | ((x & Self::EXPONENT_BIT) << Self::EXPONENT_POS),
        );
    }

    #[inline]
    fn set_fraction(&mut self, x: Self::Payload) {
        self.set_payload((self.to_bits() & !Self::FRACTION_BIT) | (x & Self::FRACTION_BIT));
    }

    #[inline]
    #[must_use]
    fn positive_infinity() -> Self {
        let mut x = Self::from_bits(Self::Payload::zero());
        x.set_exponent(Self::EXPONENT_BIT);
        x
    }

    #[inline]
    #[must_use]
    fn positive_zero() -> Self {
        Self::from_bits(Self::Payload::zero())
    }

    #[inline]
    #[must_use]
    fn negative_infinity() -> Self {
        let mut x = Self::from_bits(Self::Payload::zero());
        x.set_sign(Self::Payload::one());
        x.set_exponent(Self::EXPONENT_BIT);
        x
    }

    #[inline]
    #[must_use]
    fn negative_zero() -> Self {
        let mut x = Self::from_bits(Self::Payload::zero());
        x.set_sign(Self::Payload::one());
        x
    }

    #[inline]
    #[must_use]
    fn quiet_nan() -> Self {
        let mut x = Self::from_bits(Self::Payload::zero());
        x.set_exponent(Self::EXPONENT_BIT);
        x.set_fraction(Self::Payload::one() << (Self::EXPONENT_POS - 1));
        x
    }
}
