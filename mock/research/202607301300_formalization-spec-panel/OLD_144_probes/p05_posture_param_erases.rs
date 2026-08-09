// P5. Does the posture PARAMETER survive lowering? Existence check only, not a
// measurement: if the two arms fold to one symbol, the parameter is gone.
#![crate_type = "lib"]
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
pub trait Posture {
    type Of<T>;
    fn wrap<T>(v: T) -> Self::Of<T>;
}
impl Posture for Hot {
    type Of<T> = T;
    #[inline]
    fn wrap<T>(v: T) -> T {
        v
    }
}
impl Posture for Warm {
    type Of<T> = T;
    #[inline]
    fn wrap<T>(v: T) -> T {
        v
    }
}
#[inline]
pub fn add<P: Posture, const N: u8, S>(a: Num<N, S>, b: Num<N, S>) -> P::Of<Num<N, S>> {
    P::wrap(Num::new(a.raw().wrapping_add(b.raw())))
}
pub type StrHandle = Num<5, Warm>;
#[no_mangle]
pub fn ambient(a: StrHandle, b: StrHandle) -> StrHandle {
    add::<Warm, 5, Warm>(a, b)
}
#[no_mangle]
pub fn hot_scope(a: StrHandle, b: StrHandle) -> StrHandle {
    add::<Hot, 5, Warm>(a, b)
}
#[no_mangle]
pub fn bare(a: u32, b: u32) -> u32 {
    a.wrapping_add(b)
}
