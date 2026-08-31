//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! Which rounding modes the predicates actually name, against the six the canon
//! ratified.
//!
//! `every-predicate-names-a-declared-axis` checks the axis side of a predicate
//! entry and says in its own documentation that it does not check the values
//! side, because the grammars differ per axis and live as prose on the
//! `dimension` row rather than as a pattern. That reasoning holds for every axis
//! but one. `ruling::the_ambiguous_rounding_word_is_retired_for_six_explicit_names`
//! closes the rounding vocabulary at six names, so on this axis alone there is a
//! set to check against, and it is ratified rather than inferred.
//!
//! What the closure was for is the reason this matters. The retired word names
//! two different operations on a signed domain, dropping the low bits of a two's
//! complement value and moving toward zero, and those differ on exactly the rows
//! where signedness is live. A predicate carrying the retired word states a
//! region that cannot be resolved without going back to the instrument that
//! produced it, and it reads as a region either way.
//!
//! # Not a lint
//!
//! Gating would be right if every finding had a fix somebody could apply. Most
//! of these do not, yet. An alias is mechanical and could be gated tomorrow. The
//! retired word cannot: nobody can tell from the row which of the two operations
//! its author swept, and rewriting it to a guess would invent a region rather
//! than record one, which is worse than the drift. And a mode outside the six is
//! a question about whether the vocabulary is complete, which is canon work
//! rather than an edit.
//!
//! So this reports and orders, and the lint gets written once the vocabulary
//! question closes and every remaining row has a known correct spelling. The
//! report is what makes that question answerable, since it names every row the
//! answer has to cover.

use mockspace::tool::{ArgSpec, NotALint, Outcome, Tool, ToolContext, ToolReport};
use mockspace::RegistryView;

/// The ruling that closed the vocabulary. Read at run time rather than trusted,
/// so this tool cannot outlive the row it enforces.
const CLOSING_RULING: &str =
    "ruling::the_ambiguous_rounding_word_is_retired_for_six_explicit_names";

/// The axis, as `dimension` spells it.
const AXIS: &str = "rounding";

/// The six, as the ruling spells them.
const RATIFIED: [&str; 6] = [
    "toward_zero",
    "floor",
    "ceil",
    "half_up",
    "half_even",
    "stochastic",
];

/// Values of the axis that are not modes. `exact` names the case where nothing
/// is discarded, which `dimension::rounding` calls a value of the axis rather
/// than its absence; `any` is the notation's universal.
const NOT_A_MODE: [&str; 2] = ["exact", "any"];

/// Spellings the corpus uses for a mode that is one of the six. Mechanical to
/// repair, so they are reported apart from the ones that are not.
const ALIASES: [(&str, &str); 6] = [
    ("toward zero", "toward_zero"),
    ("toward-zero", "toward_zero"),
    ("ceiling", "ceil"),
    ("nearest-half-even", "half_even"),
    ("round to nearest even", "half_even"),
    ("nearest-half-up", "half_up"),
];

/// The word the ruling retired, in every spelling it retired.
const RETIRED: [&str; 3] = ["truncate", "trunc", "truncation"];

/// Spellings that name a real distinction the writer did not make. `nearest`
/// alone does not say which way a tie goes, and the two ways are separate
/// members of the six.
const UNDERSPECIFIED: [&str; 1] = ["nearest"];

/// The fields a predicate lives in.
const PREDICATE_FIELDS: [(&str, &str); 3] = [
    ("proposal", "predicate"),
    ("law", "holds"),
    ("law", "fails"),
];

/// The separator entries are joined with.
const JOIN: &str = ", ";

/// How a named mode stands against the ratified set.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Standing {
    /// One of the six, spelled as the ruling spells it.
    Ratified,
    /// Not a mode: `exact` or `any`.
    NotAMode,
    /// A different spelling of one of the six.
    Alias(&'static str),
    /// The retired word, which names two of the six and does not say which.
    Retired,
    /// Names a distinction it does not make.
    Underspecified,
    /// Not among the six at all.
    Unknown,
}

pub struct RoundingVocabulary;

impl Tool for RoundingVocabulary {
    fn name(&self) -> &'static str {
        "rounding-vocabulary"
    }

    fn description(&self) -> &'static str {
        "rounding modes named in predicates, against the six the canon ratified"
    }

    fn not_a_lint(&self) -> NotALint {
        NotALint::NoFailingCase
    }

    fn args(&self) -> &'static [ArgSpec] {
        &[ArgSpec {
            name: "mode",
            required: false,
            description: "report only the rows naming one spelling, by that spelling",
        }]
    }

    fn help(&self) -> &'static str {
        "With no argument: every `rounding` entry across `proposal::predicate`, \
         `law::holds` and `law::fails`, grouped by how the mode it names stands \
         against the six the canon ratified. Aliases first, because they are \
         mechanical. Then the retired word and the underspecified spellings, \
         which need the instrument that produced the row rather than an edit. \
         Then anything outside the six, which is a question about whether the \
         vocabulary is complete.\n\n\
         With a spelling: the rows naming it, in full.\n\n\
         Nothing here fails, and that is deliberate. A finding whose correct \
         repair nobody knows is not a gate."
    }

    fn run(&self, ctx: &ToolContext<'_>) -> ToolReport {
        if let Some(why) = self.vocabulary_disagrees(ctx.registry) {
            return ToolReport::inconclusive(&why);
        }

        let found = gather(ctx.registry);
        if found.is_empty() {
            return ToolReport::inconclusive(
                "no predicate names the `rounding` axis, so this says nothing about \
                 whether the vocabulary holds. Either the canon carries no rounding \
                 claims, or the predicate fields are not where this expects them.",
            );
        }

        match ctx.args.first().copied() {
            Some(mode) => one(&found, mode),
            None => report(&found),
        }
    }
}

