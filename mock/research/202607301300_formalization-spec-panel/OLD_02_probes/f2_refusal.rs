// TEST F2: obligation 5 says the projection must type-check with both
// inhabitants. It does (F1). The question it does not ask: can the generic
// arithmetic CONSTRUCT a refusal through the projection?
#![allow(dead_code)]
use core::convert::Infallible;
use notko::{ConstFromResidual, ConstTry, Just, Outcome};

#[derive(Clone, Copy)]
pub struct OutOfRange;

pub trait Quantisation {
    type Fallibility<T: Copy>: ConstTry<Output = T>;
}
pub struct Total;
impl Quantisation for Total {
    type Fallibility<T: Copy> = Just<T>;
}
pub struct Refusing;
impl Quantisation for Refusing {
    type Fallibility<T: Copy> = Outcome<T, OutOfRange>;
}

// The only bound that constructs a failure generically. Reading it aloud:
// "every quantisation's fallibility can be built from an out-of-range
// residual", which is exactly what a total quantisation must not admit.
fn add_or_refuse<Q: Quantisation>(a: u32, b: u32) -> Q::Fallibility<u32>
where
    Q::Fallibility<u32>: ConstFromResidual<Outcome<Infallible, OutOfRange>>,
{
    match a.checked_add(b) {
        Some(v) => <Q::Fallibility<u32> as ConstTry>::from_output(v),
        None => {
            <Q::Fallibility<u32> as ConstFromResidual<_>>::from_residual(Outcome::Err(OutOfRange))
        }
    }
}

fn main() {
    let _ = add_or_refuse::<Refusing>(1, 2); // satisfiable
    let _ = add_or_refuse::<Total>(1, 2); // must not be
    println!("F2 compiled");
}
