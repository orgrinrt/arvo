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

use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};
use std::sync::OnceLock;

use bench_bitpack_carrier_shared::{
    sum_d16, sum_d32, sum_d64, CarrierColumn, Plan13, Sum, LOGICAL_BITS, MASK13, OFF_D16, OFF_D32,
    OFF_D64, OFF_PACKED, TOTAL_INPUT_BYTES,
};
use bench_bitpack_plan_shared::{pack, sum_naive};
use mockspace_bench_core::Routine;

/// The four-carrier layout, named without a const parameter.
///
/// Every field of `CarrierColumn<N>` is sized from `MAX_N` rather than from `N`,
/// so the layout is identical at every instantiation and `CarrierColumn<0>` is
/// the same 125 MiB struct as `CarrierColumn<MAX_N>`. Asserted in the tests
/// below rather than trusted, because the harness casts the raw input buffer to
/// this type and a size disagreement would read the wrong offsets while every
/// timed number still looked ordinary.
pub type Layout = CarrierColumn<0>;

/// The largest thread count the pool supports.
pub const MAX_THREADS: usize = 8;

/// A slice kernel: read `[lo, hi)` of one carrier region and return the wrapping
/// sum of the decoded values.
///
/// A raw function pointer rather than a closure, because the pool is persistent
/// and shared across calls: a closure would have to be boxed on every timed
/// call, putting an allocation inside the measured region. The indirect call is
/// paid once per slice per pass, amortised over at least `N / T` elements, and
/// paid identically by every arm.
///
/// # Safety
/// `base` must point at a live `Layout` that outlives the call, and
/// `lo <= hi <= N` for the `N` the input was built at.
pub type SliceKernel = unsafe fn(base: *const Layout, lo: usize, hi: usize) -> u64;

/// The half-open element range thread `index` walks.
///
/// The last thread takes the remainder, which is zero for every key this bench
/// declares (`KEY_SPLITS` refuses any key where it would not be) but is written
/// out anyway so the function is correct for a caller that has not read the
/// refusal.
#[inline]
pub fn slice_bounds(index: usize, n: usize, threads: usize) -> (usize, usize) {
    let span = n / threads;
    let lo = index * span;
    let hi = if index + 1 == threads { n } else { lo + span };
    (lo, hi)
}

/// splitmix64, matching the carrier crate's own private copy. The duplication is
/// what `build_bytes_equals_the_carrier_crates_builder` pins.
struct SplitMix64(u64);
impl SplitMix64 {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }
}

/// Build the four-carrier input for a runtime element count.
///
/// The carrier crate's `build_input_bytes` takes its count as a const parameter,
/// which this bench cannot use because the count is decoded from `KEY` at
/// runtime. Reproduced here and pinned by a byte-equality test rather than by
/// inspection.
pub fn build_bytes(n: usize, seed: u64) -> Vec<u8> {
    let mut rng = SplitMix64(seed ^ 0xB179_ACC0_0001_5EED);
    let vals: Vec<u16> = (0..n).map(|_| (rng.next() & MASK13) as u16).collect();

    let mut buf = vec![0u8; TOTAL_INPUT_BYTES];
    for (i, &v) in vals.iter().enumerate() {
        let w = v as u64;
        buf[OFF_D64 + i * 8..OFF_D64 + i * 8 + 8].copy_from_slice(&w.to_le_bytes());
        buf[OFF_D32 + i * 4..OFF_D32 + i * 4 + 4].copy_from_slice(&(w as u32).to_le_bytes());
        buf[OFF_D16 + i * 2..OFF_D16 + i * 2 + 2].copy_from_slice(&v.to_le_bytes());
    }
    let packed_bytes = (n * LOGICAL_BITS) / 8 + 16;
    pack(&vals, &mut buf[OFF_PACKED..OFF_PACKED + packed_bytes]);
    buf
}

// ── the pool ───────────────────────────────────────────────────────────────

/// One worker's partial, on its own 128-byte line.
///
/// Apple's L2 line is 128 bytes, so two partials inside one 64-byte line would
/// still share an L2 line and every store would bounce it between cores. That is
/// a false-sharing artefact that would appear as a scaling failure and be read
/// as contention, which is exactly the confusion this bench exists to avoid.
#[repr(align(128))]
struct Padded(AtomicU64);

struct Pool {
    /// Bumped by the coordinator to publish a new job.
    generation: AtomicUsize,
    /// Incremented by each worker as it finishes its slice.
    done: AtomicUsize,
    /// Job fields, published before the generation bump and read after it.
    kernel: AtomicUsize,
    base: AtomicUsize,
    n: AtomicUsize,
    threads: AtomicUsize,
    partials: [Padded; MAX_THREADS],
}

