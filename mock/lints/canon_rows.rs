//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! Reading a canon row out of the flattened view a lint is handed.
//!
//! **This file declares no lint.** The engine scans each `mock/lints/*.rs` for
//! `lint()`, `cross_lint()`, `repo_lint()` and `message_lint()` and registers
//! only what it finds, including the file as a module either way, so a module
//! defining none of them compiles into the pack and reaches nothing. That is
//! what lets every canon lint here share one set of readers.
//!
//! # The lossy field, said plainly
//!
//! A lint is handed a [`RegistryView`], whose rows are `BTreeMap<String,
//! String>`. **A `string[]` field has already been joined with `", "` by the
//! time it arrives**, in the engine's `value_to_string`, so a lint cannot see
//! where one entry ended and the next began. For a field whose entries are
//! slugs or citations that costs nothing, because none of those carries a
//! comma. For a predicate entry it costs the boundary outright: an entry reads
//! `<axis>: <values>` and the values side is prose, so
//! `signedness: signedness in {unsigned, signed}` carries the separator inside
//! itself and [`list`] would cut it in two.
//!
//! [`predicate_entries`] is the reader for those, and it splits on `", "` only
//! where an axis name and a colon follow, which is the one place an entry can
//! begin. The residue is stated at that function.
//!
//! There is no route around this from inside a lint. The generated pack's
//! manifest carries `mockspace` plus whatever `[lint-crates]` and `mock/tools/`
//! declare, and nothing a lint file writes can add a dependency to it, so
//! parsing the registry files again with a TOML reader is not available here,
//! and it would duplicate a parse the engine has already done besides.

use mockspace::{LintError, RegistryView};

/// One finding against the registry, at the severity that blocks every gate.
///
/// **The severity that decides a refusal is the one the finding carries**,
/// which is why every canon lint goes through this rather than choosing a
/// constructor per call site. A `LintError::warning` here turns a refusal off
/// with the declared default untouched, and the arm asserting the declared
/// default stays green through it.
///
/// The location is the registry rather than a file and a line. A row is
/// addressed by `namespace::slug` everywhere else in this project, and a line
/// number into a registry file names a different row the moment anything is
/// inserted above it, which is the failure one of these lints exists to refuse.
/// So the address goes in the message, where it stays true.
///
/// `kind` is the sub-category the engine keys a per-finding severity override
/// on. It carries the name of the particular refusal where one lint states a
/// contract with more than one way of breaking it, and is `None` where the lint
/// has only the one.
pub fn finding(lint: &'static str, kind: Option<&'static str>, message: String) -> LintError {
    let mut e = LintError::error("registry".to_string(), 0, lint, message);
    e.finding_kind = kind;
    e
}

/// The separator the engine joins a `string[]` field with.
///
/// Spelled once here rather than at each call site, because it is the engine's
/// choice rather than this repository's, and a reader asking why a split is
/// written this way should land in one place.
pub const JOIN: &str = ", ";

/// The entries of a list field, or nothing where the field is absent.
///
/// An absent field and an empty array are deliberately not distinguished: both
/// mean the row names nothing, which is what every arm here asks. A scalar read
/// through this comes back as one entry, or as several where its value happens
/// to carry the separator, so it is for fields the schema declares as a list.
pub fn list<'a>(reg: &'a RegistryView, q: &str, field: &str) -> Vec<&'a str> {
    match reg.field(q, field) {
        None => Vec::new(),
        Some(v) if v.trim().is_empty() => Vec::new(),
        Some(v) => v
            .split(JOIN)
            .map(str::trim)
            .filter(|e| !e.is_empty())
            .collect(),
    }
}

/// A scalar field's value, trimmed, or `None` where it is absent or blank.
///
/// Blank counts as absent. A field present and holding nothing tells a reader
/// exactly what a missing one does, and every arm that asked
/// `is_none_or(str::is_empty)` in the crate this came from wanted both.
pub fn text<'a>(reg: &'a RegistryView, q: &str, field: &str) -> Option<&'a str> {
    reg.field(q, field).map(str::trim).filter(|v| !v.is_empty())
}

/// Whether the row carries the field at all, whatever it holds.
///
/// Distinct from [`text`]: an arm asking whether an edge was written wants
/// this, and an arm asking what the edge says wants that. Folding them makes a
/// row carrying an empty edge invisible to the controls that exist to see one.
pub fn has(reg: &RegistryView, q: &str, field: &str) -> bool {
    reg.row(q).is_some_and(|r| r.contains_key(field))
}

/// The slug half of a `namespace::slug`, or the whole of it where there is no
/// namespace.
pub fn slug(qualified: &str) -> &str {
    qualified.rsplit("::").next().unwrap_or(qualified)
}

