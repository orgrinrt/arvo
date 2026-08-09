//! probe 1, crate `capacity`: the capacity domain as a CONSUMER of the shared
//! sealed vocabulary, per D7's alias pattern (each domain aliases the shared
//! carrier to its own semantics). Local trait over the foreign sealed types:
//! orphan-rule legal, blanket, one impl.
#![no_std]
use carrier::{Nat, Pz, H, I, O};

pub trait Capacity {
    const SIZE: u128;
}
// local trait, foreign type: legal; blanket over every sealed Nat forever
impl<N: Nat> Capacity for N {
    const SIZE: u128 = N::VALUE;
}

// D7 alias: capacity semantics over the shared carrier, no new encoding
pub type Cap13 = Pz<I<O<I<H>>>>; // 13 = 0b1101

const _: () = assert!(<Cap13 as Capacity>::SIZE == 13); // declaration-site check, const-callable
