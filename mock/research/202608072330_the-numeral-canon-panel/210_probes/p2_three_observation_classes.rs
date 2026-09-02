// p2: does the carrier separate a shipped-shaped pair, and through which
// observation?
//
// WHY THIS RUNS. `156` item 7 and `163` section 3 both ask "is footprint
// observable" as one question with one answer. It is not one observation. A
// declared numeral can be looked at in three ways and this probe asks each of
// them separately of one pair, so the answer stops being a single bit.
//
// THE PAIR is the one the shipped rule creates and `163` built its model on:
// one declared width, one value set, one realisation map, two carriers.
// `warm-container-shared/src/lib.rs:5-11` records the rule: `Warm` and
// `Precise` take `rung(rung_bits(W)+1)`, `Hot` and `Cold` take the minimum. At
// W = 13 that is `u32` against `u16`.
//
// THE THREE OBSERVATIONS.
//   SIGMA_SIZE  size_of / align_of. Nullary: a function of the type, never of
//               a value. Supplied by the host language, not by arvo.
//   SIGMA_BITS  the encoding. MATLAB `bin` and `storedInteger`, IEEE 754's
//               interchange read/write, `to_le_bytes`, packing, transmute.
//   SIGMA_VAL   arithmetic, comparison, conversion.
//
// WHAT MUST FAIL, declared before the run. A probe whose every arm can only
// report "same" has measured nothing, so three arms are built that MUST
// separate and the run is void if any of them reports agreement.
//   C1  an encoding stated over the CONTAINER rather than over W must
//       separate. Without it, SIGMA_BITS reporting agreement is a dead branch.
//   C2  arithmetic with the projection back to W OMITTED must separate.
//       Without it, SIGMA_VAL reporting agreement is a property of this
//       probe's arithmetic rather than of the projection discipline, and the
//       finding would be a measurement instead of an obligation.
//   C3  a genuinely different declared width (13 against 14) must separate
//       under both SIGMA_BITS and SIGMA_VAL. Without it the comparators are
//       stuck reporting equality.
//
// SCOPE. W = 13, F = 0, unsigned, wrapping and saturating, containers u16 and
// u32, add / mul / xor, arity 2, chain length 1 and 4, threads = 1. Exhaustive
// over the 8192-value domain, and over all 8192 x 8192 ordered pairs for the
// binary arms.

const W: u32 = 13;
const N: u32 = 1 << W; // 8192 declared values
const MASK: u64 = (1u64 << W) - 1;

// ---------------------------------------------------------------------------
// The two carriers. Distinct types, so `size_of` is a real observation rather
// than a number this probe writes down.
// ---------------------------------------------------------------------------

/// The minimum native container that holds W bits: `Hot` and `Cold`.
type CMin = u16;
/// `rung(rung_bits(W) + 1)`, the shipped `Warm` / `Precise` container.
type CHead = u32;

// ---------------------------------------------------------------------------
// SIGMA_BITS, stated two ways.
// ---------------------------------------------------------------------------

/// The encoding stated over the DECLARED WIDTH. This is what MATLAB `bin`
/// returns (exactly WordLength characters) and what an IEEE 754 interchange
/// read/write moves (exactly k bits).
fn enc_over_declared_width(v: u64) -> Vec<u8> {
    let mut bits = Vec::with_capacity(W as usize);
    for i in (0..W).rev() {
        bits.push(b'0' + ((v >> i) & 1) as u8);
    }
    bits
}

/// The encoding stated over the CONTAINER. This is the arm that must separate.
fn enc_over_container_min(v: u64) -> Vec<u8> {
    (v as CMin).to_le_bytes().to_vec()
}
fn enc_over_container_head(v: u64) -> Vec<u8> {
    (v as CHead).to_le_bytes().to_vec()
}

