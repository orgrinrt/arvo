//! File 42's one argued-not-compiled residual (`42:360-364`), compiled.
//! Can a downstream crate add `impl Gcd<LocalRhs> for H`, a lying gcd on a
//! GENUINE sealed inhabitant, with the local type in the trait's parameter
//! position (orphan-legal shape per RFC 2451)? The question is whether
//! coherence admits it against the upstream blanket
//! `impl<B: Pos> Gcd<B> for H` (whose `B: Pos` cannot hold for `LocalRhs`,
//! but coherence may not be willing to conclude that).
//!
//! EXPECTED: unknown; that is why this compiles. Either E0119 (coherence
//! refuses, closing the residual outright) or clean (in which case
//! probe_4b shows the impl cannot reach any consuming position, file 42's
//! own argument, then compiled rather than argued).
//!
//! Compiled as: rustc --edition 2021 --crate-type lib
//!   --extern vu_core=libvu_core.rlib probe_4_gcd_local_rhs_coherence.rs

#![allow(dead_code)]

use vu_core::nat::{Gcd, H};

pub struct LocalRhs;

impl Gcd<LocalRhs> for H {
    type Out = H;
}
