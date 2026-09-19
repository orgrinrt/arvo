// A `use` item whose unaliased last segment is `core`, `use fake::core;`, binds
// the name `core` in this module the way `use fake as core;` does, and the
// leading-`::` path `::core::primitive::usize::BITS` still resolves through the
// real crate root rather than through it.
//
// Two assertions. The first reads the bare spelling and says it reaches the
// imported module's 9, which shows the import did bind `core` here. The second
// reads the leading-`::` spelling and says it reaches the host's pointer width,
// read through the primitive type `usize`, whose name resolution does not go
// through `core`.
//
// Outcome: WORKS. Exit 0, and both assertions hold.
#![no_std]

mod fake {
    pub mod core {
        pub mod primitive {
            #[allow(non_camel_case_types)]
            pub struct usize;
            impl usize {
                pub const BITS: u32 = 9;
            }
        }
    }
}

use fake::core;

const _: () = assert!(
    core::primitive::usize::BITS == 9,
    "the unaliased `use` did not bind the bare spelling `core`"
);

const _: () = assert!(
    ::core::primitive::usize::BITS == usize::BITS,
    "the leading-:: path resolved through the unaliased `use` of `core`"
);
