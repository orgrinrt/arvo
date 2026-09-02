// p6.  Is the post-fixpoint sufficiency condition expressible as a BOUND, and
// does it erase?
//
// p5 measured that the fold's accumulator sufficiency condition, under a
// saturating resolution read as absorbing at the top, reduces to one inequality
// on declared members: the accumulator's fraction width is at least the
// element's.  Zero unsound sequences on and above that diagonal, nonzero
// strictly below, over 65,536 sequences per cell (p5.out Q4).
//
// A measured inequality is not a design until it can be stated where the design
// states things.  The obvious spelling puts an inequality over two const
// parameters into a where clause, which needs a forbidden feature.  The
// workspace rule for that is to break the constraint into named contracts that
// each hold on their own and compose, so this probe does exactly that and
// compares two spellings:
//
//   ARM A  a post-monomorphisation assert on an associated const.  Compiles for
//          any pair and fails only when the offending instantiation is reached.
//          The record already rules on that hole, so this arm exists to be the
//          thing the other arm is better than.
//
//   ARM B  a sealed inductive Le relation on a type-level nat, bound in the
//          where clause.  Refuses at type-check, before monomorphisation.
//
// And an erasure check, because the acceptance criterion requires the typestate
// to erase on lowering: the guarded fold must emit what the unguarded fold emits.
//
// Gate-free.  No feature attributes at all; if any of this needed a forbidden
// feature the file would not build, which is the point of leaving the gate list
// empty rather than writing one out.

#![no_std]
#![allow(dead_code)]

include!("p6_core.rs");
