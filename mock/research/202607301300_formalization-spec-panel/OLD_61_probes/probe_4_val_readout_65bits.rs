#[path = "tower.rs"]
mod tower;
use tower::*;

type T = O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<H>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>;

// Case A: name the type only, never read VAL.
fn type_only() {
    let _x: core::marker::PhantomData<T> = core::marker::PhantomData;
}

// Case B: read VAL.
fn main() {
    let v = <T as Pos>::VAL;
    println!("{}", v);
}
