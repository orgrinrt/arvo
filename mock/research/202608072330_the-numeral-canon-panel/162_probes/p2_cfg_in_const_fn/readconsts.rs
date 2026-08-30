// Companion: prints the two consts so the hazard is a number rather than asm.
include!("cfg_const_body.rs");
fn main() {
    println!("  HAZARD          R(MAX+1) = {}", HAZARD);
    println!("  CONTROL_STABLE  R(MAX+1) = {}", CONTROL_STABLE);
}
