// The consumer writes the two numbers, in plain type syntax, no macro, no gate.
// Nothing anywhere computes a const argument. Precision is a projection.
use core::marker::PhantomData;

// ---- strategy axis ---------------------------------------------------------
pub trait Policy {}
pub trait Lowering {}
pub struct Warm;
impl Policy for Warm {}
impl Lowering for Warm {}

// ---- the container ladder: carried and read, five rungs the hardware has ----
pub trait Container: Copy {
    const BITS: u32;
}
impl Container for u8 {
    const BITS: u32 = 8;
}
impl Container for u16 {
    const BITS: u32 = 16;
}
impl Container for u32 {
    const BITS: u32 = 32;
}
impl Container for u64 {
    const BITS: u32 = 64;
}
impl Container for u128 {
    const BITS: u32 = 128;
}

// ---- the numeral. The two numbers a consumer writes ARE the parameters. -----
pub struct UFixed<const I: u32, const F: u32, C: Container, S: Policy + Lowering> {
    raw: C,
    _s: PhantomData<S>,
}
impl<const I: u32, const F: u32, C: Container, S: Policy + Lowering> Clone for UFixed<I, F, C, S> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<const I: u32, const F: u32, C: Container, S: Policy + Lowering> Copy for UFixed<I, F, C, S> {}

// ---- the mathematical coordinates, as projections in VALUE position ---------
pub trait Format {
    const PRECISION: u32;
    const EXPONENT: i32;
    const INTEGER_DIGITS: u32;
    type Store: Container;
}
impl<const I: u32, const F: u32, C: Container, S: Policy + Lowering> Format for UFixed<I, F, C, S> {
    const PRECISION: u32 = I + F; // legal: an associated const body is value position
    const EXPONENT: i32 = -(F as i32); // likewise
    const INTEGER_DIGITS: u32 = I;
    type Store = C;
}

impl<const I: u32, const F: u32, C: Container, S: Policy + Lowering> UFixed<I, F, C, S> {
    const FITS: () = assert!(
        I + F <= C::BITS,
        "arvo: the format's precision exceeds its container. \
         UFixed<I, F, C, S> stores I + F significant bits in C; \
         pick a wider C, or fewer bits."
    );
    pub const fn new(raw: C) -> Self {
        let () = Self::FITS;
        UFixed {
            raw,
            _s: PhantomData,
        }
    }
    pub const fn raw(self) -> C {
        self.raw
    }
}

// ---- the laws. Coordinate equalities are BOUNDS; sums are checks. ----------

/// Alignment is a parameter equality, so it is checked where the call is written.
pub fn add<
    const I: u32,
    const J: u32,
    const F: u32,
    const M: u32,
    C: Container,
    D: Container,
    S: Policy + Lowering,
>(
    a: UFixed<I, F, C, S>,
    b: UFixed<J, F, C, S>,
) -> UFixed<M, F, D, S> {
    const {
        assert!(
            M == if I > J { I } else { J } + 1,
            "arvo: add widens the integer part by one above the wider input."
        )
    }
    let _ = (a, b);
    UFixed {
        raw: unsafe { core::mem::zeroed() },
        _s: PhantomData,
    }
}

/// The product's coordinates are sums, so the relation is a check.
pub fn mul<
    const I: u32,
    const F: u32,
    const J: u32,
    const K: u32,
    const M: u32,
    const N: u32,
    C: Container,
    D: Container,
    S: Policy + Lowering,
>(
    a: UFixed<I, F, C, S>,
    b: UFixed<J, K, C, S>,
) -> UFixed<M, N, D, S> {
    const {
        assert!(
            M == I + J,
            "arvo: mul adds the integer digit counts of its inputs."
        )
    }
    const {
        assert!(
            N == F + K,
            "arvo: mul adds the fraction digit counts of its inputs."
        )
    }
    let _ = (a, b);
    UFixed {
        raw: unsafe { core::mem::zeroed() },
        _s: PhantomData,
    }
}

pub fn widen_int<
    const I: u32,
    const F: u32,
    const A: u32,
    const M: u32,
    C: Container,
    D: Container,
    S: Policy + Lowering,
>(
    x: UFixed<I, F, C, S>,
) -> UFixed<M, F, D, S> {
    const { assert!(M == I + A, "arvo: widen_int adds A integer digits.") }
    let _ = x;
    UFixed {
        raw: unsafe { core::mem::zeroed() },
        _s: PhantomData,
    }
}
