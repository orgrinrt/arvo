//! The same four arms, exhaustively, at every width from three bits to sixteen.
//!
//! The `i64` sweep beside this one samples, and a sampled law is a choice about
//! what not to find out. The rule has no width in it, so it can be run over the
//! whole matrix at a narrow width instead: at `W` bits the value set is every
//! `n` and every `d` in `[-2^(W-1), 2^(W-1)-1]`, which is `2^2W` pairs, and the
//! arithmetic is carried in `i64` so nothing the rule does can leave its domain.
//!
//! What is checked at every width:
//!
//!   - the denominator the arm returns is positive;
//!   - the sign of the returned ratio is the sign of the named one;
//!   - the returned ratio is exact wherever an exact one exists in the width;
//!   - where none exists, the relative error is at most `1 / MAX`.
//!
//! Exactness has to be decided rather than assumed, so the third check is run
//! against a search: a pair is exactly representable when some `(rn, rd)` inside
//! the width with `rd > 0` has `rn * d == n * rd`. At `W <= 8` that search is
//! exhaustive over the whole width; above it the family is derived instead, and
//! the two are cross-checked at every width where both run.
//!
//! Build: `rustc -O the_rule_over_every_width.rs -o /tmp/width_probe && /tmp/width_probe`

/// The shipped rule, parameterised on the width's extremes.
fn shipped(num: i64, den: i64, min: i64) -> (i64, i64) {
    if den > 0 {
        (num, den)
    } else if den == 0 {
        (0, 1)
    } else if den == min || num == min {
        (num, 1)
    } else {
        (-num, -den)
    }
}

fn holds_the_pair(num: i64, den: i64, _min: i64) -> (i64, i64) {
    (num, den)
}

fn zero_on_the_families(num: i64, den: i64, min: i64) -> (i64, i64) {
    if den > 0 {
        (num, den)
    } else if den == 0 {
        (0, 1)
    } else if den == min || num == min {
        (0, 1)
    } else {
        (-num, -den)
    }
}

/// The nearest-representable rule, parameterised the same way. `max` is
/// `-(min + 1)`, the largest magnitude the width carries.
///
/// **The shared power of two comes off first.** A first draft of this rule sent
/// every `min` pair straight to the substitution, and the exhaustive search in
/// section 0 refuted it: `of(min, -2)` is exactly `2^(W-2)` over one, because
/// both operands are even and the pair reduces inside the width. Only the pairs
/// where the operand that is not `min` is odd have no exact form at all.
fn nearest(num: i64, den: i64, min: i64) -> (i64, i64) {
    let max = -(min + 1);
    if den > 0 {
        return (num, den);
    }
    if den == 0 {
        return (0, 1);
    }
    if num != min && den != min {
        return (-num, -den);
    }
    if num == 0 {
        return (0, 1);
    }
    if num == min && den == min {
        return (1, 1);
    }
    // exactly one of the two is `min`, and the other decides how far the pair
    // cancels by twos
    let other = if den == min { num } else { den };
    let k = other.trailing_zeros();
    if k > 0 {
        // both are multiples of `2^k`, so the shift is an exact division and the
        // reduced pair is inside the width with room to negate
        return (-(num >> k), -(den >> k));
    }
    // the other operand is odd, so nothing cancels and the exact form needs a
    // magnitude one past the width. The nearest the width does carry:
    if den == min {
        (-num, max)
    } else {
        (max, -den)
    }
}

fn sign(x: i64) -> i64 {
    x.signum()
}

/// Whether `n/d` has an exact representation `(rn, rd)` with `rd > 0` and both
/// inside the width. Exhaustive: this is the definition, not a shortcut.
fn exactly_representable(n: i64, d: i64, min: i64, max: i64) -> bool {
    for rd in 1..=max {
        for rn in min..=max {
            if rn * d == n * rd {
                return true;
            }
        }
    }
    false
}

/// The derived answer to the same question: exactly the pairs the two families
/// name are not representable, and nothing else is.
fn derived_unrepresentable(n: i64, d: i64, min: i64) -> bool {
    if d >= 0 || n == 0 {
        return false;
    }
    if n == min && d == min {
        return false;
    }
    if n != min && d != min {
        return false;
    }
    // exactly one is `min`; the pair has an exact form exactly when the other
    // operand is even, because then the shared factor of two cancels
    let other = if d == min { n } else { d };
    other.trailing_zeros() == 0
}

struct Row {
    width: u32,
    arm: &'static str,
    den_violations: u64,
    sign_flips: u64,
    inexact_where_exact_exists: u64,
    bound_breaches: u64,
    worst_num: i128,
    worst_den: i128,
    worst_at: (i64, i64),
}

