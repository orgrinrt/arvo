// Arm A, the consumer. No feature attribute of any kind. It names the public
// signature that carries the const expression.
#![no_std]

use leaky::{widen, W};

pub const fn call() -> W<4> {
    widen(W::<3>)
}
