// The narrow arm as a const. Expected to be REFUSED at compile time, at BOTH
// profiles, because const evaluation checks arithmetic regardless of
// debug-assertions. If it compiles, the const channel does not distinguish and
// P2's finding is withdrawn.
const A: i32 = 1_500_000_000;
const B: i32 = 1_400_000_000;
const C: i32 = 2_000_000_000;

const fn narrow(a: i32, b: i32, c: i32) -> i32 {
    let t: i32 = a + b; // must be refused: overflows at const time
    t.wrapping_sub(c)
}

const NARROW: i32 = narrow(A, B, C);

fn main() {
    println!("if this printed, the const channel does not distinguish: {NARROW}");
}
