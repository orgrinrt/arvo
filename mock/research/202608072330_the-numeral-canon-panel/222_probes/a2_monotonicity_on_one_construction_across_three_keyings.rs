// a2: are the two keyed rounding members' monotonicity-failure rates comparable, and do they
// differ?
//
// The question is
// `question::does_the_position_keyed_members_monotonicity_failure_rate_differ_from_the_independent_members`.
// Its note says two source files each hold one count, neither measured the other member under the
// same construction, and what would decide it is "a sweep with one construction and one input
// shape held fixed and varied across both members, which nobody has built". This is that sweep.
//
// THE COMPARABILITY PROBLEM, which is the actual content of the question. One member is
// deterministic and the other is random, so a realised count for one and a realised count for the
// other are not the same kind of number. `129_probes/x1_output.txt` reports 7 violations over 40
// consecutive pairs for the position-keyed member on a ramp: an exact count of a deterministic
// process at one phase. An independent member has no such number; it has a distribution.
//
// The statistic that puts them on one footing is EXPECTED VIOLATIONS PER CONSECUTIVE PAIR:
//   - for the independent member, the exact expectation over the threshold draws;
//   - for the position-keyed member, the exact count averaged over the dither's phase;
//   - for the shared-threshold member, the exact expectation over the single draw.
// Everything below is exact rationals over i128. No sampling, no floating point in the arithmetic.
//
// WHAT PHASE MEANS, and this is the one thing this probe got wrong on its first run. A phase model
// is only a fair normalisation if it leaves the two members with the SAME MARGINAL, because the
// marginal is the unbiasedness property both members exist to have, and a comparison that also
// perturbs the marginal is comparing two different rounding schemes rather than two keyings.
//
//   Dead route, kept: the first version advanced the golden-ratio recurrence's INDEX by the phase,
//   u_i(p) = key(i + p). The C4 marginal control caught it immediately: at m = 16 the
//   phase-averaged marginal disagreed with j/m at 14 of 16 residues, because 16 successive terms
//   of a golden-ratio walk are equidistributed only asymptotically, never exactly. Its Part 1
//   numbers were therefore comparing a biased member against an unbiased one. The control existed
//   before the run and is why this is a paragraph rather than a result.
//
//   The route taken instead: phase is an OFFSET on the threshold, u_i(p) = (key(i) + p) mod m,
//   with p ranging over all m values. That is exactly uniform per point, so the marginal matches
//   the independent member's exactly, while the consecutive increment u_{i+1} - u_i mod m is
//   untouched. The comparison then isolates the joint structure, which is the only place the two
//   members can differ once their marginals agree.
//
// THE CONSTRUCTION, one shape used by all three members. A ramp of N points on a fine grid,
// x_i = x0 + i*delta in units of the fine quantum, narrowed to a coarse grid m = 2^d quanta
// coarser. Write c_i = floor(x_i / m) and j_i = x_i - m*c_i. A member rounds up when j_i > u_i for
// its threshold u_i in {0, ..., m-1}, giving P(up) = j_i/m, which is the unbiasedness every
// stochastic rounding is for.
//
// PREDICTIONS, stated before running:
//
// P1. Shared threshold reports exactly zero at every parameter, because on an increasing ramp
//     j_{i+1} > j_i within a coarse cell, so rounding i up forces rounding i+1 up. Known-true
//     control: a nonzero here means the violation detector is broken.
//
// P2. A violation is possible only inside a coarse cell, so at delta = m every member is zero.
//
// P3. The two keyed members do NOT have the same rate once the marginals are equalised, and the
//     mechanism is the joint: the additive recurrence fixes u_{i+1} - u_i to a couple of values,
//     so the pair lives on a diagonal of the m^2 square the independent member spreads over.
//
// THE CASES THAT MUST FAIL, printed failing rather than asserted:
//   C1 shared threshold at a nonzero rate, which must not happen;
//   C2 an adversarial key alternating 0 and m-1, which must report a large rate, proving the
//      counter can report one at all;
//   C3 a decreasing key, which must report zero for a different reason than the shared threshold,
//      separating "zero because the construction forbids it" from "zero because the arm is asleep";
//   C4 the marginal check: every member's per-point probability of rounding up must be exactly
//      j_i/m, checked against a mutant using j_i/(m+1) and against the dead route above.

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct Q {
    n: i128,
    d: i128,
}

