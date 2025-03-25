#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut,
    unused_imports,
    unused_parens,
    clippy::module_name_repetitions,
    clippy::unnecessary_cast,
    clippy::cast_sign_loss,
    clippy::cast_possible_wrap,
    clippy::cast_possible_truncation,
    clippy::needless_return,
    clippy::too_many_lines,
    clippy::useless_let_if_seq,
    clippy::if_not_else
)]

mod f32_add;
mod f32_classify;
mod f32_div;
mod f32_eq;
mod f32_eq_signaling;
mod f32_isSignalingNaN;
mod f32_le;
mod f32_le_quiet;
mod f32_lt;
mod f32_lt_quiet;
mod f32_mul;
mod f32_mulAdd;
mod f32_rem;
mod f32_roundToInt;
mod f32_sqrt;
mod f32_sub;
mod f32_to_f64;
mod f32_to_i32;
mod f32_to_i64;
mod f32_to_ui32;
mod f32_to_ui64;
mod f64_add;
mod f64_classify;
mod f64_div;
mod f64_eq;
mod f64_eq_signaling;
mod f64_isSignalingNaN;
mod f64_le;
mod f64_le_quiet;
mod f64_lt;
mod f64_lt_quiet;
mod f64_mul;
mod f64_mulAdd;
mod f64_rem;
mod f64_roundToInt;
mod f64_sqrt;
mod f64_sub;
mod f64_to_f32;
mod f64_to_i32;
mod f64_to_i64;
mod f64_to_ui32;
mod f64_to_ui64;
mod i32_to_f32;
mod i32_to_f64;
mod i64_to_f32;
mod i64_to_f64;
mod s_add128;
mod s_addMagsF32;
mod s_addMagsF64;
mod s_approxRecip32_1;
mod s_approxRecipSqrt32_1;
mod s_approxRecipSqrt_1Ks;
mod s_approxRecip_1Ks;
mod s_countLeadingZeros32;
mod s_countLeadingZeros64;
mod s_mul64To128;
mod s_mulAddF32;
mod s_mulAddF64;
mod s_normRoundPackToF32;
mod s_normRoundPackToF64;
mod s_normSubnormalF32Sig;
mod s_normSubnormalF64Sig;
mod s_roundPackToF32;
mod s_roundPackToF64;
mod s_roundToI32;
mod s_roundToI64;
mod s_roundToUI32;
mod s_roundToUI64;
mod s_shiftRightJam128;
mod s_shiftRightJam32;
mod s_shiftRightJam64;
mod s_shiftRightJam64Extra;
mod s_shortShiftLeft128;
mod s_shortShiftRightJam128;
mod s_shortShiftRightJam64;
mod s_sub128;
mod s_subMagsF32;
mod s_subMagsF64;
mod ui32_to_f32;
mod ui32_to_f64;
mod ui64_to_f32;
mod ui64_to_f64;

mod internals;
pub use internals::*;

mod riscv;
pub use riscv::*;

mod types;
pub use types::*;

pub const softfloat_tininess_afterRounding: u8 = 1;
pub const softfloat_tininess_beforeRounding: u8 = 0;

pub const softfloat_flag_invalid: u8 = 16;
pub const softfloat_flag_infinite: u8 = 8;
pub const softfloat_flag_overflow: u8 = 4;
pub const softfloat_flag_underflow: u8 = 2;
pub const softfloat_flag_inexact: u8 = 1;

pub const softfloat_round_near_even: u8 = 0;
pub const softfloat_round_minMag: u8 = 1;
pub const softfloat_round_min: u8 = 2;
pub const softfloat_round_max: u8 = 3;
pub const softfloat_round_near_maxMag: u8 = 4;
pub const softfloat_round_odd: u8 = 6;

