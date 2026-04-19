//! Lint: arvo never pays for dynamic dispatch.
//!
//! No `dyn Trait`, no `TypeId`, no `std::any::*`. Monomorphisation
//! IS the dispatch. Generic code is instantiated at the use site;
//! the compiler proves devirtualisation and we never pay for runtime
//! lookup.
//!
//! This catches three shapes:
//!   * ` dyn ` (with surrounding whitespace to avoid matching
//!     identifiers like `dyndns` or `already_done`).
//!   * `TypeId`.
//!   * `std::any` and `core::any` paths.

use mockspace::{Lint, LintContext, LintError, Severity};

pub fn lint() -> Box<dyn Lint> {
    Box::new(NoDynamicDispatch)
}

struct NoDynamicDispatch;

impl Lint for NoDynamicDispatch {
    fn name(&self) -> &'static str {
        "no-dynamic-dispatch"
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

            // Skip comments wholesale.
            if trimmed.starts_with("//") {
                continue;
            }

            // ` dyn ` — space-delimited to avoid false positives on
            // `dyn` as part of an identifier.
            let has_dyn = line.contains(" dyn ")
                || line.contains("&dyn ")
                || line.contains("&'static dyn ")
                || line.contains("<dyn ")
                || line.contains("(dyn ")
                || line.ends_with(" dyn")
                || line.trim() == "dyn";

            if has_dyn {
                errors.push(LintError::with_severity(
                    ctx.crate_name.to_string(),
                    idx + 1,
                    "no-dynamic-dispatch",
                    format!(
                        "arvo forbids `dyn Trait`; make the function generic over the trait bound instead: {}",
                        line.trim_end()
                    ),
                    Severity::HARD_ERROR,
                ));
                continue;
            }

            if line.contains("TypeId") {
                errors.push(LintError::with_severity(
                    ctx.crate_name.to_string(),
                    idx + 1,
                    "no-dynamic-dispatch",
                    format!(
                        "arvo forbids `TypeId`; the monomorphised function IS the type identity: {}",
                        line.trim_end()
                    ),
                    Severity::HARD_ERROR,
                ));
                continue;
            }

            if line.contains("std::any") || line.contains("core::any") {
                errors.push(LintError::with_severity(
                    ctx.crate_name.to_string(),
                    idx + 1,
                    "no-dynamic-dispatch",
                    format!(
                        "arvo forbids `std::any` / `core::any`; no runtime type erasure: {}",
                        line.trim_end()
                    ),
                    Severity::HARD_ERROR,
                ));
            }
        }

        errors
    }
}
