// P11. Scope check on p10: the fold there shows the APPARATUS erases, not that
// the postures are indistinguishable. Give two postures different arithmetic
// and confirm they do NOT fold, so p10's aliasing is not measuring sameness.
#![crate_type = "lib"]
#[derive(Copy, Clone)]
pub struct Ambient;
#[derive(Copy, Clone)]
pub struct Hot;
#[derive(Copy, Clone)]
pub struct Cold;
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Num<const N: u8, S>(u32, core::marker::PhantomData<S>);
impl<const N: u8, S> Num<N, S> {
    #[inline]
    pub const fn new(v: u32) -> Self {
        Num(v, core::marker::PhantomData)
    }
    #[inline]
    pub const fn raw(self) -> u32 {
        self.0
    }
}
pub trait Marker {
    fn go(a: u32, b: u32) -> u32;
}
impl Marker for Hot {
    #[inline]
    fn go(a: u32, b: u32) -> u32 {
        a.wrapping_add(b)
    }
}
impl Marker for Cold {
    #[inline]
    fn go(a: u32, b: u32) -> u32 {
        a.saturating_add(b)
    }
}
pub trait Resolve<P> {
    type Out: Marker;
}
impl<P: Marker> Resolve<P> for Ambient {
    type Out = P;
}
impl<P> Resolve<P> for Hot {
    type Out = Hot;
}
impl<P> Resolve<P> for Cold {
    type Out = Cold;
}
#[inline]
pub fn add<P: Marker, const N: u8, S: Resolve<P>>(a: Num<N, S>, b: Num<N, S>) -> Num<N, S> {
    Num::new(<<S as Resolve<P>>::Out as Marker>::go(a.raw(), b.raw()))
}
pub type H = Num<5, Ambient>;
#[no_mangle]
pub fn under_hot(a: H, b: H) -> H {
    add::<Hot, 5, Ambient>(a, b)
}
#[no_mangle]
pub fn under_cold(a: H, b: H) -> H {
    add::<Cold, 5, Ambient>(a, b)
}
