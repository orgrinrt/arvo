//! The lint, over the two crates of this tree, with no exemption anywhere.
//!
//! `outside` must report zero: it is the crate the ratified open-inventory
//! clause says has to be able to join, and it has no exemption.
//!
//! `door` must report something: it defines primitives in terms of the
//! machine's, which is the one thing the exemption exists for. A zero there
//! would mean the door was not doing its job in this tree and `outside`'s zero
//! would be evidence about a tree with no door in it.

use std::collections::BTreeMap;
use std::path::PathBuf;

use mockspace_lint_rules::testkit::LintFixture;
use mockspace_lint_rules::{CrateLint, CrateSourceFile};
use mockspace_extra_lints::lints::arvo_types_only::ArvoTypesOnly;

fn count(name: &str, path: &str) -> usize {
    let text = std::fs::read_to_string(path).expect("the probe's own source is readable");
    let fixture = LintFixture::new(&text)
        .with_crate_name(name, name)
        .with_sources(vec![CrateSourceFile {
            rel_path: PathBuf::from("src/lib.rs"),
            text: text.clone(),
        }])
        .with_primitive_introductions(BTreeMap::new());
    let findings = ArvoTypesOnly.check(&fixture.ctx());
    println!("--- {name}: {} finding(s), no exemption applied", findings.len());
    for f in findings.iter().take(4) {
        println!("      {}", f.message.lines().next().unwrap_or(""));
    }
    if findings.len() > 4 {
        println!("      ... and {} more", findings.len() - 4);
    }
    findings.len()
}

fn main() {
    let door = count("door", "../door/src/lib.rs");
    let outside = count("outside", "../outside/src/lib.rs");
    println!();
    println!("=== verdict ===");
    if door == 0 {
        println!("CONTROL FAILED: the door reported nothing, so this tree has no door");
        println!("in it and the outside crate's zero says nothing.");
        std::process::exit(1);
    }
    println!("control: the door reports {door}, so the scan reaches this tree.");
    if outside == 0 {
        println!("An outside crate declares a format of its own with no bare primitive");
        println!("on any line, against a door that carries them all.");
    } else {
        println!("The outside crate still reports {outside}. The shape does not close it.");
        std::process::exit(1);
    }
}
