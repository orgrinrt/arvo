// Written to be flagged before any pass: `run.sh` declares the first item as
// reading the fake width and leaves the second out of both lists, so no pass
// knows what the second should do.
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
const _: () = assert!(W != usize::BITS, "the shadow's width is the host's");
