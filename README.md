# softfloat-pure

This is a pure Rust library for RISC-V compatible floating point operations
(FP32 & FP46). This project is based on code from the following repositories:

 * https://github.com/ucb-bar/berkeley-softfloat-3 (BSD 3-clause license)
 * https://github.com/chipshort/softfloat-c (BSD 3-clause license)
 * https://github.com/dalance/softfloat-wrapper (Apache-2.0 or MIT license)

Berkeley Softfloat was re-translated by hand from C to Rust after the initial
C2Rust translation required extensive modifications for readability and `const`.

## Other notable floating point libraries:

 * https://github.com/rust-lang/rustc_apfloat
 * https://bellard.org/softfp/ (small C implementation)

## Test Suites

 * https://github.com/ucb-bar/berkeley-testfloat-3
 * https://github.com/PRUNERS/FLiT ?
 * https://github.com/FPBench/FPBench ?
