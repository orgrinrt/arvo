//! P6. What happens if a domain alias resolves to different strategies in
//! different lexical scopes?
//!
//! Tier two writes `StrHandle = UInt<5>` once and then writes `StrHandle`
//! everywhere. If a scoped mechanism could make `StrHandle` mean
//! `UInt<5, Hot>` inside one function and `UInt<5, Warm>` outside it, the two
//! occurrences denote different types.
//!
//! This probe does not test a mechanism. It tests the consequence, by naming
//! the two types directly and passing a value of one where the other is
//! expected. If that is refused, then any scoped mechanism that reaches
//! through an alias breaks tier two at every function boundary, and the
//! reach limit is forced rather than chosen.
//!
//! Expected: refused at E0308.

#![no_std]

pub struct Hot;
pub struct Warm;

/// Stand-in for the shipped shape: a width and a strategy.
pub struct UInt<const N: u32, S>(pub u32, core::marker::PhantomData<S>);

/// The alias as the module-scope definition resolves it.
pub type StrHandleOutside = UInt<5, Warm>;

/// The alias as a scope carrying an ambient Hot would have resolved it.
pub type StrHandleInside = UInt<5, Hot>;

pub fn store(_h: StrHandleOutside) {}

/// A value produced inside the ambient scope, handed to a function declared
/// outside it. This is the ordinary tier-two call, and it is the whole point
/// of having one name.
pub fn cross(h: StrHandleInside) {
    store(h);
}
