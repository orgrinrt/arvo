//! Sketch: const-trait arithmetic on every arvo numeric primitive.
//!
//! Validates the load-bearing claim that round 202605021200 makes:
//! every arvo numeric primitive can carry `impl const` arithmetic
//! ops on stable nightly with `feature(const_trait_impl)`, and the
//! resulting surface lets consumers compute on arvo primitives at
//! const time without falling back to `.0` field-access.
//!
//! Stress instances:
//!
//! 1. The full op matrix on USize / Cap (Add/Sub/Mul/Div/Rem/
//!    Shl/Shr/BitAnd/BitOr/BitXor/Not + Ord/PartialOrd) compiles
//!    in a const context.
//! 2. UFixed-shaped fixed-point types convert from non-const to
//!    const-trait ops without const-eval errors.
//! 3. The hilavitkutin Depth case `<R as Depth>::D + USize::ONE`
//!    type-checks at const time and yields the right value.
//! 4. Mixed-arity ops (USize + bare usize) DO NOT compile.
//! 5. Bit-ops on a Mask-shaped newtype convert to const.

#![no_std]
#![feature(const_trait_impl)]
#![feature(const_ops)]
#![allow(incomplete_features)]
#![allow(dead_code)]

use core::ops::{Add, BitAnd, BitOr, BitXor, Div, Mul, Not, Rem, Shl, Shr, Sub};

// ---------------------------------------------------------------------
// USize: the primary surface to validate.
// ---------------------------------------------------------------------

#[derive(Copy, Clone, PartialEq, Eq)]
#[repr(transparent)]
pub struct USize(pub usize);

impl USize {
    pub const ZERO: USize = USize(0);
    pub const ONE: USize = USize(1);
    pub const MAX: USize = USize(usize::MAX);
}

impl const Add<USize> for USize {
    type Output = USize;
    fn add(self, rhs: USize) -> USize {
        USize(self.0 + rhs.0)
    }
}

impl const Sub<USize> for USize {
    type Output = USize;
    fn sub(self, rhs: USize) -> USize {
        USize(self.0 - rhs.0)
    }
}

impl const Mul<USize> for USize {
    type Output = USize;
    fn mul(self, rhs: USize) -> USize {
        USize(self.0 * rhs.0)
    }
}

impl const Div<USize> for USize {
    type Output = USize;
    fn div(self, rhs: USize) -> USize {
        USize(self.0 / rhs.0)
    }
}

impl const Rem<USize> for USize {
    type Output = USize;
    fn rem(self, rhs: USize) -> USize {
        USize(self.0 % rhs.0)
    }
}

impl const Shl<USize> for USize {
    type Output = USize;
    fn shl(self, rhs: USize) -> USize {
        USize(self.0 << rhs.0)
    }
}

impl const Shr<USize> for USize {
    type Output = USize;
    fn shr(self, rhs: USize) -> USize {
        USize(self.0 >> rhs.0)
    }
}

impl const BitAnd<USize> for USize {
    type Output = USize;
    fn bitand(self, rhs: USize) -> USize {
        USize(self.0 & rhs.0)
    }
}

impl const BitOr<USize> for USize {
    type Output = USize;
    fn bitor(self, rhs: USize) -> USize {
        USize(self.0 | rhs.0)
    }
}

impl const BitXor<USize> for USize {
    type Output = USize;
    fn bitxor(self, rhs: USize) -> USize {
        USize(self.0 ^ rhs.0)
    }
}

impl const Not for USize {
    type Output = USize;
    fn not(self) -> USize {
        USize(!self.0)
    }
}

// ---------------------------------------------------------------------
// Cap: mirrors USize, body delegates through.
// ---------------------------------------------------------------------

#[derive(Copy, Clone, PartialEq, Eq)]
#[repr(transparent)]
pub struct Cap(pub USize);

impl Cap {
    pub const ZERO: Cap = Cap(USize::ZERO);
    pub const ONE: Cap = Cap(USize::ONE);
}

// Cap delegates through USize's const Add: no raw `.0.0` field access,
// the layered composition stays clean. This is the recommended pattern
// for any wrapper-of-arvo-primitive that needs arithmetic.
impl const Add<Cap> for Cap {
    type Output = Cap;
    fn add(self, rhs: Cap) -> Cap {
        Cap(self.0.add(rhs.0))
    }
}

impl const Sub<Cap> for Cap {
    type Output = Cap;
    fn sub(self, rhs: Cap) -> Cap {
        Cap(self.0.sub(rhs.0))
    }
}

