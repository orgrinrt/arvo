//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! Lint: no const in `arvo-format` carries a host primitive as its type.
//!
//! `mockspace.toml` names `arvo-format` under `[primitive-introductions]`, so the
//! bare-primitive pack skips the whole crate. That door exists for one thing: the
//! crate defining what a number is in this stack cannot express itself in types it
//! has not defined yet, and `Width::bits` has to take a count of bits from
//! somewhere. It is not a licence for the rest of the crate.
//!
//! **The door was being used for the contract.** Ten associated consts on this
//! crate's public traits were declared with host integers and a host `bool`, and
//! every one of them is a position an implementor outside this crate has to write
//! too. Outside, the pack lints run, so the contract could not be implemented
//! anywhere but here without turning a lint off.
//! `proposal::the_concept_is_closed_and_the_inventory_is_open` says a new instance
//! earns admission by supplying the concept's obligations, and that closing the
//! concept while opening the inventory makes admission a check rather than a
//! negotiation. A contract only the exempt crate can satisfy had made it a
//! negotiation with the lint config.
//!
//! So this is the exemption's hole, closed. It reads the one crate the pack skips
//! and refuses there what the pack refuses everywhere else, for consts. It says
//! nothing about a const generic parameter, which
//! `obligation::a_primitive_for_every_position_a_bare_number_would_take` excepts
//! in op's own words and which `question::the_width_surface_crossing` holds open.
//!
//! **Consts and not every position**, deliberately. A parameter or a return in
//! this crate is reachable by a caller who writes no type name, so it does not
//! block an outside implementor, and the pack already states the general rule for
//! every other crate. What only this lint can see is the crate the pack does not
//! read at all, and the contract lives in its consts.

use mockspace::{CrateLint, Lint, LintContext, LintError, Severity};

/// The crate the bare-primitive pack skips, which is the one this reads.
///
/// One name rather than a lookup into `[primitive-introductions]`, because a
/// second exempt crate is a design decision that would come with its own round,
/// and a lint that silently widened with the config would be one nobody noticed
/// widening.
const THE_EXEMPT_CRATE: &str = "arvo-format";

/// The host's own numeric and truth types, by the name they are written under.
///
/// `u128` and `i128` are here as well as the narrower ones. They are the carrier
/// the applied map's private arithmetic runs in, which is why the check below is
/// about consts rather than about every occurrence: that intermediate is a local
/// inside two private functions and is not a const anywhere.
const HOST_PRIMITIVES: &[&str] = &[
    "u8", "u16", "u32", "u64", "u128", "usize", "i8", "i16", "i32", "i64", "i128", "isize", "f32",
    "f64", "bool", "char",
];

pub fn lint() -> Box<dyn CrateLint> {
    Box::new(AContractCoordinateIsNotAHostPrimitive)
}

struct AContractCoordinateIsNotAHostPrimitive;

impl Lint for AContractCoordinateIsNotAHostPrimitive {
    fn name(&self) -> &'static str {
        "a-contract-coordinate-is-not-a-host-primitive"
    }

    fn default_severity(&self) -> Severity {
        Severity::HARD_ERROR
    }
}

