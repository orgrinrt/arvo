// Spelling C: the width is a type parameter. Five positions, bound and where first.
#![no_std]
#![allow(dead_code)]
extern crate arvo;
use arvo::*;

trait Sink<T> {
    fn take(&self, t: T);
}

// 4. a generic bound
fn bound<S: Sink<UFixedT<W13, W3, Warm>>>(_s: S) {}
// 5. a where clause
fn whereclause<S>(_s: S)
where
    S: Sink<UFixedT<W13, W3, Warm>>,
{
}
// 1. a value
fn value() {
    let _x: UFixedT<W13, W3, Warm>;
}
// 2. a struct field
struct Held {
    f: UFixedT<W13, W3, Warm>,
}
// 3. a function signature
fn sig(_x: UFixedT<W13, W3, Warm>) -> UFixedT<W13, W3, Warm> {
    _x
}

// and the thing spelling A cannot express at all: a width above the table
fn beyond(_x: UFixedT<W1000, W30, Warm>) {}
struct HeldBig {
    f: UFixedT<W40, W30, Warm>,
}

const _: () = assert!(<Sum<W13, W3> as Nat>::VAL == 16);
const _: () = assert!(<Sum<W1000, W30> as Nat>::VAL == 1030);
