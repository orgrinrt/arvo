//! `packed-word`: hilavitkutin's identity handle as a `Word` of `arvo-bits`, a
//! 28-bit id at the bottom, a 3-bit kind above and a `Bool` flag on top, each
//! operation exported by name so `cargo mock bench disasm` reads what each
//! lowers to beside the arm written by hand.

use core::hint::black_box;

use arvo_bits::{Bits, Field, Layout, Word};
use arvo_format::{Bool, Width};
use mockspace_bench_core::{FfiBenchCall, abi_hash, timed};

struct Handle;
struct Id;
struct Kind;
struct Flag;

impl Layout for Handle {
    type Container = Bits<32>;
    type Fields = (Flag, (Kind, (Id, ())));
}

impl Field for Id {
    type Layout = Handle;
    type Value = Bits<28>;
    const BELOW: Width = Width::bits(0);
}

impl Field for Kind {
    type Layout = Handle;
    type Value = Bits<3>;
    const BELOW: Width = Width::bits(28);
}

impl Field for Flag {
    type Layout = Handle;
    type Value = Bool;
    const BELOW: Width = Width::bits(31);
}

/// The word in and out of its container.
#[unsafe(no_mangle)]
pub extern "C" fn packed_word_round_trip(raw: u32) -> u32 {
    Word::<Handle>::of(Bits::masked(raw as u64)).bits().raw() as u32
}

/// The 3-bit kind.
#[unsafe(no_mangle)]
pub extern "C" fn packed_word_kind(raw: u32) -> u32 {
    Word::<Handle>::of(Bits::masked(raw as u64)).get::<Kind>().raw() as u32
}

/// The word with its 28-bit id replaced.
#[unsafe(no_mangle)]
pub extern "C" fn packed_word_with_id(raw: u32, id: u32) -> u32 {
    Word::<Handle>::of(Bits::masked(raw as u64))
        .with::<Id>(Bits::masked(id as u64))
        .bits()
        .raw() as u32
}

/// The one-bit flag.
#[unsafe(no_mangle)]
pub extern "C" fn packed_word_flag(raw: u32) -> bool {
    Word::<Handle>::of(Bits::masked(raw as u64)).get::<Flag>().get()
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
    b"packed_word_word\0".as_ptr()
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
