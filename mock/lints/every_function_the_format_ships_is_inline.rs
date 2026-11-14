//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! Lint: every function `arvo-format` ships carries `#[inline]` or
//! `#[inline(always)]`.
//!
//! A function that is not generic is compiled once, in the crate that declares
//! it, and a consumer built without link-time optimisation calls it out of line
//! because its body is not offered across the crate boundary. That is a call
//! the consumer's backend cannot see through, so the branch it guards is not
//! cleared at lowering, which
//! `ruling::never_a_runtime_check_and_one_lowered_path` asks for. The first
//! release disassembly of a consumer's sum carried 24 such calls to one-line
//! accessors of this crate. A crate-private helper is the same case, since a
//! generic public function calling it is instantiated in the consumer. A generic
//! function is offered already and the attribute costs it nothing, so the rule
//! is every function with a body, with nothing to argue over.
//!
//! What it reads: each source file of the crate outside a `tests` directory and
//! a `tests.rs`, with comments and literals blanked. Each `fn` keyword opens a
//! function; the attributes written directly above its qualifiers are its
//! attributes. A function whose attributes hold `cfg(test)` ships nowhere and is
//! skipped, and so is one ending at `;` with no body, a required trait method. A
//! function with neither `#[inline]` nor `#[inline(always)]`, or with
//! `#[inline(never)]`, is a finding at the line its item starts on.
//!
//! What it does not read: an item a macro expands to, and a function nested in
//! another function's body, which the outer one carries. `arvo-format` has
//! neither today.
//!
//! The other four crates of the workspace carry the same gap and are other
//! lanes' work; ticket `every_function_the_other_four_arvo_crates_ship_is_inline`
//! carries widening this lint to them.

use mockspace::{CrateLint, Lint, LintContext, LintError, Severity};

use crate::rust_text::{
    fn_keywords,
    has_body,
    is_test_path,
    item_head,
    name_after,
    without_comments,
};

/// The lint's own name, used in its findings and keyed by `[lints.<name>]`.
const NAME: &str = "every-function-the-format-ships-is-inline";

/// The crate it reads.
const THE_CRATE: &str = "arvo-format";

pub fn lint() -> Box<dyn CrateLint> {
    Box::new(EveryFunctionTheFormatShipsIsInline)
}

struct EveryFunctionTheFormatShipsIsInline;

impl Lint for EveryFunctionTheFormatShipsIsInline {
    /// Crate-scoped. It reads every file out of `all_sources` itself, so a
    /// finding names the file it is in.
    fn per_file(&self) -> bool {
        false
    }

    fn name(&self) -> &'static str {
        NAME
    }

    fn default_severity(&self) -> Severity {
        Severity::HARD_ERROR
    }
}

impl CrateLint for EveryFunctionTheFormatShipsIsInline {
    fn check(&self, ctx: &LintContext) -> Vec<LintError> {
        if ctx.should_skip_proc_macro_source_lint() {
            return Vec::new();
        }
        if ctx.crate_name != THE_CRATE {
            return Vec::new();
        }
        let mut errors = Vec::new();
        for file in ctx.all_sources() {
            let path = file.rel_path.to_string_lossy().to_string();
            if is_test_path(&path) {
                continue;
            }
            for (line, name, never) in uninlined(file.text) {
                let message = if never {
                    format!(
                        "`fn {name}` is marked `#[inline(never)]`. A function this crate ships is \
                         offered to the consumer's backend, so a call into it can be cleared at \
                         lowering there: mark it `#[inline]`, or move it under `#[cfg(test)]` if it \
                         ships nowhere."
                    )
                } else {
                    format!(
                        "`fn {name}` carries no `#[inline]`. Without it a consumer built without \
                         link-time optimisation calls it out of line, and its backend cannot clear \
                         the path behind the call. Mark it `#[inline]` as the last attribute above \
                         the item, or move it under `#[cfg(test)]` if it ships nowhere."
                    )
                };
                let mut error = LintError::with_severity(
                    ctx.crate_name.to_string(),
                    line,
                    NAME,
                    message,
                    Severity::HARD_ERROR,
                );
                error.path = Some(path.clone());
                errors.push(error);
            }
        }
        errors
    }
}
/// Every shipped function in `text` with no inline mark, as the line its item
/// starts on, its name, and whether it is marked `#[inline(never)]`.
///
/// Split from the context so a test can hand it text directly.
fn uninlined(text: &str) -> Vec<(usize, String, bool)> {
    let code = without_comments(text);
    let chars: Vec<char> = code.chars().collect();
    let mut found = Vec::new();
    for at in fn_keywords(&chars) {
        let Some(name) = name_after(&chars, at + 2) else {
            continue;
        };
        if !has_body(&chars, at + 2) {
            continue;
        }
        let (start, attributes) = item_head(&chars, at);
        let squeezed: Vec<String> = attributes
            .iter()
            .map(|a| a.chars().filter(|c| !c.is_whitespace()).collect())
            .collect();
        if squeezed.iter().any(|a| a.contains("cfg(test)")) {
            continue;
        }
        let never = squeezed.iter().any(|a| a == "#[inline(never)]");
        let inline = squeezed
            .iter()
            .any(|a| a == "#[inline]" || a == "#[inline(always)]");
        if inline && !never {
            continue;
        }
        let line = chars[.. start].iter().filter(|&&c| c == '\n').count() + 1;
        found.push((line, name, never));
    }
    found
}

// The generated lint crate loads this file itself through `#[path]`, at the
// crate root, so the module below names its own path.
#[cfg(test)]
#[path = "every_function_the_format_ships_is_inline/tests.rs"]
mod tests;
