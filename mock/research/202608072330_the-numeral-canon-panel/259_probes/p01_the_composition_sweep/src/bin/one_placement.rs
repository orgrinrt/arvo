//! Step 07. The intermediate that keeps the grid and drops the range bound.
//!
//! Added after reading `226_lattner_the_derivation_outputs.md`, which runs a form
//! this seat's first six steps do not: the product is rounded onto the grid and
//! then added to `c` with no range completion between, so the whole expression
//! completes once at the end. Steps 01 to 03 always complete twice, which is what
//! a composition of two declared operations does, and step 04 widens both the
//! grid and the range. This form sits between them: one completion, and an
//! intermediate whose grid is the declared one and whose range is not.
//!
//! Stated before the run, from step 02's mechanism rather than from 226's table.
//! With exactly one completion on each side, H cannot separate them, so the pair
//! agrees exactly where E holds and the overflow policy drops out entirely.
//!
//! G1 must hold: this form agrees wherever step 02 found the rounding region
//!    equivariant, which is `floor`, `ceil`, `toward_zero` and `half_up` under
//!    unsigned at every fraction length, `floor` and `ceil` under signed at every
//!    fraction length, and every mode at `F = 0`.
//! G2 must hold: the answer does not move with the overflow policy. A cell and
//!    its twin under the other policy must have the same verdict, which is what
//!    says H has dropped out rather than merely having been quiet.
//! G3 must fail: at signed saturating with `floor`, this form must agree, where
//!    step 01's two-completion form disagrees. That is the separating case
//!    between the two schedules, and without it this step is measuring step 01
//!    again under a longer name.
//! G4 must fail: this form must still disagree somewhere, or it is the fused
//!    realisation under another name and there is nothing to compare.

use arvo_format::adapt::{Adapt, DeclaredSignature, Signature};
use arvo_format::apply::{Dither, Exact, Fraction, adapt};
use arvo_format::overflow::{Saturate, Wrap};
use arvo_format::rounding::{Ceil, Floor, HalfEven, HalfUp, TowardZero};
use arvo_format::slots::Slot;
use arvo_format::standards::{Fi, Ufi};
use p01_the_composition_sweep::bounds;

/// A format wide enough that the intermediate's range bound never bites, at the
/// declared grid. The rounding coordinate is the cell's own and the policy is
/// never reached, which the widths make true and G0 measures.
type WideAt<const F: i32> = Fi<40, F>;

struct Cell {
    signedness: &'static str,
    width:      u32,
    fraction:   u32,
    mode:       &'static str,
    policy:     &'static str,
    differing:  u64,
    total:      u64,
    witness:    Option<(i64, i64, i64, i64, i64)>,
}

/// One cell: the fused answer against the one-completion composition.
///
/// `Narrow` carries the cell's mode and policy and is where both answers land.
/// `Wide` carries the same mode at the same grid with a range nothing reaches,
/// so the intermediate rounds and does not complete.
fn cell<Narrow, Wide>(
    signedness: &'static str,
    width: u32,
    fraction: u32,
    mode: &'static str,
    policy: &'static str,
) -> Cell
where
    Narrow: DeclaredSignature,
    Wide: DeclaredSignature,
{
    let (lo, hi) = bounds::<Narrow>();
    let den = 1i64 << fraction;
    let mut differing = 0u64;
    let mut total = 0u64;
    let mut witness = None;
    let mut escaped = 0u64;
    for a in lo ..= hi {
        for b in lo ..= hi {
            // The product on the grid, with no range bound reached.
            let rounded = adapt::<Wide>(
                Exact::between(Slot::ZERO, Fraction::of(a * b, den)),
                Dither::UNUSED,
            )
            .index();
            if rounded.abs() > (1i64 << 38) {
                escaped += 1;
            }
            for c in lo ..= hi {
                total += 1;
                let fused = adapt::<Narrow>(
                    Exact::between(Slot::at(c), Fraction::of(a * b, den)),
                    Dither::UNUSED,
                )
                .index();
                let once =
                    adapt::<Narrow>(Exact::on_grid(Slot::at(rounded + c)), Dither::UNUSED).index();
                if fused != once {
                    differing += 1;
                    if witness.is_none() {
                        witness = Some((a, b, c, fused, once));
                    }
                }
            }
        }
    }
    assert_eq!(escaped, 0, "the intermediate reached its own range bound");
    Cell {
        signedness,
        width,
        fraction,
        mode,
        policy,
        differing,
        total,
        witness,
    }
}

