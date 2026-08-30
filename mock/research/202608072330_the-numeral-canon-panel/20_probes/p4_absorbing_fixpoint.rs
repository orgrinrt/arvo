//! Why the saturating lane arms return in nanoseconds with the right answer.
//!
//! Hypothesis: the fold has a provable absorbing fixpoint. At W=13 the step
//! sequence is min(v+k,lim), sat_sub(v,k), min(v+k,lim) with k = 2731 and
//! lim = 8191, so interval arithmetic alone proves every step result is at
//! least k. The lane accumulator is min(lane + step, lim), so after three
//! elements every lane is provably lim, and the remaining 1021 iterations
//! per lane are the identity. LLVM peels, proves the constant, and deletes
//! the loop. The answer is right and no element after the third is read.
//!
//! Three arms distinguish the readings:
//!   const  : k and lim visible to the optimiser, as in the bench variant
//!   opaque : k and lim behind black_box, as in the crate's own diagnostic test
//!   nofix  : same shape, wrapping instead of saturating, so no fixpoint exists
//!
//! If the hypothesis holds, `const` collapses, `opaque` does not, and the
//! answer is identical, and `const` is insensitive to the input entirely.
//!
//! Build: rustc +nightly-2026-05-28 -O p4_absorbing_fixpoint.rs

use std::hint::black_box;
use std::time::Instant;

const W: u32 = 13;
const LIM: u16 = (1u16 << W) - 1;
const K: u16 = (LIM / 3) | 1;

#[inline(always)]
fn sat_steps(mut v: u16, k: u16, lim: u16) -> u16 {
    for j in 0..3 {
        v = if j % 2 == 0 {
            v.saturating_add(k).min(lim)
        } else {
            v.saturating_sub(k)
        };
    }
    v
}

#[inline(always)]
fn sat_lanes(data: &[u16], k: u16, lim: u16) -> u16 {
    const L: usize = 8;
    let mut lanes = [0u16; L];
    for ch in data.chunks_exact(L) {
        for l in 0..L {
            lanes[l] = lanes[l].saturating_add(sat_steps(ch[l], k, lim)).min(lim);
        }
    }
    let mut acc = 0u16;
    for l in 0..L {
        acc = acc.saturating_add(lanes[l]).min(lim);
    }
    acc
}

#[inline(always)]
fn wrap_lanes(data: &[u16], k: u16, lim: u16) -> u16 {
    const L: usize = 8;
    let mut lanes = [0u16; L];
    for ch in data.chunks_exact(L) {
        for l in 0..L {
            let mut v = ch[l];
            for j in 0..3 {
                v = match j {
                    0 => v.wrapping_add(k),
                    1 => v.wrapping_mul(3),
                    _ => v.wrapping_sub(k),
                };
                v &= lim;
            }
            lanes[l] = lanes[l].wrapping_add(v) & lim;
        }
    }
    let mut acc = 0u16;
    for l in 0..L {
        acc = acc.wrapping_add(lanes[l]) & lim;
    }
    acc
}

fn time<F: Fn() -> u16>(label: &str, reps: usize, f: F) -> u16 {
    for _ in 0..64 {
        black_box(f());
    }
    let t = Instant::now();
    let mut s = 0u32;
    for _ in 0..reps {
        s = s.wrapping_add(black_box(f()) as u32);
    }
    let ns = t.elapsed().as_nanos() as f64 / reps as f64;
    let v = f();
    println!("{:<44}{:>12.0}{:>12}", label, ns, v);
    black_box(s);
    v
}

fn main() {
    const N: usize = 8192;
    let a: Vec<u16> = (0..N as u16).map(|i| i.wrapping_mul(40503) & LIM).collect();
    // A second column that shares nothing with the first past element three.
    let mut b: Vec<u16> = vec![0u16; N];
    for i in 0..N {
        b[i] = if i < 24 {
            a[i]
        } else {
            (i as u16).wrapping_mul(7) & LIM
        };
    }
    let reps = 2000usize;

    println!("W={W} lim={LIM} k={K}  N={N}, 3 ops/element\n");
    println!("{:<44}{:>12}{:>12}", "arm", "ns/call", "answer");

    let s1 = time("saturating, const k and lim (bench shape)", reps, || {
        sat_lanes(&a, K, LIM)
    });
    let s2 = time("saturating, opaque k and lim (test shape)", reps, || {
        sat_lanes(black_box(&a), black_box(K), black_box(LIM))
    });
    let w1 = time("wrapping,   const k and lim (no fixpoint)", reps, || {
        wrap_lanes(&a, K, LIM)
    });

    println!();
    let s1b = time("saturating, const, DIFFERENT column b", reps, || {
        sat_lanes(&b, K, LIM)
    });
    let w1b = time("wrapping,   const, DIFFERENT column b", reps, || {
        wrap_lanes(&b, K, LIM)
    });

    println!("\nsaturating agrees across shapes: {}", s1 == s2);
    println!(
        "saturating answer is input-insensitive past element 3: a={s1} b={s1b} same={}",
        s1 == s1b
    );
    println!(
        "wrapping   answer IS input-sensitive:                 a={w1} b={w1b} same={}",
        w1 == w1b
    );
}
