//! A2, must FAIL. The identical fact carried as an associated const, consumed where a
//! const generic argument is wanted.
//!
//! This is the `const -> type` direction. If it compiles on the pinned toolchain, the
//! asymmetry does not exist and both R1's rule and 216's refinement lose their ground.
#![allow(dead_code)]

struct Acc<const A: usize>([u8; A]);

trait Carries {
    /// The fact, carried as a const.
    const W: usize;
}

struct SevenBit;
impl Carries for SevenBit {
    const W: usize = 7;
}

/// The consuming site wants it as a const generic argument.
fn build<C: Carries>() -> Acc<{ C::W }> {
    Acc([0u8; C::W])
}

fn main() {
    let a = build::<SevenBit>();
    assert_eq!(a.0.len(), 7);
}
