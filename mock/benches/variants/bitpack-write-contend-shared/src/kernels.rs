//! The write kernels: one dense baseline and two packed encoders that differ
//! only in how they treat the byte at a thread boundary.
//!
//! All three share one shape: read `vals[lo..hi]`, write the encoded form into
//! `out`. The dense kernel never has a hazard because every element occupies
//! whole, naturally aligned bytes, so no two threads' natural element
//! boundaries can land inside the same byte. The packed kernels can, because a
//! 13-bit field crosses a byte boundary wherever the running bit offset is not
//! a multiple of 8, and a thread's own first or last element may be exactly
//! that field.
//!
//! `write_packed_plain` never uses an atomic write. Called with a
//! period-aligned split it is correct, because no byte it touches is ever
//! touched by another thread. Called with a misaligned split it is the naive,
//! unguarded encoder any parallel writer would produce without thinking about
//! the packed representation, and it is included to demonstrate what that
//! costs, not as a candidate.
//!
//! `write_packed_guarded` is correct under either split: it detects the one
//! byte (if any) at each end of its own range that a neighbour could also be
//! writing, and takes that byte through an atomic fetch-or. Every other byte
//! it writes plain, so the fix is paid only at the boundary and nowhere else.

use std::sync::atomic::{AtomicU8, Ordering};

use bench_bitpack_plan_shared::{LOGICAL_BITS, MASK13};

use crate::input::split_is_guarded;

// ── the dense baseline ──────────────────────────────────────────────────────

/// # Safety
/// `vals` and `out` each hold at least `hi` elements and no other thread
/// writes `out[lo..hi]` concurrently with this call.
pub unsafe fn write_dense_d16(vals: *const u16, out: *mut u16, lo: usize, hi: usize) {
    unsafe {
        for i in lo..hi {
            *out.add(i) = *vals.add(i) & (MASK13 as u16);
        }
    }
}

// ── the shared per-byte primitives ──────────────────────────────────────────

#[inline(always)]
unsafe fn write_byte_plain(out: *mut u8, byte: usize, val: u8) {
    unsafe {
        *out.add(byte) |= val;
    }
}

#[inline(always)]
unsafe fn write_byte_atomic(out: *mut u8, byte: usize, val: u8) {
    unsafe {
        (*(out.add(byte) as *mut AtomicU8)).fetch_or(val, Ordering::Relaxed);
    }
}

/// Encode one field at logical index `i`. Every byte the field touches that
/// appears in `guards` goes through the atomic write; every other byte goes
/// through the plain one. `guards` holds at most two entries because a call's
/// own element can border at most two neighbours (the one before its range and
/// the one after), and a single-element slice is the one case where both can
/// apply to the same call.
///
/// # Safety
/// `out` holds the packed encoding of at least `i + 1` elements at this
/// width, and every non-guarded byte the field touches is not written by any
/// other thread for the duration of this call.
#[inline(always)]
unsafe fn encode_field(out: *mut u8, i: usize, v: u16, guards: [Option<usize>; 2]) {
    unsafe {
        let bit = i * LOGICAL_BITS;
        let mut byte = bit >> 3;
        let mut sh = (bit & 7) as u32;
        let mut left = LOGICAL_BITS;
        let mut field = (v as u64) & MASK13;
        while left > 0 {
            let room = 8 - sh;
            let take = if left < room as usize {
                left
            } else {
                room as usize
            };
            let chunk = (field & ((1u64 << take) - 1)) as u8;
            if guards[0] == Some(byte) || guards[1] == Some(byte) {
                write_byte_atomic(out, byte, chunk << sh);
            } else {
                write_byte_plain(out, byte, chunk << sh);
            }
            field >>= take;
            left -= take;
            sh = 0;
            byte += 1;
        }
    }
}

/// Adversarial element order: the boundary-adjacent elements first, then the
/// interior. A sequential pass would walk `lo..hi` in order; this ordering
/// exists so that whatever hazard a thread boundary carries is paid at the
/// very start of the pass, right after every thread is released together,
/// which is the moment two threads are most likely to be inside their own
/// boundary write at the same instant. It changes nothing about correctness;
/// it changes only how likely a real hazard is to be observed in one run.
#[inline(always)]
unsafe fn for_each_boundary_first(lo: usize, hi: usize, mut f: impl FnMut(usize)) {
    if hi <= lo {
        return;
    }
    f(hi - 1);
    if hi - 1 != lo {
        f(lo);
    }
    for i in (lo + 1)..(hi - 1) {
        f(i);
    }
}

// ── the two packed kernels ──────────────────────────────────────────────────

