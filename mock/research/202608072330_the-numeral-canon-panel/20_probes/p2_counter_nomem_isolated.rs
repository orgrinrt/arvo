//! p1 was flawed: its four timing brackets all stored into one `out.value`,
//! so three of the four stores were dead and LLVM could delete the work
//! behind them. That made the `WITHOUT nomem` lanes arm read zero for a
//! reason that had nothing to do with the counter. This is the same
//! experiment with each arm in its own loop and its own live accumulator,
//! so nothing is dead and the only variable is the asm option.
//!
//! Build: rustc +nightly-2026-05-28 -O p2_counter_nomem_isolated.rs

use std::hint::black_box;

#[inline(always)]
fn read_nomem() -> u64 {
    let v: u64;
    unsafe {
        core::arch::asm!("mrs {}, CNTVCT_EL0", out(reg) v, options(nostack, nomem));
    }
    v
}
#[inline(always)]
fn read_barrier() -> u64 {
    let v: u64;
    unsafe {
        core::arch::asm!("mrs {}, CNTVCT_EL0", out(reg) v, options(nostack));
    }
    v
}

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

/// One arm: `reps` timed calls, each storing through `&mut Out` exactly as a
/// bench variant does, with the store kept live by folding it out afterwards.
macro_rules! arm {
    ($name:literal, $rd:ident, $work:ident, $data:expr, $k:expr, $lim:expr, $reps:expr) => {{
        let mut out = Out { value: 0 };
        let mut sink = 0u64;
        let mut ticks = 0u64;
        for _ in 0..$reps {
            let s = $rd();
            out.value = $work($data, $k, $lim);
            let e = $rd();
            ticks += e - s;
            sink = sink.wrapping_add(out.value);
        }
        black_box(sink);
        let per = ticks as f64 / $reps as f64;
        println!("{:<36}{:>12.2}{:>12.0}", $name, per, per * (1e9 / 24e6));
    }};
}

fn main() {
    const N: usize = 8192;
    let data: Vec<u64> = (0..N as u64).map(|i| i.wrapping_mul(2654435761)).collect();
    let k: u64 = u64::MAX / 3 | 1;
    let lim = u64::MAX;
    let reps = 500usize;

    let mut w = 0u64;
    for _ in 0..500 {
        w = w.wrapping_add(sat_lanes(black_box(&data), k, lim));
    }
    black_box(w);

    println!("N = {N}, {reps} reps, 3 ops/element, saturating, u64\n");
    println!("{:<36}{:>12}{:>12}", "arm", "ticks/rep", "ns/rep");
    arm!(
        "lanes,  counter WITH nomem",
        read_nomem,
        sat_lanes,
        &data,
        k,
        lim,
        reps
    );
    arm!(
        "lanes,  counter WITHOUT nomem",
        read_barrier,
        sat_lanes,
        &data,
        k,
        lim,
        reps
    );
    arm!(
        "serial, counter WITH nomem",
        read_nomem,
        sat_serial,
        &data,
        k,
        lim,
        reps
    );
    arm!(
        "serial, counter WITHOUT nomem",
        read_barrier,
        sat_serial,
        &data,
        k,
        lim,
        reps
    );

    let t = std::time::Instant::now();
    let mut a = 0u64;
    for _ in 0..reps {
        a = a.wrapping_add(sat_lanes(black_box(&data), black_box(k), black_box(lim)));
    }
    black_box(a);
    println!(
        "\n{:<36}{:>24.0}",
        "lanes,  Instant + black_box (ns)",
        t.elapsed().as_nanos() as f64 / reps as f64
    );
    let t = std::time::Instant::now();
    let mut b = 0u64;
    for _ in 0..reps {
        b = b.wrapping_add(sat_serial(black_box(&data), black_box(k), black_box(lim)));
    }
    black_box(b);
    println!(
        "{:<36}{:>24.0}",
        "serial, Instant + black_box (ns)",
        t.elapsed().as_nanos() as f64 / reps as f64
    );
}
