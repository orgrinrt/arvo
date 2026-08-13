//! P5. What a strategy defined by imitation actually imitates.
//!
//! Three of op's stated intents describe a preference over a cost. I3 does not:
//! it names another system and says behave like it. "It should behave like
//! native primitives in regular old rust would". That is a different KIND of
//! specification, and this probe asks what it denotes at the widths arvo exists
//! to provide.
//!
//! Rust has no primitive at 13 bits, so "behave like the native primitive"
//! has two readings and they are not the same function:
//!
//!   R1 CONTAINER imitation. The value sits in the smallest native container
//!      that fits and behaves exactly like that container. `Uint<13>` wraps at
//!      2^16, because it lives in a u16 and that is what a u16 does.
//!
//!   R2 RULE imitation. Take the rule Rust applies to its own primitives, wrap
//!      at the type's own width, and apply it at the DECLARED width. `Uint<13>`
//!      wraps at 2^13.
//!
//! Part A measures how far apart they are, exhaustively, at every declared
//! width from 1 to 16 for addition and multiplication.
//!
//! Part B is about the half of Rust's behaviour that neither reading can
//! deliver. Rust's `+` panics on overflow in a debug build. That is a runtime
//! check, and I15 is categorical: "Never any runtime checks, ever. We catch
//! invalids on compile time, and unused paths we clear out when lowered."
//! So a strategy defined by imitating Rust cannot imitate Rust's debug
//! behaviour at all, and the probe establishes exactly which region of the
//! behaviour survives.
//!
//! Run: rustc --edition 2024 -O p5_what_warm_imitates.rs -o /tmp/p5 && /tmp/p5

// --------------------------------------------------------------------------
// Part B first, because it is a compile-time claim and must be seen to be one.
// --------------------------------------------------------------------------

/// The declared width the consumer wrote.
const NONNATIVE: u32 = 14; // widths 1-7 and 9-15 of the 16 swept
const W: u32 = 13;
const DECLARED_MODULUS: u64 = 1u64 << W;
/// The container the value physically sits in, if the smallest native one is
/// chosen. 13 bits fits a u16.
const CONTAINER_MODULUS: u64 = 1u64 << 16;

const fn add_rule(a: u64, b: u64) -> u64 {
    (a + b) % DECLARED_MODULUS
}
const fn add_container(a: u64, b: u64) -> u64 {
    (a + b) % CONTAINER_MODULUS
}

/// The compile-time refusal. This is the only shape of overflow detection I15
/// admits, and it is available exactly when both operands are const.
const fn add_or_refuse(a: u64, b: u64) -> u64 {
    let s = a + b;
    assert!(s < DECLARED_MODULUS, "overflow at the declared width");
    s
}

// Const positions, so these are checked by rustc and not by this program.
const OK: u64 = add_or_refuse(4000, 91); // 4091 < 8192
const _: () = assert!(OK == 4091);

// The refusal is real. Uncommenting the next line is a COMPILE error, and the
// companion file p5_mutant_const_overflow.rs is that line on its own, kept so
// the refusal is demonstrated rather than asserted.
// const BAD: u64 = add_or_refuse(8000, 8000);

