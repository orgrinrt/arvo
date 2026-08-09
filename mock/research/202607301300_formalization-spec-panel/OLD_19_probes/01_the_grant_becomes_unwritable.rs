//! PROBE 1: closing the fidelity-licence hole, and the hole in the first fix
//! I tried, both compiled rather than argued.
//!
//! 17_orchard's `bad_grant` (`17_probes/06_two_lattices_opposite_variance.rs`)
//! found that `impl ViewC<Relaxed> for Strict {}` compiles clean: a marker
//! trait with no associated items has nothing for the compiler to check, so a
//! false grant is silent.
//!
//! My first attempt (this file, before this comment was true) tried a single
//! blanket impl gated by `Assert<{ liberties_subset(G::LIBERTIES,
//! A::LIBERTIES) }>: IsTrue` in the `where` clause, so no per-pair impl would
//! ever exist to write a lie on. It does not compile:
//!
//!   error: generic parameters may not be used in const operations
//!      = help: add `#![feature(generic_const_exprs)]` to allow generic
//!        const expressions
//!
//! That is the literal forbidden pattern named in `arvo/CLAUDE.md`: "a const
//! expression computed from a generic parameter, used in type position."
//! `Assert<{ ... }>` is a type; its const argument depended on the still-
//! generic `A` and `G` of the blanket impl. A fully generic, closed-form
//! derivation of the coercion relation as ONE impl is not expressible under
//! this workspace's feature ban, and no amount of arguing past that changes
//! it; the compiler is the check.
//!
//! `witness_style` below is the shape this file argues against on paper:
//! Thread C's `Resolution::WITNESS` pattern (`07_probes/a7...rs:70-79`)
//! ported directly onto `ViewC`. It DOES compile (the wall above is specific
//! to const-generic type ARGUMENTS, not to a plain `assert!()` inside a
//! `const` block referencing `Self`/`G`, which is ordinary computation). And
//! it reproduces the exact disarming move `a7`'s own `SubstituteZero` used
//! (`a7...rs:113-127`): an implementor writing the false grant overrides the
//! default `WITNESS` to `()` in the same breath, and the lie compiles.
//!
//! `door_style` is the shape this file argues FOR. Drop the per-pair marker
//! as the load-bearing mechanism. `ViewC` either does not exist at all or is
//! purely advisory. The relation is RECOMPUTED, un-disarmably, inside every
//! function that actually consumes a fidelity-gated body, the same way
//! `resolve::<R>()`'s door recomputes `stable::<R>()` instead of trusting
//! `R::WITNESS`. No per-pair impl, no forbidden feature, and it survives a
//! grown axis with zero new code (`grown_axis`).
//!
//! Four builds:
//!   rustc -O 01...rs -o p1 && ./p1                          (door_style, default)
//!   rustc --cfg witness_style -O 01...rs -o p1w && ./p1w    (the disarmable port)
//!   rustc --cfg grown_axis -O 01...rs -o p1g && ./p1g       (door_style + a 3rd grade)
//!   rustc --cfg blanket_attempt 01...rs                     (expect the generic_const_exprs error above)

#![allow(dead_code)]

pub const REASSOC: u8 = 0b0001;
pub const CONTRACT: u8 = 0b0010;
pub const ARCP: u8 = 0b0100;
pub const NSZ: u8 = 0b1000;

pub trait CGrade {
    const LIBERTIES: u8;
    const NAME: &'static str;
}

pub struct Strict;
pub struct Relaxed;

impl CGrade for Strict {
    const LIBERTIES: u8 = 0;
    const NAME: &'static str = "Strict";
}
impl CGrade for Relaxed {
    const LIBERTIES: u8 = REASSOC | CONTRACT | ARCP;
    const NAME: &'static str = "Relaxed";
}

/// The one formula. Every liberty `g` grants must already be held by
/// `holder`. This is the entire content of "declining a liberty is always
/// sound, granting one nobody asked for is not," and it is uniform across
/// every pair a design ever adds: nothing here names `Strict` or `Relaxed`.
pub const fn liberties_subset(g: u8, holder: u8) -> bool {
    (g & !holder) == 0
}

// =========================================================== blanket_attempt
// Left in, deliberately, as a compile-fail artifact. No `feature(...)` gate
// is opened anywhere in this file: the point is what happens under the
// workspace's actual, permitted feature set. Building with
// --cfg blanket_attempt reproduces the exact error this probe's header
// quotes, at item-definition time, with no `main`-body usage needed.
#[cfg(blanket_attempt)]
mod blanket_attempt_mod {
    use super::*;

    pub struct Assert<const COND: bool>;
    pub trait IsTrue {}
    impl IsTrue for Assert<true> {}

    pub trait ViewC<G: CGrade>: CGrade {}

