//! Lint: `arvo-bits` is bit-level primitives: contracts and
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
//! tolerated: it carries no storage.
//!
//! **Known gap**, catalogued as a red test below rather than fixed:
//! only a line starting with the literal `pub struct ` is read at
//! all, so a private `struct` or a `pub(crate) struct` carrying an
//! arithmetic body is invisible to this lint, though the module doc
//! above claims "anything else with a struct body ... fails the
//! lint" with no visibility qualifier.

use mockspace::{CrateLint, Lint, LintContext, LintError, Severity};

/// Opaque-bit concrete containers permitted in `arvo-bits`.
///
/// Types here are fielded but non-arithmetic: identity containers
/// compared by `Eq`, not ordered by arithmetic. To add a new entry,
/// confirm the type satisfies both properties, then extend this
/// list in a design round.
const ALLOWED_OPAQUE_BITS: &[&str] = &[
    "Bits", // opaque N-bit container; arvo-hash ContentHash alias sits on this
];

pub fn lint() -> Box<dyn CrateLint> {
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
}

impl CrateLint for ArvoBitsTraitsOnly {
    fn check(&self, ctx: &LintContext) -> Vec<LintError> {
        // `is_proc_macro_crate` does not consider the `lint_proc_macro_source`
        // opt-in (its own doc says so); this is the "should I skip this
        // source lint" decision, which is what that method exists for.
        if ctx.should_skip_proc_macro_source_lint() {
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

            // Strip a trailing inline comment before classifying the
            // statement. Without this, `pub struct Warm; // marker`
            // does not end with `;` once trimmed (it ends with the
            // comment text), so a genuine marker struct with any
            // trailing comment misread as a fielded, disallowed one.
            let code = match trimmed.find("//") {
                Some(pos) => trimmed[..pos].trim_end(),
                None => trimmed,
            };

            // Marker struct: ends with `;` after the name. No body.
            // Example: `pub struct Warm;`
            if code.ends_with(';') && !code.contains('(') && !code.contains('{') {
                continue;
            }

            // Fielded struct: check against the opaque-bit allowlist.
            if let Some(name) = extract_struct_name(code) {
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

#[cfg(test)]
mod tests {
    use mockspace::testkit::LintFixture;

    use super::*;

    fn hits(source: &str) -> Vec<LintError> {
        let fixture = LintFixture::new(source).with_crate_name("arvo-bits", "bits");
        ArvoBitsTraitsOnly.check(&fixture.ctx())
    }

    // --- registration and severity -----------------------------------

    #[test]
    fn it_answers_to_the_name_the_gate_and_the_config_use() {
        assert_eq!(ArvoBitsTraitsOnly.name(), "arvo-bits-traits-only");
    }

    #[test]
    fn it_is_not_declared_off_so_it_runs_at_all() {
        crate::canon_lint_testkit::assert_not_declared_off(&ArvoBitsTraitsOnly);
    }

    #[test]
    fn it_reaches_the_pack_the_engine_is_handed() {
        crate::crate_lint_testkit::assert_registered("arvo-bits-traits-only");
    }

    #[test]
    fn its_findings_carry_its_own_declared_severity() {
        let errors = hits("pub struct UFixed<const I: u8, const F: u8>(u64);\n");
        crate::crate_lint_testkit::assert_findings_carry(
            &ArvoBitsTraitsOnly,
            &errors,
            Severity::HARD_ERROR,
        );
    }

    // --- scope: only the arvo-bits crate ---------------------------------

    #[test]
    fn a_fielded_struct_outside_arvo_bits_is_silent() {
        let fixture = LintFixture::new("pub struct UFixed<const I: u8, const F: u8>(u64);\n")
            .with_crate_name("arvo-bitmask", "bitmask");
        assert!(
            ArvoBitsTraitsOnly.check(&fixture.ctx()).is_empty(),
            "Mask64 / Mask256 / BitMatrix legitimately live in arvo-bitmask, not arvo-bits"
        );
    }

    // --- what fires -------------------------------------------------------

    #[test]
    fn an_unlisted_fielded_struct_fires() {
        let errors = hits("pub struct UFixed<const I: u8, const F: u8>(u64);\n");
        assert_eq!(errors.len(), 1);
    }

    #[test]
    fn a_brace_struct_fires() {
        let errors = hits("pub struct Weird { x: u64 }\n");
        assert_eq!(errors.len(), 1);
    }

    #[test]
    fn a_multi_field_tuple_struct_fires() {
        let errors = hits("pub struct Pair(u64, u64);\n");
        assert_eq!(errors.len(), 1);
    }

    #[test]
    fn every_hit_names_its_own_line() {
        let errors = hits("pub struct A(u64);\npub struct Warm;\npub struct B(u64);\n");
        assert_eq!(errors.len(), 2);
        assert_eq!(errors[0].line, 1);
        assert_eq!(errors[1].line, 3);
    }

    /// A struct whose name merely starts with an allowed name is not
    /// the allowed type: `extract_struct_name` captures the whole
    /// identifier up to the next delimiter, so `BitsExtra` is compared
    /// against the allowlist as `"BitsExtra"`, not truncated to `"Bits"`.
    #[test]
    fn a_name_only_sharing_a_prefix_with_an_allowed_type_still_fires() {
        let errors = hits("pub struct BitsExtra(u64);\n");
        assert_eq!(errors.len(), 1);
    }

    // --- what must stay silent ------------------------------------------

    #[test]
    fn a_marker_struct_is_silent() {
        assert!(hits("pub struct Warm;\n").is_empty());
        assert!(hits("pub struct Cold;\n").is_empty());
    }

    #[test]
    fn the_allowed_opaque_container_is_silent() {
        assert!(hits("pub struct Bits<const N: u8>(u64);\n").is_empty());
    }

    #[test]
    fn a_full_line_comment_mentioning_pub_struct_is_silent() {
        assert!(hits("// pub struct Example(u64) for illustration\n").is_empty());
    }

    #[test]
    fn a_doc_comment_mentioning_pub_struct_is_silent() {
        assert!(hits("/// See `pub struct Bits` for the opaque container.\n").is_empty());
        assert!(hits("//! A `pub struct UFixed(u64)` here would fail this lint.\n").is_empty());
    }

    #[test]
    fn a_private_struct_is_silent() {
        // Only public API surface is read; the doc frames this as
        // being about what the crate ships, and a private struct is
        // not part of that surface. See the catalogued gap below for
        // the case this scoping does not obviously intend to cover.
        assert!(hits("struct Internal(u64);\n").is_empty());
    }

    /// Measured independently against a committed probe and control in
    /// the design round that closed alongside this suite: a trailing
    /// inline comment on a marker struct broke the marker detection,
    /// because the check was `trimmed.ends_with(';')` against the whole
    /// line including the comment text, and a genuine marker with any
    /// trailing comment does not end in `;` once the comment is on it.
    /// Fixed above by stripping the comment before classifying.
    #[test]
    fn a_marker_struct_with_a_trailing_comment_stays_silent() {
        let errors = hits("pub struct Warm; // strategy marker\n");
        assert!(
            errors.is_empty(),
            "a marker struct commented with its purpose is still a marker struct"
        );
    }

    #[test]
    fn the_allowed_opaque_container_with_a_trailing_comment_stays_silent() {
        // This path was never broken (the name is captured before the
        // comment is reached either way), pinned so a future change to
        // the comment-stripping logic cannot regress it silently.
        assert!(hits("pub struct Bits<const N: u8>(u64); // opaque bits\n").is_empty());
    }

    #[test]
    fn a_multi_line_generic_declaration_is_read_from_its_first_line() {
        let errors = hits("pub struct Bits<\n    const N: usize,\n>(u64);\n");
        assert!(
            errors.is_empty(),
            "Bits split across lines is still Bits, matched from its declaration line"
        );
        let errors = hits("pub struct Weird<\n    const N: usize,\n>(u64);\n");
        assert_eq!(
            errors.len(),
            1,
            "an unlisted name split across lines still fires once"
        );
    }

    // --- proc-macro exemption --------------------------------------------

    #[test]
    fn a_declared_proc_macro_crate_is_skipped() {
        let fixture = LintFixture::new("pub struct UFixed(u64);\n")
            .with_crate_name("arvo-bits", "bits")
            .with_proc_macro_crates(&["arvo-bits"]);
        assert!(ArvoBitsTraitsOnly.check(&fixture.ctx()).is_empty());
    }

    #[test]
    fn the_opt_in_override_is_honoured_rather_than_ignored() {
        let fixture = LintFixture::new("pub struct UFixed(u64);\n")
            .with_crate_name("arvo-bits", "bits")
            .with_proc_macro_crates(&["arvo-bits"])
            .with_lint_proc_macro_source(true);
        assert_eq!(
            ArvoBitsTraitsOnly.check(&fixture.ctx()).len(),
            1,
            "the opt-in override should make this crate's source lintable again"
        );
    }

    // --- catalogued gaps ---------------------------------------------------

    /// The module doc claims "anything else with a struct body in
    /// arvo-bits fails the lint", with no visibility qualifier, but the
    /// scan only reads lines starting with the literal `pub struct `.
    /// A private or crate-visible arithmetic struct is invisible to it.
    /// Catalogued rather than fixed: whether `struct` and
    /// `pub(crate) struct` should be in scope is a real policy
    /// question (arvo-bits is pre-canon and nothing states it), not a
    /// one-line correction with an undoubted answer.
    #[test]
    #[ignore = "catalogue: only a literal `pub struct ` prefix is read, \
                so `struct UFixedInternal(u64);` and `pub(crate) struct \
                ...` both escape detection though the module doc claims \
                any struct body fails the lint; whether that scoping is \
                intended is a design question, not decided here"]
    fn a_non_pub_fielded_struct_escapes_detection() {
        let errors = hits("struct UFixedInternal(u64);\n");
        assert_eq!(
            errors.len(),
            1,
            "the module doc claims any struct body in arvo-bits fails the lint"
        );
    }
}
