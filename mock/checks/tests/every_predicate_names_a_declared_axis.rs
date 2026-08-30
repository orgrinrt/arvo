//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! The committed canon's predicates, and the cases that must fail.
//!
//! The planted arms are not decoration. An arm that has only ever run over a
//! clean canon returns an empty list and establishes nothing, and this file's
//! whole first version would have passed against a registry where the check was
//! commented out.

use arvo_checks::{canon, parse, predicate};

#[test]
fn the_committed_canon_names_only_declared_axes() {
    let found = predicate::undeclared_dimensions(&canon());
    assert!(
        found.is_empty(),
        "a predicate names an axis nothing declares, which converts absence from the \
         notation's strongest negative statement into a shrug: {found:#?}"
    );
}

#[test]
fn the_committed_canon_names_no_axis_twice_in_one_predicate() {
    let found = predicate::repeated_dimensions(&canon());
    assert!(
        found.is_empty(),
        "one axis carries two regions in one predicate and nothing says which holds: {found:#?}"
    );
}

/// The control for the arm above, and the reason it is not vacuous.
#[test]
fn an_axis_nothing_declares_is_reported() {
    let reg = parse(
        "planted.toml",
        r#"
[[dimension]]
id = "fraction_width"
what = "bits below the point"

[[proposal]]
id = "a_claim"
predicate = ["fraction_width: 0", "phase_of_the_moon: waxing"]
"#,
    );
    let found = predicate::undeclared_dimensions(&reg);
    assert_eq!(found.len(), 1, "{found:#?}");
    assert_eq!(found[0].kind, "predicate-names-an-undeclared-dimension");
    assert!(found[0].says.contains("phase_of_the_moon"), "{}", found[0].says);
}

#[test]
fn an_entry_with_no_axis_and_values_split_is_reported_separately() {
    let reg = parse(
        "planted.toml",
        r#"
[[dimension]]
id = "fraction_width"
what = "bits below the point"

[[proposal]]
id = "a_claim"
predicate = ["mostly small fractions"]
"#,
    );
    let found = predicate::undeclared_dimensions(&reg);
    assert_eq!(found.len(), 1, "{found:#?}");
    assert_eq!(
        found[0].kind, "predicate-entry-is-malformed",
        "a sentence with no colon is a malformed entry, not an unknown axis, and the two \
         want different fixes: {found:#?}"
    );
}

#[test]
fn an_axis_listed_with_nothing_is_reported() {
    let reg = parse(
        "planted.toml",
        r#"
[[dimension]]
id = "threads"
what = "how many threads"

[[proposal]]
id = "a_claim"
predicate = ["threads:"]
"#,
    );
    let found = predicate::undeclared_dimensions(&reg);
    assert_eq!(found.len(), 1, "{found:#?}");
    assert_eq!(found[0].kind, "predicate-entry-has-no-values");
}

#[test]
fn one_axis_named_twice_is_reported() {
    let reg = parse(
        "planted.toml",
        r#"
[[dimension]]
id = "signedness"
what = "whether it carries negatives"

[[law]]
id = "distributivity"
holds = ["signedness: unsigned", "signedness: signed"]
"#,
    );
    let found = predicate::repeated_dimensions(&reg);
    assert_eq!(found.len(), 1, "{found:#?}");
    assert_eq!(found[0].kind, "predicate-names-one-axis-twice");
}

/// A law's two region fields are both read, which is easy to get wrong by
/// wiring the walk to `predicate` alone and never noticing.
#[test]
fn a_laws_failing_region_is_read_as_well_as_its_holding_one() {
    let reg = parse(
        "planted.toml",
        r#"
[[law]]
id = "associativity"
holds = ["fraction_width: 0"]
fails = ["nothing_declares_this: everywhere"]
"#,
    );
    let found = predicate::undeclared_dimensions(&reg);
    assert_eq!(
        found.len(),
        2,
        "both `holds` and `fails` name an undeclared axis here, and a walk that reads only \
         one reports one: {found:#?}"
    );
}

/// The arm reads `predicate` and not every `string[]` beside it.
///
/// `keywords` is a list of bare words with no colon in them, so a walker over
/// every array field would report each one as malformed and the report would be
/// noise nobody reads.
#[test]
fn a_keywords_list_is_not_read_as_a_predicate() {
    let reg = parse(
        "planted.toml",
        r#"
[[proposal]]
id = "a_claim"
keywords = ["width", "signedness", "not a predicate entry"]
"#,
    );
    assert!(
        predicate::undeclared_dimensions(&reg).is_empty(),
        "keywords are not predicate entries"
    );
}
