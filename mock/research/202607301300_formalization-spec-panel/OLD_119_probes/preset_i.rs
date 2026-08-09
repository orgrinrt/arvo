#![allow(dead_code)]
use core::marker::PhantomData;
pub trait Numeral {}
pub struct Fix13_3;
impl Numeral for Fix13_3 {}
pub struct Fix7_9;
impl Numeral for Fix7_9 {}
pub trait StoredWidth {}
pub struct Minimum;
impl StoredWidth for Minimum {}
pub struct DoubleLogical;
impl StoredWidth for DoubleLogical {}
pub trait Lowering<N: Numeral> {
    type StoredWidth: StoredWidth;
}
pub struct Warm;
// SPELLING (i): the key is the numeral, so two numerals of the SAME kind
// may disagree, and nothing in the type system says they may not.
impl Lowering<Fix13_3> for Warm {
    type StoredWidth = DoubleLogical;
}
impl Lowering<Fix7_9> for Warm {
    type StoredWidth = Minimum;
}
pub struct Number<N: Numeral, S: Lowering<N>>(PhantomData<(N, S)>);
