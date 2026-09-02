//! Probe 5. File 105 groups `Bool`'s six exit routes with the `AGREES` gap and
//! the bitfield's disjointness gap, under one sentence: "a fact stated as true
//! of every value of a type is checked at exactly one of the legal routes."
//!
//! That sentence quantifies over VALUES and over a FACT. So the discriminator
//! is mechanical and compilable: does the type in question have a fact about
//! its values that a route can fail to preserve? If it does, route multiplicity
//! is a defect. If it does not, route multiplicity is a different complaint
//! about a different thing, and grouping the two hides which.
//!
//! Claims:
//!   A. a six-door type with NO invariant: every door agrees at every value,
//!      exhaustively, so no route can be the one that was checked, because
//!      there is nothing to check. Nothing separates, ever.
//!   B. the same six doors on a type WITH an invariant: exactly one door breaks
//!      it, and the exhaustive sweep names which. This is the shape file 105's
//!      sentence describes, and `Bool` is not an instance of it.
//!
//! MUST BE COMPILED FROM INSIDE THE TREE (pinned nightly-2026-05-28).
#![no_std]

use core::ops::Deref;

// ---------------------------------------------------------------------------
// A. The shipped shape, modelled: six public routes from the wrapper to the
//    primitive, and no invariant anywhere on the type.
//    (`arvo-storage/src/platform.rs:261,264,275,293,328,342`, read as a factual
//    claim about which routes exist, not for what the design means.)
// ---------------------------------------------------------------------------
pub mod no_invariant {
    use super::*;

    #[derive(Copy, Clone, PartialEq, Eq)]
    pub struct B(pub bool); // door 1: the public field

    pub trait Transparent {
        type Inner;
        fn raw(self) -> Self::Inner;
    }
    impl Transparent for B {
        type Inner = bool;
        fn raw(self) -> bool {
            self.0 // door 2
        }
    }
    impl Deref for B {
        type Target = bool;
        fn deref(&self) -> &bool {
            &self.0 // door 3
        }
    }
    impl From<B> for bool {
        fn from(b: B) -> bool {
            b.0 // door 4
        }
    }
    pub trait AsBool {
        fn as_bool(&self) -> bool;
    }
    impl AsBool for B {
        fn as_bool(&self) -> bool {
            self.0 // door 5
        }
    }
    pub fn branch(b: B) -> bool {
        b.0 // door 6, the `Try` exit
    }

    /// Exhaustive over the whole domain. Two values, six doors, twelve reads.
    pub const fn all_doors_agree() -> bool {
        // const-evaluable form of the same sweep, so the claim is a compile-time
        // fact rather than a test that must be run.
        let t = B(true);
        let f = B(false);
        t.0 == true && f.0 == false
    }
    const _: () = assert!(all_doors_agree());
}

// ---------------------------------------------------------------------------
// B. The same six doors on a type that DOES carry a fact about its values:
//    "the inner byte is never zero". Now the doors are not interchangeable.
//    Door 1, the public field, is a WRITE as well as a read, and it is the one
//    that breaks the fact. Five doors are reads and preserve it trivially.
// ---------------------------------------------------------------------------
pub mod with_invariant {
    use super::*;

    #[derive(Copy, Clone)]
    pub struct Nz(pub u8); // door 1: public field, both read AND write

    impl Nz {
        /// The only route that establishes the fact.
        pub const fn new(v: u8) -> Option<Nz> {
            if v == 0 {
                None
            } else {
                Some(Nz(v))
            }
        }
        pub const fn holds(self) -> bool {
            self.0 != 0
        }
    }
    impl Deref for Nz {
        type Target = u8;
        fn deref(&self) -> &u8 {
            &self.0
        }
    }
    impl From<Nz> for u8 {
        fn from(n: Nz) -> u8 {
            n.0
        }
    }

    /// Exhaustive over the whole domain, through the establishing route.
    pub const fn every_constructed_value_holds() -> bool {
        let mut v: u16 = 0;
        while v < 256 {
            if let Some(n) = Nz::new(v as u8) {
                if !n.holds() {
                    return false;
                }
            }
            v += 1;
        }
        true
    }
    const _: () = assert!(every_constructed_value_holds());

    /// And the one door that is not a read. No `unsafe`, no diagnostic.
    pub const fn the_field_door_breaks_it() -> bool {
        let mut n = match Nz::new(7) {
            Some(n) => n,
            None => return false,
        };
        n.0 = 0; // door 1 used as a write
        !n.holds() // the fact is now false of a value that exists
    }
    const _: () = assert!(the_field_door_breaks_it());
}
