//! What the types refuse, shown refused by the compiler.

#[test]
fn widths_the_types_do_not_admit() {
    let cases = trybuild::TestCases::new();
    cases.compile_fail("tests/ui/a_width_with_no_carrier.rs");
    cases.compile_fail("tests/ui/more_bits_than_the_digest_has.rs");
}
