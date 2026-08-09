// Probe 3: Move B, make the composition reachable from the bound. One
// identity-projection trait, one blanket impl, and Rompf's own mechanism
// (21_rompf_what_a_fact_is_keyed_on.md section 3, "a derived fact is a
// const fn whose parameters are its key") becomes directly callable from
// a generic function that only ever bounds on `Numeric`, never on
// `Number<N, S>`. Needed precisely when the fact's key cannot be supplied
// by the caller and must instead be read out of the concrete type: here,
// stood in as an adaptive choice keyed on the numeral's own width, which
// nothing at fold_quad's call site could know independently of F.
//
// rustc +nightly-2026-05-28 03_move_b_project_the_identity.rs
//   (expect: compiles, runs; the const block inside `fact_gated` is
//    evaluated once per monomorphisation, per Rompf's own discipline)
// rustc +nightly-2026-05-28 --cfg fail 03_move_b_project_the_identity.rs
//   (expect: E0080, evaluation panicked, naming the composition and the
//    arity the fact failed at; not E0277, since there is no marker trait
//    here for the compiler to report as unsatisfied)

trait Numeral {
    const WIDTH: u32;
}
#[derive(Clone, Copy)]
struct Fixed3;
impl Numeral for Fixed3 {
    const WIDTH: u32 = 3;
}
#[derive(Clone, Copy)]
struct Fixed7;
impl Numeral for Fixed7 {
    const WIDTH: u32 = 7;
}

trait Resolve {
    const WRAPS: bool;
}
#[derive(Clone, Copy)]
struct Wrap;
impl Resolve for Wrap {
    const WRAPS: bool = true;
}
#[derive(Clone, Copy)]
struct Saturate;
impl Resolve for Saturate {
    const WRAPS: bool = false;
}

#[derive(Clone, Copy)]
struct Number<N: Numeral, S: Resolve>(core::marker::PhantomData<(N, S)>);
impl<N: Numeral, S: Resolve> Number<N, S> {
    fn new() -> Self {
        Number(core::marker::PhantomData)
    }
}

// Move B: the identity-projection trait. Declared once, in whatever crate
// already hosts `Numeral` and `Resolve` (a contracts-style crate an
// algorithm crate can depend on without touching the facade). Blanket
// impl'd once, in whatever crate declares `Number<N, S>` (the facade).
trait Numeric {
    type N: Numeral;
    type S: Resolve;
}
impl<N: Numeral, S: Resolve> Numeric for Number<N, S> {
    type N = N;
    type S = S;
}

// Rompf's mechanism, unmodified: a derived fact is a const fn whose
// parameters are its key. Here: "is this composition's Add associative at
// fold arity ARITY", with the interior-safety threshold read off the
// numeral's own width, which is data the type carries and the caller
// cannot supply from outside.
const fn add_assoc_at<N: Numeral, S: Resolve, const ARITY: usize>() -> bool {
    if S::WRAPS {
        true
    } else {
        // stand-in for Rompf's real closed form: the accumulator must be
        // wide enough to hold the interior sum of ARITY - 1 elements;
        // here, arbitrarily, "wide enough" means WIDTH >= ARITY.
        N::WIDTH as usize >= ARITY
    }
}

// the algorithm crate: bound only on `Numeric` (plus whatever ops it
// needs), never on `Number<N, S>` directly, and reaches the fact by
// projecting F's own identity out through the bound.
fn fact_gated<F: Numeric + Copy, const ARITY: usize>(x: F) -> F {
    const {
        assert!(
            add_assoc_at::<F::N, F::S, ARITY>(),
            "this composition's Add is not associative at this arity; \
             widen the numeral, or pick a resolution that commutes"
        );
    }
    x
}

fn main() {
    let w = Number::<Fixed3, Wrap>::new();
    let _ = fact_gated::<_, 4>(w); // Wrap: holds at every width, every arity

    #[cfg(not(fail))]
    {
        let s7 = Number::<Fixed7, Saturate>::new();
        let _ = fact_gated::<_, 4>(s7); // Saturate, width 7 >= arity 4: holds
    }

    #[cfg(fail)]
    {
        let s3 = Number::<Fixed3, Saturate>::new();
        let _ = fact_gated::<_, 4>(s3); // Saturate, width 3 < arity 4: refused
    }

    println!("fact_gated reached the fact by projecting F's own identity through the bound");
}
