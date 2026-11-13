//! The pair behind arm D's turbofish: a call to a function generic over the
//! signature cannot infer `S` from `Num<S>`, which is a projection through the
//! carrier, and is refused with E0282; the twin names `S`.

#[test]
fn the_pairs() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/fail/*.rs");
    t.pass("tests/ui/pass/*.rs");
}
