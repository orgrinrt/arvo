// P6b. The same intent, spelled three other ways, to find which one carries it.
//
// R-a: a defaulted TYPE PARAMETER on a trait, `Out` as the parameter.
//      Default fills when the bound elides it. Two impls give two Outs.
// R-b: the same, reached by a METHOD CALL rather than through a bound.
// R-c: a defaulted MODE MARKER on the trait, with the output an ASSOCIATED TYPE
//      projected from the mode. One Out per mode, so no ambiguity anywhere.
#![allow(dead_code)]

#[derive(Debug)]
pub struct Erased(pub u32);

// ---------- R-a / R-b : Out is the trait's own defaulted type parameter ------
pub trait Sum<Out = Self> {
    fn sum(self, o: Self) -> Out;
}
impl Sum<u32> for u32 {
    fn sum(self, o: u32) -> u32 {
        self + o
    }
}
impl Sum<Erased> for u32 {
    fn sum(self, o: u32) -> Erased {
        Erased(self + o)
    }
}

// R-a: through a bound that elides the parameter. The default is used.
fn through_bound<T: Sum>(a: T, b: T) -> T {
    a.sum(b)
}
// R-a': through a bound that names it.
fn through_bound_named<T: Sum<Erased>>(a: T, b: T) -> Erased {
    a.sum(b)
}

// ---------- R-c : mode marker with a default, output projected --------------
pub struct Native;
pub struct Erase;
pub trait Mode {}
impl Mode for Native {}
impl Mode for Erase {}

pub trait Algo<M: Mode = Native> {
    type Out;
    fn run(self, o: Self) -> Self::Out;
}
impl Algo<Native> for u32 {
    type Out = u32;
    fn run(self, o: u32) -> u32 {
        self + o
    }
}
impl Algo<Erase> for u32 {
    type Out = Erased;
    fn run(self, o: u32) -> Erased {
        Erased(self + o)
    }
}

fn mode_default<T: Algo>(a: T, b: T) -> <T as Algo>::Out {
    a.run(b)
}
fn mode_named<T: Algo<Erase>>(a: T, b: T) -> <T as Algo<Erase>>::Out {
    a.run(b)
}

fn main() {
    println!("R-a  bound, default elided : {}", through_bound(3u32, 4));
    println!(
        "R-a' bound, named          : {:?}",
        through_bound_named(3u32, 4)
    );
    println!("R-c  mode default          : {}", mode_default(3u32, 4));
    println!("R-c  mode named            : {:?}", mode_named(3u32, 4));
}
