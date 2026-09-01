//! What the bare-primitive lint does to an outside crate that implements
//! `arvo-format`'s five public traits, and what it does to the same impl once
//! the constants take types.
//!
//! The question this is built for asks what the numeric-introduction door may
//! carry out. Its `unblocks` field asserts "Ten hard errors today, one per
//! associated constant, in any crate that writes an impl". That is a claim
//! about an instrument, so it is run rather than reasoned about.
//!
//! **The case that had to fail before any number here counted.** Four controls,
//! and every one of them would have caught an instrument that merely says yes:
//!
//!   C1  a source with no bare primitive at all, in a checked crate -> 0.
//!       If this reported anything the scan fires on everything.
//!   C2  the const generic parameter position, in a checked crate -> 0.
//!       If this reported, the ten findings would not be evidence about
//!       associated constants specifically, since `u32` appears in both.
//!   C3  the shipped impl under the `[primitive-introductions]` map that names
//!       the crate -> 0. If this still reported, the exemption is not what
//!       silences the introducing crate and the whole question is misframed.
//!   C4  the repaired impl in a checked crate -> 0. If this reported, the
//!       repair does not repair and arm A's number means nothing.
//!
//! Arm A is the shipped impl in a checked crate. Its expected value is 10.

use std::collections::BTreeMap;

use mockspace_lint_rules::testkit::LintFixture;
use mockspace_lint_rules::CrateLint;
use mockspace_extra_lints::lints::arvo_types_only::ArvoTypesOnly;
use mockspace_extra_lints::lints::no_bare_numeric::NoBareNumeric;

/// The impl an outside crate has to write today, against the shipped traits.
///
/// Copied position for position from `mock/crates/arvo-format/src/`: `Format`
/// (`format.rs`), `Ambient` (`ambient.rs`), `Quantum` (`quantum.rs`), `Slots`
/// (`slots.rs`) and `Operation` (`adapt.rs`). Ten associated constants, each on
/// its own line, because the lint reports at most once per line and the count
/// is the thing being measured.
const IMPL_AS_SHIPPED: &str = r#"
use arvo_format::{Ambient, Format, Operation, Quantum, Slots, Width};

pub struct MyDomain;
impl Ambient for MyDomain {
    const RADIX: u32 = 2;
    const SIGNED: bool = true;
}

pub struct MyLaw;
impl Quantum for MyLaw {
    const BASE: i32 = -4;
    const SLOPE: i32 = 0;
    const MAGNITUDES: u32 = 1;
}

pub struct MyRange;
impl Slots for MyRange {
    const MIN: i64 = -128;
    const MAX: i64 = 127;
    const WIDTH: Width = Width::bits(8);
}

pub struct MyFormat;
impl Format for MyFormat {
    type Ambient = MyDomain;
    type Quantum = MyLaw;
    type Slots = MyRange;
    const PHASE_NUM: i64 = 0;
    const PHASE_DEN: i64 = 1;
}

pub struct MyAdd;
impl Operation for MyAdd {
    type Signature = MySignature;
    const ARITY: u32 = 2;
}
"#;

/// The same impl with every associated constant carrying a type of the door's
/// own. Nothing else about the impl changes, so the difference between this and
/// the arm above is exactly the retyping and nothing else.
///
/// The type names are the coordinates of the ratified parameterisation. Whether
/// those are the right names is a design question; whether the retyping clears
/// the lint is the question here, and only the second is measured.
const IMPL_REPAIRED: &str = r#"
use arvo_format::{Ambient, Arity, Bool, Exponent, Format, MagnitudeCount,
                  Operation, Phase, Quantum, Radix, Slot, Slots, Width};

pub struct MyDomain;
impl Ambient for MyDomain {
    const RADIX: Radix = Radix::of(2);
    const SIGNED: Bool = Bool::TRUE;
}

pub struct MyLaw;
impl Quantum for MyLaw {
    const BASE: Exponent = Exponent::of(-4);
    const SLOPE: Exponent = Exponent::ZERO;
    const MAGNITUDES: MagnitudeCount = MagnitudeCount::of(1);
}

