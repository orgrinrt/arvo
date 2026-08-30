// Probe 4. Face, encoding, value: which layer a fact is keyed on, and what
// forces it.
//
// The design now has three layers where a numeral component can be named:
//
//   face      `numeral_face!(Third = 1/3)` mints a type. Per invocation.
//   encoding  `BPos<H, I<H>>`. Sealed, value-unique (`63:174-175`, ratified at
//             44b). Structural, global.
//   value     the rational 1/3. Mathematical.
//
// Value-uniqueness is what makes the encoding a legitimate proxy for the value:
// one type per value, so type equality decides value equality. That leaves TWO
// identity notions, not three, and the keying question becomes binary: does a
// fact depend on WHERE it was written, or only on WHAT it denotes?
//
// This probe checks four things:
//   4.1 two faces for the same literal project to the SAME encoding type;
//   4.2 a fact keyed on the encoding cannot distinguish them, structurally;
//   4.3 a fact keyed on the face can, and refuses the mixture (E0308);
//   4.4 a face cannot reach a numeral position at all, because the seal forces
//       the projection, which is the mechanism that makes 4.2 unavoidable
//       rather than merely conventional.
//
// 4.4 is the finding: the seal that file 61 showed forces a bridge trait is the
// same mechanism that guarantees face identity cannot leak into a type-level
// fact. The design did not have to be arranged for this. It falls out.

#![allow(dead_code)]

// ---------------------------------------------------------------------------
// The sealed encoding carrier. `sealed::Sealed` stands in for the private
// supertrait file 61 found unreachable across a proc-macro crate boundary.
// ---------------------------------------------------------------------------
mod tower {
    pub(crate) mod sealed {
        pub trait Sealed {}
    }

    /// Type-level positive magnitude: H = 1, O<P> = 2P, I<P> = 2P+1.
    pub struct H;
    pub struct O<P>(core::marker::PhantomData<P>);
    pub struct I<P>(core::marker::PhantomData<P>);

    pub trait Pos: sealed::Sealed {
        const VAL: u64;
    }
    impl sealed::Sealed for H {}
    impl Pos for H {
        const VAL: u64 = 1;
    }
    impl<P: Pos> sealed::Sealed for O<P> {}
    impl<P: Pos> Pos for O<P> {
        const VAL: u64 = 2 * P::VAL;
    }
    impl<P: Pos> sealed::Sealed for I<P> {}
    impl<P: Pos> Pos for I<P> {
        const VAL: u64 = 2 * P::VAL + 1;
    }

    /// The signed rational bias. Sealed. Value-unique: the only way to build one
    /// is with an already-reduced numerator and denominator, so two types are
    /// equal exactly when the rationals they denote are.
    pub struct BZero;
    pub struct BPos<N, D>(core::marker::PhantomData<(N, D)>);
    pub struct BNeg<N, D>(core::marker::PhantomData<(N, D)>);

    pub trait Bias: sealed::Sealed {
        const NUM: i64;
        const DEN: u64;
    }
    impl sealed::Sealed for BZero {}
    impl Bias for BZero {
        const NUM: i64 = 0;
        const DEN: u64 = 1;
    }
    impl<N: Pos, D: Pos> sealed::Sealed for BPos<N, D> {}
    impl<N: Pos, D: Pos> Bias for BPos<N, D> {
        const NUM: i64 = N::VAL as i64;
        const DEN: u64 = D::VAL;
    }
    impl<N: Pos, D: Pos> sealed::Sealed for BNeg<N, D> {}
    impl<N: Pos, D: Pos> Bias for BNeg<N, D> {
        const NUM: i64 = -(N::VAL as i64);
        const DEN: u64 = D::VAL;
    }

    /// The bridge. Deliberately UNSEALED: an unbounded, per-literal vocabulary
    /// rather than a closed carrier (`63:377-381`).
    pub trait NumeralFace {
        type Encoding: Bias;
        const DISPLAY: &'static str;
    }

    /// A numeral position. Note what it takes: `B: Bias`, the SEALED carrier.
    /// A face cannot go here; see 4.4.
    pub struct Implicit<B: Bias>(core::marker::PhantomData<B>);
    impl<B: Bias> Implicit<B> {
        pub fn new() -> Self {
            Implicit(core::marker::PhantomData)
        }
    }

    /// The projection every consumer of a face must go through.
    pub type Enc<F> = <F as NumeralFace>::Encoding;
}

use tower::*;

// ---------------------------------------------------------------------------
// What the macro emits. Two invocations, same literal, different sites.
// Reduction is done host-side, so both emit the identical reduced encoding.
// ---------------------------------------------------------------------------
mod site_a {
    use super::tower::*;
    pub struct Third; // numeral_face!(Third = 1/3)
    impl NumeralFace for Third {
        type Encoding = BPos<H, I<H>>; // 1/3
        const DISPLAY: &'static str = "1/3";
    }
}

mod site_b {
    use super::tower::*;
    pub struct OneThird; // numeral_face!(OneThird = 2/6), reduced host-side
    impl NumeralFace for OneThird {
        type Encoding = BPos<H, I<H>>; // 1/3, same reduced encoding
        const DISPLAY: &'static str = "2/6";
    }
}

mod site_c {
    use super::tower::*;
    pub struct Half; // numeral_face!(Half = 1/2)
    impl NumeralFace for Half {
        type Encoding = BPos<H, O<H>>; // 1/2
        const DISPLAY: &'static str = "1/2";
    }
}

use site_a::Third;
use site_b::OneThird;
use site_c::Half;

// ---------------------------------------------------------------------------
// 4.1 / 4.2. A fact keyed on the ENCODING. One type parameter, used twice: the
// two arguments must be the same numeral. Two distinct faces reach it.
// ---------------------------------------------------------------------------
fn law_keyed_on_encoding<B: Bias>(_x: Implicit<B>, _y: Implicit<B>) -> (i64, u64) {
    (B::NUM, B::DEN)
}

fn mk<F: NumeralFace>() -> Implicit<Enc<F>> {
    Implicit::new()
}

// ---------------------------------------------------------------------------
// 4.3. A fact keyed on the FACE. Same shape, one parameter used twice, but the
// parameter is the face rather than its projection.
// ---------------------------------------------------------------------------
struct Tagged<F: NumeralFace>(core::marker::PhantomData<F>);
fn keyed_on_face<F: NumeralFace>(_x: Tagged<F>, _y: Tagged<F>) -> &'static str {
    F::DISPLAY
}

// ---------------------------------------------------------------------------
// 4.2, structurally: no function of the encoding can observe the face, because
// the projection is a function and nothing recovers its argument. Stated as a
// type: this compiles for any face, and its result type mentions no face.
// ---------------------------------------------------------------------------
fn erases<F: NumeralFace>() -> (i64, u64) {
    (<Enc<F> as Bias>::NUM, <Enc<F> as Bias>::DEN)
}

// Probe 5c. A face placed directly in a numeral position.
//
// `Implicit<B: Bias>` takes the SEALED carrier. A macro-minted face implements
// `NumeralFace`, never `Bias`, because the seal's private supertrait is
// unreachable from the expansion crate (file 61's structural necessity).
//
// So a face cannot reach a numeral position at all, and the projection through
// `NumeralFace::Encoding` is the only route. That projection is a function, so
// nothing downstream of it can recover which face it came from. The seal that
// forced the bridge is the same mechanism that guarantees face identity never
// leaks into a type-level fact. It falls out; nobody arranged it.
//
// EXPECTED TO FAIL TO COMPILE.
type Wrong = Implicit<Third>;

fn main() {
    let _: Option<Wrong> = None;
}
