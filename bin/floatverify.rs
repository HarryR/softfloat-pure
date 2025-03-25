// SPDX-License-Identifier: BSD-3-Clause

use std::env;
use std::io::{self, BufRead, Write};
use std::process::exit;
use std::str::FromStr;

use softfloat_pure::wrapper::{Float, RoundingMode, TininessMode};
use softfloat_pure::{float32_t, float64_t, FPU};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum IntType {
    Ui32,
    Ui64,
    I32,
    I64,
}

impl FromStr for IntType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ui32" => Ok(Self::Ui32),
            "ui64" => Ok(Self::Ui64),
            "i32" => Ok(Self::I32),
            "i64" => Ok(Self::I64),
            _ => Err(format!("Unknown integer type: {s}")),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum FloatType {
    F32,
    F64,
}

impl FromStr for FloatType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "f32" => Ok(Self::F32),
            "f64" => Ok(Self::F64),
            _ => Err(format!("Unknown float type: {s}")),
        }
    }
}

#[derive(Debug, Clone)]
enum SingleOperandFn {
    ConvertIntToFloat(IntType, FloatType),
    ConvertFloatToInt(FloatType, IntType),
    ConvertFloatToFloat(FloatType, FloatType),
    RoundToInt(FloatType),
    Sqrt(FloatType),
}

#[derive(Debug, Clone)]
enum DualOperandFn {
    Add,
    Sub,
    Mul,
    Div,
    Rem,
    Eq,
    Le,
    Lt,
    EqSignaling,
    LeQuiet,
    LtQuiet,
}

#[derive(Debug, Clone)]
enum TriOperandFn {
    MulAdd,
}

#[derive(Debug, Clone)]
enum TestType {
    SingleOperand(SingleOperandFn),
    TwoOperands(FloatType, DualOperandFn),
    ThreeOperands(FloatType, TriOperandFn),
}

#[derive(Debug, Clone)]
struct TestConfig {
    test_type: TestType,
    round_mode: RoundingMode,
    detect_tininess: TininessMode,
    exact: bool,
    exit_on_error: bool,
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        print_usage();
        exit(1);
    }

    let config = parse_args(&args);

    // Process the test cases from stdin
    let stdin = io::stdin();
    let mut stdin_lock = stdin.lock();
    let mut line = String::new();
    let mut test_count = 0;
    let mut failure_count = 0;

    while stdin_lock.read_line(&mut line)? > 0 {
        // Skip empty lines or lines starting with '#'
        if line.trim().is_empty() || line.trim().starts_with('#') {
            line.clear();
            continue;
        }

        // Parse the test case
        let test_result = process_test(&config, &line);
        test_count += 1;

        match test_result {
            Ok(()) => {}
            Err(err) => {
                println!("Test failure in line {test_count}: {err}");
                failure_count += 1;
                if config.exit_on_error {
                    io::stdout().flush()?;
                    exit(1);
                }
            }
        }

        line.clear();
    }

    // Print summary
    println!("Tests run: {test_count}, Failures: {failure_count}");

    if failure_count > 0 {
        io::stdout().flush()?;
        exit(1);
    }

    Ok(())
}

