//! The sum taken through `Num::add` directly, for the contract-sum disassembly
//! reading against the generic arm.

use mockspace_bench_core::{FfiBenchCall, abi_hash, timed};

type I8Sat = arvo_numeral::Int<
    8,
    arvo_format::Adapt<arvo_format::rounding::TowardZero, arvo_format::overflow::Saturate>,
    arvo_numeral::Quiet,
>;
type I8Wrap = arvo_numeral::Int<
    8,
    arvo_format::Adapt<arvo_format::rounding::TowardZero, arvo_format::overflow::Wrap>,
    arvo_numeral::Quiet,
>;

/// The saturating eight-bit sum, exported so its disassembly can be read.
#[unsafe(no_mangle)]
pub extern "C" fn sum_i8_saturate(a: i8, b: i8) -> i8 {
    I8Sat::from_index(a).add(I8Sat::from_index(b)).index()
}

/// The wrapping eight-bit sum, exported so its disassembly can be read.
#[unsafe(no_mangle)]
pub extern "C" fn sum_i8_wrap(a: i8, b: i8) -> i8 {
    I8Wrap::from_index(a).add(I8Wrap::from_index(b)).index()
}

fn contract_sum_concrete_impl(input: &u64, output: &mut u64) {
    let a = *input as i8;
    let b = (*input >> 8) as i8;
    *output = (sum_i8_saturate(a, b) as u8 as u64) | ((sum_i8_wrap(a, b) as u8 as u64) << 8);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn bench_entry(
    input_ptr: *const u8,
    output_ptr: *mut u8,
    _n: usize,
) -> FfiBenchCall {
    let input = unsafe { &*(input_ptr as *const u64) };
    let output = unsafe { &mut *(output_ptr as *mut u64) };
    timed! {
        run { contract_sum_concrete_impl(input, output); }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn bench_name() -> *const u8 {
    b"contract_sum_concrete\0".as_ptr()
}

#[unsafe(no_mangle)]
pub extern "C" fn bench_abi_hash() -> u64 {
    abi_hash()
}

/// The bytes `bench_entry` writes at size `n`. The harness refuses the arm
/// when this is not the size of the routine's output, so change it with the
/// output type.
#[unsafe(no_mangle)]
pub extern "C" fn bench_output_size(_n: usize) -> usize {
    core::mem::size_of::<u64>()
}
