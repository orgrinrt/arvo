// Written to be flagged by the premise pass alone. The flip pass turns the
// first comparison, `1 == 1`, into `1 != 1`, which refuses the arm; the
// comparison that reads the width, `W == W`, cannot fail.
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
    1 == 1 && W == W,
    "one is not one, or the width is not itself"
);
