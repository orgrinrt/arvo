//! q5. What `arvo_format::apply::adapt` is a map onto.
//!
//! `ruling::the_format_spine_is_canon`: "Arithmetic on a format is an exact
//! operation in the ambient domain composed with a named total adaptation onto
//! that set". So the adaptation's codomain is the representable set. The shipped
//! `adapt` has signature `const fn adapt<S: DeclaredSignature>(Exact, Dither) ->
//! i64` and its doc says "Total: every position returns a slot the format admits".
//!
//! A slot is not a member of the set. The set is indexed by a slot AND a
//! magnitude: `value = phase + slot * quantum(magnitude)`. So the question is
//! whether the returned slot determines a member, and it does exactly when there
//! is one magnitude to choose from.
//!
//! **The cases that must fail, stated before the run.** The instrument counts, for
//! each format, how many members of the representable set share a returned slot.
//! For a format with one magnitude that count must be one, everywhere, or the
//! instrument is miscounting and the finding on the many-magnitude formats means
//! nothing. And at least one format must come out with a count above one, or the
//! sweep never reaches the case in question.
//!
//! `apply.rs` is checked for whether it reads the magnitude at all, by grep,
//! recorded in `RUN.md` rather than asserted here.
//!
//! Build, after `cargo build -p arvo-format`, from the mock workspace root:
//!
//! ```text
//! rustc --edition 2024 -O \
//!   --extern arvo_format=target/debug/deps/libarvo_format-<hash>.rlib \
//!   -L target/debug/deps \
//!   240_probes/q5_the_adaptations_codomain.rs -o /tmp/q5
//! ```

use arvo_format::adapt::{Adapt, Signature};
use arvo_format::ambient::Ambient;
use arvo_format::apply::{adapt, Dither, Exact};
use arvo_format::format::Format;
use arvo_format::overflow::Saturate;
use arvo_format::points::{Biased, Floating, Integer, UFixed};
use arvo_format::quantum::Quantum;
use arvo_format::rounding::HalfEven;
use arvo_format::slots::Slots;

/// Every member of the representable set, as (slot, magnitude, value in units of
/// the quantum at magnitude zero, scaled by the phase denominator so it is an
/// integer).
fn members<F: Format>() -> Vec<(i64, u32, i128)> {
    let radix = <F::Ambient as Ambient>::RADIX as i128;
    let slope = <F::Quantum as Quantum>::SLOPE;
    let mags = <F::Quantum as Quantum>::MAGNITUDES;
    let min = <F::Slots as Slots>::MIN;
    let max = <F::Slots as Slots>::MAX;
    let pn = F::PHASE_NUM as i128;
    let pd = F::PHASE_DEN as i128;

    let mut out = Vec::new();
    for m in 0..mags {
        let e = slope * (m as i32);
        assert!(e >= 0, "the probe assumes a non-negative shell exponent");
        let step = radix.pow(e as u32);
        for s in min..=max {
            // value * pd, in units of q(0), so it is an exact integer.
            out.push((s, m, pn + s as i128 * step * pd));
        }
    }
    out
}

/// How many distinct values share the worst-case slot, and which slot that is.
fn worst_slot_ambiguity<F: Format>() -> (i64, usize) {
    let ms = members::<F>();
    let min = <F::Slots as Slots>::MIN;
    let max = <F::Slots as Slots>::MAX;
    let mut worst = (min, 0usize);
    for s in min..=max {
        let mut vals: Vec<i128> = ms.iter().filter(|r| r.0 == s).map(|r| r.2).collect();
        vals.sort();
        vals.dedup();
        if vals.len() > worst.1 {
            worst = (s, vals.len());
        }
    }
    worst
}

fn report<F: Format>(name: &str) -> usize {
    let mags = <F::Quantum as Quantum>::MAGNITUDES;
    let min = <F::Slots as Slots>::MIN;
    let max = <F::Slots as Slots>::MAX;
    let ms = members::<F>();
    let mut distinct: Vec<i128> = ms.iter().map(|r| r.2).collect();
    distinct.sort();
    distinct.dedup();
    let (slot, amb) = worst_slot_ambiguity::<F>();
    println!(
        "   {name:<26} magnitudes {mags:<3} slots [{min}, {max}]  |set| {:<5} worst slot {slot} \
         denotes {amb} values",
        distinct.len()
    );
    amb
}

