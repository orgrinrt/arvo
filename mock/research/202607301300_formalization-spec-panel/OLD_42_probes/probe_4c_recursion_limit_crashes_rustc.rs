//! Probe 4c: NOT part of the standard build sweep, and expected to crash
//! the compiler. Do not add this to a CI loop or a script that builds
//! every file in this directory unattended.
//!
//! `rustc`'s own diagnostic for probe 4b suggests "consider increasing
//! the recursion limit by adding `#![recursion_limit = "256"]`". Raising
//! it to 4096 on the identical bare-Reduce-bound file does not produce a
//! clean, deeper answer; it crashes the compiler with SIGBUS inside
//! `rustc_trait_selection`'s opportunistic variable resolver, confirmed
//! reproducible on the pinned nightly. This is independent, stronger
//! evidence than a bare "overflow" diagnostic that the divergence
//! probe 4b names is a genuine unbounded search (the `Pz<O<O<O<...>>>>`
//! type growing without a fixed point), not a shallow default-limit
//! artifact a larger budget would clear: the failure mode is a crash,
//! not a slower success.
//!
//! Filed as a candidate upstream rustc issue is out of scope for this
//! dispatch; the reproducer below is the clean minimal case an issue
//! would want, kept here as the audit trail.
//!
//! Build: rustc --edition 2021 --crate-type lib \
//!        probe_4c_recursion_limit_crashes_rustc.rs --out-dir <dir>
//! Outcome: rustc crashes, SIGBUS, inside
//! `_RINvXso_...structural_impls...TypeSuperFoldableNtNtB8_7context6TyCtxt
//! E15super_fold_with...OpportunisticVarResolver...`, verbatim backtrace
//! head in OUTCOMES.md. rustc 1.98.0-nightly (57d06900f 2026-05-27).

#![recursion_limit = "4096"]
#![allow(dead_code)]

#[path = "vu_nat.rs"]
mod nat;
use nat::{Pos, Ratio, Reduce};

fn bare_reduce_bound<N: Pos, D: Pos>()
where
    Ratio<N, D>: Reduce,
{
}