impl CrateLint for AContractCoordinateIsNotAHostPrimitive {
    fn check(&self, ctx: &LintContext) -> Vec<LintError> {
        if ctx.should_skip_proc_macro_source_lint() {
            return Vec::new();
        }

        if ctx.crate_name != THE_EXEMPT_CRATE {
            return Vec::new();
        }

        // The dispatcher hands a source lint the same context once per module
        // file, with `source` swapped each time, and this one reads every file
        // out of `all_sources` so a finding can name the file it is in. Running
        // the sweep on each of those calls would report every violation as many
        // times as the crate has modules, so it runs on the pass where `source`
        // is the crate root and returns nothing on the others.
        let Some(root) = ctx.all_sources.first() else {
            return Vec::new();
        };
        if ctx.source != root.text {
            return Vec::new();
        }

        let mut errors = Vec::new();

        // Every file, test modules included and with no exemption for them. A
        // suite is where a coordinate quietly goes back to being a host integer,
        // because a window bound written as one reads as convenient rather than
        // as a claim. The constructions that must stay wrong are wrong about the
        // range they declare rather than about the type they declare it in, so
        // none of them needs the host's own to say what it says.
        for file in ctx.all_sources {
            let path = file.rel_path.to_string_lossy().to_string();
            for (idx, line) in file.text.lines().enumerate() {
                let Some(declared) = const_type_of(line) else {
                    continue;
                };
                let Some(found) = host_primitive_in(&declared) else {
                    continue;
                };
                let carries = if declared == found {
                    format!("declared as the host's own `{found}`")
                } else {
                    format!("declared as `{declared}`, which carries the host's `{found}`")
                };
                let mut error = LintError::with_severity(
                    ctx.crate_name.to_string(),
                    idx + 1,
                    "a-contract-coordinate-is-not-a-host-primitive",
                    format!(
                        "const {carries}. This crate is the one the bare-primitive pack skips, and \
                         the skip is for defining the stack's own types rather than for spelling a \
                         contract in the host's. A const here is a position an outside implementor \
                         writes too, and the pack refuses it there. Give the coordinate a type this \
                         crate owns, beside the contract that reads it."
                    ),
                    Severity::HARD_ERROR,
                );
                error.path = Some(path.clone());
                errors.push(error);
            }
        }

        errors
    }
}

/// The declared type of a const item on this line, if the line declares one.
///
/// Matches both shapes a const takes: a trait item ending in `;` with no value,
/// and an item with an initialiser. Returns the text between the colon and
/// whichever of `=` or `;` ends the type.
///
/// Returns `None` for a `const fn`, for a line whose name is not the screaming
/// case a const item carries, and for a comment.
fn const_type_of(line: &str) -> Option<String> {
    let trimmed = line.trim();
    if trimmed.starts_with("//") {
        return None;
    }
    // Strip a trailing comment before parsing, so a `// u32` note in the margin
    // is not read as a declaration and a `; // ...` tail does not swallow the
    // terminator the type ends at.
    let code = match trimmed.find("//") {
        Some(pos) => trimmed[..pos].trim_end(),
        None => trimmed,
    };

    let after = code
        .strip_prefix("const ")
        .or_else(|| code.strip_prefix("pub const "))
        .or_else(|| code.strip_prefix("pub(crate) const "))
        .or_else(|| code.strip_prefix("pub(super) const "))?;

    let colon = after.find(':')?;
    let name = after[..colon].trim();
    // A const item's name, and not `const fn`, `const _`, or a generic
    // parameter's name that happened to reach this far. Screaming case is what
    // every const in this crate is written in, and a lint that accepted anything
    // would start reading `const fn` lines as declarations of a type called `fn`.
    if name.is_empty() || !name.chars().all(|c| c.is_ascii_uppercase() || c.is_ascii_digit() || c == '_') {
        return None;
    }

    let rest = &after[colon + 1..];
    let end = rest
        .find('=')
        .into_iter()
        .chain(rest.find(';'))
        .min()
        .unwrap_or(rest.len());
    let declared = rest[..end].trim();
    if declared.is_empty() {
        None
    } else {
        Some(declared.to_string())
    }
}

/// The first host primitive named anywhere in a declared type, if any.
///
/// Matched on word boundaries, so `[u32; 3]`, `&[u32]` and `Option<i64>` all
/// count and `Width`, `Slot` and `MagnitudeCount` do not. An array or a slice of
/// the host's own is the same position wearing a container, and the crate's
/// `ADMITTED_WIDTHS` was exactly that shape before this round.
fn host_primitive_in(declared: &str) -> Option<&'static str> {
    let bytes = declared.as_bytes();
    for candidate in HOST_PRIMITIVES {
        let mut from = 0;
        while let Some(rel) = declared[from..].find(candidate) {
            let start = from + rel;
            let end = start + candidate.len();
            let before_ok = start == 0 || !is_ident_byte(bytes[start - 1]);
            let after_ok = end == bytes.len() || !is_ident_byte(bytes[end]);
            if before_ok && after_ok {
                return Some(candidate);
            }
            from = end;
        }
    }
    None
}