fn print_usage() {
    println!("Usage: floatverify <function> [options]");
    println!();
    println!(" # Please note this takes most of the same arguments as `testfloat_gen`");
    println!(" # from the berkeley-testfloat-3 package, they are designed for use together.");
    println!(" # This verifies that the softfloat implementation used by the emulator");
    println!(" # matches the reference floating point implementation for RISC-V.");
    println!(" # To run full permutation of tests use the floatverify.py script");
    println!(" #");
    println!(" # See: https://github.com/ucb-bar/berkeley-testfloat-3");
    println!();
    println!("Example Usage:");
    println!();
    println!(" ./testfloat_gen f32_to_f64 | ./floatverify f32_to_f64");
    println!();
    println!("Arguments:");
    println!();
    println!("  <function>:");
    println!("    <int>_to_<float>     <float>_add      <float>_eq");
    println!("    <float>_to_<int>     <float>_sub      <float>_le");
    println!("    <float>_to_<float>   <float>_mul      <float>_lt");
    println!("    <float>_roundToInt   <float>_mulAdd   <float>_eq_signaling");
    println!("                         <float>_div      <float>_le_quiet");
    println!("                         <float>_rem      <float>_lt_quiet");
    println!("                         <float>_sqrt");
    println!();
    println!("  [options]:");
    println!("    -rnear_even      --Round to nearest/even (default)");
    println!("    -rminMag         --Round to minimum magnitude (toward zero)");
    println!("    -rmin            --Round to minimum (down)");
    println!("    -rmax            --Round to maximum (up)");
    println!("    -rnear_maxMag    --Round to nearest/maximum magnitude");
    println!("    -rodd            --Round to odd (jamming).  (For rounding to an integer value, rounds to minimum magnitude instead.)");
    println!("    -tininessbefore  --Detect underflow tininess before rounding");
    println!("    -tininessafter   --Detect underflow tininess after rounding (default)");
    println!("    -exact           --Rounding to integer is exact");
    println!("    -notexact        --Rounding to integer is not exact (default)");
    println!("    -exit            --Exit after first error (default)");
    println!("    -noexit          --Don't exit on first error");
    println!();
    println!("  <int>:");
    println!("    ui32             --Unsigned 32-bit integer.");
    println!("    ui64             --Unsigned 64-bit integer.");
    println!("    i32              --Signed 32-bit integer.");
    println!("    i64              --Signed 64-bit integer.");
    println!();
    println!("  <float>:");
    println!("    f32              --Binary 32-bit floating-point (single-precision).");
    println!("    f64              --Binary 64-bit floating-point (double-precision).");
    println!();
}

fn parse_function(function: &str) -> Result<TestType, String> {
    if function.contains("_to_") {
        let parts: Vec<&str> = function.split("_to_").collect();
        if parts.len() != 2 {
            return Err(format!("Invalid _to_ function format: {function}"));
        }

        let source = parts[0];
        let dest = parts[1];

        if let Ok(int_type) = IntType::from_str(source) {
            if let Ok(float_type) = FloatType::from_str(dest) {
                return Ok(TestType::SingleOperand(SingleOperandFn::ConvertIntToFloat(
                    int_type, float_type,
                )));
            }
        } else if let Ok(float_type) = FloatType::from_str(source) {
            if let Ok(int_type) = IntType::from_str(dest) {
                return Ok(TestType::SingleOperand(SingleOperandFn::ConvertFloatToInt(
                    float_type, int_type,
                )));
            } else if let Ok(dest_float_type) = FloatType::from_str(dest) {
                return Ok(TestType::SingleOperand(
                    SingleOperandFn::ConvertFloatToFloat(float_type, dest_float_type),
                ));
            }
        }

        return Err(format!("Invalid conversion function: {function}"));
    } else if function.ends_with("_roundToInt") {
        let float_part = &function[0..function.len() - 11]; // Remove "_roundToInt"
        if let Ok(float_type) = FloatType::from_str(float_part) {
            return Ok(TestType::SingleOperand(SingleOperandFn::RoundToInt(
                float_type,
            )));
        }
        return Err(format!("Invalid roundToInt function: {function}"));
    } else if function.ends_with("_sqrt") {
        let float_part = &function[0..function.len() - 5]; // Remove "_sqrt"
        if let Ok(float_type) = FloatType::from_str(float_part) {
            return Ok(TestType::SingleOperand(SingleOperandFn::Sqrt(float_type)));
        }
        return Err(format!("Invalid sqrt function: {function}"));
    } else if function.ends_with("_mulAdd") {
        let float_part = &function[0..function.len() - 7]; // Remove "_mulAdd"
        if let Ok(float_type) = FloatType::from_str(float_part) {
            return Ok(TestType::ThreeOperands(float_type, TriOperandFn::MulAdd));
        }
        return Err(format!("Invalid mulAdd function: {function}"));
    }
    // Parse dual operand functions
    if let Some((prefix, suffix)) = function.split_once('_') {
        if let Ok(float_type) = FloatType::from_str(prefix) {
            match suffix {
                "add" => return Ok(TestType::TwoOperands(float_type, DualOperandFn::Add)),
                "sub" => return Ok(TestType::TwoOperands(float_type, DualOperandFn::Sub)),
                "mul" => return Ok(TestType::TwoOperands(float_type, DualOperandFn::Mul)),
                "div" => return Ok(TestType::TwoOperands(float_type, DualOperandFn::Div)),
                "rem" => return Ok(TestType::TwoOperands(float_type, DualOperandFn::Rem)),
                "eq" => return Ok(TestType::TwoOperands(float_type, DualOperandFn::Eq)),
                "le" => return Ok(TestType::TwoOperands(float_type, DualOperandFn::Le)),
                "lt" => return Ok(TestType::TwoOperands(float_type, DualOperandFn::Lt)),
                "eq_signaling" => {
                    return Ok(TestType::TwoOperands(
                        float_type,
                        DualOperandFn::EqSignaling,
                    ))
                }
                "le_quiet" => return Ok(TestType::TwoOperands(float_type, DualOperandFn::LeQuiet)),
                "lt_quiet" => return Ok(TestType::TwoOperands(float_type, DualOperandFn::LtQuiet)),
                _ => return Err(format!("Unknown operation: {suffix}")),
            }
        }
    } else {
        return Err(format!("Invalid dual operand function format: {function}"));
    }

    Err(format!("Invalid function: {function}"))
}

