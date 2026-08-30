//! Shared model for the contention bench: prices packing against carrier
//! width when several cores walk one column at once.
//!
//! ## What this adds to the carrier sweep
//!
//! `bitpack-carrier-width` measures one core. Its conclusion is that on one
//! core a dense read of a `u16` column is never bound by bytes delivered at any
//! size this host can hold, so the footprint saving buys nothing there, and
//! packing only pays once the carrier it replaces is wide enough (about 5.8 to
//! 7.0 bytes) for the dense arm to reach the memory system's ceiling.
//!
//! One core is not the declared workload. The substrate's framing is millions
//! of entities with thousands of systems mutating them per frame over
//! contiguous column-store storage, which is parallel. The measured single-core
//! figures say a `u64` walk already pulls 53 to 55 GB/s on this part, which is
//! around 80 percent of its theoretical 68.25 GB/s. A machine in that state
//! does not acquire contention gradually at eight cores. It acquires it at two.
//!
//! So this crate runs the identical kernels over the identical column, split
//! `T` ways.
//!
//! ## The shape of the contention, and what it biases
//!
//! One column, `T` threads, disjoint contiguous slices, one pass. Thread `i`
//! walks its slice of the same region the single-threaded arms walk, with the
//! same kernel. The timed region covers the whole pass, so wall time divided by
//! `N` stays directly comparable to the carrier sweep's numbers, and the
//! `T = 1` row is a cross-check against committed csvs rather than a new claim.
//!
//! That is the honest shape rather than the favourable one, and it is
//! deliberately the less favourable of the two available. Giving each thread its
//! own full column would multiply aggregate footprint by `T` and drive every arm
//! into the bandwidth wall sooner, exaggerating the effect. Splitting one column
//! holds aggregate bytes fixed at `N * w` and raises only the rate at which they
//! are demanded, which is what a morselled column pass actually does.
//!
//! No artificial pressure is added. No sibling process competes. No cache is
//! flushed beyond what the harness's own cold mode already does.
//!
//! ## Why a persistent pool rather than spawning per call
//!
//! At `n = 1048576` a single pass costs roughly 90 microseconds. Spawning four
//! threads on macOS costs tens of microseconds, so a spawn-per-call arm would be
//! reporting thread-creation cost with a column walk attached. The pool is
//! created once per worker process and the timed region only publishes a job and
//! waits, which is one release store and a spin. That cost is common to every
//! arm, and it is measured directly by the small-`n` rows where the column is
//! cache resident and any deviation from ideal scaling is the barrier rather
//! than the memory system.
//!
//! ## Why every thread is pinned here rather than by the harness
//!
//! `bench-core/src/counter.rs:139` pins the worker with
//! `pthread_set_qos_class_self_np(0x21, 0)`, QoS `USER_INTERACTIVE`, which
//! biases to P cores. `bench-harness/src/config.rs:102-109` documents that a
//! bench declaring `threaded = true` opts out of that pin entirely, because a
//! spawned thread does not inherit it and pinning only the coordinator skews the
//! workload. On a 4P + 4E part an unpinned mix of both core types measures the
//! scheduler rather than the memory system, so this crate applies the same QoS
//! to every participating thread, the coordinator included.
//!
//! ## What this crate does not re-derive
//!
//! Every kernel is the carrier crate's, imported unmodified: `sum_d16`,
//! `sum_d32`, `sum_d64`, `sum_simd_padal`, and through it
//! `bench_bitpack_plan_shared::sum_windowed`. The layout is the carrier crate's
//! `CarrierColumn`, whose field sizes do not depend on its const parameter, so
//! `CarrierColumn<0>` names the identical layout with no const arithmetic in
//! type position.
//!
//! The one thing rebuilt is input construction, because the carrier crate's
//! builder takes its element count as a const parameter and this crate takes it
//! at runtime from `KEY`. `build_bytes` is tested for byte-for-byte equality
//! against the carrier crate's builder rather than assumed to agree with it.
//!
//! This crate is bench infrastructure, not shipping arvo source, matching every
//! sibling variant crate here: no `#![no_std]`, `std` used freely.

mod input;
mod kernels;
mod pool;
mod routine;
#[cfg(test)]
mod tests;

pub use input::{build_bytes, slice_bounds, Layout, MAX_THREADS};
pub use kernels::*;
pub use pool::column_pass;
pub use routine::Contend;

/// A slice kernel: read `[lo, hi)` of one carrier region and return the wrapping
/// sum of the decoded values.
///
/// A raw function pointer rather than a closure, because the pool is persistent
/// and shared across calls: a closure would have to be boxed on every timed
/// call, putting an allocation inside the measured region. The indirect call is
/// paid once per slice per pass, amortised over at least `N / T` elements, and
/// paid identically by every arm.
///
/// The base pointer is erased to `*const u8` so the pool can carry a layout it
/// does not know about; each kernel casts it back to the one it reads.
///
/// # Safety
/// `base` must point at a live layout of the kind the kernel expects, which must
/// outlive the call, and `lo <= hi <= N` for the `N` the input was built at.
pub type SliceKernel = unsafe fn(base: *const u8, lo: usize, hi: usize) -> u64;
