//! P2. Does a trait type parameter accept a default, and can a consumer
//! override it at a call site without naming the other parameters?
//!
//! The alternative home for op's output generic once P1 refuses the function
//! position. The default sits on the contract rather than on the operation.
//!
//! Expected: accepted, with the default taken silently and the override
//! reachable by turbofish on the trait rather than on the method.

#![no_std]

/// The value the consumer piped in.
#[derive(Clone, Copy)]
pub struct Piped(pub u32);

/// A normalised form the design would hand back on request.
#[derive(Clone, Copy)]
pub struct Erased(pub u32);

/// The contract carries the output parameter, and the parameter carries the
/// default. `Out = Self` is the identity policy: hand back what came in.
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

/// Default taken: no output type named anywhere at the call site.
pub fn takes_the_default(a: Piped, b: Piped) -> Piped {
    a.dot(b)
}

/// Default overridden: the consumer names the output and nothing else.
pub fn overrides_the_default(a: Piped, b: Piped) -> Erased {
    Dot::<Piped, Erased>::dot(a, b)
}
