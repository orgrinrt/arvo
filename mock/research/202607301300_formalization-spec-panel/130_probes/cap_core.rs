// The whole surface. No feature gates, no -Z flag, no macro at the surface.
use core::marker::PhantomData;

pub trait Policy {}
pub trait Lowering {}
pub struct Hot;
pub struct Warm;
pub struct Cold;
pub struct Precise;
impl Policy for Warm {}
impl Lowering for Warm {}
impl Policy for Hot {}
impl Lowering for Hot {}

pub trait Container: Copy {
    const BITS: u32;
    const ZERO: Self;
}
impl Container for u8 {
    const BITS: u32 = 8;
    const ZERO: u8 = 0;
}
impl Container for u16 {
    const BITS: u32 = 16;
    const ZERO: u16 = 0;
}
impl Container for u32 {
    const BITS: u32 = 32;
    const ZERO: u32 = 0;
}
impl Container for u64 {
    const BITS: u32 = 64;
    const ZERO: u64 = 0;
}
impl Container for u128 {
    const BITS: u32 = 128;
    const ZERO: u128 = 0;
}
impl Container for i8 {
    const BITS: u32 = 8;
    const ZERO: i8 = 0;
}
impl Container for i16 {
    const BITS: u32 = 16;
    const ZERO: i16 = 0;
}
impl Container for i32 {
    const BITS: u32 = 32;
    const ZERO: i32 = 0;
}
impl Container for i64 {
    const BITS: u32 = 64;
    const ZERO: i64 = 0;
}
impl Container for i128 {
    const BITS: u32 = 128;
    const ZERO: i128 = 0;
}

pub struct Unsigned;
pub struct Signed;
pub trait Sign {}
impl Sign for Unsigned {}
impl Sign for Signed {}

// ---------------------------------------------------------------------------
// The numeral. Two coordinates the consumer writes, a carried container, a
// strategy, a sign. Nothing computes a const argument anywhere.
// ---------------------------------------------------------------------------
pub struct Fixed<const I: u32, const F: u32, C: Container, G: Sign, S: Policy + Lowering> {
    raw: C,
    _m: PhantomData<(G, S)>,
}
impl<const I: u32, const F: u32, C: Container, G: Sign, S: Policy + Lowering> Clone
    for Fixed<I, F, C, G, S>
{
    fn clone(&self) -> Self {
        *self
    }
}
impl<const I: u32, const F: u32, C: Container, G: Sign, S: Policy + Lowering> Copy
    for Fixed<I, F, C, G, S>
{
}

pub type UFixed<const I: u32, const F: u32, C, S> = Fixed<I, F, C, Unsigned, S>;
pub type IFixed<const I: u32, const F: u32, C, S> = Fixed<I, F, C, Signed, S>;

pub trait SignBits {
    const EXTRA: u32;
}
impl SignBits for Unsigned {
    const EXTRA: u32 = 0;
}
impl SignBits for Signed {
    const EXTRA: u32 = 1;
}

// ---------------------------------------------------------------------------
// The mathematical coordinates, as projections. Value position, always legal.
// ---------------------------------------------------------------------------
pub trait Format {
    const PRECISION: u32;
    const EXPONENT: i32;
    const INTEGER_DIGITS: u32;
    const FRACTION_DIGITS: u32;
    type Store: Container;
}
impl<const I: u32, const F: u32, C: Container, G: Sign + SignBits, S: Policy + Lowering> Format
    for Fixed<I, F, C, G, S>
{
    const PRECISION: u32 = G::EXTRA + I + F;
    const EXPONENT: i32 = -(F as i32);
    const INTEGER_DIGITS: u32 = I;
    const FRACTION_DIGITS: u32 = F;
    type Store = C;
}

impl<const I: u32, const F: u32, C: Container, G: Sign + SignBits, S: Policy + Lowering>
    Fixed<I, F, C, G, S>
{
    pub const FITS: () = assert!(
        G::EXTRA + I + F <= C::BITS,
        "arvo: the format does not fit its container.
  Fixed<I, F, C, ..> stores I + F significant digits, plus one sign digit when
  the sign axis is Signed, inside C. The instantiation printed above names I, F
  and C. Pick a wider C from the ladder, or write fewer digits."
    );
    pub const fn from_raw(raw: C) -> Self {
        let () = Self::FITS;
        Fixed {
            raw,
            _m: PhantomData,
        }
    }
    pub const fn to_raw(self) -> C {
        self.raw
    }
    pub const fn zero() -> Self {
        Self::from_raw(C::ZERO)
    }
}

// ---------------------------------------------------------------------------
// The laws. Every law is a named item, so rustc prints the law's name and its
// coordinates in the law's own order when it fails.
// ---------------------------------------------------------------------------
pub struct ProductFormat<
    const I: u32,
    const F: u32,
    const J: u32,
    const K: u32,
    const M: u32,
    const N: u32,
