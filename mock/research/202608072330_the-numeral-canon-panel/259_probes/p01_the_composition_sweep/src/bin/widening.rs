//! Step 04. The route the first three steps close off, and whether it is open.
//!
//! Steps 01 to 03 hold the format fixed. Every arm is a multiply and an add at
//! one declared signature, and the fused answer is out of reach in a large
//! region. That is a claim about compositions closed at one format, and the
//! ratified spine does not say a composition has to be closed at one format: it
//! says arithmetic on a format is an exact operation in the ambient domain
//! composed with a named total adaptation onto its representable set. Nothing
//! there fixes which format the intermediate lives in.
//!
//! So this step builds the other route. Three declared signatures, three
//! adaptation points, all three the shipped map:
//!
//!   1. a multiply whose result signature is `Fi<2W+1, 2F>`, wide enough to hold
//!      the exact product on its own grid;
//!   2. an add at that wide signature, taking the addend converted onto the wide
//!      grid, which is exact because `2^-F` is a multiple of `2^-2F`;
//!   3. one adaptation from the wide signature down to `Fi<W, F>`, under the
//!      target's own mode and policy.
//!
//! The load-bearing part is that steps 1 and 2 must adapt nothing. If either
//! rounds or completes, the route is not reproducing the fused answer, it is
//! producing a third one. That is measured per triple rather than argued.
//!
//! D1 must hold: the wide multiply and the wide add are the identity at every
//!    triple of every cell, so the only adaptation in the route is the third.
//! D2 must hold: the route reproduces the fused answer at every triple of every
//!    cell, signed saturating included, at every mode and both policies.
//! D3 must fail: the same route through a wide signature one bit too narrow must
//!    disagree with the fused answer somewhere. Without this the width in step 1
//!    is decoration and D2 would hold for a reason nobody established.
//! D4 must fail: the route with the wide add's policy left to saturate at a
//!    narrow wide-width must differ from the wrapping one, so that the wide
//!    signature's own coordinates are doing something rather than being ignored.

use arvo_format::adapt::{Adapt, DeclaredSignature, Signature};
use arvo_format::apply::{Dither, Exact, Fraction, adapt};
use arvo_format::format::Format;
use arvo_format::overflow::{Saturate, Wrap};
use arvo_format::rounding::{Ceil, Floor, HalfEven, HalfUp, TowardZero};
use arvo_format::slots::{Slot, Slots};
use arvo_format::standards::{Fi, Ufi};

/// What one cell of the route returned.
struct Route {
    triples:              u64,
    wide_multiply_moved:  u64,
    wide_add_moved:       u64,
    differing_from_fused: u64,
    witness:              Option<(i64, i64, i64, i64, i64)>,
}

/// The route, at one narrow signature and one wide signature.
///
/// `narrow` is where the answer lands and where the target's mode and policy
/// live. `wide` carries the intermediate. `fraction` is the narrow fraction
/// length, so the wide grid is `2 * fraction` and one narrow slot is `2^fraction`
/// wide slots.
fn route<Narrow, Wide>(fraction: u32) -> Route
where
    Narrow: DeclaredSignature,
    Wide: DeclaredSignature,
{
    let lo = <<Narrow::Format as Format>::Slots as Slots>::MIN.index();
    let hi = <<Narrow::Format as Format>::Slots as Slots>::MAX.index();
    let den = 1i64 << fraction;
    let scale = 1i64 << fraction;

    let mut out = Route {
        triples:              0,
        wide_multiply_moved:  0,
        wide_add_moved:       0,
        differing_from_fused: 0,
        witness:              None,
    };

    for a in lo ..= hi {
        for b in lo ..= hi {
            // Step 1. The exact product in wide slot units is `a * b` exactly,
            // because the wide quantum is the product of the two narrow ones.
            let exact_product = a * b;
            let wide_product =
                adapt::<Wide>(Exact::on_grid(Slot::at(exact_product)), Dither::UNUSED).index();
            let multiply_moved = wide_product != exact_product;

            for c in lo ..= hi {
                out.triples += 1;
                if multiply_moved {
                    out.wide_multiply_moved += 1;
                }
                // Step 2. The addend on the wide grid, and the exact sum.
                let exact_sum = exact_product + c * scale;
                let wide_sum = adapt::<Wide>(
                    Exact::on_grid(Slot::at(wide_product + c * scale)),
                    Dither::UNUSED,
                )
                .index();
                if wide_sum != exact_sum {
                    out.wide_add_moved += 1;
                }

                // Step 3. One adaptation from the wide grid onto the narrow one.
                // A wide slot index `s` is the narrow position `s / 2^fraction`.
                let landed = adapt::<Narrow>(
                    Exact::between(Slot::ZERO, Fraction::of(wide_sum, den)),
                    Dither::UNUSED,
                )
                .index();

                let fused = adapt::<Narrow>(
                    Exact::between(Slot::at(c), Fraction::of(a * b, den)),
                    Dither::UNUSED,
                )
                .index();

                if landed != fused {
                    out.differing_from_fused += 1;
                    if out.witness.is_none() {
                        out.witness = Some((a, b, c, fused, landed));
                    }
                }
            }
        }
    }
    out
}

