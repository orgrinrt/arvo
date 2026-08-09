//! PROBE 1: the first attempt at closing the bad_grant hole, and the hole
//! IN the closure, found by compiling it rather than by reasoning about it.
//!
//! File 17 found `impl ViewC<Relaxed> for Strict {}` compiles clean with
//! nothing to check, because the coercion carries no data
//! (`17_orchard_are_these_all_grades.md` section 4.3). The obvious fix is to
//! stop hand-declaring `ViewC` per pair and DERIVE it instead: liberties
//! become per-liberty marker traits (`HasReassoc`, `HasContract`, `HasArcp`),
//! and `ViewC<G>` is produced by exactly one blanket impl per target grade,
//! naming that grade's own liberty conjunction as the bound the source grade
//! must already satisfy. Same shape Dolan and Willsey already establish is
//! coherence-safe for the effect side (`14_dolan...md` section 7,
//! `15_willsey...md` section 5): marker-trait conjunctions do not collide.
//!
//! The claim this probe set out to verify: "there is nowhere left to author
//! an illegitimate direction by hand, because the only impl of `ViewC<G>` in
//! the program is the blanket." That claim is FALSE, and it is false for a
//! reason worth naming precisely, because it is a fact about Rust's
//! coherence checker that none of files 12 through 18 needed to know and
//! this dispatch's design walks straight into.
//!
//! Build:
//!   rustc -O 01_marker_conjunction_first_attempt_has_a_hole.rs -o p1a && ./p1a
//!   rustc --cfg attack_grant 01_marker_conjunction_first_attempt_has_a_hole.rs
//!     # expected: E0119 (conflicting impls). ACTUAL: compiles clean.

#![allow(dead_code)]

pub trait HasReassoc {}
pub trait HasContract {}
pub trait HasArcp {}

pub trait CGrade {
    const NAME: &'static str;
}

pub struct Strict;
impl CGrade for Strict {
    const NAME: &'static str = "Strict";
}
// Strict declares NONE of the liberty markers.

pub struct Relaxed;
impl CGrade for Relaxed {
    const NAME: &'static str = "Relaxed";
}
impl HasReassoc for Relaxed {}
impl HasContract for Relaxed {}
impl HasArcp for Relaxed {}

pub trait ViewC<G: CGrade>: CGrade {}

impl<A: CGrade> ViewC<Strict> for A {}
impl<A: HasReassoc + HasContract + HasArcp + CGrade> ViewC<Relaxed> for A {}

// The bad_grant equivalent, written by hand, alongside the blanket impl
// above. `Strict` does not satisfy `HasReassoc + HasContract + HasArcp`
// anywhere in this file, so the blanket impl's bound is, within THIS crate,
// unsatisfiable for `Strict`, and Rust's coherence checker is willing to
// treat that as proof the two impls cannot overlap. The concrete impl below
// is accepted. `Strict: ViewC<Relaxed>` now holds a second time, by the
// hand-written route, and every guarantee the derivation was supposed to
// supply is gone: nothing distinguishes this impl from the legitimate ones
// syntactically, and nothing checks it.
#[cfg(attack_grant)]
impl ViewC<Relaxed> for Strict {}

fn sum4<L: CGrade>(xs: [f64; 4], reassoc: bool) -> f64 {
    if reassoc {
        (xs[0] + xs[2]) + (xs[1] + xs[3])
    } else {
        ((xs[0] + xs[1]) + xs[2]) + xs[3]
    }
}

fn sum4_in_context<A: CGrade + ViewC<L>, L: CGrade>(xs: [f64; 4], reassoc: bool) -> f64 {
    sum4::<L>(xs, reassoc)
}

fn main() {
    let xs = [1.0e16f64, -1.0e16, 1.0, 1.0];
    println!("derived ViewC, one blanket impl per target grade:");
    println!("  legitimate calls typecheck as expected.");
    #[cfg(attack_grant)]
    {
        let v = sum4_in_context::<Strict, Relaxed>(xs, true);
        println!(
            "  a Strict operand, granted a Relaxed context by a hand impl alongside the blanket,"
        );
        println!(
            "  compiles clean and returns {:?}. The blanket-derivation move removed the",
            v
        );
        println!("  TEMPTATION to write this impl for the legitimate cases. It did not remove");
        println!("  the ABILITY to write it for an illegitimate one, because Rust's coherence");
        println!("  checker proves non-overlap from the ABSENCE of `Strict: HasReassoc` etc.");
        println!("  inside this crate, and absence of a bound is not the same fact as a");
        println!("  structural refusal. Probe 2 finds the shape that does not have this hole.");
    }
    #[cfg(not(attack_grant))]
    println!(
        "  (build with --cfg attack_grant to see the hand impl compile alongside the blanket)"
    );
}
