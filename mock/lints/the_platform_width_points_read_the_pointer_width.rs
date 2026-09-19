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
//! arms: the alias-spelling arm below, and a second arm that is an enumerated
//! list of the forms known to reach the hazard, not a closed guard on the name
//! `core` itself. A textual lint cannot close Rust name resolution, and a
//! `Cargo.toml` manifest edit is outside what a source-text lint can ever see.
//!
//! What the second arm refuses, each named separately because each is its own
//! finding when it fires:
//!
//! - `mod core` or `mod core;`, spelled plainly or as the raw identifier `mod
//!   r#core`.
//! - `use ... as core` (or `... as r#core`) closing a `use` item, including
//!   inside a braced group such as `use a::{b as core, c};`, where the alias
//!   ends the segment at a `,` or a `}` rather than at a `;`.
//! - `extern crate ... as core` (or `... as r#core`), `extern crate self as
//!   core` included, which renames the crate root itself and reaches even the
//!   leading-`::` path the alias-spelling arm relies on.
//! - `use ...::core;`, which binds the name `core` as the last, alias-free
//!   segment of a `use` path, such as `use a::b::core;`.
//!
//! Two things reach the same hazard and are not on this list. A macro that
//! expands to one of the four forms is not caught by a lint reading source
//! text, but rustc itself already refuses a macro-expanded `extern crate self
//! as core` ("cannot shadow names passed with `--extern`"), so that is not a
//! live hole. A `Cargo.toml` dependency renamed to the key `core`
//! (`core = { package = "something-else" }`) reaches `::core` exactly as
//! `extern crate self as core` does, and is unguarded here: this lint reads
//! `.rs` source only and never opens a manifest. Any other form not named
//! above, including one of the four spelled a way this enumeration does not
//! anticipate, is likewise unguarded.
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
/// arm rather than a stronger reading of the same one, and it is an enumerated
/// list of four forms rather than a closed guard on the name `core`, as the
/// module doc above states. Each form gets its own message, because a `use`
/// alias and an `extern crate` alias are refused for different reasons and a
/// reader needs to know which one fired.
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
                 unqualified `core`. Name it something else."
                    .to_string(),
            ));
        }
        for at in keyword_scoped_as_core(&code, "use") {
            let line = code[.. at].matches('\n').count() + 1;
            found.push((
                Some(path.clone()),
                line,
                "binds the name `core` through a `use ... as core` alias, which shadows the \
                 bare name `core` anywhere in this file's scope. It does not reach \
                 `::core::primitive::usize::BITS`, whose leading `::` resolves at the crate \
                 graph root rather than through whatever is in scope; it is refused because it \
                 would shadow the bare spelling `core` anywhere else in this crate. Alias it to \
                 something else."
                    .to_string(),
            ));
        }
        for at in keyword_scoped_as_core(&code, "extern crate") {
            let line = code[.. at].matches('\n').count() + 1;
            found.push((
                Some(path.clone()),
                line,
                "binds the name `core` through an `extern crate ... as core` alias, which \
                 renames the crate root itself rather than adding a competing name in scope. \
                 The platform-width aliases' leading `::` resolves through the crate root, so \
                 this alias reaches `::core::primitive::usize::BITS` too. Alias it to something \
                 else."
                    .to_string(),
            ));
        }
        for at in use_leaf_core(&code) {
            let line = code[.. at].matches('\n').count() + 1;
            found.push((
                Some(path.clone()),
                line,
                "binds the name `core` as the last, alias-free segment of a `use` path, such \
                 as `use a::b::core;`. It shadows the bare name `core` in this file's scope \
                 exactly as a `use ... as core` alias would, and does not reach \
                 `::core::primitive::usize::BITS` on its own. Rename the import or alias it to \
                 something else."
                    .to_string(),
            ));
        }
    }
    found
}

/// Every position of `as core` (or `as r#core`) that closes a `use` item or an
/// `extern crate` item opening with `keyword`, split by keyword so the two
/// reasons stay distinct: a `use` alias only shadows the bare name `core` in
/// scope, an `extern crate` alias renames the crate root and reaches even the
/// leading-`::` path.
///
/// The item's start is found by walking back to the nearest `;`, which is safe
/// because neither a `use` item nor an `extern crate` item contains one of its
/// own. `,` and `}` are included as terminators alongside `;` so an alias
/// inside a braced group, `use a::{b as core, c};`, is found at the `,` that
/// closes its own segment rather than only at the item's final `;`.
fn keyword_scoped_as_core(code: &str, keyword: &str) -> Vec<usize> {
    let mut out = Vec::new();
    for at in word_then_word(code, "as", "core", &[b';', b',', b'}']) {
        let before = &code[.. at];
        let start = before.rfind(';').map_or(0, |p| p + 1);
        if contains_word(before[start ..].trim_start(), keyword) {
            out.push(at);
        }
    }
    out
}

/// Whether `haystack` contains `word` as a whole word, word boundaries on both
/// sides.
///
/// Used rather than a prefix check because an attribute such as
/// `#[cfg(target_pointer_width = "32")]` can sit ahead of the item's own
/// keyword within the same statement.
fn contains_word(haystack: &str, word: &str) -> bool {
    let mut from = 0;
    while let Some(pos) = haystack[from ..].find(word) {
        let at = from + pos;
        let before = haystack[.. at].chars().next_back();
        let after = haystack[at + word.len() ..].chars().next();
        if !before.is_some_and(is_word_char) && !after.is_some_and(is_word_char) {
            return true;
        }
        from = at + word.len().max(1);
    }
    false
}

/// Every position where a `use` item's path ends in the bare word `core` with
/// no alias, such as `use a::b::core;`, which binds the name `core` exactly as
/// an aliased import would.
///
/// Excluded: `core` continuing as a module segment (`core::mem`, where the
/// next non-whitespace character is `::` rather than a terminator) and a
/// preceding `as`, which the sibling alias arm already reports and which this
/// scan cannot reach because `as` does not end in `::` or in the word `use`.
fn use_leaf_core(code: &str) -> Vec<usize> {
    let mut out = Vec::new();
    for (at, _) in code.match_indices("core") {
        let pre = code[.. at].trim_end();
        let boundary = pre.ends_with("::")
            || pre
                .strip_suffix("use")
                .is_some_and(|rest| !rest.chars().next_back().is_some_and(is_word_char));
        if !boundary {
            continue;
        }
        let after = code[at + "core".len() ..].chars().next();
        if after.is_some_and(is_word_char) {
            continue;
        }
        let rest = code[at + "core".len() ..].trim_start_matches(char::is_whitespace);
        if rest.starts_with("::") {
            continue;
        }
        if rest
            .as_bytes()
            .first()
            .is_some_and(|b| matches!(b, b';' | b',' | b'}'))
        {
            out.push(at);
        }
    }
    out
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
        let Some(after_after) = strip_word_allowing_raw(rest, after) else {
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

/// Strips `word` from the front of `rest`, accepting the plain spelling or the
/// raw-identifier spelling `r#word`.
///
/// `r#core` is ordinary text once `without_comments` runs on it (it opens no
/// string and no comment), so matching the plain word alone would miss `mod
/// r#core`, `... as r#core` and the rest. Only `after` is ever raw in practice
/// (nobody spells the keyword `mod` or `as` as `r#mod` or `r#as`), so only the
/// word being stripped needs this, not the caller's `before`.
fn strip_word_allowing_raw<'a>(rest: &'a str, word: &str) -> Option<&'a str> {
    rest.strip_prefix(word)
        .or_else(|| rest.strip_prefix("r#").and_then(|r| r.strip_prefix(word)))
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
