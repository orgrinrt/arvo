//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! The agent instructions quote the `standing` vocabulary, and nothing was
//! keeping the copy in step with the declaration.
//!
//! `mock/agent/MAIN.md.tmpl` generates `.claude/CLAUDE.md`, which every session
//! in this repository loads before it reads anything else. So a sentence there
//! about what a registry field means is the sentence every agent acts on, and a
//! wrong one is wrong everywhere at once, silently, because a paraphrase reads
//! exactly like a quotation.
//!
//! It has happened. The template said `standing` "records how many independent
//! instances back the claim", which drops `reached` and drops the clause
//! declaring that each instance derives before reading the other. That clause is
//! the whole of what the field counts. Four panel seats argued about what a
//! `standing` means while the answer sat in `mockspace.toml`, and the reason
//! nobody looked is that the agent surface already appeared to say it.
//!
//! The repair was to quote the declaration instead of restating it. This is what
//! keeps the quotation a quotation: the template's copy is compared against the
//! `proposal` namespace's `standing` description, and a drift in either
//! direction refuses.
//!
//! **Whitespace is normalised on both sides and nothing else is.** The
//! declaration is one long line and the template wraps it to fit a paragraph, so
//! a byte comparison would refuse on the wrap alone. Every other difference is a
//! difference in what the sentence says.
//!
//! **A lint rather than a tool.** There is no state of this repository in which
//! the instructions every agent loads may misquote the schema they are quoting,
//! so this refuses rather than reports.
use std::path::Path;

use mockspace::{Lint, LintError, RepoContext, RepoLint, Severity};

use crate::canon_rows::finding;

pub fn repo_lint() -> Box<dyn RepoLint> {
    Box::new(TheAgentSurfaceQuotes)
}

/// The lint's own name, used in its findings and keyed by `[lints.<name>]`.
const NAME: &str = "the-agent-surface-quotes-the-schema";

/// Where the declaration lives, which is outside `canon_paths`.
///
/// Worth saying rather than assuming: the field descriptions are part of the
/// registry's schema and the schema sits in the repository's mockspace
/// configuration, so the vocabulary a canon row is written in is declared in a
/// file the canon does not contain. That is a finding of its own and is filed
/// elsewhere. What this lint does is smaller and does not depend on it being
/// resolved: wherever the declaration lives, the copy of it agrees.
const SCHEMA: &str = "mockspace.toml";

/// The template that generates the instructions every session loads.
const SURFACE: &str = "mock/agent/MAIN.md.tmpl";

/// The namespace whose field is quoted.
const NAMESPACE: &str = "proposal";

/// The field whose description is quoted.
const FIELD: &str = "standing";

struct TheAgentSurfaceQuotes;
impl Lint for TheAgentSurfaceQuotes {
    fn name(&self) -> &'static str {
        NAME
    }

    fn default_severity(&self) -> Severity {
        Severity::HARD_ERROR
    }
}
impl RepoLint for TheAgentSurfaceQuotes {
    fn check_repo(&self, ctx: &RepoContext) -> Vec<LintError> {
        let root = ctx.mock_dir.parent().unwrap_or(&ctx.mock_dir).to_path_buf();
        check(&root)
    }
}

/// The verdict, over a repository root.
///
/// Split from the trait impl so a test can point it at a tree it built, and so
/// the one place that decides is one function.
fn check(root: &Path) -> Vec<LintError> {
    let Ok(schema) = std::fs::read_to_string(root.join(SCHEMA)) else {
        return vec![finding(
            NAME,
            Some("the-schema-is-not-there"),
            format!(
                "`{SCHEMA}` cannot be read, so the words the agent surface is supposed to \
                 be quoting cannot be established. This reports rather than passing, \
                 because a missing declaration and an agreeing one are the same silence."
            ),
        )];
    };
    let Some(declared) = declared_description(&schema) else {
        return vec![finding(
            NAME,
            Some("the-field-is-not-declared"),
            format!(
                "`{SCHEMA}` declares no `{FIELD}` field in the `{NAMESPACE}` namespace, so \
                 there is nothing for `{SURFACE}` to quote. Either the field was renamed \
                 and this lint names the old one, or the namespace moved."
            ),
        )];
    };
    let Ok(surface) = std::fs::read_to_string(root.join(SURFACE)) else {
        return vec![finding(
            NAME,
            Some("the-surface-is-not-there"),
            format!(
                "`{SURFACE}` cannot be read, so what every session loads about `{FIELD}` \
                 cannot be established."
            ),
        )];
    };

    if flattened(&surface).contains(&format!("\"{}\"", flattened(&declared))) {
        return Vec::new();
    }
    vec![finding(
        NAME,
        Some("the-surface-does-not-quote-the-declaration"),
        format!(
            "`{SURFACE}` does not carry the `{NAMESPACE}` namespace's `{FIELD}` description \
             as a quotation. Every session loads that file before it reads anything else, so \
             a restatement there is what every agent acts on, and a restatement is what \
             dropped the clause saying each instance derives before reading the other. Quote \
             the declaration in `{SCHEMA}`, in double quotes, wrapped however the paragraph \
             needs. The declaration reads: \"{declared}\""
        ),
    )]
}

