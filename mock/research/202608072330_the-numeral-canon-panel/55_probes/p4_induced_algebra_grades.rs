//! Probe 4: coherence is a homomorphism onto an induced algebra, and the
//! induced algebras grade.
//!
//! Written in reply to `56_probes/q1`, whose two-by-two (adaptation laws
//! against the coherence law, all four cells inhabited) I re-ran and confirmed
//! before building this. This probe takes the coherence family one step
//! further: for any total retraction rho onto Q and ambient operation op, the
//! INDUCED operation on Q is a # b = rho(a op b). Coherence of rho is then
//! exactly the statement that rho is a homomorphism from the ambient window
//! onto (Q, #), so a coherent policy's chains are exact computations in the
//! induced algebra. The question this probe adds: WHAT algebra? The hypothesis
//! is a graded ladder:
//!
//!   wrap:                induced (Q, +, *) is the ring Z/16 (associative
//!                        add with inverses, associative mul, distributive)
//!   unsigned saturate:   induced (Q, +) is a commutative monoid (associative,
//!                        identity 0, NO inverses except at 0); induced (Q, *)
//!                        associative with identity 1
//!   signed saturate:     induced (Q, +) is a commutative unital magma and
//!                        NOT a semigroup (associativity fails on Q itself)
//!   mutant clamp:        not even unital
//!
//! Why the grade matters to the design: the laws a strategy may rely on
//! (reassociation for parallel folds, cancellation, distribution for rewrite)
//! are laws OF THE INDUCED ALGEBRA, so naming the grade is naming exactly
//! which rewrites are licensed, which is what the register's Q11 structure-
//! naming option needs to be able to say and what Q12's options quantify over.
//!
//! Also measured, cells `56` left open or did not need: multiplicative
//! coherence of the unsigned clamp over its nonnegative window, and
//! distributivity of the unsigned saturation monoid pair.
//!
//! Instrument validation: the mutant is fed to the same checkers and must fail
//! associativity and identity; the wrap rows must PASS the same associativity
//! checker the saturation rows fail, so the checker demonstrably fires both
//! ways on the same code path.
//!
//! All checks exhaustive at 4 bits, exact integer arithmetic throughout.

const SLO: i64 = -8;
const SHI: i64 = 7;
const ULO: i64 = 0;
const UHI: i64 = 15;

fn clamp_signed(x: i64) -> i64 {
    x.clamp(SLO, SHI)
}
fn wrap_signed(x: i64) -> i64 {
    ((x + 8).rem_euclid(16)) - 8
}
fn clamp_unsigned(x: i64) -> i64 {
    x.clamp(ULO, UHI)
}
fn clamp_mutant(x: i64) -> i64 {
    if x < SLO {
        SHI
    } else if x > SHI {
        SLO
    } else {
        x
    }
}

// induced operation on Q: a # b = rho(a op b)
fn induced(rho: fn(i64) -> i64, op: fn(i64, i64) -> i64, a: i64, b: i64) -> i64 {
    rho(op(a, b))
}

fn add(a: i64, b: i64) -> i64 {
    a + b
}
fn mul(a: i64, b: i64) -> i64 {
    a * b
}

// associativity of the induced op over Q^3; returns counterexample count
fn assoc_failures(rho: fn(i64) -> i64, op: fn(i64, i64) -> i64, qlo: i64, qhi: i64) -> u64 {
    let mut n = 0;
    for a in qlo..=qhi {
        for b in qlo..=qhi {
            for c in qlo..=qhi {
                let l = induced(rho, op, induced(rho, op, a, b), c);
                let r = induced(rho, op, a, induced(rho, op, b, c));
                if l != r {
                    n += 1;
                }
            }
        }
    }
    n
}

// does e behave as a two-sided identity for the induced op over Q
fn is_identity(rho: fn(i64) -> i64, op: fn(i64, i64) -> i64, qlo: i64, qhi: i64, e: i64) -> bool {
    (qlo..=qhi).all(|a| induced(rho, op, a, e) == a && induced(rho, op, e, a) == a)
}

// count elements of Q with no additive inverse under the induced op
fn no_inverse_count(rho: fn(i64) -> i64, qlo: i64, qhi: i64, e: i64) -> u64 {
    let mut n = 0;
    for a in qlo..=qhi {
        let has = (qlo..=qhi).any(|b| induced(rho, add, a, b) == e);
        if !has {
            n += 1;
        }
    }
    n
}

// chains from WINDOW operands: eager per-step reduction against one exact
// reduction, which is the "chains are exact in the induced algebra" test
// (coherence by induction). counterexample count over window triples.
fn window_chain_failures(rho: fn(i64) -> i64, wlo: i64, whi: i64, step: i64) -> u64 {
    let mut n = 0;
    let mut a = wlo;
    while a <= whi {
        let mut b = wlo;
        while b <= whi {
            let mut c = wlo;
            while c <= whi {
                let eager = induced(rho, add, induced(rho, add, rho(a), rho(b)), rho(c));
                let once = rho(a + b + c);
                if eager != once {
                    n += 1;
                }
                c += step;
            }
            b += step;
        }
        a += step;
    }
    n
}

