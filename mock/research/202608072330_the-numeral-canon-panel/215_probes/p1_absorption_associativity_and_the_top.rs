// Probe 1 for seat 215. Exhaustive at eight bits.
//
// What it establishes, and the case each section must be able to fail:
//
//   A. Associativity and absorption, over every triple, for every configuration
//      in {add, mul} x {wrap, saturate} x {unsigned, signed} x F in {0,2,4} x
//      {truncate, nearest}. The controls: signed saturating multiply at F = 0
//      MUST report non-associative, and wrapping multiply at F = 0 MUST report
//      associative. If either flips, the instrument is broken and every number
//      below it is void, so the program says so and exits non-zero.
//
//   B. Whether absorption is necessary as well as sufficient. Sufficiency is a
//      theorem and measuring it proves nothing. What is worth measuring is the
//      converse: a configuration where the realised operation is associative
//      while absorption fails would show absorption is strictly stronger than
//      associativity, and therefore not "the criterion" but one sufficient
//      condition among others. That cell is what the sweep is looking for.
//
//   C. Absorbing elements, by exhaustive search rather than by argument. The
//      control: wrapping addition MUST report none, at both signednesses, at
//      every width, because addition mod 2^n is a group and cancellation
//      forbids a non-trivial absorber. A search that finds one has a bug.
//
//   D. The min-plus semiring axioms, each checked separately so a failure names
//      which axiom died rather than reporting a bare verdict.
//
//   E. Two composite-region witnesses, in both directions: a composite that
//      fails where every part holds, and a composite that holds where a part
//      fails. One direction alone does not establish non-inheritance.
//
//   F. A region where the width grade refuses a law that the interval grade
//      admits, with the law then confirmed by exhaustive check over that
//      interval rather than by the interval argument.
//
// Exact arithmetic throughout: a value is a numerator over 2^scale in i64, and
// nothing is ever evaluated in floating point. Magnitudes reach 2^24 at the
// widest, so i64 is ample and no intermediate here can itself overflow.
//
// Build: rustc -O p1_absorption_associativity_and_the_top.rs -o p1

use std::collections::BTreeSet;

const W: u32 = 8;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Policy {
    Wrap,
    Sat,
}
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Sign {
    U,
    I,
}
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Round {
    Trunc,
    Near,
}
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Op {
    Add,
    Mul,
}

#[derive(Clone, Copy, Debug)]
struct Cfg {
    op: Op,
    pol: Policy,
    sign: Sign,
    f: u32,
    rnd: Round,
}

fn lo(s: Sign) -> i64 {
    match s {
        Sign::U => 0,
        Sign::I => -(1i64 << (W - 1)),
    }
}
fn hi(s: Sign) -> i64 {
    match s {
        Sign::U => (1i64 << W) - 1,
        Sign::I => (1i64 << (W - 1)) - 1,
    }
}

/// Arithmetic shift right with the chosen rounding. `Trunc` is floor, which is
/// what a plain `>>` on a signed integer does; `Near` is round-half-to-even.
fn shr_round(num: i64, k: u32, rnd: Round) -> i64 {
    if k == 0 {
        return num;
    }
    match rnd {
        Round::Trunc => num >> k,
        Round::Near => {
            let q = num >> k;
            let rem = num - (q << k);
            let half = 1i64 << (k - 1);
            if rem > half {
                q + 1
            } else if rem < half {
                q
            } else if q & 1 == 1 {
                q + 1
            } else {
                q
            }
        }
    }
}

/// The realisation map. Takes an exact value as (numerator, scale) and returns
/// a representable raw: rescale to `f` fractional bits, then apply the range
/// policy. Both halves of `r` live here, which is what lets absorption see them
/// the same way.
fn rho(num: i64, scale: u32, c: Cfg) -> i64 {
    let raw = if scale >= c.f {
        shr_round(num, scale - c.f, c.rnd)
    } else {
        num << (c.f - scale)
    };
    match c.pol {
        Policy::Sat => raw.clamp(lo(c.sign), hi(c.sign)),
        Policy::Wrap => {
            let m = 1i64 << W;
            let r = raw.rem_euclid(m);
            match c.sign {
                Sign::U => r,
                Sign::I => {
                    if r >= (1i64 << (W - 1)) {
                        r - m
                    } else {
                        r
                    }
                }
            }
        }
    }
}

