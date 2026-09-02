// Probe C. How much a forward-only width rule over-provisions, exactly, in bits.
//
// A per-operation typing rule assigns an intermediate the width its operands
// imply. That is the only rule available to an operation, because at the moment
// it is typed nothing downstream has been seen. A chain-level rule may also
// propagate the sink's demand backward and take the smaller of the two.
//
// This probe counts the difference in bits over a set of chains. It is an exact
// static count, not a timing: no magnitude of speed or size is claimed anywhere.
//
// The backward rule uses exactly the operator partition probe B established
// empirically: an operator that is a congruence modulo 2^d passes a demand of d
// bits through to its operands; one that is not passes the full width.
//
// THE CASE THAT MUST FAIL.
//   NC10 When the sink demands every bit the forward rule produced, the two
//        assignments must be identical and the saving must be exactly zero. A
//        nonzero saving there means the backward rule is unsound rather than
//        tighter.
//   NC11 At least one chain must save zero and at least one must save more than
//        zero. A probe whose every chain saves is a probe whose chains were
//        chosen, and one whose every chain saves nothing has shown nothing.

#[derive(Clone, Copy, Debug, PartialEq)]
enum Op {
    Leaf(u32),
    Add(usize, usize),
    Mul(usize, usize),
    Shl(usize, u32),
    And(usize, usize),
    // Not congruences mod 2^d: they block backward demand.
    Shr(usize, u32),
    Div(usize, usize),
    Min(usize, usize),
}

impl Op {
    /// Does a demand of `d` low bits on the result reduce to a demand of `d` low
    /// bits on each operand? Established empirically by probe B for the first
    /// group and refuted there for the second.
    fn passes_demand(self) -> bool {
        matches!(
            self,
            Op::Add(..) | Op::Mul(..) | Op::And(..) | Op::Shl(..) | Op::Leaf(_)
        )
    }
    fn operands(self) -> Vec<usize> {
        match self {
            Op::Leaf(_) => vec![],
            Op::Add(a, b) | Op::Mul(a, b) | Op::And(a, b) | Op::Div(a, b) | Op::Min(a, b) => {
                vec![a, b]
            }
            Op::Shl(a, _) | Op::Shr(a, _) => vec![a],
        }
    }
}

struct Chain {
    name: &'static str,
    nodes: Vec<Op>,
    sink: usize,
    /// Bits the consumer keeps. `None` means it keeps everything, which is NC10.
    keeps: Option<u32>,
}

fn forward(nodes: &[Op]) -> Vec<u32> {
    let mut w = vec![0u32; nodes.len()];
    for (i, n) in nodes.iter().enumerate() {
        w[i] = match *n {
            Op::Leaf(b) => b,
            Op::Add(a, b) => w[a].max(w[b]) + 1,
            Op::Mul(a, b) => w[a] + w[b],
            Op::Shl(a, k) => w[a] + k,
            Op::And(a, b) => w[a].min(w[b]),
            Op::Shr(a, k) => w[a].saturating_sub(k).max(1),
            Op::Div(a, _) => w[a],
            Op::Min(a, b) => w[a].min(w[b]),
        };
    }
    w
}

/// Backward demand propagation. Returns the demanded width of every node.
fn backward(nodes: &[Op], fwd: &[u32], sink: usize, keeps: u32) -> Vec<u32> {
    let mut dem = vec![0u32; nodes.len()];
    dem[sink] = keeps.min(fwd[sink]);
    // Nodes are in topological order by construction, so one reverse pass suffices.
    for i in (0..nodes.len()).rev() {
        let n = nodes[i];
        let d = dem[i];
        if d == 0 {
            continue;
        }
        let pass = if n.passes_demand() { d } else { u32::MAX };
        for o in n.operands() {
            let give = pass.min(fwd[o]);
            if give > dem[o] {
                dem[o] = give;
            }
        }
    }
    dem
}

