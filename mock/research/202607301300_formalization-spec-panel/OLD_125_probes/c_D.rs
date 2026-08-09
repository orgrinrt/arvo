// Spelling D: digits munched through the tower's own arithmetic. Ten rows total.
#![no_std]
#![allow(dead_code)]
extern crate arvo;
use arvo::*;

trait Sink<T> {
    fn take(&self, t: T);
}
type UD<Iw, Fw> = UFixedT<Iw, Fw, Warm>;

// 4. bound   5. where
fn bound<S: Sink<UD<w!(1 3), w!(3)>>>(_s: S) {}
fn whereclause<S>(_s: S)
where
    S: Sink<UD<w!(1 3), w!(3)>>,
{
}
// 1. value  2. field  3. signature
fn value() {
    let _x: UD<w!(1 3), w!(3)>;
}
struct Held {
    f: UD<w!(1 3), w!(3)>,
}
fn sig(_x: UD<w!(1 3), w!(3)>) -> UD<w!(1 3), w!(3)> {
    _x
}

// the arithmetic itself, at values no table in this crate holds
const _: () = assert!(<w!(1 3) as Nat>::VAL == 13);
const _: () = assert!(<w!(0) as Nat>::VAL == 0);
const _: () = assert!(<w!(9) as Nat>::VAL == 9);
const _: () = assert!(<w!(4 0 9 5) as Nat>::VAL == 4095);
const _: () = assert!(<w!(6 5 5 3 5) as Nat>::VAL == 65535);
const _: () = assert!(<w!(1 0 0 0 0 0 0) as Nat>::VAL == 1000000);
const _: () = assert!(<Sum<w!(4 0), w!(3 0)> as Nat>::VAL == 70);