fn main() {
    println!("== how many members of the representable set share one slot ==\n");

    let mut one_magnitude_worst = 0usize;
    one_magnitude_worst = one_magnitude_worst.max(report::<Integer<4>>("Integer<4>"));
    one_magnitude_worst = one_magnitude_worst.max(report::<Integer<8>>("Integer<8>"));
    one_magnitude_worst = one_magnitude_worst.max(report::<UFixed<5, -2>>("UFixed<5,-2>"));
    one_magnitude_worst = one_magnitude_worst.max(report::<Biased<5, -1, 1>>("Biased<5,-1,1>"));

    println!();
    let mut many_magnitude_worst = 0usize;
    many_magnitude_worst = many_magnitude_worst.max(report::<Floating<3, -2, 3>>("Floating<3,-2,3>"));
    many_magnitude_worst = many_magnitude_worst.max(report::<Floating<4, -3, 5>>("Floating<4,-3,5>"));
    many_magnitude_worst = many_magnitude_worst.max(report::<Floating<5, 0, 8>>("Floating<5,0,8>"));

    println!("\n   controls:");
    println!("     worst ambiguity among one-magnitude formats: {one_magnitude_worst}");
    println!("     worst ambiguity among many-magnitude formats: {many_magnitude_worst}");
    if one_magnitude_worst != 1 {
        println!("\n   CONTROL FAILED: a one-magnitude format came out ambiguous, so the counting");
        println!("   is wrong and nothing below counts.");
        std::process::exit(2);
    }
    if many_magnitude_worst <= 1 {
        println!("\n   CONTROL FAILED: no format reached the ambiguous case.");
        std::process::exit(2);
    }
    println!("   controls hold.");

    // What `adapt` actually returns, on a position past the top of the range.
    println!("\n== what `adapt` returns on a position past the top of the range ==\n");

    type FloatSig = Signature<Floating<3, -2, 3>, Adapt<HalfEven, Saturate>>;
    type IntSig = Signature<Integer<4>, Adapt<HalfEven, Saturate>>;

    let far_out = Exact::on_grid(1000);
    let float_slot = adapt::<FloatSig>(far_out, Dither::UNUSED);
    let int_slot = adapt::<IntSig>(far_out, Dither::UNUSED);

    println!("   Exact::on_grid(1000), saturating:");
    println!("     Signature<Integer<4>, ...>        -> slot {int_slot}");
    println!("     Signature<Floating<3,-2,3>, ...>  -> slot {float_slot}");

    let radix = 2i128;
    println!(
        "\n   For `Integer<4>` that slot names one value: {} times the quantum at the only\n   \
         magnitude there is.",
        int_slot
    );
    println!("\n   For `Floating<3,-2,3>` that same slot names {} different values,", 3);
    for m in 0..3u32 {
        let step = radix.pow(m);
        println!(
            "     at magnitude {m}: {} quanta of q(0) = {} * 2^-2",
            float_slot as i128 * step,
            float_slot as i128 * step
        );
    }
    println!(
        "\n   Nothing in the returned `i64` says which, and `apply.rs` reads no magnitude:\n   \
         `MAGNITUDES` appears nowhere in it and the two occurrences of the word `magnitude`\n   \
         are both in doc comments. `RUN.md` carries the grep."
    );

    println!("\n== the statement this supports ==\n");
    println!(
        "   `adapt` is a total map onto the representable set exactly at\n   \
         `magnitudes = 1`. Above it the codomain is the slot range, which is a\n   \
         projection of the set rather than the set, so the ratified factoring's\n   \
         second half is realised for the constant-quantum family and not for the\n   \
         magnitude-indexed one."
    );
    println!("\n   findings: 1");
    std::process::exit(1);
}
