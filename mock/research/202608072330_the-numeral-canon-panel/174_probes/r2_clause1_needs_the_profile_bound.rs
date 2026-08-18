//! r2. Is "the same definedness domain" a profile-free predicate? It is not,
//! and clause 1's hypothesis quietly ranges over a profile-dependent object.
//!
//! MY FIRST FRAMING WAS WRONG AND IS RECORDED RATHER THAN DELETED. I set out
//! to show that `173` clause 1 is FALSE at `debug-assertions = on`, on the
//! reasoning that it carries the definedness condition and not the profile
//! condition, while `173` L2 records binding-free distinguishing channels at
//! `on`. The measurements below are the ones I ran for that. **They do not
//! show it**, and thinking about the hypothesis rather than the measurement is
//! what closed it: at `on` the two realisations no longer share a definedness
//! domain, so clause 1's hypothesis fails, the clause says nothing about the
//! pair, and it is vacuous rather than false. Third hypothesis of mine refuted
//! in this dispatch.
//!
//! WHAT THE SAME MEASUREMENTS DO SHOW, which is smaller and still worth an
//! amendment. Realisation A is **total at `off` and partial at `on`**, on 203
//! of 256 inputs. Realisation B is total at both. So:
//!
//!   1. Whether two realisations "have the same definedness domain" is a
//!      function of the build profile, not of the realisations alone. Clause
//!      1's equivalence classes therefore differ between profiles and the
//!      clause does not say so.
//!   2. Clause 2's "where a stretch contains a partial operation" is likewise
//!      profile-dependent: `u8 *` and `u8 +` are total at `off` and partial at
//!      `on`. The candidate has this fact under a different description, as
//!      the overflow panic among L2's binding-free channels, and the two
//!      descriptions are not obviously the same thing to a reader; under the
//!      channel description it reads as one more item to enumerate (O-5),
//!      under this one it is a shift in clause 1's hypothesis.
//!
//! THE CASES THAT MUST BEHAVE A PARTICULAR WAY:
//!   C1. At `off` the two must agree in value on every input and neither may
//!       diverge, or they are not two realisations of one stretch.
//!   C2. At `on` realisation A must diverge somewhere, or the profile does not
//!       move definedness and there is nothing here.
//!   C3. On the inputs where A does return at `on`, the two must still agree in
//!       value, or the pair differs in boundary function and the observation is
//!       about the values rather than about definedness.
//!
//! Run: see r2_run.sh.

const W: u32 = 8;
const K: u8 = 97;

/// Realisation A: the arithmetic written directly at the declared width.
/// `debug-assertions = on` makes `*` and `+` panic on overflow; `off` wraps.
#[inline(never)]
pub fn realisation_a(x: u8) -> u8 {
    let v = x * 3;
    let v = v + K;
    v
}

/// Realisation B: the same stretch with the interior kept in a wider carrier
/// and projected once at the boundary. Total everywhere, at every profile.
#[inline(never)]
pub fn realisation_b(x: u8) -> u8 {
    let v = (x as u32) * 3;
    let v = v + (K as u32);
    (v & ((1u32 << W) - 1)) as u8
}

fn main() {
    let mode = if cfg!(debug_assertions) { "on" } else { "off" };
    // The 203 panics are the measurement, not an error; silence the hook so the
    // committed artifact is readable.
    std::panic::set_hook(std::boxed::Box::new(|_| {}));
    println!("debug-assertions = {mode}");

    let mut agree = 0usize;
    let mut a_panicked = Vec::new();
    let mut value_disagree = Vec::new();

    for x in 0u8..=255 {
        let b = realisation_b(x);
        let a = std::panic::catch_unwind(|| realisation_a(x));
        match a {
            Ok(av) => {
                if av == b {
                    agree += 1;
                } else {
                    value_disagree.push((x, av, b));
                }
            }
            Err(_) => a_panicked.push(x),
        }
    }

    println!("  inputs where the two agree in value : {agree}/256");
    println!("  inputs where realisation A panicked : {}", a_panicked.len());
    println!("  inputs where the values differ      : {}", value_disagree.len());
    if let Some(&x) = a_panicked.first() {
        println!("  first distinguishing input          : x = {x}");
        println!("  realisation B at that input         : {}", realisation_b(x));
    }
    println!();

    if mode == "off" {
        assert_eq!(
            value_disagree.len(),
            0,
            "C1 FAILED: the two realisations disagree in value at off, so they are \
             not two realisations of one stretch"
        );
        assert_eq!(
            a_panicked.len(),
            0,
            "C1 FAILED: a panic escaped at off, so the profile flag is not doing \
             what this probe assumes"
        );
        println!("C1 PASS: at off the two agree on all 256 inputs and neither panics.");
        println!("         They induce the SAME boundary function on the SAME definedness");
        println!("         domain (both total), so clause 1 declares them indistinguishable.");
    } else {
        assert!(
            !a_panicked.is_empty(),
            "C2 FAILED: no input distinguished them at on, so the binding-free \
             channel clause 1 omits does not exist and this amendment is unfounded"
        );
        assert_eq!(
            value_disagree.len(),
            0,
            "C3 FAILED: the two disagree in value where both return, so this is an \
             observation about values rather than about definedness"
        );
        println!("C2 PASS: at on, realisation A diverges on {} of 256 inputs while B is total.", a_panicked.len());
        println!("C3 PASS: on the {agree} inputs where A returns, the two still agree in value.");
        println!();
        println!("SO: realisation A is total at off and partial at on. The pair satisfies");
        println!("clause 1's hypothesis at off and FAILS it at on, which means clause 1 is");
        println!("vacuous for this pair at on rather than false. What is established is that");
        println!("'the same definedness domain' is profile-dependent, so clause 1's");
        println!("equivalence classes are profile-dependent and the clause does not say so;");
        println!("and that which operations are partial is profile-dependent too, which is");
        println!("clause 2's partial-operation hypothesis resting on a moving set.");
    }
}
