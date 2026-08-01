// 21_probes/01: the key of a derived fact is the signature of the function that proves it.
//
// Question. File 18 found a law whose key omitted the accumulator, and file 19 generalised
// that every silent break in this dive is a key that omits a dimension its proof used. Can a
// design make the omission impossible rather than merely discouraged, under this workspace's
// feature bans (no `generic_const_exprs`, no full `specialization`, no dependent types)?
//
// Claim under test, in two halves.
//
//   (a) SCOPE. If the proof is a `const fn` whose parameters are the key, a proof body cannot
//       mention a parameter the key does not bind, because it is not in scope. Omission in this
//       direction is a plain name-resolution error.
//
//   (b) DIFFERENTIAL INSTANTIATION. Scope cannot catch the other direction: a proof that
//       silently DEFAULTS a parameter rather than binding it. That is what happened to the
//       accumulator (file 18 section 6: "the accumulator was silently taken to be the numeral
//       itself"). The check is to evaluate the verdict at two settings of the candidate
//       parameter and refuse if the verdict moves.
//
// Arms, all `rustc -O` on the pinned nightly, no `#![feature(..)]` anywhere:
//
//   (default)                 the working shape; prints the verdicts
//   --cfg omit_the_key        (a): a body naming an unbound parameter
//   --cfg no_sweep            the design as drafted: law keyed on (resolution, arity),
//                             accumulator defaulted to the numeral, compiles clean and lies
//   --cfg with_sweep          (b): the same law with the differential check; fails to build
//
// Model: signed 3-bit numeral, values [-4, 3]. Accumulator at scale K holds [K*-4, K*3].
// This is file 18's own instrument, reduced to the two arities that fit a hand-written
// grouping enumeration in a const fn.

const NLO: i64 = -4;
const NHI: i64 = 3;

pub const WRAP: u8 = 0;
pub const SATURATE: u8 = 1;
pub const REFUSE: u8 = 2;
pub const SUBZERO: u8 = 3;

pub const fn name(r: u8) -> &'static str {
    match r {
        WRAP => "Wrap     (Hot)",
        SATURATE => "Saturate (Warm/Cold)",
        REFUSE => "Refuse   (Precise)",
        _ => "SubZero",
    }
}

/// The recovery map, at an accumulator whose range is `[lo, hi]`.
/// `None` is refusal. This is the one definition; nothing else computes it.
const fn phi(r: u8, x: i64, lo: i64, hi: i64) -> Option<i64> {
    if x >= lo && x <= hi {
        return Some(x);
    }
    match r {
        WRAP => {
            let m = hi - lo + 1;
            let mut v = (x - lo) % m;
            if v < 0 {
                v += m;
            }
            Some(v + lo)
        }
        SATURATE => Some(if x > hi { hi } else { lo }),
        REFUSE => None,
        _ => Some(0),
    }
}

const fn step(r: u8, a: Option<i64>, b: Option<i64>, lo: i64, hi: i64) -> Option<i64> {
    match (a, b) {
        (Some(x), Some(y)) => phi(r, x + y, lo, hi),
        _ => None,
    }
}

const fn agree(a: Option<i64>, b: Option<i64>) -> bool {
    match (a, b) {
        (None, None) => true,
        (Some(x), Some(y)) => x == y,
        _ => false,
    }
}

// -------------------------------------------------------------------------------------------
// The law, keyed on everything its proof uses: the resolution, the accumulator, the arity.
// -------------------------------------------------------------------------------------------

/// Kleene agreement of every grouping of a fold of `ARITY` elements, at accumulator scale
/// `SCALE`, under resolution `R`. Groupings are written out because a const fn has no `Vec`.
pub const fn regrouping_agrees<const R: u8, const SCALE: i64, const ARITY: usize>() -> bool {
    let lo = NLO * SCALE;
    let hi = NHI * SCALE;

    let mut a = NLO;
    while a <= NHI {
        let mut b = NLO;
        while b <= NHI {
            let mut c = NLO;
            while c <= NHI {
                if ARITY == 3 {
                    // ((ab)c) and (a(bc))
                    let g0 = step(R, step(R, Some(a), Some(b), lo, hi), Some(c), lo, hi);
                    let g1 = step(R, Some(a), step(R, Some(b), Some(c), lo, hi), lo, hi);
                    if !agree(g0, g1) {
                        return false;
                    }
                } else {
                    let mut d = NLO;
                    while d <= NHI {
                        let ab = step(R, Some(a), Some(b), lo, hi);
                        let bc = step(R, Some(b), Some(c), lo, hi);
                        let cd = step(R, Some(c), Some(d), lo, hi);
                        // the five groupings of four leaves
                        let g0 = step(R, step(R, ab, Some(c), lo, hi), Some(d), lo, hi);
                        let g1 = step(R, step(R, Some(a), bc, lo, hi), Some(d), lo, hi);
                        let g2 = step(R, ab, cd, lo, hi);
                        let g3 = step(R, Some(a), step(R, bc, Some(d), lo, hi), lo, hi);
                        let g4 = step(R, Some(a), step(R, Some(b), cd, lo, hi), lo, hi);
                        if !(agree(g0, g1) && agree(g0, g2) && agree(g0, g3) && agree(g0, g4)) {
                            return false;
                        }
                        d += 1;
                    }
                }
                c += 1;
            }
            b += 1;
        }
        a += 1;
    }
    true
}

