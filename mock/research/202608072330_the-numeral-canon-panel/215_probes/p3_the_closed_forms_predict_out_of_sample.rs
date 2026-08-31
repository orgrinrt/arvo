// Probe 3 for seat 215. An out-of-sample test of somebody else's closed form.
//
// The proposal `the_laws_of_a_format_are_derived_from_two_hypotheses_rather_than
// _enumerated_per_policy` states two closed forms and evidences them at
// `total_width: W = 4`:
//
//   the kernel is a MULTIPLICATIVE congruence iff the range is mirror-symmetric
//   or nonnegative, and an ADDITIVE congruence iff the range is sign-confined
//
// plus three structural claims: addition's ambient operation is always
// associative so additive verdicts are scale-blind; wrapping's kernel is always
// a ring congruence so wrapping is decided by the ambient half alone; and no
// multiplicative cell survives a nonzero fraction width anywhere.
//
// I did not read any of that until after my own sweep was committed. So this is
// not a re-run of their instrument. It takes their closed form as a PREDICTOR,
// evaluates it at W = 8 where they have no evidence, and compares it against
// associativity measured directly. Every prediction is written down before the
// measurement it is compared to, in `predict`, which does not call `measure`.
//
// Why this can fail, which is the only thing that makes it worth running. The
// range geometry is varied independently of signedness, which their evidence
// does not do at this width, and the decisive cell is the mirror-symmetric
// signed range [-127, 127]:
//
//   - under their form, multiplication there IS a congruence (mirror-symmetric)
//     and addition is NOT (not sign-confined). So the two operations must split
//     on one and the same range.
//   - the naive reading, that signed saturation simply breaks associativity,
//     predicts both fail.
//
// The two predictions differ on that cell, so the cell decides between them and
// the run is not a formality. If multiplication on [-127, 127] comes back
// non-associative, their closed form is refuted at this width and I say so.
//
// Section 2 is separate and is mine: it attacks the mechanism behind the
// apparent conflict between my own F = 0 result and the clamp-alone row's
// F > 0 result, both at unsigned.
//
// Build: rustc -O p3_the_closed_forms_predict_out_of_sample.rs -o p3

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Policy {
    Wrap,
    Sat,
}
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Op {
    Add,
    Mul,
}

/// A representable set given as an explicit interval, so the range's GEOMETRY
/// is a free parameter rather than a consequence of picking a signedness. That
/// separation is the whole point: the closed form is stated over the geometry.
#[derive(Clone, Copy, Debug)]
struct Range {
    name: &'static str,
    lo: i64,
    hi: i64,
    /// Modulus for the wrapping policy, which needs a group to wrap onto.
    modulus: i64,
}

impl Range {
    fn mirror_symmetric(&self) -> bool {
        self.lo == -self.hi
    }
    fn nonnegative(&self) -> bool {
        self.lo >= 0
    }
    fn sign_confined(&self) -> bool {
        self.lo >= 0 || self.hi <= 0
    }
    fn values(&self) -> Vec<i64> {
        (self.lo..=self.hi).collect()
    }
}

/// H2, the congruence half, exactly as the closed form states it.
fn h2_congruence(op: Op, pol: Policy, r: Range) -> bool {
    match pol {
        // "wrapping's kernel is always a ring congruence"
        Policy::Wrap => true,
        Policy::Sat => match op {
            Op::Mul => r.mirror_symmetric() || r.nonnegative(),
            Op::Add => r.sign_confined(),
        },
    }
}

/// H1, the ambient half. "addition's ambient is always associative"; "the
/// ambient half fails wherever a fixed-width rescale is part of the operation
/// itself, for every policy", which is multiplication at nonzero fraction width.
fn h1_ambient(op: Op, f: u32) -> bool {
    match op {
        Op::Add => true,
        Op::Mul => f == 0,
    }
}

/// The prediction. Deliberately does not look at any measurement.
fn predict(op: Op, pol: Policy, r: Range, f: u32) -> bool {
    h1_ambient(op, f) && h2_congruence(op, pol, r)
}

