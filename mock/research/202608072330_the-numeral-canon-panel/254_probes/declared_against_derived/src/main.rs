// Seat 253. An exposed law verdict against a derived one, over one algebra.
//
// `question::what_the_admission_contract_asks_a_candidate_to_expose` records
// three options and every one of them asks a candidate to hand over law verdicts.
// This asks what a handed-over verdict is worth.
//
// The algebra is signed saturating addition on the window [-2, 1], which is not
// associative: (1 + 1) + (-2) saturates to 1 and then gives -1, while
// 1 + (1 + (-2)) gives 0. The control algebra is addition modulo 4, which is
// associative. Both are carried as Cayley tables over indices into a value list,
// so the law is a `const fn` over associated consts and needs no unstable
// feature to be evaluated at compile time.
//
// ARM 1 is the exposed form: one algebra, two candidates differing only in the
// verdict they declare about it. Both compile. That is the case that must fail
// for an exposed verdict, and it does not fail, which is the finding.
//
// ARM 2 is the derived form: the same verdict computed from what was exposed. It
// separates the two algebras, so it is not stuck at one answer.
//
// ARM 3 and ARM 4 are the cases that must fail for the derived form, each behind
// its own feature so one cannot mask the other: pinning the lying declaration
// against the derived verdict must refuse at compile time, and pinning the
// honest one must not.

// --- the algebras, as tables so the law is const ---------------------------------

trait Alg {
    const NAME: &'static str;
    const N: usize;
    /// The elements, for printing only.
    const VALUES: &'static [i32];
    /// Row-major Cayley table over indices: `TABLE[i * N + j]` is the index of
    /// the result of combining element `i` with element `j`.
    const TABLE: &'static [usize];
}

/// Signed saturating addition on [-2, 1]. Elements in order: -2, -1, 0, 1.
struct SatWindow;

impl Alg for SatWindow {
    const NAME: &'static str = "saturating add on [-2, 1]";
    const N: usize = 4;
    const VALUES: &'static [i32] = &[-2, -1, 0, 1];
    #[rustfmt::skip]
    const TABLE: &'static [usize] = &[
        // -2   -1    0    1
        0,    0,   0,   1, // -2
        0,    0,   1,   2, // -1
        0,    1,   2,   3, //  0
        1,    2,   3,   3, //  1
    ];
}

/// Addition modulo 4 on {0, 1, 2, 3}. The control algebra.
struct Mod4;

impl Alg for Mod4 {
    const NAME: &'static str = "add modulo 4";
    const N: usize = 4;
    const VALUES: &'static [i32] = &[0, 1, 2, 3];
    #[rustfmt::skip]
    const TABLE: &'static [usize] = &[
        0, 1, 2, 3,
        1, 2, 3, 0,
        2, 3, 0, 1,
        3, 0, 1, 2,
    ];
}

// --- the derived verdict ---------------------------------------------------------

/// Whether the algebra is associative, computed rather than asked for.
///
/// A `const fn`, so choosing derivation over declaration costs nothing at run
/// time, which is what `ruling::never_a_runtime_check_and_one_lowered_path`
/// requires of anything in a lowered path.
const fn is_associative<A: Alg>() -> bool {
    let t = A::TABLE;
    let n = A::N;
    let mut i = 0;
    while i < n {
        let mut j = 0;
        while j < n {
            let mut k = 0;
            while k < n {
                let ij = t[i * n + j];
                let jk = t[j * n + k];
                if t[ij * n + k] != t[i * n + jk] {
                    return false;
                }
                k += 1;
            }
            j += 1;
        }
        i += 1;
    }
    true
}

/// The witness, so a `false` is a fact about the algebra rather than about the
/// loop. Returns the first failing triple as values.
fn first_witness<A: Alg>() -> Option<(i32, i32, i32, i32, i32)> {
    let (t, n, v) = (A::TABLE, A::N, A::VALUES);
    for i in 0..n {
        for j in 0..n {
            for k in 0..n {
                let l = t[t[i * n + j] * n + k];
                let r = t[i * n + t[j * n + k]];
                if l != r {
                    return Some((v[i], v[j], v[k], v[l], v[r]));
                }
            }
        }
    }
    None
}

