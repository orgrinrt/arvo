// Flagged by no pass. The message holds an escaped quote, which the harness
// reads as part of the message rather than as its end, and matches against
// the panic text rustc prints, where the quote is unescaped.
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

const _: () = assert!(W == 8, "the shadow's \"8\" was not read");
