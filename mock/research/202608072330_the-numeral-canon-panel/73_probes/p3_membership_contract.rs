//! p3. Is the broad reading expressible with no new coordinate, and is p1d's
//! repair expressible as a bound?
//!
//! HYPOTHESES, written before the run.
//!
//!   H1. A carrier with a closed operation family and no magnitude, GF(2)^4
//!       under xor, is a term of the SAME telescope-shaped contract that hosts
//!       Z/16 under wrapping addition. It needs no new coordinate and no new
//!       trait: it is the term whose reduction is the identity because its
//!       ambient operation is closed on its representable set. That is `67`'s
//!       K5 (`67:641-645`) used as a construction rather than as a remark. If
//!       this compiles, the broad reading of Q21 costs zero mechanism.
//!
//!   H2. p1d found that the induced law is the conjunction of the AMBIENT
//!       domain's own law and the reduction's coherence verdict, 16 of 16 cells.
//!       I predict that conjunction is expressible as a blanket implication
//!       rather than as a per-instance assertion, so a design cannot forget it.
//!
//!   H3. And the failure mode is expressible too: a term that names its own
//!       computed algebra as its ambient domain (p1's collapse) is COHERENT and
//!       must still be refused. I predict a bound written on coherence alone
//!       ACCEPTS it, and the conjunction REFUSES it, in the same file, so the
//!       difference between the two designs is a compile outcome rather than an
//!       argument.
//!
//! PROBE DISCIPLINE. This is a spike. Its names, arities and trait shapes are
//! scaffolding chosen to reach the check and are not a design proposal. Every
//! law implementation below is a TRANSCRIBED MEASUREMENT from `73_probes/p1d`
//! and `73_probes/p1`, never a proof, and the transcription is stated at each
//! impl so nobody cites an impl as a fact.
//!
//! Pinned nightly-2026-05-28. Zero feature gates. No `dyn`, no `TypeId`, no
//! `alloc`, no `std`.

#![no_std]

use core::marker::PhantomData;

// ---------------------------------------------------------------------------
// coordinate 1: the ambient domain, with its OWN law inventory exposed
// ---------------------------------------------------------------------------

/// A carrier with an operation, in which the operation is exact.
pub trait Ambient {
    /// where the exact result lives. Wider than the representable set in
    /// general; equal to it exactly when the operation is closed.
    type Exact: Copy;
    fn embed(v: u8) -> Self::Exact;
    fn combine(a: Self::Exact, b: Self::Exact) -> Self::Exact;
}

/// The ambient domain's own associativity. This is the item `71` X3's exposure
/// list omits and `73_probes/p1d` found load-bearing: without it a reduction's
/// verdicts are a fact about which ambient domain was named.
pub trait AmbientAssociates: Ambient {}

/// The integers under addition. The exact result of adding two window values
/// leaves the window, which is what makes the reduction a real choice.
pub struct RingZ;
impl Ambient for RingZ {
    type Exact = i16;
    fn embed(v: u8) -> i16 {
        v as i16
    }
    fn combine(a: i16, b: i16) -> i16 {
        a + b
    }
}
// transcribed: `p1d` row "i4 saturate", amb-assoc = True (integer addition
// associates on the reachable set, exhaustively at the model width).
impl AmbientAssociates for RingZ {}

/// GF(2)^4 under xor. No magnitude, no order compatible with the operation
/// (`73_probes/p2`: zero compatible total orders at widths 2 and 3, and every
/// non-identity element of order 2 at every width). Its operation is CLOSED.
pub struct Gf2v4;
impl Ambient for Gf2v4 {
    type Exact = u8;
    fn embed(v: u8) -> u8 {
        v
    }
    fn combine(a: u8, b: u8) -> u8 {
        (a ^ b) & 0x0F
    }
}
// transcribed: `p1d` row "gf(2)^4 xor", amb-assoc = True.
impl AmbientAssociates for Gf2v4 {}

/// The collapsed declaration of the signed saturating numeral: it names its own
/// computed algebra as its ambient domain. `73_probes/p1` measures that this
/// computes the identical function and reports both law verdicts clean.
pub struct SatOwnAlgebra;
impl Ambient for SatOwnAlgebra {
    type Exact = i16;
    fn embed(v: u8) -> i16 {
        v as i16
    }
    fn combine(a: i16, b: i16) -> i16 {
        let s = a + b;
        if s < -8 {
            -8
        } else if s > 7 {
            7
        } else {
            s
        }
    }
}
// DELIBERATELY ABSENT: `impl AmbientAssociates for SatOwnAlgebra`.
// transcribed: `p1d` row "i4 saturate [collapsed]", amb-assoc = False, 952 of
// 4096 triples. This absence is the whole of H3.

// ---------------------------------------------------------------------------
// coordinate 2: the representable set, a constant of the type
// ---------------------------------------------------------------------------

pub trait Reach {
    type Amb: Ambient;
    const LO: i16;
    const HI: i16;
}

pub struct U4Window;
impl Reach for U4Window {
    type Amb = RingZ;
    const LO: i16 = 0;
    const HI: i16 = 15;
}

pub struct Gf2Full;
impl Reach for Gf2Full {
    type Amb = Gf2v4;
    const LO: i16 = 0;
    const HI: i16 = 15;
}

pub struct S4Collapsed;
impl Reach for S4Collapsed {
    type Amb = SatOwnAlgebra;
    const LO: i16 = -8;
    const HI: i16 = 7;
}

// ---------------------------------------------------------------------------
// coordinate 3: the reduction, and the closure fact that forces it
// ---------------------------------------------------------------------------

