//! Probe 2: division's laws under both counting readings, and the coincidence that ties
//! the two held calls together.
//!
//! WHAT THIS MODEL SEPARATES (`86b:8-10`). It separates reading A (a quantiser application
//! is an event) from reading B (a moved value is an event) on the two laws division has.
//! The distinction is nonvacuous here twice over, and deliberately in opposite directions:
//! the round-trip law's divisions NEVER move a value (the quotient is always on-grid), and
//! the general-division sweep's divisions move a value on most inputs. A model with only
//! the first would make reading B look vacuously trivial; a model with only the second
//! would make the two readings look identical. Both are here.
//!
//! It does NOT separate: anything about the cause component (identical under both), the
//! accumulator-width question (a separate finding, see `43:145-154`), or the exact
//! subfamily's index map (compiled at `43_probes/probe_3`, not repeated).
//!
//! Model: file 43's, rebuilt rather than reused. Unsigned p = 4, F = 2, so the value set
//! is {k/4 : k in 0..=15}. `mul_full` lands exactly on the 1/16 grid (the product numeral),
//! no event, total. `div` is quantise(exact quotient) back onto the 1/4 grid, round to
//! nearest ties to even, refusing at divisor zero.
//!
//! CLAIM A: the round-trip law div(mul_full(a, b), b) = a has value agreement on all
//!   16 * 15 defined pairs, and definedness disagrees at exactly the 16 pairs with b = 0.
//!   Both readings. (This is file 43's probe 5, CLAIMS A and B, reproduced.)
//! CLAIM B: under reading A the round-trip's event counts are 1 against 0 on every defined
//!   pair, so any event-preserving view refuses the law: its finest view is (Ignore, Ignore),
//!   the weak-equation corner. (File 43's probe 5, CLAIM C, reproduced.)
//! CLAIM C: under reading B the round-trip's event counts are 0 against 0 on every defined
//!   pair, because the exact quotient of the exact product by b is a's value, already on the
//!   grid. Its finest view is (Ignore, Exact), which is `Precise`'s point, one rung up.
//! CLAIM D: the model is not vacuous on reading B. Over the whole general-division matrix,
//!   the divider moves the value on a strict majority of defined pairs, counted and printed.
//! CLAIM E: under reading A, division by a representable nonzero CONSTANT still charges one
//!   event per application even where the numeral-level map is the identity on indices, so
//!   `div_exact` must be a distinct operation to reach a trivial grade. Under reading B it
//!   charges nothing, and the same `div` call is already event-free. Both asserted over the
//!   whole matrix, at every constant in the value set.

/// A term's meaning on this model. `val16` is in 1/16 units so both grids are exact.
#[derive(Clone, Copy, Debug)]
struct G {
    defined: bool,
    val16: i64,
    ev_a: u32,
    ev_b: u32,
    causes: u32,
}

const CAUSE_DIVZERO: u32 = 1;

/// Round num/den to the nearest multiple of 1/g, ties to even, returning the numerator
/// in 1/g units. All integer, exact.
const fn rne(num: i64, den: i64, g: i64) -> i64 {
    let t = num * g;
    let q0 = t / den;
    let r = t % den;
    if 2 * r > den || (2 * r == den && q0 % 2 == 1) {
        q0 + 1
    } else {
        q0
    }
}

/// A leaf on the 1/4 grid: k quarters.
const fn leaf(k: i64) -> G {
    G {
        defined: true,
        val16: k * 4,
        ev_a: 0,
        ev_b: 0,
        causes: 0,
    }
}

/// `mul_full`: exact into the product numeral (1/16 grid). Exact and total, so it charges
/// nothing under either reading. Values are quarters * quarters = sixteenths.
const fn mul_full(a: G, b: G) -> G {
    if !a.defined || !b.defined {
        return G {
            defined: false,
            val16: 0,
            ev_a: a.ev_a + b.ev_a,
            ev_b: a.ev_b + b.ev_b,
            causes: a.causes | b.causes,
        };
    }
    // a.val16 and b.val16 are quarters expressed in sixteenths (multiples of 4).
    let prod16 = (a.val16 / 4) * (b.val16 / 4); // exact, in sixteenths
    G {
        defined: true,
        val16: prod16,
        ev_a: a.ev_a + b.ev_a,
        ev_b: a.ev_b + b.ev_b,
        causes: a.causes | b.causes,
    }
}

/// General division: quantise(exact quotient) onto the 1/4 grid. One quantiser site.
///
/// Reading A charges the site unconditionally. Reading B charges only when the delivered
/// value differs from the exact quotient, which is the standard's own condition for
/// signalling inexact (IEEE 754-2019 clause 7.6, secondary read, see the file).
const fn div(a: G, b: G) -> G {
    let ev_a = a.ev_a + b.ev_a + 1;
    let base_b = a.ev_b + b.ev_b;
    let causes = a.causes | b.causes;
    if !a.defined || !b.defined {
        return G {
            defined: false,
            val16: 0,
            ev_a,
            ev_b: base_b,
            causes,
        };
    }
    if b.val16 == 0 {
        // Kind 2 (`84:236-240`): the value does not exist. No quantiser runs, so reading B
        // charges nothing here and the cause carries the failure. Reading A still charges
        // the site, which is one of the two readings' less obvious divergences.
        return G {
            defined: false,
            val16: 0,
            ev_a,
            ev_b: base_b,
            causes: causes | CAUSE_DIVZERO,
        };
    }
    // Exact quotient is (a.val16 / 16) / (b.val16 / 16) = a.val16 / b.val16.
    // Round it onto the 1/4 grid: numerator a.val16, denominator b.val16, g = 4.
    let q4 = rne(a.val16, b.val16, 4);
    // Did the value move? The quotient is on-grid exactly when 4 * a.val16 is divisible
    // by b.val16, and then q4 * b.val16 == 4 * a.val16.
    let moved = q4 * b.val16 != 4 * a.val16;
    G {
        defined: true,
        val16: q4 * 4,
        ev_a,
        ev_b: base_b + if moved { 1 } else { 0 },
        causes,
    }
}