// ---------------------------------------------------------------------------
// SIGMA_VAL. One kernel, two carriers, projection on or off.
//
// The projection is the reduction back to W after every operation. A container
// of C > W wraps at C, so without the projection the two carriers compute
// different functions; with it they compute the same one. That is the whole of
// C2.
// ---------------------------------------------------------------------------

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Op {
    Add,
    Mul,
    Xor,
}
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Policy {
    Wrap,
    Sat,
}

fn step_min(a: CMin, b: CMin, op: Op, pol: Policy, project: bool) -> u64 {
    let r: u64 = match (op, pol) {
        (Op::Add, Policy::Wrap) => (a as u64).wrapping_add(b as u64) & (CMin::MAX as u64),
        (Op::Add, Policy::Sat) => (a as u64 + b as u64).min(CMin::MAX as u64),
        (Op::Mul, Policy::Wrap) => (a as u64).wrapping_mul(b as u64) & (CMin::MAX as u64),
        (Op::Mul, Policy::Sat) => (a as u64 * b as u64).min(CMin::MAX as u64),
        (Op::Xor, _) => (a ^ b) as u64,
    };
    project_to_w(r, pol, project)
}

fn step_head(a: CHead, b: CHead, op: Op, pol: Policy, project: bool) -> u64 {
    let r: u64 = match (op, pol) {
        (Op::Add, Policy::Wrap) => (a as u64).wrapping_add(b as u64) & (CHead::MAX as u64),
        (Op::Add, Policy::Sat) => (a as u64 + b as u64).min(CHead::MAX as u64),
        (Op::Mul, Policy::Wrap) => (a as u64).wrapping_mul(b as u64) & (CHead::MAX as u64),
        (Op::Mul, Policy::Sat) => (a as u64 * b as u64).min(CHead::MAX as u64),
        (Op::Xor, _) => (a ^ b) as u64,
    };
    project_to_w(r, pol, project)
}

fn project_to_w(r: u64, pol: Policy, project: bool) -> u64 {
    if !project {
        return r;
    }
    match pol {
        Policy::Wrap => r & MASK,
        Policy::Sat => r.min(MASK),
    }
}

/// A chain of `d` steps against a fixed operand, which is where an omitted
/// projection compounds rather than cancelling.
fn chain_min(x: u64, k: u64, d: usize, op: Op, pol: Policy, project: bool) -> u64 {
    let mut v = x;
    for _ in 0..d {
        v = step_min(v as CMin, k as CMin, op, pol, project);
    }
    v
}
fn chain_head(x: u64, k: u64, d: usize, op: Op, pol: Policy, project: bool) -> u64 {
    let mut v = x;
    for _ in 0..d {
        v = step_head(v as CHead, k as CHead, op, pol, project);
    }
    v
}

// ---------------------------------------------------------------------------
// Comparators. Each returns the first separating input, or None.
// ---------------------------------------------------------------------------

fn sigma_val_binary(op: Op, pol: Policy, project: bool) -> Option<(u64, u64)> {
    for a in 0..N as u64 {
        for b in 0..N as u64 {
            let m = step_min(a as CMin, b as CMin, op, pol, project);
            let h = step_head(a as CHead, b as CHead, op, pol, project);
            if m != h {
                return Some((a, b));
            }
        }
    }
    None
}

fn sigma_val_chain(d: usize, op: Op, pol: Policy, project: bool) -> Option<(u64, u64)> {
    for x in 0..N as u64 {
        for k in 0..N as u64 {
            if chain_min(x, k, d, op, pol, project) != chain_head(x, k, d, op, pol, project) {
                return Some((x, k));
            }
        }
    }
    None
}

fn sigma_bits_over_w() -> Option<u64> {
    // Both carriers hold the same declared value, so the W-bit encoding is
    // computed from the value each carrier holds, not from a shared variable.
    for v in 0..N as u64 {
        let from_min = enc_over_declared_width((v as CMin) as u64 & MASK);
        let from_head = enc_over_declared_width((v as CHead) as u64 & MASK);
        if from_min != from_head {
            return Some(v);
        }
    }
    None
}

