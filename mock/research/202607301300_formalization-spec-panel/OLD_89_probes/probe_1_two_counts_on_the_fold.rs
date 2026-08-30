//! Probe 1: the two event-counting disciplines, separated on the fold family, and what
//! each does to file 37's ratified view table.
//!
//! WHAT THIS MODEL SEPARATES (per the separation requirement, `86b:8-10`). It separates
//! reading A (an event per quantiser APPLICATION, a static count of sites) from reading B
//! (an event per value actually MOVED, a dynamic count of firings). The distinction is
//! nonvacuous here by construction: every composition below has three addition sites in
//! every grouping, and every composition below has inputs on which zero, one or two of
//! those sites actually move the value. A model in which the quantiser fires at every site
//! could not separate them, and neither could a model in which it never fires.
//!
//! It does NOT separate: the cause component (identical under both readings, since a
//! refusal is not a quantisation event under either), the value component, or anything
//! about the multiset-versus-set question for a shared subterm (no term here shares one).
//!
//! Model: a three-bit numeral, quantum 1, so the value set is the integers. Signed domain
//! is -4..=3, unsigned is 0..=7. Four-element fold, all five groupings, every input tuple
//! (4096 signed, 4096 unsigned), plus the three-element fold (two groupings, 512 tuples)
//! that carries file 37's own witness. The accumulator is the numeral itself, so the fold
//! is BELOW interior safety and a quantiser sits at every site: this is file 37's probe 1
//! shape, rebuilt independently rather than reused.
//!
//! CLAIM A (calibration against a ratified measurement): under reading B this model
//!   reproduces file 37's own finest-view table (`37:171-179`, carried into `40` and every
//!   consolidation since) for all five compositions it lists. If it did not, nothing else
//!   here would be evidence.
//! CLAIM B: under reading A the event component of the fold-associativity law is preserved
//!   by EVERY composition, including the two that fail it under reading B. The reason is
//!   structural rather than a property of this model: every grouping of an n-element fold
//!   has exactly n-1 sites, so a site count is grouping-invariant by construction.
//! CLAIM C: the four finest views file 37 measured form an ANTICHAIN-containing set under
//!   reading B (two incomparable points) and a CHAIN under reading A. The design's ratified
//!   reason for preferring a computed finest view over a three-name ladder (`37:62-69`,
//!   `39b`) is that the lattice is not a chain; reading A removes that.
//! CLAIM D: the specific witness file 37 names, (-4, -3, 3) on the signed numeral, has
//!   fired-reduction counts 2 and 0 across its two groupings and site counts 2 and 2.
//!
//! Build: rustc --edition 2021 -O probe_1_two_counts_on_the_fold.rs --out-dir out
//! Outcome: WORKS (all four claims assert exhaustively; no sampling anywhere).
//! rustc 1.98.0-nightly (57d06900f 2026-05-27), aarch64-apple-darwin.

/// What a range end does when the exact partial sum leaves the numeral's value set.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Res {
    ReduceModulo,
    Clamp,
    Refuse,
}

/// A term's meaning: defined?, value, events under reading A (sites), events under
/// reading B (firings), refusal causes.
#[derive(Clone, Copy, Debug)]
struct G {
    defined: bool,
    val: i32,
    ev_a: u32,
    ev_b: u32,
    causes: u32,
}

const fn unit(v: i32) -> G {
    G {
        defined: true,
        val: v,
        ev_a: 0,
        ev_b: 0,
        causes: 0,
    }
}

