# softfloat-pure

[![Rust Build and TestFloat](https://github.com/HarryR/softfloat-pure/actions/workflows/rust-build-and-testfloat.yml/badge.svg)](https://github.com/HarryR/softfloat-pure/actions/workflows/rust-build-and-testfloat.yml)
[![made-with-rust](https://img.shields.io/badge/Made%20with-Rust-red.svg)](https://www.rust-lang.org/)

[![License](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![License](https://img.shields.io/badge/License-BSD_3--Clause-blue.svg)](https://opensource.org/licenses/BSD-3-Clause)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)

**A pure Rust library for [RISC-V] compatible [IEEE-754] floating point operations (single & double precision).**

[Berkeley Softfloat 3e] was re-translated by hand from C to Rust as the initial automated [C2Rust] translation from the [softfloat-c] project required extensive modifications for readability, `const` correctness, and to pass the Berkeley Testfloat suite of tests. A small idiomatic Rust layer is provided by an adaption of [softfloat-wrapper]. The goals of this project are:

 * Compatible with restrictive environments (SGX, TEE, ZKP, WASM, microcontroller, firmware etc.)
 * Pure Rust, no C libraries, minimal dependencies
 * `const` `no_std` code throughout, no runtime panics
 * IEEE 754, all rounding modes & exceptions
 * Bitwise compatible with Softfloat + passes all tests
 * Can simulate RISC-V extensions F & D

[RISC-V]: https://five-embeddev.com/riscv-user-isa-manual/Priv-v1.12/f.html
[IEEE-754]: https://en.wikipedia.org/wiki/IEEE_754
[C2Rust]: https://github.com/immunant/c2rust
[softfloat-c]: https://github.com/chipshort/softfloat-c
[Berkeley Softfloat 3e]: https://github.com/ucb-bar/berkeley-softfloat-3
[softfloat-wrapper]: https://github.com/dalance/softfloat-wrapper

## Usage

Add the dependency to your `Cargo.toml` file:

```toml
[dependencies]
softfloat_pure = { git = "https://github.com/HarryR/softfloat-pure.git" }
```

The main differences vs Berkeley SoftFloat and `softfloat-wrapper` are that there is no global or thread-local state and no implicit rounding mode. The `FPU` struct works with the `float32_t` and `float64_t` types:

```rust
use softfloat_pure::*;
let fpu = FPU::default();
let a = float32_t::from_bits(0x...);
let b = float32_t::from_bits(0x...);
let res = fpu.add(a,b, RoundingMode::RneTiesToEven);
assert_eq!(!fpu.flags.is_infinite(), false);
assert_eq!(res.to_bits(), 0x...);
```

This may seem verbose, but the RISC-V instruction set requires you to specify the rounding mode for most floating point operations, even though LLVM and GCC may only emit instructions using the default rounding mode (round to nearest, ties to even).

You can directly access the underlying SoftFloat functions, like `f32_mulAdd` via the `softfloat_pure::softfloat::*` module. These functions accept the tininess parameter (specified as either before or after rounding) in addition to the rounding mode, and return a `u8` representing any raised flags in addition to the result, for example:

```rust
use softfloat_pure::softfloat::*;
let a = float32_t::from_bits(0x...);
let b = float32_t::from_bits(0x...);
let x = f32_add(a,b, 0, 0);
assert_eq!(x.0, 0x...); // result
assert_eq!(x.1, 0);     // flags
```

## Testing

The `floatverify` binary works together with  `testfloat_gen` from the Berkeley testfloat project, we combine this with a Python script (`testfloat-permute.py`) that runs through the permutations of all floating point operations in all modes to verify whether the implementation in this library matches the Softfloat reference implementation.

For example, while also capturing coverage using `llvm-cov`:

    ./testfloat/berkeley-testfloat-3/build/Linux-x86_64-GCC/testfloat_gen -rminMag -tininessafter -notexact f32_mulAdd | cargo llvm-cov --offline --no-clean --no-cfg-coverage run -q -- f32_mulAdd -rminMag -tininessafter -notexact -exit

Note that the `Linux-x86_64-GCC` build target is built with the RISCV profile.

Currently we're at just under 90% coverage, although the full 'level 2' suite of tests takes a long time to run it provides strong confidence that this library is bitwise identical in operation to Softfloat 3e and most if not all edge cases are accounted for. It subsequently passes the [RISCOF] test suite when used with a RISC-V simulator.

[RISCOF]: https://riscof.readthedocs.io/en/stable/

# License

This project is based on code from the following repositories:

 * https://github.com/ucb-bar/berkeley-softfloat-3 (BSD 3-clause license)
 * https://github.com/chipshort/softfloat-c (BSD 3-clause license)
 * https://github.com/dalance/softfloat-wrapper (Apache-2.0 or MIT license)

This project is considered a derivative work of berkeley-softfloat-3 and softfloat-wrapper with respective licenses clearly marked in the source. Any other files without clear licensing can be used under any of the above licenses (BSD-3, Apache-2, MIT).

# Notes

As with every yak that gets shaved, all the bike sheds being painted, and rabbit holes getting discovered and subsequently traversed - sometime's it's good to record the Why and the How for prosperity and possibly even as justification of the time wasted and late-night espresso drinking.

> Initially I was building a RISC-V 32 simulator but wanted to run Go and Rust code, so I turned it into a RV64 simulator as neither Go nor Rust support RV32 out of the box, then decided to verify my RV64 implementation using the RISCOF tests, then discovered that the Go compiler emits many floating point instructions so implemented them and the RISCOF tests showed my naive floating point implementation (for the F and D extensions) didn't match the reference / conformance tests... I tried to get the `softfloat-sys` library working (which links against the Berkeley softfloat C library) but that wouldn't work in some of the environments I was targeting (SGX, WASM) or if it did work it wasn't as straightforward as I wanted.
>
> Then I discovered the `softfloat-c` crate which used C2Rust to translate the library into something I could easily include in my project, but then the RISCOF tests were still failing, so I built a `floatverify` tool that worked with the Testfloat `testfloat_gen` tool and discovered to my surprise that it had been translated using the `8086` target rather than the `RISCV` target meaning it wouldn't pass the RISCOF tests... and I couldn't get C2Rust working, and really I wanted some nice pure const Rust code that was guaranteed not to panic.
>
> After manually translating the subset of the Softfloat library from C to Rust, while cross-verifying against Testfloat, and removing all the global variables then massaging `softfloat-wrapper` into an `FPU` trait which keeps track of the exception-flags...
>
> *... a couple of weeks & several iterations later ...*
>
> Now everything works! Pure rust, thread- & async-safe, well tested, veary naiiicee!

## Other Links

### Notable floating point libraries:

 * https://github.com/rust-lang/rustc_apfloat
 * https://bellard.org/softfp/ (small C implementation)

### Test Suites

 * https://github.com/ucb-bar/berkeley-testfloat-3
 * https://github.com/PRUNERS/FLiT ?
 * https://github.com/FPBench/FPBench ?