fn display_flags(flags: u8, prefix: &str) {
    if (flags & 1) != 0 {
        println!("{prefix}inexact");
    }

    if (flags & 2) != 0 {
        println!("{prefix}underflow");
    }

    if (flags & 4) != 0 {
        println!("{prefix}overflow");
    }

    if (flags & 8) != 0 {
        println!("{prefix}infinite");
    }

    if (flags & 16) != 0 {
        println!("{prefix}invalid");
    }
}

fn display_float(float_type: FloatType, v: u64, prefix: &str) {
    match float_type {
        FloatType::F32 => {
            #[allow(clippy::cast_possible_truncation)]
            let f = float32_t::from_bits(v as u32);
            println!("{prefix}bits: {:032b}", f.to_bits());
            println!("{prefix}hex: {:8x}", f.to_bits());
            println!("{prefix}value: {}", f32::from_bits(f.to_bits()));
            println!("{prefix}fraction: {}", f.fraction());
            println!("{prefix}exponent: {}", f.exponent());
            println!("{prefix}is_nan: {}", f.is_nan());
            println!("{prefix}is_zero: {}", f.is_zero());
            println!("{prefix}is_subnormal: {}", f.is_subnormal());
            println!("{prefix}is_positive: {}", f.is_positive());
            println!("{prefix}is_negative: {}", f.is_negative());
            println!("{prefix}is_negative_zero: {}", f.is_negative_zero());
            println!(
                "{prefix}is_negative_subnormal: {}",
                f.is_negative_subnormal()
            );
            println!("{prefix}is_negative_normal: {}", f.is_negative_normal());
            println!("{prefix}is_negative_infinity: {}", f.is_negative_infinity());
            println!("{prefix}is_positive_zero: {}", f.is_positive_zero());
            println!(
                "{prefix}is_positive_subnormal: {}",
                f.is_positive_subnormal()
            );
            println!("{prefix}is_positive_normal: {}", f.is_positive_normal());
            println!("{prefix}is_positive_infinity: {}", f.is_positive_infinity());
        }
        FloatType::F64 => {
            let f = float64_t::from_bits(v);
            println!("{prefix}bits: {:064b}", f.to_bits());
            println!("{prefix}hex: {:16x}", f.to_bits());
            println!("{prefix}value: {}", f64::from_bits(f.to_bits()));
            println!("{prefix}fraction: {}", f.fraction());
            println!("{prefix}exponent: {}", f.exponent());
            println!("{prefix}is_nan: {}", f.is_nan());
            println!("{prefix}is_zero: {}", f.is_zero());
            println!("{prefix}is_subnormal: {}", f.is_subnormal());
            println!("{prefix}is_positive: {}", f.is_positive());
            println!("{prefix}is_negative: {}", f.is_negative());
            println!("{prefix}is_negative_zero: {}", f.is_negative_zero());
            println!(
                "{prefix}is_negative_subnormal: {}",
                f.is_negative_subnormal()
            );
            println!("{prefix}is_negative_normal: {}", f.is_negative_normal());
            println!("{prefix}is_negative_infinity: {}", f.is_negative_infinity());
            println!("{prefix}is_positive_zero: {}", f.is_positive_zero());
            println!(
                "{prefix}is_positive_subnormal: {}",
                f.is_positive_subnormal()
            );
            println!("{prefix}is_positive_normal: {}", f.is_positive_normal());
            println!("{prefix}is_positive_infinity: {}", f.is_positive_infinity());
        }
    }
}

