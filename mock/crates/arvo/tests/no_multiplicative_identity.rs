//! Compile-fail pin: a purely fractional type has no multiplicative identity.
//!
//! The refusal is the whole mechanism, and a refusal that nothing guards can
//! be deleted by accident: loosening the bound on `Identity<Multiplicative>`
//! would silently restore four wrong constants, and every runtime test in the
//! suite would still pass, because none of them can name the case.

#[test]
fn no_multiplicative_identity_at_zero_integer_bits() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/no_multiplicative_identity.rs");
}
