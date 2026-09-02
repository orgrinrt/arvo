//! The persistent worker pool and the timed write pass.
//!
//! Same shape as `bitpack-contend-shared`'s read pool (a persistent pool
//! rather than a spawn per call, because at these column sizes a spawn costs
//! more than the pass it would be attached to; every participating thread
//! carries the harness's own P-core QoS bias rather than the OS default,
//! since a `threaded = true` bench skips the harness's own pin). The
//! difference is the job: no partial to publish back, because a write kernel's
//! result lives in the output buffer rather than in a return value, and
//! correctness is checked by decoding that buffer after the pass rather than by
//! summing partials during it.

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Mutex, OnceLock};

use crate::input::{slice_bounds, MAX_THREADS};
use crate::WriteKernel;

struct Pool {
    generation: AtomicUsize,
    done: AtomicUsize,
    kernel: AtomicUsize,
    vals: AtomicUsize,
    out: AtomicUsize,
    n: AtomicUsize,
    threads: AtomicUsize,
}

struct PoolHandle {
    pool: &'static Pool,
    threads: usize,
}

static POOL: OnceLock<PoolHandle> = OnceLock::new();

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
            vals: AtomicUsize::new(0),
            out: AtomicUsize::new(0),
            n: AtomicUsize::new(0),
            threads: AtomicUsize::new(threads),
        }));

        for i in 1..threads {
            std::thread::Builder::new()
                .name(format!("write-contend-{i}"))
                .spawn(move || worker(pool, i))
                .expect("spawning a pool worker");
        }
        PoolHandle { pool, threads }
    });
    assert_eq!(
        handle.threads, threads,
        "the pool is sized on first use, and the harness runs one thread count \
         per worker process; a second count in the same process means that \
         contract changed."
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
        // SAFETY: the coordinator stores a live `WriteKernel` before the
        // release bump of `generation`, and this load is acquire-ordered
        // against it.
        let kernel: WriteKernel = unsafe {
            std::mem::transmute::<usize, WriteKernel>(pool.kernel.load(Ordering::Relaxed))
        };
        let vals = pool.vals.load(Ordering::Relaxed) as *const u16;
        let out = pool.out.load(Ordering::Relaxed) as *mut u8;
        let n = pool.n.load(Ordering::Relaxed);
        let threads = pool.threads.load(Ordering::Relaxed);
        let (lo, hi) = slice_bounds(index, n, threads);
        // SAFETY: the coordinator holds vals and out alive across the whole
        // pass and does not return until `done` reaches `threads - 1`. Whether
        // this call and its neighbours' calls race on a shared byte is exactly
        // the property under measurement; the pool guarantees liveness, not
        // freedom from that race.
        unsafe { kernel(vals, out, lo, hi, n) };
        pool.done.fetch_add(1, Ordering::Release);
    }
}

/// Held for the duration of a pass, because the pool is one per process and a
/// pass is what owns it.
///
/// The pool publishes its arguments through the fields above and reads back one
/// `done` counter, so **two passes in flight at once do not race for the
/// measurement, they destroy each other**: the second store of `vals` and `out`
/// redirects workers that are already running on the first pass's buffers, the
/// second `done.store(0)` erases completions the first is still waiting on, and
/// both coordinators then spin on a counter that will not reach `threads - 1`
/// again. The visible symptom is a hang; the invisible one is a pass that
/// finished having written through another pass's pointers.
///
/// The `# Safety` clause below always demanded this and nothing enforced it,
/// which held for as long as the only caller was the harness, running one
/// process per bench and per size. `cargo test` is the caller that is not that:
/// it runs the stress tests in `stress.rs` on their own threads, and three of
/// them enter here.
///
/// The cost on the timed path is one uncontended acquire per pass, against a
/// pass that spawns nothing, walks the whole column and ends on a spin barrier.
/// The alternative measured on this machine is four processes wedged between
/// two and four and a half hours.
static ONE_PASS_AT_A_TIME: Mutex<()> = Mutex::new(());

/// Run one write pass over `[0, n)` with `threads` threads.
///
/// # Safety
/// `vals` holds at least `n` elements and outlives the call. `out` holds at
/// least the encoded form of `n` elements plus headroom, outlives the call,
/// and is not read or written by anything else while this call is in flight.
#[inline(never)]
pub unsafe fn write_pass(
    threads: usize,
    n: usize,
    vals: *const u16,
    out: *mut u8,
    kernel: WriteKernel,
) {
    // Poisoning carries no information here: the guarded state is the pool's
    // own fields, every pass overwrites all of them before publishing, and a
    // panicking pass leaves nothing a later one reads.
    let _pass = ONE_PASS_AT_A_TIME
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());

    if threads == 1 {
        let _ = pool(1);
        unsafe { kernel(vals, out, 0, n, n) };
        return;
    }

    let handle = pool(threads);
    let p = handle.pool;
    p.kernel.store(kernel as usize, Ordering::Relaxed);
    p.vals.store(vals as usize, Ordering::Relaxed);
    p.out.store(out as usize, Ordering::Relaxed);
    p.n.store(n, Ordering::Relaxed);
    p.done.store(0, Ordering::Relaxed);
    p.generation.fetch_add(1, Ordering::Release);

    let (lo, hi) = slice_bounds(0, n, threads);
    // SAFETY: the caller's contract, forwarded.
    unsafe { kernel(vals, out, lo, hi, n) };

    while p.done.load(Ordering::Acquire) != threads - 1 {
        std::hint::spin_loop();
    }
}
