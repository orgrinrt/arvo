//! Does `options(nomem)` on the aarch64 counter read let LLVM move the
//! measured work out of the timing bracket?
//!
//! `mockspace/bench-core/src/counter.rs:40` reads CNTVCT_EL0 with
//! `options(nostack, nomem)`. `nomem` promises LLVM the asm neither reads
//! nor writes memory, so loads and stores may be reordered across it. The
//! x86_64 arm of the same function has no `nomem` and carries an `lfence`.
//!
//! Two counter readers, identical except for that one option. The same work
//! between them, in the same shape the bench variants use: read a slice,
//! store the answer through a `&mut`. If `nomem` is the defect, the pair
//! diverges and only the barrier form reports the real cost.
//!
//! Build: rustc +nightly-2026-05-28 -O p1_counter_nomem_is_not_a_barrier.rs

use std::hint::black_box;

#[inline(always)]
fn read_nomem() -> u64 {
    let v: u64;
    unsafe {
        core::arch::asm!("mrs {}, CNTVCT_EL0", out(reg) v, options(nostack, nomem));
    }
    v
}

/// Same instruction, without the promise that it touches no memory.
#[inline(always)]
fn read_barrier() -> u64 {
    let v: u64;
    unsafe {
        core::arch::asm!("mrs {}, CNTVCT_EL0", out(reg) v, options(nostack));
    }
    v
}

/// The lane-parallel saturating fold, the shape `run_sat_lanes` has.
#[inline(always)]
fn sat_lanes(data: &[u64], k: u64, lim: u64) -> u64 {
    const L: usize = 8;
    let mut lanes = [0u64; L];
    for ch in data.chunks_exact(L) {
        for l in 0..L {
            let mut v = ch[l];
            for j in 0..3 {
                v = if j % 2 == 0 {
                    v.saturating_add(k).min(lim)
                } else {
                    v.saturating_sub(k)
                };
            }
            lanes[l] = lanes[l].saturating_add(v).min(lim);
        }
    }
    let mut acc = 0u64;
    for l in 0..L {
        acc = acc.saturating_add(lanes[l]).min(lim);
    }
    acc
}

/// The serial saturating fold, the shape `run_sat` has.
#[inline(always)]
fn sat_serial(data: &[u64], k: u64, lim: u64) -> u64 {
    let mut acc = 0u64;
    for &x in data {
        let mut v = x;
        for j in 0..3 {
            v = if j % 2 == 0 {
                v.saturating_add(k).min(lim)
            } else {
                v.saturating_sub(k)
            };
        }
        acc = acc.saturating_add(v).min(lim);
    }
    acc
}

struct Out {
    value: u64,
}

macro_rules! bracket {
    ($rd:ident, $work:expr, $out:expr) => {{
        let s = $rd();
        $out.value = $work;
        let e = $rd();
        e - s
    }};
}

fn main() {
    const N: usize = 8192;
    let data: Vec<u64> = (0..N as u64).map(|i| i.wrapping_mul(2654435761)).collect();
    let k: u64 = u64::MAX / 3 | 1;
    let lim = u64::MAX;
    let mut out = Out { value: 0 };

    // Warm.
    for _ in 0..200 {
        out.value = sat_lanes(&data, k, lim);
    }
    black_box(out.value);

    let reps = 200usize;
    let (mut t_lanes_nomem, mut t_lanes_bar, mut t_ser_nomem, mut t_ser_bar) =
        (0u64, 0u64, 0u64, 0u64);
    for _ in 0..reps {
        t_lanes_nomem += bracket!(read_nomem, sat_lanes(&data, k, lim), out);
        t_lanes_bar += bracket!(read_barrier, sat_lanes(&data, k, lim), out);
        t_ser_nomem += bracket!(read_nomem, sat_serial(&data, k, lim), out);
        t_ser_bar += bracket!(read_barrier, sat_serial(&data, k, lim), out);
    }
    black_box(out.value);

    // CNTVCT_EL0 is 24 MHz on this host: 41.6667 ns per tick.
    let ns = |t: u64| (t as f64) * (1e9 / 24e6) / reps as f64;
    println!("N = {N} elements, {reps} reps, 3 ops/element, saturating\n");
    println!("{:<34}{:>12}{:>12}", "arm", "ticks/rep", "ns/rep");
    println!(
        "{:<34}{:>12.2}{:>12.0}",
        "lanes, counter with nomem",
        t_lanes_nomem as f64 / reps as f64,
        ns(t_lanes_nomem)
    );
    println!(
        "{:<34}{:>12.2}{:>12.0}",
        "lanes, counter WITHOUT nomem",
        t_lanes_bar as f64 / reps as f64,
        ns(t_lanes_bar)
    );
    println!(
        "{:<34}{:>12.2}{:>12.0}",
        "serial, counter with nomem",
        t_ser_nomem as f64 / reps as f64,
        ns(t_ser_nomem)
    );
    println!(
        "{:<34}{:>12.2}{:>12.0}",
        "serial, counter WITHOUT nomem",
        t_ser_bar as f64 / reps as f64,
        ns(t_ser_bar)
    );

    // Honest reference: Instant, black_boxed, no asm anywhere.
    let t = std::time::Instant::now();
    let mut a = 0u64;
    for _ in 0..reps {
        a = a.wrapping_add(sat_lanes(black_box(&data), black_box(k), black_box(lim)));
    }
    black_box(a);
    println!(
        "\n{:<34}{:>24.0}",
        "lanes, Instant + black_box (ns)",
        t.elapsed().as_nanos() as f64 / reps as f64
    );
    let t = std::time::Instant::now();
    let mut b = 0u64;
    for _ in 0..reps {
        b = b.wrapping_add(sat_serial(black_box(&data), black_box(k), black_box(lim)));
    }
    black_box(b);
    println!(
        "{:<34}{:>24.0}",
        "serial, Instant + black_box (ns)",
        t.elapsed().as_nanos() as f64 / reps as f64
    );
}
