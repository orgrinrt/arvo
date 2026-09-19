// Written to be flagged by the premise pass alone. `W != 7` against a fake
// width of 8 holds, flipped it is refused, and it still holds at the host's
// width, so it never told the fake width from the real one.
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

const _: () = assert!(W != 7, "the shadow's width is 7");