fn parse_args(args: &[String]) -> TestConfig {
    let function = &args[1];

    // Parse the function to determine the test type
    let test_type = match parse_function(function) {
        Ok(test_type) => test_type,
        Err(err) => {
            eprintln!("{err}");
            print_usage();
            exit(1);
        }
    };

    // Default settings
    let mut round_mode = RoundingMode::RneTiesToEven;
    let mut detect_tininess = TininessMode::After;
    let mut exact = false;
    let mut exit_on_error = true;

    // Parse optional arguments
    for arg in &args[2..] {
        match arg.as_str() {
            "-rnear_even" => round_mode = RoundingMode::RneTiesToEven,
            "-rminMag" => round_mode = RoundingMode::RtzTowardZero,
            "-rmin" => round_mode = RoundingMode::RdnTowardNegative,
            "-rmax" => round_mode = RoundingMode::RupTowardPositive,
            "-rnear_maxMag" => round_mode = RoundingMode::RmmTiesToAway,
            "-rodd" => round_mode = RoundingMode::Rodd,
            "-tininessbefore" => detect_tininess = TininessMode::Before,
            "-tininessafter" => detect_tininess = TininessMode::After,
            "-exact" => exact = true,
            "-notexact" => exact = false,
            "-exit" => exit_on_error = true,
            "-noexit" => exit_on_error = false,
            _ => {
                eprintln!("Unknown option: {arg}");
                print_usage();
                exit(1);
            }
        }
    }

    TestConfig {
        test_type,
        round_mode,
        detect_tininess,
        exact,
        exit_on_error,
    }
}

