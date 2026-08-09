// Probe 1: the gap itself. A generic function bound only on arithmetic ops
// never has S (or N) as a name in scope, not because it is hidden, but
// because it was never bound. This is the algorithm-crate shape as it
// stands in arvo-graph / arvo-spectral / arvo-comb today (verified by
// reading arvo-spectral/src/power.rs:38-50 and the crate table in
// 11_current_shape_draft.md section 3.7 / the arvo--lint-forbidden-*
// files, which forbid these crates from depending on the facade at all).
//
// rustc +nightly-2026-05-28 01_the_bound_never_bound_s.rs (expect: compiles)
// rustc +nightly-2026-05-28 --cfg try_name_s 01_the_bound_never_bound_s.rs
//   (predicted E0433; actual E0425, "cannot find type `S` in this scope", same category: not a name in scope, not an inaccessible one)

trait Numeral {}
trait Resolve {}

#[derive(Clone, Copy)]
struct Fixed3;
impl Numeral for Fixed3 {}
#[derive(Clone, Copy)]
struct Warm;
impl Resolve for Warm {}

#[derive(Clone, Copy)]
struct Number<N: Numeral, S: Resolve>(core::marker::PhantomData<(N, S)>);
impl<N: Numeral, S: Resolve> Number<N, S> {
    fn new() -> Self {
        Number(core::marker::PhantomData)
    }
}

trait Add2 {
    fn add2(self, other: Self) -> Self;
}
impl<N: Numeral, S: Resolve> Add2 for Number<N, S> {
    fn add2(self, other: Self) -> Self {
        other // stand-in body
    }
}

// the algorithm-crate shape: generic over a trait bound, never over
// Number<N, S>. Compare arvo-spectral/src/power.rs:38-50,
// `F: Add<Output = F> + Mul<Output = F> + Sqrt<Output = F> +
// Recip<Output = F> + TotalOrd + Copy + FromConstant`; no `S` anywhere.
fn fold3<F: Add2 + Copy>(a: F, b: F, c: F) -> F {
    #[cfg(try_name_s)]
    {
        // this is not "S is inaccessible". S is not a declared name at
        // this point in the program at all.
        let _proof: S = todo!();
    }
    a.add2(b).add2(c)
}

fn main() {
    let x = Number::<Fixed3, Warm>::new();
    let y = Number::<Fixed3, Warm>::new();
    let z = Number::<Fixed3, Warm>::new();
    let _ = fold3(x, y, z);
    println!("fold3 compiled and ran with S never named inside its body");
}
