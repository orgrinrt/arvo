// Seat 253. Does the coordinate tuple a candidate exposes determine the thing the
// ratified identity clause says a format is?
//
// `proposal::a_format_is_identified_by_its_ambient_domain_and_its_representable_set`,
// ratified through `ruling::the_format_spine_is_canon`, identifies a format by the
// pair (ambient domain, representable set) and puts encoding and adaptation choice
// outside identity. It does not say whether two different coordinate assignments
// that denote one set are one format, because it locates the redundancy in the
// encoding and in the adaptation rather than in the coordinates themselves.
//
// This enumerates the denoted set of each candidate from its own exposed
// coordinates, through the crate's own accessors, and compares the sets.
//
// The denotation is the ratified affine predicate: a value is
//     phase + slot * radix^step_exponent(magnitude)
// carried here as an exact rational over `i128`, reduced, so two spellings of one
// value compare equal. `contains` is not used for this: it reads the two index
// ranges and nothing else, so it cannot distinguish two grids at all.
//
// PAIR 1 and PAIR 2 are the cases that must fail for "the tuple is a normal
// form", and they must come out equal-set / different-tuple.
// PAIR 3 is the control: the same comparator over two candidates whose sets do
// differ, so an equality above is a fact about the grids rather than a
// comparator stuck at true.

use arvo_format::ambient::{Ambient, BinaryRationals};
use arvo_format::format::{radix, step_exponent, Format, Phase};
use arvo_format::quantum::{Constant, Indexed, Magnitude, Quantum};
use arvo_format::slots::{Signed, Slots};

// --- the candidates ------------------------------------------------------------

/// A constant step at exponent zero over four signed bits.
struct ConstantStep;
impl Format for ConstantStep {
    type Ambient = BinaryRationals;
    type Quantum = Constant<0>;
    type Slots = Signed<4>;
    const PHASE: Phase = Phase::of(0, 1);
}

/// The same grid written with the indexed law at one magnitude, where the slope
/// is reached by nothing. A different coordinate tuple.
struct IndexedAtOneMagnitude;
impl Format for IndexedAtOneMagnitude {
    type Ambient = BinaryRationals;
    type Quantum = Indexed<0, 1>;
    type Slots = Signed<4>;
    const PHASE: Phase = Phase::of(0, 1);
}

/// The first grid again with the phase written as zero over two.
struct PhaseZeroOverTwo;
impl Format for PhaseZeroOverTwo {
    type Ambient = BinaryRationals;
    type Quantum = Constant<0>;
    type Slots = Signed<4>;
    const PHASE: Phase = Phase::of(0, 2);
}

/// The control. A constant step at exponent one, which is a different set.
struct ConstantStepAtOne;
impl Format for ConstantStepAtOne {
    type Ambient = BinaryRationals;
    type Quantum = Constant<1>;
    type Slots = Signed<4>;
    const PHASE: Phase = Phase::of(0, 1);
}

// --- exact rationals, so two spellings of one value compare equal ---------------

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
struct Q {
    num: i128,
    den: i128,
}

const fn gcd(a: i128, b: i128) -> i128 {
    let (mut a, mut b) = (if a < 0 { -a } else { a }, if b < 0 { -b } else { b });
    while b != 0 {
        let t = a % b;
        a = b;
        b = t;
    }
    a
}

impl Q {
    fn of(num: i128, den: i128) -> Self {
        assert!(den != 0, "the probe built a rational with no denominator");
        let (num, den) = if den < 0 { (-num, -den) } else { (num, den) };
        let g = gcd(num, den).max(1);
        Q { num: num / g, den: den / g }
    }
    fn add(self, o: Self) -> Self {
        Q::of(self.num * o.den + o.num * self.den, self.den * o.den)
    }
    fn mul_int(self, k: i128) -> Self {
        Q::of(self.num * k, self.den)
    }
}

/// `radix^exponent` as an exact rational.
fn power(base: u32, exp: i32) -> Q {
    let b = base as i128;
    let mut acc: i128 = 1;
    let mut n = exp.unsigned_abs();
    while n > 0 {
        acc *= b;
        n -= 1;
    }
    if exp >= 0 { Q::of(acc, 1) } else { Q::of(1, acc) }
}

/// The denoted set, enumerated from the exposed coordinates through the crate's
/// own accessors and nothing else.
fn denoted<F: Format>() -> Vec<Q> {
    let r = radix::<F>().base();
    let phase = Q::of(
        <F as Format>::PHASE.numerator() as i128,
        <F as Format>::PHASE.denominator() as i128,
    );
    let min = <F::Slots as Slots>::MIN.index() as i128;
    let max = <F::Slots as Slots>::MAX.index() as i128;
    let mags = <F::Quantum as Quantum>::MAGNITUDES.count();

    let mut out = Vec::new();
    for m in 0..mags {
        let e = step_exponent::<F>(Magnitude::at(m)).power();
        let step = power(r, e);
        for s in min..=max {
            out.push(phase.add(step.mul_int(s)));
        }
    }
    out.sort();
    out.dedup();
    out
}

