//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! Lint: the platform-width points in `arvo-format` are spelled over
//! `core::primitive::usize::BITS`.
//!
//! `USize` and `ISize` are aliases, `UFixed<{ core::primitive::usize::BITS }, 0>`
//! and `Integer<{ core::primitive::usize::BITS }>`, and what makes each a
//! target-bound member of the format family rather than one fixed point is that
//! it reads the pointer width instead of naming one. `usize::BITS` is the pointer
//! width by the language's own definition, so the alias is the literal point at
//! the target's width on every target exactly when it is spelled that way. The
//! path is absolute and rooted at `core` rather than the bare name, because a
//! bare `usize` resolves through whatever is in scope and an item named `usize`
//! declared or imported into the same module would shadow the primitive type at
//! that path; `core::primitive` is nobody's to declare over.
//!
//! No test run on one host can check that. An alias written as
//! `UFixed<64, 0>` answers every question the correct one answers on a 64-bit
//! host. A const assertion at another pointer width would be evaluated by any
//! build at that target, a `cargo check` included, but this repository builds
//! and checks at the host alone, so it would guard nothing on the gate. So the
//! property is checked where it lives, in the source text, on every commit and
//! with no second build.
//!
//! Each declaration is read from its name to its semicolon, across lines, with
//! comments and whitespace dropped and a trailing comma before a closing angle
//! bracket taken out, and what is left of the right-hand side is compared
//! exactly: `UFixed<{core::primitive::usize::BITS},0>` for `USize`,
//! `Integer<{core::primitive::usize::BITS}>` for `ISize`. A right-hand side that
//! only mentions `usize::BITS` without the `core::primitive` path, in a comment,
//! inside an expression or over the other alias's family, is a finding.
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
    ("pub type USize", "UFixed<{core::primitive::usize::BITS},0>"),
    ("pub type ISize", "Integer<{core::primitive::usize::BITS}>"),
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

#[cfg(test)]
mod tests {
    use mockspace::testkit::LintFixture;

    use super::*;

    /// The shipped spelling, as `points` carries it.
    const SHIPPED: &str = "\
    pub type USize = UFixed<{ core::primitive::usize::BITS }, 0>;
    pub type ISize = Integer<{ core::primitive::usize::BITS }>;
";

    fn hits(source: &str) -> Vec<LintError> {
        let fixture = LintFixture::new(source).with_crate_name("arvo-format", "format");
        ThePlatformWidthPointsReadThePointerWidth.check(&fixture.ctx())
    }

    #[test]
    fn it_answers_to_the_name_the_gate_and_the_config_use() {
        assert_eq!(ThePlatformWidthPointsReadThePointerWidth.name(), NAME);
        assert_eq!(NAME, "the-platform-width-points-read-the-pointer-width");
    }

    #[test]
    fn it_is_not_declared_off_so_it_runs_at_all() {
        crate::canon_lint_testkit::assert_not_declared_off(
            &ThePlatformWidthPointsReadThePointerWidth,
        );
    }

    #[test]
    fn it_reaches_the_pack_the_engine_is_handed() {
        crate::crate_lint_testkit::assert_registered(NAME);
    }

    #[test]
    fn its_findings_carry_its_own_declared_severity() {
        let errors = hits(
            "pub type USize = UFixed<64, 0>;\npub type ISize = Integer<{ \
             core::primitive::usize::BITS }>;\n",
        );
        crate::crate_lint_testkit::assert_findings_carry(
            &ThePlatformWidthPointsReadThePointerWidth,
            &errors,
            Severity::HARD_ERROR,
        );
    }

    #[test]
    fn the_control_the_shipped_spelling_is_silent() {
        assert!(hits(SHIPPED).is_empty(), "{:?}", hits(SHIPPED));
    }

    #[test]
    fn a_literal_width_fires_for_each_alias() {
        for (source, alias) in [
            (
                "pub type USize = UFixed<64, 0>;\npub type ISize = Integer<{ \
                 core::primitive::usize::BITS }>;\n",
                "USize",
            ),
            (
                "pub type USize = UFixed<{ core::primitive::usize::BITS }, 0>;\npub type ISize = \
                 Integer<32>;\n",
                "ISize",
            ),
        ] {
            let errors = hits(source);
            assert_eq!(errors.len(), 1, "{alias}: {errors:?}");
            assert!(
                errors[0].message.contains(alias),
                "{alias}: {}",
                errors[0].message
            );
        }
        assert_eq!(
            hits("pub type USize = UFixed<16, 0>;\npub type ISize = Integer<16>;\n").len(),
            2
        );
    }

