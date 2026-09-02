// p3. Is a strategy expressible as a product of axis markers, with named
// presets over it, with downstream bounds written against PROPERTIES rather
// than against names, under the forbidden-feature set?
//
// 35 section 6 proposes, as a reframing rather than an option, that the canon
// state per strategy "which properties the arithmetic has (does the top
// absorb, is addition monotone, associative, invertible, does it distribute)
// rather than which policy it takes". Nobody has compiled it. This probe does,
// and separates four questions that the proposal runs together.
//
//   arm base    : the product, the presets, and the property traits, with
//                 properties IMPLEMENTED ON THE AXIS VALUE rather than declared
//                 per preset, so a preset inherits its properties instead of
//                 restating them.
//   arm bad1    : a downstream routine requiring an absorbing top, instantiated
//                 at a wrapping preset. MUST be refused.
//   arm bad2    : the same through the ergonomic blanket-implemented alias.
//                 MUST be refused.
//   arm override: a consumer taking a preset and changing one coordinate.
//   arm armswap : the same consumer source compiled against two different
//                 resolutions of the same preset name, which is what op's
//                 "the table is one arm" means mechanically. Under --cfg
//                 release_arm the preset's OBSERVABLE coordinate changes and a
//                 property bound that held stops holding. That failure is the
//                 point of the arm, not a defect in it.
//
// No feature gates. Compile with the workspace-pinned toolchain.
//
//   rustc +nightly-2026-05-28 --edition 2021 p3_axes_presets_properties.rs \
//         --crate-type lib -o /dev/null
//   ... --cfg bad1        (expected: E0277)
//   ... --cfg bad2        (expected: E0277)
//   ... --cfg release_arm (expected: E0277 at the armswap consumer)

#![allow(dead_code)]

use core::marker::PhantomData;

// ---------------------------------------------------------------------------
// The axes. Each is a closed set of markers plus a membership trait. The four
// axes are 25 section 4.1 and 4.2's list; nothing here claims the list is
// complete, and adding an axis is adding a parameter rather than editing the
// presets.
// ---------------------------------------------------------------------------

pub trait Headroom {}
pub struct Minimum;
pub struct Doubled;
impl Headroom for Minimum {}
impl Headroom for Doubled {}

pub trait Layout {}
pub struct Addressable;
pub struct Packed;
impl Layout for Addressable {}
impl Layout for Packed {}

pub trait Overflow {}
pub struct Wrap;
pub struct Saturate;
impl Overflow for Wrap {}
impl Overflow for Saturate {}

pub trait Intermediate {}
pub struct SameWidth;
pub struct Widened;
impl Intermediate for SameWidth {}
impl Intermediate for Widened {}

// ---------------------------------------------------------------------------
// The properties. Each is implemented on the AXIS VALUE, from a measurement,
// with the measurement named. This is the load-bearing arrangement: a preset
// does not declare its properties, it inherits them from the coordinates it
// assigns, so the properties cannot drift from the policy.
// ---------------------------------------------------------------------------

/// The numeral's top absorbs: `TOP + x == TOP`.
/// 35 section 3.4: 63 of 63 (W,F) cells under saturation, 0 of 63 under
/// wrapping.
#[diagnostic::on_unimplemented(
    message = "this strategy's overflow policy does not give the numeral an absorbing top",
    label = "min-plus and other tropical folds stand infinity on the top and need it to absorb",
    note = "saturating overflow supplies it; wrapping does not, and a wrapping top plus one is zero"
)]
pub trait AbsorbingTop {}
impl AbsorbingTop for Saturate {}
// deliberately no impl for Wrap.

/// `a <= b` implies `a + c <= b + c`.
/// 35 section 3.5: holds at 33 of 33 cells under saturation, fails at 33 of 33
/// under wrapping, at up to 33.07% of triples.
pub trait MonotoneAdd {}
impl MonotoneAdd for Saturate {}

/// Every value has an additive inverse.
/// 35 section 3.9: wrapping preserves it at 33 of 33 cells, saturation loses it
/// at 33 of 33, at up to 49.6% of pairs.
pub trait InvertibleAdd {}
impl InvertibleAdd for Wrap {}

// ---------------------------------------------------------------------------
// The strategy: a point in the product.
// ---------------------------------------------------------------------------

pub trait Strategy {
    type Headroom: Headroom;
    type Layout: Layout;
    type Overflow: Overflow;
    type Intermediate: Intermediate;
}

pub struct Strat<H, L, O, I>(PhantomData<(H, L, O, I)>);

impl<H: Headroom, L: Layout, O: Overflow, I: Intermediate> Strategy for Strat<H, L, O, I> {
    type Headroom = H;
    type Layout = L;
    type Overflow = O;
    type Intermediate = I;
}

// ---------------------------------------------------------------------------
// Presets, as aliases over the product. One token at a use site, which is the
// ergonomics bar the width-surface thread measured in characters.
//
// These assignments are NOT proposed as the right ones. They are the shipped
// preset table's, with the two silences filled by the least surprising value,
// so that the probe has something concrete to instantiate. p1 shows the shipped
// placement conflicts with op's stated intent, and nothing here depends on the
// placement being right.
// ---------------------------------------------------------------------------

pub type Hot = Strat<Minimum, Addressable, Wrap, SameWidth>;
pub type Warm = Strat<Doubled, Addressable, Wrap, SameWidth>;
pub type Cold = Strat<Minimum, Packed, Saturate, Widened>;
pub type Precise = Strat<Doubled, Packed, Saturate, Widened>;

// A per-axis override, so a consumer who needs one coordinate changed does not
// need a new name minted for them. A generic type alias with associated-type
// projections, no features.
pub type WithOverflow<S, O> =
    Strat<<S as Strategy>::Headroom, <S as Strategy>::Layout, O, <S as Strategy>::Intermediate>;

