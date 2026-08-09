#![no_std]
#![feature(adt_const_params)]
use core::marker::PhantomData;
pub struct Hot;
pub struct Warm;
pub struct Cold;
pub trait Strategy {}
impl Strategy for Hot {}
impl Strategy for Warm {}
impl Strategy for Cold {}
pub struct Slot<const K: usize>;
pub trait Capacity {
    const VAL: usize;
    type Array<T>: AsRef<[T]> + AsMut<[T]>;
    fn build<T: Copy>(v: T) -> Self::Array<T>;
}
impl<const K: usize> Capacity for Slot<K> {
    const VAL: usize = K;
    type Array<T> = [T; K];
    fn build<T: Copy>(v: T) -> [T; K] {
        [v; K]
    }
}

pub struct UFixed<const I: u16, const F: u16, S>(PhantomData<S>);
pub trait Stored {
    const W: usize;
}
impl<const I: u16, const F: u16, S: Strategy> Stored for UFixed<I, F, S> {
    const W: usize = (I as usize) + (F as usize);
}
pub trait IsZeroW<const F: u16> {}
pub struct FracFlag<const F: u16>;
impl IsZeroW<0> for FracFlag<0> {}
pub trait NonZeroW<const F: u16> {}

// Obligation 2, staged: the predicate is decided at expansion time and carried
// as a sealed type-level witness. One impl, no table. The agreement between the
// witness and the widths is checked at the one door, so a hand-written lie does
// not survive the build (see y_attack.rs).
mod wseal {
    pub trait Sealed {}
}
pub struct OneYes;
pub struct OneNo;
impl wseal::Sealed for OneYes {}
impl wseal::Sealed for OneNo {}
pub trait OneWitness: wseal::Sealed {
    const YES: bool;
}
impl OneWitness for OneYes {
    const YES: bool = true;
}
impl OneWitness for OneNo {
    const YES: bool = false;
}
pub struct Num<const I: u16, const F: u16, W, S>(PhantomData<(W, S)>);
pub trait HasOne {
    fn witness();
}
impl<const I: u16, const F: u16, S: Strategy> HasOne for Num<I, F, OneYes, S> {
    fn witness() {
        const { assert!(I > 0, "one-witness disagrees with the widths") };
    }
}
