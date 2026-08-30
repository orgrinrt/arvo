//! p1b: the repair for p1a. The law's truth stops being a declaration and becomes a
//! computation over the policy's own map, evaluated per instantiation at compile time.
//! The lie p1a could write is then unwritable: instantiating the licensed consumer at
//! a policy where the law is false is a const-eval failure, not a wrong answer.
//!
//! This is the law-layer form of `68`'s canon-shaped lesson at `68:145-148`:
//! "a representation's declared properties are worth exactly nothing unless validation
//! runs through the maps; validation of declarations against declarations is paper
//! checking paper."
//!
//! Two arms, one file:
//!   rustc --edition 2021 -O p1b_computed_law_refuses.rs -o p1b        (honest, compiles)
//!   rustc --edition 2021 -O --cfg lie p1b_computed_law_refuses.rs     (the lie, refused)
//!
//! Toolchain: nightly-2026-05-28. No feature gates.

const LO: i32 = -8;
const HI: i32 = 7;
const N: i32 = HI - LO + 1;

const P_WRAP: u8 = 0;
const P_SAT: u8 = 1;

/// The map, in const-callable form so a law over it can be evaluated at compile time.
const fn add(p: u8, a: i32, b: i32) -> i32 {
    match p {
        P_WRAP => {
            let mut r = (a + b - LO) % N;
            if r < 0 {
                r += N;
            }
            r + LO
        }
        _ => {
            let s = a + b;
            if s > HI {
                HI
            } else if s < LO {
                LO
            } else {
                s
            }
        }
    }
}

/// The law, computed rather than declared: exhaustive over the model window.
const fn assoc_holds(p: u8) -> bool {
    let mut a = LO;
    while a <= HI {
        let mut b = LO;
        while b <= HI {
            let mut c = LO;
            while c <= HI {
                if add(p, add(p, a, b), c) != add(p, a, add(p, b, c)) {
                    return false;
                }
                c += 1;
            }
            b += 1;
        }
        a += 1;
    }
    true
}

trait Policy {
    const TAG: u8;
}

struct Wrap;
struct SatSigned;

impl Policy for Wrap {
    const TAG: u8 = P_WRAP;
}
impl Policy for SatSigned {
    const TAG: u8 = P_SAT;
}

/// The permission is no longer writable by an author. It is a blanket impl whose
/// associated const runs the law over the policy's own map, so it is only inhabited
/// where the law is true, and the check fires per instantiation actually reached.
trait AssocProven: Policy {
    const PROOF: () = assert!(
        assoc_holds(Self::TAG),
        "this policy's addition is not associative over the model window, so a \
         reassociating consumer may not be instantiated at it"
    );
}
impl<P: Policy> AssocProven for P {}

/// The licensed consumer. The only new thing versus p1a is the first line.
fn reassociating_fold<P: AssocProven>(xs: &[i32]) -> i32 {
    let () = <P as AssocProven>::PROOF;
    match xs.len() {
        1 => xs[0],
        n => {
            let mid = n / 2;
            let l = reassociating_fold_inner(P::TAG, &xs[..mid]);
            let r = reassociating_fold_inner(P::TAG, &xs[mid..]);
            add(P::TAG, l, r)
        }
    }
}

fn reassociating_fold_inner(tag: u8, xs: &[i32]) -> i32 {
    match xs.len() {
        1 => xs[0],
        n => {
            let mid = n / 2;
            add(
                tag,
                reassociating_fold_inner(tag, &xs[..mid]),
                reassociating_fold_inner(tag, &xs[mid..]),
            )
        }
    }
}

fn main() {
    // The honest instantiation. The law is true for this policy, so the proof
    // discharges during compilation and nothing about it reaches the binary.
    let xs = [-8, -8, -8, 1];
    println!("p1b: the law computed from the policy's own map");
    println!(
        "  assoc_holds(Wrap)      = {}   (evaluated at compile time)",
        const { assoc_holds(P_WRAP) }
    );
    println!(
        "  assoc_holds(SatSigned) = {}   (evaluated at compile time)",
        const { assoc_holds(P_SAT) }
    );
    println!(
        "  reassociating_fold::<Wrap>({:?}) = {}",
        xs,
        reassociating_fold::<Wrap>(&xs)
    );

    // The instantiation p1a was able to write. Under this shape it does not compile.
    #[cfg(lie)]
    println!(
        "  reassociating_fold::<SatSigned>({:?}) = {}",
        xs,
        reassociating_fold::<SatSigned>(&xs)
    );
}
