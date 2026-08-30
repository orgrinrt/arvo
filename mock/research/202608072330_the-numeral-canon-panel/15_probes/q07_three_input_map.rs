// q07. The container map with all three inputs: (strategy, width, sign).
//
// Every ladder built in the second stretch is `width -> container`. Op's own
// words, quoted at 11:871-873 from 130b:39-43, are "the strategy guides
// container selection, not the user". This probe asks whether the arrangement
// survives when the strategy and the sign are real inputs rather than inert
// markers, and what it costs in impls.
//
// WHAT IS ASSUMED AND WHAT IS DERIVED. The record does not settle what each
// strategy does to container selection. This probe therefore does NOT invent
// that semantics as a finding. It builds the map's SHAPE with all three inputs
// live, instantiates it with one assignment marked below as an assumption, and
// measures what the shape costs. The assignment can be replaced without
// touching a line of the mechanism, which is the property being tested.
//
// The assignment used, and it is an assumption, not a result:
//   Hot      native rungs to 128 bits, then a byte buffer at align 16
//   Warm     native rungs to 128 bits, then a byte buffer at align 1
//   Precise  as Warm
//   Cold     as Warm for the standalone container, and stride exactly W bits
//
// Toolchain: rustc 1.98.0-nightly (57d06900f 2026-05-27), pin nightly-2026-05-28.
// Features: none. No -Z flags. Default solver. Edition 2024.
//
// Build: rustc +nightly-2026-05-28 --edition 2024 --crate-type lib \
//          q07_three_input_map.rs --out-dir build

#![no_std]
#![allow(dead_code)]

include!("q04_core_inc.rs");
include!("q04_literals_inc.rs");

// ---------------------------------------------------------------- the axes ---
pub struct Hot;
pub struct Warm;
pub struct Cold;
pub struct Precise;

pub struct Unsigned;
pub struct Signed;

// The sign axis supplies the native primitive family. Two impls, five rows
// each. This is a closed record over a closed vocabulary, not a width table:
// nothing here grows when a consumer writes a new width.
pub trait Signedness {
    type B8;
    type B16;
    type B32;
    type B64;
    type B128;
}
impl Signedness for Unsigned {
    type B8 = u8;
    type B16 = u16;
    type B32 = u32;
    type B64 = u64;
    type B128 = u128;
}
impl Signedness for Signed {
    type B8 = i8;
    type B16 = i16;
    type B32 = i32;
    type B64 = i64;
    type B128 = i128;
}

// The wide arm. A byte sequence at align 1, and the same at align 16, so that
// the strategy can choose the alignment without the width ladder knowing.
#[repr(C, align(16))]
pub struct Aligned16<T>(T);

// The width side, unchanged from 13's ladder: a pure function of W into one of
// six rung markers. Finite, total, and it never mentions a strategy or a sign.
pub struct R8;
pub struct R16;
pub struct R32;
pub struct R64;
pub struct R128;
pub struct RWide;

pub type RungOf<W> = Rung<
    W,
    N8,
    R8,
    Rung<W, N16, R16, Rung<W, N32, R32, Rung<W, N64, R64, Rung<W, N128, R128, RWide>>>>,
>;

// One name for the width side's bound bundle.
pub trait HasRung {
    type R;
}
impl<W> HasRung for W
where
    W: Cmp<N8> + Cmp<N16> + Cmp<N32> + Cmp<N64> + Cmp<N128>,
    Ord2<W, N128>: IfLe<R128, RWide>,
    Ord2<W, N64>: IfLe<R64, <Ord2<W, N128> as IfLe<R128, RWide>>::Out>,
    Ord2<W, N32>:
        IfLe<R32, <Ord2<W, N64> as IfLe<R64, <Ord2<W, N128> as IfLe<R128, RWide>>::Out>>::Out>,
    Ord2<W, N16>: IfLe<
        R16,
        <Ord2<W, N32> as IfLe<
            R32,
            <Ord2<W, N64> as IfLe<R64, <Ord2<W, N128> as IfLe<R128, RWide>>::Out>>::Out,
        >>::Out,
    >,
    Ord2<W, N8>: IfLe<
        R8,
        <Ord2<W, N16> as IfLe<
            R16,
            <Ord2<W, N32> as IfLe<
                R32,
                <Ord2<W, N64> as IfLe<R64, <Ord2<W, N128> as IfLe<R128, RWide>>::Out>>::Out,
            >>::Out,
        >>::Out,
    >,
{
    type R = RungOf<W>;
}

// -------------------------------------------------- the three-input map -----
// (strategy, width, sign) -> (container, stride). Two outputs, because the
// acceptance criterion at SETTLED.md:65-71 names two: "the matching container
// AND numeral representations".
//
// Container is the standalone value's type. Stride is a nat: the bits one
// element occupies inside an aggregate. They coincide for three strategies and
// come apart for Cold, which is the point.
pub trait Realise<Sn: Signedness, W> {
    type Container;
    type Stride;
}

// The per-rung realisation, so each strategy states five rows and a wide arm
// rather than repeating the comparison ladder.
pub trait AtRung<Sn: Signedness, W> {
    type Out;
    // the bits one of these occupies in an aggregate. It is NOT a function of
    // the rung alone: Hot's wide arm pads to align 16, so the same rung gives a
    // different stride under a different strategy. That is the second instance
    // of the defect noted below and it is why this sits here rather than on the
    // rung marker.
    type Bits;
}

