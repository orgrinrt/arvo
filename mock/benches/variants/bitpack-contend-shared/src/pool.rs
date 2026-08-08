//! The persistent worker pool and the timed column pass.
//!
//! See the crate root for why a pool exists at all rather than a spawn per
//! call, why the job is a function pointer rather than a closure, and why every
//! participating thread carries the same QoS the harness would have applied to
//! an unthreaded worker.

use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};
use std::sync::OnceLock;

use crate::input::{slice_bounds, Layout, MAX_THREADS};
use crate::SliceKernel;

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
