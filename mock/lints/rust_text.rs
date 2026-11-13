//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! Rust source read as text, for the crate lints that read it that way.
//!
//! This file declares no lint. The engine includes every `mock/lints/*.rs` as
//! a module at the crate root and registers only what defines `lint()` and its
//! kin, so the lints here reach this as `crate::rust_text`.

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
pub fn without_comments(text: &str) -> String {
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