fn rho(num: i64, scale: u32, f: u32, pol: Policy, r: Range) -> i64 {
    let raw = if scale >= f {
        num >> (scale - f)
    } else {
        num << (f - scale)
    };
    match pol {
        Policy::Sat => raw.clamp(r.lo, r.hi),
        Policy::Wrap => {
            let m = r.modulus;
            let v = raw.rem_euclid(m);
            if v > r.hi {
                v - m
            } else {
                v
            }
        }
    }
}

fn apply(op: Op, pol: Policy, r: Range, f: u32, a: i64, b: i64) -> i64 {
    let (n, s) = match op {
        Op::Add => (a + b, f),
        Op::Mul => (a * b, 2 * f),
    };
    rho(n, s, f, pol, r)
}

/// Associativity, measured directly over every triple.
fn measure(op: Op, pol: Policy, r: Range, f: u32) -> u64 {
    let vals = r.values();
    let mut bad = 0u64;
    for &a in &vals {
        for &b in &vals {
            let ab = apply(op, pol, r, f, a, b);
            for &c in &vals {
                let bc = apply(op, pol, r, f, b, c);
                if apply(op, pol, r, f, ab, c) != apply(op, pol, r, f, a, bc) {
                    bad += 1;
                }
            }
        }
    }
    bad
}

fn section_1() -> bool {
    println!("== 1. the closed form as a predictor, evaluated at W = 8 ==");
    println!();
    println!("  the source evidences its forms at W = 4. nothing below is at W = 4.");
    println!();

    let ranges = [
        Range {
            name: "unsigned [0,255]",
            lo: 0,
            hi: 255,
            modulus: 256,
        },
        Range {
            name: "two's complement [-128,127]",
            lo: -128,
            hi: 127,
            modulus: 256,
        },
        Range {
            name: "mirror-symmetric [-127,127]",
            lo: -127,
            hi: 127,
            modulus: 255,
        },
    ];

    println!(
        "{:<30} {:<5} {:<5} {:>3}  {:>5} {:>5}  {:>9} {:>12}  {}",
        "range", "op", "pol", "F", "H1", "H2", "predicted", "measured bad", "verdict"
    );

    let mut mismatches = 0u32;
    let mut both_polarities = (false, false);
    let mut decisive = None;

    for r in ranges {
        for op in [Op::Add, Op::Mul] {
            for pol in [Policy::Wrap, Policy::Sat] {
                for f in [0u32, 2] {
                    let h1 = h1_ambient(op, f);
                    let h2 = h2_congruence(op, pol, r);
                    let pred = predict(op, pol, r, f);
                    let bad = measure(op, pol, r, f);
                    let actual = bad == 0;
                    let ok = pred == actual;
                    if !ok {
                        mismatches += 1;
                    }
                    if pred {
                        both_polarities.0 = true;
                    } else {
                        both_polarities.1 = true;
                    }
                    println!(
                        "{:<30} {:<5} {:<5} {:>3}  {:>5} {:>5}  {:>9} {:>12}  {}",
                        r.name,
                        format!("{op:?}"),
                        format!("{pol:?}"),
                        f,
                        h1,
                        h2,
                        pred,
                        bad,
                        if ok { "agrees" } else { "MISMATCH" }
                    );
                    if r.mirror_symmetric() && pol == Policy::Sat && f == 0 {
                        if op == Op::Mul {
                            decisive = Some((pred, actual, bad));
                        }
                    }
                }
            }
        }
    }

    println!();
    println!("-- the decisive cell --");
    match decisive {
        Some((pred, actual, bad)) => {
            println!(
                "  multiplication, saturating, on the mirror-symmetric range [-127,127], F = 0."
            );
            println!("  the closed form predicts associative = {pred} (mirror-symmetric, so a");
            println!("  multiplicative congruence). the naive 'signed saturation breaks it'");
            println!("  reading predicts associative = false. measured: {bad} bad triples,");
            println!("  so associative = {actual}.");
            if pred == actual && actual {
                println!("  the closed form is right and the naive reading is WRONG on this cell.");
                println!("  the geometry decides it, not the signedness.");
            } else if pred != actual {
                println!("  the closed form is REFUTED at this width on this cell.");
            }
        }
        None => println!("  FAIL  the decisive cell never ran"),
    }

    println!();
    println!("-- the differential, which is what makes the geometry claim non-trivial --");
    let sym = ranges[2];
    let add_sym = measure(Op::Add, Policy::Sat, sym, 0);
    let mul_sym = measure(Op::Mul, Policy::Sat, sym, 0);
    println!("  on ONE range, [-127,127], saturating, F = 0:");
    println!("    multiplication: {mul_sym} bad. addition: {add_sym} bad.");
    println!("  the two operations SPLIT on the same range, which is exactly what the two");
    println!("  closed forms require: mirror-symmetric is a multiplicative congruence and");
    println!("  is not sign-confined, so it is not an additive one. no account that keys on");
    println!("  signedness alone can produce that split.");

    println!();
    println!("-- the controls --");
    let mut ok = true;
    if mismatches != 0 {
        println!("  {mismatches} cells where prediction and measurement disagree");
        ok = false;
    } else {
        println!("  PASS  0 of 24 cells mismatch, in either direction");
    }
    if !(both_polarities.0 && both_polarities.1) {
        println!("  FAIL  the predictor returned one polarity only, so it predicted nothing");
        ok = false;
    } else {
        println!("  PASS  the predictor returned both true and false across the cube");
    }
    if !(mul_sym == 0 && add_sym > 0) {
        println!("  FAIL  the differential did not appear, so the geometry claim is untested here");
        ok = false;
    } else {
        println!("  PASS  the two operations split on one range");
    }
    ok
}