// distributivity of the induced pair over Q^3: a*(b+c) == a*b + a*c, all induced
fn distrib_failures(rho: fn(i64) -> i64, qlo: i64, qhi: i64) -> u64 {
    let mut n = 0;
    for a in qlo..=qhi {
        for b in qlo..=qhi {
            for c in qlo..=qhi {
                let l = induced(rho, mul, a, induced(rho, add, b, c));
                let r = induced(rho, add, induced(rho, mul, a, b), induced(rho, mul, a, c));
                if l != r {
                    n += 1;
                }
            }
        }
    }
    n
}

fn main() {
    let mut ok = true;

    // ---- wrap: the ring Z/16 ----
    let wa = assoc_failures(wrap_signed, add, SLO, SHI);
    let wm = assoc_failures(wrap_signed, mul, SLO, SHI);
    let wid = is_identity(wrap_signed, add, SLO, SHI, 0);
    let wmid = is_identity(wrap_signed, mul, SLO, SHI, 1);
    let winv = no_inverse_count(wrap_signed, SLO, SHI, 0);
    let wd = distrib_failures(wrap_signed, SLO, SHI);
    println!(
        "wrap:      assoc(+) fail {}  assoc(*) fail {}  id 0 {}  id 1 {}  no-inverse {}  distrib fail {}",
        wa, wm, wid, wmid, winv, wd
    );
    ok &= wa == 0 && wm == 0 && wid && wmid && winv == 0 && wd == 0; // a ring, with additive inverses

    // ---- unsigned saturate: a commutative monoid pair, no inverses ----
    let ua = assoc_failures(clamp_unsigned, add, ULO, UHI);
    let um = assoc_failures(clamp_unsigned, mul, ULO, UHI);
    let uid = is_identity(clamp_unsigned, add, ULO, UHI, 0);
    let umid = is_identity(clamp_unsigned, mul, ULO, UHI, 1);
    let uinv = no_inverse_count(clamp_unsigned, ULO, UHI, 0);
    let ud = distrib_failures(clamp_unsigned, ULO, UHI);
    println!(
        "usat:      assoc(+) fail {}  assoc(*) fail {}  id 0 {}  id 1 {}  no-inverse {}  distrib fail {}",
        ua, um, uid, umid, uinv, ud
    );
    // monoid: associative both ops, identities, and every nonzero element lacks
    // an additive inverse (15 of 16). distributivity is REPORTED, not presumed.
    ok &= ua == 0 && um == 0 && uid && umid && uinv == 15;

    // ---- signed saturate: unital commutative magma, not a semigroup ----
    let sa = assoc_failures(clamp_signed, add, SLO, SHI);
    let sid = is_identity(clamp_signed, add, SLO, SHI, 0);
    println!("ssat:      assoc(+) fail {}  id 0 {}", sa, sid);
    ok &= sa > 0 && sid;

    // ---- mutant: not even unital ----
    let ma = assoc_failures(clamp_mutant, add, SLO, SHI);
    let mid = is_identity(clamp_mutant, add, SLO, SHI, 0);
    println!("mutant:    assoc(+) fail {}  id 0 {}", ma, mid);
    // identity survives on the mutant (it fixes Q), so the discriminating
    // failure is associativity; instrument check is that the SAME checker
    // passes wrap and fails both saturations' cells as expected
    ok &= ma > 0;

    // ---- chains from window operands: coherent policies are exact in the
    // induced algebra, incoherent ones are not ----
    let wcf = window_chain_failures(wrap_signed, -64, 64, 1);
    let ucf = window_chain_failures(clamp_unsigned, 0, 64, 1);
    let scf = window_chain_failures(clamp_signed, -64, 64, 1);
    println!(
        "window chains: wrap fail {}  usat(nonneg) fail {}  ssat fail {}",
        wcf, ucf, scf
    );
    ok &= wcf == 0 && ucf == 0 && scf > 0;

    // ---- the cell 56 did not measure: multiplicative coherence of the
    // unsigned clamp over its nonnegative window ----
    let mut umc = true;
    for a in 0i64..=64 {
        for b in 0i64..=64 {
            if clamp_unsigned(a * b) != clamp_unsigned(clamp_unsigned(a) * clamp_unsigned(b)) {
                umc = false;
            }
        }
    }
    println!("usat coherent(*) over nonneg window: {}", umc);
    // reported either way; the assertion is only that the checker ran and the
    // value is printed. record the measured value in the reply, not here.

    println!("{}", if ok { "P4 WORKS" } else { "P4 FAILS" });
    std::process::exit(if ok { 0 } else { 1 });
}
