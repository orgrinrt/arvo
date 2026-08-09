//! PROBE 3: what each arithmetic actually LICENSES, computed rather than
//! inherited.
//!
//! Files 13, 14 and 15 each computed part of this grid, from three different
//! angles, at three different model widths, with three different generator
//! sets. This recomputes all of it in one place, at one width, over one
//! generator vocabulary, because the claim my file makes (that the atomic law
//! facts ARE the guards on individual rewrite generators) is only worth
//! anything if the generators, not the law names, are the columns.
//!
//! A "generator" here is one family of source-level rewrites a combinator
//! might want to perform:
//!
//!   REGROUP     ((a.b).c) -> (a.(b.c))          chunked fold, n-way accumulator
//!   COMMUTE     (a.b)     -> (b.a)              head+tail with a reversed tail
//!   NEUTRAL     (a.0)     -> a                  skipping unwritten slots
//!   HOIST_MAX   a+max(b,c)-> max(a+b,a+c)       max-plus recurrence
//!   HOIST_MIN   a+min(b,c)-> min(a+b,a+c)       min-plus recurrence
//!   MUL_REGROUP ((a*b)*c) -> (a*(b*c))          reassociating a product chain
//!   DISTRIB     a*(b+c)   -> a*b+a*c            factor / expand
//!
//! A generator is IN a composition's supply set exactly when applying it never
//! changes the answer. That is checked here by exhaustion over the whole
//! representable set, under Kleene equality for the partial (refusing) rows.
//!
//! Model: 4-bit, signed [-8,7] and unsigned [0,15]. Integer rows use FRAC=0
//! so no rounding fires; the multiply rows are repeated at FRAC=2 (Q2.2,
//! matching file 15's model) so the always-firing quantisation of
//! `u_mul_fixed`/`i_mul_fixed` is in play.
//!
//! Build:  rustc -O 03_supply_table.rs && ./03_supply_table

// ---------------------------------------------------------------- arithmetic

/// The recovery rules, as partial maps from an exact result onto the
/// representable set. `None` is refusal, which is the absence of a returned
/// value rather than a value.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum Rule {
    Wrap,
    Saturate,
    Refuse,
    Exact,
}

fn recover(r: Rule, x: i64, lo: i64, hi: i64) -> Option<i64> {
    let span = hi - lo + 1;
    match r {
        Rule::Exact => Some(x),
        Rule::Wrap => Some(lo + (((x - lo) % span) + span) % span),
        Rule::Saturate => Some(if x < lo {
            lo
        } else if x > hi {
            hi
        } else {
            x
        }),
        Rule::Refuse => {
            if x < lo || x > hi {
                None
            } else {
                Some(x)
            }
        }
    }
}

fn add(r: Rule, a: i64, b: i64, lo: i64, hi: i64) -> Option<i64> {
    recover(r, a + b, lo, hi)
}

/// Fixed-point multiply exactly as shipped: full product, then an
/// unconditional arithmetic shift by FRAC (a floor, no rounding), then the
/// same range recovery every other operation gets.
/// `arith_macros.rs:33-34` (wrapping) and `:95-101` (saturating).
fn mul(r: Rule, a: i64, b: i64, frac: u32, lo: i64, hi: i64) -> Option<i64> {
    recover(r, (a * b) >> frac, lo, hi)
}

/// Kleene equality: both refuse, or both return and agree.
fn keq(x: Option<i64>, y: Option<i64>) -> bool {
    x == y
}

/// Kleene max / min, needed for the hoisting generators on a partial rule:
/// selecting between a refusal and a value has no defined answer, so the
/// selection refuses.
fn kmax(x: Option<i64>, y: Option<i64>) -> Option<i64> {
    match (x, y) {
        (Some(a), Some(b)) => Some(a.max(b)),
        _ => None,
    }
}
fn kmin(x: Option<i64>, y: Option<i64>) -> Option<i64> {
    match (x, y) {
        (Some(a), Some(b)) => Some(a.min(b)),
        _ => None,
    }
}

// ---------------------------------------------------------------- generators

struct Row {
    name: &'static str,
    lic: [bool; 7],
    first_fail: [Option<(i64, i64, i64)>; 7],
}

const GEN: [&str; 7] = [
    "REGROUP",
    "COMMUTE",
    "NEUTRAL",
    "HOIST_MAX",
    "HOIST_MIN",
    "MUL_REGROUP",
    "DISTRIB",
];

