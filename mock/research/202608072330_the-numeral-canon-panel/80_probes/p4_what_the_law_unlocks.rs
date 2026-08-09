//! p4: what a law actually buys, and where it buys nothing.
//!
//! p3 established that a law-gated arm erases when its predicate is const, and it
//! also produced a result against the thesis it was built for: at F = 0 the
//! assembler emitted `only_general_f0 = sel_static::<0>`, a symbol alias, because
//! LLVM had already performed the distributive rewrite by itself. On that shape the
//! select stage bought nothing.
//!
//! So the question is sharper than "does the law hold". It is: **does the law let
//! the design reach a lowering the backend could not reach on its own?** That is the
//! only region where an arm is worth writing, and it is narrower than the region
//! where the law is true.
//!
//! This file looks for a shape where the answer is yes. The candidate is a
//! REDUCTION of saturating additions. A backend will not reassociate a reduction it
//! cannot prove associative, and saturating addition is not associative in general.
//! Unsigned saturating addition on a nonnegative window IS associative, which is a
//! fact the typestate has and the backend does not.
//!
//!   sat_sum_seq    : the fold as written, one accumulator, strictly left
//!   sat_sum_lanes  : four accumulators combined at the end, legal ONLY under the law
//!   wrap_sum_seq   : the control, wrapping addition, associative for free, so the
//!                    backend may reassociate this one without being told anything
//!
//! Build:
//!   rustc --edition 2021 -O --crate-type=lib --emit asm p4_what_the_law_unlocks.rs -o p4.s
//!   rustc --edition 2021 -O p4_what_the_law_unlocks.rs -o p4 && ./p4
//!
//! Toolchain: nightly-2026-05-28, aarch64-apple-darwin. No feature gates, no
//! target-feature flags beyond the default for this host.

/// The fold as a consumer writes it. One loop-carried accumulator.
#[inline(never)]
pub fn sat_sum_seq(xs: &[u8]) -> u8 {
    let mut acc: u8 = 0;
    for &x in xs {
        acc = acc.saturating_add(x);
    }
    acc
}

/// The same value, reassociated into four independent chains and combined at the
/// end. This is a different expression, and it computes the same answer only
/// because the operation is associative and 0 is its identity. Nothing in the
/// language checks that; the law is what licenses writing it.
#[inline(never)]
pub fn sat_sum_lanes(xs: &[u8]) -> u8 {
    let mut a: [u8; 4] = [0; 4];
    let mut i = 0;
    while i + 4 <= xs.len() {
        a[0] = a[0].saturating_add(xs[i]);
        a[1] = a[1].saturating_add(xs[i + 1]);
        a[2] = a[2].saturating_add(xs[i + 2]);
        a[3] = a[3].saturating_add(xs[i + 3]);
        i += 4;
    }
    let mut acc = a[0]
        .saturating_add(a[1])
        .saturating_add(a[2])
        .saturating_add(a[3]);
    while i < xs.len() {
        acc = acc.saturating_add(xs[i]);
        i += 1;
    }
    acc
}

/// Attack on the first result. `sat_sum_lanes` above got WORSE code than the
/// sequential form (90 instructions against 11, no vector instructions, four calls),
/// because indexed slice access `xs[i + 3]` is not provably in bounds, so the
/// backend emitted bounds checks and panic paths and gave up on vectorising.
///
/// The repair supplies the missing proof rather than the missing intrinsic: iterate
/// over `chunks_exact(16)`, whose element is an array of known length, so no bound
/// has to be proved at all. The reassociation is still licensed by the same law
/// (associativity, plus commutativity for the lane interleave, which the panel
/// separately measured free), and the identity element 0 is still needed for the
/// lane accumulators to start empty.
#[inline(never)]
pub fn sat_sum_lanes16(xs: &[u8]) -> u8 {
    let mut acc = [0u8; 16];
    let mut it = xs.chunks_exact(16);
    for c in &mut it {
        let c: &[u8; 16] = c.try_into().unwrap();
        for l in 0..16 {
            acc[l] = acc[l].saturating_add(c[l]);
        }
    }
    let mut total: u8 = 0;
    for l in 0..16 {
        total = total.saturating_add(acc[l]);
    }
    for &x in it.remainder() {
        total = total.saturating_add(x);
    }
    total
}

