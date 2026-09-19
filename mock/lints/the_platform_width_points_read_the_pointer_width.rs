//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! Lint: the platform-width points in `arvo-format` are spelled over
//! `::core::primitive::usize::BITS`, and nothing in `arvo-format` binds the name
//! `core` to anything else.
//!
//! `USize` and `ISize` are aliases, `UFixed<{ ::core::primitive::usize::BITS },
//! 0>` and `Integer<{ ::core::primitive::usize::BITS }>`, and what makes each a
//! target-bound member of the format family rather than one fixed point is that
//! it reads the pointer width instead of naming one. `usize::BITS` is the pointer
//! width by the language's own definition, so the alias is the literal point at
//! the target's width on every target exactly when it is spelled that way. The
//! path carries a leading `::`, rooted at the crate graph rather than at
//! whatever is in scope, because a bare `usize` resolves through whatever is in
//! scope and an item named `usize` declared or imported into the same module
//! would shadow the primitive type at that path, and a bare `core` is the same
//! failure one level up: a module, an import or an `extern crate` bound to the
//! name `core` in this crate would shadow the crate root the bare path resolves
//! through.
//!
//! The leading `::` does not close every way this crate could bind the name
//! `core`, and it is not asked to alone. `mock/research/202609191400_core_shadowing_probe/`
//! shows a `mod core` and a `use ... as core` both fail against the
//! leading-`::` path, and also shows an `extern crate self as core` at the crate
//! root reaches even `::core`, because that declaration renames the crate root
//! itself rather than adding a competing name in scope. So this lint carries two
//! arms: the alias-spelling arm below, and a second arm refusing a `mod core`, a
//! `use ... as core` or an `extern crate ... as core` declared anywhere in
//! `arvo-format`, which is what closes the one path the spelling alone does not.
//!
//! No test run on one host can check the spelling. An alias written as
//! `UFixed<64, 0>` answers every question the correct one answers on a 64-bit
//! host. A free `const _` item under a `cfg` for another pointer width would be
//! evaluated by any build at that target, a `cargo check` included, but this
//! repository builds and checks at the host alone, so it would guard nothing on
//! the gate. So the property is checked where it lives, in the source text, on
//! every commit and with no second build.
//!
//! Each declaration is read from its name to its semicolon, across lines, with
//! comments and whitespace dropped and a trailing comma before a closing angle
//! bracket taken out, and what is left of the right-hand side is compared
//! exactly: `UFixed<{::core::primitive::usize::BITS},0>` for `USize`,
//! `Integer<{::core::primitive::usize::BITS}>` for `ISize`. A right-hand side
//! that only mentions `usize::BITS` without the leading-`::` `core::primitive`
//! path, in a comment, inside an expression or over the other alias's family, is
//! a finding.
//!
//! A missing alias is a finding too. Deleting one would otherwise pass as clean,
//! and a lint that reports nothing over nothing is the failure it guards.

use mockspace::{CrateLint, Lint, LintContext, LintError, Severity};

/// The lint's own name, used in its findings and keyed by `[lints.<name>]`.
const NAME: &str = "the-platform-width-points-read-the-pointer-width";

/// The crate the two aliases are declared in.
const THE_CRATE: &str = "arvo-format";

/// Each alias, by the text that opens its declaration, and the right-hand side
/// it has to read once comments and whitespace are gone.
const THE_ALIASES: &[(&str, &str)] = &[
    (
        "pub type USize",
        "UFixed<{::core::primitive::usize::BITS},0>",
    ),
    (
        "pub type ISize",
        "Integer<{::core::primitive::usize::BITS}>",
    ),
];

pub fn lint() -> Box<dyn CrateLint> {
    Box::new(ThePlatformWidthPointsReadThePointerWidth)
}

struct ThePlatformWidthPointsReadThePointerWidth;