pub trait Reduce {
    type Over: Reach;
    fn adapt(e: <<Self::Over as Reach>::Amb as Ambient>::Exact) -> u8;
}

/// The reduction is coherent: a homomorphism from the ambient onto the induced
/// operation. Transcribed measurements only.
pub trait Coherent: Reduce {}

/// The induced operation associates. NOT implemented per instance. It is the
/// conjunction, so a design cannot state one half and forget the other.
pub trait InducedAssociates {}
impl<R> InducedAssociates for R
where
    R: Coherent,
    <<R as Reduce>::Over as Reach>::Amb: AmbientAssociates,
{
}

pub struct WrapU4;
impl Reduce for WrapU4 {
    type Over = U4Window;
    fn adapt(e: i16) -> u8 {
        (e & 0x0F) as u8
    }
}
// transcribed: `p1d` row "u4 wrap", coherent = True (0 of 961 failures in `p1`).
impl Coherent for WrapU4 {}

/// The identity reduction. It is not a choice here: the ambient operation is
/// closed on the representable set, so nothing can leave and the only total
/// reduction that retracts is the identity.
pub struct IdentGf2;
impl Reduce for IdentGf2 {
    type Over = Gf2Full;
    fn adapt(e: u8) -> u8 {
        e
    }
}
// transcribed: `p1d` row "gf(2)^4 xor", coherent = True.
impl Coherent for IdentGf2 {}

pub struct IdentCollapsed;
impl Reduce for IdentCollapsed {
    type Over = S4Collapsed;
    fn adapt(e: i16) -> u8 {
        (e & 0x0F) as u8
    }
}
// transcribed: `p1d` row "i4 saturate [collapsed]", coherent = True. This impl
// is HONEST: the collapsed term really is coherent. That is the danger.
impl Coherent for IdentCollapsed {}

// ---------------------------------------------------------------------------
// closure, computed at compile time, at the model width
// ---------------------------------------------------------------------------

/// Is the ambient operation closed on the representable set? Computed, not
/// declared, which is the discipline `68` section 2.2 found missing when a
/// declared window could be overstated with no diagnostic.
const fn xor_closed_on_window() -> bool {
    let mut a = 0u8;
    while a < 16 {
        let mut b = 0u8;
        while b < 16 {
            let r = (a ^ b) & 0x0F;
            if r > 15 {
                return false;
            }
            b += 1;
        }
        a += 1;
    }
    true
}

const fn add_closed_on_window() -> bool {
    let mut a = 0i16;
    while a < 16 {
        let mut b = 0i16;
        while b < 16 {
            let r = a + b;
            if r > 15 {
                return false;
            }
            b += 1;
        }
        a += 1;
    }
    true
}

// The same function shape returns different answers for the two ambients, so
// neither const block is a tautology: one of them is FALSE.
const _: () = assert!(xor_closed_on_window());
const _: () = assert!(!add_closed_on_window());

// ---------------------------------------------------------------------------
// erasure
// ---------------------------------------------------------------------------

#[repr(transparent)]
pub struct Num<R: Reduce>(u8, PhantomData<R>);

const _: () = assert!(core::mem::size_of::<Num<WrapU4>>() == 1);
const _: () = assert!(core::mem::size_of::<Num<IdentGf2>>() == 1);
const _: () = assert!(core::mem::size_of::<Num<IdentCollapsed>>() == 1);

// ---------------------------------------------------------------------------
// the two designs, side by side, in one file
// ---------------------------------------------------------------------------

/// The design whose bound is the CONJUNCTION. This is p1d's repair.
pub fn reassociating_fold<R>(xs: &[u8]) -> u8
where
    R: Reduce + InducedAssociates,
{
    let mut acc = <<R::Over as Reach>::Amb as Ambient>::embed(0);
    let mut i = 0;
    while i < xs.len() {
        acc = <<R::Over as Reach>::Amb as Ambient>::combine(
            acc,
            <<R::Over as Reach>::Amb as Ambient>::embed(xs[i]),
        );
        i += 1;
    }
    R::adapt(acc)
}

/// The design whose bound reads only the reduction's own verdict, which is what
/// an exposure list omitting the ambient's laws would give a consumer.
pub fn reassociating_fold_verdict_only<R>(xs: &[u8]) -> u8
where
    R: Coherent,
{
    let mut acc = <<R::Over as Reach>::Amb as Ambient>::embed(0);
    let mut i = 0;
    while i < xs.len() {
        acc = <<R::Over as Reach>::Amb as Ambient>::combine(
            acc,
            <<R::Over as Reach>::Amb as Ambient>::embed(xs[i]),
        );
        i += 1;
    }
    R::adapt(acc)
}

// H1: the magnitude-free carrier and the arithmetic one are terms of one
// contract, accepted by the same bound, with no coordinate added for either.
pub fn h1_broad_reading_costs_no_mechanism(xs: &[u8]) -> (u8, u8) {
    (
        reassociating_fold::<WrapU4>(xs),
        reassociating_fold::<IdentGf2>(xs),
    )
}

// H3, the half that compiles: the WEAK bound accepts the collapsed term.
// This is the unsound design type-checking, kept in the positive file on
// purpose, because the finding is that both designs are writable and only the
// bound tells them apart.
pub fn h3_weak_bound_accepts_the_collapse(xs: &[u8]) -> u8 {
    reassociating_fold_verdict_only::<IdentCollapsed>(xs)
}

// H3, the half that must be refused, lives in the generated negatives:
//   n1: reassociating_fold::<IdentCollapsed>   -- the conjunction refuses it
//   n2: a reduction declared over one ambient attached to a reach over another
