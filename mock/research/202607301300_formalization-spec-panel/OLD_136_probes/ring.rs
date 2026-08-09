#![allow(dead_code)]
pub struct End;
pub struct N0<T>(T);
pub struct N1<T>(T);
pub struct N2<T>(T);
pub struct N3<T>(T);
pub struct N4<T>(T);
pub struct N5<T>(T);
pub struct N6<T>(T);
pub struct N7<T>(T);
pub struct N8<T>(T);
pub struct N9<T>(T);
pub struct Fixed<I, F>(I, F);
pub struct Z;

// base two: 16 = 10000, 26 = 11010
type B2_16 = N1<N0<N0<N0<N0<End>>>>>;
type B2_26 = N1<N1<N0<N1<N0<End>>>>>;
// base ten: 16, 26
type B10_16 = N1<N6<End>>;
type B10_26 = N2<N6<End>>;

fn want_b2(_x: Fixed<B2_16, Z>) {}
fn want_b10(_x: Fixed<B10_16, Z>) {}

fn main() {
    let a: Fixed<B2_26, Z> = Fixed(N1(N1(N0(N1(N0(End))))), Z);
    want_b2(a);
    let b: Fixed<B10_26, Z> = Fixed(N2(N6(End)), Z);
    want_b10(b);
}
