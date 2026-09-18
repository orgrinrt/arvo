// seat 267, probe a: exhaustive check that "nearest, ties toward positive
// infinity" (floor(x + 1/2)) is translation equivariant on the exact domain
// the fma law row states, and that "nearest, ties away from zero" is not.
//
// domain: total_width = 6, fraction_width in 0..=5, signed, values as i64
// numerators over a fixed denominator `den = 1 << fraction_width`, swept
// across the full representable range for a signed 6-bit container
// (-32..=31) and every integer shift k in -4..=4 (small shifts suffice: the
// claim is a closed-form identity, not a bounded-region empirical fact, and
// the sweep is a check on the identity rather than the source of the claim).
//
// control: half_even (round to even on a tie), which the fma law row's own
// `fails` region already lists as non-equivariant, run through the same
// equivariance check. If the harness cannot make this one fail it proves
// nothing about the other two.

fn floor_div(n: i64, d: i64) -> i64 {
    n.div_euclid(d)
}

// nearest, ties toward positive infinity: floor(x + 1/2)
fn half_up_toward_pos_inf(num: i64, den: i64) -> i64 {
    floor_div(2 * num + den, 2 * den)
}

// nearest, ties to even (control: known non-equivariant per the law row's
// own `fails` region)
fn half_even(num: i64, den: i64) -> i64 {
    let sign = if num < 0 { -1 } else { 1 };
    let mag = num.abs();
    let q = mag / den;
    let r = mag % den;
    let rounded = if 2 * r > den {
        q + 1
    } else if 2 * r < den {
        q
    } else if q % 2 == 0 {
        q
    } else {
        q + 1
    };
    sign * rounded
}

// nearest, ties away from zero
fn half_up_away_from_zero(num: i64, den: i64) -> i64 {
    let sign = if num < 0 { -1 } else { 1 };
    let mag = num.abs();
    let q = mag / den;
    let r = mag % den;
    let rounded = if 2 * r >= den { q + 1 } else { q };
    sign * rounded
}

fn check_equivariant<F: Fn(i64, i64) -> i64>(
    name: &str,
    f: F,
    den: i64,
    lo: i64,
    hi: i64,
) -> (u64, u64) {
    let mut pass = 0u64;
    let mut fail = 0u64;
    for x_num in (lo * den) ..= (hi * den) {
        for k in -4i64 ..= 4 {
            let shifted = x_num + k * den;
            let lhs = f(shifted, den);
            let rhs = f(x_num, den) + k;
            if lhs == rhs {
                pass += 1;
            } else {
                fail += 1;
            }
        }
    }
    println!("{name}: den={den} pass={pass} fail={fail}");
    (pass, fail)
}

fn main() {
    // total_width = 6, signed, representable range -32..=31 (declared width
    // container, wrap overflow policy per the law row); fraction_width in
    // 0..=5 as the law row states.
    let lo = -32i64;
    let hi = 31i64;

    println!("=== reading 1: ties toward positive infinity, floor(x + 1/2) ===");
    let mut total_pass_r1 = 0u64;
    let mut total_fail_r1 = 0u64;
    for fw in 0 ..= 5i64 {
        let den = 1i64 << fw;
        let (p, f) = check_equivariant(
            "half_up_toward_pos_inf",
            half_up_toward_pos_inf,
            den,
            lo,
            hi,
        );
        total_pass_r1 += p;
        total_fail_r1 += f;
    }
    println!("reading 1 totals: pass={total_pass_r1} fail={total_fail_r1}");

    println!("=== reading 2: ties away from zero ===");
    let mut total_pass_r2 = 0u64;
    let mut total_fail_r2 = 0u64;
    for fw in 0 ..= 5i64 {
        let den = 1i64 << fw;
        let (p, f) = check_equivariant(
            "half_up_away_from_zero",
            half_up_away_from_zero,
            den,
            lo,
            hi,
        );
        total_pass_r2 += p;
        total_fail_r2 += f;
    }
    println!("reading 2 totals: pass={total_pass_r2} fail={total_fail_r2}");

    println!("=== control: half_even, must fail per the law row's own `fails` region ===");
    let mut total_pass_c = 0u64;
    let mut total_fail_c = 0u64;
    for fw in 0 ..= 5i64 {
        let den = 1i64 << fw;
        let (p, f) = check_equivariant("half_even", half_even, den, lo, hi);
        total_pass_c += p;
        total_fail_c += f;
    }
    println!("control totals: pass={total_pass_c} fail={total_fail_c}");

    println!("=== divergence: do the two readings differ at every negative tie? ===");
    let mut checked = 0u64;
    let mut all_diverge = true;
    for fw in 0 ..= 5i64 {
        let den = 1i64 << fw;
        if den < 2 {
            continue; // need a half-step to have a tie at all
        }
        let half = den / 2;
        if half * 2 != den {
            continue; // odd denominator has no representable half-step tie
        }
        for slot in lo .. hi {
            let num = slot * den + half; // exact tie between slot and slot+1
            if num < 0 {
                let r1 = half_up_toward_pos_inf(num, den);
                let r2 = half_up_away_from_zero(num, den);
                checked += 1;
                if r1 == r2 {
                    all_diverge = false;
                    println!("  agreement at a negative tie: num={num} den={den} r1={r1} r2={r2}");
                }
            }
        }
    }
    println!("negative ties checked={checked} all_diverge={all_diverge}");

    assert!(
        total_fail_r1 == 0,
        "reading 1 must be exactly translation equivariant"
    );
    assert!(
        total_fail_r2 > 0,
        "reading 2 must NOT be exactly translation equivariant"
    );
    assert!(
        total_fail_c > 0,
        "control (half_even) must fail, or the harness proves nothing"
    );
    assert!(
        all_diverge,
        "the two readings must differ at every negative tie"
    );
    println!("ALL ASSERTIONS PASSED");
}
