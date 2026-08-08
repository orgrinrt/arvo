// q11. What the diagnostic tag from q10 breaks.
//
// A const-generic pair carried purely for the error message is only worth
// anything if it does not become part of what has to agree. It is a type
// parameter, so it IS part of type identity, and that is the thing to test
// rather than to reason about.
//
// Four checks, each with its own function so the errors are separable:
//   t1  a written alias, and a computed value of the same shape, do they unify
//   t2  the same through a tag-blind shape trait
//   t3  does the tag propagate through a shape-preserving operation
//   t4  does the tag cost anything at runtime
//
// Toolchain: rustc 1.98.0-nightly (57d06900f 2026-05-27), pin nightly-2026-05-28.
// Features: none. Edition 2024.
// Build (t1 expected to fail; that failure is the finding):
//   rustc +nightly-2026-05-28 --edition 2024 --crate-type lib q11_tag_costs.rs \
//     --out-dir build
//   rustc +nightly-2026-05-28 --edition 2024 --cfg no_t1 -O q11_tag_costs.rs \
//     --crate-type bin --out-dir build && ./build/q11_tag_costs

#![allow(dead_code)]

include!("q04_core_inc.rs");
include!("q04_literals_inc.rs");

pub struct L<const K: u32>;
pub trait Lit {
    type N;
}
macro_rules! lits { ($($k:literal => $t:ty),* $(,)?) => { $(
    impl Lit for L<$k> { type N = $t; }
)* } }
lits!(0 => N0, 3 => N3, 4 => N4, 12 => N12, 13 => N13, 16 => N16, 26 => N26, 6 => N6);
pub type NatOf<const K: u32> = <L<K> as Lit>::N;

pub struct Tag<const I: u32, const F: u32>;
pub struct Anon;

pub struct Numeral<W, F, T>(core::marker::PhantomData<(W, F, T)>);
pub type UFixed<const I: u32, const F: u32> = Numeral<Sum<NatOf<I>, NatOf<F>>, NatOf<F>, Tag<I, F>>;

// a product, in the shape 13's arrangement A gives it: the output's widths are
// projections over nats and its tag is dropped, because a tag is a label rather
// than a function of the operands.
pub trait Mul<R> {
    type Out;
}
impl<W1, F1, T1, W2, F2, T2> Mul<Numeral<W2, F2, T2>> for Numeral<W1, F1, T1>
where
    W1: Add<W2>,
    F1: Add<F2>,
{
    type Out = Numeral<Sum<W1, W2>, Sum<F1, F2>, Anon>;
}
pub type Prod<A, B> = <A as Mul<B>>::Out;

type Money = UFixed<13, 3>; // W = 16, F = 3
type Squared = UFixed<26, 6>; // W = 32, F = 6

// -- t1: does a computed product unify with the alias a consumer would write --
// EXPECTED TO FAIL. Prod gives Numeral<N32, N6, Anon>; Squared is
// Numeral<N32, N6, Tag<26, 6>>. Same shape, different type.
#[cfg(not(no_t1))]
fn t1(p: Prod<Money, Money>) -> Squared {
    p
}

// -- t2: the same, through a shape trait that ignores the tag -----------------
#[diagnostic::on_unimplemented(
    message = "this numeral does not have the width the operation expects",
    label = "widths must agree; the `Tag<I, F>` below carries them as written"
)]
pub trait SameShape<R> {}
impl<W, F, T1, T2> SameShape<Numeral<W, F, T2>> for Numeral<W, F, T1> {}

fn t2_accepts<X: SameShape<Squared>>(_: X) {}
fn t2(p: Prod<Money, Money>) {
    t2_accepts(p); // should be accepted: same W and F, different tag
}

// -- t3: does the tag survive a shape-preserving operation -------------------
pub trait Neg {
    type Out;
}
impl<W, F, T> Neg for Numeral<W, F, T> {
    type Out = Numeral<W, F, T>; // shape preserved, so the label is still true
}
fn t3(x: <Money as Neg>::Out) -> Money {
    x
}

// -- t4: does the tag cost anything at runtime -------------------------------
#[repr(transparent)]
pub struct Val<N>(u16, core::marker::PhantomData<N>);

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "C" fn q11_tagged(a: Val<Money>, b: Val<Money>) -> u16 {
    a.0.wrapping_add(b.0)
}
#[inline(never)]
#[unsafe(no_mangle)]
pub extern "C" fn q11_native(a: u16, b: u16) -> u16 {
    a.wrapping_add(b)
}

#[cfg(no_t1)]
fn main() {
    println!(
        "size_of Val<Money> = {}  size_of u16 = {}",
        core::mem::size_of::<Val<Money>>(),
        core::mem::size_of::<u16>()
    );
    println!(
        "align_of Val<Money> = {}  align_of u16 = {}",
        core::mem::align_of::<Val<Money>>(),
        core::mem::align_of::<u16>()
    );
    let x = Val::<Money>(7, core::marker::PhantomData);
    let y = Val::<Money>(35, core::marker::PhantomData);
    println!("q11_tagged(7, 35) = {}", q11_tagged(x, y));
    println!("q11_native(7, 35) = {}", q11_native(7, 35));
}
