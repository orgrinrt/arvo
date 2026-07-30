#![feature(const_trait_impl)]
use union::*;
macro_rules! row {
    ($t:ty) => {
        println!(
            "{:<16} stable1={:<5} stable2={:<5} refuses={:<5}",
            stringify!($t),
            const { stable::<$t>(0, 7, false) },
            const { stable::<$t>(-8, 7, true) },
            const { ever_refuses::<$t>(-8, 7) },
        );
    };
}
fn main() {
    row!(ReduceModulo);
    row!(TowardNegative);
    row!(TowardPositive);
    row!(SubstituteZero);
    row!(Refuse);
}
