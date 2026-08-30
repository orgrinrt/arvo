#![no_std]
extern crate tower;
use tower::*;
type W13 = Pz<I<O<I<H>>>>; // 13
type W3 = Pz<I<H>>; // 3
type W0 = Z;
const _: () = assert!(<W13 as Nat>::VAL == 13);
const _: () = assert!(<W3 as Nat>::VAL == 3);
const _: () = assert!(<Sum<W13, W3> as Nat>::VAL == 17);
const _: () = assert!(<Sum<W13, W0> as Nat>::VAL == 13);
const _: () = assert!(<Sum<W0, W13> as Nat>::VAL == 13);
type W7 = Pz<I<I<H>>>; // 7
const _: () = assert!(<Sum<W7, W7> as Nat>::VAL == 14);
type W63 = Pz<I<I<I<I<I<H>>>>>>; // 63
const _: () = assert!(<Sum<W63, W63> as Nat>::VAL == 126);
const _: () = assert!(<Sum<W63, W3> as Nat>::VAL == 66);