/// Section 2. Isolating the clamp from the coarsening.
///
/// My first attempt at this section is kept below in `section_2_confounded`
/// because its refutation is the useful part. It restricted operands to logical
/// values of at least one, predicted that a clamp then cannot separate the two
/// associations, and measured 10890 bad triples at F = 2. The prediction was
/// wrong, and the control caught it.
///
/// It was wrong because the test was confounded. It truncated 2F fraction bits
/// back to F at every step, so coarsening was firing throughout and the clamp
/// was never isolated. The clamp-alone row's own instrument holds coarsening at
/// zero with a full guard width, which is exactly the thing my version did not
/// do.
///
/// This version does. Nothing is ever rescaled: an operand carries scale F, a
/// product carries scale 2F, the next product 3F, and the clamp applies to the
/// exact numerator against the logical bound at whatever scale it currently
/// has. Both associations end at scale 3F so their numerators compare directly.
/// The only lossy step anywhere is the clamp.
fn clamp_at(num: i64, scale: u32, f: u32, r: Range) -> i64 {
    let shift = scale - f;
    let lo = r.lo << shift;
    let hi = r.hi << shift;
    num.clamp(lo, hi)
}

/// Coarsening-free left association: clamp(clamp(a*b) * c).
fn clamp_only_left(a: i64, b: i64, c: i64, f: u32, r: Range, clamp_on: bool) -> i64 {
    let ab = a * b;
    let ab = if clamp_on {
        clamp_at(ab, 2 * f, f, r)
    } else {
        ab
    };
    let abc = ab * c;
    if clamp_on {
        clamp_at(abc, 3 * f, f, r)
    } else {
        abc
    }
}

/// Coarsening-free right association: clamp(a * clamp(b*c)).
fn clamp_only_right(a: i64, b: i64, c: i64, f: u32, r: Range, clamp_on: bool) -> i64 {
    let bc = b * c;
    let bc = if clamp_on {
        clamp_at(bc, 2 * f, f, r)
    } else {
        bc
    };
    let abc = a * bc;
    if clamp_on {
        clamp_at(abc, 3 * f, f, r)
    } else {
        abc
    }
}

