// Flagged by no pass. A string holding `==` comes before the comparison, and
// the message holds `==` and `!=` too. The harness skips both when it looks
// for the comparison to flip, so it flips the one outside a string.
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

const _: () = assert!(
    "==".len() as u32 + W == 10,
    "the width beside a two-byte string was not 8, so W == 8 is false and W != 8"
);
