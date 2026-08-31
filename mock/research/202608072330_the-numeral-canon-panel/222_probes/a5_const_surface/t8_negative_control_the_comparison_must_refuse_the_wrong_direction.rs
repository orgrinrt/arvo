// t8. The case that must fail. t7 shows the comparison accepts a legal narrowing; if it also
// accepts the illegal one it is not a comparison, it is an unconstrained bound. Narrowing a
// 1-wide value into a 3-wide output must be refused.
trait Nat {
    const USIZE: usize;
}
struct Z;
struct S<N>(core::marker::PhantomData<N>);
impl Nat for Z {
    const USIZE: usize = 0;
}
impl<N: Nat> Nat for S<N> {
    const USIZE: usize = N::USIZE + 1;
}

trait AtLeast<Other> {}
impl<N: Nat> AtLeast<Z> for N {}
impl<A: Nat, B: Nat> AtLeast<S<B>> for S<A> where A: AtLeast<B> {}

type N1 = S<Z>;
type N2 = S<N1>;
type N3 = S<N2>;

fn narrow_into<Out: Nat, In: Nat>(_: In) -> Out
where
    In: AtLeast<Out>,
    Out: Default,
{
    Out::default()
}

impl Default for Z {
    fn default() -> Z {
        Z
    }
}
impl<N: Default> Default for S<N> {
    fn default() -> S<N> {
        S(core::marker::PhantomData)
    }
}

fn main() {
    let _bad: N3 = narrow_into::<N3, N1>(S(core::marker::PhantomData));
}
