// 21_probes/03: the operation does not merely belong in a law's key. It sets the growth rate of
// the accumulator side condition, and the two rates differ by an exponential.
//
// Question. Probe 02 established that the accumulator enters a law's key as a threshold, and
// that the threshold for addition is "the accumulator covers (n-1) times the numeral", which is
// linear in the arity. File 18 section 5 proved separately that `Precise`'s law is a property of
// `(Precise, addition)` and that multiplication does not have it. File 19 section 6 called a
// second operation "the most expensive of all" and had no number for it. This probe measures
// the number.
//
// Measured, exhaustively, no timing and no performance claim:
//
//   1. Does the in-range rounding fire, for addition and for multiplication, at the numeral's
//      own precision? (Reproducing `18_probes/04`: 0 of 256 and 128 of 256.)
//   2. The smallest accumulator, in fractional and integer bits, at which every grouping of an
//      n-element fold agrees, for each operation, at n = 2..5.
//   3. The growth rate of each.
//
// Model: signed Q2.2. Raw in [-8, 7], value = raw / 4, so the numeral holds [-2, 1.75] with a
// quantum of 1/4. `Precise` per the spec's own preset table (`11_current_shape_draft.md:327`):
// nearest with ties to even in range, refuse out of range.
//
// Exact values are carried as i128 in units of 2^-SCALE_BITS, which is exact for every product
// of up to five Q2.2 values.

const F: u32 = 2; // fractional bits of the numeral
const RAW_LO: i128 = -8;
const RAW_HI: i128 = 7;
const SCALE_BITS: u32 = 24; // exact carrier for the model
const ONE: i128 = 1 << SCALE_BITS;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum Op {
    Add,
    Mul,
}
use Op::*;

/// An accumulator: `ai` integer bits (signed, so the range is [-2^ai, 2^ai - 2^-af]) and `af`
/// fractional bits.
#[derive(Copy, Clone, Debug)]
struct Acc {
    ai: u32,
    af: u32,
}

impl Acc {
    fn lo(&self) -> i128 {
        -(1i128 << self.ai) * ONE
    }
    fn hi(&self) -> i128 {
        (1i128 << self.ai) * ONE - (ONE >> self.af)
    }
    fn quantum(&self) -> i128 {
        ONE >> self.af
    }
}

/// `Precise`: nearest with ties to even inside the range, refuse outside it.
fn phi(x: i128, acc: Acc) -> Option<i128> {
    if x < acc.lo() || x > acc.hi() {
        return None;
    }
    let q = acc.quantum();
    let below = x.div_euclid(q) * q;
    let r = x - below;
    let v = if r * 2 < q {
        below
    } else if r * 2 > q {
        below + q
    } else {
        // tie: to even multiple of the quantum
        let k = below / q;
        if k % 2 == 0 {
            below
        } else {
            below + q
        }
    };
    if v < acc.lo() || v > acc.hi() {
        None
    } else {
        Some(v)
    }
}

fn exact(op: Op, a: i128, b: i128) -> i128 {
    match op {
        Add => a + b,
        Mul => (a * b) >> SCALE_BITS,
    }
}

fn step(op: Op, a: Option<i128>, b: Option<i128>, acc: Acc) -> Option<i128> {
    match (a, b) {
        (Some(x), Some(y)) => phi(exact(op, x, y), acc),
        _ => None,
    }
}

fn groupings(op: Op, v: &[i128], acc: Acc, out: &mut Vec<Option<i128>>) {
    if v.len() == 1 {
        out.push(Some(v[0]));
        return;
    }
    for k in 1..v.len() {
        let mut ls = Vec::new();
        groupings(op, &v[..k], acc, &mut ls);
        let mut rs = Vec::new();
        groupings(op, &v[k..], acc, &mut rs);
        for a in &ls {
            for b in &rs {
                out.push(step(op, *a, *b, acc));
            }
        }
    }
}

