// p3: which reading of the precision coordinate makes the three sign domains a
// structure a const predicate can gate on?
//
// WHY THIS RUNS. `question::does_precision_count_the_sign_digit` says in its own
// `bound` field that nothing decides this mathematically, both readings were
// computed through to their consequences, and what is left is a convention
// chosen for what it makes expressible. It then states the criterion:
//
//   "take the reading under which the three sign domains form a structure a
//    const predicate can gate on rather than one leaving a domain incomparable"
//
// That criterion is not a matter of taste and it has never been computed. This
// probe computes it. The structure is inclusion of DENOTATIONS, which is the
// order `proposal::membership_of_the_representable_set_is_one_affine_predicate`
// makes primary and the one `p2` uses.
//
// THE THREE DOMAINS, parameterised by `d` value digits at radix `r`:
//   NON-NEGATIVE   0 ..= r^d - 1                       r^d values, no sign digit
//   SYMMETRIC      -(r^d - 1) ..= r^d - 1              2r^d - 1 values, sign digit
//   ASYMMETRIC     -r^d ..= r^d - 1                    2r^d values, sign digit
// and, only at odd radix, the fourth that has no sign digit to count:
//   BALANCED       -(r^d - 1)/2 ..= (r^d - 1)/2        r^d values, no sign digit
//
// THE TWO READINGS of `Precision = P`:
//   READING A  precision COUNTS the sign digit, so a domain carrying one has
//              d = P - 1 and a domain carrying none has d = P.
//   READING B  precision does NOT count the sign digit, so d = P throughout.
//
// WHAT MUST FAIL, declared before the run.
//   C1  At least one reading must leave a pair incomparable at some (r, P).
//       If both readings produce a chain everywhere, the stated criterion picks
//       nothing and this probe has measured a tautology.
//   C2  At least one reading must produce a chain at some (r, P). If neither
//       does, the criterion is unsatisfiable and the answer is not in it.
//   C3  The comparator must be able to report every verdict it has a branch
//       for, tested on planted ranges rather than on the three domains. The
//       first cut of this control demanded `Equal` among the three domains and
//       VOIDED the run, correctly: at one precision the three always have
//       different cardinalities, so `Equal` is unreachable there by
//       construction and the control was asking the domain set for a verdict
//       only the comparator can be asked for. That run is kept as
//       `p3_v1_c3a_demanded_a_verdict_the_domain_set_cannot_produce.out`, and
//       that the three are never equal is now reported as a result rather than
//       required as a control.
//   C4  At P = 1 under reading A the symmetric domain must denote exactly the
//       zero set, which is the consequence the question's own `unblocks` names.
//       If it does not, this probe is not modelling the reading the question
//       is about.
//
// SCOPE. radix in {2, 3, 4, 10}, precision 1..=5, exact over the whole domain
// at each point. No container, no strategy, no operation: this is about a
// representable set and nothing computes on it here.

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Dom {
    NonNegative,
    Symmetric,
    Asymmetric,
    Balanced,
}

fn pow(r: i128, d: u32) -> i128 {
    (0..d).fold(1i128, |a, _| a * r)
}

/// Inclusive range of the denotation, or `None` where the point does not exist.
fn range(dom: Dom, r: i128, d: i64) -> Option<(i128, i128)> {
    if d < 0 {
        return None;
    }
    let n = pow(r, d as u32);
    Some(match dom {
        Dom::NonNegative => (0, n - 1),
        Dom::Symmetric => (-(n - 1), n - 1),
        Dom::Asymmetric => (-n, n - 1),
        Dom::Balanced => {
            if r % 2 == 0 {
                return None;
            }
            (-((n - 1) / 2), (n - 1) / 2)
        }
    })
}

fn has_sign_digit(dom: Dom) -> bool {
    matches!(dom, Dom::Symmetric | Dom::Asymmetric)
}

