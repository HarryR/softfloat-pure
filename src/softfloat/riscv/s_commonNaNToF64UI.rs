use super::{commonNaN, defaultNaNF64UI};

/*----------------------------------------------------------------------------
| Converts the common NaN pointed to by 'aPtr' into a 64-bit floating-point
| NaN, and returns the bit pattern of this value as an unsigned integer.
*----------------------------------------------------------------------------*/
//#define softfloat_commonNaNToF64UI( aPtr ) ((uint_fast64_t) defaultNaNF64UI)

#[inline]
#[must_use]
pub const fn softfloat_commonNaNToF64UI(_aPtr: &commonNaN) -> u64 {
    defaultNaNF64UI
}
