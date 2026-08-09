//! PROBE 2: the shape that closes probe 1's hole, verified against the same
//! attack, plus the attack against ITS OWN residual trust boundary.
//!
//! Probe 1's hole exists because `ViewC<G>` is a nameable, freestanding
//! trait, and a same-crate author can always write a second concrete impl
//! of a nameable trait alongside a blanket one, provided the blanket's own
//! bound happens not to be satisfied yet. There is no version of "derive it
//! as a blanket impl over a coercion trait" that closes this, because the
//! coercion trait itself is the thing being attacked, and its existence as
//! a separately nameable item is the whole surface.
//!
//! The fix removes the surface rather than guarding it: liberties become
//! per-axis TRUTH MARKERS (`True`/`False`, exactly `07_probes/a7_door_
//! checks_directly.rs`'s vocabulary) named as associated types on `CGrade`,
//! a `Dominates<Target>` relation is declared with the refused case
//! (`Dominates<True> for False`) simply ABSENT rather than declared and
//! checked, and every function that crosses a fidelity boundary (the
//! "door") states the bound directly in its own where-clause:
//! `A::Reassoc: Dominates<L::Reassoc>`. There is no `ViewC` trait at all.
//! Nothing is derived from anything; the door is the only place the fact is
//! ever consulted, and it consults the two grades' own declarations
//! directly, the same way `resolve<R: Resolution>` in `07_probes/a7` reads
//! `R::phi` directly rather than through an intermediate coercion trait a
//! second impl could subvert.
//!
//! What this DOES close, verified: no coercion trait exists for a same-crate
//! author to attach a rogue impl to (probe 1's hole). No amount of adding
//! impls anywhere widens what `door` accepts, because `door`'s bound is not
//! satisfied by an impl, it is satisfied by the STRUCTURE of the
//! `Dominates` relation, which has exactly the cases written below and no
//! others.
//!
//! What this does NOT close, verified, and named rather than hidden: the
//! relation's own definition. `Dominates<True> for False` is absent because
//! someone decided it should be, the same way `phi`'s five recovery-map
//! bodies are hand-written and trusted (`10_leroy...md` section 8, bin
//! three, item 1). Widening the axis by adding that one line is `attack_
//! widen_the_axis` below, and it succeeds, honestly, because it is not an
//! attack on the mechanism: it is a direct edit to the leaf fact the whole
//! design rests its trust on, in the one place that fact is declared, in
//! the same crate that owns the axis. That is the correct amount of
//! closure: unattackable from the outside, editable at its own root, same
//! as every trusted leaf in this design.
//!
//! Build:
//!   rustc -O 02_the_door_states_the_bound_directly.rs -o p2 && ./p2
//!   rustc --cfg attack_call     02_the_door_states_the_bound_directly.rs
//!     # calls door::<Strict, Relaxed>() directly; expect E0277, gets it
//!   rustc --cfg attack_widen_the_axis 02_the_door_states_the_bound_directly.rs
//!     # widens Dominates itself; expect success (this is the honest,
//!     # irreducible trust boundary, not a hole)

#![allow(dead_code)]

pub struct True;
pub struct False;

/// `Self` dominates `Target` when a value licensed for `Self`'s liberty on
/// this axis may stand where `Target`'s liberty on this axis is required.
/// True dominates both; False dominates only False. The refused case,
/// `Dominates<True> for False`, is not written, not commented out, simply
/// never authored, because "declining a liberty is free, acquiring one is
/// not" (`17_orchard...md` section 4.1) has no honest witness to attach to
/// a case that should not exist.
pub trait Dominates<Target> {}
impl Dominates<False> for True {}
impl Dominates<False> for False {}
impl Dominates<True> for True {}
#[cfg(attack_widen_the_axis)]
impl Dominates<True> for False {}

pub trait CGrade {
    type Reassoc;
    type Contract;
    type Arcp;
    const NAME: &'static str;
}

pub struct Strict;
impl CGrade for Strict {
    type Reassoc = False;
    type Contract = False;
    type Arcp = False;
    const NAME: &'static str = "Strict";
}

pub struct Relaxed;
impl CGrade for Relaxed {
    type Reassoc = True;
    type Contract = True;
    type Arcp = True;
    const NAME: &'static str = "Relaxed";
}

/// The door. Every function whose body may perform a fidelity-crossing
/// substitution states the bound on its OWN signature, over the two
/// grades' own associated markers, with nothing in between for an author
/// to spoof.
fn sum4<L: CGrade>(xs: [f64; 4], reassoc: bool) -> f64 {
    if reassoc {
        (xs[0] + xs[2]) + (xs[1] + xs[3])
    } else {
        ((xs[0] + xs[1]) + xs[2]) + xs[3]
    }
}

fn sum4_in_context<A: CGrade, L: CGrade>(xs: [f64; 4], reassoc: bool) -> f64
where
    A::Reassoc: Dominates<L::Reassoc>,
{
    sum4::<L>(xs, reassoc)
}

fn main() {
    let xs = [1.0e16f64, -1.0e16, 1.0, 1.0];
    println!("the door states the bound directly, no coercion trait to attach a rogue impl to:");
    println!(
        "  Relaxed operand, Strict context: {:?}",
        sum4_in_context::<Relaxed, Strict>(xs, false)
    );
    println!(
        "  Strict operand, Strict context:  {:?}",
        sum4_in_context::<Strict, Strict>(xs, false)
    );
    println!(
        "  Relaxed operand, Relaxed context: {:?}",
        sum4_in_context::<Relaxed, Relaxed>(xs, true)
    );

    #[cfg(attack_call)]
    {
        // Strict operand, Relaxed context. No impl of anything grants this;
        // the bound is `False: Dominates<True>`, which has no impl. Refused
        // by the type system, not by a witness someone remembered to write.
        let v = sum4_in_context::<Strict, Relaxed>(xs, true);
        println!(
            "  Strict operand, Relaxed context: {:?}  <-- should not compile",
            v
        );
    }
    #[cfg(not(attack_call))]
    println!("  (build with --cfg attack_call: E0277, False: Dominates<True> has no impl)");

    #[cfg(attack_widen_the_axis)]
    {
        // This is not an attack on the door. It is a direct, honest edit to
        // the trusted leaf fact that Dominates exists to state, made in the
        // same crate that declares the axis. It succeeds because it should:
        // this is where the design's trust genuinely bottoms out, same bin
        // as the five hand-written `phi` bodies, not a hole in the door.
        let v = sum4_in_context::<Strict, Relaxed>(xs, true);
        println!(
            "  Strict widened to dominate Relaxed by editing Dominates itself: {:?}",
            v
        );
        println!("  This compiles, and it should: the axis's own definition is the trust");
        println!("  boundary, not the door. Nobody attacked the mechanism; someone edited");
        println!("  the one place the mechanism is honestly allowed to be wrong.");
    }
}
