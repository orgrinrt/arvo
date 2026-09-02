//! P8. Phase two. Q41, which no member of the derived-laws unit engaged.
//!
//! `OPTIONS.md` Q41 asks whether the strategies are partially ordered by how
//! many chain-level laws they honour. `76` offered it as an explicit falsifiable
//! candidate with `Precise` at the top and `Hot`'s honoured set a subset of it,
//! marked as its own synthesis rather than a result. `90` dropped it, `91`'s
//! check found it missing, and the register records that no later member of that
//! unit engaged it.
//!
//! It is testable exactly the way `76` said, and this probe runs the test.
//!
//! METHOD. Fix a law inventory. For each configuration of the axes that could
//! move a law (signedness, overflow policy, fraction width), decide every law
//! exhaustively over the whole domain. That gives an honoured-set per
//! configuration. Then ask whether the sets nest, which is what "a partial order
//! by how many laws are honoured" means, and report the order if they do.
//!
//! The inventory deliberately mixes two families, because the interesting
//! question is whether they agree:
//!
//!   ALGEBRAIC   the laws a rewrite needs: associativity, commutativity,
//!               distributivity, identities, and the clamp-early retraction a
//!               fold needs before it may be split.
//!   ORDER       the laws a tropical or min-plus algorithm needs: monotonicity
//!               of the operation, and absorption at the top.
//!
//! `DROPLIST.md` records a prior instance of these two families inverting across
//! the same presets, in the entry retiring the `AddAssoc` gate on the algorithm
//! crates: "associativity and the distributivity these algorithms need are
//! different, complementary laws that invert across the same presets". This
//! probe asks whether the inversion is general.
//!
//! Run: rustc --edition 2024 -O p8_q41_do_the_honoured_law_sets_nest.rs -o /tmp/p8 && /tmp/p8

use std::collections::BTreeSet;

#[derive(Copy, Clone, PartialEq, Eq)]
enum Sign {
    U,
    S,
}
#[derive(Copy, Clone, PartialEq, Eq)]
enum Pol {
    Wrap,
    Sat,
}

#[derive(Copy, Clone)]
struct Cfg {
    w: u32,
    f: u32,
    sg: Sign,
    p: Pol,
}