/// One addition site: exact sum, then classify against [lo, hi], then resolve.
///
/// Reading A adds one event for the site unconditionally, because a quantiser is present
/// at this site as a matter of the operation's type. Reading B adds one event only when
/// the resolution actually moved the delivered value off the exact sum.
const fn add(a: G, b: G, lo: i32, hi: i32, top: Res, bot: Res) -> G {
    if !a.defined || !b.defined {
        return G {
            defined: false,
            val: 0,
            ev_a: a.ev_a + b.ev_a + 1,
            ev_b: a.ev_b + b.ev_b,
            causes: a.causes | b.causes,
        };
    }
    let exact = a.val + b.val;
    let modulus = hi - lo + 1;
    let ev_a = a.ev_a + b.ev_a + 1;
    let base_b = a.ev_b + b.ev_b;
    let causes = a.causes | b.causes;

    if exact >= lo && exact <= hi {
        // In range. No resolution runs, nothing moves, no event under either reading
        // except reading A's unconditional site charge.
        return G {
            defined: true,
            val: exact,
            ev_a,
            ev_b: base_b,
            causes,
        };
    }
    let r = if exact > hi { top } else { bot };
    match r {
        Res::ReduceModulo => {
            let mut v = exact;
            while v > hi {
                v -= modulus;
            }
            while v < lo {
                v += modulus;
            }
            G {
                defined: true,
                val: v,
                ev_a,
                ev_b: base_b + 1,
                causes,
            }
        }
        Res::Clamp => {
            let v = if exact > hi { hi } else { lo };
            G {
                defined: true,
                val: v,
                ev_a,
                ev_b: base_b + 1,
                causes,
            }
        }
        Res::Refuse => G {
            defined: false,
            val: 0,
            ev_a,
            ev_b: base_b,
            causes: causes | 1,
        },
    }
}

// The five groupings of a four-element fold.
const fn g4(which: usize, a: i32, b: i32, c: i32, d: i32, lo: i32, hi: i32, t: Res, bo: Res) -> G {
    let (a, b, c, d) = (unit(a), unit(b), unit(c), unit(d));
    match which {
        0 => add(
            add(add(a, b, lo, hi, t, bo), c, lo, hi, t, bo),
            d,
            lo,
            hi,
            t,
            bo,
        ),
        1 => add(
            add(a, add(b, c, lo, hi, t, bo), lo, hi, t, bo),
            d,
            lo,
            hi,
            t,
            bo,
        ),
        2 => add(
            add(a, b, lo, hi, t, bo),
            add(c, d, lo, hi, t, bo),
            lo,
            hi,
            t,
            bo,
        ),
        3 => add(
            a,
            add(add(b, c, lo, hi, t, bo), d, lo, hi, t, bo),
            lo,
            hi,
            t,
            bo,
        ),
        _ => add(
            a,
            add(b, add(c, d, lo, hi, t, bo), lo, hi, t, bo),
            lo,
            hi,
            t,
            bo,
        ),
    }
}

// The two groupings of a three-element fold.
const fn g3(which: usize, a: i32, b: i32, c: i32, lo: i32, hi: i32, t: Res, bo: Res) -> G {
    let (a, b, c) = (unit(a), unit(b), unit(c));
    if which == 0 {
        add(add(a, b, lo, hi, t, bo), c, lo, hi, t, bo)
    } else {
        add(a, add(b, c, lo, hi, t, bo), lo, hi, t, bo)
    }
}

/// A view's detail level on one generator class.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
enum Lvl {
    Ignore = 0,
    Presence = 1,
    Exact = 2,
}

/// Does the law hold at (cause level, event level), with events counted by `count`?
/// `count` picks reading A or reading B out of the pair every term already carries.
fn holds_at(terms: &[G], cl: Lvl, el: Lvl, use_a: bool) -> bool {
    // Value clause: agree wherever both defined.
    for i in 0..terms.len() {
        for j in 0..terms.len() {
            if terms[i].defined && terms[j].defined && terms[i].val != terms[j].val {
                return false;
            }
        }
    }
    // Cause clause.
    if cl != Lvl::Ignore {
        for i in 0..terms.len() {
            let (x, y) = (terms[0].causes, terms[i].causes);
            let eq = match cl {
                Lvl::Presence => (x != 0) == (y != 0),
                _ => x == y,
            };
            if !eq {
                return false;
            }
            // Definedness rides with the cause class: a refusal is a cause.
            if terms[0].defined != terms[i].defined {
                return false;
            }
        }
    }
    // Event clause.
    if el != Lvl::Ignore {
        for i in 0..terms.len() {
            let x = if use_a { terms[0].ev_a } else { terms[0].ev_b };
            let y = if use_a { terms[i].ev_a } else { terms[i].ev_b };
            let eq = match el {
                Lvl::Presence => (x != 0) == (y != 0),
                _ => x == y,
            };
            if !eq {
                return false;
            }
        }
    }
    true
}

