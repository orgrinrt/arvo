//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! A consumer with no feature line writes and reads the handle's fields in
//! constants. Accessors bound `[const] Lane` were refused here with `E0658`,
//! recorded in `mock/research/sketches/202611200614_packed-word-const-reach/`.

use arvo_bits::{Bits, Field, Layout, Word};
use arvo_format::Bool;
use notko_hlist::{Cons, Empty};

#[derive(Clone, Copy)]
struct Id;
#[derive(Clone, Copy)]
struct Kind;
#[derive(Clone, Copy)]
struct Flag;

impl Field for Id {
    type Value = Bits<28>;
}

impl Field for Kind {
    type Value = Bits<3>;
}

impl Field for Flag {
    type Value = Bool;
}

struct Handle;

impl Layout for Handle {
    type Container = Bits<32>;
    type Fields = Cons<Id, Cons<Kind, Cons<Flag, Empty>>>;
}

const HANDLE: Word<Handle> = Word::<Handle>::of(Bits::masked(0))
    .with(Id, Bits::masked(0x0abc_def1))
    .with(Kind, Bits::masked(5))
    .with(Flag, Bool::TRUE);
const KIND: Bits<3> = HANDLE.get(Kind);
const FLAG: Bool = HANDLE.get(Flag);
const RAW: Bits<32> = HANDLE.bits();

#[test]
fn a_consumer_with_no_feature_line_reads_and_writes_in_a_constant() {
    assert_eq!(KIND.raw(), 5);
    assert!(FLAG.get());
    assert_eq!(RAW.raw(), 0xdabc_def1);
}