/// The exact operation on two raws, both at scale `f`.
fn exact2(op: Op, a: i64, b: i64, f: u32) -> (i64, u32) {
    match op {
        Op::Add => (a + b, f),
        Op::Mul => (a * b, 2 * f),
    }
}

/// The exact operation on an already-exact value and a raw.
fn exact_mixed(op: Op, n: i64, s: u32, c: i64, f: u32) -> (i64, u32) {
    match op {
        Op::Add => {
            let m = s.max(f);
            ((n << (m - s)) + (c << (m - f)), m)
        }
        Op::Mul => (n * c, s + f),
    }
}

fn apply(c: Cfg, a: i64, b: i64) -> i64 {
    let (n, s) = exact2(c.op, a, b, c.f);
    rho(n, s, c)
}

fn reprs(sign: Sign) -> Vec<i64> {
    (lo(sign)..=hi(sign)).collect()
}

#[derive(Debug, Default)]
struct Counts {
    assoc_bad: u64,
    left_absorb_bad: u64,
    right_absorb_bad: u64,
    first_assoc: Option<(i64, i64, i64, i64, i64)>,
    first_absorb: Option<(i64, i64, i64, i64, i64)>,
}

/// One configuration, every triple.
///
/// `E` is the fully exact result rounded once at the end. Left absorption is
/// `L == E`, right absorption is `R == E`, associativity is `L == R`.
fn sweep(c: Cfg) -> Counts {
    let vals = reprs(c.sign);
    let mut k = Counts::default();
    for &a in &vals {
        for &b in &vals {
            let (nab, sab) = exact2(c.op, a, b, c.f);
            let lab = rho(nab, sab, c);
            for &cc in &vals {
                // fully exact, rounded once
                let (ne, se) = exact_mixed(c.op, nab, sab, cc, c.f);
                let e = rho(ne, se, c);

                let l = apply(c, lab, cc);

                let (nbc, sbc) = exact2(c.op, b, cc, c.f);
                let rbc = rho(nbc, sbc, c);
                let r = apply(c, a, rbc);

                if l != r {
                    k.assoc_bad += 1;
                    if k.first_assoc.is_none() {
                        k.first_assoc = Some((a, b, cc, l, r));
                    }
                }
                if l != e {
                    k.left_absorb_bad += 1;
                    if k.first_absorb.is_none() {
                        k.first_absorb = Some((a, b, cc, l, e));
                    }
                }
                if r != e {
                    k.right_absorb_bad += 1;
                }
            }
        }
    }
    k
}

fn name(c: Cfg) -> String {
    format!("{:?}/{:?}/{:?}/F={}/{:?}", c.op, c.pol, c.sign, c.f, c.rnd)
}

