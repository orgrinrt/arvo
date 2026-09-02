// Seat q31a. The decisive instrument for Q31, in the shipped crate's own types.
//
// Q31's distinguisher is whether the canon ever says something true of a system
// arvo cannot host. The sharper version, decidable here rather than by reading:
// does an `ADMITTED` obligation ever refuse a candidate that *is* a number
// system? If yes, the obligation is not a statement about what a number system
// is, and calling it by the same word as one is the conflation Q31 names.
//
// Four candidates, in two pairs, each pair a refusal and its control.
//
// Pair A, the concept kind. `Unary` fixes radix 1. Refused, and there is no
// number system behind the refusal: at radix 1 the quantum never varies with the
// exponent, so every magnitude names the same value. Nothing is lost.
//
// Pair B, the hosting kind. `Sixty3` fixes a 63-bit slot range. Refused, and the
// thing refused is a perfectly good number system: the 63-bit two's complement
// integers, which every reader of this file can name and which several machines
// carry. What refuses it is `2^63` not fitting the signed 64-bit integer this
// crate counts slots in.
//
// The verdict form (`is_admissible*`) is what this file reads, because it returns
// rather than asserts, so both members of each pair can be constructed and
// compared in one run. The const form refuses at codegen and would take the file
// with it.
//
// Run: `refused_candidates.sh`.

use arvo_format::ambient::{is_admissible_ambient, Ambient, Radix};
use arvo_format::slots::{is_admissible, Slots};
use arvo_format::width::{Bool, Width};
use arvo_format::slots::Slot;

// ---- pair A: the concept kind -------------------------------------------------

/// Radix one. Refused by `Ambient::ADMITTED`.
struct Unary;
impl Ambient for Unary {
    const RADIX: Radix = Radix::of(1);
    const SIGNED: Bool = Bool::TRUE;
}

/// The control: radix two, the same shape otherwise.
struct Binary;
impl Ambient for Binary {
    const RADIX: Radix = Radix::of(2);
    const SIGNED: Bool = Bool::TRUE;
}

/// The rationals at radix two, shipped by the crate itself as an ambient domain.
use arvo_format::ambient::BinaryRationals;

// ---- pair B: the hosting kind -------------------------------------------------

/// A 63-bit two's complement slot range. Refused by `Slots::ADMITTED`.
struct Sixty3;
impl Slots for Sixty3 {
    const MIN: Slot = Slot::at(-(1i64 << 62));
    const MAX: Slot = Slot::at((1i64 << 62) - 1);
    const WIDTH: Width = Width::bits(63);
}

/// The control: 62 bits, one narrower, the same shape otherwise.
struct Sixty2;
impl Slots for Sixty2 {
    const MIN: Slot = Slot::at(-(1i64 << 61));
    const MAX: Slot = Slot::at((1i64 << 61) - 1);
    const WIDTH: Width = Width::bits(62);
}

fn main() {
    println!("== pair A, the concept kind ==");
    println!("  radix 1 admitted as an ambient domain : {}", is_admissible_ambient::<Unary>().get());
    println!("  radix 2 admitted as an ambient domain : {}", is_admissible_ambient::<Binary>().get());
    println!("  the crate's own BinaryRationals       : {}", is_admissible_ambient::<BinaryRationals>().get());

    println!("== pair B, the hosting kind ==");
    println!("  63-bit slot range admitted            : {}", is_admissible::<Sixty3>().get());
    println!("  62-bit slot range admitted            : {}", is_admissible::<Sixty2>().get());

    println!();
    println!("== what the two refusals are about ==");
    println!("  A refuses radix 1. Behind it: nothing. A radix-1 positional");
    println!("  notation names one value at every magnitude, so no set of values");
    println!("  is being kept out. The obligation is about what a number system is.");
    println!();
    println!("  B refuses a 63-bit range. Behind it: the 63-bit two's complement");
    println!("  integers, which are a number system by any account and which this");
    println!("  crate refuses because 2^63 does not fit the i64 it counts slots in.");
    println!("  The obligation is about what this implementation carries.");
    println!();
    println!("  And the crate admits BinaryRationals, whose domain is the rationals,");
    println!("  which arvo cannot host at all: I14 is in force and forbids alloc and");
    println!("  runtime growth, so no representable set here is the rationals. The");
    println!("  same crate therefore admits an unhostable domain at one obligation");
    println!("  and refuses a hostable system at another, calling both `ADMITTED`.");
}
