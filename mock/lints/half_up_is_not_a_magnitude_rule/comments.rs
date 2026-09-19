//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! The prose in a Rust source file: its comments and doc comments, and nothing
//! inside a string or a character literal.
//!
//! Whole-line comments of one kind on consecutive lines are one passage, so a
//! sentence broken across lines is read whole. A comment after code on the same
//! line is a passage of its own. An outer doc block is bound to the item it
//! documents: where that item's line names the mode, the block is read as being
//! about the mode, since rustdoc renders it under that name.

use super::reading::spellings_in;

/// One passage of prose, the line it starts on, and the spelling of the mode
/// its item carries where it is an outer doc block on one.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct Passage {
    pub(super) text:    String,
    pub(super) line:    usize,
    pub(super) subject: Option<&'static str>,
}

impl Passage {
    /// The line an offset into the text lies on.
    pub(super) fn line_of(&self, at: usize) -> usize {
        self.line + self.text[.. at.min(self.text.len())].matches('\n').count()
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Kind {
    Outer,
    Inner,
    Plain,
}

/// A run of whole-line comments of one kind being gathered.
struct Run {
    kind:  Kind,
    first: usize,
    last:  usize,
    lines: Vec<String>,
}

/// Every passage of prose in `source`.
pub(super) fn passages(source: &str) -> Vec<Passage> {
    let starts: Vec<usize> = core::iter::once(0)
        .chain(source.match_indices('\n').map(|(i, _)| i + 1))
        .collect();
    let line_of = |at: usize| starts.partition_point(|&s| s <= at);
    let lines: Vec<&str> = source.lines().collect();
    let bytes = source.as_bytes();
    let mut out = Vec::new();
    let mut run: Option<Run> = None;
    let mut i = 0;
    while i < bytes.len() {
        match bytes[i] {
            b'/' if bytes.get(i + 1) == Some(&b'/') => {
                let end = source[i ..].find('\n').map_or(source.len(), |n| i + n);
                let body = &source[i + 2 .. end];
                let (kind, text) = if body.starts_with('/') && !body.starts_with("//") {
                    (Kind::Outer, &body[1 ..])
                } else if let Some(rest) = body.strip_prefix('!') {
                    (Kind::Inner, rest)
                } else {
                    (Kind::Plain, body)
                };
                let line = line_of(i);
                let whole = source[starts[line - 1] .. i].trim().is_empty();
                let joins = whole
                    && run
                        .as_ref()
                        .is_some_and(|r| r.kind == kind && r.last + 1 == line);
                if joins {
                    let r = run.as_mut().expect("a run to join");
                    r.last = line;
                    r.lines.push(text.to_string());
                } else {
                    flush(&mut run, &lines, &mut out);
                    if whole {
                        run = Some(Run {
                            kind,
                            first: line,
                            last: line,
                            lines: vec![text.to_string()],
                        });
                    } else {
                        out.push(Passage {
                            text: text.to_string(),
                            line,
                            subject: None,
                        });
                    }
                }
                i = end;
            },
            b'/' if bytes.get(i + 1) == Some(&b'*') => {
                flush(&mut run, &lines, &mut out);
                let end = block_end(bytes, i);
                let inner_end = end.saturating_sub(2).max(i + 2);
                out.push(Passage {
                    text:    source[i + 2 .. inner_end].to_string(),
                    line:    line_of(i),
                    subject: None,
                });
                i = end;
            },
            b'"' => i = string_end(bytes, i + 1),
            b'r' | b'b' if raw_open(bytes, i).is_some() => {
                let (hashes, quote) = raw_open(bytes, i).expect("a raw string");
                i = raw_end(bytes, quote + 1, hashes);
            },
            b'\'' => i = char_end(source, i),
            _ => i += 1,
        }
    }
    flush(&mut run, &lines, &mut out);
    out
}

/// Close the run being gathered, binding an outer doc block to its item.
fn flush(run: &mut Option<Run>, lines: &[&str], out: &mut Vec<Passage>) {
    let Some(r) = run.take() else {
        return;
    };
    let subject = if r.kind == Kind::Outer { item_subject(lines, r.last) } else { None };
    out.push(Passage {
        text: r.lines.join("\n"),
        line: r.first,
        subject,
    });
}

/// The spelling of the mode on the item a doc block ending at line `last`
/// documents, past any attributes, blank lines and plain comments between.
fn item_subject(lines: &[&str], last: usize) -> Option<&'static str> {
    let item = lines.iter().skip(last).map(|l| l.trim()).find(|l| {
        !(l.is_empty() || l.starts_with("#[") || (l.starts_with("//") && !l.starts_with("///")))
    })?;
    let code = item.split("//").next().unwrap_or(item);
    spellings_in(code).first().map(|(_, s)| *s)
}

/// One past the `*/` closing the block comment opening at `from`, nesting as
/// Rust nests them.
fn block_end(bytes: &[u8], from: usize) -> usize {
    let mut depth = 0;
    let mut i = from;
    while i + 1 < bytes.len() {
        if bytes[i] == b'/' && bytes[i + 1] == b'*' {
            depth += 1;
            i += 2;
        } else if bytes[i] == b'*' && bytes[i + 1] == b'/' {
            depth -= 1;
            i += 2;
            if depth == 0 {
                return i;
            }
        } else {
            i += 1;
        }
    }
    bytes.len()
}

/// One past the quote closing a string whose body starts at `from`.
fn string_end(bytes: &[u8], from: usize) -> usize {
    let mut i = from;
    while i < bytes.len() {
        match bytes[i] {
            b'\\' => i += 2,
            b'"' => return i + 1,
            _ => i += 1,
        }
    }
    bytes.len()
}

/// Where a raw string opens at `at`: how many hashes, and where its quote is.
/// Only where `at` starts a token, so an identifier ending in `r` is not one.
fn raw_open(bytes: &[u8], at: usize) -> Option<(usize, usize)> {
    if at > 0 && (bytes[at - 1].is_ascii_alphanumeric() || bytes[at - 1] == b'_') {
        return None;
    }
    let mut i = at;
    if bytes[i] == b'b' {
        i += 1;
    }
    if bytes.get(i) != Some(&b'r') {
        return None;
    }
    i += 1;
    let hashes = bytes[i ..].iter().take_while(|&&b| b == b'#').count();
    (bytes.get(i + hashes) == Some(&b'"')).then_some((hashes, i + hashes))
}

/// One past a raw string's close, its body starting at `from`.
fn raw_end(bytes: &[u8], from: usize, hashes: usize) -> usize {
    let mut i = from;
    while i < bytes.len() {
        if bytes[i] == b'"'
            && bytes[i + 1 ..]
                .iter()
                .take(hashes)
                .filter(|&&b| b == b'#')
                .count()
                == hashes
        {
            return i + 1 + hashes;
        }
        i += 1;
    }
    bytes.len()
}

/// Past a character literal opening at `at`, or past the quote alone where it
/// opens a lifetime or a label.
fn char_end(source: &str, at: usize) -> usize {
    let rest = &source[at + 1 ..];
    if let Some(escaped) = rest.strip_prefix('\\') {
        // `'\n'`, `'\''`, `'\u{..}'`: the escape's first character, then up to
        // the closing quote.
        let skip = escaped.chars().next().map_or(0, char::len_utf8);
        return escaped[skip ..]
            .find('\'')
            .map_or(source.len(), |n| at + 2 + skip + n + 1);
    }
    let mut chars = rest.chars();
    match (chars.next(), chars.next()) {
        (Some(c), Some('\'')) => at + 1 + c.len_utf8() + 1,
        _ => at + 1,
    }
}
