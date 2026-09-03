//! Step 06. The one disagreement with a committed law row, and whether it is a
//! disagreement about arithmetic or about a word.
//!
//! Step 01 finds `half_up` failing under signed wrapping at every fraction
//! length above zero. `law::fusing_a_multiply_add_preserves_the_answer_under_signed_wrapping`
//! carries `half_up` in its holding region. Both cannot be right about one
//! operation, so the question is whether they are talking about one operation.
//!
//! They are not, necessarily. Nearest-with-a-tie has two readings on a signed
//! domain: the tie goes away from zero, or the tie goes toward positive
//! infinity. `arvo_format` implements the first and says so in its own rustdoc
//! and its own committed test. The second is what a rounding rule has to be for
//! the committed row's region to be right, because only the second reads nothing
//! but the residue and therefore commutes with translation.
//!
//! So this step models the rounding region with the tie direction as a
//! parameter, calibrates the model against the shipped map on the reading both
//! have, and then measures the reading only the model has.
//!
//! F2 must hold, and it is the calibration: the model under ties-away-from-zero
//!    must agree with `arvo_format`'s `HalfUp` at every position of every cell.
//!    Nothing after this counts if it breaks, because the model would not be a
//!    model of the shipped map.
//! F3 must hold, and it is the separation: the model under ties-toward-positive-
//!    infinity must differ from the shipped `HalfUp` somewhere. If the two
//!    readings agreed everywhere they would be one operation and there would be
//!    nothing open.
//! F1 must hold: under signed wrapping, the fused and stepwise realisations must
//!    agree at every triple of every cell when the tie goes toward positive
//!    infinity. That is the committed row's region, reproduced under the reading
//!    that makes it true.
//! F4 must hold: under signed saturating the same reading must still disagree,
//!    so the tie direction does not reach the ruling's exception.
//! F5 must hold: under unsigned both readings must agree at every triple, since
//!    a non-negative domain has no tie on a negative position for the two to
//!    differ at.
//!
//! Everything here that is a model is a model. The completion region is still
//! `arvo_format`'s, reached by adapting a grid position under `Floor`, where the
//! rounding region is the identity by construction. Only the rounding is modelled
//! and only because the mode being measured is one the crate does not ship.

use arvo_format::adapt::{Adapt, DeclaredSignature, Signature};
use arvo_format::apply::{Dither, Exact, Fraction, adapt};
use arvo_format::overflow::{Saturate, Wrap};
use arvo_format::rounding::{Floor, HalfUp};
use arvo_format::slots::Slot;
use arvo_format::standards::{Fi, Ufi};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Tie {
    /// What `arvo_format::rounding::HalfUp` does: a tie leaves zero.
    AwayFromZero,
    /// The other reading: a tie rises, whatever the sign.
    TowardPositiveInfinity,
}

/// The rounding region, modelled, with the tie direction as a parameter.
///
/// Written to mirror `arvo_format::apply::round_slot`'s shape rather than to be
/// clever: normalise the remainder into the slot, then one comparison of twice
/// the remainder against the denominator, then the tie rule. F2 is what says the
/// mirroring worked.
fn model_round(tie: Tie, slot_base: i64, num: i64, den: i64) -> i64 {
    let whole = num.div_euclid(den);
    let rem = num.rem_euclid(den);
    let slot = slot_base + whole;
    if rem == 0 {
        return slot;
    }
    if 2 * rem > den {
        return slot + 1;
    }
    if 2 * rem < den {
        return slot;
    }
    match tie {
        // The position is negative exactly when the slot is, because the
        // remainder is non-negative and below one. Away from zero on a negative
        // position is down.
        Tie::AwayFromZero => {
            if slot < 0 {
                slot
            } else {
                slot + 1
            }
        },
        Tie::TowardPositiveInfinity => slot + 1,
    }
}