fn is_ident_byte(b: u8) -> bool {
    b.is_ascii_alphanumeric() || b == b'_'
}

#[cfg(test)]
mod tests {
    use mockspace::testkit::LintFixture;

    use super::*;

    /// A fixture whose crate root is the source under test.
    ///
    /// The lint reads `all_sources` and gates on the root pass, so a fixture that
    /// set only `source` would exercise the gate and never the sweep.
    fn hits(source: &str) -> Vec<LintError> {
        let fixture = LintFixture::new(source).with_crate_name("arvo-format", "format");
        AContractCoordinateIsNotAHostPrimitive.check(&fixture.ctx())
    }

    // --- registration and severity ---------------------------------------

    #[test]
    fn it_answers_to_the_name_the_gate_and_the_config_use() {
        assert_eq!(
            AContractCoordinateIsNotAHostPrimitive.name(),
            "a-contract-coordinate-is-not-a-host-primitive"
        );
    }

    #[test]
    fn it_is_not_declared_off_so_it_runs_at_all() {
        crate::canon_lint_testkit::assert_not_declared_off(&AContractCoordinateIsNotAHostPrimitive);
    }

    #[test]
    fn it_reaches_the_pack_the_engine_is_handed() {
        crate::crate_lint_testkit::assert_registered("a-contract-coordinate-is-not-a-host-primitive");
    }

    #[test]
    fn its_findings_carry_its_own_declared_severity() {
        let errors = hits("    const PHASE_NUM: i64;\n");
        crate::crate_lint_testkit::assert_findings_carry(
            &AContractCoordinateIsNotAHostPrimitive,
            &errors,
            Severity::HARD_ERROR,
        );
    }

    // --- the negative control, before anything it reports counts ----------

    #[test]
    fn the_control_a_clean_declaration_is_silent() {
        // If this fired the lint would be reporting on every const in the crate
        // and the assertions below would pass for the wrong reason.
        assert!(
            hits("    const WIDTH: Width;\n").is_empty(),
            "a coordinate already carrying a type this crate owns was reported"
        );
    }

    #[test]
    fn the_control_the_shipped_shapes_are_all_silent() {
        let clean = "\
pub const ZERO: Self = Self(0);
const MIN: Slot;
const MAX: Slot;
const WIDTH: Width;
const PHASE: Phase;
const RADIX: Radix;
const SIGNED: Bool;
const BASE: Exponent;
const SLOPE: Exponent;
const MAGNITUDES: MagnitudeCount;
const ARITY: Arity;
const MODE: Mode;
const POLICY: Policy;
const ADMITTED: () = {};
pub const ALL_MODES: [Mode; 6] = [];
pub const SHIPPED_POLICIES: [Policy; 3] = [];
pub const ADMITTED_WIDTHS: &[Width] = &[];
";
        assert!(
            hits(clean).is_empty(),
            "the shape this round lands was reported, so the lint refuses what it is supposed to \
             admit"
        );
    }

    // --- every coordinate the round moved, as the shape it had ------------

    #[test]
    fn each_of_the_ten_coordinates_in_its_old_spelling_fires() {
        // The contract exactly as it stood before this round, one line each. Kept
        // permanently rather than described, because a lint that reports nine of
        // ten looks identical to one that reports all ten from a summary.
        let old = "\
    const RADIX: u32;
    const SIGNED: bool;
    const BASE: i32;
    const SLOPE: i32;
    const MAGNITUDES: u32;
    const MIN: i64;
    const MAX: i64;
    const PHASE_NUM: i64;
    const PHASE_DEN: i64;
    const ARITY: u32;
";
        let errors = hits(old);
        assert_eq!(
            errors.len(),
            10,
            "the contract carried ten coordinates on host types and the lint found {}",
            errors.len()
        );
    }

