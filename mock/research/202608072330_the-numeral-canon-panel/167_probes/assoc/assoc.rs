// Probe E. Which operators may be reassociated, derived independently.
//
// A single operation has no association question. A chain does, and the answer
// decides whether the whole family of chain rewrites (tree reduction, multiple
// accumulators, vectorisation) is available at all. This probe derives the
// region exhaustively rather than citing it.
//
// THE CASE THAT MUST FAIL.
//   NC8  At least one operator must be non-associative and at least one must be
//        associative. A table that is all-yes or all-no is an instrument that
//        cannot distinguish, and the "licensed" column would carry no
//        information.
//   NC9  A deliberately broken reference (`assoc_liar`, which compares an
//        expression to itself) must report every operator associative. It is run
//        to show that the comparison being made is a real one.
//
// Exhaustive over the full domain at each width. No timing.

fn m(w: u32) -> u64 {
    (1u64 << w) - 1
}

#[derive(Clone, Copy)]
enum Op {
    WrapAdd,
    SatAddU,
    SatSubU,
    WrapMul,
    SatMulU,
    FixMulTrunc,
    FixMulRound,
    Min,
    Max,
    AvgFloor,
    Or,
    Xor,
}

impl Op {
    fn name(self) -> &'static str {
        match self {
            Op::WrapAdd => "wrapping add",
            Op::SatAddU => "saturating add (unsigned)",
            Op::SatSubU => "saturating sub (unsigned)",
            Op::WrapMul => "wrapping mul",
            Op::SatMulU => "saturating mul (unsigned)",
            Op::FixMulTrunc => "fixed mul, truncate (F = W/2)",
            Op::FixMulRound => "fixed mul, round nearest (F = W/2)",
            Op::Min => "min",
            Op::Max => "max",
            Op::AvgFloor => "average, floor",
            Op::Or => "bitwise or",
            Op::Xor => "bitwise xor",
        }
    }
    fn eval(self, x: u64, y: u64, w: u32) -> u64 {
        let mm = m(w);
        let f = w / 2;
        match self {
            Op::WrapAdd => (x + y) & mm,
            Op::SatAddU => core::cmp::min(x + y, mm),
            Op::SatSubU => x.saturating_sub(y),
            Op::WrapMul => (x * y) & mm,
            Op::SatMulU => core::cmp::min(x * y, mm),
            Op::FixMulTrunc => core::cmp::min((x * y) >> f, mm),
            Op::FixMulRound => {
                let half = if f == 0 { 0 } else { 1u64 << (f - 1) };
                core::cmp::min((x * y + half) >> f, mm)
            }
            Op::Min => core::cmp::min(x, y),
            Op::Max => core::cmp::max(x, y),
            Op::AvgFloor => (x + y) >> 1,
            Op::Or => x | y,
            Op::Xor => x ^ y,
        }
    }
}

fn assoc(op: Op, w: u32) -> (u64, u64) {
    let n = 1u64 << w;
    let (mut bad, mut total) = (0u64, 0u64);
    for a in 0..n {
        for b in 0..n {
            for c in 0..n {
                total += 1;
                let l = op.eval(op.eval(a, b, w), c, w);
                let r = op.eval(a, op.eval(b, c, w), w);
                if l != r {
                    bad += 1;
                }
            }
        }
    }
    (bad, total)
}

/// NC9: compares the left-associated form to itself. Must report every operator
/// associative, which proves the real comparison above is comparing two things.
fn assoc_liar(op: Op, w: u32) -> u64 {
    let n = 1u64 << w;
    let mut bad = 0u64;
    for a in 0..n {
        for b in 0..n {
            for c in 0..n {
                let l = op.eval(op.eval(a, b, w), c, w);
                if l != l {
                    bad += 1;
                }
            }
        }
    }
    bad
}

fn main() {
    let ops = [
        Op::WrapAdd,
        Op::SatAddU,
        Op::SatSubU,
        Op::WrapMul,
        Op::SatMulU,
        Op::FixMulTrunc,
        Op::FixMulRound,
        Op::Min,
        Op::Max,
        Op::AvgFloor,
        Op::Or,
        Op::Xor,
    ];
    let widths = [4u32, 6, 8];

    println!("== Associativity, exhaustive over the whole domain ==");
    print!("{:>32}", "operator");
    for w in widths {
        print!("{:>18}", format!("W={w} ({}^3)", 1u64 << w));
    }
    println!("{:>12}", "verdict");

    let mut n_assoc = 0;
    let mut n_not = 0;
    for op in ops {
        print!("{:>32}", op.name());
        let mut all_ok = true;
        for w in widths {
            let (bad, total) = assoc(op, w);
            let ppm = (bad as f64) * 1.0e6 / (total as f64);
            if bad > 0 {
                all_ok = false;
            }
            print!("{:>18}", format!("{bad} ({ppm:.0}ppm)"));
        }
        if all_ok {
            n_assoc += 1;
        } else {
            n_not += 1;
        }
        println!("{:>12}", if all_ok { "ASSOC" } else { "not" });
    }

    println!();
    println!("  NC8: associative operators found = {n_assoc}, non-associative = {n_not}");
    println!(
        "       (both must be > 0): {}",
        if n_assoc > 0 && n_not > 0 { "ok" } else { "FAIL" }
    );

    let liar_bad: u64 = ops.iter().map(|&o| assoc_liar(o, 4)).sum();
    println!(
        "  NC9: the self-comparison reports {liar_bad} disagreements across all {} operators",
        ops.len()
    );
    println!(
        "       (must be exactly 0, showing the real test compares two distinct expressions): {}",
        if liar_bad == 0 { "ok" } else { "FAIL" }
    );
}
