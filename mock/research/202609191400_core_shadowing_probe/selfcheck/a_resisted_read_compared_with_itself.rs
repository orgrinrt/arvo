// Written to be flagged by the spelling pass alone. `W` is read through the
// leading `::` past a `mod core` shadow, and `W == W` cannot fail. The premise
// pass expects an item reading past the fake width to hold, so it does not see
// it; with the leading `::` dropped the item should read the shadow and be
// refused, and it holds.
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

pub const W: u32 = ::core::primitive::usize::BITS;

const _: () = assert!(W == W, "the width read past the shadow is not itself");