>;
impl<const I: u32, const F: u32, const J: u32, const K: u32, const M: u32, const N: u32>
    ProductFormat<I, F, J, K, M, N>
{
    pub const HOLDS: () = assert!(
        M == I + J && N == F + K,
        "arvo: the product's format does not follow from its inputs.
  The law: Fixed<I, F> times Fixed<J, K> has format Fixed<I + J, F + K>.
  The line above prints ProductFormat::<I, F, J, K, M, N> with the actual
  digit counts, in that order.
  If you wrote the call, name the output with those first four numbers added
  pairwise. If the call is inside a function you did not write, that function
  states a format relation that does not hold, and the note below names the
  function and the line. Search your own source for the two output numbers
  printed above to find which of your calls reached it."
    );
}

pub struct SumFormat<const I: u32, const J: u32, const M: u32>;
impl<const I: u32, const J: u32, const M: u32> SumFormat<I, J, M> {
    pub const HOLDS: () = assert!(
        M == (if I > J { I } else { J }) + 1,
        "arvo: the sum's integer digit count does not follow from its inputs.
  The law: adding two numerals of the same exponent widens the integer part
  to one digit above the wider input. The line above prints
  SumFormat::<I, J, M> with the actual digit counts.
  The exponents are already equal, because the signature requires it; only the
  integer width is a computed relation and only it can be got wrong here."
    );
}

pub struct WidenFormat<const I: u32, const A: u32, const M: u32>;
impl<const I: u32, const A: u32, const M: u32> WidenFormat<I, A, M> {
    pub const HOLDS: () = assert!(
        M == I + A,
        "arvo: the widened format does not follow from its input.
  The law: widening by A integer digits takes Fixed<I, F> to Fixed<I + A, F>.
  The line above prints WidenFormat::<I, A, M> with the actual digit counts."
    );
}

/// Alignment is an equality between coordinates, so it is a BOUND: the two
/// arguments share the parameter `F`. A misaligned call is refused where it is
/// written, before any monomorphisation.
pub fn add<
    const I: u32,
    const J: u32,
    const F: u32,
    const M: u32,
    C: Container,
    D: Container,
    G: Sign + SignBits,
    S: Policy + Lowering,
>(
    a: Fixed<I, F, C, G, S>,
    b: Fixed<J, F, C, G, S>,
) -> Fixed<M, F, D, G, S> {
    let () = SumFormat::<I, J, M>::HOLDS;
    let () = Fixed::<M, F, D, G, S>::FITS;
    let _ = (a, b);
    Fixed {
        raw: D::ZERO,
        _m: PhantomData,
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
    G: Sign + SignBits,
    S: Policy + Lowering,
>(
    a: Fixed<I, F, C, G, S>,
    b: Fixed<J, K, C, G, S>,
) -> Fixed<M, N, D, G, S> {
    let () = ProductFormat::<I, F, J, K, M, N>::HOLDS;
    let () = Fixed::<M, N, D, G, S>::FITS;
    let _ = (a, b);
    Fixed {
        raw: D::ZERO,
        _m: PhantomData,
    }
}

pub fn widen_int<
    const I: u32,
    const F: u32,
    const A: u32,
    const M: u32,
    C: Container,
    D: Container,
    G: Sign + SignBits,
    S: Policy + Lowering,
>(
    x: Fixed<I, F, C, G, S>,
) -> Fixed<M, F, D, G, S> {
    let () = WidenFormat::<I, A, M>::HOLDS;
    let () = Fixed::<M, F, D, G, S>::FITS;
    let _ = x;
    Fixed {
        raw: D::ZERO,
        _m: PhantomData,
    }
}

/// Changing the exponent is an operation with a name, not an assignment.
/// This is the thing a precision-keyed numeral cannot make the consumer write.
pub fn rescale<
    const I: u32,
    const F: u32,
    const J: u32,
    const K: u32,
    C: Container,
    G: Sign + SignBits,
    S: Policy + Lowering,
>(
    x: Fixed<I, F, C, G, S>,
) -> Fixed<J, K, C, G, S> {
    let () = Fixed::<J, K, C, G, S>::FITS;
    let _ = x;
    Fixed {
        raw: C::ZERO,
        _m: PhantomData,
    }
}

// ---------------------------------------------------------------------------
// The bit view, reached with an output parameter rather than a computed type.
// ---------------------------------------------------------------------------
pub struct Bits<const W: u32, C: Container, S: Policy + Lowering> {
    raw: C,
    _s: PhantomData<S>,
}
pub fn bits_of<
    const I: u32,
    const F: u32,
    const W: u32,
    C: Container,
    G: Sign + SignBits,
    S: Policy + Lowering,
>(
    x: Fixed<I, F, C, G, S>,
) -> Bits<W, C, S> {
    let () = BitWidth::<I, F, W>::HOLDS_UNSIGNED;
    Bits {
        raw: x.to_raw(),
        _s: PhantomData,
    }
}
pub struct BitWidth<const I: u32, const F: u32, const W: u32>;
impl<const I: u32, const F: u32, const W: u32> BitWidth<I, F, W> {
    pub const HOLDS_UNSIGNED: () = assert!(
        W == I + F,
        "arvo: the bit view's width must equal the format's precision."
    );
}
