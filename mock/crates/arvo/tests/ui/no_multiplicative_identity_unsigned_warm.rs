//! `UFixed<0, 8, Warm>` has no multiplicative identity.
//!
//! The pin covered one shape until a review pointed out that loosening the
//! bound on the other impl left the whole suite green. The refusal is per
//! impl and per strategy, so it is pinned per impl and per strategy. The
//! topic called the signed case the worst of the four and it was the one
//! not covered.

use arvo::strategy::{Identity, Multiplicative, Warm};
use arvo::ufixed::UFixed;
use arvo::{fbits, ibits};

fn main() {
    let _ = <UFixed<{ ibits(0) }, { fbits(8) }, Warm> as Identity<Multiplicative>>::IDENTITY;
}
