//! The narrowing, and the three laws that decide whether it is one relation or a pile.
//!
//! The narrowing conversion from numeral A to numeral B is defined as the design's own
//! quantiser applied to the exact value, with the operation set to the identity. The
//! target strategy supplies the five resolutions, taken from the ratified fixed-point
//! preset table at `110:2705-2712`:
//!
//!   Hot      in-range TowardNegative,  out-of-range ReduceModulo
//!   Warm     in-range ToEven,          out-of-range clamp
//!   Cold     in-range ToEven,          out-of-range clamp
//!   Precise  in-range ToEven,          out-of-range Refuse
//!
//! Laws checked, exhaustively over the whole shape matrix at the checked bound and over
//! every value of the source numeral, for all four strategies:
//!
//!   C1  quantise from A to A is the identity.
//!   C2  where A embeds into B, quantise agrees with embed. (The lossy map restricted
//!       to the exact region IS the exact map, so there is one relation, not two.)
//!   C3  embed then quantise equals quantise directly. (Coherence: an exact step before
//!       a lossy one changes nothing, so no consumer can get a different answer by
//!       routing through a wider intermediate.)
//!   C5  quantise is monotone. Expected to hold for Warm, Cold and Precise, whose
//!       resolutions are rounding and clamping and refusal, and to FAIL for Hot, whose
//!       ReduceModulo is not order-preserving.
//!   C4  quantise then quantise does NOT equal quantise directly, in general. This is
//!       expected to FAIL and the count of counterexamples is the artifact: it is double
//!       rounding, `110:1318-1323`, and it is why embed must stay a named relation
//!       distinct from quantise even though quantise subsumes it pointwise.
//!
//! rustc 1.98.0-nightly (57d06900f 2026-05-27), plain std probe.

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Strat {
    Hot,
    Warm,
    Cold,
    Precise,
}

const STRATS: [Strat; 4] = [Strat::Hot, Strat::Warm, Strat::Cold, Strat::Precise];

/// A numeral: I integer digits, F fraction digits, unsigned, zero bias, radix two.
/// Value set is { k / 2^F : 0 <= k < 2^(I+F) }.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct Num {
    i: u32,
    f: u32,
}

impl Num {
    fn count(&self) -> i128 {
        1i128 << (self.i + self.f)
    }
    fn embeds_into(&self, o: &Num) -> bool {
        self.i <= o.i && self.f <= o.f
    }
}

/// The exact value of raw `k` in numeral `n`, as a rational with denominator 2^FMAX.
const FMAX: u32 = 10;
fn exact(n: &Num, k: i128) -> i128 {
    k << (FMAX - n.f)
}

/// The result of a narrowing.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Out {
    Raw(i128),
    Refused,
}

/// The quantiser: an exact value (scaled by 2^FMAX) onto numeral `n`'s representable
/// set, resolved by strategy `s`. This is the whole narrowing design.
fn quantise(v_scaled: i128, n: &Num, s: Strat) -> Out {
    let step = 1i128 << (FMAX - n.f); // the target's quantum, scaled
    let hi = n.count() - 1; // the largest representable index

    // Step one: land on the target grid, using the in-range direction.
    let q = match s {
        // TowardNegative: floor division, which an arithmetic right shift gives free.
        Strat::Hot => v_scaled.div_euclid(step),
        // ToEven: round to nearest, ties to even.
        Strat::Warm | Strat::Cold | Strat::Precise => {
            let d = v_scaled.div_euclid(step);
            let r = v_scaled.rem_euclid(step);
            let twice = 2 * r;
            if twice > step || (twice == step && d % 2 != 0) {
                d + 1
            } else {
                d
            }
        }
    };

    // Step two: classify against the range, using the out-of-range resolution.
    if q >= 0 && q <= hi {
        return Out::Raw(q);
    }
    match s {
        Strat::Hot => Out::Raw(q.rem_euclid(n.count())), // ReduceModulo
        Strat::Warm | Strat::Cold => Out::Raw(q.clamp(0, hi)), // clamp
        Strat::Precise => Out::Refused,                  // Refuse
    }
}

fn convert(from: &Num, k: i128, to: &Num, s: Strat) -> Out {
    quantise(exact(from, k), to, s)
}

/// The exact embedding, defined only where the order holds.
fn embed(from: &Num, k: i128, to: &Num) -> i128 {
    debug_assert!(from.embeds_into(to));
    k << (to.f - from.f)
}

const LIM: u32 = 6;

fn shapes() -> Vec<Num> {
    let mut v = Vec::new();
    for i in 0..=LIM {
        for f in 0..=(LIM - i) {
            v.push(Num { i, f });
        }
    }
    v
}

