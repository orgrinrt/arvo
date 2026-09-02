//! Probe 1: one membership predicate covers integer, fixed-point and float value sets.
//!
//! Hypothesis: a format's representable set is characterised by a slot function
//! phi (which power of the radix the quantum sits at, as a function of magnitude)
//! plus range bounds. Membership is stated ONCE: x is representable iff
//! x / 2^phi(x) is an integer and x is within bounds. Integer formats are the
//! instance phi == 0, fixed-point is phi == -F (constant), floating-point is
//! phi(x) = max(emin, e(x) - p) where 2^(e-1) <= |x| < 2^e.
//!
//! The check: for each instance, exhaustively compare the generic predicate
//! against a DIRECT enumeration of the value set written from the instance's
//! own textbook definition. The two constructions are independent: one goes
//! through phi, the other never mentions phi.
//!
//! Instrument validation: two mutants (drop the max(emin, ..) clamp; allow
//! mantissa m == 2^p in the enumeration) must each be DETECTED as a mismatch.
//! If a mutant passes, the instrument cannot fail and proves nothing.
//!
//! All arithmetic is exact: values are integers q with x = q / 32 (denominator
//! 2^5, finer than every quantum used here), so no float rounding pollutes the
//! probe.
//!
//! Outcome is printed at the end: P1 WORKS or P1 FAILS.

// value scale: x = q / 32, i.e. x * 2^5 = q. SCALE_POW = 5.
const SCALE_POW: i32 = 5;

// bit length of |q| (0 for q == 0)
fn bitlen(q: i64) -> i32 {
    let mut n = q.abs();
    let mut l = 0;
    while n > 0 {
        n >>= 1;
        l += 1;
    }
    l
}

// e(x) with 2^(e-1) <= |x| < 2^e, for x = q / 2^SCALE_POW, q != 0
fn mag_exp(q: i64) -> i32 {
    bitlen(q) - SCALE_POW
}

/// A format, as the probe models it: a slot function plus inclusive value bounds
/// (bounds in scaled units). This is the whole parameterization under test.
struct Format {
    slot: fn(q: i64) -> i32,
    lo: i64,
    hi: i64,
}

/// THE membership predicate, written once for every instance.
/// x representable  iff  lo <= x <= hi  and  x / 2^slot(x) is an integer.
/// In scaled units: q / 2^(slot + SCALE_POW) integral, i.e. q divisible by
/// 2^(slot + SCALE_POW) when that exponent is positive, else always integral
/// (a slot finer than the probe scale divides everything the scale can write).
fn member(f: &Format, q: i64) -> bool {
    if q < f.lo || q > f.hi {
        return false;
    }
    let s = (f.slot)(q) + SCALE_POW;
    // a slot finer than the probe scale divides everything the scale can write
    if s <= 0 {
        return true;
    }
    q % (1i64 << s) == 0
}

// ---- instances, via phi ----

// unsigned integer, 4 bits: phi == 0, range [0, 15]
fn slot_int(_q: i64) -> i32 {
    0
}

// unsigned fixed-point I=2, F=2: phi == -2, range [0, 4 - 1/4]
fn slot_fixed(_q: i64) -> i32 {
    -2
}

// float p=3, emin=-2 (minimal slot), max slot 2: phi(x) = max(emin, e(x)-p)
const P: i32 = 3;
const EMIN: i32 = -2;
const EMAX: i32 = 2;
fn slot_float(q: i64) -> i32 {
    if q == 0 {
        return EMIN;
    }
    let e = mag_exp(q);
    let s = e - P; // canonical slot: x = m * 2^(e-p), m in [2^(p-1), 2^p)
    if s < EMIN {
        EMIN
    } else {
        s
    }
}

// MUTANT A: the max(emin, ..) clamp dropped
fn slot_float_mutant(q: i64) -> i32 {
    if q == 0 {
        return EMIN;
    }
    mag_exp(q) - P
}

// ---- direct enumerations, never mentioning phi ----

