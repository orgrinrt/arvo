//! P2b. Where the algebra actually varies, after P2 refuted the first guess.
//!
//! P2 predicted that wrap, saturate and exact would give different law sets.
//! They do not. Exhaustively, at four widths, all five laws hold under all
//! three. The reason is visible once the result is in hand: collapsing every
//! value at or above the maximum onto the maximum is a semiring CONGRUENCE on
//! the naturals, and so is reduction modulo 2^W, so all three policies are
//! commutative semiring quotients of one structure and inherit its laws.
//!
//! So the overflow axis does not move the algebra, and the first hypothesis was
//! wrong. This probe finds the axis that does.
//!
//! HYPOTHESIS 2: the algebra varies with the FRACTION width and the rounding
//! that a fractional multiply forces. Multiplying two values scaled by 2^F
//! gives a product scaled by 2^2F, so the result must be shifted back by F,
//! and that shift discards bits. Discarding bits is not a congruence, so the
//! laws are not inherited, and the break should appear exactly when F > 0.
//!
//! Exhaustive over every pair and triple in the domain at each configuration.
//!
//! Run: rustc --edition 2024 -O p2b_where_the_algebra_actually_varies.rs -o /tmp/p2b && /tmp/p2b

#[derive(Copy, Clone, PartialEq, Eq)]
enum Over {
    Wrap,
    Sat,
}
#[derive(Copy, Clone, PartialEq, Eq)]
enum Round {
    Trunc,
    Nearest,
}

impl Over {
    fn name(self) -> &'static str {
        match self {
            Over::Wrap => "wrap",
            Over::Sat => "saturate",
        }
    }
}
impl Round {
    fn name(self) -> &'static str {
        match self {
            Round::Trunc => "truncate",
            Round::Nearest => "nearest",
        }
    }
}

#[derive(Copy, Clone)]
struct Fmt {
    w: u32, // total stored bits
    f: u32, // fraction bits, 0 <= f <= w
    o: Over,
    r: Round,
}

impl Fmt {
    fn m(&self) -> u128 {
        1u128 << self.w
    }
    fn max(&self) -> u128 {
        self.m() - 1
    }
    fn fit(&self, x: u128) -> u128 {
        match self.o {
            Over::Wrap => x % self.m(),
            Over::Sat => x.min(self.max()),
        }
    }
    fn add(&self, a: u128, b: u128) -> u128 {
        self.fit(a + b)
    }
    /// Fixed-point multiply: the raw product is scaled by 2^(2f) and must come
    /// back to 2^f. The shift is the whole point of this probe.
    fn mul(&self, a: u128, b: u128) -> u128 {
        let p = a * b;
        let shifted = match self.r {
            Round::Trunc => p >> self.f,
            Round::Nearest => {
                if self.f == 0 {
                    p
                } else {
                    (p + (1u128 << (self.f - 1))) >> self.f
                }
            }
        };
        self.fit(shifted)
    }
}

struct Law {
    name: &'static str,
    fail: u64,
    total: u64,
    witness: Option<(u128, u128, u128, u128, u128)>,
}

impl Law {
    fn new(name: &'static str) -> Self {
        Law {
            name,
            fail: 0,
            total: 0,
            witness: None,
        }
    }
    fn note(&mut self, ok: bool, a: u128, b: u128, c: u128, l: u128, r: u128) {
        self.total += 1;
        if !ok {
            self.fail += 1;
            if self.witness.is_none() {
                self.witness = Some((a, b, c, l, r));
            }
        }
    }
    fn line(&self) -> String {
        if self.fail == 0 {
            format!("{:<22} HOLDS  (0/{})", self.name, self.total)
        } else {
            let (a, b, c, l, r) = self.witness.unwrap();
            format!(
                "{:<22} FAILS  {}/{} = {:>5.1}%   first: a={},b={},c={} -> {} vs {}",
                self.name,
                self.fail,
                self.total,
                100.0 * self.fail as f64 / self.total as f64,
                a,
                b,
                c,
                l,
                r
            )
        }
    }
}

fn run(fmt: Fmt) -> (Law, Law, Law) {
    let n = fmt.m();
    let mut mul_assoc = Law::new("(a*b)*c == a*(b*c)");
    let mut distrib = Law::new("a*(b+c) == a*b+a*c");
    let mut add_assoc = Law::new("(a+b)+c == a+(b+c)");
    for a in 0..n {
        for b in 0..n {
            for c in 0..n {
                let l = fmt.mul(fmt.mul(a, b), c);
                let r = fmt.mul(a, fmt.mul(b, c));
                mul_assoc.note(l == r, a, b, c, l, r);

                let l = fmt.mul(a, fmt.add(b, c));
                let r = fmt.add(fmt.mul(a, b), fmt.mul(a, c));
                distrib.note(l == r, a, b, c, l, r);

                let l = fmt.add(fmt.add(a, b), c);
                let r = fmt.add(a, fmt.add(b, c));
                add_assoc.note(l == r, a, b, c, l, r);
            }
        }
    }
    (add_assoc, mul_assoc, distrib)
}

fn main() {
    println!("P2b. Which axis moves the algebra");
    println!("=================================");
    println!();
    println!("Exhaustive over every triple in the domain, per configuration.");
    println!("W is total stored bits, F is fraction bits.");
    println!();

    let w = 6u32;
    for o in [Over::Wrap, Over::Sat] {
        for r in [Round::Trunc, Round::Nearest] {
            println!(
                "--- overflow = {:<9} rounding = {:<9} W = {w} ---",
                o.name(),
                r.name()
            );
            for f in 0..=4u32 {
                let fmt = Fmt { w, f, o, r };
                let (aa, ma, di) = run(fmt);
                println!("  F = {f}");
                println!("      {}", aa.line());
                println!("      {}", ma.line());
                println!("      {}", di.line());
            }
            println!();
        }
    }

    println!("Reading:");
    println!();
    println!("Additive associativity is insensitive to every axis varied here.");
    println!("Multiplicative associativity and distributivity are insensitive to");
    println!("the OVERFLOW axis and sensitive to the FRACTION axis, breaking as");
    println!("soon as F > 0 and never at F = 0.");
    println!();
    println!("So 'the strategy changes what the correct answer is' is true, and it");
    println!("is true through one specific axis rather than through the marker as a");
    println!("whole. A strategy that fixes rounding fixes which rewrites are legal.");
    println!("A strategy that fixes overflow does not touch that question at all.");
    println!("Two things wearing one name, distinguished by a measurement.");
}
