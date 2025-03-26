// SPDX-License-Identifier: MIT OR Apache-2.0

use core::borrow::Borrow;

use super::{
    softfloat::{
        float32_t, float64_t, i32_to_f32, i32_to_f64, i64_to_f32, i64_to_f64, init_detectTininess,
        ui32_to_f32, ui32_to_f64, ui64_to_f32, ui64_to_f64,
    },
    wrapper::{ExceptionFlags, Float, RoundingMode, TininessMode},
};

#[derive(Copy, Clone, Debug)]
pub struct FPU {
    pub flags: ExceptionFlags,
    detect_tininess: u8,
}

impl FPU {
    #[inline]
    #[must_use]
    pub fn new(tininess: TininessMode) -> Self {
        Self {
            flags: ExceptionFlags::default(),
            detect_tininess: tininess.to_softfloat(),
        }
    }
}

impl Default for FPU {
    #[inline]
    fn default() -> Self {
        Self {
            flags: ExceptionFlags::default(),
            detect_tininess: init_detectTininess,
        }
    }
}

impl FPU {
    #[inline]
    const fn flagged_f64(&mut self, args: (float64_t, u8)) -> float64_t {
        self.flags.merge(args.1);
        args.0
    }

    #[inline]
    const fn flagged_f32(&mut self, args: (float32_t, u8)) -> float32_t {
        self.flags.merge(args.1);
        args.0
    }

    #[inline]
    fn flagged<X>(&mut self, args: (X, u8)) -> X {
        self.flags.merge(args.1);
        args.0
    }
}

impl FPU {
    #[inline]
    #[must_use]
    pub fn to_i32<F, T>(&mut self, x: T, rnd: RoundingMode, exact: bool) -> i32
    where
        F: Float,
        T: Borrow<F>,
    {
        self.flagged(x.borrow().to_i32(rnd, exact))
    }

    #[inline]
    #[must_use]
    pub fn to_i64<F, T>(&mut self, a: T, rnd: RoundingMode, exact: bool) -> i64
    where
        F: Float,
        T: Borrow<F>,
    {
        self.flagged(a.borrow().to_i64(rnd, exact))
    }

    #[inline]
    #[must_use]
    pub fn to_u64<F, T>(&mut self, a: T, rnd: RoundingMode, exact: bool) -> u64
    where
        F: Float,
        T: Borrow<F>,
    {
        self.flagged(a.borrow().to_u64(rnd, exact))
    }

    #[inline]
    #[must_use]
    pub fn to_u32<F, T>(&mut self, a: T, rnd: RoundingMode, exact: bool) -> u32
    where
        F: Float,
        T: Borrow<F>,
    {
        self.flagged(a.borrow().to_u32(rnd, exact))
    }

    #[inline]
    #[must_use]
    pub fn to_f64<F, T>(&mut self, a: T, rnd: RoundingMode) -> float64_t
    where
        F: Float,
        T: Borrow<F>,
    {
        self.flagged(a.borrow().to_f64(rnd, self.detect_tininess))
    }

    #[inline]
    #[must_use]
    pub fn to_f32<F, T>(&mut self, a: T, rnd: RoundingMode) -> float32_t
    where
        F: Float,
        T: Borrow<F>,
    {
        self.flagged(a.borrow().to_f32(rnd, self.detect_tininess))
    }

    #[inline]
    #[must_use]
    pub fn is_signaling_nan<F, T>(&mut self, a: T) -> bool
    where
        F: Float,
        T: Borrow<F>,
    {
        a.borrow().is_signaling_nan()
    }

    #[inline]
    #[must_use]
    pub fn lt<F, T>(&mut self, a: T, b: T) -> bool
    where
        F: Float,
        T: Borrow<F>,
    {
        self.flagged(a.borrow().lt(b.borrow()))
    }

    #[inline]
    #[must_use]
    pub fn le<F, T>(&mut self, a: T, b: T) -> bool
    where
        F: Float,
        T: Borrow<F>,
    {
        self.flagged(a.borrow().le(b.borrow()))
    }

    #[inline]
    #[must_use]
    pub fn eq<F, T>(&mut self, a: T, b: T) -> bool
    where
        F: Float,
        T: Borrow<F>,
    {
        self.flagged(a.borrow().eq(b.borrow()))
    }

    #[inline]
    #[must_use]
    pub fn eq_signaling<F, T>(&mut self, a: T, b: T) -> bool
    where
        F: Float,
        T: Borrow<F>,
    {
        self.flagged(a.borrow().eq_signaling(b.borrow()))
    }