struct PoolHandle {
    pool: &'static Pool,
    threads: usize,
}

static POOL: OnceLock<PoolHandle> = OnceLock::new();

/// QoS `USER_INTERACTIVE`, the same call `bench-core`'s `pin_to_perf_cores`
/// makes, applied to the calling thread.
fn bias_to_perf_cores() {
    #[cfg(target_os = "macos")]
    unsafe {
        unsafe extern "C" {
            fn pthread_set_qos_class_self_np(
                qos_class: u32,
                relative_priority: std::os::raw::c_int,
            ) -> std::os::raw::c_int;
        }
        let _ = pthread_set_qos_class_self_np(0x21, 0);
    }
}

/// Spin iterations before falling back to yielding.
///
/// Long enough that a worker never sleeps inside a timed pass, short enough that
/// it does not burn a core through the harness's cooldowns and process setup
/// between passes.
const SPIN_BUDGET: u32 = 200_000;

fn pool(threads: usize) -> &'static PoolHandle {
    let handle = POOL.get_or_init(|| {
        assert!(
            (1..=MAX_THREADS).contains(&threads),
            "thread count {threads} outside 1..={MAX_THREADS}"
        );
        bias_to_perf_cores();

        let pool: &'static Pool = Box::leak(Box::new(Pool {
            generation: AtomicUsize::new(0),
            done: AtomicUsize::new(0),
            kernel: AtomicUsize::new(0),
            base: AtomicUsize::new(0),
            n: AtomicUsize::new(0),
            threads: AtomicUsize::new(threads),
            partials: std::array::from_fn(|_| Padded(AtomicU64::new(0))),
        }));

        // Worker `i` handles slice `i`; slice 0 is the coordinator's, so the
        // pool holds `threads - 1` of them and no thread is idle during a pass.
        for i in 1..threads {
            std::thread::Builder::new()
                .name(format!("contend-{i}"))
                .spawn(move || worker(pool, i))
                .expect("spawning a pool worker");
        }
        PoolHandle { pool, threads }
    });
    assert_eq!(
        handle.threads, threads,
        "the pool is sized on first use, and the harness runs one size, hence \
         one thread count, per worker process. A second count in the same \
         process means that contract changed and every number after the change \
         would be a different measurement wearing the same arm's name."
    );
    handle
}

fn worker(pool: &'static Pool, index: usize) {
    bias_to_perf_cores();
    let mut seen = 0usize;
    loop {
        let mut spins = 0u32;
        loop {
            let g = pool.generation.load(Ordering::Acquire);
            if g != seen {
                seen = g;
                break;
            }
            spins = spins.saturating_add(1);
            if spins < SPIN_BUDGET {
                std::hint::spin_loop();
            } else {
                std::thread::yield_now();
            }
        }
        // SAFETY: the coordinator stores a live `SliceKernel` before the release
        // bump of `generation`, and this load is acquire-ordered against it.
        let kernel: SliceKernel = unsafe {
            std::mem::transmute::<usize, SliceKernel>(pool.kernel.load(Ordering::Relaxed))
        };
        let base = pool.base.load(Ordering::Relaxed) as *const Layout;
        let n = pool.n.load(Ordering::Relaxed);
        let threads = pool.threads.load(Ordering::Relaxed);
        let (lo, hi) = slice_bounds(index, n, threads);
        // SAFETY: the coordinator holds the input alive across the whole pass
        // and does not return until `done` reaches `threads - 1`.
        let partial = unsafe { kernel(base, lo, hi) };
        pool.partials[index].0.store(partial, Ordering::Relaxed);
        pool.done.fetch_add(1, Ordering::Release);
    }
}

/// Run one column pass over `[0, n)` with `threads` threads and return the
/// wrapping sum of the partials.
///
/// The coordinator takes slice 0 itself rather than waiting on `threads`
/// workers, so the pass uses exactly `threads` cores and none is idle inside the
/// measured region.
///
/// # Safety
/// `base` must point at a live `Layout` built for at least `n` elements, and `n`
/// must be a multiple of `threads` times the packed period.
#[inline(never)]
pub unsafe fn column_pass(
    threads: usize,
    n: usize,
    base: *const Layout,
    kernel: SliceKernel,
) -> u64 {
    if threads == 1 {
        // Through the pool's init so the coordinator is QoS-biased the same way,
        // but with no barrier: a barrier nobody waits on is not the same code
        // path and would price the wrong thing.
        let _ = pool(1);
        return unsafe { kernel(base, 0, n) };
    }

    let handle = pool(threads);
    let p = handle.pool;
    p.kernel.store(kernel as usize, Ordering::Relaxed);
    p.base.store(base as usize, Ordering::Relaxed);
    p.n.store(n, Ordering::Relaxed);
    p.done.store(0, Ordering::Relaxed);
    p.generation.fetch_add(1, Ordering::Release);

    let (lo, hi) = slice_bounds(0, n, threads);
    // SAFETY: the caller's contract, forwarded.
    let mut total = unsafe { kernel(base, lo, hi) };

    while p.done.load(Ordering::Acquire) != threads - 1 {
        std::hint::spin_loop();
    }
    for i in 1..threads {
        total = total.wrapping_add(p.partials[i].0.load(Ordering::Relaxed));
    }
    total
}