// Doubling by ADDITION drags an `Add` bound per level and the bounds nest, which
// is the avalanche 13:214-231 describes. It is avoidable rather than nameable:
// on a little-endian binary nat, doubling is prepending an even digit, and the
// canonical constructor `Ev` is total over every nat. So scaling by 8 or 16
// costs three or four `MkE` bounds that always hold, and zero `Add` bounds.
pub type Sh1<A> = Ev<A>;
pub type Sh3<A> = Sh1<Sh1<Sh1<A>>>; // times 8
pub type Sh4<A> = Sh1<Sh3<A>>; // times 16
pub type Up16<A> = Sh4<CH<CH<CH<CH<A>>>>>; // 16 * ceil(A / 16)

// Hot: native to 128, then a byte buffer at align 16.
impl<Sn: Signedness, W> AtRung<Sn, W> for (Hot, R8) {
    type Out = Sn::B8;
    type Bits = N8;
}
impl<Sn: Signedness, W> AtRung<Sn, W> for (Hot, R16) {
    type Out = Sn::B16;
    type Bits = N16;
}
impl<Sn: Signedness, W> AtRung<Sn, W> for (Hot, R32) {
    type Out = Sn::B32;
    type Bits = N32;
}
impl<Sn: Signedness, W> AtRung<Sn, W> for (Hot, R64) {
    type Out = Sn::B64;
    type Bits = N64;
}
impl<Sn: Signedness, W> AtRung<Sn, W> for (Hot, R128) {
    type Out = Sn::B128;
    type Bits = N128;
}
// The one place a strategy changes the stride at a rung it shares: Hot's wide
// arm is align 16, so a 30-byte payload occupies 32.
impl<Sn: Signedness, W> AtRung<Sn, W> for (Hot, RWide)
where
    W: CeilHalf,
    CH<W>: CeilHalf,
    CH<CH<W>>: CeilHalf,
    Bytes<W>: Buf + CeilHalf,
    CH<Bytes<W>>: CeilHalf,
    CH<CH<Bytes<W>>>: CeilHalf,
    CH<CH<CH<Bytes<W>>>>: CeilHalf,
    CH<CH<CH<CH<Bytes<W>>>>>: MkE,
    Sh1<CH<CH<CH<CH<Bytes<W>>>>>>: MkE,
    Sh1<Sh1<CH<CH<CH<CH<Bytes<W>>>>>>>: MkE,
    Sh1<Sh1<Sh1<CH<CH<CH<CH<Bytes<W>>>>>>>>: MkE,
    Up16<Bytes<W>>: MkE,
    Sh1<Up16<Bytes<W>>>: MkE,
    Sh1<Sh1<Up16<Bytes<W>>>>: MkE,
{
    type Out = Aligned16<BufOf<Bytes<W>>>;
    type Bits = Sh3<Up16<Bytes<W>>>;
}

// Warm, Cold and Precise share the align-1 wide arm. Written out per strategy
// rather than blanket-implemented, because a blanket impl over the strategy is
// exactly the thing that makes the axis inert again.
macro_rules! align1_family {
    ($($S:ty),*) => { $(
        impl<Sn: Signedness, W> AtRung<Sn, W> for ($S, R8) { type Out = Sn::B8; type Bits = N8; }
        impl<Sn: Signedness, W> AtRung<Sn, W> for ($S, R16) { type Out = Sn::B16; type Bits = N16; }
        impl<Sn: Signedness, W> AtRung<Sn, W> for ($S, R32) { type Out = Sn::B32; type Bits = N32; }
        impl<Sn: Signedness, W> AtRung<Sn, W> for ($S, R64) { type Out = Sn::B64; type Bits = N64; }
        impl<Sn: Signedness, W> AtRung<Sn, W> for ($S, R128) { type Out = Sn::B128; type Bits = N128; }
        impl<Sn: Signedness, W> AtRung<Sn, W> for ($S, RWide)
        where
            W: CeilHalf, CH<W>: CeilHalf, CH<CH<W>>: CeilHalf,
            Bytes<W>: Buf + MkE, Sh1<Bytes<W>>: MkE, Sh1<Sh1<Bytes<W>>>: MkE,
        { type Out = BufOf<Bytes<W>>; type Bits = Sh3<Bytes<W>>; }
    )* };
}
align1_family!(Warm, Cold, Precise);

// Cold packs at the logical width; the others occupy their container, whose
// width the (strategy, rung) pair above states.
pub trait StrideOf<Sn: Signedness, W> {
    type Out;
}
impl<Sn: Signedness, W> StrideOf<Sn, W> for Cold {
    type Out = W;
}
macro_rules! rung_stride {
    ($($S:ty),*) => { $(
        impl<Sn: Signedness, W> StrideOf<Sn, W> for $S
        where W: HasRung, ($S, <W as HasRung>::R): AtRung<Sn, W>,
        { type Out = <($S, <W as HasRung>::R) as AtRung<Sn, W>>::Bits; }
    )* };
}
rung_stride!(Hot, Warm, Precise);

// And the map itself: one impl, generic over all three axes.
impl<S, Sn: Signedness, W> Realise<Sn, W> for S
where
    W: HasRung,
    (S, <W as HasRung>::R): AtRung<Sn, W>,
    S: StrideOf<Sn, W>,
{
    type Container = <(S, <W as HasRung>::R) as AtRung<Sn, W>>::Out;
    type Stride = <S as StrideOf<Sn, W>>::Out;
}

pub type ContainerOf<S, Sn, W> = <S as Realise<Sn, W>>::Container;
pub type StrideBits<S, Sn, W> = <S as Realise<Sn, W>>::Stride;
