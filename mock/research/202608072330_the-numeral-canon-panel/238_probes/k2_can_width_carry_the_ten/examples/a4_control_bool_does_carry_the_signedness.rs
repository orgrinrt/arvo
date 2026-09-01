// Control, the other direction. One of the ten really does close with a type
// the door already has.
//
// `Ambient::SIGNED: bool` is a truth value and `Bool` is the stack's truth
// value, defined eleven lines from `Width` in the same file. This builds, so
// the answer to the first option is "one of the ten", not "none of them", and
// the arms above are about the other nine rather than about associated
// constants being unable to take an arvo type at all.

use arvo_format::width::Bool;

pub trait AmbientByBool {
    const SIGNED: Bool;
}

pub struct MyDomain;

impl AmbientByBool for MyDomain {
    const SIGNED: Bool = Bool::TRUE;
}

fn main() {
    let _ = <MyDomain as AmbientByBool>::SIGNED;
}