// ── the kernels ────────────────────────────────────────────────────────────
//
// One per arm, each a thin wrapper over the carrier crate's own inlined sum, so
// the inner loop is exactly the code the single-core sweep measured. They live
// here rather than in each variant crate so the pool's function-pointer type is
// satisfied by identically-shaped functions and the tests can drive all five
// through one table.

/// # Safety
/// See [`SliceKernel`].
pub unsafe fn kern_d16(base: *const Layout, lo: usize, hi: usize) -> u64 {
    let col = unsafe { &*base };
    sum_d16(&col.d16[lo..hi], hi - lo)
}

/// # Safety
/// See [`SliceKernel`].
pub unsafe fn kern_d32(base: *const Layout, lo: usize, hi: usize) -> u64 {
    let col = unsafe { &*base };
    sum_d32(&col.d32[lo..hi], hi - lo)
}

/// # Safety
/// See [`SliceKernel`].
pub unsafe fn kern_d64(base: *const Layout, lo: usize, hi: usize) -> u64 {
    let col = unsafe { &*base };
    sum_d64(&col.d64[lo..hi], hi - lo)
}

/// # Safety
/// See [`SliceKernel`]. `lo` must be a multiple of the packed period so the
/// slice starts on a byte boundary.
pub unsafe fn kern_packed(base: *const Layout, lo: usize, hi: usize) -> u64 {
    let col = unsafe { &*base };
    let byte_lo = (lo * LOGICAL_BITS) / 8;
    unsafe { bench_bitpack_plan_shared::sum_windowed::<Plan13>(&col.packed[byte_lo..], hi - lo) }
}

/// # Safety
/// See [`SliceKernel`]. `lo` must be a multiple of the packed period so the
/// slice starts on a byte boundary.
#[cfg(target_arch = "aarch64")]
pub unsafe fn kern_packed_simd(base: *const Layout, lo: usize, hi: usize) -> u64 {
    let col = unsafe { &*base };
    let byte_lo = (lo * LOGICAL_BITS) / 8;
    unsafe {
        bench_bitpack_carrier_shared::sum_simd_padal::<Plan13>(&col.packed[byte_lo..], hi - lo)
    }
}

// ── the routine ────────────────────────────────────────────────────────────

/// One row of the sweep: `KEY = N * 10 + T`.
///
/// The two parameters travel together because the bench macro dispatches on
/// exactly one const parameter and the harness keys a row by its `n`. Packing
/// them into one integer is the idiom every sibling bench in this directory
/// already uses (`warm-clamp-arity` keys are `W * 1000 + ...`). Decoding is
/// arithmetic on associated consts, which is ordinary const evaluation and needs
/// nothing in type position, so `generic_const_exprs` stays out of it.
pub struct Contend<const KEY: usize>;

impl<const KEY: usize> Contend<KEY> {
    /// Elements in the column.
    pub const N: usize = KEY / 10;
    /// Threads walking it.
    pub const T: usize = KEY % 10;
    /// Refuses at monomorphisation when a key cannot be split cleanly.
    ///
    /// Two conditions, both of which would otherwise fail silently at a read
    /// rather than loudly at compile time: the thread count must be in range,
    /// and each slice must start on a packed-period boundary so the packed arm's
    /// byte offset is a whole number of bytes.
    pub const KEY_SPLITS: () = {
        assert!(Self::T >= 1 && Self::T <= MAX_THREADS);
        assert!(Self::N % (Self::T * 8) == 0);
    };
}

impl<const KEY: usize> Routine for Contend<KEY> {
    type Input = Layout;
    type Output = Sum;

    fn build_input(_seed: u64) -> Self::Input {
        unreachable!(
            "Contend::build_input is never called by the real bench path \
             (routine_bridge! only takes build_input_bytes as a function pointer) \
             and is not safe to call at any KEY: Self::Input is MAX_N-sized for \
             every monomorphisation. Use build_input_bytes."
        )
    }