fn main() {
    use Op::*;

    let chains = vec![
        // A four-term fixed-point multiply-accumulate over 16-bit inputs, result
        // stored back into a 16-bit column.
        Chain {
            name: "MAC x4, 16-bit inputs, 16-bit sink",
            nodes: vec![
                Leaf(16),
                Leaf(16),
                Leaf(16),
                Leaf(16),
                Leaf(16),
                Leaf(16),
                Leaf(16),
                Leaf(16),
                Mul(0, 1),
                Mul(2, 3),
                Mul(4, 5),
                Mul(6, 7),
                Add(8, 9),
                Add(10, 11),
                Add(12, 13),
            ],
            sink: 14,
            keeps: Some(16),
        },
        // The identical chain with the consumer keeping everything: NC10.
        Chain {
            name: "MAC x4, same chain, consumer keeps everything (NC10)",
            nodes: vec![
                Leaf(16),
                Leaf(16),
                Leaf(16),
                Leaf(16),
                Leaf(16),
                Leaf(16),
                Leaf(16),
                Leaf(16),
                Mul(0, 1),
                Mul(2, 3),
                Mul(4, 5),
                Mul(6, 7),
                Add(8, 9),
                Add(10, 11),
                Add(12, 13),
            ],
            sink: 14,
            keeps: None,
        },
        // Horner evaluation of a degree-4 polynomial, 12-bit coefficients,
        // 12-bit sink.
        Chain {
            name: "Horner degree 4, 12-bit, 12-bit sink",
            nodes: vec![
                Leaf(12),
                Leaf(12),
                Leaf(12),
                Leaf(12),
                Leaf(12),
                Leaf(12),
                Mul(0, 1),
                Add(6, 2),
                Mul(7, 1),
                Add(8, 3),
                Mul(9, 1),
                Add(10, 4),
                Mul(11, 1),
                Add(12, 5),
            ],
            sink: 13,
            keeps: Some(12),
        },
        // The same Horner chain with one right shift in the middle, which blocks
        // the demand from reaching anything above it.
        Chain {
            name: "Horner degree 4 with a shift in the middle, 12-bit sink",
            nodes: vec![
                Leaf(12),
                Leaf(12),
                Leaf(12),
                Leaf(12),
                Leaf(12),
                Leaf(12),
                Mul(0, 1),
                Add(6, 2),
                Mul(7, 1),
                Shr(8, 4),
                Add(9, 3),
                Mul(10, 1),
                Add(11, 4),
                Mul(12, 1),
                Add(13, 5),
            ],
            sink: 14,
            keeps: Some(12),
        },
        // A chain whose every operator blocks the demand: nothing can narrow.
        Chain {
            name: "all-blocking chain (div and min), 8-bit sink",
            nodes: vec![
                Leaf(24),
                Leaf(24),
                Leaf(24),
                Div(0, 1),
                Min(3, 2),
                Div(4, 1),
            ],
            sink: 5,
            keeps: Some(8),
        },
    ];

    println!("== Forward-only width assignment against forward-plus-backward ==");
    println!(
        "{:>52}  {:>10}  {:>10}  {:>8}  {:>8}",
        "chain", "fwd bits", "both bits", "saved", "pct"
    );
    let mut zero_savers = 0;
    let mut positive_savers = 0;
    let mut nc10_ok = true;
    for c in &chains {
        let fwd = forward(&c.nodes);
        let keeps = c.keeps.unwrap_or(fwd[c.sink]);
        let dem = backward(&c.nodes, &fwd, c.sink, keeps);
        // Only intermediates count; leaves are given by the consumer.
        let mut fsum = 0u32;
        let mut bsum = 0u32;
        for (i, n) in c.nodes.iter().enumerate() {
            if matches!(n, Op::Leaf(_)) {
                continue;
            }
            fsum += fwd[i];
            bsum += dem[i].max(1).min(fwd[i]);
        }
        let saved = fsum - bsum;
        let pct = (saved as f64) * 100.0 / (fsum as f64);
        if saved == 0 {
            zero_savers += 1;
        } else {
            positive_savers += 1;
        }
        if c.keeps.is_none() && saved != 0 {
            nc10_ok = false;
        }
        println!(
            "{:>52}  {:>10}  {:>10}  {:>8}  {:>7.1}%",
            c.name, fsum, bsum, saved, pct
        );
    }
    println!();
    println!(
        "  NC10 (consumer keeps everything => saving must be exactly 0): {}",
        if nc10_ok { "ok" } else { "FAIL" }
    );
    println!(
        "  NC11 (chains saving zero = {zero_savers}, chains saving more = {positive_savers}; both must be > 0): {}",
        if zero_savers > 0 && positive_savers > 0 { "ok" } else { "FAIL" }
    );
}