    #[test]
    fn a_width_read_from_somewhere_else_fires() {
        // `u64::BITS` reads a width and is still a fixed one.
        let errors = hits(
            "pub type USize = UFixed<{ u64::BITS }, 0>;\npub type ISize = Integer<{ \
             core::primitive::usize::BITS }>;\n",
        );
        assert_eq!(errors.len(), 1);
    }

    #[test]
    fn a_missing_alias_fires() {
        let errors = hits("pub type USize = UFixed<{ core::primitive::usize::BITS }, 0>;\n");
        assert_eq!(errors.len(), 1);
        assert!(errors[0].message.contains("ISize"));
        assert_eq!(hits("").len(), 2);
    }

    #[test]
    fn a_commented_out_alias_is_not_the_alias() {
        let errors = hits(
            "// pub type USize = UFixed<{ usize::BITS }, 0>;\npub type ISize = Integer<{ \
             core::primitive::usize::BITS }>;\n",
        );
        assert_eq!(errors.len(), 1);
        assert!(errors[0].message.contains("not declared"));
    }

    #[test]
    fn a_longer_name_is_not_the_alias() {
        let errors = hits(
            "pub type USizeLike = UFixed<8, 0>;\npub type ISize = Integer<{ \
             core::primitive::usize::BITS }>;\n",
        );
        assert_eq!(errors.len(), 1);
        assert!(errors[0].message.contains("not declared"));
    }

    /// The findings for a source whose `ISize` is the shipped one and whose
    /// `USize` is `usize_decl`, or the other way round when `on_isize`.
    fn one_bad(decl: &str, on_isize: bool) -> Vec<LintError> {
        if on_isize {
            hits(&format!(
                "pub type USize = UFixed<{{ core::primitive::usize::BITS }}, 0>;\n{decl}\n"
            ))
        } else {
            hits(&format!(
                "{decl}\npub type ISize = Integer<{{ core::primitive::usize::BITS }}>;\n"
            ))
        }
    }

    #[test]
    fn a_literal_width_beside_a_comment_naming_the_pointer_width_fires() {
        for (decl, on_isize) in [
            ("pub type USize = UFixed<64, 0>; // usize::BITS", false),
            ("pub type USize = UFixed<64, 0>; /* usize::BITS */", false),
            ("pub type ISize = Integer</* usize::BITS */ 64>;", true),
            ("pub type ISize = Integer<32>; // was { usize::BITS }", true),
        ] {
            assert_eq!(one_bad(decl, on_isize).len(), 1, "{decl}");
        }
    }

    #[test]
    fn a_width_that_only_mentions_the_pointer_width_fires() {
        for (decl, on_isize) in [
            (
                "pub type USize = UFixed<{ if usize::BITS == 64 { 64 } else { 32 } }, 0>;",
                false,
            ),
            ("pub type USize = UFixed<{ usize::BITS * 2 }, 0>;", false),
            ("pub type USize = UFixed<{ usize::BITS }, 1>;", false),
            ("pub type ISize = Integer<{ usize::BITS - 1 }>;", true),
            (
                "pub type ISize = Integer<{ core::cmp::min(usize::BITS, 32) }>;",
                true,
            ),
        ] {
            assert_eq!(one_bad(decl, on_isize).len(), 1, "{decl}");
        }
    }

    #[test]
    fn an_alias_spelled_with_the_other_point_fires() {
        // Each alias reads the pointer width and names the wrong point: `USize`
        // signed, `ISize` unsigned.
        assert_eq!(
            one_bad("pub type USize = Integer<{ usize::BITS }>;", false).len(),
            1
        );
        assert_eq!(
            one_bad("pub type ISize = UFixed<{ usize::BITS }, 0>;", true).len(),
            1
        );
        assert_eq!(
            hits(
                "pub type USize = Integer<{ usize::BITS }>;\npub type ISize = UFixed<{ usize::BITS }, \
                 0>;\n"
            )
            .len(),
            2
        );
    }

    #[test]
    fn a_correct_alias_across_lines_and_around_comments_is_silent() {
        for source in [
            "pub type USize =\n    UFixed<{ core::primitive::usize::BITS }, 0>;\npub type ISize \
             =\n    Integer<{ core::primitive::usize::BITS }>;\n",
            "pub type USize = UFixed<\n    { core::primitive::usize::BITS },\n    0,\n>;\npub type \
             ISize = Integer<\n    { core::primitive::usize::BITS },\n>;\n",
            "pub type USize = UFixed<{ core::primitive::usize::BITS }, 0>; // the pointer \
             width\npub type ISize = Integer<{core::primitive::usize::BITS}>;\n",
            "pub type USize = UFixed<{ core::primitive::usize::BITS } /* read */, 0>;\npub type \
             ISize = Integer<\n    // the pointer width\n    { core::primitive::usize::BITS \
             }\n>;\n",
        ] {
            assert!(hits(source).is_empty(), "{source}: {:?}", hits(source));
        }
    }

