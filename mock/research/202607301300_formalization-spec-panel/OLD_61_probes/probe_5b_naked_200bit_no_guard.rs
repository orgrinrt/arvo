#[path = "tower.rs"]
mod tower;
use tower::*;

type Huge = I<O<O<I<I<I<O<O<O<O<O<O<I<I<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<H>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>;

fn main() {
    let v = <Huge as Pos>::VAL;
    println!("{}", v);
}
