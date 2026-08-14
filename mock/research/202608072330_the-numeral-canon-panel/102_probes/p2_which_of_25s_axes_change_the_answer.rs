//! p2. Do `25` section 7's four axes change the value a program computes, and where?
//!
//! `25` section 7 proposes that a strategy assigns one value on each of four
//! independent axes: the headroom a container carries beyond the declared width,
//! whether values are packed or individually addressable, what an operation does
//! when its result does not fit, and what precision an intermediate carries.
//!
//! `97` section 3.2 gives the polarity test: an **observable** coordinate is one
//! where moving it changes the value the program computes, so the consumer must be
//! told and it sits in input position. An **unobservable** one changes only cost,
//! so the resolver may pick it freely and no consumer can observe that it did.
//!
//! Nobody has run that test on `25`'s own axis list.
//!
//! ## The first version of this probe was setup that helps, and the fix is the finding
//!
//! Version one swept only additive chains ending in a mask, and reported headroom
//! and intermediate precision as SAME ANSWER: 0 of 640. That output is kept beside
//! this file as `p2_first_version_setup_that_helped.out`. It is not noise, it is a
//! theorem I had not noticed: reduction mod `2^W` is a ring homomorphism, so any
//! composition of `+`, `-` and `*` gives the same low `W` bits whatever width it was
//! computed at. A construction made only of those cannot show a headroom difference
//! and cannot show an intermediate-width difference. My sweep proved the law it was
//! standing on rather than the axis it was aiming at.
//!
//! So the axis is not the whole story and neither is the verdict. What decides it is
//! whether a **non-ring step** intervenes: a shift, a division, a saturation, a
//! comparison. Version two sweeps both regimes and reports the boundary, which is a
//! predicate rather than a verdict.
//!
//! This is a spike. It checks one thing. Its names, widths and loop shapes are
//! scaffolding to reach the check, not design decisions. It uses `std` because a
//! probe printing a count is not shipping code; it contains no `dyn`, no `TypeId`,
//! no `alloc`-shaped growth, and no feature gate.
//!
//! Build: rustc -O --edition 2021 p2_which_of_25s_axes_change_the_answer.rs

const W: u32 = 13;
const MAX_W: u64 = (1u64 << W) - 1;

fn column(seed: u64, n: usize) -> Vec<u64> {
    let mut x = seed | 1;
    (0..n)
        .map(|_| {
            x ^= x << 13;
            x ^= x >> 7;
            x ^= x << 17;
            x & MAX_W
        })
        .collect()
}

// ---------------------------------------------------------------------------
// Two regimes. RING: only + and * and a final mask. NONRING: the same chain with
// one shift in it, which is the cheapest non-ring step there is.
// ---------------------------------------------------------------------------

// Axis 1, headroom: the width of the container the work happens in.

fn headroom_min_ring(v: &[u64]) -> u64 {
    let mut a: u16 = 1;
    for &x in v {
        a = a
            .wrapping_mul(x as u16)
            .wrapping_add(x as u16)
            .wrapping_sub(7);
    }
    (a as u64) & MAX_W
}
fn headroom_up_ring(v: &[u64]) -> u64 {
    let mut a: u32 = 1;
    for &x in v {
        a = a
            .wrapping_mul(x as u32)
            .wrapping_add(x as u32)
            .wrapping_sub(7);
    }
    (a as u64) & MAX_W
}
fn headroom_min_nonring(v: &[u64]) -> u64 {
    let mut a: u16 = 1;
    for &x in v {
        a = (a.wrapping_mul(x as u16) >> 3).wrapping_add(x as u16);
    }
    (a as u64) & MAX_W
}
fn headroom_up_nonring(v: &[u64]) -> u64 {
    let mut a: u32 = 1;
    for &x in v {
        a = (a.wrapping_mul(x as u32) >> 3).wrapping_add(x as u32);
    }
    (a as u64) & MAX_W
}

// Axis 2, packing: where the bits live. Same arithmetic either way.

fn pack(vals: &[u64]) -> Vec<u64> {
    let mut words = vec![0u64; (vals.len() * W as usize).div_ceil(64) + 1];
    for (i, &v) in vals.iter().enumerate() {
        let bit = i * W as usize;
        let (word, off) = (bit / 64, bit % 64);
        words[word] |= v << off;
        if off + W as usize > 64 {
            words[word + 1] |= v >> (64 - off);
        }
    }
    words
}
fn unpack_at(words: &[u64], i: usize) -> u64 {
    let bit = i * W as usize;
    let (word, off) = (bit / 64, bit % 64);
    let mut v = words[word] >> off;
    if off + W as usize > 64 {
        v |= words[word + 1] << (64 - off);
    }
    v & MAX_W
}
fn packing_dense(v: &[u64]) -> u64 {
    let mut a: u32 = 1;
    for &x in v {
        a = (a.wrapping_mul(x as u32) >> 3).wrapping_add(x as u32);
    }
    (a as u64) & MAX_W
}
fn packing_bitpacked(v: &[u64]) -> u64 {
    let w = pack(v);
    let mut a: u32 = 1;
    for i in 0..v.len() {
        let x = unpack_at(&w, i);
        a = (a.wrapping_mul(x as u32) >> 3).wrapping_add(x as u32);
    }
    (a as u64) & MAX_W
}

