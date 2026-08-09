// Probe 4, half two: the consumer-side crate (stands in for hilavitkutin
// or vehje, or any future arvo extension not written by arvo itself).
// Declares its own operation, `MyOp`, unknown to arvo at the time arvo
// was compiled, and asks two questions of the orphan rule against
// `04_orphan_rule_arvo_crate.rs`'s `Associative<Op>`:
//
//   can_extend_as_numeral_self: can this crate grant its own operation an
//     algebraic fact by writing `impl Associative<MyOp> for Number<N>`,
//     i.e. with arvo's Number<N> as Self? (Move A's actual shape)
//
//   can_extend_as_op_self: can this crate grant its own operation an
//     algebraic fact by writing `impl Associative<Number<N>> for MyOp`,
//     i.e. with the consumer's own MyOp as Self? (the operation-as-Self
//     reading the main file's section on laws-of-operations argues for)
//
// rustc +nightly-2026-05-28 --edition 2021 -L /tmp/orphan_probe \
//   --extern arvo_probe=/tmp/orphan_probe/liborphan_probe_arvo.rlib \
//   --crate-type lib --cfg as_numeral_self 05_orphan_rule_consumer_crate.rs
//   (expect: E0117, orphan rule violation, arvo owns both the trait and Self)
//
// rustc +nightly-2026-05-28 --edition 2021 -L /tmp/orphan_probe \
//   --extern arvo_probe=/tmp/orphan_probe/liborphan_probe_arvo.rlib \
//   --crate-type lib --cfg as_op_self 05_orphan_rule_consumer_crate.rs
//   (expect: compiles clean, this crate's MyOp is a local Self type)

extern crate arvo_probe;
use arvo_probe::{Associative, Fixed3, Number};

pub struct MyOp;

#[cfg(as_numeral_self)]
impl Associative<MyOp> for Number<Fixed3> {}

#[cfg(as_op_self)]
impl Associative<Number<Fixed3>> for MyOp {}