fn section_a_and_b() -> bool {
    println!("== A/B. associativity and absorption, exhaustive over all 2^24 triples per row ==");
    println!();
    println!(
        "{:<34} {:>12} {:>12} {:>12}  {}",
        "configuration", "assoc bad", "L-absorb bad", "R-absorb bad", "verdict"
    );

    let mut ok = true;
    let mut accidental: Vec<String> = Vec::new();
    let mut control_signed_sat_mul = None;
    let mut control_wrap_mul = None;

    for op in [Op::Add, Op::Mul] {
        for pol in [Policy::Wrap, Policy::Sat] {
            for sign in [Sign::U, Sign::I] {
                for (f, rnd) in [
                    (0u32, Round::Trunc),
                    (2, Round::Trunc),
                    (2, Round::Near),
                    (4, Round::Trunc),
                    (4, Round::Near),
                ] {
                    let c = Cfg {
                        op,
                        pol,
                        sign,
                        f,
                        rnd,
                    };
                    let k = sweep(c);
                    let assoc = k.assoc_bad == 0;
                    let absorb = k.left_absorb_bad == 0 && k.right_absorb_bad == 0;
                    let verdict = match (assoc, absorb) {
                        (true, true) => "associative, absorption holds",
                        (true, false) => "ACCIDENTAL: associative, absorption fails",
                        (false, true) => "IMPOSSIBLE: absorption holds, not associative",
                        (false, false) => "not associative, absorption fails",
                    };
                    if assoc && !absorb {
                        accidental.push(name(c));
                    }
                    if !assoc && absorb {
                        println!(
                            "  !! the sufficiency theorem is contradicted by {}",
                            name(c)
                        );
                        ok = false;
                    }
                    println!(
                        "{:<34} {:>12} {:>12} {:>12}  {}",
                        name(c),
                        k.assoc_bad,
                        k.left_absorb_bad,
                        k.right_absorb_bad,
                        verdict
                    );
                    if let Some(w) = k.first_assoc {
                        if f == 0 {
                            println!(
                                "        first associativity witness: a={} b={} c={} -> L={} R={}",
                                w.0, w.1, w.2, w.3, w.4
                            );
                        }
                    }
                    if op == Op::Mul && pol == Policy::Sat && sign == Sign::I && f == 0 {
                        control_signed_sat_mul = Some(assoc);
                    }
                    if op == Op::Mul && pol == Policy::Wrap && sign == Sign::I && f == 0 {
                        control_wrap_mul = Some(assoc);
                    }
                }
            }
        }
    }

    println!();
    println!("-- the controls, which decide whether anything above counts --");
    match control_signed_sat_mul {
        Some(false) => {
            println!("  PASS  signed saturating multiply at F=0 reports non-associative")
        }
        other => {
            println!("  FAIL  signed saturating multiply at F=0 reported {other:?}, expected non-associative");
            ok = false;
        }
    }
    match control_wrap_mul {
        Some(true) => println!("  PASS  signed wrapping multiply at F=0 reports associative"),
        other => {
            println!(
                "  FAIL  signed wrapping multiply at F=0 reported {other:?}, expected associative"
            );
            ok = false;
        }
    }
    println!();
    if accidental.is_empty() {
        println!(
            "  no configuration is associative while absorption fails, over the swept space.\n  \
             so absorption is not observed to be strictly stronger here. it remains PROVEN\n  \
             sufficient and OBSERVED necessary, and those are different standings."
        );
    } else {
        println!("  associative while absorption fails, so absorption is strictly stronger:");
        for a in &accidental {
            println!("    {a}");
        }
    }
    ok
}

fn section_c() -> bool {
    println!();
    println!("== C. absorbing elements, by exhaustive search ==");
    println!();
    let mut ok = true;
    for op in [Op::Add, Op::Mul] {
        for pol in [Policy::Wrap, Policy::Sat] {
            for sign in [Sign::U, Sign::I] {
                let c = Cfg {
                    op,
                    pol,
                    sign,
                    f: 0,
                    rnd: Round::Trunc,
                };
                let vals = reprs(sign);
                let mut absorbers = Vec::new();
                for &t in &vals {
                    if vals.iter().all(|&x| apply(c, t, x) == t) {
                        absorbers.push(t);
                    }
                }
                println!(
                    "  {:<26} absorbing elements: {:?}",
                    format!("{:?}/{:?}/{:?}", op, pol, sign),
                    absorbers
                );
                if op == Op::Add && pol == Policy::Wrap && !absorbers.is_empty() {
                    println!(
                        "  FAIL  wrapping addition reported an absorber, which group cancellation forbids"
                    );
                    ok = false;
                }
            }
        }
    }
    println!();
    println!("  the control: wrapping addition must report none at both signednesses.");
    println!("  note that wrapping MULTIPLY absorbs at 0, which is the ring annihilator and");
    println!("  is not the element a min-plus fold needs. min-plus takes + as its product, so");
    println!("  the annihilator it needs is one for +, and that is what wrapping has none of.");
    ok
}

