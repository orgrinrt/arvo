//! The positive control the dispatch asks for by name: the seal must not
//! close the design against its own legitimate extension. Three
//! extensions a real downstream wants, all compiled against the fully
//! sealed vu_core, all EXPECTED CLEAN:
//!
//! 1. A new OPERATION over the sealed encoding, by structural recursion
//!    over the public constructors (`H`/`O`/`I` are public types; a LOCAL
//!    trait implemented per constructor is orphan-legal). Here: bit
//!    length, the shape any convention crate's derived-fact machinery
//!    takes. Const-asserted against genuine inhabitants.
//! 2. A new NUMERAL by composition: MATLAB's slope 1, bias 1/2 (file 39's
//!    witness, `39:135-136`), spelled from sealed parts. No new
//!    inhabitant needed, because every positive integer already has
//!    exactly one spelling, which is the point of the encoding.
//! 3. A new CONVENTION contract: a local trait whose associated types are
//!    bounded on the sealed traits, instantiated by a local marker type.
//!    This is the `conv-matlab`/`conv-systemc` crate shape in miniature:
//!    the contract layer stays open while the carrier layer is closed.
//!
//! Compiled as: rustc --edition 2021 --crate-type lib
//!   --extern vu_core=libvu_core.rlib probe_6_extension_positive_control.rs

#![allow(dead_code)]

use vu_core::bias::{Bias, ReducedBiasPos};
use vu_core::nat::{Adjustment, Reduced, H, I, O};

// --- 1. a new operation, structural recursion over sealed constructors ---

pub trait BitLen {
    const LEN: u64;
}
impl BitLen for H {
    const LEN: u64 = 1;
}
impl<P: BitLen> BitLen for O<P> {
    const LEN: u64 = P::LEN + 1;
}
impl<P: BitLen> BitLen for I<P> {
    const LEN: u64 = P::LEN + 1;
}

const _: () = assert!(<O<O<I<H>>> as BitLen>::LEN == 4); // 12 is four bits
const _: () = assert!(<H as BitLen>::LEN == 1);

// --- 2. the MATLAB numeral pieces, composed from sealed parts ---

/// slope 1: the adjustment 1/1, named through the normalising alias.
pub type MatlabSlope = Reduced<H, H>;
/// bias one half: the signed rational 1/2, normalised at the naming site.
pub type MatlabBias = ReducedBiasPos<H, O<H>>;

const _: () = assert!(<MatlabSlope as Adjustment>::NUM == 1);
const _: () = assert!(<MatlabSlope as Adjustment>::DEN == 1);
const _: () = assert!(<MatlabBias as Bias>::NUM == 1);
const _: () = assert!(<MatlabBias as Bias>::DEN == 2);

// --- 3. the convention contract: open trait over sealed carriers ---

pub trait Convention {
    type Slope: Adjustment;
    type Offset: Bias;
}

pub struct MatlabNumerictype;

impl Convention for MatlabNumerictype {
    type Slope = MatlabSlope;
    type Offset = MatlabBias;
}

// a derived fact read through the contract, the shape a law key uses
const _: () = assert!(<<MatlabNumerictype as Convention>::Offset as Bias>::DEN == 2);
