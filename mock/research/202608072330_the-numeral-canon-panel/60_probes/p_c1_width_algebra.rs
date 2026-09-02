// Probe C1: the exact-op width algebra is statable as trait contracts under the
// feature ban, and the trait solver chains it.
//
// Hypothesis: a chain claim needs the format concept to name the format an
// op's EXACT result lands in (for multiply: integer widths add, fraction
// widths add), plus an explicit adaptation back into a boundary format. Both
// are expressible on the pinned nightly with NO forbidden features: no
// generic_const_exprs, no specialization, no dyn, no TypeId. The general
// impl-for-all-widths spelling is refused (see p_c2, a deliberate compile
// failure); the accepted shape is a bounded enumeration of exact-mul impls,
// which is not a workaround but the same boundedness the window derivation
// arrives at independently: exact intermediates are bounded by the container,
// so the set of windows is finite.
//
// The chain (2,6) * (3,5) * (1,7) type-derives to Q<6,18> through the solver,
// is verified by a function that ONLY accepts Q<6,18> (a type-level assertion:
// if the derived type were anything else this probe would not compile), and
// adapts back to Q<2,6> with a value check against a hand-computed constant.
//
// Shortcuts (spike): non-negative raws only (the RNE narrow here is not
// audited for negative operands), i128 carrier for every width, three
// hand-picked width triples. None bear on the statability claim.

#![allow(dead_code)]

#[derive(Clone, Copy, PartialEq, Debug)]
struct Q<const I: u32, const F: u32>(i128);

trait ExactMul<R> {
    type Out;
    fn emul(self, r: R) -> Self::Out;
}

macro_rules! exact_mul {
    ($(($i1:literal,$f1:literal) x ($i2:literal,$f2:literal) => ($io:literal,$fo:literal));* $(;)?) => {$(
        impl ExactMul<Q<$i2,$f2>> for Q<$i1,$f1> {
            type Out = Q<$io,$fo>;
            // exact: integer multiply, no adaptation. The output FORMAT is the
            // load-bearing part: I and F both add.
            fn emul(self, r: Q<$i2,$f2>) -> Q<$io,$fo> { Q(self.0 * r.0) }
        }
    )*};
}

exact_mul! {
    (2,6) x (3,5) => (5,11);
    (5,11) x (1,7) => (6,18);
}

// Explicit, first-class adaptation: RNE on the fractional side. Const
// arithmetic on width parameters is legal in a fn body; only type position is
// banned, and Adapt needs none there.
trait Adapt<T> {
    fn adapt(self) -> T;
}

impl<const I1: u32, const F1: u32, const I2: u32, const F2: u32> Adapt<Q<I2, F2>> for Q<I1, F1> {
    fn adapt(self) -> Q<I2, F2> {
        if F1 >= F2 {
            let sh = F1 - F2;
            if sh == 0 {
                return Q(self.0);
            }
            let half = 1i128 << (sh - 1);
            let q = self.0 >> sh;
            let rem = self.0 & ((1i128 << sh) - 1);
            Q(if rem > half || (rem == half && (q & 1) == 1) {
                q + 1
            } else {
                q
            })
        } else {
            Q(self.0 << (F2 - F1))
        }
    }
}

// type-level assertion: only compiles if the chained exact type is Q<6,18>
fn assert_is_q_6_18(x: Q<6, 18>) -> Q<6, 18> {
    x
}

fn main() {
    let x: Q<2, 6> = Q(96); // 1.5
    let y: Q<3, 5> = Q(72); // 2.25
    let z: Q<1, 7> = Q(64); // 0.5

    let chained = x.emul(y).emul(z);
    let chained = assert_is_q_6_18(chained);
    // 1.5 * 2.25 * 0.5 = 1.6875 exactly, at F=18: 1.6875 * 2^18 = 442368
    assert_eq!(chained.0, 442_368);

    let back: Q<2, 6> = chained.adapt();
    // 1.6875 at F=6: 108, exact (no rounding needed on this value)
    assert_eq!(back.0, 108);

    // a value where the adapt genuinely rounds: raw 442369 at F=18 is
    // 1.687503814..., nearest F=6 value is still 108
    let dirty: Q<6, 18> = Q(442_369);
    let back2: Q<2, 6> = dirty.adapt();
    assert_eq!(back2.0, 108);

    // and a tie: raw at F=18 exactly halfway between F=6 neighbours 108 and
    // 109 is 108.5/2^6 scale: (108*4096 + 2048) = 444416; RNE goes to even 108
    let tie: Q<6, 18> = Q(444_416);
    let back3: Q<2, 6> = tie.adapt();
    assert_eq!(back3.0, 108, "tie must go to even");

    println!(
        "chained exact type derived by solver: Q<6,18>, raw {}",
        chained.0
    );
    println!("adapt back to Q<2,6>: {}", back.0);
    println!("OUTCOME: WORKS");
}
