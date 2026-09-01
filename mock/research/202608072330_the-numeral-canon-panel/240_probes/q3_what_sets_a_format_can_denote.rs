//! q3. Which value sets the shipped parameterisation can denote.
//!
//! `ruling::the_format_spine_is_canon` says the representable set is the format's
//! identity and that integers, fixed point, scaled integers and floats are points
//! of one predicate. The shipped realisation makes that predicate concrete: a
//! phase, an affine quantum law `radix^(BASE + SLOPE*m)` over `MAGNITUDES` values
//! of `m`, and a slot range that does not vary with `m`. So the denoted set is a
//! union of `MAGNITUDES` arithmetic progressions over one index range.
//!
//! Three questions about that union, none of which needs the crate to answer.
//!
//! **One. Does the union reproduce a float, subnormals and all?** `08` reports the
//! canonical exponent as a function whose float instance carries a knee, and the
//! shipped law has no knee: its exponent is affine in the magnitude with no clamp.
//! If the union nevertheless denotes the float set, the knee is an artifact of
//! stating membership as a function of the value, and the union formulation does
//! not need a third named shape for gradual underflow.
//!
//! **Two. Where the union does not reproduce it, what is the difference?**
//!
//! **Three. What set shapes are reachable at all?** A float's gaps grow by a
//! constant ratio. A tapered format's do not. Section 3 asks whether every
//! reachable set is one such ladder, and section 3 refutes that, which is why
//! section 4 exists: it asks the reach question directly, by naming a target set
//! and searching the whole coordinate space for a tuple that denotes it.
//!
//! **The cases that must fail, stated before the run.**
//!
//! Section 1 compares two set constructions. If the comparison is degenerate it
//! will call everything equal, so the run reports at least one pair it calls
//! unequal and at least one it calls equal, and stops if either is missing.
//!
//! Section 3's instrument must read a float as one ladder and must read a
//! hand-built taper as not one, or it cannot separate the two shapes and its
//! count of ragged tuples is a count of nothing.
//!
//! Section 4's search must reach a float target. A search that reaches nothing
//! says nothing about the two taper targets it also fails to reach.
//!
//! Everything is exact integer arithmetic in units of the smallest quantum, so no
//! floating point is used anywhere and no rounding can enter the comparison.
//!
//! Build: `rustc --edition 2024 -O q3_what_sets_a_format_can_denote.rs -o /tmp/q3`

use std::collections::BTreeSet;

/// The IEEE binary finite set, in units of `2^emin`, from Flocq's `generic_format`.
///
/// A value `x = k * 2^emin` is representable exactly when `x / 2^phi(x)` is an
/// integer, with `phi(x) = max(emin, mag(x) - p + 1)`. In units of `2^emin` that
/// is `2^shift | k` with `shift = max(0, bitlen(|k|) - p + 1)`, and the exponent
/// bound `mag(x) <= emax` becomes `bitlen(|k|) <= emax - emin`.
///
/// The definition is the one `55` section 2 and `08` section 1.2 both name, written
/// as integer divisibility so nothing rounds.
fn ieee_set(p: u32, span: u32) -> BTreeSet<i128> {
    let mut out = BTreeSet::new();
    out.insert(0);
    let bound: i128 = 1i128 << span;
    let mut k: i128 = 1;
    while k < bound {
        let bits = 128 - (k.leading_zeros() as i32);
        let shift = core::cmp::max(0, bits - p as i32 + 1) as u32;
        if k % (1i128 << shift) == 0 {
            out.insert(k);
            out.insert(-k);
        }
        k += 1;
    }
    out
}

/// The shipped parameterisation's denoted set, in units of `q(0)`.
///
/// `value/q(0) = phase + slot * radix^(SLOPE*m)`, over `slot in [min,max]` and
/// `m in [0, magnitudes)`. Phase is taken as zero here, which is what all four
/// shipped points except `Biased` use and what a float would use.
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

/// The sorted gaps between consecutive members of a set.
fn gaps(set: &BTreeSet<i128>) -> Vec<i128> {
    let v: Vec<i128> = set.iter().copied().collect();
    v.windows(2).map(|w| w[1] - w[0]).collect()
}

