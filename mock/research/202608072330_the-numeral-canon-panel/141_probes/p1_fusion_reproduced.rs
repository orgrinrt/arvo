// p1: reproduce 139 section 4's fusion table on an independently written model,
// before refuting anything that rests on it.
//
// 139 reports that fusing a multiply-add (never reducing the intermediate
// product) changes the answer at up to 42.14% of triples, and gives a table over
// W=6, F in 0..=5, signedness x overflow. I have not read 139_probes/p2_firewall.rs.
// I wrote this from the prose description of what the two arms are, so if the two
// models disagree, that disagreement is itself the finding.
//
// MODEL. Declared width W total bits, F fraction bits. A raw integer r denotes
// r / 2^F. Unsigned range [0, 2^W). Signed range [-2^(W-1), 2^(W-1)).
//
//   stepwise(a,b,c) = reduce( reduce( shift(a*b) ) + c )
//   fused(a,b,c)    = reduce( shift(a*b) + c )
//
// The only difference is whether the reduction (wrap or saturate) is applied to
// the shifted product before c is added. That is exactly what "the intermediate
// is never rounded or reduced" means for this operation.
//
// TRUNCATION IS A DIMENSION AND I SWEEP BOTH POSITIONS, because 139's prose says
// "rounding = truncate" and Rust spells two different things that way: integer
// division truncates toward zero, arithmetic shift-right floors. They differ on
// negative products, which is precisely the half of the domain where the signed
// rows live. If only one of the two reproduces 139's numbers, that tells us which
// model 139 built, and it is a dimension neither of our predicates carried.
//
// PREDICTIONS, written before the first run:
//   Q1. unsigned wrap: exactly 0.00% at every F, both truncation modes. This is
//       provable rather than measured: reduction is mod 2^W, a ring homomorphism,
//       and shift-then-add commutes with it when the shift precedes the reduction
//       in both arms... except it does NOT precede it in the stepwise arm at F>0,
//       so I am predicting 0.00% and the reason I give is incomplete. Watch this.
//   Q2. unsigned saturating: 0.00% at every F. One-sided clamp of a monotone map.
//   Q3. signed wrap at F=0: 0.00%. No shift, so reduction commutes.
//   Q4. signed wrap at F>0: nonzero and rising with F.
//   Q5. signed saturating at F=0: nonzero. Two-sided clamp is not a congruence.
//   Q6. At least one cell will disagree with 139's number under at least one
//       truncation mode, because two independently written models of a shift on
//       negative values agreeing exactly would be luck.
//
// CONTROLS, each of which must behave a stated way or the instrument is void:
//   C1 (must be zero): a "fused" arm that is literally the stepwise arm must
//      report 0.00% in every cell. If it does not, the comparator is broken.
//   C2 (must be nonzero): a "fused" arm that clamps to the wrong bound must be
//      caught in every cell. If it is not, the comparator cannot see a difference.
//   C3 (reach): count the triples where the unreduced shifted product actually
//      leaves the declared range. Where that count is zero the cell is vacuous and
//      a 0.00% there says nothing, and the cell is printed as VACUOUS rather than
//      as agreement. This is 139's own setup-that-helps lesson applied to me.
//   C4 (reach): count the triples where the shift is inexact, so a cell at F>0
//      that never discards a bit is flagged rather than believed.
//
// Run: rustc -O -o /tmp/p1 p1_fusion_reproduced.rs && /tmp/p1

#[derive(Clone, Copy, PartialEq, Eq)]
enum Sign {
    U,
    S,
}
#[derive(Clone, Copy, PartialEq, Eq)]
enum Ovf {
    Wrap,
    Sat,
}
#[derive(Clone, Copy, PartialEq, Eq)]
enum Trunc {
    /// Integer division: truncates toward zero.
    TowardZero,
    /// Arithmetic shift right: floors.
    Floor,
}

fn lo(s: Sign, w: u32) -> i128 {
    match s {
        Sign::U => 0,
        Sign::S => -(1i128 << (w - 1)),
    }
}
fn hi(s: Sign, w: u32) -> i128 {
    match s {
        Sign::U => (1i128 << w) - 1,
        Sign::S => (1i128 << (w - 1)) - 1,
    }
}

fn reduce(v: i128, s: Sign, o: Ovf, w: u32) -> i128 {
    match o {
        Ovf::Sat => v.clamp(lo(s, w), hi(s, w)),
        Ovf::Wrap => {
            let m = 1i128 << w;
            let r = v.rem_euclid(m);
            match s {
                Sign::U => r,
                Sign::S => {
                    if r >= (1i128 << (w - 1)) {
                        r - m
                    } else {
                        r
                    }
                }
            }
        }
    }
}

fn shift(p: i128, f: u32, t: Trunc) -> i128 {
    if f == 0 {
        return p;
    }
    match t {
        Trunc::TowardZero => p / (1i128 << f),
        Trunc::Floor => p >> f,
    }
}

