//! `bit-set`: whether two 64-bit sets meet, in the host's own `u64`.
//!
//! The baseline the tier arm is read against: the same three operations
//! written on the host integer, exported by the same names.

use core::hint::black_box;

use mockspace_bench_core::{FfiBenchCall, abi_hash, timed};

/// Whether the two sets share a member.
#[unsafe(no_mangle)]
pub extern "C" fn bit_set_meet(a: u64, b: u64) -> bool {
    a & b != 0
}

/// The union of the two sets.
#[unsafe(no_mangle)]
pub extern "C" fn bit_set_union(a: u64, b: u64) -> u64 {
    a | b
}

/// Whether the set is empty.
#[unsafe(no_mangle)]
pub extern "C" fn bit_set_is_empty(a: u64) -> bool {
    a == 0
}

/// How many of the 16-byte records of the input hold two sets that meet.
fn meeting(input: &[u8]) -> u64 {
    let mut count = 0;
    for record in input.chunks_exact(16) {
        let a = u64::from_le_bytes(record[.. 8].try_into().unwrap());
        let b = u64::from_le_bytes(record[8 ..].try_into().unwrap());
        count += bit_set_meet(a, b) as u64;
    }
    count
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn bench_entry(
    input_ptr: *const u8,
    output_ptr: *mut u8,
    n: usize,
) -> FfiBenchCall {
    let input = unsafe { core::slice::from_raw_parts(input_ptr, n) };
    let output = unsafe { &mut *(output_ptr as *mut [u8; 8]) };
    timed! {
        run {
            *output = meeting(black_box(input)).to_le_bytes();
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn bench_name() -> *const u8 {
    b"bit_set_host\0".as_ptr()
}

#[unsafe(no_mangle)]
pub extern "C" fn bench_abi_hash() -> u64 {
    abi_hash()
}

/// The count's eight bytes, at every length.
#[unsafe(no_mangle)]
pub extern "C" fn bench_output_size(_n: usize) -> usize {
    8
}
