// Probe 1. The crossing contract's statement 2 is not merely unchecked under a
// hole. It is ill-typed.
//
// `63:186-191` states three statements:
//
//   1. decode . encode = id  on values, always.
//   2. encode . decode  is idempotent on data, always (canonicalisation).
//   3. encode . decode = id on data iff the encoding is injective.
//
// File 66 section 6 found that none of the three says decode lands in the value
// set, and proposed a fourth statement in front. This probe asks a sharper
// question: what happens to statements 2 and 3 when decode escapes V?
//
// The answer is that they stop being propositions. `encode . decode` requires
// decode's output to be in encode's DOMAIN, and encode's domain is the value
// set (statement 1 says so: it quantifies "on values"). The design does not
// have a total `encode : Q -> D`; it has `encode : V -> D` and, separately, a
// quantiser `Q -> V + {Overflow, UnderflowRefused}` (`63:200-206`), which is
// partial by construction.
//
// So statement 0 is not one more statement in a list of four. It is the
// well-formedness side condition of two of the existing three. That decides
// "in front of" on structural grounds rather than on stylistic ones.
//
// This file types the three maps honestly and writes statement 2 out. It is
// EXPECTED TO FAIL TO COMPILE. The compiler error is the finding.

#![allow(dead_code)]

#[path = "model.rs"]
mod model;
use model::*;

/// A datum: a field tuple the physical encoding can hold. Nothing about it
/// promises that it denotes anything the numeral has.
#[derive(Clone, Copy, Debug)]
pub struct Datum {
    pub m: i128,
    pub q: i32,
}

/// A value: an element of V(N). The only constructor checks membership, which
/// is what makes this type mean what its name says. This is the perimeter
/// discipline `what-you-can-observe-is-what-you-guaranteed.md` states: the
/// field is private and `new` is the only door.
#[derive(Clone, Copy, Debug)]
pub struct Value {
    inner: Val,
}

impl Value {
    /// The only constructor. Refuses anything outside V(N).
    pub fn new(f: &Fmt, v: Val) -> Option<Value> {
        if v.is_zero() || enumerate(f).iter().any(|w| w.eq_exact(&v, f.r)) {
            Some(Value { inner: v })
        } else {
            None
        }
    }
    pub fn get(&self) -> Val {
        self.inner
    }
}

/// encode : V -> D. Total on values, by construction: every value has a datum.
pub fn encode(_f: &Fmt, v: Value) -> Datum {
    let x = v.get();
    Datum { m: x.m, q: x.q }
}

/// decode : D -> Q. This is the honest typing. The decode map is the numeral's
/// own arithmetic on the physical fields, `m * r^q`. That formula is total on
/// the field tuple and lands in the rationals. Nothing in it knows what V is.
pub fn decode(_f: &Fmt, d: Datum) -> Val {
    Val { m: d.m, q: d.q }
}

/// Statement 2, written out: `encode . decode` on data.
///
/// This does not compile. `decode` returns a `Val` (a rational) and `encode`
/// takes a `Value` (an element of V). There is no coercion, because supplying
/// one is exactly the obligation statement 0 names.
pub fn statement_two_canonicalise(f: &Fmt, d: Datum) -> Datum {
    encode(f, decode(f, d))
}

fn main() {}