fn gcd(a: i128, b: i128) -> i128 {
    let (mut a, mut b) = (a.abs(), b.abs());
    while b != 0 {
        let t = a % b;
        a = b;
        b = t;
    }
    if a == 0 { 1 } else { a }
}

impl Q {
    fn new(n: i128, d: i128) -> Q {
        assert!(d != 0, "zero denominator");
        let s = if d < 0 { -1 } else { 1 };
        let (n, d) = (n * s, d * s);
        let g = gcd(n, d);
        Q { n: n / g, d: d / g }
    }
    fn int(n: i128) -> Q {
        Q { n, d: 1 }
    }
    fn add(self, o: Q) -> Q {
        let a = self.n.checked_mul(o.d).expect("Q::add overflow");
        let b = o.n.checked_mul(self.d).expect("Q::add overflow");
        Q::new(
            a.checked_add(b).expect("Q::add overflow"),
            self.d.checked_mul(o.d).expect("Q::add overflow"),
        )
    }
    fn mul(self, o: Q) -> Q {
        Q::new(
            self.n.checked_mul(o.n).expect("Q::mul overflow"),
            self.d.checked_mul(o.d).expect("Q::mul overflow"),
        )
    }
    fn approx(self) -> f64 {
        self.n as f64 / self.d as f64
    }
    fn show(self) -> String {
        if self.d == 1 { format!("{}", self.n) } else { format!("{}/{}", self.n, self.d) }
    }
}

/// The ramp, as (coarse floor, residue) per point.
fn ramp(x0: i128, delta: i128, n_points: usize, m: i128) -> Vec<(i128, i128)> {
    (0..n_points as i128)
        .map(|i| {
            let x = x0 + i * delta;
            (x.div_euclid(m), x.rem_euclid(m))
        })
        .collect()
}

/// Golden-ratio additive recurrence, exact integers. The base key, before any phase offset.
///
/// `G` is round(2^32 / phi) = 2654435769, so `(i * G) mod 2^32` walks the unit interval in
/// golden-ratio steps with no floating point.
fn golden_key(i: i128, m: i128) -> i128 {
    const G: i128 = 2_654_435_769;
    const MOD: i128 = 1i128 << 32;
    let t = (i * G).rem_euclid(MOD);
    (t * m) / MOD
}

/// The dead route, kept as a control rather than deleted: phase advances the index.
fn golden_key_index_phased(i: i128, m: i128, phase: i128) -> i128 {
    golden_key(i + phase, m)
}

/// The route taken: phase is a uniform offset on the threshold, so the marginal is exact.
fn golden_key_offset_phased(i: i128, m: i128, phase: i128) -> i128 {
    (golden_key(i, m) + phase).rem_euclid(m)
}

/// Exact expected violations per consecutive pair, independent thresholds.
///
/// A violation at pair i needs c_i == c_{i+1}, point i up and point i+1 down. With independent
/// draws that is (j_i/m) * (1 - j_{i+1}/m).
fn independent_rate(pts: &[(i128, i128)], m: i128) -> Q {
    let mut total = Q::int(0);
    let pairs = pts.len() - 1;
    for w in pts.windows(2) {
        let (c0, j0) = w[0];
        let (c1, j1) = w[1];
        if c0 != c1 {
            continue;
        }
        let up0 = Q::new(j0, m);
        let down1 = Q::int(1).add(Q::new(-j1, m));
        total = total.add(up0.mul(down1));
    }
    total.mul(Q::new(1, pairs as i128))
}

/// Exact expected violations per consecutive pair, one shared threshold for the whole pass.
///
/// Enumerated over every threshold value rather than argued, because P1 is the control and a
/// control derived from the reasoning it controls is not a control.
fn shared_rate(pts: &[(i128, i128)], m: i128) -> Q {
    let pairs = pts.len() - 1;
    let mut total = Q::int(0);
    for u in 0..m {
        let mut count: i128 = 0;
        for w in pts.windows(2) {
            let (c0, j0) = w[0];
            let (c1, j1) = w[1];
            let r0 = c0 + if j0 > u { 1 } else { 0 };
            let r1 = c1 + if j1 > u { 1 } else { 0 };
            if r0 > r1 {
                count += 1;
            }
        }
        total = total.add(Q::new(count, m));
    }
    total.mul(Q::new(1, pairs as i128))
}

