// Probe 6. The `Adjustment` residual (`63:864-865`): does it need its own entry
// point, or can it share `Bias`'s emission machinery under a different wrapping
// constructor?
//
// The residual is posed as a question about MACHINERY. Under the keying rule it
// is a question about IDENTITY, and it answers itself.
//
// `Bias` and `Adjustment` are both signed, gcd-normalised, value-unique sealed
// rationals (`63:174-175`). Their parse, reduce and decompose steps are the
// same arithmetic on the same digits. So the machinery genuinely is shared, and
// a design that writes it twice has a missing generator.
//
// But they are not the same FACT. `Implicit<E: Exponent, A: Adjustment,
// B: Bias>` (`63:161`) puts them in two positions of one type, and they enter
// the value map at different places: an adjustment scales, a bias offsets. File
// 66 relies on exactly that difference in section 3.2, where the exponent-shift
// symmetry is proved conditional on "no `Numeral` member contributing a nonzero
// additive constant to the value" and holds for `Ranged` only because `Ranged`
// carries no `Bias`.
//
// So: if one shared constructor mints a type usable in both positions, the two
// can be exchanged. This probe compiles both arrangements and measures what the
// exchange costs.

#![allow(dead_code)]

// --------------------------------------------------------------------------
// Shared host-side machinery. ONE generator, used by both doors below. This is
// the half of the residual whose answer is "yes, share it".
// --------------------------------------------------------------------------
const fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}
/// What the macro does host-side for either role: parse, reduce, decompose.
const fn reduce(n: i64, d: u64) -> (i64, u64) {
    let g = gcd(n.unsigned_abs(), d) as i64;
    (n / g, d / (g as u64))
}

// --------------------------------------------------------------------------
// ARRANGEMENT 1: one shared face type, role carried by argument position only.
// --------------------------------------------------------------------------
mod shared_door {
    pub trait Rational {
        const NUM: i64;
        const DEN: u64;
    }
    /// One emission, usable at either position.
    pub struct Rat<const N: i64, const D: u64>;
    impl<const N: i64, const D: u64> Rational for Rat<N, D> {
        const NUM: i64 = N;
        const DEN: u64 = D;
    }

    /// value(k) = A * k * 2^E + B. A scales, B offsets.
    pub struct Implicit<const E: i32, A: Rational, B: Rational>(core::marker::PhantomData<(A, B)>);

    pub fn value<const E: i32, A: Rational, B: Rational>(k: i64) -> f64 {
        let a = A::NUM as f64 / A::DEN as f64;
        let b = B::NUM as f64 / B::DEN as f64;
        a * (k as f64) * (2f64).powi(E) + b
    }
}

// --------------------------------------------------------------------------
// ARRANGEMENT 2: two doors, one shared generator underneath. The wrapping
// constructor carries the role, so the role is a fact in the type.
// --------------------------------------------------------------------------
mod role_typed_doors {
    pub trait Adjustment {
        const NUM: i64;
        const DEN: u64;
    }
    pub trait Bias {
        const NUM: i64;
        const DEN: u64;
    }
    /// `adjustment!(EXPR)` emits this.
    pub struct Adj<const N: i64, const D: u64>;
    /// `raw_bias!(EXPR)` emits this.
    pub struct Bia<const N: i64, const D: u64>;

    impl<const N: i64, const D: u64> Adjustment for Adj<N, D> {
        const NUM: i64 = N;
        const DEN: u64 = D;
    }
    impl<const N: i64, const D: u64> Bias for Bia<N, D> {
        const NUM: i64 = N;
        const DEN: u64 = D;
    }

    pub struct Implicit<const E: i32, A: Adjustment, B: Bias>(core::marker::PhantomData<(A, B)>);

    pub fn value<const E: i32, A: Adjustment, B: Bias>(k: i64) -> f64 {
        let a = A::NUM as f64 / A::DEN as f64;
        let b = B::NUM as f64 / B::DEN as f64;
        a * (k as f64) * (2f64).powi(E) + b
    }
}

fn main() {
    // Reduction is the shared generator, and it is the same call for both
    // roles. This is the half of the residual that answers "share it".
    assert_eq!(reduce(2, 6), (1, 3));
    assert_eq!(reduce(-4, 8), (-1, 2));

    // ---- ARRANGEMENT 1: the exchange is silent -------------------------
    use shared_door as sd;
    type X = sd::Rat<1, 3>; // meant as the adjustment
    type Y = sd::Rat<7, 1>; // meant as the bias

    let right = sd::value::<2, X, Y>(3); // (1/3)*3*4 + 7 = 11
    let swapped = sd::value::<2, Y, X>(3); // 7*3*4 + 1/3 = 84.333...

    assert!((right - 11.0).abs() < 1e-12);
    assert!((swapped - 84.0 - 1.0 / 3.0).abs() < 1e-12);
    // Both compile. Both run. They denote different numbers. Nothing said so.
    assert!((right - swapped).abs() > 70.0);

    // ---- ARRANGEMENT 2: the right one compiles ------------------------
    use role_typed_doors as rt;
    type A = rt::Adj<1, 3>;
    type B = rt::Bia<7, 1>;
    let right2 = rt::value::<2, A, B>(3);
    assert!((right2 - right).abs() < 1e-12);
    // and the exchange does not compile at all; see probe_6b.

    println!("  shared generator, reduce(2,6) = {:?}", reduce(2, 6));
    println!("  arrangement 1 (one door):  correct {right}, swapped {swapped}, both compile");
    println!("  arrangement 2 (two doors): correct {right2}; swapped refuses (probe_6b)");
    println!(
        "\n  the swap changes the value by {:.3}",
        (swapped - right).abs()
    );
    println!("  ALL ASSERTIONS PASSED");
}
