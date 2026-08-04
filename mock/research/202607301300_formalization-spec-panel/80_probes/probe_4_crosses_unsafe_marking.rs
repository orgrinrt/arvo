// Probe 4, file 80. The `Crosses` marking: an `unsafe trait` admits no safe impl.
//
// The consolidated crossing-contract text says "Where the encoding is one the tower
// generates, the impl is blanket and safe (D16)" (68:271, carried into 78 section
// 1.4/1.23) about a trait declared `pub unsafe trait Crosses<N: Numeral>: Lowering`.
// Rust does not admit a safe impl of an unsafe trait: every impl of an unsafe trait
// is spelled `unsafe impl`, and a safe spelling is refused with E0200. So the
// sentence, read literally, does not compile. What IS expressible, and what D16's
// derived-safe/asserted-unsafe split has to mean here, is a single `unsafe impl`
// written once inside the trusted crate, with the safety argument discharged by
// construction (the Send/Sync shape), against per-declaration `unsafe impl`s at
// consumer sites for hand-laid layouts. The obligation's LOCATION differs; its
// syntactic form cannot.
//
// This file is the compiling half: the blanket `unsafe impl` inside the declaring
// module, plus a scoped demonstration that the bound is consumable. The refusing
// half (the safe impl, E0200) is probe_4b, kept as a separate file so this one
// stays green.
//
// Build: rustc --edition 2021 --crate-type=lib --emit=metadata. Compiles clean.
#![no_std]
#![allow(dead_code)]

pub trait Numeral {}
pub trait Lowering {
    // stand-in for Encoding/StoredWidth/Layout/Door
}

/// The obligation a Lowering owes a Numeral (statements 0 and P live here).
pub unsafe trait Crosses<N: Numeral>: Lowering {}

pub struct SomeNumeral;
impl Numeral for SomeNumeral {}

/// A tower-generated encoding: the marker the tower's own machinery attaches.
pub trait TowerGenerated<N: Numeral>: Lowering {}

pub struct GeneratedLowering;
impl Lowering for GeneratedLowering {}
impl TowerGenerated<SomeNumeral> for GeneratedLowering {}

// The derived route: one `unsafe impl`, written once, inside the trusted crate,
// its obligation discharged by construction for every tower-generated encoding.
// Syntactically it is still `unsafe impl`; "safe" can only describe who carries
// the proof obligation, never the spelling.
unsafe impl<N: Numeral, L: TowerGenerated<N>> Crosses<N> for L {}

// A hand-laid layout: the consumer's per-declaration assertion, same spelling,
// different prover.
pub struct HandLaid;
impl Lowering for HandLaid {}
unsafe impl Crosses<SomeNumeral> for HandLaid {}

// The bound is consumable by a door (and, per 68:270, by nothing law-shaped).
pub fn door<N: Numeral, L: Crosses<N>>(_l: &L) {}
