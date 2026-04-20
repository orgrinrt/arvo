//! Lint: arvo sizes are const; no runtime grow.
//!
//! Flags the classic dynamic-collection mutators: `.push(`,
//! `.resize(`, `.extend(`, `Vec::with_capacity`. The presence of
//! these calls means the container is expected to grow at runtime,
//! which arvo doesn't support.
//!
//! Severity is `ADVISORY` by default (warn but never block). The
//! harder `no-alloc-enforcer` lint already rejects `Vec` / `String` /
//! `Box` outright; this lint exists as a data indicator for reviews
//! — any hit is a signal that someone is mentally modelling a
//! runtime-grow pattern even if they haven't written `Vec` yet.

use mockspace::{Lint, LintContext, LintError, Severity};

pub fn lint() -> Box<dyn Lint> {
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

    fn check(&self, ctx: &LintContext) -> Vec<LintError> {
        if ctx.is_proc_macro_crate() {
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