fn process_test(config: &TestConfig, line: &str) -> Result<(), String> {
    let parts: Vec<&str> = line.split_whitespace().collect();

    let mut fpu = FPU::new(config.detect_tininess);

    match &config.test_type {
        TestType::SingleOperand(_) => {
            if parts.len() != 3 {
                return Err(format!(
                    "Expected 3 fields for single-operand test, got {}",
                    parts.len()
                ));
            }

            let input = parse_hex(parts[0])?;
            let expected_output = parse_hex(parts[1])?;
            let expected_flags = parse_hex_u8(parts[2])?;

            // Call your implementation function here
            let actual_output = process_single_operand(&mut fpu, config, input)?;

            let actual_flags = fpu.flags.to_bits();

            // Compare results
            if actual_output != expected_output || actual_flags != expected_flags {
                if actual_flags != expected_flags {
                    display_flags(actual_flags, "\tactflags: ");
                    display_flags(expected_flags, "\texpflags: ");
                }
                return Err(format!(
                    "Mismatch: input 0x{input:X}  |  expected 0x{expected_output:X} flags 0x{expected_flags:02X}  |  got 0x{actual_output:X}  flags 0x{actual_flags:02X}"
                ));
            }
        }
        TestType::TwoOperands(float_type, _) => {
            if parts.len() != 4 {
                return Err(format!(
                    "Expected 4 fields for two-operand test, got {}",
                    parts.len()
                ));
            }

            let input1 = parse_hex(parts[0])?;
            let input2 = parse_hex(parts[1])?;
            let expected_output = parse_hex(parts[2])?;
            let expected_flags = parse_hex_u8(parts[3])?;

            // Call your implementation function here
            let actual_output = process_two_operands(&mut fpu, config, input1, input2)?;

            let actual_flags = fpu.flags.to_bits();

            // Compare results
            if actual_output != expected_output || actual_flags != expected_flags {
                if actual_output != expected_output {
                    display_float(*float_type, input1, "\tinput1  ");
                    display_float(*float_type, input2, "\tinput2  ");
                    display_float(*float_type, expected_output, "\texpout  ");
                    display_float(*float_type, actual_output, "\tactout  ");
                }
                if actual_flags != expected_flags {
                    display_flags(actual_flags, "\tactflags: ");
                    display_flags(expected_flags, "\texpflags: ");
                }
                return Err(format!(
                    "Mismatch: input1 0x{input1:X}  input2 0x{input2:X}  |  expected 0x{expected_output:X} flags 0x{expected_flags:02X}  |  got 0x{actual_output:X}  flags 0x{actual_flags:02X}"
                ));
            }
        }
        TestType::ThreeOperands(..) => {
            if parts.len() != 5 {
                return Err(format!(
                    "Expected 5 fields for three-operand test, got {}",
                    parts.len()
                ));
            }

            let input1 = parse_hex(parts[0])?;
            let input2 = parse_hex(parts[1])?;
            let input3 = parse_hex(parts[2])?;
            let expected_output = parse_hex(parts[3])?;
            let expected_flags = parse_hex_u8(parts[4])?;

            // Call your implementation function here
            let (float_type, actual_output) =
                process_three_operands(&mut fpu, config, input1, input2, input3)?;

            let actual_flags = fpu.flags.to_bits();

            // Compare results
            if actual_output != expected_output || actual_flags != expected_flags {
                if actual_output != expected_output {
                    display_float(float_type, input1, "\tinput1  ");
                    display_float(float_type, input2, "\tinput2  ");
                    display_float(float_type, input3, "\tinput3  ");
                    display_float(float_type, expected_output, "\texpout  ");
                    display_float(float_type, actual_output, "\tactout  ");
                }
                if actual_flags != expected_flags {
                    display_flags(actual_flags, "\tactflags: ");
                    display_flags(expected_flags, "\texpflags: ");
                }

                return Err(format!(
                    "Mismatch: input1 0x{input1:X}  input2 0x{input2:X}  input3 0x{input3:X}  |  expected 0x{expected_output:X}  flags 0x{expected_flags:02X}  |  got 0x{actual_output:X}  flags 0x{actual_flags:02X}"
                ));
            }
        }
    }

    Ok(())
}

// Helper function to parse a hex string into u64
fn parse_hex(hex_str: &str) -> Result<u64, String> {
    u64::from_str_radix(hex_str, 16)
        .map_err(|e| format!("Failed to parse hex value '{hex_str}': {e}"))
}

// Helper function to parse a hex string into u8
fn parse_hex_u8(hex_str: &str) -> Result<u8, String> {
    u8::from_str_radix(hex_str, 16)
        .map_err(|e| format!("Failed to parse hex value '{hex_str}': {e}"))
}

const fn asf32(v: u64) -> float32_t {
    #[allow(clippy::cast_possible_truncation)]
    float32_t { v: v as u32 }
}