/// The finest view under which the law holds over every input, or None if no view does.
/// Computed as the join of the holding views, which is well defined because the holding
/// set is downward closed and closed under join (`37:126-143`).
fn finest(inputs: &dyn Fn(usize, &mut Vec<G>), n_inputs: usize, use_a: bool) -> Option<(Lvl, Lvl)> {
    let levels = [Lvl::Ignore, Lvl::Presence, Lvl::Exact];
    let mut best: Option<(Lvl, Lvl)> = None;
    for &cl in levels.iter() {
        for &el in levels.iter() {
            let mut ok = true;
            let mut buf: Vec<G> = Vec::new();
            for k in 0..n_inputs {
                buf.clear();
                inputs(k, &mut buf);
                if !holds_at(&buf, cl, el, use_a) {
                    ok = false;
                    break;
                }
            }
            if ok {
                best = Some(match best {
                    None => (cl, el),
                    Some((c0, e0)) => {
                        (if cl > c0 { cl } else { c0 }, if el > e0 { el } else { e0 })
                    }
                });
            }
        }
    }
    // The join of holding views must itself hold; assert it rather than assume it.
    if let Some((cl, el)) = best {
        let mut buf: Vec<G> = Vec::new();
        for k in 0..n_inputs {
            buf.clear();
            inputs(k, &mut buf);
            assert!(holds_at(&buf, cl, el, use_a), "join closure failed: the design's own uniqueness argument (37:136-143) does not hold in this model");
        }
    }
    best
}

struct Comp {
    name: &'static str,
    lo: i32,
    hi: i32,
    top: Res,
    bot: Res,
}