    fn build_input_bytes(seed: u64) -> Vec<u8> {
        let () = Self::KEY_SPLITS;
        build_bytes(Self::N, seed)
    }

    /// The carrier crate's four independent checks, at this row's `N`.
    ///
    /// Ground truth from the `u16` region, then the `u32` and `u64` regions
    /// against it, then the packed region through `sum_naive`, an index-driven
    /// decoder no timed arm here runs. A defect shared between `pack` and
    /// `sum_windowed`, which touch the same period arithmetic, is therefore not
    /// invisible.
    fn validate_output(input: &Self::Input, output: &Self::Output) -> Result<(), &'static str> {
        let n = Self::N;
        let mut expect: u64 = 0;
        for &v in input.d16[..n].iter() {
            expect = expect.wrapping_add(v as u64);
        }
        if output.value != expect {
            return Err("column sum mismatch: the timed arm produced a different \
                 value stream than the u16 ground truth");
        }
        if sum_d32(&input.d32[..n], n) != expect {
            return Err("u32 carrier region disagrees with the u16 ground truth");
        }
        if sum_d64(&input.d64[..n], n) != expect {
            return Err("u64 carrier region disagrees with the u16 ground truth");
        }
        let packed_bytes = (n * LOGICAL_BITS) / 8 + 16;
        if sum_naive(&input.packed[..packed_bytes], n) != expect {
            return Err("packed region mismatch: sum_naive's independent decode \
                 disagrees with the u16 ground truth");
        }
        Ok(())
    }

    /// Elements in the pass, not elements per thread, so per-element cost is
    /// wall time over `N` and composes directly with the single-core sweep.
    fn ops_per_call(_input: &Self::Input) -> u64 {
        Self::N as u64
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bench_bitpack_carrier_shared::MAX_N;

    /// Every `(n, t)` the bench declares. Named once so no test can sample a
    /// subset of the matrix without that being visible here.
    const NS: [usize; 4] = [16384, 1048576, 4194304, 8388608];
    const TS: [usize; 4] = [1, 2, 4, 8];

    /// The layout is independent of the const parameter, which is what makes
    /// `CarrierColumn<0>` a legitimate name for it and what the harness's cast
    /// of the raw input buffer relies on.
    #[test]
    fn layout_is_independent_of_the_const_parameter() {
        assert_eq!(core::mem::size_of::<Layout>(), TOTAL_INPUT_BYTES);
        assert_eq!(
            core::mem::size_of::<Layout>(),
            core::mem::size_of::<CarrierColumn<{ MAX_N }>>()
        );
        assert_eq!(
            core::mem::size_of::<Layout>(),
            core::mem::size_of::<CarrierColumn<16384>>()
        );
    }

    /// The runtime-`n` builder agrees byte for byte with the carrier crate's
    /// const-`N` one. Without this the two benches could drift into measuring
    /// different value streams while both looked fine, and every claim that this
    /// file's numbers compose with the carrier sweep's would be void.
    #[test]
    fn build_bytes_equals_the_carrier_crates_builder() {
        for seed in 0u64..4 {
            let mine = build_bytes(16384, seed);
            let theirs = <CarrierColumn<16384> as Routine>::build_input_bytes(seed);
            assert_eq!(mine.len(), theirs.len(), "length differs at seed {seed}");
            assert!(mine == theirs, "byte streams differ at seed {seed}");
        }
        let mine = build_bytes(131072, 9);
        let theirs = <CarrierColumn<131072> as Routine>::build_input_bytes(9);
        assert!(mine == theirs, "byte streams differ at n = 131072");
    }

    /// Key decoding over every key the bench declares, not a sample of them.
    #[test]
    fn every_declared_key_decodes_and_splits() {
        for n in NS {
            for t in TS {
                let key = n * 10 + t;
                assert_eq!(key / 10, n, "key {key} decodes to the wrong n");
                assert_eq!(key % 10, t, "key {key} decodes to the wrong t");
                assert_eq!(
                    n % (t * 8),
                    0,
                    "n = {n} does not split {t} ways on a packed-period boundary"
                );
            }
        }
    }

    /// The slice bounds tile the column exactly, at every declared thread count.
    /// A gap or an overlap would change the answer, and a gap in the middle of a
    /// large column is precisely the defect a sum check at one thread count
    /// would not see.
    #[test]
    fn slices_tile_the_column_at_every_thread_count() {
        for n in NS {
            for t in TS {
                let mut expect_lo = 0usize;
                for i in 0..t {
                    let (lo, hi) = slice_bounds(i, n, t);
                    assert_eq!(lo, expect_lo, "gap or overlap at n={n} t={t} slice {i}");
                    assert!(hi > lo, "empty slice at n={n} t={t} slice {i}");
                    assert_eq!(lo % 8, 0, "slice {i} at n={n} t={t} is off the period");
                    expect_lo = hi;
                }
                assert_eq!(
                    expect_lo, n,
                    "slices do not cover the column at n={n} t={t}"
                );
            }
        }
    }

    /// A split pass equals the whole pass, for every kernel and every declared
    /// thread count. Driven through `slice_bounds` rather than through
    /// `column_pass`, because the pool is sized once per process by design and a
    /// test cannot exercise four thread counts through it. This is the half that
    /// could be wrong: the slicing arithmetic and the packed byte offset. The
    /// pool itself is covered by the test below.
    #[test]
    fn a_split_pass_equals_the_whole_pass_for_every_kernel() {
        const N: usize = 16384;
        let buf = build_bytes(N, 5);
        let col: &Layout = unsafe { &*(buf.as_ptr() as *const Layout) };
        let base = col as *const Layout;

        let mut truth = 0u64;
        for &v in col.d16[..N].iter() {
            truth = truth.wrapping_add(v as u64);
        }

        let mut kernels: Vec<(&str, SliceKernel)> = vec![
            ("d16", kern_d16 as SliceKernel),
            ("d32", kern_d32 as SliceKernel),
            ("d64", kern_d64 as SliceKernel),
            ("packed", kern_packed as SliceKernel),
        ];
        #[cfg(target_arch = "aarch64")]
        kernels.push(("packed-simd", kern_packed_simd as SliceKernel));

        for (name, k) in kernels {
            for t in TS {
                let mut got = 0u64;
                for i in 0..t {
                    let (lo, hi) = slice_bounds(i, N, t);
                    got = got.wrapping_add(unsafe { k(base, lo, hi) });
                }
                assert_eq!(got, truth, "kernel {name} split {t} ways disagrees");
            }
        }
    }

    /// The pool itself: workers pick up the job, compute their slice, and the
    /// coordinator's total matches the ground truth. One thread count only,
    /// because the pool is deliberately sized once per process and a test that
    /// resized it would be testing a contract the bench does not have.
    #[test]
    fn the_pool_computes_the_same_total_as_a_serial_pass() {
        const N: usize = 16384;
        let buf = build_bytes(N, 13);
        let col: &Layout = unsafe { &*(buf.as_ptr() as *const Layout) };
        let base = col as *const Layout;

        let mut truth = 0u64;
        for &v in col.d16[..N].iter() {
            truth = truth.wrapping_add(v as u64);
        }
        let got = unsafe { column_pass(4, N, base, kern_d16 as SliceKernel) };
        assert_eq!(got, truth, "the four-thread pool disagrees with the column");

        // and a second pass over the same pool, because a generation counter
        // that only works once is a defect a single call cannot see
        let got2 = unsafe { column_pass(4, N, base, kern_packed as SliceKernel) };
        assert_eq!(got2, truth, "the second pass through the pool disagrees");
    }

    /// `validate_output` refuses a wrong answer. A validation pass that cannot
    /// fail is not a validation pass, and this bench's fidelity argument rests
    /// on the harness calling it.
    #[test]
    fn validate_output_rejects_a_wrong_sum() {
        const KEY: usize = 163841; // n = 16384, t = 1
        let buf = build_bytes(16384, 3);
        let col: &Layout = unsafe { &*(buf.as_ptr() as *const Layout) };
        let good = Sum {
            value: sum_d16(&col.d16[..16384], 16384),
        };
        assert!(<Contend<KEY> as Routine>::validate_output(col, &good).is_ok());
        let bad = Sum {
            value: good.value.wrapping_add(1),
        };
        assert!(
            <Contend<KEY> as Routine>::validate_output(col, &bad).is_err(),
            "validate_output accepted a sum off by one, so it would accept a broken arm"
        );
    }

    /// The refusal's arithmetic. A key whose slices do not land on a
    /// packed-period boundary would mis-address the packed arm, and
    /// `KEY_SPLITS` is what stops it at monomorphisation; the condition it
    /// tests is asserted here because a compile-fail test needs a trybuild
    /// harness this directory does not have.
    #[test]
    fn a_key_that_does_not_split_would_be_refused() {
        // 8200 elements do not divide four ways onto a period-8 boundary
        assert_ne!(8200 % (4 * 8), 0);
        // every key the bench declares does
        for n in NS {
            for t in TS {
                assert_eq!(n % (t * 8), 0);
            }
        }
    }
}
