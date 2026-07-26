//! `IFixed<0, 7, Precise>` has no multiplicative identity.
//!
//! One cell of the impl-by-strategy grid. Every cell is pinned, because a
//! regression is per impl and per strategy: loosening the bound on one impl
//! under one strategy is a change the other cells cannot see.

use arvo::ifixed::IFixed;
use arvo::strategy::{Identity, Multiplicative, Precise};
use arvo::{fbits, ibits};

fn main() {
    let _ = <IFixed<{ ibits(0) }, { fbits(7) }, Precise> as Identity<Multiplicative>>::IDENTITY;
}
