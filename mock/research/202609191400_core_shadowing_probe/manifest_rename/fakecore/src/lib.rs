// A crate that re-exports the real `core` whole and then shadows one module of
// it, so a dependent that renames it to `core` still finds `core::prelude` for
// its own prelude, and finds this `primitive` instead of the real one.
#![no_std]

pub use core::*;

pub mod primitive {
    #[allow(non_camel_case_types)]
    pub struct usize;
    impl usize {
        pub const BITS: u32 = 8;
    }
}
