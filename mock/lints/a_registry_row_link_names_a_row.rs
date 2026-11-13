//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! A `namespace::slug` written in a row's prose that names no row.
//!
//! The engine refuses a typed field naming a row that is not there, and the
//! panel-prose lint reads a slug in the research documents. Neither reads a row
//! link written in backticks inside a registry string field: a `bound`, a
//! `note`, an `answered`, a `says`. A `bound` once linked a retirement by a slug
//! that no retirement row carries: it was the id of a proposal, the retirement
//! meant was another, and every gate stayed green because the slug half of the
//! link resolves in some namespace and the namespace half was never asked.
//!
//! **Both halves are read.** A backticked span that is exactly `a::b`, with `a`
//! a namespace the registry declares, has to name a row of that namespace. Where
//! it names none and the slug is the id of a row in another namespace, the
//! finding says which, because that is the wrong-namespace shape and the repair
//! is to write the namespace the row is in.
//!
//! **A registry file stem is not a namespace.** `proposal-the-later-topics` is
//! a file that holds `proposal` rows, so `proposal-the-later-topics::<slug>`
//! resolves nowhere, and the engine's own `cargo mock query` refuses it while
//! the other lints passed. A span whose first segment is a declared namespace
//! followed by `-` and more words is refused as that.
//!
//! **What is not read.** A span of three or more segments is a citation into
//! the research tree and is another lint's. A first segment that is no declared
//! namespace (`panel`, `std`, a crate) names no registry row and says nothing
//! here. The `quote`, `provenance` and `lives` fields are skipped: the first is
//! op's own words, kept as he wrote them, and the other two hold citations.
//!
//! The frozen panel catalogue under `research/` carries two file-stem links of
//! its own. It is written once and is not the live canon, so it is outside
//! this lint.

use mockspace::{Lint, LintError, RegistryView, RepoContext, RepoLint, Severity};

use crate::canon_rows::finding;

const LINT: &str = "a-registry-row-link-names-a-row";

/// The fields whose text is not a row link: op's words, and citations.
const SKIPPED_FIELDS: [&str; 3] = ["quote", "provenance", "lives"];

pub fn repo_lint() -> Box<dyn RepoLint> {
    Box::new(ARegistryRowLinkNamesARow)
}

struct ARegistryRowLinkNamesARow;

impl Lint for ARegistryRowLinkNamesARow {
    fn name(&self) -> &'static str {
        LINT
    }

    fn default_severity(&self) -> Severity {
        Severity::HARD_ERROR
    }
}

impl RepoLint for ARegistryRowLinkNamesARow {
    fn check_repo(&self, ctx: &RepoContext) -> Vec<LintError> {
        check(ctx.registry())
    }
}

/// Every refused link in every row's string fields.
fn check(reg: &RegistryView) -> Vec<LintError> {
    let namespaces: Vec<&str> = reg.namespaces().collect();
    let mut out = Vec::new();
    for ns in &namespaces {
        for q in reg.rows_in(ns) {
            let Some(row) = reg.row(q) else { continue };
            for (field, value) in row {
                if SKIPPED_FIELDS.contains(&field.as_str()) {
                    continue;
                }
                for (first, slug) in links_in(value) {
                    if let Some(why) = refusal(reg, &namespaces, first, slug) {
                        out.push(finding(
                            LINT,
                            None,
                            format!("`{q}` carries `{first}::{slug}` in `{field}`: {why}"),
                        ));
                    }
                }
            }
        }
    }
    out
}

