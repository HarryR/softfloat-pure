use super::super::softfloat_flag_invalid;
use super::commonNaN;

/*----------------------------------------------------------------------------
| Assuming 'uiA' has the bit pattern of a 64-bit floating-point NaN, converts
| this NaN to the common NaN form, and stores the resulting common NaN at the
| location pointed to by 'zPtr'.  If the NaN is a signaling NaN, the invalid
| exception is raised.
*----------------------------------------------------------------------------*/
//#define softfloat_f64UIToCommonNaN( uiA, zPtr ) if ( ! ((uiA) & UINT64_C( 0x0008000000000000 )) ) softfloat_raiseFlags( softfloat_flag_invalid )

#[inline]
#[must_use]
pub const fn softfloat_f64UIToCommonNaN(uiA: u64) -> (commonNaN, u8) {
    if (uiA & 0x0008_0000_0000_0000) == 0 {
        return (commonNaN::default(), softfloat_flag_invalid);
    }
    (commonNaN::default(), 0)
}
