//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! Rows whose `gap` or `note` says an axis is undeclared, with the declared
//! dimensions beside each sentence.
//!
//! A row that says "no `dimension` row declares an encoding axis" was true on
//! the day it was written. A `dimension` row added since can make it false, and
//! nothing reads the sentence, so it stays. The search for these sentences was
//! done by hand once and returned twenty-six hits, of which fourteen were
//! instances and twelve were something else; it also missed two on its first try
//! because a phrase split over a TOML line continuation is invisible to a plain
//! grep. This reads the parsed registry, where a continuation is already joined,
//! and normalises whitespace on top of that.
//!
//! # Not a lint
//!
//! Whether a sentence is still true is a judgement about what a `dimension` row
//! declares, and the row that makes it false need not share a word with the
//! sentence. A gate would have to guess, and a guess that refuses a true
//! sentence or passes a false one is worse than the report. So this lists each
//! candidate and the declared dimensions whose id or keywords occur in it, and a
//! reader classifies each: still undeclared, declared since, or cannot tell.
//!
//! # What it reads
//!
//! The `gap` and `note` fields of every row in every namespace, split into
//! sentences. A sentence is a candidate when it speaks of an axis (`axis`,
//! `axes`, `dimension`, `coordinate`) and carries a phrase saying that axis is
//! not declared. The phrase list is [`CUES`], and a sentence matching none of
//! them is invisible here, so the report covers the phrasings the registry has
//! used rather than every way to say it.

use mockspace::RegistryView;
use mockspace::tool::{ArgSpec, NotALint, Purpose, Tool, ToolContext, ToolReport};

/// The fields a row says what it has not got in.
const FIELDS: [&str; 2] = ["gap", "note"];

/// Words saying the sentence is about an axis.
const AXIS_WORDS: [&str; 6] =
    ["axis", "axes", "dimension", "dimensions", "coordinate", "coordinates"];

/// Phrases saying an axis is not declared, lowercase. Each is a phrasing the
/// registry has used. A new phrasing is added here, and a row that had it is the
/// case that goes in the tests.
const CUES: [&str; 12] = [
    "undeclared",
    "not declared",
    "isn't declared",
    "is not yet declared",
    "no `dimension` row",
    "no dimension row",
    "no declared axis",
    "does not declare",
    "declares no",
    "is not an axis",
    "has no axis",
    "no axis for",
];

/// The namespace whose rows are the declared axes.
const DIMENSION: &str = "dimension";

pub struct AxisUndeclaredRows;

impl Tool for AxisUndeclaredRows {
    fn name(&self) -> &'static str {
        "axis-undeclared-rows"
    }

    fn description(&self) -> &'static str {
        "rows whose gap or note says an axis is undeclared, beside the declared dimensions"
    }

    fn purpose(&self) -> Purpose {
        Purpose::Check(NotALint::NoFailingCase)
    }

    fn args(&self) -> &'static [ArgSpec] {
        &[ArgSpec {
            name:        "row",
            required:    false,
            description: "report only the candidates of one row, by its `namespace::slug`",
        }]
    }

    fn help(&self) -> &'static str {
        "With no argument: every sentence in a `gap` or `note` that speaks of an axis \
         and says it is not declared, with the declared dimensions whose id or \
         keywords occur in the same sentence.\n\n\
         With a row: that row's candidates only.\n\n\
         Nothing here fails. Whether a sentence is still true depends on what a \
         `dimension` row declares, which the report lays beside it and a reader \
         judges: still undeclared, declared since, or cannot tell."
    }

    fn run(&self, ctx: &ToolContext<'_>) -> ToolReport {
        let declared = declared_dimensions(ctx.registry);
        if declared.is_empty() {
            return ToolReport::inconclusive(
                "the registry carries no `dimension` rows, so no sentence can be laid \
                 against a declared axis and this says nothing.",
            );
        }
        let (found, examined) = gather(ctx.registry, &declared);
        let wanted = ctx.args.first().copied();
        let shown: Vec<&Candidate> = found
            .iter()
            .filter(|c| wanted.is_none_or(|w| c.row == w))
            .collect();
        report(&shown, examined, declared.len())
    }
}

/// A declared dimension: its row, its slug, and the words that name it.
struct Declared {
    row:   String,
    names: Vec<String>,
}

/// One sentence that says an axis is undeclared.
struct Candidate {
    row:      String,
    field:    &'static str,
    sentence: String,
    /// Declared dimensions whose id or keywords occur in the sentence.
    beside:   Vec<String>,
}

/// Every `dimension` row with the words that name it: its id with underscores
/// as spaces, and each of its `keywords` that is longer than one character.
fn declared_dimensions(reg: &RegistryView) -> Vec<Declared> {
    reg.rows_in(DIMENSION)
        .iter()
        .map(|row| {
            let slug = row.split_once("::").map_or(row.as_str(), |(_, s)| s);
            let mut names = vec![slug.replace('_', " "), slug.to_string()];
            if let Some(keywords) = reg.field(row, "keywords") {
                names.extend(keyword_list(keywords));
            }
            names.retain(|n| n.chars().count() > 1);
            Declared {
                row: row.clone(),
                names,
            }
        })
        .collect()
}

