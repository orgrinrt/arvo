//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! Lint: the platform-width points in `arvo-format` are spelled over `usize::BITS`.
//!
//! `USize` and `ISize` are aliases, `UFixed<{ usize::BITS }, 0>` and
//! `Integer<{ usize::BITS }>`, and what makes each a target-bound member of the
//! format family rather than one fixed point is that it reads the pointer width
//! instead of naming one. `usize::BITS` is the pointer width by the language's own
//! definition, so the alias is the literal point at the target's width on every
//! target exactly when it is spelled that way.
//!
//! No test run on one host can check that. An alias written as
//! `UFixed<64, 0>` answers every question the correct one answers on a 64-bit
//! host, and the arm that would tell them apart only runs on a target this suite
//! is not run on. A `cargo check` at such a target does not help, because it
//! type-checks an assertion without evaluating it. So the property is checked
//! where it lives, in the source text, on every commit and with no second build.
//!
//! **A missing alias is a finding too.** Deleting one would otherwise pass as
//! clean, and a lint that reports nothing over nothing is the failure it guards.

use mockspace::{CrateLint, Lint, LintContext, LintError, Severity};

/// The lint's own name, used in its findings and keyed by `[lints.<name>]`.
const NAME: &str = "the-platform-width-points-read-the-pointer-width";

/// The crate the two aliases are declared in.
const THE_CRATE: &str = "arvo-format";

/// The declarations this reads, by the text that opens each.
const THE_ALIASES: &[&str] = &["pub type USize", "pub type ISize"];

/// What each right-hand side has to read.
const THE_POINTER_WIDTH: &str = "usize::BITS";

pub fn lint() -> Box<dyn CrateLint> {
    Box::new(ThePlatformWidthPointsReadThePointerWidth)
}

struct ThePlatformWidthPointsReadThePointerWidth;

impl Lint for ThePlatformWidthPointsReadThePointerWidth {
    fn name(&self) -> &'static str {
        NAME
    }

    fn default_severity(&self) -> Severity {
        Severity::HARD_ERROR
    }
}

impl CrateLint for ThePlatformWidthPointsReadThePointerWidth {
    fn check(&self, ctx: &LintContext) -> Vec<LintError> {
        if ctx.should_skip_proc_macro_source_lint() {
            return Vec::new();
        }
        if ctx.crate_name != THE_CRATE {
            return Vec::new();
        }
        // The dispatcher hands a source lint the same context once per module
        // file, and this one reads every file out of `all_sources`, so it runs
        // on the pass where `source` is the crate root and on no other.
        let Some(root) = ctx.all_sources.first() else {
            return Vec::new();
        };
        if ctx.source != root.text {
            return Vec::new();
        }
        let files: Vec<(String, &str)> = ctx
            .all_sources
            .iter()
            .map(|f| (f.rel_path.to_string_lossy().to_string(), f.text.as_str()))
            .collect();
        verdicts(&files)
            .into_iter()
            .map(|(path, line, message)| {
                let mut error = LintError::with_severity(
                    ctx.crate_name.to_string(),
                    line,
                    NAME,
                    message,
                    Severity::HARD_ERROR,
                );
                error.path = path;
                error
            })
            .collect()
    }
}

/// Every finding over the crate's files, as a path, a line and a message.
///
/// Split from the context so a test can hand it text directly.
fn verdicts(files: &[(String, &str)]) -> Vec<(Option<String>, usize, String)> {
    let mut found = Vec::new();
    for alias in THE_ALIASES {
        let mut seen = false;
        for (path, text) in files {
            for (idx, line) in text.lines().enumerate() {
                let code = line.trim_start();
                if code.starts_with("//") || !code.starts_with(alias) {
                    continue;
                }
                // The name must end where the alias text ends, so `USizeLike`
                // is not read as `USize`.
                let rest = &code[alias.len() ..];
                if rest.starts_with(|c: char| c.is_ascii_alphanumeric() || c == '_') {
                    continue;
                }
                seen = true;
                if !rest.contains(THE_POINTER_WIDTH) {
                    found.push((
                        Some(path.clone()),
                        idx + 1,
                        format!(
                            "`{alias}` does not read `{THE_POINTER_WIDTH}`. A platform-width point \
                             is the literal point at the target's pointer width, and it is that on \
                             every target only when the width is read rather than written: spell \
                             it `UFixed<{{ usize::BITS }}, 0>` or `Integer<{{ usize::BITS }}>`."
                        ),
                    ));
                }
            }
        }
        if !seen {
            found.push((
                None,
                0,
                format!(
                    "`{alias}` is not declared anywhere in `{THE_CRATE}`. The design ships both \
                     platform-width points in `points`, and this lint has nothing to check without \
                     them; restore the alias, or change the design and this lint together."
                ),
            ));
        }
    }
    found
}