fn section_d() {
    println!();
    println!("== D. the min-plus semiring axioms, one line per axiom ==");
    println!();
    for pol in [Policy::Sat, Policy::Wrap] {
        for sign in [Sign::U, Sign::I] {
            let c = Cfg {
                op: Op::Add,
                pol,
                sign,
                f: 0,
                rnd: Round::Trunc,
            };
            let vals = reprs(sign);
            let top = hi(sign);
            let zero = 0i64;

            let otimes = |a: i64, b: i64| apply(c, a, b);
            let oplus = |a: i64, b: i64| a.min(b);

            let mut o_assoc = true;
            let mut o_comm = true;
            let mut distrib = true;
            for &a in &vals {
                for &b in &vals {
                    if otimes(a, b) != otimes(b, a) {
                        o_comm = false;
                    }
                    for &x in &vals {
                        if otimes(otimes(a, b), x) != otimes(a, otimes(b, x)) {
                            o_assoc = false;
                        }
                        if otimes(oplus(a, b), x) != oplus(otimes(a, x), otimes(b, x)) {
                            distrib = false;
                        }
                    }
                }
            }
            let o_ident = vals.iter().all(|&x| otimes(zero, x) == x);
            let p_ident = vals.iter().all(|&x| oplus(top, x) == x);
            let annih = vals.iter().all(|&x| otimes(top, x) == top);

            println!(
                "  {:?}/{:?}  otimes assoc {}  otimes comm {}  otimes ident(0) {}  \
                 oplus ident(TOP) {}  TOP annihilates {}  distributes {}",
                pol,
                sign,
                yn(o_assoc),
                yn(o_comm),
                yn(o_ident),
                yn(p_ident),
                yn(annih),
                yn(distrib)
            );
            if o_assoc && o_comm && o_ident && p_ident && annih && distrib {
                println!("        -> a complete min-plus semiring, with TOP = {top}");
            } else {
                let mut missing = Vec::new();
                if !o_assoc {
                    missing.push("otimes associativity");
                }
                if !o_comm {
                    missing.push("otimes commutativity");
                }
                if !o_ident {
                    missing.push("otimes identity");
                }
                if !p_ident {
                    missing.push("oplus identity");
                }
                if !annih {
                    missing.push("annihilating top");
                }
                if !distrib {
                    missing.push("distributivity");
                }
                println!(
                    "        -> not a min-plus carrier. missing: {}",
                    missing.join(", ")
                );
            }
        }
    }
    println!();
    println!("  TOP is a reachable value rather than a fresh point, so 'no path' and 'a path");
    println!("  costing TOP' are the same bit pattern. the arm carries `all reachable costs <");
    println!("  TOP` and is unsound without it. that is a caveat on the carrier, not on the");
    println!("  axioms, every one of which is checked above over the whole value set.");
}

fn yn(b: bool) -> &'static str {
    if b {
        "yes"
    } else {
        "NO "
    }
}