// -------------------------------------------------------------------------------------------
// (a) SCOPE: a body cannot name what the signature does not bind.
// -------------------------------------------------------------------------------------------

#[cfg(omit_the_key)]
pub const fn regrouping_agrees_without_the_accumulator<const R: u8, const ARITY: usize>() -> bool {
    // SCALE is not a parameter of this function. The proof needs it. It is not in scope.
    let lo = NLO * SCALE;
    let hi = NHI * SCALE;
    let _ = (lo, hi, R, ARITY);
    true
}

// -------------------------------------------------------------------------------------------
// (b) DIFFERENTIAL INSTANTIATION: catching a DEFAULTED parameter, which scope cannot see.
// -------------------------------------------------------------------------------------------

/// The law as the draft keys it: on the composition and nothing else. The accumulator is not
/// omitted from the proof, it is defaulted to the numeral (`SCALE = 1`), which is invisible
/// from the signature. This is `11_current_shape_draft.md:334-338` in miniature.
#[cfg(no_sweep)]
pub fn add_assoc_as_drafted<const R: u8, const ARITY: usize>() -> bool {
    regrouping_agrees::<R, 1, ARITY>()
}

/// The claim a key on `(R, ARITY)` alone implicitly makes: the verdict does not depend on the
/// accumulator. Two settings is the cheapest possible witness of the claim being false.
#[cfg(with_sweep)]
pub const fn verdict_is_accumulator_independent<const R: u8, const ARITY: usize>() -> bool {
    let at_1 = regrouping_agrees::<R, 1, ARITY>();
    let at_3 = regrouping_agrees::<R, 3, ARITY>();
    at_1 == at_3
}

/// The same law, with the check the key's own shape entails. `const { .. }` in a function body
/// is computation rather than a const generic argument in type position, which is the one
/// place this workspace's ban on `generic_const_exprs` leaves open (file 19 section 1).
#[cfg(with_sweep)]
pub fn add_assoc_with_the_check<const R: u8, const ARITY: usize>() -> bool {
    const {
        assert!(
            verdict_is_accumulator_independent::<R, ARITY>(),
            "this law's verdict moves when the accumulator moves, so the accumulator belongs \
             in its key; a fact keyed on the composition alone is asserting something false"
        );
    }
    regrouping_agrees::<R, 1, ARITY>()
}

fn main() {
    #[cfg(not(any(no_sweep, with_sweep, omit_the_key)))]
    {
        println!("verdict of `every grouping agrees`, by resolution and accumulator scale\n");
        println!(
            "{:<22} {:>8} {:>8} {:>8} {:>8}",
            "resolution", "n=3 K1", "n=3 K3", "n=4 K1", "n=4 K3"
        );
        macro_rules! row {
            ($r:expr) => {
                println!(
                    "{:<22} {:>8} {:>8} {:>8} {:>8}",
                    name($r),
                    regrouping_agrees::<{ $r }, 1, 3>(),
                    regrouping_agrees::<{ $r }, 3, 3>(),
                    regrouping_agrees::<{ $r }, 1, 4>(),
                    regrouping_agrees::<{ $r }, 3, 4>(),
                );
            };
        }
        row!(WRAP);
        row!(SATURATE);
        row!(REFUSE);
        row!(SUBZERO);
        println!(
            "\nany row whose K1 and K3 columns differ is a row where a law keyed on the\n\
             composition alone would assert one of the two and mean the other."
        );
    }

    #[cfg(no_sweep)]
    {
        println!("the drafted key, compiled clean, accumulator defaulted and invisible:");
        println!(
            "  add_assoc_as_drafted::<SATURATE, 3>() = {}",
            add_assoc_as_drafted::<SATURATE, 3>()
        );
        println!("  ... and at an accumulator of scale 3 the same composition IS associative.");
    }

    #[cfg(with_sweep)]
    {
        println!(
            "wrap survives the check: {}",
            add_assoc_with_the_check::<WRAP, 3>()
        );
        // The next line is the finding. Uncommented, the build fails.
        println!("saturate: {}", add_assoc_with_the_check::<SATURATE, 3>());
    }

    #[cfg(omit_the_key)]
    {
        println!("{}", regrouping_agrees_without_the_accumulator::<WRAP, 3>());
    }
}
