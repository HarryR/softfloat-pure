use super::uint128;

#[must_use]
pub const fn softfloat_mul64To128(a: u64, b: u64) -> uint128 {
    let a32: u32 = (a >> 32) as u32;
    let a0: u32 = a as u32;
    let b32: u32 = (b >> 32) as u32;
    let b0: u32 = b as u32;
    let mut z_v0 = (a0 as u64).wrapping_mul(b0 as u64);
    let mid1 = (a32 as u64).wrapping_mul(b0 as u64);
    let mut mid = mid1.wrapping_add((a0 as u64).wrapping_mul(b32 as u64));
    let mut z_v64 = (a32 as u64).wrapping_mul(b32 as u64);
    z_v64 = z_v64.wrapping_add((((mid < mid1) as u64) << 32) | mid >> 32);
    mid <<= 32;
    z_v0 = z_v0.wrapping_add(mid);
    z_v64 = z_v64.wrapping_add((z_v0 < mid) as u64);
    return uint128 {
        v0: z_v0,
        v64: z_v64,
    };
}
