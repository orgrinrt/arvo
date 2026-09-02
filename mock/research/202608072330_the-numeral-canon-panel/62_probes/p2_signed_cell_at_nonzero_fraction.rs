//! Probe p2: the signed cell at F > 0, which is the empty cell `59` P4 names.
//!
//! WHAT IS MEASURED. Signed saturating and signed wrapping multiplication at
//! F = 1, 2, 3 across three widths, under BOTH rescale spellings the signed
//! domain admits and the unsigned domain cannot distinguish:
//!
//!   trunc: (a*b) / 2^F   truncating division, rounds toward zero, an odd
//!          function: trunc(-x) = -trunc(x)
//!   floor: (a*b) >> F    arithmetic shift right, rounds toward -infinity,
//!          translation-covariant: floor(x + k*2^F) = floor(x) + k
//!
//! On nonnegative products the two coincide, which is why every unsigned
//! probe in this unit (`57_probes/p3`, `61_probes/q2`) was silently measuring
//! both at once. On negative products they differ, so the signed cell is the
//! first place the panel's "the rescale" is actually two different maps.
//!
//! QUESTIONS, with predictions stated before running:
//!
//!   1. Does signed multiplication at F > 0 fail associativity and
//!      distributivity under both policies and both spellings? Predict yes,
//!      every configuration: the coarsening argument (`58` section 2.2) is
//!      sign-free, since it is about information discarded per pairwise step.
//!
//!   2. Does the symmetric-clamp rescue from p1 (mul assoc exact at F = 0)
//!      survive the fraction axis? Predict no: coarsening alone is sufficient
//!      to break associativity (`57_probes/p4` CoarsenOnly, unsigned), so at
//!      F > 0 the signed cell has TWO independent mechanisms, asymmetry and
//!      coarsening, and removing the asymmetry no longer rescues anything.
//!
//!   3. Is the coarsening sufficient in the signed domain with no reduction
//!      at all (the CoarsenOnly arm transplanted to signed operands, both
//!      spellings)? Predict yes, both spellings, with witnesses on negative
//!      operands where the two spellings give different wrong answers.
//!
//!   4. Does the rescale spelling change the failure counts (magnitude)
//!      without changing their existence? Predict yes, matching the shape of
//!      `58` section 3.1's rounding-mode result: the spelling is a rounding
//!      rule for negatives, and rounding rules move counts, never zeroes.
//!
//!   5. Does the additive half stay exactly where F = 0 left it (broken for
//!      saturation, exact group for wrap), at every F? Predict yes: no add
//!      closure below reads the scale, which is checkable by inspection, the
//!      same argument `58` made and `61` re-made for wrap.
//!
//! INSTRUMENT VALIDATION. The F = 0 rows must reproduce p1's committed
//! counts exactly (signed sat 2c w=4: mul_assoc 160, add_assoc 952; signed
//! sat sym w=4: mul_assoc 0; signed wrap w=4: all zero), and both verdicts
//! (fails, holds) must be observed somewhere in the sweep.
//!
//! Build and run:
//!   rustc +nightly-2026-05-28 -O --edition 2021 \
//!       -o p2 p2_signed_cell_at_nonzero_fraction.rs && ./p2

