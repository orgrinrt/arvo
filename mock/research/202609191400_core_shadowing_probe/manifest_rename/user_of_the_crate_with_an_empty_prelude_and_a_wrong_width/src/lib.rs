// The control for `user_of_the_crate_with_an_empty_prelude`'s array-length
// trick: the same rename, the same empty-prelude stand-in, and a wrong width
// asserted instead of the right one, to show the check can actually fail
// rather than only ever holding.
//
// Outcome: REFUSED, `mismatched types`, on the array-length mismatch, since
// the renamed crate's `primitive::usize::BITS` is 8 rather than 9.
#![no_std]

pub const W: u32 = ::core::primitive::usize::BITS;

const _: [(); 1] = [(); (W == 9) as usize];