/// The `description` of one field of one namespace, as the schema declares it.
///
/// Hand-scanned rather than parsed, because the lint pack carries no TOML
/// dependency and the shape being looked for is two lines deep. A namespace
/// opens at `[[registry.namespace]]` and names itself with `key`; a field opens
/// at `[[registry.namespace.field]]` and names itself with `name`. The walk
/// tracks which namespace it is inside, then which field, and returns the
/// `description` of the one that matches both.
fn declared_description(schema: &str) -> Option<String> {
    let mut namespace: Option<String> = None;
    let mut field: Option<String> = None;
    for line in schema.lines() {
        let line = line.trim();
        if line == "[[registry.namespace]]" {
            namespace = None;
            field = None;
            continue;
        }
        if line == "[[registry.namespace.field]]" {
            field = None;
            continue;
        }
        if line.starts_with('[') {
            // Any other table header leaves the registry's namespace section, so
            // a `standing` field declared under some unrelated table cannot be
            // picked up by mistake.
            namespace = None;
            field = None;
            continue;
        }
        if let Some(value) = quoted_value(line, "key") {
            namespace = Some(value);
            continue;
        }
        if let Some(value) = quoted_value(line, "name") {
            field = Some(value);
            continue;
        }
        if namespace.as_deref() == Some(NAMESPACE) && field.as_deref() == Some(FIELD) {
            if let Some(value) = quoted_value(line, "description") {
                return Some(value);
            }
        }
    }
    None
}

/// The value of a `<key> = "<value>"` line, where the line is that assignment.
///
/// The registry's descriptions carry backticks and apostrophes and no escaped
/// quotes, so the value is what sits between the first and last double quote on
/// the line. A line whose value is not a string, or is a different key, gives
/// nothing.
fn quoted_value(line: &str, key: &str) -> Option<String> {
    let rest = line.strip_prefix(key)?.trim_start();
    let rest = rest.strip_prefix('=')?.trim_start();
    let inner = rest.strip_prefix('"')?;
    let end = inner.rfind('"')?;
    Some(inner[..end].to_string())
}

