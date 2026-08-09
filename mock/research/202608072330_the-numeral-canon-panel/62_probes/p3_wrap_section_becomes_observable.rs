//! Probe p3: does the wrap representative section stay arithmetic-neutral
//! once the fraction axis is in play?
//!
//! WHY. `56` section 4 deflated the wrapping fork: given a declared Q,
//! wrap-as-domain and wrap-as-composite are observationally equivalent, and
//! the representative range (unsigned [0, 2^w - 1] against signed
//! [-2^(w-1), 2^(w-1) - 1]) is "chosen at declaration time... no cast of a
//! stored value consults a runtime policy". `55b` section 4 accepted it. Both
//! established it at F = 0, where the induced operations are the transported
//! ring operations of Z/2^w and the section genuinely is a relabelling
//! (`55_probes/p4`'s bare `a * b`; my p1 section 5 confirms the signed
//! section is the same ring).
//!
//! At F > 0 the induced multiply is wrap(rescale(a' * b')) where a' is the
//! REPRESENTATIVE. The rescale is not a map of residue classes: two
//! representatives of one class rescale to different classes. So the
//! HYPOTHESIS, stated before running with a hand witness:
//!
//!   at F > 0 the section choice is observable in the induced arithmetic.
//!   Two declarations of "the same" width-w wrapping numeral with different
//!   representative ranges compute different products of the same residue
//!   classes. Hand witness, w = 4, F = 1, floor spelling: classes 9 and 3.
//!   Unsigned section: 9 * 3 = 27, >> 1 = 13, class 13. Signed section:
//!   -7 * 3 = -21, >> 1 = -11, class -11 mod 16 = 5. Class 13 against
//!   class 5.
//!
//! At F = 0 the disagreement count must be zero (that is the transport fact
//! and the instrument's own validation); at F > 0 it is predicted nonzero
//! for both rescale spellings.
//!
//! WHAT THIS BEARS ON. Not the deflation itself: that compared two FILINGS
//! of one declared numeral, and it survives. What it bounds is the sentence
//! "the section is chosen at declaration and is a relabelling": at F > 0
//! the choice of section is a choice of arithmetic, so unsigned-wrap and
//! signed-wrap at one width are the same ring at F = 0 and two different
//! (both non-associative, per p2) magmas at F > 0.
//!
//! MUTANT CHECK. A deliberately misaligned class map (offset by one) must
//! report disagreements at F = 0, so a zero at F = 0 is a measurement rather
//! than a checker that cannot fire.
//!
//! Build and run:
//!   rustc +nightly-2026-05-28 -O --edition 2021 \
//!       -o p3 p3_wrap_section_becomes_observable.rs && ./p3

#[derive(Clone, Copy, PartialEq, Eq)]
enum Rescale {
    Trunc,
    Floor,
}

fn rescale(x: i64, f: u32, r: Rescale) -> i64 {
    match r {
        Rescale::Trunc => x / (1i64 << f),
        Rescale::Floor => x >> f,
    }
}

/// induced multiply on residue classes 0..m-1, computed through the given
/// representative section. `rep` maps a class to its representative; the
/// result is mapped back to a class by rem_euclid.
fn class_mul(ca: i64, cb: i64, m: i64, f: u32, r: Rescale, signed_section: bool) -> i64 {
    let rep = |c: i64| {
        if signed_section && c >= m / 2 {
            c - m
        } else {
            c
        }
    };
    rescale(rep(ca) * rep(cb), f, r).rem_euclid(m)
}

fn disagreements(w: u32, f: u32, r: Rescale) -> (u64, Option<(i64, i64, i64, i64)>) {
    let m = 1i64 << w;
    let mut d = 0u64;
    let mut wit = None;
    for ca in 0..m {
        for cb in 0..m {
            let u = class_mul(ca, cb, m, f, r, false);
            let s = class_mul(ca, cb, m, f, r, true);
            if u != s {
                d += 1;
                if wit.is_none() {
                    wit = Some((ca, cb, u, s));
                }
            }
        }
    }
    (d, wit)
}

fn main() {
    let mut ok = true;

    println!("=== section 1: F = 0, the section must be a relabelling (zero disagreements) ===");
    println!();
    let mut f0_all_zero = true;
    for w in [4u32, 5, 6] {
        for (rn, r) in [("trunc", Rescale::Trunc), ("floor", Rescale::Floor)] {
            let (d, _) = disagreements(w, 0, r);
            println!(
                "  w={} F=0 {}: {} disagreements over {} class pairs",
                w,
                rn,
                d,
                (1u64 << w) * (1u64 << w)
            );
            f0_all_zero &= d == 0;
        }
    }
    println!();
    println!(
        "  F = 0 transport fact reproduced (both sections one ring): {}",
        f0_all_zero
    );
    ok &= f0_all_zero;
    println!();

    println!("=== section 2: mutant check, a misaligned class map must fire at F = 0 ===");
    println!();
    {
        // a genuinely misaligned map: rep(c) = c - m + 1 for the upper half,
        // which is a representative of the WRONG class (off by one), unlike
        // this probe's first mutant attempt (rep(c) = c - m with a shifted
        // threshold), which is still congruent to c mod m and therefore not
        // a mutant at all. that first attempt reported zero and correctly
        // failed the run; it is kept in p3_output.v1_bad_mutant.txt.
        let m = 16i64;
        let mut d = 0u64;
        for ca in 0..m {
            for cb in 0..m {
                let rep_bad = |c: i64| if c >= m / 2 { c - m + 1 } else { c };
                let u = (ca * cb).rem_euclid(m);
                let s = (rep_bad(ca) * rep_bad(cb)).rem_euclid(m);
                if u != s {
                    d += 1;
                }
            }
        }
        println!(
            "  misaligned section at w=4 F=0: {} disagreements (must be > 0)",
            d
        );
        ok &= d > 0;
    }
    println!();

    println!("=== section 3: F > 0, the section becomes arithmetic ===");
    println!();
    let mut all_nonzero = true;
    for w in [4u32, 5, 6] {
        for f in 1..=3u32 {
            for (rn, r) in [("trunc", Rescale::Trunc), ("floor", Rescale::Floor)] {
                let (d, wit) = disagreements(w, f, r);
                let total = (1u64 << w) * (1u64 << w);
                println!(
                    "  w={} F={} {}: {} of {} class pairs disagree, witness {:?}",
                    w, f, rn, d, total, wit
                );
                all_nonzero &= d > 0;
            }
        }
    }
    println!();
    println!(
        "  every F > 0 configuration distinguishes the sections, both spellings: {}",
        all_nonzero
    );
    ok &= all_nonzero;
    // the hand witness from the header, checked explicitly
    let u = class_mul(9, 3, 16, 1, Rescale::Floor, false);
    let s = class_mul(9, 3, 16, 1, Rescale::Floor, true);
    println!(
        "  hand witness w=4 F=1 floor, classes (9, 3): unsigned section -> {}, signed section -> {} (predicted 13 vs 5)",
        u, s
    );
    ok &= u == 13 && s == 5;

    println!();
    println!("{}", if ok { "P3 WORKS" } else { "P3 FAILS" });
    std::process::exit(if ok { 0 } else { 1 });
}
