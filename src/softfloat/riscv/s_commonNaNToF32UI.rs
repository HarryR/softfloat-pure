use super::{commonNaN, defaultNaNF32UI};

/*----------------------------------------------------------------------------
| Converts the common NaN pointed to by 'aPtr' into a 32-bit floating-point
| NaN, and returns the bit pattern of this value as an unsigned integer.
*----------------------------------------------------------------------------*/
//#define softfloat_commonNaNToF32UI( aPtr ) ((uint_fast32_t) defaultNaNF32UI)

#[inline]
#[must_use]
pub const fn softfloat_commonNaNToF32UI(_aPtr: &commonNaN) -> u32 {
    defaultNaNF32UI
}
