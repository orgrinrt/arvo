//! Shared model for the write-contention bench: prices, and where naive, shows
//! the correctness cost of writing a packed column from several threads at
//! once, against the same question already answered for reads by
//! `bitpack-contend-shared`.
//!
//! ## Why this is a different question from the read case
//!
//! `26` and `27` split one column into `T` disjoint slices and read each with
//! the identical kernel every single-core arm uses, and both crates enforce a
//! period-aligned split (`KEY_SPLITS`) so no byte a thread reads is ever
//! written by another thread; reads never race regardless. A write does not
//! get that for free: a 13-bit field that starts mid-byte forces a
//! read-modify-write on the byte it starts in, and if that byte also carries
//! bits from the neighbouring thread's own first or last field, two threads
//! perform an unsynchronised read-modify-write on the same byte.
//!
//! Op's own framing of the exposure, quoted in `OPTIONS.md` Q32 and the reason
//! this crate exists: "a sub-byte packed field shared between threads is a
//! read-modify-write on a byte two threads both want."
//!
//! ## What "misaligned" means here, and why it is chosen rather than assumed
//!
//! A parallel scheduler that splits `N` elements into `T` roughly equal
//! morsels has no reason to know a particular column's packed period, and
//! nothing in the read bench's own `KEY_SPLITS` refusal generalises to a
//! caller outside this directory. So this crate runs the write side at two
//! kinds of `(N, T)` pair, chosen and pinned by a test rather than assumed:
//! **safe** pairs where every internal boundary lands on a period boundary,
//! and **race** pairs where it does not. The same `write_packed_plain` kernel
//! runs in both; only the `(N, T)` choice differs, which isolates the hazard
//! to the split rather than to anything about the encoder.
//!
//! ## Why a persistent pool rather than spawning per call
//!
//! Identical reasoning to `bitpack-contend-shared`: at these column sizes a
//! spawn costs more than the pass it would be attached to, so the pool is
//! created once per worker process and the timed region only publishes a job
//! and waits.
//!
//! ## Why every thread is pinned here rather than by the harness
//!
//! `threaded = true` benches skip the harness's own P-core pin (a spawned
//! thread never inherits it), so this crate applies the same QoS to every
//! participating thread itself, coordinator included, matching
//! `bitpack-contend-shared`'s own reasoning.
//!
//! This crate is bench infrastructure, not shipping arvo source, matching
//! every sibling variant crate here: no `#![no_std]`, `std` used freely.

mod input;
mod kernels;
mod pool;
mod routine;

pub use input::{build_bytes, slice_bounds, split_is_guarded, Layout, MAX_THREADS};
pub use kernels::{kern_dense_d16, kern_packed_guarded, kern_packed_plain, kern_packed_windowed};
pub use pool::write_pass;
pub use routine::{Sum, WriteContend};

/// A write kernel: read `vals[lo..hi]`, encode into `out`. `n` is the column's
/// true length, needed only by the guarded kernel to tell an internal
/// boundary from the column's own edge; the other two ignore it.
///
/// # Safety
/// `vals` holds at least `hi` elements and `out` holds the encoded form of at
/// least `hi` elements plus whatever headroom the encoding needs, and both
/// outlive the call.
pub type WriteKernel = unsafe fn(vals: *const u16, out: *mut u8, lo: usize, hi: usize, n: usize);

/// Decode the dense scratch region after a write pass and return its sum.
/// # Safety
/// `out` holds at least `n` valid `u16`s.
pub unsafe fn decode_dense_sum(out: *const u16, n: usize) -> u64 {
    let mut s = 0u64;
    unsafe {
        for i in 0..n {
            s = s.wrapping_add(*out.add(i) as u64);
        }
    }
    s
}

/// Decode the packed scratch region after a write pass and return its sum,
/// through the same independent `sum_naive` decoder every read bench in this
/// directory uses as its ground-truth check, so a write kernel's fidelity is
/// judged by a decoder none of the write kernels themselves share code with.
/// # Safety
/// `out` holds the packed encoding of at least `n` elements plus headroom.
pub unsafe fn decode_packed_sum(out: &[u8], n: usize) -> u64 {
    bench_bitpack_plan_shared::sum_naive(out, n)
}

mod stress;
