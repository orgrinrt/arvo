//! Lint: `arvo-bits` is bit-level primitives — contracts and
//! opaque-bit concrete containers. No arithmetic fielded structs.
//!
//! The `arvo-bits` crate ships two kinds of surface:
//!
//! - Bit-level trait contracts (`BitWidth`, `BitAccess`,
//!   `BitSequence`) with default methods and blanket impls.
//! - Opaque-bit concrete containers: fielded `pub struct`s whose
//!   values are non-arithmetic identities. Listed in
//!   `ALLOWED_OPAQUE_BITS` below.
//!
//! Arithmetic fielded types (`UFixed`, `IFixed`) live in L0 `arvo`;
//! mask concretes (`Mask64`, `Mask256`, `BitMatrix`) live in L2
//! `arvo-bitmask`. Anything else with a struct body in `arvo-bits`
//! fails the lint.
//!
//! A field-less marker struct (`pub struct Foo;`) is always
//! tolerated — it carries no storage.

use mockspace::{Lint, LintContext, LintError, Severity};

/// Opaque-bit concrete containers permitted in `arvo-bits`.
///
/// Types here are fielded but non-arithmetic: identity containers
/// compared by `Eq`, not ordered by arithmetic. To add a new entry,
/// confirm the type satisfies both properties, then extend this
/// list in a design round.
const ALLOWED_OPAQUE_BITS: &[&str] = &[
    "Bits", // opaque N-bit container; arvo-hash ContentHash alias sits on this
];

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

            // Fielded struct: check against the opaque-bit allowlist.
            if let Some(name) = extract_struct_name(trimmed) {
                if ALLOWED_OPAQUE_BITS.contains(&name.as_str()) {
                    continue;
                }
            }

            errors.push(LintError::with_severity(
                ctx.crate_name.to_string(),
                idx + 1,
                "arvo-bits-traits-only",
                format!(
                    "arvo-bits hosts bit contracts + opaque-bit concretes only; \
                     arithmetic fielded types live in L0 arvo, masks in arvo-bitmask. \
                     If this is a new opaque-bit container, add it to \
                     ALLOWED_OPAQUE_BITS in a design round: {}",
                    trimmed
                ),
                Severity::HARD_ERROR,
            ));
        }

        errors
    }
}

/// Extract the struct name from a `pub struct` line. Returns
/// `Some("Bits")` for `pub struct Bits<const N: u8>(u64);` and
/// similar shapes.
fn extract_struct_name(line: &str) -> Option<String> {
    // Strip leading `pub struct `.
    let after = line.strip_prefix("pub struct ")?;
    // Name ends at the first `<`, `(`, `{`, whitespace, or `;`.
    let end = after
        .find(|c: char| c == '<' || c == '(' || c == '{' || c.is_whitespace() || c == ';')
        .unwrap_or(after.len());
    let name = &after[..end];
    if name.is_empty() {
        None
    } else {
        Some(name.to_string())
    }
}
