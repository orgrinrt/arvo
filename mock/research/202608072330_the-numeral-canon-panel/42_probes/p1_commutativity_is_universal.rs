// p1: is commutativity of + and * worth tracking as a per-strategy fact, or
// is it a structural theorem that holds regardless of overflow policy and
// sign domain?
//
// 35 and 40 measured associativity, distributivity, monotonicity, the
// absorbing top and the additive inverse as varying per overflow policy and
// per sign domain (35 sections 3.5 through 3.7; 40 section 5.1's axis table).
// 18 measured commutativity of + and * for INTERVAL arithmetic under outward
// rounding (18 section 2.2, both at 100%), but nothing in the panel measured
// scalar commutativity under the overflow policies that gate every other
// law here. This closes that gap directly, exhaustively rather than by
// argument, because "run the experiment, not the argument" applies to a
// claim that looks obvious as much as to one that does not.
//
// Model: same as 35_probes/p2_laws.rs. A numeral of total width W with F
// fractional bits holds raw values in [0, 2^W) unsigned or a signed range
// for the signed arm. Addition adds raws (or the two's-complement
// equivalent); multiplication multiplies and rescales by F. The policy
// decides what happens when the exact result leaves the representable
// range: wrap takes it modulo 2^W, saturate clamps to the nearest end.
//
// The claim under test: clip(a+b) == clip(b+a) and clip(shift(a*b)) ==
// clip(shift(b*a)) for EVERY pair, EVERY (W,F) in the box, EVERY policy,
// EVERY sign domain, because the pre-clip quantity (a+b or a*b) is already
// symmetric in a and b before the policy or the width ever act on it. If
// that is right, commutativity is not a per-strategy axis fact at all: it
// is a free consequence of building the operation as (clip or clamp) after
// (exact addition or exact multiplication), and it costs the design nothing
// to state once, for every strategy, rather than per strategy.
//
// Exhaustive over the whole (W,F) box, W = 1..=7, F = 0..=W, every ordered
// pair (unsigned) or triple-free pair (signed), both policies.
//
// Build and run:
//   rustc +nightly-2026-05-28 -O --edition 2021 -o p1 p1_commutativity_is_universal.rs && ./p1

#[derive(Clone, Copy, PartialEq, Eq)]
enum Policy {
    Wrap,
    Saturate,
}

// ---------------------------------------------------------------------------
// Unsigned
// ---------------------------------------------------------------------------

#[inline(always)]
fn u_clip(x: u128, w: u32, p: Policy) -> u128 {
    let m: u128 = 1u128 << w;
    match p {
        Policy::Wrap => x & (m - 1),
        Policy::Saturate => {
            if x >= m {
                m - 1
            } else {
                x
            }
        }
    }
}

#[inline(always)]
fn u_add(a: u128, b: u128, w: u32, p: Policy) -> u128 {
    u_clip(a + b, w, p)
}

#[inline(always)]
fn u_mul(a: u128, b: u128, w: u32, f: u32, p: Policy) -> u128 {
    u_clip((a * b) >> f, w, p)
}

fn unsigned_box(policy: Policy) -> (u64, u64, u64, u64) {
    let mut add_pairs = 0u64;
    let mut add_fail = 0u64;
    let mut mul_pairs = 0u64;
    let mut mul_fail = 0u64;
    for w in 1u32..=7 {
        for f in 0u32..=w {
            let n: u128 = 1u128 << w;
            for a in 0u128..n {
                for b in 0u128..n {
                    add_pairs += 1;
                    if u_add(a, b, w, policy) != u_add(b, a, w, policy) {
                        add_fail += 1;
                    }
                    mul_pairs += 1;
                    if u_mul(a, b, w, f, policy) != u_mul(b, a, w, f, policy) {
                        mul_fail += 1;
                    }
                }
            }
        }
    }
    (add_pairs, add_fail, mul_pairs, mul_fail)
}

// ---------------------------------------------------------------------------
// Signed, two's complement raw storage, same shift-and-clip mechanism.
// ---------------------------------------------------------------------------

#[inline(always)]
fn s_clip(x: i128, w: u32, p: Policy) -> i128 {
    // width w total bits, signed range [-2^(w-1), 2^(w-1) - 1]
    let lo: i128 = -(1i128 << (w - 1));
    let hi: i128 = (1i128 << (w - 1)) - 1;
    match p {
        Policy::Wrap => {
            let m: i128 = 1i128 << w;
            let mut y = x % m;
            if y < lo {
                y += m;
            }
            if y > hi {
                y -= m;
            }
            y
        }
        Policy::Saturate => {
            if x < lo {
                lo
            } else if x > hi {
                hi
            } else {
                x
            }
        }
    }
}

#[inline(always)]
fn s_add(a: i128, b: i128, w: u32, p: Policy) -> i128 {
    s_clip(a + b, w, p)
}

#[inline(always)]
fn s_mul(a: i128, b: i128, w: u32, f: u32, p: Policy) -> i128 {
    // arithmetic shift right by f, truncating toward negative infinity,
    // which is what a signed fixed-point multiply's rescale does.
    let prod = a * b;
    let shifted = prod >> f;
    s_clip(shifted, w, p)
}

fn signed_box(policy: Policy) -> (u64, u64, u64, u64) {
    let mut add_pairs = 0u64;
    let mut add_fail = 0u64;
    let mut mul_pairs = 0u64;
    let mut mul_fail = 0u64;
    for w in 2u32..=7 {
        for f in 0u32..=(w - 1) {
            let lo: i128 = -(1i128 << (w - 1));
            let hi: i128 = (1i128 << (w - 1)) - 1;
            for a in lo..=hi {
                for b in lo..=hi {
                    add_pairs += 1;
                    if s_add(a, b, w, policy) != s_add(b, a, w, policy) {
                        add_fail += 1;
                    }
                    mul_pairs += 1;
                    if s_mul(a, b, w, f, policy) != s_mul(b, a, w, f, policy) {
                        mul_fail += 1;
                    }
                }
            }
        }
    }
    (add_pairs, add_fail, mul_pairs, mul_fail)
}

fn main() {
    println!("commutativity, exhaustive, W = 1..=7 unsigned / 2..=7 signed, F = 0..=W or W-1");
    println!();
    for (name, policy) in [("wrap", Policy::Wrap), ("saturate", Policy::Saturate)] {
        let (ap, af, mp, mf) = unsigned_box(policy);
        println!(
            "unsigned {:<9} add: {:>10} pairs, {:>6} commute-failures | mul: {:>10} pairs, {:>6} commute-failures",
            name, ap, af, mp, mf
        );
    }
    for (name, policy) in [("wrap", Policy::Wrap), ("saturate", Policy::Saturate)] {
        let (ap, af, mp, mf) = signed_box(policy);
        println!(
            "signed   {:<9} add: {:>10} pairs, {:>6} commute-failures | mul: {:>10} pairs, {:>6} commute-failures",
            name, ap, af, mp, mf
        );
    }
    println!();
    println!("if every commute-failures column above reads 0, commutativity of + and * holds");
    println!("under both overflow policies and both sign domains, over the whole box measured,");
    println!("and is therefore not a fact that varies with the strategy: it is free.");
}
