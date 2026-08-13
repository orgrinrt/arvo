//! P2. Does the policy choice change WHICH LAWS HOLD, exhaustively.
//!
//! HYPOTHESIS: I9 says "strategies are the variables that change what the
//! correct answer is". If that is more than a slogan, then changing one policy
//! choice, with everything else held fixed, must change the set of algebraic
//! laws the numeral satisfies. And the laws are what license a rewrite, so a
//! rewrite legal under one strategy is illegal under another, which is the
//! operational content of the intent.
//!
//! This is checked EXHAUSTIVELY at model widths rather than sampled: every
//! pair and every triple in the domain, for each policy, for each width.
//! There is no selection of inputs, so there is no opportunity to feed the
//! implementation only what it handles.
//!
//! Policies compared, all at the same declared width W and the same format:
//!   wrap      : result taken modulo 2^W
//!   saturate  : result clamped to [0, 2^W - 1]
//!   exact     : computed in a wider carrier and never narrowed
//!
//! Laws checked: associativity of +, commutativity of +, associativity of *,
//! distributivity of * over +, and existence of an additive identity.
//!
//! Run: rustc --edition 2024 -O p2_policy_selects_the_algebra.rs -o /tmp/p2 && /tmp/p2

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum Policy {
    Wrap,
    Saturate,
    Exact,
}

impl Policy {
    fn name(self) -> &'static str {
        match self {
            Policy::Wrap => "wrap",
            Policy::Saturate => "saturate",
            Policy::Exact => "exact",
        }
    }
}

/// The whole numeral model. `w` is the declared width in bits; values live in
/// `0 ..= 2^w - 1` except under `Exact`, where they live in the wide carrier
/// and the declared width only bounds the INPUTS.
#[derive(Copy, Clone)]
struct Num {
    w: u32,
    p: Policy,
}

impl Num {
    fn modulus(&self) -> u128 {
        1u128 << self.w
    }
    fn max(&self) -> u128 {
        self.modulus() - 1
    }
    fn add(&self, a: u128, b: u128) -> u128 {
        let s = a + b;
        match self.p {
            Policy::Wrap => s % self.modulus(),
            Policy::Saturate => s.min(self.max()),
            Policy::Exact => s,
        }
    }
    fn mul(&self, a: u128, b: u128) -> u128 {
        let s = a * b;
        match self.p {
            Policy::Wrap => s % self.modulus(),
            Policy::Saturate => s.min(self.max()),
            Policy::Exact => s,
        }
    }
    /// The input domain. Exhaustive.
    fn domain(&self) -> impl Iterator<Item = u128> + '_ {
        0..self.modulus()
    }
}

struct LawResult {
    name: &'static str,
    total: u64,
    fail: u64,
}

impl LawResult {
    fn verdict(&self) -> String {
        if self.fail == 0 {
            "HOLDS everywhere".to_string()
        } else {
            format!(
                "FAILS on {}/{} ({:.1}%)",
                self.fail,
                self.total,
                100.0 * self.fail as f64 / self.total as f64
            )
        }
    }
}

fn check(n: &Num) -> Vec<LawResult> {
    let mut add_comm = LawResult { name: "a+b == b+a", total: 0, fail: 0 };
    let mut add_assoc = LawResult { name: "(a+b)+c == a+(b+c)", total: 0, fail: 0 };
    let mut mul_assoc = LawResult { name: "(a*b)*c == a*(b*c)", total: 0, fail: 0 };
    let mut distrib = LawResult { name: "a*(b+c) == a*b + a*c", total: 0, fail: 0 };
    let mut ident = LawResult { name: "a+0 == a", total: 0, fail: 0 };

    for a in n.domain() {
        ident.total += 1;
        if n.add(a, 0) != a {
            ident.fail += 1;
        }
        for b in n.domain() {
            add_comm.total += 1;
            if n.add(a, b) != n.add(b, a) {
                add_comm.fail += 1;
            }
            for c in n.domain() {
                add_assoc.total += 1;
                if n.add(n.add(a, b), c) != n.add(a, n.add(b, c)) {
                    add_assoc.fail += 1;
                }
                mul_assoc.total += 1;
                if n.mul(n.mul(a, b), c) != n.mul(a, n.mul(b, c)) {
                    mul_assoc.fail += 1;
                }
                distrib.total += 1;
                if n.mul(a, n.add(b, c)) != n.add(n.mul(a, b), n.mul(a, c)) {
                    distrib.fail += 1;
                }
            }
        }
    }
    vec![add_comm, add_assoc, mul_assoc, distrib, ident]
}

fn main() {
    println!("P2. The policy selects the algebra");
    println!("==================================");
    println!();
    println!("Exhaustive over the whole domain at each width. No sampling.");
    println!();

    for w in [3u32, 4, 5, 6] {
        println!("--- declared width W = {w} ({} values) ---", 1u128 << w);
        for p in [Policy::Wrap, Policy::Saturate, Policy::Exact] {
            let n = Num { w, p };
            let rs = check(&n);
            println!("  policy = {}", p.name());
            for r in rs {
                println!("      {:<24} {}", r.name, r.verdict());
            }
        }
        println!();
    }

    println!("What this says about a rewrite:");
    println!();
    println!("Reassociating a sum, or fusing a*b + a*c into a*(b+c), is licensed by");
    println!("a law. The law is a property of the POLICY, not of the operation and");
    println!("not of the width. So the same source expression admits a fused kernel");
    println!("under one policy and does not under another, with everything else");
    println!("held identical. That is I9 stated as something checkable rather than");
    println!("as a slogan: the strategy is what makes an answer correct, because it");
    println!("is what decides which rewrites preserve the answer.");
}
