//! Lint: a `FIXME` marker names the ticket opened with it.
//!
//! The workspace rule `mark-placeholders-fixme` says every marker is
//! `FIXME(<ticket id>): <reason>`, the ticket filed in the same act, and that a
//! lint refuses one without. This is that lint for arvo's Rust sources. A
//! bare `FIXME:` is a gap nobody owns, and `FIXME(op)` or `FIXME(canon
//! silent)` names a person or a state where a ticket belongs.
//!
//! A marker is a comment whose own text opens with `FIXME`. A comment that
//! only mentions the word further in, a doc line saying "marked `// FIXME:`
//! at the declaration", is prose about a marker and is not read as one. The
//! comments come from the one parse the engine holds
//! (`code_only_source::comment_lines`), so a `FIXME` inside a string literal
//! is never a marker either.
//!
//! **What is checked is the shape of the id, not its existence.** A ticket id
//! is a snake_case key of at least twelve characters with an underscore in it,
//! and the tracker that says whether it resolves is outside this repository, so
//! a lint here cannot read it. A key that is well formed and names nothing
//! passes this lint; asking the tracker is a different check.
//!
//! Severity is `HARD_ERROR`, because the rule says a lint errors hard.

use mockspace::{CrateLint, Lint, LintContext, LintError, Severity};

const NAME: &str = "a-fixme-names-its-ticket";

/// The shortest a ticket key is, so `op` and `todo` are not read as one.
const SHORTEST_ID: usize = 12;

pub fn lint() -> Box<dyn CrateLint> {
    Box::new(AFixmeNamesItsTicket)
}

struct AFixmeNamesItsTicket;

impl Lint for AFixmeNamesItsTicket {
    fn name(&self) -> &'static str {
        NAME
    }

    fn default_severity(&self) -> Severity {
        Severity::HARD_ERROR
    }
}

impl CrateLint for AFixmeNamesItsTicket {
    fn check(&self, ctx: &LintContext) -> Vec<LintError> {
        crate::code_only_source::comment_lines(ctx)
            .into_iter()
            .filter_map(|(line, text)| {
                let marker = marker_text(text)?;
                if names_a_ticket(marker) {
                    return None;
                }
                Some(LintError::with_severity(
                    ctx.crate_name.to_string(),
                    line,
                    NAME,
                    format!(
                        "`{}` is a placeholder marker with no ticket id; write \
                         `FIXME(<ticket id>): <reason>` and file the ticket in the same act",
                        text.trim()
                    ),
                    Severity::HARD_ERROR,
                ))
            })
            .collect()
    }
}

/// The text after the comment's own leader, when it opens with `FIXME`.
fn marker_text(line: &str) -> Option<&str> {
    let mut rest = line.trim_start();
    for leader in ["//!", "///", "//", "/*!", "/**", "/*", "*"] {
        if let Some(after) = rest.strip_prefix(leader) {
            rest = after;
            break;
        }
    }
    rest.trim_start().strip_prefix("FIXME")
}

/// Whether what follows `FIXME` is `(<ticket id>):`.
fn names_a_ticket(after: &str) -> bool {
    let Some(inner) = after.strip_prefix('(') else {
        return false;
    };
    let Some((id, tail)) = inner.split_once(')') else {
        return false;
    };
    tail.starts_with(':')
        && id.len() >= SHORTEST_ID
        && id.contains('_')
        && id
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_')
}

#[cfg(test)]
mod tests {
    use mockspace::testkit::LintFixture;

    use super::*;

    fn hits(source: &str) -> Vec<LintError> {
        AFixmeNamesItsTicket.check(&LintFixture::new(source).ctx())
    }

    // --- what must fire ------------------------------------------------

    #[test]
    fn a_bare_marker_fires() {
        let errors = hits("// FIXME: the carried default is not settled.\nfn f() {}\n");
        assert_eq!(errors.len(), 1, "{errors:?}");
        assert_eq!(errors[0].line, 1);
    }