fn main() {
    let sh = shapes();
    println!("shapes {} strategies {}", sh.len(), STRATS.len());

    // C1: quantise A to A is the identity.
    let mut c1_checked = 0u64;
    let mut c1_fail = 0u64;
    for a in &sh {
        for &s in &STRATS {
            for k in 0..a.count() {
                c1_checked += 1;
                if convert(a, k, a, s) != Out::Raw(k) {
                    c1_fail += 1;
                    if c1_fail < 4 {
                        println!("C1 FAIL {a:?} {s:?} k={k} got {:?}", convert(a, k, a, s));
                    }
                }
            }
        }
    }
    println!("C1 checked {c1_checked} failures {c1_fail}");

    // C2: where A embeds into B, quantise agrees with embed.
    let mut c2_checked = 0u64;
    let mut c2_fail = 0u64;
    for a in &sh {
        for b in &sh {
            if !a.embeds_into(b) {
                continue;
            }
            for &s in &STRATS {
                for k in 0..a.count() {
                    c2_checked += 1;
                    let want = Out::Raw(embed(a, k, b));
                    let got = convert(a, k, b, s);
                    if got != want {
                        c2_fail += 1;
                        if c2_fail < 4 {
                            println!("C2 FAIL {a:?}->{b:?} {s:?} k={k} want {want:?} got {got:?}");
                        }
                    }
                }
            }
        }
    }
    println!("C2 checked {c2_checked} failures {c2_fail}");

    // C3: embed then quantise equals quantise directly.
    let mut c3_checked = 0u64;
    let mut c3_fail = 0u64;
    for a in &sh {
        for b in &sh {
            if !a.embeds_into(b) {
                continue;
            }
            for c in &sh {
                for &s in &STRATS {
                    for k in 0..a.count() {
                        c3_checked += 1;
                        let direct = convert(a, k, c, s);
                        let routed = convert(b, embed(a, k, b), c, s);
                        if direct != routed {
                            c3_fail += 1;
                            if c3_fail < 4 {
                                println!(
                                    "C3 FAIL {a:?}->{b:?}->{c:?} {s:?} k={k} direct {direct:?} routed {routed:?}"
                                );
                            }
                        }
                    }
                }
            }
        }
    }
    println!("C3 checked {c3_checked} failures {c3_fail}");

    // C4: quantise then quantise, expected to disagree. Counterexamples are the point.
    let mut c4_checked = 0u64;
    let mut c4_diff = 0u64;
    let mut per_strat = [0u64; 4];
    let mut first_per: [Option<String>; 4] = [None, None, None, None];
    let mut first: Option<String> = None;
    for a in &sh {
        for b in &sh {
            for c in &sh {
                for &s in &STRATS {
                    for k in 0..a.count() {
                        c4_checked += 1;
                        let direct = convert(a, k, c, s);
                        let routed = match convert(a, k, b, s) {
                            Out::Raw(m) => convert(b, m, c, s),
                            Out::Refused => Out::Refused,
                        };
                        if direct != routed {
                            c4_diff += 1;
                            let si = STRATS.iter().position(|x| *x == s).unwrap();
                            per_strat[si] += 1;
                            if first_per[si].is_none() && b.f > 0 && c.f > 0 && a.f > b.f {
                                first_per[si] = Some(format!(
                                    "{a:?} -> {b:?} -> {c:?} under {s:?}, k={k}: direct {direct:?}, routed {routed:?}"
                                ));
                            }
                            if first.is_none() {
                                first = Some(format!(
                                    "{a:?} -> {b:?} -> {c:?} under {s:?}, k={k}: direct {direct:?}, routed {routed:?}"
                                ));
                            }
                        }
                    }
                }
            }
        }
    }
    // C5: is quantise monotone? Rounding and clamping are; ReduceModulo is not.
    let mut c5_checked = 0u64;
    let mut c5_fail = [0u64; 4];
    for a in &sh {
        for b in &sh {
            for (si, &s) in STRATS.iter().enumerate() {
                for k in 0..a.count() {
                    for l in k..a.count() {
                        c5_checked += 1;
                        let x = convert(a, k, b, s);
                        let y = convert(a, l, b, s);
                        if let (Out::Raw(u), Out::Raw(v)) = (x, y) {
                            if u > v {
                                c5_fail[si] += 1;
                            }
                        }
                    }
                }
            }
        }
    }
    println!("C5 checked {c5_checked} monotonicity failures per strategy:");
    for (i, st) in STRATS.iter().enumerate() {
        println!("   {st:?} {}", c5_fail[i]);
    }

    println!("C4 checked {c4_checked} disagreements {c4_diff}");
    if let Some(f) = first {
        println!("C4 first counterexample: {f}");
    }
    for (i, st) in STRATS.iter().enumerate() {
        println!("C4 {st:?} disagreements {}", per_strat[i]);
        if let Some(f) = &first_per[i] {
            println!("   two lossy grid steps: {f}");
        }
    }
}
