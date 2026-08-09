// Spelling B: a call-site macro. Bound and where clause first, per the brief.
#![no_std]
#![allow(dead_code)]
#[macro_use]
extern crate arvo;
use arvo::*;

trait Sink<T> {
    fn take(&self, t: T);
}

// 4. a generic bound
fn bound<S: Sink<ufixed!(13, 3, Warm)>>(_s: S) {}
// 5. a where clause
fn whereclause<S>(_s: S)
where
    S: Sink<ufixed!(13, 3, Warm)>,
{
}
// 1. a value
fn value() {
    let _x: ufixed!(13, 3, Warm);
}
// 2. a struct field
struct Held {
    f: ufixed!(13, 3, Warm),
}
// 3. a function signature
fn sig(_x: ufixed!(13, 3, Warm)) -> ufixed!(13, 3, Warm) {
    _x
}