/// The text with every run of whitespace collapsed to one space.
///
/// The declaration is one long line and the quotation is wrapped to a paragraph
/// width, so the wrap is the one difference between them that carries nothing.
fn flattened(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use mockspace::Lint;

    use super::{
        check, declared_description, flattened, quoted_value, TheAgentSurfaceQuotes, FIELD, NAME,
        NAMESPACE, SCHEMA, SURFACE,
    };
    use crate::canon_lint_testkit::{
        assert_findings_block_at, assert_not_declared_off, assert_registered, ctx_at, plant,
        planted_tree, repo_root, view,
    };

    /// The declaration as this repository actually carries it.
    const DECLARED: &str = "How many independent instances reached it, over the region they \
                            share. `one_expert`; `two_experts`, each deriving before reading \
                            the other; `three_or_more`; `cross_topic` where separate topics \
                            arrived at it without citing each other, which is the strongest \
                            thing this panel produces; `contested` where it is stated because \
                            somebody stated it and somebody else disagrees.";

    /// A schema declaring one namespace with one field, as the real file shapes it.
    fn schema_declaring(namespace: &str, field: &str, description: &str) -> String {
        format!(
            "[[registry.namespace]]\nkey = \"{namespace}\"\ntitle = \"T\"\n\n\
             [[registry.namespace.field]]\nname = \"topic\"\ntype = \"topic\"\n\
             description = \"something else entirely\"\n\n\
             [[registry.namespace.field]]\nname = \"{field}\"\ntype = \"string\"\n\
             required = true\ndescription = \"{description}\"\n"
        )
    }

    /// A tree carrying a schema and a surface, either of which the caller shapes.
    fn tree_with(what: &str, schema: &str, surface: &str) -> PathBuf {
        let dir = planted_tree(what);
        plant(&dir, SCHEMA, schema);
        plant(&dir, SURFACE, surface);
        dir
    }

    /// A surface quoting the words it is handed, wrapped the way a paragraph is.
    ///
    /// Wrapped deliberately rather than written on one line: the wrap is the
    /// difference the real template carries, so an arm asserting a match over an
    /// unwrapped copy would pass without exercising the normalisation.
    fn surface_quoting(words: &str) -> String {
        let mut wrapped = String::from("  in these words:\n  \"");
        let mut column = 0;
        for word in words.split_whitespace() {
            if column > 60 {
                wrapped.push_str("\n  ");
                column = 0;
            } else if column > 0 {
                wrapped.push(' ');
                column += 1;
            }
            wrapped.push_str(word);
            column += word.len();
        }
        wrapped.push_str("\"\n");
        wrapped
    }

    #[test]
    fn the_lint_is_named_registered_and_refuses() {
        let lint = TheAgentSurfaceQuotes;
        assert_eq!(lint.name(), NAME);
        assert_eq!(lint.default_severity(), mockspace::Severity::HARD_ERROR);
        assert_registered(NAME);
        assert_not_declared_off(&lint);
    }

    #[test]
    fn a_finding_carries_what_a_finding_owes() {
        let dir = tree_with(
            "finding-shape",
            &schema_declaring(NAMESPACE, FIELD, DECLARED),
            "  `standing` records how many independent instances back the claim.\n",
        );
        let empty = view(&[], &[]);
        assert_findings_block_at(&TheAgentSurfaceQuotes, &ctx_at(&dir.join("mock"), &empty));
    }

    #[test]
    fn a_verbatim_quotation_passes_however_it_is_wrapped() {
        let dir = tree_with(
            "quoted-verbatim",
            &schema_declaring(NAMESPACE, FIELD, DECLARED),
            &surface_quoting(DECLARED),
        );
        assert!(
            check(&dir).is_empty(),
            "a wrapped verbatim quotation is the state this lint exists to protect"
        );
    }

    #[test]
    fn the_paraphrase_this_lint_was_written_for_refuses() {
        // The real one, verbatim, from before the repair. It drops `reached` and
        // it drops the ordering clause, which is the whole of what the field
        // counts.
        let dir = tree_with(
            "the-paraphrase",
            &schema_declaring(NAMESPACE, FIELD, DECLARED),
            "  `standing` records how many independent instances back the claim, and\n  \
             a claim with two is still a proposal until a ruling stamps it.\n",
        );
        let out = check(&dir);
        assert_eq!(out.len(), 1);
        assert_eq!(
            out[0].finding_kind,
            Some("the-surface-does-not-quote-the-declaration")
        );
        assert!(
            out[0].message.contains(DECLARED),
            "the finding hands over the words that should have been quoted"
        );
    }

    #[test]
    fn one_dropped_clause_refuses() {
        // The control on the arm above: the failure this catches is not a
        // rewritten paragraph, it is a clause missing from an otherwise
        // faithful copy, and that is what a paraphrase looks like.
        let mangled = DECLARED.replace(", each deriving before reading the other", "");
        let dir = tree_with(
            "one-clause-short",
            &schema_declaring(NAMESPACE, FIELD, DECLARED),
            &surface_quoting(&mangled),
        );
        assert_eq!(check(&dir).len(), 1, "a missing clause is a misquotation");
    }

    #[test]
    fn a_drift_in_the_declaration_refuses_too() {
        // The other direction, and the one nobody watches for. Editing the
        // schema leaves the quotation behind, and the quotation is what agents
        // read.
        let dir = tree_with(
            "declaration-moved",
            &schema_declaring(NAMESPACE, FIELD, "How many instances reached it, and nothing more."),
            &surface_quoting(DECLARED),
        );
        assert_eq!(check(&dir).len(), 1, "the copy is stale, not the schema");
    }

    #[test]
    fn a_quotation_without_its_quotes_refuses() {
        // The words alone are a restatement that happens to be accurate today.
        // What makes it a quotation is that it is marked as one, so the next
        // person editing the paragraph knows not to reword it.
        let dir = tree_with(
            "no-quotes",
            &schema_declaring(NAMESPACE, FIELD, DECLARED),
            &format!("  in these words: {DECLARED}\n"),
        );
        assert_eq!(check(&dir).len(), 1);
    }

    #[test]
    fn a_missing_schema_is_reported_rather_than_passed() {
        let dir = planted_tree("no-schema");
        plant(&dir, SURFACE, &surface_quoting(DECLARED));
        let out = check(&dir);
        assert_eq!(out.len(), 1);
        assert_eq!(out[0].finding_kind, Some("the-schema-is-not-there"));
    }

    #[test]
    fn a_missing_surface_is_reported_rather_than_passed() {
        let dir = planted_tree("no-surface");
        plant(&dir, SCHEMA, &schema_declaring(NAMESPACE, FIELD, DECLARED));
        let out = check(&dir);
        assert_eq!(out.len(), 1);
        assert_eq!(out[0].finding_kind, Some("the-surface-is-not-there"));
    }

    #[test]
    fn an_undeclared_field_is_reported_rather_than_passed() {
        let dir = tree_with(
            "field-renamed",
            &schema_declaring(NAMESPACE, "footing", DECLARED),
            &surface_quoting(DECLARED),
        );
        let out = check(&dir);
        assert_eq!(out.len(), 1);
        assert_eq!(out[0].finding_kind, Some("the-field-is-not-declared"));
    }

    #[test]
    fn the_field_is_read_from_the_right_namespace() {
        // A `standing` in a different namespace is a different field with a
        // different description, and reading it would make this lint refuse a
        // correct quotation or accept a wrong one depending on which came first
        // in the file.
        let mut schema = schema_declaring("ruling", FIELD, "an entirely different sentence");
        schema.push('\n');
        schema.push_str(&schema_declaring(NAMESPACE, FIELD, DECLARED));
        let dir = tree_with("two-namespaces", &schema, &surface_quoting(DECLARED));
        assert!(
            check(&dir).is_empty(),
            "the description came from the wrong namespace"
        );
    }

    #[test]
    fn a_field_under_an_unrelated_table_is_not_picked_up() {
        // The control on the walk: a table header that is not the registry's
        // namespace closes the namespace, so a stray `name` and `description`
        // under, say, a lint's own configuration cannot answer for the schema.
        let schema = format!(
            "{}\n[lints.something]\nname = \"{FIELD}\"\ndescription = \"not the schema\"\n",
            schema_declaring(NAMESPACE, "footing", DECLARED)
        );
        let dir = tree_with("stray-table", &schema, &surface_quoting(DECLARED));
        let out = check(&dir);
        assert_eq!(out.len(), 1);
        assert_eq!(out[0].finding_kind, Some("the-field-is-not-declared"));
    }

    #[test]
    fn the_value_reader_takes_the_whole_string() {
        // The descriptions carry backticks, semicolons and apostrophes. Taking
        // to the last quote rather than the first is what makes a sentence with
        // a quoted phrase inside it survive, which is the shape a description
        // quoting somebody would take.
        assert_eq!(
            quoted_value("description = \"a `thing`; and it's fine\"", "description").as_deref(),
            Some("a `thing`; and it's fine")
        );
        assert_eq!(
            quoted_value("description = \"he said \"no\" and left\"", "description").as_deref(),
            Some("he said \"no\" and left")
        );
        assert_eq!(quoted_value("name = \"standing\"", "description"), None);
        assert_eq!(quoted_value("description = 4", "description"), None);
    }

    #[test]
    fn flattening_touches_the_wrap_and_nothing_else() {
        assert_eq!(flattened("a\n  b   c\n"), "a b c");
        assert_ne!(flattened("a b"), flattened("a  b c"));
    }

    #[test]
    fn the_repository_quotes_its_own_declaration() {
        // The arm that is about this repository rather than about a planted
        // tree, and the one that fires the day somebody rewords the paragraph.
        let found = check(&repo_root());
        assert!(
            found.is_empty(),
            "this repository's own agent surface has stopped quoting its schema: {:?}",
            found.iter().map(|e| &e.message).collect::<Vec<_>>()
        );
    }

    #[test]
    fn the_declaration_is_found_where_it_actually_lives() {
        // The control on the arm above: it asserts the repository is clean, and
        // a lint that could not find the declaration at all would report clean
        // for the wrong reason. It cannot: an unfound declaration is a finding.
        // What this pins is that the walk reaches the real file rather than a
        // shape only the planted trees have.
        let schema = std::fs::read_to_string(repo_root().join(SCHEMA))
            .expect("the repository carries its own schema");
        let declared = declared_description(&schema).expect("the field is declared");
        assert!(
            declared.contains("each deriving before reading the other"),
            "the clause the paraphrase dropped is what this lint keeps"
        );
    }
}
