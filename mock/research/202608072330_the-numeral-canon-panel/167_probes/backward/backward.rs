// Probe B. Backward information: whether the width an intermediate needs is a
// function of what produced it, or also of what consumes it.
//
// A per-operation typing rule fixes an operation's result type from its operand
// types. It is a forward rule and it is complete for that job. This probe asks
// whether the forward rule is sufficient to choose an intermediate's width, by
// exhibiting a fact about the CONSUMER that changes the answer.
//
// The fact: if the chain's result is finally truncated to K bits, then for any
// operator that is a congruence modulo 2^K, only the low K bits of every operand
// and every intermediate can affect the answer. The whole chain may then be
// computed in K bits rather than in W. That is a backward dataflow fact and the
// forward rule cannot express it, because at the moment the intermediate is
// typed, its consumer has not been seen.
//
// THE CASE THAT MUST FAIL, and it is the point of the probe rather than a
// formality: the narrowing is NOT universally licensed. Every operator that is
// not a congruence mod 2^K must show a disagreement. If the "narrow" arm agreed
// with "full" for right-shift, division, or saturating addition, this probe
// would be measuring nothing, because it would agree for every operator and
// could not distinguish a licensed rewrite from an unlicensed one.
//
// Exhaustive over the full input domain at the stated widths. No timing.

fn mask(k: u32) -> u64 {
    if k >= 64 {
        u64::MAX
    } else {
        (1u64 << k) - 1
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Op {
    Add,
    Sub,
    Mul,
    Shr1,
    Div,
    SatAdd,
    Min,
    And,
}

impl Op {
    fn name(self) -> &'static str {
        match self {
            Op::Add => "wrapping +",
            Op::Sub => "wrapping -",
            Op::Mul => "wrapping *",
            Op::Shr1 => "x >> 1 then + y",
            Op::Div => "/ (y|1)",
            Op::SatAdd => "saturating + at width",
            Op::Min => "min",
            Op::And => "&",
        }
    }
    /// Evaluate at working width `w`, operands already reduced to `w` bits.
    fn eval(self, x: u64, y: u64, w: u32) -> u64 {
        let m = mask(w);
        match self {
            Op::Add => x.wrapping_add(y) & m,
            Op::Sub => x.wrapping_sub(y) & m,
            Op::Mul => x.wrapping_mul(y) & m,
            Op::Shr1 => ((x >> 1).wrapping_add(y)) & m,
            Op::Div => (x / (y | 1)) & m,
            Op::SatAdd => core::cmp::min(x + y, m),
            Op::Min => core::cmp::min(x, y),
            Op::And => x & y,
        }
    }
}

/// A three-step chain: t1 = op(a,b); t2 = op(t1,c); result = op(t2,a).
/// Evaluated entirely at width `w`, then truncated to `k`.
fn chain(op: Op, a: u64, b: u64, c: u64, w: u32, k: u32) -> u64 {
    let t1 = op.eval(a, b, w);
    let t2 = op.eval(t1, c, w);
    let t3 = op.eval(t2, a, w);
    t3 & mask(k)
}

fn main() {
    const W: u32 = 12; // declared working width
    const K: u32 = 6; // width the consumer finally keeps

    println!("== Backward narrowing: compute the whole chain at K instead of W ==");
    println!("   working width W = {W}, consumer keeps K = {K} low bits");
    println!("   chain: t1 = op(a,b); t2 = op(t1,c); r = op(t2,a); observed = r & mask(K)");
    println!(
        "   exhaustive over a,b,c in 0..2^{W} would be 2^{} triples; this sweeps",
        3 * W
    );
    println!("   a,b exhaustively at 2^{W} each and c over a fixed 64-value spread.\n");

    println!(
        "{:>26}  {:>14}  {:>14}  {:>10}",
        "operator", "triples", "disagreements", "licensed?"
    );

    let cs: Vec<u64> = (0..64u64).map(|i| (i * 61) & mask(W)).collect();

    let mut licensed = Vec::new();
    let mut unlicensed = Vec::new();

    for op in [
        Op::Add,
        Op::Sub,
        Op::Mul,
        Op::And,
        Op::Shr1,
        Op::Div,
        Op::SatAdd,
        Op::Min,
    ] {
        let mut bad: u64 = 0;
        let mut total: u64 = 0;
        for a in 0..(1u64 << W) {
            for b in 0..(1u64 << W) {
                let c = cs[((a ^ b) & 63) as usize];
                total += 1;
                let full = chain(op, a, b, c, W, K);
                // The narrowed arm: every operand reduced to K bits up front,
                // the whole chain evaluated at width K.
                let km = mask(K);
                let narrow = chain(op, a & km, b & km, c & km, K, K);
                if full != narrow {
                    bad += 1;
                }
            }
        }
        let ok = bad == 0;
        if ok {
            licensed.push(op.name());
        } else {
            unlicensed.push((op.name(), bad));
        }
        println!(
            "{:>26}  {:>14}  {:>14}  {:>10}",
            op.name(),
            total,
            bad,
            if ok { "YES" } else { "no" }
        );
    }

    println!();
    println!("  licensed  ({}): {}", licensed.len(), licensed.join(", "));
    println!(
        "  unlicensed ({}): {}",
        unlicensed.len(),
        unlicensed
            .iter()
            .map(|(n, b)| format!("{n} [{b} disagreements]"))
            .collect::<Vec<_>>()
            .join(", ")
    );
    println!();
    println!("  NEGATIVE CONTROL: the unlicensed count must be nonzero. If every operator");
    println!("  were licensed, this probe could not tell a valid rewrite from an invalid one.");
    println!(
        "  unlicensed operators found: {}  (must be > 0)",
        unlicensed.len()
    );
}
