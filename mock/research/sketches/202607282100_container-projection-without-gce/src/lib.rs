//! Sketch: express the Pattern C container projection without `generic_const_exprs`.
//!
//! Hypothesis: the projection's GCE dependency comes from computing the tag and
//! byte count with const FUNCTIONS in const-generic argument position
//! (`Project<{ tag_hot_cold(N) }, Sign, { bytes_for_u16(N) }, Hot>`). Carrying
//! the selection as TYPESTATE instead, the way `Capacity` carries `Array`,
//! removes the expression from type position entirely, so no const-generic
//! feature is needed at all.
//!
//! Shape B below is the candidate. Shape A (associated-const path, retaining
//! `const N: u16`) is in `shape_a.rs` for comparison.
//!
//! NO feature gates in this file. That absence is the claim under test.

#![no_std]

// ---------------------------------------------------------------------------
// The axes, unchanged from the real crate.
// ---------------------------------------------------------------------------

pub trait Signedness: Copy + 'static {}
#[derive(Copy, Clone)]
pub struct Unsigned;
#[derive(Copy, Clone)]
pub struct Signed;
impl Signedness for Unsigned {}
impl Signedness for Signed {}

pub trait Strategy: Copy + 'static {}
#[derive(Copy, Clone)]
pub struct Hot;
#[derive(Copy, Clone)]
pub struct Cold;
#[derive(Copy, Clone)]
pub struct Warm;
#[derive(Copy, Clone)]
pub struct Precise;
impl Strategy for Hot {}
impl Strategy for Cold {}
impl Strategy for Warm {}
impl Strategy for Precise {}

// ---------------------------------------------------------------------------
// The bucket, previously a `const TAG: usize`, now a type.
//
// This is the whole move. A tag is a closed vocabulary of slots, so it is an
// enum of types rather than an integer computed by a function.
// ---------------------------------------------------------------------------

pub trait Bucket: Copy + 'static {}
#[derive(Copy, Clone)]
pub struct B8;
#[derive(Copy, Clone)]
pub struct B16;
#[derive(Copy, Clone)]
pub struct B32;
#[derive(Copy, Clone)]
pub struct B64;
#[derive(Copy, Clone)]
pub struct B128;
/// Wide bucket, carrying its byte count as the type's own const parameter.
/// `[u8; BYTES]` with BYTES the type's own parameter is stable const generics,
/// which is exactly why `Capacity`'s `[T; N]` never needed GCE either.
#[derive(Copy, Clone)]
pub struct BWide<const BYTES: usize>;

impl Bucket for B8 {}
impl Bucket for B16 {}
impl Bucket for B32 {}
impl Bucket for B64 {}
impl Bucket for B128 {}
impl<const BYTES: usize> Bucket for BWide<BYTES> {}

// ---------------------------------------------------------------------------
// Width as typestate. `Wid<N>` is the analogue of `Dim<N>`.
//
// Each strategy family names its own bucket for a given width, as an
// ASSOCIATED TYPE. No function call, no const expression, nothing in type
// position but a path.
// ---------------------------------------------------------------------------

#[derive(Copy, Clone)]
pub struct Wid<const N: u16>;

/// The width vocabulary, keyed by the strategy family that reads it.
///
/// Two families exist because the real crate has two tag functions:
/// `tag_hot_cold` and `tag_warm_precise`. The family is a type parameter here
/// rather than a choice of function.
pub trait WidthFor<F: Family>: Copy + 'static {
    type Bkt: Bucket;
}

pub trait Family: Copy + 'static {}
#[derive(Copy, Clone)]
pub struct HotCold;
#[derive(Copy, Clone)]
pub struct WarmPrecise;
impl Family for HotCold {}
impl Family for WarmPrecise {}

/// Generate the per-width mapping. The real crate would expand this over its
/// full supported range; the sketch covers the boundaries plus representative
/// interior widths, which is what the claim needs.
///
/// Per-N impl tables are explicitly licensed by `arvo-compile-time-last.md`:
/// "Spend trait-solver work on per-N const-trait impls (4 strategies x 64+
/// widths x 2 sign = hundreds of impls) when the alternative is a runtime
/// container check."
macro_rules! widths {
    ($fam:ty, $( $n:literal => $bkt:ty ),* $(,)?) => {
        $( impl WidthFor<$fam> for Wid<$n> { type Bkt = $bkt; } )*
    };
}

widths!(HotCold,
    1 => B8, 3 => B8, 7 => B8, 8 => B8,
    9 => B16, 13 => B16, 16 => B16,
    17 => B32, 23 => B32, 32 => B32,
    33 => B64, 47 => B64, 64 => B64,
    65 => B128, 128 => B128,
);