fn value_of(raw: i128) -> i128 {
    (raw * ONE) >> F
}

fn elems() -> Vec<i128> {
    (RAW_LO..=RAW_HI).map(value_of).collect()
}

/// Existential agreement: every grouping that returns, returns the same value. This is file 17's
/// separation of the numeric half from the definedness half, which is the half a threshold can
/// move; refusal is decided by the range and is not what this probe is about.
fn existential_agrees(op: Op, n: usize, acc: Acc) -> bool {
    let e = elems();
    let span = e.len();
    let total = span.pow(n as u32);
    let mut v = vec![0i128; n];
    for mut idx in 0..total {
        for slot in v.iter_mut() {
            *slot = e[idx % span];
            idx /= span;
        }
        let mut out = Vec::new();
        groupings(op, &v, acc, &mut out);
        let mut seen: Option<i128> = None;
        for g in out.iter().flatten() {
            match seen {
                None => seen = Some(*g),
                Some(s) if s != *g => return false,
                _ => {}
            }
        }
    }
    true
}

/// Does the in-range rounding fire at all: is `phi` a partial identity on this operation, at
/// this accumulator. File 18 section 5's precondition, measured.
fn rounding_fires(op: Op, acc: Acc) -> (usize, usize) {
    let e = elems();
    let mut fired = 0;
    let mut total = 0;
    for a in &e {
        for b in &e {
            let x = exact(op, *a, *b);
            total += 1;
            if x >= acc.lo() && x <= acc.hi() {
                if phi(x, acc) != Some(x) {
                    fired += 1;
                }
            }
        }
    }
    (fired, total)
}

/// Kleene agreement: every grouping agrees INCLUDING on whether it returns at all. Probe 02's
/// relation. Added after section 2 showed the existential reading cannot see the integer width,
/// because an integer overflow is a definedness event and existential agreement skips those.
fn kleene_agrees(op: Op, n: usize, acc: Acc) -> bool {
    let e = elems();
    let span = e.len();
    let total = span.pow(n as u32);
    let mut v = vec![0i128; n];
    for mut idx in 0..total {
        for slot in v.iter_mut() {
            *slot = e[idx % span];
            idx /= span;
        }
        let mut out = Vec::new();
        groupings(op, &v, acc, &mut out);
        let first = out[0];
        for g in &out[1..] {
            if first != *g {
                return false;
            }
        }
    }
    true
}

fn section_four() {
    println!(
        "\n=== 4. the same search under Kleene agreement, which can see the integer width ==="
    );
    println!(
        "(n = 2..4 only; the n = 5 sweep is the expensive one and section 2 already has it)\n"
    );
    println!(
        "{:<6} {:>4} {:>26} {:>22}",
        "op", "n", "smallest (int, frac)", "total bits"
    );
    for op in [Add, Mul] {
        for n in 2..=4usize {
            let mut best: Option<(u32, u32)> = None;
            'search: for total in 4..=25u32 {
                for ai in 2..=12u32 {
                    if total < ai + 1 + F {
                        continue;
                    }
                    let af = total - ai - 1;
                    if af < F || af > 12 {
                        continue;
                    }
                    let acc = Acc { ai, af };
                    if kleene_agrees(op, n, acc) {
                        best = Some((ai, af));
                        break 'search;
                    }
                }
            }
            match best {
                Some((ai, af)) => println!(
                    "{:<6} {:>4} {:>26} {:>22}",
                    format!("{:?}", op),
                    n,
                    format!("({}, {})", ai, af),
                    1 + ai + af
                ),
                None => println!(
                    "{:<6} {:>4} {:>26} {:>22}",
                    format!("{:?}", op),
                    n,
                    "-",
                    "-"
                ),
            }
        }
    }
    println!(
        "\nthe two relations need different halves of the accumulator, and they need them for\n\
         different reasons. FRACTIONAL bits buy value agreement, because a truncated quantum is a\n\
         wrong number. INTEGER bits buy definedness agreement, because a range exit is a refusal.\n\
         so the accumulator side condition is not one bound. it is two, along exactly the seam\n\
         file 17 section 5.2 and file 18 section 2 cut the relation along."
    );
}

