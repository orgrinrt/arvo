// If macro_rules! cannot see digits, could the VALUE of the literal (known
// via a const fn over the stringified text, or directly as a const-generic
// parameter) drive a recursive TYPE-LEVEL peel instead? This tests the
// const-generic route directly against the numeral-magnitude use case
// (freshly compiled here; the consolidation only compiled this wall for the
// exponent, 58:143-148, and this file does not take that as read for a
// different quantity without re-checking it).
#![cfg_attr(feature = "gce", feature(generic_const_exprs))]
#![cfg_attr(feature = "mgca", feature(min_generic_const_args))]

pub trait Nat {
    const VAL: u64;
}
pub struct Z;
impl Nat for Z {
    const VAL: u64 = 0;
}
pub struct S<N: Nat>(core::marker::PhantomData<N>);
impl<N: Nat> Nat for S<N> {
    const VAL: u64 = N::VAL + 1;
}

// Attempt: a trait keyed on a const u64, recursing by computing V/2 and V%2
// as new const arguments to place in another type's const-generic position.
pub trait FromU64<const V: u64> {
    type Out: Nat;
}

#[cfg(feature = "gce")]
impl FromU64<0> for () {
    type Out = Z;
}
#[cfg(feature = "gce")]
impl<const V: u64> FromU64<V> for ()
where
    (): FromU64<{ V / 2 }>,
{
    type Out = S<<() as FromU64<{ V / 2 }>>::Out>;
}

#[cfg(feature = "mgca")]
impl FromU64<0> for () {
    type Out = Z;
}
#[cfg(feature = "mgca")]
impl<const V: u64> FromU64<V> for ()
where
    (): FromU64<{ const { V / 2 } }>,
{
    type Out = S<<() as FromU64<{ const { V / 2 } }>>::Out>;
}

// Neither cfg: the plain attempt with no gate, to see the bare-language error.
#[cfg(not(any(feature = "gce", feature = "mgca")))]
impl FromU64<0> for () {
    type Out = Z;
}
#[cfg(not(any(feature = "gce", feature = "mgca")))]
impl<const V: u64> FromU64<V> for ()
where
    (): FromU64<{ V / 2 }>,
{
    type Out = S<<() as FromU64<{ V / 2 }>>::Out>;
}

fn main() {
    let _v: u64 = <() as FromU64<37>>::Out::VAL;
}
