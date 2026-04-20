//! Lint: arvo is `#![no_std]`. Any `use std::` import is a violation.
//!
//! arvo is a numeric + analysis substrate with zero platform
//! dependency. Crates must not import `std::*`. Consumers that need
//! platform integration pull in their own `std`-bearing crates.
//!
//! Scope: every arvo crate under `mock/crates/`. The lint scans raw
//! source (line by line) to catch imports before they reach the
//! compile pipeline — giving a clear arvo-specific message rather
//! than a generic `unresolved import` error from `no_std` mode.

use mockspace::{Lint, LintContext, LintError, Severity};

pub fn lint() -> Box<dyn Lint> {
    Box::new(NoStdEnforcer)
}

struct NoStdEnforcer;

impl Lint for NoStdEnforcer {
    fn name(&self) -> &'static str {
        "no-std-enforcer"
    }

    fn default_severity(&self) -> Severity {
        Severity::HARD_ERROR
    }

    fn check(&self, ctx: &LintContext) -> Vec<LintError> {
        if ctx.is_proc_macro_crate() {
            return Vec::new();
        }

        let mut errors = Vec::new();

        for (idx, line) in ctx.source.lines().enumerate() {
            let trimmed = line.trim_start();

            if trimmed.starts_with("//") {
                continue;
            }

            // Match `use std::`, `use ::std::`, `extern crate std`.
            let hit = trimmed.starts_with("use std::")
                || trimmed.starts_with("use ::std::")
                || trimmed.starts_with("pub use std::")
                || trimmed.starts_with("pub use ::std::")
                || trimmed.starts_with("extern crate std");

            if hit {
                errors.push(LintError::with_severity(
                    ctx.crate_name.to_string(),
                    idx + 1,
                    "no-std-enforcer",
                    format!(
                        "arvo is `#![no_std]`; `use std::` is forbidden: {}",
                        trimmed.trim_end()
                    ),
                    Severity::HARD_ERROR,
                ));
            }
        }

        errors
    }
}
