#![no_std]

// Q31 probes. Every arm here answers one question: does arvo's own admission
// obligation refuse a set that the ratified factoring says the arithmetic
// happens in, and does it refuse it for a reason about the machine rather than
// about mathematics.
//
// Read `README.md` beside this for what each arm establishes and what its
// control is. Nothing here is shipping code and none of its spellings are
// design decisions.

use arvo_format::slots::{is_admissible, Slot, Slots};
use arvo_format::width::{Bool, Width};

/// A slot range written by hand at an arbitrary width, so a probe can name a
/// range the shipped `Signed`/`Unsigned` impl set does not reach.
///
/// The shipped impls stop at 62 by construction, so there is no way to ask the
/// question "what does the obligation say at 63" without writing the impl.
pub struct Grid<const MIN: i64, const MAX: i64, const BITS: u32>;

impl<const MIN: i64, const MAX: i64, const BITS: u32> Slots for Grid<MIN, MAX, BITS> {
    const MIN: Slot = Slot::at(MIN);
    const MAX: Slot = Slot::at(MAX);
    const WIDTH: Width = Width::bits(BITS);
}

/// The two's complement grid of `BITS` bits, as a `Grid`.
///
/// Written as a function rather than a type alias because the bounds are
/// arithmetic in the parameter and a type alias cannot compute them on this pin.
pub const fn twos_complement_bounds(bits: u32) -> (i128, i128) {
    let half = 1i128 << (bits - 1);
    (-half, half - 1)
}

/// The slot range the exact product of two two's complement grids lands in.
///
/// The ambient domain is closed under exact multiplication, which is what
/// `proposal::arithmetic_on_a_format_factors_as_an_adaptation_of_an_exact_operation`
/// requires for its sentence to name anything: the exact operation happens
/// there and the adaptation brings the result back. So the exact product of a
/// `w`-bit grid with itself is a real element of the ambient domain, and this is
/// the slot range it occupies.
pub const fn exact_product_bounds(bits: u32) -> (i128, i128) {
    let (lo, hi) = twos_complement_bounds(bits);
    // The four corner products. The extremes of the product of two intervals are
    // among the products of their endpoints.
    let a = lo * lo;
    let b = lo * hi;
    let c = hi * lo;
    let d = hi * hi;
    let mut min = a;
    if b < min { min = b; }
    if c < min { min = c; }
    if d < min { min = d; }
    let mut max = a;
    if b > max { max = b; }
    if c > max { max = c; }
    if d > max { max = d; }
    (min, max)
}

/// How many bits a slot index needs to address a span.
pub const fn bits_for_span(min: i128, max: i128) -> u32 {
    let span = max - min + 1;
    let mut n = 0u32;
    let mut acc = 1i128;
    while acc < span {
        acc <<= 1;
        n += 1;
    }
    n
}

/// The verdict form of the obligation, which does not force the const and so can
/// be asked about a construction that must keep compiling.
pub const fn admits<S: Slots>() -> Bool {
    is_admissible::<S>()
}
