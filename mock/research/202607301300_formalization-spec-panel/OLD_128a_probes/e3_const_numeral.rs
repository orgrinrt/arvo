//! E3: the numeral keyed on the PAIR of const params, `Dim<N>`-style.
//! Precision as an associated CONST rather than an associated type.
//! Question: is canonicity delivered, and where exactly does it break?
#![no_std]
#![feature(const_trait_impl)]

pub struct Hot;
pub struct Warm;
pub trait Policy {}
impl Policy for Hot {}
impl Policy for Warm {}

/// Encoding coordinates, one blanket impl, no cap, no enumeration.
pub struct Fix<const I: u16, const F: u16>;

pub trait Numeral {
    /// Mathematical coordinate, DERIVED in value position where
    /// arithmetic is unrestricted.
    const PRECISION: u16;
    const EXPONENT: i32;
}

impl<const I: u16, const F: u16> Numeral for Fix<I, F> {
    const PRECISION: u16 = I + F; // value position: legal, no gate
    const EXPONENT: i32 = -(F as i32);
}

pub struct Number<N: Numeral, S: Policy>(core::marker::PhantomData<(N, S)>);

pub type UFixed<const I: u16, const F: u16, S> = Number<Fix<I, F>, S>;

// --- canonicity of the numeral: free, by const-argument equality ---
const _: () = assert!(<Fix<13, 3> as Numeral>::PRECISION == 16);
const _: () = assert!(<Fix<8, 8> as Numeral>::PRECISION == 16);
const _: () = assert!(<Fix<13, 3> as Numeral>::PRECISION == <Fix<8, 8> as Numeral>::PRECISION);

// --- arbitrary widths, no cap, no range chosen by the design ---
const _: () = assert!(<Fix<40000, 25535> as Numeral>::PRECISION == 65535);
const _: () = assert!(<Fix<0, 1> as Numeral>::PRECISION == 1);

// --- the agreement point op names, at VALUE level: no E0308 ---
pub const fn widths_agree<A: Numeral, B: Numeral>() -> bool {
    A::PRECISION == B::PRECISION
}
const _: () = assert!(widths_agree::<Fix<13, 3>, Fix<8, 8>>());
