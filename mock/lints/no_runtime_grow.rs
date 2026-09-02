//! Lint: arvo sizes are const; no runtime grow.
//!
//! Flags the classic dynamic-collection mutators: `.push(`,
//! `.resize(`, `.extend(`, `Vec::with_capacity`. The presence of
//! these calls means the container is expected to grow at runtime,
//! which arvo doesn't support.
//!
//! Severity is `ADVISORY` by default (warn but never block). The
//! harder `no-alloc-enforcer` lint already rejects `Vec` / `String` /
//! `Box` outright; this lint exists as a data indicator for reviews:
//! any hit is a signal that someone is mentally modelling a
//! runtime-grow pattern even if they haven't written `Vec` yet.
//!
//! **This repo's own `mockspace.toml` overrides that default.**
//! `[lints.no-runtime-grow]` sets `commit = build = push = "error"`,
//! which is exactly `Severity::HARD_ERROR` at every gate. A configured
//! `base` override restamps every finding's severity regardless of
//! what the lint's own `check` set, so in this repository this lint
//! does block, at every gate, contradicting the "warn but never
//! block" sentence above. See the test that measures this against the
//! same public entry point the real gate uses.

use mockspace::{CrateLint, Lint, LintContext, LintError, Severity};

pub fn lint() -> Box<dyn CrateLint> {
    Box::new(NoRuntimeGrow)
}

struct NoRuntimeGrow;

impl Lint for NoRuntimeGrow {
    fn name(&self) -> &'static str {
        "no-runtime-grow"
    }

    fn default_severity(&self) -> Severity {
        Severity::ADVISORY
    }
}

impl CrateLint for NoRuntimeGrow {
    fn check(&self, ctx: &LintContext) -> Vec<LintError> {
        // `is_proc_macro_crate` does not consider the `lint_proc_macro_source`
        // opt-in (its own doc says so); this is the "should I skip this
        // source lint" decision, which is what that method exists for.
        if ctx.should_skip_proc_macro_source_lint() {
            return Vec::new();
        }

        let mut errors = Vec::new();

        let patterns: &[(&str, &str)] = &[
            (".push(", "`.push(...)`"),
            (".resize(", "`.resize(...)`"),
            (".extend(", "`.extend(...)`"),
            ("Vec::with_capacity", "`Vec::with_capacity(...)`"),
        ];

        for (idx, line) in ctx.source.lines().enumerate() {
            let trimmed = line.trim_start();

            if trimmed.starts_with("//") {
                continue;
            }

            for (needle, display) in patterns {
                if line.contains(needle) {
                    errors.push(LintError::with_severity(
                        ctx.crate_name.to_string(),
                        idx + 1,
                        "no-runtime-grow",
                        format!(
                            "{} indicates runtime grow; arvo sizes are const (use `[T; N]` or const-generic `Seq<T, N>`): {}",
                            display,
                            line.trim_end()
                        ),
                        Severity::ADVISORY,
                    ));
                    break;
                }
            }
        }

        errors
    }
}

#[cfg(test)]
mod tests {
    use mockspace::testkit::LintFixture;

    use super::*;

    fn hits(source: &str) -> Vec<LintError> {
        let fixture = LintFixture::new(source);
        NoRuntimeGrow.check(&fixture.ctx())
    }

    // --- registration and severity -----------------------------------

    #[test]
    fn it_answers_to_the_name_the_gate_and_the_config_use() {
        assert_eq!(NoRuntimeGrow.name(), "no-runtime-grow");
    }

    #[test]
    fn it_is_not_declared_off_so_it_runs_at_all() {
        crate::canon_lint_testkit::assert_not_declared_off(&NoRuntimeGrow);
    }

    #[test]
    fn it_reaches_the_pack_the_engine_is_handed() {
        crate::crate_lint_testkit::assert_registered("no-runtime-grow");
    }

    /// Its own declared default is `ADVISORY`, and every finding it
    /// produces carries that severity when nothing overrides it. This
    /// is the claim the module doc's "warn but never block" rests on,
    /// and the next test shows it does not survive this repo's actual
    /// configuration.
    #[test]
    fn its_undeclared_findings_carry_its_advisory_default() {
        let errors = hits("v.push(1);\n");
        crate::crate_lint_testkit::assert_findings_carry(
            &NoRuntimeGrow,
            &errors,
            Severity::ADVISORY,
        );
    }

    /// This repo's `mockspace.toml` carries:
    ///
    /// ```toml
    /// [lints.no-runtime-grow]
    /// commit = "error"
    /// build = "error"
    /// push = "error"
    /// ```
    ///
    /// which is `Severity::HARD_ERROR`. Run through the same public
    /// `mockspace::check_crate_with_extra` entry point the real gate
    /// uses, with that override applied, the finding's severity is
    /// restamped from `ADVISORY` to `HARD_ERROR`: this lint blocks
    /// every gate in this repository today, whatever its module doc
    /// says.
    #[test]
    fn the_configured_override_in_this_repos_mockspace_toml_makes_it_block() {
        let fixture = LintFixture::new("v.push(1);\n");
        let errors = crate::crate_lint_testkit::findings_under_override(
            &fixture.ctx(),
            Box::new(NoRuntimeGrow),
            "no-runtime-grow",
            Severity::HARD_ERROR,
        );
        assert_eq!(errors.len(), 1);
        assert_eq!(
            errors[0].severity,
            Severity::HARD_ERROR,
            "the configured override should restamp the ADVISORY finding to HARD_ERROR"
        );
    }

