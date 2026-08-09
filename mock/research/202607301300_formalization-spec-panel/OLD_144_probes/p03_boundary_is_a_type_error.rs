// P3. If the injected use retargets the body but not the signature, does a
// retargeted value silently cross the boundary, or is it refused?
// EXPECT: refused. The split cannot leak a wrong-posture value.
#![allow(dead_code, unused_imports)]
pub struct Hot;
pub struct Warm;
pub struct Num<const N: u8, S>(core::marker::PhantomData<S>);
pub mod posture {
    pub mod warm {
        pub type UInt<const N: u8> = super::super::Num<N, super::super::Warm>;
    }
    pub mod hot {
        pub type UInt<const N: u8> = super::super::Num<N, super::super::Hot>;
    }
}
use posture::warm::UInt;

fn leaks() -> UInt<5> {
    use crate::posture::hot::*;
    let x: UInt<5> = Num(core::marker::PhantomData);
    x
}
fn main() {
    let _ = leaks();
}
