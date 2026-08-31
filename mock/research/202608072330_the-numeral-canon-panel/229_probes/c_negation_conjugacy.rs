#![allow(dead_code)]

// Probe C. Which operations are odd under negation, and what each one's
// conjugate is.
//
// A mode is odd when R(-x) == -R(x) everywhere. Where it is not, there is some
// other mode N with R(-x) == -N(x), and N is the conjugate. The conjugate is
// what the mode becomes when the sign convention flips, so a vocabulary that
// names a mode without naming its conjugate cannot express the mirror of a
// claim it already carries.
//
// dimension::signedness records that the ambient domains laws are stated over
// are closed under negation, and law::quantise_then_reduce_commutes is stated
// over such a domain, so conjugacy is not a decorative property here.
//
// Build and run:
//   rustc -O c_negation_conjugacy.rs -o /tmp/c && /tmp/c > c_output.txt

include!("modes.rs");

/// Count x where `round(a, -x) != -round(b, x)`, over the x whose negation is
/// also representable. On a two's complement signed domain the most negative
/// value has no representable negation and is skipped, which is a fact about
/// the domain rather than about the operation.
fn conjugacy_failures(a: Mode, b: Mode, w: u32, f: u32, signed: bool) -> u64 {
    let d = domain(w, signed);
    let (lo, hi) = (*d.start(), *d.end());
    let mut n = 0u64;
    for k in lo..=hi {
        let neg = -k;
        if neg < lo || neg > hi {
            continue;
        }
        if round(a, neg, f) != -round(b, k, f) {
            n += 1;
        }
    }
    n
}

fn main() {
    println!("PROBE C: negation conjugacy");
    println!();

    println!("== FIXTURE ==");
    let (ok, bad) = check_fixture();
    println!("  {} of {} fixture rows correct", ok, ok + bad);
    if bad != 0 {
        std::process::exit(1);
    }
    println!();

    println!("== CONTROL 1 (must hold) ==");
    println!("  toward_zero is odd by construction: it keeps the sign and takes");
    println!("  the same magnitude, so it is its own conjugate on every row.");
    let mut c1 = 0u64;
    for w in [4u32, 6, 8] {
        for f in 0..w {
            c1 += conjugacy_failures(Mode::TowardZero, Mode::TowardZero, w, f, true);
        }
    }
    println!("    failures over W in {{4,6,8}}, F in 0..W, signed: {}", c1);
    println!();

    println!("== CONTROL 2 (the case that must fail) ==");
    println!("  floor is not odd: floor(-x) is -ceil(x), so it must disagree with");
    println!("  itself on every non-integer. A zero here means the instrument is");
    println!("  not evaluating the negated argument at all.");
    let c2 = conjugacy_failures(Mode::Floor, Mode::Floor, 8, 2, true);
    println!("    floor against itself at W = 8, F = 2, signed: {}", c2);
    println!();

    println!("== ODDNESS: is each mode its own conjugate? W = 8, signed ==");
    println!("  mode                     F=1    F=2    F=3    F=4   verdict");
    for m in ALL_MODES {
        let mut row = String::new();
        let mut tot = 0u64;
        for f in 1..=4u32 {
            let n = conjugacy_failures(m, m, 8, f, true);
            tot += n;
            row.push_str(&format!("{:>7}", n));
        }
        println!(
            "  {:22}{}   {}",
            mode_name(m),
            row,
            if tot == 0 { "ODD" } else { "not odd" }
        );
    }
    println!();

    println!("== CONJUGATE OF EACH MODE, found by search over the mode set ==");
    println!("  R(-x) == -N(x) for all x. W = 8, F in 1..=4, signed.");
    for a in ALL_MODES {
        let mut found: Vec<&'static str> = Vec::new();
        for b in ALL_MODES {
            let mut tot = 0u64;
            for f in 1..=4u32 {
                tot += conjugacy_failures(a, b, 8, f, true);
            }
            if tot == 0 {
                found.push(mode_name(b));
            }
        }
        let shown = if found.is_empty() {
            "NONE IN THIS SET".to_string()
        } else {
            found.join(", ")
        };
        println!("  {:22} -> {}", mode_name(a), shown);
    }
    println!();

    println!("== THE SIX NAMES, AND WHETHER THE SET HOLDS EACH CONJUGATE ==");
    println!("  the vocabulary is floor, ceil, toward_zero, half_up, half_even,");
    println!("  stochastic. stochastic is not a function and is probe D's subject.");
    for (name, m, in_set) in [
        ("floor", Mode::Floor, true),
        ("ceil", Mode::Ceil, true),
        ("toward_zero", Mode::TowardZero, true),
        ("half_up[toward +inf]", Mode::HalfUpTowardPosInf, true),
        ("half_up[away from 0]", Mode::HalfUpAwayFromZero, true),
        ("half_even", Mode::HalfEven, true),
    ] {
        let _ = in_set;
        let mut conj = "NONE IN THIS SET";
        for b in ALL_MODES {
            let mut tot = 0u64;
            for f in 1..=4u32 {
                tot += conjugacy_failures(m, b, 8, f, true);
            }
            if tot == 0 {
                conj = mode_name(b);
                break;
            }
        }
        let named_in_vocabulary = matches!(
            conj,
            "floor" | "ceil" | "toward_zero" | "half_even" | "half_up[toward +inf]" | "half_up[away from 0]"
        );
        println!(
            "  {:22} conjugate {:24} in the six? {}",
            name,
            conj,
            if named_in_vocabulary { "yes" } else { "NO" }
        );
    }
    println!();

    println!("== VERDICTS ==");
    println!("  control 1 (toward_zero is odd):  {}", if c1 == 0 { "PASS" } else { "FAIL" });
    println!("  control 2 (floor is not):        {}", if c2 > 0 { "PASS" } else { "FAIL" });
    let sound = c1 == 0 && c2 > 0;
    println!("  instrument: {}", if sound { "sound" } else { "INVALID" });
    if !sound {
        std::process::exit(1);
    }
}
