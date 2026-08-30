//! P6. What the chain clause costs structurally.
//!
//! I7 says the accuracy-weighting preset is "the most precise possible answer,
//! especially within chains and ops, not only alone". The last clause is doing
//! work that is easy to read past. A strategy whose objective is evaluated over
//! a CHAIN rather than over an operation cannot be implemented by choosing a
//! better lowering for each operation, because the per-operation optimum is not
//! the chain optimum.
//!
//! HYPOTHESIS: a chain-weighting preset forces the result type of an operation
//! to differ from its input type, so the strategy changes the SHAPE of the
//! operator surface and not only what the operator lowers to. If that is right,
//! it is the strongest structural consequence a strategy has, and it is one the
//! marker-on-a-value framing cannot express, because the marker sits on a value
//! and this decision is about a whole expression.
//!
//! Part A measures the accuracy gap between rounding at every step and rounding
//! once at the end, exhaustively over the domain, for chain lengths 2 to 5.
//!
//! Part B computes what the intermediate must be able to hold for the
//! round-once arm to be exact, which is the structural claim: if the required
//! width grows with chain length, the operator cannot be closed over its input
//! type and the surface has to open.
//!
//! Run: rustc --edition 2024 -O p6_the_chain_clause.rs -o /tmp/p6 && /tmp/p6

/// Model format: unsigned, W total bits, F fraction bits, value = raw / 2^F.
const W: u32 = 8;
const F: u32 = 4;
const N: u64 = 1 << W;

/// One multiply, rounded back to the format immediately. This is what a
/// per-operation-optimal arm does, and it is what a closed operator must do,
/// because its result has to fit the input type.
fn mul_rounded(a: u64, b: u64) -> u64 {
    let p = a * b; // scaled by 2^(2F)
    let s = (p + (1 << (F - 1))) >> F; // back to 2^F, round to nearest
    s.min(N - 1)
}

/// The same chain with no intermediate rounding: the product is kept at its
/// full scale and brought back once, at the end. This is what a chain-optimal
/// arm does, and its intermediate does not fit the input type.
fn chain_exact(vals: &[u64]) -> u128 {
    let mut acc: u128 = vals[0] as u128;
    let mut scale: u32 = F; // acc is scaled by 2^scale
    for &v in &vals[1..] {
        acc *= v as u128;
        scale += F;
    }
    // one rounding, at the end
    let half: u128 = 1 << (scale - F - 1);
    let r = (acc + half) >> (scale - F);
    r.min((N - 1) as u128)
}

fn chain_per_op(vals: &[u64]) -> u64 {
    let mut acc = vals[0];
    for &v in &vals[1..] {
        acc = mul_rounded(acc, v);
    }
    acc
}

fn main() {
    println!("P6. The chain clause");
    println!("====================");
    println!();
    println!("Model: unsigned fixed point, W = {W} total bits, F = {F} fraction bits.");
    println!("Exhaustive over the whole domain for chain length 2 and 3; uniformly");
    println!("stepped for 4 and 5, and the step is stated rather than hidden.");
    println!();
    println!("PART A. Per-operation rounding against one rounding at the end");
    println!("--------------------------------------------------------------");
    println!();
    println!(
        "  {:>5}  {:>12}  {:>10}  {:>10}  {:>12}",
        "len", "chains", "differ", "% differ", "max abs err"
    );

    for len in 2..=5usize {
        let step: u64 = match len {
            2 | 3 => 1,
            4 => 2,
            _ => 4,
        };
        let mut total: u64 = 0;
        let mut differ: u64 = 0;
        let mut maxerr: u64 = 0;
        let mut idx = std::vec![0u64; len];
        loop {
            let a = chain_per_op(&idx);
            let b = chain_exact(&idx) as u64;
            total += 1;
            if a != b {
                differ += 1;
                let e = if a > b { a - b } else { b - a };
                if e > maxerr {
                    maxerr = e;
                }
            }
            // odometer over the (stepped) domain
            let mut k = 0;
            loop {
                if k == len {
                    break;
                }
                idx[k] += step;
                if idx[k] < N {
                    break;
                }
                idx[k] = 0;
                k += 1;
            }
            if k == len {
                break;
            }
        }
        println!(
            "  {:>5}  {:>12}  {:>10}  {:>9.1}%  {:>12}",
            len,
            total,
            differ,
            100.0 * differ as f64 / total as f64,
            maxerr
        );
        if step != 1 {
            println!("       (stepped by {step})");
        }
    }

    println!();
    println!("PART B. What the intermediate has to hold");
    println!("-----------------------------------------");
    println!();
    println!("For the round-once arm to be exact, the intermediate after k");
    println!("multiplies is scaled by 2^(k*F) and its integer part can reach the");
    println!("product of k values each below 2^(W-F).");
    println!();
    println!(
        "  {:>5}  {:>18}  {:>18}  {:>22}",
        "len", "intermediate bits", "input type bits", "fits the input type?"
    );
    for len in 2..=8u32 {
        let int_bits = (W - F) * len;
        let frac_bits = F * len;
        let need = int_bits + frac_bits;
        println!(
            "  {:>5}  {:>18}  {:>18}  {:>22}",
            len,
            need,
            W,
            if need <= W { "yes" } else { "no" }
        );
    }

    println!();
    println!("Reading, and it is the structural point:");
    println!();
    println!("The required intermediate width grows LINEARLY in chain length, so");
    println!("no fixed input type can hold it past length one. A preset whose");
    println!("objective is chain accuracy therefore cannot be implemented by an");
    println!("operator that is closed over its operand type. Its multiply must");
    println!("return something wider than what it consumed, and a collapse back to");
    println!("the declared format has to be a separate, explicit step.");
    println!();
    println!("So the strategy is not only choosing a lowering for a fixed operator");
    println!("surface. It is choosing WHICH OPERATOR SURFACE EXISTS: closed for a");
    println!("preset that rounds per operation, opening into a widening tower for a");
    println!("preset that does not. Two presets, two different algebraic objects,");
    println!("and the difference shows up in the signature rather than in the");
    println!("generated code.");
    println!();
    println!("That is a consequence a marker attached to a VALUE cannot carry on");
    println!("its own, because the decision is about an EXPRESSION. Something has");
    println!("to see the chain: either the operator's result type opens and the");
    println!("collapse is written by the consumer, or a staging layer sees the");
    println!("whole expression. Both are real designs and they are not the same.");
}
