//! P4. Is the trait parameter's default actually taken, or was P2's result
//! ordinary inference from the return type?
//!
//! P2's `takes_the_default` returned `Piped`, so inference could have reached
//! the same place from the return position. This removes every contextual
//! clue: the result is bound with no annotation and only consumed by a
//! function that accepts anything.
//!
//! Expected: accepted, and only if the default is genuinely load-bearing.

#![no_std]

#[derive(Clone, Copy)]
pub struct Piped(pub u32);

#[derive(Clone, Copy)]
pub struct Erased(pub u32);

pub trait Dot<Rhs = Self, Out = Self> {
    fn dot(self, rhs: Rhs) -> Out;
}

impl Dot<Piped, Piped> for Piped {
    fn dot(self, rhs: Piped) -> Piped {
        Piped(self.0 * rhs.0)
    }
}

impl Dot<Piped, Erased> for Piped {
    fn dot(self, rhs: Piped) -> Erased {
        Erased(self.0 * rhs.0)
    }
}

pub fn sink<X>(_x: X) {}

/// No annotation, no return-position clue, two impls in scope. If the default
/// were not load-bearing this would be ambiguous.
pub fn no_context(a: Piped, b: Piped) {
    let r = a.dot(b);
    sink(r);
}
