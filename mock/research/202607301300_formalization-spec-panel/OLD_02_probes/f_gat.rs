// TEST F: sketch obligation 5, the fallibility projection as a GAT bounded on
// notko::ConstTry, with Just<T> and Outcome<T,_> both satisfying it. Then the
// question the obligation does not ask: can a generic operation actually
// RETURN a refusal through the projection?
#![allow(dead_code)]
use notko::{ConstTry, Just, Outcome};

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

// (1) the projection type-checks, and both inhabit it. This is the obligation.
fn ok_value<Q: Quantisation>(v: u32) -> Q::Fallibility<u32>
where
    Q::Fallibility<u32>: ConstTry<Output = u32>,
{
    <Q::Fallibility<u32> as ConstTry>::from_output(v)
}

// (2) the part the obligation does not test: a generic operation that must
//     sometimes REFUSE. There is no bound under which this body can be written.
fn add_or_refuse<Q: Quantisation>(a: u32, b: u32) -> Q::Fallibility<u32> {
    match a.checked_add(b) {
        Some(v) => <Q::Fallibility<u32> as ConstTry>::from_output(v),
        None => todo!("no bound in scope constructs the refusal"),
    }
}

fn main() {
    let _ = ok_value::<Total>(3);
    let _ = ok_value::<Refusing>(3);
    println!("F1 OK: the GAT projection resolves for both inhabitants");
}