    #[test]
    fn a_wrong_alias_across_lines_is_found_at_its_opening_line() {
        let errors = hits(
            "pub type USize = UFixed<{ core::primitive::usize::BITS }, 0>;\n\npub type ISize \
             =\n    Integer<64>;\n",
        );
        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].line, 3);
    }

    #[test]
    fn a_declaration_inside_a_literal_is_not_a_declaration() {
        for source in [
            "const S: &str = \"pub type USize = UFixed<64, 0>;\";\n",
            "const S: &str = r#\"pub type USize = UFixed<64, 0>;\"#;\n",
            "const S: &[u8] = br\"pub type USize = UFixed<64, 0>;\";\n",
        ] {
            let errors = hits(&format!(
                "{source}pub type ISize = Integer<{{ core::primitive::usize::BITS }}>;\n"
            ));
            assert_eq!(errors.len(), 1, "{source}: {errors:?}");
            assert!(errors[0].message.contains("not declared"), "{source}");
        }
    }

    #[test]
    fn a_comment_marker_inside_a_literal_opens_nothing() {
        // Were the marker read, the declaration after it would vanish and the
        // alias would be reported missing; were the quote character read as a
        // string, the same.
        for opener in [
            "const S: &str = \"/*\";",
            "const S: &str = \"// \\\" /*\";",
            "const C: char = '\"';",
            "const C: char = '\\'';",
            "fn f<'a>(x: &'a str) -> &'a str { x }",
        ] {
            let source = format!(
                "{opener}\npub type USize = UFixed<{{ core::primitive::usize::BITS }}, 0>;\npub \
                 type ISize = Integer<{{ core::primitive::usize::BITS }}>; /* */\n"
            );
            assert!(hits(&source).is_empty(), "{opener}: {:?}", hits(&source));
            let wrong = source.replace(
                "UFixed<{ core::primitive::usize::BITS }, 0>",
                "UFixed<64, 0>",
            );
            assert_eq!(hits(&wrong).len(), 1, "{opener}");
        }
    }

    #[test]
    fn a_nested_block_comment_is_dropped_whole() {
        let errors = hits(
            "/* outer /* inner */ pub type USize = UFixed<64, 0>; */\npub type USize = \
             UFixed<{ core::primitive::usize::BITS }, 0>;\npub type ISize = Integer<{ \
             core::primitive::usize::BITS }>;\n",
        );
        assert!(errors.is_empty(), "{errors:?}");
    }

    #[test]
    fn stripping_keeps_the_lines_where_they_were() {
        let text = "a /* x\ny */ b // z\n\"q\nr\" 'c'\n";
        let stripped = without_comments(text);
        assert_eq!(stripped.lines().count(), text.lines().count());
        assert_eq!(stripped.matches('\n').count(), text.matches('\n').count());
        assert!(stripped.contains(" b "));
        assert!(!stripped.contains('x') && !stripped.contains('z') && !stripped.contains('q'));
    }

    #[test]
    fn it_reads_the_crate_once_rather_than_once_per_file() {
        assert!(!ThePlatformWidthPointsReadThePointerWidth.per_file());
    }

    #[test]
    fn another_crate_is_not_read() {
        let fixture = LintFixture::new("pub type USize = UFixed<64, 0>;\n")
            .with_crate_name("arvo-placement", "placement");
        assert!(
            ThePlatformWidthPointsReadThePointerWidth
                .check(&fixture.ctx())
                .is_empty()
        );
    }

    #[test]
    fn a_module_named_usize_in_scope_still_fires_on_the_bare_spelling() {
        // A module or item literally named `usize`, declared or brought into scope
        // in the same file, would take precedence over the primitive type at a bare
        // `usize::BITS` path, so a build under that shadow could read a `BITS` this
        // lint never sees. The lint cannot resolve names, but it does not need to:
        // comparing the text against `core::primitive::usize::BITS` exactly finds
        // the bare spelling regardless of whether anything in scope shadows it, so
        // the hole is closed by construction rather than by checking for the shadow.
        let errors = hits(
            "mod usize {\n    pub const BITS: u32 = 99;\n}\npub type USize = UFixed<{ usize::BITS \
             }, 0>;\npub type ISize = Integer<{ core::primitive::usize::BITS }>;\n",
        );
        assert_eq!(errors.len(), 1);
        assert!(errors[0].message.contains("USize"));
    }
}