impl Cfg {
    fn label(&self) -> String {
        format!(
            "{}/{}/F={}",
            match self.sg {
                Sign::U => "unsigned",
                Sign::S => "signed",
            },
            match self.p {
                Pol::Wrap => "wrap",
                Pol::Sat => "saturate",
            },
            self.f
        )
    }
    fn lo(&self) -> i128 {
        match self.sg {
            Sign::U => 0,
            Sign::S => -(1i128 << (self.w - 1)),
        }
    }
    fn hi(&self) -> i128 {
        match self.sg {
            Sign::U => (1i128 << self.w) - 1,
            Sign::S => (1i128 << (self.w - 1)) - 1,
        }
    }
    fn fit(&self, x: i128) -> i128 {
        match self.p {
            Pol::Sat => x.clamp(self.lo(), self.hi()),
            Pol::Wrap => {
                let m = 1i128 << self.w;
                let r = x.rem_euclid(m);
                if r > self.hi() {
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
    /// Fixed-point multiply: the product is scaled by 2^(2F) and comes back by F.
    /// Truncation throughout; P2b already showed the rounding mode moves the
    /// failure rate and not the boundary.
    fn mul(&self, a: i128, b: i128) -> i128 {
        let p = a * b;
        let s = if p >= 0 {
            p >> self.f
        } else {
            -((-p) >> self.f)
        };
        self.fit(s)
    }
    /// The representation of one, which is what a multiplicative identity has
    /// to be tested against at F > 0.
    fn one(&self) -> i128 {
        1i128 << self.f
    }
    fn dom(&self) -> Vec<i128> {
        (self.lo()..=self.hi()).collect()
    }
}

/// Every law in the inventory, named once so the honoured-set is a set of names
/// rather than a positional tuple nobody can read.
const LAWS: [&str; 11] = [
    "add-comm",
    "add-assoc",
    "add-ident",
    "mul-comm",
    "mul-assoc",
    "mul-ident",
    "mul-zero",
    "distrib",
    "retract-add",  // clamp at each step == clamp once at the end, for a+b+c
    "monotone-add", // a <= b  implies  a+c <= b+c
    "absorb-top",   // hi + x == hi for x >= 0
];

fn honoured(c: &Cfg) -> BTreeSet<&'static str> {
    let d = c.dom();
    let mut ok = [true; 11];

    for &a in &d {
        if c.add(a, 0) != a {
            ok[2] = false;
        }
        if c.mul(a, c.one()) != a {
            ok[5] = false;
        }
        if c.mul(a, 0) != 0 {
            ok[6] = false;
        }
        if a >= 0 && c.add(c.hi(), a) != c.hi() {
            ok[10] = false;
        }
        for &b in &d {
            if c.add(a, b) != c.add(b, a) {
                ok[0] = false;
            }
            if c.mul(a, b) != c.mul(b, a) {
                ok[3] = false;
            }
            for &cc in &d {
                if c.add(c.add(a, b), cc) != c.add(a, c.add(b, cc)) {
                    ok[1] = false;
                }
                if c.mul(c.mul(a, b), cc) != c.mul(a, c.mul(b, cc)) {
                    ok[4] = false;
                }
                if c.mul(a, c.add(b, cc)) != c.add(c.mul(a, b), c.mul(a, cc)) {
                    ok[7] = false;
                }
                // retraction: fitting at every step equals fitting once at the end
                if c.add(c.add(a, b), cc) != c.fit(a + b + cc) {
                    ok[8] = false;
                }
                // monotonicity of add in its first argument
                if a <= b && c.add(a, cc) > c.add(b, cc) {
                    ok[9] = false;
                }
            }
        }
    }
    LAWS.iter()
        .enumerate()
        .filter(|(i, _)| ok[*i])
        .map(|(_, n)| *n)
        .collect()
}

fn main() {
    println!("P8. Q41: do the honoured-law sets nest");
    println!("======================================");
    println!();
    println!("Exhaustive over the whole domain at W = 5, every pair and triple.");
    println!("Law inventory, 11 laws, two families:");
    println!("  algebraic : add-comm add-assoc add-ident mul-comm mul-assoc");
    println!("              mul-ident mul-zero distrib retract-add");
    println!("  order     : monotone-add absorb-top");
    println!();

    let w = 5u32;
    let mut cfgs: Vec<Cfg> = Vec::new();
    for sg in [Sign::U, Sign::S] {
        for p in [Pol::Wrap, Pol::Sat] {
            for f in [0u32, 1] {
                cfgs.push(Cfg { w, f, sg, p });
            }
        }
    }

    let sets: Vec<(String, BTreeSet<&'static str>)> =
        cfgs.iter().map(|c| (c.label(), honoured(c))).collect();

    println!("{:<22} {:>3}  honoured", "configuration", "n");
    for (l, s) in &sets {
        let mut v: Vec<&str> = s.iter().copied().collect();
        v.sort_by_key(|n| LAWS.iter().position(|x| x == n).unwrap());
        println!("{:<22} {:>3}  {}", l, s.len(), v.join(" "));
    }

    println!();
    println!("Pairwise containment. `<` means the row's set is a strict subset of");
    println!("the column's, so the column honours strictly more.");
    println!();
    print!("{:<22}", "");
    for (l, _) in &sets {
        print!(
            " {:>10}",
            l.split('/').take(2).collect::<Vec<_>>().join("/")
        );
    }
    println!();
    let mut all_comparable = true;
    let mut incomparable: Vec<(String, String)> = Vec::new();
    for (la, sa) in &sets {
        print!("{:<22}", la);
        for (lb, sb) in &sets {
            let sym = if sa == sb {
                "="
            } else if sa.is_subset(sb) {
                "<"
            } else if sb.is_subset(sa) {
                ">"
            } else {
                all_comparable = false;
                if la < lb {
                    incomparable.push((la.clone(), lb.clone()));
                }
                "X"
            };
            print!(" {:>10}", sym);
        }
        println!();
    }

    println!();
    if all_comparable {
        println!("Every pair is comparable: the honoured sets form a CHAIN, so Q41's");
        println!("option (a) holds over this inventory and this axis set.");
    } else {
        println!("NOT every pair is comparable. Q41's option (b) holds over this");
        println!("inventory: there is a partial order and it is not a ladder.");
        println!("Incomparable pairs, which are the whole content of that answer:");
        for (a, b) in &incomparable {
            let sa = &sets.iter().find(|(l, _)| l == a).unwrap().1;
            let sb = &sets.iter().find(|(l, _)| l == b).unwrap().1;
            let only_a: Vec<&str> = sa.difference(sb).copied().collect();
            let only_b: Vec<&str> = sb.difference(sa).copied().collect();
            println!("  {a}  vs  {b}");
            println!("      only in the first : {}", only_a.join(" "));
            println!("      only in the second: {}", only_b.join(" "));
        }
    }

    println!();
    println!("The direction, which is the part that answers 76's conjecture:");
    println!();
    let by_name = |n: &str| sets.iter().find(|(l, _)| l == n).map(|(_, s)| s).unwrap();
    for f in [0u32, 1] {
        for sg in ["unsigned", "signed"] {
            let wrap = by_name(&format!("{sg}/wrap/F={f}"));
            let sat = by_name(&format!("{sg}/saturate/F={f}"));
            let rel = if wrap == sat {
                "honour exactly the same laws"
            } else if sat.is_subset(wrap) {
                "WRAPPING honours strictly more"
            } else if wrap.is_subset(sat) {
                "SATURATING honours strictly more"
            } else {
                "neither contains the other"
            };
            println!(
                "  {sg:<8} F={f}: {rel}  ({} against {})",
                wrap.len(),
                sat.len()
            );
        }
    }
    println!();
    println!("76's candidate put the accuracy-first intent at the TOP and the");
    println!("speed-first intent's honoured set inside it. The measurement above");
    println!("says that is RIGHT for unsigned and WRONG for signed, and that");
    println!("signedness is the dimension deciding which of Q41's options holds.");
    println!();
    println!("UNSIGNED. Saturating honours everything wrapping does and two laws");
    println!("more, monotone-add and absorb-top, which wrapping loses because a");
    println!("wrap carries a large value back past a small one. So the sets nest,");
    println!("saturating is on top, and Q41's option (a) holds. That is 76's");
    println!("direction, established rather than conjectured.");
    println!();
    println!("SIGNED. The sets stop nesting. Saturating still gains monotone-add");
    println!("and absorb-top, and it now LOSES add-assoc, mul-assoc, distrib and");
    println!("retract-add, because a two-sided clamp is not a semiring congruence");
    println!("while a one-sided clamp is (see p7). So the two families pull apart");
    println!("and Q41's option (b) holds: a real partial order that is not a ladder.");
    println!();
    println!("So Q41's three options are not competing answers. They are the");
    println!("answers on different regions, and the predicate separating them is");
    println!("the signedness. A single ordering over 'the strategies' does not");
    println!("exist, and the reason is nameable rather than a shrug: the algebraic");
    println!("family and the order family agree on unsigned and conflict on signed.");
    println!();
    println!("What this does not reach: 76's own phrase was chain-level ACCURACY");
    println!("facts, which are a third family not in this inventory and which order");
    println!("these configurations by how little they lose rather than by which");
    println!("identities they satisfy. Nothing here measures that family, so by the");
    println!("notation nothing here claims anything about it.");
}
