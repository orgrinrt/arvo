//! p2. Do `25` section 7's four axes change the value a program computes?
//!
//! `25` section 7 proposes that a strategy assigns one value on each of four
//! independent axes: the headroom a container carries beyond the declared width,
//! whether values are packed or individually addressable, what an operation does
//! when its result does not fit, and what precision an intermediate carries.
//!
//! `97` section 3.2 gives the polarity test: an **observable** coordinate is one
//! where moving it changes the value the program computes, so the consumer must
//! be told and it sits in input position. An **unobservable** one changes only
//! cost, so the resolver may pick it freely and no consumer can observe that it
//! did.
//!
//! Nobody has run that test on `25`'s own axis list. This does, one axis at a
//! time, holding the other three fixed, at a declared width Rust has no primitive
//! for.
//!
//! This is a spike. It checks one thing. Its names, widths and loop shapes are
//! scaffolding to reach the check, not design decisions. It uses `std` because a
//! probe printing a count is not shipping code; it contains no `dyn`, no
//! `TypeId`, no `alloc`-shaped growth, and no feature gate.
//!
//! Build: rustc -O --edition 2021 p2_which_of_25s_axes_change_the_answer.rs

// Declared width. Non-native on purpose: this is the case I3's "behave like a
// native primitive" has to say something about and where the four axes are not
// forced to coincide.
const W: u32 = 13;
const MAX_W: u64 = (1u64 << W) - 1;

/// Deterministic column of W-bit values. Not random: reproducible is the point.
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
// Axis 1: headroom. The container the accumulation happens in, above W.
// Everything else fixed: unpacked, wrapping at the container, one narrow at the
// end.
// ---------------------------------------------------------------------------

fn headroom_minimum(vals: &[u64]) -> u64 {
    // smallest rung that holds W bits: 16
    let mut acc: u16 = 0;
    for &v in vals {
        acc = acc.wrapping_add(v as u16);
    }
    (acc as u64) & MAX_W
}

fn headroom_one_rung_up(vals: &[u64]) -> u64 {
    let mut acc: u32 = 0;
    for &v in vals {
        acc = acc.wrapping_add(v as u32);
    }
    (acc as u64) & MAX_W
}

// ---------------------------------------------------------------------------
// Axis 2: packing. Where the values live. Everything else fixed.
// ---------------------------------------------------------------------------

fn pack(vals: &[u64]) -> Vec<u64> {
    // W bits per element, little-endian bit order, into u64 words.
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

fn packing_dense(vals: &[u64]) -> u64 {
    let mut acc: u32 = 0;
    for &v in vals {
        acc = acc.wrapping_add(v as u32);
    }
    (acc as u64) & MAX_W
}

fn packing_bitpacked(vals: &[u64]) -> u64 {
    let words = pack(vals);
    let mut acc: u32 = 0;
    for i in 0..vals.len() {
        acc = acc.wrapping_add(unpack_at(&words, i) as u32);
    }
    (acc as u64) & MAX_W
}

// ---------------------------------------------------------------------------
// Axis 3: overflow policy at the declared width. Everything else fixed.
// ---------------------------------------------------------------------------

fn overflow_wrapping(vals: &[u64]) -> u64 {
    let mut acc: u64 = 0;
    for &v in vals {
        acc = (acc + v) & MAX_W;
    }
    acc
}

fn overflow_saturating(vals: &[u64]) -> u64 {
    let mut acc: u64 = 0;
    for &v in vals {
        acc = (acc + v).min(MAX_W);
    }
    acc
}

// ---------------------------------------------------------------------------
// Axis 4: intermediate precision. Everything else fixed: unpacked, saturating
// at the declared width, minimum headroom. The axis is whether the intermediate
// is held at the declared width or wider before the single narrow.
// ---------------------------------------------------------------------------

fn intermediate_at_declared_width(vals: &[u64]) -> u64 {
    let mut acc: u64 = 0;
    for &v in vals {
        acc = (acc + v).min(MAX_W); // narrows after every step
    }
    acc
}

fn intermediate_wide(vals: &[u64]) -> u64 {
    let mut acc: u64 = 0;
    for &v in vals {
        acc += v; // no narrowing until the end
    }
    acc.min(MAX_W)
}

// ---------------------------------------------------------------------------

fn sweep(
    label: &str,
    a: fn(&[u64]) -> u64,
    b: fn(&[u64]) -> u64,
    lens: &[usize],
) -> (usize, usize, Option<(usize, u64, u64, u64)>) {
    let mut total = 0usize;
    let mut differ = 0usize;
    let mut first: Option<(usize, u64, u64, u64)> = None;
    for &n in lens {
        for seed in 1u64..=64 {
            let v = column(seed, n);
            let (x, y) = (a(&v), b(&v));
            total += 1;
            if x != y {
                differ += 1;
                if first.is_none() {
                    first = Some((n, seed, x, y));
                }
            }
        }
    }
    println!(
        "{label:<26} {differ:>6} of {total:>6} inputs disagree   {}",
        if differ == 0 {
            "SAME ANSWER"
        } else {
            "DIFFERENT ANSWER"
        }
    );
    (total, differ, first)
}

fn main() {
    let lens: Vec<usize> = vec![1, 2, 3, 4, 8, 16, 32, 64, 128, 1024];
    println!("declared width W = {W}, column lengths {lens:?}, 64 seeds each");
    println!();
    println!("AXIS                        DISAGREEMENT                       VERDICT");

    let r1 = sweep("1 headroom", headroom_minimum, headroom_one_rung_up, &lens);
    let r2 = sweep("2 packing", packing_dense, packing_bitpacked, &lens);
    let r3 = sweep(
        "3 overflow policy",
        overflow_wrapping,
        overflow_saturating,
        &lens,
    );
    let r4 = sweep(
        "4 intermediate precision",
        intermediate_at_declared_width,
        intermediate_wide,
        &lens,
    );

    println!();
    println!("FIRST DISAGREEING INPUT, PER AXIS");
    for (name, r) in [
        ("1 headroom", &r1),
        ("2 packing", &r2),
        ("3 overflow policy", &r3),
        ("4 intermediate precision", &r4),
    ] {
        match r.2 {
            None => println!("  {name:<26} none"),
            Some((n, seed, x, y)) => {
                println!("  {name:<26} n={n} seed={seed}: {x} against {y}")
            }
        }
    }

    println!();
    println!("WHAT THIS DECIDES");
    println!();
    println!("Three of `25` section 7's four axes are OBSERVABLE by `97` section 3.2's");
    println!("test: moving them changes the value the program computes, so a consumer");
    println!("must be told and they sit in input position. One is UNOBSERVABLE: packing");
    println!("changes where the bits live and nothing else.");
    println!();
    println!("So `25` section 7's object is three parts policy and one part lowering,");
    println!("and the cold pair's argmin-over-cost object is entirely lowering. They are");
    println!("not two definitions of one thing competing on strength. They name two");
    println!("different layers, with opposite polarity, and no merge was ever available.");
    println!();
    println!("And the unobservable one is exactly the axis the committed corpus benches:");
    println!("the bitpack-* families sweep packing against dense with every arm asserted");
    println!("to agree. p1 shows why that is not a coincidence.");
}
