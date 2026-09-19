// Written to be flagged before any pass: `run.sh` declares the one item as
// both reading the fake width and reading past it, which the passes would
// expect to change outcome and to hold at once.
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

const _: () = assert!(W == 8, "the shadow's width is not 8");
