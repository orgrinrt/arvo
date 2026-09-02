// GENERATED from gen.py's SHAPE block plus a projection.

// ---- the shape algebra, in (total width, fraction width) coordinates -------
// A numeral's shape is a pair of nats (W, F). The integer width I = W - F is a
// DERIVED VIEW and is the only coordinate that ever goes negative, which is why
// it is not stored. q01 measures I < 0 at 15 of 6561 product pairs and W < 0 and
// F < 0 at zero of them.
pub struct Shape<W, F>(W, F);

// tight product (q02: exact at 6561 of 6561).
//   W = W1 + W2 - 1 when either total width is 1, else W1 + W2
//   F = F1 + F2
pub trait ProdS<R> {
    type Out;
}
pub type Prod<A, B> = <A as ProdS<B>>::Out;

impl<W1, F1, W2, F2> ProdS<Shape<W2, F2>> for Shape<W1, F1>
where
    W1: Add<W2> + Cmp<N1>,
    F1: Add<F2>,
    W2: Cmp<N1>,
    Sum<W1, W2>: Sub<N1>,
    Ord2<W2, N1>: IfEq<Dif<Sum<W1, W2>, N1>, Sum<W1, W2>>,
    Ord2<W1, N1>:
        IfEq<Dif<Sum<W1, W2>, N1>, <Ord2<W2, N1> as IfEq<Dif<Sum<W1, W2>, N1>, Sum<W1, W2>>>::Out>,
{
    type Out = Shape<
        <Ord2<W1, N1> as IfEq<
            Dif<Sum<W1, W2>, N1>,
            <Ord2<W2, N1> as IfEq<Dif<Sum<W1, W2>, N1>, Sum<W1, W2>>>::Out,
        >>::Out,
        Sum<F1, F2>,
    >;
}

// tight sum (q03: exact at 6561 of 6561).
//   F  = max(F1, F2)
//   d_i = F - F_i          natural, since F is the max
//   A_i = W_i + d_i
//   W  = max(A1, A2) + [ min(A1, A2) > max(d1, d2) ]
pub trait SumS<R> {
    type Out;
}
pub type Plus<A, B> = <A as SumS<B>>::Out;

pub type FOut<F1, F2> = Max<F1, F2>;
pub type D<Fo, Fi> = Dif<Fo, Fi>;
pub type A_<W, Fo, Fi> = Sum<W, D<Fo, Fi>>;

impl<W1, F1, W2, F2> SumS<Shape<W2, F2>> for Shape<W1, F1>
where
    F1: Cmp<F2>,
    Ord2<F1, F2>: IfLe<F2, F1> + IfLe<F1, F2>,
    FOut<F1, F2>: Sub<F1> + Sub<F2>,
    W1: Add<D<FOut<F1, F2>, F1>>,
    W2: Add<D<FOut<F1, F2>, F2>>,
    D<FOut<F1, F2>, F1>: Cmp<D<FOut<F1, F2>, F2>>,
    Ord2<D<FOut<F1, F2>, F1>, D<FOut<F1, F2>, F2>>: IfLe<D<FOut<F1, F2>, F2>, D<FOut<F1, F2>, F1>>,
    A_<W1, FOut<F1, F2>, F1>: Cmp<A_<W2, FOut<F1, F2>, F2>>,
    Ord2<A_<W1, FOut<F1, F2>, F1>, A_<W2, FOut<F1, F2>, F2>>: IfLe<A_<W2, FOut<F1, F2>, F2>, A_<W1, FOut<F1, F2>, F1>>
        + IfLe<A_<W1, FOut<F1, F2>, F1>, A_<W2, FOut<F1, F2>, F2>>,
    Min<A_<W1, FOut<F1, F2>, F1>, A_<W2, FOut<F1, F2>, F2>>:
        Cmp<Max<D<FOut<F1, F2>, F1>, D<FOut<F1, F2>, F2>>>,
    Max<A_<W1, FOut<F1, F2>, F1>, A_<W2, FOut<F1, F2>, F2>>: AddC<Z>,
    Ord2<
        Min<A_<W1, FOut<F1, F2>, F1>, A_<W2, FOut<F1, F2>, F2>>,
        Max<D<FOut<F1, F2>, F1>, D<FOut<F1, F2>, F2>>,
    >: IfGt<
        SumC<Max<A_<W1, FOut<F1, F2>, F1>, A_<W2, FOut<F1, F2>, F2>>, Z>,
        Max<A_<W1, FOut<F1, F2>, F1>, A_<W2, FOut<F1, F2>, F2>>,
    >,
{
    type Out = Shape<
        <Ord2<
            Min<A_<W1, FOut<F1, F2>, F1>, A_<W2, FOut<F1, F2>, F2>>,
            Max<D<FOut<F1, F2>, F1>, D<FOut<F1, F2>, F2>>,
        > as IfGt<
            SumC<Max<A_<W1, FOut<F1, F2>, F1>, A_<W2, FOut<F1, F2>, F2>>, Z>,
            Max<A_<W1, FOut<F1, F2>, F1>, A_<W2, FOut<F1, F2>, F2>>,
        >>::Out,
        FOut<F1, F2>,
    >;
}

// projection out of a Shape, so a numeral can be built from an operation's
// result without the operation having to know about numerals.
pub trait Parts {
    type W;
    type F;
}
impl<W, F> Parts for Shape<W, F> {
    type W = W;
    type F = F;
}
pub type WOf<T> = <T as Parts>::W;
pub type FOf<T> = <T as Parts>::F;
