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

// Obligation 2, unstaged: `I > 0` compared against a const parameter has no
// expression under the permitted features except an impl table. The table is
// two-dimensional in (I, F) because the predicate the review installed reads
// both, and it must cover the substrate's own dispatch ceiling rather than the
// consumer's current maximum, per arvo-toolbox-not-policer.md's ban on a cap
// below what the substrate dispatches.
pub trait HasOne {}

impl<S: Strategy> HasOne for UFixed<1, 0, S> {}
impl<S: Strategy> HasOne for UFixed<1, 1, S> {}
impl<S: Strategy> HasOne for UFixed<1, 2, S> {}
impl<S: Strategy> HasOne for UFixed<1, 3, S> {}
impl<S: Strategy> HasOne for UFixed<1, 4, S> {}
impl<S: Strategy> HasOne for UFixed<1, 5, S> {}
impl<S: Strategy> HasOne for UFixed<1, 6, S> {}
impl<S: Strategy> HasOne for UFixed<1, 7, S> {}
impl<S: Strategy> HasOne for UFixed<1, 8, S> {}
impl<S: Strategy> HasOne for UFixed<1, 9, S> {}
impl<S: Strategy> HasOne for UFixed<1, 10, S> {}
impl<S: Strategy> HasOne for UFixed<1, 11, S> {}
impl<S: Strategy> HasOne for UFixed<1, 12, S> {}
impl<S: Strategy> HasOne for UFixed<1, 13, S> {}
impl<S: Strategy> HasOne for UFixed<1, 14, S> {}
impl<S: Strategy> HasOne for UFixed<1, 15, S> {}
impl<S: Strategy> HasOne for UFixed<1, 16, S> {}
impl<S: Strategy> HasOne for UFixed<1, 17, S> {}
impl<S: Strategy> HasOne for UFixed<1, 18, S> {}
impl<S: Strategy> HasOne for UFixed<1, 19, S> {}
impl<S: Strategy> HasOne for UFixed<1, 20, S> {}
impl<S: Strategy> HasOne for UFixed<1, 21, S> {}
impl<S: Strategy> HasOne for UFixed<1, 22, S> {}
impl<S: Strategy> HasOne for UFixed<1, 23, S> {}
impl<S: Strategy> HasOne for UFixed<1, 24, S> {}
impl<S: Strategy> HasOne for UFixed<1, 25, S> {}
impl<S: Strategy> HasOne for UFixed<1, 26, S> {}
impl<S: Strategy> HasOne for UFixed<1, 27, S> {}
impl<S: Strategy> HasOne for UFixed<1, 28, S> {}
impl<S: Strategy> HasOne for UFixed<1, 29, S> {}
impl<S: Strategy> HasOne for UFixed<1, 30, S> {}
impl<S: Strategy> HasOne for UFixed<1, 31, S> {}
impl<S: Strategy> HasOne for UFixed<2, 0, S> {}
impl<S: Strategy> HasOne for UFixed<2, 1, S> {}
impl<S: Strategy> HasOne for UFixed<2, 2, S> {}
impl<S: Strategy> HasOne for UFixed<2, 3, S> {}
impl<S: Strategy> HasOne for UFixed<2, 4, S> {}
impl<S: Strategy> HasOne for UFixed<2, 5, S> {}
impl<S: Strategy> HasOne for UFixed<2, 6, S> {}
impl<S: Strategy> HasOne for UFixed<2, 7, S> {}
impl<S: Strategy> HasOne for UFixed<2, 8, S> {}
impl<S: Strategy> HasOne for UFixed<2, 9, S> {}
impl<S: Strategy> HasOne for UFixed<2, 10, S> {}
impl<S: Strategy> HasOne for UFixed<2, 11, S> {}
impl<S: Strategy> HasOne for UFixed<2, 12, S> {}
impl<S: Strategy> HasOne for UFixed<2, 13, S> {}
impl<S: Strategy> HasOne for UFixed<2, 14, S> {}
impl<S: Strategy> HasOne for UFixed<2, 15, S> {}
impl<S: Strategy> HasOne for UFixed<2, 16, S> {}
impl<S: Strategy> HasOne for UFixed<2, 17, S> {}
impl<S: Strategy> HasOne for UFixed<2, 18, S> {}
impl<S: Strategy> HasOne for UFixed<2, 19, S> {}
impl<S: Strategy> HasOne for UFixed<2, 20, S> {}
impl<S: Strategy> HasOne for UFixed<2, 21, S> {}
impl<S: Strategy> HasOne for UFixed<2, 22, S> {}
impl<S: Strategy> HasOne for UFixed<2, 23, S> {}
impl<S: Strategy> HasOne for UFixed<2, 24, S> {}
impl<S: Strategy> HasOne for UFixed<2, 25, S> {}
impl<S: Strategy> HasOne for UFixed<2, 26, S> {}
impl<S: Strategy> HasOne for UFixed<2, 27, S> {}
impl<S: Strategy> HasOne for UFixed<2, 28, S> {}
impl<S: Strategy> HasOne for UFixed<2, 29, S> {}
impl<S: Strategy> HasOne for UFixed<2, 30, S> {}
impl<S: Strategy> HasOne for UFixed<3, 0, S> {}
impl<S: Strategy> HasOne for UFixed<3, 1, S> {}
impl<S: Strategy> HasOne for UFixed<3, 2, S> {}
impl<S: Strategy> HasOne for UFixed<3, 3, S> {}
impl<S: Strategy> HasOne for UFixed<3, 4, S> {}
impl<S: Strategy> HasOne for UFixed<3, 5, S> {}
impl<S: Strategy> HasOne for UFixed<3, 6, S> {}
impl<S: Strategy> HasOne for UFixed<3, 7, S> {}
impl<S: Strategy> HasOne for UFixed<3, 8, S> {}
impl<S: Strategy> HasOne for UFixed<3, 9, S> {}
impl<S: Strategy> HasOne for UFixed<3, 10, S> {}
impl<S: Strategy> HasOne for UFixed<3, 11, S> {}
impl<S: Strategy> HasOne for UFixed<3, 12, S> {}
impl<S: Strategy> HasOne for UFixed<3, 13, S> {}
impl<S: Strategy> HasOne for UFixed<3, 14, S> {}
impl<S: Strategy> HasOne for UFixed<3, 15, S> {}
impl<S: Strategy> HasOne for UFixed<3, 16, S> {}
impl<S: Strategy> HasOne for UFixed<3, 17, S> {}
impl<S: Strategy> HasOne for UFixed<3, 18, S> {}
impl<S: Strategy> HasOne for UFixed<3, 19, S> {}
impl<S: Strategy> HasOne for UFixed<3, 20, S> {}
impl<S: Strategy> HasOne for UFixed<3, 21, S> {}
impl<S: Strategy> HasOne for UFixed<3, 22, S> {}
impl<S: Strategy> HasOne for UFixed<3, 23, S> {}
impl<S: Strategy> HasOne for UFixed<3, 24, S> {}
impl<S: Strategy> HasOne for UFixed<3, 25, S> {}
impl<S: Strategy> HasOne for UFixed<3, 26, S> {}
impl<S: Strategy> HasOne for UFixed<3, 27, S> {}
impl<S: Strategy> HasOne for UFixed<3, 28, S> {}
impl<S: Strategy> HasOne for UFixed<3, 29, S> {}
impl<S: Strategy> HasOne for UFixed<4, 0, S> {}
impl<S: Strategy> HasOne for UFixed<4, 1, S> {}
impl<S: Strategy> HasOne for UFixed<4, 2, S> {}
impl<S: Strategy> HasOne for UFixed<4, 3, S> {}
impl<S: Strategy> HasOne for UFixed<4, 4, S> {}
impl<S: Strategy> HasOne for UFixed<4, 5, S> {}
impl<S: Strategy> HasOne for UFixed<4, 6, S> {}
impl<S: Strategy> HasOne for UFixed<4, 7, S> {}
impl<S: Strategy> HasOne for UFixed<4, 8, S> {}
impl<S: Strategy> HasOne for UFixed<4, 9, S> {}
impl<S: Strategy> HasOne for UFixed<4, 10, S> {}
impl<S: Strategy> HasOne for UFixed<4, 11, S> {}
impl<S: Strategy> HasOne for UFixed<4, 12, S> {}
impl<S: Strategy> HasOne for UFixed<4, 13, S> {}
impl<S: Strategy> HasOne for UFixed<4, 14, S> {}
impl<S: Strategy> HasOne for UFixed<4, 15, S> {}
impl<S: Strategy> HasOne for UFixed<4, 16, S> {}
impl<S: Strategy> HasOne for UFixed<4, 17, S> {}
impl<S: Strategy> HasOne for UFixed<4, 18, S> {}
impl<S: Strategy> HasOne for UFixed<4, 19, S> {}
impl<S: Strategy> HasOne for UFixed<4, 20, S> {}
impl<S: Strategy> HasOne for UFixed<4, 21, S> {}
impl<S: Strategy> HasOne for UFixed<4, 22, S> {}
impl<S: Strategy> HasOne for UFixed<4, 23, S> {}
impl<S: Strategy> HasOne for UFixed<4, 24, S> {}
impl<S: Strategy> HasOne for UFixed<4, 25, S> {}
impl<S: Strategy> HasOne for UFixed<4, 26, S> {}
impl<S: Strategy> HasOne for UFixed<4, 27, S> {}
impl<S: Strategy> HasOne for UFixed<4, 28, S> {}
impl<S: Strategy> HasOne for UFixed<5, 0, S> {}
impl<S: Strategy> HasOne for UFixed<5, 1, S> {}
impl<S: Strategy> HasOne for UFixed<5, 2, S> {}
impl<S: Strategy> HasOne for UFixed<5, 3, S> {}
impl<S: Strategy> HasOne for UFixed<5, 4, S> {}
impl<S: Strategy> HasOne for UFixed<5, 5, S> {}
impl<S: Strategy> HasOne for UFixed<5, 6, S> {}
impl<S: Strategy> HasOne for UFixed<5, 7, S> {}
impl<S: Strategy> HasOne for UFixed<5, 8, S> {}
impl<S: Strategy> HasOne for UFixed<5, 9, S> {}
impl<S: Strategy> HasOne for UFixed<5, 10, S> {}
impl<S: Strategy> HasOne for UFixed<5, 11, S> {}
impl<S: Strategy> HasOne for UFixed<5, 12, S> {}
impl<S: Strategy> HasOne for UFixed<5, 13, S> {}
impl<S: Strategy> HasOne for UFixed<5, 14, S> {}
impl<S: Strategy> HasOne for UFixed<5, 15, S> {}
impl<S: Strategy> HasOne for UFixed<5, 16, S> {}
impl<S: Strategy> HasOne for UFixed<5, 17, S> {}
impl<S: Strategy> HasOne for UFixed<5, 18, S> {}
impl<S: Strategy> HasOne for UFixed<5, 19, S> {}
impl<S: Strategy> HasOne for UFixed<5, 20, S> {}
impl<S: Strategy> HasOne for UFixed<5, 21, S> {}
impl<S: Strategy> HasOne for UFixed<5, 22, S> {}
impl<S: Strategy> HasOne for UFixed<5, 23, S> {}
impl<S: Strategy> HasOne for UFixed<5, 24, S> {}
impl<S: Strategy> HasOne for UFixed<5, 25, S> {}
impl<S: Strategy> HasOne for UFixed<5, 26, S> {}
impl<S: Strategy> HasOne for UFixed<5, 27, S> {}
impl<S: Strategy> HasOne for UFixed<6, 0, S> {}
impl<S: Strategy> HasOne for UFixed<6, 1, S> {}
impl<S: Strategy> HasOne for UFixed<6, 2, S> {}
impl<S: Strategy> HasOne for UFixed<6, 3, S> {}
impl<S: Strategy> HasOne for UFixed<6, 4, S> {}
impl<S: Strategy> HasOne for UFixed<6, 5, S> {}
impl<S: Strategy> HasOne for UFixed<6, 6, S> {}
impl<S: Strategy> HasOne for UFixed<6, 7, S> {}
impl<S: Strategy> HasOne for UFixed<6, 8, S> {}
impl<S: Strategy> HasOne for UFixed<6, 9, S> {}
impl<S: Strategy> HasOne for UFixed<6, 10, S> {}
impl<S: Strategy> HasOne for UFixed<6, 11, S> {}
impl<S: Strategy> HasOne for UFixed<6, 12, S> {}
impl<S: Strategy> HasOne for UFixed<6, 13, S> {}
impl<S: Strategy> HasOne for UFixed<6, 14, S> {}
impl<S: Strategy> HasOne for UFixed<6, 15, S> {}
impl<S: Strategy> HasOne for UFixed<6, 16, S> {}
impl<S: Strategy> HasOne for UFixed<6, 17, S> {}
impl<S: Strategy> HasOne for UFixed<6, 18, S> {}
impl<S: Strategy> HasOne for UFixed<6, 19, S> {}
impl<S: Strategy> HasOne for UFixed<6, 20, S> {}
impl<S: Strategy> HasOne for UFixed<6, 21, S> {}
impl<S: Strategy> HasOne for UFixed<6, 22, S> {}
impl<S: Strategy> HasOne for UFixed<6, 23, S> {}
impl<S: Strategy> HasOne for UFixed<6, 24, S> {}
impl<S: Strategy> HasOne for UFixed<6, 25, S> {}
impl<S: Strategy> HasOne for UFixed<6, 26, S> {}
impl<S: Strategy> HasOne for UFixed<7, 0, S> {}
impl<S: Strategy> HasOne for UFixed<7, 1, S> {}
impl<S: Strategy> HasOne for UFixed<7, 2, S> {}
impl<S: Strategy> HasOne for UFixed<7, 3, S> {}
impl<S: Strategy> HasOne for UFixed<7, 4, S> {}
impl<S: Strategy> HasOne for UFixed<7, 5, S> {}
impl<S: Strategy> HasOne for UFixed<7, 6, S> {}
impl<S: Strategy> HasOne for UFixed<7, 7, S> {}
impl<S: Strategy> HasOne for UFixed<7, 8, S> {}
impl<S: Strategy> HasOne for UFixed<7, 9, S> {}
impl<S: Strategy> HasOne for UFixed<7, 10, S> {}
impl<S: Strategy> HasOne for UFixed<7, 11, S> {}
impl<S: Strategy> HasOne for UFixed<7, 12, S> {}
impl<S: Strategy> HasOne for UFixed<7, 13, S> {}
impl<S: Strategy> HasOne for UFixed<7, 14, S> {}
impl<S: Strategy> HasOne for UFixed<7, 15, S> {}
impl<S: Strategy> HasOne for UFixed<7, 16, S> {}
impl<S: Strategy> HasOne for UFixed<7, 17, S> {}
impl<S: Strategy> HasOne for UFixed<7, 18, S> {}
impl<S: Strategy> HasOne for UFixed<7, 19, S> {}
impl<S: Strategy> HasOne for UFixed<7, 20, S> {}
impl<S: Strategy> HasOne for UFixed<7, 21, S> {}
impl<S: Strategy> HasOne for UFixed<7, 22, S> {}
impl<S: Strategy> HasOne for UFixed<7, 23, S> {}
impl<S: Strategy> HasOne for UFixed<7, 24, S> {}
impl<S: Strategy> HasOne for UFixed<7, 25, S> {}
impl<S: Strategy> HasOne for UFixed<8, 0, S> {}
impl<S: Strategy> HasOne for UFixed<8, 1, S> {}
impl<S: Strategy> HasOne for UFixed<8, 2, S> {}
impl<S: Strategy> HasOne for UFixed<8, 3, S> {}
impl<S: Strategy> HasOne for UFixed<8, 4, S> {}
impl<S: Strategy> HasOne for UFixed<8, 5, S> {}
impl<S: Strategy> HasOne for UFixed<8, 6, S> {}
impl<S: Strategy> HasOne for UFixed<8, 7, S> {}
impl<S: Strategy> HasOne for UFixed<8, 8, S> {}
impl<S: Strategy> HasOne for UFixed<8, 9, S> {}
impl<S: Strategy> HasOne for UFixed<8, 10, S> {}
impl<S: Strategy> HasOne for UFixed<8, 11, S> {}
impl<S: Strategy> HasOne for UFixed<8, 12, S> {}
impl<S: Strategy> HasOne for UFixed<8, 13, S> {}
impl<S: Strategy> HasOne for UFixed<8, 14, S> {}
impl<S: Strategy> HasOne for UFixed<8, 15, S> {}
impl<S: Strategy> HasOne for UFixed<8, 16, S> {}
impl<S: Strategy> HasOne for UFixed<8, 17, S> {}
impl<S: Strategy> HasOne for UFixed<8, 18, S> {}
impl<S: Strategy> HasOne for UFixed<8, 19, S> {}
impl<S: Strategy> HasOne for UFixed<8, 20, S> {}
impl<S: Strategy> HasOne for UFixed<8, 21, S> {}
impl<S: Strategy> HasOne for UFixed<8, 22, S> {}
impl<S: Strategy> HasOne for UFixed<8, 23, S> {}
impl<S: Strategy> HasOne for UFixed<8, 24, S> {}
impl<S: Strategy> HasOne for UFixed<9, 0, S> {}
impl<S: Strategy> HasOne for UFixed<9, 1, S> {}
impl<S: Strategy> HasOne for UFixed<9, 2, S> {}
impl<S: Strategy> HasOne for UFixed<9, 3, S> {}
impl<S: Strategy> HasOne for UFixed<9, 4, S> {}
impl<S: Strategy> HasOne for UFixed<9, 5, S> {}
impl<S: Strategy> HasOne for UFixed<9, 6, S> {}
impl<S: Strategy> HasOne for UFixed<9, 7, S> {}
impl<S: Strategy> HasOne for UFixed<9, 8, S> {}
impl<S: Strategy> HasOne for UFixed<9, 9, S> {}
impl<S: Strategy> HasOne for UFixed<9, 10, S> {}
impl<S: Strategy> HasOne for UFixed<9, 11, S> {}
impl<S: Strategy> HasOne for UFixed<9, 12, S> {}
impl<S: Strategy> HasOne for UFixed<9, 13, S> {}
impl<S: Strategy> HasOne for UFixed<9, 14, S> {}
impl<S: Strategy> HasOne for UFixed<9, 15, S> {}
impl<S: Strategy> HasOne for UFixed<9, 16, S> {}
impl<S: Strategy> HasOne for UFixed<9, 17, S> {}
impl<S: Strategy> HasOne for UFixed<9, 18, S> {}
impl<S: Strategy> HasOne for UFixed<9, 19, S> {}
impl<S: Strategy> HasOne for UFixed<9, 20, S> {}
impl<S: Strategy> HasOne for UFixed<9, 21, S> {}
impl<S: Strategy> HasOne for UFixed<9, 22, S> {}
impl<S: Strategy> HasOne for UFixed<9, 23, S> {}
impl<S: Strategy> HasOne for UFixed<10, 0, S> {}
impl<S: Strategy> HasOne for UFixed<10, 1, S> {}
impl<S: Strategy> HasOne for UFixed<10, 2, S> {}
impl<S: Strategy> HasOne for UFixed<10, 3, S> {}
impl<S: Strategy> HasOne for UFixed<10, 4, S> {}
impl<S: Strategy> HasOne for UFixed<10, 5, S> {}
impl<S: Strategy> HasOne for UFixed<10, 6, S> {}
impl<S: Strategy> HasOne for UFixed<10, 7, S> {}
impl<S: Strategy> HasOne for UFixed<10, 8, S> {}
impl<S: Strategy> HasOne for UFixed<10, 9, S> {}
impl<S: Strategy> HasOne for UFixed<10, 10, S> {}
impl<S: Strategy> HasOne for UFixed<10, 11, S> {}
impl<S: Strategy> HasOne for UFixed<10, 12, S> {}
impl<S: Strategy> HasOne for UFixed<10, 13, S> {}
impl<S: Strategy> HasOne for UFixed<10, 14, S> {}
impl<S: Strategy> HasOne for UFixed<10, 15, S> {}
impl<S: Strategy> HasOne for UFixed<10, 16, S> {}
impl<S: Strategy> HasOne for UFixed<10, 17, S> {}
impl<S: Strategy> HasOne for UFixed<10, 18, S> {}
impl<S: Strategy> HasOne for UFixed<10, 19, S> {}
impl<S: Strategy> HasOne for UFixed<10, 20, S> {}
impl<S: Strategy> HasOne for UFixed<10, 21, S> {}
impl<S: Strategy> HasOne for UFixed<10, 22, S> {}
impl<S: Strategy> HasOne for UFixed<11, 0, S> {}
impl<S: Strategy> HasOne for UFixed<11, 1, S> {}
impl<S: Strategy> HasOne for UFixed<11, 2, S> {}
impl<S: Strategy> HasOne for UFixed<11, 3, S> {}
impl<S: Strategy> HasOne for UFixed<11, 4, S> {}
impl<S: Strategy> HasOne for UFixed<11, 5, S> {}
impl<S: Strategy> HasOne for UFixed<11, 6, S> {}
impl<S: Strategy> HasOne for UFixed<11, 7, S> {}
impl<S: Strategy> HasOne for UFixed<11, 8, S> {}
impl<S: Strategy> HasOne for UFixed<11, 9, S> {}
impl<S: Strategy> HasOne for UFixed<11, 10, S> {}
impl<S: Strategy> HasOne for UFixed<11, 11, S> {}
impl<S: Strategy> HasOne for UFixed<11, 12, S> {}
impl<S: Strategy> HasOne for UFixed<11, 13, S> {}
impl<S: Strategy> HasOne for UFixed<11, 14, S> {}
impl<S: Strategy> HasOne for UFixed<11, 15, S> {}
impl<S: Strategy> HasOne for UFixed<11, 16, S> {}
impl<S: Strategy> HasOne for UFixed<11, 17, S> {}
impl<S: Strategy> HasOne for UFixed<11, 18, S> {}
impl<S: Strategy> HasOne for UFixed<11, 19, S> {}
impl<S: Strategy> HasOne for UFixed<11, 20, S> {}
impl<S: Strategy> HasOne for UFixed<11, 21, S> {}
impl<S: Strategy> HasOne for UFixed<12, 0, S> {}
impl<S: Strategy> HasOne for UFixed<12, 1, S> {}
impl<S: Strategy> HasOne for UFixed<12, 2, S> {}
impl<S: Strategy> HasOne for UFixed<12, 3, S> {}
impl<S: Strategy> HasOne for UFixed<12, 4, S> {}
impl<S: Strategy> HasOne for UFixed<12, 5, S> {}
impl<S: Strategy> HasOne for UFixed<12, 6, S> {}
impl<S: Strategy> HasOne for UFixed<12, 7, S> {}
impl<S: Strategy> HasOne for UFixed<12, 8, S> {}
impl<S: Strategy> HasOne for UFixed<12, 9, S> {}
impl<S: Strategy> HasOne for UFixed<12, 10, S> {}
impl<S: Strategy> HasOne for UFixed<12, 11, S> {}
impl<S: Strategy> HasOne for UFixed<12, 12, S> {}
impl<S: Strategy> HasOne for UFixed<12, 13, S> {}
impl<S: Strategy> HasOne for UFixed<12, 14, S> {}
impl<S: Strategy> HasOne for UFixed<12, 15, S> {}
impl<S: Strategy> HasOne for UFixed<12, 16, S> {}
impl<S: Strategy> HasOne for UFixed<12, 17, S> {}
impl<S: Strategy> HasOne for UFixed<12, 18, S> {}
impl<S: Strategy> HasOne for UFixed<12, 19, S> {}
impl<S: Strategy> HasOne for UFixed<12, 20, S> {}
impl<S: Strategy> HasOne for UFixed<13, 0, S> {}
impl<S: Strategy> HasOne for UFixed<13, 1, S> {}
impl<S: Strategy> HasOne for UFixed<13, 2, S> {}
impl<S: Strategy> HasOne for UFixed<13, 3, S> {}
impl<S: Strategy> HasOne for UFixed<13, 4, S> {}
impl<S: Strategy> HasOne for UFixed<13, 5, S> {}
impl<S: Strategy> HasOne for UFixed<13, 6, S> {}
impl<S: Strategy> HasOne for UFixed<13, 7, S> {}
impl<S: Strategy> HasOne for UFixed<13, 8, S> {}
impl<S: Strategy> HasOne for UFixed<13, 9, S> {}
impl<S: Strategy> HasOne for UFixed<13, 10, S> {}
impl<S: Strategy> HasOne for UFixed<13, 11, S> {}
impl<S: Strategy> HasOne for UFixed<13, 12, S> {}
impl<S: Strategy> HasOne for UFixed<13, 13, S> {}
impl<S: Strategy> HasOne for UFixed<13, 14, S> {}
impl<S: Strategy> HasOne for UFixed<13, 15, S> {}
impl<S: Strategy> HasOne for UFixed<13, 16, S> {}
impl<S: Strategy> HasOne for UFixed<13, 17, S> {}
impl<S: Strategy> HasOne for UFixed<13, 18, S> {}
impl<S: Strategy> HasOne for UFixed<13, 19, S> {}
impl<S: Strategy> HasOne for UFixed<14, 0, S> {}
impl<S: Strategy> HasOne for UFixed<14, 1, S> {}
impl<S: Strategy> HasOne for UFixed<14, 2, S> {}
impl<S: Strategy> HasOne for UFixed<14, 3, S> {}
impl<S: Strategy> HasOne for UFixed<14, 4, S> {}
impl<S: Strategy> HasOne for UFixed<14, 5, S> {}
impl<S: Strategy> HasOne for UFixed<14, 6, S> {}
impl<S: Strategy> HasOne for UFixed<14, 7, S> {}
impl<S: Strategy> HasOne for UFixed<14, 8, S> {}
impl<S: Strategy> HasOne for UFixed<14, 9, S> {}
impl<S: Strategy> HasOne for UFixed<14, 10, S> {}
impl<S: Strategy> HasOne for UFixed<14, 11, S> {}
impl<S: Strategy> HasOne for UFixed<14, 12, S> {}
impl<S: Strategy> HasOne for UFixed<14, 13, S> {}
impl<S: Strategy> HasOne for UFixed<14, 14, S> {}
impl<S: Strategy> HasOne for UFixed<14, 15, S> {}
impl<S: Strategy> HasOne for UFixed<14, 16, S> {}
impl<S: Strategy> HasOne for UFixed<14, 17, S> {}
impl<S: Strategy> HasOne for UFixed<14, 18, S> {}
impl<S: Strategy> HasOne for UFixed<15, 0, S> {}
impl<S: Strategy> HasOne for UFixed<15, 1, S> {}
impl<S: Strategy> HasOne for UFixed<15, 2, S> {}
impl<S: Strategy> HasOne for UFixed<15, 3, S> {}
impl<S: Strategy> HasOne for UFixed<15, 4, S> {}
impl<S: Strategy> HasOne for UFixed<15, 5, S> {}
impl<S: Strategy> HasOne for UFixed<15, 6, S> {}
impl<S: Strategy> HasOne for UFixed<15, 7, S> {}
impl<S: Strategy> HasOne for UFixed<15, 8, S> {}
impl<S: Strategy> HasOne for UFixed<15, 9, S> {}
impl<S: Strategy> HasOne for UFixed<15, 10, S> {}
impl<S: Strategy> HasOne for UFixed<15, 11, S> {}
impl<S: Strategy> HasOne for UFixed<15, 12, S> {}
impl<S: Strategy> HasOne for UFixed<15, 13, S> {}
impl<S: Strategy> HasOne for UFixed<15, 14, S> {}
impl<S: Strategy> HasOne for UFixed<15, 15, S> {}
impl<S: Strategy> HasOne for UFixed<15, 16, S> {}
impl<S: Strategy> HasOne for UFixed<15, 17, S> {}
impl<S: Strategy> HasOne for UFixed<16, 0, S> {}
impl<S: Strategy> HasOne for UFixed<16, 1, S> {}
impl<S: Strategy> HasOne for UFixed<16, 2, S> {}
impl<S: Strategy> HasOne for UFixed<16, 3, S> {}
impl<S: Strategy> HasOne for UFixed<16, 4, S> {}
impl<S: Strategy> HasOne for UFixed<16, 5, S> {}
impl<S: Strategy> HasOne for UFixed<16, 6, S> {}
impl<S: Strategy> HasOne for UFixed<16, 7, S> {}
impl<S: Strategy> HasOne for UFixed<16, 8, S> {}
impl<S: Strategy> HasOne for UFixed<16, 9, S> {}
impl<S: Strategy> HasOne for UFixed<16, 10, S> {}
impl<S: Strategy> HasOne for UFixed<16, 11, S> {}
impl<S: Strategy> HasOne for UFixed<16, 12, S> {}
impl<S: Strategy> HasOne for UFixed<16, 13, S> {}
impl<S: Strategy> HasOne for UFixed<16, 14, S> {}
impl<S: Strategy> HasOne for UFixed<16, 15, S> {}
impl<S: Strategy> HasOne for UFixed<16, 16, S> {}
impl<S: Strategy> HasOne for UFixed<17, 0, S> {}
impl<S: Strategy> HasOne for UFixed<17, 1, S> {}
impl<S: Strategy> HasOne for UFixed<17, 2, S> {}
impl<S: Strategy> HasOne for UFixed<17, 3, S> {}
impl<S: Strategy> HasOne for UFixed<17, 4, S> {}
impl<S: Strategy> HasOne for UFixed<17, 5, S> {}
impl<S: Strategy> HasOne for UFixed<17, 6, S> {}
impl<S: Strategy> HasOne for UFixed<17, 7, S> {}
impl<S: Strategy> HasOne for UFixed<17, 8, S> {}
impl<S: Strategy> HasOne for UFixed<17, 9, S> {}
impl<S: Strategy> HasOne for UFixed<17, 10, S> {}
impl<S: Strategy> HasOne for UFixed<17, 11, S> {}
impl<S: Strategy> HasOne for UFixed<17, 12, S> {}
impl<S: Strategy> HasOne for UFixed<17, 13, S> {}
impl<S: Strategy> HasOne for UFixed<17, 14, S> {}
impl<S: Strategy> HasOne for UFixed<17, 15, S> {}
impl<S: Strategy> HasOne for UFixed<18, 0, S> {}
impl<S: Strategy> HasOne for UFixed<18, 1, S> {}
impl<S: Strategy> HasOne for UFixed<18, 2, S> {}
impl<S: Strategy> HasOne for UFixed<18, 3, S> {}
impl<S: Strategy> HasOne for UFixed<18, 4, S> {}
impl<S: Strategy> HasOne for UFixed<18, 5, S> {}
impl<S: Strategy> HasOne for UFixed<18, 6, S> {}
impl<S: Strategy> HasOne for UFixed<18, 7, S> {}
impl<S: Strategy> HasOne for UFixed<18, 8, S> {}
impl<S: Strategy> HasOne for UFixed<18, 9, S> {}
impl<S: Strategy> HasOne for UFixed<18, 10, S> {}
impl<S: Strategy> HasOne for UFixed<18, 11, S> {}
impl<S: Strategy> HasOne for UFixed<18, 12, S> {}
impl<S: Strategy> HasOne for UFixed<18, 13, S> {}
impl<S: Strategy> HasOne for UFixed<18, 14, S> {}
impl<S: Strategy> HasOne for UFixed<19, 0, S> {}
impl<S: Strategy> HasOne for UFixed<19, 1, S> {}
impl<S: Strategy> HasOne for UFixed<19, 2, S> {}
impl<S: Strategy> HasOne for UFixed<19, 3, S> {}
impl<S: Strategy> HasOne for UFixed<19, 4, S> {}
impl<S: Strategy> HasOne for UFixed<19, 5, S> {}
impl<S: Strategy> HasOne for UFixed<19, 6, S> {}
impl<S: Strategy> HasOne for UFixed<19, 7, S> {}
impl<S: Strategy> HasOne for UFixed<19, 8, S> {}
impl<S: Strategy> HasOne for UFixed<19, 9, S> {}
impl<S: Strategy> HasOne for UFixed<19, 10, S> {}
impl<S: Strategy> HasOne for UFixed<19, 11, S> {}
impl<S: Strategy> HasOne for UFixed<19, 12, S> {}
impl<S: Strategy> HasOne for UFixed<19, 13, S> {}
impl<S: Strategy> HasOne for UFixed<20, 0, S> {}
impl<S: Strategy> HasOne for UFixed<20, 1, S> {}
impl<S: Strategy> HasOne for UFixed<20, 2, S> {}
impl<S: Strategy> HasOne for UFixed<20, 3, S> {}
impl<S: Strategy> HasOne for UFixed<20, 4, S> {}
impl<S: Strategy> HasOne for UFixed<20, 5, S> {}
impl<S: Strategy> HasOne for UFixed<20, 6, S> {}
impl<S: Strategy> HasOne for UFixed<20, 7, S> {}
impl<S: Strategy> HasOne for UFixed<20, 8, S> {}
impl<S: Strategy> HasOne for UFixed<20, 9, S> {}
impl<S: Strategy> HasOne for UFixed<20, 10, S> {}
impl<S: Strategy> HasOne for UFixed<20, 11, S> {}
impl<S: Strategy> HasOne for UFixed<20, 12, S> {}
impl<S: Strategy> HasOne for UFixed<21, 0, S> {}
impl<S: Strategy> HasOne for UFixed<21, 1, S> {}
impl<S: Strategy> HasOne for UFixed<21, 2, S> {}
impl<S: Strategy> HasOne for UFixed<21, 3, S> {}
impl<S: Strategy> HasOne for UFixed<21, 4, S> {}
impl<S: Strategy> HasOne for UFixed<21, 5, S> {}
impl<S: Strategy> HasOne for UFixed<21, 6, S> {}
impl<S: Strategy> HasOne for UFixed<21, 7, S> {}
impl<S: Strategy> HasOne for UFixed<21, 8, S> {}
impl<S: Strategy> HasOne for UFixed<21, 9, S> {}
impl<S: Strategy> HasOne for UFixed<21, 10, S> {}
impl<S: Strategy> HasOne for UFixed<21, 11, S> {}
impl<S: Strategy> HasOne for UFixed<22, 0, S> {}
impl<S: Strategy> HasOne for UFixed<22, 1, S> {}
impl<S: Strategy> HasOne for UFixed<22, 2, S> {}
impl<S: Strategy> HasOne for UFixed<22, 3, S> {}
impl<S: Strategy> HasOne for UFixed<22, 4, S> {}
impl<S: Strategy> HasOne for UFixed<22, 5, S> {}
impl<S: Strategy> HasOne for UFixed<22, 6, S> {}
impl<S: Strategy> HasOne for UFixed<22, 7, S> {}
impl<S: Strategy> HasOne for UFixed<22, 8, S> {}
impl<S: Strategy> HasOne for UFixed<22, 9, S> {}
impl<S: Strategy> HasOne for UFixed<22, 10, S> {}
impl<S: Strategy> HasOne for UFixed<23, 0, S> {}
impl<S: Strategy> HasOne for UFixed<23, 1, S> {}
impl<S: Strategy> HasOne for UFixed<23, 2, S> {}
impl<S: Strategy> HasOne for UFixed<23, 3, S> {}
impl<S: Strategy> HasOne for UFixed<23, 4, S> {}
impl<S: Strategy> HasOne for UFixed<23, 5, S> {}
impl<S: Strategy> HasOne for UFixed<23, 6, S> {}
impl<S: Strategy> HasOne for UFixed<23, 7, S> {}
impl<S: Strategy> HasOne for UFixed<23, 8, S> {}
impl<S: Strategy> HasOne for UFixed<23, 9, S> {}
impl<S: Strategy> HasOne for UFixed<24, 0, S> {}
impl<S: Strategy> HasOne for UFixed<24, 1, S> {}
impl<S: Strategy> HasOne for UFixed<24, 2, S> {}
impl<S: Strategy> HasOne for UFixed<24, 3, S> {}
impl<S: Strategy> HasOne for UFixed<24, 4, S> {}
impl<S: Strategy> HasOne for UFixed<24, 5, S> {}
impl<S: Strategy> HasOne for UFixed<24, 6, S> {}
impl<S: Strategy> HasOne for UFixed<24, 7, S> {}
impl<S: Strategy> HasOne for UFixed<24, 8, S> {}
impl<S: Strategy> HasOne for UFixed<25, 0, S> {}
impl<S: Strategy> HasOne for UFixed<25, 1, S> {}
impl<S: Strategy> HasOne for UFixed<25, 2, S> {}
impl<S: Strategy> HasOne for UFixed<25, 3, S> {}
impl<S: Strategy> HasOne for UFixed<25, 4, S> {}
impl<S: Strategy> HasOne for UFixed<25, 5, S> {}
impl<S: Strategy> HasOne for UFixed<25, 6, S> {}
impl<S: Strategy> HasOne for UFixed<25, 7, S> {}
impl<S: Strategy> HasOne for UFixed<26, 0, S> {}
impl<S: Strategy> HasOne for UFixed<26, 1, S> {}
impl<S: Strategy> HasOne for UFixed<26, 2, S> {}
impl<S: Strategy> HasOne for UFixed<26, 3, S> {}
impl<S: Strategy> HasOne for UFixed<26, 4, S> {}
impl<S: Strategy> HasOne for UFixed<26, 5, S> {}
impl<S: Strategy> HasOne for UFixed<26, 6, S> {}
impl<S: Strategy> HasOne for UFixed<27, 0, S> {}
impl<S: Strategy> HasOne for UFixed<27, 1, S> {}
impl<S: Strategy> HasOne for UFixed<27, 2, S> {}
impl<S: Strategy> HasOne for UFixed<27, 3, S> {}
impl<S: Strategy> HasOne for UFixed<27, 4, S> {}
impl<S: Strategy> HasOne for UFixed<27, 5, S> {}
impl<S: Strategy> HasOne for UFixed<28, 0, S> {}
impl<S: Strategy> HasOne for UFixed<28, 1, S> {}
impl<S: Strategy> HasOne for UFixed<28, 2, S> {}
impl<S: Strategy> HasOne for UFixed<28, 3, S> {}
impl<S: Strategy> HasOne for UFixed<28, 4, S> {}
impl<S: Strategy> HasOne for UFixed<29, 0, S> {}
impl<S: Strategy> HasOne for UFixed<29, 1, S> {}
impl<S: Strategy> HasOne for UFixed<29, 2, S> {}
impl<S: Strategy> HasOne for UFixed<29, 3, S> {}
impl<S: Strategy> HasOne for UFixed<30, 0, S> {}
impl<S: Strategy> HasOne for UFixed<30, 1, S> {}
impl<S: Strategy> HasOne for UFixed<30, 2, S> {}
impl<S: Strategy> HasOne for UFixed<31, 0, S> {}
impl<S: Strategy> HasOne for UFixed<31, 1, S> {}
impl<S: Strategy> HasOne for UFixed<32, 0, S> {}
pub type N0 = UFixed<1, 0, Hot>;
pub const W0: usize = <N0 as Stored>::W;
pub fn one_ok_0()
where
    N0: HasOne,
{
}
pub const F0_IS_ZERO: bool = true;
pub type N1 = UFixed<2, 0, Warm>;
pub const W1: usize = <N1 as Stored>::W;
pub fn one_ok_1()
where
    N1: HasOne,
{
}
pub const F1_IS_ZERO: bool = true;
pub type N2 = UFixed<3, 0, Cold>;
pub const W2: usize = <N2 as Stored>::W;
pub fn one_ok_2()
where
    N2: HasOne,
{
}
pub const F2_IS_ZERO: bool = true;
pub type N3 = UFixed<4, 0, Hot>;
pub const W3: usize = <N3 as Stored>::W;
pub fn one_ok_3()
where
    N3: HasOne,
{
}
pub const F3_IS_ZERO: bool = true;
pub type N4 = UFixed<5, 0, Warm>;
pub const W4: usize = <N4 as Stored>::W;
pub fn one_ok_4()
where
    N4: HasOne,
{
}
pub const F4_IS_ZERO: bool = true;
pub type N5 = UFixed<6, 0, Cold>;
pub const W5: usize = <N5 as Stored>::W;
pub fn one_ok_5()
where
    N5: HasOne,
{
}
pub const F5_IS_ZERO: bool = true;
pub type N6 = UFixed<7, 0, Hot>;
pub const W6: usize = <N6 as Stored>::W;
pub fn one_ok_6()
where
    N6: HasOne,
{
}
pub const F6_IS_ZERO: bool = true;
pub type N7 = UFixed<11, 0, Warm>;
pub const W7: usize = <N7 as Stored>::W;
pub fn one_ok_7()
where
    N7: HasOne,
{
}
pub const F7_IS_ZERO: bool = true;
pub type N8 = UFixed<14, 0, Cold>;
pub const W8: usize = <N8 as Stored>::W;
pub fn one_ok_8()
where
    N8: HasOne,
{
}
pub const F8_IS_ZERO: bool = true;
pub type N9 = UFixed<16, 0, Hot>;
pub const W9: usize = <N9 as Stored>::W;
pub fn one_ok_9()
where
    N9: HasOne,
{
}
pub const F9_IS_ZERO: bool = true;
pub type N10 = UFixed<27, 0, Warm>;
pub const W10: usize = <N10 as Stored>::W;
pub fn one_ok_10()
where
    N10: HasOne,
{
}
pub const F10_IS_ZERO: bool = true;
pub type N11 = UFixed<28, 0, Cold>;
pub const W11: usize = <N11 as Stored>::W;
pub fn one_ok_11()
where
    N11: HasOne,
{
}
pub const F11_IS_ZERO: bool = true;
pub type N12 = UFixed<64, 0, Hot>;
pub const W12: usize = <N12 as Stored>::W;
pub fn one_ok_12()
where
    N12: HasOne,
{
}
pub const F12_IS_ZERO: bool = true;
pub type N13 = UFixed<0, 16, Warm>;
pub const W13: usize = <N13 as Stored>::W;
pub const F13_IS_ZERO: bool = false;
pub const SW0: usize = 5;
pub const SW1: usize = 7;
pub const SW2: usize = 9;
pub const SW3: usize = 11;
pub const SW4: usize = 16;
pub const SW5: usize = 20;
pub const SW6: usize = 23;
pub const SW7: usize = 38;
pub const SW8: usize = 42;
pub const SW9: usize = 80;
pub const SW10: usize = 43;
pub type C0 = Slot<1>;
pub fn build0() -> <C0 as Capacity>::Array<u32> {
    C0::build(0)
}
pub type C1 = Slot<3>;
pub fn build1() -> <C1 as Capacity>::Array<u32> {
    C1::build(0)
}
pub type C2 = Slot<4>;
pub fn build2() -> <C2 as Capacity>::Array<u32> {
    C2::build(0)
}
pub type C3 = Slot<7>;
pub fn build3() -> <C3 as Capacity>::Array<u32> {
    C3::build(0)
}
pub type C4 = Slot<8>;
pub fn build4() -> <C4 as Capacity>::Array<u32> {
    C4::build(0)
}
pub type C5 = Slot<13>;
pub fn build5() -> <C5 as Capacity>::Array<u32> {
    C5::build(0)
}
pub type C6 = Slot<16>;
pub fn build6() -> <C6 as Capacity>::Array<u32> {
    C6::build(0)
}
pub type C7 = Slot<28>;
pub fn build7() -> <C7 as Capacity>::Array<u32> {
    C7::build(0)
}
pub type C8 = Slot<32>;
pub fn build8() -> <C8 as Capacity>::Array<u32> {
    C8::build(0)
}
pub type C9 = Slot<64>;
pub fn build9() -> <C9 as Capacity>::Array<u32> {
    C9::build(0)
}