fn sigma_bits_over_container() -> Option<u64> {
    for v in 0..N as u64 {
        if enc_over_container_min(v) != enc_over_container_head(v) {
            return Some(v);
        }
    }
    None
}

// C3: a genuinely different declared width must separate.
fn sigma_bits_across_widths() -> Option<u64> {
    for v in 0..N as u64 {
        let at13 = {
            let mut b = Vec::new();
            for i in (0..13).rev() {
                b.push(b'0' + ((v >> i) & 1) as u8);
            }
            b
        };
        let at14 = {
            let mut b = Vec::new();
            for i in (0..14).rev() {
                b.push(b'0' + ((v >> i) & 1) as u8);
            }
            b
        };
        if at13 != at14 {
            return Some(v);
        }
    }
    None
}

fn sigma_val_across_widths() -> Option<(u64, u64)> {
    for a in 0..N as u64 {
        for b in 0..N as u64 {
            let at13 = (a.wrapping_add(b)) & ((1u64 << 13) - 1);
            let at14 = (a.wrapping_add(b)) & ((1u64 << 14) - 1);
            if at13 != at14 {
                return Some((a, b));
            }
        }
    }
    None
}

fn verdict(sep: bool, required_sep: bool) -> &'static str {
    if sep == required_sep {
        "as required"
    } else {
        "*** NOT AS REQUIRED ***"
    }
}

fn line(label: &str, what: &str, sep: bool, required: bool, witness: String) {
    println!(
        "  {:<10} {:<46} {:<10} required={:<10} {:<24} {}",
        label,
        what,
        if sep { "SEPARATES" } else { "agrees" },
        if required { "SEPARATES" } else { "agrees" },
        witness,
        verdict(sep, required)
    );
}

