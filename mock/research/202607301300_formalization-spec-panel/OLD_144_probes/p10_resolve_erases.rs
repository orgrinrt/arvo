// P10. Does the precedence indirection survive lowering? Existence check, not a
// measurement: if the arms fold to one symbol the whole apparatus is gone.
#![crate_type = "lib"]
#[derive(Copy, Clone)]
pub struct Ambient;
#[derive(Copy, Clone)]
pub struct Hot;
#[derive(Copy, Clone)]
pub struct Warm;
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
    #[inline]
    fn go(a: u32, b: u32) -> u32 {
        a.wrapping_add(b)
    }
}
impl Marker for Hot {}
impl Marker for Warm {}
pub trait Resolve<P> {
    type Out: Marker;
}
impl<P: Marker> Resolve<P> for Ambient {
    type Out = P;
}
impl<P> Resolve<P> for Hot {
    type Out = Hot;
}
impl<P> Resolve<P> for Warm {
    type Out = Warm;
}
#[inline]
pub fn add<P: Marker, const N: u8, S: Resolve<P>>(a: Num<N, S>, b: Num<N, S>) -> Num<N, S> {
    Num::new(<<S as Resolve<P>>::Out as Marker>::go(a.raw(), b.raw()))
}
pub type StrHandle = Num<5, Ambient>;
pub type Declared = Num<5, Warm>;
#[no_mangle]
pub fn ambient_scope(a: StrHandle, b: StrHandle) -> StrHandle {
    add::<Warm, 5, Ambient>(a, b)
}
#[no_mangle]
pub fn hot_scope(a: StrHandle, b: StrHandle) -> StrHandle {
    add::<Hot, 5, Ambient>(a, b)
}
#[no_mangle]
pub fn declared_under_hot(a: Declared, b: Declared) -> Declared {
    add::<Hot, 5, Warm>(a, b)
}
#[no_mangle]
pub fn bare(a: u32, b: u32) -> u32 {
    a.wrapping_add(b)
}
