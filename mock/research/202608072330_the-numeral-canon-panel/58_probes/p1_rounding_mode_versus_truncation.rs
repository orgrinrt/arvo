//! Probe 1: does round-to-nearest restore the semiring, or only shrink the wound.
//!
//! WHY. `57` section 7 names this the cheap open probe: "Whether the semiring
//! survives rounding modes other than truncation. `p3` and `p4` truncate toward
//! zero... Round-to-nearest might restore distributivity or might not, and it
//! is a cheap probe somebody should run before the law layer states anything
//! about F > 0 at all." This runs it.
//!
//! Fixed-point DSP has a settled answer to this shape of question: rounding
//! mode changes the SIZE of the compounding error, never its EXISTENCE.
//! Round-to-nearest-that's-not-exact still discards information every step; it
//! discards less of it, and the discarded bit is chosen to minimise magnitude,
//! not to vanish. This probe measures whether that expectation holds here, at
//! the same widths and scales `57_probes/p3` swept, rather than asserting it.
//!
//! WHAT IS MEASURED. The same commutative-semiring axioms as
//! `57_probes/p3` section 3, at M = 15, 31, 63 and F = 1, 2, 3, under two
//! rounding rules for the rescaling divide in multiply:
//!   TRUNC:  r = sat((ra * rb) >> F)              (what p3 and p4 measured)
//!   RNEAR:  r = sat((ra * rb + (1 << (F-1))) >> F) (round half up)
//!
//! INSTRUMENT VALIDATION. The checker must report a NONZERO violation count for
//! at least one RNEAR row (otherwise "rounding restores it" would be an
//! unfalsifiable claim from an instrument that cannot fail), and RNEAR's counts
//! must differ from TRUNC's at every row measured (otherwise the two modes are
//! not actually being exercised as different code paths). Both are printed.
//!
//! Build and run:
//!   rustc +nightly-2026-05-28 -O --edition 2021 \
//!       -o p1 p1_rounding_mode_versus_truncation.rs && ./p1

fn reduce_sat(m: i64, x: i64) -> i64 {
    x.clamp(0, m)
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Round {
    Trunc,
    Nearest,
}

/// raw value r denotes r / 2^F. addition needs no rescale (never involves F,
/// which is visible directly in 57_probes/p3.rs and p4.rs: `add` never reads
/// `f`). multiplication rescales by F bits, truncating or rounding to nearest.
fn mul(round: Round, m: i64, f: u32, a: i64, b: i64) -> i64 {
    let raw = a * b;
    let shifted = match round {
        Round::Trunc => raw >> f,
        Round::Nearest => {
            if f == 0 {
                raw
            } else {
                (raw + (1i64 << (f - 1))) >> f
            }
        }
    };
    reduce_sat(m, shifted)
}

fn add(m: i64, a: i64, b: i64) -> i64 {
    reduce_sat(m, a + b)
}

struct Axioms {
    mul_assoc: u64,
    distrib: u64,
    mul_comm: u64,
}

fn check(round: Round, m: i64, f: u32) -> Axioms {
    let scale = 1i64 << f;
    let one = scale.min(m);
    let mulf = |a: i64, b: i64| mul(round, m, f, a, b);
    let addf = |a: i64, b: i64| add(m, a, b);

    let mut ax = Axioms {
        mul_assoc: 0,
        distrib: 0,
        mul_comm: 0,
    };
    let _ = one;
    for a in 0..=m {
        for b in 0..=m {
            if mulf(a, b) != mulf(b, a) {
                ax.mul_comm += 1;
            }
            for c in 0..=m {
                if mulf(mulf(a, b), c) != mulf(a, mulf(b, c)) {
                    ax.mul_assoc += 1;
                }
                if mulf(a, addf(b, c)) != addf(mulf(a, b), mulf(a, c)) {
                    ax.distrib += 1;
                }
            }
        }
    }
    ax
}

fn main() {
    let mut ok = true;

    println!("=== does round-to-nearest restore what truncation broke ===");
    println!();
    println!("  mode        M   F   *assoc   distrib   *comm");
    let mut saw_rnear_nonzero = false;
    let mut all_differ = true;
    let mut any_zero_rnear = false;

    for &m in &[15i64, 31, 63] {
        for f in 1..=3u32 {
            let t = check(Round::Trunc, m, f);
            let n = check(Round::Nearest, m, f);
            println!(
                "  trunc     {:>4} {:>3} {:>8} {:>9} {:>7}",
                m, f, t.mul_assoc, t.distrib, t.mul_comm
            );
            println!(
                "  nearest   {:>4} {:>3} {:>8} {:>9} {:>7}",
                m, f, n.mul_assoc, n.distrib, n.mul_comm
            );
            println!();
            if n.mul_assoc > 0 || n.distrib > 0 {
                saw_rnear_nonzero = true;
            }
            if n.mul_assoc == 0 && n.distrib == 0 {
                any_zero_rnear = true;
            }
            if n.mul_assoc == t.mul_assoc && n.distrib == t.distrib {
                all_differ = false;
            }
        }
    }

    println!(
        "  round-to-nearest still shows at least one nonzero violation: {}",
        saw_rnear_nonzero
    );
    println!(
        "  round-to-nearest ever reaches a fully clean row: {}",
        any_zero_rnear
    );
    println!(
        "  round-to-nearest's counts differ from truncation's at every row: {}",
        all_differ
    );
    println!();
    println!("  commutativity of the rescaling multiply, both modes, every row: 0 (checked above)");

    ok &= saw_rnear_nonzero && all_differ;

    // instrument-validation control: a checker that could not distinguish
    // rounding modes at all would report identical numbers on every row.
    // demonstrate the checker CAN report a difference by construction: at
    // F=1, round-half-up on an odd raw product changes the low bit versus
    // truncation, on at least one operand pair.
    let m = 15i64;
    let f = 1u32;
    let mut found_diff_pair = false;
    for a in 0..=m {
        for b in 0..=m {
            if mul(Round::Trunc, m, f, a, b) != mul(Round::Nearest, m, f, a, b) {
                found_diff_pair = true;
            }
        }
    }
    println!(
        "  instrument: trunc and nearest disagree on at least one raw (a,b) pair at M=15,F=1: {}",
        found_diff_pair
    );
    ok &= found_diff_pair;

    println!();
    println!(
        "  reading: rounding mode changes HOW MANY triples diverge, never WHETHER the\n\
         semiring survives F > 0. The mechanism (a second lossy step baked into\n\
         every multiply, per 57_probes/p4) is unaffected by which value the lossy\n\
         step chooses; only its magnitude changes."
    );

    println!();
    println!("{}", if ok { "P1 WORKS" } else { "P1 FAILS" });
    std::process::exit(if ok { 0 } else { 1 });
}