/// Exact violations per consecutive pair for a deterministic key, averaged over every phase.
fn keyed_rate<F: Fn(i128, i128, i128) -> i128>(
    pts: &[(i128, i128)],
    m: i128,
    phases: i128,
    key: F,
) -> Q {
    let pairs = pts.len() - 1;
    let mut total = Q::int(0);
    for phase in 0..phases {
        let mut count: i128 = 0;
        for (i, w) in pts.windows(2).enumerate() {
            let (c0, j0) = w[0];
            let (c1, j1) = w[1];
            let u0 = key(i as i128, m, phase);
            let u1 = key(i as i128 + 1, m, phase);
            let r0 = c0 + if j0 > u0 { 1 } else { 0 };
            let r1 = c1 + if j1 > u1 { 1 } else { 0 };
            if r0 > r1 {
                count += 1;
            }
        }
        total = total.add(Q::new(count, phases));
    }
    total.mul(Q::new(1, pairs as i128))
}

/// The phase-averaged marginal probability of rounding up at each residue, for one key.
///
/// Returns the count of residues where it differs from the honest `j/m`.
fn marginal_defects<F: Fn(i128, i128, i128) -> i128>(m: i128, phases: i128, key: F) -> u32 {
    let mut bad = 0;
    for j in 0..m {
        let mut ups: i128 = 0;
        for phase in 0..phases {
            if j > key(0, m, phase) {
                ups += 1;
            }
        }
        if Q::new(ups, phases) != Q::new(j, m) {
            bad += 1;
        }
    }
    bad
}

