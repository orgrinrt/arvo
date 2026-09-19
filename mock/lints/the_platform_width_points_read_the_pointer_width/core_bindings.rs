//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! The lint's second arm: the three forms that bind the name `core`, read as
//! tokens.
//!
//! The text is first stripped of comments and of the inside of every literal,
//! then cut into words and punctuation, so whitespace, a line break or a comment
//! between two words is simply not a token. A raw identifier is a word with its
//! `r#` taken off and marked raw: it matches `core` as `core` does, and it never
//! matches a keyword, since `r#use` names something rather than opening an item.

use super::without_comments;

/// What kind of token a piece of stripped source is.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(super) enum Kind {
    /// A word, a keyword included.
    Word,
    /// A raw identifier, with its `r#` taken off.
    Raw,
    /// One punctuation mark, or `::` as one.
    Punct,
}

/// A token and the byte offset it starts at.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(super) struct Token<'a> {
    pub(super) text: &'a str,
    pub(super) kind: Kind,
    pub(super) at:   usize,
}

impl Token<'_> {
    /// Whether this is the keyword `word`, which a raw identifier never is.
    fn is_keyword(&self, word: &str) -> bool {
        self.kind == Kind::Word && self.text == word
    }

    /// Whether this names `core`, plain or raw.
    fn is_core(&self) -> bool {
        self.kind != Kind::Punct && self.text == "core"
    }

    /// Whether this is the punctuation `mark`.
    fn is(&self, mark: &str) -> bool {
        self.kind == Kind::Punct && self.text == mark
    }

    /// Whether this can be a path segment or a name.
    fn is_name(&self) -> bool {
        self.kind != Kind::Punct
    }
}

/// Whether `c` continues a word.
fn is_word_char(c: char) -> bool {
    c.is_alphanumeric() || c == '_'
}

/// The tokens of `code`, which is expected already stripped.
pub(super) fn tokens(code: &str) -> Vec<Token<'_>> {
    let mut out = Vec::new();
    let mut chars = code.char_indices().peekable();
    while let Some((at, c)) = chars.next() {
        if c.is_whitespace() {
            continue;
        }
        if is_word_char(c) {
            let mut end = at + c.len_utf8();
            while let Some(&(i, d)) = chars.peek() {
                if !is_word_char(d) {
                    break;
                }
                end = i + d.len_utf8();
                chars.next();
            }
            let word = &code[at .. end];
            // `r#` followed by a word start is a raw identifier; `r#"` is what
            // is left of a raw string and stays three tokens.
            let raw_start = code[end ..]
                .strip_prefix('#')
                .and_then(|rest| rest.chars().next())
                .is_some_and(|d| d.is_alphabetic() || d == '_');
            if word == "r" && raw_start {
                chars.next();
                let start = end + 1;
                let mut stop = start;
                while let Some(&(i, d)) = chars.peek() {
                    if !is_word_char(d) {
                        break;
                    }
                    stop = i + d.len_utf8();
                    chars.next();
                }
                out.push(Token {
                    text: &code[start .. stop],
                    kind: Kind::Raw,
                    at,
                });
            } else {
                out.push(Token {
                    text: word,
                    kind: Kind::Word,
                    at,
                });
            }
            continue;
        }
        if c == ':' && chars.peek().is_some_and(|&(_, d)| d == ':') {
            chars.next();
            out.push(Token {
                text: &code[at .. at + 2],
                kind: Kind::Punct,
                at,
            });
            continue;
        }
        out.push(Token {
            text: &code[at .. at + c.len_utf8()],
            kind: Kind::Punct,
            at,
        });
    }
    out
}

/// Which of the three forms a finding is.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(super) enum Form {
    /// A module named `core`.
    Mod,
    /// A `use` item binding the name `core`.
    Use,
    /// An `extern crate` item aliased `as core`.
    ExternCrate,
}

/// Every binding of the name `core` in `text`, as the form and the byte offset
/// of the token that binds it in the stripped text.
pub(super) fn core_bindings_in(text: &str) -> (String, Vec<(Form, usize)>) {
    let code = without_comments(text);
    let found = {
        let toks = tokens(&code);
        let mut found = Vec::new();
        let mut i = 0;
        while i < toks.len() {
            let t = toks[i];
            if t.is_keyword("mod") && toks.get(i + 1).is_some_and(Token::is_core) {
                found.push((Form::Mod, toks[i + 1].at));
            } else if t.is_keyword("extern")
                && toks.get(i + 1).is_some_and(|n| n.is_keyword("crate"))
            {
                if let [name, r#as, alias, ..] = toks.get(i + 2 ..).unwrap_or(&[]) {
                    if name.is_name() && r#as.is_keyword("as") && alias.is_core() {
                        found.push((Form::ExternCrate, alias.at));
                    }
                }
            } else if t.is_keyword("use") {
                // Scanning goes on from the next token rather than past the
                // item, so a `use<..>` capture bound, whose tree reads as empty,
                // hides nothing after it.
                let end = item_end(&toks, i + 1);
                let mut bound = Vec::new();
                use_tree(&toks[i + 1 .. end], 0, None, &mut bound);
                found.extend(
                    bound
                        .into_iter()
                        .filter(|b| b.is_core())
                        .map(|b| (Form::Use, b.at)),
                );
            }
            i += 1;
        }
        found
    };
    (code, found)
}

