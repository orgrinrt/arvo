//! r1. Does the deferral theorem's conclusion survive a finite carrier?
//!
//! `172` 4.1 proves the theorem in a setting that begins "A chain of total steps
//! over **exact values**". In that setting the fully deferred placement always
//! exists, and the proof is correct and I sign it.
//!
//! `173` clause 5 and L12 both state the conclusion WITHOUT that hypothesis:
//! "deferring every interior resolution to the boundary is pointwise optimal".
//! Read as canon by an implementer, that sentence says defer. Under I14 sizes
//! are const and containers are finite, so the fully deferred placement is
//! frequently NOT REALISABLE, which is the whole reason `60`'s window exists
//! (candidate clause 7) and the whole content of the sharing band (clause 9).
//!
//! So the question this probe asks is the one the candidate's own clause 7 and
//! clause 9 presuppose an answer to and clause 5 does not state: **when full
//! deferral is unrealisable, is there a pointwise optimum among the placements
//! that ARE realisable?**
//!
//! A placement is realisable at carrier `C` when no intermediate it produces
//! leaves `C` on any input. Resolution brings a value back into the declared
//! range, so resolving more makes a placement more likely realisable; the
//! deferred one is the least likely.
//!
//! THE CASES THAT MUST BEHAVE A PARTICULAR WAY:
//!   C1. At a carrier wide enough for full deferral, full deferral must BE the
//!       pointwise optimum, on every chain. If not, this instrument disagrees
//!       with a proved theorem and nothing it reports is worth reading.
//!   C2. The realisability test must exclude at least one placement at some
//!       carrier, or "realisable" is doing no work and the question is empty.
//!   C3. There must exist a chain and a carrier at which NO realisable
//!       placement is pointwise optimal. If none is found, the amendment this
//!       probe is testing for is cosmetic and it says so.
//!
//! Run: rustc -O r1_finiteness_and_the_deferral_theorem.rs -o /tmp/r1 && /tmp/r1

const W: u32 = 8;
const DOMAIN: u128 = 1 << W;
const LIMIT: u128 = (1 << W) - 1;

#[derive(Clone, Copy, Debug)]
enum Step {
    AddK(u128),
    MulK(u128),
    ShrK(u32),
    XorK(u128),
}

fn apply(s: Step, v: u128) -> u128 {
    match s {
        Step::AddK(k) => v + k,
        Step::MulK(k) => v * k,
        Step::ShrK(g) => v >> g,
        Step::XorK(k) => v ^ k,
    }
}

/// The boundary resolution: nearest-point selection onto [0, 2^W).
fn pi(v: u128) -> u128 {
    if v > LIMIT { LIMIT } else { v }
}

fn exact(x: u128, steps: &[Step]) -> u128 {
    let mut v = x;
    for &s in steps {
        v = apply(s, v);
    }
    v
}

/// Run a placement. Returns None when some intermediate leaves the carrier,
/// which is what "not realisable at this carrier" means.
fn run(x: u128, steps: &[Step], mask: u32, carrier: u32) -> Option<u128> {
    let cap: u128 = if carrier >= 127 { u128::MAX } else { (1u128 << carrier) - 1 };
    let mut v = x;
    for (i, &s) in steps.iter().enumerate() {
        v = apply(s, v);
        if v > cap {
            return None;
        }
        if i + 1 < steps.len() && (mask >> i) & 1 == 1 {
            v = pi(v);
        }
    }
    Some(pi(v))
}

fn realisable(steps: &[Step], mask: u32, carrier: u32) -> bool {
    (0..DOMAIN).all(|x| run(x, steps, mask, carrier).is_some())
}

fn dist(a: u128, b: u128) -> u128 {
    if a > b { a - b } else { b - a }
}

/// Is `m` pointwise no worse than every other realisable placement?
fn is_pointwise_optimal(steps: &[Step], m: u32, carrier: u32, realis: &[u32]) -> bool {
    for &other in realis {
        if other == m {
            continue;
        }
        for x in 0..DOMAIN {
            let a = run(x, steps, m, carrier).unwrap();
            let b = run(x, steps, other, carrier).unwrap();
            let e = exact(x, steps);
            if dist(a, e) > dist(b, e) {
                return false;
            }
        }
    }
    true
}

fn chains() -> Vec<Vec<Step>> {
    let alphabet = [
        Step::AddK(97),
        Step::MulK(3),
        Step::ShrK(2),
        Step::XorK(0b1011_0110),
        Step::MulK(5),
        Step::ShrK(1),
        Step::AddK(13),
        Step::MulK(7),
    ];
    let mut rng: u64 = 0x5EED_1234_ABCD_9876;
    let mut next = || {
        rng ^= rng << 13;
        rng ^= rng >> 7;
        rng ^= rng << 17;
        rng
    };
    let mut out = Vec::new();
    for _ in 0..600 {
        let d = 3 + (next() % 3) as usize;
        out.push((0..d).map(|_| alphabet[(next() % 8) as usize]).collect());
    }
    out
}

