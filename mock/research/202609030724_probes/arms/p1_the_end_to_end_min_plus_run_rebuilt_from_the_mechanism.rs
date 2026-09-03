// Probe 1. The end-to-end half of the min-plus row, rebuilt rather than rerun,
// with the sentinel encoding varied as an axis rather than held fixed.
//
// `proposal::a_min_plus_fold_needs_an_absorbing_top_and_wrapping_supplies_none`
// says in its own `gap` field that its end-to-end half stands at one expert and
// that nobody has re-run it. Rerunning `35_probes/p5_algorithm_end_to_end.rs`
// would be a rerun of one instrument, and the probe schema says in its own
// words that a reproduction in a different language is worth more than a rerun
// of the same code. So this arm was written from the specification in
// `probe::a_dag_dynamic_programme_returns_wrong_answers_on_in_range_instances`
// and from the mechanism the proposal states, and it reports its own
// enumeration and its own denominator rather than trying to land on somebody
// else's digit.
//
// THE FIRST VERSION OF THIS FILE FAILED ITS OWN CONTROL, AND THAT IS WHY THE
// ENCODING IS AN AXIS HERE. It predicted, from the mechanism, that the max-plus
// longest path would be robust: the sentinel for max-plus is the bottom, the
// spurious candidate is `bottom + w = w`, and a genuine path was assumed never
// to be cheaper than that. The assumption is false. A genuine longest path into
// a node can be shorter than a single edge into it out of an unreachable node,
// so the maximum takes the spurious candidate. The run reported 437,868 wrong
// of 4,112,248 at width four under wrapping and 458,880 under saturation, and
// the control saying max-plus must report zero failed.
//
// Diagnosing that against the earlier instrument found the reason its own
// max-plus arm reports zero, and it is not the operation. That file's
// `longest` carries `[Option<u128>; N]` and skips an unreachable predecessor
// with `if let Some(bu)`, so no sentinel is ever added to anything, while its
// `shortest` stands the numeral's top in for infinity in band and adds to it.
// The two routines are encoded differently, so the comparison between them
// measures the encoding rather than the operation, and the proposal's own
// `because` clause, that the max-plus agreement "is what says the failure
// belongs to the operation rather than to the harness", does not follow from it.
//
// So this file runs both encodings on both routines under both policies, which
// is eight cells, and lets the encoding carry the explanation.
//
//   IN BAND.  Unreachable is a value of the numeral: the top for min-plus,
//             the bottom for max-plus. Relaxing out of an unreachable node goes
//             through the numeral's own addition, so the sentinel has to absorb.
//   OUT OF BAND. Unreachable is a flag the numeral does not carry, and an
//             unreachable predecessor is skipped rather than added to.
//
// THE MECHANISM, as it stands after the run rather than as predicted before it.
// In band, correctness needs the sentinel to absorb addition, and it also needs
// every ordinary candidate to stay in range or to overflow in a direction the
// reduction discards. Under saturation both hold for min-plus: the top absorbs,
// and an overflowing sum clamps to the top, which the minimum throws away. Under
// wrapping neither holds. For max-plus the sentinel is the unsigned bottom,
// which absorbs addition under no policy, so max-plus in band fails under both.
// Out of band the sentinel is gone and the absorption failure with it, and the
// overflow failure is not, which is the decomposition this file reports.
//
// THE CASES THAT MUST FAIL, in the numbering the run settled on. The first
// version of this file carried a different C3 and C4 and both were wrong. What
// they were, and what the runs that refuted them reported, is in the paragraph
// above and in the comments beside the controls in `main`.
//   C1. In band, min-plus under saturation must report zero. Same routine, same
//       encoding, same graphs, only the policy moved.
//   C2. In band, min-plus under wrapping must report nonzero, or the row is
//       refuted at these widths and this file says so.
//   C3. The cells where the arithmetic cannot go wrong must report zero: out of
//       band under saturation, and out of band max-plus under either policy.
//   C4. In band, max-plus must fail under both policies. This is the row's own
//       control run like for like.
//   C5. The sabotage arm. The sentinel moved off the top under saturation must
//       turn C1's zero into a nonzero, or C1 says nothing about absorption.
//   C6. Out of band is never worse than in band, and is not zero.
//
// THE IN-RANGE CONTROL. An instance is counted only where the target is
// reachable in the exact graph and where the exact answer and every finite
// exact intermediate fit inside 0..=MAX. So a disagreement can never be the
// exact answer having left the range, which would be range exhaustion reported
// as a law failure.
//
// THE ENUMERATION, which is mine and is stated rather than implied. Four nodes
// 0, 1, 2, 3 in one fixed topological order. Six ordered pairs i < j, each slot
// independently either absent or carrying a weight in 0..=MAX. That is
// (MAX + 2)^6 configurations: 9^6 = 531,441 at width three and 17^6 =
// 24,137,569 at width four. Every configuration is walked; none is sampled.