// --- ARM 1: the verdict as something the candidate exposes -----------------------

/// A candidate that declares its own law verdict, which is the shape all three
/// recorded options ask for.
trait Declares {
    const ASSOCIATIVE: bool;
    /// The algebra it computes in. One algebra, whatever it declares about it.
    type Ambient: Alg;
}

/// The declared verdict as a free parameter over one fixed algebra.
struct Declared<const SAYS: bool>;

impl<const SAYS: bool> Declares for Declared<SAYS> {
    const ASSOCIATIVE: bool = SAYS;
    type Ambient = SatWindow;
}

// --- ARM 3 and ARM 4: the cases that must fail for the derived form ---------------

#[cfg(feature = "the_lie_is_refused")]
const _: () = assert!(
    <Declared<true> as Declares>::ASSOCIATIVE
        == is_associative::<<Declared<true> as Declares>::Ambient>(),
    "a declared verdict disagreeing with the algebra it is about"
);

#[cfg(feature = "the_honest_declaration_is_refused_too")]
const _: () = assert!(
    <Declared<false> as Declares>::ASSOCIATIVE
        == is_associative::<<Declared<false> as Declares>::Ambient>(),
    "a declared verdict disagreeing with the algebra it is about"
);

// The derived verdicts, forced into consts so the run cannot be confused with a
// run-time computation.
const SAT_IS_ASSOCIATIVE: bool = is_associative::<SatWindow>();
const MOD4_IS_ASSOCIATIVE: bool = is_associative::<Mod4>();

fn main() {
    println!("ALGEBRAS, and the derived verdict is a const");
    println!(
        "  {:<26} associative : {}",
        <SatWindow as Alg>::NAME,
        SAT_IS_ASSOCIATIVE
    );
    println!(
        "  {:<26} associative : {}",
        <Mod4 as Alg>::NAME,
        MOD4_IS_ASSOCIATIVE
    );
    match first_witness::<SatWindow>() {
        Some((a, b, c, l, r)) => {
            println!("  witness on the first : ({a}+{b})+{c} = {l}, {a}+({b}+{c}) = {r}")
        }
        None => println!("  witness on the first : none, which would break this probe"),
    }
    match first_witness::<Mod4>() {
        Some((a, b, c, l, r)) => {
            println!("  witness on the control: ({a}+{b})+{c} = {l}, {a}+({b}+{c}) = {r}")
        }
        None => println!("  witness on the control: none, as it should be"),
    }
    println!();

    println!("ARM 1: the verdict exposed by the candidate, over ONE algebra");
    println!(
        "  Declared<true>  : ASSOCIATIVE = {:<5} over {}",
        <Declared<true> as Declares>::ASSOCIATIVE,
        <<Declared<true> as Declares>::Ambient as Alg>::NAME
    );
    println!(
        "  Declared<false> : ASSOCIATIVE = {:<5} over {}",
        <Declared<false> as Declares>::ASSOCIATIVE,
        <<Declared<false> as Declares>::Ambient as Alg>::NAME
    );
    println!("  both compile, and the algebra is the same type in both");
    println!("  => an exposed verdict admits both truth values over one algebra");
    println!("  => the case that must fail for an exposed verdict does not fail");
    println!();

    println!("ARM 2: the verdict derived from what was exposed");
    println!("  derived over Declared<_>'s own ambient : {SAT_IS_ASSOCIATIVE}");
    println!("  derived over the control algebra       : {MOD4_IS_ASSOCIATIVE}");
    println!("  => it separates the two algebras, so it is not stuck at one answer");
    println!("  => and it contradicts Declared<true>, which nothing in ARM 1 can");
}
