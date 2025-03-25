// SPDX-License-Identifier: MIT OR Apache-2.0

use super::super::softfloat::{
    f64_add, f64_div, f64_eq, f64_eq_signaling, f64_isSignalingNaN, f64_le, f64_le_quiet, f64_lt,
    f64_lt_quiet, f64_mul, f64_mulAdd, f64_rem, f64_roundToInt, f64_sqrt, f64_sub, f64_to_f32,
    f64_to_i32, f64_to_i64, f64_to_ui32, f64_to_ui64, float32_t, float64_t, i32_to_f64, i64_to_f64,
    ui32_to_f64, ui64_to_f64,
};
use super::{Float, RoundingMode};
use core::borrow::Borrow;

impl float64_t {
    /// Converts primitive `f64` to `float64_t`
    #[inline]
    #[must_use]
    pub fn from_f32(v: f32, rnd: RoundingMode, detect_tininess: u8) -> (Self, u8) {
        float32_t::from_bits(v.to_bits()).to_f64(rnd, detect_tininess)
    }

    /// Converts primitive `f64` to `float64_t`
    #[inline]
    #[must_use]
    pub const fn from_f64(v: f64) -> Self {
        Self::from_bits(v.to_bits())
    }
}

impl Float for float64_t {
    type Payload = u64;

    const EXPONENT_BIT: Self::Payload = 0x7ff;
    const FRACTION_BIT: Self::Payload = 0xf_ffff_ffff_ffff;
    const SIGN_POS: usize = 63;
    const EXPONENT_POS: usize = 52;

    #[inline]
    fn set_payload(&mut self, x: Self::Payload) {
        self.v = x;
    }

    #[inline]
    fn from_bits(v: Self::Payload) -> Self {
        Self { v }
    }

    #[inline]
    fn to_bits(&self) -> Self::Payload {
        self.v
    }

    fn add<T: Borrow<Self>>(&self, x: T, rnd: RoundingMode, detect_tininess: u8) -> (Self, u8) {
        f64_add(*self, *x.borrow(), rnd.to_softfloat(), detect_tininess)
    }

    fn sub<T: Borrow<Self>>(&self, x: T, rnd: RoundingMode, detect_tininess: u8) -> (Self, u8) {
        f64_sub(*self, *x.borrow(), rnd.to_softfloat(), detect_tininess)
    }

    fn mul<T: Borrow<Self>>(&self, x: T, rnd: RoundingMode, detect_tininess: u8) -> (Self, u8) {
        f64_mul(*self, *x.borrow(), rnd.to_softfloat(), detect_tininess)
    }

    fn fused_mul_add<T: Borrow<Self>>(
        &self,
        x: T,
        y: T,
        rnd: RoundingMode,
        detect_tininess: u8,
    ) -> (Self, u8) {
        f64_mulAdd(
            *self,
            *x.borrow(),
            *y.borrow(),
            rnd.to_softfloat(),
            detect_tininess,
        )
    }

    fn div<T: Borrow<Self>>(&self, x: T, rnd: RoundingMode, detect_tininess: u8) -> (Self, u8) {
        f64_div(*self, *x.borrow(), rnd.to_softfloat(), detect_tininess)
    }

    fn rem<T: Borrow<Self>>(&self, x: T, rnd: RoundingMode, detect_tininess: u8) -> (Self, u8) {
        f64_rem(*self, *x.borrow(), rnd.to_softfloat(), detect_tininess)
    }

    fn sqrt(&self, rnd: RoundingMode, detect_tininess: u8) -> (Self, u8) {
        f64_sqrt(*self, rnd.to_softfloat(), detect_tininess)
    }

    fn eq<T: Borrow<Self>>(&self, x: T) -> (bool, u8) {
        f64_eq(*self, *x.borrow())
    }

    fn lt<T: Borrow<Self>>(&self, x: T) -> (bool, u8) {
        f64_lt(*self, *x.borrow())
    }

    fn le<T: Borrow<Self>>(&self, x: T) -> (bool, u8) {
        f64_le(*self, *x.borrow())
    }

    fn lt_quiet<T: Borrow<Self>>(&self, x: T) -> (bool, u8) {
        f64_lt_quiet(*self, *x.borrow())
    }

    fn le_quiet<T: Borrow<Self>>(&self, x: T) -> (bool, u8) {
        f64_le_quiet(*self, *x.borrow())
    }

    fn eq_signaling<T: Borrow<Self>>(&self, x: T) -> (bool, u8) {
        f64_eq_signaling(*self, *x.borrow())
    }

    fn is_signaling_nan(&self) -> bool {
        f64_isSignalingNaN(*self)
    }

    fn from_u32(x: u32, _rnd: RoundingMode, _detect_tininess: u8) -> (Self, u8) {
        (ui32_to_f64(x), 0)
    }

    fn from_u64(x: u64, rnd: RoundingMode, detect_tininess: u8) -> (Self, u8) {
        ui64_to_f64(x, rnd.to_softfloat(), detect_tininess)
    }

    fn from_i32(x: i32, _rnd: RoundingMode, _detect_tininess: u8) -> (Self, u8) {
        (i32_to_f64(x), 0)
    }

    fn from_i64(x: i64, rnd: RoundingMode, detect_tininess: u8) -> (Self, u8) {
        i64_to_f64(x, rnd.to_softfloat(), detect_tininess)
    }

    fn to_u32(&self, rnd: RoundingMode, exact: bool) -> (u32, u8) {
        f64_to_ui32(*self, rnd.to_softfloat(), exact)
    }

    fn to_u64(&self, rnd: RoundingMode, exact: bool) -> (u64, u8) {
        f64_to_ui64(*self, rnd.to_softfloat(), exact)
    }

    fn to_i32(&self, rnd: RoundingMode, exact: bool) -> (i32, u8) {
        f64_to_i32(*self, rnd.to_softfloat(), exact)
    }

    fn to_i64(&self, rnd: RoundingMode, exact: bool) -> (i64, u8) {
        f64_to_i64(*self, rnd.to_softfloat(), exact)
    }

    fn round_to_integral(&self, rnd: RoundingMode, exact: bool) -> (Self, u8) {
        f64_roundToInt(*self, rnd.to_softfloat(), exact)
    }

    fn to_f32(&self, rnd: RoundingMode, detect_tininess: u8) -> (float32_t, u8) {
        f64_to_f32(*self, rnd.to_softfloat(), detect_tininess)
    }

    fn to_f64(&self, _rnd: RoundingMode, _detect_tininess: u8) -> (float64_t, u8) {
        (*self, 0)
    }
}
