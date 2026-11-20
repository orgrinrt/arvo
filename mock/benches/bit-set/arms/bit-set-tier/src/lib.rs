//! `bit-set`: whether two 64-bit sets meet, through `arvo-bits`' bit tier.
//!
//! `bit_set_meet`, `bit_set_union` and `bit_set_is_empty` are the three
//! operations `obligation::set_operations_over_a_fixed_size_bit_set` names,
//! each through a function generic over `BitLogic` at `Bits<64>`, exported by
//! name so `cargo mock bench disasm` reads what each lowers to.

use core::hint::black_box;

use arvo_bits::{BitLogic, Bits};
use mockspace_bench_core::{FfiBenchCall, abi_hash, timed};

fn meet<B: BitLogic>(a: B, b: B) -> bool {
    !a.and(b).is_empty().get()
}

/// Whether the two sets share a member.
#[unsafe(no_mangle)]
pub extern "C" fn bit_set_meet(a: u64, b: u64) -> bool {
    meet(Bits::<64>::masked(a), Bits::<64>::masked(b))
}

/// The union of the two sets.
#[unsafe(no_mangle)]
pub extern "C" fn bit_set_union(a: u64, b: u64) -> u64 {
    BitLogic::or(Bits::<64>::masked(a), Bits::<64>::masked(b)).raw()
}

/// Whether the set is empty.
#[unsafe(no_mangle)]
pub extern "C" fn bit_set_is_empty(a: u64) -> bool {
    BitLogic::is_empty(Bits::<64>::masked(a)).get()
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
    b"bit_set_tier\0".as_ptr()
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
