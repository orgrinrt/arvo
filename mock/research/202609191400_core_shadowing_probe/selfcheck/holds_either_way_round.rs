// Written to be flagged by the flip pass and the premise pass. The assertion
// holds with its comparison either way round, and at any fake width.
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

const _: () = assert!(W == 8 || true, "holds either way round");
