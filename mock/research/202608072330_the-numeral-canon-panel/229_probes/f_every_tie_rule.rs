#![allow(dead_code)]

// Probe F. Is the trade probe B and probe E found forced, or is there a tie
// rule nobody has named that escapes it?
//
// A nearest mode is fully determined by what it does at a tie, so the space of
// nearest modes over a finite domain is finite: one per assignment of up or
// down to each tie value. This enumerates the whole space at four small
// (W, F) points and asks which assignments give a translation equivariant mode
// and which give a mode with zero mean error on the symmetric domain.
//
// If the two sets are disjoint the trade is a theorem over this domain rather
// than an accident of which two readings of `half_up` happen to be current.
//
// The controls are that each set is non-empty on its own, so an empty
// intersection is a result and not a test that cannot pass.
//
// Build and run:
//   rustc -O f_every_tie_rule.rs -o /tmp/f && /tmp/f > f_output.txt

include!("modes.rs");

/// The tie values of the domain, in increasing order. A nearest mode's whole
/// freedom is one bit per entry.
fn ties(w: u32, f: u32) -> Vec<i64> {
    let s: i64 = 1i64 << f;
    domain(w, true).filter(|k| 2 * frem(*k, s) == s).collect()
}

/// Round under "nearest, with this tie rule". Bit `i` of `rule` set means the
/// `i`th tie in `ties` goes up.
fn round_with_rule(k: i64, f: u32, tie_list: &[i64], rule: u64) -> i64 {
    let s: i64 = 1i64 << f;
    let q = fdiv(k, s);
    let r = frem(k, s);
    let twice = 2 * r;
    if twice > s {
        q + 1
    } else if twice < s {
        q
    } else {
        let i = tie_list.iter().position(|t| *t == k).unwrap();
        if (rule >> i) & 1 == 1 {
            q + 1
        } else {
            q
        }
    }
}

fn is_equivariant(w: u32, f: u32, tie_list: &[i64], rule: u64) -> bool {
    let s: i64 = 1i64 << f;
    let d = domain(w, true);
    let (lo, hi) = (*d.start(), *d.end());
    for k in lo..=hi {
        let base = round_with_rule(k, f, tie_list, rule);
        let mut t = 1i64;
        while k + t * s <= hi {
            if round_with_rule(k + t * s, f, tie_list, rule) != base + t {
                return false;
            }
            t += 1;
        }
        let mut t = -1i64;
        while k + t * s >= lo {
            if round_with_rule(k + t * s, f, tie_list, rule) != base + t {
                return false;
            }
            t -= 1;
        }
    }
    true
}

/// Zero summed error over the symmetric domain, which drops the most negative
/// value because its negation is not representable.
fn is_unbiased(w: u32, f: u32, tie_list: &[i64], rule: u64) -> bool {
    let s: i64 = 1i64 << f;
    let d = domain(w, true);
    let (lo, hi) = (*d.start() + 1, *d.end());
    let mut num: i64 = 0;
    for k in lo..=hi {
        num += round_with_rule(k, f, tie_list, rule) * s - k;
    }
    num == 0
}

/// Which rule number reproduces a named mode, so the named modes can be
/// located inside the enumeration rather than assumed to be in it.
fn rule_of(m: Mode, f: u32, tie_list: &[i64]) -> u64 {
    let s: i64 = 1i64 << f;
    let mut rule = 0u64;
    for (i, t) in tie_list.iter().enumerate() {
        let q = fdiv(*t, s);
        if round(m, *t, f) == q + 1 {
            rule |= 1 << i;
        }
    }
    rule
}

fn main() {
    println!("PROBE F: every nearest mode over the domain, by enumerating tie rules");
    println!();

    println!("== FIXTURE ==");
    let (ok, bad) = check_fixture();
    println!("  {} of {} fixture rows correct", ok, ok + bad);
    if bad != 0 {
        std::process::exit(1);
    }
    println!();

    let points = [(6u32, 2u32), (6, 3), (8, 4), (8, 5)];
    let mut all_disjoint = true;
    let mut equi_nonempty = true;
    let mut unbi_nonempty = true;
    let mut named_found = true;

    for (w, f) in points {
        let tie_list = ties(w, f);
        let t = tie_list.len();
        let space = 1u64 << t;
        let mut equi = 0u64;
        let mut unbi = 0u64;
        let mut both = 0u64;
        let mut equi_rules: Vec<u64> = Vec::new();
        for rule in 0..space {
            let e = is_equivariant(w, f, &tie_list, rule);
            let u = is_unbiased(w, f, &tie_list, rule);
            if e {
                equi += 1;
                if equi_rules.len() < 8 {
                    equi_rules.push(rule);
                }
            }
            if u {
                unbi += 1;
            }
            if e && u {
                both += 1;
            }
        }
        println!("== W = {}, F = {} ==", w, f);
        println!(
            "  ties in the domain: {}, so {} nearest modes exist over it",
            t, space
        );
        println!("  translation equivariant: {}", equi);
        println!("  zero mean error:         {}", unbi);
        println!("  BOTH:                    {}", both);
        print!("  the equivariant rules, as tie bitmasks: ");
        for r in &equi_rules {
            print!("{:#0width$b} ", r, width = t + 2);
        }
        println!();
        // Locate the named modes inside the enumeration.
        for m in [
            Mode::HalfUpTowardPosInf,
            Mode::HalfUpAwayFromZero,
            Mode::HalfDownTowardNegInf,
            Mode::HalfEven,
        ] {
            let r = rule_of(m, f, &tie_list);
            // Confirm the reconstruction really is that mode, on every value.
            let mut agrees = true;
            for k in domain(w, true) {
                if round_with_rule(k, f, &tie_list, r) != round(m, k, f) {
                    agrees = false;
                    break;
                }
            }
            if !agrees {
                named_found = false;
            }
            println!(
                "  {:24} rule {:#0width$b}  equivariant {:5}  unbiased {:5}  {}",
                mode_name(m),
                r,
                is_equivariant(w, f, &tie_list, r),
                is_unbiased(w, f, &tie_list, r),
                if agrees {
                    "reconstructed"
                } else {
                    "RECONSTRUCTION FAILED"
                },
                width = t + 2
            );
        }
        println!();
        if both != 0 {
            all_disjoint = false;
        }
        if equi == 0 {
            equi_nonempty = false;
        }
        if unbi == 0 {
            unbi_nonempty = false;
        }
    }

    println!("== CONTROLS ==");
    println!("  Each property must be satisfiable on its own, or an empty");
    println!("  intersection would say nothing.");
    println!(
        "    some rule is equivariant: {}",
        if equi_nonempty { "PASS" } else { "FAIL" }
    );
    println!(
        "    some rule is unbiased:    {}",
        if unbi_nonempty { "PASS" } else { "FAIL" }
    );
    println!(
        "    named modes reconstructed inside the enumeration: {}",
        if named_found { "PASS" } else { "FAIL" }
    );
    println!();

    println!("== RESULT ==");
    if all_disjoint {
        println!("  No deterministic nearest mode over these domains is both");
        println!("  translation equivariant and unbiased. The two sets are disjoint");
        println!("  at every point swept, so the trade is forced rather than a");
        println!("  property of the two readings that happen to be current.");
    } else {
        println!("  A rule satisfying both exists. The trade is not forced.");
    }

    let sound = equi_nonempty && unbi_nonempty && named_found;
    println!();
    println!("  instrument: {}", if sound { "sound" } else { "INVALID" });
    if !sound {
        std::process::exit(1);
    }
}