widths!(WarmPrecise,
    1 => B16, 3 => B16, 7 => B16, 8 => B16,
    9 => B32, 13 => B32, 16 => B32,
    17 => B64, 23 => B64, 32 => B64,
    33 => B128, 47 => B128, 64 => B128,
);

// ---------------------------------------------------------------------------
// The projection, now over TYPES rather than over computed consts.
// ---------------------------------------------------------------------------

pub trait Project<Bkt: Bucket, Sign: Signedness, S: Strategy> {
    type T: Copy + 'static;
}

#[derive(Copy, Clone)]
pub struct Picker;

macro_rules! project {
    ($( $bkt:ty, $sign:ty, $strat:ty => $t:ty );* $(;)?) => {
        $( impl Project<$bkt, $sign, $strat> for Picker { type T = $t; } )*
    };
}

project!(
    B8,   Unsigned, Hot  => u8;   B16,  Unsigned, Hot  => u16;
    B32,  Unsigned, Hot  => u32;  B64,  Unsigned, Hot  => u64;
    B128, Unsigned, Hot  => u128;
    B8,   Signed,   Hot  => i8;   B16,  Signed,   Hot  => i16;
    B32,  Signed,   Hot  => i32;  B64,  Signed,   Hot  => i64;
    B128, Signed,   Hot  => i128;
    B16,  Unsigned, Warm => u16;  B32,  Unsigned, Warm => u32;
    B64,  Unsigned, Warm => u64;  B128, Unsigned, Warm => u128;
    B16,  Signed,   Warm => i16;  B32,  Signed,   Warm => i32;
    B64,  Signed,   Warm => i64;  B128, Signed,   Warm => i128;
);

// ---------------------------------------------------------------------------
// The user-facing trait. Compare against the real crate's four impls:
//
//   impl<const N: u16, Sign> BitsContainerFor<N, Sign> for Hot
//   where Picker: Project<{ tag_hot_cold(N) }, Sign, { bytes_for_u16(N) }, Hot>
//
// Here the const expression is gone. `W::Bkt` is a path, not a computation.
// ---------------------------------------------------------------------------

pub trait BitsContainerFor<W, Sign: Signedness> {
    type T: Copy + 'static;
}

impl<W, Sign> BitsContainerFor<W, Sign> for Hot
where
    Sign: Signedness,
    W: WidthFor<HotCold>,
    Picker: Project<W::Bkt, Sign, Hot>,
{
    type T = <Picker as Project<W::Bkt, Sign, Hot>>::T;
}

impl<W, Sign> BitsContainerFor<W, Sign> for Warm
where
    Sign: Signedness,
    W: WidthFor<WarmPrecise>,
    Picker: Project<W::Bkt, Sign, Warm>,
{
    type T = <Picker as Project<W::Bkt, Sign, Warm>>::T;
}

// ---------------------------------------------------------------------------
// Proof of resolution. If these type-check, the projection resolved with no
// const-generic feature enabled anywhere in this crate.
// ---------------------------------------------------------------------------

/// The consumer-facing storage type, `Bits<W, S>` where W is typestate.
#[derive(Copy, Clone)]
pub struct Bits<W, S: Strategy, Sign: Signedness = Unsigned>(
    pub <S as BitsContainerFor<W, Sign>>::T,
)
where
    S: BitsContainerFor<W, Sign>;

const _: () = {
    // Hot at 13 bits picks u16.
    let _: <Hot as BitsContainerFor<Wid<13>, Unsigned>>::T = 0u16;
    // Hot at 47 bits picks u64.
    let _: <Hot as BitsContainerFor<Wid<47>, Unsigned>>::T = 0u64;
    // Hot signed at 7 bits picks i8.
    let _: <Hot as BitsContainerFor<Wid<7>, Signed>>::T = 0i8;
    // Warm at 13 bits picks u32, the 2x-logical rule.
    let _: <Warm as BitsContainerFor<Wid<13>, Unsigned>>::T = 0u32;
    // Warm signed at 32 bits picks i64.
    let _: <Warm as BitsContainerFor<Wid<32>, Signed>>::T = 0i64;
};

/// A generic function threading the width as typestate, which is the shape a
/// downstream crate needs and the shape that overflowed GCE under the old form.
pub fn threaded<W, S, Sign>(
    v: <S as BitsContainerFor<W, Sign>>::T,
) -> <S as BitsContainerFor<W, Sign>>::T
where
    S: Strategy + BitsContainerFor<W, Sign>,
    Sign: Signedness,
{
    v
}

const _: () = {
    let _ = threaded::<Wid<13>, Hot, Unsigned>;
    let _ = threaded::<Wid<64>, Warm, Signed>;
};
