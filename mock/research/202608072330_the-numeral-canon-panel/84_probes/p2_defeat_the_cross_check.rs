//! p2: 80's section 4.3 mechanism, reproduced exactly, licensing a false law.
//!
//! The construction is `80_probes/p2c_closed_form_checked_on_a_model.rs` with one
//! substitution: the law is L_16, "every product of 16 consecutive integers
//! vanishes at this width", in the same wrapping fragment, arity 1 (so the
//! sweep is CHEAPER than p2c's arity-3 sweep and the band could even be wider).
//!
//! The closed form an author would plausibly write is `true` at every width:
//! sixteen consecutive integers contain eight even numbers, four multiples of
//! four, and so on, so the product "is divisible by a large power of two". That
//! reasoning is right and its conclusion is wrong: the power is exactly
//! v2(16!) = 15, so the law is true at widths 1..=15 and false from 16 up
//! (p1_threshold_family.rs, verified against exhaustive sweeps).
//!
//! Everything the mechanism checks, passes:
//!   - the agreement between the closed form and the swept verdict over the
//!     model band (widths 2..=5, p2c's own band) holds, at compile time;
//!   - the perturbation control still bites: `--cfg badclosed` makes the closed
//!     form disagree inside the band and the build is refused;
//!   - the arm gated on the closed form at width 64 is licensed.
//!
//! And the licensed law is FALSE at the gated width:
//!   - `--cfg audit` adds one top-level const asserting the law at width 64 on
//!     the single witness x = 16. It refuses in constant work (16 multiplies),
//!     printing the witness. The mechanism never runs this check, because the
//!     mechanism has no idea the witness exists.
//!
//!   rustc --edition 2021 -O p2_defeat_the_cross_check.rs -o p2      (licenses)
//!   rustc --edition 2021 -O --cfg badclosed p2_... .rs              (refused, control)
//!   rustc --edition 2021 -O --cfg audit p2_... .rs                  (refused: the law is false at 64)
//!
//! Toolchain: pinned nightly-2026-05-28. No feature gates.

const MODEL_MIN_W: u32 = 2;
const MODEL_MAX_W: u32 = 5; // p2c's own band

const K: u64 = 16;

const fn mask_of(w: u32) -> u64 {
    if w >= 64 {
        u64::MAX
    } else {
        (1u64 << w) - 1
    }
}

/// The law's left-hand side through the map: (x)(x-1)...(x-15) wrapping at width w.
const fn falling_prod(x: u64, w: u32) -> u64 {
    let m = mask_of(w);
    let mut acc: u64 = 1;
    let mut i: u64 = 0;
    while i < K {
        acc = acc.wrapping_mul(x.wrapping_sub(i) & m) & m;
        i += 1;
    }
    acc
}

/// The closed-form verdict an arm gates on. Constant time, any width.
/// The (wrong) generalisation: "the product of sixteen consecutive integers is
/// divisible by a huge power of two, so it vanishes at every width".
const fn closed_verdict(_w: u32) -> bool {
    #[cfg(badclosed)]
    {
        // the perturbation control: one band entry made to lie
        if _w == 4 {
            return false;
        }
    }
    true
}

/// The swept verdict through the map. Arity 1, so this is cheap; the whole
/// band costs under a thousand const-eval multiplies.
const fn swept_verdict(w: u32) -> bool {
    let n = 1u64 << w;
    let mut x = 0u64;
    while x < n {
        if falling_prod(x, w) != 0 {
            return false;
        }
        x += 1;
    }
    true
}

const fn closed_agrees_with_sweep() -> bool {
    let mut w = MODEL_MIN_W;
    while w <= MODEL_MAX_W {
        if closed_verdict(w) != swept_verdict(w) {
            return false;
        }
        w += 1;
    }
    true
}

const AGREEMENT: () = assert!(
    closed_agrees_with_sweep(),
    "the closed-form law verdict disagrees with the swept verdict somewhere in the \
     model band, so the closed form is wrong and no arm may be gated on it"
);

/// An arm gated on the closed verdict at a width no sweep can reach.
struct Numeral64;

trait Licensed {
    const CHECK: ();
}
impl Licensed for Numeral64 {
    const CHECK: () = {
        let () = AGREEMENT;
        assert!(
            closed_verdict(64),
            "the law does not hold at this width, so the consumer that rewrites \
             through it may not be instantiated"
        );
    };
}

fn consumer_that_rewrites_through_the_law<L: Licensed>() -> &'static str {
    let () = <L as Licensed>::CHECK;
    "licensed"
}

/// The audit the mechanism does not contain: the law itself, at the gated
/// width, on the one witness. Sixteen multiplies. Refuses under --cfg audit.
#[cfg(audit)]
const AUDIT: () = assert!(
    falling_prod(16, 64) == 0,
    "the licensed law is FALSE at the gated width: the product of the sixteen \
     consecutive integers ending at x = 16 is 16! = 2^15 * odd, nonzero mod 2^64"
);

fn main() {
    println!("p2: 80's cross-check mechanism licensing a false law");
    println!("  model band: widths {}..={}", MODEL_MIN_W, MODEL_MAX_W);
    println!(
        "  agreement over the band, evaluated at compile time: {}",
        const { closed_agrees_with_sweep() }
    );
    println!(
        "  closed_verdict(w = 64) = {}   (constant time, no enumeration)",
        const { closed_verdict(64) }
    );
    println!(
        "  consumer_that_rewrites_through_the_law::<Numeral64>() = {}",
        consumer_that_rewrites_through_the_law::<Numeral64>()
    );
    println!();
    // The refutation, visible at runtime in the same binary the mechanism
    // licensed: swept verdicts up to width 18.
    println!("  swept verdict per width (runtime, exhaustive):");
    for w in 2..=18u32 {
        let n = 1u64 << w;
        let mut wit: Option<u64> = None;
        let mut x = 0u64;
        while x < n {
            if falling_prod(x, w) != 0 {
                wit = Some(x);
                break;
            }
            x += 1;
        }
        match wit {
            None => println!("    w = {:>2}: true", w),
            Some(x0) => println!("    w = {:>2}: FALSE, witness x = {}", w, x0),
        }
    }
    println!();
    println!("  the band (and every band ending below width 16) agrees with the");
    println!("  closed form; the arm is licensed at width 64; the law is false");
    println!("  there. Build with --cfg audit for the compile-time refutation.");
}
