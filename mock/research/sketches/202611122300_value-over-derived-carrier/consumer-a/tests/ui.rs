//! The compile-fail pairs against arm A: each failing file beside a twin that
//! differs in the offending token only (the private field's twin reads through
//! the `raw` door, since no single token makes the field reachable).

#[test]
fn the_pairs() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/fail/*.rs");
    t.pass("tests/ui/pass/*.rs");
}
