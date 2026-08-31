//! p1b: is the access width recoverable from the carrier?
//!
//! Seat 226. p1 asked the wrong question and its own run said so. Counting
//! shared jump points measures how differently two ladders are shaped; it does
//! not answer what the ownership clause needs answered, which is whether a
//! lowering site holding the carrier can recompute the access width without
//! consulting the strategy. That is exactly the question of whether the access
//! width is a FUNCTION of the carrier:
//!
//!   access is a function of carrier  <=>  for all W1, W2:
//!       carrier(W1) == carrier(W2)  implies  access(W1) == access(W2)
//!
//! A violating pair is a proof that it is not, and one pair is enough. Two
//! declared widths sharing a carrier and needing different loads means the
//! carrier does not determine the load, so a site holding only the carrier
//! cannot produce the load and the derivation must carry it.
//!
//! The same test is run in the other direction, because two facts neither of
//! which determines the other are two independent outputs rather than one with
//! a derived companion.
//!
//! Both packing rules are run, so the answer does not depend on which one a
//! design ships.
//!
//! The cases that must fail, run and reported:
//!   C1  access := carrier is a function of the carrier by construction and
//!       must report zero violating pairs. Without it a nonzero count says
//!       nothing, because nothing shows the search can come back empty.
//!   C2  a rule that ignores the width entirely (constant u8) must also report
//!       zero, and must report a nonzero count in the reverse direction. One
//!       control that can only pass is not a control.
//!   C3  the search must find the planted pair in a rule built to violate at
//!       exactly one carrier class, and must find no others.

const NATIVE: [u32; 5] = [8, 16, 32, 64, 128];
const MAX_W: u32 = 128;

fn gcd(a: u32, b: u32) -> u32 { if b == 0 { a } else { gcd(b, a % b) } }
fn nat(bits: u32) -> u32 { NATIVE.iter().copied().find(|&n| n >= bits).unwrap_or(0) }

fn carrier(w: u32) -> u32 { nat(w) }
fn access_tight(w: u32) -> u32 { nat(((8 - gcd(w, 8)) + w).div_ceil(8) * 8) }
fn access_loose(w: u32) -> u32 { nat((w + 7).div_ceil(8) * 8) }

fn access_clone(w: u32) -> u32 { carrier(w) }          // C1
fn access_const(_w: u32) -> u32 { 8 }                   // C2
fn access_planted(w: u32) -> u32 {                      // C3: violates only in the u8 class
    if w == 5 { 64 } else { carrier(w) }
}

/// Every pair (W1, W2) with g(W1) == g(W2) and f(W1) != f(W2).
/// Empty means f is a function of g.
fn violations(f: impl Fn(u32) -> u32, g: impl Fn(u32) -> u32) -> Vec<(u32, u32, u32, u32)> {
    let mut out = Vec::new();
    for a in 1..=MAX_W {
        for b in (a + 1)..=MAX_W {
            if g(a) == g(b) && f(a) != f(b) {
                out.push((a, b, f(a), f(b)));
            }
        }
    }
    out
}

fn line(name: &str, v: &[(u32, u32, u32, u32)]) {
    match v.first() {
        Some((a, b, fa, fb)) => println!(
            "  {name:<34} NOT a function: {} pairs, first W={a} and W={b} share the divisor and need {fa} and {fb}",
            v.len()
        ),
        None => println!("  {name:<34} is a function (0 violating pairs)"),
    }
}

fn main() {
    println!("== is the access width a function of the carrier? ==");
    let t = violations(access_tight, carrier);
    let l = violations(access_loose, carrier);
    line("tight packing rule", &t);
    line("loose packing rule", &l);

    println!("\n== and the carrier, a function of the access width? ==");
    let rt = violations(carrier, access_tight);
    let rl = violations(carrier, access_loose);
    line("tight packing rule", &rt);
    line("loose packing rule", &rl);

    println!("\n== controls ==");
    let c1 = violations(access_clone, carrier);
    line("C1 access := carrier", &c1);
    let c2f = violations(access_const, carrier);
    let c2r = violations(carrier, access_const);
    line("C2 access := const u8, forward", &c2f);
    line("C2 access := const u8, reverse", &c2r);
    let c3 = violations(access_planted, carrier);
    line("C3 planted single violation", &c3);
    let c3_only_five = c3.iter().all(|&(a, b, _, _)| a == 5 || b == 5);

    println!("\n== verdict ==");
    let ok_c1 = c1.is_empty();
    let ok_c2 = c2f.is_empty() && !c2r.is_empty();
    let ok_c3 = !c3.is_empty() && c3_only_five;
    println!("  C1 a rule equal to the carrier reports zero:        {ok_c1}");
    println!("  C2 a constant rule reports zero forward, nonzero reverse: {ok_c2}");
    println!("  C3 the planted violation is found and nothing else: {ok_c3}");
    let separated = !t.is_empty() && !l.is_empty() && !rt.is_empty() && !rl.is_empty();
    println!("  neither determines the other, under both packing rules: {separated}");
    let pass = ok_c1 && ok_c2 && ok_c3 && separated;
    println!("\n  RESULT: {}", if pass {
        "carrier and access width are two independent facts of the declared width"
    } else { "INCONCLUSIVE" });
    std::process::exit(if pass { 0 } else { 1 });
}

// `nat` returns 0 for "no native container reaches this many bits". That is a
// real state rather than an error: p1b's first run panicked at W = 123, where
// the tight rule needs 136 bits of window and the widest native type is 128.
// The access ladder runs out before the carrier ladder does, which is itself a
// separation between them and is reported rather than hidden.