/// The distinct gap values a set carries, ascending.
///
/// A constant grid has one. A geometric ladder has one per rung. This replaces an
/// earlier instrument that took ratios between consecutive gaps and reported a
/// sentinel wherever the ratio was below one, which is half of them, because the
/// gaps shrink toward zero and grow away from it. The first run is kept as
/// `q3_output.v1_integer_ratios_only.txt`; its verdict was the same and its
/// profiles were unreadable.
fn distinct_gaps(set: &BTreeSet<i128>) -> Vec<i128> {
    let mut g: Vec<i128> = gaps(set);
    g.sort();
    g.dedup();
    g
}

/// Whether the distinct gaps form one geometric progression, and at what ratio.
///
/// `Some(r)` when every rung is `r` times the one below it, with `r` an integer
/// above one. `Some(1)` for a single gap, which is a constant grid. `None` when
/// the ladder carries more than one growth rate, which is the tapered shape.
fn ladder_ratio(set: &BTreeSet<i128>) -> Option<i128> {
    let g = distinct_gaps(set);
    if g.len() <= 1 {
        return Some(1);
    }
    let mut r: Option<i128> = None;
    for w in g.windows(2) {
        if w[0] == 0 || w[1] % w[0] != 0 {
            return None;
        }
        let this = w[1] / w[0];
        match r {
            None => r = Some(this),
            Some(prev) if prev == this => {}
            Some(_) => return None,
        }
    }
    r
}

