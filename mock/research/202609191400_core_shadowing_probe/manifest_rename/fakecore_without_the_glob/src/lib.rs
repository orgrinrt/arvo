// The same shadowing crate as `fakecore`, without the `pub use core::*;` that
// re-exports the real `core`.
#![no_std]

pub mod primitive {
    #[allow(non_camel_case_types)]
    pub struct usize;
    impl usize {
        pub const BITS: u32 = 8;
    }
}