pub struct MyRange;
impl Slots for MyRange {
    const MIN: Slot = Slot::at(-128);
    const MAX: Slot = Slot::at(127);
    const WIDTH: Width = Width::bits(8);
}

pub struct MyFormat;
impl Format for MyFormat {
    type Ambient = MyDomain;
    type Quantum = MyLaw;
    type Slots = MyRange;
    const PHASE: Phase = Phase::of(0, 1);
}

pub struct MyAdd;
impl Operation for MyAdd {
    type Signature = MySignature;
    const ARITY: Arity = Arity::of(2);
}
"#;

/// C1. A crate that names no machine type anywhere.
const CONTROL_CLEAN: &str = r#"
use arvo_format::{Format, Width};

pub struct Carrier;
impl Carrier {
    pub const fn width() -> Width {
        Width::bits(8)
    }
}
"#;

/// C2. The one position op excepted, in a checked crate.
///
/// `u32` is the same token the arm above is counted on, so if the exception
/// did not hold here the ten would not be a fact about associated constants.
const CONTROL_CONST_GENERIC: &str = r#"
pub struct Signed<const BITS: u32>;
pub struct Biased<const EXP: i32, const PHASE: i64>;
"#;

/// The map `mockspace.toml [primitive-introductions]` declares.
fn introducing() -> BTreeMap<String, Vec<String>> {
    let mut m = BTreeMap::new();
    m.insert("arvo-format".to_string(), vec!["numeric".to_string()]);
    m
}

/// Run both pack lints over one planted source under one introductions map.
///
/// Both are run because `no_bare_numeric.rs` says the two are "semantically
/// equivalent today" and a claim of equivalence is cheaper to check than to
/// trust. A disagreement between them would itself be the finding.
fn run(label: &str, crate_name: &str, source: &str, intro: BTreeMap<String, Vec<String>>) -> usize {
    let fixture = LintFixture::new(source)
        .with_crate_name(crate_name, crate_name)
        .with_primitive_introductions(intro);
    let ctx = fixture.ctx();

    let a = ArvoTypesOnly.check(&ctx);
    let n = NoBareNumeric.check(&ctx);

    println!("--- {label}");
    println!("    crate_name              = {crate_name}");
    println!("    arvo-types-only         = {} finding(s)", a.len());
    println!("    no-bare-numeric         = {} finding(s)", n.len());
    if a.len() != n.len() {
        println!("    !! the two lints disagree, which the pack's own doc says cannot happen");
    }
    for e in &a {
        println!("      {}", e.message.lines().next().unwrap_or(""));
    }
    a.len()
}

fn main() {
    println!("pack: mockspace-extra-lints @ f34cc1b0c87becd8230d46d807ebb3573e30c009");
    println!("rules: mockspace-lint-rules @ a9268f62943c317ea4f5ee4279b528ffdb4ae936");
    println!();

    let c1 = run("C1 clean source, checked crate", "outside-crate", CONTROL_CLEAN, BTreeMap::new());
    let c2 = run("C2 const generic parameters, checked crate", "outside-crate", CONTROL_CONST_GENERIC, BTreeMap::new());
    let c3 = run("C3 shipped impl, introducing crate", "arvo-format", IMPL_AS_SHIPPED, introducing());
    let c4 = run("C4 repaired impl, checked crate", "outside-crate", IMPL_REPAIRED, BTreeMap::new());
    let arm_a = run("A  shipped impl, checked crate", "outside-crate", IMPL_AS_SHIPPED, BTreeMap::new());

    println!();
    println!("=== verdict ===");
    let controls_held = c1 == 0 && c2 == 0 && c3 == 0 && c4 == 0;
    println!("controls C1..C4 all zero: {controls_held}");
    println!("arm A findings: {arm_a}");
    if !controls_held {
        println!("INSTRUMENT INVALID: a control fired, so arm A's number is not evidence.");
    } else if arm_a == 10 {
        println!("The row's `unblocks` holds: ten hard errors, one per associated constant.");
    } else {
        println!("The row's `unblocks` does NOT hold at this count.");
    }
}
