// A shadowing crate carrying a `prelude` module with nothing in it at all,
// no `rust_2024` submodule and none of the real `core`. Isolates whether the
// edition's prelude import needs the exact `rust_2024` path or is satisfied
// by any `prelude` module.
#![no_std]

pub mod prelude {}

pub mod primitive {
    #[allow(non_camel_case_types)]
    pub struct usize;
    impl usize {
        pub const BITS: u32 = 8;
    }
}
