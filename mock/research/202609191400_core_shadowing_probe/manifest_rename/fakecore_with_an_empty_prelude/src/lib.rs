// A shadowing crate carrying nothing of the real `core`: a hand-written
// `prelude::rust_2024` module with nothing in it, which is the path the
// edition 2024 prelude import names, and its own `primitive`.
#![no_std]

pub mod prelude {
    pub mod rust_2024 {}
}

pub mod primitive {
    #[allow(non_camel_case_types)]
    pub struct usize;
    impl usize {
        pub const BITS: u32 = 8;
    }
}