fn run(width: u32, arm_name: &'static str, arm: fn(i64, i64, i64) -> (i64, i64), search: bool) -> Row {
    let min = -(1i64 << (width - 1));
    let max = (1i64 << (width - 1)) - 1;
    let bound = max as i128;
    let mut r = Row {
        width,
        arm: arm_name,
        den_violations: 0,
        sign_flips: 0,
        inexact_where_exact_exists: 0,
        bound_breaches: 0,
        worst_num: 0,
        worst_den: 1,
        worst_at: (0, 0),
    };
    for n in min..=max {
        for d in min..=max {
            let (rn, rd) = arm(n, d, min);
            if rd <= 0 {
                r.den_violations += 1;
            }
            if d == 0 {
                continue;
            }
            if sign(rn) * sign(rd) != sign(n) * sign(d) {
                r.sign_flips += 1;
            }
            // exact means `rn/rd == n/d`, cross-multiplied
            let exact = (rn as i128) * (d as i128) == (n as i128) * (rd as i128);
            if !exact {
                let representable = if search {
                    exactly_representable(n, d, min, max)
                } else {
                    !derived_unrepresentable(n, d, min)
                };
                if representable {
                    r.inexact_where_exact_exists += 1;
                }
                if n != 0 {
                    // relative error `|rn*d - n*rd| / (|rd| * |n|)`
                    let en = ((rn as i128) * (d as i128) - (n as i128) * (rd as i128)).abs();
                    let ed = (rd as i128).abs() * (n as i128).abs();
                    if en * bound > ed {
                        r.bound_breaches += 1;
                    }
                    if en * r.worst_den > r.worst_num * ed {
                        r.worst_num = en;
                        r.worst_den = ed;
                        r.worst_at = (n, d);
                    }
                }
            }
        }
    }
    r
}

fn main() {
    // --- the two answers to "is this pair representable" have to agree --------
    println!("0. the exhaustive search and the derived family, cross-checked");
    for width in 3..=8u32 {
        let min = -(1i64 << (width - 1));
        let max = (1i64 << (width - 1)) - 1;
        let mut disagreements = 0u64;
        for n in min..=max {
            for d in min..=max {
                if d == 0 {
                    continue;
                }
                let searched = !exactly_representable(n, d, min, max);
                let derived = derived_unrepresentable(n, d, min);
                if searched != derived {
                    disagreements += 1;
                }
            }
        }
        println!("   W = {width:>2}: disagreements {disagreements}");
        assert_eq!(disagreements, 0, "the derived family is not the searched one at W = {width}");
    }
    // the control: a family that is deliberately wrong has to disagree
    let (min, max) = (-8i64, 7i64);
    let mut mutant = 0u64;
    for n in min..=max {
        for d in min..=max {
            if d == 0 {
                continue;
            }
            let searched = !exactly_representable(n, d, min, max);
            // dropping the `n == min` half of the family
            let wrong = d == min && n != min && n != 0;
            if searched != wrong {
                mutant += 1;
            }
        }
    }
    println!("   the control, a family missing one half: {mutant} disagreements at W = 4");
    assert!(mutant > 0, "the cross-check cannot detect a wrong family");
    println!();

    // --- the arms over the whole width matrix --------------------------------
    println!("1. the four arms, exhaustive at every width");
    println!(
        "   {:<4} {:<24} {:>10} {:>10} {:>14} {:>9}  {}",
        "W", "arm", "den <= 0", "sign flip", "lost an exact", "breaches", "worst relative"
    );
    let arms: [(&'static str, fn(i64, i64, i64) -> (i64, i64)); 4] = [
        ("shipped", shipped),
        ("holds the pair", holds_the_pair),
        ("zero on the families", zero_on_the_families),
        ("nearest representable", nearest),
    ];
    for width in [3u32, 4, 5, 6, 7, 8, 12, 16] {
        for (name, arm) in arms {
            // the exhaustive search is quadratic in the width's value set, so it
            // runs where it is affordable and the derived family carries the rest
            let r = run(width, name, arm, width <= 8);
            println!(
                "   {:<4} {:<24} {:>10} {:>10} {:>14} {:>9}  {}/{} at ({}, {})",
                r.width,
                r.arm,
                r.den_violations,
                r.sign_flips,
                r.inexact_where_exact_exists,
                r.bound_breaches,
                r.worst_num,
                r.worst_den,
                r.worst_at.0,
                r.worst_at.1
            );
        }
        println!();
    }

    // --- the claims, asserted rather than eyeballed ---------------------------
    println!("2. the claims about the nearest arm, asserted at every width");
    for width in [3u32, 4, 5, 6, 7, 8, 12, 16] {
        let r = run(width, "nearest", nearest, width <= 8);
        assert_eq!(r.den_violations, 0, "W = {width}: a non-positive denominator");
        assert_eq!(r.sign_flips, 0, "W = {width}: a sign flip");
        assert_eq!(r.inexact_where_exact_exists, 0, "W = {width}: lost an exact answer");
        assert_eq!(r.bound_breaches, 0, "W = {width}: breached 1/MAX");
    }
    println!("   positive denominator, sign preserved, exact where an exact exists,");
    println!("   relative error at most 1/MAX: all four hold at W in {{3,4,5,6,7,8,12,16}}");

    println!("\n3. the same claims against the shipped arm, which has to fail them");
    for width in [3u32, 4, 8, 16] {
        let r = run(width, "shipped", shipped, width <= 8);
        assert_eq!(r.den_violations, 0, "W = {width}: the shipped arm does hold this one");
        assert!(r.sign_flips > 0, "W = {}: the sign check cannot be failing", width);
        assert!(r.bound_breaches > 0, "W = {}: the bound check cannot be failing", width);
        println!(
            "   W = {width:>2}: {} sign flips, {} breaches of 1/MAX",
            r.sign_flips, r.bound_breaches
        );
    }
}
