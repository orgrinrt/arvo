//! Lint: `arvo-bits` is contracts only.
//!
//! The `arvo-bits` crate declares bit-level contracts — traits with
//! default methods and blanket impls. It must not declare fielded
//! `pub struct`s. Concrete storage types (`Mask64`, `Mask256`,
//! `BitMatrix`) live in `arvo-bitmask`.
//!
//! A field-less marker struct (e.g. `pub struct Foo;`) is tolerated
//! because it carries no storage — it exists for trait-level
//! tagging. A struct with a body (`pub struct Foo { ... }` or
//! `pub struct Foo(T);`) fails the lint.

use mockspace::{Lint, LintContext, LintError, Severity};

pub fn lint() -> Box<dyn Lint> {
    Box::new(ArvoBitsTraitsOnly)
}

struct ArvoBitsTraitsOnly;

impl Lint for ArvoBitsTraitsOnly {
    fn name(&self) -> &'static str {
        "arvo-bits-traits-only"
    }

    fn default_severity(&self) -> Severity {
        Severity::HARD_ERROR
    }

    fn check(&self, ctx: &LintContext) -> Vec<LintError> {
        if ctx.is_proc_macro_crate() {
            return Vec::new();
        }

        if ctx.crate_name != "arvo-bits" {
            return Vec::new();
        }

        let mut errors = Vec::new();

        for (idx, line) in ctx.source.lines().enumerate() {
            let trimmed = line.trim();

            if trimmed.starts_with("//") {
                continue;
            }

            if !trimmed.starts_with("pub struct ") {
                continue;
            }

            // Marker struct: ends with `;` after the name. No body.
            // Example: `pub struct Warm;`
            if trimmed.ends_with(';') && !trimmed.contains('(') && !trimmed.contains('{') {
                continue;
            }

            // Anything else (tuple struct, field struct) is forbidden.
            errors.push(LintError::with_severity(
                ctx.crate_name.to_string(),
                idx + 1,
                "arvo-bits-traits-only",
                format!(
                    "arvo-bits is contracts only; move concrete storage to arvo-bitmask: {}",
                    trimmed
                ),
                Severity::HARD_ERROR,
            ));
        }

        errors
    }
}
