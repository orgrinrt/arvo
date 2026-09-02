//! Probe 4: file 41's own composition-wall finding ("Reduce composes
//! safely only as a bare, top-level type alias") is corroborated but
//! narrower than stated. This file isolates the trigger down past where
//! file 41 stopped: it is not the Strip2/Gcd/ExactDivOdd/AsPos MACHINERY
//! that diverges when named as a bound, and it is not any wrapper
//! position in the abstract (two independent synthetic isolations of
//! that hypothesis, `Wrap<P>` over a bare parameter and `Wrap<P>` over an
//! unresolved projection, both compile clean during this investigation
//! and are not committed here because they are negative controls, not
//! findings; recorded in OUTCOMES.md). It is specifically NAMING THE
//! `Reduce` TRAIT ITSELF AS A BOUND (`T: Reduce`) that diverges, because
//! doing so forces the solver to SELECT and CONFIRM Reduce's one blanket
//! impl (discharging its own where-clauses eagerly, as part of
//! confirming the impl applies) rather than treating them as ordinary
//! deferred assumptions on the caller. Probe 4b is the bare `T: Reduce`
//! bound, unmodified from file 41's probe 2(b), which still fails.
//!
//! This file: the full where-clause chain from Reduce's own impl,
//! copied verbatim onto an unrelated function's signature, as ordinary
//! assumptions. Compiles clean. Nothing about the chain itself diverges.
//!
//! The consequence: a generic trait CAN compose the reduction machinery,
//! provided it spells the chain directly rather than naming `Reduce` as
//! a bound. Probe 5 builds one.
//!
//! Build: rustc --edition 2021 --crate-type lib \
//!        probe_4_reduce_chain_as_bare_bounds.rs --out-dir <dir>
//! Outcome: WORKS. rustc 1.98.0-nightly (57d06900f 2026-05-27).

#![allow(dead_code)]

#[path = "vu_nat.rs"]
mod nat;
use nat::{AsPos, ExactDivOdd, Gcd, Pos, Pz, Ratio, Strip2};

/// The exact where-clause chain Reduce's own impl requires, spelled
/// directly on an unrelated function. Compiles: these are ordinary
/// deferred assumptions, never confirmed at definition time, unlike a
/// bound naming `Reduce` itself (probe 4b).
fn full_chain_as_bare_bounds<N: Pos, D: Pos>()
where
    Ratio<N, D>: Strip2,
    <Ratio<N, D> as Strip2>::N: Gcd<<Ratio<N, D> as Strip2>::D>,
    Pz<<Ratio<N, D> as Strip2>::N>:
        ExactDivOdd<<<Ratio<N, D> as Strip2>::N as Gcd<<Ratio<N, D> as Strip2>::D>>::Out>,
    Pz<<Ratio<N, D> as Strip2>::D>:
        ExactDivOdd<<<Ratio<N, D> as Strip2>::N as Gcd<<Ratio<N, D> as Strip2>::D>>::Out>,
    <Pz<<Ratio<N, D> as Strip2>::N> as ExactDivOdd<
        <<Ratio<N, D> as Strip2>::N as Gcd<<Ratio<N, D> as Strip2>::D>>::Out,
    >>::Out: AsPos,
    <Pz<<Ratio<N, D> as Strip2>::D> as ExactDivOdd<
        <<Ratio<N, D> as Strip2>::N as Gcd<<Ratio<N, D> as Strip2>::D>>::Out,
    >>::Out: AsPos,
{
}
