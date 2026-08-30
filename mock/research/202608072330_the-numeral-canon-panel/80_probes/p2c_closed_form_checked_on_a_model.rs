//! p2c: the escape from p2b's wall, and exactly how much of it is still a declaration.
//!
//! p2b establishes that the swept form of a law verdict cannot produce a POSITIVE
//! answer at a width arvo ships: at arity 3 the frontier is width 5 (p2_frontier.py),
//! and confirming a true law at width 8 is refused under `long_running_const_eval`.
//!
//! The only thing that scales is a CLOSED-FORM verdict: a function of the typestate
//! evaluated in constant time, independent of the width. Wrapping addition realises
//! the cyclic group of its width and every group is associative, so its verdict is
//! `true` at every width without enumerating anything.
//!
//! But a closed form is a declaration again, which is what p1a showed is worth
//! nothing on its own. This file builds the thing that is neither: the closed form is
//! kept, and it is CROSS-CHECKED against the swept form at every width the sweep can
//! actually reach, at compile time. What remains unchecked is then exactly one named
//! thing, the transfer of the agreement from the model widths to the shipped ones,
//! rather than the whole verdict.
//!
//!   rustc --edition 2021 -O p2c_closed_form_checked_on_a_model.rs -o p2c
//!   rustc --edition 2021 -O --cfg badclosed p2c_closed_form_checked_on_a_model.rs
//!
//! Expected: the first compiles and runs, and gates an arm at width 64 instantly.
//! The second perturbs one entry of the closed form and the agreement check refuses.
//!
//! Toolchain: nightly-2026-05-28. No feature gates.

const P_WRAP: u8 = 0;
const P_SAT: u8 = 1;

/// The widest width at which the swept form is evaluable for an arity-3 law,
/// measured by p2_frontier.py on this toolchain and this host.
const MODEL_MAX_W: u32 = 5;
const MODEL_MIN_W: u32 = 2;

const fn lo(w: u32) -> i64 {
    -(1i64 << (w - 1))
}
const fn hi(w: u32) -> i64 {
    (1i64 << (w - 1)) - 1
}

const fn add(p: u8, w: u32, a: i64, b: i64) -> i64 {
    let (l, h) = (lo(w), hi(w));
    if p == P_WRAP {
        let n = h - l + 1;
        let mut r = (a + b - l) % n;
        if r < 0 {
            r += n;
        }
        r + l
    } else {
        let s = a + b;
        if s > h {
            h
        } else if s < l {
            l
        } else {
            s
        }
    }
}

/// The verdict that an arm actually gates on. Constant time, any width.
/// Its content is a theorem, not a computation: wrapping addition at width w is
/// addition in Z/2^w, which is a group, and a group is associative. Saturation
/// has no such argument and its verdict is false.
const fn closed_verdict(p: u8, _w: u32) -> bool {
    if p == P_WRAP {
        true
    } else {
        // The perturbation: one entry of the closed form made to lie.
        #[cfg(badclosed)]
        {
            true
        }
        #[cfg(not(badclosed))]
        {
            false
        }
    }
}

/// The verdict computed through the map. Exhaustive, and only evaluable at widths
/// under the frontier, which is why it cannot be what an arm gates on.
const fn swept_verdict(p: u8, w: u32) -> bool {
    let (l, h) = (lo(w), hi(w));
    let mut a = l;
    while a <= h {
        let mut b = l;
        while b <= h {
            let mut c = l;
            while c <= h {
                if add(p, w, add(p, w, a, b), c) != add(p, w, a, add(p, w, b, c)) {
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

/// The discipline `68` asks for, applied where it can run: the declaration is checked
/// against the maps. Not at the shipped width, which is impossible, but over the whole
/// model band, which is possible and is more than nothing.
const fn closed_agrees_with_sweep() -> bool {
    let mut w = MODEL_MIN_W;
    while w <= MODEL_MAX_W {
        if closed_verdict(P_WRAP, w) != swept_verdict(P_WRAP, w) {
            return false;
        }
        if closed_verdict(P_SAT, w) != swept_verdict(P_SAT, w) {
            return false;
        }
        w += 1;
    }
    true
}

const AGREEMENT: () = assert!(
    closed_agrees_with_sweep(),
    "the closed-form law verdict disagrees with the swept verdict somewhere in the \
     model band, so the closed form is wrong and no arm may be gated on it"
);

/// An arm gated on the closed verdict, at a width no sweep can reach.
struct Policy<const P: u8>;

trait Licensed {
    const CHECK: ();
}
impl<const P: u8> Licensed for Policy<P> {
    const CHECK: () = {
        let () = AGREEMENT;
        assert!(
            closed_verdict(P, 64),
            "this policy's addition is not associative, so a reassociating consumer \
             may not be instantiated at it"
        );
    };
}

fn reassociating_consumer<L: Licensed>() -> &'static str {
    let () = <L as Licensed>::CHECK;
    "licensed"
}

fn main() {
    println!("p2c: closed-form verdict, cross-checked against the sweep on a model band");
    println!("  model band: widths {}..={}", MODEL_MIN_W, MODEL_MAX_W);
    println!(
        "  agreement over the band, evaluated at compile time: {}",
        const { closed_agrees_with_sweep() }
    );
    println!(
        "  closed_verdict(wrap, w = 64) = {}   (constant time, no enumeration)",
        const { closed_verdict(P_WRAP, 64) }
    );
    println!(
        "  closed_verdict(sat,  w = 64) = {}",
        const { closed_verdict(P_SAT, 64) }
    );
    println!(
        "  reassociating_consumer::<Policy<{}>>() = {}",
        P_WRAP,
        reassociating_consumer::<Policy<P_WRAP>>()
    );
    println!();
    println!("What is still unchecked, and it is now exactly one named thing:");
    println!("  the transfer of the agreement from widths 2..=5 to width 64.");
}