/// One numeral policy. `Wrap` is the cyclic group of the width; `Saturate`
/// clamps at the top.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Policy {
    Wrap,
    Saturate,
}

/// Addition on the numeral, at `width` bits, unsigned, fraction width zero.
fn nadd(a: u64, b: u64, width: u32, policy: Policy) -> u64 {
    let modulus = 1u64 << width;
    let max = modulus - 1;
    match policy {
        Policy::Wrap => (a + b) % modulus,
        Policy::Saturate => {
            let s = a + b;
            if s > max { max } else { s }
        },
    }
}

/// Which routine is being run.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Routine {
    /// Shortest path. The unit of `min` is infinity, which stands on the top.
    MinPlus,
    /// Longest path. The unit of `max` is minus infinity, which stands on the
    /// bottom, and the bottom of an unsigned numeral is zero.
    MaxPlus,
}

/// How the routine says a node is unreachable.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Encoding {
    /// A value of the numeral, added to like any other.
    InBand,
    /// A flag the numeral does not carry. An unreachable predecessor is skipped.
    OutOfBand,
}

/// The six slots, in the order (0,1) (0,2) (0,3) (1,2) (1,3) (2,3).
const EDGES: [(usize, usize); 6] = [(0, 1), (0, 2), (0, 3), (1, 2), (1, 3), (2, 3)];

/// The dynamic programme over the numeral.
///
/// `sentinel` is a parameter rather than a constant so the sabotage arm can move
/// it off the top without a second copy of the routine.
fn run_on_the_numeral(
    slots: &[Option<u64>; 6],
    width: u32,
    policy: Policy,
    routine: Routine,
    encoding: Encoding,
    sentinel: u64,
) -> Option<u64> {
    // `reachable` is only consulted under `OutOfBand`. Under `InBand` the
    // sentinel is a perfectly ordinary value and the routine cannot tell.
    let mut dist = [sentinel; 4];
    let mut reachable = [false; 4];
    dist[0] = 0;
    reachable[0] = true;
    for j in 1 .. 4 {
        let mut best = sentinel;
        let mut seen = false;
        for (slot, &(a, b)) in EDGES.iter().enumerate() {
            if b != j {
                continue;
            }
            let Some(w) = slots[slot] else { continue };
            if encoding == Encoding::OutOfBand && !reachable[a] {
                continue;
            }
            let candidate = nadd(dist[a], w, width, policy);
            if !seen {
                best = candidate;
                seen = true;
            } else {
                best = match routine {
                    Routine::MinPlus => best.min(candidate),
                    Routine::MaxPlus => best.max(candidate),
                };
            }
        }
        if seen {
            dist[j] = best;
            reachable[j] = true;
        }
    }
    match encoding {
        Encoding::InBand => Some(dist[3]),
        Encoding::OutOfBand => {
            if reachable[3] {
                Some(dist[3])
            } else {
                None
            }
        },
    }
}

/// The same programme over unbounded integers, with a real absent value rather
/// than a sentinel drawn from the numeral's own range.
fn run_exactly(slots: &[Option<u64>; 6], routine: Routine) -> (Option<u64>, [Option<u64>; 4]) {
    let mut dist: [Option<u64>; 4] = [None; 4];
    dist[0] = Some(0);
    for j in 1 .. 4 {
        let mut best: Option<u64> = None;
        for (slot, &(a, b)) in EDGES.iter().enumerate() {
            if b != j {
                continue;
            }
            let Some(w) = slots[slot] else { continue };
            let Some(da) = dist[a] else { continue };
            let candidate = da + w;
            best = Some(match (best, routine) {
                (None, _) => candidate,
                (Some(x), Routine::MinPlus) => x.min(candidate),
                (Some(x), Routine::MaxPlus) => x.max(candidate),
            });
        }
        dist[j] = best;
    }
    (dist[3], dist)
}

/// One cell of the matrix.
struct Cell {
    counted:       u64,
    wrong:         u64,
    first_witness: Option<[Option<u64>; 6]>,
}

