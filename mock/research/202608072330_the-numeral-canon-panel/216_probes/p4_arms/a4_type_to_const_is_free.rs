//! A4, must compile. The `type -> const` direction: a fact carried as an associated TYPE,
//! projected down to a const value generically.
//!
//! Together with A2 this is the asymmetry: the lattice has a direction, and a fact carried
//! in the stronger sort reaches the weaker one for nothing while the reverse is refused.
#![allow(dead_code)]

trait Width {
    const BITS: usize;
}
struct W7;
impl Width for W7 {
    const BITS: usize = 7;
}

trait Carries {
    /// The fact, carried as a type.
    type W: Width;
}

struct SevenBit;
impl Carries for SevenBit {
    type W = W7;
}

/// Projected to a const, generically, with no gate.
fn bits<C: Carries>() -> usize {
    <C::W as Width>::BITS
}

fn main() {
    assert_eq!(bits::<SevenBit>(), 7);
}
