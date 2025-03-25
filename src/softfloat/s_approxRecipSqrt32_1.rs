use super::{softfloat_approxRecipSqrt_1k0s, softfloat_approxRecipSqrt_1k1s};

#[must_use]
pub const fn softfloat_approxRecipSqrt32_1(oddExpA: u32, a: u32) -> u32 {
    let index = ((a >> 27) & 0xe).wrapping_add(oddExpA) as usize;
    let eps = (a >> 12) as u16;
    let r0 = softfloat_approxRecipSqrt_1k0s[index]
        .wrapping_sub(softfloat_approxRecipSqrt_1k1s[index].wrapping_mul(eps as u64) >> 20);
    let mut ESqrR0 = (r0 as u32).wrapping_mul(r0 as u32);
    if oddExpA == 0 {
        ESqrR0 <<= 1;
    }
    let sigma0 = !((ESqrR0 as u64).wrapping_mul(a as u64) >> 23) as u32;
    let mut r =
        ((r0 as u32) << 16).wrapping_add(((r0 as u64).wrapping_mul(sigma0 as u64) >> 25) as u32);
    let sqrSigma0 = ((sigma0 as u64).wrapping_mul(sigma0 as u64) >> 32) as u32;
    r = (r as u64).wrapping_add(
        ((r >> 1)
            .wrapping_add(r >> 3)
            .wrapping_sub((r0 as u32) << 14) as u64)
            .wrapping_mul(sqrSigma0 as u64)
            >> 48,
    ) as u32;
    if r & 0x8000_0000 == 0 {
        r = 0x8000_0000;
    }
    return r;
}
