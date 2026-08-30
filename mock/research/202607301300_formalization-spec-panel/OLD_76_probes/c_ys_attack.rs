//! C. Attack on arm `ys`, the staged route-Y predicate.
//!
//! `ys` moves `OneRepresentable` off the type checker and onto the declaration
//! site: the macro decides `I > 0` at expansion time and the type carries a
//! sealed witness. That is only a discharge of the obligation if a consumer
//! cannot write the witness itself and lie. If it can, `ys` is cheap because it
//! is not doing the job, and its number does not belong beside route Z's.
//!
//! Two attacks: name the wrong witness directly, and implement the witness
//! trait on a foreign marker.
#![no_std]
#![feature(adt_const_params)]

use core::marker::PhantomData;

pub struct Hot;
pub trait Strategy {}
impl Strategy for Hot {}

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

// ATTACK 1. A purely fractional numeral claiming the affirmative witness. This
// is exactly the `UFixed<0, F>::ONE` defect the review spent a stretch finding,
// re-offered through the staged door.
pub type Forged = Num<0, 8, OneYes, Hot>;
pub fn attack_1() {
    <Forged as HasOne>::witness();
}

// ATTACK 2. A downstream marker of the consumer's own, offered as a witness,
// which is the orphan-rule-legal route file 64 used to defeat `Unbounded`.
pub struct MyOwnYes;
impl OneWitness for MyOwnYes {
    const YES: bool = true;
}