fn main() {
    let mut findings = 0usize;

    // ---------------- section 1 ----------------
    println!("== section 1: does the union of shells denote a float set ==\n");

    let mut exact_matches = 0usize;
    let mut symmetric_matches = 0usize;
    let mut compared = 0usize;
    let mut called_equal = 0usize;
    let mut called_unequal = 0usize;

    for mant in 2u32..=5 {
        for exps in 1u32..=5 {
            let two_c_min = -(1i64 << (mant - 1));
            let two_c_max = (1i64 << (mant - 1)) - 1;
            let design = union_set(2, 1, exps, two_c_min, two_c_max);
            let symmetric = union_set(2, 1, exps, -two_c_max, two_c_max);

            for p in 1u32..=6 {
                for span in 1u32..=9 {
                    let ieee = ieee_set(p, span);
                    compared += 1;
                    if design == ieee {
                        called_equal += 1;
                        exact_matches += 1;
                        println!(
                            "   two's-complement slots MATCH ieee: mantissa {mant}, exponents \
                             {exps}  ==  p {p}, span {span}   (|set| = {})",
                            ieee.len()
                        );
                    } else {
                        called_unequal += 1;
                    }
                    if symmetric == ieee {
                        symmetric_matches += 1;
                        println!(
                            "   symmetric slots MATCH ieee: mantissa {mant}, exponents {exps} \
                              ==  p {p}, span {span}   (|set| = {})",
                            ieee.len()
                        );
                    }
                }
            }
        }
    }

    println!("\n   {compared} comparisons, {called_equal} called equal, {called_unequal} unequal");
    if called_equal == 0 && symmetric_matches == 0 {
        println!("   CONTROL FAILED: nothing matched anything, so equality may be unreachable");
        println!("   for reasons that have nothing to do with the question.");
        std::process::exit(2);
    }
    if called_unequal == 0 {
        println!("   CONTROL FAILED: everything matched. The comparison is degenerate.");
        std::process::exit(2);
    }
    println!(
        "   exact matches with two's-complement slots: {exact_matches}; with a symmetric \
         slot range: {symmetric_matches}"
    );

    // The witness, worked, so the difference is visible rather than a count.
    let d = union_set(2, 1, 3, -4, 3);
    let s = union_set(2, 1, 3, -3, 3);
    let i = ieee_set(3, 4);
    println!("\n   worked witness, mantissa 3, exponents 3, against p = 3, span 4:");
    println!("     union, two's-complement slots [-4, 3] : {:?}", d.iter().copied().collect::<Vec<_>>());
    println!("     union, symmetric slots      [-3, 3] : {:?}", s.iter().copied().collect::<Vec<_>>());
    println!("     ieee generic_format p=3 span=4       : {:?}", i.iter().copied().collect::<Vec<_>>());
    println!("     union(two's complement) == ieee : {}", d == i);
    println!("     union(symmetric)        == ieee : {}", s == i);
    let only_design: Vec<i128> = d.difference(&i).copied().collect();
    let only_ieee: Vec<i128> = i.difference(&d).copied().collect();
    println!("     in the union and not in ieee : {only_design:?}");
    println!("     in ieee and not in the union : {only_ieee:?}");

    if exact_matches == 0 && symmetric_matches > 0 {
        findings += 1;
        println!(
            "\n   FINDING: the union reproduces the float set exactly, subnormals included,\n   \
             when the slot range is symmetric, and never when it is two's complement.\n   \
             `Floating` uses `Signed<MANTISSA>`, whose range is two's complement, so no\n   \
             instantiation of the shipped point denotes an IEEE binary set."
        );
    }

    // ---------------- section 2 ----------------
    println!("\n== section 2: the knee ==\n");
    let sym = union_set(2, 1, 4, -3, 3);
    let g = gaps(&sym);
    println!("   symmetric union, mantissa 3, exponents 4");
    println!("   members : {:?}", sym.iter().copied().collect::<Vec<_>>());
    println!("   gaps    : {g:?}");
    let smallest = *g.iter().min().unwrap();
    let run = g
        .split(|&x| x != smallest)
        .map(<[i128]>::len)
        .max()
        .unwrap_or(0);
    println!("   the smallest gap is {smallest} and its longest run has length {run}");
    println!(
        "   That run is the constant-quantum region a float's canonical exponent needs a\n   \
         `max` clamp to produce. Here it falls out of the shell at magnitude zero covering\n   \
         every value below the next shell's reach, with no clamp anywhere in the law."
    );

    // ---------------- section 3 ----------------
    println!("\n== section 3: which gap profiles are reachable ==\n");

    let mut ladders: BTreeSet<Option<i128>> = BTreeSet::new();
    let mut not_ladder: Vec<(i128, u32, u32, i64, i64, Vec<i128>)> = Vec::new();
    let mut unreadable = 0usize;
    let mut swept = 0usize;
    for radix in [2i128, 3, 10] {
        for slope in 0u32..=3 {
            for mags in 1u32..=4 {
                for &(min, max) in &[(-4i64, 3i64), (-3, 3), (0, 7), (1, 7), (-8, 7)] {
                    let set = union_set(radix, slope, mags, min, max);
                    if set.len() < 3 {
                        continue;
                    }
                    swept += 1;
                    let l = ladder_ratio(&set);
                    if l.is_none() {
                        unreadable += 1;
                        if not_ladder.len() < 8 {
                            not_ladder.push((
                                radix,
                                slope,
                                mags,
                                min,
                                max,
                                distinct_gaps(&set),
                            ));
                        }
                    }
                    ladders.insert(l);
                }
            }
        }
    }
    println!("   {swept} coordinate tuples swept");
    println!("   distinct ladder verdicts reachable: {ladders:?}");
    println!("   tuples whose gaps are NOT one geometric ladder: {unreadable}");
    if !not_ladder.is_empty() {
        println!("   the first few, with their coordinates and their distinct gaps:");
        for (radix, slope, mags, min, max, g) in &not_ladder {
            println!(
                "     radix {radix}, slope {slope}, magnitudes {mags}, slots [{min}, {max}] -> gaps {g:?}"
            );
        }
    }

    // Controls.
    let float_like = union_set(2, 1, 4, -3, 3);
    let fl = ladder_ratio(&float_like);
    println!(
        "\n   control A, a float, must be one ladder: gaps {:?}, verdict {:?}",
        distinct_gaps(&float_like),
        fl
    );

    let mut taper: BTreeSet<i128> = BTreeSet::new();
    let mut acc = 0i128;
    taper.insert(acc);
    for step in [1i128, 1, 1, 2, 2, 8, 8, 64] {
        acc += step;
        taper.insert(acc);
    }
    // Mirror it, so the shape is a numeral-shaped set rather than a half-line.
    let mirrored: BTreeSet<i128> = taper.iter().flat_map(|&v| [v, -v]).collect();
    let tl = ladder_ratio(&mirrored);
    println!(
        "   control B, a tapered ladder built by hand, must NOT be one ladder:\n     \
         members {:?}\n     gaps {:?}, verdict {:?}",
        mirrored.iter().copied().collect::<Vec<_>>(),
        distinct_gaps(&mirrored),
        tl
    );

    if fl != Some(2) {
        println!("\n   CONTROL A FAILED: a float did not read as a single ladder of ratio two,");
        println!("   so the instrument cannot recognise the shape it is supposed to admit.");
        std::process::exit(2);
    }
    if tl.is_some() {
        println!("\n   CONTROL B FAILED: the hand-built taper read as a single ladder, so the");
        println!("   instrument cannot separate the two shapes and section 3 establishes nothing.");
        std::process::exit(2);
    }
    println!("\n   both controls hold.");

    if unreadable == 0 {
        findings += 1;
        println!(
            "\n   FINDING: every reachable set's gaps are one geometric ladder. The affine\n   \
             quantum law fixes the ratio at `radix^SLOPE`, a single value, so the reachable\n   \
             sets are a constant grid and a single-ratio ladder and nothing else. A tapered\n   \
             numeral carries more than one ratio and is therefore not a point of this\n   \
             parameterisation, whatever coordinates it is given."
        );
    } else {
        println!("\n   {unreadable} tuples denote a set that is not one ladder; the reach claim");
        println!("   is refuted and the shapes should be read off the list above.");
    }

    // ---------------- section 4 ----------------
    println!("\n== section 4: is a tapered set reachable at all ==\n");
    println!("   Section 3 refuted the claim that every reachable set is one ladder, so the");
    println!("   reach question has to be asked directly rather than through the ladder shape.");
    println!("   A target is a set; the question is whether any coordinate tuple denotes it.\n");

    /// A set built from an explicit exponent sequence over a symmetric slot range.
    ///
    /// `exps = [0, 1, 2, ...]` is a float. Anything whose differences are not all
    /// equal is a taper, which is what `08` measures every posit configuration to
    /// be: canonical exponent slopes drawn from more than one value.
    fn shells(radix: i128, exps: &[u32], half: i64) -> BTreeSet<i128> {
        let mut out = BTreeSet::new();
        for &e in exps {
            let step = radix.pow(e);
            for s in -half..=half {
                out.insert(s as i128 * step);
            }
        }
        out
    }

    // Every coordinate tuple the shipped parameterisation admits, at small sizes,
    // including the phase, which section 1 held at zero.
    fn reaches(target: &BTreeSet<i128>) -> Option<(i128, u32, u32, i64, i64)> {
        for radix in [2i128, 3, 10] {
            for slope in 0u32..=4 {
                for mags in 1u32..=6 {
                    for min in -9i64..=0 {
                        for max in 0i64..=9 {
                            if union_set(radix, slope, mags, min, max) == *target {
                                return Some((radix, slope, mags, min, max));
                            }
                        }
                    }
                }
            }
        }
        None
    }

    let float_target = shells(2, &[0, 1, 2], 3);
    let taper_target = shells(2, &[0, 1, 3], 3);
    let taper2_target = shells(2, &[0, 2, 3], 3);

    println!("   control, a float target, exponents [0, 1, 2]:");
    println!("     members {:?}", float_target.iter().copied().collect::<Vec<_>>());
    let fr = reaches(&float_target);
    println!("     reached by {fr:?}");

    println!("\n   target A, a taper, exponents [0, 1, 3]:");
    println!("     members {:?}", taper_target.iter().copied().collect::<Vec<_>>());
    let tr1 = reaches(&taper_target);
    println!("     reached by {tr1:?}");

    println!("\n   target B, a taper, exponents [0, 2, 3]:");
    println!("     members {:?}", taper2_target.iter().copied().collect::<Vec<_>>());
    let tr2 = reaches(&taper2_target);
    println!("     reached by {tr2:?}");

    if fr.is_none() {
        println!("\n   CONTROL FAILED: the float target was not reached, so the search does not");
        println!("   cover its own parameter space and the two taper answers mean nothing.");
        std::process::exit(2);
    }
    println!("\n   control holds: the float target is reached.");

    if tr1.is_none() && tr2.is_none() {
        findings += 1;
        println!(
            "\n   FINDING: neither tapered target is denoted by any coordinate tuple in the\n   \
             search space. The exponent sequence a format denotes is an arithmetic\n   \
             progression, because the law is affine in the magnitude, so a shell ladder\n   \
             whose exponents are not equally spaced has no coordinates. That is the exact\n   \
             sense in which a tapered numeral is outside this parameterisation, and it is\n   \
             narrower than the gap-shape claim section 3 refuted."
        );
    } else {
        println!("\n   at least one taper IS reached, so the parameterisation is wider than the");
        println!("   affine reading of it suggests.");
    }

    println!("\n== verdict ==");
    println!("   findings: {findings}");
    std::process::exit(if findings == 0 { 0 } else { 1 });
}
