//! Probe q1: the adaptation slot carries two independent law families, and the
//! classical overflow responses separate along them.
//!
//! Hypothesis. `55` classifies wrap as "not an adaptation" because it fails
//! monotonicity and distance minimisation, and concludes wrap belongs to the
//! ambient-domain slot. This probe tests a sharper structure: there are TWO
//! independent law families a total retraction rho: ambient -> Q can satisfy,
//!
//!   A-laws (adaptation):  monotone, and distance minimising, over the window;
//!   C-law  (coherence):   rho(a op b) == rho(rho(a) op rho(b)) for all a, b
//!                          in the window (the homomorphism-shaped law that
//!                          makes chains exact: reduce-eagerly == reduce-once).
//!
//! and the classical pool inhabits ALL FOUR cells of the 2x2:
//!
//!   A and C:      unsigned add-only saturation (one reachable bound)
//!   A, not C:     signed two-sided saturation
//!   C, not A:     two's-complement wrap
//!   neither:      clamp-to-opposite-bound (the deliberate mutant)
//!
//! If all four cells are inhabited, then "wrap is not an adaptation" is one
//! half of a symmetric statement whose other half is "saturation is not
//! coherent", and neither law family subsumes the other. The chain-exactness
//! consequence is measured too: for each map, the count of triples (a, b, c)
//! in Q^3 with rho(rho(a+b)+c) != rho(a+b+c), which is the eager-vs-once
//! reduction divergence that Q12's reassociation measurements are about.
//!
//! Instrument validation is by construction: each check must report false on
//! at least one row (monotone and nearest fail on wrap and on the mutant;
//! coherence fails on signed clamp and on the mutant). A checker that cannot
//! fail would report every cell identically; the four distinct row profiles
//! are the demonstration that every checker fires both ways.
//!
//! All checks exhaustive. Signed Q = [-8, 7], unsigned Q = [0, 15], ambient
//! window [-64, 64] (signed) and [0, 64] (unsigned add-only).

const LO: i64 = -8;
const HI: i64 = 7;
const ULO: i64 = 0;
const UHI: i64 = 15;

fn clamp_signed(x: i64) -> i64 {
    x.clamp(LO, HI)
}

fn wrap_signed(x: i64) -> i64 {
    ((x + 8).rem_euclid(16)) - 8
}

fn clamp_unsigned(x: i64) -> i64 {
    x.clamp(ULO, UHI)
}

// the mutant: overflow resolves to the opposite bound
fn clamp_mutant(x: i64) -> i64 {
    if x < LO {
        HI
    } else if x > HI {
        LO
    } else {
        x
    }
}

fn is_nearest(x: i64, r: i64, lo: i64, hi: i64) -> bool {
    (lo..=hi).all(|c| (r - x).abs() <= (c - x).abs())
}

struct Profile {
    retraction: bool,
    monotone: bool,
    nearest: bool,
    coherent_add: bool,
    chain_divergences: u64,
}

fn profile(rho: fn(i64) -> i64, qlo: i64, qhi: i64, wlo: i64, whi: i64) -> Profile {
    let retraction = (qlo..=qhi).all(|x| rho(x) == x);
    let mut monotone = true;
    for x in wlo..=whi {
        for y in wlo..=whi {
            if x <= y && rho(x) > rho(y) {
                monotone = false;
            }
        }
    }
    let nearest = (wlo..=whi).all(|x| is_nearest(x, rho(x), qlo, qhi));
    let mut coherent_add = true;
    for a in wlo..=whi {
        for b in wlo..=whi {
            if rho(a + b) != rho(rho(a) + rho(b)) {
                coherent_add = false;
            }
        }
    }
    // chain divergence over Q^3: eager reduction vs reduce-once
    let mut chain_divergences = 0u64;
    for a in qlo..=qhi {
        for b in qlo..=qhi {
            for c in qlo..=qhi {
                if rho(rho(a + b) + c) != rho(a + b + c) {
                    chain_divergences += 1;
                }
            }
        }
    }
    Profile {
        retraction,
        monotone,
        nearest,
        coherent_add,
        chain_divergences,
    }
}

fn report(name: &str, p: &Profile) {
    println!(
        "{:<16} retraction {}  monotone {}  nearest {}  coherent(+) {}  chain-divergent triples {}",
        name, p.retraction, p.monotone, p.nearest, p.coherent_add, p.chain_divergences
    );
}

fn main() {
    let mut ok = true;

    let cs = profile(clamp_signed, LO, HI, -64, 64);
    let ws = profile(wrap_signed, LO, HI, -64, 64);
    let cu = profile(clamp_unsigned, ULO, UHI, 0, 64);
    let cm = profile(clamp_mutant, LO, HI, -64, 64);

    report("clamp signed", &cs);
    report("wrap signed", &ws);
    report("clamp unsigned", &cu);
    report("mutant clamp", &cm);

    // cell 1: A and not C (signed clamp)
    ok &= cs.retraction && cs.monotone && cs.nearest && !cs.coherent_add;
    ok &= cs.chain_divergences > 0;

    // cell 2: C and not A (wrap)
    ok &= ws.retraction && !ws.monotone && !ws.nearest && ws.coherent_add;
    ok &= ws.chain_divergences == 0;

    // cell 3: A and C (unsigned add-only clamp, over nonnegative window)
    ok &= cu.retraction && cu.monotone && cu.nearest && cu.coherent_add;
    ok &= cu.chain_divergences == 0;

    // cell 4: neither (mutant)
    ok &= cm.retraction && !cm.monotone && !cm.nearest && !cm.coherent_add;
    ok &= cm.chain_divergences > 0;

    // coherence for multiplication: wrap holds it (ring hom), signed clamp does not
    let mut wrap_mul = true;
    let mut clamp_mul = true;
    for a in -64i64..=64 {
        for b in -64i64..=64 {
            if wrap_signed(a * b) != wrap_signed(wrap_signed(a) * wrap_signed(b)) {
                wrap_mul = false;
            }
            if clamp_signed(a * b) != clamp_signed(clamp_signed(a) * clamp_signed(b)) {
                clamp_mul = false;
            }
        }
    }
    println!(
        "wrap coherent(*): {}   clamp signed coherent(*): {}",
        wrap_mul, clamp_mul
    );
    ok &= wrap_mul && !clamp_mul;

    // the reachability tie to 42: the unsigned clamp's coherence is exactly the
    // one-reachable-bound case. Make the floor reachable by allowing signed
    // operands into the same one-sided-looking clamp and coherence must break.
    let cu_signed_window = profile(clamp_unsigned, ULO, UHI, -64, 64);
    println!(
        "clamp unsigned over a signed window: coherent(+) {} (floor now reachable)",
        cu_signed_window.coherent_add
    );
    ok &= !cu_signed_window.coherent_add;

    println!("{}", if ok { "Q1 WORKS" } else { "Q1 FAILS" });
    std::process::exit(if ok { 0 } else { 1 });
}
