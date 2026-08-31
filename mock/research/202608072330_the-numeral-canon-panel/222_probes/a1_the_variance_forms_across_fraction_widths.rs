// a1: does the stochastic-rounding variance form actually hold at a second fraction width?
//
// The question is `question::does_the_rounding_variance_form_hold_at_a_second_fraction_width`.
// Its own note says the forms were "stated as algebraic in the fraction width from a check at
// one fraction", and that one sweep at a second fraction is cheap and nobody has run it.
//
// The two forms under test, in units of the coarse quantum (one ulp of the target grid):
//
//     comonotone (one shared threshold for the whole pass):   Var = n^2 * f * (1 - f)
//     independent (a fresh threshold per rounding):           Var = n   * f * (1 - f)
//
// where n is the chain length and f the fractional residue of the value against the coarse grid.
//
// PREDICTIONS, stated before running:
//
// P1. Both forms hold at every attainable residue and every fraction width, in ulp units, and
//     the fraction width cannot enter the argument at all, because the error of one rounding is
//     defined in units of the quantum and the fraction width is what sets the quantum. So a
//     sweep over the fraction width should find zero movement, which is a `construction` warrant
//     rather than a swept one.
//
// P2. In ABSOLUTE units the forms are not width-free: they scale by exactly 4^-Fc. So the
//     unqualified sentence "the variance forms are algebraic in the fraction width" is true only
//     once the unit is stated, and the unit is stated nowhere I could find in the registry.
//
// P3. The residue the two prior instruments used is attainable at no fraction width. Both
//     `128_probes/r3_output.txt` and `130_probes/y1_output.txt` report f(1-f) = 2/9, which forces
//     f = 1/3 or f = 2/3. Neither is a binary rational, so neither is the residue of any narrowing
//     between two binary fixed-point grids. That does not falsify the forms; it means the
//     instruments never varied the fraction-width axis, because they were not standing on it.
//
// EXACTNESS. Everything below is integer arithmetic in i128, and every arithmetic operation is
// checked, because a probe that wraps silently reports a wrong number in exactly the shape of a
// right one. Write m = 2^d for d = Ff - Fc the number of bits dropped, and j the residue
// numerator, so f = j/m and the residue set attainable at that pair of widths is exactly
// { j/m : 0 <= j < m }. Scale each per-rounding error by m so it is an integer: rounding up costs
// (m - j), rounding down costs (-j). A chain of n roundings with k of them up has scaled total
// error S = k*m - n*j. Then Var(error in ulp) = Var(S) / m^2, and the two predictions become the
// integer identities
//
//     Var(S) = n^2 * j * (m - j)     (comonotone)
//     Var(S) = n   * j * (m - j)     (independent)
//
// which is what the arms below check, with no floating point anywhere.
//
// INDEPENDENCE OF THE INSTRUMENTS. Three routes compute the independent coupling's variance and
// none of them is allowed to assume the answer:
//   (1) full enumeration of all 2^n outcome vectors with exact rational weights;
//   (2) the binomial closed form over k with exact rational weights;
//   (3) an integer-only second central moment over the binomial counts, which is what reaches the
//       long chains, and which never uses Var(sum of independents) = sum of variances, the very
//       identity the predicted form encodes.
// Using the additivity identity to check a claim the identity states would be circular, and (3)
// is written the way it is to avoid exactly that.
//
// THE CASES THAT MUST FAIL. Four, and all four are printed failing rather than asserted to fail:
//   C1 a deterministic nearest rounding, whose variance is 0 and must match neither form;
//   C2 an antithetic coupling at n = 2, which is neither of the two couplings;
//   C3 a biased scheme rounding up with probability f^2, whose mean is not the value;
//   C4 a mutated predictor that swaps the two closed forms, which must disagree wherever n > 1.
// C4 is the one that makes the zeros mean anything: it shows the comparison can fire at all.

/// Exact rational over i128, normalised. Only what the arms below need.
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
    if a == 0 {
        1
    } else {
        a
    }
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
        let num = a.checked_add(b).expect("Q::add overflow");
        let den = self.d.checked_mul(o.d).expect("Q::add overflow");
        Q::new(num, den)
    }
    fn sub(self, o: Q) -> Q {
        let a = self.n.checked_mul(o.d).expect("Q::sub overflow");
        let b = o.n.checked_mul(self.d).expect("Q::sub overflow");
        let num = a.checked_sub(b).expect("Q::sub overflow");
        let den = self.d.checked_mul(o.d).expect("Q::sub overflow");
        Q::new(num, den)
    }
    fn mul(self, o: Q) -> Q {
        Q::new(
            self.n.checked_mul(o.n).expect("Q::mul overflow"),
            self.d.checked_mul(o.d).expect("Q::mul overflow"),
        )
    }
    fn show(self) -> String {
        if self.d == 1 {
            format!("{}", self.n)
        } else {
            format!("{}/{}", self.n, self.d)
        }
    }
}