fn main() {
    println!("### p2. three observation classes over one shipped-shaped carrier pair");
    println!(
        "### pair: W={W}, F=0, unsigned. carrier A = {} ({} bytes), carrier B = {} ({} bytes)",
        "u16",
        std::mem::size_of::<CMin>(),
        "u32",
        std::mem::size_of::<CHead>()
    );
    println!("### domain: all {N} declared values; binary arms over all {N} x {N} ordered pairs");
    println!();

    // -- SIGMA_SIZE ---------------------------------------------------------
    println!("SIGMA_SIZE  (nullary; supplied by the host language, not by arvo)");
    let sz_sep = std::mem::size_of::<CMin>() != std::mem::size_of::<CHead>();
    let al_sep = std::mem::align_of::<CMin>() != std::mem::align_of::<CHead>();
    line(
        "S1",
        "size_of",
        sz_sep,
        true,
        format!(
            "{} vs {}",
            std::mem::size_of::<CMin>(),
            std::mem::size_of::<CHead>()
        ),
    );
    line(
        "S2",
        "align_of",
        al_sep,
        true,
        format!(
            "{} vs {}",
            std::mem::align_of::<CMin>(),
            std::mem::align_of::<CHead>()
        ),
    );
    println!();

    // -- SIGMA_BITS ---------------------------------------------------------
    println!("SIGMA_BITS  (the encoding: MATLAB bin, IEEE 754 interchange, to_le_bytes)");
    let b_w = sigma_bits_over_w();
    line(
        "B1",
        "encoding stated over the declared width W",
        b_w.is_some(),
        false,
        b_w.map_or("-".into(), |v| format!("v={v}")),
    );
    let b_c = sigma_bits_over_container();
    line(
        "C1 ctl",
        "encoding stated over the container",
        b_c.is_some(),
        true,
        b_c.map_or("-".into(), |v| format!("v={v}")),
    );
    println!();

    // -- SIGMA_VAL ----------------------------------------------------------
    println!("SIGMA_VAL   (arithmetic, with the projection back to W after every step)");
    for op in [Op::Add, Op::Mul, Op::Xor] {
        for pol in [Policy::Wrap, Policy::Sat] {
            let r = sigma_val_binary(op, pol, true);
            line(
                "V",
                &format!("{op:?}/{pol:?}, arity 2, projected"),
                r.is_some(),
                false,
                r.map_or("-".into(), |(a, b)| format!("a={a},b={b}")),
            );
        }
    }
    for d in [1usize, 4] {
        let r = sigma_val_chain(d, Op::Add, Policy::Wrap, true);
        line(
            "V",
            &format!("chain d={d}, Add/Wrap, projected"),
            r.is_some(),
            false,
            r.map_or("-".into(), |(x, k)| format!("x={x},k={k}")),
        );
    }
    println!();

    // -- C2, in two halves. -------------------------------------------------
    //
    // The first cut of this section required every unprojected arm to separate
    // and three of the four reported agreement. That was the instrument
    // failing to reach the path rather than the finding failing: over a 13-bit
    // domain at arity 2 an unprojected sum is at most 16382, which fits the
    // narrower carrier, so the two carriers cannot disagree and removing the
    // projection does not make them. The failing run is kept beside this one
    // at `p2_v1_controls_that_did_not_reach_the_path.out`.
    //
    // What it bought is a sharper statement than the one designed for. The
    // projection is not the only thing hiding the carrier: HEADROOM
    // SUFFICIENCY hides it too, and the two regions have a boundary. So the
    // arms below are split by whether the unprojected intermediate can exceed
    // the narrower carrier's range, and each half declares the verdict its own
    // region requires.
    println!("SIGMA_VAL   C2a: unprojected AND the intermediate can exceed the narrow carrier");
    for (op, pol) in [(Op::Mul, Policy::Wrap), (Op::Mul, Policy::Sat)] {
        let r = sigma_val_binary(op, pol, false);
        line(
            "C2a ctl",
            &format!("{op:?}/{pol:?}, arity 2, projection OMITTED"),
            r.is_some(),
            true,
            r.map_or("-".into(), |(a, b)| format!("a={a},b={b}")),
        );
    }
    for (d, pol) in [(8usize, Policy::Wrap), (16, Policy::Wrap), (8, Policy::Sat)] {
        let r = sigma_val_chain(d, Op::Add, pol, false);
        line(
            "C2a ctl",
            &format!("chain d={d}, Add/{pol:?}, projection OMITTED"),
            r.is_some(),
            true,
            r.map_or("-".into(), |(x, k)| format!("x={x},k={k}")),
        );
    }
    println!();

    println!("SIGMA_VAL   C2b: unprojected AND the intermediate cannot exceed it (the region)");
    for (op, pol) in [(Op::Add, Policy::Wrap), (Op::Add, Policy::Sat)] {
        let r = sigma_val_binary(op, pol, false);
        line(
            "C2b",
            &format!("{op:?}/{pol:?}, arity 2, projection OMITTED"),
            r.is_some(),
            false,
            r.map_or("-".into(), |(a, b)| format!("a={a},b={b}")),
        );
    }
    let r = sigma_val_chain(4, Op::Add, Policy::Wrap, false);
    line(
        "C2b",
        "chain d=4, Add/Wrap, projection OMITTED",
        r.is_some(),
        false,
        r.map_or("-".into(), |(x, k)| format!("x={x},k={k}")),
    );
    println!();

    println!("controls: the comparators are not stuck reporting equality");
    let c3b = sigma_bits_across_widths();
    line(
        "C3 ctl",
        "encoding at W=13 against W=14",
        c3b.is_some(),
        true,
        c3b.map_or("-".into(), |v| format!("v={v}")),
    );
    let c3v = sigma_val_across_widths();
    line(
        "C3 ctl",
        "Add/Wrap at W=13 against W=14",
        c3v.is_some(),
        true,
        c3v.map_or("-".into(), |(a, b)| format!("a={a},b={b}")),
    );
    println!();

    println!("### reading: the carrier separates the pair through SIGMA_SIZE and through");
    println!("### nothing else. C2 establishes that SIGMA_VAL's blindness is bought by the");
    println!("### projection rather than given, so it is an obligation on the design and not");
    println!("### an observation about it.");
}
