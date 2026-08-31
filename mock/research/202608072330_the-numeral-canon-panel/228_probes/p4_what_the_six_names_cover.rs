// p4: which rounding rules the ratified six actually have a name for.
//
// The question `is_the_rounding_vocabulary_complete_at_six` asks whether the
// vocabulary is short a name, on the evidence that three predicates sweep
// `away from zero`. Completeness is only answerable against a reference set,
// so this uses the one the design has to live beside:
//
//   IEEE 754-2008 clause 4.3 names five rounding-direction attributes,
//   roundTiesToEven, roundTiesToAway, roundTowardPositive, roundTowardNegative
//   and roundTowardZero. It names no away-from-zero DIRECTED attribute.
//
//   Python's decimal documents ROUND_HALF_UP as "round to nearest with ties
//   going away from zero" and ROUND_UP as "round away from zero". Java's
//   RoundingMode.HALF_UP behaves as RoundingMode.UP on a tie, and UP is
//   documented as "round away from zero".
//
// So the name `half_up` denotes roundTiesToAway in both of those libraries,
// and denotes floor(x + 1/2) in every instrument in this corpus. This decides
// coverage differently, and that is the point of the table below.
//
// Membership is decided by function equality on the domain, exhaustively, not
// by matching words.
//
// Run: rustc --edition 2024 -O p4_what_the_six_names_cover.rs -o /tmp/p4 && /tmp/p4

type F = fn(i128, i128) -> i128;

fn floor_(x: i128, d: i128) -> i128 { x.div_euclid(d) }
fn ceil_(x: i128, d: i128) -> i128 { -((-x).div_euclid(d)) }
fn toward_zero(x: i128, d: i128) -> i128 { x / d }
fn away_from_zero(x: i128, d: i128) -> i128 {
    if x >= 0 { ceil_(x, d) } else { floor_(x, d) }
}
/// floor(x + 1/2). What every instrument in this corpus implements for the
/// name `half_up`, `149_probes/y1` saying so in a comment.
fn half_up_pinf(x: i128, d: i128) -> i128 { floor_(2 * x + d, 2 * d) }
/// IEEE roundTiesToAway. What Java and Python mean by HALF_UP.
fn ties_away(x: i128, d: i128) -> i128 {
    if x >= 0 { floor_(2 * x + d, 2 * d) } else { -floor_(-2 * x + d, 2 * d) }
}
/// Ties toward zero. The fourth nearest rule, in neither standard, included so
/// the reference set is not built only from rules that happen to have a name.
fn ties_toward_zero(x: i128, d: i128) -> i128 {
    if x >= 0 { -floor_(-2 * x + d, 2 * d) } else { floor_(2 * x + d, 2 * d) }
}
fn ties_even(x: i128, d: i128) -> i128 {
    let lo = floor_(x, d);
    let r2 = 2 * (x - lo * d);
    if r2 > d { lo + 1 } else if r2 < d { lo } else if lo % 2 == 0 { lo } else { lo + 1 }
}

fn same(a: F, b: F, s: u32, lo: i128, hi: i128) -> bool {
    let d = 1i128 << s;
    (lo..=hi).all(|x| a(x, d) == b(x, d))
}