#[derive(Clone, Copy, PartialEq, Eq)]
enum Policy {
    Sat,
    Wrap,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Rescale {
    /// (x) / 2^f, truncating division, toward zero
    Trunc,
    /// (x) >> f, arithmetic shift, toward -infinity
    Floor,
}

#[derive(Clone, Copy)]
struct Fmt {
    lo: i64,
    hi: i64,
    f: u32,
    p: Policy,
    r: Rescale,
}

impl Fmt {
    fn reduce(&self, x: i64) -> i64 {
        let m = self.hi - self.lo + 1;
        match self.p {
            Policy::Sat => x.clamp(self.lo, self.hi),
            Policy::Wrap => (x - self.lo).rem_euclid(m) + self.lo,
        }
    }
    fn rescale(&self, x: i64) -> i64 {
        match self.r {
            Rescale::Trunc => x / (1i64 << self.f),
            Rescale::Floor => x >> self.f,
        }
    }
    fn add(&self, a: i64, b: i64) -> i64 {
        // never reads f or the rescale: the additive half is scale-blind by
        // construction, which is the F-independence argument in code.
        self.reduce(a + b)
    }
    fn mul(&self, a: i64, b: i64) -> i64 {
        self.reduce(self.rescale(a * b))
    }
}

#[derive(Default)]
struct Counts {
    add_assoc: u64,
    add_inv_missing: u64,
    mul_assoc: u64,
    mul_comm: u64,
    distrib: u64,
}

fn check(f: Fmt) -> Counts {
    let mut c = Counts::default();
    for a in f.lo..=f.hi {
        if !(f.lo..=f.hi).any(|b| f.add(a, b) == 0) {
            c.add_inv_missing += 1;
        }
        for b in f.lo..=f.hi {
            if f.mul(a, b) != f.mul(b, a) {
                c.mul_comm += 1;
            }
            for x in f.lo..=f.hi {
                if f.add(f.add(a, b), x) != f.add(a, f.add(b, x)) {
                    c.add_assoc += 1;
                }
                if f.mul(f.mul(a, b), x) != f.mul(a, f.mul(b, x)) {
                    c.mul_assoc += 1;
                }
                if f.mul(a, f.add(b, x)) != f.add(f.mul(a, b), f.mul(a, x)) {
                    c.distrib += 1;
                }
            }
        }
    }
    c
}

/// the CoarsenOnly arm on signed operands: no reduction map anywhere, just
/// the per-step rescale, exactly `57_probes/p4`'s CoarsenOnly transplanted
/// past zero. counts associativity failures of
/// r(r(a*b)*c) == r(a*r(b*c)) over the signed box, per spelling.
fn coarsen_only(bound: i64, f: u32, r: Rescale) -> (u64, Option<(i64, i64, i64, i64, i64)>) {
    let res = |x: i64| match r {
        Rescale::Trunc => x / (1i64 << f),
        Rescale::Floor => x >> f,
    };
    let mut fails = 0u64;
    let mut wit: Option<(i64, i64, i64, i64, i64)> = None;
    for a in -bound..=bound {
        for b in -bound..=bound {
            for c in -bound..=bound {
                let l = res(res(a * b) * c);
                let rr = res(a * res(b * c));
                if l != rr {
                    fails += 1;
                    // prefer a witness with a negative operand, where the
                    // spellings genuinely differ
                    if wit.is_none() || (a < 0 || b < 0 || c < 0) && wit.unwrap().0 >= 0 {
                        wit = Some((a, b, c, l, rr));
                    }
                }
            }
        }
    }
    (fails, wit)
}

fn main() {
    let mut ok = true;

    println!("=== section 1: instrument reproduction against p1's committed F = 0 counts ===");
    println!();
    let f0 = |lo: i64, hi: i64, p: Policy| Fmt {
        lo,
        hi,
        f: 0,
        p,
        r: Rescale::Trunc,
    };
    let c_2c = check(f0(-8, 7, Policy::Sat));
    let c_sym = check(f0(-7, 7, Policy::Sat));
    let c_wr = check(f0(-8, 7, Policy::Wrap));
    println!(
        "  signed sat 2c  w=4 F=0: mul_assoc {} (want 160), add_assoc {} (want 952)",
        c_2c.mul_assoc, c_2c.add_assoc
    );
    println!(
        "  signed sat sym w=4 F=0: mul_assoc {} (want 0)",
        c_sym.mul_assoc
    );
    println!(
        "  signed wrap    w=4 F=0: mul_assoc {} add_assoc {} distrib {} (want all 0)",
        c_wr.mul_assoc, c_wr.add_assoc, c_wr.distrib
    );
    let repro = c_2c.mul_assoc == 160
        && c_2c.add_assoc == 952
        && c_sym.mul_assoc == 0
        && c_wr.mul_assoc == 0
        && c_wr.add_assoc == 0
        && c_wr.distrib == 0;
    println!("  reproduction: {}", repro);
    ok &= repro;
    println!();

    println!("=== section 2: the signed cell at F > 0, all policies, both spellings ===");
    println!();
    println!(
        "{:>22} {:>2} {:>6} {:>8} {:>8} {:>8} {:>8} {:>7}",
        "format", "F", "spell", "+assoc", "*assoc", "distrib", "*comm", "no-inv"
    );
    let mut frac_all_fail_mul = true;
    let mut frac_sym_all_fail_mul = true;
    let mut add_matches_f0 = true;
    let mut spelling_changed_some_count = false;
    let mut spelling_changed_no_existence = true;
    for w in [4u32, 5, 6] {
        let hi = (1i64 << (w - 1)) - 1;
        let lo = -(1i64 << (w - 1));
        let h = hi;
        // f0 baselines per family for the additive-half equality check
        let base_2c = check(f0(lo, hi, Policy::Sat));
        let base_sym = check(f0(-h, h, Policy::Sat));
        let base_wr = check(f0(lo, hi, Policy::Wrap));
        for f in 1..=3u32 {
            let mut per_family_counts: Vec<(u64, u64)> = Vec::new();
            for (name, flo, fhi, p, base) in [
                ("signed sat 2c", lo, hi, Policy::Sat, &base_2c),
                ("signed sat sym", -h, h, Policy::Sat, &base_sym),
                ("signed wrap", lo, hi, Policy::Wrap, &base_wr),
            ] {
                let mut pair = (0u64, 0u64);
                for (rn, r) in [("trunc", Rescale::Trunc), ("floor", Rescale::Floor)] {
                    let c = check(Fmt {
                        lo: flo,
                        hi: fhi,
                        f,
                        p,
                        r,
                    });
                    println!(
                        "{:>18} w={} {:>2} {:>6} {:>8} {:>8} {:>8} {:>8} {:>7}",
                        name,
                        w,
                        f,
                        rn,
                        c.add_assoc,
                        c.mul_assoc,
                        c.distrib,
                        c.mul_comm,
                        c.add_inv_missing
                    );
                    frac_all_fail_mul &= c.mul_assoc > 0 && c.distrib > 0;
                    if name == "signed sat sym" {
                        frac_sym_all_fail_mul &= c.mul_assoc > 0;
                    }
                    add_matches_f0 &= c.add_assoc == base.add_assoc;
                    spelling_changed_no_existence &= c.mul_assoc > 0;
                    if r == Rescale::Trunc {
                        pair.0 = c.mul_assoc;
                    } else {
                        pair.1 = c.mul_assoc;
                    }
                }
                if pair.0 != pair.1 {
                    spelling_changed_some_count = true;
                }
                per_family_counts.push(pair);
            }
            let _ = per_family_counts;
        }
    }
    println!();
    println!(
        "  every F > 0 configuration fails *assoc and distrib, both policies, both spellings: {}",
        frac_all_fail_mul
    );
    println!(
        "  the symmetric-clamp rescue does NOT survive the fraction axis: {}",
        frac_sym_all_fail_mul
    );
    println!(
        "  additive counts identical to the family's own F = 0 row at every F (scale-blind): {}",
        add_matches_f0
    );
    println!(
        "  the spelling moved at least one count: {}   and zeroed none: {}",
        spelling_changed_some_count, spelling_changed_no_existence
    );
    ok &= frac_all_fail_mul
        && frac_sym_all_fail_mul
        && add_matches_f0
        && spelling_changed_some_count
        && spelling_changed_no_existence;
    println!();

    println!("=== section 3: coarsening alone, signed operands, no reduction map anywhere ===");
    println!();
    let mut coarsen_fails_both = true;
    for f in 1..=2u32 {
        for (rn, r) in [("trunc", Rescale::Trunc), ("floor", Rescale::Floor)] {
            let (fails, wit) = coarsen_only(15, f, r);
            println!(
                "  box [-15,15]^3  F={}  {}: {} assoc failures, witness {:?}",
                f, rn, fails, wit
            );
            coarsen_fails_both &= fails > 0;
        }
    }
    println!();
    println!(
        "  coarsening is sufficient to break signed *assoc with no clamp and no wrap present: {}",
        coarsen_fails_both
    );
    ok &= coarsen_fails_both;
    // one concrete triple where the two spellings give different wrong answers,
    // so the spelling is genuinely a second rounding axis on negatives
    {
        let t = |x: i64| x / 2;
        let fl = |x: i64| x >> 1;
        let (a, b, c) = (-3i64, 5, 7);
        let lt = t(t(a * b) * c);
        let rt = t(a * t(b * c));
        let lf = fl(fl(a * b) * c);
        let rf = fl(a * fl(b * c));
        println!(
            "  spelling witness F=1 (a,b,c)=({},{},{}): trunc gives {} vs {}, floor gives {} vs {}",
            a, b, c, lt, rt, lf, rf
        );
    }

    println!();
    println!("{}", if ok { "P2 WORKS" } else { "P2 FAILS" });
    std::process::exit(if ok { 0 } else { 1 });
}