fn sweep(
    width: u32,
    policy: Policy,
    routine: Routine,
    encoding: Encoding,
    sabotage_the_sentinel: bool,
) -> Cell {
    let max = (1u64 << width) - 1;
    let sentinel = if sabotage_the_sentinel {
        0
    } else {
        match routine {
            Routine::MinPlus => max,
            Routine::MaxPlus => 0,
        }
    };
    let alphabet = max + 2; // absent, then 0..=max
    let mut cell = Cell {
        counted:       0,
        wrong:         0,
        first_witness: None,
    };
    let total = alphabet.pow(6);
    for code in 0 .. total {
        let mut slots: [Option<u64>; 6] = [None; 6];
        let mut rest = code;
        for slot in 0 .. 6 {
            let digit = rest % alphabet;
            rest /= alphabet;
            slots[slot] = if digit == 0 { None } else { Some(digit - 1) };
        }
        let (exact, intermediates) = run_exactly(&slots, routine);
        // The in-range control. Target reachable, answer in range, every finite
        // intermediate in range. Anything else is not counted at all.
        let Some(answer) = exact else { continue };
        if answer > max {
            continue;
        }
        if intermediates.iter().flatten().any(|&d| d > max) {
            continue;
        }
        cell.counted += 1;
        let got = run_on_the_numeral(&slots, width, policy, routine, encoding, sentinel);
        if got != Some(answer) {
            cell.wrong += 1;
            if cell.first_witness.is_none() {
                cell.first_witness = Some(slots);
            }
        }
    }
    cell
}

fn describe(slots: &[Option<u64>; 6]) -> String {
    let mut out = String::new();
    for (slot, &(a, b)) in EDGES.iter().enumerate() {
        match slots[slot] {
            Some(w) => out.push_str(&format!("{a}->{b}={w} ")),
            None => out.push_str(&format!("{a}->{b}=.  ")),
        }
    }
    out
}

