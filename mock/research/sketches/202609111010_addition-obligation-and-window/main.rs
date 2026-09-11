//! Sketch: does rustc accept the three shapes the addition round rests on.
//!
//! H1. An obligation carried as an inherent associated const on a generic
//!     struct, forced inside a generic `const fn` with `let () = ...;`, refuses
//!     a magnitude-indexed format at build, and at check when the call is bound
//!     in a `const` item.
//! H2. A slot range over two const generic bounds, `Window<LO, HI>`, implements
//!     the open `Slots` trait with its width derived in the associated const,
//!     with no `generic_const_exprs`, and passes the slot range's obligation.
//! H3. The exact step for addition, written from the denotation
//!     `value = (phase + slot) * quantum`, composed with the crate's own `adapt`,
//!     reproduces the canon's witness count for signed saturating addition at
//!     four bits (952 divergent triples) and zero for wrapping and for unsigned
//!     saturation.
//! H4. The carry obligation (the exact sum stays inside the slot coordinate) is
//!     reachable: some admitted format fails it and is refused at check.
//!
//! Scaffolding. Names, arities and which cases are instantiated are chosen to
//! reach the checks and are not design decisions.

use core::marker::PhantomData;

use arvo_format::adapt::{Adapt, DeclaredSignature, Signature};
use arvo_format::ambient::BinaryRationals;
use arvo_format::apply::{Dither, Exact, Fraction, adapt};
use arvo_format::format::{Format, Phase};
use arvo_format::overflow::{Saturate, Wrap};
use arvo_format::points::{Biased, Integer, UFixed};
use arvo_format::quantum::{Constant, is_constant_family};
use arvo_format::rounding::Floor;
use arvo_format::slots::{Slot, Slots};
use arvo_format::width::Width;

// --- H1 and H4 ---------------------------------------------------------------

/// Whole and fractional part of the phase, one width up, sign normalised.
const fn phase_parts<F: Format>() -> (i128, i128, i128) {
    let n = F::PHASE.numerator() as i128;
    let d = F::PHASE.denominator() as i128;
    let (n, d) = if d < 0 { (-n, -d) } else { (n, d) };
    (n.div_euclid(d), n.rem_euclid(d), d)
}

const fn sum_is_carried<F: Format>() -> bool {
    let (whole, rem, _) = phase_parts::<F>();
    let ceil = if rem == 0 { whole } else { whole + 1 };
    let lo = 2 * (<F::Slots as Slots>::MIN.index() as i128) + whole;
    let hi = 2 * (<F::Slots as Slots>::MAX.index() as i128) + ceil;
    lo >= i64::MIN as i128 && hi <= i64::MAX as i128
}

struct Addable<F>(PhantomData<F>);

impl<F: Format> Addable<F> {
    const ADMITTED: () = {
        assert!(
            is_constant_family::<F::Quantum>().get(),
            "addition over a magnitude-indexed quantum is refused: the applied map carries no \
             magnitude coordinate"
        );
        assert!(
            sum_is_carried::<F>(),
            "the exact sum of two members leaves the slot coordinate"
        );
    };
}

/// The exact step: where the ambient sum of two members sits on the grid.
///
/// `(phase + a) q + (phase + b) q = (phase + (a + b + phase)) q`, so the
/// position is `a + b + phase` in slot units, and neither the quantum nor the
/// radix appears.
const fn sum_position<F: Format>(a: Slot, b: Slot) -> Exact {
    let () = Addable::<F>::ADMITTED;
    let (whole, rem, d) = phase_parts::<F>();
    let slot = (a.index() as i128) + (b.index() as i128) + whole;
    Exact::between(Slot::at(slot as i64), Fraction::of(rem as i64, d as i64))
}

const fn add<S: DeclaredSignature>(a: Slot, b: Slot) -> Slot {
    adapt::<S>(sum_position::<S::Format>(a, b), Dither::UNUSED)
}

#[cfg(feature = "refuse_indexed_at_build")]
fn refused_at_build() -> Exact {
    use arvo_format::points::Floating;
    sum_position::<Floating<4, -3, 4>>(Slot::at(1), Slot::at(2))
}

#[cfg(feature = "refuse_indexed_at_check")]
const REFUSED_AT_CHECK: Exact = {
    use arvo_format::points::Floating;
    sum_position::<Floating<4, -3, 4>>(Slot::at(1), Slot::at(2))
};

/// A phase of `i64::MAX` whole quanta over an eight-bit signed range: every
/// coordinate admitted on its own, and the sum of two members is not carried.
///
/// The slot range is named by its full path so the default build, which does
/// not compile this arm, has no import a fixer would strip.
#[cfg(feature = "refuse_uncarried_sum_at_check")]
struct FarPhase;

