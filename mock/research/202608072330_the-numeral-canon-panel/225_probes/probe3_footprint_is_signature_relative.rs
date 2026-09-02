// Probe 3, seat 225. The footprint observation is induced by a signature, and which
// signatures exist is decided by the realisation, not by the numeral's declaration.
//
// Three compile-time facts, each shown rather than argued:
//
//   fact 1: a standalone numeral type cannot make its footprint unobservable.
//     `size_of` is ambient, const, and total over Sized types; no design choice
//     removes it. Shown by const-evaluating it for two carriers of one declaration.
//
//   fact 2: two carriers for one declaration are two observably different types.
//     Shown by the same const observation returning 2 and 4.
//
//   fact 3: at shared occupancy no per-element footprint observation exists.
//     Five 13-bit elements packed into a dense bit stream occupy ceil(65/8) = 9
//     bytes; 9 is not 5 times any whole per-element byte count, so there is no
//     value a per-element `size_of` could return. The observation is not hidden,
//     it is nonexistent: the packed element is not a place, so no signature over
//     it can be written at all.
//
// The must-fail case is the companion file
// `probe3_must_fail_one_primitive_two_carriers.rs`, which asserts at const time
// that the two carriers of fact 2 have equal footprint. It does not compile, and
// its rustc stderr is committed beside it. That is the strongest form available:
// the claim "one primitive, two carriers, footprint inside the contract" has no
// expressible form, rather than a wrong value.
//
// holds for: W = 13, container in {u16, u32, dense bit stream}, threads = 1,
// toolchain = the committed rustc in probe3_out.txt. Fact 1 and fact 2 are
// compile-time facts; access pattern: any (no data path exists in the probe).

use core::mem::size_of;

#[allow(dead_code)]
struct Declared13InU16(u16); // one declaration, carrier u16
#[allow(dead_code)]
struct Declared13InU32(u32); // same declaration, carrier u32

// fact 1 and fact 2: the observation is const, ambient, and separates the carriers.
const FOOTPRINT_U16: usize = size_of::<Declared13InU16>();
const FOOTPRINT_U32: usize = size_of::<Declared13InU32>();
const SEPARATED: bool = FOOTPRINT_U16 != FOOTPRINT_U32;

// fact 3: shared occupancy. 5 elements of 13 bits in a dense stream.
const ELEMENTS: usize = 5;
const DECLARED_BITS: usize = 13;
const STREAM_BYTES: usize = (ELEMENTS * DECLARED_BITS + 7) / 8; // 9

fn main() {
    println!("fact 1: footprint is const-observable through the ambient layout signature");
    println!("  size_of::<Declared13InU16>() = {FOOTPRINT_U16}");
    println!("  size_of::<Declared13InU32>() = {FOOTPRINT_U32}");

    println!("fact 2: the observation separates the two carriers: {SEPARATED}");
    if !SEPARATED {
        println!("  INSTRUMENT BROKEN: two different carriers report one footprint");
        std::process::exit(2);
    }

    println!("fact 3: shared occupancy has no per-element footprint");
    println!("  {ELEMENTS} elements x {DECLARED_BITS} bits pack into {STREAM_BYTES} bytes");
    // If a per-element byte footprint existed it would be STREAM_BYTES / ELEMENTS
    // exactly. It is not exact, so no per-element observation can exist.
    if STREAM_BYTES % ELEMENTS == 0 {
        println!("  UNEXPECTED: stream divides evenly; pick a width where it does not");
        std::process::exit(2);
    }
    println!("  9 / 5 is not whole: there is no value a per-element size could return.");
    println!();
    println!("see probe3_must_fail_one_primitive_two_carriers.stderr for the must-fail case");
}