pub fn fold_generic<C: Capacity>(seed: u32) -> u32 {
    let mut a = C::build(seed);
    let s: &mut [u32] = a.as_mut();
    let mut i = 0;
    while i < s.len() {
        s[i] = s[i].wrapping_add(i as u32);
        i += 1;
    }
    let r: &[u32] = a.as_ref();
    let mut acc = 0u32;
    let mut j = 0;
    while j < r.len() {
        acc = acc.wrapping_add(r[j]);
        j += 1;
    }
    acc
}
pub fn scaled_fold<const I: u16, const F: u16, S: Strategy, C: Capacity>(seed: u32) -> u32
where
    UFixed<I, F, S>: Stored,
{
    fold_generic::<C>(seed).wrapping_mul(<UFixed<I, F, S> as Stored>::W as u32)
}

pub fn call0() -> u32 {
    scaled_fold::<1, 0, Hot, C0>(0)
}
pub fn call1() -> u32 {
    scaled_fold::<2, 0, Warm, C1>(1)
}
pub fn call2() -> u32 {
    scaled_fold::<3, 0, Cold, C2>(2)
}
pub fn call3() -> u32 {
    scaled_fold::<4, 0, Hot, C3>(3)
}
pub fn call4() -> u32 {
    scaled_fold::<5, 0, Warm, C4>(4)
}
pub fn call5() -> u32 {
    scaled_fold::<6, 0, Cold, C5>(5)
}
pub fn call6() -> u32 {
    scaled_fold::<7, 0, Hot, C6>(6)
}
pub fn call7() -> u32 {
    scaled_fold::<11, 0, Warm, C7>(7)
}
pub fn call8() -> u32 {
    scaled_fold::<14, 0, Cold, C8>(8)
}
pub fn call9() -> u32 {
    scaled_fold::<16, 0, Hot, C9>(9)
}