// ---------------------------------------------------------------------
// UFixed-shaped: validate that the existing ops convert to impl const
// without const-eval errors. The real arvo UFixed has more complex
// internals (Strategy dispatch, container projection); the sketch
// here uses a minimal stand-in that exercises the same const-trait
// pattern.
// ---------------------------------------------------------------------

#[derive(Copy, Clone, PartialEq, Eq)]
#[repr(transparent)]
pub struct UFixedToy<const W: u8>(pub u64);

impl<const W: u8> UFixedToy<W> {
    pub const ZERO: UFixedToy<W> = UFixedToy(0);
    pub const ONE: UFixedToy<W> = UFixedToy(1);
}

impl<const W: u8> const Add<UFixedToy<W>> for UFixedToy<W> {
    type Output = UFixedToy<W>;
    fn add(self, rhs: UFixedToy<W>) -> UFixedToy<W> {
        UFixedToy(self.0 + rhs.0)
    }
}

impl<const W: u8> const Sub<UFixedToy<W>> for UFixedToy<W> {
    type Output = UFixedToy<W>;
    fn sub(self, rhs: UFixedToy<W>) -> UFixedToy<W> {
        UFixedToy(self.0 - rhs.0)
    }
}

impl<const W: u8> const Mul<UFixedToy<W>> for UFixedToy<W> {
    type Output = UFixedToy<W>;
    fn mul(self, rhs: UFixedToy<W>) -> UFixedToy<W> {
        UFixedToy(self.0 * rhs.0)
    }
}

// ---------------------------------------------------------------------
// Mask-shaped: bit-ops only, no arithmetic. Validates const-trait on
// the bit-op family.
// ---------------------------------------------------------------------

#[derive(Copy, Clone, PartialEq, Eq)]
#[repr(transparent)]
pub struct Mask64Toy(pub u64);

impl Mask64Toy {
    pub const EMPTY: Mask64Toy = Mask64Toy(0);
    pub const FULL: Mask64Toy = Mask64Toy(!0u64);
}

impl const BitAnd<Mask64Toy> for Mask64Toy {
    type Output = Mask64Toy;
    fn bitand(self, rhs: Mask64Toy) -> Mask64Toy {
        Mask64Toy(self.0 & rhs.0)
    }
}

impl const BitOr<Mask64Toy> for Mask64Toy {
    type Output = Mask64Toy;
    fn bitor(self, rhs: Mask64Toy) -> Mask64Toy {
        Mask64Toy(self.0 | rhs.0)
    }
}

impl const Not for Mask64Toy {
    type Output = Mask64Toy;
    fn not(self) -> Mask64Toy {
        Mask64Toy(!self.0)
    }
}

// ---------------------------------------------------------------------
// Stress 1: full-matrix const evaluation on USize. If any of these
// don't const-fold, the build fails.
// ---------------------------------------------------------------------

const _USIZE_ADD: USize = USize(5).add(USize(3));
const _USIZE_SUB: USize = USize(10).sub(USize(4));
const _USIZE_MUL: USize = USize(6).mul(USize(7));
const _USIZE_DIV: USize = USize(20).div(USize(4));
const _USIZE_REM: USize = USize(13).rem(USize(5));
const _USIZE_SHL: USize = USize(1).shl(USize(4));
const _USIZE_SHR: USize = USize(64).shr(USize(2));
const _USIZE_AND: USize = USize(0xFF).bitand(USize(0x0F));
const _USIZE_OR: USize = USize(0xF0).bitor(USize(0x0F));
const _USIZE_XOR: USize = USize(0xFF).bitxor(USize(0xAA));
const _USIZE_NOT: USize = (USize(0)).not();

const _USIZE_ZERO_PLUS_ONE: USize = USize::ZERO.add(USize::ONE);

// Validate the full-matrix sums fold to expected values.
const _ASSERT_ADD: () = assert!(_USIZE_ADD.0 == 8);
const _ASSERT_SUB: () = assert!(_USIZE_SUB.0 == 6);
const _ASSERT_MUL: () = assert!(_USIZE_MUL.0 == 42);
const _ASSERT_DIV: () = assert!(_USIZE_DIV.0 == 5);
const _ASSERT_REM: () = assert!(_USIZE_REM.0 == 3);
const _ASSERT_SHL: () = assert!(_USIZE_SHL.0 == 16);
const _ASSERT_SHR: () = assert!(_USIZE_SHR.0 == 16);
const _ASSERT_AND: () = assert!(_USIZE_AND.0 == 0x0F);
const _ASSERT_OR: () = assert!(_USIZE_OR.0 == 0xFF);
const _ASSERT_XOR: () = assert!(_USIZE_XOR.0 == 0x55);
const _ASSERT_ZERO_PLUS_ONE: () = assert!(_USIZE_ZERO_PLUS_ONE.0 == 1);