/// Second attack. `sat_sum_lanes16`'s loop is four instructions per sixteen
/// elements, but the control (`wrap_sum_seq`, which the backend vectorises on its
/// own) unrolls to four vector accumulators and sixty-four elements per iteration.
/// The law licenses the same unroll here; nothing else was stopping it. And the
/// horizontal combine at the end is done lane by lane in the previous arm, sixteen
/// scalar `umov`/`cmp`/`csel` triples, so it is folded here as a tree instead, which
/// the same associativity licenses.
#[inline(never)]
pub fn sat_sum_lanes64(xs: &[u8]) -> u8 {
    let mut a0 = [0u8; 16];
    let mut a1 = [0u8; 16];
    let mut a2 = [0u8; 16];
    let mut a3 = [0u8; 16];
    let mut it = xs.chunks_exact(64);
    for c in &mut it {
        for l in 0..16 {
            a0[l] = a0[l].saturating_add(c[l]);
            a1[l] = a1[l].saturating_add(c[16 + l]);
            a2[l] = a2[l].saturating_add(c[32 + l]);
            a3[l] = a3[l].saturating_add(c[48 + l]);
        }
    }
    // tree combine of the four accumulator vectors, then of the sixteen lanes
    let mut b = [0u8; 16];
    for l in 0..16 {
        b[l] = a0[l]
            .saturating_add(a1[l])
            .saturating_add(a2[l].saturating_add(a3[l]));
    }
    let mut w = 16;
    while w > 1 {
        let h = w / 2;
        for l in 0..h {
            b[l] = b[l].saturating_add(b[l + h]);
        }
        w = h;
    }
    let mut total = b[0];
    for &x in it.remainder() {
        total = total.saturating_add(x);
    }
    total
}

/// Control. Wrapping addition is associative unconditionally (it is addition in the
/// cyclic group of the width), so a backend is free to reassociate this one with no
/// help from any typestate.
#[inline(never)]
pub fn wrap_sum_seq(xs: &[u8]) -> u8 {
    let mut acc: u8 = 0;
    for &x in xs {
        acc = acc.wrapping_add(x);
    }
    acc
}

/// The law the reassociation rests on, checked over the whole domain rather than
/// asserted. Exhaustive at 8 bits, arity 3: 2^24 triples, run at RUNTIME, because
/// p2 established that a positive verdict at this width and arity is exactly what
/// const evaluation refuses.
pub fn assoc_holds_u8_saturating() -> (u64, u64) {
    let mut total: u64 = 0;
    let mut bad: u64 = 0;
    for a in 0..=255u8 {
        for b in 0..=255u8 {
            for c in 0..=255u8 {
                total += 1;
                if a.saturating_add(b).saturating_add(c) != a.saturating_add(b.saturating_add(c)) {
                    bad += 1;
                }
            }
        }
    }
    (total, bad)
}

#[cfg(not(feature = "lib"))]
fn main() {
    let n = 4096usize;
    let xs: Vec<u8> = (0..n).map(|i| ((i * 7 + 3) % 5) as u8).collect();

    println!("p4: what the law unlocks");
    let (total, bad) = assoc_holds_u8_saturating();
    println!(
        "  u8 saturating add, associativity, exhaustive: {} of {} triples fail",
        bad, total
    );
    println!(
        "  sat_sum_seq   = {}",
        sat_sum_seq(std::hint::black_box(&xs))
    );
    println!(
        "  sat_sum_lanes = {}",
        sat_sum_lanes(std::hint::black_box(&xs))
    );
    println!(
        "  sat_sum_lanes16 = {}",
        sat_sum_lanes16(std::hint::black_box(&xs))
    );
    println!(
        "  sat_sum_lanes64 = {}",
        sat_sum_lanes64(std::hint::black_box(&xs))
    );
    println!(
        "  wrap_sum_seq  = {}",
        wrap_sum_seq(std::hint::black_box(&xs))
    );

    // The reassociated arm must agree with the sequential one on every input, or the
    // law is not licensing what this file says it licenses.
    let mut disagree = 0;
    for len in 0..=300usize {
        for seed in 0..64usize {
            let v: Vec<u8> = (0..len)
                .map(|i| ((i * 31 + seed * 17) % 251) as u8)
                .collect();
            if sat_sum_seq(&v) != sat_sum_lanes(&v) {
                disagree += 1;
            }
            if sat_sum_seq(&v) != sat_sum_lanes16(&v) {
                disagree += 1;
            }
            if sat_sum_seq(&v) != sat_sum_lanes64(&v) {
                disagree += 1;
            }
        }
    }
    println!(
        "  seq vs the three reassociated arms, lengths 0..=300 x 64 seeds: {} disagreements",
        disagree
    );
}
