//! q4. The obligation a `Format`'s coordinates do not carry, derived and swept.
//!
//! q3 section 3 refuted my own claim that every reachable set is one geometric
//! ladder: 93 of 240 swept coordinate tuples denote a set whose distinct gaps are
//! not powers of one ratio, with shapes like `[1, 4, 5, 8]` and `[1, 2, 3]`. This
//! probe is the attack on that.
//!
//! **The mechanism.** The denoted set is `union over m of { s * r^(SLOPE*m) }` over
//! one slot range `[a, b]`. Shell `m+1` is coarser than shell `m` by a factor
//! `R = radix^SLOPE`. Where shell `m` stops, shell `m+1` has to take over, and the
//! join is clean exactly when the distance from `b` up to the first shell-`m+1`
//! point above it is itself a power of `R`, and likewise from `a` downward. Where
//! it is not, the union carries a gap that is no power of `R`, and the set is not
//! a numeral of any convention: it is a ragged union nobody asked for.
//!
//! **The obligation.** `d_up` is the distance from `b` to the least multiple of `R`
//! strictly above it, `d_down` the distance from `a` to the greatest multiple of
//! `R` strictly below it. The tuple denotes a clean ladder exactly when both are
//! powers of `R`. That is decidable in const time from the coordinates a `Format`
//! already carries, and it is the format-level analogue of `Slots::ADMITTED`,
//! which the crate does carry at the slot level.
//!
//! **The cases that must fail, stated before the run.**
//!
//! The sweep must contain tuples the oracle calls clean AND tuples it calls
//! ragged, or the comparison is answering one way.
//!
//! A mutant obligation that checks only `d_up` must be reported as differing from
//! the oracle, or the sweep never reaches a tuple that is ragged only at the
//! bottom and the second half of the condition is unexercised.
//!
//! A second mutant that returns `true` always must be reported as differing, which
//! is the same demand stated so a reader can see it was made.
//!
//! Build: `rustc --edition 2024 -O q4_the_obligation_the_format_does_not_carry.rs -o /tmp/q4`

use std::collections::BTreeSet;

fn union_set(radix: i128, slope: u32, magnitudes: u32, min: i64, max: i64) -> BTreeSet<i128> {
    let mut out = BTreeSet::new();
    for m in 0..magnitudes {
        let step = radix.pow(slope * m);
        for s in min..=max {
            out.insert(s as i128 * step);
        }
    }
    out
}

fn gaps(set: &BTreeSet<i128>) -> Vec<i128> {
    let v: Vec<i128> = set.iter().copied().collect();
    v.windows(2).map(|w| w[1] - w[0]).collect()
}

/// The oracle: are every gap's values powers of `R`, so the set is one ladder.
///
/// Computed from the enumerated set, so it knows nothing about the condition being
/// tested.
fn is_clean_ladder(set: &BTreeSet<i128>, r: i128) -> bool {
    if r <= 1 {
        // One shell, or a degenerate ratio. Every gap equal is the constant grid.
        let g = gaps(set);
        return g.windows(2).all(|w| w[0] == w[1]);
    }
    gaps(set).into_iter().all(|g| is_power_of(g, r))
}

fn is_power_of(mut n: i128, r: i128) -> bool {
    if n <= 0 || r <= 1 {
        return false;
    }
    while n % r == 0 {
        n /= r;
    }
    n == 1
}

/// The derived obligation, from the coordinates alone.
///
/// Written in the shape a `const fn` over a `Format`'s associated items would
/// take: integer arithmetic and a bounded loop, no set, no allocation.
fn shells_tile(radix: i128, slope: u32, magnitudes: u32, min: i64, max: i64) -> bool {
    if magnitudes <= 1 {
        // One shell. A single arithmetic progression is a ladder by construction.
        return true;
    }
    let r = radix.pow(slope);
    if r <= 1 {
        // A slope of zero repeats one shell, so the union is that shell.
        return true;
    }
    let b = max as i128;
    let a = min as i128;
    // Least multiple of `r` strictly above `b`.
    let up = (b.div_euclid(r) + 1) * r;
    // Greatest multiple of `r` strictly below `a`.
    let down = {
        let q = a.div_euclid(r);
        if q * r == a {
            (q - 1) * r
        } else {
            q * r
        }
    };
    is_power_of(up - b, r) && is_power_of(a - down, r)
}

