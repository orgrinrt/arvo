// p4. Pricing a missed merge by WHERE the two spellings meet.
//
// `110` F8 says two names for one primitive is "a compile error with no
// in-language repair", and `110:282` in the same file says a canonicalisation
// that splits where it could have merged "costs names and nothing else".
// `111` section 7 calls that a contradiction and locates the resolution in
// `110`'s own F9 (parameterise by what R reads, so the second spelling never
// exists).
//
// That resolution is right about what to build and it does not price the act,
// which is what the contradiction was about.  This probe prices it, and the
// price is not one number: it depends on where the two spellings meet.
//
// THREE SITES, PREDICTED BEFORE COMPILING
// ---------------------------------------
// S1. A MONOMORPHIC site never sees the split.  Each spelling has its own
//     call and neither mentions the other.  Cost: zero.
//
// S2. A POLYMORPHIC site CAN be repaired, by abstracting over the parameter.
//     `110`'s "no repair" is about making the two types equal, which is
//     correct and is not the only way to write one function over both.  I
//     predict a generic function accepts both spellings with no feature gate,
//     and that this is the repair `110` did not consider.
//
// S3. A STORAGE site cannot be repaired.  A homogeneous container is one type
//     by construction, so two spellings cannot share one column whatever
//     abstraction the signatures use.  I predict this is where the wall
//     actually is, and it lands on the storage-minimising path, which I17
//     names as not to be deprioritised.
//
// AND THE DIRECTION COUNT
// -----------------------
// p1 classifies an axis by how many directions admit a total
// denotation-preserving map: two means spurious, one means a refinement, zero
// means observable.  Here that count is checked against what the compiler
// will accept, on three axes carried three ways, all in one file:
//
//   - RADIX at F = 0, spurious.  Both casts exist, both are the identity.
//   - a declared BOUND, a refinement.  The widening exists and is the
//     identity; the tightening must be refused.
//   - the OVERFLOW POLICY, observable.  A cast can be written, and it does not
//     commute with the operation, which is measured here rather than asserted.
//
// No `#![feature(...)]` line.  No `dyn`, no `TypeId`, no alloc in the parts
// that model the design (`std` is used only to print).

use core::marker::PhantomData;

// ---------------------------------------------------------------------------
// A spurious parameter: the radix, at a fraction width of zero.
// `110` F5 measures it definitionally degenerate; nothing in the value set or
// in R reads it.
// ---------------------------------------------------------------------------

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct Spur<const RADIX: u32>(u8);

impl<const RADIX: u32> Spur<RADIX> {
    const fn new(v: u8) -> Self {
        Self(v)
    }
    const fn add(self, other: Self) -> Self {
        Self(self.0.saturating_add(other.0))
    }
    const fn raw(self) -> u8 {
        self.0
    }
}

// S2: the repair `110` did not consider.  One function over both spellings.
fn sum_any_radix<const R: u32>(xs: &[Spur<R>]) -> u8 {
    let mut acc = 0u8;
    let mut i = 0;
    while i < xs.len() {
        acc = acc.saturating_add(xs[i].raw());
        i += 1;
    }
    acc
}

// And the two casts, both of which exist and are both the identity.
const fn spur_2_to_10(x: Spur<2>) -> Spur<10> {
    Spur::<10>::new(x.raw())
}
const fn spur_10_to_2(x: Spur<10>) -> Spur<2> {
    Spur::<2>::new(x.raw())
}

// ---------------------------------------------------------------------------
// A refinement parameter: a declared upper bound, carried as a type.
// The bound is a trait's associated const, which is why no arithmetic appears
// in type position and no forbidden feature is needed.
// ---------------------------------------------------------------------------

trait Bound {
    const HI: u8;
}

struct Lit<const N: u8>;
impl<const N: u8> Bound for Lit<N> {
    const HI: u8 = N;
}

/// The propagated bound of a sum, as an associated const's body.
struct BSum<A, B>(PhantomData<(A, B)>);
impl<A: Bound, B: Bound> Bound for BSum<A, B> {
    const HI: u8 = {
        // saturating at the container so an over-container declaration is a
        // value the FITS check below can refuse rather than an overflow here
        let a = A::HI as u16;
        let b = B::HI as u16;
        let s = a + b;
        if s > 255 { 255 } else { s as u8 }
    };
}

#[repr(transparent)]
struct Ref<B: Bound>(u8, PhantomData<B>);

// Hand-written rather than derived.  `#[derive(Copy)]` on a type with a
// phantom parameter adds an implicit `B: Copy` bound, which a marker type
// carrying only an associated const does not satisfy.  Worth one line because
// it is a real cost of carrying a refinement as a type parameter, and it is
// paid once at the definition rather than at every use.
impl<B: Bound> Clone for Ref<B> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<B: Bound> Copy for Ref<B> {}

impl<B: Bound> Ref<B> {
    const fn raw(self) -> u8 {
        self.0
    }
}