/// n choose k, exact, small n only.
fn binom(n: u32, k: u32) -> i128 {
    let mut r: i128 = 1;
    for i in 0..k {
        r = r * (n - i) as i128 / (i + 1) as i128;
    }
    r
}

fn checked_pow(b: i128, e: u32) -> Option<i128> {
    let mut r: i128 = 1;
    for _ in 0..e {
        r = r.checked_mul(b)?;
    }
    Some(r)
}

fn pow(b: i128, e: u32) -> i128 {
    checked_pow(b, e).expect("pow overflow")
}

/// Route 1. Exact mean and variance of the scaled chain error S, by full enumeration of all 2^n
/// outcome vectors. The slow honest instrument the closed forms are checked against.
fn independent_by_enumeration(n: u32, m: i128, j: i128) -> (Q, Q) {
    let mut mean = Q::int(0);
    let mut second = Q::int(0);
    for mask in 0u32..(1u32 << n) {
        let k = mask.count_ones();
        let mut p = Q::int(1);
        for _ in 0..k {
            p = p.mul(Q::new(j, m));
        }
        for _ in 0..(n - k) {
            p = p.mul(Q::new(m - j, m));
        }
        let s = Q::int(k as i128 * m - n as i128 * j);
        mean = mean.add(p.mul(s));
        second = second.add(p.mul(s).mul(s));
    }
    (mean, second.sub(mean.mul(mean)))
}

/// Route 2. The same by the binomial closed form over k, exact rationals.
fn independent_by_binomial(n: u32, m: i128, j: i128) -> Option<(Q, Q)> {
    let denom = checked_pow(m, n)?;
    let mut mean = Q::int(0);
    let mut second = Q::int(0);
    for k in 0..=n {
        let ways = binom(n, k);
        let a = checked_pow(j, k)?;
        let b = checked_pow(m - j, n - k)?;
        let w = ways.checked_mul(a)?.checked_mul(b)?;
        let p = Q::new(w, denom);
        let s = Q::int(k as i128 * m - n as i128 * j);
        mean = mean.add(p.mul(s));
        second = second.add(p.mul(s).mul(s));
    }
    Some((mean, second.sub(mean.mul(mean))))
}

/// Route 3. Integer-only second central moment over the binomial counts.
///
/// `T = sum P_k = m^n`, `M1 = sum P_k * k`, `M2 = sum P_k * k^2`, all exact integers, and then
/// `Var(k) = (M2*T - M1^2) / T^2`. Since `S = k*m - n*j` is affine in `k`, `Var(S) = m^2*Var(k)`.
/// Nowhere does this use additivity of variance across independent summands, which is the
/// identity the predicted form encodes, so this route can disagree with the prediction.
///
/// `None` where the exact arithmetic would leave i128. Skipping loudly beats wrapping quietly,
/// and the skip count is printed so the coverage claim stays honest.
fn independent_by_integer_moments(n: u32, m: i128, j: i128) -> Option<(Q, Q)> {
    let t = checked_pow(m, n)?;
    let mut m1: i128 = 0;
    let mut m2: i128 = 0;
    for k in 0..=n {
        let ways = binom(n, k);
        let a = checked_pow(j, k)?;
        let b = checked_pow(m - j, n - k)?;
        let p = ways.checked_mul(a)?.checked_mul(b)?;
        m1 = m1.checked_add(p.checked_mul(k as i128)?)?;
        m2 = m2.checked_add(p.checked_mul((k as i128).checked_mul(k as i128)?)?)?;
    }
    // internal consistency: the weights must sum to m^n. A silent disagreement here would
    // invalidate everything below it, so it is checked rather than assumed.
    let mut tsum: i128 = 0;
    for k in 0..=n {
        let ways = binom(n, k);
        let a = checked_pow(j, k)?;
        let b = checked_pow(m - j, n - k)?;
        tsum = tsum.checked_add(ways.checked_mul(a)?.checked_mul(b)?)?;
    }
    assert_eq!(
        tsum, t,
        "binomial weights do not sum to m^n at n={n} m={m} j={j}"
    );

    let num = m2.checked_mul(t)?.checked_sub(m1.checked_mul(m1)?)?;
    let den = t.checked_mul(t)?;
    let var_k = Q::new(num, den);
    let mean_s = Q::new(m1, t).mul(Q::int(m)).sub(Q::int(n as i128 * j));
    let var_s = var_k.mul(Q::int(m)).mul(Q::int(m));
    Some((mean_s, var_s))
}

