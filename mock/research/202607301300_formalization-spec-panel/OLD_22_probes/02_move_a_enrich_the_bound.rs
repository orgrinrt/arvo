// Probe 2: Move A, enrich the bound. A marker trait keyed by (Op, ARITY),
// where ARITY is supplied by the caller (the combinator), not read out of
// the composition. This is Dolan's atomic-fact shape
// (14_dolan_which_algebra_is_this.md section "reading two", "treat
// associativity... as its own atomic, independently-derived fact... a
// marker trait keyed on Op") extended with a caller-supplied const
// dimension, which is the extra key Rompf's file 21 measured the design
// actually needs (arity, accumulator). No conditional-impl coherence
// machinery is exercised here; each impl targets a distinct (Self, Op,
// ARITY) tuple by construction, so there is nothing to arbitrate between,
// matching Dolan's point that atomic marker traits are a conjunction, not
// competing evidence.
//
// rustc +nightly-2026-05-28 02_move_a_enrich_the_bound.rs (expect: compiles, runs)
// rustc +nightly-2026-05-28 --cfg fail 02_move_a_enrich_the_bound.rs
//   (expect: E0277, `Number<Fixed3, Saturate>` does not implement
//    `AssociativeAt<Add, 4>`, at the algorithm crate's own call site)

trait Numeral {}
#[derive(Clone, Copy)]
struct Fixed3;
impl Numeral for Fixed3 {}

trait Resolve {}
#[derive(Clone, Copy)]
struct Wrap;
impl Resolve for Wrap {}
#[derive(Clone, Copy)]
struct Saturate;
impl Resolve for Saturate {}

#[derive(Clone, Copy)]
struct Number<N: Numeral, S: Resolve>(core::marker::PhantomData<(N, S)>);
impl<N: Numeral, S: Resolve> Number<N, S> {
    fn new() -> Self {
        Number(core::marker::PhantomData)
    }
}

struct Add;

// the marker trait a caller can bound on: "this composition's Add is
// (Kleene-)associative at fold arity ARITY".
trait AssociativeAt<Op, const ARITY: usize> {}

// Wrap is associative at every arity (stands in for the design's real
// derivation: wrapping addition is the operation of a cyclic group, so it
// folds regardless of accumulator width).
impl<N: Numeral, const ARITY: usize> AssociativeAt<Add, ARITY> for Number<N, Wrap> {}

// Saturate is associative only up to interior safety; here a stand-in
// single point (arity 1) for Rompf's real closed form (K = n - 1 on the
// accumulator, `21_probes/02`).
impl<N: Numeral> AssociativeAt<Add, 1> for Number<N, Saturate> {}

// the algorithm crate: bound on the marker trait, at an arity the
// combinator itself supplies (it is doing the regrouping, so it already
// knows how many-way it is splitting). Never reads S; never reads N.
fn fold_quad<F: Copy, const ARITY: usize>(x: F, _n: F) -> F
where
    F: AssociativeAt<Add, ARITY>,
{
    x // stand-in body: a real four-way-split accumulator would go here
}

fn main() {
    let w = Number::<Fixed3, Wrap>::new();
    let _ = fold_quad::<_, 4>(w, w); // Wrap holds at every arity: compiles

    #[cfg(fail)]
    {
        let s = Number::<Fixed3, Saturate>::new();
        let _ = fold_quad::<_, 4>(s, s); // Saturate at arity 4: refused
    }

    #[cfg(not(fail))]
    {
        let s = Number::<Fixed3, Saturate>::new();
        let _ = fold_quad::<_, 1>(s, s); // Saturate at arity 1: compiles
    }

    println!("fold_quad reached the fact through a bound, keyed by a caller-supplied arity");
}
