// SPDX-License-Identifier: MIT OR Apache-2.0

use super::super::softfloat::{
    f32_add, f32_div, f32_eq, f32_eq_signaling, f32_isSignalingNaN, f32_le, f32_le_quiet, f32_lt,
    f32_lt_quiet, f32_mul, f32_mulAdd, f32_rem, f32_roundToInt, f32_sqrt, f32_sub, f32_to_f64,
    f32_to_i32, f32_to_i64, f32_to_ui32, f32_to_ui64, float32_t, float64_t, i32_to_f32, i64_to_f32,
    ui32_to_f32, ui64_to_f32,
};
use super::{Float, RoundingMode};
use core::borrow::Borrow;

impl float32_t {
    /// Converts primitive `f32` to `float32_t`
    #[inline]
    #[must_use]
    pub const fn from_f32(v: f32) -> (Self, u8) {
        (Self::from_bits(v.to_bits()), 0)
    }

    /// Converts primitive `f64` to `float32_t`
    #[inline]
    #[must_use]
    pub fn from_f64(v: f64, rnd: RoundingMode, detect_tininess: u8) -> (Self, u8) {
        float64_t::from_bits(v.to_bits()).to_f32(rnd, detect_tininess)
    }
}

impl Float for float32_t {
    type Payload = u32;

    const EXPONENT_BIT: Self::Payload = 0xff;
    const FRACTION_BIT: Self::Payload = 0x7f_ffff;
    const SIGN_POS: usize = 31;
    const EXPONENT_POS: usize = 23;

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

    #[inline]
    fn add<T: Borrow<Self>>(&self, x: T, rnd: RoundingMode, detect_tininess: u8) -> (Self, u8) {
        f32_add(*self, *x.borrow(), rnd.to_softfloat(), detect_tininess)
    }

    #[inline]
    fn sub<T: Borrow<Self>>(&self, x: T, rnd: RoundingMode, detect_tininess: u8) -> (Self, u8) {
        f32_sub(*self, *x.borrow(), rnd.to_softfloat(), detect_tininess)
    }

    #[inline]
    fn mul<T: Borrow<Self>>(&self, x: T, rnd: RoundingMode, detect_tininess: u8) -> (Self, u8) {
        f32_mul(*self, *x.borrow(), rnd.to_softfloat(), detect_tininess)
    }

    #[inline]
    fn fused_mul_add<T: Borrow<Self>>(
        &self,
        x: T,
        y: T,
        rnd: RoundingMode,
        detect_tininess: u8,
    ) -> (Self, u8) {
        f32_mulAdd(
            *self,
            *x.borrow(),
            *y.borrow(),
            rnd.to_softfloat(),
            detect_tininess,
        )
    }

    #[inline]
    fn div<T: Borrow<Self>>(&self, x: T, rnd: RoundingMode, detect_tininess: u8) -> (Self, u8) {
        f32_div(*self, *x.borrow(), rnd.to_softfloat(), detect_tininess)
    }

    #[inline]
    fn rem<T: Borrow<Self>>(&self, x: T, rnd: RoundingMode, detect_tininess: u8) -> (Self, u8) {
        f32_rem(*self, *x.borrow(), rnd.to_softfloat(), detect_tininess)
    }

    #[inline]
    fn sqrt(&self, rnd: RoundingMode, detect_tininess: u8) -> (Self, u8) {
        f32_sqrt(*self, rnd.to_softfloat(), detect_tininess)
    }

    #[inline]
    fn eq<T: Borrow<Self>>(&self, x: T) -> (bool, u8) {
        f32_eq(*self, *x.borrow())
    }

    #[inline]
    fn lt<T: Borrow<Self>>(&self, x: T) -> (bool, u8) {
        f32_lt(*self, *x.borrow())
    }

    #[inline]
    fn le<T: Borrow<Self>>(&self, x: T) -> (bool, u8) {
        f32_le(*self, *x.borrow())
    }

    #[inline]
    fn lt_quiet<T: Borrow<Self>>(&self, x: T) -> (bool, u8) {
        f32_lt_quiet(*self, *x.borrow())
    }

    #[inline]
    fn le_quiet<T: Borrow<Self>>(&self, x: T) -> (bool, u8) {
        f32_le_quiet(*self, *x.borrow())
    }

    #[inline]
    fn eq_signaling<T: Borrow<Self>>(&self, x: T) -> (bool, u8) {
        f32_eq_signaling(*self, *x.borrow())
    }

    #[inline]
    fn is_signaling_nan(&self) -> bool {
        f32_isSignalingNaN(*self)
    }

    #[inline]
    fn from_u32(x: u32, rnd: RoundingMode, detect_tininess: u8) -> (Self, u8) {
        ui32_to_f32(x, rnd.to_softfloat(), detect_tininess)
    }

    #[inline]
    fn from_u64(x: u64, rnd: RoundingMode, detect_tininess: u8) -> (Self, u8) {
        ui64_to_f32(x, rnd.to_softfloat(), detect_tininess)
    }

    #[inline]
    fn from_i32(x: i32, rnd: RoundingMode, detect_tininess: u8) -> (Self, u8) {
        i32_to_f32(x, rnd.to_softfloat(), detect_tininess)
    }

    #[inline]
    fn from_i64(x: i64, rnd: RoundingMode, detect_tininess: u8) -> (Self, u8) {
        i64_to_f32(x, rnd.to_softfloat(), detect_tininess)
    }

    #[inline]
    fn to_u32(&self, rnd: RoundingMode, exact: bool) -> (u32, u8) {
        f32_to_ui32(*self, rnd.to_softfloat(), exact)
    }

    #[inline]
    fn to_u64(&self, rnd: RoundingMode, exact: bool) -> (u64, u8) {
        f32_to_ui64(*self, rnd.to_softfloat(), exact)
    }

    #[inline]
    fn to_i32(&self, rnd: RoundingMode, exact: bool) -> (i32, u8) {
        f32_to_i32(*self, rnd.to_softfloat(), exact)
    }

    #[inline]
    fn to_i64(&self, rnd: RoundingMode, exact: bool) -> (i64, u8) {
        f32_to_i64(*self, rnd.to_softfloat(), exact)
    }

    #[inline]
    fn round_to_integral(&self, rnd: RoundingMode, exact: bool) -> (Self, u8) {
        f32_roundToInt(*self, rnd.to_softfloat(), exact)
    }

    #[inline]
    fn to_f32(&self, _rnd: RoundingMode, _detect_tininess: u8) -> (float32_t, u8) {
        (*self, 0)
    }

    #[inline]
    fn to_f64(&self, _rnd: RoundingMode, _detect_tininess: u8) -> (float64_t, u8) {
        f32_to_f64(*self)
    }
}