// Axis 3, overflow policy at the declared width.

fn overflow_wrapping(v: &[u64]) -> u64 {
    let mut a: u64 = 0;
    for &x in v {
        a = (a + x) & MAX_W;
    }
    a
}
fn overflow_saturating(v: &[u64]) -> u64 {
    let mut a: u64 = 0;
    for &x in v {
        a = (a + x).min(MAX_W);
    }
    a
}

// Axis 4, intermediate precision: how wide the value is held between steps.
// RING regime: narrow after each step against narrow once, additively.
// NONRING regime: the same, with the shift that makes the lost bits matter.

fn intermediate_narrow_ring(v: &[u64]) -> u64 {
    let mut a: u64 = 0;
    for &x in v {
        a = (a + x) & MAX_W;
    }
    a
}
fn intermediate_wide_ring(v: &[u64]) -> u64 {
    let mut a: u64 = 0;
    for &x in v {
        a += x;
    }
    a & MAX_W
}
fn intermediate_narrow_nonring(v: &[u64]) -> u64 {
    // the product is narrowed to the declared width before the shift
    let mut a: u64 = 1;
    for &x in v {
        a = (((a * x) & MAX_W) >> 3) + x;
        a &= MAX_W;
    }
    a
}
fn intermediate_wide_nonring(v: &[u64]) -> u64 {
    // the product keeps its full width across the shift, narrowing after
    let mut a: u64 = 1;
    for &x in v {
        a = ((a * x) >> 3) + x;
        a &= MAX_W;
    }
    a
}

// ---------------------------------------------------------------------------

fn sweep(a: fn(&[u64]) -> u64, b: fn(&[u64]) -> u64, lens: &[usize]) -> (usize, usize) {
    let (mut total, mut differ) = (0usize, 0usize);
    for &n in lens {
        for seed in 1u64..=64 {
            let v = column(seed, n);
            total += 1;
            if a(&v) != b(&v) {
                differ += 1;
            }
        }
    }
    (total, differ)
}

fn row(label: &str, ring: (usize, usize), nonring: (usize, usize)) {
    let verdict = match (ring.1 > 0, nonring.1 > 0) {
        (false, false) => "UNOBSERVABLE everywhere swept",
        (false, true) => "OBSERVABLE only past a non-ring step",
        (true, true) => "OBSERVABLE in both regimes",
        (true, false) => "observable in the ring regime only (unexpected)",
    };
    println!(
        "{label:<26} {:>4}/{:<4}   {:>4}/{:<4}   {verdict}",
        ring.1, ring.0, nonring.1, nonring.0
    );
}

fn main() {
    let lens: Vec<usize> = vec![1, 2, 3, 4, 8, 16, 32, 64, 128, 1024];
    println!("declared width W = {W}, column lengths {lens:?}, 64 seeds each");
    println!();
    println!("RING     = chain of +, - and * only, masked to W at the end");
    println!("NONRING  = the same chain with one `>> 3` in it");
    println!();
    println!("{:<26} {:>9}   {:>9}   VERDICT", "AXIS", "ring", "nonring");

    row(
        "1 headroom",
        sweep(headroom_min_ring, headroom_up_ring, &lens),
        sweep(headroom_min_nonring, headroom_up_nonring, &lens),
    );
    // packing carries no ring/nonring distinction: it is the same arithmetic
    // either way by construction, so both columns run the nonring chain.
    let pk = sweep(packing_dense, packing_bitpacked, &lens);
    row("2 packing", pk, pk);
    let ov = sweep(overflow_wrapping, overflow_saturating, &lens);
    row("3 overflow policy", ov, ov);
    row(
        "4 intermediate precision",
        sweep(intermediate_narrow_ring, intermediate_wide_ring, &lens),
        sweep(
            intermediate_narrow_nonring,
            intermediate_wide_nonring,
            &lens,
        ),
    );

    println!();
    println!("WHAT THIS DECIDES");
    println!();
    println!("`25` section 7's axis list is not one kind of thing. Packing is");
    println!("UNOBSERVABLE: it moves bits and nothing else, and it is exactly the axis");
    println!("the committed bitpack-* families bench with every arm asserted to agree.");
    println!("Overflow policy is OBSERVABLE unconditionally. Headroom and intermediate");
    println!("precision are observable CONDITIONALLY, and the condition is the same one");
    println!("for both: they are invisible across any composition of + - * because");
    println!("reduction mod 2^W is a ring homomorphism, and they become visible at the");
    println!("first step that is not a ring operation.");
    println!();
    println!("Two consequences the unit does not currently carry.");
    println!();
    println!("ONE. `25` section 7 and the cold pair's argmin definition describe two");
    println!("different layers with opposite polarity, not two strengths of one claim.");
    println!("Three of `25`'s four axes sit in input position; an argmin over cost");
    println!("ranges over output position. No merge of the two was ever available, and");
    println!("the 72-of-15625 gap `97` measured is the smaller of the two differences.");
    println!();
    println!("TWO. Headroom and intermediate precision have the same observability");
    println!("predicate here, which is evidence toward `25` section 8's own open");
    println!("question about whether that column is one axis or two, from the side");
    println!("nobody has looked at: they are distinguishable as mechanisms and they are");
    println!("not distinguishable by what a consumer can observe.");
}