#[cfg(test)]
mod tests {
    use mockspace::testkit::LintFixture;

    use super::*;

    /// The shipped spelling, as `points` carries it.
    const SHIPPED: &str = "\
    pub type USize = UFixed<{ usize::BITS }, 0>;
    pub type ISize = Integer<{ usize::BITS }>;
";

    fn hits(source: &str) -> Vec<LintError> {
        let fixture = LintFixture::new(source).with_crate_name("arvo-format", "format");
        ThePlatformWidthPointsReadThePointerWidth.check(&fixture.ctx())
    }

    #[test]
    fn it_answers_to_the_name_the_gate_and_the_config_use() {
        assert_eq!(ThePlatformWidthPointsReadThePointerWidth.name(), NAME);
        assert_eq!(NAME, "the-platform-width-points-read-the-pointer-width");
    }

    #[test]
    fn it_is_not_declared_off_so_it_runs_at_all() {
        crate::canon_lint_testkit::assert_not_declared_off(
            &ThePlatformWidthPointsReadThePointerWidth,
        );
    }

    #[test]
    fn it_reaches_the_pack_the_engine_is_handed() {
        crate::crate_lint_testkit::assert_registered(NAME);
    }

    #[test]
    fn its_findings_carry_its_own_declared_severity() {
        let errors =
            hits("pub type USize = UFixed<64, 0>;\npub type ISize = Integer<{ usize::BITS }>;\n");
        crate::crate_lint_testkit::assert_findings_carry(
            &ThePlatformWidthPointsReadThePointerWidth,
            &errors,
            Severity::HARD_ERROR,
        );
    }

    #[test]
    fn the_control_the_shipped_spelling_is_silent() {
        assert!(hits(SHIPPED).is_empty(), "{:?}", hits(SHIPPED));
    }

    #[test]
    fn a_literal_width_fires_for_each_alias() {
        for (source, alias) in [
            (
                "pub type USize = UFixed<64, 0>;\npub type ISize = Integer<{ usize::BITS }>;\n",
                "USize",
            ),
            (
                "pub type USize = UFixed<{ usize::BITS }, 0>;\npub type ISize = Integer<32>;\n",
                "ISize",
            ),
        ] {
            let errors = hits(source);
            assert_eq!(errors.len(), 1, "{alias}: {errors:?}");
            assert!(
                errors[0].message.contains(alias),
                "{alias}: {}",
                errors[0].message
            );
        }
        assert_eq!(
            hits("pub type USize = UFixed<16, 0>;\npub type ISize = Integer<16>;\n").len(),
            2
        );
    }

    #[test]
    fn a_width_read_from_somewhere_else_fires() {
        // `u64::BITS` reads a width and is still a fixed one.
        let errors = hits(
            "pub type USize = UFixed<{ u64::BITS }, 0>;\npub type ISize = Integer<{ usize::BITS }>;\n",
        );
        assert_eq!(errors.len(), 1);
    }

    #[test]
    fn a_missing_alias_fires() {
        let errors = hits("pub type USize = UFixed<{ usize::BITS }, 0>;\n");
        assert_eq!(errors.len(), 1);
        assert!(errors[0].message.contains("ISize"));
        assert_eq!(hits("").len(), 2);
    }

    #[test]
    fn a_commented_out_alias_is_not_the_alias() {
        let errors = hits(
            "// pub type USize = UFixed<{ usize::BITS }, 0>;\npub type ISize = Integer<{ usize::BITS }>;\n",
        );
        assert_eq!(errors.len(), 1);
        assert!(errors[0].message.contains("not declared"));
    }

    #[test]
    fn a_longer_name_is_not_the_alias() {
        let errors = hits(
            "pub type USizeLike = UFixed<8, 0>;\npub type ISize = Integer<{ usize::BITS }>;\n",
        );
        assert_eq!(errors.len(), 1);
        assert!(errors[0].message.contains("not declared"));
    }

    #[test]
    fn another_crate_is_not_read() {
        let fixture = LintFixture::new("pub type USize = UFixed<64, 0>;\n")
            .with_crate_name("arvo-placement", "placement");
        assert!(
            ThePlatformWidthPointsReadThePointerWidth
                .check(&fixture.ctx())
                .is_empty()
        );
    }
}