fn main() {
    let comps = [
        Comp {
            name: "Hot, unsigned wrapping",
            lo: 0,
            hi: 7,
            top: Res::ReduceModulo,
            bot: Res::ReduceModulo,
        },
        Comp {
            name: "Hot, signed wrapping",
            lo: -4,
            hi: 3,
            top: Res::ReduceModulo,
            bot: Res::ReduceModulo,
        },
        Comp {
            name: "Warm / Cold, saturating",
            lo: -4,
            hi: 3,
            top: Res::Clamp,
            bot: Res::Clamp,
        },
        Comp {
            name: "Precise, refusing",
            lo: -4,
            hi: 3,
            top: Res::Refuse,
            bot: Res::Refuse,
        },
        Comp {
            name: "Refuse at one end, reduce at the other",
            lo: -4,
            hi: 3,
            top: Res::Refuse,
            bot: Res::ReduceModulo,
        },
    ];

    println!("composition                              | finest view, reading B | finest view, reading A");
    println!("-----------------------------------------|------------------------|-----------------------");

    let mut b_views: Vec<Option<(Lvl, Lvl)>> = Vec::new();
    let mut a_views: Vec<Option<(Lvl, Lvl)>> = Vec::new();

    for c in comps.iter() {
        let span = (c.hi - c.lo + 1) as usize;
        let n = span * span * span * span;
        let build = |k: usize, out: &mut Vec<G>| {
            let a = c.lo + (k % span) as i32;
            let b = c.lo + ((k / span) % span) as i32;
            let cc = c.lo + ((k / (span * span)) % span) as i32;
            let d = c.lo + ((k / (span * span * span)) % span) as i32;
            for w in 0..5 {
                out.push(g4(w, a, b, cc, d, c.lo, c.hi, c.top, c.bot));
            }
        };
        let vb = finest(&build, n, false);
        let va = finest(&build, n, true);
        println!("{:40} | {:22} | {:?}", c.name, format!("{:?}", vb), va);
        b_views.push(vb);
        a_views.push(va);
    }

    // CLAIM A. Calibration: reading B reproduces file 37's ratified table row for row.
    // 37:171-179: Precise interior-safe (Exact,Exact) is not in this model (this fold is
    // below interior safety); the five rows below ARE the five this model covers.
    assert_eq!(
        b_views[0],
        Some((Lvl::Exact, Lvl::Exact)),
        "Hot unsigned should be graded (37:174)"
    );
    assert_eq!(
        b_views[1],
        Some((Lvl::Exact, Lvl::Ignore)),
        "Hot signed should be Kleene and no more (37:175)"
    );
    assert_eq!(
        b_views[2], None,
        "Warm/Cold saturating: the law is false (37:178)"
    );
    assert_eq!(
        b_views[3],
        Some((Lvl::Ignore, Lvl::Exact)),
        "Precise below interior safety: the point with no name (37:176)"
    );
    assert_eq!(
        b_views[4],
        Some((Lvl::Ignore, Lvl::Ignore)),
        "refuse one end, reduce the other: the weak equation (37:177)"
    );
    println!("\nCLAIM A holds: reading B reproduces file 37's table, all five rows.");

    // CLAIM B. Under reading A the event component is preserved by every composition
    // whose law holds at all, INCLUDING the two that fail it under reading B.
    for (i, c) in comps.iter().enumerate() {
        if let Some((_, el)) = a_views[i] {
            assert_eq!(
                el,
                Lvl::Exact,
                "under reading A the event level should be Exact for {}",
                c.name
            );
        }
    }
    // And the two rows that actually move:
    assert_eq!(
        a_views[1],
        Some((Lvl::Exact, Lvl::Exact)),
        "Hot signed moves from (Exact,Ignore) to (Exact,Exact) under reading A"
    );
    assert_eq!(
        a_views[4],
        Some((Lvl::Ignore, Lvl::Exact)),
        "the weak-equation row moves from (Ignore,Ignore) to (Ignore,Exact) under reading A"
    );
    println!("CLAIM B holds: under reading A every surviving composition has event level Exact.");

    // The structural reason, asserted rather than left as prose: every grouping has n-1
    // sites, so a site count is grouping-invariant by construction, for every input.
    for c in comps.iter() {
        let span = (c.hi - c.lo + 1) as usize;
        for k in 0..(span * span * span * span) {
            let a = c.lo + (k % span) as i32;
            let b = c.lo + ((k / span) % span) as i32;
            let cc = c.lo + ((k / (span * span)) % span) as i32;
            let d = c.lo + ((k / (span * span * span)) % span) as i32;
            for w in 0..5 {
                assert_eq!(
                    g4(w, a, b, cc, d, c.lo, c.hi, c.top, c.bot).ev_a,
                    3,
                    "site count must be n-1 = 3 in every grouping"
                );
            }
        }
    }
    println!("           and the site count is exactly 3 in every grouping of every input, all compositions.");

    // CLAIM C. Chain-ness. Under B the set contains two incomparable points; under A it
    // does not. Comparability on the product order.
    let cmp = |x: (Lvl, Lvl), y: (Lvl, Lvl)| -> bool {
        (x.0 <= y.0 && x.1 <= y.1) || (y.0 <= x.0 && y.1 <= x.1)
    };
    let live_b: Vec<(Lvl, Lvl)> = b_views.iter().filter_map(|v| *v).collect();
    let live_a: Vec<(Lvl, Lvl)> = a_views.iter().filter_map(|v| *v).collect();
    let mut incomp_b = 0;
    for i in 0..live_b.len() {
        for j in (i + 1)..live_b.len() {
            if !cmp(live_b[i], live_b[j]) {
                incomp_b += 1;
            }
        }
    }
    let mut incomp_a = 0;
    for i in 0..live_a.len() {
        for j in (i + 1)..live_a.len() {
            if !cmp(live_a[i], live_a[j]) {
                incomp_a += 1;
            }
        }
    }
    assert!(
        incomp_b > 0,
        "under reading B the measured views must contain an incomparable pair (37:181-185)"
    );
    assert_eq!(
        incomp_a, 0,
        "under reading A the measured views must form a chain"
    );
    println!(
        "CLAIM C holds: incomparable pairs under B = {}, under A = {}.",
        incomp_b, incomp_a
    );

    // CLAIM D. File 37's own witness, recomputed.
    let w0 = g3(0, -4, -3, 3, -4, 3, Res::ReduceModulo, Res::ReduceModulo);
    let w1 = g3(1, -4, -3, 3, -4, 3, Res::ReduceModulo, Res::ReduceModulo);
    assert_eq!(
        (w0.val, w1.val),
        (-4, -4),
        "the witness must deliver one value under both groupings"
    );
    assert_eq!(
        (w0.ev_b, w1.ev_b),
        (2, 0),
        "fired-reduction counts 2 and 0 (37:214-216)"
    );
    assert_eq!((w0.ev_a, w1.ev_a), (2, 2), "site counts agree at 2 and 2");
    println!(
        "CLAIM D holds: witness (-4,-3,3) delivers -4 both ways; firings 2 vs 0, sites 2 vs 2."
    );

    println!(
        "\nAll claims assert. No sampling: every input tuple of every composition was walked."
    );
}
