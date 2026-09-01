//! Take `arvo-placement`'s real sources, append the open-inventory test's own
//! text to them, and run the gate.
//!
//! The test text is lifted verbatim from
//! `mock/crates/arvo-format/src/tests.rs`, the `Ternary` block. Nothing about
//! it is rewritten, because the question is what the existing test costs on the
//! other side of the door rather than what a better one would cost.
//!
//! **The case that had to fail.** `arvo-placement` unmodified must report zero,
//! or the appended text is not what the findings are about. That control is the
//! first arm.

use std::collections::BTreeMap;
use std::path::PathBuf;

use mockspace_extra_lints::lints::arvo_types_only::ArvoTypesOnly;
use mockspace_lint_rules::testkit::LintFixture;
use mockspace_lint_rules::{CrateLint, CrateSourceFile};

/// Verbatim from `arvo-format/src/tests.rs`, the block under
/// "--- a format the crate does not know about".
const TERNARY: &str = r#"
struct Ternary;

impl Format for Ternary {
    type Ambient = DecimalRationals;
    type Quantum = Constant<-1>;
    type Slots = Signed<3>;
    const PHASE_NUM: i64 = 0;
    const PHASE_DEN: i64 = 1;
}

#[test]
fn the_format_inventory_admits_a_member_this_crate_does_not_know_about() {
    assert_eq!(radix::<Ternary>(), 10);
    assert_eq!(step_exponent::<Ternary>(0), -1);
    assert!(has_additive_identity::<Ternary>());
    assert!(contains::<Ternary>(0, 0));
    assert!(!contains::<Ternary>(4, 0));
}
"#;

fn placement_sources() -> Vec<CrateSourceFile> {
    ["src/lib.rs", "src/tests.rs"]
        .iter()
        .map(|rel| CrateSourceFile {
            rel_path: PathBuf::from(rel),
            text: std::fs::read_to_string(
                PathBuf::from("../../../../crates/arvo-placement").join(rel),
            )
            .expect("arvo-placement's sources are readable"),
        })
        .collect()
}

fn run(label: &str, files: Vec<CrateSourceFile>) -> usize {
    let root = files[0].text.clone();
    let fixture = LintFixture::new(&root)
        .with_crate_name("arvo-placement", "arvo-placement")
        .with_sources(files)
        .with_primitive_introductions(BTreeMap::new());
    let findings = ArvoTypesOnly.check(&fixture.ctx());
    println!("--- {label}: {} finding(s)", findings.len());
    for f in &findings {
        println!("      {}", f.message.lines().next().unwrap_or(""));
    }
    findings.len()
}

mod extra;

fn main() {
    println!("=== control: arvo-placement as it is ===");
    let control = run("unmodified", placement_sources());
    println!();

    println!("=== the same crate, with the open-inventory test appended ===");
    let mut with = placement_sources();
    with[1].text.push_str(TERNARY);
    let moved = run("with the `Ternary` block", with);
    println!();

    println!("=== the same crate, with a format that borrows nothing ===");
    let mut whole = placement_sources();
    whole[1].text.push_str(extra::WHOLE_FORMAT);
    let all = run("with every trait implemented outside", whole);
    println!();
    println!("=== verdict ===");
    if control != 0 {
        println!("CONTROL FAILED: the crate reports {control} before anything is added.");
        std::process::exit(1);
    }
    println!("control: the crate reports nothing before the block is added.");
    if moved == 0 {
        println!("Moving the test changes nothing, so it would not enforce the ten.");
    } else {
        println!("Moving the test turns the ratified open-inventory clause into {moved}");
        println!("gate finding(s) that stay red until the coordinates are typed.");
        println!("No new lint is needed for the exported half of the rule; the");
        println!("existing one enforces it as soon as the test is on the side that");
        println!("has no exemption.");
        println!();
        println!("And `Ternary` reaches two of the ten because it borrows this crate's");
        println!("own ambient, quantum and slot range. A format that borrows nothing");
        println!("reaches {all}, which is the whole of what an outside implementor writes.");
    }
}