/// # Safety
/// `vals` holds at least `hi` elements. `out` holds the packed encoding of at
/// least `hi` elements plus headroom. Correctness additionally requires that
/// no byte this call writes is written by any other thread, which holds when
/// `lo` and `hi` are each a multiple of the packed period (8 at width 13) and
/// does not hold otherwise; the caller states which case it is in by the
/// `(n, threads)` pair it chooses, not by anything this function checks.
pub unsafe fn write_packed_plain(vals: *const u16, out: *mut u8, lo: usize, hi: usize) {
    unsafe {
        for_each_boundary_first(lo, hi, |i| {
            encode_field(out, i, *vals.add(i), [None, None]);
        });
    }
}

/// # Safety
/// `vals` holds at least `hi` elements. `out` holds the packed encoding of at
/// least `hi` elements plus headroom. `n` is the column's true length, which
/// is what tells this call whether `lo` and `hi` are internal boundaries (and
/// therefore need the atomic guard) or the column's own edges (which do not,
/// because there is no neighbour past either end).
pub unsafe fn write_packed_guarded(vals: *const u16, out: *mut u8, lo: usize, hi: usize, n: usize) {
    unsafe {
        let left_guard = split_is_guarded(lo, n).then(|| (lo * LOGICAL_BITS) / 8);
        let right_guard = split_is_guarded(hi, n).then(|| (hi * LOGICAL_BITS) / 8);
        for_each_boundary_first(lo, hi, |i| {
            let guards = if i == lo && i == hi - 1 {
                [left_guard, right_guard]
            } else if i == lo {
                [left_guard, None]
            } else if i == hi - 1 {
                [right_guard, None]
            } else {
                [None, None]
            };
            encode_field(out, i, *vals.add(i), guards);
        });
    }
}

// ── the windowed encoder: attacking the naive loop's cost ──────────────────
//
// `write_packed_plain` costs three read-modify-write byte operations for most
// elements (13 bits split across up to three bytes when the running bit
// offset lands late in a byte) and recomputes its shift and byte index from
// scratch every element. The read side's `sum_windowed` does the equivalent
// job with two 64-bit unaligned loads and eight shift-mask extracts per
// GROUP of eight elements, because the group's window layout is a fixed
// property of the width, known at monomorphisation, and does not depend on
// where the running index happens to be. The encode side has the identical
// structure available and nothing here used it.
//
// This kernel builds each group's window value (an OR of every lane's
// shifted field) in registers, then merges each window into `out` with one
// unaligned 64-bit load, OR, store. Windows can overlap in their byte range
// (two adjacent windows both touching the same middle bytes of a 13-byte
// group), which is exactly why the merge reads before it writes: a plain
// store would erase whatever the previous window in the same group already
// deposited into the overlap. Requires `lo` and `hi` to each be a multiple of
// the packed period, the same contract `sum_windowed` carries, so it is only
// used where a split is already period-aligned by construction; the
// thread-boundary hazard this crate exists to demonstrate does not have a
// windowed-and-guarded counterpart here, which is named as a real gap in the
// findings rather than built under time pressure to close it.

#[inline(always)]
unsafe fn ld_u64_at(buf: *const u8, at: usize) -> u64 {
    unsafe { u64::from_le(core::ptr::read_unaligned(buf.add(at) as *const u64)) }
}

#[inline(always)]
unsafe fn merge_u64_at(buf: *mut u8, at: usize, bits: u64) {
    unsafe {
        let cur = ld_u64_at(buf as *const u8, at);
        core::ptr::write_unaligned(buf.add(at) as *mut u64, (cur | bits).to_le());
    }
}

/// # Safety
/// `vals` holds at least `hi` elements. `out` holds the packed encoding of at
/// least `hi` elements plus 8 bytes of write headroom (a window merge may
/// touch up to 8 bytes past the group's own span). `lo` and `hi` are each a
/// multiple of `bench_bitpack_plan_shared::Plan13::P` (8), so every byte this
/// call touches belongs to this thread alone.
pub unsafe fn write_packed_windowed(vals: *const u16, out: *mut u8, lo: usize, hi: usize) {
    use bench_bitpack_plan_shared::Packing;
    type K = bench_bitpack_plan_shared::Pack<LOGICAL_BITS>;
    let () = K::WINDOW_FITS;
    debug_assert_eq!(lo % K::P, 0);
    debug_assert_eq!(hi % K::P, 0);
    let mut base = (lo * LOGICAL_BITS) / 8;
    let mut i = lo;
    unsafe {
        while i < hi {
            let mut win = [0u64; 8];
            let mut j = 0usize;
            while j < K::P {
                let (wi, sh) = K::LANES[j];
                let v = (*vals.add(i + j) as u64) & MASK13;
                win[wi as usize] |= v << sh;
                j += 1;
            }
            let mut w = 0usize;
            while w < K::NWIN {
                merge_u64_at(out, base + K::WINDOWS[w] as usize, win[w]);
                w += 1;
            }
            base += K::G;
            i += K::P;
        }
    }
}