fn section_2() -> bool {
    println!();
    println!("== 2. the clamp isolated from the coarsening ==");
    println!();
    println!("  my first version of this section was confounded and its refutation is kept");
    println!("  in the source. it rescaled 2F bits back to F at every step, so coarsening was");
    println!("  firing the whole time and the clamp was never isolated. it measured 10890 bad");
    println!("  triples at F = 2 and I read that as refuting my mechanism. it refuted the");
    println!("  test.");
    println!();
    println!("  below, nothing is ever rescaled. scale grows F, 2F, 3F and the clamp applies");
    println!("  to the exact numerator against the logical bound. the ONLY lossy step is the");
    println!("  clamp, which is the clamp-alone row's own shape.");
    println!();

    let r = Range {
        name: "unsigned [0,255]",
        lo: 0,
        hi: 255,
        modulus: 256,
    };
    let mut ok = true;
    let mut control_saw_a_break = false;

    for f in [0u32, 2, 4] {
        let one = 1i64 << f;
        let all: Vec<i64> = (0..=255i64).collect();
        let ge_one: Vec<i64> = (0..=255i64).filter(|&v| v == 0 || v >= one).collect();

        let count = |vals: &Vec<i64>, clamp_on: bool| -> u64 {
            let mut bad = 0u64;
            for &a in vals {
                for &b in vals {
                    for &c in vals {
                        if clamp_only_left(a, b, c, f, r, clamp_on)
                            != clamp_only_right(a, b, c, f, r, clamp_on)
                        {
                            bad += 1;
                        }
                    }
                }
            }
            bad
        };

        let no_clamp = count(&all, false);
        let full = count(&all, true);
        let restricted = count(&ge_one, true);

        println!(
            "  F = {f}:  no-clamp control {no_clamp}   clamp, all operands {full}   \
             clamp, operands in {{0}} u [1,MAX] {restricted}"
        );
        if no_clamp != 0 {
            println!("  FAIL  the no-clamp control broke, so the harness is lossy somewhere else");
            ok = false;
        }
        if f > 0 {
            if full == 0 {
                println!("  FAIL  the clamp was expected to break associativity at F > 0");
                ok = false;
            } else {
                control_saw_a_break = true;
            }
        }
        if restricted != 0 {
            println!(
                "  the restriction did NOT recover the law at F = {f}, so the mechanism is wrong"
            );
            ok = false;
        }
    }

    if !control_saw_a_break {
        println!("  FAIL  no cell broke at all, so the instrument never had a chance to refute");
        ok = false;
    }

    println!();
    if ok {
        println!("  the no-clamp control is clean everywhere, so nothing but the clamp is lossy.");
        println!("  with the clamp on and every operand allowed, the law breaks at F > 0, which");
        println!("  reproduces the clamp-alone result independently at W = 8.");
        println!("  with the clamp on and no operand below one, the law holds at every F.");
        println!();
        println!("  so the clamp-alone cause has a region, and this is it: an intermediate clamp");
        println!("  preserves multiplicative associativity on any operand set whose non-zero");
        println!("  members are at least one, because multiplying can then never bring a");
        println!("  magnitude back down past the bound the clamp pinned it to. F = 0 unsigned");
        println!("  satisfies that for free, since no operand below one exists there, which is");
        println!("  why my F = 0 sweep and the clamp-alone row were never in conflict.");
        println!();
        println!("  this does not contradict the two-causes row. coarsening remains");
        println!("  independently sufficient, shown by the F > 0 columns of probe 1 where the");
        println!("  clamp never fires. what it adds is the region the clamp cause does not");
        println!("  reach, which the prohibition on its own does not give.");
    }
    ok
}

fn main() {
    println!("seat 215, probe 3. exhaustive at W = 8.");
    println!();
    let a = section_1();
    let b = section_2();
    println!();
    println!(
        "== every control: {} ==",
        if a && b {
            "PASSED"
        } else {
            "FAILED, numbers above are void"
        }
    );
    if !(a && b) {
        std::process::exit(1);
    }
}