impl<B: Bound> Ref<B> {
    /// Construction is where the declaration is discharged.  A value above the
    /// declared bound is not representable through this door.
    fn declare(v: u8) -> Option<Self> {
        if v <= B::HI {
            Some(Ref(v, PhantomData))
        } else {
            None
        }
    }
}

/// Weakening: the one direction a refinement admits.  It is the identity on
/// the representation, and the wrong direction is refused before the program
/// exists.
struct Widen<From, To>(PhantomData<(From, To)>);
impl<From: Bound, To: Bound> Widen<From, To> {
    const CHECK: () = assert!(
        From::HI <= To::HI,
        "widening must not tighten the declared bound"
    );
}

fn widen<From: Bound, To: Bound>(x: Ref<From>) -> Ref<To> {
    let () = Widen::<From, To>::CHECK;
    Ref(x.raw(), PhantomData)
}

/// The licensed arm: no completion, because the propagated bound discharged.
fn add_licensed<A: Bound, B: Bound>(a: Ref<A>, b: Ref<B>) -> Ref<BSum<A, B>> {
    Ref(a.raw() + b.raw(), PhantomData)
}

/// The general arm, for comparison.
fn add_general(a: u8, b: u8) -> u8 {
    a.saturating_add(b)
}

// ---------------------------------------------------------------------------
// An observable parameter: the overflow policy.
// ---------------------------------------------------------------------------

trait Policy {
    fn complete(exact: u16, hi: u8) -> u8;
}
struct Sat;
struct Wrap;
impl Policy for Sat {
    fn complete(exact: u16, hi: u8) -> u8 {
        if exact > hi as u16 { hi } else { exact as u8 }
    }
}
impl Policy for Wrap {
    fn complete(exact: u16, hi: u8) -> u8 {
        (exact % (hi as u16 + 1)) as u8
    }
}

#[repr(transparent)]
struct Obs<P: Policy>(u8, PhantomData<P>);

impl<P: Policy> Clone for Obs<P> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<P: Policy> Copy for Obs<P> {}

impl<P: Policy> Obs<P> {
    const HI: u8 = 15;
    fn new(v: u8) -> Self {
        Obs(v, PhantomData)
    }
    fn add(self, o: Self) -> Self {
        Obs(
            P::complete(self.0 as u16 + o.0 as u16, Self::HI),
            PhantomData,
        )
    }
    fn raw(self) -> u8 {
        self.0
    }
}

/// The cast exists, exactly as it does for the spurious axis.  What differs is
/// whether it commutes with the operation, which is measured below.
fn reinterpret_sat_as_wrap(x: Obs<Sat>) -> Obs<Wrap> {
    Obs(x.raw(), PhantomData)
}

// S2 for an observable axis: a generic function also accepts both spellings.
fn sum_any_policy<P: Policy>(xs: &[Obs<P>]) -> u8 {
    let mut acc = Obs::<P>::new(0);
    let mut i = 0;
    while i < xs.len() {
        acc = acc.add(xs[i]);
        i += 1;
    }
    acc.raw()
}

// ---------------------------------------------------------------------------
// The three emitted-code questions, in `#[inline(never)]` bodies so the
// symbols exist to compare.
// ---------------------------------------------------------------------------

#[inline(never)]
#[no_mangle]
pub fn widen_100_to_200(x: u8) -> u8 {
    widen::<Lit<100>, Lit<200>>(Ref(x, PhantomData)).raw()
}

#[inline(never)]
#[no_mangle]
pub fn widen_7_to_255(x: u8) -> u8 {
    widen::<Lit<7>, Lit<255>>(Ref(x, PhantomData)).raw()
}

#[inline(never)]
#[no_mangle]
pub fn plain_identity(x: u8) -> u8 {
    x
}

#[inline(never)]
#[no_mangle]
pub fn cast_radix_2_to_10(x: u8) -> u8 {
    spur_2_to_10(Spur::<2>::new(x)).raw()
}

#[inline(never)]
#[no_mangle]
pub fn cast_radix_10_to_2(x: u8) -> u8 {
    spur_10_to_2(Spur::<10>::new(x)).raw()
}

#[inline(never)]
#[no_mangle]
pub fn add_licensed_100_100(a: u8, b: u8) -> u8 {
    add_licensed(
        Ref::<Lit<100>>(a, PhantomData),
        Ref::<Lit<100>>(b, PhantomData),
    )
    .raw()
}

#[inline(never)]
#[no_mangle]
pub fn add_general_u8(a: u8, b: u8) -> u8 {
    add_general(a, b)
}

