// Can a const expression observe the build profile?
//
// If it can, `evaluation_site` and `build_profile` are not orthogonal in
// general, which bounds the independence the matrix shows for the three
// arithmetic operations. Reporting a limit on my own finding is the point of
// this arm.
const PROFILE_SEEN_AT_CONST: bool = cfg!(debug_assertions);
fn main() {
    println!("const saw debug_assertions = {}", PROFILE_SEEN_AT_CONST);
    println!("run   saw debug_assertions = {}", cfg!(debug_assertions));
}
