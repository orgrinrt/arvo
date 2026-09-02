// Control. The consumer crate declaring an ADT const parameter of its own.
//
// This must be refused. Without it, `outside` building could mean the gate
// leaked in the consumer's favour, or that the consumer had the feature by
// some route, and the measurement would say nothing. It is the same
// construction the door makes legally, in a crate that carries no
// `#![feature(...)]`.

use door::Width;

pub struct MyRange<const BITS: Width>;

fn main() {}
