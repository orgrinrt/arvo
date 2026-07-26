//! Compile-fail pin: a purely fractional type has no multiplicative identity.
//!
//! The refusal is the whole mechanism, and a refusal that nothing guards can
//! be deleted by accident: loosening the bound on `Identity<Multiplicative>`
//! would silently restore four wrong constants, and every runtime test in the
//! suite would still pass, because none of them can name the case.

#[test]
fn no_multiplicative_identity_at_zero_integer_bits() {
    let t = trybuild::TestCases::new();
    // Per impl and per strategy. One case is not a pin: a review found that
    // loosening the bound on `IFixed` left the suite green, because only the
    // `UFixed` refusal was named, and the topic had called the signed case the
    // worst of the four.
    t.compile_fail("tests/ui/no_multiplicative_identity.rs");
    t.compile_fail("tests/ui/no_multiplicative_identity_signed_hot.rs");
    t.compile_fail("tests/ui/no_multiplicative_identity_unsigned_warm.rs");
    t.compile_fail("tests/ui/no_multiplicative_identity_unsigned_cold.rs");
    t.compile_fail("tests/ui/no_multiplicative_identity_unsigned_precise.rs");
    t.compile_fail("tests/ui/no_multiplicative_identity_signed_warm.rs");
    t.compile_fail("tests/ui/no_multiplicative_identity_signed_cold.rs");
    t.compile_fail("tests/ui/no_multiplicative_identity_signed_precise.rs");

    // A different refusal, and one nothing pinned: unsigned has no minus one,
    // so `SignedIdentity` must not reach `UFixed` at any width or strategy.
    t.compile_fail("tests/ui/no_signed_identity_on_unsigned.rs");
}
