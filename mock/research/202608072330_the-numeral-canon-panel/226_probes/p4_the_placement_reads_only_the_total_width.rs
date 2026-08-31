//! p4: does the placement computation move when the signedness or the fraction
//! width move?
//!
//! Seat 226. `p1b` established that the carrier and the access width are two
//! independent facts, over the total width and the container set. Its predicate
//! names those axes and no others, and under the predicate discipline an absent
//! axis says the finding holds nowhere that axis exists. Signedness and fraction
//! width exist for every numeral arvo declares, so `p1b` as it stands holds
//! nowhere, which is not what its instrument shows and not what I mean.
//!
//! `ruling::the_warrant_is_a_token_and_a_clause_on_the_values_side` gives the
//! spelling for an axis that cannot enter an argument, `construction`, and
//! obliges the row's evidence to name an instrument that varied that axis and
//! found no movement. This is that instrument. It exists to earn two entries in
//! a predicate and nothing else.
//!
//! Sweep: total width 1 to 128, fraction width 0 to W-1, signedness both. For
//! each, the carrier and both access rules are computed and compared against the
//! same total width at fraction width 0 and unsigned. Any movement is a report.
//!
//! The case that must fail, run and reported:
//!   C1  a placement rule that DOES read the two axes, adding a bit for a sign
//!       and a bit per fraction digit above eight, must be reported as moving.
//!       Without it a zero movement count says nothing, because nothing shows
//!       the comparison can ever see a difference.

const NATIVE: [u32; 5] = [8, 16, 32, 64, 128];
const MAX_W: u32 = 128;

fn gcd(a: u32, b: u32) -> u32 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}
fn nat(bits: u32) -> u32 {
    NATIVE.iter().copied().find(|&n| n >= bits).unwrap_or(0)
}

/// The three real placement rules. Each takes the whole declared signature and
/// each reads only the total width out of it.
fn carrier(w: u32, _f: u32, _s: bool) -> u32 {
    nat(w)
}
fn access_tight(w: u32, _f: u32, _s: bool) -> u32 {
    nat(((8 - gcd(w, 8)) + w).div_ceil(8) * 8)
}
fn access_loose(w: u32, _f: u32, _s: bool) -> u32 {
    nat((w + 7).div_ceil(8) * 8)
}

/// C1: a rule that reads all three coordinates.
fn carrier_reads_everything(w: u32, f: u32, s: bool) -> u32 {
    nat(w + u32::from(s) + f / 8)
}

/// Cells where `g` differs from its own value at fraction width 0, unsigned.
fn movement(g: impl Fn(u32, u32, bool) -> u32) -> (usize, usize, Option<(u32, u32, bool)>) {
    let mut moved = 0usize;
    let mut cells = 0usize;
    let mut first = None;
    for w in 1..=MAX_W {
        let base = g(w, 0, false);
        for f in 0..w {
            for s in [false, true] {
                cells += 1;
                if g(w, f, s) != base {
                    moved += 1;
                    if first.is_none() {
                        first = Some((w, f, s));
                    }
                }
            }
        }
    }
    (moved, cells, first)
}

fn line(name: &str, g: impl Fn(u32, u32, bool) -> u32) -> usize {
    let (moved, cells, first) = movement(g);
    println!("  {name:<24} moved in {moved} of {cells} cells, first {first:?}");
    moved
}

fn main() {
    println!("== the real placement rules, over the whole declared signature ==");
    let a = line("carrier", carrier);
    let b = line("access, tight rule", access_tight);
    let c = line("access, loose rule", access_loose);

    println!("\n== C1 control: a rule that reads the sign and the fraction ==");
    let d = line("carrier_reads_everything", carrier_reads_everything);

    println!("\n== verdict ==");
    let still = a == 0 && b == 0 && c == 0;
    let control_fires = d > 0;
    println!("  the three real rules move in no cell:        {still}");
    println!("  C1 the reading rule is reported as moving:   {control_fires}");
    let pass = still && control_fires;
    println!(
        "\n  RESULT: {}",
        if pass {
            "placement is a function of the total width alone; signedness any, fraction_width any"
        } else {
            "INCONCLUSIVE"
        }
    );
    std::process::exit(if pass { 0 } else { 1 });
}
