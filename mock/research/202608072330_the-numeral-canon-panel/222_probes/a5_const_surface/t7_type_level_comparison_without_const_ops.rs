// t7. Option 7's "free type-level comparison": checking a declared output width is wide enough,
// by trait resolution rather than by a const operation. Peano nats keep it short; the point is
// that no const expression appears anywhere.
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
    let _wide: N1 = narrow_into::<N1, N3>(S(core::marker::PhantomData));
    println!("{} {}", N3::USIZE, N1::USIZE);
}
