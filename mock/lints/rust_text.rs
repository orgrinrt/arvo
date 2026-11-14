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

/// Whether a crate-relative path is a test module: under a `tests` directory or
/// a file named `tests.rs`.
pub fn is_test_path(path: &str) -> bool {
    let parts: Vec<&str> = path.split(['/', '\\']).collect();
    let (file, dirs) = parts.split_last().unwrap_or((&"", &[]));
    *file == "tests.rs" || dirs.contains(&"tests")
}

pub fn is_word(c: char) -> bool {
    c.is_ascii_alphanumeric() || c == '_'
}

/// The offset of every `fn` keyword standing as a whole word.
pub fn fn_keywords(chars: &[char]) -> Vec<usize> {
    let mut out = Vec::new();
    let mut i = 0;
    while i + 1 < chars.len() {
        if chars[i] == 'f'
            && chars[i + 1] == 'n'
            && (i == 0 || !is_word(chars[i - 1]))
            && chars.get(i + 2).is_some_and(|c| c.is_whitespace())
        {
            out.push(i);
        }
        i += 1;
    }
    out
}

/// The identifier after the `fn` keyword, if one follows.
pub fn name_after(chars: &[char], mut i: usize) -> Option<String> {
    while i < chars.len() && chars[i].is_whitespace() {
        i += 1;
    }
    let start = i;
    while i < chars.len() && is_word(chars[i]) {
        i += 1;
    }
    (i > start).then(|| chars[start .. i].iter().collect())
}

/// Whether the signature from `i` on reaches a `{` before a `;`, outside every
/// parenthesis, bracket and angle bracket, which is the body. A brace inside an
/// angle bracket is a const generic argument, and the `>` of a `->` closes
/// nothing.
pub fn has_body(chars: &[char], mut i: usize) -> bool {
    let mut depth = 0i32;
    while i < chars.len() {
        match chars[i] {
            '(' | '[' | '<' => depth += 1,
            ')' | ']' => depth -= 1,
            '>' if i > 0 && chars[i - 1] == '-' => {},
            '>' => depth -= 1,
            '{' if depth == 0 => return true,
            ';' if depth == 0 => return false,
            _ => {},
        }
        i += 1;
    }
    false
}

/// Where the item holding the `fn` at `at` starts, and its attributes.
///
/// Walks back from the keyword over the qualifiers (`pub` with its restriction,
/// `const`, `unsafe`, `async`, `extern` with its ABI, `default`) and then over
/// every `#[...]` directly above them. The start is the first character of the
/// first qualifier, so the line a finding names is the one a mark goes above.
pub fn item_head(chars: &[char], at: usize) -> (usize, Vec<String>) {
    const QUALIFIERS: &[&str] = &["pub", "const", "unsafe", "async", "extern", "default"];
    let mut start = at;
    let mut i = at;
    loop {
        let mut j = i;
        while j > 0 && chars[j - 1].is_whitespace() {
            j -= 1;
        }
        if j == 0 {
            break;
        }
        match chars[j - 1] {
            ')' => {
                // `pub(crate)` and its kin.
                let Some(open) = chars[.. j - 1].iter().rposition(|&c| c == '(') else {
                    break;
                };
                i = open;
            },
            '"' => {
                // An ABI string, blanked inside, `extern "C"`.
                let Some(open) = chars[.. j - 1].iter().rposition(|&c| c == '"') else {
                    break;
                };
                i = open;
            },
            c if is_word(c) => {
                let mut k = j;
                while k > 0 && is_word(chars[k - 1]) {
                    k -= 1;
                }
                let word: String = chars[k .. j].iter().collect();
                if !QUALIFIERS.contains(&word.as_str()) {
                    break;
                }
                i = k;
                start = k;
            },
            _ => break,
        }
    }
    let mut attributes = Vec::new();
    let mut i = start;
    loop {
        let mut j = i;
        while j > 0 && chars[j - 1].is_whitespace() {
            j -= 1;
        }
        if j == 0 || chars[j - 1] != ']' {
            break;
        }
        let mut depth = 0i32;
        let mut k = j;
        let mut open = None;
        while k > 0 {
            k -= 1;
            match chars[k] {
                ']' => depth += 1,
                '[' => {
                    depth -= 1;
                    if depth == 0 {
                        open = Some(k);
                        break;
                    }
                },
                _ => {},
            }
        }
        let Some(open) = open else {
            break;
        };
        if open == 0 || chars[open - 1] != '#' {
            break;
        }
        attributes.push(chars[open - 1 .. j].iter().collect());
        i = open - 1;
    }
    (start, attributes)
}