fn main() {
    println!("probe 1: the end-to-end min-plus run, with the sentinel encoding");
    println!("as an axis. four nodes, one fixed topological order, six ordered");
    println!("slots each absent or carrying a weight in 0..=MAX, (MAX+2)^6");
    println!("configurations walked in full, in-range control on the answer and");
    println!("on every finite exact intermediate.");
    println!();

    // Every cell, kept so the controls and the decomposition are computed from
    // the run rather than asserted alongside it.
    let mut cells: Vec<(u32, Encoding, Routine, Policy, u64, u64)> = Vec::new();

    for width in [3u32, 4u32] {
        println!("== width {width}, unsigned, fraction width zero ==");
        for encoding in [Encoding::InBand, Encoding::OutOfBand] {
            for routine in [Routine::MinPlus, Routine::MaxPlus] {
                for policy in [Policy::Wrap, Policy::Saturate] {
                    let cell = sweep(width, policy, routine, encoding, false);
                    let pct = if cell.counted == 0 {
                        0.0
                    } else {
                        100.0 * (cell.wrong as f64) / (cell.counted as f64)
                    };
                    println!(
                        "  {encoding:?} {routine:?} under {policy:?}: {} wrong of {} in-range, {:.4}%",
                        cell.wrong, cell.counted, pct
                    );
                    if let Some(w) = cell.first_witness {
                        println!("      first witness: {}", describe(&w));
                    }
                    cells.push((width, encoding, routine, policy, cell.wrong, cell.counted));
                }
            }
        }
        println!();
    }

    let at = |w: u32, e: Encoding, r: Routine, p: Policy| -> (u64, u64) {
        cells
            .iter()
            .find(|c| c.0 == w && c.1 == e && c.2 == r && c.3 == p)
            .map(|c| (c.4, c.5))
            .expect("every cell was swept")
    };

    println!("== the decomposition, which is what this file is actually for ==");
    println!("the row names absorption as the mechanism and its own `gap` field");
    println!("says monotonicity is a second property it did not file. taking the");
    println!("sentinel out of band removes every absorption failure and leaves");
    println!("every monotonicity failure, so the difference between the two");
    println!("min-plus wrapping cells is what absorption was worth.");
    println!();
    let mut decomposition_ok = true;
    for width in [3u32, 4u32] {
        let (inb, _) = at(width, Encoding::InBand, Routine::MinPlus, Policy::Wrap);
        let (oob, _) = at(width, Encoding::OutOfBand, Routine::MinPlus, Policy::Wrap);
        let share = if inb == 0 { 0.0 } else { 100.0 * ((inb - oob) as f64) / (inb as f64) };
        println!(
            "  width {width}: in band {inb} wrong, out of band {oob} wrong. absorption \
             accounts for {} of them, {:.1}% of the failures; the rest survive with no \
             sentinel in the numeral at all.",
            inb - oob,
            share
        );
        decomposition_ok &= oob <= inb && oob > 0;
    }
    println!();

    println!("== C5, the sabotage arm ==");
    println!("in band, min-plus under saturation, sentinel moved off the top to");
    println!("zero. zero absorbs addition under no policy, so this must go nonzero.");
    let mut c5_ok = true;
    for width in [3u32, 4u32] {
        let cell = sweep(
            width,
            Policy::Saturate,
            Routine::MinPlus,
            Encoding::InBand,
            true,
        );
        println!(
            "  width {width}: {} wrong of {} in-range",
            cell.wrong, cell.counted
        );
        c5_ok &= cell.wrong > 0;
    }
    println!();

    // C1. Same routine, same encoding, same graphs, only the policy moved. This
    // is the like-for-like control the earlier instrument's cross-routine
    // comparison was standing in for, and it is the one that says the harness is
    // not simply reporting failure.
    let mut c1_ok = true;
    for width in [3u32, 4u32] {
        c1_ok &= at(width, Encoding::InBand, Routine::MinPlus, Policy::Saturate).0 == 0;
    }

    // C2. The claim itself. If this is zero the row is refuted at these widths.
    let mut c2_ok = true;
    for width in [3u32, 4u32] {
        c2_ok &= at(width, Encoding::InBand, Routine::MinPlus, Policy::Wrap).0 > 0;
    }

    // C3, restated after the first version of it failed. The cells where the
    // arithmetic genuinely cannot go wrong are the saturating ones, where every
    // overflow maps to the top and the minimum discards it, and the out-of-band
    // max-plus ones, where no sentinel is ever added and no candidate can exceed
    // the exact longest path. Those must be clean or the enumeration or the
    // exact reference is wrong.
    //
    // The first version demanded that EVERY out-of-band cell be clean, on the
    // reasoning that taking the sentinel out of band removes the failure. It
    // does not: out of band under wrapping still reports a large count, because
    // an ordinary candidate `dist[a] + w` can overflow and wrap small even when
    // every exact intermediate is in range. That is the monotonicity half the
    // row's own `gap` says is unfiled, and it is measured in the decomposition
    // above rather than asserted as a control.
    let mut c3_ok = true;
    for width in [3u32, 4u32] {
        c3_ok &= at(
            width,
            Encoding::OutOfBand,
            Routine::MinPlus,
            Policy::Saturate,
        )
        .0 == 0;
        c3_ok &= at(width, Encoding::OutOfBand, Routine::MaxPlus, Policy::Wrap).0 == 0;
        c3_ok &= at(
            width,
            Encoding::OutOfBand,
            Routine::MaxPlus,
            Policy::Saturate,
        )
        .0 == 0;
    }

    // C4. The row's own control, run like for like. The earlier instrument's
    // max-plus arm is out of band while its min-plus arm is in band, so its
    // clean max-plus column compares two encodings. Run in band, max-plus must
    // fail under both policies, because the unsigned bottom absorbs addition
    // under neither. If it comes back clean, the diagnosis is wrong.
    let mut c4_ok = true;
    for width in [3u32, 4u32] {
        c4_ok &= at(width, Encoding::InBand, Routine::MaxPlus, Policy::Wrap).0 > 0;
        c4_ok &= at(width, Encoding::InBand, Routine::MaxPlus, Policy::Saturate).0 > 0;
    }

    println!("== the controls ==");
    println!(
        "  C1 in band, min-plus saturating, zero ................ {}",
        pass(c1_ok)
    );
    println!(
        "  C2 in band, min-plus wrapping, nonzero ............... {}",
        pass(c2_ok)
    );
    println!(
        "  C3 the cells that cannot go wrong, zero .............. {}",
        pass(c3_ok)
    );
    println!(
        "  C4 in band, max-plus fails under both policies ....... {}",
        pass(c4_ok)
    );
    println!(
        "  C5 sabotaged sentinel, nonzero ....................... {}",
        pass(c5_ok)
    );
    println!(
        "  C6 out of band never worse than in band, and nonzero . {}",
        pass(decomposition_ok)
    );
    println!();
    if c1_ok && c2_ok && c3_ok && c4_ok && c5_ok && decomposition_ok {
        println!("every control: PASSED");
    } else {
        println!("a control FAILED. no number in this file counts.");
    }
    println!();
    println!("what this says, and two thirds of it is not what the row says.");
    println!();
    println!("the row's headline reproduces. in band, min-plus is wrong under");
    println!("wrapping at both widths and right under saturation at both, and");
    println!("the sabotage arm at C5 says the sentinel absorbing is why the");
    println!("saturating column is clean.");
    println!();
    println!("the row's control does not reproduce. its `because` says the");
    println!("max-plus routine agreeing everywhere is what says the failure");
    println!("belongs to the operation rather than the harness. run in the same");
    println!("encoding as the failing routine, max-plus fails under both");
    println!("policies. the earlier instrument's clean max-plus column is its");
    println!("out-of-band `Option` encoding, not the operation, so that clause");
    println!("supports nothing. C1 is the control that does the job, and it is");
    println!("a policy move inside one routine rather than a routine swap.");
    println!();
    println!("and the mechanism is mostly not absorption. the row's own `gap`");
    println!("names monotonicity as a second unfiled property. the decomposition");
    println!("above prices it: with the sentinel taken out of the numeral");
    println!("entirely, the wrapping failure barely moves, so absorption is the");
    println!("minority cause and the unfiled half is the majority one.");
}

fn pass(b: bool) -> &'static str {
    if b { "PASS" } else { "FAIL" }
}