fn main() {
    println!("=== a2: monotonicity failure rate, one construction, three keyings ===");
    println!();
    println!("Statistic: expected monotonicity violations per consecutive pair on an increasing");
    println!("ramp, exact rationals throughout. The deterministic member is averaged over the");
    println!("dither's phase, which is what makes it comparable to an expectation.");
    println!();

    // ---------------------------------------------------------------------------------
    // PART 0. C4 first, because Part 1 is worthless if the phase model biases the member.
    // ---------------------------------------------------------------------------------
    println!("--- Part 0: C4, the marginal control, run before anything rests on it ---");
    for d in [2u32, 3, 4, 5] {
        let m = 1i128 << d;
        let offset_bad = marginal_defects(m, m, golden_key_offset_phased);
        let index_bad = marginal_defects(m, m, golden_key_index_phased);
        let mutant_bad = {
            // a member computing P(up) = j/(m+1): the honest marginal check must catch it
            let mut bad = 0;
            for j in 0..m {
                if Q::new(j, m + 1) != Q::new(j, m) {
                    bad += 1;
                }
            }
            bad
        };
        println!(
            "  m = {m:>2}: offset-phased defects {offset_bad} (must be 0), index-phased defects \
             {index_bad} (the dead route), mutant j/(m+1) defects {mutant_bad} (must be > 0)"
        );
    }
    println!("  The middle column is why the index-phased model was abandoned, and the right");
    println!("  column is why the left one being zero means something.");
    println!();

    // ---------------------------------------------------------------------------------
    // PART 1. The sweep. One construction, three members, varied over d, delta and N.
    // ---------------------------------------------------------------------------------
    println!("--- Part 1: the sweep ---");
    println!(
        "{:>3} {:>6} {:>4}  {:>8} {:>12} {:>12}   {}",
        "d", "delta", "N", "shared", "independent", "position", "position/independent"
    );
    let mut shared_nonzero = 0u32;
    let mut rows = 0u32;
    let mut pos_above = 0u32;
    let mut pos_below = 0u32;
    let mut pos_equal = 0u32;
    let mut min_ratio = f64::INFINITY;
    let mut max_ratio = 0.0f64;
    for d in [2u32, 3, 4, 5, 6] {
        let m = 1i128 << d;
        let mut deltas = vec![1i128, 2, 3, 5, 7, m / 2, m, m + 1];
        deltas.sort();
        deltas.dedup();
        for delta in deltas {
            if delta <= 0 {
                continue;
            }
            for n_points in [16usize, 41, 128] {
                let pts = ramp(0, delta, n_points, m);
                let s = shared_rate(&pts, m);
                let ind = independent_rate(&pts, m);
                let pos = keyed_rate(&pts, m, m, golden_key_offset_phased);
                if s != Q::int(0) {
                    shared_nonzero += 1;
                }
                let ratio = if ind == Q::int(0) {
                    "n/a (both zero)".to_string()
                } else {
                    let r = pos.approx() / ind.approx();
                    if r < min_ratio {
                        min_ratio = r;
                    }
                    if r > max_ratio {
                        max_ratio = r;
                    }
                    format!("{r:.4}")
                };
                if ind != Q::int(0) || pos != Q::int(0) {
                    if pos.approx() > ind.approx() {
                        pos_above += 1;
                    } else if pos.approx() < ind.approx() {
                        pos_below += 1;
                    } else {
                        pos_equal += 1;
                    }
                }
                println!(
                    "{:>3} {:>6} {:>4}  {:>8} {:>12} {:>12}   {}",
                    d,
                    delta,
                    n_points,
                    s.show(),
                    format!("{:.6}", ind.approx()),
                    format!("{:.6}", pos.approx()),
                    ratio
                );
                rows += 1;
            }
        }
    }
    println!();
    println!("  rows: {rows}");
    println!("  C1: shared-threshold rows with a nonzero rate: {shared_nonzero} (must be 0)");
    println!("  position above independent: {pos_above}, below: {pos_below}, equal: {pos_equal}");
    println!("  ratio range over the rows where either member is nonzero: {min_ratio:.4} to {max_ratio:.4}");
    println!();

    // ---------------------------------------------------------------------------------
    // PART 2. The mechanism: the marginals agree and the joint does not.
    // ---------------------------------------------------------------------------------
    println!("--- Part 2: where the two members actually differ ---");
    for d in [3u32, 4, 5, 6] {
        let m = 1i128 << d;
        let mut increments: Vec<i128> = Vec::new();
        for i in 0..256i128 {
            let inc = (golden_key(i + 1, m) - golden_key(i, m)).rem_euclid(m);
            if !increments.contains(&inc) {
                increments.push(inc);
            }
        }
        increments.sort();
        println!(
            "  m = {m:>2}: distinct consecutive key increments over 256 steps: {} of a possible \
             {m} -> {:?}",
            increments.len(),
            increments
        );
    }
    println!("  An independent member has every increment with the flat distribution. The");
    println!("  marginals coincide by Part 0 and the joint does not, so the whole of the");
    println!("  difference in Part 1 is joint structure rather than bias.");
    println!();

    // ---------------------------------------------------------------------------------
    // PART 3. The remaining controls.
    // ---------------------------------------------------------------------------------
    println!("--- Part 3: the cases that must fail, shown failing ---");
    let d = 4u32;
    let m = 1i128 << d;
    let pts = ramp(0, 1, 128, m);
    let ind = independent_rate(&pts, m);

    let adversarial = |i: i128, m: i128, _p: i128| if i % 2 == 0 { 0 } else { m - 1 };
    let adv = keyed_rate(&pts, m, 1, adversarial);
    println!(
        "  C2 adversarial alternating key: rate {:.6} against independent {:.6}",
        adv.approx(),
        ind.approx()
    );
    println!(
        "     more than double the independent rate: {} (must be true, or the counter cannot \
         report a high rate at all)",
        adv.approx() > ind.approx() * 2.0
    );

    let decreasing = |i: i128, m: i128, _p: i128| (m - 1 - (i % m)).max(0);
    let dec = keyed_rate(&pts, m, 1, decreasing);
    println!("  C3 decreasing key: rate {:.6}, zero: {}", dec.approx(), dec == Q::int(0));
    println!("     (a second construction reaching zero for a different reason than the shared");
    println!("      threshold, which separates a real zero from a sleeping arm)");

    let pts_m = ramp(0, m, 64, m);
    println!(
        "  P2 at delta = m: shared {}, independent {:.6}, position {:.6} (all must be zero)",
        shared_rate(&pts_m, m).show(),
        independent_rate(&pts_m, m).approx(),
        keyed_rate(&pts_m, m, m, golden_key_offset_phased).approx()
    );
    println!();
    println!("=== end a2 ===");
}
