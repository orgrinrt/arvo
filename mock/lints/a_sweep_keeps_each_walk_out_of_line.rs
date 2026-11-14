//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! Lint: every walk `arvo-format`'s sweeps call keeps its `run` out of line.
//!
//! A sweep over formats or signatures calls a `PerFormat` or `PerSignature`
//! walk's `run` once per type, a few hundred times from one function. Each call
//! is a distinct monomorphic function called once, so LLVM inlines every one of
//! them, and the walk's counters run as one chain through every loop in one
//! body. In an optimised build scalar evolution follows that chain recursively,
//! and at 340 inlined walks it overflowed rustc's stack: ticket
//! `rustc_overflows_its_stack_in_llvm_building_arvo_format_tests`. Marking the
//! walk `#[inline(never)]` keeps each type's body its own function, so the
//! build does not depend on how many types a sweep walks. The design states it
//! in `## The laws this crate owes tests for`.
//!
//! What it reads: each test module of the crate, under a `tests` directory or a
//! `tests.rs`, with comments and literals blanked. An `impl` whose header names
//! one of the walk traits before `for` opens a block, and every `fn run` with a
//! body inside it is a finding unless its attributes hold `#[inline(never)]`.
//! The trait declarations themselves are not impls and are not read.
//!
//! What it does not read: a walk an `impl` produced by a macro declares, which
//! `arvo-format` does not have today.

use mockspace::{CrateLint, Lint, LintContext, LintError, Severity};

use crate::rust_text::{
    fn_keywords,
    has_body,
    is_test_path,
    is_word,
    item_head,
    name_after,
    without_comments,
};

/// The lint's own name, used in its findings and keyed by `[lints.<name>]`.
const NAME: &str = "a-sweep-keeps-each-walk-out-of-line";

/// The crate it reads.
const THE_CRATE: &str = "arvo-format";

/// The traits a sweep calls once per type.
const WALKS: &[&str] = &["PerFormat", "PerSignature"];

pub fn lint() -> Box<dyn CrateLint> {
    Box::new(ASweepKeepsEachWalkOutOfLine)
}

struct ASweepKeepsEachWalkOutOfLine;

impl Lint for ASweepKeepsEachWalkOutOfLine {
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

impl CrateLint for ASweepKeepsEachWalkOutOfLine {
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
            if !is_test_path(&path) {
                continue;
            }
            for (line, walk) in inlined_walks(file.text) {
                let message = format!(
                    "this `{walk}` walk's `fn run` is not `#[inline(never)]`. A sweep calls it once per \
                     type from one function, and inlined there every type's body joins one function \
                     whose counters chain through every loop, which overflowed rustc's stack in an \
                     optimised build. Mark it `#[inline(never)]`."
                );
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

/// Every `fn run` in a walk impl in `text` without `#[inline(never)]`, as the
/// line its item starts on and the walk trait the impl names.
///
/// Split from the context so a test can hand it text directly.
fn inlined_walks(text: &str) -> Vec<(usize, &'static str)> {
    let code = without_comments(text);
    let chars: Vec<char> = code.chars().collect();
    let mut found = Vec::new();
    for (walk, open, close) in walk_impls(&chars) {
        for at in fn_keywords(&chars) {
            if at <= open || at >= close {
                continue;
            }
            if name_after(&chars, at + 2).as_deref() != Some("run") || !has_body(&chars, at + 2) {
                continue;
            }
            let (start, attributes) = item_head(&chars, at);
            let never = attributes
                .iter()
                .map(|a| a.chars().filter(|c| !c.is_whitespace()).collect::<String>())
                .any(|a| a == "#[inline(never)]");
            if !never {
                let line = chars[.. start].iter().filter(|&&c| c == '\n').count() + 1;
                found.push((line, walk));
            }
        }
    }
    found
}

/// Each `impl` block whose header names a walk trait before `for`: the trait,
/// and the offsets of the block's opening and closing braces.
fn walk_impls(chars: &[char]) -> Vec<(&'static str, usize, usize)> {
    let mut out = Vec::new();
    let mut i = 0;
    while i + 4 < chars.len() {
        let is_impl = chars[i .. i + 4] == ['i', 'm', 'p', 'l']
            && (i == 0 || !is_word(chars[i - 1]))
            && !is_word(chars[i + 4]);
        if !is_impl {
            i += 1;
            continue;
        }
        let Some(open) = chars[i ..]
            .iter()
            .position(|&c| c == '{' || c == ';')
            .map(|p| i + p)
        else {
            break;
        };
        if chars[open] == ';' {
            i = open + 1;
            continue;
        }
        let header: String = chars[i .. open].iter().collect();
        let walk = WALKS.iter().find(|w| names_before_for(&header, w));
        let close = matching_brace(chars, open);
        if let Some(walk) = walk {
            out.push((*walk, open, close));
        }
        i = open + 1;
    }
    out
}

/// Whether `header` names `walk` as a whole word directly before `for`.
fn names_before_for(header: &str, walk: &str) -> bool {
    let words: Vec<&str> = header
        .split(|c: char| !is_word(c))
        .filter(|w| !w.is_empty())
        .collect();
    words
        .windows(2)
        .any(|pair| pair[0] == walk && pair[1] == "for")
}

/// The offset of the brace closing the one at `open`, or the end of the text.
fn matching_brace(chars: &[char], open: usize) -> usize {
    let mut depth = 0i32;
    for (k, &c) in chars.iter().enumerate().skip(open) {
        match c {
            '{' => depth += 1,
            '}' => {
                depth -= 1;
                if depth == 0 {
                    return k;
                }
            },
            _ => {},
        }
    }
    chars.len()
}

// The generated lint crate loads this file itself through `#[path]`, at the
// crate root, so the module below names its own path.
#[cfg(test)]
#[path = "a_sweep_keeps_each_walk_out_of_line/tests.rs"]
mod tests;
