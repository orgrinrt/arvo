// `Width` where a declared width is actually declared. This is why every const
// generic parameter in the crate is spelled with a machine integer instead.
//
// The position takes an integer, a `bool` or a `char`, so a transparent newtype
// over a bit count is refused for being a struct rather than for anything about
// what it holds. The help names the compiler feature that lifts the restriction,
// which is kept in the expected output on purpose: the escape exists, it is a
// gate, and a crate declaring a format of its own would have to carry it too.
//
// Nothing else is declared here, so there is no second reason this could fail.

use arvo_format::width::Width;

pub struct Signed<const BITS: Width>;

fn main() {}