fn section_e() -> bool {
    println!();
    println!("== E. a composite's region, in both directions ==");
    println!();
    let mut ok = true;

    // Direction one: the composite fails where every part holds.
    //
    // Parts are individual multiplies under signed saturating F = 0, each
    // carrying the region "this site does not saturate". Both associations use
    // the same three leaves and the same two multiplies, and each multiply, at
    // its own site, is asked only whether IT saturated.
    let c = Cfg {
        op: Op::Mul,
        pol: Policy::Sat,
        sign: Sign::I,
        f: 0,
        rnd: Round::Trunc,
    };
    let mut found_one = None;
    for a in lo(Sign::I)..=hi(Sign::I) {
        for b in lo(Sign::I)..=hi(Sign::I) {
            for cc in lo(Sign::I)..=hi(Sign::I) {
                let inner_r = b * cc; // exact
                let r_site_ok = inner_r >= lo(Sign::I) && inner_r <= hi(Sign::I);
                let outer_r = a * inner_r;
                let r_outer_ok = outer_r >= lo(Sign::I) && outer_r <= hi(Sign::I);
                let inner_l = a * b;
                let l_site_ok = inner_l >= lo(Sign::I) && inner_l <= hi(Sign::I);
                // every part of the RIGHT association is within range
                if r_site_ok && r_outer_ok && !l_site_ok {
                    let l = apply(c, apply(c, a, b), cc);
                    let r = apply(c, a, apply(c, b, cc));
                    if l != r {
                        found_one = Some((a, b, cc, l, r));
                        break;
                    }
                }
            }
            if found_one.is_some() {
                break;
            }
        }
        if found_one.is_some() {
            break;
        }
    }
    match found_one {
        Some((a, b, cc, l, r)) => println!(
            "  composite fails where every part of one association holds:\n    \
             signed saturating multiply, F=0, a={a} b={b} c={cc}\n    \
             every site of a*(b*c) is in range, and (a*b)*c = {l} against a*(b*c) = {r}"
        ),
        None => {
            println!("  FAIL  no witness found for direction one");
            ok = false;
        }
    }

    // Direction two: the composite holds where a part fails.
    //
    // Wrapping addition. The part `a + b` escapes the declared range for many
    // pairs, so a per-part region "does not escape" excludes them. The composite
    // `(a + b) - b` is nevertheless exactly `a` for every pair, because the wrap
    // is a group quotient. So the composite's region is strictly LARGER than the
    // intersection of its parts', which kills inheritance in the weak direction
    // as well as the strong one.
    let cw = Cfg {
        op: Op::Add,
        pol: Policy::Wrap,
        sign: Sign::U,
        f: 0,
        rnd: Round::Trunc,
    };
    let mut escaping = 0u64;
    let mut composite_bad = 0u64;
    for a in 0..=255i64 {
        for b in 0..=255i64 {
            if a + b > 255 {
                escaping += 1;
            }
            let s = apply(cw, a, b);
            // subtract b, still wrapping
            let back = rho(s - b, 0, cw);
            if back != a {
                composite_bad += 1;
            }
        }
    }
    println!();
    println!(
        "  composite holds where a part fails:\n    \
         unsigned wrapping, F=0. pairs where a+b escapes the declared range: {escaping} of 65536.\n    \
         pairs where (a+b)-b != a: {composite_bad}."
    );
    if composite_bad != 0 || escaping == 0 {
        println!("  FAIL  expected many escaping parts and zero composite failures");
        ok = false;
    }
    ok
}

