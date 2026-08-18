//! p2. The intermediate carrier a chain needs is a property of the sequence,
//! not of any operation in it, so there is no per-operation answer to size.
//!
//! Claim under test: for a chain of exact operations on a declared W-bit
//! unsigned numeral, the narrowest container that holds every intermediate
//! exactly is a function of the whole sequence. Two chains built from the
//! IDENTICAL multiset of steps, differing only in order, need different
//! containers; and one step needs different widths in the two.
//!
//! Everything is verified exhaustively over the whole declared domain
//! `0..2^W` at `W = 8`, so nothing here is sampled.
//!
//! THE CASES THAT MUST FAIL / RETURN "SAME":
//!   - a pair of commuting steps must report the SAME requirement in both
//!     orders. If every pair reported a difference the instrument would be
//!     reporting order-sensitivity it had built in.
//!   - the computed width must be TIGHT: one bit narrower must actually
//!     overflow for some input. A width that is merely sufficient proves
//!     nothing about a narrowest.
//!
//! Run: rustc -O p2_the_carrier_has_no_local_answer.rs -o /tmp/p2 && /tmp/p2

const W: u32 = 8;
const DOMAIN: u128 = 1 << W;

#[derive(Clone, Copy, Debug, PartialEq)]
enum Step { AddK(u128), MulK(u128) }

fn apply(s: Step, v: u128) -> u128 {
    match s { Step::AddK(k) => v + k, Step::MulK(k) => v * k }
}

fn bits_for(v: u128) -> u32 { 128 - v.leading_zeros() }

/// The exact maximum reached at each point of the chain, and the maximum over
/// all of them, computed by running every value in the declared domain. No
/// interval reasoning: the number is observed.
fn observed_widths(steps: &[Step]) -> (Vec<u32>, u32) {
    let mut per_step = vec![0u128; steps.len()];
    for x in 0..DOMAIN {
        let mut v = x;
        for (i, &s) in steps.iter().enumerate() {
            v = apply(s, v);
            if v > per_step[i] { per_step[i] = v; }
        }
    }
    let widths: Vec<u32> = per_step.iter().map(|&m| bits_for(m)).collect();
    let peak = widths.iter().copied().max().unwrap_or(W);
    (widths, peak)
}

/// Does a container of `bits` hold every intermediate of this chain, for every
/// input in the declared domain? Observed, not derived.
fn fits(steps: &[Step], bits: u32) -> bool {
    let cap: u128 = if bits >= 128 { u128::MAX } else { (1u128 << bits) - 1 };
    for x in 0..DOMAIN {
        let mut v = x;
        for &s in steps {
            v = apply(s, v);
            if v > cap { return false; }
        }
    }
    true
}

fn report(name: &str, steps: &[Step]) -> u32 {
    let (per, peak) = observed_widths(steps);
    println!("  {name:14} steps={steps:?}");
    println!("  {:14} per-step widths = {per:?}, chain needs {peak} bits", "");
    peak
}

fn main() {
    println!("W = {W}, declared domain = 0..{DOMAIN} (exhaustive)");
    println!();

    let mul3 = Step::MulK(3);
    let add200 = Step::AddK(200);

    println!("SAME MULTISET, TWO ORDERS");
    let a = [mul3, add200];
    let b = [add200, mul3];
    let wa = report("A: *3 then +200", &a);
    let wb = report("B: +200 then *3", &b);
    assert_ne!(
        wa, wb,
        "the two orders need the same container, so this pair does not \
         demonstrate order-sensitivity and the claim is untested here"
    );
    println!("  -> A needs {wa} bits, B needs {wb} bits, from the same two steps.");
    println!();

    println!("THE SAME STEP, TWO REQUIREMENTS");
    let (pa, _) = observed_widths(&a);
    let (pb, _) = observed_widths(&b);
    // `*3` is step 0 in A and step 1 in B.
    println!("  *3 as the first step needs  {} bits", pa[0]);
    println!("  *3 as the second step needs {} bits", pb[1]);
    assert_ne!(
        pa[0], pb[1],
        "the same operation needs the same width in both positions, so there \
         IS a local answer for it and the claim fails"
    );
    println!();

    println!("CONTROL: commuting steps must report the SAME requirement");
    let c = [Step::MulK(3), Step::MulK(5)];
    let d = [Step::MulK(5), Step::MulK(3)];
    let wc = report("C: *3 then *5", &c);
    let wd = report("D: *5 then *3", &d);
    assert_eq!(
        wc, wd,
        "CONTROL FAILED: two commuting multiplications reported different \
         chain requirements, so this instrument invents order-sensitivity"
    );
    println!("  -> both need {wc} bits, as they must.");
    println!();

    println!("CONTROL: the reported width must be TIGHT");
    for (name, steps, want) in [("A", &a[..], wa), ("B", &b[..], wb), ("C", &c[..], wc)] {
        let ok = fits(steps, want);
        let narrower = fits(steps, want - 1);
        println!("  {name}: fits({want}) = {ok}, fits({}) = {narrower}", want - 1);
        assert!(ok, "the reported width does not actually hold the chain");
        assert!(
            !narrower,
            "CONTROL FAILED: {name} fits one bit narrower, so the reported \
             width is not the narrowest and 'needs' is the wrong word"
        );
    }
    println!();

    println!("HOW FAR THE GAP GOES: the same step multiset, worst and best order");
    // Four steps, all orders, same multiset.
    let steps = [Step::MulK(3), Step::AddK(200), Step::MulK(2), Step::AddK(250)];
    let mut best = u32::MAX;
    let mut worst = 0u32;
    let mut idx = [0usize, 1, 2, 3];
    let mut perms: Vec<[usize; 4]> = Vec::new();
    permute(&mut idx, 0, &mut perms);
    for p in perms.iter() {
        let seq: Vec<Step> = p.iter().map(|&i| steps[i]).collect();
        let (_, peak) = observed_widths(&seq);
        if peak < best { best = peak; }
        if peak > worst { worst = peak; }
    }
    println!("  24 orderings of the same four steps: narrowest {best} bits, widest {worst} bits");
    assert!(
        worst > best,
        "CONTROL FAILED: every ordering needs the same width, so order is not \
         a dimension of the carrier requirement after all"
    );
    println!();
    println!("RESULT: the carrier requirement is a property of the sequence. Both \
controls behaved as required.");
}

fn permute(a: &mut [usize; 4], k: usize, out: &mut Vec<[usize; 4]>) {
    if k == 4 { out.push(*a); return; }
    for i in k..4 {
        a.swap(k, i);
        permute(a, k + 1, out);
        a.swap(k, i);
    }
}
