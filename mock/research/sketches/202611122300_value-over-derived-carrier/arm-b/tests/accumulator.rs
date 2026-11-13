//! The exact accumulator on arm B.
//!
//! `narrow` of an accumulated sum equals `adapt` of the exact sum in every order;
//! an accumulator that adapts at each step does not, at the range edge under
//! `Saturate`; the term bound per ladder carrier, and what one term past it does.

use arm_b::{Num, Wide};
use common::__private::arvo_format::overflow::{Saturate, Wrap};
use common::__private::arvo_format::points::{Integer, UFixed};
use common::__private::arvo_format::rounding::TowardZero;
use common::__private::arvo_format::{Adapt, Signature};
use common::oracle::oracle_adapt;
use common::{Machine, Sole};

type Sig<F, O> = Signature<F, Adapt<TowardZero, O>>;
type I8W = Sig<Integer<8>, Wrap>;
type I8S = Sig<Integer<8>, Saturate>;
type U8W = Sig<UFixed<8, 0>, Wrap>;
type U8S = Sig<UFixed<8, 0>, Saturate>;

const SIGNED_8: [i128; 12] = [-128, -127, -64, -2, -1, 0, 1, 2, 63, 64, 126, 127];
const UNSIGNED_8: [i128; 12] = [0, 1, 2, 63, 64, 127, 128, 129, 191, 253, 254, 255];

/// Every order of four positions.
fn orders() -> impl Iterator<Item = [usize; 4]> {
    (0 .. 4usize).flat_map(|a| {
        (0 .. 4usize).flat_map(move |b| {
            (0 .. 4usize).flat_map(move |c| {
                (0 .. 4usize).filter_map(move |d| {
                    let o = [a, b, c, d];
                    let distinct = (0 .. 4).all(|i| (0 .. 4).all(|j| i == j || o[i] != o[j]));
                    distinct.then_some(o)
                })
            })
        })
    })
}

fn member<S: Sole>(i: i128) -> Num<S> {
    Num::<S>::from_raw(S::Repr::from_index(i))
}

/// Over every 4-tuple of the set and every order: how many times `narrow` of the
/// accumulated sum differs from `adapt` of the exact sum, and how many times an
/// accumulator that adapts at each step does.
fn sweep<S: Sole>(set: &[i128]) -> (usize, usize, usize) {
    let (mut narrow_wrong, mut stepwise_wrong, mut cases) = (0, 0, 0);
    for &a in set {
        for &b in set {
            for &c in set {
                for &d in set {
                    let terms = [a, b, c, d];
                    let want = oracle_adapt::<S>(a + b + c + d);
                    for o in orders() {
                        cases += 1;
                        let mut acc = Wide::<S>::ZERO;
                        let mut step = member::<S>(0);
                        for &i in &o {
                            acc = acc.add(member::<S>(terms[i]).widen());
                            step = step.add(member::<S>(terms[i]));
                        }
                        if acc.narrow().raw().index() != want {
                            narrow_wrong += 1;
                        }
                        if step.raw().index() != want {
                            stepwise_wrong += 1;
                        }
                    }
                }
            }
        }
    }
    (narrow_wrong, stepwise_wrong, cases)
}

#[test]
fn narrow_of_four_is_adapt_of_the_exact_sum_in_every_order_at_width_8() {
    let cases = 12usize.pow(4) * 24;
    let i8w = sweep::<I8W>(&SIGNED_8);
    let i8s = sweep::<I8S>(&SIGNED_8);
    let u8w = sweep::<U8W>(&UNSIGNED_8);
    let u8s = sweep::<U8S>(&UNSIGNED_8);
    for (narrow_wrong, _, n) in [i8w, i8s, u8w, u8s] {
        assert_eq!(n, cases);
        assert_eq!(narrow_wrong, 0);
    }
    // Wrapping is a homomorphism onto the residues, so adapting at each step
    // loses nothing. Unsigned members are never negative, so a saturating
    // partial sum only climbs and `min(MAX, sum)` is order-free: it loses
    // nothing either. Signed saturation is where the stepwise accumulator fails.
    assert_eq!(i8w.1, 0);
    assert_eq!(u8w.1, 0);
    assert_eq!(u8s.1, 0);
    assert!(
        i8s.1 > 0,
        "stepwise signed saturation should fail somewhere at the edge"
    );
    std::println!(
        "stepwise signed saturating accumulator wrong in {} of {cases}",
        i8s.1
    );
}