/// The fields that hold a predicate, and which namespace each belongs to.
///
/// Named rather than discovered, because a field's type is `string[]` and a
/// walker over every `string[]` would read `keywords` and `options` too, and
/// report every bare word in them as a malformed entry.
///
/// Here rather than in either lint that reads it, because both do and a written
/// list spelled twice is a list that will disagree with itself. It sits beside
/// [`predicate_entries`], the reader written for exactly these fields.
pub const PREDICATE_FIELDS: [(&str, &str); 3] = [
    ("proposal", "predicate"),
    ("law", "holds"),
    ("law", "fails"),
];

/// The predicate entries of one field, recovered from the joined string.
///
/// **An entry begins where an axis name and a colon follow the separator**, and
/// nowhere else. An axis name is a registry slug, so it is lowercase letters,
/// digits and underscores, which is what makes the boundary findable at all:
/// the values side is prose and carries `", "` freely, and it does not carry a
/// slug followed immediately by a colon.
///
/// The residue, since it is real: a values side ending a clause with
/// `, word: ` splits there and the tail is then reported as an entry naming an
/// undeclared axis. Nothing in the committed canon does it, and the failure is
/// loud rather than quiet, which is the direction to fail in. The alternative
/// is a lint that cannot read a predicate at all.
pub fn predicate_entries<'a>(reg: &'a RegistryView, q: &str, field: &str) -> Vec<&'a str> {
    let Some(joined) = reg.field(q, field) else {
        return Vec::new();
    };
    split_entries(joined)
}

/// [`predicate_entries`] over a string, which is what its tests drive.
pub fn split_entries(joined: &str) -> Vec<&str> {
    let mut out = Vec::new();
    let mut start = 0;
    let mut at = 0;
    while let Some(found) = joined[at..].find(JOIN) {
        let cut = at + found;
        let after = &joined[cut + JOIN.len()..];
        if starts_an_entry(after) {
            let entry = joined[start..cut].trim();
            if !entry.is_empty() {
                out.push(entry);
            }
            start = cut + JOIN.len();
        }
        at = cut + JOIN.len();
    }
    let last = joined[start..].trim();
    if !last.is_empty() {
        out.push(last);
    }
    out
}

/// Whether the text opens with `<slug>:`, which is where an entry begins.
fn starts_an_entry(s: &str) -> bool {
    let Some((name, _)) = s.split_once(':') else {
        return false;
    };
    !name.is_empty()
        && name.starts_with(|c: char| c.is_ascii_lowercase())
        && name
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_')
}

/// The axis a predicate entry names and the values it gives it.
///
/// `None` where the entry has no colon at all, which is the malformed case
/// rather than an unknown axis. The two are reported separately because the
/// fixes differ: one is a typo and the other is an axis to declare.
pub fn split_axis(entry: &str) -> Option<(&str, &str)> {
    let (axis, values) = entry.split_once(':')?;
    Some((axis.trim(), values.trim()))
}

