// P2. Does an inner `use` injected into a function body retarget unqualified
// arvo-shaped names in the BODY while leaving the SIGNATURE resolving outward?
// This is the whole question behind "one injected use is the arvo half of the
// rewrite", and behind whether the precedence rule falls out of Rust's own
// scoping rather than needing a mechanism.

#![allow(dead_code, unused_imports)]

pub struct Hot;
pub struct Warm;

pub struct Num<const N: u8, S>(core::marker::PhantomData<S>);

impl<const N: u8, S> Num<N, S> {
    pub fn tag() -> &'static str {
        core::any::type_name::<S>()
    }
}

// arvo ships one alias set per posture. Names are identical across the sets.
pub mod posture {
    pub mod warm {
        pub type UInt<const N: u8> = super::super::Num<N, super::super::Warm>;
    }
    pub mod hot {
        pub type UInt<const N: u8> = super::super::Num<N, super::super::Hot>;
    }
}

// The ambient default a consumer gets with a plain `use arvo::*`.
use posture::warm::UInt;

// A tier-two domain alias, defined at module scope with no posture written.
type StrHandle = UInt<5>;

// --- case A: no attribute. Body and signature both see the ambient Warm. ---
fn plain() -> &'static str {
    let _x: UInt<5> = Num(core::marker::PhantomData);
    UInt::<5>::tag()
}

// --- case B: the macro injected `use posture::hot::*;` at the top of the body. ---
fn annotated() -> &'static str {
    use crate::posture::hot::*;
    let _x: UInt<5> = Num(core::marker::PhantomData);
    UInt::<5>::tag()
}

// --- case C: the name also appears in the SIGNATURE. The injected use cannot
// reach it, because a signature resolves in the enclosing scope. ---
fn signature_typed(_x: UInt<5>) -> &'static str {
    use crate::posture::hot::*;
    UInt::<5>::tag() // body sees Hot
}

fn signature_of() -> &'static str {
    // what the signature of `signature_typed` actually took
    fn probe<T>(_: fn(T) -> &'static str) -> &'static str {
        core::any::type_name::<T>()
    }
    probe(signature_typed)
}

// --- case D: a module-scope alias is immune, because it was resolved where it
// was written. This is the tier-two consumer's StrHandle. ---
fn via_alias() -> &'static str {
    use crate::posture::hot::*;
    let _x: StrHandle = Num(core::marker::PhantomData);
    StrHandle::tag()
}

// --- case E: an explicitly spelled posture is immune by construction. ---
fn explicit() -> &'static str {
    use crate::posture::hot::*;
    Num::<5, Warm>::tag()
}

fn main() {
    println!("A plain          body = {}", plain());
    println!("B annotated      body = {}", annotated());
    println!(
        "C sig-typed      body = {}",
        signature_typed(Num(core::marker::PhantomData))
    );
    println!("C sig-typed      sig  = {}", signature_of());
    println!("D via alias      body = {}", via_alias());
    println!("E explicit       body = {}", explicit());
}