fn main() {
    println!("# step 04: the fused answer through a widened intermediate");
    println!();

    let mut cells = 0u64;
    let mut wide_moves = 0u64;
    let mut route_differences = 0u64;

    macro_rules! modes {
        ($narrow:ty, $wide:ty, $sign:literal, $w:literal, $f:literal) => {{
            modes!(@one $narrow, $wide, $sign, $w, $f, Floor, "floor");
            modes!(@one $narrow, $wide, $sign, $w, $f, Ceil, "ceil");
            modes!(@one $narrow, $wide, $sign, $w, $f, TowardZero, "toward_zero");
            modes!(@one $narrow, $wide, $sign, $w, $f, HalfUp, "half_up");
            modes!(@one $narrow, $wide, $sign, $w, $f, HalfEven, "half_even");
        }};
        (@one $narrow:ty, $wide:ty, $sign:literal, $w:literal, $f:literal, $m:ident, $mn:literal) => {{
            modes!(@run $narrow, $wide, $sign, $w, $f, $m, $mn, Wrap, "wrap");
            modes!(@run $narrow, $wide, $sign, $w, $f, $m, $mn, Saturate, "saturate");
        }};
        (@run $narrow:ty, $wide:ty, $sign:literal, $w:literal, $f:literal, $m:ident, $mn:literal, $o:ident, $on:literal) => {{
            let r = route::<
                Signature<$narrow, Adapt<$m, $o>>,
                // The intermediate wraps and floors, and D1 measures that neither
                // ever acts. A policy has to be named because a signature has
                // one; naming a policy that never fires is what makes the wide
                // signature an exact carrier rather than a second adaptation.
                Signature<$wide, Adapt<Floor, Wrap>>,
            >($f);
            cells += 1;
            wide_moves += r.wide_multiply_moved + r.wide_add_moved;
            route_differences += r.differing_from_fused;
            println!(
                "route {} W={} F={} mode={} policy={} triples={} wide_mul_moved={} wide_add_moved={} differing={} witness={:?}",
                $sign, $w, $f, $mn, $on,
                r.triples, r.wide_multiply_moved, r.wide_add_moved, r.differing_from_fused, r.witness
            );
        }};
    }

    macro_rules! both {
        ($w:literal, $f:literal, $wide:literal, $widef:literal) => {{
            modes!(Ufi<$w, $f>, Ufi<$wide, $widef>, "unsigned", $w, $f);
            modes!(Fi<$w, $f>, Fi<$wide, $widef>, "signed", $w, $f);
        }};
    }

    both!(3, 0, 7, 0);
    both!(3, 1, 7, 2);
    both!(3, 2, 7, 4);
    both!(4, 0, 9, 0);
    both!(4, 1, 9, 2);
    both!(4, 2, 9, 4);
    both!(4, 3, 9, 6);
    both!(5, 0, 11, 0);
    both!(5, 1, 11, 2);
    both!(5, 2, 11, 4);
    both!(5, 3, 11, 6);
    both!(5, 4, 11, 8);
    both!(6, 0, 13, 0);
    both!(6, 1, 13, 2);
    both!(6, 2, 13, 4);
    both!(6, 3, 13, 6);
    both!(6, 4, 13, 8);
    both!(6, 5, 13, 10);

    println!();
    println!("# --- the arms ---");
    println!("D1 detail: {cells} cells, wide adaptations that moved a value: {wide_moves}");
    verdict(
        "D1",
        "the two wide adaptation points must be the identity everywhere",
        wide_moves == 0 && cells > 0,
    );
    println!(
        "D2 detail: triples where the route disagreed with the fused answer: {route_differences}"
    );
    verdict(
        "D2",
        "the route must reproduce the fused answer at every triple of every cell",
        route_differences == 0,
    );
    arm_d3();
    arm_d4();
}

fn verdict(name: &str, expectation: &str, held: bool) {
    println!(
        "{name}: {expectation} -> {}",
        if held { "HELD" } else { "BROKEN" }
    );
}

/// D3. One bit too narrow, at the cell where the exact product needs the width.
///
/// Signed width 6 needs the intermediate to hold `-32 * -32 = 1024` at fraction
/// zero, which wants eleven signed bits at the least and gets thirteen above.
/// Ten bits reach 511, so the wide multiply completes and the route stops
/// reproducing the fused answer. If it did not, the width would be decoration.
fn arm_d3() {
    let narrow_wide = route::<
        Signature<Fi<6, 0>, Adapt<Floor, Saturate>>,
        Signature<Fi<10, 0>, Adapt<Floor, Wrap>>,
    >(0);
    println!(
        "D3 detail: wide width 10 at signed W=6 F=0: wide_mul_moved={} differing={} witness={:?}",
        narrow_wide.wide_multiply_moved, narrow_wide.differing_from_fused, narrow_wide.witness
    );
    verdict(
        "D3",
        "a wide signature one bit too narrow must break the route",
        narrow_wide.wide_multiply_moved > 0 && narrow_wide.differing_from_fused > 0,
    );
}

/// D4. The wide signature's own policy is read.
///
/// The same too-narrow width under saturate rather than wrap must produce a
/// different failure, which says the route is reading the wide signature's
/// coordinates rather than ignoring them.
fn arm_d4() {
    let wrapping = route::<
        Signature<Fi<6, 0>, Adapt<Floor, Saturate>>,
        Signature<Fi<10, 0>, Adapt<Floor, Wrap>>,
    >(0);
    let saturating = route::<
        Signature<Fi<6, 0>, Adapt<Floor, Saturate>>,
        Signature<Fi<10, 0>, Adapt<Floor, Saturate>>,
    >(0);
    println!(
        "D4 detail: too-narrow wide under wrap differing={}, under saturate differing={}",
        wrapping.differing_from_fused, saturating.differing_from_fused
    );
    verdict(
        "D4",
        "the wide signature's policy must change the failure, so it is being read",
        wrapping.differing_from_fused != saturating.differing_from_fused,
    );
}
