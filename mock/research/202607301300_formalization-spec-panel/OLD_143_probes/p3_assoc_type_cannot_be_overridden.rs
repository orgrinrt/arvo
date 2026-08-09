//! P3. Can the output be an associated type instead of a trait parameter?
//!
//! An associated type is determined by the impl, so the consumer cannot
//! choose it. The only route to two answers for one self type is a second
//! impl, which coherence refuses.
//!
//! Expected: refused at E0119. Recorded because it is what forces the output
//! into parameter position rather than associated position, and because the
//! only route past E0119 is specialisation, which is forbidden.

#![no_std]

#[derive(Clone, Copy)]
pub struct Piped(pub u32);

#[derive(Clone, Copy)]
pub struct Erased(pub u32);

pub trait DotAssoc<Rhs> {
    type Out;
    fn dot(self, rhs: Rhs) -> Self::Out;
}

impl DotAssoc<Piped> for Piped {
    type Out = Piped;
    fn dot(self, rhs: Piped) -> Piped {
        Piped(self.0 * rhs.0)
    }
}

// The consumer wants the erased form instead. There is nowhere to say so.
impl DotAssoc<Piped> for Piped {
    type Out = Erased;
    fn dot(self, rhs: Piped) -> Erased {
        Erased(self.0 * rhs.0)
    }
}
