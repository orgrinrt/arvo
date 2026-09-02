//! Probe 1. The lowering door as a carrier born sealed, and the environment
//! declaration it cannot be reached without.
//!
//! Two carriers, both born under the carrier-at-birth rule (48:2.3): the door
//! kind, and the float environment the hardware door declares. The claim under
//! test is structural, not stylistic: there must be no inhabitant of `FloatEnv`
//! meaning "unspecified", because `HostFloat<Unspecified>` would be exactly the
//! lowering that changes delivered values, which the design already forbids
//! (58:798-806).
//!
//! Positive compile. The four attack routes are probe_1b through probe_1e.

mod sealed {
    pub trait Sealed {}
}

// ---- carrier one: the declared float environment -------------------------
//
// Every inhabitant NAMES a control state. There is no "unspecified" or
// "inherit" or "whatever the process has" inhabitant, by construction: the
// enumeration below is the whole set and the trait is sealed on it.

pub trait FloatEnv: sealed::Sealed + 'static {
    /// What a build layer reads. Nothing in arvo acts on it.
    const ROUNDING: Rounding;
    const FLUSH_TO_ZERO: bool;
    const DENORMALS_ARE_ZERO: bool;
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum Rounding {
    Nearest,
    TowardZero,
    TowardPositive,
    TowardNegative,
}

/// IEEE 754 default: round-to-nearest-ties-even, subnormals live.
pub struct IeeeDefault;
/// The audio/DSP deployment: nearest, but FZ and DAZ on.
pub struct FlushingNearest;
/// A directed-rounding deployment (interval arithmetic upper bound).
pub struct DirectedUp;

impl sealed::Sealed for IeeeDefault {}
impl sealed::Sealed for FlushingNearest {}
impl sealed::Sealed for DirectedUp {}

impl FloatEnv for IeeeDefault {
    const ROUNDING: Rounding = Rounding::Nearest;
    const FLUSH_TO_ZERO: bool = false;
    const DENORMALS_ARE_ZERO: bool = false;
}
impl FloatEnv for FlushingNearest {
    const ROUNDING: Rounding = Rounding::Nearest;
    const FLUSH_TO_ZERO: bool = true;
    const DENORMALS_ARE_ZERO: bool = true;
}
impl FloatEnv for DirectedUp {
    const ROUNDING: Rounding = Rounding::TowardPositive;
    const FLUSH_TO_ZERO: bool = false;
    const DENORMALS_ARE_ZERO: bool = false;
}

// ---- carrier two: the door ----------------------------------------------

pub trait LoweringDoor: sealed::Sealed + 'static {}

/// The design's own round-first quantiser. Delivers the numeral's value under
/// every environment, because it reads none.
pub struct Quantised;
/// The host's float instruction, under a declared environment. `E` is not
/// optional and has no default: naming this door names a control state.
pub struct HostFloat<E: FloatEnv>(core::marker::PhantomData<E>);

impl sealed::Sealed for Quantised {}
impl<E: FloatEnv> sealed::Sealed for HostFloat<E> {}
impl LoweringDoor for Quantised {}
impl<E: FloatEnv> LoweringDoor for HostFloat<E> {}

// ---- the door is a Lowering member --------------------------------------
//
// It sits here, and not on Policy or Numeral, because `Lowering` is the axis
// no law may read (49:151, restated 58:163-169). A door that changed a
// law-visible fact would not be a lowering; a door that does not is exactly
// what belongs on this axis. Compiled consequence in probe_4.

pub trait Lowering {
    type Door: LoweringDoor;
}

pub struct SoftBinary32;
pub struct HardBinary32;
pub struct HardBinary32Flushing;

impl Lowering for SoftBinary32 {
    type Door = Quantised;
}
impl Lowering for HardBinary32 {
    type Door = HostFloat<IeeeDefault>;
}
impl Lowering for HardBinary32Flushing {
    type Door = HostFloat<FlushingNearest>;
}

/// What a build layer reads off one monomorphised call site. Four scalars, no
/// machinery: arvo declares, the build layer acts. This function is the whole
/// of arvo's side of the receipt contract.
pub const fn receipt<L: Lowering>() -> Option<(Rounding, bool, bool)>
where
    L::Door: DeclaresEnv,
{
    <L::Door as DeclaresEnv>::DECLARED
}

pub trait DeclaresEnv {
    const DECLARED: Option<(Rounding, bool, bool)>;
}
impl DeclaresEnv for Quantised {
    const DECLARED: Option<(Rounding, bool, bool)> = None;
}
impl<E: FloatEnv> DeclaresEnv for HostFloat<E> {
    const DECLARED: Option<(Rounding, bool, bool)> =
        Some((E::ROUNDING, E::FLUSH_TO_ZERO, E::DENORMALS_ARE_ZERO));
}

fn main() {
    assert_eq!(receipt::<SoftBinary32>(), None);
    assert_eq!(
        receipt::<HardBinary32>(),
        Some((Rounding::Nearest, false, false))
    );
    assert_eq!(
        receipt::<HardBinary32Flushing>(),
        Some((Rounding::Nearest, true, true))
    );
    println!("probe_1 WORKS: door sealed, env sealed, receipt is four scalars off the type");
}