use std::collections::BTreeSet;

fn enum_int() -> BTreeSet<i64> {
    // {k : 0 <= k <= 15}, scaled
    (0..=15).map(|k| k << SCALE_POW).collect()
}

fn enum_fixed() -> BTreeSet<i64> {
    // {k / 4 : 0 <= k < 16}, scaled: k * 2^(5-2)
    (0..16).map(|k| k << (SCALE_POW - 2)).collect()
}

fn enum_float(m_top: i64) -> BTreeSet<i64> {
    // {m * 2^e : |m| < m_top, emin <= e <= emax}, scaled by 2^5.
    // m_top = 2^p for the true set; the mutant enumeration passes 2^p + 1.
    let mut s = BTreeSet::new();
    for e in EMIN..=EMAX {
        for m in -(m_top - 1)..=(m_top - 1) {
            let sh = e + SCALE_POW; // >= 3 here, always nonnegative
            s.insert(m << sh);
        }
    }
    s
}

// collect the generic predicate's set over the whole scaled range
fn predicate_set(f: &Format) -> BTreeSet<i64> {
    let mut s = BTreeSet::new();
    let mut q = f.lo;
    while q <= f.hi {
        if member(f, q) {
            s.insert(q);
        }
        q += 1;
    }
    s
}

fn main() {
    let mut ok = true;

    // instance 1: unsigned 4-bit integer
    let f_int = Format {
        slot: slot_int,
        lo: 0,
        hi: 15 << SCALE_POW,
    };
    let a = predicate_set(&f_int);
    let b = enum_int();
    println!(
        "int4:    predicate {} values, enumeration {} values, equal: {}",
        a.len(),
        b.len(),
        a == b
    );
    ok &= a == b && a.len() == 16;

    // instance 2: unsigned fixed 2.2
    let f_fix = Format {
        slot: slot_fixed,
        lo: 0,
        hi: (16 << (SCALE_POW - 2)) - 1,
    };
    let a = predicate_set(&f_fix);
    let b = enum_fixed();
    println!(
        "fix2.2:  predicate {} values, enumeration {} values, equal: {}",
        a.len(),
        b.len(),
        a == b
    );
    ok &= a == b && a.len() == 16;

    // instance 3: float p=3, emin=-2, emax=2
    let top = (((1i64 << P) - 1) << (EMAX + SCALE_POW)) as i64; // largest finite, scaled
    let f_flt = Format {
        slot: slot_float,
        lo: -top,
        hi: top,
    };
    let a = predicate_set(&f_flt);
    let b = enum_float(1 << P);
    println!(
        "float:   predicate {} values, enumeration {} values, equal: {}",
        a.len(),
        b.len(),
        a == b
    );
    ok &= a == b;

    // subnormal spot check: 1/4 (= 2^emin, smallest positive) in; 1/8 out
    ok &= a.contains(&(1 << (SCALE_POW - 2)));
    ok &= !a.contains(&(1 << (SCALE_POW - 3)));

    // ---- instrument validation: both mutants must be DETECTED ----

    // mutant A: clamp dropped in phi. Finer slots below emin admit values the
    // true set excludes (e.g. 1/8): sets must now DIFFER.
    let f_mut = Format {
        slot: slot_float_mutant,
        lo: -top,
        hi: top,
    };
    let am = predicate_set(&f_mut);
    let detect_a = am != b;
    println!("mutant A (no emin clamp) detected: {}", detect_a);
    ok &= detect_a;

    // mutant B: enumeration allows m == 2^p. Extra values appear: must DIFFER
    // from the predicate set.
    let bm = enum_float((1 << P) + 1);
    let detect_b = a != bm;
    println!(
        "mutant B (mantissa overflow in enumeration) detected: {}",
        detect_b
    );
    ok &= detect_b;

    println!("{}", if ok { "P1 WORKS" } else { "P1 FAILS" });
    std::process::exit(if ok { 0 } else { 1 });
}
