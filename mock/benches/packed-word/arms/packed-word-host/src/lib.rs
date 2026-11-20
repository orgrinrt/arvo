//! `packed-word`: hilavitkutin's identity handle written by hand on the host's
//! 32-bit integer, a 28-bit id at the bottom, a 3-bit kind above and a flag on
//! top. The arm the packed word is held against: each operation exported by
//! name so `cargo mock bench disasm` reads what each lowers to.

use core::hint::black_box;

use mockspace_bench_core::{FfiBenchCall, abi_hash, timed};

const ID: u32 = (1 << 28) - 1;
const KIND: u32 = 0b111 << 28;

/// The word in and out of its container.
#[unsafe(no_mangle)]
pub extern "C" fn packed_word_round_trip(raw: u32) -> u32 {
    raw
}

/// The 3-bit kind.
#[unsafe(no_mangle)]
pub extern "C" fn packed_word_kind(raw: u32) -> u32 {
    (raw & KIND) >> 28
}

/// The word with its 28-bit id replaced.
#[unsafe(no_mangle)]
pub extern "C" fn packed_word_with_id(raw: u32, id: u32) -> u32 {
    (raw & !ID) | (id & ID)
}

/// The one-bit flag.
#[unsafe(no_mangle)]
pub extern "C" fn packed_word_flag(raw: u32) -> bool {
    raw >> 31 == 1
}

/// Over the 8-byte records of the input, each a word and a value, the sum of
/// every word's kind, flag and word with its id replaced by the value.
fn folded(input: &[u8]) -> u64 {
    let mut sum = 0u64;
    for record in input.chunks_exact(8) {
        let raw = packed_word_round_trip(u32::from_le_bytes(record[.. 4].try_into().unwrap()));
        let value = u32::from_le_bytes(record[4 ..].try_into().unwrap());
        sum = sum
            .wrapping_add(packed_word_kind(raw) as u64)
            .wrapping_add(packed_word_flag(raw) as u64)
            .wrapping_add(packed_word_with_id(raw, value) as u64);
    }
    sum
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
            *output = folded(black_box(input)).to_le_bytes();
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn bench_name() -> *const u8 {
    b"packed_word_host\0".as_ptr()
}

#[unsafe(no_mangle)]
pub extern "C" fn bench_abi_hash() -> u64 {
    abi_hash()
}

/// The sum's eight bytes, at every length.
#[unsafe(no_mangle)]
pub extern "C" fn bench_output_size(_n: usize) -> usize {
    8
}