fn stepwise(a: i128, b: i128, c: i128, s: Sign, o: Ovf, w: u32, f: u32, t: Trunc) -> i128 {
    let p = reduce(shift(a * b, f, t), s, o, w);
    reduce(p + c, s, o, w)
}

fn fused(a: i128, b: i128, c: i128, s: Sign, o: Ovf, w: u32, f: u32, t: Trunc) -> i128 {
    reduce(shift(a * b, f, t) + c, s, o, w)
}

/// C2's arm: fuses, but clamps the high side to hi-1. Must be caught everywhere.
fn fused_wrong(a: i128, b: i128, c: i128, s: Sign, o: Ovf, w: u32, f: u32, t: Trunc) -> i128 {
    let v = shift(a * b, f, t) + c;
    match o {
        Ovf::Sat => v.clamp(lo(s, w), hi(s, w) - 1),
        Ovf::Wrap => reduce(v, s, o, w).min(hi(s, w) - 1),
    }
}

struct Cell {
    diff: u64,
    total: u64,
    /// C3: triples whose unreduced shifted product leaves the declared range.
    reach_ovf: u64,
    /// C4: triples whose shift discards a bit.
    reach_inexact: u64,
    c1: u64,
    c2: u64,
}

fn sweep(s: Sign, o: Ovf, w: u32, f: u32, t: Trunc) -> Cell {
    let (l, h) = (lo(s, w), hi(s, w));
    let mut cell = Cell {
        diff: 0,
        total: 0,
        reach_ovf: 0,
        reach_inexact: 0,
        c1: 0,
        c2: 0,
    };
    for a in l..=h {
        for b in l..=h {
            let prod = a * b;
            let sh = shift(prod, f, t);
            let inexact = f > 0 && sh * (1i128 << f) != prod;
            let out = sh < l || sh > h;
            for c in l..=h {
                cell.total += 1;
                if inexact {
                    cell.reach_inexact += 1;
                }
                if out {
                    cell.reach_ovf += 1;
                }
                let st = stepwise(a, b, c, s, o, w, f, t);
                if st != fused(a, b, c, s, o, w, f, t) {
                    cell.diff += 1;
                }
                if st != stepwise(a, b, c, s, o, w, f, t) {
                    cell.c1 += 1;
                }
                if st != fused_wrong(a, b, c, s, o, w, f, t) {
                    cell.c2 += 1;
                }
            }
        }
    }
    cell
}

fn main() {
    let w = 6u32;
    println!("p1: fusing a multiply-add, reproduced on an independent model");
    println!("W = {w}, exhaustive over all triples per cell\n");

    for t in [Trunc::TowardZero, Trunc::Floor] {
        let tn = match t {
            Trunc::TowardZero => "truncate toward zero (integer division)",
            Trunc::Floor => "floor (arithmetic shift right)",
        };
        println!("=== rounding = {tn} ===");
        println!(
            "{:<22} {:>8} {:>8} {:>8} {:>8} {:>8} {:>8}",
            "cell", "F=0", "F=1", "F=2", "F=3", "F=4", "F=5"
        );
        for s in [Sign::U, Sign::S] {
            for o in [Ovf::Wrap, Ovf::Sat] {
                let name = format!(
                    "{}, {}",
                    if s == Sign::U { "unsigned" } else { "signed" },
                    if o == Ovf::Wrap {
                        "wrapping"
                    } else {
                        "saturating"
                    }
                );
                let mut row = format!("{name:<22}");
                let mut notes: Vec<String> = Vec::new();
                for f in 0..=5u32 {
                    let cell = sweep(s, o, w, f, t);
                    let pct = 100.0 * cell.diff as f64 / cell.total as f64;
                    row.push_str(&format!(" {pct:>7.2}%"));
                    if cell.c1 != 0 {
                        notes.push(format!("F={f} C1 BROKEN ({} nonzero)", cell.c1));
                    }
                    if cell.c2 == 0 {
                        notes.push(format!("F={f} C2 TOOTHLESS (wrong arm never caught)"));
                    }
                    if cell.reach_ovf == 0 {
                        notes.push(format!("F={f} C3 VACUOUS (product never leaves range)"));
                    }
                    if f > 0 && cell.reach_inexact == 0 {
                        notes.push(format!("F={f} C4 VACUOUS (shift never inexact)"));
                    }
                }
                println!("{row}");
                for n in notes {
                    println!("    ! {n}");
                }
            }
        }
        println!();
    }

    // Reach detail for the row that carries 139's headline number.
    println!("=== reach detail, signed saturating, floor ===");
    for f in 0..=5u32 {
        let cell = sweep(Sign::S, Ovf::Sat, w, f, Trunc::Floor);
        println!(
            "F={f}: diff {:>6}/{:<6} ({:.2}%)  product-out-of-range {:>6}  shift-inexact {:>6}  wrong-arm-caught {:>6}",
            cell.diff, cell.total,
            100.0 * cell.diff as f64 / cell.total as f64,
            cell.reach_ovf, cell.reach_inexact, cell.c2
        );
    }
}
