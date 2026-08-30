// Negative control for p1_compositional_predicate_search.rs: mutate P4's
// (no ceiling, underflow) arm from `a == 0` to `a <= 1`, a plausible-looking
// off-by-one, and confirm the sufficiency/necessity check actually bites.
fn lhs(a: u8, b: u8, c: u8) -> u8 {
    a.saturating_add(b).saturating_sub(c)
}
fn rhs(a: u8, b: u8, c: u8) -> u8 {
    a.saturating_add(b.saturating_sub(c))
}
fn holds(a: u8, b: u8, c: u8) -> bool {
    lhs(a, b, c) == rhs(a, b, c)
}

fn p4_mutant(a: u8, b: u8, c: u8) -> bool {
    let ab = a as i32 + b as i32;
    let ceiling = ab > 255;
    let underflow = b < c;
    match (ceiling, underflow) {
        (false, false) => true,
        (true, false) => c == 0,
        (false, true) => a <= 1, // MUTANT: was `a == 0`
        (true, true) => false,
    }
}

fn main() {
    let mut suff = 0u64;
    let mut nec = 0u64;
    for a in 0u16..=255 {
        for b in 0u16..=255 {
            for c in 0u16..=255 {
                let (a, b, c) = (a as u8, b as u8, c as u8);
                let pred = p4_mutant(a, b, c);
                let law = holds(a, b, c);
                if pred && !law {
                    suff += 1;
                }
                if !pred && law {
                    nec += 1;
                }
            }
        }
    }
    println!(
        "p4_mutant (a<=1 instead of a==0): sufficiency_violations={} necessity_violations={}",
        suff, nec
    );
}