/// The index of the `;` closing a `use` item whose tree starts at `from`, or the
/// end of the tokens. Braces nest, so a `;` inside one is not the item's.
fn item_end(toks: &[Token<'_>], from: usize) -> usize {
    let mut depth = 0usize;
    for (i, t) in toks.iter().enumerate().skip(from) {
        if t.is("{") {
            depth += 1;
        } else if t.is("}") {
            depth = depth.saturating_sub(1);
        } else if t.is(";") && depth == 0 {
            return i;
        }
    }
    toks.len()
}

/// Every token a use tree binds, from `pos`, pushed onto `bound`, and the
/// position after the tree.
///
/// `prefix` is the last path segment before the tree, which a `self` in it
/// binds. A path's last segment binds itself, an `as` binds its alias, and a
/// glob binds nothing this walk can name. A malformed tree stops the walk
/// where it stops parsing, keeping what it found.
fn use_tree<'a>(
    toks: &[Token<'a>],
    mut pos: usize,
    prefix: Option<Token<'a>>,
    bound: &mut Vec<Token<'a>>,
) -> usize {
    let mut last = prefix;
    if toks.get(pos).is_some_and(|t| t.is("::")) {
        pos += 1;
    }
    loop {
        let Some(t) = toks.get(pos) else {
            return pos;
        };
        if t.is("*") {
            return pos + 1;
        }
        if t.is("{") {
            pos += 1;
            loop {
                if toks.get(pos).is_some_and(|t| t.is("}")) {
                    return pos + 1;
                }
                let next = use_tree(toks, pos, last, bound);
                if next == pos {
                    return pos;
                }
                pos = next;
                match toks.get(pos) {
                    Some(t) if t.is(",") => pos += 1,
                    Some(t) if t.is("}") => return pos + 1,
                    _ => return pos,
                }
            }
        }
        if !t.is_name() {
            return pos;
        }
        let segment = *t;
        pos += 1;
        if toks.get(pos).is_some_and(|t| t.is("::")) {
            last = Some(segment);
            pos += 1;
            continue;
        }
        if toks.get(pos).is_some_and(|t| t.is_keyword("as")) {
            if let Some(alias) = toks.get(pos + 1).filter(|a| a.is_name()) {
                if alias.text != "_" {
                    bound.push(*alias);
                }
                return pos + 2;
            }
            return pos + 1;
        }
        if segment.is_keyword("self") {
            if let Some(parent) = last {
                bound.push(Token {
                    at: segment.at,
                    ..parent
                });
            }
        } else {
            bound.push(segment);
        }
        return pos;
    }
}

/// Every binding of the name `core` in each file, as a path, a line and a
/// message.
pub(super) fn core_bindings(files: &[(String, &str)]) -> Vec<(Option<String>, usize, String)> {
    let mut found = Vec::new();
    for (path, text) in files {
        let (code, bindings) = core_bindings_in(text);
        for (form, at) in bindings {
            let line = code[.. at].matches('\n').count() + 1;
            found.push((Some(path.clone()), line, message(form).to_string()));
        }
    }
    found
}

/// The finding's message for each form.
pub(super) fn message(form: Form) -> &'static str {
    match form {
        Form::Mod => {
            "declares a module named `core`, which shadows the crate `core` for a bare `core::...` \
             path in the module that declares it. The platform-width aliases read \
             `::core::primitive::usize::BITS`, whose leading `::` resolves among the crates rather \
             than among that module's names, so this does not reach them; it is refused because \
             another path in that module may be spelled bare. Name the module something else."
        },
        Form::Use => {
            "binds the name `core` in a `use` item, as an alias, as a path's last segment, or \
             through a `self` under a `core` segment. A `use` binds only in the module that holds \
             it, and there it shadows the crate `core` for a bare `core::...` path. It does not \
             reach `::core::primitive::usize::BITS`, whose leading `::` resolves among the crates \
             rather than among that module's names; it is refused because another path in that \
             module may be spelled bare. Import it under another name."
        },
        Form::ExternCrate => {
            "binds the name `core` through an `extern crate ... as core` alias. At the crate root \
             that alias makes `::core` itself name the aliased crate, so it reaches \
             `::core::primitive::usize::BITS`, which the leading `::` does not resist. Alias it to \
             something else."
        },
    }
}
