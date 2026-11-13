//! The control. Had today's alias taken the adaptation as it is, with no
//! projection, a consumer would have written its generic code over
//! `A: Adaptation`. That code meets the later alias, which reads its parameter
//! through `NamesAdaptation`, and must stop compiling: that is the break the
//! projection exists to take out of the future.

use arvo_format::Adaptation;
use value_later as value;

fn generic<A: Adaptation>(v: value::Uint<8, A>) -> value::Uint<8, A> {
    v
}

fn main() {}
