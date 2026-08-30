// PROBE 1. Hypothesis: on a fixed-width container, associativity of `+` is a
// theorem of wrapping arithmetic (it is arithmetic in Z/2^N Z, a ring) and is
// NOT a theorem of saturating arithmetic on the same container, even though
// both are candidate semantics a strategy marker could pick for the same
// declared width.
//
// This is checked EXHAUSTIVELY over the whole u8 domain (256^3 = 16,777,216
// triples), not sampled, per the workspace's own instrument against sampled
// laws. u8 stands in for "a Hot-style minimal fixed-width container" for the
// purpose of this probe; the claim generalises to any width because the ring
// argument for wrapping is width-independent and the saturating counterexample
// only needs headroom near the top of the range.
//
// Outcome recorded at the bottom of the committed run log (probe1_output.txt).

fn main() {
    // Wrapping: claim is TOTAL associativity across the whole domain.
    let mut wrapping_counterexample: Option<(u8, u8, u8)> = None;
    'outer_w: for a in 0u8..=255 {
        for b in 0u8..=255 {
            for c in 0u8..=255 {
                let lhs = a.wrapping_add(b).wrapping_add(c);
                let rhs = a.wrapping_add(b.wrapping_add(c));
                if lhs != rhs {
                    wrapping_counterexample = Some((a, b, c));
                    break 'outer_w;
                }
            }
        }
    }

    // Saturating: find the first counterexample (existence disproof of the
    // universal claim), and also count how many triples fail so we know this
    // is not a rare edge condition but a routine one.
    let mut saturating_counterexample: Option<(u8, u8, u8)> = None;
    let mut saturating_failures: u64 = 0;
    for a in 0u8..=255 {
        for b in 0u8..=255 {
            for c in 0u8..=255 {
                let lhs = a.saturating_add(b).saturating_add(c);
                let rhs = a.saturating_add(b.saturating_add(c));
                if lhs != rhs {
                    saturating_failures += 1;
                    if saturating_counterexample.is_none() {
                        saturating_counterexample = Some((a, b, c));
                    }
                }
            }
        }
    }

    println!("wrapping_add associativity, exhaustive over u8^3 (16,777,216 triples):");
    match wrapping_counterexample {
        None => println!("  HOLDS UNIVERSALLY. Zero counterexamples found."),
        Some(t) => println!("  FAILS. First counterexample: {:?}", t),
    }

    println!("saturating_add associativity, exhaustive over u8^3 (16,777,216 triples):");
    match saturating_counterexample {
        None => println!("  HOLDS UNIVERSALLY. Zero counterexamples found."),
        Some((a, b, c)) => {
            let lhs = a.saturating_add(b).saturating_add(c);
            let rhs = a.saturating_add(b.saturating_add(c));
            println!(
                "  FAILS. First counterexample: a={} b={} c={} -> (a+b)+c={} a+(b+c)={}",
                a, b, c, lhs, rhs
            );
        }
    }
    println!(
        "  total failing triples out of 16,777,216: {}",
        saturating_failures
    );
}
