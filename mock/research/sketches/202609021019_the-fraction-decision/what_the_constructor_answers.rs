//! What `Fraction::of` answers against the ratio it was handed, and what three
//! candidate constructors answer instead.
//!
//! Four arms, all measured against one oracle. The oracle is exact: a returned
//! pair `(rn, rd)` is compared with the named `(n, d)` by cross-multiplying, and
//! the relative error `|rn*d - n*rd| / (|rd| * |n|)` is compared with a claimed
//! bound by cross-multiplying again.
//!
//! **Every product is checked.** The first version of this file formed those
//! products in `i128` and silently wrapped: `|rn*d - n*rd| * i64::MAX` reaches
//! `2^189` on the shipped arm, so section 5 reported a worst error of `0/-1` and
//! a bound the shipped arm breached fewer times than it does. Magnitudes are
//! `u128` here and every multiply is `checked_mul`. An overflow is a definite
//! breach of the bound rather than a wrap, because the comparand `|rd| * |n|` is
//! at most `2^126` and an overflowing product is above that.
//!
//! Build: `rustc -O what_the_constructor_answers.rs -o /tmp/frac_probe && /tmp/frac_probe`

// --- the arms --------------------------------------------------------------

/// What the crate ships. Two families fall through to a denominator of one with
/// the numerator untouched.
fn shipped(num: i64, den: i64) -> (i64, i64) {
    if den > 0 {
        (num, den)
    } else if den == 0 {
        (0, 1)
    } else if den == i64::MIN || num == i64::MIN {
        (num, 1)
    } else {
        (-num, -den)
    }
}

/// The `Phase` answer, applied here: hold the pair, normalise nothing.
fn holds_the_pair(num: i64, den: i64) -> (i64, i64) {
    (num, den)
}

/// The two families read as naming nothing, which is what the constructor did
/// for every negative denominator before the family was narrowed to two.
fn zero_on_the_families(num: i64, den: i64) -> (i64, i64) {
    if den > 0 {
        (num, den)
    } else if den == 0 {
        (0, 1)
    } else if den == i64::MIN || num == i64::MIN {
        (0, 1)
    } else {
        (-num, -den)
    }
}

/// The nearest-representable rule: normalise exactly wherever the width admits
/// an exact form, and fall back only where it does not.
///
/// **The shared power of two comes off first.** A first draft sent every
/// `i64::MIN` pair straight to the substitution, and the exhaustive search in
/// `the_rule_over_every_width.rs` refuted it: `of(i64::MIN, -2)` is exactly
/// `2^62` over one, because both operands are even and the pair reduces inside
/// the type. Only the pairs whose other operand is odd have no exact form.
fn nearest(num: i64, den: i64) -> (i64, i64) {
    if den > 0 {
        return (num, den);
    }
    if den == 0 {
        return (0, 1);
    }
    if num != i64::MIN && den != i64::MIN {
        return (-num, -den);
    }
    if num == 0 {
        return (0, 1);
    }
    if num == i64::MIN && den == i64::MIN {
        return (1, 1);
    }
    let other = if den == i64::MIN { num } else { den };
    let k = other.trailing_zeros();
    if k > 0 {
        return (-(num >> k), -(den >> k));
    }
    if den == i64::MIN {
        (-num, i64::MAX)
    } else {
        (i64::MAX, -den)
    }
}

// --- the oracle ------------------------------------------------------------

fn mag(x: i64) -> u128 {
    x.unsigned_abs() as u128
}

fn sign(x: i64) -> i32 {
    x.signum() as i32
}

/// `|rn*d - n*rd|` as a magnitude, exactly. Every term is at most `2^126`, so
/// the difference of two of them is at most `2^127`, which is one past `i128`
/// and inside `u128` once the sign is taken off first.
fn error_numerator(r: (i64, i64), n: i64, d: i64) -> u128 {
    let a = mag(r.0) * mag(d);
    let b = mag(n) * mag(r.1);
    let sa = sign(r.0) * sign(d);
    let sb = sign(n) * sign(r.1);
    if sa == sb {
        if a > b {
            a - b
        } else {
            b - a
        }
    } else {
        a + b
    }
}

/// `|rd| * |n|`, the denominator the error numerator sits over once the whole
/// thing is made relative. At most `2^126`.
fn error_denominator(r: (i64, i64), n: i64) -> u128 {
    mag(r.1) * mag(n)
}

