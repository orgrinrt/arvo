// A macro expanding to `extern crate self as core;` at the crate root, which is
// the third form the lint refuses, reached through a macro the lint does not
// expand. The compiler refuses it before the leading-`::` path is resolved:
// "macro-expanded `extern crate` items cannot shadow names passed with
// `--extern`". The assertion below would hold if the hijack went through, so a
// build succeeding here would mean the compiler let it.
//
// Outcome: FAILS TO COMPILE, with exactly that error, which `run.sh` checks for.
#![no_std]

macro_rules! hijack {
    () => {
        extern crate self as core;
    };
}

hijack!();

pub mod primitive {
    #[allow(non_camel_case_types)]
    pub struct usize;
    impl usize {
        pub const BITS: u32 = 8;
    }
}

pub const W: u32 = ::core::primitive::usize::BITS;

const _: () = assert!(
    W == 8,
    "the macro-expanded alias did not take over `::core`"
);
