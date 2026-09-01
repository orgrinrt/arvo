// `Quantum::BASE` retyped to `Width`, at the exponent fixed point uses.
//
// `Constant<EXP>` sets `BASE = EXP`, and the crate's own doc says `EXP = -F` is
// fixed point at fraction width `F`. So the negative exponent is not an edge
// case of this coordinate, it is the case that distinguishes fixed point from
// the integers.

use arvo_format::width::Width;

pub trait QuantumByWidth {
    const BASE: Width;
}

pub struct MyLaw;

impl QuantumByWidth for MyLaw {
    const BASE: Width = Width::bits(-4);
}

fn main() {}
