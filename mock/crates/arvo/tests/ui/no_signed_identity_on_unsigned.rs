//! `UFixed` has no `SignedIdentity`, at any width or strategy.
//!
//! Unsigned spans `[0, 2^I)` and contains no negative value, so minus one is
//! not a value of the type and the impl does not exist. This was expressible
//! nowhere and pinned nowhere: the `SignedIdentity` work asserted what the
//! trait DOES reach and never that it stops at the sign boundary.
//!
//! The forwarding impl on `Bits<N, S, Sign>` is bounded on the container's own
//! `SignedIdentity`, which exists for `i8`..=`i128` and nothing unsigned, so
//! the refusal falls out of the bound rather than being stipulated. A
//! regression that widened that bound would restore a `NEG_ONE` on a type with
//! no negative values, and nothing else in the suite would notice.

use arvo::strategy::{Hot, SignedIdentity};
use arvo::ufixed::UFixed;
use arvo::{fbits, ibits};

fn main() {
    let _ = <UFixed<{ ibits(1) }, { fbits(7) }, Hot> as SignedIdentity>::NEG_ONE;
}
