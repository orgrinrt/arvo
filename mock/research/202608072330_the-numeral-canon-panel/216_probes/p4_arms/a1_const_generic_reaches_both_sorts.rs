//! A1, must compile. A fact carried as a const generic parameter, consumed in BOTH sorts:
//! as a const generic argument to another type, and as a plain const value.
//!
//! This is the "strongest sort" end of the lattice. If this fails, the refinement in
//! 216's file is wrong and the row's rule is the only available one.
#![allow(dead_code)]

struct Acc<const A: usize>([u8; A]);

/// The fact, carried as a const generic parameter.
struct Fmt<const W: usize>;

impl<const W: usize> Fmt<W> {
    /// Consumed as a const value.
    const DOUBLED: usize = W * 2;
}

/// Consumed as a const generic argument, in the same program, from the same carrier.
fn build<const W: usize>() -> Acc<W> {
    Acc([0u8; W])
}

fn main() {
    let a = build::<7>();
    let n = <Fmt<7>>::DOUBLED;
    assert_eq!(a.0.len(), 7);
    assert_eq!(n, 14);
}