/// The exposed tuple, as a printable list of the coordinates a candidate fixes.
fn tuple<F: Format>() -> String {
    format!(
        "RADIX={} SIGNED={} BASE={} SLOPE={} MAGNITUDES={} MIN={} MAX={} WIDTH={} PHASE={}/{}",
        <F::Ambient as Ambient>::RADIX.base(),
        <F::Ambient as Ambient>::SIGNED.get(),
        <F::Quantum as Quantum>::BASE.power(),
        <F::Quantum as Quantum>::SLOPE.power(),
        <F::Quantum as Quantum>::MAGNITUDES.count(),
        <F::Slots as Slots>::MIN.index(),
        <F::Slots as Slots>::MAX.index(),
        <F::Slots as Slots>::WIDTH.count(),
        <F as Format>::PHASE.numerator(),
        <F as Format>::PHASE.denominator(),
    )
}

fn compare<A: Format, B: Format>(label: &str, a: &str, b: &str) {
    let (ta, tb) = (tuple::<A>(), tuple::<B>());
    let (sa, sb) = (denoted::<A>(), denoted::<B>());
    println!("{label}");
    println!("  {a}");
    println!("    tuple : {ta}");
    println!("    set   : {} values, first {:?} last {:?}", sa.len(), sa.first(), sa.last());
    println!("  {b}");
    println!("    tuple : {tb}");
    println!("    set   : {} values, first {:?} last {:?}", sb.len(), sb.first(), sb.last());
    println!("  same ambient type : {}", core::any::type_name::<A::Ambient>() == core::any::type_name::<B::Ambient>());
    println!("  tuples equal      : {}", ta == tb);
    println!("  denoted sets equal: {}", sa == sb);
    println!();
}

fn main() {
    println!("probe: the exposed tuple against the set it denotes\n");

    compare::<ConstantStep, IndexedAtOneMagnitude>(
        "PAIR 1 (must fail for 'the tuple is a normal form'): the quantum law spelled two ways",
        "ConstantStep          Quantum = Constant<0>",
        "IndexedAtOneMagnitude Quantum = Indexed<0, 1>",
    );

    compare::<ConstantStep, PhaseZeroOverTwo>(
        "PAIR 2 (must fail for 'the tuple is a normal form'): the phase spelled two ways",
        "ConstantStep     PHASE = 0/1",
        "PhaseZeroOverTwo PHASE = 0/2",
    );

    compare::<ConstantStep, ConstantStepAtOne>(
        "PAIR 3 (the control): two candidates whose sets do differ",
        "ConstantStep      Quantum = Constant<0>",
        "ConstantStepAtOne Quantum = Constant<1>",
    );

    contains_agrees::<ConstantStep, ConstantStepAtOne>(
        "PAIR 3 again, through the shipped membership predicate",
    );
    contains_agrees::<ConstantStep, ConstantStep2Bits>(
        "CONTROL for that, a candidate with a different slot range",
    );
    println!();

    // The second control, on the comparator rather than on the candidates: a
    // format compared with itself must come out equal on both axes, so an
    // inequality above is not a comparator that never returns true.
    compare::<ConstantStep, ConstantStep>(
        "PAIR 4 (the comparator's own control): a candidate against itself",
        "ConstantStep",
        "ConstantStep",
    );
}

/// The shipped membership predicate, over the union of two index rectangles.
///
/// Separate from the set comparison above because it answers a different
/// question. `contains` is documented as "the affine predicate, evaluated", and
/// it reads `magnitude_in_range` and `slot_in_range` and nothing else, so it
/// consults neither the quantum's exponent nor the phase nor the radix.
fn contains_agrees<A: Format, B: Format>(label: &str) {
    use arvo_format::format::contains;
    let mut same = 0usize;
    let mut differ = 0usize;
    for m in 0u32..4 {
        for s in -10i64..=10 {
            let (x, y) = (
                contains::<A>(arvo_format::slots::Slot::at(s), Magnitude::at(m)).get(),
                contains::<B>(arvo_format::slots::Slot::at(s), Magnitude::at(m)).get(),
            );
            if x == y { same += 1 } else { differ += 1 }
        }
    }
    println!("{label}: contains agrees on {same} of {} points, differs on {differ}", same + differ);
}

/// The control for the `contains` arm: a different slot range, which is one of
/// the two things `contains` does read.
struct ConstantStep2Bits;
impl Format for ConstantStep2Bits {
    type Ambient = BinaryRationals;
    type Quantum = Constant<0>;
    type Slots = Signed<2>;
    const PHASE: Phase = Phase::of(0, 1);
}
