//! Probe 2: saturation and wrapping sit on different sides of the format concept.
//!
//! Hypothesis: in the model "computed op = exact op in the ambient domain,
//! then a total adaptation map back onto the representable set", saturation IS
//! an adaptation map (a monotone, distance-minimising retraction onto the set,
//! exactly like rounding is), while wrapping is NOT one (not distance
//! minimising, not monotone). Wrapping is instead EXACT arithmetic in a
//! different ambient domain, the finite ring Z/2^N: no adaptation happens at
//! all. So "wrap" belongs to the format's domain parameter and "saturate"
//! belongs to its adaptation parameter, and a concept that files them in the
//! same slot is filing an algebra change together with an error policy.
//!
//! All checks exhaustive at 4 bits. Signed range [-8, 7] for the associativity
//! arm (unsigned add-only saturation IS associative, min distributes; the
//! signed two-sided clamp is where associativity breaks, and the probe checks
//! both statements rather than assuming either).
//!
//! Instrument validation: a mutant adaptation map (round toward zero on
//! overflow, i.e. clamp to the WRONG bound on one side) must be detected as
//! not distance minimising.

const LO: i64 = -8;
const HI: i64 = 7;

fn clamp(x: i64) -> i64 {
    if x < LO {
        LO
    } else if x > HI {
        HI
    } else {
        x
    }
}

// wrap onto [-8, 7] (two's complement wrap, i.e. Z/16 with signed residues)
fn wrap(x: i64) -> i64 {
    let m = ((x + 8).rem_euclid(16)) - 8;
    m
}

// MUTANT adaptation: overflow resolves to the opposite bound (plainly wrong,
// exists so the distance-minimisation check is shown able to fail)
fn clamp_mutant(x: i64) -> i64 {
    if x < LO {
        HI
    } else if x > HI {
        LO
    } else {
        x
    }
}

// is r the (a) distance-minimising element of [LO, HI] for x?
fn is_nearest(x: i64, r: i64) -> bool {
    (LO..=HI).all(|c| (r - x).abs() <= (c - x).abs())
}

fn main() {
    let mut ok = true;
    let wide = -64..=64i64; // ambient window, well past the representable range

    // 1. clamp is a retraction: fixes every representable value
    let retraction = (LO..=HI).all(|x| clamp(x) == x);
    println!("clamp is a retraction: {}", retraction);
    ok &= retraction;

    // 2. clamp is distance minimising everywhere in the window
    let nearest = wide.clone().all(|x| is_nearest(x, clamp(x)));
    println!("clamp is distance minimising: {}", nearest);
    ok &= nearest;

    // 3. clamp is monotone
    let mut mono = true;
    for x in wide.clone() {
        for y in wide.clone() {
            if x <= y && clamp(x) > clamp(y) {
                mono = false;
            }
        }
    }
    println!("clamp is monotone: {}", mono);
    ok &= mono;

    // 4. wrap is a retraction too (fixes representables)...
    let wr = (LO..=HI).all(|x| wrap(x) == x);
    println!("wrap is a retraction: {}", wr);
    ok &= wr;

    // ...but 5. wrap is NOT distance minimising (count witnesses)
    let wrap_far = wide.clone().filter(|&x| !is_nearest(x, wrap(x))).count();
    println!("wrap not-nearest witnesses in window: {}", wrap_far);
    ok &= wrap_far > 0;

    // and 6. wrap is NOT monotone (find a witness)
    let mut wrap_mono_witness = None;
    'outer: for x in wide.clone() {
        for y in wide.clone() {
            if x <= y && wrap(x) > wrap(y) {
                wrap_mono_witness = Some((x, y));
                break 'outer;
            }
        }
    }
    println!("wrap monotonicity violation: {:?}", wrap_mono_witness);
    ok &= wrap_mono_witness.is_some();

    // 7. wrap IS a ring homomorphism image: wrap(a + b) == wrap(wrap(a) + wrap(b))
    // exhaustively over the window, i.e. wrapped add is EXACT in Z/16
    let hom = wide
        .clone()
        .all(|a| wide.clone().all(|b| wrap(a + b) == wrap(wrap(a) + wrap(b))));
    println!("wrap add is exact in Z/16 (hom property): {}", hom);
    ok &= hom;

    // 8. wrapped add is associative exhaustively over representables
    let mut wrap_assoc = true;
    for a in LO..=HI {
        for b in LO..=HI {
            for c in LO..=HI {
                if wrap(wrap(a + b) + c) != wrap(a + wrap(b + c)) {
                    wrap_assoc = false;
                }
            }
        }
    }
    println!("wrapped add associative: {}", wrap_assoc);
    ok &= wrap_assoc;

    // 9. saturating add is NOT associative (count counterexamples, signed)
    let mut sat_assoc_fail = 0u64;
    for a in LO..=HI {
        for b in LO..=HI {
            for c in LO..=HI {
                if clamp(clamp(a + b) + c) != clamp(a + clamp(b + c)) {
                    sat_assoc_fail += 1;
                }
            }
        }
    }
    println!(
        "saturating add associativity counterexamples (signed): {}",
        sat_assoc_fail
    );
    ok &= sat_assoc_fail > 0;

    // 10. and the flip side that keeps the claim honest: UNSIGNED add-only
    // saturation IS associative (clamp at one bound composes with min), so the
    // associativity loss is a property of the two-sided signed clamp under
    // mixed-sign inputs, not of saturation per se.
    let ulo = 0i64;
    let uhi = 15i64;
    let uclamp = |x: i64| {
        if x > uhi {
            uhi
        } else if x < ulo {
            ulo
        } else {
            x
        }
    };
    let mut usat_assoc = true;
    for a in ulo..=uhi {
        for b in ulo..=uhi {
            for c in ulo..=uhi {
                if uclamp(uclamp(a + b) + c) != uclamp(a + uclamp(b + c)) {
                    usat_assoc = false;
                }
            }
        }
    }
    println!("unsigned add-only saturation associative: {}", usat_assoc);
    ok &= usat_assoc;

    // instrument validation: the mutant clamp must FAIL distance minimisation
    let mutant_detected = wide.clone().any(|x| !is_nearest(x, clamp_mutant(x)));
    println!(
        "mutant adaptation detected as non-minimising: {}",
        mutant_detected
    );
    ok &= mutant_detected;

    println!("{}", if ok { "P2 WORKS" } else { "P2 FAILS" });
    std::process::exit(if ok { 0 } else { 1 });
}
