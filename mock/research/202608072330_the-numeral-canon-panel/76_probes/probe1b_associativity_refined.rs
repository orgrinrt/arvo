// PROBE 1b. Follow-up to probe1: exhaustive u8 saturating_add turned out to
// be associative (probe1_output.txt), which refutes the naive hypothesis that
// "saturating breaks associativity" outright. That refutation is itself the
// finding worth keeping: a claimed law's fate under a strategy is an empirical
// question, not a thing to assert from familiarity with the strategy's name.
//
// This probe checks the two places saturating arithmetic is actually known to
// misbehave: signed saturation (clamped on both ends, not just the top) and
// mixed add/sub rearrangement (the associativity-like law a working consumer
// would actually reach for: (a+b)-c == a+(b-c)).

fn main() {
    // (a) i8 saturating_add, exhaustive over i8^3 (256^3 = 16,777,216 triples).
    let mut i8_counterexample: Option<(i8, i8, i8)> = None;
    let mut i8_failures: u64 = 0;
    for a in -128i16..=127 {
        for b in -128i16..=127 {
            for c in -128i16..=127 {
                let a = a as i8;
                let b = b as i8;
                let c = c as i8;
                let lhs = a.saturating_add(b).saturating_add(c);
                let rhs = a.saturating_add(b.saturating_add(c));
                if lhs != rhs {
                    i8_failures += 1;
                    if i8_counterexample.is_none() {
                        i8_counterexample = Some((a, b, c));
                    }
                }
            }
        }
    }
    println!("i8 saturating_add associativity, exhaustive (16,777,216 triples):");
    match i8_counterexample {
        None => println!("  HOLDS UNIVERSALLY."),
        Some((a, b, c)) => {
            let lhs = a.saturating_add(b).saturating_add(c);
            let rhs = a.saturating_add(b.saturating_add(c));
            println!(
                "  FAILS. counterexample a={} b={} c={} -> (a+b)+c={} a+(b+c)={}, total failures={}",
                a, b, c, lhs, rhs, i8_failures
            );
        }
    }

    // (b) u8 mixed rearrangement: (a+b)-c == a+(b-c), saturating throughout.
    // This is the law a consumer chaining a gain-then-loss actually leans on.
    let mut mixed_counterexample: Option<(u8, u8, u8)> = None;
    let mut mixed_failures: u64 = 0;
    for a in 0u8..=255 {
        for b in 0u8..=255 {
            for c in 0u8..=255 {
                let lhs = a.saturating_add(b).saturating_sub(c);
                let rhs = a.saturating_add(b.saturating_sub(c));
                if lhs != rhs {
                    mixed_failures += 1;
                    if mixed_counterexample.is_none() {
                        mixed_counterexample = Some((a, b, c));
                    }
                }
            }
        }
    }
    println!("u8 saturating (a+b)-c == a+(b-c), exhaustive (16,777,216 triples):");
    match mixed_counterexample {
        None => println!("  HOLDS UNIVERSALLY."),
        Some((a, b, c)) => {
            let lhs = a.saturating_add(b).saturating_sub(c);
            let rhs = a.saturating_add(b.saturating_sub(c));
            println!(
                "  FAILS. counterexample a={} b={} c={} -> (a+b)-c={} a+(b-c)={}, total failures={} ({:.4}% of domain)",
                a, b, c, lhs, rhs, mixed_failures,
                100.0 * (mixed_failures as f64) / 16_777_216.0
            );
        }
    }

    // (c) wrapping comparison for the same mixed law, as a control: does the
    // ring semantics preserve the rearrangement that saturation breaks?
    let mut wrap_mixed_counterexample: Option<(u8, u8, u8)> = None;
    for a in 0u8..=255 {
        for b in 0u8..=255 {
            for c in 0u8..=255 {
                let lhs = a.wrapping_add(b).wrapping_sub(c);
                let rhs = a.wrapping_add(b.wrapping_sub(c));
                if lhs != rhs {
                    wrap_mixed_counterexample = Some((a, b, c));
                    break;
                }
            }
        }
    }
    println!("u8 wrapping (a+b)-c == a+(b-c), exhaustive (16,777,216 triples):");
    match wrap_mixed_counterexample {
        None => println!("  HOLDS UNIVERSALLY (ring: Z/256Z, no exceptions)."),
        Some(t) => println!("  FAILS. counterexample {:?}", t),
    }
}
