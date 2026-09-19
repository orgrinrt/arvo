// Flagged by no pass, which is the limit it pins. `W == W` cannot fail, and it
// sits in one item beside `W == 8`, which does: the flip pass flips `W == 8`,
// and the premise pass sees the item refused by it. Each pass judges an item
// whole.
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

const _: () = assert!(W == 8 && W == W, "the shadow's width is not 8");