const fn asf64(v: u64) -> float64_t {
    float64_t { v }
}

#[allow(
    clippy::cast_sign_loss,
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap
)]
fn process_single_operand(fpu: &mut FPU, config: &TestConfig, input: u64) -> Result<u64, String> {
    let rnd = config.round_mode;
    let exact = config.exact;
    match &config.test_type {
        TestType::SingleOperand(op) => match op {
            SingleOperandFn::ConvertIntToFloat(int_type, float_type) => match float_type {
                FloatType::F32 => Ok(u64::from(
                    match int_type {
                        IntType::Ui32 => fpu.f32_from_u32(input as u32, rnd),
                        IntType::Ui64 => fpu.f32_from_u64(input, rnd),
                        IntType::I32 => fpu.f32_from_i32(input as u32 as i32, rnd),
                        IntType::I64 => fpu.f32_from_i64(input as i64, rnd),
                    }
                    .to_bits(),
                )),
                FloatType::F64 => Ok(match int_type {
                    IntType::Ui32 => fpu.f64_from_u32(input as u32),
                    IntType::Ui64 => fpu.f64_from_u64(input, rnd),
                    IntType::I32 => fpu.f64_from_i32(input as u32 as i32),
                    IntType::I64 => fpu.f64_from_i64(input as i64, rnd),
                }
                .to_bits()),
            },
            SingleOperandFn::ConvertFloatToInt(float_type, int_type) => match float_type {
                FloatType::F32 => {
                    let f = asf32(input);
                    Ok(match int_type {
                        IntType::Ui32 => u64::from(fpu.to_u32(f, rnd, exact)),
                        IntType::Ui64 => fpu.to_u64(f, rnd, exact),
                        IntType::I32 => u64::from(fpu.to_i32(f, rnd, exact) as u32),
                        IntType::I64 => fpu.to_i64(f, rnd, exact) as u64,
                    })
                }
                FloatType::F64 => {
                    let f = asf64(input);
                    Ok(match int_type {
                        IntType::Ui32 => u64::from(fpu.to_u32(f, rnd, exact)),
                        IntType::Ui64 => fpu.to_u64(f, rnd, exact),
                        IntType::I32 => u64::from(fpu.to_i32(f, rnd, exact) as u32),
                        IntType::I64 => fpu.to_i64(f, rnd, exact) as u64,
                    })
                }
            },
            SingleOperandFn::ConvertFloatToFloat(src_type, dest_type) => {
                match (src_type, dest_type) {
                    (FloatType::F32, FloatType::F64) => Ok(fpu.to_f64(asf32(input), rnd).to_bits()),
                    (FloatType::F64, FloatType::F32) => {
                        Ok(u64::from(fpu.to_f32(asf64(input), rnd).to_bits()))
                    }
                    _ => Err(format!(
                        "Unsupported float-to-float conversion: {src_type:?} to {dest_type:?}"
                    )),
                }
            }
            SingleOperandFn::RoundToInt(float_type) => match float_type {
                FloatType::F32 => Ok(u64::from(
                    fpu.round_to_int(asf32(input), rnd, exact).to_bits(),
                )),
                FloatType::F64 => Ok(fpu.round_to_int(asf64(input), rnd, exact).to_bits()),
            },
            SingleOperandFn::Sqrt(float_type) => match float_type {
                FloatType::F32 => Ok(u64::from(fpu.sqrt(asf32(input), rnd).to_bits())),
                FloatType::F64 => Ok(fpu.sqrt(asf64(input), rnd).to_bits()),
            },
        },
        _ => Err("Invalid test type for single operand function".to_string()),
    }
}

