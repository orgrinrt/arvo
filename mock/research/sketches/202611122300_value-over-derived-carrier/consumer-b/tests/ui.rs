//! The compile-fail pairs against arm B: each failing file beside a twin that
//! differs in the offending token only (the private field's twin reads through
//! the `raw` door, since no single token makes the field reachable). Arm B has
//! no carrier parameter, so the wrongly named carrier has no pair here.
//!
//! Two pairs on `Counted`: 129 terms of `UFixed<8, 0>` (bound 128) against 128,
//! with the recursion limit raised to 256 so the count reaches its own check;
//! and 128 terms against 127 at the default limit, where the nested count type
//! is refused before any check runs.

#[test]
fn the_pairs() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/fail/*.rs");
    t.pass("tests/ui/pass/*.rs");
}