/// Mutant one: the top join only.
fn tiles_top_only(radix: i128, slope: u32, magnitudes: u32, _min: i64, max: i64) -> bool {
    if magnitudes <= 1 {
        return true;
    }
    let r = radix.pow(slope);
    if r <= 1 {
        return true;
    }
    let b = max as i128;
    let up = (b.div_euclid(r) + 1) * r;
    is_power_of(up - b, r)
}

/// Mutant two: admits everything.
fn tiles_always(_: i128, _: u32, _: u32, _: i64, _: i64) -> bool {
    true
}

fn main() {
    let mut clean = 0usize;
    let mut ragged = 0usize;
    let mut derived_wrong = 0usize;
    let mut top_only_wrong = 0usize;
    let mut always_wrong = 0usize;
    let mut swept = 0usize;
    let mut first_derived_witness = None;
    let mut first_top_only_witness = None;

    for radix in [2i128, 3, 5, 10] {
        for slope in 0u32..=4 {
            for magnitudes in 1u32..=5 {
                for min in -12i64..=0 {
                    for max in 0i64..=12 {
                        if min == 0 && max == 0 {
                            continue;
                        }
                        let set = union_set(radix, slope, magnitudes, min, max);
                        if set.len() < 3 {
                            continue;
                        }
                        swept += 1;
                        let r = radix.pow(slope);
                        let oracle = is_clean_ladder(&set, r);
                        if oracle {
                            clean += 1
                        } else {
                            ragged += 1
                        }

                        if shells_tile(radix, slope, magnitudes, min, max) != oracle {
                            derived_wrong += 1;
                            if first_derived_witness.is_none() {
                                first_derived_witness =
                                    Some((radix, slope, magnitudes, min, max, oracle, gaps(&set)));
                            }
                        }
                        if tiles_top_only(radix, slope, magnitudes, min, max) != oracle {
                            top_only_wrong += 1;
                            if first_top_only_witness.is_none() {
                                first_top_only_witness =
                                    Some((radix, slope, magnitudes, min, max, oracle, gaps(&set)));
                            }
                        }
                        if tiles_always(radix, slope, magnitudes, min, max) != oracle {
                            always_wrong += 1;
                        }
                    }
                }
            }
        }
    }

    println!("== the tiling obligation, swept ==\n");
    println!("   {swept} coordinate tuples");
    println!("   the oracle calls {clean} clean and {ragged} ragged");
    println!(
        "   ragged is {:.1} per cent of the swept space",
        100.0 * ragged as f64 / swept as f64
    );

    println!("\n   controls:");
    println!("     both verdicts present: {}", clean > 0 && ragged > 0);
    println!("     mutant `always true` caught on {always_wrong} tuples");
    println!("     mutant `top join only` caught on {top_only_wrong} tuples");
    if clean == 0 || ragged == 0 {
        println!("\n   CONTROL FAILED: the oracle answers one way.");
        std::process::exit(2);
    }
    if always_wrong == 0 || top_only_wrong == 0 {
        println!("\n   CONTROL FAILED: a mutant was never caught, so the sweep does not reach");
        println!("   the region the condition is about.");
        std::process::exit(2);
    }
    if let Some(w) = first_top_only_witness {
        println!("     first tuple the top-only mutant gets wrong: {w:?}");
    }
    println!("   controls hold.");

    println!("\n   the derived obligation differs from the oracle on {derived_wrong} tuples");
    if derived_wrong == 0 {
        println!("\n   THE OBLIGATION HOLDS over the swept space, from the coordinates alone.");
        println!("   `Format` carries every input it needs: the ambient radix, the quantum's");
        println!("   slope and magnitude count, and the slot range's ends. Nothing in");
        println!("   `arvo-format` asks the question, so a `Format` impl denoting a ragged set");
        println!("   compiles and is admitted, where a `Slots` impl with an inverted range is");
        println!("   refused by `Slots::ADMITTED`. The two coordinates are checked at one tier");
        println!("   and unchecked at the tier above it.");
        std::process::exit(0);
    } else {
        println!("\n   THE OBLIGATION FAILS on {derived_wrong} tuples; it is not the condition.");
        if let Some(w) = first_derived_witness {
            println!("   first witness: {w:?}");
        }
        std::process::exit(1);
    }
}
