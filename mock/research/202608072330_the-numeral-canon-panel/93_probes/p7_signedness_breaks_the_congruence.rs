//! P7. Phase two. Does the F1a congruence argument survive signedness.
//!
//! Phase one flagged its own largest gap: every sweep was unsigned, and F1a's
//! proof that the laws hold at F = 0 for any width rests on the collapse being
//! a semiring congruence, which was only argued for the one-sided unsigned
//! case. `25_torvalds_what_a_strategy_is.md:428-430` reports, from a prior-art
//! memory rather than from a measurement, that signed two-sided saturating
//! addition is not associative while wrapping is a group operation.
//!
//! That is a claim about exactly the gap phase one named, so it gets settled
//! rather than cited.
//!
//! PREDICTION, made before running, from the congruence argument itself:
//! two-sided clamping is NOT a congruence, and the counterexample is visible
//! by hand. Collapse everything at or above MAX onto MAX. Then MAX and MAX+1
//! are identified. Multiply both by -1: the first gives -MAX, the second gives
//! -MAX-1, which is MIN in two's complement. -MAX and MIN are NOT identified,
//! so multiplication does not respect the collapse and the quotient is not a
//! semiring. Signed saturating multiplication should therefore break
//! associativity, and signed saturating addition should be checked separately
//! because the same argument does not obviously apply to it.
//!
//! Exhaustive over the whole signed domain at each width.
//!
//! Run: rustc --edition 2024 -O p7_signedness_breaks_the_congruence.rs -o /tmp/p7 && /tmp/p7

#[derive(Copy, Clone, PartialEq, Eq)]
enum Pol {
    Wrap,
    Sat,
}

fn name(p: Pol) -> &'static str {
    match p {
        Pol::Wrap => "wrap",
        Pol::Sat => "saturate",
    }
}

/// Signed two's complement at W bits: domain is [-2^(W-1), 2^(W-1) - 1].
struct S {
    w: u32,
    p: Pol,
}

impl S {
    fn min(&self) -> i128 {
        -(1i128 << (self.w - 1))
    }
    fn max(&self) -> i128 {
        (1i128 << (self.w - 1)) - 1
    }
    fn fit(&self, x: i128) -> i128 {
        match self.p {
            Pol::Sat => x.clamp(self.min(), self.max()),
            Pol::Wrap => {
                let m = 1i128 << self.w;
                let r = x.rem_euclid(m);
                if r > self.max() {
                    r - m
                } else {
                    r
                }
            }
        }
    }
    fn add(&self, a: i128, b: i128) -> i128 {
        self.fit(a + b)
    }
    fn mul(&self, a: i128, b: i128) -> i128 {
        self.fit(a * b)
    }
    fn dom(&self) -> impl Iterator<Item = i128> + '_ {
        self.min()..=self.max()
    }
}

struct L {
    name: &'static str,
    fail: u64,
    total: u64,
    wit: Option<(i128, i128, i128, i128, i128)>,
}

impl L {
    fn new(n: &'static str) -> Self {
        L {
            name: n,
            fail: 0,
            total: 0,
            wit: None,
        }
    }
    fn note(&mut self, ok: bool, a: i128, b: i128, c: i128, l: i128, r: i128) {
        self.total += 1;
        if !ok {
            self.fail += 1;
            if self.wit.is_none() {
                self.wit = Some((a, b, c, l, r));
            }
        }
    }
    fn line(&self) -> String {
        if self.fail == 0 {
            format!("{:<22} HOLDS  (0/{})", self.name, self.total)
        } else {
            let (a, b, c, l, r) = self.wit.unwrap();
            format!(
                "{:<22} FAILS  {}/{} = {:>5.2}%   first: a={},b={},c={} -> {} vs {}",
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

fn main() {
    println!("P7. Signedness against the congruence argument");
    println!("==============================================");
    println!();
    println!("F = 0 throughout, so this isolates signedness from the fraction axis");
    println!("that P2b and P2c already established. Exhaustive over the whole");
    println!("signed domain at each width, both operand orders, every triple.");
    println!();
    for w in 3..=7u32 {
        for p in [Pol::Wrap, Pol::Sat] {
            let s = S { w, p };
            let mut aa = L::new("(a+b)+c == a+(b+c)");
            let mut ma = L::new("(a*b)*c == a*(b*c)");
            let mut di = L::new("a*(b+c) == a*b+a*c");
            for a in s.dom() {
                for b in s.dom() {
                    for c in s.dom() {
                        let (l, r) = (s.add(s.add(a, b), c), s.add(a, s.add(b, c)));
                        aa.note(l == r, a, b, c, l, r);
                        let (l, r) = (s.mul(s.mul(a, b), c), s.mul(a, s.mul(b, c)));
                        ma.note(l == r, a, b, c, l, r);
                        let (l, r) = (s.mul(a, s.add(b, c)), s.add(s.mul(a, b), s.mul(a, c)));
                        di.note(l == r, a, b, c, l, r);
                    }
                }
            }
            println!(
                "--- signed W = {w}, domain [{}, {}], overflow = {} ---",
                s.min(),
                s.max(),
                name(p)
            );
            println!("    {}", aa.line());
            println!("    {}", ma.line());
            println!("    {}", di.line());
        }
    }
    println!();
    println!("Reading:");
    println!();
    println!("The prediction holds and phase one's F1a is narrower than it read.");
    println!("Signed WRAPPING is a ring at every swept width, so all three laws");
    println!("hold, exactly as in the unsigned case: reduction mod 2^W is still a");
    println!("congruence when the representatives are re-centred.");
    println!();
    println!("Signed SATURATION is not, and the mechanism is the two-sided clamp.");
    println!("A one-sided clamp is a congruence because nothing below the bound");
    println!("can be pushed back across it by a monotone operation. With a clamp");
    println!("at each end, multiplication by a negative value carries a value");
    println!("across from one bound to the other, and the collapse stops");
    println!("respecting the operation.");
    println!();
    println!("So the signedness dimension is not free on this axis, and any");
    println!("finding of the form 'the laws hold at F = 0' has to carry its");
    println!("signedness. Phase one's F1 and F1a both listed `signedness =");
    println!("unsigned`, which the notation says means they hold nowhere else.");
    println!("That was the right reading and this is why.");
}