#[test]
fn stepwise_saturation_fails_at_the_edge_and_depends_on_order() {
    let terms = [127, 1, -1, -127];
    let exact = oracle_adapt::<I8S>(0);
    let mut acc = Wide::<I8S>::ZERO;
    let mut step = member::<I8S>(0);
    for t in terms {
        acc = acc.add(member::<I8S>(t).widen());
        step = step.add(member::<I8S>(t));
    }
    assert_eq!(acc.narrow().raw().index(), exact);
    // 127 + 1 saturates at 127, and the -1 that would have brought it back
    // lands on 126: the stepwise answer is -1, the exact one 0.
    assert_eq!(step.raw(), -1);
    let mut reordered = member::<I8S>(0);
    for t in [-1, 127, 1, -127] {
        reordered = reordered.add(member::<I8S>(t));
    }
    assert_eq!(reordered.raw(), 0);
}

#[test]
fn the_term_bound_per_ladder_carrier_at_full_width() {
    assert_eq!(Wide::<Sig<Integer<8>, Wrap>>::MAX_TERMS, 1 << 8);
    assert_eq!(Wide::<Sig<UFixed<8, 0>, Wrap>>::MAX_TERMS, 1 << 7);
    assert_eq!(Wide::<Sig<Integer<16>, Wrap>>::MAX_TERMS, 1 << 16);
    assert_eq!(Wide::<Sig<UFixed<16, 0>, Wrap>>::MAX_TERMS, 1 << 15);
    assert_eq!(Wide::<Sig<Integer<32>, Wrap>>::MAX_TERMS, 1 << 32);
    assert_eq!(Wide::<Sig<UFixed<32, 0>, Wrap>>::MAX_TERMS, 1 << 31);
    assert_eq!(Wide::<Sig<Integer<64>, Wrap>>::MAX_TERMS, 1 << 64);
    assert_eq!(Wide::<Sig<UFixed<64, 0>, Wrap>>::MAX_TERMS, 1 << 63);
    // The narrowest width a carrier holds: one bit, where the bound is widest.
    assert_eq!(Wide::<Sig<Integer<1>, Wrap>>::MAX_TERMS, 1 << 15);
    assert_eq!(Wide::<Sig<UFixed<1, 0>, Wrap>>::MAX_TERMS, (1 << 15) - 1);
    assert_eq!(Wide::<Sig<Integer<33>, Wrap>>::MAX_TERMS, 1 << 95);
}

/// `n` copies of `m`, accumulated.
fn copies<S: Sole>(m: i128, n: u128) -> Wide<S> {
    let mut acc = Wide::<S>::ZERO;
    for _ in 0 .. n {
        acc = acc.add(member::<S>(m).widen());
    }
    acc
}

#[test]
fn at_the_bound_the_sum_is_exact() {
    let n = Wide::<I8S>::MAX_TERMS;
    assert_eq!(copies::<I8S>(-128, n).raw(), -32768);
    assert_eq!(
        copies::<I8S>(-128, n).narrow().raw().index(),
        oracle_adapt::<I8S>(-32768)
    );
    let n = Wide::<U8S>::MAX_TERMS;
    assert_eq!(copies::<U8S>(255, n).raw(), 32640);
    assert_eq!(
        copies::<U8S>(255, n).narrow().raw().index(),
        oracle_adapt::<U8S>(32640)
    );
}

#[test]
#[cfg(debug_assertions)]
#[should_panic(expected = "attempt to add with overflow")]
fn one_term_past_the_bound_panics_with_overflow_checks() {
    let _ = copies::<I8S>(-128, Wide::<I8S>::MAX_TERMS + 1);
}

#[test]
#[cfg(not(debug_assertions))]
fn one_term_past_the_bound_wraps_without_overflow_checks() {
    // -32768 - 128 wraps to 32640 in the register, and the saturating narrow
    // returns the top of the range where the exact sum is below the bottom.
    let past = copies::<I8S>(-128, Wide::<I8S>::MAX_TERMS + 1);
    assert_eq!(past.raw(), 32640);
    assert_eq!(past.narrow().raw(), 127);
    assert_eq!(oracle_adapt::<I8S>(-32768 - 128), -128);
}