/// `d` under each reading.
fn digits(dom: Dom, p: i64, counts_sign: bool) -> i64 {
    if counts_sign && has_sign_digit(dom) {
        p - 1
    } else {
        p
    }
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
enum Rel {
    Equal,
    Subset,
    Superset,
    Incomparable,
}

fn rel(a: (i128, i128), b: (i128, i128)) -> Rel {
    let a_in_b = b.0 <= a.0 && a.1 <= b.1;
    let b_in_a = a.0 <= b.0 && b.1 <= a.1;
    match (a_in_b, b_in_a) {
        (true, true) => Rel::Equal,
        (true, false) => Rel::Subset,
        (false, true) => Rel::Superset,
        (false, false) => Rel::Incomparable,
    }
}

fn main() {
    println!("### p3. does the precision coordinate count the sign digit?");
    println!("### criterion, from the question's own bound: take the reading under which the");
    println!("### three sign domains form a structure a const predicate can gate on rather");
    println!("### than one leaving a domain incomparable.");
    println!();

    let radices: [i128; 4] = [2, 3, 4, 10];
    let precisions: [i64; 5] = [1, 2, 3, 4, 5];
    let three = [Dom::NonNegative, Dom::Symmetric, Dom::Asymmetric];

    let mut seen_equal = false;
    let mut seen_subset = false;
    let mut seen_incomparable = false;

    // reading -> (points where the three are a chain, points where they are not)
    let mut tally = [(0usize, 0usize); 2];
    let mut first_break = [String::new(), String::new()];

    for (ri, &counts_sign) in [true, false].iter().enumerate() {
        for &r in &radices {
            for &p in &precisions {
                let mut sets = Vec::new();
                let mut skip = false;
                for &d in &three {
                    match range(d, r, digits(d, p, counts_sign)) {
                        Some(x) => sets.push((d, x)),
                        None => skip = true,
                    }
                }
                if skip {
                    continue;
                }
                let mut chain = true;
                for i in 0..sets.len() {
                    for j in (i + 1)..sets.len() {
                        let v = rel(sets[i].1, sets[j].1);
                        match v {
                            Rel::Equal => seen_equal = true,
                            Rel::Subset | Rel::Superset => seen_subset = true,
                            Rel::Incomparable => {
                                seen_incomparable = true;
                                chain = false;
                                if first_break[ri].is_empty() {
                                    first_break[ri] = format!(
                                        "r={r} P={p}: {:?}{:?} vs {:?}{:?} are incomparable",
                                        sets[i].0, sets[i].1, sets[j].0, sets[j].1
                                    );
                                }
                            }
                        }
                    }
                }
                if chain {
                    tally[ri].0 += 1;
                } else {
                    tally[ri].1 += 1;
                }
            }
        }
    }

    // C4: reading A at P = 1 makes the symmetric domain the zero set.
    let c4 = range(Dom::Symmetric, 2, digits(Dom::Symmetric, 1, true)) == Some((0, 0));

    // C3: the comparator itself, on planted ranges. Every branch reachable.
    let c3_equal = rel((-2, 2), (-2, 2)) == Rel::Equal;
    let c3_sub = rel((-1, 1), (-2, 2)) == Rel::Subset;
    let c3_sup = rel((-2, 2), (-1, 1)) == Rel::Superset;
    let c3_inc = rel((0, 4), (-2, 2)) == Rel::Incomparable;
    let c3 = c3_equal && c3_sub && c3_sup && c3_inc;

    let mut void = false;
    println!("CONTROLS");
    for (name, ok, req) in [
        (
            "C1  some reading leaves a pair incomparable",
            seen_incomparable,
            "incomparable seen",
        ),
        (
            "C2  some reading produces a chain",
            tally[0].0 + tally[1].0 > 0,
            "a chain seen",
        ),
        (
            "C3  the comparator reaches all four branches",
            c3,
            "planted, all four",
        ),
        (
            "C4  reading A at P=1 makes symmetric the zero set",
            c4,
            "{0}",
        ),
    ] {
        println!(
            "  {name:<52} {:>11}  required={req}",
            if ok { "as required" } else { "*** VOID ***" }
        );
        void |= !ok;
    }
    println!();
    if void {
        println!("*** A CONTROL DID NOT REPORT ITS REQUIRED VERDICT. NOTHING BELOW COUNTS. ***");
        std::process::exit(1);
    }

    println!(
        "  among the three domains at one precision, `Equal` occurs: {}   (never, by cardinality)",
        seen_equal
    );
    println!(
        "  strict inclusion occurs: {seen_subset};  incomparability occurs: {seen_incomparable}"
    );
    println!();

    println!("MEASUREMENT  (20 points: radix in {{2,3,4,10}} x precision 1..=5)");
    for (ri, label) in [
        (0usize, "READING A  precision COUNTS the sign digit"),
        (1, "READING B  precision does NOT count it"),
    ] {
        println!(
            "  {label:<44}  chain at {:>2} of {:>2} points, broken at {:>2}",
            tally[ri].0,
            tally[ri].0 + tally[ri].1,
            tally[ri].1
        );
        if !first_break[ri].is_empty() {
            println!("      first break: {}", first_break[ri]);
        } else {
            println!("      no break");
        }
    }
    println!();

    println!("THE TWO CONSEQUENCES THE QUESTION'S `unblocks` NAMES");
    println!(
        "  symmetric at P=1 under reading A denotes {:?}   (a zero-width numeral, already in the space)",
        range(Dom::Symmetric, 2, digits(Dom::Symmetric, 1, true)).unwrap()
    );
    println!(
        "  symmetric at P=1 under reading B denotes {:?}",
        range(Dom::Symmetric, 2, digits(Dom::Symmetric, 1, false)).unwrap()
    );
    println!();
    println!("  do two of the three collapse at an odd radix?");
    for &r in &radices {
        let p = 3i64;
        let bal = range(Dom::Balanced, r, p);
        let sym = range(Dom::Symmetric, r, digits(Dom::Symmetric, p, false)).unwrap();
        let asym = range(Dom::Asymmetric, r, digits(Dom::Asymmetric, p, false)).unwrap();
        match bal {
            None => println!("    r={r:>2}  even radix: no balanced point exists, nothing collapses"),
            Some(b) => println!(
                "    r={r:>2}  balanced {b:?} vs symmetric {sym:?} -> {:?};  vs asymmetric {asym:?} -> {:?}",
                rel(b, sym),
                rel(b, asym)
            ),
        }
    }
    println!();
    println!("  a balanced point carries NO sign digit, so at an odd radix the reading");
    println!("  question is vacuous for it: there is nothing for precision to count.");
}
