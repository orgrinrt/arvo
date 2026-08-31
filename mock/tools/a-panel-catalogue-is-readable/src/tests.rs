//! Every refusal planted, and every correct shape planted beside it.
//!
//! The four readers each walk a document, and a walk that silently found no rows
//! would report every catalogue clean. That is the shape this corpus has already
//! produced twice: an arm keyed on a string that matched nothing, reporting clean
//! over a corpus it never reached. So each arm here plants the defect it looks
//! for and requires the reader to find it.

use std::path::{Path, PathBuf};

use mockspace::Lint;

use super::{catalogues, check, rows, APanelCatalogueIsReadable};

/// A mock directory with the catalogues named, under the first declared root.
///
/// Removed and recreated on entry rather than on exit, because a test that fails
/// leaves its tree behind and that is the tree somebody then wants to look at.
fn tree(what: &str, files: &[(&str, &str)]) -> PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "arvo-catalogue-{what}-{}-{:?}",
        std::process::id(),
        std::thread::current().id()
    ));
    let _ = std::fs::remove_dir_all(&dir);
    let root = dir.join("research/202608072330_the-numeral-canon-panel");
    std::fs::create_dir_all(&root).expect("a planted tree");
    for (name, text) in files {
        std::fs::write(root.join(name), text).expect("a planted catalogue");
    }
    dir
}

/// The finding kinds one planted tree produces.
fn kinds(what: &str, files: &[(&str, &str)]) -> Vec<Option<&'static str>> {
    check(&tree(what, files))
        .into_iter()
        .map(|e| e.finding_kind)
        .collect()
}

/// A catalogue with nothing wrong in it, so every arm has a silent case.
const SOUND: &str = "[[item]]\nid = \"a\"\nconfidence = 0.5\n\n[[item]]\nid = \"b\"\n";

#[test]
fn a_catalogue_that_does_not_parse_is_reported() {
    let k = kinds(
        "parse",
        &[("broken.toml", "[[item]]\nid = \"unterminated\n")],
    );
    assert_eq!(k, [Some("a-catalogue-does-not-parse")], "{k:?}");
}

#[test]
fn control_a_sound_catalogue_is_silent() {
    assert!(kinds("sound", &[("fine.toml", SOUND)]).is_empty());
}

#[test]
fn a_row_carrying_none_of_the_identity_keys_is_reported() {
    let k = kinds(
        "anonymous",
        &[("c.toml", "[[item]]\nverdict = \"absorbed\"\n")],
    );
    assert_eq!(k, [Some("a-catalogue-row-cannot-be-named")], "{k:?}");
}

#[test]
fn every_identity_key_names_a_row_rather_than_only_the_first() {
    // Three spellings of one property, and a reader that accepted `id` alone
    // would report every file-keyed catalogue as anonymous. That was the first
    // version of this check and a catalogue corrected it.
    for key in ["id", "path", "file"] {
        let text = format!("[[entry]]\n{key} = \"a\"\n");
        assert!(
            kinds(&format!("named-{key}"), &[("c.toml", &text)]).is_empty(),
            "`{key}` is an identity and is not accepted as one"
        );
    }
}

#[test]
fn two_rows_claiming_one_id_are_reported() {
    let k = kinds(
        "dup",
        &[("c.toml", "[[item]]\nid = \"a\"\n\n[[item]]\nid = \"a\"\n")],
    );
    assert_eq!(k, [Some("two-rows-claim-one-identifier")], "{k:?}");
}

#[test]
fn control_one_file_carried_three_times_is_not_a_duplicate() {
    // `file` and `path` name a subject rather than a row, and a catalogue
    // legitimately splits one panel file into three rows with three verdicts.
    // The second version of this check demanded uniqueness of whatever key it
    // found and broke on exactly that, which is why only `id` promises it.
    let text = "[[entry]]\nfile = \"42.md\"\nverdict = \"absorbed\"\n\n\
                [[entry]]\nfile = \"42.md\"\nverdict = \"carried\"\n\n\
                [[entry]]\nfile = \"42.md\"\nverdict = \"dropped\"\n";
    assert!(kinds("split", &[("c.toml", text)]).is_empty());
}

#[test]
fn duplicate_ids_are_counted_per_file_rather_than_across_the_tree() {
    // Two catalogues about different subjects may each mint a row named after
    // the same thing. A reader sharing one set across files would report a
    // duplicate that is not one, and every arm above plants a single file.
    let one = "[[item]]\nid = \"a\"\n";
    assert!(kinds("percfile", &[("x.toml", one), ("y.toml", one)]).is_empty());
}

#[test]
fn a_confidence_outside_zero_to_one_is_reported_and_one_inside_is_not() {
    for bad in ["1.5", "-0.2", "42", "\"high\""] {
        let text = format!("[[item]]\nid = \"a\"\nconfidence = {bad}\n");
        let k = kinds(&format!("conf-bad-{bad}"), &[("c.toml", &text)]);
        assert_eq!(k, [Some("a-confidence-is-out-of-range")], "{bad} passed");
    }
    for good in ["0.0", "1.0", "0.37", "0", "1"] {
        let text = format!("[[item]]\nid = \"a\"\nconfidence = {good}\n");
        assert!(
            kinds(&format!("conf-good-{good}"), &[("c.toml", &text)]).is_empty(),
            "{good} was refused"
        );
    }
}

