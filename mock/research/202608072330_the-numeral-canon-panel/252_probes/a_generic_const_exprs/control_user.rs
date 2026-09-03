// Arm A control. Same consumer shape, same dependency, same absent feature, but
// it names only the signature with no const expression in it. If this fails too,
// the arm proves nothing about const expressions and only that the dependency
// cannot be consumed at all.
#![no_std]

use leaky::{W, identity};

pub const fn call() -> W<3> {
    identity(W::<3>)
}