/// Why one `first::slug` link is refused, or `None` where it resolves or is not
/// a registry link at all.
fn refusal(reg: &RegistryView, namespaces: &[&str], first: &str, slug: &str) -> Option<String> {
    if namespaces.contains(&first) {
        if reg.row(&format!("{first}::{slug}")).is_some() {
            return None;
        }
        let elsewhere: Vec<&str> = namespaces
            .iter()
            .copied()
            .filter(|ns| reg.row(&format!("{ns}::{slug}")).is_some())
            .collect();
        return Some(match elsewhere.as_slice() {
            [] => format!("no `{first}` row has that id"),
            found => {
                format!(
                    "no `{first}` row has that id; it is the id of a row in `{}`, so the link \
                 names the wrong namespace",
                    found.join("`, `")
                )
            },
        });
    }
    let stem_of = namespaces.iter().find(|ns| {
        first
            .strip_prefix(**ns)
            .is_some_and(|rest| rest.starts_with('-') && rest.len() > 1)
    })?;
    Some(format!(
        "`{first}` is a registry file stem, not a namespace; the row is `{stem_of}::{slug}`"
    ))
}

/// The `first::slug` spans of backticks in `text`: each span exactly two
/// segments, the first a lowercase name that may carry `-`, the second a slug.
fn links_in(text: &str) -> Vec<(&str, &str)> {
    let mut parts: Vec<&str> = text.split('`').collect();
    // An even count of parts means an odd count of backticks, so the last part
    // opened a span nothing closed and is not one.
    if parts.len() % 2 == 0 {
        parts.pop();
    }
    parts
        .into_iter()
        .skip(1)
        .step_by(2)
        .filter_map(|span| {
            let (first, slug) = span.split_once("::")?;
            let name = |s: &str, dash: bool| {
                s.chars().next().is_some_and(|c| c.is_ascii_lowercase())
                    && s.chars().all(|c| {
                        c.is_ascii_lowercase()
                            || c.is_ascii_digit()
                            || c == '_'
                            || (dash && c == '-')
                    })
            };
            let slug_ok = !slug.is_empty()
                && slug
                    .chars()
                    .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_');
            (name(first, true) && slug_ok).then_some((first, slug))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use mockspace::{Lint, RepoLint};

    use super::*;
    use crate::canon_lint_testkit::{
        Judge,
        assert_findings_block,
        assert_not_declared_off,
        assert_registered,
        ctx,
        view,
    };

    type Rows<'a> = &'a [(&'a str, &'a [(&'a str, &'a str)])];

    fn found(rows: Rows) -> Vec<String> {
        ARegistryRowLinkNamesARow
            .judge(ctx(&view(rows, &[])))
            .into_iter()
            .map(|e| e.message)
            .collect()
    }

    /// The two namespaces every arm plants rows in, with one real row each.
    const BASE: [(&str, &[(&str, &str)]); 2] = [
        (
            "retirement::a_nonzero_phase_removes_the_additive_identity",
            &[("says", "x")],
        ),
        (
            "proposal::a_nonzero_phase_leaves_the_set_without_an_identity",
            &[("says", "x")],
        ),
    ];

    fn with(field: &'static str, value: &'static str) -> Vec<String> {
        let mut rows: Vec<(&str, &[(&str, &str)])> = BASE.to_vec();
        let fields: &'static [(&'static str, &'static str)] = Box::leak(Box::new([(field, value)]));
        rows.push(("question::the_host", fields));
        found(&rows)
    }

    // --- the wrong namespace, in each of the three fields the ticket names ---

    const WRONG: &str = "see `retirement::a_nonzero_phase_leaves_the_set_without_an_identity` here";

    #[test]
    fn a_wrong_namespace_in_a_bound_is_refused() {
        let f = with("bound", WRONG);
        assert_eq!(f.len(), 1, "{f:?}");
        assert!(f[0].contains("the wrong namespace"), "{}", f[0]);
        assert!(
            f[0].contains("`proposal`"),
            "names where the row is: {}",
            f[0]
        );
    }

    #[test]
    fn a_wrong_namespace_in_a_note_is_refused() {
        assert_eq!(with("note", WRONG).len(), 1);
    }

    #[test]
    fn a_wrong_namespace_in_an_answered_is_refused() {
        assert_eq!(with("answered", WRONG).len(), 1);
    }

    #[test]
    fn a_slug_that_names_no_row_in_any_namespace_is_refused() {
        let f = with("note", "`retirement::a_row_nobody_wrote`");
        assert_eq!(f.len(), 1, "{f:?}");
        assert!(f[0].contains("no `retirement` row has that id"), "{}", f[0]);
        assert!(!f[0].contains("wrong namespace"), "{}", f[0]);
    }

    // --- a registry file stem used as a namespace ---

    #[test]
    fn a_file_stem_used_as_the_namespace_is_refused() {
        let f = with(
            "note",
            "`proposal-the-later-topics::a_nonzero_phase_leaves_the_set_without_an_identity`",
        );
        assert_eq!(f.len(), 1, "{f:?}");
        assert!(f[0].contains("registry file stem"), "{}", f[0]);
        assert!(f[0].contains("`proposal::a_nonzero_phase"), "{}", f[0]);
    }

    // --- what must stay silent ---

    /// The control for every arm above: the same links, written right.
    #[test]
    fn control_the_right_namespace_resolves() {
        let ok = "see `retirement::a_nonzero_phase_removes_the_additive_identity` and \
                  `proposal::a_nonzero_phase_leaves_the_set_without_an_identity`";
        assert!(with("bound", ok).is_empty());
        assert!(with("note", ok).is_empty());
    }

    #[test]
    fn a_span_that_is_not_a_registry_link_is_silent() {
        assert!(
            with(
                "note",
                "`std::fmt` and `panel::something_else` and `crate_x::y`"
            )
            .is_empty()
        );
    }

    #[test]
    fn a_three_segment_citation_is_another_lints() {
        assert!(with("note", "`retirement::nothing_here::#anchor`").is_empty());
    }

    #[test]
    fn a_link_outside_backticks_is_prose_and_silent() {
        assert!(with("note", "retirement::a_row_nobody_wrote is a bare word").is_empty());
    }

    #[test]
    fn a_quote_is_op_s_own_words_and_is_not_read() {
        assert!(with("quote", "`retirement::a_row_nobody_wrote`").is_empty());
    }

    #[test]
    fn a_provenance_citation_is_not_read() {
        assert!(with("provenance", "`panel::a_dir::a_file`").is_empty());
    }

    #[test]
    fn a_first_segment_that_merely_starts_with_a_namespace_word_is_silent() {
        // `proposals` is no file stem of `proposal`: the rest after the
        // namespace has to open on `-`.
        assert!(with("note", "`proposals::a_row` and `lawn::mower`").is_empty());
    }

    #[test]
    fn an_empty_registry_is_silent_rather_than_a_panic() {
        assert!(found(&[]).is_empty());
    }

    #[test]
    fn the_span_scanner_takes_only_whole_backticked_pairs() {
        assert_eq!(links_in("a `x::y` b `p-q::r_1` c"), vec![
            ("x", "y"),
            ("p-q", "r_1")
        ]);
        assert!(links_in("`X::y` `x::Y` `x::` `::y` `x::y::z`").is_empty());
        assert!(links_in("unbalanced `x::y").is_empty());
    }

    // --- registration and severity ---

    #[test]
    fn it_answers_to_the_name_the_gate_and_the_config_use() {
        assert_eq!(ARegistryRowLinkNamesARow.name(), LINT);
    }

    #[test]
    fn it_is_not_declared_off_so_it_runs_at_all() {
        assert_not_declared_off(&ARegistryRowLinkNamesARow);
    }

    #[test]
    fn it_reaches_the_pack_the_engine_is_handed() {
        assert_registered(LINT);
    }

    #[test]
    fn its_findings_block_every_gate() {
        let mut rows: Vec<(&str, &[(&str, &str)])> = BASE.to_vec();
        let fields: &'static [(&'static str, &'static str)] =
            Box::leak(Box::new([("note", "`retirement::nobody`")]));
        rows.push(("question::the_host", fields));
        assert_findings_block(&ARegistryRowLinkNamesARow, &view(&rows, &[]));
    }
}