fn process_two_operands(
    fpu: &mut FPU,
    config: &TestConfig,
    input1: u64,
    input2: u64,
) -> Result<u64, String> {
    let rnd = config.round_mode;
    match &config.test_type {
        TestType::TwoOperands(float_type, op) => Ok(match op {
            DualOperandFn::Add => match float_type {
                FloatType::F32 => u64::from(fpu.add(asf32(input1), asf32(input2), rnd).to_bits()),
                FloatType::F64 => fpu.add(asf64(input1), asf64(input2), rnd).to_bits(),
            },
            DualOperandFn::Sub => match float_type {
                FloatType::F32 => u64::from(fpu.sub(asf32(input1), asf32(input2), rnd).to_bits()),
                FloatType::F64 => fpu.sub(asf64(input1), asf64(input2), rnd).to_bits(),
            },
            DualOperandFn::Mul => match float_type {
                FloatType::F32 => u64::from(fpu.mul(asf32(input1), asf32(input2), rnd).to_bits()),
                FloatType::F64 => fpu.mul(asf64(input1), asf64(input2), rnd).to_bits(),
            },
            DualOperandFn::Div => match float_type {
                FloatType::F32 => u64::from(fpu.div(asf32(input1), asf32(input2), rnd).to_bits()),
                FloatType::F64 => fpu.div(asf64(input1), asf64(input2), rnd).to_bits(),
            },
            DualOperandFn::Rem => match float_type {
                FloatType::F32 => u64::from(fpu.rem(asf32(input1), asf32(input2), rnd).to_bits()),
                FloatType::F64 => fpu.rem(asf64(input1), asf64(input2), rnd).to_bits(),
            },
            DualOperandFn::Eq => match float_type {
                FloatType::F32 => u64::from(fpu.eq(asf32(input1), asf32(input2))),
                FloatType::F64 => u64::from(fpu.eq(asf64(input1), asf64(input2))),
            },
            DualOperandFn::Le => match float_type {
                FloatType::F32 => u64::from(fpu.le(asf32(input1), asf32(input2))),
                FloatType::F64 => u64::from(fpu.le(asf64(input1), asf64(input2))),
            },
            DualOperandFn::Lt => match float_type {
                FloatType::F32 => u64::from(fpu.lt(asf32(input1), asf32(input2))),
                FloatType::F64 => u64::from(fpu.lt(asf64(input1), asf64(input2))),
            },
            DualOperandFn::EqSignaling => match float_type {
                FloatType::F32 => u64::from(fpu.eq_signaling(asf32(input1), asf32(input2))),
                FloatType::F64 => u64::from(fpu.eq_signaling(asf64(input1), asf64(input2))),
            },
            DualOperandFn::LeQuiet => match float_type {
                FloatType::F32 => u64::from(fpu.le_quiet(asf32(input1), asf32(input2))),
                FloatType::F64 => u64::from(fpu.le_quiet(asf64(input1), asf64(input2))),
            },
            DualOperandFn::LtQuiet => match float_type {
                FloatType::F32 => u64::from(fpu.lt_quiet(asf32(input1), asf32(input2))),
                FloatType::F64 => u64::from(fpu.lt_quiet(asf64(input1), asf64(input2))),
            },
        }),
        _ => Err("Invalid test type for two operand function".to_string()),
    }
}

fn process_three_operands(
    fpu: &mut FPU,
    config: &TestConfig,
    input1: u64,
    input2: u64,
    input3: u64,
) -> Result<(FloatType, u64), String> {
    let rnd = config.round_mode;
    match &config.test_type {
        TestType::ThreeOperands(float_type, op) => match op {
            TriOperandFn::MulAdd => Ok(match float_type {
                FloatType::F32 => (
                    FloatType::F32,
                    u64::from(
                        fpu.mul_add(asf32(input1), asf32(input2), asf32(input3), rnd)
                            .to_bits(),
                    ),
                ),
                FloatType::F64 => (
                    FloatType::F64,
                    fpu.mul_add(asf64(input1), asf64(input2), asf64(input3), rnd)
                        .to_bits(),
                ),
            }),
        },
        _ => Err("Invalid test type for three operand function".to_string()),
    }
}
