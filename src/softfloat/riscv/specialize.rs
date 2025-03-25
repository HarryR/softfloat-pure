/*============================================================================

This C header file is part of the SoftFloat IEEE Floating-Point Arithmetic
Package, Release 3e, by John R. Hauser.

Copyright 2011, 2012, 2013, 2014, 2015, 2016, 2018 The Regents of the
University of California.  All rights reserved.

Redistribution and use in source and binary forms, with or without
modification, are permitted provided that the following conditions are met:

 1. Redistributions of source code must retain the above copyright notice,
    this list of conditions, and the following disclaimer.

 2. Redistributions in binary form must reproduce the above copyright notice,
    this list of conditions, and the following disclaimer in the documentation
    and/or other materials provided with the distribution.

 3. Neither the name of the University nor the names of its contributors may
    be used to endorse or promote products derived from this software without
    specific prior written permission.

THIS SOFTWARE IS PROVIDED BY THE REGENTS AND CONTRIBUTORS "AS IS", AND ANY
EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED
WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE, ARE
DISCLAIMED.  IN NO EVENT SHALL THE REGENTS OR CONTRIBUTORS BE LIABLE FOR ANY
DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES
(INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES;
LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND
ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
(INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

=============================================================================*/

use super::super::softfloat_tininess_afterRounding;

/*----------------------------------------------------------------------------
| Default value for 'softfloat_detectTininess'.
*----------------------------------------------------------------------------*/
pub const init_detectTininess: u8 = softfloat_tininess_afterRounding;

/*----------------------------------------------------------------------------
| The values to return on conversions to 32-bit integer formats that raise an
| invalid exception.
*----------------------------------------------------------------------------*/
pub const ui32_fromPosOverflow: u32 = 0xFFFF_FFFF;
pub const ui32_fromNegOverflow: u32 = 0;
pub const ui32_fromNaN: u32 = 0xFFFF_FFFF;
pub const i32_fromPosOverflow: i32 = 0x7FFF_FFFF;
pub const i32_fromNegOverflow: i32 = (-0x7FFF_FFFF - 1);
pub const i32_fromNaN: i32 = 0x7FFF_FFFF;

/*----------------------------------------------------------------------------
| The values to return on conversions to 64-bit integer formats that raise an
| invalid exception.
*----------------------------------------------------------------------------*/
pub const ui64_fromPosOverflow: u64 = 0xFFFF_FFFF_FFFF_FFFF;
pub const ui64_fromNegOverflow: u64 = 0;
pub const ui64_fromNaN: u64 = 0xFFFF_FFFF_FFFF_FFFF;
pub const i64_fromPosOverflow: i64 = 0x7FFF_FFFF_FFFF_FFFF;
pub const i64_fromNegOverflow: i64 = -0x7FFF_FFFF_FFFF_FFFF - 1;
pub const i64_fromNaN: i64 = 0x7FFF_FFFF_FFFF_FFFF;

/*----------------------------------------------------------------------------
| "Common NaN" structure, used to transfer NaN representations from one format
| to another.
*----------------------------------------------------------------------------*/
pub struct commonNaN {}
impl commonNaN {
    #[inline]
    #[must_use]
    pub(crate) const fn default() -> Self {
        Self {}
    }
}

/*----------------------------------------------------------------------------
| The bit pattern for a default generated 32-bit floating-point NaN.
*----------------------------------------------------------------------------*/
pub const defaultNaNF32UI: u32 = 0x7FC0_0000;

/*----------------------------------------------------------------------------
| Returns true when 32-bit unsigned integer 'uiA' has the bit pattern of a
| 32-bit floating-point signaling NaN.
| Note:  This macro evaluates its argument more than once.
*----------------------------------------------------------------------------*/
#[inline]
#[must_use]
pub const fn softfloat_isSigNaNF32UI(uiA: u32) -> bool {
    (((uiA) & 0x7FC0_0000) == 0x7F80_0000) && ((uiA) & 0x003F_FFFF) != 0
}

/*----------------------------------------------------------------------------
| The bit pattern for a default generated 64-bit floating-point NaN.
*----------------------------------------------------------------------------*/
pub const defaultNaNF64UI: u64 = 0x7FF8_0000_0000_0000;

/*----------------------------------------------------------------------------
| Returns true when 64-bit unsigned integer 'uiA' has the bit pattern of a
| 64-bit floating-point signaling NaN.
| Note:  This macro evaluates its argument more than once.
*----------------------------------------------------------------------------*/
//#define softfloat_isSigNaNF64UI( uiA ) ((((uiA) & UINT64_C( 0x7FF8000000000000 )) == UINT64_C( 0x7FF0000000000000 )) && ((uiA) & UINT64_C( 0x0007FFFFFFFFFFFF )))
#[inline]
#[must_use]
pub const fn softfloat_isSigNaNF64UI(uiA: u64) -> bool {
    ((uiA & 0x7FF8_0000_0000_0000) == 0x7FF0_0000_0000_0000) && ((uiA) & 0x0007_FFFF_FFFF_FFFF) != 0
}
