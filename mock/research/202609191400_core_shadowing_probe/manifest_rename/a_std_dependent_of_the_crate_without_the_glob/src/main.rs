// The rename of `user`, onto a crate that does not re-export the real `core`,
// on a dependent that carries no `#![no_std]` at all. A crate with the std
// prelude imports it through the name `std`, not through the name `core`, so
// nothing here needs the renamed crate to carry a `prelude` module.
//
// Outcome: WORKS, meaning `cargo check` exits 0, with the leading-`::` path
// reading the renamed crate's `primitive::usize::BITS`, 8, and no dependence
// on anything the no_std arms needed.
fn main() {
    let w: u32 = ::core::primitive::usize::BITS;
    assert!(w == 8, "the manifest rename did not take over `::core`");
    assert!(
        w != usize::BITS,
        "the renamed answer is the real pointer width"
    );
}