fn section_f() -> bool {
    println!();
    println!("== F. the width grade against the interval grade ==");
    println!();
    // Signed saturating multiply, F = 0, is non-associative over the full
    // declared range, which section A establishes. The width grade for a triple
    // product of 8-bit operands is 24 bits, so a containment-dependent law is
    // refused outright at width 8.
    //
    // The interval grade is finer. Restrict every operand to [-5, 5]. Then every
    // partial and total product has magnitude at most 125, which is inside
    // [-128, 127], so nothing saturates and the law holds. Checked here rather
    // than argued.
    let c = Cfg {
        op: Op::Mul,
        pol: Policy::Sat,
        sign: Sign::I,
        f: 0,
        rnd: Round::Trunc,
    };
    let mut bad_full = 0u64;
    for a in -128..=127i64 {
        for b in -128..=127i64 {
            for cc in -128..=127i64 {
                if apply(c, apply(c, a, b), cc) != apply(c, a, apply(c, b, cc)) {
                    bad_full += 1;
                }
            }
        }
    }
    let mut bad_interval = 0u64;
    let mut n_interval = 0u64;
    for a in -5..=5i64 {
        for b in -5..=5i64 {
            for cc in -5..=5i64 {
                n_interval += 1;
                if apply(c, apply(c, a, b), cc) != apply(c, a, apply(c, b, cc)) {
                    bad_interval += 1;
                }
            }
        }
    }
    println!(
        "  signed saturating multiply, F=0.\n    \
         width grade, operands in [-128,127]: {bad_full} of 16777216 triples non-associative.\n    \
         interval grade, operands in [-5,5]:  {bad_interval} of {n_interval} triples non-associative."
    );
    // The largest symmetric interval that still carries the law, found rather
    // than assumed, so the boundary is measured instead of guessed.
    let mut largest = 0i64;
    for k in 1..=127i64 {
        let mut bad = 0u64;
        'outer: for a in -k..=k {
            for b in -k..=k {
                for cc in -k..=k {
                    if apply(c, apply(c, a, b), cc) != apply(c, a, apply(c, b, cc)) {
                        bad += 1;
                        break 'outer;
                    }
                }
            }
        }
        if bad == 0 {
            largest = k;
        } else {
            break;
        }
    }
    println!("    the largest symmetric interval [-k,k] carrying associativity is k = {largest}.");
    println!();
    println!(
        "  k = 11 rather than k = 5, and the gap is the finding. I predicted 5, reasoning that\n  \
         the TRIPLE product must stay in range: 5^3 = 125 fits [-128,127] and 6^3 = 216 does not.\n  \
         that is wrong. 11^2 = 121 fits and 12^2 = 144 does not, so the boundary is set by the\n  \
         PAIRWISE product. containment is needed at the nested position and nowhere else: once\n  \
         both inner products are exact, the two associations hand the outer clamp the SAME exact\n  \
         value, so the outer clamp cannot separate them however hard it saturates."
    );

    // So the honest predicate is about the nesting rather than about the result.
    // Sufficiency is the argument above. Necessity is a measurement: count the
    // triples where an inner product does leave the range and the two
    // associations agree anyway. Those are exactly the cases a gate on pairwise
    // containment refuses while the law in fact holds, which is the predicate's
    // own laxness, measured instead of asserted.
    let mut inner_escapes = 0u64;
    let mut escapes_but_agrees = 0u64;
    for a in -128..=127i64 {
        for b in -128..=127i64 {
            for cc in -128..=127i64 {
                let ab = a * b;
                let bc = b * cc;
                let inner_ok = ab >= lo(Sign::I)
                    && ab <= hi(Sign::I)
                    && bc >= lo(Sign::I)
                    && bc <= hi(Sign::I);
                if !inner_ok {
                    inner_escapes += 1;
                    if apply(c, apply(c, a, b), cc) == apply(c, a, apply(c, b, cc)) {
                        escapes_but_agrees += 1;
                    }
                }
            }
        }
    }
    println!();
    println!(
        "  the pairwise-containment predicate, over all 16777216 triples:\n    \
         triples where an inner product leaves the range: {inner_escapes}\n    \
         of those, triples where the two associations agree anyway: {escapes_but_agrees}\n    \
         so the predicate is SUFFICIENT and not necessary. it refuses {escapes_but_agrees} triples\n    \
         on which the law does hold, and that is the price of a const-decidable gate stated as a\n    \
         number rather than as an adjective."
    );

    if bad_full == 0 || bad_interval != 0 || largest != 11 {
        println!(
            "  FAIL  expected the full range to break, [-5,5] to hold, and the boundary at k = 11"
        );
        return false;
    }
    if inner_escapes == 0 {
        println!("  FAIL  the pairwise predicate never fired, so its laxness went unmeasured");
        return false;
    }
    true
}

fn main() {
    println!("seat 215, probe 1. W = {W}, exhaustive over the declared range.");
    println!();
    let a = section_a_and_b();
    let c = section_c();
    let e = section_e();
    let f = section_f();
    section_d();
    println!();
    let all = a && c && e && f;
    println!(
        "== every control: {} ==",
        if all {
            "PASSED"
        } else {
            "FAILED, numbers above are void"
        }
    );
    // Keep the set type used so the import is not dead weight if trimmed later.
    let _: BTreeSet<i64> = BTreeSet::new();
    if !all {
        std::process::exit(1);
    }
}
