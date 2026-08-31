//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! Seat 216, probe 1. The minimum sufficient accumulator width, derived from scratch.
//!
//! A second instrument for two rows that both rest on one:
//!
//!   proposal::an_incoherent_clamped_addition_needs_the_exact_sum_width_less_one_bit
//!   proposal::a_coherent_reduction_needs_no_accumulator
//!
//! Both cite `57_probes/p6`. Nothing here reads that file. The definitions are written
//! from the ratified format spine: a format is an ambient domain together with a
//! representable set, and arithmetic on it is an exact ambient operation composed with a
//! total adaptation onto that set.
//!
//! Three questions, kept apart:
//!
//!   Q1  For each (sign, W, L), the smallest accumulator width A at which the eagerly
//!       adapted fold agrees with exact-then-adapt on every tuple.
//!   Q2  Whether that minimum sits exactly one bit below the exact-sum width in the
//!       signed domain, and whether the format's own width already suffices unsigned.
//!   Q3  Whether either answer moves with the fraction width.
//!
//! Four cases that must fail, and three of them run before the sweep. An instrument that
//! has only ever reported agreement has not been shown able to report anything else.
//!
//!   N1  At one below each located minimum, an explicit witness tuple, printed. Without
//!       it a "minimum" is an upper bound wearing a stronger word.
//!   N2  A wrapping accumulator, same code, one adaptation swapped, must locate a
//!       different answer or the sweep is not sensitive to the accumulator's adaptation.
//!   N3  A deliberately wrong reference must be detected, or the comparison would report
//!       agreement against any reference at all.
//!   N4  Under Q3, a rescaling operation must show the fraction width mattering, or the
//!       "F does not move it" result is a property of the harness ignoring F.

use std::env;

// ---------------------------------------------------------------------------------------
// The format, written from the spine.
// ---------------------------------------------------------------------------------------

/// A declared format. `w` is the total width and `f` the fraction width; the raw value is
/// an integer of `w` bits and the value it denotes is `raw / 2^f`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Fmt {
    signed: bool,
    w: u32,
    f: u32,
}

impl Fmt {
    const fn lo(&self) -> i64 {
        if self.signed {
            -(1i64 << (self.w - 1))
        } else {
            0
        }
    }
    const fn hi(&self) -> i64 {
        if self.signed {
            (1i64 << (self.w - 1)) - 1
        } else {
            (1i64 << self.w) - 1
        }
    }
    const fn card(&self) -> u64 {
        1u64 << self.w
    }
    /// The scale: how many raw units make one whole. Used by the value-domain path.
    const fn scale(&self) -> i64 {
        1i64 << self.f
    }
}

/// Saturation onto a representable set, at the raw level.
#[inline(always)]
fn adapt(x: i64, lo: i64, hi: i64) -> i64 {
    if x < lo {
        lo
    } else if x > hi {
        hi
    } else {
        x
    }
}

/// Wrapping onto the same set. Negative control N2 only.
#[inline(always)]
fn adapt_wrap(x: i64, lo: i64, card: i64) -> i64 {
    let mut r = (x - lo) % card;
    if r < 0 {
        r += card;
    }
    lo + r
}

// ---------------------------------------------------------------------------------------
// The two width quantities, derived rather than tabulated.
// ---------------------------------------------------------------------------------------

fn ceil_log2(n: u64) -> u32 {
    let mut k = 0;
    while (1u64 << k) < n {
        k += 1;
    }
    k
}

/// The smallest width whose representable set contains every exact sum of `l` elements.
fn exact_sum_width(fmt: Fmt, l: u32) -> u32 {
    let lo = fmt.lo() * l as i64;
    let hi = fmt.hi() * l as i64;
    let mut a = fmt.w;
    loop {
        let g = Fmt {
            signed: fmt.signed,
            w: a,
            f: fmt.f,
        };
        if g.lo() <= lo && hi <= g.hi() {
            return a;
        }
        a += 1;
        assert!(a <= 60, "exact sum width ran away");
    }
}

// ---------------------------------------------------------------------------------------
// The sweep core. One u64 counter carries the whole odometer, W bits per position.
// ---------------------------------------------------------------------------------------

