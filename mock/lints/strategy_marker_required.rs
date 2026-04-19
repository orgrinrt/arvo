//! Lint: numeric types carry `S: Strategy`.
//!
//! Every `pub struct` or `pub type` defined in a numeric crate
//! (`arvo`, or any `arvo-*` crate that declares numeric containers)
//! must carry the `S: Strategy` generic parameter — or include a
//! `// strategy-exempt:` comment explaining why not.
//!
//! Default severity is `PUSH_GATE` (warn on build, error on push):
//! strategy markers are mandatory for correctness but new types land
//! incrementally, so commit-time blocking is too harsh.
//!
//! Currently scoped to the `arvo` crate only. Subcrate numeric types
//! (if any) should opt in by matching the scope list below.

use mockspace::{Lint, LintContext, LintError, Severity};

const NUMERIC_CRATES: &[&str] = &["arvo"];

pub fn lint() -> Box<dyn Lint> {
    Box::new(StrategyMarkerRequired)
}

struct StrategyMarkerRequired;

impl Lint for StrategyMarkerRequired {
    fn name(&self) -> &'static str {
        "strategy-marker-required"
    }

    fn default_severity(&self) -> Severity {
        Severity::PUSH_GATE
    }

    fn check(&self, ctx: &LintContext) -> Vec<LintError> {
        if ctx.is_proc_macro_crate() {
            return Vec::new();
        }

        if !NUMERIC_CRATES.iter().any(|c| *c == ctx.crate_name) {
            return Vec::new();
        }

        let mut errors = Vec::new();
        let mut prev_was_exempt = false;

        for (idx, line) in ctx.source.lines().enumerate() {
            let trimmed = line.trim();

            // Opt-out comment on the line immediately before the decl.
            if trimmed.starts_with("// strategy-exempt:") {
                prev_was_exempt = true;
                continue;
            }

            // Only inspect top-level pub declarations.
            let is_pub_struct = trimmed.starts_with("pub struct ");
            let is_pub_type = trimmed.starts_with("pub type ");
            let is_pub_enum = trimmed.starts_with("pub enum ");

            if !(is_pub_struct || is_pub_type || is_pub_enum) {
                // Reset exempt only on non-blank, non-attribute lines.
                if !trimmed.is_empty()
                    && !trimmed.starts_with("#[")
                    && !trimmed.starts_with("///")
                    && !trimmed.starts_with("//!")
                {
                    prev_was_exempt = false;
                }
                continue;
            }

            if prev_was_exempt {
                prev_was_exempt = false;
                continue;
            }

            // Heuristic: does the decl mention a Strategy-like parameter?
            // Acceptable forms: `S: Strategy`, `S: Strategy = Warm`,
            // `Strategy<...>`, `<S: Strategy`. Bare numeric-primitive
            // aliases (e.g. `pub type Bool = ...`) are tolerated only
            // if they reference a known strategied type on their RHS.
            let mentions_strategy = line.contains("Strategy")
                || line.contains("UFixed<")
                || line.contains("IFixed<")
                || line.contains("FastFloat")
                || line.contains("StrictFloat");

            if !mentions_strategy {
                errors.push(LintError::with_severity(
                    ctx.crate_name.to_string(),
                    idx + 1,
                    "strategy-marker-required",
                    format!(
                        "numeric type missing `S: Strategy` parameter (add `S: Strategy = Warm` or `// strategy-exempt: <reason>`): {}",
                        trimmed
                    ),
                    Severity::PUSH_GATE,
                ));
            }
        }

        errors
    }
}