    #[test]
    fn an_initialiser_fires_as_well_as_a_declaration() {
        // An impl-side const is the same position: it names the type, and an
        // outside crate writing it is refused by the pack for that line.
        let errors = hits("    const RADIX: u32 = 2;\n");
        assert_eq!(errors.len(), 1);
    }

    #[test]
    fn a_slice_of_the_host_fires() {
        // What `ADMITTED_WIDTHS` was. A container around the host's own type is
        // the same position wearing a wrapper.
        let errors = hits("pub const ADMITTED_WIDTHS: &[u32] = &[];\n");
        assert_eq!(errors.len(), 1);
    }

    #[test]
    fn an_array_of_the_host_fires() {
        let errors = hits("pub const SIZES: [u64; 4] = [0; 4];\n");
        assert_eq!(errors.len(), 1);
    }

    #[test]
    fn a_generic_over_the_host_fires() {
        let errors = hits("const LIMIT: Option<i64> = None;\n");
        assert_eq!(errors.len(), 1);
    }

    // --- what must not fire, which is where a word-boundary check earns it --

    #[test]
    fn a_type_whose_name_merely_contains_a_primitive_is_silent() {
        // `Bool` contains no primitive token, but a name like `Boolish` or
        // `MyU32Thing` would match a substring search and must not match this one.
        let errors = hits(
            "const A: Boolish;\nconst B: MyU32Thing;\nconst C: Nu32;\nconst D: u32x;\n",
        );
        assert!(
            errors.is_empty(),
            "a substring match fired where a word-boundary match must not: {errors:?}"
        );
    }

    #[test]
    fn a_const_fn_is_not_a_declaration() {
        let errors = hits("    pub const fn bits(n: u32) -> Self {\n");
        assert!(
            errors.is_empty(),
            "a `const fn` taking a host parameter was read as a const item"
        );
    }

    #[test]
    fn a_const_generic_parameter_is_silent() {
        // Op excepts this position by name and an open question holds the width
        // crossing. A lint firing here would be answering it.
        let errors = hits("pub struct Signed<const BITS: u32>;\n");
        assert!(
            errors.is_empty(),
            "a const generic parameter was reported, which is the excepted position"
        );
    }

    #[test]
    fn a_comment_naming_a_primitive_is_silent() {
        let errors = hits("// const MIN: i64; the shape before the round\n");
        assert!(errors.is_empty());
    }

    #[test]
    fn a_trailing_comment_does_not_make_a_clean_line_fire() {
        let errors = hits("    const WIDTH: Width; // was u32\n");
        assert!(
            errors.is_empty(),
            "a note in the margin was read as the declared type"
        );
    }

    #[test]
    fn a_trailing_comment_does_not_hide_a_dirty_line() {
        let errors = hits("    const MIN: i64; // the lowest admitted index\n");
        assert_eq!(errors.len(), 1);
    }

    // --- scope -----------------------------------------------------------

    #[test]
    fn a_host_typed_const_outside_the_exempt_crate_is_silent_here() {
        // Not because it is allowed. The pack's own lints read every other crate
        // and refuse it there, and two lints reporting one line would make the
        // count in any sweep of this class wrong.
        let fixture =
            LintFixture::new("    const MIN: i64;\n").with_crate_name("arvo-placement", "placement");
        assert!(
            AContractCoordinateIsNotAHostPrimitive
                .check(&fixture.ctx())
                .is_empty()
        );
    }

    #[test]
    fn the_finding_names_the_file_it_is_in() {
        let errors = hits("    const MIN: i64;\n");
        assert_eq!(errors.len(), 1);
        assert!(
            errors[0].path.is_some(),
            "a finding with no path reads as a location that points at nothing"
        );
    }
}