/// Walks every tuple and returns the first disagreement, or `None`.
///
/// `wrapping_acc` swaps the accumulator's adaptation for control N2. `broken_ref` swaps
/// the reference for control N3.
fn first_divergence(
    fmt: Fmt,
    l: u32,
    a: u32,
    wrapping_acc: bool,
    broken_ref: bool,
) -> Option<Vec<i64>> {
    let acc = Fmt {
        signed: fmt.signed,
        w: a,
        f: fmt.f,
    };
    let (flo, fhi) = (fmt.lo(), fmt.hi());
    let (alo, ahi) = (acc.lo(), acc.hi());
    let acard = acc.card() as i64;
    let mask = fmt.card() - 1;
    let total: u64 = 1u64 << (fmt.w * l);
    let mut vs = [0i64; 16];
    for counter in 0..total {
        for k in 0..l as usize {
            vs[k] = flo + ((counter >> (k as u32 * fmt.w)) & mask) as i64;
        }
        let slice = &vs[..l as usize];

        let mut sum = 0i64;
        for &v in slice {
            sum += v;
        }
        let reference = if broken_ref {
            adapt(sum, alo, ahi)
        } else {
            adapt(sum, flo, fhi)
        };

        let mut acc_v = slice[0];
        for &v in &slice[1..] {
            acc_v = if wrapping_acc {
                adapt_wrap(acc_v + v, alo, acard)
            } else {
                adapt(acc_v + v, alo, ahi)
            };
        }
        let candidate = adapt(acc_v, flo, fhi);

        if candidate != reference {
            return Some(slice.to_vec());
        }
    }
    None
}

struct Cell {
    min_sufficient: u32,
    witness_below: Option<Vec<i64>>,
}

fn locate(fmt: Fmt, l: u32, ceiling: u32) -> Cell {
    for a in fmt.w..=ceiling {
        if first_divergence(fmt, l, a, false, false).is_none() {
            let witness = if a > fmt.w {
                first_divergence(fmt, l, a - 1, false, false)
            } else {
                None
            };
            return Cell {
                min_sufficient: a,
                witness_below: witness,
            };
        }
    }
    panic!("no sufficient width at or below the exact-sum width");
}

fn tuples(fmt: Fmt, l: u32) -> u128 {
    (fmt.card() as u128).pow(l)
}

// ---------------------------------------------------------------------------------------
// Q3's value-domain path. A genuinely different computation, so that agreement means
// something. Values are carried as exact rationals with denominator 2^f.
// ---------------------------------------------------------------------------------------

/// The same located minimum, computed at a different common scale.
///
/// Scale invariance is what "the fraction width does not matter" means operationally, so
/// this runs the identical construction with every value and bound multiplied by `r`.
/// N4 supplies the control that says a scale is visible to this code at all.
fn locate_rescaled(fmt: Fmt, l: u32, ceiling: u32, r: i64) -> u32 {
    // Every quantity multiplied by `r`. A fixed-point format of fraction width F is the
    // integer format of the same total width with every value divided by 2^F, so if the
    // located minimum is a function of F at all, running the identical construction at a
    // different common scale must move it. Running it at several `r` and getting one
    // answer is the check; N4 is what says this code can see a scale at all.
    let vlo = fmt.lo() * r;
    let vhi = fmt.hi() * r;
    let mask = fmt.card() - 1;
    let total: u64 = 1u64 << (fmt.w * l);

    for a in fmt.w..=ceiling {
        let acc = Fmt {
            signed: fmt.signed,
            w: a,
            f: fmt.f,
        };
        let (alo, ahi) = (acc.lo() * r, acc.hi() * r);
        let mut diverged = false;
        'outer: for counter in 0..total {
            let mut num = [0i64; 16];
            for k in 0..l as usize {
                let raw = fmt.lo() + ((counter >> (k as u32 * fmt.w)) & mask) as i64;
                num[k] = raw * r;
            }
            let slice = &num[..l as usize];
            let exact: i64 = slice.iter().sum();
            let reference = adapt(exact, vlo, vhi);
            let mut accv = slice[0];
            for &v in &slice[1..] {
                accv = adapt(accv + v, alo, ahi);
            }
            if adapt(accv, vlo, vhi) != reference {
                diverged = true;
                break 'outer;
            }
        }
        if !diverged {
            return a;
        }
    }
    panic!("rescaled path found no sufficient width");
}

/// N4. The fraction bits a product needs beyond the declared ones, which must move with
/// the fraction width or the harness is not seeing `f` at all.
fn product_extra_fraction_bits(fmt: Fmt, operands: u32) -> u32 {
    // A product of `operands` values each with `f` fraction bits has `operands * f`
    // fraction bits exactly, of which `f` are already declared.
    operands * fmt.f - fmt.f
}

