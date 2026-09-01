//! Run the bare-primitive lint over the three real crates, with the exemption
//! taken away, and report what it finds in each.
//!
//! What this measures is how much of `arvo-format` the crate-wide exemption is
//! actually carrying. The two crates above it are the control: both are checked
//! today and both must come back at zero, which is what says the number from
//! `arvo-format` is a fact about that crate rather than about the scan being
//! noisy or about the stack being generally dirty.
//!
//! **The case that had to fail.** If `arvo-placement` or `arvo-strategy`
//! reported anything, the instrument would be reporting on ordinary source and
//! the `arvo-format` figure would mean nothing. They are checked at every gate
//! today, so a nonzero there is also a live defect in the repository and would
//! be the finding instead.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use mockspace_extra_lints::lints::arvo_types_only::ArvoTypesOnly;
use mockspace_lint_rules::testkit::LintFixture;
use mockspace_lint_rules::{CrateLint, CrateSourceFile};

/// Every `.rs` under `src/`, root first, which is what the engine hands a
/// `CrateLint`.
fn sources(root: &Path) -> Vec<CrateSourceFile> {
    let mut found = Vec::new();
    walk(root, root, &mut found);
    found.sort_by(|a: &CrateSourceFile, b: &CrateSourceFile| {
        let ar = a.rel_path == *"src/lib.rs";
        let br = b.rel_path == *"src/lib.rs";
        br.cmp(&ar).then_with(|| a.rel_path.cmp(&b.rel_path))
    });
    found
}

fn walk(base: &Path, dir: &Path, out: &mut Vec<CrateSourceFile>) {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };
    for e in entries.flatten() {
        let p = e.path();
        if p.is_dir() {
            walk(base, &p, out);
        } else if p.extension().map(|x| x == "rs").unwrap_or(false) {
            let rel = p.strip_prefix(base).unwrap_or(&p);
            out.push(CrateSourceFile {
                rel_path: PathBuf::from("src").join(rel),
                text: std::fs::read_to_string(&p).unwrap_or_default(),
            });
        }
    }
}

fn report(crate_name: &str, dir: &str, exempt: bool) -> usize {
    let root = PathBuf::from(dir).join("src");
    let files = sources(&root);
    let root_text = files.first().map(|f| f.text.clone()).unwrap_or_default();

    let mut intro = BTreeMap::new();
    if exempt {
        intro.insert(crate_name.to_string(), vec!["numeric".to_string()]);
    }

    let fixture = LintFixture::new(&root_text)
        .with_crate_name(crate_name, crate_name)
        .with_sources(files.clone())
        .with_primitive_introductions(intro);

    let findings = ArvoTypesOnly.check(&fixture.ctx());

    println!(
        "--- {crate_name}  ({} file(s) under src/, exemption {})",
        files.len(),
        if exempt { "APPLIED" } else { "REMOVED" }
    );
    println!("    findings: {}", findings.len());

    let mut per_file: BTreeMap<String, usize> = BTreeMap::new();
    for f in &findings {
        // The message carries "in <rel path> line <n>", which is what the
        // engine would print at the gate.
        let m = &f.message;
        let file = m
            .split(" in ")
            .nth(1)
            .and_then(|s| s.split(" line ").next())
            .unwrap_or("?")
            .to_string();
        *per_file.entry(file).or_default() += 1;
    }
    for (file, n) in &per_file {
        println!("      {n:>3}  {file}");
    }
    println!();
    findings.len()
}

fn main() {
    println!("pack: mockspace-extra-lints @ f34cc1b0c87becd8230d46d807ebb3573e30c009");
    println!("tree: arvo @ the commit this probe is committed on");
    println!();

    println!("=== the two checked crates, as they are gated today ===");
    let placement = report("arvo-placement", "../../../../crates/arvo-placement", false);
    let strategy = report("arvo-strategy", "../../../../crates/arvo-strategy", false);

    println!("=== the introducing crate, as it is gated today ===");
    let exempt = report("arvo-format", "../../../../crates/arvo-format", true);

    println!("=== the introducing crate, with the exemption taken away ===");
    let bare = report("arvo-format", "../../../../crates/arvo-format", false);

    println!("=== verdict ===");
    let controls_held = placement == 0 && strategy == 0 && exempt == 0;
    println!("controls (both checked crates clean, exemption silences): {controls_held}");
    if !controls_held {
        println!("INSTRUMENT INVALID: a control fired.");
    } else {
        println!("The exemption is carrying {bare} line(s) in `arvo-format`.");
        println!("The question names ten of them, which are the ten an outside");
        println!("implementor has to write in its own source.");
    }
}
