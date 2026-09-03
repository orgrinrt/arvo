//! Step 02. The two conditions, measured apart, and the composition table
//! predicted from them.
//!
//! Step 01 produces a table. A table is not a mechanism, and a table cannot say
//! what happens at a width it did not run. This step separates the applied map's
//! two regions and measures each on its own, then predicts step 01 cell by cell.
//!
//! `fused` is `complete(round(ab + c))`. `stepwise` is
//! `complete(complete(round(ab)) + c)`, because the outer rounding is dead: a
//! slot plus a slot is a slot. So two conditions decide the pair.
//!
//! E, equivariance of the rounding region: `round(x + c) = round(x) + c` for
//!    every product position `x` and every representable `c`. Measured through a
//!    format wide enough that the completion never fires, so what is left is the
//!    rounding alone. Both halves are the shipped map.
//! H, homomorphism of the completion region: `complete(y + c) =
//!    complete(complete(y) + c)` for every rounded product `y` and every
//!    representable `c`. Measured on grid positions only, so the rounding is the
//!    identity and what is left is the completion alone.
//!
//! The prediction: E and H together are sufficient for the cell to agree. They
//! are also necessary under wrap, because two neighbouring slots stay distinct
//! under a wrap whose span is above one. Under saturation they are not
//! necessary, because completion can collapse two different answers onto one
//! bound, so a cell can agree with E broken. Both halves are checked here rather
//! than assumed.
//!
//! B1 must hold: the wide format must never complete. If it did, E would be
//!    measuring the two regions together and would say nothing about either.
//! B2 must hold: at least one cell must break E and at least one must hold it,
//!    or the instrument is insensitive.
//! B3 must hold: at least one cell must break H and at least one must hold it.
//! B4 must hold: no cell where E and H both hold may disagree in step 01.
//! B5 must hold under wrap: a wrap cell that breaks E or H may not agree, since
//!    under wrap the two conditions are necessary as well as sufficient. Cells
//!    that break a condition and agree anyway are printed, and the saturating
//!    ones among them are a real region rather than a defect.

use arvo_format::adapt::{Adapt, Signature};
use arvo_format::apply::{Dither, Exact, Fraction, adapt};
use arvo_format::overflow::{Saturate, Wrap};
use arvo_format::rounding::{Ceil, Floor, HalfEven, HalfUp, TowardZero};
use arvo_format::slots::Slot;
use arvo_format::standards::{Fi, Ufi};
use p01_the_composition_sweep::{Cell, bounds, fused, natural_cell, stepwise};

/// A format wide enough that nothing the sweep reaches ever leaves its range, at
/// every fraction length the sweep uses. The widest position the sweep visits is
/// under 2^17 slots and this range runs to 2^39, and B1 measures that rather
/// than trusting the sentence.
type WideAt<const F: i32> = Fi<40, F>;

/// The rounding region alone, under a signature that never completes.
fn round_only<const F: i32>(mode_marker: &str, slot: i64, num: i64, den: i64) -> i64 {
    let position = Exact::between(Slot::at(slot), Fraction::of(num, den));
    match mode_marker {
        "floor" => adapt::<Signature<WideAt<F>, Adapt<Floor, Wrap>>>(position, Dither::UNUSED),
        "ceil" => adapt::<Signature<WideAt<F>, Adapt<Ceil, Wrap>>>(position, Dither::UNUSED),
        "toward_zero" => {
            adapt::<Signature<WideAt<F>, Adapt<TowardZero, Wrap>>>(position, Dither::UNUSED)
        },
        "half_up" => adapt::<Signature<WideAt<F>, Adapt<HalfUp, Wrap>>>(position, Dither::UNUSED),
        "half_even" => {
            adapt::<Signature<WideAt<F>, Adapt<HalfEven, Wrap>>>(position, Dither::UNUSED)
        },
        other => panic!("no such mode: {other}"),
    }
    .index()
}

/// The completion region alone, on a grid position so no rounding happens.
fn complete_only<S: arvo_format::adapt::DeclaredSignature>(slot: i64) -> i64 {
    adapt::<S>(Exact::on_grid(Slot::at(slot)), Dither::UNUSED).index()
}

