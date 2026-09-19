// Written to be flagged before any pass: two items carry one message, so a
// refusal with it could come from either.
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

const _: () = assert!(W == 8, "the shadow's width was not read");
const _: () = assert!(W != usize::BITS, "the shadow's width was not read");
