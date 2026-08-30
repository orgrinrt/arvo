// Probe 01: are the algebraic laws derivable by blanket impls over the composition?
//
// D51 says law markers are computed from the axes rather than declared per type.
// The hard part is that a law depends on MORE THAN ONE axis: saturating addition
// is associative for unsigned operands and not for signed ones, because two-sided
// clamping is what breaks it. So the derivation has to condition on the overflow
// policy AND the signedness jointly.
//
// If this resolves, `Monoid<Add>` exists exactly where the mathematics says it
// does, and a fold over a two-sided saturating type fails to compile.
//
// Run: rustc --edition 2021 01_derived_laws.rs -o /tmp/q01 && /tmp/q01

use core::marker::PhantomData;

// --- the axes --------------------------------------------------------------

pub struct Wrap;
pub struct Saturate;
pub struct SaturateSym;

pub struct Unsigned;
pub struct Signed;

pub struct Trunc;
pub struct FullPrecision;
pub struct Fixed<const F: i32>;

// --- the laws, as markers rather than as prose in a doc comment ------------

/// The operation is associative for this configuration.
pub trait Associative {}
/// The operation is commutative for this configuration.
pub trait Commutative {}
/// Every element has an inverse.
pub trait Invertible {}

// --- law membership, derived from the (overflow, signedness) pair ----------
//
// The pair is the unit of truth, because neither axis decides alone.

/// Addition is associative for this (overflow, signedness) pair.
pub trait AddAssoc {}
/// Addition is commutative for this pair.
pub trait AddComm {}
/// Addition has inverses for this pair.
pub trait AddInv {}

// Wrapping is Z/2^n Z: a full abelian group, both signednesses.
impl AddAssoc for (Wrap, Unsigned) {}
impl AddAssoc for (Wrap, Signed) {}
impl AddComm for (Wrap, Unsigned) {}
impl AddComm for (Wrap, Signed) {}
impl AddInv for (Wrap, Unsigned) {}
impl AddInv for (Wrap, Signed) {}

// Unsigned saturating is truncated addition, min(a + b, M): a commutative
// monoid. Associative because saturation occurs at a single boundary and
// cannot be walked back.
impl AddAssoc for (Saturate, Unsigned) {}
impl AddComm for (Saturate, Unsigned) {}
// No AddInv: nothing undoes a clamp.

// Signed saturating is two-sided. Commutative, monotone, NOT associative:
//   (127 + 1) + (-1) = 126   but   127 + (1 + (-1)) = 127
impl AddComm for (Saturate, Signed) {}
// No AddAssoc for (Saturate, Signed). That absence is the whole point.

// Symmetric saturation does not rescue it. The cause is two-sided clamping,
// not the asymmetric negative extreme.
impl AddComm for (SaturateSym, Unsigned) {}
impl AddAssoc for (SaturateSym, Unsigned) {}
impl AddComm for (SaturateSym, Signed) {}
// No AddAssoc for (SaturateSym, Signed) either.

// --- the composition -------------------------------------------------------

pub struct Num<Fmt, Sign, Round, Over, Grow>(PhantomData<(Fmt, Sign, Round, Over, Grow)>);

pub struct Add;

// --- the structures, as blanket impls over the composition -----------------

pub trait Semigroup<Op> {}
pub trait CommutativeSemigroup<Op>: Semigroup<Op> {}
pub trait AbelianGroup<Op>: CommutativeSemigroup<Op> {}

impl<Fmt, Sign, Round, Over, Grow> Semigroup<Add> for Num<Fmt, Sign, Round, Over, Grow> where
    (Over, Sign): AddAssoc
{
}

impl<Fmt, Sign, Round, Over, Grow> CommutativeSemigroup<Add> for Num<Fmt, Sign, Round, Over, Grow> where
    (Over, Sign): AddAssoc + AddComm
{
}

impl<Fmt, Sign, Round, Over, Grow> AbelianGroup<Add> for Num<Fmt, Sign, Round, Over, Grow> where
    (Over, Sign): AddAssoc + AddComm + AddInv
{
}

// --- consumers that state what they need -----------------------------------

/// A fold needs associativity, or the answer depends on the grouping.
fn fold_requires_assoc<T: Semigroup<Add>>() -> &'static str {
    "folded"
}

/// Subtraction-by-inverse needs the group structure.
fn requires_group<T: AbelianGroup<Add>>() -> &'static str {
    "inverted"
}

fn main() {
    type WrapU = Num<Fixed<8>, Unsigned, Trunc, Wrap, FullPrecision>;
    type WrapI = Num<Fixed<8>, Signed, Trunc, Wrap, FullPrecision>;
    type SatU = Num<Fixed<8>, Unsigned, Trunc, Saturate, FullPrecision>;
    type SatSymU = Num<Fixed<8>, Unsigned, Trunc, SaturateSym, FullPrecision>;

    // Wrapping folds and inverts, both signednesses.
    assert_eq!(fold_requires_assoc::<WrapU>(), "folded");
    assert_eq!(fold_requires_assoc::<WrapI>(), "folded");
    assert_eq!(requires_group::<WrapU>(), "inverted");
    assert_eq!(requires_group::<WrapI>(), "inverted");

    // Unsigned saturating folds (truncated addition is a commutative monoid)
    // but has no inverses, so `requires_group::<SatU>()` would not compile.
    assert_eq!(fold_requires_assoc::<SatU>(), "folded");
    assert_eq!(fold_requires_assoc::<SatSymU>(), "folded");

    // The refusal is exercised in 02_refusal.rs, which must fail to compile.

    println!("01 WORKS: laws derived over the composition, blanket impls resolve");
}
