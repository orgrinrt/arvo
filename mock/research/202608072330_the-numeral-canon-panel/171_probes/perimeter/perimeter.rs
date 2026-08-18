// Probe P3. Are the two perimeters the same perimeter?
//
// My delimiter in 167 is BINDING-based: the region ends where an intermediate is
// named. The observability rule's is OBSERVATION-based: a guarantee holds over
// the operations through which a type can be observed. If those coincide, my
// derivation might be the rule under another name. If they can come apart, they
// are different principles and my route does not pass through the rule.
//
// The question in one sentence: can an intermediate be BOUND, at a type that
// differs between the two implementations, and still admit NO distinguishing
// context?
//
// THE CASES THAT MUST FAIL
//   C-F  A binding at a type that is the SAME in both implementations must be
//        indistinguishable. Otherwise this probe reports differences that are
//        artifacts of the binding rather than of the representation.
//   C-G  The opaque arm must actually be opaque: the caller must not be able to
//        write the concrete type. If it can, opacity was never tested.
//   C-H  size_of_val must distinguish at least one pair, else the "always admits
//        a distinguishing context" claim is vacuous.

use core::mem::size_of_val;

// Two representations of the same intermediate quantity.
#[inline(never)]
fn mk_wide(a: i32, b: i32) -> i64 {
    a as i64 + b as i64
}
#[inline(never)]
fn mk_narrow(a: i32, b: i32) -> i32 {
    a.wrapping_add(b)
}

// The same quantity behind an OPAQUE type: the caller cannot name i64 or i32.
trait Carry: Copy {
    fn finish(self, c: i32) -> i32;
}
impl Carry for i64 {
    fn finish(self, c: i32) -> i32 {
        (self - c as i64) as i32
    }
}
impl Carry for i32 {
    fn finish(self, c: i32) -> i32 {
        self.wrapping_sub(c)
    }
}

#[inline(never)]
fn mk_wide_opaque(a: i32, b: i32) -> impl Carry {
    a as i64 + b as i64
}
#[inline(never)]
fn mk_narrow_opaque(a: i32, b: i32) -> impl Carry {
    a.wrapping_add(b)
}

// C-F's pair: both implementations bind the intermediate at the SAME type.
#[inline(never)]
fn mk_same_a(a: i32, b: i32) -> i64 {
    a as i64 + b as i64
}
#[inline(never)]
fn mk_same_b(a: i32, b: i32) -> i64 {
    (a as i64).wrapping_add(b as i64)
}

fn main() {
    let (a, b, c) = (1_500_000_000i32, 1_400_000_000i32, 2_000_000_000i32);
    println!("== Probe P3: binding perimeter against observation perimeter ==");
    println!("profile: debug_assertions = {}", cfg!(debug_assertions));
    println!();

    // 1. Transparent binding at differing types.
    let tw = mk_wide(a, b);
    let tn = mk_narrow(a, b);
    let d1 = size_of_val(&tw) != size_of_val(&tn);
    println!("1. transparent binding, differing types:");
    println!(
        "   size_of_val  wide {}  narrow {}  distinguishes: {d1}",
        size_of_val(&tw),
        size_of_val(&tn)
    );

    // 2. OPAQUE binding at differing types. The caller cannot name the type;
    //    can it still distinguish?
    let ow = mk_wide_opaque(a, b);
    let on = mk_narrow_opaque(a, b);
    let d2 = size_of_val(&ow) != size_of_val(&on);
    println!("2. OPAQUE binding (impl Carry), differing types:");
    println!(
        "   size_of_val  wide {}  narrow {}  distinguishes: {d2}",
        size_of_val(&ow),
        size_of_val(&on)
    );
    println!(
        "   both still compute the same final value: {} == {} -> {}",
        ow.finish(c),
        on.finish(c),
        ow.finish(c) == on.finish(c)
    );

    // C-G: the opaque type cannot be named by the caller. The line below is the
    //      proof obligation and it lives in perimeter_opaque_MUST_NOT_COMPILE.rs,
    //      which is expected to be refused.
    println!("   C-G: see perimeter_opaque_MUST_NOT_COMPILE.rs, expected to be refused");

    // 3. C-F: binding at the SAME type in both.
    let sa = mk_same_a(a, b);
    let sb = mk_same_b(a, b);
    let d3 = size_of_val(&sa) != size_of_val(&sb);
    println!("3. C-F, binding at the SAME type in both implementations:");
    println!(
        "   size_of_val  {}  {}  distinguishes: {d3}   (must be false)",
        size_of_val(&sa),
        size_of_val(&sb)
    );
    println!("   values equal: {}", sa == sb);

    println!();
    println!(
        "C-H  size_of_val distinguishes at least one pair: {}",
        d1 || d2
    );
    println!();
    println!("VERDICT");
    println!(
        "  a binding at a differing type admits a distinguishing context: {}",
        d1 && d2
    );
    println!("  opacity does NOT extend the region: {d2}");
    println!("  a binding at the same type does not: {}", !d3);
}
