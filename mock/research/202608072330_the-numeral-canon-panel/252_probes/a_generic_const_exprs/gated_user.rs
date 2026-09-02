// Arm A, the discriminating control. Byte-for-byte the consumer of `user.rs`
// with the feature turned on in the consumer's own crate.
//
// Without this arm the E0308 in `user.rs` has two readings and no way to pick:
// the consumer needs the gate, or an unevaluated const never normalises across
// a crate boundary for anybody. If this one builds, the failure is caused by
// the absent feature and the bound does reach the consumer. If it fails too,
// the bound reaches nobody usefully and the finding is about const
// normalisation rather than about containment.
#![no_std]
#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

use leaky::{widen, W};

pub const fn call() -> W<4> {
    widen(W::<3>)
}
