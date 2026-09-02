// CARRIER MUTANT 1, must FAIL.
//
// Asserts the opposite of what the lib asserts about the shipped copy. If this
// built, the lib's assertion would be vacuous and the whole carrier arm would
// prove nothing.
fn main() {
    const _: () = assert!(q31_carrier::verdict::SHIPPED_ADMITS_63);
}
