// The control for `a_std_dependent_of_the_crate_without_the_glob`'s const
// assertions: the same rename, the same without-the-glob stand-in, and a
// wrong width asserted instead of the right one, to show the check can
// actually fail rather than only ever holding.
//
// Outcome: REFUSED, the assertion's own message, since the renamed crate's
// `primitive::usize::BITS` is 8 rather than 9.
pub const W: u32 = ::core::primitive::usize::BITS;

const _: () = assert!(W == 9, "the manifest rename did not take over `::core`");
