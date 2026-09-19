// The rename of `user`, onto a crate that does not re-export the real `core`,
// on a dependent that carries no `#![no_std]` at all. A crate with the std
// prelude imports it through the name `std`, not through the name `core`, so
// nothing here needs the renamed crate to carry a `prelude` module.
//
// The checks are `const _: () = assert!(...)`, evaluated at check time, the
// same trick every no_std arm in this fixture uses. A lib crate carrying no
// `#![no_std]` still has that form in scope through the std prelude, so
// nothing about dropping the attribute changes how the check runs.
//
// Outcome: WORKS, meaning `cargo check` exits 0, with the leading-`::` path
// reading the renamed crate's `primitive::usize::BITS`, 8, and no dependence
// on anything the no_std arms needed. The wrong-width sibling is
// `a_std_dependent_of_the_crate_without_the_glob_and_a_wrong_width`.
pub const W: u32 = ::core::primitive::usize::BITS;

const _: () = assert!(W == 8, "the manifest rename did not take over `::core`");
const _: () = assert!(
    W != usize::BITS,
    "the renamed answer is the real pointer width"
);
