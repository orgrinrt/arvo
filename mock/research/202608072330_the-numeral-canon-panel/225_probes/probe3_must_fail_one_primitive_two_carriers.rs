// Probe 3's must-fail companion, seat 225. DOES NOT COMPILE, BY DESIGN.
//
// The claim under test: "a 13-bit declared numeral carried in u16 and the same
// declaration carried in u32 are one primitive, with the footprint inside the
// contract". If that claim held, the assertion below would be writable and true.
//
// rustc refuses it at const-evaluation time, and the refusal is the finding:
// once the footprint is inside the contract, two carriers cannot be one
// primitive, not as a wrong value but as an unexpressible sentence. The stderr
// is committed beside this file.

use core::mem::size_of;

#[allow(dead_code)]
struct Declared13InU16(u16);
#[allow(dead_code)]
struct Declared13InU32(u32);

// "one primitive" with footprint in the contract demands one footprint:
const _: () = assert!(
    size_of::<Declared13InU16>() == size_of::<Declared13InU32>(),
    "one primitive, two carriers: the footprint contract cannot be satisfied"
);

fn main() {}