/// Comonotone: one shared threshold, so every rounding in the pass agrees.
fn comonotone(n: u32, m: i128, j: i128) -> (Q, Q) {
    let up = Q::new(j, m);
    let down = Q::new(m - j, m);
    let s_up = Q::int(n as i128 * m - n as i128 * j);
    let s_down = Q::int(-(n as i128) * j);
    let mean = up.mul(s_up).add(down.mul(s_down));
    let second = up.mul(s_up).mul(s_up).add(down.mul(s_down).mul(s_down));
    (mean, second.sub(mean.mul(mean)))
}

/// Antithetic at n = 2: the second threshold is 1 - U. Up_1 iff U < f; up_2 iff 1 - U < f.
/// Disjoint when f <= 1/2 and overlapping when f > 1/2, which is why this is neither of the two
/// couplings and is control C2.
fn antithetic_pair(m: i128, j: i128) -> (Q, Q) {
    let both = if 2 * j >= m {
        Q::new(2 * j - m, m)
    } else {
        Q::int(0)
    };
    let neither = if 2 * j <= m {
        Q::new(m - 2 * j, m)
    } else {
        Q::int(0)
    };
    let one = Q::int(1).sub(both).sub(neither);
    let s = |k: i128| Q::int(k * m - 2 * j);
    let mean = both.mul(s(2)).add(one.mul(s(1))).add(neither.mul(s(0)));
    let second = both
        .mul(s(2))
        .mul(s(2))
        .add(one.mul(s(1)).mul(s(1)))
        .add(neither.mul(s(0)).mul(s(0)));
    (mean, second.sub(mean.mul(mean)))
}

/// Control C3: independent draws, but rounding up with probability f^2 rather than f.
fn biased_independent(n: u32, m: i128, j: i128) -> (Q, Q) {
    let mut mean = Q::int(0);
    let mut second = Q::int(0);
    let pu = Q::new(j, m).mul(Q::new(j, m));
    let pd = Q::int(1).sub(pu);
    for mask in 0u32..(1u32 << n) {
        let k = mask.count_ones();
        let mut p = Q::int(1);
        for _ in 0..k {
            p = p.mul(pu);
        }
        for _ in 0..(n - k) {
            p = p.mul(pd);
        }
        let s = Q::int(k as i128 * m - n as i128 * j);
        mean = mean.add(p.mul(s));
        second = second.add(p.mul(s).mul(s));
    }
    (mean, second.sub(mean.mul(mean)))
}