fn main() {
    println!("p4. Where a missed merge actually costs something");
    println!("{}", "=".repeat(74));

    // -- S1: a monomorphic site never sees the split --------------------
    println!();
    println!("S1. Monomorphic sites");
    let a2 = [Spur::<2>::new(3), Spur::<2>::new(4)];
    let a10 = [Spur::<10>::new(3), Spur::<10>::new(4)];
    println!("  sum over the radix-2 spelling  : {}", sum_any_radix(&a2));
    println!("  sum over the radix-10 spelling : {}", sum_any_radix(&a10));
    println!("  neither call mentions the other spelling, so the cost is zero");

    // -- S2: the repair 110 did not consider ----------------------------
    println!();
    println!("S2. One function over both spellings, by abstracting the parameter");
    println!(
        "  sum_any_radix is ONE generic function and both arrays above went \
         through it"
    );
    println!(
        "  sum_any_policy likewise accepts Obs<Sat> and Obs<Wrap>: {} and {}",
        sum_any_policy(&[Obs::<Sat>::new(9), Obs::<Sat>::new(9)]),
        sum_any_policy(&[Obs::<Wrap>::new(9), Obs::<Wrap>::new(9)])
    );
    println!(
        "  so a missed merge is repairable at a FUNCTION boundary, for a \
         spurious axis and for an observable one alike"
    );

    // -- what the abstraction does NOT recover ---------------------------
    println!();
    println!("S3. The storage boundary, which the abstraction does not reach");
    println!(
        "  a homogeneous container is one type by construction, so no generic \
         signature lets"
    );
    println!(
        "  Spur<2> and Spur<10> share one array, one slice or one column. The \
         compile-fail arm"
    );
    println!("  is `p4b`, whose diagnostic is recorded in `p4b_expected_failure.txt`.");

    // -- the direction count, measured ----------------------------------
    println!();
    println!("THE DIRECTION COUNT, checked against what the operations do");
    println!();
    let mut spur_bad = 0usize;
    let mut spur_n = 0usize;
    for x in 0u8..=255 {
        for y in 0u8..=255 {
            spur_n += 1;
            let l = Spur::<2>::new(x).add(Spur::<2>::new(y)).raw();
            let r = spur_10_to_2(
                spur_2_to_10(Spur::<2>::new(x)).add(spur_2_to_10(Spur::<2>::new(y))),
            )
            .raw();
            if l != r {
                spur_bad += 1;
            }
        }
    }
    println!(
        "  spurious (radix): the cast commutes with add on {}/{} pairs, \
         both directions exist",
        spur_n - spur_bad,
        spur_n
    );

    let mut obs_bad = 0usize;
    let mut obs_n = 0usize;
    for x in 0u8..=15 {
        for y in 0u8..=15 {
            obs_n += 1;
            let l = reinterpret_sat_as_wrap(Obs::<Sat>::new(x).add(Obs::<Sat>::new(y))).raw();
            let r = reinterpret_sat_as_wrap(Obs::<Sat>::new(x))
                .add(reinterpret_sat_as_wrap(Obs::<Sat>::new(y)))
                .raw();
            if l != r {
                obs_bad += 1;
            }
        }
    }
    println!(
        "  observable (policy): the cast commutes with add on {}/{} pairs, \
         so it is a REINTERPRETATION and not a map of the algebra",
        obs_n - obs_bad,
        obs_n
    );

    // the refinement: one direction, and it is the identity
    let mut ref_bad = 0usize;
    let mut ref_n = 0usize;
    for x in 0u8..=100 {
        ref_n += 1;
        let narrow = Ref::<Lit<100>>::declare(x).unwrap();
        let wide: Ref<Lit<200>> = widen(narrow);
        if wide.raw() != narrow.raw() {
            ref_bad += 1;
        }
    }
    println!(
        "  refinement (bound): widening is the identity on {}/{} \
         representations, and the tightening is refused before the program \
         exists (`p4c`)",
        ref_n - ref_bad,
        ref_n
    );

    // -- the licensed arm agrees with the general arm inside the bound ---
    println!();
    println!("THE LICENSED ARM, inside and outside the declared bound");
    let mut inside_diff = 0usize;
    let mut inside_n = 0usize;
    for a in 0u8..=100 {
        for b in 0u8..=100 {
            inside_n += 1;
            if add_licensed_100_100(a, b) != add_general_u8(a, b) {
                inside_diff += 1;
            }
        }
    }
    let mut outside_diff = 0usize;
    let mut outside_n = 0usize;
    for a in 0u16..=255 {
        for b in 0u16..=255 {
            if a <= 100 && b <= 100 {
                continue;
            }
            outside_n += 1;
            let licensed = (a as u8).wrapping_add(b as u8);
            if licensed != add_general_u8(a as u8, b as u8) {
                outside_diff += 1;
            }
        }
    }
    println!(
        "  inside the declared bound : {}/{} disagree",
        inside_diff, inside_n
    );
    println!(
        "  outside it                : {}/{} disagree",
        outside_diff, outside_n
    );

    println!();
    println!("Declared bound sums: BSum<Lit<100>, Lit<100>>::HI = {}", <BSum<Lit<100>, Lit<100>> as Bound>::HI);
    println!("                     BSum<Lit<200>, Lit<100>>::HI = {}", <BSum<Lit<200>, Lit<100>> as Bound>::HI);
}
