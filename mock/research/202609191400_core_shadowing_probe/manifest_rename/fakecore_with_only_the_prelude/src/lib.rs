// The shadowing crate of `fakecore`, re-exporting only the real `core`'s
// `prelude` module rather than all of `core`. A dependent that renames it to
// `core` finds `core::prelude` for its own prelude and this `primitive` in
// place of the real one, and nothing else of `core` at all.
#![no_std]

pub use core::prelude;

pub mod primitive {
    #[allow(non_camel_case_types)]
    pub struct usize;
    impl usize {
        pub const BITS: u32 = 8;
    }
}
