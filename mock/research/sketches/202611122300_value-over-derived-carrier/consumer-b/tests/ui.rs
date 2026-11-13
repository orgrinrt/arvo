//! The compile-fail pairs against arm B: each failing file beside a twin that
//! differs in the offending token only (the private field's twin reads through
//! the `raw` door, since no single token makes the field reachable). Arm B has
//! no carrier parameter, so the wrongly named carrier has no pair here.
//!
//! Four pairs on `Counted`. In a function, with the recursion limit raised so the
//! count reaches its own check: 129 terms of `UFixed<8, 0>` (bound 128) against
//! 128, and 257 of `Integer<8>` (bound 256) against 256. In a `const` item, 129
//! against 128, where const evaluation refuses the exact add first. And 128
//! terms against 127 at the default limit, where the nested count type is
//! refused before any check runs.

#[test]
fn the_pairs() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/fail/*.rs");
    t.pass("tests/ui/pass/*.rs");
}
