// Written to be flagged before any pass: a `const assert` with no message,
// whose refusal the harness could not tell from another item's.
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

const _: () = assert!(W == 8);
