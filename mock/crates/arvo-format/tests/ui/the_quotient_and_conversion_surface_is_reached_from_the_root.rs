// Every item the quotient and the conversion add, and the exact door they build
// through, named from the crate root by a consumer with no feature line, the way
// `Cargo.toml` of an outside crate names this one.

use arvo_format::adapt::{Adapt, Arity, Operation, Signature};
use arvo_format::overflow::Saturate;
use arvo_format::points::{Integer, UFixed};
use arvo_format::rounding::HalfEven;
use arvo_format::{
    Bool,
    Convert,
    Dither,
    Div,
    Divisor,
    Exact,
    Remainder,
    Slot,
    convert,
    converted_position,
    div,
    is_convertible,
    is_divisible,
    quotient_position,
};

type S = Signature<Integer<8>, Adapt<HalfEven, Saturate>>;

const THREE: Divisor<Integer<8>> = Divisor::constant::<3>();
const QUOTIENT: Slot = div::<S>(Slot::at(100), THREE, Dither::UNUSED);
const POSITION: Exact = quotient_position::<Integer<8>>(Slot::at(100), THREE);
const CONVERTED: Slot = convert::<UFixed<8, -2>, S>(Slot::at(10), Dither::UNUSED);
const PLACED: Exact = converted_position::<UFixed<8, -2>, Integer<8>>(Slot::at(10));
const DIVISIBLE: Bool = is_divisible::<Integer<8>>();
const CONVERTIBLE: Bool = is_convertible::<UFixed<8, -2>, Integer<8>>();
// The door a consumer's own operation builds its position through, the one the
// quotient builds through: `100 / 3` is 33 and a third.
const BUILT: Exact = Exact::at(Slot::at(33), Remainder::of(1, 3));
const ARITIES: (Arity, Arity) = (
    <Div<S> as Operation>::ARITY,
    <Convert<UFixed<8, -2>, S> as Operation>::ARITY,
);

fn main() {
    let _ = (
        QUOTIENT,
        POSITION,
        CONVERTED,
        PLACED,
        DIVISIBLE,
        CONVERTIBLE,
        ARITIES,
        THREE.slot(),
    );
    assert_eq!(BUILT, POSITION);
    // A third is neither a tie nor on the grid, a half is a tie, and a zero
    // denominator names no ratio and reads as on the grid.
    let third = Exact::at(Slot::ZERO, Remainder::of(7, 3));
    assert!(!third.is_tie().get() && !third.is_on_grid().get());
    assert!(Exact::at(Slot::ZERO, Remainder::of(1, 2)).is_tie().get());
    assert!(
        Exact::at(Slot::ZERO, Remainder::of(5, 0))
            .is_on_grid()
            .get()
    );
}