/// Lowercase words with punctuation dropped, so a quotation matches the
/// sentence it quotes across a difference in emphasis or a trailing comma.
pub fn normalise(s: &str) -> Vec<String> {
    s.split_whitespace()
        .map(|w| {
            w.chars()
                .filter(|c| c.is_alphanumeric())
                .flat_map(char::to_lowercase)
                .collect::<String>()
        })
        .filter(|w| !w.is_empty())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::canon_lint_testkit::view;

    #[test]
    fn a_list_field_reads_back_as_the_entries_it_was_written_with() {
        let v = view(&[("proposal::p", &[("evidence", "one, two, three")])], &[]);
        assert_eq!(list(&v, "proposal::p", "evidence"), ["one", "two", "three"]);
    }

    #[test]
    fn an_absent_field_and_an_empty_array_both_read_as_nothing() {
        // The engine writes an empty array as the empty string, so the two
        // arrive here indistinguishable, and every arm wants both.
        let v = view(
            &[
                ("proposal::absent", &[("says", "x")]),
                ("proposal::empty", &[("evidence", "")]),
                ("proposal::blank", &[("evidence", "   ")]),
            ],
            &[],
        );
        assert!(list(&v, "proposal::absent", "evidence").is_empty());
        assert!(list(&v, "proposal::empty", "evidence").is_empty());
        assert!(list(&v, "proposal::blank", "evidence").is_empty());
        assert!(list(&v, "proposal::nosuch", "evidence").is_empty());
    }

    #[test]
    fn has_sees_a_field_that_holds_nothing_and_text_does_not() {
        // The distinction two arms turn on: whether an edge was written at all,
        // against what it says. Folding them would make a row carrying an empty
        // `obligation` invisible to the untiered-namespace control.
        let v = view(&[("probe::p", &[("obligation", "")])], &[]);
        assert!(has(&v, "probe::p", "obligation"));
        assert_eq!(text(&v, "probe::p", "obligation"), None);
        assert!(!has(&v, "probe::p", "nosuch"));
        assert!(!has(&v, "probe::nosuch", "obligation"));
    }

    #[test]
    fn text_trims_and_reads_a_blank_field_as_absent() {
        let v = view(
            &[(
                "ruling::r",
                &[("quote", " he said it "), ("instead", "\t \n")],
            )],
            &[],
        );
        assert_eq!(text(&v, "ruling::r", "quote"), Some("he said it"));
        assert_eq!(text(&v, "ruling::r", "instead"), None);
    }

    #[test]
    fn a_predicate_splits_where_an_axis_begins_and_nowhere_else() {
        // The case a plain separator split gets wrong, taken from a row in the
        // committed canon. Its third and seventh entries carry the separator
        // inside themselves, so the naive split reports nine entries where the
        // row wrote seven, two of them naming axes nobody declared.
        let written = [
            "total_width: W in 3..=7",
            "fraction_width: F = 0",
            "signedness: signedness in {unsigned, signed}",
            "overflow_policy: overflow policy in {saturate, clamp}",
            "operation: operations {add, mul}",
            "arity: arity = 3",
            "container: interval numerals containing zero, 100 of them",
        ];
        let joined = written.join(JOIN);
        assert_eq!(split_entries(&joined), written);
        // The control. Without it this asserts that a fixture round-trips
        // through a reader that could be the identity and prove nothing.
        assert_ne!(
            joined.split(JOIN).count(),
            written.len(),
            "the naive split agreed, so this fixture establishes nothing"
        );
    }

    #[test]
    fn a_single_entry_carrying_the_separator_stays_one_entry() {
        assert_eq!(
            split_entries("container: interval numerals containing zero, 100 of them"),
            ["container: interval numerals containing zero, 100 of them"]
        );
    }

    #[test]
    fn an_entry_with_no_colon_survives_so_the_malformed_arm_can_report_it() {
        // A malformed entry is a finding rather than something to swallow, so
        // the reader hands it on whole. Two malformed entries in a row have no
        // boundary to find and come back as one, which is stated here rather
        // than hidden: there is nothing in the text that says where the first
        // ended.
        assert_eq!(split_entries("no colon here"), ["no colon here"]);
        assert_eq!(
            split_entries("width: 8, no colon here"),
            ["width: 8, no colon here"]
        );
    }

    #[test]
    fn an_empty_predicate_is_no_entries_rather_than_one_blank() {
        assert!(split_entries("").is_empty());
        assert!(split_entries("   ").is_empty());
        let v = view(&[("proposal::p", &[("says", "x")])], &[]);
        assert!(predicate_entries(&v, "proposal::p", "predicate").is_empty());
    }

    #[test]
    fn an_axis_name_is_a_slug_so_a_capitalised_or_spaced_word_does_not_open_an_entry() {
        // The discrimination the split rests on. A registry slug is lowercase,
        // digits and underscores, so a values side running into a capital or a
        // space before its colon is prose rather than the next entry.
        assert!(starts_an_entry("width: 8"));
        assert!(starts_an_entry("fraction_width: F = 0"));
        assert!(starts_an_entry("f2: x"));
        assert!(!starts_an_entry("Width: 8"));
        assert!(!starts_an_entry("two words: x"));
        assert!(!starts_an_entry("no colon at all"));
        assert!(!starts_an_entry(": leading colon"));
        assert!(!starts_an_entry("100 of them"));
    }

    #[test]
    fn an_axis_splits_off_its_values_and_a_colonless_entry_does_not() {
        assert_eq!(
            split_axis("width: W in 3..=7"),
            Some(("width", "W in 3..=7"))
        );
        assert_eq!(split_axis("width:"), Some(("width", "")));
        assert_eq!(split_axis("no colon"), None);
    }

    #[test]
    fn the_slug_half_is_taken_and_an_unqualified_name_survives_whole() {
        assert_eq!(slug("proposal::a_thing"), "a_thing");
        assert_eq!(slug("a_thing"), "a_thing");
    }

    #[test]
    fn normalise_drops_punctuation_and_case_and_keeps_word_order() {
        assert_eq!(
            normalise("No repair, at a Homogeneous container."),
            ["no", "repair", "at", "a", "homogeneous", "container"]
        );
        assert!(normalise("  ,,, ").is_empty());
    }
}
