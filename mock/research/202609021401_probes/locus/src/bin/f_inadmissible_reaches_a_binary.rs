//! Arm F. Whether an inadmissible declaration reaches a produced binary.
//!
//! `arvo-format/DESIGN.md.tmpl:684` says the guarantee is that "an inadmissible
//! range or an inadmissible law cannot reach a produced binary", and `:737`
//! repeats it for the phase denominator. An obligation is a const, and a const
//! is evaluated where it is used, so the claim holds only if every route from a
//! declaration to a binary passes through a use of `ADMITTED`.
//!
//! This arm takes the other route. It declares the inadmissible format, reads
//! its `PHASE` coordinate straight off the impl, and forces nothing. Nothing in
//! it is contrived: reading an associated const of a trait one implements is the
//! ordinary thing to do with it.
//!
//! Predicted, if the design's sentence is right: `cargo build` refuses.
//! Predicted, if the obligation is use-forced: `cargo build` succeeds, runs, and
//! prints a zero denominator out of a binary that was produced.
//!
//! The control is arm B, which shares this declaration verbatim and differs only
//! in reaching it through a function that forces the obligation. If B refuses and
//! F does not, the difference is the forcing site and not the declaration.

use arvo_format::ambient::BinaryRationals;
use arvo_format::format::{Format, Phase};
use arvo_format::quantum::Constant;
use arvo_format::slots::Signed;

struct NoDenominator;

impl Format for NoDenominator {
    type Ambient = BinaryRationals;
    type Quantum = Constant<0>;
    type Slots = Signed<8>;

    const PHASE: Phase = Phase::of(1, 0);
}

fn main() {
    let p = <NoDenominator as Format>::PHASE;
    println!(
        "F: an inadmissible declaration reached a binary. PHASE = {}/{}, denotes={}",
        p.numerator(),
        p.denominator(),
        p.denotes().get()
    );
}