/// Whether the relative error is at most `1 / bound`, exactly.
///
/// `en / ed <= 1 / bound` is `en * bound <= ed`. An overflow on the left is a
/// breach, because `ed` is at most `2^126` and the product is above `u128::MAX`.
fn within(en: u128, ed: u128, bound: u128) -> bool {
    match en.checked_mul(bound) {
        Some(p) => p <= ed,
        None => false,
    }
}

/// Whether `a / b` is a strictly larger relative error than `c / d`, exactly.
///
/// `a/b > c/d` is `a*d > c*b`. On an overflow the comparison is reported as
/// undecided rather than guessed, and the caller counts those.
fn greater(a: (u128, u128), c: (u128, u128)) -> Option<bool> {
    match (a.0.checked_mul(c.1), c.0.checked_mul(a.1)) {
        (Some(l), Some(r)) => Some(l > r),
        _ => None,
    }
}

fn sign_agrees(r: (i64, i64), n: i64, d: i64) -> bool {
    sign(r.0) * sign(r.1) == sign(n) * sign(d)
}

// --- the sweep -------------------------------------------------------------

fn interesting() -> Vec<i64> {
    let mut v = vec![
        i64::MIN,
        i64::MIN + 1,
        i64::MIN + 2,
        -(1i64 << 62),
        -1_000_000_007,
        -7,
        -3,
        -2,
        -1,
        0,
        1,
        2,
        3,
        7,
        1_000_000_007,
        1i64 << 62,
        i64::MAX - 1,
        i64::MAX,
    ];
    // a spread that is neither a power of two nor near an edge, so a rule that
    // only holds on edges shows up
    let mut x: i64 = 1;
    for _ in 0..40 {
        x = x.wrapping_mul(3).wrapping_add(11);
        v.push(x);
        v.push(-x);
    }
    v.sort_unstable();
    v.dedup();
    v
}

struct Verdict {
    name: &'static str,
    denominator_violations: usize,
    sign_flips: usize,
    inexact: usize,
    undecided: usize,
    worst: (u128, u128),
    worst_at: (i64, i64),
}

fn measure(name: &'static str, arm: fn(i64, i64) -> (i64, i64), pairs: &[(i64, i64)]) -> Verdict {
    let mut v = Verdict {
        name,
        denominator_violations: 0,
        sign_flips: 0,
        inexact: 0,
        undecided: 0,
        worst: (0, 1),
        worst_at: (0, 1),
    };
    for &(n, d) in pairs {
        let r = arm(n, d);
        if r.1 <= 0 {
            v.denominator_violations += 1;
        }
        // a zero denominator names no ratio, so there is nothing to be exact
        // about and no sign to compare
        if d == 0 {
            continue;
        }
        if !sign_agrees(r, n, d) {
            v.sign_flips += 1;
        }
        let en = error_numerator(r, n, d);
        if en == 0 {
            continue;
        }
        v.inexact += 1;
        if n == 0 {
            // the named ratio is zero, so a relative error is not defined; the
            // absolute error is already counted as inexact above
            continue;
        }
        let ed = error_denominator(r, n);
        match greater((en, ed), v.worst) {
            Some(true) => {
                v.worst = (en, ed);
                v.worst_at = (n, d);
            }
            Some(false) => {}
            None => v.undecided += 1,
        }
    }
    v
}

fn row(v: &Verdict) {
    println!(
        "   {:<24} {:>9} {:>10} {:>8} {:>10}  {} / {} at of({}, {})",
        v.name,
        v.denominator_violations,
        v.sign_flips,
        v.inexact,
        v.undecided,
        v.worst.0,
        v.worst.1,
        v.worst_at.0,
        v.worst_at.1
    );
}

fn header() {
    println!(
        "   {:<24} {:>9} {:>10} {:>8} {:>10}  {}",
        "arm", "den <= 0", "sign flip", "inexact", "undecided", "worst relative error"
    );
}

