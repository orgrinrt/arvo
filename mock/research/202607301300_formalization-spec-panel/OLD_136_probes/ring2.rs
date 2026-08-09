#![allow(dead_code)]
pub struct End;
pub struct N0<T>(T);
pub struct N1<T>(T);
pub struct Fixed<I>(I);
type A = N1<N0<N1<N1<N0<N1<N0<N1<N1<N0<N1<N0<N0<N1<N1<N1<N0<N1<N0<N1<End>>>>>>>>>>>>>>>>>>>>;
type B = N1<N0<N1<N1<N0<N1<N0<N1<N1<N0<N1<N0<N0<N1<N1<N1<N0<N1<N1<N0<End>>>>>>>>>>>>>>>>>>>>;
fn want(_x: Fixed<A>) {}
fn main() {
    let v: Fixed<B> = Fixed(unsafe { core::mem::zeroed() });
    want(v);
}
