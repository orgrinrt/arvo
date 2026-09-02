// Probe 01: does the SHIPPED fixed-point multiply distribute over the
// SHIPPED addition, per strategy? Neither file 13 nor file 14 tested
// multiplication; both flagged it as entirely untested (draft 5.2,
// "expected to be where the work first gets genuinely hard";
// 14_dolan..md section 8, "run the exhaustive small-model probe first").
// This models `u_mul_fixed`/`i_mul_fixed` from
// `arvo-strategy/src/arith_macros.rs` exactly as shipped: an unconditional
// `(a.wrapping_mul(b)) >> FRAC` (truncating, no round-to-nearest, no
// ties-to-even) for the wrapping macro, and the identical shift-then-clamp
// for the saturating macro (`arith_macros.rs:94-101`, `213-220`). This is
// deliberately NOT the draft's stated preset table (11_current_shape_draft.md:325-333,
// "in-range: nearest, ties to even" for Warm/Cold/Precise): that table is
// the round's stated INTENT for a redefinition not yet shipped (draft 3.5's
// own preamble, "each of the four is redefined from what its name states
// as intent... rather than from what it happens to do today"). This probe
// tests today's actual code, because that is what a distributivity claim
// would be a claim about if made right now.
//
// Model: signed fixed point, FRAC=2 fractional bits, 4-bit raw range
// [-8,7] (representable reals: raw/4.0, i.e. [-2.0, 1.75]). Recovery maps
// mirror the earlier probes: Wrap (mod 16 into [-8,7]), Saturate (clamp).
// `wide` is i64 throughout so the intermediate multiply never overflows
// the *model*; the shipped code widens for real via u128/i128 (arith_macros.rs:256-541).
//
// Tests: a * (b + c) == a*b + a*c (distributivity), and (a*b)*c == a*(b*c)
// (associativity of the shipped multiply itself), exhaustively over the
// 16^3 = 4096 triples, plus the same DP-shaped check file 13/14 used
// (does regrouping a chain of two `*` applications, mixed with `+`,
// change the answer).
//
// Run: rustc -O 01_distributivity_over_add_shipped_mul.rs -o /tmp/distrib && /tmp/distrib

const FRAC: i64 = 2;
const LO: i64 = -8;
const HI: i64 = 7;
const RANGE: i64 = HI - LO + 1; // 16

fn wrap(x: i64) -> i64 {
    let m = ((x - LO) % RANGE + RANGE) % RANGE;
    m + LO
}

fn saturate(x: i64) -> i64 {
    if x < LO {
        LO
    } else if x > HI {
        HI
    } else {
        x
    }
}

// shipped Hot/Warm/Cold add: raw + raw, then wrap.
fn wrap_add(a: i64, b: i64) -> i64 {
    wrap(a + b)
}

// shipped Precise add: raw + raw, then saturate (arith_macros.rs:66-71).
fn sat_add(a: i64, b: i64) -> i64 {
    saturate(a + b)
}

// shipped Hot/Warm/Cold u_mul_fixed/i_mul_fixed: `a.wrapping_mul(b) >> FRAC`,
// arith_macros.rs:34, 148 (unconditional truncating shift, no rounding),
// then the OUTER result is representable-range wrapped like every other op
// (the shift itself can still land outside [LO,HI] before the container's
// own wraparound is applied on store/use; matching that, we wrap the final
// value the same way `u_add`/`i_add` do, since wrapping strategies wrap
// every op result to the container, not just add).
fn wrap_mul_fixed(a: i64, b: i64) -> i64 {
    let raw = a.wrapping_mul(b);
    wrap(raw >> FRAC)
}

// shipped Precise i_mul_fixed: `(a.wrapping_mul(b) >> FRAC)` then clamp to
// the logical hi/lo (arith_macros.rs:213-221, no rounding: same shift as
// the wrapping macro, only the post-shift clamp differs).
fn sat_mul_fixed(a: i64, b: i64) -> i64 {
    let raw = a.wrapping_mul(b);
    saturate(raw >> FRAC)
}