/// The words of a `keywords` value, which the view holds as one string.
///
/// Read as the quoted items of a TOML array when it has quotes, else split on
/// commas, so either spelling of the stored value gives the same words.
fn keyword_list(raw: &str) -> Vec<String> {
    let quoted: Vec<String> = raw
        .split('"')
        .skip(1)
        .step_by(2)
        .map(|s| s.trim().to_lowercase())
        .filter(|s| !s.is_empty())
        .collect();
    if !quoted.is_empty() {
        return quoted;
    }
    raw.trim_matches(|c| c == '[' || c == ']')
        .split(',')
        .map(|s| s.trim().to_lowercase())
        .filter(|s| !s.is_empty())
        .collect()
}

/// Every candidate in the registry, and how many sentences were examined.
fn gather(reg: &RegistryView, declared: &[Declared]) -> (Vec<Candidate>, usize) {
    let mut out = Vec::new();
    let mut examined = 0usize;
    let namespaces: Vec<&str> = reg.namespaces().collect();
    for ns in namespaces {
        for row in reg.rows_in(ns) {
            for field in FIELDS {
                let Some(text) = reg.field(row, field) else {
                    continue;
                };
                for sentence in sentences(text) {
                    examined += 1;
                    if !says_an_axis_is_undeclared(&sentence) {
                        continue;
                    }
                    let lower = sentence.to_lowercase();
                    let beside = declared
                        .iter()
                        .filter(|d| d.names.iter().any(|n| lower.contains(n.as_str())))
                        .map(|d| d.row.clone())
                        .collect();
                    out.push(Candidate {
                        row: row.clone(),
                        field,
                        sentence,
                        beside,
                    });
                }
            }
        }
    }
    (out, examined)
}

/// Sentences of a value, whitespace joined to single spaces first.
///
/// A full stop ends a sentence only outside backticks and when whitespace or
/// the end follows, so `0.5` and a `path.rs` span stay whole.
fn sentences(text: &str) -> Vec<String> {
    let flat: String = text.split_whitespace().collect::<Vec<_>>().join(" ");
    let chars: Vec<char> = flat.chars().collect();
    let (mut out, mut cur, mut in_code) = (Vec::new(), String::new(), false);
    for (i, &c) in chars.iter().enumerate() {
        cur.push(c);
        match c {
            '`' => in_code = !in_code,
            '.' | '?' if !in_code => {
                let ends = chars.get(i + 1).is_none_or(|n| n.is_whitespace());
                if ends {
                    push_trimmed(&mut out, &mut cur);
                }
            },
            _ => {},
        }
    }
    push_trimmed(&mut out, &mut cur);
    out
}

fn push_trimmed(out: &mut Vec<String>, cur: &mut String) {
    let s = cur.trim();
    if !s.is_empty() {
        out.push(s.to_string());
    }
    cur.clear();
}

/// Whether one sentence speaks of an axis and says it is not declared.
fn says_an_axis_is_undeclared(sentence: &str) -> bool {
    let lower = sentence.to_lowercase();
    let words: Vec<&str> = lower
        .split(|c: char| !c.is_ascii_alphanumeric())
        .filter(|w| !w.is_empty())
        .collect();
    let about_an_axis = AXIS_WORDS.iter().any(|a| words.contains(a));
    about_an_axis && CUES.iter().any(|cue| lower.contains(cue))
}

/// The report, grouped by row so a row with several sentences reads together.
fn report(found: &[&Candidate], examined: usize, declared: usize) -> ToolReport {
    if found.is_empty() {
        return ToolReport::reported(
            format!(
                "no sentence in a `gap` or `note` says an axis is undeclared. That is a \
                 fact about the {} phrasings this reads, over {examined} sentences and \
                 {declared} declared dimensions, and not about every way to say it.\n",
                CUES.len()
            ),
            examined,
        );
    }
    let mut rows: Vec<&str> = found.iter().map(|c| c.row.as_str()).collect();
    rows.dedup();
    let mut s = format!(
        "{} sentence(s) on {} row(s) say an axis is undeclared, over {examined} sentences \
         and {declared} declared dimensions.\n\n",
        found.len(),
        rows.len()
    );
    for row in rows {
        s.push_str(&format!("{row}\n"));
        for c in found.iter().filter(|c| c.row == row) {
            s.push_str(&format!("  `{}`: {}\n", c.field, c.sentence));
            if c.beside.is_empty() {
                s.push_str("    declared dimensions in the sentence: none\n");
            } else {
                s.push_str(&format!(
                    "    declared dimensions in the sentence: {}\n",
                    c.beside.join(", ")
                ));
            }
        }
        s.push('\n');
    }
    ToolReport::reported(s, examined)
}

#[cfg(test)]
mod tests;

mockspace::lint_pack! {
    tools: [AxisUndeclaredRows],
}