fn main() {
    println!("=== a1: the stochastic-rounding variance forms across fraction widths ===");
    println!();
    println!("Units. Everything is in coarse-grid ulp, scaled by m = 2^d so it is integral.");
    println!("Predicted Var(S): comonotone n^2*j*(m-j), independent n*j*(m-j), with f = j/m.");
    println!();

    // ---------------------------------------------------------------------------------
    // PART 0. Is the residue the prior instruments used attainable on any binary grid?
    // ---------------------------------------------------------------------------------
    println!("--- Part 0: is f = 1/3 (the residue behind f(1-f) = 2/9) attainable at any d? ---");
    let mut found = false;
    for d in 1u32..=24 {
        let m = 1i128 << d;
        for j in 0..m {
            if j * 3 == m {
                found = true;
                println!("  d = {d}: j = {j} gives exactly 1/3");
            }
        }
    }
    println!(
        "  attainable at d in 1..=24: {found}  (predicted false: 1/3 has no finite binary expansion)"
    );
    println!("  Positive control on the same walk, so the loop is shown able to find something:");
    let mut half_found = 0;
    for d in 1u32..=24 {
        let m = 1i128 << d;
        for j in 0..m {
            if j * 2 == m {
                half_found += 1;
            }
        }
    }
    println!("  residues equal to 1/2 found at {half_found} of 24 widths (predicted 24)");
    println!("  so the two prior checks stood at a residue no narrowing between binary grids");
    println!("  produces, which is why neither of them varied the fraction-width axis.");
    println!();

    // ---------------------------------------------------------------------------------
    // PART 1. The closed forms against full enumeration, over every attainable residue.
    // ---------------------------------------------------------------------------------
    println!("--- Part 1: closed forms vs full 2^n enumeration, every attainable residue ---");
    let mut checked = 0u64;
    let mut mismatches = 0u64;
    let mut route_disagreements = 0u64;
    for d in 1u32..=4 {
        let m = 1i128 << d;
        for j in 0..m {
            for n in 1u32..=10 {
                let (mean_e, var_e) = independent_by_enumeration(n, m, j);
                let (mean_b, var_b) =
                    independent_by_binomial(n, m, j).expect("part 1 stays inside i128");
                let (mean_i, var_i) =
                    independent_by_integer_moments(n, m, j).expect("part 1 stays inside i128");
                if mean_e != mean_b || var_e != var_b || mean_e != mean_i || var_e != var_i {
                    route_disagreements += 1;
                    println!("  ROUTES DISAGREE d={d} j={j} n={n}");
                }
                let predicted_ind = Q::int(n as i128 * j * (m - j));
                if var_e != predicted_ind {
                    mismatches += 1;
                    println!(
                        "  INDEPENDENT MISMATCH d={d} j={j} n={n}: got {} predicted {}",
                        var_e.show(),
                        predicted_ind.show()
                    );
                }
                if mean_e != Q::int(0) {
                    mismatches += 1;
                    println!(
                        "  INDEPENDENT BIASED d={d} j={j} n={n}: mean {}",
                        mean_e.show()
                    );
                }
                let (mean_c, var_c) = comonotone(n, m, j);
                let predicted_com = Q::int((n as i128) * (n as i128) * j * (m - j));
                if var_c != predicted_com {
                    mismatches += 1;
                    println!(
                        "  COMONOTONE MISMATCH d={d} j={j} n={n}: got {} predicted {}",
                        var_c.show(),
                        predicted_com.show()
                    );
                }
                if mean_c != Q::int(0) {
                    mismatches += 1;
                    println!(
                        "  COMONOTONE BIASED d={d} j={j} n={n}: mean {}",
                        mean_c.show()
                    );
                }
                checked += 1;
            }
        }
    }
    println!("  cells checked: {checked}");
    println!("  closed-form mismatches: {mismatches} (predicted 0)");
    println!("  three-route disagreements: {route_disagreements} (predicted 0)");
    println!("  residues covered: every j/m for d in 1..=4, which is the whole attainable set at");
    println!("  those width pairs, so this span is exhaustive rather than sampled.");
    println!();

    // ---------------------------------------------------------------------------------
    // PART 2. Longer chains, via the integer-moment route, which is the one that reaches.
    // ---------------------------------------------------------------------------------
    println!("--- Part 2: long chains via the integer-moment route ---");
    let mut long_checked = 0u64;
    let mut long_mismatch = 0u64;
    let mut long_skipped = 0u64;
    let mut max_n_per_d: Vec<(u32, u32)> = Vec::new();
    for d in 1u32..=6 {
        let m = 1i128 << d;
        let mut deepest = 0u32;
        for j in 0..m {
            for n in 1u32..=64 {
                match independent_by_integer_moments(n, m, j) {
                    None => {
                        long_skipped += 1;
                        continue;
                    }
                    Some((mean, var)) => {
                        let predicted = Q::int(n as i128 * j * (m - j));
                        if var != predicted || mean != Q::int(0) {
                            long_mismatch += 1;
                            println!(
                                "  LONG MISMATCH d={d} j={j} n={n}: {} vs {}",
                                var.show(),
                                predicted.show()
                            );
                        }
                        if n > deepest {
                            deepest = n;
                        }
                    }
                }
                let (mean_c, var_c) = comonotone(n, m, j);
                let predicted_c = Q::int((n as i128) * (n as i128) * j * (m - j));
                if var_c != predicted_c || mean_c != Q::int(0) {
                    long_mismatch += 1;
                    println!("  LONG COMONOTONE MISMATCH d={d} j={j} n={n}");
                }
                long_checked += 1;
            }
        }
        max_n_per_d.push((d, deepest));
    }
    println!("  cells attempted: {long_checked}, mismatches: {long_mismatch} (predicted 0)");
    println!("  cells skipped for leaving i128: {long_skipped}");
    print!("  deepest chain actually reached per d:");
    for (d, n) in &max_n_per_d {
        print!(" d={d}:n<={n}");
    }
    println!();
    println!();

    // ---------------------------------------------------------------------------------
    // PART 3. The axis itself: does the coarse fraction width move anything?
    // ---------------------------------------------------------------------------------
    println!("--- Part 3: varying the coarse fraction width Fc, which is the axis in question ---");
    println!("  In ulp units the width cannot enter: the scaled error is k*m - n*j and Fc is");
    println!("  absent from that expression. In absolute units the variance carries 4^-Fc.");
    println!();
    let d = 3u32;
    let m = 1i128 << d;
    let j = 3i128; // f = 3/8, a residue attainable at exactly this width pair
    let n = 6u32;
    let (_, var_ulp) = independent_by_integer_moments(n, m, j).expect("fits");
    let mut ulp_values = Vec::new();
    for fc in [0u32, 1, 3, 8, 16, 31] {
        let var_in_ulp2 = Q::new(var_ulp.n, var_ulp.d * m * m);
        let scale = Q::new(1, pow(2, 2 * fc));
        let var_abs = var_in_ulp2.mul(scale);
        println!(
            "  Fc = {:>2}: Var(ulp^2) = {:<10}  Var(absolute) = {}",
            fc,
            var_in_ulp2.show(),
            var_abs.show()
        );
        ulp_values.push(var_in_ulp2);
    }
    let all_same = ulp_values.windows(2).all(|w| w[0] == w[1]);
    println!("  ulp-unit variance identical across every Fc: {all_same} (predicted true)");
    println!("  absolute-unit variance moves with Fc, by exactly 4^-Fc, as the column shows.");
    println!();
    println!("  So the sentence is width-free only in ulp units, and the corpus does not state");
    println!("  the unit. That is the finding, and it is a wording defect rather than a false");
    println!("  claim: `fraction_width: F any` is earned by construction in ulp and is false");
    println!("  in absolute units, where the correct entry is a scaling rather than a span.");
    println!();

    // ---------------------------------------------------------------------------------
    // PART 4. The four controls, shown failing.
    // ---------------------------------------------------------------------------------
    println!("--- Part 4: the cases that must fail, shown failing ---");
    let m = 8i128;
    let j = 3i128; // f = 3/8
    let n = 4u32;
    let pred_ind = Q::int(n as i128 * j * (m - j));
    let pred_com = Q::int((n as i128) * (n as i128) * j * (m - j));

    let var_det = Q::int(0);
    println!(
        "  C1 deterministic nearest: Var(S) = {}, independent form = {}, comonotone form = {}",
        var_det.show(),
        pred_ind.show(),
        pred_com.show()
    );
    println!(
        "     matches independent: {}, matches comonotone: {} (both must be false)",
        var_det == pred_ind,
        var_det == pred_com
    );

    let (mean_a, var_a) = antithetic_pair(m, j);
    let pred_ind_2 = Q::int(2 * j * (m - j));
    let pred_com_2 = Q::int(4 * j * (m - j));
    println!(
        "  C2 antithetic n=2: mean = {}, Var(S) = {}; independent form = {}, comonotone form = {}",
        mean_a.show(),
        var_a.show(),
        pred_ind_2.show(),
        pred_com_2.show()
    );
    println!(
        "     matches independent: {}, matches comonotone: {} (both must be false)",
        var_a == pred_ind_2,
        var_a == pred_com_2
    );

    let (mean_b, var_b) = biased_independent(n, m, j);
    println!(
        "  C3 biased (up with probability f^2): mean = {} (must not be 0), Var(S) = {}, form = {}",
        mean_b.show(),
        var_b.show(),
        pred_ind.show()
    );
    println!(
        "     mean is zero: {}, matches independent form: {} (both must be false)",
        mean_b == Q::int(0),
        var_b == pred_ind
    );

    let mut swap_disagreements = 0u64;
    let mut swap_cells = 0u64;
    for d in 1u32..=4 {
        let m = 1i128 << d;
        for j in 1..m {
            for n in 2u32..=8 {
                let (_, var_i) = independent_by_integer_moments(n, m, j).expect("fits");
                let mutated = Q::int((n as i128) * (n as i128) * j * (m - j));
                if var_i != mutated {
                    swap_disagreements += 1;
                }
                swap_cells += 1;
            }
        }
    }
    println!(
        "  C4 mutated predictor (comonotone form against independent data): {swap_disagreements} of \
         {swap_cells} cells disagree"
    );
    println!("     (must be all of them for n > 1; if this were 0 the Part 1 zeros would be worth");
    println!("      nothing, because the comparison would be incapable of reporting a difference)");
    println!();
    println!("=== end a1 ===");
}