    // --- the four patterns -----------------------------------------------

    #[test]
    fn a_push_call_fires() {
        assert_eq!(hits("v.push(1);\n").len(), 1);
    }

    #[test]
    fn a_resize_call_fires() {
        assert_eq!(hits("v.resize(4, 0);\n").len(), 1);
    }

    #[test]
    fn an_extend_call_fires() {
        assert_eq!(hits("v.extend(other);\n").len(), 1);
    }

    #[test]
    fn a_vec_with_capacity_call_fires() {
        assert_eq!(hits("let v = Vec::with_capacity(4);\n").len(), 1);
    }

    #[test]
    fn only_the_first_pattern_on_a_line_is_reported() {
        let errors = hits("v.push(1); v.extend(other);\n");
        assert_eq!(errors.len(), 1);
    }

    #[test]
    fn every_hit_names_its_own_line() {
        let errors = hits("fn a() {}\nv.push(1);\nfn b() {}\nv.extend(o);\n");
        assert_eq!(errors.len(), 2);
        assert_eq!(errors[0].line, 2);
        assert_eq!(errors[1].line, 4);
    }

    // --- what must stay silent ----------------------------------------

    #[test]
    fn a_similarly_named_method_that_is_not_the_pattern_is_silent() {
        // `.push_str(` and `.resized(` do not carry the literal
        // substrings `.push(` / `.resize(`, since neither has an open
        // paren directly after the matched prefix.
        assert!(hits("s.push_str(\"x\");\n").is_empty());
        assert!(hits("v.resized(4);\n").is_empty());
    }

    #[test]
    fn a_full_line_comment_mentioning_the_patterns_is_silent() {
        assert!(hits("// avoid .push(, .resize(, .extend(\n").is_empty());
    }

    #[test]
    fn a_doc_comment_mentioning_the_patterns_is_silent() {
        // The comment skip here is a single whole-line `//` check ahead
        // of the pattern loop, so `///` and `//!` are caught by the
        // same branch as a plain `//` line; there is no separate
        // doc-comment carve-out to test the way `no-alloc-enforcer`
        // needed one.
        assert!(hits("/// Never call `.push(` on arvo containers.\n").is_empty());
        assert!(hits("//! `.extend(` is forbidden here too.\n").is_empty());
    }

    #[test]
    fn a_fixed_size_array_construction_is_silent() {
        assert!(hits("let a: [u8; 4] = [0; 4];\n").is_empty());
    }

    #[test]
    fn the_pattern_still_fires_inside_a_macro_body() {
        let errors = hits("macro_rules! m {\n    () => { v.push(1); };\n}\n");
        assert_eq!(errors.len(), 1);
    }

    #[test]
    fn the_pattern_still_fires_inside_a_cfg_test_module() {
        let errors = hits("#[cfg(test)]\nmod tests {\n    fn t() { v.push(1); }\n}\n");
        assert_eq!(errors.len(), 1);
    }

    // --- proc-macro exemption --------------------------------------------

    #[test]
    fn a_declared_proc_macro_crate_is_skipped() {
        let fixture = LintFixture::new("v.push(1);\n")
            .with_crate_name("acme-macros", "macros")
            .with_proc_macro_crates(&["acme-macros"]);
        assert!(NoRuntimeGrow.check(&fixture.ctx()).is_empty());
    }

    #[test]
    fn the_opt_in_override_is_honoured_rather_than_ignored() {
        let fixture = LintFixture::new("v.push(1);\n")
            .with_crate_name("acme-macros", "macros")
            .with_proc_macro_crates(&["acme-macros"])
            .with_lint_proc_macro_source(true);
        assert_eq!(
            NoRuntimeGrow.check(&fixture.ctx()).len(),
            1,
            "the opt-in override should make this crate's source lintable again"
        );
    }

    // --- catalogued gaps ---------------------------------------------------

    #[test]
    #[ignore = "catalogue: a trailing `//` comment on a real line of code \
                is scanned as code; `let v = w; // could .push( here \
                later` false-positives; needs tree-sitter-aware scanning \
                to fix"]
    fn a_trailing_comment_after_real_code_is_wrongly_flagged() {
        let errors = hits("let v = w; // could .push( here later\n");
        assert!(
            errors.is_empty(),
            "a trailing comment mentioning the pattern is not a call of it"
        );
    }

    #[test]
    #[ignore = "catalogue: a string literal naming one of the forbidden \
                patterns false-positives; needs tree-sitter-aware \
                scanning to fix"]
    fn a_string_literal_naming_a_forbidden_pattern_is_wrongly_flagged() {
        let errors = hits("let msg = \"call .push( next\";\n");
        assert!(
            errors.is_empty(),
            "text inside a string literal is not a call of the pattern"
        );
    }

    #[test]
    #[ignore = "catalogue: a block-comment interior line naming one of \
                the forbidden patterns false-positives; needs \
                tree-sitter-aware scanning to fix"]
    fn an_interior_line_of_a_block_comment_is_wrongly_flagged() {
        let errors = hits("/*\nv.push(1);\n*/\n");
        assert!(
            errors.is_empty(),
            "a block-comment interior line should not read as real source"
        );
    }
}
