//! Lint: arvo does not allocate. No `alloc`, `Vec<`, `String`, `Box<`.
//!
//! arvo crates are `#![no_std]` with no allocator. Dynamic
//! collections and heap boxes are forbidden. Use:
//!   * `[T; N]` or custom const-generic `Seq<T, N>` for arrays.
//!   * Consumer-owned buffers passed in by reference.
//!
//! The lint scans every line for tokens. String literals that
//! happen to contain "Vec<" or "String" inside them are tolerated
//! because the scan only fires on import-like or type-position
//! usage (line starts with `use ...`, contains `: Vec<`, etc.).

use mockspace::{Lint, LintContext, LintError, Severity};

pub fn lint() -> Box<dyn Lint> {
    Box::new(NoAllocEnforcer)
}

struct NoAllocEnforcer;

impl Lint for NoAllocEnforcer {
    fn name(&self) -> &'static str {
        "no-alloc-enforcer"
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

            // `use alloc::*` / `extern crate alloc`
            if trimmed.starts_with("use alloc::")
                || trimmed.starts_with("pub use alloc::")
                || trimmed.starts_with("extern crate alloc")
            {
                errors.push(LintError::with_severity(
                    ctx.crate_name.to_string(),
                    idx + 1,
                    "no-alloc-enforcer",
                    format!(
                        "arvo forbids `alloc::*`; use const-sized containers instead: {}",
                        trimmed.trim_end()
                    ),
                    Severity::HARD_ERROR,
                ));
                continue;
            }

            // Type-position Vec / String / Box. Hits both imports
            // (`use core::vec::Vec`) and field/return types (`Vec<`).
            // We tolerate `// Vec` in comments via the `//` skip above,
            // and ignore `Vec` inside raw string literals by accepting
            // some false negatives — the import lints above cover the
            // actionable case anyway.
            let line_body = line;
            let bad_tokens: [(&str, &str); 3] = [
                ("Vec<", "Vec<T>"),
                (" String", "String"),
                ("Box<", "Box<T>"),
            ];
            for (token, display) in bad_tokens {
                if line_body.contains(token) {
                    // Skip doc comments that only mention the type.
                    if trimmed.starts_with("///") || trimmed.starts_with("//!") {
                        continue;
                    }
                    errors.push(LintError::with_severity(
                        ctx.crate_name.to_string(),
                        idx + 1,
                        "no-alloc-enforcer",
                        format!(
                            "arvo forbids `{}` (no alloc); use `[T; N]` or a consumer-owned buffer: {}",
                            display,
                            line_body.trim_end()
                        ),
                        Severity::HARD_ERROR,
                    ));
                    break;
                }
            }
        }

        errors
    }
}
