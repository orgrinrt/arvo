//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! The exception the frozen root cannot express.
//!
//! `panel` is frozen so a line citation into a numbered member file is honest.
//! The same root holds ledgers that are still being written, and freezing is
//! per root, so the declaration permits a line citation into those as well.
//! This is the refusal the declaration cannot make.

use arvo_checks::{canon, citation, parse};

#[test]
fn the_committed_canon_cites_no_moving_line() {
    let found = citation::line_citations_into_living_ledgers(&canon());
    assert!(
        found.is_empty(),
        "a line citation into a file that is still being written resolves forever and \
         points at different text after every edit above it: {found:#?}"
    );
}

#[test]
fn the_committed_canon_has_no_citation_without_a_target() {
    let found = citation::citations_with_no_target(&canon());
    assert!(found.is_empty(), "{found:#?}");
}

#[test]
fn a_line_into_a_ledger_is_reported() {
    let reg = parse(
        "planted.toml",
        r#"
[[ruling]]
id = "a_call"
provenance = ["panel::202608072330_the-numeral-canon-panel::OPTIONS::2656"]
"#,
    );
    let found = citation::line_citations_into_living_ledgers(&reg);
    assert_eq!(found.len(), 1, "{found:#?}");
    assert_eq!(found[0].kind, "line-citation-into-a-living-ledger");
    assert!(found[0].says.contains("OPTIONS"), "{}", found[0].says);
}

/// The first half of the control: a heading into the same file is the fix, so
/// it must pass. An arm that refused both would be refusing every citation into
/// the ledgers, which is not the rule and would push authors to cite nothing.
#[test]
fn a_heading_into_the_same_ledger_is_fine() {
    let reg = parse(
        "planted.toml",
        r#"
[[ruling]]
id = "a_call"
provenance = ["panel::202608072330_the-numeral-canon-panel::OPTIONS::#q41-whether-an-arms-predicate-may-read-data"]
"#,
    );
    assert!(
        citation::line_citations_into_living_ledgers(&reg).is_empty(),
        "a heading anchor fails loudly when renamed and is exactly what to write instead"
    );
}

/// The second half: a line into a numbered member file is honest and must pass,
/// because those are written once. An arm refusing every line number would be
/// refusing the corpus's own citation style, and the corpus is right about it.
#[test]
fn a_line_into_a_numbered_member_file_is_fine() {
    let reg = parse(
        "planted.toml",
        r#"
[[ruling]]
id = "a_call"
provenance = ["panel::202608072330_the-numeral-canon-panel::109_bellard_the_primitive_derived_cold::156"]
"#,
    );
    assert!(citation::line_citations_into_living_ledgers(&reg).is_empty());
}

/// A ledger named with its extension still resolves to the ledger.
///
/// Citations may omit the extension and the engine finds the file either way,
/// so an arm matching on the bare stem alone would miss half of them.
#[test]
fn a_ledger_named_with_its_extension_is_still_caught() {
    let reg = parse(
        "planted.toml",
        r#"
[[ruling]]
id = "a_call"
provenance = ["panel::202608072330_the-numeral-canon-panel::AGREEMENTS.md::468"]
"#,
    );
    let found = citation::line_citations_into_living_ledgers(&reg);
    assert_eq!(found.len(), 1, "{found:#?}");
    assert!(found[0].says.contains("AGREEMENTS"), "{}", found[0].says);
}

/// A probe's `lives` field is a citation too, and reading only `provenance`
/// would leave the one namespace whose whole point is where the evidence sits.
#[test]
fn a_probes_location_is_read_as_a_citation() {
    let reg = parse(
        "planted.toml",
        r#"
[[probe]]
id = "an_instrument"
lives = ["panel::202608072330_the-numeral-canon-panel::RULES::12"]
"#,
    );
    let found = citation::line_citations_into_living_ledgers(&reg);
    assert_eq!(found.len(), 1, "{found:#?}");
}

#[test]
fn a_citation_that_is_only_a_root_is_reported() {
    let reg = parse(
        "planted.toml",
        r#"
[[ruling]]
id = "a_call"
provenance = ["panel"]
"#,
    );
    let found = citation::citations_with_no_target(&reg);
    assert_eq!(found.len(), 1, "{found:#?}");
    assert_eq!(found[0].kind, "citation-names-no-target");
}
