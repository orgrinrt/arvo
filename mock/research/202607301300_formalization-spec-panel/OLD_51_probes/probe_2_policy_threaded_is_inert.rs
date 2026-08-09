// Tick 3, shape A: thread a Policy parameter through mul_full's growth
// trait and show the result numeral is identical no matter which Policy
// is named at the call site. Rust does not require a generic parameter
// to be used in a trait impl's body, so this compiles regardless of
// whether the parameter matters; the two const checks at the bottom are
// what shows it does not matter, by requiring the two policy-conditioned
// projections to unify to one type.

#![allow(dead_code)]

pub trait Numeral {
    const P: u32;
}
pub struct N8;
impl Numeral for N8 {
    const P: u32 = 8;
}
pub struct N16;
impl Numeral for N16 {
    const P: u32 = 16;
}

pub struct MulNum<N1, N2>(core::marker::PhantomData<(N1, N2)>);
impl<N1: Numeral, N2: Numeral> Numeral for MulNum<N1, N2> {
    const P: u32 = N1::P + N2::P;
}

pub struct MulFull;

// Growth, threaded with a Policy, does not read it.
pub trait Growth {}
pub struct GrowUnbounded;
impl Growth for GrowUnbounded {}
pub struct GrowBounded;
impl Growth for GrowBounded {}

pub trait Policy {
    type Growth: Growth;
}
pub struct PolicyA;
impl Policy for PolicyA {
    type Growth = GrowUnbounded;
}
pub struct PolicyB;
impl Policy for PolicyB {
    type Growth = GrowBounded;
}

pub trait MulFullGrowthPolicyThreaded<N1: Numeral, N2: Numeral, P: Policy> {
    type Out: Numeral;
}
// mul_full's own closure formula (49:269) names N1 and N2 only. Adding P
// to the trait's parameter list does not give the impl anything new to
// compute with; P appears nowhere on the right-hand side because there
// is nothing for it to change. This is the only correct impl: any
// alternative that DID read P would disagree with the ratified formula
// for at least one instantiation of P, which is exactly what probe 3
// shows is refused.
impl<N1: Numeral, N2: Numeral, P: Policy> MulFullGrowthPolicyThreaded<N1, N2, P> for MulFull {
    type Out = MulNum<N1, N2>;
}

fn assert_same<T>(_a: T, _b: T) {}

fn make_a() -> <MulFull as MulFullGrowthPolicyThreaded<N8, N16, PolicyA>>::Out {
    unreachable!()
}
fn make_b() -> <MulFull as MulFullGrowthPolicyThreaded<N8, N16, PolicyB>>::Out {
    unreachable!()
}

fn check() {
    // if the two projections named different concrete types, this line
    // would refuse to compile (E0308, mismatched types). it compiles:
    // the same call, mul_full, with the same operands, produces the
    // identical result numeral under PolicyA and under PolicyB.
    if false {
        assert_same(make_a(), make_b());
    }
}

const _: () = assert!(
    <<MulFull as MulFullGrowthPolicyThreaded<N8, N16, PolicyA>>::Out as Numeral>::P
        == <<MulFull as MulFullGrowthPolicyThreaded<N8, N16, PolicyB>>::Out as Numeral>::P,
    "growth is unchanged by which Policy the caller names"
);

fn main() {
    check();
    println!(
        "Growth threaded through the trait, unread: Out is identical under PolicyA and PolicyB"
    );
}