    #[test]
    fn a_marker_naming_a_person_or_a_state_fires() {
        assert_eq!(hits("// FIXME(op): decide.\n").len(), 1);
        assert_eq!(hits("// FIXME(canon silent): decide.\n").len(), 1);
    }

    #[test]
    fn a_marker_with_no_colon_after_the_id_fires() {
        assert_eq!(hits("// FIXME(arvo_a_real_ticket_key) decide.\n").len(), 1);
    }

    #[test]
    fn a_marker_with_an_unclosed_id_fires() {
        assert_eq!(hits("// FIXME(arvo_a_real_ticket_key: decide.\n").len(), 1);
    }

    #[test]
    fn a_doc_comment_marker_fires() {
        assert_eq!(
            hits("/// FIXME: documented later.\npub fn f() {}\n").len(),
            1
        );
        assert_eq!(hits("//! FIXME: this module.\n").len(), 1);
    }

    #[test]
    fn a_marker_on_an_interior_line_of_a_block_comment_fires_on_that_line() {
        let errors = hits("/*\n * FIXME: later.\n */\nfn f() {}\n");
        assert_eq!(errors.len(), 1, "{errors:?}");
        assert_eq!(errors[0].line, 2);
    }

    #[test]
    fn a_trailing_marker_after_code_fires() {
        assert_eq!(hits("let n = 1; // FIXME: placeholder.\n").len(), 1);
    }

    #[test]
    fn every_bare_marker_is_reported_on_its_own_line() {
        let errors = hits("// FIXME: a.\nfn f() {}\n// FIXME: b.\n");
        assert_eq!(errors.iter().map(|e| e.line).collect::<Vec<_>>(), vec![
            1, 3
        ]);
    }

    // --- what must stay silent -----------------------------------------

    #[test]
    fn a_marker_naming_its_ticket_is_silent() {
        let source = "// FIXME(arvo_quotient_and_conversion_positions_past_the_slot_index): the\n";
        assert!(hits(source).is_empty());
    }

    /// The control for the arms above: the same marker, bare, fires. A lint
    /// that refused every marker would pass the arms above and fail here.
    #[test]
    fn control_the_same_marker_without_its_id_fires() {
        assert_eq!(hits("// FIXME: the\n").len(), 1);
    }

    #[test]
    fn a_comment_mentioning_the_word_further_in_is_prose_and_silent() {
        assert!(hits("//! impl below is marked `// FIXME:` at the declaration.\n").is_empty());
        assert!(hits("// a later FIXME(op) will say.\n").is_empty());
    }

    #[test]
    fn the_word_in_a_string_literal_is_not_a_marker() {
        assert!(hits("let s = \"// FIXME: not a comment\";\n").is_empty());
    }

    #[test]
    fn a_comment_with_no_marker_is_silent() {
        assert!(hits("// a note\n/// docs\nfn f() {}\n").is_empty());
    }

    #[test]
    fn an_id_too_short_to_be_a_ticket_key_is_not_one() {
        assert_eq!(hits("// FIXME(todo_it): later.\n").len(), 1);
        assert_eq!(hits("// FIXME(a_b_c_d_e_f_g): later.\n").len(), 0);
    }

    // --- registration and severity -------------------------------------

    #[test]
    fn it_answers_to_the_name_the_gate_and_the_config_use() {
        assert_eq!(AFixmeNamesItsTicket.name(), NAME);
    }

    #[test]
    fn it_is_not_declared_off_so_it_runs_at_all() {
        crate::canon_lint_testkit::assert_not_declared_off(&AFixmeNamesItsTicket);
    }

    #[test]
    fn it_reaches_the_pack_the_engine_is_handed() {
        crate::crate_lint_testkit::assert_registered(NAME);
    }

    #[test]
    fn its_findings_block_at_every_gate() {
        let errors = hits("// FIXME: x.\n");
        crate::crate_lint_testkit::assert_findings_carry(
            &AFixmeNamesItsTicket,
            &errors,
            Severity::HARD_ERROR,
        );
    }
}