fn main() {
    let vals = interesting();
    let mut pairs = Vec::new();
    for &n in &vals {
        for &d in &vals {
            pairs.push((n, d));
        }
    }
    println!(
        "swept {} pairs over {} values per axis\n",
        pairs.len(),
        vals.len()
    );

    // --- 0. the oracle's own control -----------------------------------------
    //
    // The oracle has to call an exact answer exact and a wrong one wrong, or
    // every column below is a fact about the oracle.
    println!("0. the oracle, checked against three answers whose verdict is known");
    let cases: [((i64, i64), i64, i64, u128, &str); 3] = [
        ((-3, 7), 3, -7, 0, "exact, sign moved"),
        ((3, 1), 3, -7, 24, "the shipped answer on a pair it does normalise"),
        ((1, 2), 1, 2, 0, "the identity"),
    ];
    for (r, n, d, expect, what) in cases {
        let en = error_numerator(r, n, d);
        println!("   {what}: error numerator {en}, expected {expect}");
        assert_eq!(en, expect, "the oracle disagrees with a hand-computed case");
    }
    println!();

    // --- 1. the reported behaviour, reproduced --------------------------------
    println!("1. the two pairs the brief names, under the shipped constructor");
    for &(n, d) in &[(3i64, i64::MIN), (i64::MIN, -7i64)] {
        let r = shipped(n, d);
        println!(
            "   of({n}, {d}) = {}/{}   named sign {}   answered sign {}",
            r.0,
            r.1,
            sign(n) * sign(d),
            sign(r.0) * sign(r.1)
        );
    }
    println!();

    // --- 2. is the positive denominator load-bearing --------------------------
    //
    // The reader is `Exact::between`, which splits with `div_euclid` and
    // `rem_euclid` and stores the remainder over the same denominator. The
    // question is only whether the stored remainder lands in `[0, 1)`, which is
    // the reading the rounding modes are stated over.
    println!("2. what the euclidean split stores when the denominator is negative");
    let mut negative_outside = 0usize;
    let mut negative_total = 0usize;
    let mut positive_inside = 0usize;
    for &(n, d) in &pairs {
        if d == 0 {
            continue;
        }
        let rem = match n.checked_rem_euclid(d) {
            Some(r) => r,
            None => continue,
        };
        // the stored remainder is `rem / d`, and `rem_euclid` gives `rem >= 0`
        let in_unit = if d > 0 { rem < d } else { rem == 0 };
        if d < 0 {
            negative_total += 1;
            if !in_unit {
                negative_outside += 1;
            }
        } else if in_unit {
            positive_inside += 1;
        }
    }
    println!("   negative denominators whose stored remainder leaves [0, 1): {negative_outside} of {negative_total}");
    println!("   positive denominators whose stored remainder stays in [0, 1): {positive_inside}  (the control)");
    println!();

    // --- 3. the arms -----------------------------------------------------------
    println!("3. four arms against the exact ratio");
    let arms: [(&'static str, fn(i64, i64) -> (i64, i64)); 4] = [
        ("shipped", shipped),
        ("holds the pair", holds_the_pair),
        ("zero on the families", zero_on_the_families),
        ("nearest representable", nearest),
    ];
    header();
    let mut verdicts = Vec::new();
    for (name, arm) in arms {
        let v = measure(name, arm, &pairs);
        row(&v);
        verdicts.push(v);
    }
    println!();

    // --- 4. the control --------------------------------------------------------
    println!("4. the control: an arm that has to score badly, and does");
    fn always_half(_num: i64, _den: i64) -> (i64, i64) {
        (1, 2)
    }
    header();
    let c = measure("always one half", always_half, &pairs);
    row(&c);
    assert!(
        c.inexact > verdicts[3].inexact && c.sign_flips > verdicts[0].sign_flips,
        "the control has to be worse on both columns, or the columns are not measuring"
    );
    println!("   worse on both columns than every arm, so the columns move\n");

    // --- 5. the bound in closed form -------------------------------------------
    println!("5. the nearest arm's bound, stated and checked");
    let bound = i64::MAX as u128;
    for (name, arm) in [
        ("nearest representable", nearest as fn(i64, i64) -> (i64, i64)),
        ("shipped", shipped as fn(i64, i64) -> (i64, i64)),
    ] {
        let mut breaches = 0usize;
        let mut worst_breach = (0i64, 0i64);
        for &(n, d) in &pairs {
            if d == 0 || n == 0 {
                continue;
            }
            let r = arm(n, d);
            let en = error_numerator(r, n, d);
            let ed = error_denominator(r, n);
            if !within(en, ed, bound) {
                breaches += 1;
                worst_breach = (n, d);
            }
        }
        println!("   {name:<24} breaches of `relative error <= 1/i64::MAX`: {breaches:>5}   last at of({}, {})", worst_breach.0, worst_breach.1);
    }
    println!("   the shipped row is the control: a bound nothing breaches distinguishes nothing\n");

    // --- 6. `is_tie` against the comparison `round_slot` makes ------------------
    println!("6. `is_tie` in i64 against the same comparison in i128");
    let mut leaves_i64 = 0usize;
    let mut disagreements = 0usize;
    let mut witness = (0i64, 0i64);
    for &(n, d) in &pairs {
        if d <= 0 {
            continue;
        }
        let rem = n.rem_euclid(d);
        let wide = (rem as i128) * 2 == d as i128;
        match rem.checked_mul(2) {
            Some(t) => {
                if (t == d) != wide {
                    disagreements += 1;
                }
            }
            None => {
                leaves_i64 += 1;
                witness = (n, d);
            }
        }
    }
    println!("   stored remainders whose doubling leaves i64: {leaves_i64}, e.g. the remainder of of({}, {})", witness.0, witness.1);
    println!("   disagreements where the doubling fits: {disagreements}");
    // the control: a wide comparison that is deliberately wrong has to disagree
    let mut mutant_disagreements = 0usize;
    for &(n, d) in &pairs {
        if d <= 0 {
            continue;
        }
        let rem = n.rem_euclid(d);
        let mutant = (rem as i128) * 2 == (d as i128) + 1;
        if let Some(t) = rem.checked_mul(2) {
            if (t == d) != mutant {
                mutant_disagreements += 1;
            }
        }
    }
    println!("   the same check against a mutated wide form: {mutant_disagreements} disagreements  (the control)");
    assert!(
        mutant_disagreements > 0,
        "the comparison cannot detect anything if a mutated oracle also agrees"
    );

    // --- 7. what the shipped rule costs at i64, in closed form ---------------
    //
    // The exhaustive run beside this one gives the counts at eight widths. The
    // closed form below reproduces every one of them, which is what licenses
    // reading it at 64 where no enumeration is available.
    println!("\n7. the shipped rule's damage, closed form checked against the exhaustive widths");
    fn lost_an_exact(w: u32) -> u128 {
        (1u128 << (w - 1)) + (1u128 << (w - 2)) - 2
    }
    fn sign_flips(w: u32) -> u128 {
        (1u128 << w) + (1u128 << (w - 1)) - 2
    }
    // the numbers the exhaustive probe printed, transcribed so a drift in either
    // file shows up here
    for (w, lost, flips) in [(3u32, 4u128, 10u128), (4, 10, 22), (8, 190, 382), (16, 49150, 98302)] {
        assert_eq!(lost_an_exact(w), lost, "closed form disagrees at W = {w}");
        assert_eq!(sign_flips(w), flips, "closed form disagrees at W = {w}");
        println!("   W = {w:>2}: lost an exact {lost}, sign flips {flips}  (both match the exhaustive run)");
    }
    println!("   W = 64: lost an exact {}, sign flips {}", lost_an_exact(64), sign_flips(64));
    println!("   of 2^128 pairs, so the families are {} and {} of the input space",
        lost_an_exact(64), sign_flips(64));

    // --- 8. the two places `Exact` does its own arithmetic in i64 -------------
    //
    // `Fraction`'s invariant exists for one reader, so what that reader does with
    // it is part of the same question. Both of these are `i64` where the
    // neighbouring `round_slot` is `i128`.
    println!("\n8. `Exact::between` and `is_tie`, in the carrier they are written in");
    let mut carry_overflows = 0usize;
    let mut carry_witness = (0i64, 0i64, 0i64);
    for &(n, d) in &pairs {
        if d <= 0 {
            continue;
        }
        let whole = n.div_euclid(d);
        for &slot in &[0i64, i64::MAX, i64::MIN, i64::MAX - 1] {
            if slot.checked_add(whole).is_none() {
                carry_overflows += 1;
                carry_witness = (slot, n, d);
            }
        }
    }
    println!("   slot carries that leave i64 in `between`: {carry_overflows}, e.g. slot {} with of({}, {})",
        carry_witness.0, carry_witness.1, carry_witness.2);
    // the control: a carry of zero can never overflow, so a rule that reported
    // an overflow there would be reporting on nothing
    let mut zero_carry_overflows = 0usize;
    for &slot in &[0i64, i64::MAX, i64::MIN] {
        if slot.checked_add(0).is_none() {
            zero_carry_overflows += 1;
        }
    }
    println!("   the same check with a carry of zero: {zero_carry_overflows}  (the control)");
    assert_eq!(zero_carry_overflows, 0);
    assert!(carry_overflows > 0, "the carry check cannot be detecting anything");
}
