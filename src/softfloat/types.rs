#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(C)]
pub struct float32_t {
    pub v: u32,
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(C)]
pub struct float64_t {
    pub v: u64,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(C)]
pub struct uint128 {
    pub v0: u64,
    pub v64: u64,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct exp16_sig32 {
    pub exp: i16,
    pub sig: u32,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct exp16_sig64 {
    pub exp: i16,
    pub sig: u64,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct uint64_extra {
    pub extra: u64,
    pub v: u64,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct uint128_extra {
    pub extra: u64,
    pub v: uint128,
}

impl float32_t {
    #[inline]
    #[must_use]
    pub const fn from_bits(v: u32) -> Self {
        Self { v }
    }

    #[inline]
    #[must_use]
    pub const fn to_bits(self) -> u32 {
        self.v
    }
}

impl float64_t {
    #[inline]
    #[must_use]
    pub const fn from_bits(v: u64) -> Self {
        Self { v }
    }

    #[inline]
    #[must_use]
    pub const fn to_bits(self) -> u64 {
        self.v
    }
}
