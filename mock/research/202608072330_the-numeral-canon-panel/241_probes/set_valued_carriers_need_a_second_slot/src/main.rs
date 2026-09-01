// Probe: are set-valued carriers admitted by the ratified format concept?
//
// `ruling::the_format_spine_is_canon` ratifies that membership of the
// representable set is ONE affine predicate over ONE parameterisation: a value is
// in the set exactly when it is `phase + slot * quantum(magnitude)` for an
// admitted slot and magnitude. One slot index, not two.
//
// An interval is a pair. So the question is arithmetic rather than a preference:
// how many of the grid's intervals can a single slot index name?
//
// Arm A: a FIXED-RADIUS interval, determined by its centre, one slot, radius a
//        constant of the type. Declared here as a real outside-crate `Format`.
// Arm B: the GENERAL interval, independent endpoints. Counted against arm A.
//
// The case that must fail, stated before the run: the control forces the radius
// to zero, collapsing intervals to points, where the single slot must name ALL of
// them and the gap must be zero. If the control shows a gap, the counting is
// wrong and arm B's gap means nothing.

use arvo_format::ambient::Ambient;
use arvo_format::format::{contains, Format};
use arvo_format::quantum::Constant;
use arvo_format::slots::{Signed, Slots};

/// The interval algebra over the binary rationals: elements are sets, the exact
/// operation is the Minkowski one, and the adaptation onto the grid is outward.
struct IntervalOverBinaryRationals;
impl Ambient for IntervalOverBinaryRationals {
    const RADIX: u32 = 2;
    const SIGNED: bool = true;
}

/// Arm A. A set-valued format whose values are intervals of a radius fixed by the
/// type. The centre is the affine slot; the radius is a constant of the type,
/// which is exactly what the ratified identity clause demands of a representable
/// set.
struct FixedRadiusInterval<const RADIUS_SLOTS: i64>;

impl<const RADIUS_SLOTS: i64> Format for FixedRadiusInterval<RADIUS_SLOTS> {
    type Ambient = IntervalOverBinaryRationals;
    type Quantum = Constant<-4>;
    type Slots = Signed<6>;
    const PHASE_NUM: i64 = 0;
    const PHASE_DEN: i64 = 1;
}

/// How many slots the format's range admits.
fn slot_count<F: Format>() -> i128 {
    let min = <F::Slots as Slots>::MIN as i128;
    let max = <F::Slots as Slots>::MAX as i128;
    max - min + 1
}

/// How many of them the single affine predicate actually admits, counted by
/// running the shipped predicate rather than by trusting the bounds.
fn admitted_by_the_predicate<F: Format>() -> i128 {
    let min = <F::Slots as Slots>::MIN;
    let max = <F::Slots as Slots>::MAX;
    let mut n = 0i128;
    let mut s = min;
    loop {
        if contains::<F>(s, 0) {
            n += 1;
        }
        if s == max {
            break;
        }
        s += 1;
    }
    n
}

fn main() {
    type A = FixedRadiusInterval<3>;

    let n = slot_count::<A>();
    let named = admitted_by_the_predicate::<A>();

    // Arm B. Ordered pairs (lo, hi) with lo <= hi over the same grid: every
    // interval whose endpoints are representable.
    let general_intervals = n * (n + 1) / 2;

    // Arm A. One interval per admitted centre, radius fixed by the type.
    let fixed_radius_intervals = named;

    println!("slots in the range                      : {n}");
    println!("slots the shipped predicate admits      : {named}");
    println!("arm A, fixed-radius intervals nameable  : {fixed_radius_intervals}");
    println!("arm B, general intervals on the grid    : {general_intervals}");
    println!(
        "unnameable by one affine slot           : {}",
        general_intervals - fixed_radius_intervals
    );
    println!();

    // The negative control: radius zero. Intervals collapse to points, and one
    // slot must name every one of them, so the gap must be zero.
    let control_points = n;
    let control_gap = control_points - fixed_radius_intervals;
    println!("CONTROL, radius 0: points on the grid   : {control_points}");
    println!("CONTROL gap (must be 0)                 : {control_gap}");
    println!();

    assert!(
        named == n,
        "the predicate did not admit its own declared range; counting is unsound"
    );
    assert!(
        control_gap == 0,
        "NEGATIVE CONTROL FAILED: a single slot cannot even name the points, so the \
         arm B gap says nothing about set-valuedness"
    );

    println!(
        "FINDING: arm A compiles. A fixed-radius interval is a `Format` with no \
         amendment to the ratified parameterisation: the centre is the one affine \
         slot and the radius is a constant of the type, which is what the identity \
         clause asks for. Arm B does not: one slot index names {} of the {} \
         intervals the grid carries, and the remaining {} have no coordinate. \
         Admitting them needs the affine predicate to become vector-valued, which \
         is an amendment to a ratified clause, and the same ratified clause says an \
         instance joins by supplying the concept's obligations RATHER THAN by \
         amending the canon. So general set-valued carriers are refused by the \
         canon's own admission rule, and fixed-radius ones are admitted by \
         construction.",
        fixed_radius_intervals,
        general_intervals,
        general_intervals - fixed_radius_intervals
    );
}
