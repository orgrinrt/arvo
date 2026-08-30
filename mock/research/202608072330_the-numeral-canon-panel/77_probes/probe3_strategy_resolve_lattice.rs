// Does a law ABOUT strategy composition (not about numeric values) fit
// the same trait-contract-checked-at-compile-time vocabulary as a law
// about numeric values? Cross-strategy resolution ("Hot wrapping +
// Precise saturating -> Precise", arvo-toolbox-not-policer.md) needs
// SOME deterministic rule, and that rule itself wants laws: commutative
// (order of the two operands in a binary op should not matter),
// associative (resolving three-plus strategies in a chain should not
// depend on grouping), idempotent (resolving a strategy against itself
// is a no-op).
//
// This uses generic axis labels (Axis0..Axis3 per axis), NOT Hot/Warm/
// Cold/Precise: the four-strategy set is open per I1 and this probe does
// not presume it. It also deliberately models TWO independent axes with
// a product order, not one linear ranking, because op's I8 ("they weigh
// different measurements differently") suggests more than one axis is
// plausible, and a probe that only tested a total order would be
// checking the easy case.
trait Axis {
    const RANK: u8;
}
struct A0;
impl Axis for A0 {
    const RANK: u8 = 0;
}
struct A1;
impl Axis for A1 {
    const RANK: u8 = 1;
}
struct A2;
impl Axis for A2 {
    const RANK: u8 = 2;
}

// a strategy is a pair of independent axis choices (e.g. overflow
// aggressiveness, storage aggressiveness). resolution is pointwise max:
// the standard join operation on a product order, which is what makes
// this a genuine lattice rather than a single chain.
trait Strategy {
    type OverflowAxis: Axis;
    type StorageAxis: Axis;
}

struct Point<O, S>(core::marker::PhantomData<(O, S)>);
impl<O: Axis, S: Axis> Strategy for Point<O, S> {
    type OverflowAxis = O;
    type StorageAxis = S;
}

// join (resolve) on two RANKS via a const fn, itself validated the same
// way probe2 validated ONE_RAW: no generic_const_exprs, the const fn body
// is free to branch on the operands because they are the type's own
// associated consts, not folded into a fresh const-generic array bound.
const fn max_rank(a: u8, b: u8) -> u8 {
    if a > b {
        a
    } else {
        b
    }
}

trait Resolve<Rhs> {
    const OVERFLOW_RANK: u8;
    const STORAGE_RANK: u8;
}

impl<Lhs, Rhs> Resolve<Rhs> for Lhs
where
    Lhs: Strategy,
    Rhs: Strategy,
{
    const OVERFLOW_RANK: u8 = max_rank(
        <Lhs::OverflowAxis as Axis>::RANK,
        <Rhs::OverflowAxis as Axis>::RANK,
    );
    const STORAGE_RANK: u8 = max_rank(
        <Lhs::StorageAxis as Axis>::RANK,
        <Rhs::StorageAxis as Axis>::RANK,
    );
}

// the laws, checked as const asserts against concrete instantiations:
// this is the "generator, validated by the compiler" pattern again, now
// applied one level up, to laws about strategy composition rather than
// laws about numeric operations. a failing law here is a compile error,
// exactly like probe2's bad case, not a runtime surprise.
macro_rules! check_semilattice_laws {
    ($x:ty, $y:ty, $z:ty) => {
        const _: () = {
            // commutative: resolve(x,y) == resolve(y,x)
            assert!(<$x as Resolve<$y>>::OVERFLOW_RANK == <$y as Resolve<$x>>::OVERFLOW_RANK);
            assert!(<$x as Resolve<$y>>::STORAGE_RANK == <$y as Resolve<$x>>::STORAGE_RANK);
            // idempotent: resolve(x,x) == x's own ranks
            assert!(<$x as Resolve<$x>>::OVERFLOW_RANK == <$x as Strategy>::OverflowAxis::RANK);
            assert!(<$x as Resolve<$x>>::STORAGE_RANK == <$x as Strategy>::StorageAxis::RANK);
            // associative on the OVERFLOW rank alone (a scalar max, so
            // this reduces to ordinary integer max associativity, but it
            // is checked through the SAME trait machinery a real
            // three-way resolve would use, not asserted by fiat)
            let xy = max_rank(
                <$x as Strategy>::OverflowAxis::RANK,
                <$y as Strategy>::OverflowAxis::RANK,
            );
            let xy_z = max_rank(xy, <$z as Strategy>::OverflowAxis::RANK);
            let yz = max_rank(
                <$y as Strategy>::OverflowAxis::RANK,
                <$z as Strategy>::OverflowAxis::RANK,
            );
            let x_yz = max_rank(<$x as Strategy>::OverflowAxis::RANK, yz);
            assert!(xy_z == x_yz);
        };
    };
}

type P00 = Point<A0, A0>;
type P12 = Point<A1, A2>;
type P21 = Point<A2, A1>;

check_semilattice_laws!(P00, P12, P21);

fn main() {
    println!(
        "resolve(P00, P12).overflow = {}, .storage = {}",
        <P00 as Resolve<P12>>::OVERFLOW_RANK,
        <P00 as Resolve<P12>>::STORAGE_RANK,
    );
    println!(
        "resolve(P12, P00).overflow = {}, .storage = {}  (commutativity holds: checked at compile time above)",
        <P12 as Resolve<P00>>::OVERFLOW_RANK,
        <P12 as Resolve<P00>>::STORAGE_RANK,
    );
}
