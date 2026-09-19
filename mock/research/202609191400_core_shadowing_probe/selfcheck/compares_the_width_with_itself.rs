// Written to be flagged by the premise pass alone. `W == W` cannot fail, and
// flipped it is `W != W`, refused, so the flip pass does not see it.
#![no_std]

mod core {
    pub mod primitive {
        #[allow(non_camel_case_types)]
        pub struct usize;
        impl usize {
            pub const BITS: u32 = 8;
        }
    }
}

pub const W: u32 = core::primitive::usize::BITS;

const _: () = assert!(W == W, "the shadow's width is not itself");
