//! p15. The copy-from-another-crate case, which is where the marker lives.
//!
//! `11_probes/c_orphan/consumer_partition.rs` established that under the const
//! KEYING two markers do not compose: `Fixed<13,0,LibA>` does not flow into
//! `Fixed<13,0,Arvo>`, because the marker is a parameter of the numeral and
//! therefore part of its identity. That is what makes "bring your own widths"
//! partition the ecosystem: two crates that both declared width 13 have two
//! incompatible 13-bit numerals.
//!
//! Under the DOOR shape the marker is a parameter of the door, not of the
//! numeral. It selects which nat a literal maps to and then it is gone. So two
//! crates that declare 13 against different markers should produce the SAME
//! numeral type.
//!
//! If that holds, the partition problem is a property of the keying rather than
//! of the bring-your-own-widths idea, and it dissolves.
//!
//! No `#![feature]`, no `-Z` flag.
//!
//! rustc +nightly-2026-05-28 --edition 2024 -O --crate-type=lib \
//!       --emit=metadata -o out/p15.meta p15_markers_do_not_partition.rs
#![no_std]
#![crate_type = "lib"]

use core::marker::PhantomData;

include!("ladder.rs");

pub struct Warm;
pub type Sum<A, B> = <A as Add<B>>::O;
pub type Cont<W> = <W as Container>::C;

#[repr(transparent)]
pub struct Fixed<WI, WF, S>
where
    WI: Add<WF>,
    Sum<WI, WF>: Container,
{
    raw: Cont<Sum<WI, WF>>,
    _m: PhantomData<(WI, WF, S)>,
}

pub struct Idx<const N: u32>;
pub trait ToNat<M> {
    type N;
}
pub type NatOf<const N: u32, M> = <Idx<N> as ToNat<M>>::N;

// --- three separate parties, each declaring its own widths ------------------
pub struct Arvo;
impl ToNat<Arvo> for Idx<13> {
    type N = T13;
}
impl ToNat<Arvo> for Idx<0> {
    type N = T0;
}

pub struct LibA;
impl ToNat<LibA> for Idx<13> {
    type N = T13;
}
impl ToNat<LibA> for Idx<0> {
    type N = T0;
}

pub struct LibB;
impl ToNat<LibB> for Idx<13> {
    type N = T13;
}
impl ToNat<LibB> for Idx<0> {
    type N = T0;
}
// LibB also declares one nobody else has, which is the point of the marker.
pub type T4711 = D1<D1<D1<D0<D0<D1<D1<D0<D0<D1<D0<D0<D1<Term>>>>>>>>>>>>>;
impl ToNat<LibB> for Idx<4711> {
    type N = T4711;
}

// --- each party's own one-line alias, written once per crate ----------------
pub type ArvoUInt<const N: u32> = Fixed<NatOf<N, Arvo>, T0, Warm>;
pub type AUInt<const N: u32> = Fixed<NatOf<N, LibA>, T0, Warm>;
pub type BUInt<const N: u32> = Fixed<NatOf<N, LibB>, T0, Warm>;

// --- the alias-definition sites, which is what the bar governs --------------
pub type ArvoHandle = ArvoUInt<13>;
pub type AHandle = AUInt<13>;
pub type BHandle = BUInt<13>;

// --- THE QUESTION. Three crates, three markers, one type? -------------------
pub fn a_flows_to_arvo(x: AHandle) -> ArvoHandle {
    x
}
pub fn b_flows_to_a(x: BHandle) -> AHandle {
    x
}
pub fn arvo_flows_to_b(x: ArvoHandle) -> BHandle {
    x
}

// --- and a width only one party declared is still an ordinary numeral -------
pub type BOdd = BUInt<4711>;
const _: () = assert!(core::mem::size_of::<BOdd>() == 592); // 4711 bits, 74 words
pub fn odd_is_a_numeral(x: BOdd) -> Fixed<T4711, T0, Warm> {
    x
}
