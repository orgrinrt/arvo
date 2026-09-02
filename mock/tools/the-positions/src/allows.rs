//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! Where a consumer turned the rule off, which is the demand stated by the
//! consumer rather than read off an identifier.
//!
//! **This is the only measurement here that does not depend on a judgement.**
//! Every other number rests on the role reading, the carrier rule or the
//! visibility approximation. A `lint:allow(no-bare-numeric)` is a consumer
//! saying, in the source, at the site, that the rule forbids what it needs and
//! that it is proceeding anyway. That is a demand on arvo with a return address.
//!
//! The four names are the lints that enforce the constraint the obligation comes
//! from: two for the numeric family, one for a leaked inner field, one for
//! static text. `no-public-raw-field` is included because a newtype whose inner
//! integer is `pub` is the primitive at an API position wearing a wrapper, which
//! is the same position under a different lint.
//!
//! `reason:` and `tracked:` are pulled out where the site carries them, since a
//! suppression naming a task is a demand somebody has already filed and a
//! suppression naming none is one nobody has.

/// The lints whose suppression means the stack asked arvo for something.
pub const WATCHED: &[&str] = &[
    "no-bare-numeric",
    "arvo-types-only",
    "no-public-raw-field",
    "no-bare-static-str",
];

/// One suppression.
pub struct Allow {
    pub tree: String,
    pub path: String,
    pub line: usize,
    pub lint: &'static str,
    /// The `reason:` text where the site gives one, up to the next marker.
    pub reason: String,
    /// The `tracked:` identifier where the site gives one.
    pub tracked: String,
    /// Whether the marker sits on a line that is nothing but a comment.
    ///
    /// **Such a marker is inert under the pack that reads it**, and the pack
    /// says so in its own source rather than by inference. `line_lint_allowed`
    /// is strictly per line and looks at no neighbour, and `no_bare_numeric`
    /// skips any line whose trimmed text starts with `//` before it scans at
    /// all. So a marker written above the item it means to cover suppresses
    /// nothing, and the line it meant to cover is still checked.
    pub comment_only: bool,
    /// The line as written, so a report can say what the marker sits beside.
    pub text: String,
}

/// Every suppression of a watched lint in one file.
///
/// Line-oriented, and correctly so: a `lint:allow` is a comment and the thing
/// that reads it in anger is itself a line scan, so matching the same way is
/// what makes the count the same count.
pub fn allows_in(tree: &str, path: &str, source: &str) -> Vec<Allow> {
    let mut out = Vec::new();
    for (idx, line) in source.lines().enumerate() {
        for lint in WATCHED {
            let needle = format!("lint:allow({lint})");
            if !line.contains(&needle) {
                continue;
            }
            out.push(Allow {
                tree: tree.to_string(),
                path: path.to_string(),
                line: idx + 1,
                lint,
                reason: field_after(line, "reason:"),
                tracked: field_after(line, "tracked:"),
                comment_only: line.trim_start().starts_with("//"),
                text: line.trim().to_string(),
            });
        }
    }
    out
}

/// The text after a marker, stopping at the next marker on the same line.
///
/// A site commonly writes `reason: ... tracked: #72`, and several write two
/// `lint:allow` markers before the reason, so a naive read to end of line takes
/// the tracking identifier into the reason and a naive read to the next space
/// takes one word of it.
fn field_after(line: &str, marker: &str) -> String {
    let Some(at) = line.find(marker) else {
        return String::new();
    };
    let rest = &line[at + marker.len()..];
    let end = ["reason:", "tracked:", "lint:allow("]
        .iter()
        .filter_map(|m| rest.find(m))
        .min()
        .unwrap_or(rest.len());
    rest[..end].trim().trim_end_matches(';').trim().to_string()
}

/// Whether the pack that reads these markers would have flagged this line.
///
/// **The pack's own rule, reimplemented rather than approximated**, because the
/// question is about that instrument rather than about mine. It strips the line
/// comment, blanks the contents of strings and character literals, and looks for
/// a primitive's name on a word boundary. It also returns early on any line
/// whose trimmed text starts with `//`.
///
/// This exists because 644 markers sat on lines carrying nothing my parse could
/// see, and "my parse cannot see it" is a statement about my parse. Under the
/// pack's own rule the question has an answer: either the line would have been
/// flagged, in which case the marker is load-bearing, or it would not, in which
/// case the marker suppresses a diagnostic that was never going to fire.
#[must_use]
pub fn the_pack_would_flag(line: &str) -> bool {
    if line.trim_start().starts_with("//") {
        return false;
    }
    let scan = blank_literals(&before_line_comment(line));
    BARE.iter().any(|prim| word_boundary(&scan, prim))
}

/// The names the pack refuses, in its own order.
const BARE: &[&str] = &[
    "u8", "u16", "u32", "u64", "u128", "i8", "i16", "i32", "i64", "i128", "f32", "f64", "usize",
    "isize", "bool",
];

fn before_line_comment(line: &str) -> String {
    match line.find("//") {
        Some(i) => line[..i].to_string(),
        None => line.to_string(),
    }
}

/// The line with the body of every string and character literal removed.
fn blank_literals(line: &str) -> String {
    let bytes = line.as_bytes();
    let mut out = String::with_capacity(line.len());
    let mut i = 0usize;
    while i < bytes.len() {
        match bytes[i] {
            b'"' => {
                out.push('"');
                i += 1;
                while i < bytes.len() {
                    if bytes[i] == b'\\' && i + 1 < bytes.len() {
                        i += 2;
                        continue;
                    }
                    if bytes[i] == b'"' {
                        out.push('"');
                        i += 1;
                        break;
                    }
                    i += 1;
                }
            }
            b'\'' => {
                out.push('\'');
                i += 1;
                let start = i;
                while i < bytes.len() {
                    if bytes[i] == b'\\' && i + 1 < bytes.len() {
                        i += 2;
                        continue;
                    }
                    if bytes[i] == b'\'' && i != start {
                        out.push('\'');
                        i += 1;
                        break;
                    }
                    i += 1;
                }
            }
            b => {
                out.push(b as char);
                i += 1;
            }
        }
    }
    out
}

/// Whether `needle` appears in `hay` with a non-identifier byte on each side.
fn word_boundary(hay: &str, needle: &str) -> bool {
    let bytes = hay.as_bytes();
    let n = needle.as_bytes();
    if n.is_empty() || n.len() > bytes.len() {
        return false;
    }
    let ident = |b: u8| b.is_ascii_alphanumeric() || b == b'_';
    let mut i = 0usize;
    while i + n.len() <= bytes.len() {
        if &bytes[i..i + n.len()] == n {
            let before = i == 0 || !ident(bytes[i - 1]);
            let after = i + n.len() >= bytes.len() || !ident(bytes[i + n.len()]);
            if before && after {
                return true;
            }
        }
        i += 1;
    }
    false
}