pub type WithLayout<S, L> =
    Strat<<S as Strategy>::Headroom, L, <S as Strategy>::Overflow, <S as Strategy>::Intermediate>;

// ---------------------------------------------------------------------------
// Downstream: a routine bounded on a property rather than on a name.
// ---------------------------------------------------------------------------

pub struct Numeral<const W: u32, S>(PhantomData<S>);

/// The shape arvo-graph's relaxation has: it stands infinity on the top and
/// relaxes, so it needs the top to absorb and addition to be monotone.
pub fn shortest_path<const W: u32, S>(_weights: &[Numeral<W, S>]) -> Numeral<W, S>
where
    S: Strategy,
    S::Overflow: AbsorbingTop + MonotoneAdd,
{
    Numeral(PhantomData)
}

/// An incrementally maintained aggregate, which needs to withdraw a
/// contribution and therefore needs the additive inverse.
pub fn retractable_sum<const W: u32, S>(_xs: &[Numeral<W, S>]) -> Numeral<W, S>
where
    S: Strategy,
    S::Overflow: InvertibleAdd,
{
    Numeral(PhantomData)
}

// The ergonomic form: the property lifted to the strategy by a blanket impl, so
// a consumer writes `S: Tropical` instead of `S::Overflow: AbsorbingTop + ...`.
pub trait Tropical {}
impl<S> Tropical for S
where
    S: Strategy,
    S::Overflow: AbsorbingTop + MonotoneAdd,
{
}

pub fn shortest_path_ergonomic<const W: u32, S: Strategy + Tropical>(
    _weights: &[Numeral<W, S>],
) -> Numeral<W, S> {
    Numeral(PhantomData)
}

// ---------------------------------------------------------------------------
// Positive arms: what a consumer can actually reach.
// ---------------------------------------------------------------------------

pub fn positive_named_preset(w: &[Numeral<13, Cold>]) -> Numeral<13, Cold> {
    // Cold saturates in this arm, so the tropical bound is satisfied by the
    // preset without the consumer naming a policy.
    shortest_path(w)
}

pub fn positive_ergonomic(w: &[Numeral<13, Precise>]) -> Numeral<13, Precise> {
    shortest_path_ergonomic(w)
}

pub fn positive_retraction(xs: &[Numeral<13, Hot>]) -> Numeral<13, Hot> {
    // Hot wraps, so it has the inverse and lacks absorption. The opposite
    // preset satisfies the opposite bound, which is the whole point of keying
    // the bound on the property.
    retractable_sum(xs)
}

// The override: a consumer who wants Hot's storage and a saturating top, which
// p2 records as a point no name reaches.
pub type HotTropical = WithOverflow<Hot, Saturate>;

pub fn positive_override(w: &[Numeral<13, HotTropical>]) -> Numeral<13, HotTropical> {
    shortest_path(w)
}

// And the reverse direction: a packed, minimal, saturating column, which p2
// names as the unreachable point with a consumer.
pub type ColdMinimalTropical = WithLayout<HotTropical, Packed>;

pub fn positive_override_two(
    w: &[Numeral<13, ColdMinimalTropical>],
) -> Numeral<13, ColdMinimalTropical> {
    shortest_path(w)
}

// ---------------------------------------------------------------------------
// Negative controls. Each MUST be refused.
// ---------------------------------------------------------------------------

#[cfg(bad1)]
pub fn negative_wrapping_preset(w: &[Numeral<13, Hot>]) -> Numeral<13, Hot> {
    // Hot wraps. 35 section 3.5 measures this exact instantiation getting
    // 48.9% of in-range DAG shortest paths wrong.
    shortest_path(w)
}

#[cfg(bad2)]
pub fn negative_wrapping_ergonomic(w: &[Numeral<13, Warm>]) -> Numeral<13, Warm> {
    shortest_path_ergonomic(w)
}

// ---------------------------------------------------------------------------
// The arm swap. op, quoted at 25 section 3.3: the preset tables are "one arm,
// perhaps the one we reserve for debug assertions time, and we write separate
// arms for release and such then".
//
// Mechanically an arm is a different resolution of the same preset name. The
// consumer source below is BYTE-IDENTICAL under both arms. Under the default
// arm `SwapWarm` saturates and the tropical bound holds. Under --cfg
// release_arm it wraps, imitating a native Rust primitive in release, and the
// same consumer source stops compiling.
//
// That is not a defect in the arm mechanism. It is what it means for an axis to
// be OBSERVABLE: moving it changes which programs are correct, so it cannot be
// resolved per arm the way headroom and layout can.
// ---------------------------------------------------------------------------

#[cfg(not(release_arm))]
pub type SwapWarm = Strat<Doubled, Addressable, Saturate, SameWidth>;
#[cfg(release_arm)]
pub type SwapWarm = Strat<Doubled, Addressable, Wrap, SameWidth>;

// Unobservable coordinates differ between the arms too, and no consumer notices.
#[cfg(not(release_arm))]
pub type SwapHot = Strat<Doubled, Addressable, Wrap, SameWidth>;
#[cfg(release_arm)]
pub type SwapHot = Strat<Minimum, Packed, Wrap, SameWidth>;

pub fn armswap_consumer_observable(w: &[Numeral<13, SwapWarm>]) -> Numeral<13, SwapWarm> {
    shortest_path(w)
}

pub fn armswap_consumer_unobservable(xs: &[Numeral<13, SwapHot>]) -> Numeral<13, SwapHot> {
    // Headroom and layout moved between the arms. This compiles in both,
    // because no bound in the layer above can name them.
    retractable_sum(xs)
}
