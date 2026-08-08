//! Call the shipped bench cdylibs the way the harness does, and time them
//! from outside with a clock the variant cannot influence.
//!
//! The committed run `precise-container-width-l1_n130103` reports the
//! `warm-container-kernel` arm at 4 ns per call for 8192 saturating
//! elements, and the harness's derived `e2e` agrees at 65 ns. Both are
//! physically impossible on this host. This asks the dylib directly:
//! what does it return in `run_ticks`, what does it write to the output
//! buffer, and how long does the call actually take.
//!
//! Nothing here is a benchmark. It is an ad-hoc quick spike whose only
//! job is to decide whether a committed number is measuring the work.

use bench_warm_container_shared::{key_d, key_n, key_op, key_w, Case, ALL_KEYS};
use mockspace_bench_core::{FfiBenchCall, Routine};
use std::time::Instant;

type Entry = unsafe extern "C" fn(*const u8, *mut u8, usize) -> FfiBenchCall;

unsafe fn load(path: &str) -> Entry {
    let c = std::ffi::CString::new(path).unwrap();
    let h = unsafe { libc::dlopen(c.as_ptr(), libc::RTLD_NOW) };
    assert!(!h.is_null(), "dlopen failed for {path}: {:?}", unsafe {
        std::ffi::CStr::from_ptr(libc::dlerror())
    });
    let s = std::ffi::CString::new("bench_entry").unwrap();
    let p = unsafe { libc::dlsym(h, s.as_ptr()) };
    assert!(!p.is_null(), "no bench_entry in {path}");
    unsafe { std::mem::transmute::<*mut libc::c_void, Entry>(p) }
}

macro_rules! probe_key {
    ($key:literal, $arms:expr) => {{
        const KEY: usize = $key;
        let input = <Case<KEY> as Routine>::build_input_bytes(0x1234_5678_9ABC_DEF0);
        let osz = core::mem::size_of::<<Case<KEY> as Routine>::Output>();
        println!(
            "\nKEY {} : W={} n={} op={} D={}   (op 0 wrapping, 1 saturating)",
            KEY,
            key_w(KEY),
            key_n(KEY),
            key_op(KEY),
            key_d(KEY)
        );
        println!(
            "{:<32}{:>14}{:>16}{:>22}",
            "arm", "run_ticks", "outer ns/call", "output value"
        );
        for (name, path) in $arms {
            let entry = unsafe { load(path) };
            let mut out = vec![0u8; osz];
            // warm
            for _ in 0..64 {
                unsafe { entry(input.as_ptr(), out.as_mut_ptr(), KEY) };
            }
            let reps = 400usize;
            let mut ticks = 0u64;
            let t = Instant::now();
            for _ in 0..reps {
                let r = unsafe { entry(input.as_ptr(), out.as_mut_ptr(), KEY) };
                ticks += r.run_ticks;
            }
            let outer = t.elapsed().as_nanos() as f64 / reps as f64;
            let val = u64::from_ne_bytes(out[..8].try_into().unwrap());
            println!(
                "{:<32}{:>14.2}{:>16.0}{:>22}",
                name,
                ticks as f64 / reps as f64,
                outer,
                val
            );
        }
    }};
}

fn main() {
    let base = "/Users/orgrinrt/Dev/clause-dev/arvo/mock/target/release/";
    let arms: Vec<(&str, String)> = [
        "headroom",
        "minimum",
        "native",
        "kernel",
        "lanes_deferred",
        "plusone",
    ]
    .iter()
    .map(|a| (*a, format!("{base}libbench_warm_container_{a}.dylib")))
    .collect();
    let arms: Vec<(&str, &str)> = arms.iter().map(|(a, p)| (*a, p.as_str())).collect();

    assert!(ALL_KEYS.contains(&130103) && ALL_KEYS.contains(&130003));
    probe_key!(130103, &arms); // Precise saturating, W=13: the 4 ns cell
    probe_key!(640103, &arms); // Precise saturating, W=64
    probe_key!(130003, &arms); // Warm wrapping, W=13: sane numbers, control
    probe_key!(640003, &arms); // Warm wrapping, W=64: the headroom headline
}