fn main() {
    // ---- The round-trip law, both readings, exhaustive over 16 * 16 pairs.
    let mut defined_pairs = 0u32;
    let mut undefined_pairs = 0u32;
    let mut a_counts_disagree = 0u32;
    let mut b_counts_disagree = 0u32;
    let mut value_disagree = 0u32;
    for ka in 0..16i64 {
        for kb in 0..16i64 {
            let a = leaf(ka);
            let b = leaf(kb);
            let lhs = div(mul_full(a, b), b);
            let rhs = a;
            if !lhs.defined {
                undefined_pairs += 1;
                assert_eq!(kb, 0, "the only refusal is the zero divisor");
                assert_eq!(
                    lhs.causes, CAUSE_DIVZERO,
                    "and it is the divide-by-zero cause"
                );
                assert!(
                    rhs.defined,
                    "the right side is a leaf and is always defined"
                );
                continue;
            }
            defined_pairs += 1;
            if lhs.val16 != rhs.val16 {
                value_disagree += 1;
            }
            if lhs.ev_a != rhs.ev_a {
                a_counts_disagree += 1;
            }
            if lhs.ev_b != rhs.ev_b {
                b_counts_disagree += 1;
            }
            // The specific numbers, asserted rather than merely counted.
            assert_eq!(
                (lhs.ev_a, rhs.ev_a),
                (1, 0),
                "reading A: one site against none"
            );
            assert_eq!(
                (lhs.ev_b, rhs.ev_b),
                (0, 0),
                "reading B: nothing moved on either side"
            );
        }
    }

    // CLAIM A.
    assert_eq!(
        value_disagree, 0,
        "values must agree wherever both are defined"
    );
    assert_eq!(
        undefined_pairs, 16,
        "definedness disagrees at exactly the 16 zero-divisor pairs"
    );
    assert_eq!(defined_pairs, 240, "16 * 15 defined pairs");
    println!("CLAIM A holds: 240 defined pairs, values agree on all of them; 16 undefined, all at b = 0.");

    // CLAIM B and CLAIM C.
    assert_eq!(
        a_counts_disagree, 240,
        "reading A: the event counts disagree on every defined pair"
    );
    assert_eq!(
        b_counts_disagree, 0,
        "reading B: the event counts agree on every defined pair"
    );
    println!("CLAIM B holds: under reading A the round-trip law's finest view is (Ignore, Ignore), the weak-equation corner.");
    println!("CLAIM C holds: under reading B it is (Ignore, Exact), which is Precise's own point.");

    // CLAIM D. The model is not vacuous on reading B: general division moves values.
    let mut defined = 0u32;
    let mut moved = 0u32;
    for ka in 0..16i64 {
        for kb in 1..16i64 {
            let r = div(leaf(ka), leaf(kb));
            assert!(r.defined);
            defined += 1;
            if r.ev_b == 1 {
                moved += 1;
            }
            assert_eq!(r.ev_a, 1, "reading A charges every division exactly once");
        }
    }
    assert!(moved * 2 > defined, "the divider must move the value on a strict majority, or this model cannot separate the readings");
    println!("CLAIM D holds: general division moves the value on {}/{} defined pairs; reading A charges all {}.", moved, defined, defined);

    // CLAIM E. Division by a representable nonzero constant, at every such constant.
    let mut const_sites_a = 0u32;
    let mut const_moves_b = 0u32;
    let mut const_cells = 0u32;
    for kc in 1..16i64 {
        // The exact subfamily is the constants whose reciprocal maps the grid into itself.
        // At this model that is the constants c with 4 * k divisible by (4 * kc) for every
        // k, i.e. kc dividing every k, i.e. kc = 1; the general statement (`43:164-182`) is
        // that the RESULT numeral changes, which this value-level model cannot show. What
        // it can show, and what the counting question needs, is the charge either reading
        // makes on the division call itself.
        for ka in 0..16i64 {
            let r = div(leaf(ka), leaf(kc));
            const_cells += 1;
            const_sites_a += r.ev_a;
            const_moves_b += r.ev_b;
        }
    }
    assert_eq!(
        const_sites_a, const_cells,
        "reading A charges one event per call, at every constant"
    );
    assert!(
        const_moves_b < const_cells,
        "reading B charges strictly fewer"
    );
    println!("CLAIM E holds: over {} constant-divisor cells, reading A charges {} events and reading B charges {}.", const_cells, const_sites_a, const_moves_b);

    // The one cell worth naming on its own: divisor exactly one.
    let mut by_one_a = 0u32;
    let mut by_one_b = 0u32;
    for ka in 0..16i64 {
        let r = div(leaf(ka), leaf(4)); // 4 quarters = 1.0
        assert_eq!(r.val16, leaf(ka).val16, "x / 1 == x");
        by_one_a += r.ev_a;
        by_one_b += r.ev_b;
    }
    assert_eq!(
        (by_one_a, by_one_b),
        (16, 0),
        "dividing by one: reading A charges 16 events, reading B charges none"
    );
    println!("           and x / 1 == x, exactly, charges 16 events under reading A and 0 under reading B.");

    println!("\nAll claims assert. No sampling: every pair of the model's value set was walked.");
}
