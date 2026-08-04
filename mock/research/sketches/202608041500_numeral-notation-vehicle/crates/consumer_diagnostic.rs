//! FAILS test, on purpose: does MY vehicle's own emitted face survive a
//! type mismatch legibly, or does it decay to the encoding the moment an
//! ordinary equality check fires? File 56 section 7 already compiled this
//! question against a hand-written toy face; this file reproduces it
//! against a face THIS macro actually emits, because file 56's own finding
//! was general ("a raw type-equality mismatch always prints the fully
//! expanded type, for an alias OR A MACRO-PRODUCED ALIAS ALIKE") but had
//! not been checked against a macro that computes its encoding from
//! digits rather than from a hand-typed const generic.
//!
//! Two distinct numerals, `N37` and `N38`, one character apart in decimal
//! spelling (the brief's own near-miss example), a function that wants
//! `N37` specifically, called with `N38`.

#[path = "tower.rs"]
mod tower;
use tower::*;

extern crate numeral_pm;
use numeral_pm::numeral_face;

numeral_face!(N37 = 37);
numeral_face!(N38 = 38);

fn needs_n37(_: N37) {}

fn main() {
    needs_n37(N38);
}
