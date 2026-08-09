//! Probe 3. The door projection that survives: the strategy selects a default
//! LOWERING through the numeral, and the door is that lowering's own member.
//!
//! Three facts decide a door and they come from three places, which is why no
//! projection keyed on the strategy alone can be total (probe 2):
//!
//!   - whether this target's silicon implements the numeral: a TARGET fact,
//!     carried by the numeral as a type (`Numeral::Host`);
//!   - which control state the deployment guarantees: a DEPLOYMENT fact,
//!     carried by the lowering (`HostFloat<E>`);
//!   - which door the preset prefers where both exist: a STRATEGY fact.
//!
//! The tag is a TYPE, not a const, because it is computed and has to appear in
//! a bound. That is the spine rule (58:85-99) firing at the lowering axis. It
//! is also why this probe does NOT copy the shipped `BitsContainerFor` shape,
//! whose `where Picker: Project<{ tag_hot_cold(N) }, ...>` clause
//! (arvo-strategy/src/container.rs:254-258) is a const expression over a
//! generic const parameter in a bound and needs the forbidden
//! `generic_const_exprs` (arvo-strategy/src/lib.rs:11). See probe_3b.
//!
//! NEGATIVE CONTROL for probe_3. `Hot` at a numeral the host does not
//! implement. Expected: refusal at the bound, with the carrier's own
//! diagnostic naming every legal alternative.

mod sealed {
    pub trait Sealed {}
}
use core::marker::PhantomData;

// --- doors and environments (probe_1's carriers, unchanged) --------------
pub trait FloatEnv: sealed::Sealed + 'static {}
pub struct IeeeDefault;
pub struct FlushingNearest;
impl sealed::Sealed for IeeeDefault {}
impl sealed::Sealed for FlushingNearest {}
impl FloatEnv for IeeeDefault {}
impl FloatEnv for FlushingNearest {}

pub trait LoweringDoor: sealed::Sealed + 'static {}
pub struct Quantised;
pub struct HostFloat<E: FloatEnv>(PhantomData<E>);
impl sealed::Sealed for Quantised {}
impl<E: FloatEnv> sealed::Sealed for HostFloat<E> {}
impl LoweringDoor for Quantised {}
impl<E: FloatEnv> LoweringDoor for HostFloat<E> {}

// --- the host-implementedness carrier, born sealed ----------------------
#[diagnostic::on_unimplemented(
    message = "`{Self}` is not a numeral this target's floating-point unit implements",
    note = "The `Hot` preset lowers a float operation to the host instruction, which \
            exists only for the numerals the target provides (binary16/32/64 on \
            aarch64-apple-darwin). Choose `Warm`, `Cold` or `Precise`, which lower \
            through the software quantiser at every numeral, or choose a numeral the \
            host implements."
)]
pub trait HostImplemented: sealed::Sealed + 'static {}

/// The target has an instruction for this numeral.
pub struct Hosted;
/// It does not. Not an error, and not a fallback: a fact.
pub struct NotHosted;
impl sealed::Sealed for Hosted {}
impl sealed::Sealed for NotHosted {}
impl HostImplemented for Hosted {}
// deliberately no `impl HostImplemented for NotHosted`. Absence IS the
// mechanism, exactly as the absence of a `Project` impl is what makes
// `Uint<100, Warm>` a compile error rather than a silent widening
// (arvo-strategy/src/container.rs:110-112).

// --- numerals ------------------------------------------------------------
pub trait Numeral: 'static {
    type Host: sealed::Sealed + 'static;
}
pub struct Binary32;
pub struct Binary64;
/// p=11, emin=-14, emax=15, Underflow=Abrupt. A legal `Ranged` numeral with no
/// instruction anywhere: abrupt underflow alone puts it off every FPU.
pub struct Ranged11Abrupt;
/// A decimal numeral. Radix ten, no hardware on any target this workspace pins.
pub struct Decimal32;
impl Numeral for Binary32 {
    type Host = Hosted;
}
impl Numeral for Binary64 {
    type Host = Hosted;
}
impl Numeral for Ranged11Abrupt {
    type Host = NotHosted;
}
impl Numeral for Decimal32 {
    type Host = NotHosted;
}

// --- lowerings -----------------------------------------------------------
pub trait Lowering: 'static {
    type Door: LoweringDoor;
    // Encoding / StoredWidth / Layout elided; unchanged from 58:871-876.
}
pub struct SoftwareLowering<N: Numeral>(PhantomData<N>);
pub struct HostLowering<N: Numeral, E: FloatEnv>(PhantomData<(N, E)>);
impl<N: Numeral> Lowering for SoftwareLowering<N> {
    type Door = Quantised;
}
impl<N: Numeral, E: FloatEnv> Lowering for HostLowering<N, E> {
    type Door = HostFloat<E>;
}

// --- the strategy axis ---------------------------------------------------
pub trait Strategy: 'static {
    const RANK: u16;
}
pub struct Hot;
pub struct Warm;
pub struct Cold;
pub struct Precise;
impl Strategy for Hot {
    const RANK: u16 = 0;
}
impl Strategy for Warm {
    const RANK: u16 = 1;
}
impl Strategy for Cold {
    const RANK: u16 = 2;
}
impl Strategy for Precise {
    const RANK: u16 = 3;
}

/// What the strategy actually selects: a DEFAULT lowering for a numeral. Not a
/// door. The door is that lowering's own member, and a consumer who names a
/// lowering explicitly has already named a door, which is what the axis is for.
pub trait DefaultLowering<N: Numeral>: Strategy {
    type L: Lowering;
}

/// `Hot` routes through the numeral's own host-implementedness. One impl, total
/// over the trait's own domain, partial over numerals by the BOUND rather than
/// by a second impl.
impl<N: Numeral> DefaultLowering<N> for Hot
where
    N::Host: HostImplemented,
{
    type L = HostLowering<N, IeeeDefault>;
}
impl<N: Numeral> DefaultLowering<N> for Warm {
    type L = SoftwareLowering<N>;
}
impl<N: Numeral> DefaultLowering<N> for Cold {
    type L = SoftwareLowering<N>;
}
impl<N: Numeral> DefaultLowering<N> for Precise {
    type L = SoftwareLowering<N>;
}

pub type DoorOf<S, N> = <<S as DefaultLowering<N>>::L as Lowering>::Door;

// --- the check -----------------------------------------------------------
trait Same<T> {}
impl<T> Same<T> for T {}
fn assert_door<S, N, D>()
where
    N: Numeral,
    S: DefaultLowering<N>,
    DoorOf<S, N>: Same<D>,
{
}

fn main() {
    // Hot on a hosted numeral: the hardware door, environment declared.
    assert_door::<Hot, Binary32, HostFloat<IeeeDefault>>();
    assert_door::<Hot, Binary64, HostFloat<IeeeDefault>>();
    // Every other preset, at every numeral: the quantiser.
    assert_door::<Warm, Binary32, Quantised>();
    assert_door::<Warm, Ranged11Abrupt, Quantised>();
    assert_door::<Cold, Binary32, Quantised>();
    assert_door::<Cold, Decimal32, Quantised>();
    assert_door::<Precise, Binary32, Quantised>();
    assert_door::<Precise, Decimal32, Quantised>();
    // A consumer whose deployment runs FZ names its own lowering; arvo decides
    // no environment on that consumer's behalf.
    fn _explicit<L: Lowering>() {}
    _explicit::<HostLowering<Binary32, FlushingNearest>>();
    assert_door::<Hot, Ranged11Abrupt, Quantised>();
    assert_door::<Hot, Decimal32, HostFloat<IeeeDefault>>();
}