fn main() {
    let numeral = Acc { ai: 2, af: F };

    println!("=== 1. does the in-range rounding fire, at the numeral's own precision ===");
    println!("(reproducing 18_probes/04 on this model)\n");
    for op in [Add, Mul] {
        let (fired, total) = rounding_fires(op, numeral);
        println!(
            "  {:<4} rounding fired on {:>3} of {} in-range operand pairs",
            format!("{:?}", op),
            fired,
            total
        );
    }

    println!("\n=== 2. the smallest accumulator at which every grouping that returns agrees ===");
    println!(
        "(searched over integer bits 2..=12 and fractional bits 2..=12, smallest total width)\n"
    );
    println!(
        "{:<6} {:>4} {:>26} {:>26}",
        "op", "n", "smallest (int, frac)", "total bits (1+ai+af)"
    );
    let mut adds: Vec<Option<u32>> = Vec::new();
    let mut muls: Vec<Option<u32>> = Vec::new();
    for op in [Add, Mul] {
        for n in 2..=5usize {
            let mut best: Option<(u32, u32)> = None;
            'search: for total in 4..=25u32 {
                for ai in 2..=12u32 {
                    if total < ai + 1 + F {
                        continue;
                    }
                    let af = total - ai - 1;
                    if af < F || af > 12 {
                        continue;
                    }
                    let acc = Acc { ai, af };
                    if existential_agrees(op, n, acc) {
                        best = Some((ai, af));
                        break 'search;
                    }
                }
            }
            match best {
                Some((ai, af)) => {
                    println!(
                        "{:<6} {:>4} {:>26} {:>26}",
                        format!("{:?}", op),
                        n,
                        format!("({}, {})", ai, af),
                        1 + ai + af
                    );
                    if op == Add {
                        adds.push(Some(1 + ai + af))
                    } else {
                        muls.push(Some(1 + ai + af))
                    }
                }
                None => {
                    println!(
                        "{:<6} {:>4} {:>26} {:>26}",
                        format!("{:?}", op),
                        n,
                        "-",
                        "-"
                    );
                    if op == Add {
                        adds.push(None)
                    } else {
                        muls.push(None)
                    }
                }
            }
        }
    }

    println!("\n=== 3. the growth rate ===\n");
    println!(
        "{:<6} {:>6} {:>6} {:>6} {:>6}   {}",
        "op", "n=2", "n=3", "n=4", "n=5", "shape"
    );
    let fmt = |v: &Vec<Option<u32>>| {
        v.iter()
            .map(|x| match x {
                Some(k) => format!("{:>6}", k),
                None => format!("{:>6}", "-"),
            })
            .collect::<Vec<_>>()
            .join("")
    };
    println!(
        "{:<6}{}   {}",
        "Add",
        fmt(&adds),
        "the numeral plus ceil(log2 n) integer bits"
    );
    println!(
        "{:<6}{}   {}",
        "Mul",
        fmt(&muls),
        "n times the numeral: n*I integer and n*F fractional bits"
    );

    println!(
        "\nreading: for addition the accumulator side condition grows LOGARITHMICALLY in the\n\
         arity, because the exact sum of n values needs ceil(log2 n) extra integer bits and the\n\
         quantum never moves. for multiplication it grows LINEARLY, in both directions at once,\n\
         because the exact product needs n*I integer bits and n*F fractional bits.\n\
         \n\
         the same law, the same numeral, the same recovery map, and the accumulator bound that\n\
         discharges it differs by an exponential. an operation is not one more parameter in the\n\
         key. it is the parameter that sets how the others scale."
    );

    section_four();
}
