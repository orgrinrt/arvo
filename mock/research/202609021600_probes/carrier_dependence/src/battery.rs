//! The verdict battery, printed rather than asserted.
//!
//! Nine candidate declarations put through the shipped verdict functions. It
//! encodes no expectation: it prints what came back, and the runner diffs the
//! vectors across carrier mutations. A row that never changes under any arm is a
//! row the carrier does not reach.
//!
//! Verdicts and not `ADMITTED`, on purpose: a verdict is a `const fn` that can be
//! evaluated on a construction that must keep compiling, so one binary can carry
//! candidates the obligation would refuse.

use arvo_format::quantum::{is_admissible_quantum, Exponent, MagnitudeCount, Quantum};
use arvo_format::slots::{is_admissible, Slot, Slots};
use arvo_format::width::Width;

macro_rules! range {
    ($n:ident, $min:expr, $max:expr, $w:expr) => {
        struct $n;
        impl Slots for $n {
            const MIN: Slot = Slot::at($min);
            const MAX: Slot = Slot::at($max);
            const WIDTH: Width = Width::bits($w);
        }
    };
}

macro_rules! quantum {
    ($n:ident, $base:expr, $slope:expr, $mags:expr) => {
        struct $n;
        impl Quantum for $n {
            const BASE: Exponent = Exponent::of($base);
            const SLOPE: Exponent = Exponent::of($slope);
            const MAGNITUDES: MagnitudeCount = MagnitudeCount::of($mags);
        }
    };
}

// Boundary of slots.rs:219, the width ceiling. A contiguous ordered 63-bit grid.
range!(Grid63, -4611686018427387904, 4611686018427387903, 63);
// Control one below it. Nothing about the two differs except one bit of width.
range!(Grid62, -2305843009213693952, 2305843009213693951, 62);
// Control: ordinary, admitted everywhere, so an arm that refuses it is broken.
range!(Grid8, -128, 127, 8);
// Boundary of slots.rs:211.
range!(Inverted, 8, -8, 8);
// Boundary of slots.rs:215.
range!(WidthZero, 0, 0, 0);
// A width far wider than its span, which the crate ships and admits.
range!(WiderThanSpan, 0, 3, 13);
// Boundary of slots.rs:232.
range!(SpanOverWidth, 0, 100, 2);

// Boundary of quantum.rs:318.
quantum!(ZeroMagnitudes, 0, 0, 0);
// Boundary of quantum.rs:322: reach runs past what an Exponent carries.
quantum!(ReachPast, 0, 2147483647, 4);
// Control beside it: the same shape with a reach that fits.
quantum!(ReachFits, 0, 1, 4);
// Control: the flat integer law.
quantum!(Flat, 0, 0, 1);

fn main() {
    // One line per candidate. `1` admitted, `0` refused.
    print!("Grid63={} ", is_admissible::<Grid63>().get() as u8);
    print!("Grid62={} ", is_admissible::<Grid62>().get() as u8);
    print!("Grid8={} ", is_admissible::<Grid8>().get() as u8);
    print!("Inverted={} ", is_admissible::<Inverted>().get() as u8);
    print!("WidthZero={} ", is_admissible::<WidthZero>().get() as u8);
    print!(
        "WiderThanSpan={} ",
        is_admissible::<WiderThanSpan>().get() as u8
    );
    print!(
        "SpanOverWidth={} ",
        is_admissible::<SpanOverWidth>().get() as u8
    );
    print!(
        "ZeroMagnitudes={} ",
        is_admissible_quantum::<ZeroMagnitudes>().get() as u8
    );
    print!(
        "ReachPast={} ",
        is_admissible_quantum::<ReachPast>().get() as u8
    );
    print!(
        "ReachFits={} ",
        is_admissible_quantum::<ReachFits>().get() as u8
    );
    println!("Flat={}", is_admissible_quantum::<Flat>().get() as u8);
}
