//! The map's row assertion, shown failing on a wrong row, beside its twin.

#[test]
fn the_wrong_row_is_refused() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/fail/*.rs");
    t.pass("tests/ui/pass/*.rs");
}
