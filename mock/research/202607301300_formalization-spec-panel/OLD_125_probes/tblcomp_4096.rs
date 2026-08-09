#![no_std]
#![allow(dead_code)]
extern crate tower;
pub use tower::*;
pub struct Idx<const N: u16>;
pub trait AdmittedWidth {
    type Out: Nat;
}
impl AdmittedWidth for Idx<{ <Z as Nat>::VAL as u16 }> {
    type Out = Z;
}
impl AdmittedWidth for Idx<{ <Pz<H> as Nat>::VAL as u16 }> {
    type Out = Pz<H>;
}
impl AdmittedWidth for Idx<{ <Pz<O<H>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<H>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<H>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<H>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<H>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<H>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<H>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<H>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<H>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<H>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<H>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<H>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<H>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<H>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<H>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<H>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<H>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<H>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<H>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<H>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<H>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<H>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<H>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<H>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<H>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<H>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<H>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<H>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<H>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<H>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<H>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<H>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<H>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<H>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<H>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<H>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<H>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<H>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<H>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<H>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<H>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<H>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<H>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<H>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<H>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<H>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<H>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<H>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<H>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<H>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<H>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<H>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<H>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<H>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<H>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<H>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<H>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<H>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<H>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<H>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<O<H>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<O<H>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<O<H>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<O<H>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<O<H>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<O<H>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<O<H>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<O<H>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<O<H>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<O<H>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<O<H>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<O<H>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<O<H>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<O<H>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<O<H>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<O<H>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<O<H>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<O<H>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<O<H>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<O<H>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<O<H>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<O<H>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<O<H>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<O<H>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<O<H>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<O<H>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<O<H>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<O<H>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<O<H>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<O<H>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<O<H>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<O<H>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<I<H>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<I<H>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<I<H>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<I<H>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<I<H>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<I<H>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<I<H>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<I<H>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<I<H>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<I<H>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<I<H>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<I<H>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<I<H>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<I<H>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<I<H>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<I<H>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<I<H>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<I<H>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<I<H>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<I<H>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<I<H>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<I<H>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<I<H>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<I<H>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<I<H>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<I<H>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<I<H>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<I<H>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<I<H>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<I<H>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<I<H>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<I<H>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<O<O<H>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<O<O<H>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<O<O<H>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<O<O<H>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<O<O<H>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<O<O<H>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<O<O<H>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<O<O<H>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<O<O<H>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<O<O<H>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<O<O<H>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<O<O<H>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<O<O<H>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<O<O<H>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<O<O<H>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<O<O<H>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<O<O<H>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<O<O<H>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<O<O<H>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<O<O<H>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<O<O<H>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<O<O<H>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<O<O<H>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<O<O<H>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<O<O<H>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<O<O<H>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<O<O<H>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<O<O<H>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<O<O<H>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<O<O<H>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<O<O<H>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<O<O<H>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<I<O<H>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<I<O<H>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<I<O<H>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<I<O<H>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<I<O<H>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<I<O<H>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<I<O<H>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<I<O<H>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<I<O<H>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<I<O<H>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<I<O<H>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<I<O<H>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<I<O<H>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<I<O<H>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<I<O<H>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<I<O<H>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<I<O<H>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<I<O<H>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<I<O<H>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<I<O<H>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<I<O<H>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<I<O<H>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<I<O<H>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<I<O<H>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<I<O<H>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<I<O<H>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<I<O<H>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<I<O<H>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<I<O<H>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<I<O<H>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<I<O<H>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<I<O<H>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<O<I<H>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<O<I<H>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<O<I<H>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<O<I<H>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<O<I<H>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<O<I<H>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<O<I<H>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<O<I<H>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<O<I<H>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<O<I<H>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<O<I<H>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<O<I<H>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<O<I<H>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<O<I<H>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<O<I<H>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<O<I<H>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<O<I<H>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<O<I<H>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<O<I<H>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<O<I<H>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<O<I<H>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<O<I<H>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<O<I<H>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<O<I<H>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<O<I<H>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<O<I<H>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<O<I<H>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<O<I<H>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<O<I<H>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<O<I<H>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<O<I<H>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<O<I<H>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<I<I<H>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<I<I<H>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<I<I<H>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<I<I<H>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<I<I<H>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<I<I<H>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<I<I<H>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<I<I<H>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<I<I<H>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<I<I<H>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<I<I<H>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<I<I<H>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<I<I<H>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<I<I<H>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<I<I<H>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<I<I<H>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<I<I<H>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<I<I<H>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<I<I<H>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<I<I<H>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<I<I<H>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<I<I<H>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<I<I<H>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<I<I<H>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<I<I<H>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<I<I<H>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<I<I<H>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<I<I<H>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<I<I<H>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<I<I<H>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<I<I<H>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<I<I<H>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<O<O<O<H>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<O<O<O<H>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<O<O<O<H>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<O<O<O<H>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<O<O<O<H>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<O<O<O<H>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<O<O<O<H>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<O<O<O<H>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<O<O<O<H>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<O<O<O<H>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<O<O<O<H>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<O<O<O<H>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<O<O<O<H>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<O<O<O<H>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<O<O<O<H>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<O<O<O<H>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<O<O<O<H>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<O<O<O<H>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<O<O<O<H>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<O<O<O<H>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<O<O<O<H>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<O<O<O<H>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<O<O<O<H>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<O<O<O<H>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<O<O<O<H>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<O<O<O<H>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<O<O<O<H>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<O<O<O<H>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<O<O<O<H>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<O<O<O<H>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<O<O<O<H>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<O<O<O<H>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<I<O<O<H>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<I<O<O<H>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<I<O<O<H>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<I<O<O<H>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<I<O<O<H>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<I<O<O<H>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<I<O<O<H>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<I<O<O<H>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<I<O<O<H>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<I<O<O<H>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<I<O<O<H>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<I<O<O<H>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<I<O<O<H>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<I<O<O<H>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<I<O<O<H>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<I<O<O<H>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<I<O<O<H>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<I<O<O<H>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<I<O<O<H>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<I<O<O<H>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<I<O<O<H>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<I<O<O<H>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<I<O<O<H>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<I<O<O<H>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<I<O<O<H>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<I<O<O<H>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<I<O<O<H>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<I<O<O<H>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<I<O<O<H>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<I<O<O<H>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<I<O<O<H>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<I<O<O<H>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<O<I<O<H>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<O<I<O<H>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<O<I<O<H>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<O<I<O<H>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<O<I<O<H>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<O<I<O<H>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<O<I<O<H>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<O<I<O<H>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<O<I<O<H>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<O<I<O<H>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<O<I<O<H>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<O<I<O<H>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<O<I<O<H>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<O<I<O<H>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<O<I<O<H>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<O<I<O<H>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<O<I<O<H>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<O<I<O<H>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<O<I<O<H>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<O<I<O<H>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<O<I<O<H>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<O<I<O<H>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<O<I<O<H>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<O<I<O<H>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<O<I<O<H>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<O<I<O<H>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<O<I<O<H>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<O<I<O<H>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<O<I<O<H>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<O<I<O<H>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<O<I<O<H>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<O<I<O<H>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<I<I<O<H>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<I<I<O<H>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<I<I<O<H>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<I<I<O<H>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<I<I<O<H>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<I<I<O<H>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<I<I<O<H>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<I<I<O<H>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<I<I<O<H>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<I<I<O<H>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<I<I<O<H>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<I<I<O<H>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<I<I<O<H>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<I<I<O<H>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<I<I<O<H>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<I<I<O<H>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<I<I<O<H>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<I<I<O<H>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<I<I<O<H>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<I<I<O<H>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<I<I<O<H>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<I<I<O<H>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<I<I<O<H>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<I<I<O<H>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<I<I<O<H>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<I<I<O<H>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<I<I<O<H>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<I<I<O<H>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<I<I<O<H>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<I<I<O<H>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<I<I<O<H>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<I<I<O<H>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<O<O<I<H>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<O<O<I<H>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<O<O<I<H>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<O<O<I<H>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<O<O<I<H>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<O<O<I<H>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<O<O<I<H>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<O<O<I<H>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<O<O<I<H>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<O<O<I<H>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<O<O<I<H>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<O<O<I<H>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<O<O<I<H>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<O<O<I<H>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<O<O<I<H>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<O<O<I<H>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<O<O<I<H>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<O<O<I<H>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<O<O<I<H>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<O<O<I<H>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<O<O<I<H>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<O<O<I<H>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<O<O<I<H>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<O<O<I<H>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<O<O<I<H>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<O<O<I<H>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<O<O<I<H>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<O<O<I<H>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<O<O<I<H>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<O<O<I<H>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<O<O<I<H>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<O<O<I<H>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<I<O<I<H>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<I<O<I<H>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<I<O<I<H>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<I<O<I<H>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<I<O<I<H>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<I<O<I<H>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<I<O<I<H>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<I<O<I<H>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<I<O<I<H>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<I<O<I<H>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<I<O<I<H>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<I<O<I<H>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<I<O<I<H>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<I<O<I<H>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<I<O<I<H>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<I<O<I<H>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<I<O<I<H>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<I<O<I<H>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<I<O<I<H>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<I<O<I<H>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<I<O<I<H>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<I<O<I<H>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<I<O<I<H>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<I<O<I<H>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<I<O<I<H>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<I<O<I<H>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<I<O<I<H>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<I<O<I<H>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<I<O<I<H>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<I<O<I<H>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<I<O<I<H>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<I<O<I<H>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<O<I<I<H>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<O<I<I<H>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<O<I<I<H>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<O<I<I<H>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<O<I<I<H>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<O<I<I<H>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<O<I<I<H>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<O<I<I<H>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<O<I<I<H>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<O<I<I<H>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<O<I<I<H>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<O<I<I<H>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<O<I<I<H>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<O<I<I<H>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<O<I<I<H>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<O<I<I<H>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<O<I<I<H>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<O<I<I<H>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<O<I<I<H>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<O<I<I<H>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<O<I<I<H>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<O<I<I<H>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<O<I<I<H>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<O<I<I<H>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<O<I<I<H>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<O<I<I<H>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<O<I<I<H>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<O<I<I<H>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<O<I<I<H>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<O<I<I<H>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<O<I<I<H>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<O<I<I<H>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<I<I<I<H>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<I<I<I<H>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<I<I<I<H>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<I<I<I<H>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<I<I<I<H>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<I<I<I<H>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<I<I<I<H>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<I<I<I<H>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<I<I<I<H>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<I<I<I<H>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<I<I<I<H>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<I<I<I<H>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<I<I<I<H>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<I<I<I<H>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<I<I<I<H>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<I<I<I<H>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<I<I<I<H>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<I<I<I<H>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<I<I<I<H>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<I<I<I<H>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<I<I<I<H>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<I<I<I<H>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<I<I<I<H>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<I<I<I<H>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<I<I<I<H>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<I<I<I<H>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<I<I<I<H>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<I<I<I<H>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<I<I<I<H>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<I<I<I<H>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<I<I<I<H>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<I<I<I<H>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<O<O<O<O<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<O<O<O<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<O<O<O<O<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<O<O<O<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<O<O<O<O<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<O<O<O<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<O<O<O<O<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<O<O<O<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<O<O<O<O<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<O<O<O<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<O<O<O<O<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<O<O<O<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<O<O<O<O<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<O<O<O<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<O<O<O<O<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<O<O<O<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<O<O<O<O<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<O<O<O<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<O<O<O<O<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<O<O<O<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<O<O<O<O<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<O<O<O<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<O<O<O<O<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<O<O<O<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<O<O<O<O<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<O<O<O<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<O<O<O<O<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<O<O<O<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<O<O<O<O<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<O<O<O<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<O<O<O<O<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<O<O<O<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<I<O<O<O<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<I<O<O<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<I<O<O<O<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<I<O<O<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<I<O<O<O<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<I<O<O<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<I<O<O<O<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<I<O<O<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<I<O<O<O<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<I<O<O<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<I<O<O<O<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<I<O<O<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<I<O<O<O<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<I<O<O<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<I<O<O<O<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<I<O<O<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<I<O<O<O<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<I<O<O<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<I<O<O<O<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<I<O<O<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<I<O<O<O<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<I<O<O<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<I<O<O<O<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<I<O<O<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<I<O<O<O<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<I<O<O<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<I<O<O<O<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<I<O<O<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<I<O<O<O<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<I<O<O<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<I<O<O<O<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<I<O<O<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<O<I<O<O<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<O<I<O<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<O<I<O<O<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<O<I<O<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<O<I<O<O<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<O<I<O<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<O<I<O<O<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<O<I<O<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<O<I<O<O<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<O<I<O<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<O<I<O<O<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<O<I<O<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<O<I<O<O<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<O<I<O<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<O<I<O<O<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<O<I<O<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<O<I<O<O<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<O<I<O<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<O<I<O<O<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<O<I<O<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<O<I<O<O<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<O<I<O<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<O<I<O<O<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<O<I<O<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<O<I<O<O<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<O<I<O<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<O<I<O<O<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<O<I<O<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<O<I<O<O<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<O<I<O<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<O<I<O<O<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<O<I<O<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<I<I<O<O<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<I<I<O<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<I<I<O<O<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<I<I<O<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<I<I<O<O<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<I<I<O<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<I<I<O<O<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<I<I<O<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<I<I<O<O<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<I<I<O<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<I<I<O<O<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<I<I<O<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<I<I<O<O<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<I<I<O<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<I<I<O<O<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<I<I<O<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<I<I<O<O<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<I<I<O<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<I<I<O<O<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<I<I<O<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<I<I<O<O<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<I<I<O<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<I<I<O<O<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<I<I<O<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<I<I<O<O<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<I<I<O<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<I<I<O<O<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<I<I<O<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<I<I<O<O<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<I<I<O<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<I<I<O<O<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<I<I<O<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<O<O<I<O<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<O<O<I<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<O<O<I<O<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<O<O<I<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<O<O<I<O<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<O<O<I<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<O<O<I<O<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<O<O<I<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<O<O<I<O<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<O<O<I<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<O<O<I<O<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<O<O<I<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<O<O<I<O<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<O<O<I<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<O<O<I<O<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<O<O<I<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<O<O<I<O<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<O<O<I<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<O<O<I<O<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<O<O<I<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<O<O<I<O<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<O<O<I<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<O<O<I<O<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<O<O<I<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<O<O<I<O<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<O<O<I<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<O<O<I<O<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<O<O<I<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<O<O<I<O<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<O<O<I<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<O<O<I<O<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<O<O<I<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<I<O<I<O<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<I<O<I<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<I<O<I<O<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<I<O<I<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<I<O<I<O<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<I<O<I<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<I<O<I<O<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<I<O<I<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<I<O<I<O<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<I<O<I<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<I<O<I<O<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<I<O<I<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<I<O<I<O<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<I<O<I<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<I<O<I<O<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<I<O<I<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<I<O<I<O<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<I<O<I<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<I<O<I<O<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<I<O<I<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<I<O<I<O<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<I<O<I<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<I<O<I<O<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<I<O<I<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<I<O<I<O<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<I<O<I<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<I<O<I<O<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<I<O<I<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<I<O<I<O<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<I<O<I<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<I<O<I<O<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<I<O<I<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<O<I<I<O<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<O<I<I<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<O<I<I<O<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<O<I<I<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<O<I<I<O<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<O<I<I<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<O<I<I<O<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<O<I<I<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<O<I<I<O<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<O<I<I<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<O<I<I<O<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<O<I<I<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<O<I<I<O<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<O<I<I<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<O<I<I<O<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<O<I<I<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<O<I<I<O<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<O<I<I<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<O<I<I<O<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<O<I<I<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<O<I<I<O<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<O<I<I<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<O<I<I<O<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<O<I<I<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<O<I<I<O<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<O<I<I<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<O<I<I<O<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<O<I<I<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<O<I<I<O<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<O<I<I<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<O<I<I<O<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<O<I<I<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<I<I<I<O<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<I<I<I<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<I<I<I<O<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<I<I<I<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<I<I<I<O<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<I<I<I<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<I<I<I<O<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<I<I<I<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<I<I<I<O<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<I<I<I<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<I<I<I<O<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<I<I<I<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<I<I<I<O<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<I<I<I<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<I<I<I<O<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<I<I<I<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<I<I<I<O<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<I<I<I<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<I<I<I<O<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<I<I<I<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<I<I<I<O<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<I<I<I<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<I<I<I<O<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<I<I<I<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<I<I<I<O<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<I<I<I<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<I<I<I<O<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<I<I<I<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<I<I<I<O<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<I<I<I<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<I<I<I<O<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<I<I<I<O<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<O<O<O<I<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<O<O<O<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<O<O<O<I<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<O<O<O<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<O<O<O<I<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<O<O<O<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<O<O<O<I<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<O<O<O<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<O<O<O<I<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<O<O<O<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<O<O<O<I<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<O<O<O<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<O<O<O<I<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<O<O<O<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<O<O<O<I<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<O<O<O<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<O<O<O<I<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<O<O<O<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<O<O<O<I<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<O<O<O<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<O<O<O<I<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<O<O<O<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<O<O<O<I<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<O<O<O<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<O<O<O<I<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<O<O<O<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<O<O<O<I<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<O<O<O<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<O<O<O<I<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<O<O<O<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<O<O<O<I<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<O<O<O<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<I<O<O<I<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<I<O<O<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<I<O<O<I<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<I<O<O<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<I<O<O<I<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<I<O<O<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<I<O<O<I<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<I<O<O<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<I<O<O<I<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<I<O<O<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<I<O<O<I<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<I<O<O<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<I<O<O<I<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<I<O<O<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<I<O<O<I<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<I<O<O<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<I<O<O<I<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<I<O<O<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<I<O<O<I<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<I<O<O<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<I<O<O<I<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<I<O<O<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<I<O<O<I<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<I<O<O<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<I<O<O<I<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<I<O<O<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<I<O<O<I<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<I<O<O<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<I<O<O<I<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<I<O<O<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<I<O<O<I<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<I<O<O<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<O<I<O<I<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<O<I<O<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<O<I<O<I<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<O<I<O<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<O<I<O<I<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<O<I<O<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<O<I<O<I<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<O<I<O<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<O<I<O<I<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<O<I<O<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<O<I<O<I<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<O<I<O<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<O<I<O<I<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<O<I<O<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<O<I<O<I<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<O<I<O<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<O<I<O<I<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<O<I<O<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<O<I<O<I<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<O<I<O<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<O<I<O<I<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<O<I<O<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<O<I<O<I<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<O<I<O<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<O<I<O<I<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<O<I<O<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<O<I<O<I<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<O<I<O<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<O<I<O<I<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<O<I<O<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<O<I<O<I<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<O<I<O<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<I<I<O<I<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<I<I<O<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<I<I<O<I<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<I<I<O<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<I<I<O<I<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<I<I<O<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<I<I<O<I<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<I<I<O<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<I<I<O<I<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<I<I<O<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<I<I<O<I<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<I<I<O<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<I<I<O<I<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<I<I<O<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<I<I<O<I<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<I<I<O<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<I<I<O<I<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<I<I<O<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<I<I<O<I<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<I<I<O<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<I<I<O<I<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<I<I<O<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<I<I<O<I<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<I<I<O<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<I<I<O<I<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<I<I<O<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<I<I<O<I<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<I<I<O<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<I<I<O<I<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<I<I<O<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<I<I<O<I<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<I<I<O<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<O<O<I<I<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<O<O<I<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<O<O<I<I<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<O<O<I<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<O<O<I<I<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<O<O<I<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<O<O<I<I<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<O<O<I<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<O<O<I<I<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<O<O<I<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<O<O<I<I<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<O<O<I<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<O<O<I<I<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<O<O<I<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<O<O<I<I<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<O<O<I<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<O<O<I<I<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<O<O<I<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<O<O<I<I<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<O<O<I<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<O<O<I<I<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<O<O<I<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<O<O<I<I<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<O<O<I<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<O<O<I<I<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<O<O<I<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<O<O<I<I<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<O<O<I<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<O<O<I<I<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<O<O<I<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<O<O<I<I<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<O<O<I<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<I<O<I<I<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<I<O<I<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<I<O<I<I<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<I<O<I<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<I<O<I<I<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<I<O<I<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<I<O<I<I<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<I<O<I<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<I<O<I<I<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<I<O<I<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<I<O<I<I<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<I<O<I<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<I<O<I<I<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<I<O<I<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<I<O<I<I<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<I<O<I<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<I<O<I<I<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<I<O<I<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<I<O<I<I<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<I<O<I<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<I<O<I<I<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<I<O<I<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<I<O<I<I<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<I<O<I<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<I<O<I<I<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<I<O<I<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<I<O<I<I<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<I<O<I<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<I<O<I<I<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<I<O<I<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<I<O<I<I<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<I<O<I<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<O<I<I<I<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<O<I<I<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<O<I<I<I<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<O<I<I<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<O<I<I<I<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<O<I<I<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<O<I<I<I<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<O<I<I<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<O<I<I<I<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<O<I<I<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<O<I<I<I<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<O<I<I<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<O<I<I<I<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<O<I<I<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<O<I<I<I<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<O<I<I<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<O<I<I<I<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<O<I<I<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<O<I<I<I<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<O<I<I<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<O<I<I<I<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<O<I<I<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<O<I<I<I<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<O<I<I<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<O<I<I<I<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<O<I<I<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<O<I<I<I<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<O<I<I<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<O<I<I<I<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<O<I<I<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<O<I<I<I<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<O<I<I<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<I<I<I<I<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<I<I<I<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<I<I<I<I<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<I<I<I<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<I<I<I<I<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<I<I<I<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<I<I<I<I<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<I<I<I<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<I<I<I<I<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<I<I<I<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<I<I<I<I<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<I<I<I<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<I<I<I<I<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<I<I<I<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<I<I<I<I<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<I<I<I<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<I<I<I<I<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<I<I<I<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<I<I<I<I<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<I<I<I<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<I<I<I<I<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<I<I<I<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<I<I<I<I<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<I<I<I<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<I<I<I<I<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<I<I<I<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<I<I<I<I<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<I<I<I<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<I<I<I<I<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<I<I<I<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<I<I<I<I<H>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<I<I<I<I<H>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<O<O<O<O<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<O<O<O<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<O<O<O<O<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<O<O<O<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<O<O<O<O<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<O<O<O<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<O<O<O<O<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<O<O<O<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<O<O<O<O<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<O<O<O<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<O<O<O<O<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<O<O<O<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<O<O<O<O<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<O<O<O<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<O<O<O<O<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<O<O<O<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<O<O<O<O<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<O<O<O<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<O<O<O<O<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<O<O<O<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<O<O<O<O<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<O<O<O<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<O<O<O<O<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<O<O<O<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<O<O<O<O<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<O<O<O<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<O<O<O<O<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<O<O<O<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<O<O<O<O<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<O<O<O<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<O<O<O<O<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<O<O<O<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<I<O<O<O<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<I<O<O<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<I<O<O<O<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<I<O<O<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<I<O<O<O<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<I<O<O<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<I<O<O<O<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<I<O<O<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<I<O<O<O<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<I<O<O<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<I<O<O<O<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<I<O<O<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<I<O<O<O<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<I<O<O<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<I<O<O<O<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<I<O<O<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<I<O<O<O<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<I<O<O<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<I<O<O<O<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<I<O<O<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<I<O<O<O<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<I<O<O<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<I<O<O<O<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<I<O<O<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<I<O<O<O<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<I<O<O<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<I<O<O<O<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<I<O<O<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<I<O<O<O<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<I<O<O<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<I<O<O<O<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<I<O<O<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<O<I<O<O<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<O<I<O<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<O<I<O<O<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<O<I<O<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<O<I<O<O<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<O<I<O<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<O<I<O<O<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<O<I<O<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<O<I<O<O<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<O<I<O<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<O<I<O<O<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<O<I<O<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<O<I<O<O<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<O<I<O<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<O<I<O<O<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<O<I<O<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<O<I<O<O<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<O<I<O<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<O<I<O<O<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<O<I<O<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<O<I<O<O<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<O<I<O<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<O<I<O<O<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<O<I<O<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<O<I<O<O<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<O<I<O<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<O<I<O<O<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<O<I<O<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<O<I<O<O<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<O<I<O<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<O<I<O<O<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<O<I<O<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<I<I<O<O<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<I<I<O<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<I<I<O<O<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<I<I<O<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<I<I<O<O<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<I<I<O<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<I<I<O<O<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<I<I<O<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<I<I<O<O<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<I<I<O<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<I<I<O<O<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<I<I<O<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<I<I<O<O<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<I<I<O<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<I<I<O<O<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<I<I<O<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<I<I<O<O<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<I<I<O<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<I<I<O<O<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<I<I<O<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<I<I<O<O<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<I<I<O<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<I<I<O<O<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<I<I<O<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<I<I<O<O<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<I<I<O<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<I<I<O<O<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<I<I<O<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<I<I<O<O<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<I<I<O<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<I<I<O<O<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<I<I<O<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<O<O<I<O<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<O<O<I<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<O<O<I<O<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<O<O<I<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<O<O<I<O<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<O<O<I<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<O<O<I<O<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<O<O<I<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<O<O<I<O<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<O<O<I<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<O<O<I<O<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<O<O<I<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<O<O<I<O<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<O<O<I<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<O<O<I<O<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<O<O<I<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<O<O<I<O<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<O<O<I<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<O<O<I<O<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<O<O<I<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<O<O<I<O<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<O<O<I<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<O<O<I<O<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<O<O<I<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<O<O<I<O<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<O<O<I<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<O<O<I<O<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<O<O<I<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<O<O<I<O<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<O<O<I<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<O<O<I<O<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<O<O<I<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<I<O<I<O<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<I<O<I<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<I<O<I<O<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<I<O<I<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<I<O<I<O<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<I<O<I<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<I<O<I<O<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<I<O<I<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<I<O<I<O<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<I<O<I<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<I<O<I<O<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<I<O<I<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<I<O<I<O<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<I<O<I<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<I<O<I<O<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<I<O<I<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<I<O<I<O<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<I<O<I<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<I<O<I<O<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<I<O<I<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<I<O<I<O<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<I<O<I<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<I<O<I<O<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<I<O<I<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<I<O<I<O<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<I<O<I<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<I<O<I<O<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<I<O<I<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<I<O<I<O<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<I<O<I<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<I<O<I<O<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<I<O<I<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<O<I<I<O<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<O<I<I<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<O<I<I<O<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<O<I<I<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<O<I<I<O<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<O<I<I<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<O<I<I<O<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<O<I<I<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<O<I<I<O<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<O<I<I<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<O<I<I<O<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<O<I<I<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<O<I<I<O<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<O<I<I<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<O<I<I<O<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<O<I<I<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<O<I<I<O<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<O<I<I<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<O<I<I<O<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<O<I<I<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<O<I<I<O<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<O<I<I<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<O<I<I<O<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<O<I<I<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<O<I<I<O<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<O<I<I<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<O<I<I<O<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<O<I<I<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<O<I<I<O<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<O<I<I<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<O<I<I<O<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<O<I<I<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<I<I<I<O<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<I<I<I<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<I<I<I<O<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<I<I<I<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<I<I<I<O<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<I<I<I<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<I<I<I<O<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<I<I<I<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<I<I<I<O<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<I<I<I<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<I<I<I<O<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<I<I<I<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<I<I<I<O<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<I<I<I<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<I<I<I<O<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<I<I<I<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<I<I<I<O<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<I<I<I<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<I<I<I<O<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<I<I<I<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<I<I<I<O<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<I<I<I<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<I<I<I<O<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<I<I<I<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<I<I<I<O<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<I<I<I<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<I<I<I<O<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<I<I<I<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<I<I<I<O<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<I<I<I<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<I<I<I<O<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<I<I<I<O<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<O<O<O<I<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<O<O<O<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<O<O<O<I<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<O<O<O<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<O<O<O<I<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<O<O<O<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<O<O<O<I<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<O<O<O<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<O<O<O<I<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<O<O<O<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<O<O<O<I<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<O<O<O<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<O<O<O<I<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<O<O<O<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<O<O<O<I<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<O<O<O<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<O<O<O<I<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<O<O<O<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<O<O<O<I<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<O<O<O<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<O<O<O<I<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<O<O<O<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<O<O<O<I<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<O<O<O<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<O<O<O<I<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<O<O<O<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<O<O<O<I<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<O<O<O<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<O<O<O<I<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<O<O<O<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<O<O<O<I<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<O<O<O<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<I<O<O<I<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<I<O<O<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<I<O<O<I<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<I<O<O<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<I<O<O<I<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<I<O<O<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<I<O<O<I<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<I<O<O<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<I<O<O<I<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<I<O<O<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<I<O<O<I<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<I<O<O<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<I<O<O<I<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<I<O<O<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<I<O<O<I<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<I<O<O<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<I<O<O<I<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<I<O<O<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<I<O<O<I<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<I<O<O<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<I<O<O<I<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<I<O<O<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<I<O<O<I<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<I<O<O<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<I<O<O<I<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<I<O<O<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<I<O<O<I<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<I<O<O<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<I<O<O<I<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<I<O<O<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<I<O<O<I<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<I<O<O<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<O<I<O<I<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<O<I<O<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<O<I<O<I<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<O<I<O<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<O<I<O<I<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<O<I<O<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<O<I<O<I<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<O<I<O<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<O<I<O<I<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<O<I<O<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<O<I<O<I<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<O<I<O<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<O<I<O<I<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<O<I<O<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<O<I<O<I<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<O<I<O<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<O<I<O<I<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<O<I<O<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<O<I<O<I<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<O<I<O<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<O<I<O<I<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<O<I<O<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<O<I<O<I<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<O<I<O<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<O<I<O<I<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<O<I<O<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<O<I<O<I<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<O<I<O<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<O<I<O<I<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<O<I<O<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<O<I<O<I<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<O<I<O<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<I<I<O<I<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<I<I<O<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<I<I<O<I<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<I<I<O<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<I<I<O<I<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<I<I<O<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<I<I<O<I<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<I<I<O<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<I<I<O<I<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<I<I<O<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<I<I<O<I<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<I<I<O<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<I<I<O<I<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<I<I<O<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<I<I<O<I<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<I<I<O<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<I<I<O<I<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<I<I<O<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<I<I<O<I<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<I<I<O<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<I<I<O<I<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<I<I<O<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<I<I<O<I<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<I<I<O<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<I<I<O<I<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<I<I<O<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<I<I<O<I<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<I<I<O<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<I<I<O<I<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<I<I<O<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<I<I<O<I<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<I<I<O<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<O<O<I<I<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<O<O<I<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<O<O<I<I<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<O<O<I<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<O<O<I<I<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<O<O<I<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<O<O<I<I<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<O<O<I<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<O<O<I<I<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<O<O<I<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<O<O<I<I<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<O<O<I<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<O<O<I<I<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<O<O<I<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<O<O<I<I<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<O<O<I<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<O<O<I<I<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<O<O<I<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<O<O<I<I<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<O<O<I<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<O<O<I<I<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<O<O<I<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<O<O<I<I<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<O<O<I<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<O<O<I<I<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<O<O<I<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<O<O<I<I<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<O<O<I<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<O<O<I<I<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<O<O<I<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<O<O<I<I<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<O<O<I<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<I<O<I<I<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<I<O<I<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<I<O<I<I<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<I<O<I<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<I<O<I<I<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<I<O<I<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<I<O<I<I<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<I<O<I<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<I<O<I<I<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<I<O<I<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<I<O<I<I<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<I<O<I<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<I<O<I<I<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<I<O<I<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<I<O<I<I<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<I<O<I<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<I<O<I<I<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<I<O<I<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<I<O<I<I<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<I<O<I<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<I<O<I<I<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<I<O<I<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<I<O<I<I<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<I<O<I<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<I<O<I<I<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<I<O<I<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<I<O<I<I<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<I<O<I<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<I<O<I<I<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<I<O<I<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<I<O<I<I<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<I<O<I<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<O<I<I<I<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<O<I<I<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<O<I<I<I<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<O<I<I<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<O<I<I<I<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<O<I<I<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<O<I<I<I<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<O<I<I<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<O<I<I<I<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<O<I<I<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<O<I<I<I<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<O<I<I<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<O<I<I<I<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<O<I<I<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<O<I<I<I<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<O<I<I<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<O<I<I<I<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<O<I<I<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<O<I<I<I<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<O<I<I<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<O<I<I<I<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<O<I<I<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<O<I<I<I<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<O<I<I<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<O<I<I<I<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<O<I<I<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<O<I<I<I<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<O<I<I<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<O<I<I<I<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<O<I<I<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<O<I<I<I<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<O<I<I<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<I<I<I<I<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<I<I<I<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<I<I<I<I<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<I<I<I<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<I<I<I<I<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<I<I<I<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<I<I<I<I<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<I<I<I<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<I<I<I<I<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<I<I<I<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<I<I<I<I<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<I<I<I<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<I<I<I<I<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<I<I<I<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<I<I<I<I<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<I<I<I<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<I<I<I<I<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<I<I<I<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<I<I<I<I<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<I<I<I<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<I<I<I<I<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<I<I<I<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<I<I<I<I<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<I<I<I<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<I<I<I<I<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<I<I<I<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<I<I<I<I<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<I<I<I<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<I<I<I<I<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<I<I<I<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<I<I<I<I<O<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<I<I<I<I<O<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<O<O<O<O<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<O<O<O<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<O<O<O<O<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<O<O<O<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<O<O<O<O<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<O<O<O<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<O<O<O<O<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<O<O<O<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<O<O<O<O<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<O<O<O<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<O<O<O<O<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<O<O<O<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<O<O<O<O<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<O<O<O<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<O<O<O<O<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<O<O<O<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<O<O<O<O<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<O<O<O<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<O<O<O<O<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<O<O<O<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<O<O<O<O<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<O<O<O<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<O<O<O<O<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<O<O<O<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<O<O<O<O<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<O<O<O<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<O<O<O<O<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<O<O<O<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<O<O<O<O<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<O<O<O<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<O<O<O<O<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<O<O<O<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<I<O<O<O<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<I<O<O<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<I<O<O<O<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<I<O<O<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<I<O<O<O<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<I<O<O<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<I<O<O<O<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<I<O<O<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<I<O<O<O<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<I<O<O<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<I<O<O<O<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<I<O<O<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<I<O<O<O<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<I<O<O<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<I<O<O<O<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<I<O<O<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<I<O<O<O<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<I<O<O<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<I<O<O<O<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<I<O<O<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<I<O<O<O<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<I<O<O<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<I<O<O<O<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<I<O<O<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<I<O<O<O<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<I<O<O<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<I<O<O<O<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<I<O<O<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<I<O<O<O<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<I<O<O<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<I<O<O<O<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<I<O<O<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<O<I<O<O<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<O<I<O<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<O<I<O<O<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<O<I<O<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<O<I<O<O<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<O<I<O<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<O<I<O<O<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<O<I<O<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<O<I<O<O<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<O<I<O<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<O<I<O<O<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<O<I<O<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<O<I<O<O<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<O<I<O<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<O<I<O<O<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<O<I<O<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<O<I<O<O<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<O<I<O<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<O<I<O<O<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<O<I<O<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<O<I<O<O<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<O<I<O<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<O<I<O<O<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<O<I<O<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<O<I<O<O<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<O<I<O<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<O<I<O<O<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<O<I<O<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<O<I<O<O<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<O<I<O<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<O<I<O<O<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<O<I<O<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<I<I<O<O<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<I<I<O<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<I<I<O<O<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<I<I<O<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<I<I<O<O<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<I<I<O<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<I<I<O<O<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<I<I<O<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<I<I<O<O<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<I<I<O<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<I<I<O<O<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<I<I<O<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<I<I<O<O<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<I<I<O<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<I<I<O<O<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<I<I<O<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<I<I<O<O<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<I<I<O<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<I<I<O<O<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<I<I<O<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<I<I<O<O<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<I<I<O<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<I<I<O<O<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<I<I<O<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<I<I<O<O<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<I<I<O<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<I<I<O<O<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<I<I<O<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<I<I<O<O<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<I<I<O<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<I<I<O<O<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<I<I<O<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<O<O<I<O<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<O<O<I<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<O<O<I<O<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<O<O<I<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<O<O<I<O<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<O<O<I<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<O<O<I<O<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<O<O<I<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<O<O<I<O<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<O<O<I<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<O<O<I<O<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<O<O<I<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<O<O<I<O<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<O<O<I<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<O<O<I<O<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<O<O<I<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<O<O<I<O<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<O<O<I<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<O<O<I<O<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<O<O<I<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<O<O<I<O<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<O<O<I<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<O<O<I<O<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<O<O<I<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<O<O<I<O<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<O<O<I<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<O<O<I<O<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<O<O<I<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<O<O<I<O<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<O<O<I<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<O<O<I<O<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<O<O<I<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<I<O<I<O<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<I<O<I<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<I<O<I<O<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<I<O<I<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<I<O<I<O<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<I<O<I<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<I<O<I<O<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<I<O<I<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<I<O<I<O<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<I<O<I<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<I<O<I<O<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<I<O<I<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<I<O<I<O<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<I<O<I<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<I<O<I<O<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<I<O<I<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<I<O<I<O<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<I<O<I<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<I<O<I<O<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<I<O<I<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<I<O<I<O<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<I<O<I<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<I<O<I<O<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<I<O<I<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<I<O<I<O<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<I<O<I<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<I<O<I<O<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<I<O<I<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<I<O<I<O<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<I<O<I<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<I<O<I<O<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<I<O<I<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<O<I<I<O<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<O<I<I<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<O<I<I<O<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<O<I<I<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<O<I<I<O<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<O<I<I<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<O<I<I<O<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<O<I<I<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<O<I<I<O<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<O<I<I<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<O<I<I<O<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<O<I<I<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<O<I<I<O<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<O<I<I<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<O<I<I<O<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<O<I<I<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<O<I<I<O<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<O<I<I<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<O<I<I<O<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<O<I<I<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<O<I<I<O<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<O<I<I<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<O<I<I<O<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<O<I<I<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<O<I<I<O<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<O<I<I<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<O<I<I<O<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<O<I<I<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<O<I<I<O<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<O<I<I<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<O<I<I<O<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<O<I<I<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<I<I<I<O<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<I<I<I<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<I<I<I<O<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<I<I<I<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<I<I<I<O<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<I<I<I<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<I<I<I<O<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<I<I<I<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<I<I<I<O<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<I<I<I<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<I<I<I<O<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<I<I<I<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<I<I<I<O<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<I<I<I<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<I<I<I<O<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<I<I<I<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<I<I<I<O<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<I<I<I<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<I<I<I<O<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<I<I<I<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<I<I<I<O<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<I<I<I<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<I<I<I<O<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<I<I<I<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<I<I<I<O<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<I<I<I<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<I<I<I<O<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<I<I<I<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<I<I<I<O<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<I<I<I<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<I<I<I<O<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<I<I<I<O<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<O<O<O<I<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<O<O<O<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<O<O<O<I<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<O<O<O<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<O<O<O<I<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<O<O<O<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<O<O<O<I<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<O<O<O<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<O<O<O<I<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<O<O<O<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<O<O<O<I<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<O<O<O<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<O<O<O<I<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<O<O<O<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<O<O<O<I<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<O<O<O<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<O<O<O<I<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<O<O<O<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<O<O<O<I<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<O<O<O<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<O<O<O<I<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<O<O<O<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<O<O<O<I<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<O<O<O<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<O<O<O<I<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<O<O<O<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<O<O<O<I<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<O<O<O<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<O<O<O<I<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<O<O<O<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<O<O<O<I<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<O<O<O<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<I<O<O<I<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<I<O<O<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<I<O<O<I<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<I<O<O<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<I<O<O<I<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<I<O<O<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<I<O<O<I<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<I<O<O<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<I<O<O<I<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<I<O<O<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<I<O<O<I<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<I<O<O<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<I<O<O<I<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<I<O<O<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<I<O<O<I<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<I<O<O<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<I<O<O<I<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<I<O<O<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<I<O<O<I<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<I<O<O<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<I<O<O<I<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<I<O<O<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<I<O<O<I<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<I<O<O<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<I<O<O<I<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<I<O<O<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<I<O<O<I<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<I<O<O<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<I<O<O<I<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<I<O<O<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<I<O<O<I<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<I<O<O<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<O<I<O<I<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<O<I<O<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<O<I<O<I<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<O<I<O<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<O<I<O<I<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<O<I<O<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<O<I<O<I<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<O<I<O<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<O<I<O<I<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<O<I<O<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<O<I<O<I<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<O<I<O<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<O<I<O<I<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<O<I<O<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<O<I<O<I<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<O<I<O<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<O<I<O<I<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<O<I<O<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<O<I<O<I<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<O<I<O<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<O<I<O<I<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<O<I<O<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<O<I<O<I<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<O<I<O<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<O<I<O<I<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<O<I<O<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<O<I<O<I<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<O<I<O<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<O<I<O<I<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<O<I<O<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<O<I<O<I<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<O<I<O<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<I<I<O<I<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<I<I<O<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<I<I<O<I<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<I<I<O<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<I<I<O<I<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<I<I<O<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<I<I<O<I<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<I<I<O<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<I<I<O<I<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<I<I<O<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<I<I<O<I<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<I<I<O<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<I<I<O<I<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<I<I<O<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<I<I<O<I<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<I<I<O<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<I<I<O<I<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<I<I<O<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<I<I<O<I<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<I<I<O<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<I<I<O<I<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<I<I<O<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<I<I<O<I<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<I<I<O<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<I<I<O<I<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<I<I<O<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<I<I<O<I<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<I<I<O<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<I<I<O<I<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<I<I<O<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<I<I<O<I<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<I<I<O<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<O<O<I<I<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<O<O<I<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<O<O<I<I<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<O<O<I<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<O<O<I<I<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<O<O<I<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<O<O<I<I<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<O<O<I<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<O<O<I<I<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<O<O<I<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<O<O<I<I<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<O<O<I<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<O<O<I<I<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<O<O<I<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<O<O<I<I<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<O<O<I<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<O<O<I<I<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<O<O<I<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<O<O<I<I<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<O<O<I<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<O<O<I<I<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<O<O<I<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<O<O<I<I<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<O<O<I<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<O<O<I<I<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<O<O<I<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<O<O<I<I<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<O<O<I<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<O<O<I<I<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<O<O<I<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<O<O<I<I<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<O<O<I<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<I<O<I<I<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<I<O<I<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<I<O<I<I<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<I<O<I<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<I<O<I<I<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<I<O<I<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<I<O<I<I<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<I<O<I<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<I<O<I<I<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<I<O<I<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<I<O<I<I<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<I<O<I<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<I<O<I<I<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<I<O<I<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<I<O<I<I<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<I<O<I<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<I<O<I<I<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<I<O<I<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<I<O<I<I<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<I<O<I<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<I<O<I<I<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<I<O<I<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<I<O<I<I<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<I<O<I<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<I<O<I<I<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<I<O<I<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<I<O<I<I<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<I<O<I<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<I<O<I<I<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<I<O<I<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<I<O<I<I<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<I<O<I<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<O<I<I<I<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<O<I<I<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<O<I<I<I<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<O<I<I<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<O<I<I<I<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<O<I<I<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<O<I<I<I<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<O<I<I<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<O<I<I<I<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<O<I<I<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<O<I<I<I<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<O<I<I<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<O<I<I<I<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<O<I<I<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<O<I<I<I<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<O<I<I<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<O<I<I<I<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<O<I<I<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<O<I<I<I<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<O<I<I<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<O<I<I<I<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<O<I<I<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<O<I<I<I<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<O<I<I<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<O<I<I<I<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<O<I<I<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<O<I<I<I<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<O<I<I<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<O<I<I<I<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<O<I<I<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<O<I<I<I<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<O<I<I<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<I<I<I<I<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<I<I<I<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<I<I<I<I<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<I<I<I<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<I<I<I<I<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<I<I<I<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<I<I<I<I<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<I<I<I<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<I<I<I<I<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<I<I<I<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<I<I<I<I<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<I<I<I<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<I<I<I<I<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<I<I<I<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<I<I<I<I<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<I<I<I<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<I<I<I<I<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<I<I<I<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<I<I<I<I<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<I<I<I<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<I<I<I<I<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<I<I<I<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<I<I<I<I<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<I<I<I<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<I<I<I<I<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<I<I<I<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<I<I<I<I<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<I<I<I<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<I<I<I<I<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<I<I<I<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<I<I<I<I<I<H>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<I<I<I<I<I<H>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<O<O<O<O<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<O<O<O<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<O<O<O<O<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<O<O<O<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<O<O<O<O<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<O<O<O<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<O<O<O<O<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<O<O<O<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<O<O<O<O<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<O<O<O<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<O<O<O<O<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<O<O<O<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<O<O<O<O<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<O<O<O<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<O<O<O<O<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<O<O<O<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<O<O<O<O<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<O<O<O<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<O<O<O<O<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<O<O<O<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<O<O<O<O<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<O<O<O<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<O<O<O<O<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<O<O<O<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<O<O<O<O<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<O<O<O<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<O<O<O<O<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<O<O<O<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<O<O<O<O<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<O<O<O<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<O<O<O<O<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<O<O<O<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<I<O<O<O<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<I<O<O<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<I<O<O<O<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<I<O<O<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<I<O<O<O<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<I<O<O<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<I<O<O<O<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<I<O<O<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<I<O<O<O<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<I<O<O<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<I<O<O<O<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<I<O<O<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<I<O<O<O<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<I<O<O<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<I<O<O<O<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<I<O<O<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<I<O<O<O<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<I<O<O<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<I<O<O<O<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<I<O<O<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<I<O<O<O<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<I<O<O<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<I<O<O<O<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<I<O<O<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<I<O<O<O<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<I<O<O<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<I<O<O<O<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<I<O<O<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<I<O<O<O<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<I<O<O<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<I<O<O<O<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<I<O<O<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<O<I<O<O<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<O<I<O<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<O<I<O<O<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<O<I<O<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<O<I<O<O<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<O<I<O<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<O<I<O<O<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<O<I<O<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<O<I<O<O<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<O<I<O<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<O<I<O<O<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<O<I<O<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<O<I<O<O<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<O<I<O<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<O<I<O<O<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<O<I<O<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<O<I<O<O<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<O<I<O<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<O<I<O<O<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<O<I<O<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<O<I<O<O<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<O<I<O<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<O<I<O<O<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<O<I<O<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<O<I<O<O<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<O<I<O<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<O<I<O<O<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<O<I<O<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<O<I<O<O<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<O<I<O<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<O<I<O<O<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<O<I<O<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<I<I<O<O<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<I<I<O<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<I<I<O<O<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<I<I<O<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<I<I<O<O<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<I<I<O<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<I<I<O<O<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<I<I<O<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<I<I<O<O<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<I<I<O<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<I<I<O<O<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<I<I<O<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<I<I<O<O<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<I<I<O<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<I<I<O<O<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<I<I<O<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<I<I<O<O<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<I<I<O<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<I<I<O<O<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<I<I<O<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<I<I<O<O<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<I<I<O<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<I<I<O<O<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<I<I<O<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<I<I<O<O<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<I<I<O<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<I<I<O<O<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<I<I<O<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<I<I<O<O<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<I<I<O<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<I<I<O<O<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<I<I<O<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<O<O<I<O<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<O<O<I<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<O<O<I<O<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<O<O<I<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<O<O<I<O<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<O<O<I<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<O<O<I<O<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<O<O<I<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<O<O<I<O<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<O<O<I<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<O<O<I<O<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<O<O<I<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<O<O<I<O<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<O<O<I<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<O<O<I<O<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<O<O<I<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<O<O<I<O<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<O<O<I<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<O<O<I<O<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<O<O<I<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<O<O<I<O<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<O<O<I<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<O<O<I<O<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<O<O<I<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<O<O<I<O<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<O<O<I<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<O<O<I<O<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<O<O<I<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<O<O<I<O<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<O<O<I<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<O<O<I<O<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<O<O<I<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<I<O<I<O<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<I<O<I<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<I<O<I<O<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<I<O<I<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<I<O<I<O<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<I<O<I<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<I<O<I<O<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<I<O<I<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<I<O<I<O<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<I<O<I<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<I<O<I<O<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<I<O<I<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<I<O<I<O<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<I<O<I<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<I<O<I<O<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<I<O<I<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<I<O<I<O<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<I<O<I<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<I<O<I<O<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<I<O<I<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<I<O<I<O<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<I<O<I<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<I<O<I<O<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<I<O<I<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<I<O<I<O<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<I<O<I<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<I<O<I<O<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<I<O<I<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<I<O<I<O<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<I<O<I<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<I<O<I<O<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<I<O<I<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<O<I<I<O<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<O<I<I<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<O<I<I<O<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<O<I<I<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<O<I<I<O<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<O<I<I<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<O<I<I<O<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<O<I<I<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<O<I<I<O<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<O<I<I<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<O<I<I<O<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<O<I<I<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<O<I<I<O<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<O<I<I<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<O<I<I<O<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<O<I<I<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<O<I<I<O<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<O<I<I<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<O<I<I<O<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<O<I<I<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<O<I<I<O<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<O<I<I<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<O<I<I<O<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<O<I<I<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<O<I<I<O<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<O<I<I<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<O<I<I<O<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<O<I<I<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<O<I<I<O<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<O<I<I<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<O<I<I<O<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<O<I<I<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<I<I<I<O<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<I<I<I<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<I<I<I<O<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<I<I<I<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<I<I<I<O<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<I<I<I<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<I<I<I<O<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<I<I<I<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<I<I<I<O<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<I<I<I<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<I<I<I<O<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<I<I<I<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<I<I<I<O<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<I<I<I<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<I<I<I<O<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<I<I<I<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<I<I<I<O<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<I<I<I<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<I<I<I<O<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<I<I<I<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<I<I<I<O<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<I<I<I<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<I<I<I<O<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<I<I<I<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<I<I<I<O<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<I<I<I<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<I<I<I<O<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<I<I<I<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<I<I<I<O<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<I<I<I<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<I<I<I<O<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<I<I<I<O<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<O<O<O<I<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<O<O<O<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<O<O<O<I<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<O<O<O<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<O<O<O<I<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<O<O<O<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<O<O<O<I<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<O<O<O<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<O<O<O<I<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<O<O<O<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<O<O<O<I<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<O<O<O<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<O<O<O<I<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<O<O<O<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<O<O<O<I<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<O<O<O<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<O<O<O<I<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<O<O<O<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<O<O<O<I<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<O<O<O<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<O<O<O<I<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<O<O<O<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<O<O<O<I<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<O<O<O<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<O<O<O<I<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<O<O<O<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<O<O<O<I<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<O<O<O<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<O<O<O<I<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<O<O<O<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<O<O<O<I<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<O<O<O<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<I<O<O<I<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<I<O<O<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<I<O<O<I<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<I<O<O<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<I<O<O<I<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<I<O<O<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<I<O<O<I<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<I<O<O<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<I<O<O<I<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<I<O<O<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<I<O<O<I<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<I<O<O<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<I<O<O<I<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<I<O<O<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<I<O<O<I<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<I<O<O<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<I<O<O<I<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<I<O<O<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<I<O<O<I<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<I<O<O<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<I<O<O<I<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<I<O<O<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<I<O<O<I<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<I<O<O<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<I<O<O<I<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<I<O<O<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<I<O<O<I<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<I<O<O<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<I<O<O<I<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<I<O<O<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<I<O<O<I<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<I<O<O<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<O<I<O<I<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<O<I<O<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<O<I<O<I<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<O<I<O<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<O<I<O<I<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<O<I<O<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<O<I<O<I<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<O<I<O<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<O<I<O<I<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<O<I<O<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<O<I<O<I<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<O<I<O<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<O<I<O<I<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<O<I<O<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<O<I<O<I<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<O<I<O<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<O<I<O<I<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<O<I<O<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<O<I<O<I<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<O<I<O<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<O<I<O<I<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<O<I<O<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<O<I<O<I<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<O<I<O<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<O<I<O<I<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<O<I<O<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<O<I<O<I<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<O<I<O<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<O<I<O<I<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<O<I<O<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<O<I<O<I<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<O<I<O<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<I<I<O<I<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<I<I<O<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<I<I<O<I<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<I<I<O<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<I<I<O<I<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<I<I<O<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<I<I<O<I<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<I<I<O<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<I<I<O<I<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<I<I<O<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<I<I<O<I<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<I<I<O<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<I<I<O<I<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<I<I<O<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<I<I<O<I<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<I<I<O<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<I<I<O<I<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<I<I<O<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<I<I<O<I<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<I<I<O<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<I<I<O<I<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<I<I<O<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<I<I<O<I<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<I<I<O<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<I<I<O<I<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<I<I<O<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<I<I<O<I<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<I<I<O<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<I<I<O<I<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<I<I<O<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<I<I<O<I<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<I<I<O<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<O<O<I<I<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<O<O<I<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<O<O<I<I<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<O<O<I<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<O<O<I<I<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<O<O<I<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<O<O<I<I<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<O<O<I<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<O<O<I<I<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<O<O<I<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<O<O<I<I<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<O<O<I<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<O<O<I<I<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<O<O<I<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<O<O<I<I<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<O<O<I<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<O<O<I<I<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<O<O<I<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<O<O<I<I<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<O<O<I<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<O<O<I<I<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<O<O<I<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<O<O<I<I<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<O<O<I<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<O<O<I<I<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<O<O<I<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<O<O<I<I<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<O<O<I<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<O<O<I<I<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<O<O<I<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<O<O<I<I<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<O<O<I<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<I<O<I<I<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<I<O<I<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<I<O<I<I<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<I<O<I<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<I<O<I<I<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<I<O<I<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<I<O<I<I<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<I<O<I<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<I<O<I<I<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<I<O<I<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<I<O<I<I<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<I<O<I<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<I<O<I<I<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<I<O<I<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<I<O<I<I<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<I<O<I<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<I<O<I<I<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<I<O<I<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<I<O<I<I<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<I<O<I<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<I<O<I<I<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<I<O<I<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<I<O<I<I<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<I<O<I<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<I<O<I<I<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<I<O<I<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<I<O<I<I<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<I<O<I<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<I<O<I<I<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<I<O<I<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<I<O<I<I<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<I<O<I<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<O<I<I<I<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<O<I<I<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<O<I<I<I<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<O<I<I<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<O<I<I<I<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<O<I<I<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<O<I<I<I<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<O<I<I<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<O<I<I<I<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<O<I<I<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<O<I<I<I<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<O<I<I<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<O<I<I<I<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<O<I<I<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<O<I<I<I<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<O<I<I<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<O<I<I<I<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<O<I<I<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<O<I<I<I<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<O<I<I<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<O<I<I<I<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<O<I<I<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<O<I<I<I<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<O<I<I<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<O<I<I<I<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<O<I<I<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<O<I<I<I<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<O<I<I<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<O<I<I<I<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<O<I<I<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<O<I<I<I<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<O<I<I<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<I<I<I<I<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<I<I<I<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<I<I<I<I<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<I<I<I<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<I<I<I<I<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<I<I<I<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<I<I<I<I<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<I<I<I<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<I<I<I<I<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<I<I<I<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<I<I<I<I<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<I<I<I<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<I<I<I<I<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<I<I<I<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<I<I<I<I<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<I<I<I<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<I<I<I<I<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<I<I<I<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<I<I<I<I<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<I<I<I<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<I<I<I<I<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<I<I<I<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<I<I<I<I<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<I<I<I<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<I<I<I<I<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<I<I<I<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<I<I<I<I<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<I<I<I<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<I<I<I<I<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<I<I<I<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<I<I<I<I<O<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<I<I<I<I<O<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<O<O<O<O<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<O<O<O<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<O<O<O<O<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<O<O<O<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<O<O<O<O<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<O<O<O<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<O<O<O<O<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<O<O<O<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<O<O<O<O<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<O<O<O<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<O<O<O<O<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<O<O<O<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<O<O<O<O<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<O<O<O<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<O<O<O<O<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<O<O<O<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<O<O<O<O<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<O<O<O<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<O<O<O<O<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<O<O<O<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<O<O<O<O<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<O<O<O<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<O<O<O<O<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<O<O<O<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<O<O<O<O<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<O<O<O<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<O<O<O<O<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<O<O<O<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<O<O<O<O<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<O<O<O<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<O<O<O<O<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<O<O<O<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<I<O<O<O<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<I<O<O<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<I<O<O<O<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<I<O<O<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<I<O<O<O<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<I<O<O<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<I<O<O<O<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<I<O<O<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<I<O<O<O<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<I<O<O<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<I<O<O<O<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<I<O<O<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<I<O<O<O<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<I<O<O<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<I<O<O<O<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<I<O<O<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<I<O<O<O<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<I<O<O<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<I<O<O<O<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<I<O<O<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<I<O<O<O<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<I<O<O<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<I<O<O<O<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<I<O<O<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<I<O<O<O<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<I<O<O<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<I<O<O<O<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<I<O<O<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<I<O<O<O<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<I<O<O<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<I<O<O<O<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<I<O<O<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<O<I<O<O<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<O<I<O<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<O<I<O<O<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<O<I<O<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<O<I<O<O<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<O<I<O<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<O<I<O<O<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<O<I<O<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<O<I<O<O<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<O<I<O<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<O<I<O<O<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<O<I<O<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<O<I<O<O<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<O<I<O<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<O<I<O<O<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<O<I<O<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<O<I<O<O<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<O<I<O<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<O<I<O<O<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<O<I<O<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<O<I<O<O<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<O<I<O<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<O<I<O<O<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<O<I<O<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<O<I<O<O<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<O<I<O<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<O<I<O<O<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<O<I<O<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<O<I<O<O<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<O<I<O<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<O<I<O<O<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<O<I<O<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<I<I<O<O<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<I<I<O<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<I<I<O<O<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<I<I<O<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<I<I<O<O<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<I<I<O<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<I<I<O<O<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<I<I<O<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<I<I<O<O<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<I<I<O<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<I<I<O<O<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<I<I<O<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<I<I<O<O<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<I<I<O<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<I<I<O<O<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<I<I<O<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<I<I<O<O<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<I<I<O<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<I<I<O<O<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<I<I<O<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<I<I<O<O<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<I<I<O<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<I<I<O<O<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<I<I<O<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<I<I<O<O<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<I<I<O<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<I<I<O<O<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<I<I<O<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<I<I<O<O<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<I<I<O<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<I<I<O<O<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<I<I<O<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<O<O<I<O<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<O<O<I<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<O<O<I<O<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<O<O<I<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<O<O<I<O<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<O<O<I<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<O<O<I<O<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<O<O<I<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<O<O<I<O<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<O<O<I<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<O<O<I<O<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<O<O<I<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<O<O<I<O<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<O<O<I<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<O<O<I<O<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<O<O<I<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<O<O<I<O<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<O<O<I<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<O<O<I<O<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<O<O<I<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<O<O<I<O<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<O<O<I<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<O<O<I<O<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<O<O<I<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<O<O<I<O<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<O<O<I<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<O<O<I<O<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<O<O<I<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<O<O<I<O<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<O<O<I<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<O<O<I<O<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<O<O<I<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<I<O<I<O<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<I<O<I<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<I<O<I<O<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<I<O<I<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<I<O<I<O<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<I<O<I<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<I<O<I<O<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<I<O<I<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<I<O<I<O<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<I<O<I<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<I<O<I<O<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<I<O<I<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<I<O<I<O<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<I<O<I<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<I<O<I<O<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<I<O<I<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<I<O<I<O<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<I<O<I<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<I<O<I<O<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<I<O<I<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<I<O<I<O<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<I<O<I<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<I<O<I<O<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<I<O<I<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<I<O<I<O<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<I<O<I<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<I<O<I<O<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<I<O<I<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<I<O<I<O<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<I<O<I<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<I<O<I<O<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<I<O<I<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<O<I<I<O<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<O<I<I<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<O<I<I<O<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<O<I<I<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<O<I<I<O<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<O<I<I<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<O<I<I<O<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<O<I<I<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<O<I<I<O<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<O<I<I<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<O<I<I<O<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<O<I<I<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<O<I<I<O<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<O<I<I<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<O<I<I<O<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<O<I<I<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<O<I<I<O<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<O<I<I<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<O<I<I<O<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<O<I<I<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<O<I<I<O<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<O<I<I<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<O<I<I<O<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<O<I<I<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<O<I<I<O<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<O<I<I<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<O<I<I<O<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<O<I<I<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<O<I<I<O<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<O<I<I<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<O<I<I<O<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<O<I<I<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<I<I<I<O<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<I<I<I<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<I<I<I<O<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<I<I<I<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<I<I<I<O<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<I<I<I<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<I<I<I<O<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<I<I<I<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<I<I<I<O<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<I<I<I<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<I<I<I<O<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<I<I<I<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<I<I<I<O<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<I<I<I<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<I<I<I<O<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<I<I<I<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<I<I<I<O<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<I<I<I<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<I<I<I<O<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<I<I<I<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<I<I<I<O<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<I<I<I<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<I<I<I<O<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<I<I<I<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<I<I<I<O<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<I<I<I<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<I<I<I<O<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<I<I<I<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<I<I<I<O<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<I<I<I<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<I<I<I<O<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<I<I<I<O<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<O<O<O<I<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<O<O<O<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<O<O<O<I<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<O<O<O<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<O<O<O<I<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<O<O<O<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<O<O<O<I<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<O<O<O<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<O<O<O<I<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<O<O<O<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<O<O<O<I<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<O<O<O<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<O<O<O<I<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<O<O<O<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<O<O<O<I<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<O<O<O<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<O<O<O<I<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<O<O<O<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<O<O<O<I<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<O<O<O<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<O<O<O<I<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<O<O<O<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<O<O<O<I<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<O<O<O<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<O<O<O<I<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<O<O<O<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<O<O<O<I<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<O<O<O<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<O<O<O<I<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<O<O<O<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<O<O<O<I<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<O<O<O<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<I<O<O<I<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<I<O<O<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<I<O<O<I<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<I<O<O<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<I<O<O<I<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<I<O<O<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<I<O<O<I<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<I<O<O<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<I<O<O<I<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<I<O<O<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<I<O<O<I<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<I<O<O<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<I<O<O<I<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<I<O<O<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<I<O<O<I<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<I<O<O<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<I<O<O<I<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<I<O<O<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<I<O<O<I<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<I<O<O<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<I<O<O<I<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<I<O<O<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<I<O<O<I<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<I<O<O<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<I<O<O<I<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<I<O<O<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<I<O<O<I<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<I<O<O<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<I<O<O<I<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<I<O<O<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<I<O<O<I<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<I<O<O<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<O<I<O<I<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<O<I<O<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<O<I<O<I<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<O<I<O<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<O<I<O<I<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<O<I<O<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<O<I<O<I<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<O<I<O<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<O<I<O<I<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<O<I<O<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<O<I<O<I<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<O<I<O<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<O<I<O<I<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<O<I<O<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<O<I<O<I<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<O<I<O<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<O<I<O<I<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<O<I<O<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<O<I<O<I<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<O<I<O<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<O<I<O<I<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<O<I<O<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<O<I<O<I<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<O<I<O<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<O<I<O<I<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<O<I<O<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<O<I<O<I<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<O<I<O<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<O<I<O<I<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<O<I<O<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<O<I<O<I<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<O<I<O<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<I<I<O<I<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<I<I<O<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<I<I<O<I<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<I<I<O<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<I<I<O<I<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<I<I<O<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<I<I<O<I<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<I<I<O<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<I<I<O<I<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<I<I<O<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<I<I<O<I<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<I<I<O<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<I<I<O<I<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<I<I<O<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<I<I<O<I<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<I<I<O<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<I<I<O<I<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<I<I<O<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<I<I<O<I<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<I<I<O<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<I<I<O<I<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<I<I<O<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<I<I<O<I<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<I<I<O<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<I<I<O<I<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<I<I<O<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<I<I<O<I<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<I<I<O<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<I<I<O<I<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<I<I<O<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<I<I<O<I<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<I<I<O<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<O<O<I<I<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<O<O<I<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<O<O<I<I<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<O<O<I<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<O<O<I<I<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<O<O<I<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<O<O<I<I<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<O<O<I<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<O<O<I<I<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<O<O<I<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<O<O<I<I<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<O<O<I<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<O<O<I<I<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<O<O<I<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<O<O<I<I<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<O<O<I<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<O<O<I<I<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<O<O<I<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<O<O<I<I<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<O<O<I<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<O<O<I<I<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<O<O<I<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<O<O<I<I<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<O<O<I<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<O<O<I<I<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<O<O<I<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<O<O<I<I<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<O<O<I<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<O<O<I<I<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<O<O<I<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<O<O<I<I<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<O<O<I<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<I<O<I<I<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<I<O<I<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<I<O<I<I<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<I<O<I<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<I<O<I<I<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<I<O<I<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<I<O<I<I<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<I<O<I<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<I<O<I<I<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<I<O<I<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<I<O<I<I<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<I<O<I<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<I<O<I<I<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<I<O<I<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<I<O<I<I<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<I<O<I<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<I<O<I<I<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<I<O<I<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<I<O<I<I<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<I<O<I<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<I<O<I<I<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<I<O<I<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<I<O<I<I<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<I<O<I<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<I<O<I<I<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<I<O<I<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<I<O<I<I<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<I<O<I<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<I<O<I<I<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<I<O<I<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<I<O<I<I<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<I<O<I<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<O<I<I<I<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<O<I<I<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<O<I<I<I<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<O<I<I<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<O<I<I<I<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<O<I<I<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<O<I<I<I<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<O<I<I<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<O<I<I<I<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<O<I<I<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<O<I<I<I<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<O<I<I<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<O<I<I<I<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<O<I<I<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<O<I<I<I<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<O<I<I<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<O<I<I<I<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<O<I<I<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<O<I<I<I<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<O<I<I<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<O<I<I<I<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<O<I<I<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<O<I<I<I<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<O<I<I<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<O<I<I<I<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<O<I<I<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<O<I<I<I<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<O<I<I<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<O<I<I<I<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<O<I<I<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<O<I<I<I<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<O<I<I<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<I<I<I<I<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<I<I<I<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<I<I<I<I<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<I<I<I<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<I<I<I<I<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<I<I<I<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<I<I<I<I<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<I<I<I<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<I<I<I<I<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<I<I<I<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<I<I<I<I<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<I<I<I<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<I<I<I<I<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<I<I<I<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<I<I<I<I<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<I<I<I<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<I<I<I<I<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<I<I<I<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<I<I<I<I<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<I<I<I<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<I<I<I<I<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<I<I<I<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<I<I<I<I<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<I<I<I<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<I<I<I<I<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<I<I<I<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<I<I<I<I<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<I<I<I<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<I<I<I<I<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<I<I<I<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<I<I<I<I<I<O<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<I<I<I<I<I<O<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<O<O<O<O<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<O<O<O<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<O<O<O<O<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<O<O<O<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<O<O<O<O<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<O<O<O<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<O<O<O<O<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<O<O<O<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<O<O<O<O<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<O<O<O<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<O<O<O<O<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<O<O<O<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<O<O<O<O<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<O<O<O<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<O<O<O<O<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<O<O<O<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<O<O<O<O<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<O<O<O<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<O<O<O<O<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<O<O<O<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<O<O<O<O<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<O<O<O<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<O<O<O<O<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<O<O<O<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<O<O<O<O<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<O<O<O<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<O<O<O<O<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<O<O<O<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<O<O<O<O<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<O<O<O<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<O<O<O<O<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<O<O<O<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<I<O<O<O<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<I<O<O<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<I<O<O<O<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<I<O<O<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<I<O<O<O<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<I<O<O<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<I<O<O<O<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<I<O<O<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<I<O<O<O<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<I<O<O<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<I<O<O<O<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<I<O<O<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<I<O<O<O<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<I<O<O<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<I<O<O<O<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<I<O<O<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<I<O<O<O<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<I<O<O<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<I<O<O<O<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<I<O<O<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<I<O<O<O<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<I<O<O<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<I<O<O<O<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<I<O<O<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<I<O<O<O<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<I<O<O<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<I<O<O<O<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<I<O<O<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<I<O<O<O<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<I<O<O<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<I<O<O<O<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<I<O<O<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<O<I<O<O<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<O<I<O<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<O<I<O<O<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<O<I<O<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<O<I<O<O<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<O<I<O<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<O<I<O<O<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<O<I<O<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<O<I<O<O<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<O<I<O<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<O<I<O<O<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<O<I<O<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<O<I<O<O<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<O<I<O<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<O<I<O<O<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<O<I<O<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<O<I<O<O<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<O<I<O<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<O<I<O<O<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<O<I<O<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<O<I<O<O<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<O<I<O<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<O<I<O<O<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<O<I<O<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<O<I<O<O<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<O<I<O<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<O<I<O<O<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<O<I<O<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<O<I<O<O<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<O<I<O<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<O<I<O<O<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<O<I<O<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<I<I<O<O<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<I<I<O<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<I<I<O<O<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<I<I<O<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<I<I<O<O<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<I<I<O<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<I<I<O<O<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<I<I<O<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<I<I<O<O<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<I<I<O<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<I<I<O<O<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<I<I<O<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<I<I<O<O<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<I<I<O<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<I<I<O<O<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<I<I<O<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<I<I<O<O<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<I<I<O<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<I<I<O<O<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<I<I<O<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<I<I<O<O<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<I<I<O<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<I<I<O<O<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<I<I<O<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<I<I<O<O<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<I<I<O<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<I<I<O<O<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<I<I<O<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<I<I<O<O<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<I<I<O<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<I<I<O<O<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<I<I<O<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<O<O<I<O<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<O<O<I<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<O<O<I<O<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<O<O<I<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<O<O<I<O<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<O<O<I<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<O<O<I<O<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<O<O<I<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<O<O<I<O<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<O<O<I<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<O<O<I<O<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<O<O<I<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<O<O<I<O<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<O<O<I<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<O<O<I<O<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<O<O<I<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<O<O<I<O<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<O<O<I<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<O<O<I<O<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<O<O<I<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<O<O<I<O<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<O<O<I<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<O<O<I<O<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<O<O<I<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<O<O<I<O<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<O<O<I<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<O<O<I<O<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<O<O<I<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<O<O<I<O<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<O<O<I<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<O<O<I<O<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<O<O<I<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<I<O<I<O<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<I<O<I<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<I<O<I<O<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<I<O<I<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<I<O<I<O<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<I<O<I<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<I<O<I<O<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<I<O<I<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<I<O<I<O<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<I<O<I<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<I<O<I<O<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<I<O<I<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<I<O<I<O<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<I<O<I<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<I<O<I<O<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<I<O<I<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<I<O<I<O<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<I<O<I<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<I<O<I<O<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<I<O<I<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<I<O<I<O<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<I<O<I<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<I<O<I<O<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<I<O<I<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<I<O<I<O<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<I<O<I<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<I<O<I<O<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<I<O<I<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<I<O<I<O<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<I<O<I<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<I<O<I<O<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<I<O<I<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<O<I<I<O<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<O<I<I<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<O<I<I<O<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<O<I<I<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<O<I<I<O<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<O<I<I<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<O<I<I<O<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<O<I<I<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<O<I<I<O<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<O<I<I<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<O<I<I<O<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<O<I<I<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<O<I<I<O<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<O<I<I<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<O<I<I<O<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<O<I<I<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<O<I<I<O<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<O<I<I<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<O<I<I<O<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<O<I<I<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<O<I<I<O<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<O<I<I<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<O<I<I<O<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<O<I<I<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<O<I<I<O<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<O<I<I<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<O<I<I<O<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<O<I<I<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<O<I<I<O<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<O<I<I<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<O<I<I<O<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<O<I<I<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<I<I<I<O<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<I<I<I<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<I<I<I<O<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<I<I<I<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<I<I<I<O<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<I<I<I<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<I<I<I<O<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<I<I<I<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<I<I<I<O<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<I<I<I<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<I<I<I<O<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<I<I<I<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<I<I<I<O<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<I<I<I<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<I<I<I<O<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<I<I<I<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<I<I<I<O<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<I<I<I<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<I<I<I<O<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<I<I<I<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<I<I<I<O<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<I<I<I<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<I<I<I<O<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<I<I<I<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<I<I<I<O<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<I<I<I<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<I<I<I<O<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<I<I<I<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<I<I<I<O<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<I<I<I<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<I<I<I<O<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<I<I<I<O<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<O<O<O<I<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<O<O<O<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<O<O<O<I<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<O<O<O<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<O<O<O<I<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<O<O<O<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<O<O<O<I<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<O<O<O<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<O<O<O<I<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<O<O<O<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<O<O<O<I<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<O<O<O<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<O<O<O<I<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<O<O<O<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<O<O<O<I<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<O<O<O<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<O<O<O<I<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<O<O<O<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<O<O<O<I<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<O<O<O<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<O<O<O<I<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<O<O<O<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<O<O<O<I<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<O<O<O<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<O<O<O<I<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<O<O<O<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<O<O<O<I<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<O<O<O<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<O<O<O<I<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<O<O<O<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<O<O<O<I<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<O<O<O<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<I<O<O<I<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<I<O<O<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<I<O<O<I<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<I<O<O<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<I<O<O<I<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<I<O<O<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<I<O<O<I<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<I<O<O<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<I<O<O<I<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<I<O<O<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<I<O<O<I<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<I<O<O<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<I<O<O<I<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<I<O<O<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<I<O<O<I<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<I<O<O<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<I<O<O<I<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<I<O<O<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<I<O<O<I<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<I<O<O<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<I<O<O<I<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<I<O<O<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<I<O<O<I<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<I<O<O<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<I<O<O<I<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<I<O<O<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<I<O<O<I<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<I<O<O<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<I<O<O<I<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<I<O<O<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<I<O<O<I<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<I<O<O<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<O<I<O<I<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<O<I<O<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<O<I<O<I<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<O<I<O<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<O<I<O<I<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<O<I<O<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<O<I<O<I<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<O<I<O<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<O<I<O<I<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<O<I<O<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<O<I<O<I<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<O<I<O<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<O<I<O<I<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<O<I<O<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<O<I<O<I<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<O<I<O<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<O<I<O<I<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<O<I<O<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<O<I<O<I<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<O<I<O<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<O<I<O<I<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<O<I<O<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<O<I<O<I<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<O<I<O<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<O<I<O<I<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<O<I<O<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<O<I<O<I<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<O<I<O<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<O<I<O<I<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<O<I<O<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<O<I<O<I<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<O<I<O<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<I<I<O<I<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<I<I<O<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<I<I<O<I<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<I<I<O<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<I<I<O<I<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<I<I<O<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<I<I<O<I<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<I<I<O<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<I<I<O<I<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<I<I<O<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<I<I<O<I<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<I<I<O<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<I<I<O<I<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<I<I<O<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<I<I<O<I<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<I<I<O<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<I<I<O<I<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<I<I<O<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<I<I<O<I<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<I<I<O<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<I<I<O<I<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<I<I<O<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<I<I<O<I<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<I<I<O<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<I<I<O<I<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<I<I<O<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<I<I<O<I<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<I<I<O<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<I<I<O<I<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<I<I<O<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<I<I<O<I<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<I<I<O<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<O<O<I<I<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<O<O<I<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<O<O<I<I<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<O<O<I<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<O<O<I<I<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<O<O<I<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<O<O<I<I<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<O<O<I<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<O<O<I<I<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<O<O<I<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<O<O<I<I<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<O<O<I<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<O<O<I<I<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<O<O<I<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<O<O<I<I<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<O<O<I<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<O<O<I<I<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<O<O<I<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<O<O<I<I<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<O<O<I<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<O<O<I<I<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<O<O<I<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<O<O<I<I<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<O<O<I<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<O<O<I<I<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<O<O<I<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<O<O<I<I<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<O<O<I<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<O<O<I<I<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<O<O<I<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<O<O<I<I<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<O<O<I<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<I<O<I<I<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<I<O<I<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<I<O<I<I<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<I<O<I<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<I<O<I<I<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<I<O<I<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<I<O<I<I<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<I<O<I<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<I<O<I<I<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<I<O<I<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<I<O<I<I<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<I<O<I<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<I<O<I<I<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<I<O<I<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<I<O<I<I<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<I<O<I<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<I<O<I<I<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<I<O<I<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<I<O<I<I<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<I<O<I<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<I<O<I<I<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<I<O<I<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<I<O<I<I<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<I<O<I<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<I<O<I<I<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<I<O<I<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<I<O<I<I<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<I<O<I<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<I<O<I<I<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<I<O<I<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<I<O<I<I<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<I<O<I<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<O<I<I<I<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<O<I<I<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<O<I<I<I<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<O<I<I<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<O<I<I<I<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<O<I<I<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<O<I<I<I<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<O<I<I<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<O<I<I<I<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<O<I<I<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<O<I<I<I<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<O<I<I<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<O<I<I<I<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<O<I<I<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<O<I<I<I<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<O<I<I<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<O<I<I<I<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<O<I<I<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<O<I<I<I<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<O<I<I<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<O<I<I<I<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<O<I<I<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<O<I<I<I<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<O<I<I<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<O<I<I<I<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<O<I<I<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<O<I<I<I<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<O<I<I<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<O<I<I<I<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<O<I<I<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<O<I<I<I<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<O<I<I<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<I<I<I<I<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<I<I<I<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<I<I<I<I<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<I<I<I<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<I<I<I<I<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<I<I<I<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<I<I<I<I<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<I<I<I<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<I<I<I<I<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<I<I<I<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<I<I<I<I<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<I<I<I<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<I<I<I<I<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<I<I<I<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<I<I<I<I<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<I<I<I<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<I<I<I<I<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<I<I<I<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<I<I<I<I<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<I<I<I<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<I<I<I<I<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<I<I<I<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<I<I<I<I<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<I<I<I<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<I<I<I<I<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<I<I<I<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<I<I<I<I<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<I<I<I<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<I<I<I<I<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<I<I<I<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<I<I<I<I<O<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<I<I<I<I<O<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<O<O<O<O<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<O<O<O<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<O<O<O<O<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<O<O<O<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<O<O<O<O<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<O<O<O<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<O<O<O<O<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<O<O<O<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<O<O<O<O<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<O<O<O<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<O<O<O<O<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<O<O<O<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<O<O<O<O<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<O<O<O<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<O<O<O<O<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<O<O<O<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<O<O<O<O<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<O<O<O<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<O<O<O<O<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<O<O<O<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<O<O<O<O<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<O<O<O<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<O<O<O<O<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<O<O<O<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<O<O<O<O<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<O<O<O<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<O<O<O<O<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<O<O<O<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<O<O<O<O<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<O<O<O<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<O<O<O<O<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<O<O<O<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<I<O<O<O<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<I<O<O<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<I<O<O<O<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<I<O<O<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<I<O<O<O<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<I<O<O<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<I<O<O<O<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<I<O<O<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<I<O<O<O<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<I<O<O<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<I<O<O<O<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<I<O<O<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<I<O<O<O<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<I<O<O<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<I<O<O<O<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<I<O<O<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<I<O<O<O<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<I<O<O<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<I<O<O<O<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<I<O<O<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<I<O<O<O<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<I<O<O<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<I<O<O<O<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<I<O<O<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<I<O<O<O<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<I<O<O<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<I<O<O<O<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<I<O<O<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<I<O<O<O<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<I<O<O<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<I<O<O<O<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<I<O<O<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<O<I<O<O<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<O<I<O<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<O<I<O<O<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<O<I<O<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<O<I<O<O<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<O<I<O<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<O<I<O<O<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<O<I<O<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<O<I<O<O<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<O<I<O<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<O<I<O<O<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<O<I<O<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<O<I<O<O<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<O<I<O<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<O<I<O<O<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<O<I<O<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<O<I<O<O<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<O<I<O<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<O<I<O<O<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<O<I<O<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<O<I<O<O<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<O<I<O<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<O<I<O<O<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<O<I<O<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<O<I<O<O<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<O<I<O<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<O<I<O<O<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<O<I<O<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<O<I<O<O<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<O<I<O<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<O<I<O<O<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<O<I<O<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<I<I<O<O<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<I<I<O<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<I<I<O<O<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<I<I<O<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<I<I<O<O<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<I<I<O<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<I<I<O<O<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<I<I<O<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<I<I<O<O<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<I<I<O<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<I<I<O<O<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<I<I<O<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<I<I<O<O<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<I<I<O<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<I<I<O<O<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<I<I<O<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<I<I<O<O<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<I<I<O<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<I<I<O<O<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<I<I<O<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<I<I<O<O<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<I<I<O<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<I<I<O<O<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<I<I<O<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<I<I<O<O<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<I<I<O<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<I<I<O<O<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<I<I<O<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<I<I<O<O<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<I<I<O<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<I<I<O<O<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<I<I<O<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<O<O<I<O<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<O<O<I<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<O<O<I<O<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<O<O<I<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<O<O<I<O<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<O<O<I<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<O<O<I<O<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<O<O<I<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<O<O<I<O<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<O<O<I<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<O<O<I<O<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<O<O<I<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<O<O<I<O<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<O<O<I<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<O<O<I<O<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<O<O<I<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<O<O<I<O<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<O<O<I<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<O<O<I<O<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<O<O<I<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<O<O<I<O<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<O<O<I<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<O<O<I<O<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<O<O<I<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<O<O<I<O<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<O<O<I<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<O<O<I<O<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<O<O<I<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<O<O<I<O<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<O<O<I<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<O<O<I<O<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<O<O<I<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<I<O<I<O<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<I<O<I<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<I<O<I<O<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<I<O<I<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<I<O<I<O<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<I<O<I<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<I<O<I<O<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<I<O<I<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<I<O<I<O<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<I<O<I<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<I<O<I<O<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<I<O<I<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<I<O<I<O<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<I<O<I<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<I<O<I<O<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<I<O<I<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<I<O<I<O<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<I<O<I<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<I<O<I<O<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<I<O<I<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<I<O<I<O<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<I<O<I<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<I<O<I<O<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<I<O<I<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<I<O<I<O<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<I<O<I<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<I<O<I<O<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<I<O<I<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<I<O<I<O<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<I<O<I<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<I<O<I<O<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<I<O<I<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<O<I<I<O<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<O<I<I<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<O<I<I<O<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<O<I<I<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<O<I<I<O<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<O<I<I<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<O<I<I<O<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<O<I<I<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<O<I<I<O<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<O<I<I<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<O<I<I<O<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<O<I<I<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<O<I<I<O<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<O<I<I<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<O<I<I<O<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<O<I<I<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<O<I<I<O<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<O<I<I<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<O<I<I<O<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<O<I<I<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<O<I<I<O<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<O<I<I<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<O<I<I<O<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<O<I<I<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<O<I<I<O<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<O<I<I<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<O<I<I<O<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<O<I<I<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<O<I<I<O<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<O<I<I<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<O<I<I<O<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<O<I<I<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<I<I<I<O<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<I<I<I<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<I<I<I<O<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<I<I<I<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<I<I<I<O<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<I<I<I<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<I<I<I<O<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<I<I<I<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<I<I<I<O<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<I<I<I<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<I<I<I<O<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<I<I<I<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<I<I<I<O<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<I<I<I<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<I<I<I<O<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<I<I<I<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<I<I<I<O<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<I<I<I<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<I<I<I<O<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<I<I<I<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<I<I<I<O<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<I<I<I<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<I<I<I<O<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<I<I<I<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<I<I<I<O<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<I<I<I<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<I<I<I<O<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<I<I<I<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<I<I<I<O<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<I<I<I<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<I<I<I<O<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<I<I<I<O<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<O<O<O<I<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<O<O<O<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<O<O<O<I<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<O<O<O<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<O<O<O<I<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<O<O<O<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<O<O<O<I<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<O<O<O<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<O<O<O<I<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<O<O<O<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<O<O<O<I<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<O<O<O<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<O<O<O<I<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<O<O<O<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<O<O<O<I<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<O<O<O<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<O<O<O<I<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<O<O<O<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<O<O<O<I<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<O<O<O<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<O<O<O<I<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<O<O<O<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<O<O<O<I<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<O<O<O<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<O<O<O<I<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<O<O<O<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<O<O<O<I<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<O<O<O<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<O<O<O<I<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<O<O<O<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<O<O<O<I<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<O<O<O<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<I<O<O<I<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<I<O<O<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<I<O<O<I<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<I<O<O<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<I<O<O<I<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<I<O<O<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<I<O<O<I<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<I<O<O<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<I<O<O<I<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<I<O<O<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<I<O<O<I<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<I<O<O<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<I<O<O<I<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<I<O<O<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<I<O<O<I<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<I<O<O<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<I<O<O<I<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<I<O<O<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<I<O<O<I<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<I<O<O<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<I<O<O<I<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<I<O<O<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<I<O<O<I<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<I<O<O<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<I<O<O<I<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<I<O<O<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<I<O<O<I<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<I<O<O<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<I<O<O<I<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<I<O<O<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<I<O<O<I<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<I<O<O<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<O<I<O<I<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<O<I<O<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<O<I<O<I<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<O<I<O<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<O<I<O<I<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<O<I<O<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<O<I<O<I<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<O<I<O<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<O<I<O<I<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<O<I<O<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<O<I<O<I<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<O<I<O<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<O<I<O<I<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<O<I<O<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<O<I<O<I<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<O<I<O<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<O<I<O<I<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<O<I<O<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<O<I<O<I<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<O<I<O<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<O<I<O<I<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<O<I<O<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<O<I<O<I<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<O<I<O<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<O<I<O<I<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<O<I<O<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<O<I<O<I<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<O<I<O<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<O<I<O<I<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<O<I<O<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<O<I<O<I<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<O<I<O<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<I<I<O<I<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<I<I<O<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<I<I<O<I<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<I<I<O<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<I<I<O<I<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<I<I<O<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<I<I<O<I<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<I<I<O<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<I<I<O<I<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<I<I<O<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<I<I<O<I<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<I<I<O<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<I<I<O<I<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<I<I<O<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<I<I<O<I<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<I<I<O<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<I<I<O<I<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<I<I<O<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<I<I<O<I<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<I<I<O<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<I<I<O<I<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<I<I<O<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<I<I<O<I<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<I<I<O<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<I<I<O<I<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<I<I<O<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<I<I<O<I<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<I<I<O<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<I<I<O<I<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<I<I<O<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<I<I<O<I<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<I<I<O<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<O<O<I<I<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<O<O<I<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<O<O<I<I<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<O<O<I<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<O<O<I<I<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<O<O<I<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<O<O<I<I<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<O<O<I<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<O<O<I<I<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<O<O<I<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<O<O<I<I<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<O<O<I<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<O<O<I<I<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<O<O<I<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<O<O<I<I<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<O<O<I<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<O<O<I<I<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<O<O<I<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<O<O<I<I<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<O<O<I<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<O<O<I<I<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<O<O<I<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<O<O<I<I<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<O<O<I<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<O<O<I<I<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<O<O<I<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<O<O<I<I<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<O<O<I<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<O<O<I<I<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<O<O<I<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<O<O<I<I<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<O<O<I<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<I<O<I<I<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<I<O<I<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<I<O<I<I<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<I<O<I<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<I<O<I<I<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<I<O<I<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<I<O<I<I<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<I<O<I<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<I<O<I<I<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<I<O<I<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<I<O<I<I<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<I<O<I<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<I<O<I<I<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<I<O<I<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<I<O<I<I<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<I<O<I<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<I<O<I<I<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<I<O<I<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<I<O<I<I<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<I<O<I<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<I<O<I<I<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<I<O<I<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<I<O<I<I<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<I<O<I<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<I<O<I<I<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<I<O<I<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<I<O<I<I<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<I<O<I<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<I<O<I<I<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<I<O<I<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<I<O<I<I<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<I<O<I<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<O<I<I<I<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<O<I<I<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<O<I<I<I<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<O<I<I<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<O<I<I<I<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<O<I<I<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<O<I<I<I<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<O<I<I<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<O<I<I<I<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<O<I<I<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<O<I<I<I<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<O<I<I<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<O<I<I<I<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<O<I<I<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<O<I<I<I<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<O<I<I<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<O<I<I<I<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<O<I<I<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<O<I<I<I<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<O<I<I<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<O<I<I<I<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<O<I<I<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<O<I<I<I<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<O<I<I<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<O<I<I<I<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<O<I<I<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<O<I<I<I<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<O<I<I<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<O<I<I<I<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<O<I<I<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<O<I<I<I<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<O<I<I<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<I<I<I<I<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<I<I<I<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<I<I<I<I<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<I<I<I<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<I<I<I<I<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<I<I<I<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<I<I<I<I<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<I<I<I<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<I<I<I<I<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<I<I<I<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<I<I<I<I<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<I<I<I<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<I<I<I<I<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<I<I<I<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<I<I<I<I<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<I<I<I<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<I<I<I<I<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<I<I<I<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<I<I<I<I<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<I<I<I<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<I<I<I<I<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<I<I<I<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<I<I<I<I<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<I<I<I<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<I<I<I<I<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<I<I<I<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<I<I<I<I<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<I<I<I<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<I<I<I<I<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<I<I<I<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<I<I<I<I<I<I<H>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<I<I<I<I<I<I<H>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<O<O<O<O<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<O<O<O<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<O<O<O<O<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<O<O<O<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<O<O<O<O<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<O<O<O<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<O<O<O<O<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<O<O<O<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<O<O<O<O<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<O<O<O<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<O<O<O<O<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<O<O<O<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<O<O<O<O<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<O<O<O<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<O<O<O<O<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<O<O<O<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<O<O<O<O<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<O<O<O<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<O<O<O<O<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<O<O<O<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<O<O<O<O<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<O<O<O<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<O<O<O<O<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<O<O<O<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<O<O<O<O<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<O<O<O<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<O<O<O<O<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<O<O<O<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<O<O<O<O<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<O<O<O<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<O<O<O<O<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<O<O<O<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<I<O<O<O<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<I<O<O<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<I<O<O<O<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<I<O<O<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<I<O<O<O<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<I<O<O<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<I<O<O<O<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<I<O<O<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<I<O<O<O<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<I<O<O<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<I<O<O<O<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<I<O<O<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<I<O<O<O<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<I<O<O<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<I<O<O<O<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<I<O<O<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<I<O<O<O<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<I<O<O<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<I<O<O<O<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<I<O<O<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<I<O<O<O<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<I<O<O<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<I<O<O<O<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<I<O<O<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<I<O<O<O<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<I<O<O<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<I<O<O<O<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<I<O<O<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<I<O<O<O<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<I<O<O<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<I<O<O<O<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<I<O<O<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<O<I<O<O<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<O<I<O<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<O<I<O<O<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<O<I<O<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<O<I<O<O<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<O<I<O<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<O<I<O<O<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<O<I<O<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<O<I<O<O<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<O<I<O<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<O<I<O<O<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<O<I<O<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<O<I<O<O<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<O<I<O<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<O<I<O<O<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<O<I<O<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<O<I<O<O<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<O<I<O<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<O<I<O<O<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<O<I<O<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<O<I<O<O<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<O<I<O<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<O<I<O<O<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<O<I<O<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<O<I<O<O<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<O<I<O<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<O<I<O<O<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<O<I<O<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<O<I<O<O<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<O<I<O<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<O<I<O<O<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<O<I<O<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<I<I<O<O<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<I<I<O<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<I<I<O<O<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<I<I<O<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<I<I<O<O<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<I<I<O<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<I<I<O<O<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<I<I<O<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<I<I<O<O<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<I<I<O<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<I<I<O<O<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<I<I<O<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<I<I<O<O<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<I<I<O<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<I<I<O<O<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<I<I<O<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<I<I<O<O<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<I<I<O<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<I<I<O<O<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<I<I<O<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<I<I<O<O<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<I<I<O<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<I<I<O<O<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<I<I<O<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<I<I<O<O<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<I<I<O<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<I<I<O<O<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<I<I<O<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<I<I<O<O<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<I<I<O<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<I<I<O<O<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<I<I<O<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<O<O<I<O<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<O<O<I<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<O<O<I<O<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<O<O<I<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<O<O<I<O<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<O<O<I<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<O<O<I<O<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<O<O<I<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<O<O<I<O<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<O<O<I<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<O<O<I<O<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<O<O<I<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<O<O<I<O<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<O<O<I<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<O<O<I<O<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<O<O<I<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<O<O<I<O<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<O<O<I<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<O<O<I<O<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<O<O<I<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<O<O<I<O<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<O<O<I<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<O<O<I<O<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<O<O<I<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<O<O<I<O<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<O<O<I<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<O<O<I<O<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<O<O<I<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<O<O<I<O<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<O<O<I<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<O<O<I<O<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<O<O<I<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<I<O<I<O<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<I<O<I<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<I<O<I<O<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<I<O<I<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<I<O<I<O<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<I<O<I<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<I<O<I<O<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<I<O<I<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<I<O<I<O<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<I<O<I<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<I<O<I<O<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<I<O<I<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<I<O<I<O<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<I<O<I<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<I<O<I<O<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<I<O<I<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<I<O<I<O<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<I<O<I<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<I<O<I<O<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<I<O<I<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<I<O<I<O<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<I<O<I<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<I<O<I<O<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<I<O<I<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<I<O<I<O<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<I<O<I<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<I<O<I<O<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<I<O<I<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<I<O<I<O<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<I<O<I<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<I<O<I<O<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<I<O<I<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<O<I<I<O<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<O<I<I<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<O<I<I<O<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<O<I<I<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<O<I<I<O<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<O<I<I<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<O<I<I<O<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<O<I<I<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<O<I<I<O<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<O<I<I<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<O<I<I<O<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<O<I<I<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<O<I<I<O<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<O<I<I<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<O<I<I<O<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<O<I<I<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<O<I<I<O<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<O<I<I<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<O<I<I<O<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<O<I<I<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<O<I<I<O<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<O<I<I<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<O<I<I<O<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<O<I<I<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<O<I<I<O<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<O<I<I<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<O<I<I<O<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<O<I<I<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<O<I<I<O<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<O<I<I<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<O<I<I<O<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<O<I<I<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<I<I<I<O<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<I<I<I<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<I<I<I<O<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<I<I<I<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<I<I<I<O<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<I<I<I<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<I<I<I<O<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<I<I<I<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<I<I<I<O<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<I<I<I<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<I<I<I<O<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<I<I<I<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<I<I<I<O<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<I<I<I<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<I<I<I<O<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<I<I<I<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<I<I<I<O<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<I<I<I<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<I<I<I<O<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<I<I<I<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<I<I<I<O<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<I<I<I<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<I<I<I<O<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<I<I<I<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<I<I<I<O<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<I<I<I<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<I<I<I<O<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<I<I<I<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<I<I<I<O<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<I<I<I<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<I<I<I<O<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<I<I<I<O<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<O<O<O<I<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<O<O<O<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<O<O<O<I<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<O<O<O<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<O<O<O<I<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<O<O<O<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<O<O<O<I<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<O<O<O<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<O<O<O<I<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<O<O<O<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<O<O<O<I<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<O<O<O<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<O<O<O<I<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<O<O<O<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<O<O<O<I<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<O<O<O<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<O<O<O<I<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<O<O<O<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<O<O<O<I<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<O<O<O<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<O<O<O<I<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<O<O<O<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<O<O<O<I<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<O<O<O<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<O<O<O<I<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<O<O<O<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<O<O<O<I<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<O<O<O<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<O<O<O<I<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<O<O<O<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<O<O<O<I<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<O<O<O<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<I<O<O<I<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<I<O<O<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<I<O<O<I<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<I<O<O<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<I<O<O<I<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<I<O<O<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<I<O<O<I<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<I<O<O<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<I<O<O<I<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<I<O<O<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<I<O<O<I<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<I<O<O<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<I<O<O<I<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<I<O<O<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<I<O<O<I<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<I<O<O<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<I<O<O<I<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<I<O<O<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<I<O<O<I<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<I<O<O<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<I<O<O<I<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<I<O<O<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<I<O<O<I<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<I<O<O<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<I<O<O<I<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<I<O<O<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<I<O<O<I<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<I<O<O<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<I<O<O<I<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<I<O<O<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<I<O<O<I<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<I<O<O<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<O<I<O<I<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<O<I<O<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<O<I<O<I<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<O<I<O<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<O<I<O<I<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<O<I<O<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<O<I<O<I<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<O<I<O<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<O<I<O<I<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<O<I<O<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<O<I<O<I<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<O<I<O<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<O<I<O<I<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<O<I<O<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<O<I<O<I<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<O<I<O<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<O<I<O<I<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<O<I<O<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<O<I<O<I<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<O<I<O<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<O<I<O<I<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<O<I<O<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<O<I<O<I<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<O<I<O<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<O<I<O<I<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<O<I<O<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<O<I<O<I<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<O<I<O<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<O<I<O<I<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<O<I<O<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<O<I<O<I<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<O<I<O<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<I<I<O<I<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<I<I<O<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<I<I<O<I<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<I<I<O<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<I<I<O<I<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<I<I<O<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<I<I<O<I<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<I<I<O<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<I<I<O<I<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<I<I<O<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<I<I<O<I<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<I<I<O<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<I<I<O<I<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<I<I<O<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<I<I<O<I<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<I<I<O<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<I<I<O<I<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<I<I<O<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<I<I<O<I<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<I<I<O<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<I<I<O<I<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<I<I<O<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<I<I<O<I<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<I<I<O<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<I<I<O<I<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<I<I<O<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<I<I<O<I<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<I<I<O<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<I<I<O<I<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<I<I<O<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<I<I<O<I<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<I<I<O<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<O<O<I<I<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<O<O<I<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<O<O<I<I<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<O<O<I<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<O<O<I<I<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<O<O<I<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<O<O<I<I<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<O<O<I<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<O<O<I<I<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<O<O<I<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<O<O<I<I<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<O<O<I<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<O<O<I<I<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<O<O<I<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<O<O<I<I<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<O<O<I<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<O<O<I<I<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<O<O<I<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<O<O<I<I<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<O<O<I<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<O<O<I<I<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<O<O<I<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<O<O<I<I<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<O<O<I<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<O<O<I<I<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<O<O<I<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<O<O<I<I<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<O<O<I<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<O<O<I<I<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<O<O<I<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<O<O<I<I<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<O<O<I<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<I<O<I<I<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<I<O<I<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<I<O<I<I<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<I<O<I<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<I<O<I<I<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<I<O<I<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<I<O<I<I<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<I<O<I<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<I<O<I<I<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<I<O<I<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<I<O<I<I<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<I<O<I<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<I<O<I<I<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<I<O<I<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<I<O<I<I<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<I<O<I<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<I<O<I<I<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<I<O<I<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<I<O<I<I<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<I<O<I<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<I<O<I<I<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<I<O<I<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<I<O<I<I<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<I<O<I<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<I<O<I<I<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<I<O<I<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<I<O<I<I<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<I<O<I<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<I<O<I<I<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<I<O<I<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<I<O<I<I<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<I<O<I<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<O<I<I<I<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<O<I<I<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<O<I<I<I<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<O<I<I<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<O<I<I<I<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<O<I<I<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<O<I<I<I<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<O<I<I<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<O<I<I<I<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<O<I<I<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<O<I<I<I<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<O<I<I<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<O<I<I<I<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<O<I<I<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<O<I<I<I<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<O<I<I<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<O<I<I<I<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<O<I<I<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<O<I<I<I<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<O<I<I<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<O<I<I<I<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<O<I<I<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<O<I<I<I<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<O<I<I<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<O<I<I<I<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<O<I<I<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<O<I<I<I<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<O<I<I<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<O<I<I<I<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<O<I<I<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<O<I<I<I<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<O<I<I<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<I<I<I<I<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<I<I<I<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<I<I<I<I<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<I<I<I<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<I<I<I<I<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<I<I<I<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<I<I<I<I<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<I<I<I<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<I<I<I<I<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<I<I<I<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<I<I<I<I<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<I<I<I<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<I<I<I<I<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<I<I<I<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<I<I<I<I<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<I<I<I<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<I<I<I<I<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<I<I<I<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<I<I<I<I<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<I<I<I<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<I<I<I<I<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<I<I<I<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<I<I<I<I<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<I<I<I<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<I<I<I<I<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<I<I<I<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<I<I<I<I<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<I<I<I<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<I<I<I<I<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<I<I<I<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<I<I<I<I<O<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<I<I<I<I<O<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<O<O<O<O<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<O<O<O<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<O<O<O<O<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<O<O<O<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<O<O<O<O<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<O<O<O<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<O<O<O<O<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<O<O<O<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<O<O<O<O<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<O<O<O<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<O<O<O<O<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<O<O<O<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<O<O<O<O<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<O<O<O<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<O<O<O<O<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<O<O<O<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<O<O<O<O<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<O<O<O<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<O<O<O<O<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<O<O<O<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<O<O<O<O<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<O<O<O<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<O<O<O<O<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<O<O<O<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<O<O<O<O<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<O<O<O<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<O<O<O<O<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<O<O<O<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<O<O<O<O<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<O<O<O<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<O<O<O<O<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<O<O<O<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<I<O<O<O<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<I<O<O<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<I<O<O<O<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<I<O<O<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<I<O<O<O<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<I<O<O<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<I<O<O<O<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<I<O<O<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<I<O<O<O<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<I<O<O<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<I<O<O<O<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<I<O<O<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<I<O<O<O<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<I<O<O<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<I<O<O<O<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<I<O<O<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<I<O<O<O<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<I<O<O<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<I<O<O<O<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<I<O<O<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<I<O<O<O<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<I<O<O<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<I<O<O<O<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<I<O<O<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<I<O<O<O<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<I<O<O<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<I<O<O<O<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<I<O<O<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<I<O<O<O<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<I<O<O<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<I<O<O<O<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<I<O<O<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<O<I<O<O<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<O<I<O<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<O<I<O<O<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<O<I<O<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<O<I<O<O<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<O<I<O<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<O<I<O<O<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<O<I<O<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<O<I<O<O<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<O<I<O<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<O<I<O<O<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<O<I<O<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<O<I<O<O<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<O<I<O<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<O<I<O<O<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<O<I<O<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<O<I<O<O<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<O<I<O<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<O<I<O<O<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<O<I<O<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<O<I<O<O<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<O<I<O<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<O<I<O<O<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<O<I<O<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<O<I<O<O<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<O<I<O<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<O<I<O<O<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<O<I<O<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<O<I<O<O<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<O<I<O<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<O<I<O<O<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<O<I<O<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<I<I<O<O<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<I<I<O<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<I<I<O<O<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<I<I<O<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<I<I<O<O<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<I<I<O<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<I<I<O<O<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<I<I<O<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<I<I<O<O<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<I<I<O<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<I<I<O<O<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<I<I<O<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<I<I<O<O<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<I<I<O<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<I<I<O<O<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<I<I<O<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<I<I<O<O<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<I<I<O<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<I<I<O<O<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<I<I<O<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<I<I<O<O<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<I<I<O<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<I<I<O<O<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<I<I<O<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<I<I<O<O<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<I<I<O<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<I<I<O<O<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<I<I<O<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<I<I<O<O<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<I<I<O<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<I<I<O<O<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<I<I<O<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<O<O<I<O<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<O<O<I<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<O<O<I<O<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<O<O<I<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<O<O<I<O<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<O<O<I<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<O<O<I<O<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<O<O<I<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<O<O<I<O<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<O<O<I<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<O<O<I<O<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<O<O<I<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<O<O<I<O<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<O<O<I<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<O<O<I<O<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<O<O<I<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<O<O<I<O<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<O<O<I<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<O<O<I<O<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<O<O<I<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<O<O<I<O<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<O<O<I<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<O<O<I<O<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<O<O<I<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<O<O<I<O<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<O<O<I<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<O<O<I<O<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<O<O<I<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<O<O<I<O<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<O<O<I<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<O<O<I<O<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<O<O<I<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<I<O<I<O<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<I<O<I<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<I<O<I<O<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<I<O<I<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<I<O<I<O<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<I<O<I<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<I<O<I<O<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<I<O<I<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<I<O<I<O<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<I<O<I<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<I<O<I<O<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<I<O<I<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<I<O<I<O<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<I<O<I<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<I<O<I<O<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<I<O<I<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<I<O<I<O<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<I<O<I<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<I<O<I<O<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<I<O<I<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<I<O<I<O<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<I<O<I<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<I<O<I<O<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<I<O<I<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<I<O<I<O<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<I<O<I<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<I<O<I<O<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<I<O<I<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<I<O<I<O<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<I<O<I<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<I<O<I<O<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<I<O<I<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<O<I<I<O<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<O<I<I<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<O<I<I<O<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<O<I<I<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<O<I<I<O<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<O<I<I<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<O<I<I<O<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<O<I<I<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<O<I<I<O<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<O<I<I<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<O<I<I<O<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<O<I<I<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<O<I<I<O<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<O<I<I<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<O<I<I<O<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<O<I<I<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<O<I<I<O<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<O<I<I<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<O<I<I<O<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<O<I<I<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<O<I<I<O<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<O<I<I<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<O<I<I<O<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<O<I<I<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<O<I<I<O<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<O<I<I<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<O<I<I<O<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<O<I<I<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<O<I<I<O<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<O<I<I<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<O<I<I<O<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<O<I<I<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<I<I<I<O<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<I<I<I<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<I<I<I<O<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<I<I<I<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<I<I<I<O<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<I<I<I<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<I<I<I<O<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<I<I<I<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<I<I<I<O<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<I<I<I<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<I<I<I<O<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<I<I<I<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<I<I<I<O<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<I<I<I<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<I<I<I<O<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<I<I<I<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<I<I<I<O<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<I<I<I<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<I<I<I<O<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<I<I<I<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<I<I<I<O<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<I<I<I<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<I<I<I<O<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<I<I<I<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<I<I<I<O<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<I<I<I<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<I<I<I<O<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<I<I<I<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<I<I<I<O<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<I<I<I<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<I<I<I<O<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<I<I<I<O<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<O<O<O<I<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<O<O<O<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<O<O<O<I<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<O<O<O<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<O<O<O<I<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<O<O<O<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<O<O<O<I<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<O<O<O<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<O<O<O<I<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<O<O<O<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<O<O<O<I<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<O<O<O<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<O<O<O<I<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<O<O<O<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<O<O<O<I<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<O<O<O<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<O<O<O<I<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<O<O<O<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<O<O<O<I<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<O<O<O<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<O<O<O<I<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<O<O<O<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<O<O<O<I<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<O<O<O<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<O<O<O<I<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<O<O<O<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<O<O<O<I<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<O<O<O<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<O<O<O<I<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<O<O<O<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<O<O<O<I<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<O<O<O<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<I<O<O<I<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<I<O<O<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<I<O<O<I<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<I<O<O<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<I<O<O<I<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<I<O<O<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<I<O<O<I<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<I<O<O<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<I<O<O<I<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<I<O<O<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<I<O<O<I<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<I<O<O<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<I<O<O<I<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<I<O<O<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<I<O<O<I<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<I<O<O<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<I<O<O<I<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<I<O<O<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<I<O<O<I<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<I<O<O<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<I<O<O<I<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<I<O<O<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<I<O<O<I<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<I<O<O<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<I<O<O<I<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<I<O<O<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<I<O<O<I<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<I<O<O<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<I<O<O<I<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<I<O<O<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<I<O<O<I<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<I<O<O<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<O<I<O<I<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<O<I<O<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<O<I<O<I<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<O<I<O<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<O<I<O<I<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<O<I<O<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<O<I<O<I<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<O<I<O<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<O<I<O<I<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<O<I<O<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<O<I<O<I<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<O<I<O<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<O<I<O<I<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<O<I<O<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<O<I<O<I<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<O<I<O<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<O<I<O<I<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<O<I<O<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<O<I<O<I<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<O<I<O<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<O<I<O<I<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<O<I<O<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<O<I<O<I<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<O<I<O<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<O<I<O<I<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<O<I<O<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<O<I<O<I<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<O<I<O<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<O<I<O<I<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<O<I<O<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<O<I<O<I<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<O<I<O<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<I<I<O<I<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<I<I<O<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<I<I<O<I<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<I<I<O<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<I<I<O<I<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<I<I<O<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<I<I<O<I<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<I<I<O<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<I<I<O<I<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<I<I<O<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<I<I<O<I<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<I<I<O<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<I<I<O<I<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<I<I<O<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<I<I<O<I<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<I<I<O<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<I<I<O<I<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<I<I<O<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<I<I<O<I<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<I<I<O<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<I<I<O<I<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<I<I<O<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<I<I<O<I<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<I<I<O<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<I<I<O<I<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<I<I<O<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<I<I<O<I<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<I<I<O<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<I<I<O<I<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<I<I<O<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<I<I<O<I<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<I<I<O<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<O<O<I<I<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<O<O<I<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<O<O<I<I<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<O<O<I<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<O<O<I<I<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<O<O<I<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<O<O<I<I<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<O<O<I<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<O<O<I<I<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<O<O<I<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<O<O<I<I<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<O<O<I<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<O<O<I<I<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<O<O<I<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<O<O<I<I<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<O<O<I<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<O<O<I<I<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<O<O<I<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<O<O<I<I<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<O<O<I<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<O<O<I<I<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<O<O<I<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<O<O<I<I<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<O<O<I<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<O<O<I<I<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<O<O<I<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<O<O<I<I<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<O<O<I<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<O<O<I<I<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<O<O<I<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<O<O<I<I<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<O<O<I<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<I<O<I<I<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<I<O<I<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<I<O<I<I<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<I<O<I<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<I<O<I<I<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<I<O<I<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<I<O<I<I<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<I<O<I<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<I<O<I<I<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<I<O<I<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<I<O<I<I<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<I<O<I<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<I<O<I<I<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<I<O<I<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<I<O<I<I<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<I<O<I<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<I<O<I<I<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<I<O<I<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<I<O<I<I<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<I<O<I<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<I<O<I<I<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<I<O<I<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<I<O<I<I<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<I<O<I<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<I<O<I<I<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<I<O<I<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<I<O<I<I<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<I<O<I<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<I<O<I<I<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<I<O<I<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<I<O<I<I<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<I<O<I<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<O<I<I<I<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<O<I<I<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<O<I<I<I<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<O<I<I<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<O<I<I<I<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<O<I<I<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<O<I<I<I<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<O<I<I<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<O<I<I<I<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<O<I<I<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<O<I<I<I<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<O<I<I<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<O<I<I<I<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<O<I<I<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<O<I<I<I<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<O<I<I<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<O<I<I<I<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<O<I<I<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<O<I<I<I<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<O<I<I<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<O<I<I<I<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<O<I<I<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<O<I<I<I<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<O<I<I<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<O<I<I<I<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<O<I<I<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<O<I<I<I<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<O<I<I<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<O<I<I<I<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<O<I<I<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<O<I<I<I<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<O<I<I<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<I<I<I<I<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<I<I<I<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<I<I<I<I<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<I<I<I<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<I<I<I<I<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<I<I<I<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<I<I<I<I<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<I<I<I<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<I<I<I<I<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<I<I<I<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<I<I<I<I<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<I<I<I<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<I<I<I<I<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<I<I<I<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<I<I<I<I<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<I<I<I<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<I<I<I<I<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<I<I<I<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<I<I<I<I<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<I<I<I<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<I<I<I<I<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<I<I<I<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<I<I<I<I<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<I<I<I<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<I<I<I<I<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<I<I<I<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<I<I<I<I<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<I<I<I<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<I<I<I<I<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<I<I<I<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<I<I<I<I<I<O<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<I<I<I<I<I<O<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<O<O<O<O<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<O<O<O<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<O<O<O<O<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<O<O<O<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<O<O<O<O<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<O<O<O<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<O<O<O<O<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<O<O<O<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<O<O<O<O<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<O<O<O<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<O<O<O<O<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<O<O<O<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<O<O<O<O<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<O<O<O<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<O<O<O<O<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<O<O<O<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<O<O<O<O<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<O<O<O<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<O<O<O<O<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<O<O<O<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<O<O<O<O<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<O<O<O<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<O<O<O<O<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<O<O<O<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<O<O<O<O<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<O<O<O<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<O<O<O<O<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<O<O<O<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<O<O<O<O<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<O<O<O<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<O<O<O<O<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<O<O<O<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<I<O<O<O<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<I<O<O<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<I<O<O<O<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<I<O<O<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<I<O<O<O<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<I<O<O<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<I<O<O<O<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<I<O<O<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<I<O<O<O<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<I<O<O<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<I<O<O<O<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<I<O<O<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<I<O<O<O<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<I<O<O<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<I<O<O<O<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<I<O<O<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<I<O<O<O<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<I<O<O<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<I<O<O<O<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<I<O<O<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<I<O<O<O<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<I<O<O<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<I<O<O<O<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<I<O<O<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<I<O<O<O<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<I<O<O<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<I<O<O<O<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<I<O<O<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<I<O<O<O<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<I<O<O<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<I<O<O<O<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<I<O<O<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<O<I<O<O<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<O<I<O<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<O<I<O<O<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<O<I<O<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<O<I<O<O<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<O<I<O<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<O<I<O<O<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<O<I<O<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<O<I<O<O<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<O<I<O<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<O<I<O<O<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<O<I<O<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<O<I<O<O<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<O<I<O<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<O<I<O<O<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<O<I<O<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<O<I<O<O<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<O<I<O<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<O<I<O<O<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<O<I<O<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<O<I<O<O<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<O<I<O<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<O<I<O<O<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<O<I<O<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<O<I<O<O<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<O<I<O<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<O<I<O<O<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<O<I<O<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<O<I<O<O<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<O<I<O<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<O<I<O<O<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<O<I<O<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<I<I<O<O<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<I<I<O<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<I<I<O<O<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<I<I<O<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<I<I<O<O<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<I<I<O<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<I<I<O<O<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<I<I<O<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<I<I<O<O<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<I<I<O<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<I<I<O<O<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<I<I<O<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<I<I<O<O<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<I<I<O<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<I<I<O<O<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<I<I<O<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<I<I<O<O<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<I<I<O<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<I<I<O<O<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<I<I<O<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<I<I<O<O<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<I<I<O<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<I<I<O<O<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<I<I<O<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<I<I<O<O<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<I<I<O<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<I<I<O<O<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<I<I<O<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<I<I<O<O<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<I<I<O<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<I<I<O<O<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<I<I<O<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<O<O<I<O<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<O<O<I<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<O<O<I<O<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<O<O<I<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<O<O<I<O<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<O<O<I<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<O<O<I<O<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<O<O<I<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<O<O<I<O<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<O<O<I<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<O<O<I<O<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<O<O<I<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<O<O<I<O<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<O<O<I<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<O<O<I<O<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<O<O<I<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<O<O<I<O<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<O<O<I<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<O<O<I<O<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<O<O<I<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<O<O<I<O<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<O<O<I<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<O<O<I<O<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<O<O<I<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<O<O<I<O<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<O<O<I<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<O<O<I<O<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<O<O<I<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<O<O<I<O<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<O<O<I<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<O<O<I<O<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<O<O<I<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<I<O<I<O<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<I<O<I<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<I<O<I<O<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<I<O<I<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<I<O<I<O<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<I<O<I<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<I<O<I<O<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<I<O<I<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<I<O<I<O<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<I<O<I<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<I<O<I<O<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<I<O<I<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<I<O<I<O<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<I<O<I<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<I<O<I<O<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<I<O<I<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<I<O<I<O<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<I<O<I<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<I<O<I<O<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<I<O<I<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<I<O<I<O<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<I<O<I<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<I<O<I<O<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<I<O<I<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<I<O<I<O<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<I<O<I<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<I<O<I<O<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<I<O<I<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<I<O<I<O<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<I<O<I<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<I<O<I<O<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<I<O<I<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<O<I<I<O<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<O<I<I<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<O<I<I<O<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<O<I<I<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<O<I<I<O<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<O<I<I<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<O<I<I<O<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<O<I<I<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<O<I<I<O<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<O<I<I<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<O<I<I<O<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<O<I<I<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<O<I<I<O<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<O<I<I<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<O<I<I<O<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<O<I<I<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<O<I<I<O<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<O<I<I<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<O<I<I<O<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<O<I<I<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<O<I<I<O<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<O<I<I<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<O<I<I<O<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<O<I<I<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<O<I<I<O<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<O<I<I<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<O<I<I<O<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<O<I<I<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<O<I<I<O<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<O<I<I<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<O<I<I<O<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<O<I<I<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<I<I<I<O<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<I<I<I<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<I<I<I<O<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<I<I<I<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<I<I<I<O<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<I<I<I<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<I<I<I<O<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<I<I<I<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<I<I<I<O<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<I<I<I<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<I<I<I<O<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<I<I<I<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<I<I<I<O<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<I<I<I<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<I<I<I<O<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<I<I<I<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<I<I<I<O<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<I<I<I<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<I<I<I<O<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<I<I<I<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<I<I<I<O<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<I<I<I<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<I<I<I<O<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<I<I<I<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<I<I<I<O<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<I<I<I<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<I<I<I<O<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<I<I<I<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<I<I<I<O<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<I<I<I<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<I<I<I<O<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<I<I<I<O<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<O<O<O<I<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<O<O<O<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<O<O<O<I<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<O<O<O<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<O<O<O<I<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<O<O<O<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<O<O<O<I<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<O<O<O<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<O<O<O<I<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<O<O<O<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<O<O<O<I<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<O<O<O<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<O<O<O<I<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<O<O<O<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<O<O<O<I<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<O<O<O<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<O<O<O<I<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<O<O<O<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<O<O<O<I<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<O<O<O<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<O<O<O<I<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<O<O<O<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<O<O<O<I<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<O<O<O<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<O<O<O<I<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<O<O<O<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<O<O<O<I<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<O<O<O<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<O<O<O<I<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<O<O<O<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<O<O<O<I<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<O<O<O<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<I<O<O<I<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<I<O<O<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<I<O<O<I<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<I<O<O<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<I<O<O<I<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<I<O<O<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<I<O<O<I<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<I<O<O<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<I<O<O<I<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<I<O<O<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<I<O<O<I<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<I<O<O<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<I<O<O<I<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<I<O<O<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<I<O<O<I<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<I<O<O<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<I<O<O<I<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<I<O<O<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<I<O<O<I<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<I<O<O<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<I<O<O<I<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<I<O<O<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<I<O<O<I<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<I<O<O<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<I<O<O<I<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<I<O<O<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<I<O<O<I<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<I<O<O<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<I<O<O<I<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<I<O<O<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<I<O<O<I<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<I<O<O<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<O<I<O<I<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<O<I<O<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<O<I<O<I<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<O<I<O<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<O<I<O<I<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<O<I<O<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<O<I<O<I<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<O<I<O<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<O<I<O<I<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<O<I<O<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<O<I<O<I<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<O<I<O<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<O<I<O<I<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<O<I<O<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<O<I<O<I<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<O<I<O<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<O<I<O<I<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<O<I<O<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<O<I<O<I<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<O<I<O<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<O<I<O<I<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<O<I<O<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<O<I<O<I<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<O<I<O<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<O<I<O<I<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<O<I<O<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<O<I<O<I<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<O<I<O<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<O<I<O<I<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<O<I<O<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<O<I<O<I<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<O<I<O<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<I<I<O<I<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<I<I<O<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<I<I<O<I<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<I<I<O<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<I<I<O<I<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<I<I<O<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<I<I<O<I<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<I<I<O<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<I<I<O<I<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<I<I<O<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<I<I<O<I<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<I<I<O<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<I<I<O<I<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<I<I<O<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<I<I<O<I<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<I<I<O<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<I<I<O<I<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<I<I<O<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<I<I<O<I<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<I<I<O<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<I<I<O<I<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<I<I<O<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<I<I<O<I<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<I<I<O<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<I<I<O<I<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<I<I<O<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<I<I<O<I<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<I<I<O<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<I<I<O<I<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<I<I<O<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<I<I<O<I<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<I<I<O<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<O<O<I<I<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<O<O<I<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<O<O<I<I<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<O<O<I<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<O<O<I<I<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<O<O<I<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<O<O<I<I<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<O<O<I<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<O<O<I<I<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<O<O<I<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<O<O<I<I<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<O<O<I<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<O<O<I<I<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<O<O<I<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<O<O<I<I<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<O<O<I<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<O<O<I<I<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<O<O<I<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<O<O<I<I<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<O<O<I<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<O<O<I<I<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<O<O<I<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<O<O<I<I<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<O<O<I<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<O<O<I<I<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<O<O<I<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<O<O<I<I<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<O<O<I<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<O<O<I<I<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<O<O<I<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<O<O<I<I<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<O<O<I<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<I<O<I<I<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<I<O<I<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<I<O<I<I<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<I<O<I<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<I<O<I<I<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<I<O<I<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<I<O<I<I<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<I<O<I<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<I<O<I<I<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<I<O<I<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<I<O<I<I<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<I<O<I<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<I<O<I<I<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<I<O<I<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<I<O<I<I<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<I<O<I<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<I<O<I<I<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<I<O<I<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<I<O<I<I<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<I<O<I<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<I<O<I<I<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<I<O<I<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<I<O<I<I<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<I<O<I<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<I<O<I<I<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<I<O<I<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<I<O<I<I<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<I<O<I<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<I<O<I<I<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<I<O<I<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<I<O<I<I<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<I<O<I<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<O<I<I<I<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<O<I<I<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<O<I<I<I<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<O<I<I<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<O<I<I<I<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<O<I<I<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<O<I<I<I<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<O<I<I<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<O<I<I<I<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<O<I<I<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<O<I<I<I<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<O<I<I<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<O<I<I<I<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<O<I<I<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<O<I<I<I<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<O<I<I<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<O<I<I<I<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<O<I<I<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<O<I<I<I<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<O<I<I<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<O<I<I<I<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<O<I<I<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<O<I<I<I<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<O<I<I<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<O<I<I<I<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<O<I<I<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<O<I<I<I<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<O<I<I<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<O<I<I<I<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<O<I<I<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<O<I<I<I<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<O<I<I<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<I<I<I<I<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<I<I<I<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<I<I<I<I<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<I<I<I<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<I<I<I<I<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<I<I<I<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<I<I<I<I<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<I<I<I<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<I<I<I<I<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<I<I<I<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<I<I<I<I<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<I<I<I<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<I<I<I<I<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<I<I<I<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<I<I<I<I<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<I<I<I<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<I<I<I<I<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<I<I<I<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<I<I<I<I<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<I<I<I<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<I<I<I<I<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<I<I<I<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<I<I<I<I<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<I<I<I<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<I<I<I<I<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<I<I<I<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<I<I<I<I<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<I<I<I<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<I<I<I<I<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<I<I<I<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<I<I<I<I<O<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<I<I<I<I<O<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<O<O<O<O<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<O<O<O<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<O<O<O<O<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<O<O<O<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<O<O<O<O<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<O<O<O<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<O<O<O<O<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<O<O<O<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<O<O<O<O<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<O<O<O<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<O<O<O<O<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<O<O<O<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<O<O<O<O<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<O<O<O<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<O<O<O<O<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<O<O<O<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<O<O<O<O<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<O<O<O<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<O<O<O<O<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<O<O<O<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<O<O<O<O<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<O<O<O<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<O<O<O<O<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<O<O<O<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<O<O<O<O<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<O<O<O<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<O<O<O<O<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<O<O<O<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<O<O<O<O<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<O<O<O<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<O<O<O<O<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<O<O<O<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<I<O<O<O<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<I<O<O<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<I<O<O<O<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<I<O<O<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<I<O<O<O<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<I<O<O<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<I<O<O<O<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<I<O<O<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<I<O<O<O<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<I<O<O<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<I<O<O<O<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<I<O<O<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<I<O<O<O<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<I<O<O<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<I<O<O<O<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<I<O<O<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<I<O<O<O<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<I<O<O<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<I<O<O<O<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<I<O<O<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<I<O<O<O<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<I<O<O<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<I<O<O<O<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<I<O<O<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<I<O<O<O<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<I<O<O<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<I<O<O<O<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<I<O<O<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<I<O<O<O<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<I<O<O<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<I<O<O<O<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<I<O<O<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<O<I<O<O<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<O<I<O<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<O<I<O<O<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<O<I<O<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<O<I<O<O<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<O<I<O<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<O<I<O<O<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<O<I<O<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<O<I<O<O<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<O<I<O<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<O<I<O<O<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<O<I<O<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<O<I<O<O<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<O<I<O<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<O<I<O<O<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<O<I<O<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<O<I<O<O<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<O<I<O<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<O<I<O<O<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<O<I<O<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<O<I<O<O<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<O<I<O<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<O<I<O<O<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<O<I<O<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<O<I<O<O<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<O<I<O<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<O<I<O<O<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<O<I<O<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<O<I<O<O<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<O<I<O<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<O<I<O<O<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<O<I<O<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<I<I<O<O<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<I<I<O<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<I<I<O<O<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<I<I<O<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<I<I<O<O<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<I<I<O<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<I<I<O<O<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<I<I<O<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<I<I<O<O<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<I<I<O<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<I<I<O<O<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<I<I<O<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<I<I<O<O<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<I<I<O<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<I<I<O<O<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<I<I<O<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<I<I<O<O<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<I<I<O<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<I<I<O<O<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<I<I<O<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<I<I<O<O<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<I<I<O<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<I<I<O<O<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<I<I<O<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<I<I<O<O<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<I<I<O<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<I<I<O<O<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<I<I<O<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<I<I<O<O<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<I<I<O<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<I<I<O<O<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<I<I<O<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<O<O<I<O<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<O<O<I<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<O<O<I<O<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<O<O<I<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<O<O<I<O<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<O<O<I<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<O<O<I<O<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<O<O<I<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<O<O<I<O<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<O<O<I<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<O<O<I<O<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<O<O<I<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<O<O<I<O<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<O<O<I<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<O<O<I<O<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<O<O<I<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<O<O<I<O<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<O<O<I<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<O<O<I<O<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<O<O<I<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<O<O<I<O<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<O<O<I<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<O<O<I<O<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<O<O<I<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<O<O<I<O<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<O<O<I<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<O<O<I<O<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<O<O<I<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<O<O<I<O<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<O<O<I<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<O<O<I<O<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<O<O<I<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<I<O<I<O<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<I<O<I<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<I<O<I<O<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<I<O<I<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<I<O<I<O<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<I<O<I<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<I<O<I<O<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<I<O<I<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<I<O<I<O<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<I<O<I<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<I<O<I<O<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<I<O<I<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<I<O<I<O<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<I<O<I<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<I<O<I<O<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<I<O<I<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<I<O<I<O<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<I<O<I<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<I<O<I<O<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<I<O<I<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<I<O<I<O<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<I<O<I<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<I<O<I<O<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<I<O<I<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<I<O<I<O<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<I<O<I<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<I<O<I<O<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<I<O<I<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<I<O<I<O<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<I<O<I<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<I<O<I<O<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<I<O<I<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<O<I<I<O<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<O<I<I<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<O<I<I<O<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<O<I<I<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<O<I<I<O<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<O<I<I<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<O<I<I<O<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<O<I<I<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<O<I<I<O<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<O<I<I<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<O<I<I<O<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<O<I<I<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<O<I<I<O<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<O<I<I<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<O<I<I<O<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<O<I<I<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<O<I<I<O<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<O<I<I<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<O<I<I<O<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<O<I<I<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<O<I<I<O<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<O<I<I<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<O<I<I<O<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<O<I<I<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<O<I<I<O<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<O<I<I<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<O<I<I<O<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<O<I<I<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<O<I<I<O<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<O<I<I<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<O<I<I<O<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<O<I<I<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<I<I<I<O<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<I<I<I<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<I<I<I<O<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<I<I<I<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<I<I<I<O<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<I<I<I<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<I<I<I<O<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<I<I<I<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<I<I<I<O<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<I<I<I<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<I<I<I<O<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<I<I<I<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<I<I<I<O<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<I<I<I<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<I<I<I<O<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<I<I<I<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<I<I<I<O<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<I<I<I<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<I<I<I<O<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<I<I<I<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<I<I<I<O<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<I<I<I<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<I<I<I<O<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<I<I<I<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<I<I<I<O<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<I<I<I<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<I<I<I<O<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<I<I<I<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<I<I<I<O<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<I<I<I<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<I<I<I<O<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<I<I<I<O<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<O<O<O<I<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<O<O<O<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<O<O<O<I<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<O<O<O<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<O<O<O<I<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<O<O<O<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<O<O<O<I<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<O<O<O<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<O<O<O<I<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<O<O<O<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<O<O<O<I<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<O<O<O<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<O<O<O<I<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<O<O<O<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<O<O<O<I<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<O<O<O<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<O<O<O<I<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<O<O<O<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<O<O<O<I<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<O<O<O<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<O<O<O<I<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<O<O<O<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<O<O<O<I<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<O<O<O<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<O<O<O<I<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<O<O<O<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<O<O<O<I<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<O<O<O<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<O<O<O<I<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<O<O<O<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<O<O<O<I<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<O<O<O<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<I<O<O<I<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<I<O<O<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<I<O<O<I<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<I<O<O<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<I<O<O<I<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<I<O<O<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<I<O<O<I<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<I<O<O<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<I<O<O<I<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<I<O<O<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<I<O<O<I<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<I<O<O<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<I<O<O<I<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<I<O<O<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<I<O<O<I<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<I<O<O<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<I<O<O<I<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<I<O<O<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<I<O<O<I<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<I<O<O<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<I<O<O<I<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<I<O<O<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<I<O<O<I<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<I<O<O<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<I<O<O<I<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<I<O<O<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<I<O<O<I<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<I<O<O<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<I<O<O<I<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<I<O<O<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<I<O<O<I<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<I<O<O<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<O<I<O<I<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<O<I<O<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<O<I<O<I<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<O<I<O<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<O<I<O<I<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<O<I<O<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<O<I<O<I<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<O<I<O<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<O<I<O<I<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<O<I<O<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<O<I<O<I<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<O<I<O<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<O<I<O<I<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<O<I<O<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<O<I<O<I<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<O<I<O<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<O<I<O<I<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<O<I<O<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<O<I<O<I<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<O<I<O<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<O<I<O<I<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<O<I<O<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<O<I<O<I<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<O<I<O<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<O<I<O<I<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<O<I<O<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<O<I<O<I<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<O<I<O<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<O<I<O<I<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<O<I<O<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<O<I<O<I<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<O<I<O<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<I<I<O<I<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<I<I<O<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<I<I<O<I<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<I<I<O<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<I<I<O<I<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<I<I<O<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<I<I<O<I<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<I<I<O<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<I<I<O<I<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<I<I<O<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<I<I<O<I<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<I<I<O<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<I<I<O<I<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<I<I<O<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<I<I<O<I<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<I<I<O<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<I<I<O<I<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<I<I<O<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<I<I<O<I<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<I<I<O<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<I<I<O<I<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<I<I<O<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<I<I<O<I<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<I<I<O<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<I<I<O<I<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<I<I<O<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<I<I<O<I<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<I<I<O<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<I<I<O<I<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<I<I<O<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<I<I<O<I<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<I<I<O<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<O<O<I<I<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<O<O<I<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<O<O<I<I<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<O<O<I<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<O<O<I<I<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<O<O<I<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<O<O<I<I<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<O<O<I<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<O<O<I<I<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<O<O<I<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<O<O<I<I<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<O<O<I<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<O<O<I<I<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<O<O<I<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<O<O<I<I<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<O<O<I<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<O<O<I<I<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<O<O<I<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<O<O<I<I<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<O<O<I<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<O<O<I<I<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<O<O<I<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<O<O<I<I<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<O<O<I<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<O<O<I<I<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<O<O<I<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<O<O<I<I<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<O<O<I<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<O<O<I<I<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<O<O<I<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<O<O<I<I<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<O<O<I<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<I<O<I<I<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<I<O<I<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<I<O<I<I<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<I<O<I<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<I<O<I<I<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<I<O<I<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<I<O<I<I<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<I<O<I<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<I<O<I<I<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<I<O<I<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<I<O<I<I<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<I<O<I<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<I<O<I<I<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<I<O<I<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<I<O<I<I<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<I<O<I<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<I<O<I<I<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<I<O<I<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<I<O<I<I<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<I<O<I<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<I<O<I<I<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<I<O<I<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<I<O<I<I<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<I<O<I<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<I<O<I<I<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<I<O<I<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<I<O<I<I<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<I<O<I<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<I<O<I<I<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<I<O<I<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<I<O<I<I<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<I<O<I<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<O<I<I<I<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<O<I<I<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<O<I<I<I<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<O<I<I<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<O<I<I<I<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<O<I<I<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<O<I<I<I<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<O<I<I<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<O<I<I<I<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<O<I<I<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<O<I<I<I<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<O<I<I<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<O<I<I<I<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<O<I<I<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<O<I<I<I<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<O<I<I<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<O<I<I<I<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<O<I<I<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<O<I<I<I<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<O<I<I<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<O<I<I<I<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<O<I<I<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<O<I<I<I<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<O<I<I<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<O<I<I<I<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<O<I<I<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<O<I<I<I<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<O<I<I<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<O<I<I<I<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<O<I<I<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<O<I<I<I<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<O<I<I<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<I<I<I<I<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<I<I<I<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<I<I<I<I<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<I<I<I<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<I<I<I<I<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<I<I<I<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<I<I<I<I<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<I<I<I<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<I<I<I<I<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<I<I<I<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<I<I<I<I<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<I<I<I<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<I<I<I<I<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<I<I<I<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<I<I<I<I<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<I<I<I<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<I<I<I<I<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<I<I<I<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<I<I<I<I<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<I<I<I<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<I<I<I<I<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<I<I<I<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<I<I<I<I<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<I<I<I<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<I<I<I<I<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<I<I<I<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<I<I<I<I<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<I<I<I<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<I<I<I<I<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<I<I<I<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<I<I<I<I<I<I<O<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<I<I<I<I<I<I<O<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<O<O<O<O<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<O<O<O<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<O<O<O<O<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<O<O<O<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<O<O<O<O<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<O<O<O<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<O<O<O<O<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<O<O<O<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<O<O<O<O<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<O<O<O<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<O<O<O<O<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<O<O<O<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<O<O<O<O<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<O<O<O<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<O<O<O<O<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<O<O<O<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<O<O<O<O<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<O<O<O<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<O<O<O<O<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<O<O<O<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<O<O<O<O<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<O<O<O<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<O<O<O<O<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<O<O<O<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<O<O<O<O<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<O<O<O<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<O<O<O<O<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<O<O<O<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<O<O<O<O<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<O<O<O<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<O<O<O<O<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<O<O<O<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<I<O<O<O<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<I<O<O<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<I<O<O<O<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<I<O<O<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<I<O<O<O<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<I<O<O<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<I<O<O<O<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<I<O<O<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<I<O<O<O<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<I<O<O<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<I<O<O<O<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<I<O<O<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<I<O<O<O<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<I<O<O<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<I<O<O<O<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<I<O<O<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<I<O<O<O<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<I<O<O<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<I<O<O<O<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<I<O<O<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<I<O<O<O<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<I<O<O<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<I<O<O<O<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<I<O<O<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<I<O<O<O<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<I<O<O<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<I<O<O<O<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<I<O<O<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<I<O<O<O<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<I<O<O<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<I<O<O<O<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<I<O<O<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<O<I<O<O<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<O<I<O<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<O<I<O<O<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<O<I<O<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<O<I<O<O<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<O<I<O<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<O<I<O<O<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<O<I<O<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<O<I<O<O<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<O<I<O<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<O<I<O<O<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<O<I<O<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<O<I<O<O<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<O<I<O<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<O<I<O<O<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<O<I<O<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<O<I<O<O<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<O<I<O<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<O<I<O<O<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<O<I<O<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<O<I<O<O<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<O<I<O<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<O<I<O<O<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<O<I<O<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<O<I<O<O<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<O<I<O<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<O<I<O<O<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<O<I<O<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<O<I<O<O<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<O<I<O<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<O<I<O<O<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<O<I<O<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<I<I<O<O<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<I<I<O<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<I<I<O<O<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<I<I<O<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<I<I<O<O<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<I<I<O<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<I<I<O<O<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<I<I<O<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<I<I<O<O<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<I<I<O<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<I<I<O<O<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<I<I<O<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<I<I<O<O<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<I<I<O<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<I<I<O<O<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<I<I<O<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<I<I<O<O<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<I<I<O<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<I<I<O<O<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<I<I<O<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<I<I<O<O<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<I<I<O<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<I<I<O<O<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<I<I<O<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<I<I<O<O<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<I<I<O<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<I<I<O<O<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<I<I<O<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<I<I<O<O<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<I<I<O<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<I<I<O<O<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<I<I<O<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<O<O<I<O<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<O<O<I<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<O<O<I<O<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<O<O<I<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<O<O<I<O<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<O<O<I<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<O<O<I<O<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<O<O<I<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<O<O<I<O<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<O<O<I<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<O<O<I<O<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<O<O<I<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<O<O<I<O<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<O<O<I<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<O<O<I<O<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<O<O<I<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<O<O<I<O<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<O<O<I<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<O<O<I<O<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<O<O<I<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<O<O<I<O<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<O<O<I<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<O<O<I<O<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<O<O<I<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<O<O<I<O<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<O<O<I<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<O<O<I<O<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<O<O<I<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<O<O<I<O<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<O<O<I<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<O<O<I<O<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<O<O<I<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<I<O<I<O<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<I<O<I<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<I<O<I<O<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<I<O<I<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<I<O<I<O<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<I<O<I<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<I<O<I<O<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<I<O<I<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<I<O<I<O<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<I<O<I<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<I<O<I<O<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<I<O<I<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<I<O<I<O<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<I<O<I<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<I<O<I<O<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<I<O<I<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<I<O<I<O<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<I<O<I<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<I<O<I<O<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<I<O<I<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<I<O<I<O<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<I<O<I<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<I<O<I<O<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<I<O<I<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<I<O<I<O<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<I<O<I<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<I<O<I<O<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<I<O<I<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<I<O<I<O<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<I<O<I<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<I<O<I<O<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<I<O<I<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<O<I<I<O<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<O<I<I<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<O<I<I<O<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<O<I<I<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<O<I<I<O<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<O<I<I<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<O<I<I<O<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<O<I<I<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<O<I<I<O<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<O<I<I<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<O<I<I<O<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<O<I<I<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<O<I<I<O<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<O<I<I<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<O<I<I<O<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<O<I<I<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<O<I<I<O<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<O<I<I<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<O<I<I<O<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<O<I<I<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<O<I<I<O<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<O<I<I<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<O<I<I<O<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<O<I<I<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<O<I<I<O<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<O<I<I<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<O<I<I<O<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<O<I<I<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<O<I<I<O<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<O<I<I<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<O<I<I<O<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<O<I<I<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<I<I<I<O<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<I<I<I<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<I<I<I<O<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<I<I<I<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<I<I<I<O<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<I<I<I<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<I<I<I<O<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<I<I<I<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<I<I<I<O<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<I<I<I<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<I<I<I<O<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<I<I<I<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<I<I<I<O<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<I<I<I<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<I<I<I<O<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<I<I<I<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<I<I<I<O<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<I<I<I<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<I<I<I<O<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<I<I<I<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<I<I<I<O<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<I<I<I<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<I<I<I<O<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<I<I<I<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<I<I<I<O<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<I<I<I<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<I<I<I<O<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<I<I<I<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<I<I<I<O<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<I<I<I<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<I<I<I<O<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<I<I<I<O<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<O<O<O<I<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<O<O<O<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<O<O<O<I<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<O<O<O<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<O<O<O<I<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<O<O<O<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<O<O<O<I<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<O<O<O<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<O<O<O<I<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<O<O<O<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<O<O<O<I<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<O<O<O<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<O<O<O<I<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<O<O<O<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<O<O<O<I<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<O<O<O<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<O<O<O<I<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<O<O<O<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<O<O<O<I<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<O<O<O<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<O<O<O<I<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<O<O<O<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<O<O<O<I<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<O<O<O<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<O<O<O<I<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<O<O<O<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<O<O<O<I<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<O<O<O<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<O<O<O<I<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<O<O<O<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<O<O<O<I<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<O<O<O<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<I<O<O<I<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<I<O<O<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<I<O<O<I<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<I<O<O<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<I<O<O<I<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<I<O<O<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<I<O<O<I<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<I<O<O<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<I<O<O<I<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<I<O<O<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<I<O<O<I<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<I<O<O<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<I<O<O<I<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<I<O<O<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<I<O<O<I<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<I<O<O<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<I<O<O<I<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<I<O<O<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<I<O<O<I<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<I<O<O<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<I<O<O<I<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<I<O<O<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<I<O<O<I<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<I<O<O<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<I<O<O<I<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<I<O<O<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<I<O<O<I<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<I<O<O<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<I<O<O<I<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<I<O<O<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<I<O<O<I<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<I<O<O<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<O<I<O<I<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<O<I<O<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<O<I<O<I<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<O<I<O<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<O<I<O<I<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<O<I<O<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<O<I<O<I<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<O<I<O<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<O<I<O<I<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<O<I<O<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<O<I<O<I<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<O<I<O<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<O<I<O<I<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<O<I<O<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<O<I<O<I<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<O<I<O<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<O<I<O<I<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<O<I<O<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<O<I<O<I<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<O<I<O<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<O<I<O<I<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<O<I<O<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<O<I<O<I<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<O<I<O<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<O<I<O<I<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<O<I<O<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<O<I<O<I<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<O<I<O<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<O<I<O<I<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<O<I<O<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<O<I<O<I<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<O<I<O<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<I<I<O<I<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<I<I<O<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<I<I<O<I<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<I<I<O<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<I<I<O<I<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<I<I<O<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<I<I<O<I<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<I<I<O<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<I<I<O<I<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<I<I<O<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<I<I<O<I<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<I<I<O<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<I<I<O<I<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<I<I<O<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<I<I<O<I<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<I<I<O<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<I<I<O<I<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<I<I<O<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<I<I<O<I<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<I<I<O<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<I<I<O<I<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<I<I<O<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<I<I<O<I<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<I<I<O<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<I<I<O<I<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<I<I<O<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<I<I<O<I<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<I<I<O<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<I<I<O<I<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<I<I<O<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<I<I<O<I<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<I<I<O<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<O<O<I<I<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<O<O<I<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<O<O<I<I<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<O<O<I<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<O<O<I<I<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<O<O<I<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<O<O<I<I<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<O<O<I<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<O<O<I<I<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<O<O<I<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<O<O<I<I<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<O<O<I<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<O<O<I<I<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<O<O<I<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<O<O<I<I<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<O<O<I<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<O<O<I<I<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<O<O<I<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<O<O<I<I<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<O<O<I<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<O<O<I<I<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<O<O<I<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<O<O<I<I<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<O<O<I<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<O<O<I<I<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<O<O<I<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<O<O<I<I<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<O<O<I<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<O<O<I<I<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<O<O<I<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<O<O<I<I<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<O<O<I<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<I<O<I<I<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<I<O<I<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<I<O<I<I<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<I<O<I<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<I<O<I<I<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<I<O<I<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<I<O<I<I<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<I<O<I<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<I<O<I<I<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<I<O<I<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<I<O<I<I<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<I<O<I<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<I<O<I<I<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<I<O<I<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<I<O<I<I<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<I<O<I<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<I<O<I<I<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<I<O<I<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<I<O<I<I<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<I<O<I<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<I<O<I<I<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<I<O<I<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<I<O<I<I<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<I<O<I<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<I<O<I<I<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<I<O<I<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<I<O<I<I<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<I<O<I<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<I<O<I<I<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<I<O<I<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<I<O<I<I<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<I<O<I<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<O<I<I<I<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<O<I<I<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<O<I<I<I<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<O<I<I<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<O<I<I<I<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<O<I<I<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<O<I<I<I<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<O<I<I<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<O<I<I<I<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<O<I<I<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<O<I<I<I<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<O<I<I<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<O<I<I<I<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<O<I<I<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<O<I<I<I<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<O<I<I<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<O<I<I<I<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<O<I<I<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<O<I<I<I<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<O<I<I<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<O<I<I<I<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<O<I<I<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<O<I<I<I<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<O<I<I<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<O<I<I<I<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<O<I<I<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<O<I<I<I<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<O<I<I<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<O<I<I<I<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<O<I<I<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<O<I<I<I<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<O<I<I<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<I<I<I<I<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<I<I<I<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<I<I<I<I<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<I<I<I<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<I<I<I<I<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<I<I<I<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<I<I<I<I<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<I<I<I<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<I<I<I<I<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<I<I<I<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<I<I<I<I<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<I<I<I<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<I<I<I<I<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<I<I<I<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<I<I<I<I<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<I<I<I<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<I<I<I<I<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<I<I<I<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<I<I<I<I<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<I<I<I<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<I<I<I<I<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<I<I<I<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<I<I<I<I<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<I<I<I<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<I<I<I<I<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<I<I<I<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<I<I<I<I<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<I<I<I<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<I<I<I<I<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<I<I<I<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<I<I<I<I<O<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<I<I<I<I<O<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<O<O<O<O<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<O<O<O<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<O<O<O<O<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<O<O<O<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<O<O<O<O<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<O<O<O<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<O<O<O<O<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<O<O<O<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<O<O<O<O<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<O<O<O<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<O<O<O<O<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<O<O<O<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<O<O<O<O<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<O<O<O<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<O<O<O<O<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<O<O<O<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<O<O<O<O<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<O<O<O<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<O<O<O<O<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<O<O<O<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<O<O<O<O<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<O<O<O<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<O<O<O<O<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<O<O<O<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<O<O<O<O<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<O<O<O<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<O<O<O<O<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<O<O<O<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<O<O<O<O<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<O<O<O<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<O<O<O<O<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<O<O<O<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<I<O<O<O<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<I<O<O<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<I<O<O<O<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<I<O<O<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<I<O<O<O<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<I<O<O<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<I<O<O<O<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<I<O<O<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<I<O<O<O<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<I<O<O<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<I<O<O<O<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<I<O<O<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<I<O<O<O<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<I<O<O<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<I<O<O<O<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<I<O<O<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<I<O<O<O<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<I<O<O<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<I<O<O<O<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<I<O<O<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<I<O<O<O<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<I<O<O<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<I<O<O<O<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<I<O<O<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<I<O<O<O<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<I<O<O<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<I<O<O<O<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<I<O<O<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<I<O<O<O<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<I<O<O<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<I<O<O<O<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<I<O<O<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<O<I<O<O<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<O<I<O<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<O<I<O<O<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<O<I<O<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<O<I<O<O<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<O<I<O<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<O<I<O<O<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<O<I<O<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<O<I<O<O<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<O<I<O<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<O<I<O<O<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<O<I<O<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<O<I<O<O<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<O<I<O<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<O<I<O<O<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<O<I<O<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<O<I<O<O<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<O<I<O<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<O<I<O<O<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<O<I<O<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<O<I<O<O<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<O<I<O<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<O<I<O<O<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<O<I<O<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<O<I<O<O<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<O<I<O<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<O<I<O<O<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<O<I<O<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<O<I<O<O<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<O<I<O<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<O<I<O<O<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<O<I<O<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<I<I<O<O<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<I<I<O<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<I<I<O<O<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<I<I<O<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<I<I<O<O<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<I<I<O<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<I<I<O<O<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<I<I<O<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<I<I<O<O<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<I<I<O<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<I<I<O<O<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<I<I<O<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<I<I<O<O<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<I<I<O<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<I<I<O<O<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<I<I<O<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<I<I<O<O<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<I<I<O<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<I<I<O<O<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<I<I<O<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<I<I<O<O<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<I<I<O<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<I<I<O<O<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<I<I<O<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<I<I<O<O<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<I<I<O<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<I<I<O<O<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<I<I<O<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<I<I<O<O<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<I<I<O<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<I<I<O<O<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<I<I<O<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<O<O<I<O<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<O<O<I<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<O<O<I<O<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<O<O<I<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<O<O<I<O<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<O<O<I<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<O<O<I<O<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<O<O<I<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<O<O<I<O<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<O<O<I<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<O<O<I<O<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<O<O<I<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<O<O<I<O<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<O<O<I<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<O<O<I<O<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<O<O<I<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<O<O<I<O<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<O<O<I<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<O<O<I<O<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<O<O<I<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<O<O<I<O<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<O<O<I<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<O<O<I<O<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<O<O<I<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<O<O<I<O<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<O<O<I<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<O<O<I<O<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<O<O<I<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<O<O<I<O<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<O<O<I<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<O<O<I<O<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<O<O<I<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<I<O<I<O<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<I<O<I<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<I<O<I<O<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<I<O<I<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<I<O<I<O<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<I<O<I<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<I<O<I<O<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<I<O<I<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<I<O<I<O<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<I<O<I<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<I<O<I<O<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<I<O<I<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<I<O<I<O<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<I<O<I<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<I<O<I<O<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<I<O<I<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<I<O<I<O<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<I<O<I<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<I<O<I<O<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<I<O<I<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<I<O<I<O<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<I<O<I<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<I<O<I<O<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<I<O<I<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<I<O<I<O<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<I<O<I<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<I<O<I<O<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<I<O<I<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<I<O<I<O<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<I<O<I<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<I<O<I<O<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<I<O<I<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<O<I<I<O<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<O<I<I<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<O<I<I<O<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<O<I<I<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<O<I<I<O<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<O<I<I<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<O<I<I<O<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<O<I<I<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<O<I<I<O<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<O<I<I<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<O<I<I<O<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<O<I<I<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<O<I<I<O<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<O<I<I<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<O<I<I<O<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<O<I<I<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<O<I<I<O<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<O<I<I<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<O<I<I<O<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<O<I<I<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<O<I<I<O<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<O<I<I<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<O<I<I<O<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<O<I<I<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<O<I<I<O<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<O<I<I<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<O<I<I<O<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<O<I<I<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<O<I<I<O<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<O<I<I<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<O<I<I<O<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<O<I<I<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<I<I<I<O<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<I<I<I<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<I<I<I<O<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<I<I<I<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<I<I<I<O<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<I<I<I<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<I<I<I<O<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<I<I<I<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<I<I<I<O<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<I<I<I<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<I<I<I<O<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<I<I<I<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<I<I<I<O<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<I<I<I<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<I<I<I<O<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<I<I<I<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<I<I<I<O<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<I<I<I<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<I<I<I<O<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<I<I<I<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<I<I<I<O<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<I<I<I<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<I<I<I<O<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<I<I<I<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<I<I<I<O<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<I<I<I<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<I<I<I<O<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<I<I<I<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<I<I<I<O<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<I<I<I<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<I<I<I<O<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<I<I<I<O<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<O<O<O<I<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<O<O<O<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<O<O<O<I<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<O<O<O<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<O<O<O<I<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<O<O<O<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<O<O<O<I<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<O<O<O<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<O<O<O<I<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<O<O<O<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<O<O<O<I<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<O<O<O<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<O<O<O<I<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<O<O<O<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<O<O<O<I<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<O<O<O<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<O<O<O<I<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<O<O<O<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<O<O<O<I<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<O<O<O<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<O<O<O<I<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<O<O<O<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<O<O<O<I<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<O<O<O<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<O<O<O<I<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<O<O<O<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<O<O<O<I<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<O<O<O<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<O<O<O<I<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<O<O<O<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<O<O<O<I<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<O<O<O<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<I<O<O<I<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<I<O<O<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<I<O<O<I<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<I<O<O<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<I<O<O<I<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<I<O<O<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<I<O<O<I<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<I<O<O<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<I<O<O<I<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<I<O<O<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<I<O<O<I<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<I<O<O<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<I<O<O<I<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<I<O<O<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<I<O<O<I<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<I<O<O<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<I<O<O<I<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<I<O<O<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<I<O<O<I<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<I<O<O<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<I<O<O<I<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<I<O<O<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<I<O<O<I<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<I<O<O<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<I<O<O<I<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<I<O<O<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<I<O<O<I<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<I<O<O<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<I<O<O<I<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<I<O<O<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<I<O<O<I<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<I<O<O<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<O<I<O<I<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<O<I<O<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<O<I<O<I<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<O<I<O<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<O<I<O<I<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<O<I<O<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<O<I<O<I<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<O<I<O<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<O<I<O<I<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<O<I<O<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<O<I<O<I<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<O<I<O<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<O<I<O<I<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<O<I<O<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<O<I<O<I<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<O<I<O<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<O<I<O<I<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<O<I<O<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<O<I<O<I<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<O<I<O<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<O<I<O<I<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<O<I<O<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<O<I<O<I<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<O<I<O<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<O<I<O<I<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<O<I<O<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<O<I<O<I<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<O<I<O<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<O<I<O<I<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<O<I<O<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<O<I<O<I<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<O<I<O<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<I<I<O<I<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<I<I<O<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<I<I<O<I<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<I<I<O<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<I<I<O<I<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<I<I<O<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<I<I<O<I<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<I<I<O<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<I<I<O<I<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<I<I<O<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<I<I<O<I<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<I<I<O<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<I<I<O<I<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<I<I<O<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<I<I<O<I<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<I<I<O<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<I<I<O<I<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<I<I<O<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<I<I<O<I<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<I<I<O<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<I<I<O<I<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<I<I<O<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<I<I<O<I<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<I<I<O<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<I<I<O<I<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<I<I<O<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<I<I<O<I<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<I<I<O<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<I<I<O<I<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<I<I<O<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<I<I<O<I<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<I<I<O<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<O<O<I<I<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<O<O<I<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<O<O<I<I<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<O<O<I<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<O<O<I<I<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<O<O<I<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<O<O<I<I<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<O<O<I<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<O<O<I<I<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<O<O<I<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<O<O<I<I<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<O<O<I<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<O<O<I<I<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<O<O<I<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<O<O<I<I<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<O<O<I<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<O<O<I<I<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<O<O<I<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<O<O<I<I<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<O<O<I<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<O<O<I<I<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<O<O<I<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<O<O<I<I<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<O<O<I<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<O<O<I<I<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<O<O<I<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<O<O<I<I<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<O<O<I<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<O<O<I<I<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<O<O<I<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<O<O<I<I<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<O<O<I<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<I<O<I<I<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<I<O<I<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<I<O<I<I<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<I<O<I<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<I<O<I<I<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<I<O<I<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<I<O<I<I<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<I<O<I<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<I<O<I<I<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<I<O<I<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<I<O<I<I<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<I<O<I<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<I<O<I<I<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<I<O<I<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<I<O<I<I<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<I<O<I<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<I<O<I<I<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<I<O<I<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<I<O<I<I<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<I<O<I<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<I<O<I<I<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<I<O<I<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<I<O<I<I<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<I<O<I<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<I<O<I<I<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<I<O<I<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<I<O<I<I<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<I<O<I<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<I<O<I<I<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<I<O<I<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<I<O<I<I<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<I<O<I<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<O<I<I<I<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<O<I<I<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<O<I<I<I<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<O<I<I<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<O<I<I<I<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<O<I<I<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<O<I<I<I<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<O<I<I<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<O<I<I<I<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<O<I<I<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<O<I<I<I<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<O<I<I<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<O<I<I<I<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<O<I<I<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<O<I<I<I<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<O<I<I<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<O<I<I<I<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<O<I<I<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<O<I<I<I<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<O<I<I<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<O<I<I<I<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<O<I<I<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<O<I<I<I<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<O<I<I<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<O<I<I<I<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<O<I<I<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<O<I<I<I<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<O<I<I<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<O<I<I<I<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<O<I<I<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<O<I<I<I<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<O<I<I<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<I<I<I<I<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<I<I<I<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<I<I<I<I<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<I<I<I<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<I<I<I<I<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<I<I<I<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<I<I<I<I<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<I<I<I<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<I<I<I<I<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<I<I<I<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<I<I<I<I<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<I<I<I<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<I<I<I<I<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<I<I<I<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<I<I<I<I<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<I<I<I<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<I<I<I<I<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<I<I<I<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<I<I<I<I<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<I<I<I<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<I<I<I<I<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<I<I<I<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<I<I<I<I<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<I<I<I<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<I<I<I<I<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<I<I<I<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<I<I<I<I<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<I<I<I<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<I<I<I<I<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<I<I<I<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<I<I<I<I<I<O<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<I<I<I<I<I<O<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<O<O<O<O<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<O<O<O<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<O<O<O<O<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<O<O<O<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<O<O<O<O<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<O<O<O<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<O<O<O<O<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<O<O<O<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<O<O<O<O<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<O<O<O<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<O<O<O<O<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<O<O<O<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<O<O<O<O<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<O<O<O<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<O<O<O<O<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<O<O<O<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<O<O<O<O<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<O<O<O<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<O<O<O<O<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<O<O<O<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<O<O<O<O<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<O<O<O<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<O<O<O<O<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<O<O<O<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<O<O<O<O<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<O<O<O<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<O<O<O<O<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<O<O<O<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<O<O<O<O<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<O<O<O<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<O<O<O<O<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<O<O<O<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<I<O<O<O<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<I<O<O<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<I<O<O<O<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<I<O<O<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<I<O<O<O<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<I<O<O<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<I<O<O<O<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<I<O<O<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<I<O<O<O<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<I<O<O<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<I<O<O<O<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<I<O<O<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<I<O<O<O<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<I<O<O<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<I<O<O<O<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<I<O<O<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<I<O<O<O<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<I<O<O<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<I<O<O<O<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<I<O<O<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<I<O<O<O<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<I<O<O<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<I<O<O<O<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<I<O<O<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<I<O<O<O<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<I<O<O<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<I<O<O<O<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<I<O<O<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<I<O<O<O<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<I<O<O<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<I<O<O<O<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<I<O<O<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<O<I<O<O<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<O<I<O<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<O<I<O<O<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<O<I<O<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<O<I<O<O<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<O<I<O<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<O<I<O<O<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<O<I<O<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<O<I<O<O<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<O<I<O<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<O<I<O<O<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<O<I<O<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<O<I<O<O<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<O<I<O<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<O<I<O<O<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<O<I<O<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<O<I<O<O<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<O<I<O<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<O<I<O<O<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<O<I<O<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<O<I<O<O<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<O<I<O<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<O<I<O<O<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<O<I<O<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<O<I<O<O<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<O<I<O<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<O<I<O<O<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<O<I<O<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<O<I<O<O<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<O<I<O<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<O<I<O<O<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<O<I<O<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<I<I<O<O<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<I<I<O<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<I<I<O<O<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<I<I<O<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<I<I<O<O<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<I<I<O<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<I<I<O<O<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<I<I<O<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<I<I<O<O<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<I<I<O<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<I<I<O<O<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<I<I<O<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<I<I<O<O<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<I<I<O<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<I<I<O<O<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<I<I<O<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<I<I<O<O<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<I<I<O<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<I<I<O<O<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<I<I<O<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<I<I<O<O<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<I<I<O<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<I<I<O<O<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<I<I<O<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<I<I<O<O<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<I<I<O<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<I<I<O<O<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<I<I<O<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<I<I<O<O<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<I<I<O<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<I<I<O<O<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<I<I<O<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<O<O<I<O<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<O<O<I<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<O<O<I<O<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<O<O<I<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<O<O<I<O<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<O<O<I<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<O<O<I<O<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<O<O<I<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<O<O<I<O<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<O<O<I<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<O<O<I<O<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<O<O<I<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<O<O<I<O<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<O<O<I<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<O<O<I<O<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<O<O<I<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<O<O<I<O<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<O<O<I<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<O<O<I<O<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<O<O<I<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<O<O<I<O<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<O<O<I<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<O<O<I<O<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<O<O<I<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<O<O<I<O<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<O<O<I<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<O<O<I<O<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<O<O<I<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<O<O<I<O<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<O<O<I<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<O<O<I<O<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<O<O<I<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<I<O<I<O<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<I<O<I<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<I<O<I<O<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<I<O<I<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<I<O<I<O<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<I<O<I<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<I<O<I<O<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<I<O<I<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<I<O<I<O<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<I<O<I<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<I<O<I<O<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<I<O<I<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<I<O<I<O<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<I<O<I<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<I<O<I<O<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<I<O<I<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<I<O<I<O<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<I<O<I<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<I<O<I<O<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<I<O<I<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<I<O<I<O<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<I<O<I<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<I<O<I<O<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<I<O<I<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<I<O<I<O<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<I<O<I<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<I<O<I<O<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<I<O<I<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<I<O<I<O<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<I<O<I<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<I<O<I<O<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<I<O<I<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<O<I<I<O<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<O<I<I<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<O<I<I<O<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<O<I<I<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<O<I<I<O<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<O<I<I<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<O<I<I<O<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<O<I<I<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<O<I<I<O<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<O<I<I<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<O<I<I<O<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<O<I<I<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<O<I<I<O<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<O<I<I<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<O<I<I<O<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<O<I<I<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<O<I<I<O<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<O<I<I<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<O<I<I<O<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<O<I<I<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<O<I<I<O<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<O<I<I<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<O<I<I<O<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<O<I<I<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<O<I<I<O<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<O<I<I<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<O<I<I<O<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<O<I<I<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<O<I<I<O<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<O<I<I<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<O<I<I<O<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<O<I<I<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<I<I<I<O<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<I<I<I<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<I<I<I<O<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<I<I<I<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<I<I<I<O<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<I<I<I<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<I<I<I<O<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<I<I<I<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<I<I<I<O<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<I<I<I<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<I<I<I<O<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<I<I<I<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<I<I<I<O<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<I<I<I<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<I<I<I<O<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<I<I<I<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<I<I<I<O<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<I<I<I<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<I<I<I<O<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<I<I<I<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<I<I<I<O<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<I<I<I<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<I<I<I<O<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<I<I<I<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<I<I<I<O<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<I<I<I<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<I<I<I<O<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<I<I<I<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<I<I<I<O<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<I<I<I<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<I<I<I<O<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<I<I<I<O<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<O<O<O<I<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<O<O<O<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<O<O<O<I<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<O<O<O<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<O<O<O<I<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<O<O<O<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<O<O<O<I<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<O<O<O<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<O<O<O<I<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<O<O<O<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<O<O<O<I<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<O<O<O<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<O<O<O<I<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<O<O<O<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<O<O<O<I<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<O<O<O<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<O<O<O<I<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<O<O<O<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<O<O<O<I<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<O<O<O<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<O<O<O<I<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<O<O<O<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<O<O<O<I<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<O<O<O<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<O<O<O<I<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<O<O<O<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<O<O<O<I<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<O<O<O<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<O<O<O<I<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<O<O<O<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<O<O<O<I<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<O<O<O<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<I<O<O<I<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<I<O<O<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<I<O<O<I<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<I<O<O<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<I<O<O<I<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<I<O<O<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<I<O<O<I<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<I<O<O<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<I<O<O<I<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<I<O<O<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<I<O<O<I<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<I<O<O<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<I<O<O<I<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<I<O<O<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<I<O<O<I<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<I<O<O<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<I<O<O<I<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<I<O<O<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<I<O<O<I<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<I<O<O<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<I<O<O<I<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<I<O<O<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<I<O<O<I<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<I<O<O<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<I<O<O<I<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<I<O<O<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<I<O<O<I<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<I<O<O<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<I<O<O<I<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<I<O<O<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<I<O<O<I<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<I<O<O<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<O<I<O<I<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<O<I<O<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<O<I<O<I<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<O<I<O<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<O<I<O<I<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<O<I<O<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<O<I<O<I<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<O<I<O<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<O<I<O<I<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<O<I<O<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<O<I<O<I<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<O<I<O<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<O<I<O<I<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<O<I<O<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<O<I<O<I<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<O<I<O<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<O<I<O<I<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<O<I<O<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<O<I<O<I<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<O<I<O<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<O<I<O<I<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<O<I<O<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<O<I<O<I<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<O<I<O<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<O<I<O<I<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<O<I<O<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<O<I<O<I<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<O<I<O<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<O<I<O<I<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<O<I<O<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<O<I<O<I<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<O<I<O<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<I<I<O<I<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<I<I<O<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<I<I<O<I<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<I<I<O<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<I<I<O<I<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<I<I<O<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<I<I<O<I<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<I<I<O<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<I<I<O<I<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<I<I<O<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<I<I<O<I<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<I<I<O<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<I<I<O<I<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<I<I<O<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<I<I<O<I<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<I<I<O<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<I<I<O<I<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<I<I<O<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<I<I<O<I<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<I<I<O<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<I<I<O<I<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<I<I<O<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<I<I<O<I<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<I<I<O<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<I<I<O<I<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<I<I<O<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<I<I<O<I<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<I<I<O<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<I<I<O<I<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<I<I<O<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<I<I<O<I<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<I<I<O<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<O<O<I<I<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<O<O<I<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<O<O<I<I<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<O<O<I<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<O<O<I<I<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<O<O<I<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<O<O<I<I<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<O<O<I<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<O<O<I<I<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<O<O<I<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<O<O<I<I<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<O<O<I<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<O<O<I<I<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<O<O<I<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<O<O<I<I<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<O<O<I<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<O<O<I<I<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<O<O<I<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<O<O<I<I<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<O<O<I<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<O<O<I<I<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<O<O<I<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<O<O<I<I<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<O<O<I<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<O<O<I<I<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<O<O<I<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<O<O<I<I<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<O<O<I<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<O<O<I<I<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<O<O<I<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<O<O<I<I<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<O<O<I<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<I<O<I<I<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<I<O<I<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<I<O<I<I<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<I<O<I<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<I<O<I<I<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<I<O<I<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<I<O<I<I<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<I<O<I<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<I<O<I<I<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<I<O<I<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<I<O<I<I<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<I<O<I<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<I<O<I<I<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<I<O<I<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<I<O<I<I<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<I<O<I<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<I<O<I<I<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<I<O<I<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<I<O<I<I<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<I<O<I<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<I<O<I<I<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<I<O<I<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<I<O<I<I<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<I<O<I<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<I<O<I<I<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<I<O<I<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<I<O<I<I<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<I<O<I<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<I<O<I<I<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<I<O<I<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<I<O<I<I<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<I<O<I<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<O<I<I<I<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<O<I<I<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<O<I<I<I<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<O<I<I<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<O<I<I<I<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<O<I<I<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<O<I<I<I<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<O<I<I<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<O<I<I<I<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<O<I<I<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<O<I<I<I<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<O<I<I<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<O<I<I<I<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<O<I<I<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<O<I<I<I<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<O<I<I<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<O<I<I<I<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<O<I<I<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<O<I<I<I<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<O<I<I<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<O<I<I<I<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<O<I<I<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<O<I<I<I<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<O<I<I<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<O<I<I<I<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<O<I<I<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<O<I<I<I<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<O<I<I<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<O<I<I<I<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<O<I<I<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<O<I<I<I<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<O<I<I<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<I<I<I<I<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<I<I<I<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<I<I<I<I<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<I<I<I<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<I<I<I<I<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<I<I<I<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<I<I<I<I<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<I<I<I<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<I<I<I<I<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<I<I<I<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<I<I<I<I<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<I<I<I<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<I<I<I<I<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<I<I<I<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<I<I<I<I<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<I<I<I<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<I<I<I<I<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<I<I<I<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<I<I<I<I<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<I<I<I<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<I<I<I<I<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<I<I<I<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<I<I<I<I<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<I<I<I<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<I<I<I<I<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<I<I<I<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<I<I<I<I<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<I<I<I<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<I<I<I<I<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<I<I<I<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<I<I<I<I<O<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<I<I<I<I<O<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<O<O<O<O<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<O<O<O<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<O<O<O<O<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<O<O<O<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<O<O<O<O<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<O<O<O<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<O<O<O<O<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<O<O<O<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<O<O<O<O<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<O<O<O<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<O<O<O<O<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<O<O<O<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<O<O<O<O<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<O<O<O<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<O<O<O<O<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<O<O<O<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<O<O<O<O<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<O<O<O<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<O<O<O<O<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<O<O<O<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<O<O<O<O<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<O<O<O<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<O<O<O<O<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<O<O<O<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<O<O<O<O<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<O<O<O<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<O<O<O<O<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<O<O<O<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<O<O<O<O<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<O<O<O<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<O<O<O<O<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<O<O<O<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<I<O<O<O<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<I<O<O<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<I<O<O<O<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<I<O<O<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<I<O<O<O<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<I<O<O<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<I<O<O<O<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<I<O<O<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<I<O<O<O<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<I<O<O<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<I<O<O<O<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<I<O<O<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<I<O<O<O<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<I<O<O<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<I<O<O<O<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<I<O<O<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<I<O<O<O<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<I<O<O<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<I<O<O<O<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<I<O<O<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<I<O<O<O<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<I<O<O<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<I<O<O<O<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<I<O<O<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<I<O<O<O<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<I<O<O<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<I<O<O<O<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<I<O<O<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<I<O<O<O<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<I<O<O<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<I<O<O<O<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<I<O<O<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<O<I<O<O<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<O<I<O<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<O<I<O<O<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<O<I<O<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<O<I<O<O<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<O<I<O<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<O<I<O<O<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<O<I<O<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<O<I<O<O<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<O<I<O<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<O<I<O<O<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<O<I<O<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<O<I<O<O<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<O<I<O<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<O<I<O<O<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<O<I<O<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<O<I<O<O<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<O<I<O<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<O<I<O<O<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<O<I<O<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<O<I<O<O<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<O<I<O<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<O<I<O<O<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<O<I<O<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<O<I<O<O<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<O<I<O<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<O<I<O<O<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<O<I<O<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<O<I<O<O<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<O<I<O<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<O<I<O<O<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<O<I<O<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<I<I<O<O<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<I<I<O<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<I<I<O<O<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<I<I<O<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<I<I<O<O<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<I<I<O<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<I<I<O<O<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<I<I<O<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<I<I<O<O<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<I<I<O<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<I<I<O<O<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<I<I<O<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<I<I<O<O<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<I<I<O<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<I<I<O<O<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<I<I<O<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<I<I<O<O<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<I<I<O<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<I<I<O<O<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<I<I<O<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<I<I<O<O<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<I<I<O<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<I<I<O<O<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<I<I<O<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<I<I<O<O<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<I<I<O<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<I<I<O<O<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<I<I<O<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<I<I<O<O<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<I<I<O<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<I<I<O<O<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<I<I<O<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<O<O<I<O<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<O<O<I<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<O<O<I<O<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<O<O<I<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<O<O<I<O<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<O<O<I<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<O<O<I<O<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<O<O<I<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<O<O<I<O<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<O<O<I<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<O<O<I<O<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<O<O<I<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<O<O<I<O<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<O<O<I<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<O<O<I<O<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<O<O<I<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<O<O<I<O<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<O<O<I<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<O<O<I<O<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<O<O<I<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<O<O<I<O<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<O<O<I<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<O<O<I<O<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<O<O<I<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<O<O<I<O<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<O<O<I<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<O<O<I<O<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<O<O<I<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<O<O<I<O<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<O<O<I<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<O<O<I<O<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<O<O<I<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<I<O<I<O<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<I<O<I<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<I<O<I<O<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<I<O<I<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<I<O<I<O<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<I<O<I<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<I<O<I<O<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<I<O<I<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<I<O<I<O<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<I<O<I<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<I<O<I<O<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<I<O<I<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<I<O<I<O<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<I<O<I<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<I<O<I<O<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<I<O<I<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<I<O<I<O<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<I<O<I<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<I<O<I<O<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<I<O<I<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<I<O<I<O<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<I<O<I<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<I<O<I<O<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<I<O<I<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<I<O<I<O<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<I<O<I<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<I<O<I<O<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<I<O<I<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<I<O<I<O<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<I<O<I<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<I<O<I<O<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<I<O<I<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<O<I<I<O<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<O<I<I<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<O<I<I<O<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<O<I<I<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<O<I<I<O<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<O<I<I<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<O<I<I<O<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<O<I<I<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<O<I<I<O<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<O<I<I<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<O<I<I<O<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<O<I<I<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<O<I<I<O<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<O<I<I<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<O<I<I<O<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<O<I<I<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<O<I<I<O<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<O<I<I<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<O<I<I<O<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<O<I<I<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<O<I<I<O<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<O<I<I<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<O<I<I<O<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<O<I<I<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<O<I<I<O<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<O<I<I<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<O<I<I<O<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<O<I<I<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<O<I<I<O<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<O<I<I<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<O<I<I<O<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<O<I<I<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<I<I<I<O<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<I<I<I<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<I<I<I<O<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<I<I<I<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<I<I<I<O<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<I<I<I<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<I<I<I<O<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<I<I<I<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<I<I<I<O<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<I<I<I<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<I<I<I<O<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<I<I<I<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<I<I<I<O<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<I<I<I<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<I<I<I<O<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<I<I<I<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<I<I<I<O<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<I<I<I<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<I<I<I<O<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<I<I<I<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<I<I<I<O<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<I<I<I<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<I<I<I<O<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<I<I<I<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<I<I<I<O<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<I<I<I<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<I<I<I<O<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<I<I<I<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<I<I<I<O<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<I<I<I<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<I<I<I<O<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<I<I<I<O<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<O<O<O<I<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<O<O<O<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<O<O<O<I<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<O<O<O<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<O<O<O<I<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<O<O<O<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<O<O<O<I<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<O<O<O<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<O<O<O<I<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<O<O<O<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<O<O<O<I<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<O<O<O<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<O<O<O<I<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<O<O<O<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<O<O<O<I<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<O<O<O<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<O<O<O<I<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<O<O<O<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<O<O<O<I<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<O<O<O<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<O<O<O<I<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<O<O<O<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<O<O<O<I<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<O<O<O<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<O<O<O<I<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<O<O<O<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<O<O<O<I<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<O<O<O<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<O<O<O<I<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<O<O<O<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<O<O<O<I<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<O<O<O<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<I<O<O<I<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<I<O<O<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<I<O<O<I<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<I<O<O<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<I<O<O<I<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<I<O<O<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<I<O<O<I<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<I<O<O<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<I<O<O<I<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<I<O<O<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<I<O<O<I<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<I<O<O<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<I<O<O<I<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<I<O<O<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<I<O<O<I<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<I<O<O<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<I<O<O<I<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<I<O<O<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<I<O<O<I<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<I<O<O<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<I<O<O<I<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<I<O<O<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<I<O<O<I<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<I<O<O<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<I<O<O<I<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<I<O<O<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<I<O<O<I<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<I<O<O<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<I<O<O<I<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<I<O<O<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<I<O<O<I<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<I<O<O<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<O<I<O<I<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<O<I<O<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<O<I<O<I<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<O<I<O<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<O<I<O<I<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<O<I<O<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<O<I<O<I<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<O<I<O<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<O<I<O<I<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<O<I<O<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<O<I<O<I<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<O<I<O<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<O<I<O<I<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<O<I<O<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<O<I<O<I<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<O<I<O<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<O<I<O<I<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<O<I<O<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<O<I<O<I<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<O<I<O<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<O<I<O<I<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<O<I<O<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<O<I<O<I<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<O<I<O<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<O<I<O<I<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<O<I<O<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<O<I<O<I<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<O<I<O<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<O<I<O<I<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<O<I<O<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<O<I<O<I<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<O<I<O<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<I<I<O<I<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<I<I<O<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<I<I<O<I<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<I<I<O<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<I<I<O<I<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<I<I<O<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<I<I<O<I<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<I<I<O<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<I<I<O<I<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<I<I<O<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<I<I<O<I<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<I<I<O<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<I<I<O<I<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<I<I<O<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<I<I<O<I<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<I<I<O<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<I<I<O<I<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<I<I<O<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<I<I<O<I<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<I<I<O<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<I<I<O<I<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<I<I<O<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<I<I<O<I<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<I<I<O<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<I<I<O<I<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<I<I<O<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<I<I<O<I<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<I<I<O<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<I<I<O<I<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<I<I<O<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<I<I<O<I<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<I<I<O<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<O<O<I<I<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<O<O<I<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<O<O<I<I<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<O<O<I<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<O<O<I<I<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<O<O<I<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<O<O<I<I<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<O<O<I<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<O<O<I<I<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<O<O<I<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<O<O<I<I<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<O<O<I<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<O<O<I<I<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<O<O<I<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<O<O<I<I<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<O<O<I<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<O<O<I<I<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<O<O<I<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<O<O<I<I<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<O<O<I<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<O<O<I<I<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<O<O<I<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<O<O<I<I<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<O<O<I<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<O<O<I<I<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<O<O<I<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<O<O<I<I<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<O<O<I<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<O<O<I<I<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<O<O<I<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<O<O<I<I<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<O<O<I<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<I<O<I<I<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<I<O<I<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<I<O<I<I<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<I<O<I<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<I<O<I<I<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<I<O<I<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<I<O<I<I<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<I<O<I<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<I<O<I<I<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<I<O<I<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<I<O<I<I<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<I<O<I<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<I<O<I<I<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<I<O<I<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<I<O<I<I<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<I<O<I<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<I<O<I<I<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<I<O<I<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<I<O<I<I<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<I<O<I<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<I<O<I<I<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<I<O<I<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<I<O<I<I<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<I<O<I<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<I<O<I<I<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<I<O<I<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<I<O<I<I<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<I<O<I<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<I<O<I<I<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<I<O<I<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<I<O<I<I<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<I<O<I<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<O<I<I<I<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<O<I<I<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<O<I<I<I<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<O<I<I<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<O<I<I<I<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<O<I<I<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<O<I<I<I<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<O<I<I<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<O<I<I<I<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<O<I<I<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<O<I<I<I<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<O<I<I<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<O<I<I<I<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<O<I<I<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<O<I<I<I<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<O<I<I<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<O<I<I<I<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<O<I<I<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<O<I<I<I<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<O<I<I<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<O<I<I<I<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<O<I<I<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<O<I<I<I<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<O<I<I<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<O<I<I<I<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<O<I<I<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<O<I<I<I<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<O<I<I<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<O<I<I<I<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<O<I<I<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<O<I<I<I<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<O<I<I<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<O<I<I<I<I<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<O<I<I<I<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<O<I<I<I<I<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<O<I<I<I<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<O<I<I<I<I<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<O<I<I<I<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<O<I<I<I<I<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<O<I<I<I<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<O<I<I<I<I<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<O<I<I<I<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<O<I<I<I<I<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<O<I<I<I<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<O<I<I<I<I<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<O<I<I<I<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<O<I<I<I<I<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<O<I<I<I<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<O<I<I<I<I<I<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<O<I<I<I<I<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<O<I<I<I<I<I<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<O<I<I<I<I<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<O<I<I<I<I<I<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<O<I<I<I<I<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<O<I<I<I<I<I<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<O<I<I<I<I<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<O<I<I<I<I<I<I<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<O<I<I<I<I<I<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<O<I<I<I<I<I<I<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<I<I<I<I<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<O<I<I<I<I<I<I<I<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<O<I<I<I<I<I<I<I<I<I<I<H>>>>>>>>>>>>;
}
impl AdmittedWidth for Idx<{ <Pz<I<I<I<I<I<I<I<I<I<I<I<H>>>>>>>>>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<I<I<I<I<I<I<I<I<I<I<H>>>>>>>>>>>>;
}