/// The completion region, which is not modelled. Adapting a grid position under
/// `Floor` reaches the shipped completion with the rounding region inert.
fn complete<S: DeclaredSignature>(slot: i64) -> i64 {
    adapt::<S>(Exact::on_grid(Slot::at(slot)), Dither::UNUSED).index()
}

/// The applied map under the modelled rounding and the shipped completion.
fn model_adapt<S: DeclaredSignature>(tie: Tie, slot_base: i64, num: i64, den: i64) -> i64 {
    complete::<S>(model_round(tie, slot_base, num, den))
}

struct Counts {
    triples:   u64,
    differing: u64,
    witness:   Option<(i64, i64, i64, i64, i64)>,
}

/// Fused against stepwise under the modelled rounding, at one cell.
///
/// `S` is the signature whose completion is used. Its rounding coordinate is
/// `Floor` and is never reached, because every position handed to `complete` is
/// on the grid.
fn model_cell<S: DeclaredSignature>(tie: Tie, lo: i64, hi: i64, fraction: u32) -> Counts {
    let den = 1i64 << fraction;
    let mut out = Counts {
        triples:   0,
        differing: 0,
        witness:   None,
    };
    for a in lo ..= hi {
        for b in lo ..= hi {
            let product = model_adapt::<S>(tie, 0, a * b, den);
            for c in lo ..= hi {
                out.triples += 1;
                let fused = model_adapt::<S>(tie, c, a * b, den);
                let stepwise = complete::<S>(product + c);
                if fused != stepwise {
                    out.differing += 1;
                    if out.witness.is_none() {
                        out.witness = Some((a, b, c, fused, stepwise));
                    }
                }
            }
        }
    }
    out
}