/// # Safety
/// See [`write_packed_windowed`]. `n` is accepted and ignored, matching the
/// shape every arm in this pool shares.
pub unsafe fn kern_packed_windowed(
    vals: *const u16,
    out: *mut u8,
    lo: usize,
    hi: usize,
    _n: usize,
) {
    unsafe { write_packed_windowed(vals, out, lo, hi) }
}

// ── adapters to the pool's uniform kernel shape ─────────────────────────────
//
// The pool's job carries one function-pointer type so it does not need to know
// which arm it is running. The three kernels above differ in output pointer
// type (dense writes `u16`, packed writes `u8`) and in whether they need `n`
// (only the guarded kernel does, to tell an internal boundary from the
// column's own edge), so each gets a thin same-shape wrapper here.

/// # Safety
/// See [`write_dense_d16`]. `n` is accepted and ignored, matching the shape
/// every arm in this pool shares.
pub unsafe fn kern_dense_d16(vals: *const u16, out: *mut u8, lo: usize, hi: usize, _n: usize) {
    unsafe { write_dense_d16(vals, out as *mut u16, lo, hi) }
}

/// # Safety
/// See [`write_packed_plain`]. `n` is accepted and ignored: this kernel never
/// consults the column's true length, which is exactly why it cannot tell an
/// internal boundary from an edge and therefore cannot guard one.
pub unsafe fn kern_packed_plain(vals: *const u16, out: *mut u8, lo: usize, hi: usize, _n: usize) {
    unsafe { write_packed_plain(vals, out, lo, hi) }
}

