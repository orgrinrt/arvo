//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! A ruling he ratified whose `ratification` names no record of his words.
//!
//! The schema says a `ratification` carries what was put to him, the options and
//! his answer verbatim, "which is what separates a blessing from somebody having
//! typed the word". A row at `rung = "ratified"` with `ratified_by` of `op` or
//! `both` claims his word, so its `ratification` names where that word is
//! written: one of his own round files in the panel tree (`panel::<dir>::<n>_op_<slug>`)
//! or the intent catalogue there (`INTENTS`), or a capture in the workspace's
//! store of his responses, named by its file (`<stamp>_<slug>.md`). A seat's
//! file or a research note under `mock::` is somebody's reading of him and is
//! not a record of his words, however faithfully it quotes them. A row raised by default
//! from the written history names the ruling that raises it and the passage as
//! well; the ruling alone is not a record of his words about that row.
//!
//! Two ways to break it, reported apart:
//!
//! - `missing`: no `ratification` at all on such a row.
//! - `no-record`: a `ratification` naming none of the three. Thirty-six rows
//!   once cited an agent's research note in place of his words, and the gate
//!   passed them, which is the case this exists for.
//!
//! `experts` is outside it: an expert promotion carries `promotion`, not his
//! words. Whether the cited file holds what the field quotes is not checked here;
//! `a-citation-names-a-target` resolves citations, and quoting is read by hand.
use mockspace::{Lint, LintError, RepoContext, RepoLint, Severity};

use crate::canon_citations::file_named;
use crate::canon_rows::{finding, text};
pub fn repo_lint() -> Box<dyn RepoLint> {
    Box::new(ARatificationNamesARecordOfHisWords)
}

const LINT: &str = "a-ratification-names-a-record-of-his-words";

/// The routes that claim his word.
const HIS: [&str; 2] = ["op", "both"];

/// The root of the panel tree, where his rounds are recorded.
const PANEL: &str = "panel::";

/// The panel files that record his words: his own round files, numbered and
/// named `<n>_op_<slug>`, and the intent catalogue, which carries each intent's
/// words verbatim. A seat's file or a consolidation is the panel's reading.
fn is_his_record(file: &str) -> bool {
    file == "INTENTS"
        || file.split_once('_').is_some_and(|(n, rest)| {
            n.chars().all(|c| c.is_ascii_digit()) && rest.starts_with("op_")
        })
}

struct ARatificationNamesARecordOfHisWords;
impl Lint for ARatificationNamesARecordOfHisWords {
    fn name(&self) -> &'static str {
        LINT
    }

    fn default_severity(&self) -> Severity {
        Severity::HARD_ERROR
    }
}

/// A capture's file name, bare: a stamp of eight to twelve digits, optionally
/// a second run of four (`20260830_2320_`), an underscore and a slug of
/// lowercase letters, digits and hyphens, ending `.md`. A path is not one,
/// since a capture is cited by its name in the store and a path with a stamp in
/// it is a research note of the tree.
fn is_a_capture(token: &str) -> bool {
    let Some(stem) = token.strip_suffix(".md") else {
        return false;
    };
    let digits = |s: &str| !s.is_empty() && s.chars().all(|c| c.is_ascii_digit());
    let Some((stamp, rest)) = stem.split_once('_') else {
        return false;
    };
    if !digits(stamp) || !(8 ..= 12).contains(&stamp.len()) {
        return false;
    }
    let slug = match rest.split_once('_') {
        Some((clock, slug)) if clock.len() == 4 && digits(clock) => slug,
        _ => rest,
    };
    !slug.is_empty()
        && slug
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
}

/// Whether the text names a record: some backticked span is a citation into a
/// record root or a capture's file name.
fn names_a_record(ratification: &str) -> bool {
    ratification
        .split('`')
        .skip(1)
        .step_by(2)
        .map(str::trim)
        .any(|span| {
            is_a_capture(span)
                || (span.starts_with(PANEL) && file_named(span).is_some_and(is_his_record))
        })
}

impl RepoLint for ARatificationNamesARecordOfHisWords {
    fn check_repo(&self, ctx: &RepoContext) -> Vec<LintError> {
        let reg = ctx.registry();
        reg.rows_in("ruling")
            .iter()
            .filter(|q| text(reg, q, "rung") == Some("ratified"))
            .filter(|q| text(reg, q, "ratified_by").is_some_and(|by| HIS.contains(&by)))
            .filter_map(|q| match text(reg, q, "ratification") {
                Some(r) if names_a_record(r) => None,
                Some(_) => Some(finding(
                    LINT,
                    Some("no-record"),
                    format!(
                        "`{q}` claims his word and its `ratification` names no record of it. \
                         Cite where his words are written, in backticks: his own round file in the \
                         panel tree or `INTENTS` (`panel::...`), or the capture \
                         in the store of his responses by its file name (`<stamp>_<slug>.md`), \
                         with the line."
                    ),
                )),
                None => Some(finding(
                    LINT,
                    Some("missing"),
                    format!(
                        "`{q}` claims his word and carries no `ratification`. Say what was put \
                         to him, the options and his answer, and name the record of it."
                    ),
                )),
            })
            .collect()
    }
}
#[cfg(test)]
mod tests {
    use mockspace::Lint;

