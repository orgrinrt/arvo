//! B3, must FAIL, and the failure is NOT the staging boundary either.
//!
//! Everything is at stage zero: the length is a literal and the accumulator width is a
//! const with no generic parameter in it anywhere. The refusal is an arithmetic fault
//! inside const evaluation, which is a third cause distinct from B1's and B2's.
//!
//! A first version of this arm made `acc_width` a const fn of a const generic `N`. That
//! was refused for the same reason A2 is refused, not for overflow, so it established
//! nothing about a third cause and was replaced. The first version's diagnostic is in
//! `p4_v1_b3_was_refused_for_the_wrong_reason.out`.
#![allow(dead_code)]

struct Acc<const A: usize>([u8; A]);

/// Overflows in const evaluation. No generic parameter appears.
const ACC_WIDTH: usize = usize::MAX - 1 + 3;

fn fold(_xs: [i8; 3]) -> Acc<ACC_WIDTH> {
    Acc([0u8; ACC_WIDTH])
}

fn main() {
    let _ = fold([1i8, 2, 3]);
}