impl RoundingVocabulary {
    /// Whether the six this holds are the six the ruling names.
    ///
    /// A tool carrying its own copy of a ratified set is a second copy that can
    /// drift, and the drift is silent: every row would keep passing against a
    /// vocabulary nobody ratified. So the copy is checked against the row on
    /// every run, and a disagreement stops the tool rather than being reported
    /// as a finding about the canon, because at that point the instrument is
    /// what is wrong.
    fn vocabulary_disagrees(&self, reg: &RegistryView) -> Option<String> {
        let Some(says) = reg.field(CLOSING_RULING, "says") else {
            return Some(format!(
                "`{CLOSING_RULING}` is not in the registry, or carries no `says`. The \
                 six names below are only the vocabulary while that row says they are, \
                 so with the row gone this tool is checking against nothing."
            ));
        };
        let missing: Vec<&str> = RATIFIED
            .iter()
            .copied()
            .filter(|m| !says.contains(m))
            .collect();
        if missing.is_empty() {
            return None;
        }
        Some(format!(
            "`{CLOSING_RULING}` no longer names {}. This tool holds a copy of the six \
             and the copy has drifted from the row, so every result it would print is \
             against a vocabulary nobody ratified. Fix the list in this tool.",
            missing.join(", ")
        ))
    }
}

/// One `rounding` entry, and where it was written.
struct Entry {
    row: String,
    field: &'static str,
    /// The whole values side, as written.
    values: String,
    /// One mode named in it, and how it stands.
    mode: String,
    standing: Standing,
}

/// Every mode named by every `rounding` entry in the canon.
fn gather(reg: &RegistryView) -> Vec<Entry> {
    let mut out = Vec::new();
    for (namespace, field) in PREDICATE_FIELDS {
        for row in reg.rows_in(namespace) {
            let Some(joined) = reg.field(row, field) else {
                continue;
            };
            for entry in split_entries(joined) {
                let Some((axis, values)) = entry.split_once(':') else {
                    continue;
                };
                if axis.trim() != AXIS {
                    continue;
                }
                for mode in modes_in(values) {
                    out.push(Entry {
                        row: row.clone(),
                        field,
                        values: values.trim().to_string(),
                        standing: classify(&mode),
                        mode,
                    });
                }
            }
        }
    }
    out
}

/// The modes a values side names.
///
/// Three shapes reach here: a bare mode, `in {a, b, c}`, and a leading repeat of
/// the axis name, which several rows carry as `rounding: rounding = nearest`.
/// The repeat is stripped rather than reported, because it is the axis grammar's
/// own form and not a claim about a mode.
fn modes_in(values: &str) -> Vec<String> {
    let v = values.trim();
    let v = v.strip_prefix(AXIS).map(str::trim).unwrap_or(v);
    let v = v.strip_prefix('=').map(str::trim).unwrap_or(v);

    if let Some(open) = v.find('{')
        && let Some(close) = v[open..].find('}') {
            return v[open + 1..open + close]
                .split(',')
                .map(|m| m.trim().to_string())
                .filter(|m| !m.is_empty())
                .collect();
        }
    // A trailing clause after a comma is commentary on the mode rather than a
    // second mode, so only the head is read.
    let head = v.split(',').next().unwrap_or(v).trim();
    let head = head.strip_prefix("in").map(str::trim).unwrap_or(head);
    if head.is_empty() {
        Vec::new()
    } else {
        vec![head.to_string()]
    }
}

/// How one spelling stands against the ratified set.
fn classify(mode: &str) -> Standing {
    let m = mode.trim().to_lowercase();
    if RATIFIED.contains(&m.as_str()) {
        return Standing::Ratified;
    }
    if NOT_A_MODE.contains(&m.as_str()) {
        return Standing::NotAMode;
    }
    if RETIRED
        .iter()
        .any(|r| m.split_whitespace().any(|w| w == *r))
    {
        return Standing::Retired;
    }
    if let Some((_, to)) = ALIASES.iter().find(|(from, _)| *from == m) {
        return Standing::Alias(to);
    }
    if UNDERSPECIFIED.contains(&m.as_str()) {
        return Standing::Underspecified;
    }
    Standing::Unknown
}