    use crate::canon_lint_testkit::{
        assert_findings_block,
        assert_not_declared_off,
        assert_registered,
        findings,
        view,
    };

    fn found(rows: &[(&str, &[(&str, &str)])]) -> Vec<String> {
        findings(
            &super::ARatificationNamesARecordOfHisWords,
            &view(rows, &[]),
        )
    }

    fn his(ratification: &str) -> Vec<String> {
        found(&[("ruling::r", &[
            ("rung", "ratified"),
            ("ratified_by", "op"),
            ("ratification", ratification),
        ])])
    }

    #[test]
    fn a_ratification_citing_an_agents_note_by_prose_is_reported() {
        // The planted case: the shape thirty-six rows once had.
        let f = his("Recorded in the research note beside the reading; he agreed.");
        assert_eq!(f.len(), 1, "{f:?}");
        assert!(f[0].contains("no record"), "{}", f[0]);
    }

    #[test]
    fn the_default_ruling_alone_is_not_a_record() {
        let f = his("Ratified by default under `ruling::what_the_written_history_shows`.");
        assert_eq!(f.len(), 1, "{f:?}");
    }

    #[test]
    fn a_missing_ratification_is_reported() {
        for by in ["op", "both"] {
            let f = found(&[("ruling::r", &[("rung", "ratified"), ("ratified_by", by)])]);
            assert_eq!(f.len(), 1, "{by}: {f:?}");
            assert!(f[0].contains("carries no `ratification`"), "{}", f[0]);
        }
    }

    #[test]
    fn control_each_record_form_passes() {
        for r in [
            "Asked in `panel::p::211_op_round`.",
            "Recorded at `panel::p::217_op_the_last_round::#and-then-he-left`.",
            "Verbatim at `panel::p::INTENTS::#i13-the-work`.",
            "In the capture `202608311631_arvo-design-bounds-naming.md`, line 47.",
        ] {
            assert!(his(r).is_empty(), "{r:?} was not read as a record");
        }
    }

    #[test]
    fn an_agents_reading_of_him_is_not_a_record() {
        // A research note or a seat quotes him; it is not where he said it.
        for r in [
            "Recorded at `mock::research/x/NOTES::#his-words`.",
            "Argued at `panel::p::213_kiselyov_second_read::442`.",
            "Consolidated at `panel::p::90_giesen_consolidation::127`.",
        ] {
            assert_eq!(his(r).len(), 1, "{r:?} passed as a record of his words");
        }
    }

    #[test]
    fn a_record_root_outside_backticks_is_not_read() {
        // Prose mentioning the panel tree names no file.
        assert_eq!(his("Somewhere under panel:: in the tree.").len(), 1);
    }

    #[test]
    fn control_an_expert_promotion_owes_none_of_his_words() {
        let f = found(&[("ruling::r", &[
            ("rung", "ratified"),
            ("ratified_by", "experts"),
        ])]);
        assert!(f.is_empty(), "{f:?}");
    }

    #[test]
    fn control_a_row_below_ratified_is_left_alone() {
        let f = found(&[("ruling::r", &[("rung", "stated"), ("ratified_by", "op")])]);
        assert!(f.is_empty(), "{f:?}");
    }

    #[test]
    fn a_research_path_with_a_stamp_is_not_a_capture() {
        let f = his("Read in `202611132130_eight-rulings-third-read/NOTES.md`.");
        assert_eq!(f.len(), 1, "{f:?}");
    }

    #[test]
    fn a_bare_number_is_not_a_capture() {
        assert_eq!(his("In `1.md`.").len(), 1);
    }

    #[test]
    fn control_both_capture_stamp_shapes_pass() {
        assert!(his("In `202608311355_ruling-batch-arvo.md`, line 28.").is_empty());
        assert!(his("In `20260830_2320_arvo-panel-steering-opening.md`, line 26.").is_empty());
    }

    #[test]
    fn the_advice_names_no_root_the_lint_refuses() {
        let f = his("Nothing cited.");
        assert!(!f[0].contains("mock::"), "{}", f[0]);
    }

    #[test]
    fn its_findings_block_every_gate() {
        let v = view(
            &[("ruling::r", &[("rung", "ratified"), ("ratified_by", "op")])],
            &[],
        );
        assert_findings_block(&super::ARatificationNamesARecordOfHisWords, &v);
    }

    #[test]
    fn it_is_not_declared_off_so_it_runs_at_all() {
        assert_not_declared_off(&super::ARatificationNamesARecordOfHisWords);
    }

    #[test]
    fn it_answers_to_the_name_the_gate_and_the_config_use() {
        assert_eq!(
            super::ARatificationNamesARecordOfHisWords.name(),
            "a-ratification-names-a-record-of-his-words"
        );
    }

    #[test]
    fn it_reaches_the_pack_the_engine_is_handed() {
        assert_registered("a-ratification-names-a-record-of-his-words");
    }
}