fn main() {
    let cs = chains();
    println!("W = {W}, S = [0,{LIMIT}], pi = nearest-point selection (clamp)");
    println!("{} chains of depth 3..=5, inputs exhaustive over 0..{DOMAIN}", cs.len());
    println!();

    // ---- C1: at a carrier where full deferral is realisable, it must win ----
    let wide = 120u32;
    let mut c1_checked = 0usize;
    for c in cs.iter() {
        let interior = c.len() - 1;
        let all: Vec<u32> = (0..(1u32 << interior))
            .filter(|&m| realisable(c, m, wide))
            .collect();
        assert!(all.contains(&0), "full deferral not realisable at a 120-bit carrier");
        assert!(
            is_pointwise_optimal(c, 0, wide, &all),
            "full deferral is not the pointwise optimum at a wide carrier, so this \
             instrument disagrees with the theorem 172 4.1 proves"
        );
        c1_checked += 1;
    }
    println!("C1 full deferral is the pointwise optimum on all {c1_checked} chains at a 120-bit carrier : true");

    // ---- the real sweep -------------------------------------------------
    let mut excluded_somewhere = false;
    let mut no_optimum: Vec<(usize, u32, usize)> = Vec::new();
    let mut defer_unrealisable = 0usize;
    let mut had_optimum_but_not_defer = 0usize;

    for (ci, c) in cs.iter().enumerate() {
        let interior = c.len() - 1;
        let top = 1u32 << interior;
        for carrier in 8..=24u32 {
            let realis: Vec<u32> = (0..top).filter(|&m| realisable(c, m, carrier)).collect();
            if realis.is_empty() {
                continue;
            }
            if realis.len() < top as usize {
                excluded_somewhere = true;
            }
            let defer_ok = realis.contains(&0);
            if !defer_ok {
                defer_unrealisable += 1;
                let opt: Vec<u32> = realis
                    .iter()
                    .copied()
                    .filter(|&m| is_pointwise_optimal(c, m, carrier, &realis))
                    .collect();
                if opt.is_empty() {
                    no_optimum.push((ci, carrier, realis.len()));
                } else {
                    had_optimum_but_not_defer += 1;
                }
            }
        }
    }

    println!("C2 the realisability test excludes some placement somewhere            : {excluded_somewhere}");
    assert!(excluded_somewhere, "realisable is doing no work, so the question is empty");
    println!();
    println!("cells where full deferral is NOT realisable          : {defer_unrealisable}");
    println!("  of those, some realisable placement IS the optimum : {had_optimum_but_not_defer}");
    println!("  of those, NO realisable placement is the optimum   : {}", no_optimum.len());
    println!();

    // ---- the refutation sent me here: if an optimum always exists when full
    // deferral is unrealisable, WHICH placement is it? The useful answer for a
    // canon is an executable rule, and the obvious candidate is "resolve as
    // little as the carrier allows", i.e. a subset-minimal realisable
    // placement. Tested rather than assumed, with the case that must fail
    // stated: if the optimum is ever NOT subset-minimal, the rule is wrong.
    let mut opt_is_minimal = 0usize;
    let mut opt_not_minimal = 0usize;
    let mut opt_unique = 0usize;
    let mut opt_multiple = 0usize;
    let mut minimal_count_gt1 = 0usize;
    for (ci, c) in cs.iter().enumerate() {
        let _ = ci;
        let interior = c.len() - 1;
        let top = 1u32 << interior;
        for carrier in 8..=24u32 {
            let realis: Vec<u32> = (0..top).filter(|&m| realisable(c, m, carrier)).collect();
            if realis.is_empty() || realis.contains(&0) {
                continue;
            }
            let minimal: Vec<u32> = realis
                .iter()
                .copied()
                .filter(|&m| !realis.iter().any(|&o| o != m && (o & m) == o))
                .collect();
            if minimal.len() > 1 {
                minimal_count_gt1 += 1;
            }
            let opt: Vec<u32> = realis
                .iter()
                .copied()
                .filter(|&m| is_pointwise_optimal(c, m, carrier, &realis))
                .collect();
            if opt.len() == 1 { opt_unique += 1; } else if opt.len() > 1 { opt_multiple += 1; }
            for &o in opt.iter() {
                if minimal.contains(&o) { opt_is_minimal += 1; } else { opt_not_minimal += 1; }
            }
        }
    }
    println!("Given that an optimum exists, WHICH placement is it?");
    println!("  cells whose realisable set has >1 subset-minimal element : {minimal_count_gt1}");
    println!("  cells with a unique optimum                              : {opt_unique}");
    println!("  cells with several optima                                : {opt_multiple}");
    println!("  optima that ARE subset-minimal                           : {opt_is_minimal}");
    println!("  optima that are NOT subset-minimal                       : {opt_not_minimal}");
    let rule_holds = opt_not_minimal == 0;
    println!("  C4 every optimum is a subset-minimal realisable placement : {rule_holds}");
    println!();

    // ---- and the question that decides what the amendment should say: does
    // the finite-carrier optimum ATTAIN the theorem's value, pi(exact)? If it
    // does, clause 5 promises something still achievable and only its named
    // placement is unavailable. If it does not, the theorem states a lower
    // bound that finiteness does not reach, which is a different sentence.
    let mut attains = 0usize;
    let mut falls_short = 0usize;
    let mut worst_shortfall = 0u128;
    let mut shortfall_witness: Option<(usize, u32, u128, u128, u128)> = None;
    for (ci, c) in cs.iter().enumerate() {
        let interior = c.len() - 1;
        let top = 1u32 << interior;
        for carrier in 8..=24u32 {
            let realis: Vec<u32> = (0..top).filter(|&m| realisable(c, m, carrier)).collect();
            if realis.is_empty() || realis.contains(&0) {
                continue;
            }
            let opt = match realis
                .iter()
                .copied()
                .find(|&m| is_pointwise_optimal(c, m, carrier, &realis))
            {
                Some(m) => m,
                None => continue,
            };
            let mut short = false;
            for x in 0..DOMAIN {
                let e = exact(x, c);
                let got = run(x, c, opt, carrier).unwrap();
                let ideal = pi(e);
                if got != ideal {
                    short = true;
                    let d = dist(got, e) - dist(ideal, e);
                    if d > worst_shortfall {
                        worst_shortfall = d;
                        shortfall_witness = Some((ci, carrier, x, got, ideal));
                    }
                }
            }
            if short { falls_short += 1; } else { attains += 1; }
        }
    }
    println!("Does the finite-carrier optimum attain the theorem's value pi(exact)?");
    println!("  cells where it attains it        : {attains}");
    println!("  cells where it falls short of it : {falls_short}");
    println!("  worst shortfall in absolute error: {worst_shortfall}");
    if let Some((ci, carrier, x, got, ideal)) = shortfall_witness {
        println!("  witness: chain {ci} at carrier {carrier}, x={x}: optimum gives {got}, pi(exact) is {ideal}");
    }
    let c5 = falls_short > 0;
    println!("  C5 the theorem's value is sometimes unattainable under a finite carrier : {c5}");
    println!();

    let c3 = !no_optimum.is_empty();
    println!("C3 a chain and carrier exist where no realisable placement is optimal  : {c3}");
    if let Some(&(ci, carrier, n)) = no_optimum.first() {
        println!();
        println!("witness: chain {ci} = {:?}", cs[ci]);
        println!("         carrier {carrier} bits, {n} realisable placements, none pointwise optimal");
        let c = &cs[ci];
        let top = 1u32 << (c.len() - 1);
        let realis: Vec<u32> = (0..top).filter(|&m| realisable(c, m, carrier)).collect();
        // exhibit the incomparability explicitly: two placements each strictly
        // better than the other at some input.
        'outer: for &a in realis.iter() {
            for &b in realis.iter() {
                if a >= b {
                    continue;
                }
                let mut a_wins = None;
                let mut b_wins = None;
                for x in 0..DOMAIN {
                    let e = exact(x, c);
                    let da = dist(run(x, c, a, carrier).unwrap(), e);
                    let db = dist(run(x, c, b, carrier).unwrap(), e);
                    if da < db && a_wins.is_none() {
                        a_wins = Some((x, da, db));
                    }
                    if db < da && b_wins.is_none() {
                        b_wins = Some((x, db, da));
                    }
                }
                if let (Some(aw), Some(bw)) = (a_wins, b_wins) {
                    println!("         placements {a:#06b} and {b:#06b} are incomparable:");
                    println!("           at x={} placement {a:#06b} is closer ({} vs {})", aw.0, aw.1, aw.2);
                    println!("           at x={} placement {b:#06b} is closer ({} vs {})", bw.0, bw.1, bw.2);
                    break 'outer;
                }
            }
        }
    }
    println!();
    if c3 {
        println!("RESULT: the theorem's conclusion does NOT survive finiteness. Where the");
        println!("carrier cannot hold the exact intermediate, full deferral is unavailable and");
        println!("the remaining placements can be pairwise incomparable, so no pointwise");
        println!("optimum exists and clause 5 selects nothing. That is exactly the region");
        println!("clause 7's window and clause 9's band govern, and clause 5 should say so.");
    } else {
        println!("RESULT: my hypothesis is REFUTED. Every cell where full deferral was");
        println!("unrealisable still had a pointwise optimum among the realisable placements,");
        println!("so finiteness does not destroy the conclusion; it only makes the named");
        println!("placement unavailable. The executable form of clause 5 under a finite");
        println!("carrier is therefore not 'defer everything' but 'resolve as little as the");
        println!("carrier allows', and C4 reports whether that rule is exactly right.");
    }
}
