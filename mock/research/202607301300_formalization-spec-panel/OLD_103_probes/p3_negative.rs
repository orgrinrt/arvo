//! Probe 3. The typestate seal, checked across the crate boundary D15's
//! placement creates. The sketch verified it inside one crate; D15 puts the
//! wrapper in notko and the closure in a consumer, so the seal has to survive
//! that split or the type parameter is decoration.
#![no_std]
#![allow(dead_code)]
use p1_arvo::Bool;
use p1_foundation::{Cons, Nil, Pred};
type L2<A, B> = Cons<A, Cons<B, Nil>>;
type L1<A> = Cons<A, Nil>;

// NEGATIVE: declares arity two, supplies arity one.
pub fn liar() -> Pred<L2<u32, u32>, impl Fn(&u32) -> Bool> {
    Pred::new(|a: &u32| Bool::new(*a > 0))
}