pub use f32_add::f32_add;
pub use f32_classify::f32_classify;
pub use f32_div::f32_div;
pub use f32_eq::f32_eq;
pub use f32_eq_signaling::f32_eq_signaling;
pub use f32_isSignalingNaN::f32_isSignalingNaN;
pub use f32_le::f32_le;
pub use f32_le_quiet::f32_le_quiet;
pub use f32_lt::f32_lt;
pub use f32_lt_quiet::f32_lt_quiet;
pub use f32_mul::f32_mul;
pub use f32_mulAdd::f32_mulAdd;
pub use f32_rem::f32_rem;
pub use f32_roundToInt::f32_roundToInt;
pub use f32_sqrt::f32_sqrt;
pub use f32_sub::f32_sub;
pub use f32_to_f64::f32_to_f64;
pub use f32_to_i32::f32_to_i32;
pub use f32_to_i64::f32_to_i64;
pub use f32_to_ui32::f32_to_ui32;
pub use f32_to_ui64::f32_to_ui64;
pub use f64_add::f64_add;
pub use f64_classify::f64_classify;
pub use f64_div::f64_div;
pub use f64_eq::f64_eq;
pub use f64_eq_signaling::f64_eq_signaling;
pub use f64_isSignalingNaN::f64_isSignalingNaN;
pub use f64_le::f64_le;
pub use f64_le_quiet::f64_le_quiet;
pub use f64_lt::f64_lt;
pub use f64_lt_quiet::f64_lt_quiet;
pub use f64_mul::f64_mul;
pub use f64_mulAdd::f64_mulAdd;
pub use f64_rem::f64_rem;
pub use f64_roundToInt::f64_roundToInt;
pub use f64_sqrt::f64_sqrt;
pub use f64_sub::f64_sub;
pub use f64_to_f32::f64_to_f32;
pub use f64_to_i32::f64_to_i32;
pub use f64_to_i64::f64_to_i64;
pub use f64_to_ui32::f64_to_ui32;
pub use f64_to_ui64::f64_to_ui64;
pub use i32_to_f32::i32_to_f32;
pub use i32_to_f64::i32_to_f64;
pub use i64_to_f32::i64_to_f32;
pub use i64_to_f64::i64_to_f64;
pub use s_addMagsF32::softfloat_addMagsF32;
pub use s_addMagsF64::softfloat_addMagsF64;
pub use s_approxRecip32_1::softfloat_approxRecip32_1;
pub use s_approxRecipSqrt32_1::softfloat_approxRecipSqrt32_1;
pub use s_approxRecipSqrt_1Ks::softfloat_approxRecipSqrt_1k0s;
pub use s_approxRecipSqrt_1Ks::softfloat_approxRecipSqrt_1k1s;
pub use s_countLeadingZeros32::softfloat_countLeadingZeros32;
pub use s_countLeadingZeros64::softfloat_countLeadingZeros64;

pub use s_add128::softfloat_add128;
pub use s_approxRecip_1Ks::softfloat_approxRecip_1k0s;
pub use s_approxRecip_1Ks::softfloat_approxRecip_1k1s;
pub use s_mul64To128::softfloat_mul64To128;
pub use s_mulAddF32::softfloat_mulAddF32;
pub use s_mulAddF64::softfloat_mulAddF64;
pub use s_normRoundPackToF32::softfloat_normRoundPackToF32;
pub use s_normRoundPackToF64::softfloat_normRoundPackToF64;
pub use s_normSubnormalF32Sig::softfloat_normSubnormalF32Sig;
pub use s_normSubnormalF64Sig::softfloat_normSubnormalF64Sig;
pub use s_roundPackToF32::*;
pub use s_roundPackToF64::softfloat_roundPackToF64;
pub use s_roundToI32::softfloat_roundToI32;
pub use s_roundToI64::softfloat_roundToI64;
pub use s_roundToUI32::softfloat_roundToUI32;
pub use s_roundToUI64::softfloat_roundToUI64;
pub use s_shiftRightJam128::softfloat_shiftRightJam128;
pub use s_shiftRightJam32::softfloat_shiftRightJam32;
pub use s_shiftRightJam64::softfloat_shiftRightJam64;
pub use s_shiftRightJam64Extra::softfloat_shiftRightJam64Extra;
pub use s_shortShiftLeft128::softfloat_shortShiftLeft128;
pub use s_shortShiftRightJam128::softfloat_shortShiftRightJam128;
pub use s_shortShiftRightJam64::softfloat_shortShiftRightJam64;
pub use s_sub128::softfloat_sub128;
pub use s_subMagsF32::softfloat_subMagsF32;
pub use s_subMagsF64::softfloat_subMagsF64;
pub use ui32_to_f32::ui32_to_f32;
pub use ui32_to_f64::ui32_to_f64;
pub use ui64_to_f32::ui64_to_f32;
pub use ui64_to_f64::ui64_to_f64;
