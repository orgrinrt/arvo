// C-G. The caller must not be able to name the opaque type. If this COMPILES,
// the opaque arm in perimeter.rs was never opaque and result 2 means nothing.
// Expected: refused.
trait Carry: Copy {
    fn finish(self, c: i32) -> i32;
}
impl Carry for i64 {
    fn finish(self, c: i32) -> i32 {
        (self - c as i64) as i32
    }
}

fn mk_wide_opaque(a: i32, b: i32) -> impl Carry {
    a as i64 + b as i64
}

fn main() {
    // Naming the concrete type behind the opaque return. Must be refused.
    let t: i64 = mk_wide_opaque(1, 2);
    println!("{t}");
}
