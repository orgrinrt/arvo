// Written to be flagged before any pass: the message holds an escape other
// than a quote or a backslash, which the harness does not match against the
// panic text rustc prints, so it refuses the message rather than misreading it.
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

const _: () = assert!(W == 8, "the shadow's width\nwas not read");