    impl<A, G> ViewC<G> for A
    where
        A: CGrade,
        G: CGrade,
        Assert<{ liberties_subset(G::LIBERTIES, A::LIBERTIES) }>: IsTrue,
    {
    }
}

// =============================================================== witness_style
// Thread C's own pattern, ported. Compiles. Disarmable, for the same
// structural reason `a7`'s `SubstituteZero` was: the check lives in a
// default the implementor also controls.
#[cfg(witness_style)]
mod witnessed {
    use super::*;

    pub trait ViewC<G: CGrade>: CGrade {
        const WITNESS: () = {
            assert!(
                liberties_subset(G::LIBERTIES, Self::LIBERTIES),
                "ViewC<G> for Self grants a liberty Self does not hold"
            );
        };
    }

    impl ViewC<Strict> for Strict {}
    impl ViewC<Relaxed> for Relaxed {}
    impl ViewC<Strict> for Relaxed {}

    // The exact move at `07_probes/a7_door_checks_directly.rs:113-127`,
    // applied here: override the default WITNESS to the unit value so
    // nothing is ever asserted. The false grant compiles.
    impl ViewC<Relaxed> for Strict {
        const WITNESS: () = ();
    }

    // Forcing the (disarmed) witness compiles clean. Nothing catches this.
    const _: () = <Strict as ViewC<Relaxed>>::WITNESS;

    pub fn licensed_view<A: CGrade + ViewC<G>, G: CGrade>() -> &'static str {
        G::NAME
    }
}

// =================================================================== door_style
// No per-pair marker at all. `ViewC` does not exist as a trust-bearing
// trait; the relation is recomputed, un-disarmably, inside the one place
// any fidelity-gated body is actually selected. This mirrors
// `resolve::<R>()`'s door (`a7...rs:187-196`): a plain `const { assert!(...) }`
// block referencing a still-generic parameter's associated const is
// ordinary computation, not a const-generic type argument, so it needs no
// feature this workspace forbids, and it fires per monomorphisation with no
// implementor anywhere in a position to opt out of it.
#[cfg(not(any(witness_style, blanket_attempt)))]
mod door {
    use super::*;

    pub fn licensed_view<A: CGrade, G: CGrade>() -> &'static str {
        const {
            assert!(
                liberties_subset(G::LIBERTIES, A::LIBERTIES),
                "a value under this licence may not be viewed at a grade that grants a liberty it does not hold"
            );
        }
        G::NAME
    }
}
#[cfg(not(any(witness_style, blanket_attempt)))]
use door::licensed_view;
#[cfg(witness_style)]
use witnessed::licensed_view;

#[cfg(grown_axis)]
pub struct Wild;
#[cfg(grown_axis)]
impl CGrade for Wild {
    const LIBERTIES: u8 = REASSOC | CONTRACT | ARCP | NSZ;
    const NAME: &'static str = "Wild";
}

fn main() {
    #[cfg(blanket_attempt)]
    {
        println!("this arm should not build at all; see the module above.");
        return;
    }

    println!("the door, called with every real pair (no per-pair impl exists anywhere):");
    println!(
        "  licensed_view::<Strict, Strict>()   = {}",
        licensed_view::<Strict, Strict>()
    );
    println!(
        "  licensed_view::<Relaxed, Relaxed>()  = {}",
        licensed_view::<Relaxed, Relaxed>()
    );
    println!(
        "  licensed_view::<Relaxed, Strict>()   = {}",
        licensed_view::<Relaxed, Strict>()
    );
    // licensed_view::<Strict, Relaxed>() left commented: under door_style it
    // is a compile-time const-eval panic at THIS call site, not a type
    // error naming a missing impl. Uncomment to see it.
    // let _ = licensed_view::<Strict, Relaxed>();

    #[cfg(grown_axis)]
    {
        println!();
        println!("grown_axis: a third grade, zero new code beyond its own CGrade impl:");
        println!(
            "  licensed_view::<Wild, Strict>()   = {}",
            licensed_view::<Wild, Strict>()
        );
        println!(
            "  licensed_view::<Wild, Relaxed>()  = {}",
            licensed_view::<Wild, Relaxed>()
        );
        // licensed_view::<Relaxed, Wild>() would panic at const-eval: Wild
        // grants nsz, Relaxed does not hold it. No new impl was needed to
        // make this refusal correct; the formula already covers it.
    }

    #[cfg(witness_style)]
    {
        println!();
        println!("witness_style: the disarmed grant compiled and IS reachable:");
        println!(
            "  licensed_view::<Strict, Relaxed>() = {}   <-- WRONG, and it compiled",
            licensed_view::<Strict, Relaxed>()
        );
    }

    println!();
    println!("OK: door_style needs no per-pair impl, no forbidden feature, and no witness to");
    println!("keep honest, because there is nothing declared to keep honest. The relation is");
    println!("computed at every consumption site, which is where witness_style's disarming");
    println!("move has nothing left to disarm.");
}
