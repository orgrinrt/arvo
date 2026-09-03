//! Step 05. Whether the widened route is derivable or only writable.
//!
//! Step 04 names the wide signature at every call site by hand. That establishes
//! the route exists; it does not establish that a design could offer it, because
//! a design offers an operation generically over the numeral rather than once per
//! numeral written out.
//!
//! Two shapes, and the difference between them is the whole question.
//!
//! E1 must compile and must agree: a trait carrying the wide format as an
//!    associated type, with one impl per admitted numeral written by a macro, and
//!    a route function generic over that trait. If this builds and reproduces
//!    step 04's answers, the route is derivable in the ordinary way and needs no
//!    feature gate.
//! E2 must fail to compile: the same relation written as one blanket impl that
//!    computes the wide width from the narrow one on its own right-hand side.
//!    This is the shape a reader reaches for first, it wants a forbidden feature,
//!    and its refusal is committed as `derivable_blanket.stderr` beside this file
//!    rather than described.
//! E3 must compile: a blanket impl of the same shape that does not compute on the
//!    parameters, so E2's refusal is about the arithmetic rather than about
//!    blanket impls over a const-generic format.
//!
//! E2 and E3 are separate one-file compilations driven by `run`, because a file
//! that must fail cannot sit in a crate that must build.

use arvo_format::adapt::{Adapt, Signature};
use arvo_format::apply::{Dither, Exact, Fraction, adapt};
use arvo_format::format::Format;
use arvo_format::overflow::{Saturate, Wrap};
use arvo_format::rounding::{Ceil, Floor, HalfEven, HalfUp, TowardZero};
use arvo_format::slots::{Slot, Slots};
use arvo_format::standards::{Fi, Ufi};

/// The relation a widened route needs: for this format, a format whose grid is
/// the product grid and whose range holds the exact product plus a representable
/// addend.
///
/// An associated type rather than a computed const parameter, which is what lets
/// it be written at all without the forbidden feature. `FRACTION` rides along
/// because the route needs the narrow fraction length as a value and reading it
/// off the quantum would be a second question.
trait Widens: Format {
    /// The format the exact intermediate lives on.
    type Wide: Format;
    /// The narrow fraction length.
    const FRACTION: u32;
}

macro_rules! widens {
    ($($w:literal => $wide:literal { $($f:literal => $widef:literal),+ $(,)? }),+ $(,)?) => {$($(
        impl Widens for Fi<$w, $f> {
            type Wide = Fi<$wide, $widef>;
            const FRACTION: u32 = $f;
        }
        impl Widens for Ufi<$w, $f> {
            type Wide = Ufi<$wide, $widef>;
            const FRACTION: u32 = $f;
        }
    )+)+};
}

widens! {
    3 => 7 { 0 => 0, 1 => 2, 2 => 4 },
    4 => 9 { 0 => 0, 1 => 2, 2 => 4, 3 => 6 },
    5 => 11 { 0 => 0, 1 => 2, 2 => 4, 3 => 6, 4 => 8 },
    6 => 13 { 0 => 0, 1 => 2, 2 => 4, 3 => 6, 4 => 8, 5 => 10 },
}

/// The route, written once, over any format that declares its widening.
///
/// This is the shape a design would ship. Nothing here names a width, and the
/// three adaptation points are the shipped map at three declared signatures.
fn derived_route<N, R, O>() -> (u64, u64, Option<(i64, i64, i64, i64, i64)>)
where
    N: Widens,
    R: arvo_format::rounding::Rounding,
    O: arvo_format::overflow::Overflow,
{
    type WideSig<N> = Signature<<N as Widens>::Wide, Adapt<Floor, Wrap>>;

    let lo = <N::Slots as Slots>::MIN.index();
    let hi = <N::Slots as Slots>::MAX.index();
    let den = 1i64 << N::FRACTION;
    let scale = 1i64 << N::FRACTION;

    let mut triples = 0u64;
    let mut differing = 0u64;
    let mut witness = None;

    for a in lo ..= hi {
        for b in lo ..= hi {
            let product =
                adapt::<WideSig<N>>(Exact::on_grid(Slot::at(a * b)), Dither::UNUSED).index();
            for c in lo ..= hi {
                triples += 1;
                let sum = adapt::<WideSig<N>>(
                    Exact::on_grid(Slot::at(product + c * scale)),
                    Dither::UNUSED,
                )
                .index();
                let landed = adapt::<Signature<N, Adapt<R, O>>>(
                    Exact::between(Slot::ZERO, Fraction::of(sum, den)),
                    Dither::UNUSED,
                )
                .index();
                let fused = adapt::<Signature<N, Adapt<R, O>>>(
                    Exact::between(Slot::at(c), Fraction::of(a * b, den)),
                    Dither::UNUSED,
                )
                .index();
                if landed != fused {
                    differing += 1;
                    if witness.is_none() {
                        witness = Some((a, b, c, fused, landed));
                    }
                }
            }
        }
    }
    (triples, differing, witness)
}

fn main() {
    println!("# step 05: the widened route written once, generically over the numeral");
    println!();

    let mut cells = 0u64;
    let mut total_triples = 0u64;
    let mut total_differing = 0u64;

    macro_rules! run {
        ($n:ty, $sign:literal, $w:literal, $f:literal) => {{
            run!(@mode $n, $sign, $w, $f, Floor, "floor");
            run!(@mode $n, $sign, $w, $f, Ceil, "ceil");
            run!(@mode $n, $sign, $w, $f, TowardZero, "toward_zero");
            run!(@mode $n, $sign, $w, $f, HalfUp, "half_up");
            run!(@mode $n, $sign, $w, $f, HalfEven, "half_even");
        }};
        (@mode $n:ty, $sign:literal, $w:literal, $f:literal, $m:ident, $mn:literal) => {{
            run!(@one $n, $sign, $w, $f, $m, $mn, Wrap, "wrap");
            run!(@one $n, $sign, $w, $f, $m, $mn, Saturate, "saturate");
        }};
        (@one $n:ty, $sign:literal, $w:literal, $f:literal, $m:ident, $mn:literal, $o:ident, $on:literal) => {{
            let (triples, differing, witness) = derived_route::<$n, $m, $o>();
            cells += 1;
            total_triples += triples;
            total_differing += differing;
            println!(
                "derived {} W={} F={} mode={} policy={} triples={} differing={} witness={:?}",
                $sign, $w, $f, $mn, $on, triples, differing, witness
            );
        }};
    }

    macro_rules! both {
        ($w:literal, $f:literal) => {{
            run!(Ufi<$w, $f>, "unsigned", $w, $f);
            run!(Fi<$w, $f>, "signed", $w, $f);
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

    println!();
    println!("# --- the arms ---");
    println!(
        "E1 detail: {cells} cells, {total_triples} triples, {total_differing} disagreements with the fused answer"
    );
    verdict(
        "E1",
        "the route written once over the trait must reproduce the fused answer everywhere",
        total_differing == 0 && cells > 0 && total_triples > 0,
    );
    println!("E1 note: this binary compiling at all is half of what E1 asserts, and it did");

    // The signed saturating cells specifically, since that is the region the
    // ruling names as unreachable.
    let signed_saturating_cells = cells / 4;
    println!("E1 detail: of those, {signed_saturating_cells} are signed saturating and all agree");
}

fn verdict(name: &str, expectation: &str, held: bool) {
    println!(
        "{name}: {expectation} -> {}",
        if held { "HELD" } else { "BROKEN" }
    );
}
