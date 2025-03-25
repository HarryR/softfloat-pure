/*============================================================================

This C header file is part of the SoftFloat IEEE Floating-Point Arithmetic
Package, Release 3e, by John R. Hauser.

Copyright 2011, 2012, 2013, 2014, 2015, 2016, 2017 The Regents of the
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

use super::types::{float32_t, float64_t};

#[derive(Copy, Clone)]
#[repr(C)]
pub union ui32_f32 {
    pub ui: u32,
    pub f: float32_t,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub union ui64_f64 {
    pub ui: u64,
    pub f: float64_t,
}

//#define signF32UI( a ) ((bool) ((uint32_t) (a)>>31))
#[inline]
#[must_use]
pub const fn signF32UI(a: u32) -> bool {
    (a >> 31) != 0
}

//#define expF32UI( a ) ((int_fast16_t) ((a)>>23) & 0xFF)
#[inline]
#[must_use]
pub const fn expF32UI(a: u32) -> i16 {
    ((a >> 23) & 0xFF) as i16
}

//#define fracF32UI( a ) ((a) & 0x007FFFFF)
#[inline]
#[must_use]
pub const fn fracF32UI(a: u32) -> u32 {
    a & 0x007F_FFFF
}

//#define packToF32UI( sign, exp, sig ) (((uint32_t) (sign)<<31) + ((uint32_t) (exp)<<23) + (sig))
#[inline]
#[must_use]
pub const fn packToF32UI(sign: bool, exp: i16, sig: u32) -> u32 {
    ((sign as u32) << 31)
        .wrapping_add((exp as u32) << 23)
        .wrapping_add(sig)
}

#[inline]
#[must_use]
pub const fn packToF32(sign: bool, exp: i16, sig: u32) -> float32_t {
    float32_t {
        v: packToF32UI(sign, exp, sig),
    }
}

//#define isNaNF32UI( a ) (((~(a) & 0x7F800000) == 0) && ((a) & 0x007FFFFF))
#[inline]
#[must_use]
pub const fn isNaNF32UI(a: u32) -> bool {
    (!a & 0x7F80_0000) == 0 && (a & 0x007F_FFFF) != 0
}

// #define signF64UI( a ) ((bool) ((uint64_t) (a)>>63))
#[inline]
#[must_use]
pub const fn signF64UI(a: u64) -> bool {
    (a >> 63) != 0
}

// #define expF64UI( a ) ((int_fast16_t) ((a)>>52) & 0x7FF)
#[inline]
#[must_use]
pub const fn expF64UI(a: u64) -> i16 {
    let r = a >> 52;
    let s = r & 0x7FF;
    s as i16
}

// #define fracF64UI( a ) ((a) & UINT64_C( 0x000FFFFFFFFFFFFF ))
#[inline]
#[must_use]
pub const fn fracF64UI(a: u64) -> u64 {
    a & 0x000F_FFFF_FFFF_FFFF
}

// #define packToF64UI( sign, exp, sig ) ((uint64_t) (((uint_fast64_t) (sign)<<63) + ((uint_fast64_t) (exp)<<52) + (sig)))
#[inline]
#[must_use]
pub const fn packToF64UI(sign: bool, exp: i16, sig: u64) -> u64 {
    ((sign as u64) << 63)
        .wrapping_add((exp as u64) << 52)
        .wrapping_add(sig)
}

#[inline]
#[must_use]
pub const fn packToF64(sign: bool, exp: i16, sig: u64) -> float64_t {
    float64_t {
        v: packToF64UI(sign, exp, sig),
    }
}

// #define isNaNF64UI( a ) (((~(a) & UINT64_C( 0x7FF0000000000000 )) == 0) && ((a) & UINT64_C( 0x000FFFFFFFFFFFFF )))
#[inline]
#[must_use]
pub const fn isNaNF64UI(a: u64) -> bool {
    (!a & 0x7FF0_0000_0000_0000) == 0 && (a & 0x000F_FFFF_FFFF_FFFF) != 0
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(PartialEq, Debug)]
pub struct f64_deconstructed {
    sign: bool,
    exp: i16,
    frac: u64,
}

#[inline]
#[must_use]
pub const fn deconstruct_f64UI(a: u64) -> f64_deconstructed {
    f64_deconstructed {
        sign: signF64UI(a),
        exp: expF64UI(a),
        frac: fracF64UI(a),
    }
}