impl Lint for ThePlatformWidthPointsReadThePointerWidth {
    /// Crate-scoped. It reads every file out of `all_sources` itself, and a
    /// missing alias is a fact about the crate rather than about one file.
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

impl CrateLint for ThePlatformWidthPointsReadThePointerWidth {
    fn check(&self, ctx: &LintContext) -> Vec<LintError> {
        if ctx.should_skip_proc_macro_source_lint() {
            return Vec::new();
        }
        if ctx.crate_name != THE_CRATE {
            return Vec::new();
        }
        let files: Vec<(String, &str)> = ctx
            .all_sources
            .iter()
            .map(|f| (f.rel_path.to_string_lossy().to_string(), f.text.as_str()))
            .collect();
        verdicts(&files)
            .into_iter()
            .chain(core_bindings(&files))
            .map(|(path, line, message)| {
                let mut error = LintError::with_severity(
                    ctx.crate_name.to_string(),
                    line,
                    NAME,
                    message,
                    Severity::HARD_ERROR,
                );
                error.path = path;
                error
            })
            .collect()
    }
}

/// Every finding over the crate's files, as a path, a line and a message.
///
/// Split from the context so a test can hand it text directly.
fn verdicts(files: &[(String, &str)]) -> Vec<(Option<String>, usize, String)> {
    let mut found = Vec::new();
    for (alias, want) in THE_ALIASES {
        let mut seen = false;
        for (path, text) in files {
            let code = without_comments(text);
            for (at, rhs) in declarations(&code, alias) {
                seen = true;
                if rhs != *want {
                    let line = code[.. at].matches('\n').count() + 1;
                    found.push((
                        Some(path.clone()),
                        line,
                        format!(
                            "`{alias}` reads `{rhs}` where it has to read `{want}`, compared with \
                             comments and whitespace dropped. A platform-width point is the literal \
                             point at the target's pointer width, and it is that on every target \
                             only when the width is read rather than written, over the family the \
                             alias names: spell it `{want}` and nothing around it."
                        ),
                    ));
                }
            }
        }
        if !seen {
            found.push((
                None,
                0,
                format!(
                    "`{alias}` is not declared anywhere in `{THE_CRATE}`. The design ships both \
                     platform-width points in `points`, and this lint has nothing to check without \
                     them; restore the alias, or change the design and this lint together."
                ),
            ));
        }
    }
    found
}

/// Whether this crate binds the name `core` to anything of its own, as a path,
/// a line and a message, over every file.
///
/// The leading `::` on the alias spelling resists a `usize` or a `core` bound in
/// scope, but not a crate reference bound to the name `core` at the crate root:
/// `extern crate self as core;` renames the crate root itself, which a
/// leading-`::` path resolves through rather than around. So this is a separate
/// arm rather than a stronger reading of the same one: a `mod core`, a `use ...
/// as core` or an `extern crate ... as core`, anywhere in the crate, is a
/// finding regardless of whether either alias is even in the same file.
fn core_bindings(files: &[(String, &str)]) -> Vec<(Option<String>, usize, String)> {
    let mut found = Vec::new();
    for (path, text) in files {
        let code = without_comments(text);
        for at in word_then_word(&code, "mod", "core", &[b'{', b';']) {
            let line = code[.. at].matches('\n').count() + 1;
            found.push((
                Some(path.clone()),
                line,
                "declares `mod core`, which shadows the crate `core` for a bare path anywhere \
                 in scope of it. The platform-width aliases read `::core::primitive::usize::BITS`, \
                 with the leading `::`, exactly so a module in this crate cannot take over an \
                 unqualified `core`; a module actually named `core` closes that gap by never \
                 needing one. Name it something else."
                    .to_string(),
            ));
        }
        for at in word_then_word(&code, "as", "core", &[b';']) {
            let line = code[.. at].matches('\n').count() + 1;
            found.push((
                Some(path.clone()),
                line,
                "binds the name `core` through a `use ... as core` or an `extern crate ... as \
                 core`. Either one takes over `::core` too: a `use` alias shadows the name in \
                 scope, and an `extern crate` alias renames the crate root itself, which the \
                 leading-`::` path on the platform-width aliases resolves through rather than \
                 around. Alias it to something else."
                    .to_string(),
            ));
        }
    }
    found
}

/// Whether `c` is a word character for the purposes of this lint's boundary
/// checks: ASCII alphanumeric or underscore.
fn is_word_char(c: char) -> bool {
    c.is_ascii_alphanumeric() || c == '_'
}

/// Every position at which the whole word `after` appears immediately following
/// the whole word `before`, with only whitespace between them, and with the
/// first non-whitespace character following `after` one of `terminators`.
///
/// `code` is expected already stripped of comments and literals, as
/// `without_comments` produces. The terminator check is what keeps `as core` from
/// firing on a cast to a path that merely starts with `core`, such as `x as
/// core::ffi::c_int`, whose next character after `core` is `:` rather than a
/// statement's `;`.
fn word_then_word(code: &str, before: &str, after: &str, terminators: &[u8]) -> Vec<usize> {
    let mut out = Vec::new();
    for (at, _) in code.match_indices(before) {
        let pre = code[.. at].chars().next_back();
        let post_before = code[at + before.len() ..].chars().next();
        if pre.is_some_and(is_word_char) || post_before.is_some_and(is_word_char) {
            continue;
        }
        let rest = code[at + before.len() ..].trim_start_matches(char::is_whitespace);
        let Some(after_after) = rest.strip_prefix(after) else {
            continue;
        };
        if after_after.chars().next().is_some_and(is_word_char) {
            continue;
        }
        let tail = after_after.trim_start_matches(char::is_whitespace);
        if tail
            .as_bytes()
            .first()
            .is_some_and(|b| terminators.contains(b))
        {
            out.push(at);
        }
    }
    out
}

/// Every declaration of `alias` in `code`, as its byte offset and its right-hand
/// side with whitespace dropped and a comma before a closing `>` taken out.
///
/// The alias text must stand as whole words, so `USizeLike` is not `USize` and
/// `xpub type USize` is not a declaration. A declaration with no `=` or no `;`
/// after it reads as an empty right-hand side, which no alias wants.
fn declarations(code: &str, alias: &str) -> Vec<(usize, String)> {
    let is_word = |c: char| c.is_ascii_alphanumeric() || c == '_';
    let mut out = Vec::new();
    for (at, _) in code.match_indices(alias) {
        let before = code[.. at].chars().next_back();
        let after = code[at + alias.len() ..].chars().next();
        if before.is_some_and(is_word) || after.is_some_and(is_word) {
            continue;
        }
        let rest = &code[at + alias.len() ..];
        let end = rest.find(';').unwrap_or(rest.len());
        let squeezed: String = rest[.. end]
            .chars()
            .filter(|c| !c.is_whitespace())
            .collect();
        let rhs = squeezed.strip_prefix('=').unwrap_or("").replace(",>", ">");
        out.push((at, rhs));
    }
    out
}

/// Whether the `r` at `i` opens a raw string rather than ending an identifier:
/// nothing word-like before it, or a `b` that is itself a word's start.
fn starts_a_literal(chars: &[char], i: usize) -> bool {
    let is_word = |c: char| c.is_ascii_alphanumeric() || c == '_';
    match i.checked_sub(1).map(|p| chars[p]) {
        None => true,
        Some('b') => i < 2 || !is_word(chars[i - 2]),
        Some(p) => !is_word(p),
    }
}

/// The text with every comment and the inside of every literal replaced by
/// spaces, newlines kept, so line numbers read the same as in the source.
///
/// Line comments, block comments nested as Rust nests them, strings, raw strings
/// included, and characters. A literal is blanked so a comment marker inside one
/// opens nothing and a declaration's text inside one is not a declaration. A
/// lifetime is told from a character by the quote that closes it.
fn without_comments(text: &str) -> String {
    let chars: Vec<char> = text.chars().collect();
    let mut out = String::with_capacity(text.len());
    let blank = |c: char| if c == '\n' { '\n' } else { ' ' };
    let mut i = 0;
    while i < chars.len() {
        let c = chars[i];
        let next = chars.get(i + 1).copied();
        if c == '/' && next == Some('/') {
            while i < chars.len() && chars[i] != '\n' {
                out.push(' ');
                i += 1;
            }
        } else if c == '/' && next == Some('*') {
            let mut depth = 0;
            while i < chars.len() {
                if chars[i] == '/' && chars.get(i + 1) == Some(&'*') {
                    depth += 1;
                    out.push_str("  ");
                    i += 2;
                } else if chars[i] == '*' && chars.get(i + 1) == Some(&'/') {
                    depth -= 1;
                    out.push_str("  ");
                    i += 2;
                    if depth == 0 {
                        break;
                    }
                } else {
                    out.push(blank(chars[i]));
                    i += 1;
                }
            }
        } else if c == 'r'
            && starts_a_literal(&chars, i)
            && (next == Some('"') || next == Some('#'))
        {
            // A raw string: `r`, some hashes, a quote, and the same hashes after
            // the closing quote. Anything else starting with `r#` is an ident.
            let hashes = chars[i + 1 ..].iter().take_while(|&&h| h == '#').count();
            if chars.get(i + 1 + hashes) != Some(&'"') {
                out.push(c);
                i += 1;
                continue;
            }
            let open = i + 2 + hashes;
            out.extend(&chars[i .. open]);
            i = open;
            while i < chars.len() {
                let closes = chars[i] == '"'
                    && chars[i + 1 ..]
                        .iter()
                        .take(hashes)
                        .filter(|&&h| h == '#')
                        .count()
                        == hashes;
                if closes {
                    out.extend(&chars[i ..= i + hashes]);
                    i += hashes + 1;
                    break;
                }
                out.push(blank(chars[i]));
                i += 1;
            }
        } else if c == '"' {
            out.push(c);
            i += 1;
            while i < chars.len() {
                let d = chars[i];
                i += 1;
                if d == '"' {
                    out.push(d);
                    break;
                }
                out.push(blank(d));
                if d == '\\' && i < chars.len() {
                    out.push(blank(chars[i]));
                    i += 1;
                }
            }
        } else if c == '\'' {
            // `'x'` or `'\n'` and its longer escapes are characters; `'a` with
            // no closing quote is a lifetime and is copied as it is. The escaped
            // character may itself be a quote, so after a backslash the search
            // for the closing one starts past it.
            let close = if next == Some('\\') {
                chars
                    .get(i + 3 ..)
                    .and_then(|rest| rest.iter().position(|&d| d == '\''))
                    .map(|p| i + 3 + p)
            } else if chars.get(i + 2) == Some(&'\'') {
                Some(i + 2)
            } else {
                None
            };
            match close {
                Some(end) => {
                    out.push('\'');
                    out.extend(chars[i + 1 .. end].iter().map(|&d| blank(d)));
                    out.push('\'');
                    i = end + 1;
                },
                None => {
                    out.push(c);
                    i += 1;
                },
            }
        } else {
            out.push(c);
            i += 1;
        }
    }
    out
}

// The generated lint crate loads this file itself through `#[path]`, at the
// crate root, so a bare `mod tests;` would look for `tests.rs` beside every
// other lint file rather than beside this one. The explicit `#[path]` here
// points it at the right place regardless.
#[cfg(test)]
#[path = "the_platform_width_points_read_the_pointer_width/tests.rs"]
mod tests;
