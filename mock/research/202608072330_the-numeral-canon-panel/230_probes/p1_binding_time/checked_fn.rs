// The same operation written so the compile-time refusal does not depend on a
// codegen flag: the overflow is a value the body inspects rather than a check
// the backend may or may not have emitted.
#[allow(dead_code)]
const fn checked_add(a: u8, b: u8) -> u8 {
    match a.checked_add(b) {
        Some(v) => v,
        None => panic!("operand sum leaves the representable set"),
    }
}

#[cfg(feature = "const_site")]
const OUT: u8 = checked_add(200, 100);

fn main() {
    #[cfg(feature = "const_site")]
    println!("const {}", OUT);
    #[cfg(not(feature = "const_site"))]
    {
        let a = std::hint::black_box(200u8);
        let b = std::hint::black_box(100u8);
        println!("run {}", checked_add(a, b));
    }
}