struct Arith {
    name: &'static str,
    add: fn(i64, i64) -> i64,
    mul: fn(i64, i64) -> i64,
}

fn check_distributes(ar: &Arith) {
    let mut ok = true;
    let mut ex: Option<(i64, i64, i64, i64, i64)> = None;
    for a in LO..=HI {
        for b in LO..=HI {
            for c in LO..=HI {
                let lhs = (ar.mul)(a, (ar.add)(b, c));
                let rhs = (ar.add)((ar.mul)(a, b), (ar.mul)(a, c));
                if lhs != rhs {
                    ok = false;
                    ex = Some((a, b, c, lhs, rhs));
                    break;
                }
            }
            if !ok {
                break;
            }
        }
        if !ok {
            break;
        }
    }
    match ex {
        None => println!("{}: a*(b+c) == a*b+a*c: yes", ar.name),
        Some((a, b, c, lhs, rhs)) => println!(
            "{}: a*(b+c) == a*b+a*c: NO at (a={a},b={b},c={c}): lhs={lhs} rhs={rhs}",
            ar.name
        ),
    }
}

fn check_mul_assoc(ar: &Arith) {
    let mut ok = true;
    let mut ex: Option<(i64, i64, i64, i64, i64)> = None;
    for a in LO..=HI {
        for b in LO..=HI {
            for c in LO..=HI {
                let lhs = (ar.mul)((ar.mul)(a, b), c);
                let rhs = (ar.mul)(a, (ar.mul)(b, c));
                if lhs != rhs {
                    ok = false;
                    ex = Some((a, b, c, lhs, rhs));
                    break;
                }
            }
            if !ok {
                break;
            }
        }
        if !ok {
            break;
        }
    }
    match ex {
        None => println!("{}: (a*b)*c == a*(b*c): yes", ar.name),
        Some((a, b, c, lhs, rhs)) => println!(
            "{}: (a*b)*c == a*(b*c): NO at (a={a},b={b},c={c}): lhs={lhs} rhs={rhs}",
            ar.name
        ),
    }
}

fn main() {
    println!(
        "model: signed Q{{4-FRAC}}.{{FRAC}}, FRAC={FRAC}, raw range [{LO},{HI}] (real values [{},{}])\n",
        LO as f64 / 4.0,
        HI as f64 / 4.0
    );

    let variants = [
        Arith {
            name: "Wrap (Hot)",
            add: wrap_add,
            mul: wrap_mul_fixed,
        },
        Arith {
            name: "Saturate (Warm/Cold/Precise shape)",
            add: sat_add,
            mul: sat_mul_fixed,
        },
    ];

    println!("distributivity of shipped * over shipped +:");
    for ar in &variants {
        check_distributes(ar);
    }
    println!("\nassociativity of shipped * alone:");
    for ar in &variants {
        check_mul_assoc(ar);
    }

    // A concrete three-term chain, `a*b + a*c` vs `a*(b+c)`, at the exact
    // counterexample style file 13/14 used for +: report the first
    // disagreement's real-valued interpretation too, so the size of the
    // error is visible, not just its existence.
    println!("\nfirst distributivity counterexample, decoded to real values (raw/4.0):");
    for ar in &variants {
        'search: for a in LO..=HI {
            for b in LO..=HI {
                for c in LO..=HI {
                    let lhs = (ar.mul)(a, (ar.add)(b, c));
                    let rhs = (ar.add)((ar.mul)(a, b), (ar.mul)(a, c));
                    if lhs != rhs {
                        println!(
                            "  {}: a={:.2} b={:.2} c={:.2} -> a*(b+c)={:.2} vs a*b+a*c={:.2}",
                            ar.name,
                            a as f64 / 4.0,
                            b as f64 / 4.0,
                            c as f64 / 4.0,
                            lhs as f64 / 4.0,
                            rhs as f64 / 4.0
                        );
                        break 'search;
                    }
                }
            }
        }
    }
}