fn main() {
    println!("# step 06: the tie direction, and which reading the committed row's region needs");
    println!();

    let mut calibration_gap = 0u64;
    let mut calibration_positions = 0u64;
    let mut separation_hits = 0u64;
    let mut separation_witness: Option<(i64, i64, i64, i64)> = None;
    let mut signed_wrap_ties_up_differing = 0u64;
    let mut signed_wrap_cells = 0u64;
    let mut signed_saturate_ties_up_agreeing = 0u64;
    let mut signed_saturate_cells = 0u64;
    let mut unsigned_readings_differ = 0u64;
    let mut unsigned_cells = 0u64;

    macro_rules! cell {
        ($w:literal, $f:literal) => {{
            let slo = -(1i64 << ($w - 1));
            let shi = (1i64 << ($w - 1)) - 1;
            let ulo = 0i64;
            let uhi = (1i64 << $w) - 1;
            let den = 1i64 << $f;

            // F2, the calibration, over every position the signed cell reaches:
            // the modelled away-from-zero rounding under the shipped completion,
            // against the shipped `HalfUp` under the same completion.
            for a in slo ..= shi {
                for b in slo ..= shi {
                    for c in slo ..= shi {
                        calibration_positions += 1;
                        let modelled = model_adapt::<Signature<Fi<$w, $f>, Adapt<Floor, Wrap>>>(
                            Tie::AwayFromZero,
                            c,
                            a * b,
                            den,
                        );
                        let shipped = adapt::<Signature<Fi<$w, $f>, Adapt<HalfUp, Wrap>>>(
                            Exact::between(Slot::at(c), Fraction::of(a * b, den)),
                            Dither::UNUSED,
                        )
                        .index();
                        if modelled != shipped {
                            calibration_gap += 1;
                        }
                        // F3, the separation, on the same positions.
                        let other = model_adapt::<Signature<Fi<$w, $f>, Adapt<Floor, Wrap>>>(
                            Tie::TowardPositiveInfinity,
                            c,
                            a * b,
                            den,
                        );
                        if other != shipped {
                            separation_hits += 1;
                            if separation_witness.is_none() {
                                separation_witness = Some((a, b, c, other));
                            }
                        }
                    }
                }
            }

            // F1, signed wrapping under the other reading.
            let r = model_cell::<Signature<Fi<$w, $f>, Adapt<Floor, Wrap>>>(
                Tie::TowardPositiveInfinity,
                slo,
                shi,
                $f,
            );
            signed_wrap_cells += 1;
            signed_wrap_ties_up_differing += r.differing;
            println!(
                "ties_up signed W={} F={} policy=wrap triples={} differing={} witness={:?}",
                $w, $f, r.triples, r.differing, r.witness
            );

            // F4, signed saturating under the other reading.
            let s = model_cell::<Signature<Fi<$w, $f>, Adapt<Floor, Saturate>>>(
                Tie::TowardPositiveInfinity,
                slo,
                shi,
                $f,
            );
            signed_saturate_cells += 1;
            if s.differing == 0 {
                signed_saturate_ties_up_agreeing += 1;
            }
            println!(
                "ties_up signed W={} F={} policy=saturate triples={} differing={} witness={:?}",
                $w, $f, s.triples, s.differing, s.witness
            );

            // F5, the two readings on a non-negative domain.
            unsigned_cells += 1;
            for a in ulo ..= uhi {
                for b in ulo ..= uhi {
                    for c in ulo ..= uhi {
                        let away = model_adapt::<Signature<Ufi<$w, $f>, Adapt<Floor, Wrap>>>(
                            Tie::AwayFromZero,
                            c,
                            a * b,
                            den,
                        );
                        let up = model_adapt::<Signature<Ufi<$w, $f>, Adapt<Floor, Wrap>>>(
                            Tie::TowardPositiveInfinity,
                            c,
                            a * b,
                            den,
                        );
                        if away != up {
                            unsigned_readings_differ += 1;
                        }
                    }
                }
            }
        }};
    }

    cell!(3, 0);
    cell!(3, 1);
    cell!(3, 2);
    cell!(4, 0);
    cell!(4, 1);
    cell!(4, 2);
    cell!(4, 3);
    cell!(5, 0);
    cell!(5, 1);
    cell!(5, 2);
    cell!(5, 3);
    cell!(5, 4);
    cell!(6, 0);
    cell!(6, 1);
    cell!(6, 2);
    cell!(6, 3);
    cell!(6, 4);
    cell!(6, 5);

    println!();
    println!("# --- the arms ---");
    println!(
        "F2 detail: {calibration_positions} positions, {calibration_gap} where the model and the shipped HalfUp disagree"
    );
    verdict(
        "F2",
        "the model under ties-away-from-zero must be the shipped HalfUp",
        calibration_gap == 0 && calibration_positions > 0,
    );
    println!(
        "F3 detail: {separation_hits} positions where ties-toward-positive-infinity differs from the shipped HalfUp, witness={separation_witness:?}"
    );
    verdict(
        "F3",
        "the two readings must be two operations",
        separation_hits > 0,
    );
    println!(
        "F1 detail: {signed_wrap_cells} signed wrapping cells, {signed_wrap_ties_up_differing} differing triples in total"
    );
    verdict(
        "F1",
        "under ties-toward-positive-infinity, signed wrapping must agree at every triple",
        signed_wrap_ties_up_differing == 0 && signed_wrap_cells > 0,
    );
    println!(
        "F4 detail: {signed_saturate_cells} signed saturating cells, {signed_saturate_ties_up_agreeing} of them agreeing"
    );
    verdict(
        "F4",
        "the tie direction must not rescue signed saturating",
        signed_saturate_ties_up_agreeing == 0 && signed_saturate_cells > 0,
    );
    println!(
        "F5 detail: {unsigned_cells} unsigned cells, {unsigned_readings_differ} positions where the two readings differ"
    );
    verdict(
        "F5",
        "on a non-negative domain the two readings must be one function",
        unsigned_readings_differ == 0 && unsigned_cells > 0,
    );
}

fn verdict(name: &str, expectation: &str, held: bool) {
    println!(
        "{name}: {expectation} -> {}",
        if held { "HELD" } else { "BROKEN" }
    );
}
