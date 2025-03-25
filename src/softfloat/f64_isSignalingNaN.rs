use super::float64_t;

#[inline]
#[must_use]
pub const fn f64_isSignalingNaN(a: float64_t) -> bool {
    return (a.v & 0x7ff8_0000_0000_0000) == 0x7ff0_0000_0000_0000
        && (a.v & 0x0007_ffff_ffff_ffff) != 0;
}