/// Entries recovered from a joined predicate field.
///
/// A copy of the reader the lints share, because a tool is a separate crate and
/// cannot reach `mock/lints/`. Stated rather than hidden: two copies of one
/// parser will disagree eventually, and the fix when they do is moving it into
/// the engine's own surface rather than reconciling them here.
fn split_entries(joined: &str) -> Vec<&str> {
    let mut out = Vec::new();
    let (mut start, mut at) = (0usize, 0usize);
    while let Some(found) = joined[at..].find(JOIN) {
        let cut = at + found;
        let after = &joined[cut + JOIN.len()..];
        if starts_an_entry(after) {
            let entry = joined[start..cut].trim();
            if !entry.is_empty() {
                out.push(entry);
            }
            start = cut + JOIN.len();
        }
        at = cut + JOIN.len();
    }
    let last = joined[start..].trim();
    if !last.is_empty() {
        out.push(last);
    }
    out
}

/// Whether the text opens with `<slug>:`, which is where an entry begins.
fn starts_an_entry(s: &str) -> bool {
    let Some((name, _)) = s.split_once(':') else {
        return false;
    };
    !name.is_empty()
        && name
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_')
}

/// The rows naming one spelling.
fn one(found: &[Entry], mode: &str) -> ToolReport {
    let hits: Vec<&Entry> = found
        .iter()
        .filter(|e| e.mode.eq_ignore_ascii_case(mode))
        .collect();
    if hits.is_empty() {
        return ToolReport::inconclusive(format!(
            "no predicate names `{mode}`. That is a fact about this spelling and not \
             about the mode: another spelling of it may be all over the canon."
        ));
    }
    let mut s = format!("{} entr(y/ies) name `{mode}`.\n\n", hits.len());
    for e in &hits {
        s.push_str(&format!("  {} `{}`\n    {}\n", e.row, e.field, e.values));
    }
    ToolReport {
        outcome: Outcome::Clean {
            examined: found.len(),
        },
        output: s,
    }
}

/// Everything, grouped by standing.
fn report(found: &[Entry]) -> ToolReport {
    let pick = |want: fn(&Standing) -> bool| -> Vec<&Entry> {
        found.iter().filter(|e| want(&e.standing)).collect()
    };

    let aliases = pick(|s| matches!(s, Standing::Alias(_)));
    let retired = pick(|s| matches!(s, Standing::Retired));
    let vague = pick(|s| matches!(s, Standing::Underspecified));
    let unknown = pick(|s| matches!(s, Standing::Unknown));
    let clean = pick(|s| matches!(s, Standing::Ratified | Standing::NotAMode));

    let mut s = format!(
        "{} rounding mode(s) named across {} entries. {} already spelled as the \
         canon spells them.\n\n",
        found.len(),
        found.len(),
        clean.len()
    );

    let mut section = |title: &str, why: &str, rows: &[&Entry]| {
        if rows.is_empty() {
            return;
        }
        s.push_str(&format!("## {} ({})\n{}\n\n", title, rows.len(), why));
        for e in rows {
            let target = match e.standing {
                Standing::Alias(to) => format!(" -> `{to}`"),
                _ => String::new(),
            };
            s.push_str(&format!(
                "  `{}`{}\n    {} `{}`: {}\n",
                e.mode, target, e.row, e.field, e.values
            ));
        }
        s.push('\n');
    };

    section(
        "A different spelling of one of the six",
        "Mechanical. The mode is not in doubt, only how it is written, so each of \
         these can be rewritten to the name on the right without reading anything.",
        &aliases,
    );
    section(
        "The retired word",
        "Not mechanical. On a signed domain it names two of the six, dropping the \
         low bits and moving toward zero, and they differ on exactly the rows where \
         signedness is live. Which one a row means is a fact about the instrument \
         that produced it, so each of these needs its provenance opened rather than \
         an edit. Rewriting one to a guess invents a region.",
        &retired,
    );
    section(
        "Names a distinction it does not make",
        "`nearest` does not say which way a tie goes, and the two ways are separate \
         members of the six. Same repair as above: read the instrument.",
        &vague,
    );
    section(
        "Outside the six",
        "Neither a spelling of a ratified mode nor a retired word. Either the \
         vocabulary is incomplete and the ruling wants a further name, or the row \
         names a mode the design does not ship and its region is smaller than it \
         reads. That is a canon question rather than an edit, and it is the one \
         this report exists to make answerable.",
        &unknown,
    );

    let open = aliases.len() + retired.len() + vague.len() + unknown.len();
    s.push_str(&format!("{open} entr(y/ies) need attention.\n"));
    if open == 0 {
        s.push_str("Every named mode is one of the six, spelled as the ruling spells it.\n");
        return ToolReport {
            outcome: Outcome::Clean {
                examined: found.len(),
            },
            output: s,
        };
    }
    ToolReport {
        outcome: Outcome::Clean {
            examined: found.len(),
        },
        output: s,
    }
}

#[cfg(test)]
mod tests;

mockspace::lint_pack! {
    tools: [RoundingVocabulary],
}