fn main() {
    println!("P5. What a strategy defined by imitation imitates");
    println!("=================================================");
    println!();
    println!("PART A. The two readings of 'behave like a native primitive'");
    println!("-----------------------------------------------------------");
    println!();
    println!("Exhaustive over every ordered pair in the declared domain, per width.");
    println!("A width is 'native' when Rust has a primitive of exactly that size.");
    println!();
    println!(
        "  {:>2}  {:>9}  {:>12}  {:>12}  {:>12}  {:>12}",
        "W", "native?", "add pairs", "add differ", "mul pairs", "mul differ"
    );

    let mut native_total_diff = 0u64;
    let mut nonnative_widths_with_diff = 0u32;

    for w in 1..=16u32 {
        let decl = 1u64 << w;
        // Smallest native container that holds w bits.
        let cw = if w <= 8 {
            8
        } else if w <= 16 {
            16
        } else {
            32
        };
        let cont = 1u64 << cw;
        let native = w == cw;

        let mut add_pairs = 0u64;
        let mut add_diff = 0u64;
        let mut mul_pairs = 0u64;
        let mut mul_diff = 0u64;

        // Cap the exhaustive sweep so the widest cases stay tractable, and say
        // so rather than quietly sampling: above 2^11 values the sweep steps.
        let step = if decl > 2048 { decl / 2048 } else { 1 };
        let mut a = 0u64;
        while a < decl {
            let mut b = 0u64;
            while b < decl {
                let r1 = (a + b) % cont;
                let r2 = (a + b) % decl;
                add_pairs += 1;
                if r1 != r2 {
                    add_diff += 1;
                }
                let m1 = (a * b) % cont;
                let m2 = (a * b) % decl;
                mul_pairs += 1;
                if m1 != m2 {
                    mul_diff += 1;
                }
                b += step;
            }
            a += step;
        }

        if native {
            native_total_diff += add_diff + mul_diff;
        } else if add_diff + mul_diff > 0 {
            nonnative_widths_with_diff += 1;
        }

        println!(
            "  {:>2}  {:>9}  {:>12}  {:>12}  {:>12}  {:>12}",
            w,
            if native { "yes" } else { "no" },
            add_pairs,
            add_diff,
            mul_pairs,
            mul_diff
        );
        if step != 1 {
            println!("      (stepped by {step}; the sweep is uniform, not a sample of a region)");
        }
    }

    println!();
    println!("  widths where Rust HAS a primitive: the two readings differ on {native_total_diff} of the");
    println!("  pairs swept, which is zero, because at a native width the declared");
    println!("  width IS the container width and there is nothing to disagree about.");
    println!();
    println!("  widths where Rust has NO primitive: {nonnative_widths_with_diff} of {NONNATIVE} show the readings");
    println!("  disagreeing. Those are exactly the widths arvo exists to provide.");
    println!();
    println!("  So 'behave like a native primitive' is a total specification on the");
    println!("  widths Rust already serves and an ambiguous one on the widths it");
    println!("  does not. The imitation is well defined precisely where it is not");
    println!("  needed. Op's own refinement resolves which way it goes: the intent");
    println!("  is the intuitive best choice, and imitation is the vehicle rather");
    println!("  than the requirement.");
    println!();
    println!("PART B. The half of Rust's behaviour that cannot be imitated at all");
    println!("------------------------------------------------------------------");
    println!();
    println!("Rust's `+` on a primitive has two behaviours, not one:");
    println!("  release: wrap at the type's width");
    println!("  debug  : panic on overflow");
    println!();
    println!("The panic is a runtime check. I15 forbids runtime checks without");
    println!("qualification. So a strategy defined by imitating Rust can imitate");
    println!("the release half and can never imitate the debug half.");
    println!();
    println!("What is available instead, and its exact region:");
    println!();
    println!("  const operands  : a compile-time refusal. `add_or_refuse(4000, 91)`");
    println!("                    is a const item in this file and evaluated to {OK}.");
    println!("                    `add_or_refuse(8000, 8000)` in the same position");
    println!("                    does not compile; see p5_mutant_const_overflow.rs");
    println!("                    and its committed output.");
    println!("  runtime operands: nothing. Not a weaker check, not a debug-only");
    println!("                    check. Nothing, by I15.");
    println!();
    println!("That is a predicate rather than a defect: the debug half of the");
    println!("imitation holds where both operands are const-available, and does not");
    println!("hold anywhere else. Op's addendum to I13 is what makes the region");
    println!("worth naming: the admissible category is whatever is available at");
    println!("const time, which is wider than the typestate.");
    println!();
    println!("Demonstrating that the two readings are live at runtime too, at W={W}:");
    let cases = [(4000u64, 91u64), (8000, 8000), (5000, 4000)];
    for (a, b) in cases {
        println!(
            "  a={a:<5} b={b:<5}  rule(mod 2^{W})={:<6} container(mod 2^16)={:<6}  agree={}",
            add_rule(a, b),
            add_container(a, b),
            add_rule(a, b) == add_container(a, b)
        );
    }
}