fn main() {
    println!("# step 07: one completion, with the intermediate on the declared grid");
    println!();

    let mut rows: Vec<Cell> = Vec::new();

    macro_rules! run {
        ($nfmt:ty, $wfmt:ty, $sign:literal, $w:literal, $f:literal) => {{
            run!(@mode $nfmt, $wfmt, $sign, $w, $f, Floor, "floor");
            run!(@mode $nfmt, $wfmt, $sign, $w, $f, Ceil, "ceil");
            run!(@mode $nfmt, $wfmt, $sign, $w, $f, TowardZero, "toward_zero");
            run!(@mode $nfmt, $wfmt, $sign, $w, $f, HalfUp, "half_up");
            run!(@mode $nfmt, $wfmt, $sign, $w, $f, HalfEven, "half_even");
        }};
        (@mode $nfmt:ty, $wfmt:ty, $sign:literal, $w:literal, $f:literal, $m:ident, $mn:literal) => {{
            rows.push(cell::<
                Signature<$nfmt, Adapt<$m, Wrap>>,
                Signature<$wfmt, Adapt<$m, Wrap>>,
            >($sign, $w, $f, $mn, "wrap"));
            rows.push(cell::<
                Signature<$nfmt, Adapt<$m, Saturate>>,
                Signature<$wfmt, Adapt<$m, Wrap>>,
            >($sign, $w, $f, $mn, "saturate"));
        }};
    }

    macro_rules! both {
        ($w:literal, $f:literal) => {{
            run!(Ufi<$w, $f>, WideAt<$f>, "unsigned", $w, $f);
            run!(Fi<$w, $f>, WideAt<$f>, "signed", $w, $f);
        }};
    }

    both!(3, 0);
    both!(3, 1);
    both!(3, 2);
    both!(4, 0);
    both!(4, 1);
    both!(4, 2);
    both!(4, 3);
    both!(5, 0);
    both!(5, 1);
    both!(5, 2);
    both!(5, 3);
    both!(5, 4);
    both!(6, 0);
    both!(6, 1);
    both!(6, 2);
    both!(6, 3);
    both!(6, 4);
    both!(6, 5);

    for row in &rows {
        println!(
            "once {} W={} F={} mode={} policy={} differing={} of {} witness={:?}",
            row.signedness,
            row.width,
            row.fraction,
            row.mode,
            row.policy,
            row.differing,
            row.total,
            row.witness
        );
    }

    println!();
    println!("# --- the arms ---");

    // G1: agreement must match the equivariance region step 02 measured.
    let mut g1_breaks = 0u64;
    for row in &rows {
        let equivariant = if row.fraction == 0 {
            true
        } else if row.signedness == "unsigned" {
            matches!(row.mode, "floor" | "ceil" | "toward_zero" | "half_up")
        } else {
            matches!(row.mode, "floor" | "ceil")
        };
        if equivariant != (row.differing == 0) {
            g1_breaks += 1;
            println!(
                "G1 BREAK {} W={} F={} {} {}: equivariant={} agreeing={}",
                row.signedness,
                row.width,
                row.fraction,
                row.mode,
                row.policy,
                equivariant,
                row.differing == 0
            );
        }
    }
    println!("G1 detail: {} cells, {g1_breaks} breaks", rows.len());
    verdict(
        "G1",
        "this form must agree exactly where the rounding region is equivariant",
        g1_breaks == 0 && !rows.is_empty(),
    );

    // G2: the policy must not move the verdict.
    let mut g2_breaks = 0u64;
    for row in rows.iter().filter(|r| r.policy == "wrap") {
        let twin = rows.iter().find(|r| {
            r.policy == "saturate"
                && r.signedness == row.signedness
                && r.width == row.width
                && r.fraction == row.fraction
                && r.mode == row.mode
        });
        match twin {
            Some(t) if (t.differing == 0) == (row.differing == 0) => {},
            _ => g2_breaks += 1,
        }
    }
    println!("G2 detail: {g2_breaks} cells whose verdict moved with the policy");
    verdict(
        "G2",
        "with one completion the overflow policy must drop out",
        g2_breaks == 0,
    );

    // G3: the separating case against step 01.
    let separating: Vec<&Cell> = rows
        .iter()
        .filter(|r| r.signedness == "signed" && r.policy == "saturate" && r.mode == "floor")
        .collect();
    let all_agree = separating.iter().all(|r| r.differing == 0);
    println!(
        "G3 detail: {} signed saturating floor cells, all agreeing under one completion: {all_agree}",
        separating.len()
    );
    verdict(
        "G3",
        "signed saturating at floor must agree here and disagrees in step 01",
        all_agree && !separating.is_empty(),
    );

    // G4: it must still be a different function from the fused one somewhere.
    let disagreeing = rows.iter().filter(|r| r.differing > 0).count();
    println!("G4 detail: {disagreeing} cells disagreeing");
    verdict(
        "G4",
        "this form must not be the fused realisation under another name",
        disagreeing > 0,
    );
}

fn verdict(name: &str, expectation: &str, held: bool) {
    println!(
        "{name}: {expectation} -> {}",
        if held { "HELD" } else { "BROKEN" }
    );
}
