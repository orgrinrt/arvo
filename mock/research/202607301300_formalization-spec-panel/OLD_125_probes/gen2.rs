#![no_std]
extern crate tower;
use tower::*;

pub struct Idx<const N: u16>;
pub trait AdmittedWidth {
    type Out: Nat;
}

macro_rules! row {
    ($n:ty) => {
        impl AdmittedWidth for Idx<{ <$n as Nat>::VAL as u16 }> {
            type Out = $n;
        }
    };
}
macro_rules! emit {
    ( [] ; $($n:ty),* ) => { $( row!(Pz<$n>); )* };
    ( [ $_x:tt $($d:tt)* ] ; $($n:ty),* ) => {
        $( row!(Pz<$n>); )*
        emit!( [ $($d)* ] ; $( O<$n>, I<$n> ),* );
    };
}
impl AdmittedWidth for Idx<0> {
    type Out = Z;
}
emit!( [x x x x] ; H ); // 5 levels: widths 1 ..= 31

const _: () = assert!(<<Idx<13> as AdmittedWidth>::Out as Nat>::VAL == 13);
const _: () = assert!(<<Idx<31> as AdmittedWidth>::Out as Nat>::VAL == 31);
const _: () = assert!(<<Idx<0> as AdmittedWidth>::Out as Nat>::VAL == 0);
const _: () = assert!(<<Idx<24> as AdmittedWidth>::Out as Nat>::VAL == 24);
