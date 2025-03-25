use super::{float32_t, softfloat_isSigNaNF32UI};

#[must_use]
#[inline]
pub const fn f32_isSignalingNaN(a: float32_t) -> bool {
    return softfloat_isSigNaNF32UI(a.v);
}
