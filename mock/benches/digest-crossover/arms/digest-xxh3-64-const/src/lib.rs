//! `digest-crossover`: `Xxh3::<64>::of_const` over a run of `n` bytes.
//!
//! The run reaches the digest as a slice whose length the optimiser cannot
//! see, through `black_box`, because the switched digest is called on slices
//! of a length known only at run time. A fixed `[u8; N]` would let the loop
//! unroll to the exact length and time a shape no caller has.

use core::hint::black_box;

use arvo_bits::Bytes;
use arvo_digest::Xxh3;
use mockspace_bench_core::{FfiBenchCall, abi_hash, timed};

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
            let bytes: &[u8] = black_box(input);
            *output = Xxh3::<64>::of_const(Bytes::of(bytes)).raw().raw().to_le_bytes();
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn bench_name() -> *const u8 {
    b"digest_xxh3_64_const\0".as_ptr()
}

#[unsafe(no_mangle)]
pub extern "C" fn bench_abi_hash() -> u64 {
    abi_hash()
}

/// The digest's eight bytes, at every length.
#[unsafe(no_mangle)]
pub extern "C" fn bench_output_size(_n: usize) -> usize {
    8
}