#[test]
fn control_a_row_scoring_nothing_is_left_alone() {
    // Optional by design: a catalogue need not score its rows, and a reader
    // demanding one would report most of the corpus.
    assert!(kinds("noconf", &[("c.toml", "[[item]]\nid = \"a\"\n")]).is_empty());
}

#[test]
fn a_table_under_any_key_is_walked_rather_than_one_the_reader_knows() {
    // `[[item]]` and `[[entry]]` both appear and nothing says a later catalogue
    // has to pick either. A reader keyed on one covers nothing the day somebody
    // picks a different word, and reports clean while doing it.
    for key in ["item", "entry", "something_nobody_has_used_yet"] {
        let text = format!("[[{key}]]\nverdict = \"absorbed\"\n");
        let k = kinds(&format!("key-{key}"), &[("c.toml", &text)]);
        assert_eq!(
            k,
            [Some("a-catalogue-row-cannot-be-named")],
            "`[[{key}]]` was not walked"
        );
    }
}

#[test]
fn a_tree_holding_no_catalogue_at_all_is_the_finding_rather_than_a_pass() {
    // Every other check here runs over the population, so an empty one makes all
    // of them vacuous while each still passes. That is the shape the whole
    // migration exists to refuse.
    let k = kinds("empty", &[]);
    assert_eq!(k, [Some("no-catalogue-found")], "{k:?}");
}

#[test]
fn a_non_toml_file_beside_a_catalogue_is_not_read_as_one() {
    let dir = tree("nontoml", &[("fine.toml", SOUND)]);
    std::fs::write(
        dir.join("research/202608072330_the-numeral-canon-panel/notes.md"),
        "not a catalogue at all [[[",
    )
    .expect("a planted note");
    assert!(check(&dir).is_empty());
}

#[test]
fn every_declared_root_is_walked() {
    // Three roots, and a walk reading the first alone would cover the panel
    // directory and miss `mock/registry_catalogue` entirely, which is where one
    // of the committed catalogues sits. Every arm above plants into the first.
    for root in [
        "research/202608072330_the-numeral-canon-panel",
        "research/202608072330_the-numeral-canon-panel/catalogue",
        "registry_catalogue",
    ] {
        let dir = std::env::temp_dir().join(format!(
            "arvo-catalogue-root-{}-{:?}",
            std::process::id(),
            std::thread::current().id()
        ));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(dir.join(root)).expect("a planted root");
        std::fs::write(dir.join(root).join("c.toml"), SOUND).expect("a planted catalogue");
        assert_eq!(
            catalogues(&dir).len(),
            1,
            "`{root}` is a declared root and is not walked"
        );
    }
}

#[test]
fn several_defects_in_one_tree_are_all_reported() {
    let k = kinds(
        "several",
        &[
            ("broken.toml", "[[item]]\nid = \"unterminated\n"),
            ("dup.toml", "[[item]]\nid = \"a\"\n\n[[item]]\nid = \"a\"\n"),
            ("conf.toml", "[[item]]\nid = \"b\"\nconfidence = 9\n"),
        ],
    );
    assert_eq!(k.len(), 3, "{k:?}");
    assert!(k.contains(&Some("a-catalogue-does-not-parse")), "{k:?}");
    assert!(k.contains(&Some("two-rows-claim-one-identifier")), "{k:?}");
    assert!(k.contains(&Some("a-confidence-is-out-of-range")), "{k:?}");
}

#[test]
fn the_reader_finds_the_rows_a_document_holds() {
    // The walk itself, so a reader that returned nothing could not make every
    // arm above pass by finding no rows to complain about.
    let doc = SOUND
        .parse::<toml_edit::DocumentMut>()
        .expect("the fixture parses");
    assert_eq!(rows(&doc).len(), 2);
}

#[test]
fn its_findings_block_every_gate() {
    // The severity that decides a refusal is the one the finding carries, not
    // the one `default_severity` returns.
    let found = check(&tree(
        "severity",
        &[("broken.toml", "[[item]]\nid = \"x\n")],
    ));
    assert!(!found.is_empty(), "nothing was found, so this says nothing");
    for e in &found {
        assert_eq!(
            e.severity,
            mockspace::Severity::HARD_ERROR,
            "`{}` reported `{}` at a severity that does not block every gate",
            e.lint_name,
            e.message
        );
    }
}

#[test]
fn it_is_not_declared_off_so_it_runs_at_all() {
    assert!(
        !APanelCatalogueIsReadable.default_severity().is_off(),
        "it declares itself off, so it never runs and its predicate is dead code"
    );
}

#[test]
fn it_answers_to_the_name_the_gate_and_the_config_use() {
    assert_eq!(
        APanelCatalogueIsReadable.name(),
        "a-panel-catalogue-is-readable"
    );
}

#[test]
fn the_crate_directory_is_named_after_the_lint_it_declares() {
    // How the pack's own guard finds this file: a repo lint declared by a tool
    // crate is looked for at `mock/tools/<name>/src/lib.rs`. A crate renamed out
    // of that shape drops out of every check the guard performs, silently.
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    assert_eq!(
        here.file_name().and_then(|n| n.to_str()),
        Some(APanelCatalogueIsReadable.name())
    );
}
