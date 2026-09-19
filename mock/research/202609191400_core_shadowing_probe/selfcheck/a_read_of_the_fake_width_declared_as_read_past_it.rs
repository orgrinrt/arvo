// Written to be flagged by the premise pass and the spelling pass. `run.sh`
// declares the one item as reading past the fake width, and it reads the
// shadow through the bare spelling: the premise pass expects it to hold and it
// is refused, and the spelling pass finds no leading-`::` `core` path to drop.
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