fn supply(r: Rule, lo: i64, hi: i64, frac: u32) -> Row {
    let mut lic = [true; 7];
    let mut first_fail: [Option<(i64, i64, i64)>; 7] = [None; 7];

    let mut note = |i: usize,
                    ok: bool,
                    w: (i64, i64, i64),
                    lic: &mut [bool; 7],
                    ff: &mut [Option<(i64, i64, i64)>; 7]| {
        if !ok && lic[i] {
            lic[i] = false;
            ff[i] = Some(w);
        }
    };

    for a in lo..=hi {
        for b in lo..=hi {
            // COMMUTE
            let ok = keq(add(r, a, b, lo, hi), add(r, b, a, lo, hi));
            note(1, ok, (a, b, 0), &mut lic, &mut first_fail);

            // NEUTRAL (b ignored, checked once per a but cheap to leave here)
            let ok = keq(add(r, a, 0, lo, hi), Some(a));
            note(2, ok, (a, 0, 0), &mut lic, &mut first_fail);

            for c in lo..=hi {
                // REGROUP: (a+b)+c  vs  a+(b+c), both sides through the rule
                let l = match add(r, a, b, lo, hi) {
                    Some(ab) => add(r, ab, c, lo, hi),
                    None => None,
                };
                let rr = match add(r, b, c, lo, hi) {
                    Some(bc) => add(r, a, bc, lo, hi),
                    None => None,
                };
                note(0, keq(l, rr), (a, b, c), &mut lic, &mut first_fail);

                // HOIST_MAX: a + max(b,c) vs max(a+b, a+c)
                let l = add(r, a, b.max(c), lo, hi);
                let rr = kmax(add(r, a, b, lo, hi), add(r, a, c, lo, hi));
                note(3, keq(l, rr), (a, b, c), &mut lic, &mut first_fail);

                // HOIST_MIN
                let l = add(r, a, b.min(c), lo, hi);
                let rr = kmin(add(r, a, b, lo, hi), add(r, a, c, lo, hi));
                note(4, keq(l, rr), (a, b, c), &mut lic, &mut first_fail);

                // MUL_REGROUP
                let l = match mul(r, a, b, frac, lo, hi) {
                    Some(ab) => mul(r, ab, c, frac, lo, hi),
                    None => None,
                };
                let rr = match mul(r, b, c, frac, lo, hi) {
                    Some(bc) => mul(r, a, bc, frac, lo, hi),
                    None => None,
                };
                note(5, keq(l, rr), (a, b, c), &mut lic, &mut first_fail);

                // DISTRIB: a*(b+c) vs a*b + a*c
                let l = match add(r, b, c, lo, hi) {
                    Some(bc) => mul(r, a, bc, frac, lo, hi),
                    None => None,
                };
                let rr = match (mul(r, a, b, frac, lo, hi), mul(r, a, c, frac, lo, hi)) {
                    (Some(x), Some(y)) => add(r, x, y, lo, hi),
                    _ => None,
                };
                note(6, keq(l, rr), (a, b, c), &mut lic, &mut first_fail);
            }
        }
    }

    Row {
        name: match r {
            Rule::Wrap => "Wrap      (Hot)",
            Rule::Saturate => "Saturate  (Precise-shape)",
            Rule::Refuse => "Refuse    (Precise as specced)",
            Rule::Exact => "Exact     (unbounded)",
        },
        lic,
        first_fail,
    }
}

fn print_block(title: &str, lo: i64, hi: i64, frac: u32) {
    println!("\n=== {title}   range [{lo},{hi}], FRAC={frac} ===");
    print!("{:<32}", "rule");
    for g in GEN {
        print!("{:>12}", g);
    }
    println!();
    let rows = [
        supply(Rule::Wrap, lo, hi, frac),
        supply(Rule::Saturate, lo, hi, frac),
        supply(Rule::Refuse, lo, hi, frac),
        supply(Rule::Exact, lo, hi, frac),
    ];
    for row in &rows {
        print!("{:<32}", row.name);
        for i in 0..7 {
            print!("{:>12}", if row.lic[i] { "yes" } else { "NO" });
        }
        println!();
    }
    println!("  first counterexample per refused generator:");
    for row in &rows {
        for i in 0..7 {
            if let Some((a, b, c)) = row.first_fail[i] {
                println!(
                    "    {:<32} {:<12} at (a={a}, b={b}, c={c})",
                    row.name, GEN[i]
                );
            }
        }
    }
}

fn main() {
    // Integer rows: FRAC = 0, so no quantisation fires on the multiply and the
    // only law failures are range-recovery failures.
    print_block("SIGNED, integer", -8, 7, 0);
    print_block("UNSIGNED, integer", 0, 15, 0);

    // Fixed-point rows: FRAC = 2 (Q2.2), so `mul` quantises on EVERY call
    // whether or not anything left the range. This is the second, independent
    // source of law failure that addition does not have.
    print_block("SIGNED, Q2.2", -8, 7, 2);
    print_block("UNSIGNED, Q2.2", 0, 15, 2);
}