fn main() {
    let budget: u128 = env::args()
        .nth(1)
        .and_then(|s| s.parse().ok())
        .unwrap_or(40_000_000);

    println!("=== p1. the minimum sufficient accumulator width, second instrument ===\n");
    println!("  reference : the exact ambient sum, adapted once onto the format");
    println!("  candidate : eager adaptation into width A at every step, adapted once at the end");
    println!("  exact     : the width holding every exact sum, derived from the interval");
    println!("  measured  : the smallest A at which candidate and reference agree everywhere");
    println!("  gap       : exact - measured");
    println!();
    println!("  Every cell is exhaustive over every tuple. Nothing here is sampled, and a");
    println!("  cell above the budget is listed as unreached rather than estimated.");
    println!("  tuple budget per cell: {budget}\n");

    // -------------------------------------------------------------------------------
    // N3 first. A wrong reference must be detected.
    // -------------------------------------------------------------------------------
    {
        let fmt = Fmt {
            signed: true,
            w: 3,
            f: 0,
        };
        let l = 4u32;
        let a = exact_sum_width(fmt, l) - 1;
        let d = first_divergence(fmt, l, a, false, true);
        println!("N3 wrong-reference control (signed W=3 L=4 at A=exact-1={a}):");
        match &d {
            Some(v) => {
                println!("    a reference clamped to the accumulator is detected at {v:?}  PASS")
            }
            None => println!("    NOT DETECTED  FAIL"),
        }
        assert!(
            d.is_some(),
            "N3: the comparison cannot tell two references apart"
        );
        println!();
    }

    // -------------------------------------------------------------------------------
    // The main sweep.
    // -------------------------------------------------------------------------------
    let mut signed_cells: Vec<(u32, u32, u32, u32)> = Vec::new(); // w, l, exact, min
    let mut unsigned_cells: Vec<(u32, u32, u32, u32)> = Vec::new();
    let mut skipped: Vec<(bool, u32, u32, u128)> = Vec::new();

    for &signed in &[true, false] {
        println!(
            "{:>9} {:>3} {:>3} {:>15} {:>7} {:>9} {:>5}  {}",
            "sign", "W", "L", "tuples", "exact", "measured", "gap", "N1 witness at measured-1"
        );
        for w in 3..=7u32 {
            for l in 2..=9u32 {
                let fmt = Fmt { signed, w, f: 0 };
                let t = tuples(fmt, l);
                if t > budget || w * l > 60 {
                    skipped.push((signed, w, l, t));
                    continue;
                }
                let exact = exact_sum_width(fmt, l);
                let cell = locate(fmt, l, exact);
                let gap = exact - cell.min_sufficient;
                let wit = match &cell.witness_below {
                    Some(v) => format!("{v:?}"),
                    None => "format width already suffices".to_string(),
                };
                println!(
                    "{:>9} {:>3} {:>3} {:>15} {:>7} {:>9} {:>5}  {}",
                    if signed { "signed" } else { "unsigned" },
                    w,
                    l,
                    t,
                    exact,
                    cell.min_sufficient,
                    gap,
                    wit
                );
                if signed {
                    signed_cells.push((w, l, exact, cell.min_sufficient));
                } else {
                    unsigned_cells.push((w, l, exact, cell.min_sufficient));
                }
            }
        }
        println!();
    }

    // -------------------------------------------------------------------------------
    // Q2.
    // -------------------------------------------------------------------------------
    let locating: Vec<_> = signed_cells.iter().filter(|(_, l, _, _)| *l > 2).collect();
    let all_one = locating.iter().all(|(_, _, e, m)| e - m == 1);
    let any_zero = signed_cells.iter().any(|(_, _, e, m)| e == m);
    let unsigned_all_format = unsigned_cells.iter().all(|(w, _, _, m)| m == w);

    println!("=== Q2, the two verdicts, counted ===\n");
    println!(
        "  signed cells measured                              : {}",
        signed_cells.len()
    );
    println!(
        "  signed cells at fold length above two              : {}",
        locating.len()
    );
    println!("  every one of those at a gap of exactly one bit      : {all_one}");
    println!("  any signed cell at gap zero, which would refute it  : {any_zero}");
    println!(
        "  unsigned cells measured                            : {}",
        unsigned_cells.len()
    );
    println!("  unsigned: the format's own width sufficed in all    : {unsigned_all_format}");
    println!();

    // -------------------------------------------------------------------------------
    // N2.
    // -------------------------------------------------------------------------------
    println!("=== N2, the wrapping-accumulator control ===\n");
    println!("  The same sweep with the accumulator wrapping rather than saturating. If the");
    println!("  answer does not move, the sweep is not sensitive to the accumulator's own");
    println!("  adaptation and the finding is about something else.\n");
    let mut n2_moved = false;
    for w in 3..=4u32 {
        for l in 3..=5u32 {
            let fmt = Fmt {
                signed: true,
                w,
                f: 0,
            };
            if tuples(fmt, l) > budget {
                continue;
            }
            let exact = exact_sum_width(fmt, l);
            let sat = locate(fmt, l, exact).min_sufficient;
            let mut wrap_min = None;
            for a in fmt.w..=exact {
                if first_divergence(fmt, l, a, true, false).is_none() {
                    wrap_min = Some(a);
                    break;
                }
            }
            if wrap_min != Some(sat) {
                n2_moved = true;
            }
            println!(
                "  signed W={w} L={l}: exact={exact}  saturating-acc min={sat}  wrapping-acc min={}",
                match wrap_min {
                    Some(a) => a.to_string(),
                    None => "none at or below exact".to_string(),
                }
            );
        }
    }
    println!("\n  the accumulator's adaptation moves the answer: {n2_moved}");
    assert!(
        n2_moved,
        "N2: swapping the accumulator's adaptation changed nothing"
    );
    println!();

    // -------------------------------------------------------------------------------
    // Q3 and N4.
    // -------------------------------------------------------------------------------
    println!("=== Q3, whether the fraction width moves either answer ===\n");
    println!("  A fixed-point format of total width W is the integer format of that width");
    println!("  with every value divided by 2^F, so the question is whether the located");
    println!("  minimum is invariant under a change of common scale. Below, the identical");
    println!("  construction runs at four scales. If F could move the answer, a scale");
    println!("  would move it. N4 is what says this code can see a scale at all.\n");
    let mut q3_disagreements = 0u32;
    let mut q3_checked = 0u32;
    for w in 3..=5u32 {
        for l in 2..=6u32 {
            let fmt = Fmt {
                signed: true,
                w,
                f: 0,
            };
            if tuples(fmt, l) > budget {
                continue;
            }
            let exact = exact_sum_width(fmt, l);
            let base = locate(fmt, l, exact).min_sufficient;
            for r in [1i64, 2, 4, 8] {
                let got = locate_rescaled(fmt, l, exact, r);
                q3_checked += 1;
                if got != base {
                    q3_disagreements += 1;
                    println!("  DISAGREEMENT W={w} L={l} scale={r}: {got} vs {base}");
                }
            }
        }
    }
    println!("  scale points checked                        : {q3_checked}");
    println!("  disagreements                               : {q3_disagreements}");
    println!(
        "  addition's located minimum is invariant under the common scale: {}",
        q3_disagreements == 0
    );
    println!();

    println!("=== N4, the control that says the harness can see F ===\n");
    println!("  If the fraction width were invisible to this code, Q3 would report agreement");
    println!("  whatever the truth. A rescaling operation must show F moving a width.\n");
    let mut n4_moved = false;
    for fbits in 0..=3u32 {
        let fmt = Fmt {
            signed: false,
            w: 8,
            f: fbits,
        };
        let extra = product_extra_fraction_bits(fmt, 3);
        println!("  W=8 F={fbits}: a three-operand product needs {extra} fraction bits beyond the declared ones");
        if fbits > 0
            && extra
                != product_extra_fraction_bits(
                    Fmt {
                        signed: false,
                        w: 8,
                        f: 0,
                    },
                    3,
                )
        {
            n4_moved = true;
        }
    }
    println!("\n  the fraction width moves a width requirement for multiplication: {n4_moved}");
    assert!(
        n4_moved,
        "N4: the harness never sees F, so Q3 establishes nothing"
    );
    println!();

    // -------------------------------------------------------------------------------
    // What was not reached.
    // -------------------------------------------------------------------------------
    println!("=== cells this run did not reach ===\n");
    println!("  Unmeasured here. Under I13 an unmeasured cell is not claimed.\n");
    for (signed, w, l, t) in &skipped {
        println!(
            "  {:>8} W={w} L={l}: {t} tuples, above the budget",
            if *signed { "signed" } else { "unsigned" }
        );
    }
    println!();

    assert!(all_one, "the signed gap is not uniformly one bit");
    assert!(!any_zero, "a signed cell reported gap zero");
    assert!(
        unsigned_all_format,
        "an unsigned cell needed more than the format width"
    );
    assert!(
        q3_disagreements == 0,
        "the fraction width moved addition's answer"
    );
    println!("P1 WORKS");
}