fn main() {
    println!("# step 02: the two conditions apart, and step 01 predicted from them");
    println!();

    let mut rows: Vec<Cell> = Vec::new();
    p01_the_composition_sweep::the_grid!(rows);

    let mut sufficiency_breaks = 0u64;
    let mut agreeing_with_a_condition_broken: Vec<String> = Vec::new();
    let mut equivariant_cells = 0u64;
    let mut non_equivariant_cells = 0u64;
    let mut homomorphic_cells = 0u64;
    let mut non_homomorphic_cells = 0u64;
    let mut wide_completions = 0u64;
    let mut wide_positions = 0u64;

    for row in &rows {
        let (lo, hi) = range_of(row.signedness, row.width);
        let den = 1i64 << row.fraction;

        // E, over exactly the products and addends this cell reaches. No early
        // exit, because the wide-format control counts every position visited
        // and a shortened loop would weaken it without saying so.
        let mut equivariant = true;
        let mut e_witness = None;
        for a in lo ..= hi {
            for b in lo ..= hi {
                let product = a * b;
                let base = round_only_at(row.fraction, row.mode, 0, product, den);
                wide_positions += 1;
                if base.abs() > (1i64 << 38) {
                    wide_completions += 1;
                }
                for c in lo ..= hi {
                    let shifted = round_only_at(row.fraction, row.mode, c, product, den);
                    wide_positions += 1;
                    if shifted.abs() > (1i64 << 38) {
                        wide_completions += 1;
                    }
                    if shifted != base + c {
                        equivariant = false;
                        if e_witness.is_none() {
                            e_witness = Some((a, b, c, base, shifted));
                        }
                    }
                }
            }
        }

        // H, over exactly the rounded products this cell reaches.
        let mut homomorphic = true;
        let mut h_witness = None;
        'outer: for a in lo ..= hi {
            for b in lo ..= hi {
                let y = round_only_at(row.fraction, row.mode, 0, a * b, den);
                for c in lo ..= hi {
                    let direct = complete_of(row.signedness, row.width, row.policy, y + c);
                    let staged = complete_of(
                        row.signedness,
                        row.width,
                        row.policy,
                        complete_of(row.signedness, row.width, row.policy, y) + c,
                    );
                    if direct != staged {
                        homomorphic = false;
                        h_witness = Some((a, b, c, direct, staged));
                        break 'outer;
                    }
                }
            }
        }

        if equivariant {
            equivariant_cells += 1;
        } else {
            non_equivariant_cells += 1;
        }
        if homomorphic {
            homomorphic_cells += 1;
        } else {
            non_homomorphic_cells += 1;
        }

        let predicted = equivariant && homomorphic;
        let measured = row.agrees();
        if predicted && !measured {
            sufficiency_breaks += 1;
            println!(
                "B4 BREAK {} W={} F={} {} {}: E and H both hold and the cell disagrees",
                row.signedness, row.width, row.fraction, row.mode, row.policy
            );
        }
        if !predicted && measured {
            agreeing_with_a_condition_broken.push(format!(
                "{} W={} F={} {} {} (E={equivariant} H={homomorphic})",
                row.signedness, row.width, row.fraction, row.mode, row.policy
            ));
        }

        println!(
            "mech {} W={} F={} mode={} policy={} E={} H={} predicted={} measured={} e_witness={:?} h_witness={:?}",
            row.signedness,
            row.width,
            row.fraction,
            row.mode,
            row.policy,
            equivariant,
            homomorphic,
            predicted,
            measured,
            e_witness,
            h_witness,
        );
    }

    println!();
    println!("# --- the arms ---");
    println!(
        "B1 detail: wide-format positions visited={wide_positions}, of which completed={wide_completions}"
    );
    verdict(
        "B1",
        "the wide format must never complete, so E is the rounding alone",
        wide_completions == 0 && wide_positions > 0,
    );
    println!(
        "B2 detail: equivariant cells={equivariant_cells}, non-equivariant={non_equivariant_cells}"
    );
    verdict(
        "B2",
        "E must separate: some cells equivariant and some not",
        equivariant_cells > 0 && non_equivariant_cells > 0,
    );
    println!(
        "B3 detail: homomorphic cells={homomorphic_cells}, non-homomorphic={non_homomorphic_cells}"
    );
    verdict(
        "B3",
        "H must separate: some cells homomorphic and some not",
        homomorphic_cells > 0 && non_homomorphic_cells > 0,
    );
    verdict(
        "B4",
        "E and H together must be sufficient: no such cell may disagree",
        sufficiency_breaks == 0,
    );
    println!(
        "B5 detail: {} cells agree with E or H broken",
        agreeing_with_a_condition_broken.len()
    );
    for line in &agreeing_with_a_condition_broken {
        println!("B5 cell: {line}");
    }
    let under_wrap = agreeing_with_a_condition_broken
        .iter()
        .filter(|s| s.contains(" wrap "))
        .count();
    verdict(
        "B5",
        "under wrap the conditions must also be necessary, so no wrap cell may sit here",
        under_wrap == 0,
    );

    println!();
    println!("# --- a second reading of A5, by hand ---");
    hand_check_half_up();
}