fn main() {
    let s = 3u32;
    let (lo, hi) = (-512i128, 512i128);
    println!("p4: what the six ratified names cover");
    println!("    function equality decided exhaustively on x in {lo}..={hi}, d = 2^{s}");
    println!();

    // The ratified vocabulary, as two candidate readings differing only in
    // which operation `half_up` denotes.
    let reading_a: Vec<(&str, F)> = vec![
        ("toward_zero", toward_zero), ("floor", floor_), ("ceil", ceil_),
        ("half_up", half_up_pinf), ("half_even", ties_even),
    ];
    let reading_b: Vec<(&str, F)> = vec![
        ("toward_zero", toward_zero), ("floor", floor_), ("ceil", ceil_),
        ("half_up", ties_away), ("half_even", ties_even),
    ];

    // The reference set. `stochastic` is omitted because it is not a function
    // and equality does not apply to it; it is ratified and uncontested.
    let reference: Vec<(&str, &str, F)> = vec![
        ("roundTowardNegative", "IEEE 754-2008", floor_),
        ("roundTowardPositive", "IEEE 754-2008", ceil_),
        ("roundTowardZero", "IEEE 754-2008", toward_zero),
        ("roundTiesToEven", "IEEE 754-2008", ties_even),
        ("roundTiesToAway", "IEEE 754-2008", ties_away),
        ("floor(x + 1/2)", "this corpus", half_up_pinf),
        ("away from zero", "decimal ROUND_UP", away_from_zero),
        ("ties toward zero", "neither", ties_toward_zero),
    ];

    for (label, vocab) in [("A: half_up = floor(x + 1/2), the corpus's instruments", &reading_a),
                           ("B: half_up = roundTiesToAway, Java and Python", &reading_b)] {
        println!("## reading {label}");
        println!();
        println!("{:<22} {:<18} {:<12} {}", "reference rule", "named in", "covered?", "by which ratified name");
        for (rname, origin, rf) in &reference {
            let hit = vocab.iter().find(|(_, vf)| same(*rf, *vf, s, lo, hi));
            match hit {
                Some((vn, _)) => println!("{:<22} {:<18} {:<12} {}", rname, origin, "yes", vn),
                None => println!("{:<22} {:<18} {:<12} -", rname, origin, "NO"),
            }
        }
        println!();
    }

    // ---- the same table on a non-negative domain --------------------------
    println!("## and on a non-negative domain, where the design's unsigned rows live");
    println!();
    println!("{:<22} {:<12} {}", "reference rule", "covered?", "by which ratified name (reading A)");
    for (rname, _, rf) in &reference {
        let hit = reading_a.iter().find(|(_, vf)| same(*rf, *vf, s, 0, hi));
        match hit {
            Some((vn, _)) => println!("{:<22} {:<12} {}", rname, "yes", vn),
            None => println!("{:<22} {:<12} -", rname, "NO"),
        }
    }
    println!();

    // ---- controls ----------------------------------------------------------
    println!("## controls");
    println!();
    let mut ok = true;
    // C1: the equality test must be able to say no. floor and ceil differ.
    if !same(floor_, ceil_, s, lo, hi) {
        println!("  C1 EXPECTED-FAIL ok: floor and ceil are not equal, so the test can say no");
    } else { println!("  C1 BROKEN"); ok = false; }
    // C2: and to say yes. floor equals roundTowardNegative by construction.
    if same(floor_, floor_, s, lo, hi) {
        println!("  C2 EXPECTED-PASS ok: the test can say yes");
    } else { println!("  C2 BROKEN"); ok = false; }
    // C3: the two readings of half_up must differ signed and agree unsigned,
    // or the whole table collapses to one reading.
    let differ_signed = !same(half_up_pinf, ties_away, s, lo, hi);
    let agree_unsigned = same(half_up_pinf, ties_away, s, 0, hi);
    if differ_signed && agree_unsigned {
        println!("  C3 EXPECTED ok: the readings differ signed and agree on the non-negative domain");
    } else {
        println!("  C3 BROKEN: differ_signed={differ_signed} agree_unsigned={agree_unsigned}");
        ok = false;
    }
    // C4: away_from_zero must be covered on the non-negative domain and not
    // signed, or the away_from_zero finding is about nothing.
    let a_unsigned = reading_a.iter().any(|(_, vf)| same(away_from_zero, *vf, s, 0, hi));
    let a_signed = reading_a.iter().any(|(_, vf)| same(away_from_zero, *vf, s, lo, hi));
    if a_unsigned && !a_signed {
        println!("  C4 EXPECTED ok: away_from_zero has a ratified name on the non-negative domain and none signed");
    } else {
        println!("  C4 BROKEN: unsigned={a_unsigned} signed={a_signed}");
        ok = false;
    }
    println!();
    println!("controls: {}", if ok { "clean" } else { "BROKEN" });
}