#[cfg(feature = "refuse_uncarried_sum_at_check")]
impl Format for FarPhase {
    type Ambient = BinaryRationals;
    type Quantum = Constant<0>;
    type Slots = arvo_format::slots::Signed<8>;

    const PHASE: Phase = Phase::of(i64::MAX, 1);
}

#[cfg(feature = "refuse_uncarried_sum_at_check")]
const REFUSED_UNCARRIED: Exact = sum_position::<FarPhase>(Slot::at(1), Slot::at(2));

// --- H2 ----------------------------------------------------------------------

const fn width_for(lo: i64, hi: i64) -> u32 {
    // Smallest w with hi - lo < 2^w.
    let span = (hi as i128) - (lo as i128);
    let mut w = 1u32;
    while (1i128 << w) <= span {
        w += 1;
    }
    w
}

struct Window<const LO: i64, const HI: i64>;

impl<const LO: i64, const HI: i64> Slots for Window<LO, HI> {
    const MAX: Slot = Slot::at(HI);
    const MIN: Slot = Slot::at(LO);
    const WIDTH: Width = Width::bits(width_for(LO, HI));
}

struct Over<const LO: i64, const HI: i64>;

impl<const LO: i64, const HI: i64> Format for Over<LO, HI> {
    type Ambient = BinaryRationals;
    type Quantum = Constant<0>;
    type Slots = Window<LO, HI>;

    const PHASE: Phase = Phase::ZERO;
}

// --- H3 ----------------------------------------------------------------------

fn divergent<S: DeclaredSignature>() -> u64 {
    let lo = <<S::Format as Format>::Slots as Slots>::MIN.index();
    let hi = <<S::Format as Format>::Slots as Slots>::MAX.index();
    let mut n = 0;
    for a in lo ..= hi {
        for b in lo ..= hi {
            let ab = add::<S>(Slot::at(a), Slot::at(b));
            for c in lo ..= hi {
                let left = add::<S>(ab, Slot::at(c));
                let right = add::<S>(Slot::at(a), add::<S>(Slot::at(b), Slot::at(c)));
                if left != right {
                    n += 1;
                }
            }
        }
    }
    n
}

fn main() {
    #[cfg(feature = "refuse_indexed_at_build")]
    let _ = refused_at_build();
    #[cfg(feature = "refuse_indexed_at_check")]
    let _ = REFUSED_AT_CHECK;
    #[cfg(feature = "refuse_uncarried_sum_at_check")]
    let _ = REFUSED_UNCARRIED;

    println!(
        "H3 Integer<4> saturate divergent = {}",
        divergent::<Signature<Integer<4>, Adapt<Floor, Saturate>>>()
    );
    println!(
        "H3 Integer<4> wrap divergent = {}",
        divergent::<Signature<Integer<4>, Adapt<Floor, Wrap>>>()
    );
    println!(
        "H3 UFixed<4,0> saturate divergent = {}",
        divergent::<Signature<UFixed<4, 0>, Adapt<Floor, Saturate>>>()
    );
    println!(
        "H3 UFixed<4,-3> saturate divergent = {}",
        divergent::<Signature<UFixed<4, -3>, Adapt<Floor, Saturate>>>()
    );
    println!(
        "H3 Biased<4,-2,0> saturate divergent = {}",
        divergent::<Signature<Biased<4, -2, 0>, Adapt<Floor, Saturate>>>()
    );
    println!(
        "H2 Window<-4,0> saturate divergent = {}",
        divergent::<Signature<Over<-4, 0>, Adapt<Floor, Saturate>>>()
    );
    println!(
        "H2 Window<-1,7> saturate divergent = {}",
        divergent::<Signature<Over<-1, 7>, Adapt<Floor, Saturate>>>()
    );
    println!(
        "H2 Window<-4,0> width = {}",
        <Window<-4, 0> as Slots>::WIDTH.count()
    );
    // The half-step phase: the sum of two members sits on a tie.
    let e = sum_position::<Biased<4, 0, 1>>(Slot::at(1), Slot::at(2));
    println!(
        "H3 Biased<4,0,1> 1+2 position slot={} tie={}",
        e.slot().index(),
        e.is_tie().get()
    );
    // H4 control: the widest shipped phase at the widest shipped range is still
    // carried, so the carry refusal is reachable only by an outside format.
    println!(
        "H4 Biased<62,0,i64::MAX> carried = {}, Biased<62,0,i64::MIN> carried = {}",
        sum_is_carried::<Biased<62, 0, { i64::MAX }>>(),
        sum_is_carried::<Biased<62, 0, { i64::MIN }>>()
    );
    let t = std::time::Instant::now();
    let w8 = divergent::<Signature<Integer<8>, Adapt<Floor, Saturate>>>();
    println!(
        "H3 Integer<8> saturate divergent = {} ({} ms, debug, ad-hoc spike, not a measurement)",
        w8,
        t.elapsed().as_millis()
    );
}
