// The one body every consumer includes, so the four consumers differ only in
// which arm `contracts` names and which feature their manifest turns on. No
// `#![feature]` line appears in any consumer.

use core::cmp::Ordering;

use arvo_format::Adapt;
use arvo_format::overflow::Saturate;
use arvo_format::rounding::TowardZero;
use arvo_numeral::{Int, Num, Quiet, Uint};
use contracts::{Order, Sum, Zero};

/// The highest running sum of the members, starting from the additive
/// identity: a function generic over a numeral that adds, orders and has an
/// additive identity, which is the whole of what the obligation fixes.
pub fn best_running_sum<T: Sum + Order + Zero + Copy>(xs: [T; 4]) -> T {
    let mut running = T::ZERO;
    let mut best = T::ZERO;
    for x in xs {
        running = running.sum(x);
        if matches!(running.order(best), Ordering::Greater) {
            best = running;
        }
    }
    best
}

/// A signed eight-bit integer that saturates, for the run against the value.
pub type I8 = Int<8, Adapt<TowardZero, Saturate>, Quiet>;

/// An outside instance: a tally of this crate's own, implemented from here,
/// which shows the set of instances is open to a crate with no feature line.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Tally(pub Uint<8, Adapt<TowardZero, Saturate>, Quiet>);

impl Sum for Tally {
    fn sum(self, rhs: Self) -> Self {
        Tally(self.0.add(rhs.0))
    }
}

impl Order for Tally {
    fn order(self, rhs: Self) -> Ordering {
        self.0.cmp(rhs.0)
    }
}

impl Zero for Tally {
    const ZERO: Self = Tally(Num::constant::<0>());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_value_runs_through_the_contract() {
        let xs = [
            I8::constant::<3>(),
            I8::constant::<-5>(),
            I8::constant::<9>(),
            I8::constant::<-1>(),
        ];
        // Running sums 3, -2, 7, 6: the best is 7.
        assert_eq!(best_running_sum(xs), I8::constant::<7>());
    }

    #[test]
    fn the_outside_instance_runs_through_the_contract() {
        let t = |v| {
            Tally(Uint::<8, Adapt<TowardZero, Saturate>, Quiet>::adapted(
                arvo_format::Exact::on_grid(arvo_format::Slot::at(v)),
            ))
        };
        // Saturates at 255 on the third member and holds there.
        assert_eq!(best_running_sum([t(200), t(50), t(30), t(0)]), t(255));
    }
}