/// # Safety
/// See [`write_packed_guarded`].
pub unsafe fn kern_packed_guarded(vals: *const u16, out: *mut u8, lo: usize, hi: usize, n: usize) {
    unsafe { write_packed_guarded(vals, out, lo, hi, n) }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::input::{build_bytes, slice_bounds, Layout, OFF_PACKED_OUT, OFF_VALS};
    use bench_bitpack_plan_shared::sum_naive;

    fn truth(vals: &[u16], n: usize) -> u64 {
        let mut s = 0u64;
        for &v in &vals[..n] {
            s = s.wrapping_add((v & (MASK13 as u16)) as u64);
        }
        s
    }

    fn decode_dense(out: &[u16], n: usize) -> u64 {
        let mut s = 0u64;
        for &v in &out[..n] {
            s = s.wrapping_add(v as u64);
        }
        s
    }

    fn run_single_threaded(
        n: usize,
        seed: u64,
        kernel: impl Fn(*const u16, *mut u8, usize, usize),
    ) -> (u64, u64) {
        let mut buf = build_bytes(n, seed);
        let vals_ptr = buf[OFF_VALS..].as_ptr() as *const u16;
        let want = {
            let vals = unsafe { std::slice::from_raw_parts(vals_ptr, n) };
            truth(vals, n)
        };
        let out_ptr = buf[OFF_PACKED_OUT..].as_mut_ptr();
        kernel(vals_ptr, out_ptr, 0, n);
        let got = sum_naive(&buf[OFF_PACKED_OUT..], n);
        (want, got)
    }

    /// The windowed encoder agrees with the ground truth at every size on the
    /// declared boundary, since its own safety contract requires `lo`/`hi` to
    /// be period multiples; run at 0, 1 and 2 periods and at a large size
    /// past L1, both single-shot and split across several period-aligned
    /// calls the way a real threaded pass would issue them.
    #[test]
    fn windowed_agrees_single_call() {
        for &n in &[0usize, 8, 16, 65536] {
            let (want, got) = run_single_threaded(n, 11, |v, o, lo, hi| unsafe {
                write_packed_windowed(v, o, lo, hi)
            });
            assert_eq!(got, want, "write_packed_windowed disagrees at n={n}");
        }
    }

    #[test]
    fn windowed_agrees_split_across_period_aligned_calls() {
        for &n in &[65536usize, 2_097_152] {
            for &t in &[1usize, 2, 4] {
                let (want, got) = run_sequential_split(n, t, 12, |v, o, lo, hi| unsafe {
                    write_packed_windowed(v, o, lo, hi)
                });
                assert_eq!(got, want, "windowed, split n={n} t={t}");
            }
        }
    }

    /// A single-threaded pass with no boundary at all agrees with the ground
    /// truth for every kernel, at sizes on both sides of a byte boundary.
    #[test]
    fn single_threaded_plain_agrees() {
        for &n in &[1usize, 2, 7, 8, 100, 65536] {
            let (want, got) = run_single_threaded(n, 5, |v, o, lo, hi| unsafe {
                write_packed_plain(v, o, lo, hi)
            });
            assert_eq!(got, want, "write_packed_plain disagrees at n={n}");
        }
    }

    #[test]
    fn single_threaded_guarded_agrees() {
        for &n in &[1usize, 2, 7, 8, 100, 65536] {
            let (want, got) = run_single_threaded(n, 6, |v, o, lo, hi| unsafe {
                write_packed_guarded(v, o, lo, hi, n)
            });
            assert_eq!(got, want, "write_packed_guarded disagrees at n={n}");
        }
    }

    /// A *sequential simulation* of a split pass: encode each thread's slice
    /// one after another, with no real concurrency, so this test isolates
    /// whether the per-slice encoding is correct from whether concurrent
    /// execution corrupts it. Both kernels must agree here regardless of
    /// alignment, because nothing races when the calls are sequential.
    fn run_sequential_split(
        n: usize,
        threads: usize,
        seed: u64,
        kernel: impl Fn(*const u16, *mut u8, usize, usize),
    ) -> (u64, u64) {
        let mut buf = build_bytes(n, seed);
        let vals_ptr = buf[OFF_VALS..].as_ptr() as *const u16;
        let want = {
            let vals = unsafe { std::slice::from_raw_parts(vals_ptr, n) };
            truth(vals, n)
        };
        let out_ptr = buf[OFF_PACKED_OUT..].as_mut_ptr();
        for t in 0..threads {
            let (lo, hi) = slice_bounds(t, n, threads);
            kernel(vals_ptr, out_ptr, lo, hi);
        }
        let got = sum_naive(&buf[OFF_PACKED_OUT..], n);
        (want, got)
    }

    #[test]
    fn sequential_split_plain_agrees_aligned() {
        for &n in &[65536usize, 2_097_152] {
            for &t in &[1usize, 2, 4] {
                let (want, got) = run_sequential_split(n, t, 7, |v, o, lo, hi| unsafe {
                    write_packed_plain(v, o, lo, hi)
                });
                assert_eq!(got, want, "aligned n={n} t={t}");
            }
        }
    }

    #[test]
    fn sequential_split_plain_agrees_even_misaligned() {
        // Sequential calls never race regardless of alignment, so the naive
        // kernel is still correct here; only concurrent execution exposes the
        // hazard, which is the point `pool.rs`'s stress test demonstrates.
        for &n in &[65534usize, 2_097_150] {
            for &t in &[1usize, 2, 4] {
                let (want, got) = run_sequential_split(n, t, 8, |v, o, lo, hi| unsafe {
                    write_packed_plain(v, o, lo, hi)
                });
                assert_eq!(got, want, "misaligned, sequential n={n} t={t}");
            }
        }
    }

    #[test]
    fn sequential_split_guarded_agrees_misaligned() {
        for &n in &[65534usize, 2_097_150] {
            for &t in &[1usize, 2, 4] {
                let (want, got) = run_sequential_split(n, t, 9, |v, o, lo, hi| unsafe {
                    write_packed_guarded(v, o, lo, hi, n)
                });
                assert_eq!(got, want, "misaligned, guarded, sequential n={n} t={t}");
            }
        }
    }

    /// The dense kernel decoded through the harness's own truth check, at
    /// sizes on both sides of the drain this crate does not have but the
    /// pattern is worth pinning anyway.
    #[test]
    fn dense_write_agrees() {
        for &n in &[1usize, 100, 65536] {
            let mut buf = build_bytes(n, 10);
            let vals_ptr = buf[OFF_VALS..].as_ptr() as *const u16;
            let want = {
                let vals = unsafe { std::slice::from_raw_parts(vals_ptr, n) };
                truth(vals, n)
            };
            let out_ptr = buf[crate::input::OFF_DENSE_OUT..].as_mut_ptr() as *mut u16;
            unsafe { write_dense_d16(vals_ptr, out_ptr, 0, n) };
            let out = unsafe { std::slice::from_raw_parts(out_ptr, n) };
            assert_eq!(decode_dense(out, n), want, "dense write disagrees at n={n}");
        }
    }

    /// `Layout`'s size matches the offsets it is built from.
    #[test]
    fn layout_size_is_consistent() {
        assert_eq!(core::mem::size_of::<Layout>(), crate::input::TOTAL_BYTES);
    }
}
