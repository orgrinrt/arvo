//! Probe 1: the grade projection does not dissolve the composition wall; it
//! stands clear of it by a structural property of its bound chain, and one
//! plausible refactor puts the wall inside the consumer-facing fold signature.
//!
//! File 47 (`47:434-441`) says the projection means "the `generic_const_exprs`
//! wall is not near the problem", and its probe 3 compiles clean, which I have
//! reproduced. But files 41 and 42 established a second, unrelated wall in this
//! exact terrain: naming a trait with one unconditional blanket impl as a bound
//! forces eager confirmation, and confirmation over `Reduce`'s chain diverges
//! (`41:section 3`, E0275; `42:185-227`, the corrected boundary; `46:section
//! 6.2`, it also fires on one rigid non-inhabitant). File 47's mechanism stays
//! clear of that wall only because every trait in its chain either
//! pattern-matches on constructor heads (`Cmp`, multi-candidate, deferred for
//! abstract inputs) or has finite non-recursive obligations
//! (`InteriorSafety`'s single blanket). Nothing in file 47 states that
//! property, and nothing pins it.
//!
//! CLAIM: one plausible refactor of the safety computation, "publish the
//! reduced headroom ratio", spelled as the where-clause `Ratio<Hd, Am1>:
//! Reduce` on the fold's own signature, reproduces the E0275 divergence at the
//! consumer-facing definition site. The positive control is file 47's probe 3
//! itself, rebuilt clean in this dispatch.
//!
//! EXPECTED: FAILS, E0275, `overflow evaluating the requirement`, the same
//! signature files 41/42/46 record. If this ever compiles, the solver changed
//! and every wall finding in this review needs re-grounding (grounded on: pin).
//!
//! Compiled as: rustc --edition 2021 --crate-type lib
//!   --extern tower=libtower.rlib probe_1_the_wall_is_one_refactor_away.rs
//!   (tower built from 47_probes/tower.rs, unmodified)

#![allow(dead_code)]

use tower::nat::{Pos, Ratio, Reduce};

/// The refactor a later hand would plausibly write: interior safety as a
/// reduced ratio of headroom to arity, computed by the tower's own `Reduce`
/// instead of by `Cmp`. The bound is never used by the body; per file 41's
/// probe 2 that does not matter, because the divergence is in confirming the
/// bound, not in using it.
pub fn regroup_fold_reduced_headroom<Hd, Am1>(_xs: &[i32])
where
    Hd: Pos,
    Am1: Pos,
    Ratio<Hd, Am1>: Reduce,
{
}