fn verdict(name: &str, expectation: &str, held: bool) {
    println!(
        "{name}: {expectation} -> {}",
        if held { "HELD" } else { "BROKEN" }
    );
}

fn range_of(signedness: &str, width: u32) -> (i64, i64) {
    if signedness == "unsigned" {
        (0, (1i64 << width) - 1)
    } else {
        (-(1i64 << (width - 1)), (1i64 << (width - 1)) - 1)
    }
}

/// The rounding region at fraction length `f`, on the position
/// `slot + num / den`, under a format that never completes.
fn round_only_at(f: u32, mode: &str, slot: i64, num: i64, den: i64) -> i64 {
    match f {
        0 => round_only::<0>(mode, slot, num, den),
        1 => round_only::<1>(mode, slot, num, den),
        2 => round_only::<2>(mode, slot, num, den),
        3 => round_only::<3>(mode, slot, num, den),
        4 => round_only::<4>(mode, slot, num, den),
        5 => round_only::<5>(mode, slot, num, den),
        6 => round_only::<6>(mode, slot, num, den),
        7 => round_only::<7>(mode, slot, num, den),
        other => panic!("no fraction length {other} in this sweep"),
    }
}

/// The completion region for one cell, on a grid position.
///
/// The completion reads the slot range only, and the slot range is a function of
/// width and signedness rather than of fraction length, so the single fraction
/// length named here reaches the same range as every other would.
fn complete_of(signedness: &str, width: u32, policy: &str, slot: i64) -> i64 {
    macro_rules! at {
        ($fmt:ty) => {
            match policy {
                "wrap" => complete_only::<Signature<$fmt, Adapt<Floor, Wrap>>>(slot),
                "saturate" => complete_only::<Signature<$fmt, Adapt<Floor, Saturate>>>(slot),
                other => panic!("no such policy: {other}"),
            }
        };
    }
    macro_rules! widths {
        ($($w:literal),+ $(,)?) => {
            match (signedness, width) {
                $(("unsigned", $w) => at!(Ufi<$w, 0>),)+
                $(("signed", $w) => at!(Fi<$w, 0>),)+
                other => panic!("no such cell: {other:?}"),
            }
        };
    }
    widths!(3, 4, 5, 6, 7, 8)
}

/// The witness A5 turns on, worked out on paper and asserted here.
///
/// Signed, width 6, fraction 1, so the quantum is one half and the slots run
/// from -32 to 31. Take a = -31/2, b = -1/2, c = -16.
///
/// Fused: `a * b` is 31/4, plus c is -33/4, which in slot units is -16.5. A tie.
/// `HalfUp` sends a tie away from zero, so it lands on -17.
///
/// Stepwise: `a * b` is 31/4, which is 15.5 slots. Also a tie, and away from zero
/// on a positive position is up, so 16. Then 16 plus -32 is -16, in range under
/// wrap.
///
/// Two answers, one slot apart, and the whole of the difference is that the tie
/// rule read the sign of a position that translation moved across zero.
fn hand_check_half_up() {
    type S = Signature<Fi<6, 1>, Adapt<HalfUp, Wrap>>;
    let den = 1i64 << 1;
    let (lo, hi) = bounds::<S>();
    println!("hand check: signed W=6 F=1 half_up wrap, slots {lo} to {hi}");
    let f = fused::<S>(-31, -1, -32, den);
    let s = stepwise::<S, S>(-31, -1, -32, den);
    println!("hand check: a=-31 b=-1 c=-32 fused={f} stepwise={s}");
    verdict(
        "A5-hand",
        "the paper answer is fused = -17 and stepwise = -16",
        f == -17 && s == -16,
    );

    // The control: the same triple under floor agrees, so the disagreement is the
    // tie rule rather than the triple.
    type Down = Signature<Fi<6, 1>, Adapt<Floor, Wrap>>;
    let fd = fused::<Down>(-31, -1, -32, den);
    let sd = stepwise::<Down, Down>(-31, -1, -32, den);
    println!("hand check control: the same triple under floor, fused={fd} stepwise={sd}");
    verdict(
        "A5-hand-control",
        "the same triple under floor must agree",
        fd == sd,
    );

    // The unsigned twin: half_up on a non-negative domain never sees a tie on a
    // negative position, so the same cell agrees at every triple there.
    type U = Signature<Ufi<6, 1>, Adapt<HalfUp, Wrap>>;
    let cell = natural_cell::<U>("unsigned", 6, 1, "half_up", "wrap");
    println!(
        "hand check twin: unsigned W=6 F=1 half_up wrap differing={} of {}",
        cell.differing, cell.total
    );
    verdict(
        "A5-hand-twin",
        "the unsigned twin of the failing cell must agree at every triple",
        cell.agrees(),
    );
}
