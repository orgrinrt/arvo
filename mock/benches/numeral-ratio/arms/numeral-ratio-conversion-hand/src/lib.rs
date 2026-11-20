//! The conversion, hand: see `bench-numeral-ratio-shared`.

use core::hint::black_box;

use bench_numeral_ratio_shared::{conversion_hand, output_size};
use mockspace_bench_core::{FfiBenchCall, abi_hash, timed};

#[unsafe(no_mangle)]
pub unsafe extern "C" fn bench_entry(
    input_ptr: *const u8,
    output_ptr: *mut u8,
    n: usize,
) -> FfiBenchCall {
    let input = unsafe { core::slice::from_raw_parts(input_ptr, n) };
    let output = unsafe { core::slice::from_raw_parts_mut(output_ptr, output_size(n)) };
    timed! {
        run {
            conversion_hand(black_box(input), output);
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn bench_name() -> *const u8 {
    b"numeral_ratio_conversion_hand\0".as_ptr()
}

#[unsafe(no_mangle)]
pub extern "C" fn bench_abi_hash() -> u64 {
    abi_hash()
}

#[unsafe(no_mangle)]
pub extern "C" fn bench_output_size(n: usize) -> usize {
    output_size(n)
}