// ---------------------------------------------------------------------
// Stress 2: Cap arithmetic.
// ---------------------------------------------------------------------

const _CAP_SUM: Cap = Cap(USize(10)).add(Cap(USize(5)));
const _ASSERT_CAP_SUM: () = assert!(_CAP_SUM.0.0 == 15);

// ---------------------------------------------------------------------
// Stress 3: the hilavitkutin Depth case at the type level. Recursive
// trait that counts cons-list depth using only USize const arithmetic.
// ---------------------------------------------------------------------

mod depth_sealed {
    pub trait Sealed {}
}

#[allow(private_bounds)]
pub trait Depth: depth_sealed::Sealed {
    const D: USize;
}

impl depth_sealed::Sealed for () {}
impl<H, R: Depth> depth_sealed::Sealed for (H, R) {}

impl Depth for () {
    const D: USize = USize::ZERO;
}

impl<H, R: Depth> Depth for (H, R) {
    const D: USize = R::D.add(USize::ONE);
}

type Cons5 = ((), ((), ((), ((), ((), ())))));
const _ASSERT_DEPTH_5: () = assert!(<Cons5 as Depth>::D.0 == 5);

// ---------------------------------------------------------------------
// Stress 4: UFixedToy const arithmetic with const-generic parameter.
// ---------------------------------------------------------------------

const _UFIX_ADD: UFixedToy<7> = UFixedToy::<7>(10).add(UFixedToy::<7>(5));
const _ASSERT_UFIX_ADD: () = assert!(_UFIX_ADD.0 == 15);

const _UFIX_ZERO_PLUS_ONE: UFixedToy<7> = UFixedToy::<7>::ZERO.add(UFixedToy::<7>::ONE);
const _ASSERT_UFIX_ZERO_PLUS_ONE: () = assert!(_UFIX_ZERO_PLUS_ONE.0 == 1);

// ---------------------------------------------------------------------
// Stress 5: Mask bit-ops at const time.
// ---------------------------------------------------------------------

const _MASK_AND: Mask64Toy = Mask64Toy(0xFF).bitand(Mask64Toy(0x0F));
const _MASK_OR: Mask64Toy = Mask64Toy(0xF0).bitor(Mask64Toy(0x0F));
const _MASK_NOT_EMPTY: Mask64Toy = Mask64Toy::EMPTY.not();
const _ASSERT_MASK_AND: () = assert!(_MASK_AND.0 == 0x0F);
const _ASSERT_MASK_OR: () = assert!(_MASK_OR.0 == 0xFF);
const _ASSERT_MASK_NOT: () = assert!(_MASK_NOT_EMPTY.0 == !0u64);

// ---------------------------------------------------------------------
// Stress 6: operator-syntax const evaluation. The above tests use
// trait-method syntax (`x.add(y)`); these test that operator syntax
// (`x + y`) also const-evaluates.
// ---------------------------------------------------------------------

// NOTE: in current nightly, operator syntax in const context requires
// the trait impl to be `const Add` AND the call to be in a const
// context with the const-trait usage explicitly enabled. The form
// `const X: USize = USize(1) + USize(2);` should work; if rustc
// disagrees, we ship trait-method syntax (`USize(1).add(USize(2))`)
// in the implementing crates as the load-bearing path.

const _OP_SYNTAX_ADD: USize = USize(1) + USize(2);
const _ASSERT_OP_SYNTAX_ADD: () = assert!(_OP_SYNTAX_ADD.0 == 3);

// ---------------------------------------------------------------------
// Negative validation: mixed-arity ops MUST NOT compile.
// Uncomment to verify manually; permanent inclusion would block the
// build.
// ---------------------------------------------------------------------

// const _MIXED_ARITY: USize = USize(1) + 1usize;  // expected: error
// const _MIXED_ARITY_2: usize = 1usize + USize(1).0;  // requires .0 (the wrong shape)

// ---------------------------------------------------------------------
// Findings to capture in FINDINGS.md after rustc validates the sketch:
//   - which ops compile cleanly with `impl const`
//   - which (if any) hit const-eval limitations and need a workaround
//   - whether operator syntax works in const context, or whether the
//     implementing-crate code must use trait-method syntax everywhere
//   - whether Cap can use Deref + USize ops directly, or whether it
//     needs its own impls (Sketch ships own impls; trade-off in DOC)
// ---------------------------------------------------------------------
