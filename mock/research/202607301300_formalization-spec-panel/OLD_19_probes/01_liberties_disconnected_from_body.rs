//! PROBE 1: even with an inclusion witness added to `ViewC` (file 17's own
//! proposal, section 4.3: "the liberty sets are data, inclusion between them
//! is decidable, a const check can refuse an impl whose declared direction
//! disagrees with the sets"), nothing connects the declared `LIBERTIES` array
//! to the body a liberty-gated function actually runs. This is 09's
//! disconnection finding (phi vs. delivery) transplanted to the fidelity axis,
//! and it survives a witness that only checks the SET, not the BODY.
//!
//! Three things are shown, each compiled and run:
//!
//!   (1) the inclusion witness Orchard proposed, built and working: a grade
//!       whose LIBERTIES do not shrink correctly under `ViewC` is refused at
//!       const eval.
//!   (2) UNDER-CLAIMING: a body reaches for a reassociating regrouping while
//!       its grade's own `LIBERTIES` array does not list `reassoc`. This
//!       compiles clean and the witness has nothing to say about it, because
//!       the witness only relates two DECLARED sets to each other, never a
//!       declared set to the CODE that runs under it.
//!   (3) OVER-CLAIMING: a grade declares `contract` in `LIBERTIES` and no
//!       body anywhere ever reaches for it. Also compiles clean, also
//!       unwitnessed, and it is the quieter of the two failures: an
//!       over-claiming grant is not wrong today, only meaningless, until the
//!       day something reads `LIBERTIES` to decide what a downstream target
//!       may do with it (16c's obligation), at which point it becomes a
//!       promise nothing backs.
//!
//! Build:
//!   rustc -O 01_liberties_disconnected_from_body.rs -o p1 && ./p1

#![allow(dead_code)]

// ---- the grade, with file 17's proposed inclusion witness added -----------

pub struct Strict;
pub struct Relaxed;

pub trait CGrade {
    const LIBERTIES: &'static [&'static str];
    const NAME: &'static str;
}
impl CGrade for Strict {
    const LIBERTIES: &'static [&'static str] = &[];
    const NAME: &'static str = "Strict";
}
impl CGrade for Relaxed {
    const LIBERTIES: &'static [&'static str] = &["reassoc", "contract", "arcp"];
    const NAME: &'static str = "Relaxed";
}

const fn is_subset(small: &[&str], big: &[&str]) -> bool {
    let mut i = 0;
    while i < small.len() {
        let mut found = false;
        let mut j = 0;
        while j < big.len() {
            if str_eq(small[i], big[j]) {
                found = true;
            }
            j += 1;
        }
        if !found {
            return false;
        }
        i += 1;
    }
    true
}
const fn str_eq(a: &str, b: &str) -> bool {
    let (a, b) = (a.as_bytes(), b.as_bytes());
    if a.len() != b.len() {
        return false;
    }
    let mut i = 0;
    while i < a.len() {
        if a[i] != b[i] {
            return false;
        }
        i += 1;
    }
    true
}

/// File 17 section 4.3's own proposal, built: a view is only grantable when
/// the target's liberties are a subset of the source's. This is a REAL
/// witness over REAL data and it does what it says: `bad_grant` (17_probes/06)
/// is now refused.
pub trait ViewC<G: CGrade>: CGrade {
    const WITNESS: () = assert!(
        is_subset(<G as CGrade>::LIBERTIES, <Self as CGrade>::LIBERTIES),
        "ViewC<G> for Self requires G's liberties to be a subset of Self's"
    );
}
impl ViewC<Strict> for Strict {}
impl ViewC<Relaxed> for Relaxed {}
impl ViewC<Strict> for Relaxed {}
// `impl ViewC<Relaxed> for Strict {}` would now fail: uncomment to see it,
// `LIBERTIES(Relaxed) ⊄ LIBERTIES(Strict) = []`.
const _: () = <Relaxed as ViewC<Strict>>::WITNESS;
const _: () = <Strict as ViewC<Strict>>::WITNESS;
const _: () = <Relaxed as ViewC<Relaxed>>::WITNESS;

// ---- (2) UNDER-CLAIMING: the body reaches for a liberty its grade never declared

/// This body regroups (the `reassoc` shape) under a grade whose own
/// `LIBERTIES` array does not contain `"reassoc"` at all. There is no bound,
/// no cfg, nothing anywhere that reads `L::LIBERTIES` to decide whether this
/// body is allowed to exist. It compiles, because Rust source code does not
/// consult an unrelated const array before choosing what arithmetic to write.
fn sum4_secretly_relaxed<L: CGrade>(xs: [f64; 4]) -> f64 {
    // a regrouping: two independent chains, then combined.
    (xs[0] + xs[2]) + (xs[1] + xs[3])
}

// ---- (3) OVER-CLAIMING: the grade promises a liberty no body ever exercises

/// A third grade, declaring `contract` (fused multiply-add licence) with no
/// corresponding body anywhere in this file reaching for `.mul_add()` under
/// it. Nothing refuses this. `LIBERTIES` is inspectable prose with a type
/// attached, not a specification anything checks a body against.
pub struct OverPromised;
impl CGrade for OverPromised {
    const LIBERTIES: &'static [&'static str] = &["contract"];
    const NAME: &'static str = "OverPromised";
}
impl ViewC<Strict> for OverPromised {}
const _: () = <OverPromised as ViewC<Strict>>::WITNESS;

fn main() {
    println!("(1) the inclusion witness Orchard proposed: real, and it works.");
    println!("    Relaxed -> Strict: witness discharged at compile time.");
    println!(
        "    Strict::LIBERTIES = {:?}, Relaxed::LIBERTIES = {:?}",
        Strict::LIBERTIES,
        Relaxed::LIBERTIES
    );
    println!();
    println!("(2) UNDER-CLAIMING, compiled and run:");
    let xs = [1.0e16f64, -1.0e16, 1.0, 1.0];
    println!(
        "    sum4_secretly_relaxed::<Strict>({:?}) = {}",
        xs,
        sum4_secretly_relaxed::<Strict>(xs)
    );
    println!(
        "    Strict::LIBERTIES contains \"reassoc\": {}",
        Strict::LIBERTIES.contains(&"reassoc")
    );
    println!("    ^ the body regrouped anyway. The witness in (1) never looked at this");
    println!("      function; it only relates two DECLARED sets to each other.");
    println!();
    println!("(3) OVER-CLAIMING, compiled clean:");
    println!(
        "    OverPromised::LIBERTIES = {:?}, and no body in this file uses \"contract\" under it",
        OverPromised::LIBERTIES
    );
    println!("    ^ a promise with nothing behind it. Harmless until something (a");
    println!("      downstream target, per 16c) reads LIBERTIES to decide what it may do.");
}