    #[inline]
    #[must_use]
    pub fn lt_quiet<F, T>(&mut self, a: T, b: T) -> bool
    where
        F: Float,
        T: Borrow<F>,
    {
        self.flagged(a.borrow().lt_quiet(b))
    }

    #[inline]
    #[must_use]
    pub fn le_quiet<F, T>(&mut self, a: T, b: T) -> bool
    where
        F: Float,
        T: Borrow<F>,
    {
        self.flagged(a.borrow().le_quiet(b))
    }

    #[inline]
    #[must_use]
    pub fn add<F, T>(&mut self, a: T, b: T, rnd: RoundingMode) -> F
    where
        F: Float,
        T: Borrow<F>,
    {
        self.flagged(a.borrow().add(b, rnd, self.detect_tininess))
    }

    #[inline]
    #[must_use]
    pub fn mul_add<F, T>(&mut self, a: T, b: T, c: T, rnd: RoundingMode) -> F
    where
        F: Float,
        T: Borrow<F>,
    {
        self.flagged(a.borrow().fused_mul_add(b, c, rnd, self.detect_tininess))
    }

    #[inline]
    #[must_use]
    pub fn div<F, T>(&mut self, a: T, b: T, rnd: RoundingMode) -> F
    where
        F: Float,
        T: Borrow<F>,
    {
        self.flagged(a.borrow().div(b, rnd, self.detect_tininess))
    }

    #[inline]
    #[must_use]
    pub fn mul<F, T>(&mut self, a: T, b: T, rnd: RoundingMode) -> F
    where
        F: Float,
        T: Borrow<F>,
    {
        self.flagged(a.borrow().mul(b, rnd, self.detect_tininess))
    }

    #[inline]
    #[must_use]
    pub fn rem<F, T>(&mut self, a: T, b: T, rnd: RoundingMode) -> F
    where
        F: Float,
        T: Borrow<F>,
    {
        self.flagged(a.borrow().rem(b, rnd, self.detect_tininess))
    }

    #[inline]
    #[must_use]
    pub fn sub<F, T>(&mut self, a: T, b: T, rnd: RoundingMode) -> F
    where
        F: Float,
        T: Borrow<F>,
    {
        self.flagged(a.borrow().sub(b, rnd, self.detect_tininess))
    }

    #[inline]
    #[must_use]
    pub fn round_to_int<F, T>(&mut self, a: T, rnd: RoundingMode, exact: bool) -> F
    where
        F: Float,
        T: Borrow<F>,
    {
        self.flagged(a.borrow().round_to_integral(rnd, exact))
    }

    #[inline]
    #[must_use]
    pub fn sqrt<F, T>(&mut self, a: T, rnd: RoundingMode) -> F
    where
        F: Float,
        T: Borrow<F>,
    {
        self.flagged(a.borrow().sqrt(rnd, self.detect_tininess))
    }
}

impl FPU {
    #[inline]
    #[must_use]
    pub fn f32_from_i64(&mut self, a: i64, rnd: RoundingMode) -> float32_t {
        self.flagged_f32(i64_to_f32(a, rnd.to_softfloat(), self.detect_tininess))
    }

    #[inline]
    #[must_use]
    pub fn f32_from_i32(&mut self, a: i32, rnd: RoundingMode) -> float32_t {
        self.flagged_f32(i32_to_f32(a, rnd.to_softfloat(), self.detect_tininess))
    }

    #[inline]
    #[must_use]
    pub fn f32_from_u64(&mut self, a: u64, rnd: RoundingMode) -> float32_t {
        self.flagged_f32(ui64_to_f32(a, rnd.to_softfloat(), self.detect_tininess))
    }

    #[inline]
    #[must_use]
    pub fn f32_from_u32(&mut self, a: u32, rnd: RoundingMode) -> float32_t {
        self.flagged_f32(ui32_to_f32(a, rnd.to_softfloat(), self.detect_tininess))
    }
}

impl FPU {
    #[inline]
    #[must_use]
    pub fn f64_from_i64(&mut self, a: i64, rnd: RoundingMode) -> float64_t {
        self.flagged_f64(i64_to_f64(a, rnd.to_softfloat(), self.detect_tininess))
    }

    #[inline]
    #[must_use]
    pub fn f64_from_i32(&mut self, a: i32) -> float64_t {
        i32_to_f64(a)
    }

    #[inline]
    #[must_use]
    pub fn f64_from_u64(&mut self, a: u64, rnd: RoundingMode) -> float64_t {
        self.flagged_f64(ui64_to_f64(a, rnd.to_softfloat(), self.detect_tininess))
    }

    #[inline]
    #[must_use]
    pub fn f64_from_u32(&mut self, a: u32) -> float64_t {
        ui32_to_f64(a)
    }
}
