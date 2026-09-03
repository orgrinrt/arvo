//! Arm H. The coordinate on its own, in a const item, with no `Format` anywhere.
//!
//! `Phase` carries no obligation of its own, so nothing should refuse this. It
//! is the arm that says where the enforcement is not: not in the constructor,
//! not in the type, not at const evaluation of the coordinate.
//!
//! The control is arm B, which shares the same constructor call and refuses,
//! which is what makes this arm a statement about the locus rather than about
//! the value.

use arvo_format::format::Phase;

const NOWHERE: Phase = Phase::of(1, 0);

fn main() {
    println!(
        "H: {}/{} sat in a const item with nothing to say about it",
        NOWHERE.numerator(),
        NOWHERE.denominator()
    );
}
