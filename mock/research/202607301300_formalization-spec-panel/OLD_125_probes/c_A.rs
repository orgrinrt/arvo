// Spelling A, incumbent: const parameters + the table. Five positions.
#![no_std]
#![allow(dead_code)]
extern crate arvo;
use arvo::*;

trait Sink<T> {
    fn take(&self, t: T);
}

// 1. a value
fn value() {
    let _x: UFixed<13, 3, Warm>;
}
// 2. a struct field
struct Held {
    f: UFixed<13, 3, Warm>,
}
// 3. a function signature
fn sig(_x: UFixed<13, 3, Warm>) -> UFixed<13, 3, Warm> {
    _x
}
// 4. a generic bound
fn bound<S: Sink<UFixed<13, 3, Warm>>>(_s: S) {}
// 5. a where clause
fn whereclause<S>(_s: S)
where
    S: Sink<UFixed<13, 3, Warm>>,
{
}

const _: () = assert!(<<FixedNumeral<Sum<NatOf<{13}>, NatOf<{3}>>, NonNegative> as Numeral>::Precision as Nat>::VAL == 16);
