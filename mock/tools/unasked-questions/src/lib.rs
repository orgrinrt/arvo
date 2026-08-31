//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! Which questions are actually waiting on op, as against which merely say so.
//!
//! `awaiting-a-ruling` reports the `ruling` side: things he said that nobody has
//! asked him to bless. This is the other side, and the two do not overlap. A
//! `question` row names its `decider`, and where that is `op` the row is his to
//! settle. Reading that field alone overcounts, in two ways the field cannot
//! express on its own.
//!
//! **A question a ruling already answers is settled and still names him.** That
//! is correct history rather than a defect: he did decide it. But the row stays
//! in the namespace as audit trail, the rendered table does not carry the
//! incoming `answers` edge, and so every roster built by reading `decider` puts
//! an answered question back in the queue. Six of twenty-nine were in that state
//! when this was written, two of them from the round immediately before.
//!
//! **A question carrying a `bound` and still naming him is a filing error**, and
//! the schema says so in the `bound` field's own description. A bound is written
//! when a question filed as his is put to him and he hands it back: what he owed
//! was the constraint rather than the answer. A row carrying both says he both
//! did and did not keep it.
//!
//! # Ordering, and why it is only a suggestion
//!
//! By whether the row names something it `unblocks`, then by how many rows
//! reference it. The schema's own words on the first: a question that unblocks
//! nothing is one to ask later or never. Neither signal knows what a question
//! costs him to answer, and a cheap question unblocking three topics beats an
//! expensive one unblocking four. **The dispatcher reads the rows and picks.**
//!
//! # Not a lint
//!
//! There is no failing case. A question sitting at `decider = op` is the ordinary
//! resting state of something nobody has asked yet, and most of the namespace
//! will sit there legitimately for a long time. The filing error is the one thing
//! here that is genuinely wrong, and it is reported rather than gated, because
//! gating on it would block a commit over a row somebody is mid-way through
//! repairing.

use mockspace::tool::{ArgSpec, NotALint, Outcome, Tool, ToolContext, ToolReport};

/// The namespace of things the canon has not settled.
const QUESTION: &str = "question";

/// The namespace carrying op's own statements. A row here referencing a question
/// can only do so through `answers`, which is the only field in it typed to
/// `question`, so a referrer from this namespace means settled.
const RULING: &str = "ruling";

/// The field naming whose call it is.
const DECIDER: &str = "decider";

/// The value meaning his and nobody else's.
const HIS: &str = "op";

/// The field recording that he was asked and handed it back.
const BOUND: &str = "bound";

/// The field naming what becomes writable once it is answered.
const UNBLOCKS: &str = "unblocks";

/// The batch size op named for putting questions to him.
///
/// `AskUserQuestion` takes four at most, so a batch is asked in rounds and this
/// is the size of a round rather than a threshold anything fails.
const BATCH: usize = 4;

pub struct UnaskedQuestions;

impl Tool for UnaskedQuestions {
    fn name(&self) -> &'static str {
        "unasked-questions"
    }

    fn description(&self) -> &'static str {
        "questions still waiting on op, with the ones a ruling already answered taken out"
    }

    fn not_a_lint(&self) -> NotALint {
        NotALint::NoFailingCase
    }

    fn args(&self) -> &'static [ArgSpec] {
        &[ArgSpec {
            name: "key",
            required: false,
            description: "report one question in full, by its slug, before putting it to him",
        }]
    }

    fn help(&self) -> &'static str {
        "With no argument: every `question` row naming op as its decider that no \
         `ruling` answers, ordered by whether it names something it unblocks and \
         then by how many rows reference it. Questions a ruling has already \
         settled are counted and excluded rather than hidden, because the whole \
         reason this tool exists is that they are invisible from the `decider` \
         field alone.\n\n\
         With a slug: that question alone, in full. What it asks, what it is open \
         between, what it unblocks, what references it, and whether anything has \
         answered it.\n\n\
         Nothing here fails. A question waiting on op is the ordinary state of one \
         nobody has asked yet. The exception is a row carrying a `bound` and still \
         naming him, which the schema calls a filing error and which is reported \
         at the top."
    }

    fn run(&self, ctx: &ToolContext<'_>) -> ToolReport {
        let rows = ctx.registry.rows_in(QUESTION);
        if rows.is_empty() {
            return ToolReport::inconclusive(
                "no `question` rows are declared, so nothing is waiting on op and this \
                 says nothing about whether anything should be",
            );
        }
        match ctx.args.first().copied() {
            Some(key) => self.one(ctx, rows, key),
            None => self.batch(ctx, rows),
        }
    }
}

impl UnaskedQuestions {
    fn batch(&self, ctx: &ToolContext<'_>, rows: &[String]) -> ToolReport {
        let his: Vec<&String> = rows
            .iter()
            .filter(|q| ctx.registry.field(q, DECIDER) == Some(HIS))
            .collect();

        let mut open: Vec<(&String, bool, usize)> = Vec::new();
        let mut settled: Vec<&String> = Vec::new();
        let mut misfiled: Vec<&String> = Vec::new();

        for q in &his {
            if ctx.registry.field(q, BOUND).is_some() {
                misfiled.push(q);
            }
            if answered_by(ctx, q).is_empty() {
                let unblocks = ctx
                    .registry
                    .field(q, UNBLOCKS)
                    .is_some_and(|v| !v.trim().is_empty());
                open.push((q, unblocks, ctx.registry.referrers(q).len()));
            } else {
                settled.push(q);
            }
        }

        open.sort_by(|a, b| {
            b.1.cmp(&a.1)
                .then_with(|| b.2.cmp(&a.2))
                .then_with(|| a.0.cmp(b.0))
        });

        let mut s = String::new();

        if !misfiled.is_empty() {
            s.push_str(&format!(
                "{} row(s) carry a `bound` and still name op. The schema calls that a \
                 filing error: a bound is written when a question filed as his is put \
                 to him and handed back, so the decider moved off him at that moment.\n",
                misfiled.len()
            ));
            for q in &misfiled {
                s.push_str(&format!("  {}\n", slug(q)));
            }
            s.push('\n');
        }

        s.push_str(&format!(
            "{} of {} questions name op, and {} of those a ruling has already \
             answered.\n\n",
            his.len(),
            rows.len(),
            settled.len()
        ));

        if open.is_empty() {
            s.push_str(
                "Nothing is waiting on him. Every question that names him has an \
                 answering ruling.\n",
            );
            return ToolReport {
                outcome: Outcome::Clean {
                    examined: rows.len(),
                },
                output: s,
            };
        }

        s.push_str(&format!("{} still waiting on him:\n\n", open.len()));
        for (q, unblocks, refs) in &open {
            let note = match (unblocks, refs) {
                (true, 0) => "unblocks something".to_string(),
                (true, n) => format!("unblocks something, {n} row(s) reference it"),
                (false, 0) => "unblocks nothing named".to_string(),
                (false, n) => format!("unblocks nothing named, {n} row(s) reference it"),
            };
            s.push_str(&format!("  {}   ({note})\n", slug(q)));
        }

        s.push('\n');
        if open.len() >= BATCH {
            s.push_str(&format!(
                "That is past the {BATCH} a round can carry, so a round of asking is \
                 owed. Read each with `unasked-questions <slug>` first.\n\n\
                 Ordering is by whether a row names something it unblocks and then by \
                 what references it, which is a suggestion and not a ranking. Neither \
                 signal knows what a question costs him to answer, and a cheap one \
                 unblocking three topics beats an expensive one unblocking four."
            ));
        } else {
            s.push_str(&format!(
                "Fewer than the {BATCH} a round can carry. Asking one at a time is what \
                 batching exists to stop."
            ));
        }

        if !settled.is_empty() {
            s.push_str("\n\nExcluded, because a ruling answers each:\n");
            for q in &settled {
                let by: Vec<String> = answered_by(ctx, q).iter().map(|r| slug(r)).collect();
                s.push_str(&format!("  {}   <- {}\n", slug(q), by.join(", ")));
            }
        }

        ToolReport {
            outcome: Outcome::Clean {
                examined: rows.len(),
            },
            output: s,
        }
    }

    fn one(&self, ctx: &ToolContext<'_>, rows: &[String], key: &str) -> ToolReport {
        let Some(q) = rows.iter().find(|q| *q == key || slug(q) == key) else {
            return ToolReport::inconclusive(format!(
                "no `question` row matches `{key}`, so this is a statement about the \
                 spelling rather than about the canon. `unasked-questions` with no \
                 argument lists every slug."
            ));
        };

        let decider = ctx.registry.field(q, DECIDER).unwrap_or("(unset)");
        let mut s = format!("{}\n\n  decider: {decider}\n", slug(q));

        let answers = answered_by(ctx, q);
        if !answers.is_empty() {
            s.push_str(&format!(
                "\n  Answered. `{}` settles it, so this is not waiting on anybody and \
                 asking about it would be a repeat.\n",
                answers
                    .iter()
                    .map(|r| slug(r))
                    .collect::<Vec<_>>()
                    .join("`, `")
            ));
        } else if decider != HIS {
            s.push_str(&format!(
                "\n  Not his. Reported in full anyway, because a row put to him that \
                 names `{decider}` is worth catching before it reaches him.\n"
            ));
        }

        if decider == HIS && ctx.registry.field(q, BOUND).is_some() {
            s.push_str(
                "\n  Carries a `bound` and still names him, which the schema calls a \
                 filing error. Read the bound: it records him handing this back.\n",
            );
        }

        for field in ["asks", "options", "unblocks", "bound", "topic", "note"] {
            if let Some(v) = ctx.registry.field(q, field)
                && !v.trim().is_empty()
            {
                s.push_str(&format!("\n  {field}:\n    {}\n", v.trim()));
            }
        }

        let refs = ctx.registry.referrers(q);
        s.push('\n');
        match refs.len() {
            0 => s.push_str(
                "  Nothing references it. Cheapest to be wrong about, and by the \
                 schema's own reading a question that unblocks nothing is one to ask \
                 later or never.\n",
            ),
            n => {
                s.push_str(&format!("  {n} row(s) reference it:\n"));
                for r in refs {
                    s.push_str(&format!("    {r}\n"));
                }
            }
        }

        ToolReport {
            outcome: Outcome::Clean { examined: 1 },
            output: s,
        }
    }
}

/// The rulings that settle this question.
///
/// Read off the engine's typed reverse edges rather than by parsing `answers`.
/// `answers` is the only field on a `ruling` typed to `question`, so a referrer
/// in that namespace can only have arrived through it.
///
/// **A `proposal` also carries `answers` and is deliberately not counted.** The
/// schema is explicit that a proposal does not settle what it names while it is
/// a proposal, so a question with a proposal against it and no ruling is still
/// waiting on him, and that is exactly the case a namespace-blind reading would
/// hide.
fn answered_by(ctx: &ToolContext<'_>, q: &str) -> Vec<String> {
    ctx.registry
        .referrers(q)
        .iter()
        .filter(|r| r.starts_with(&format!("{RULING}::")))
        .cloned()
        .collect()
}

/// The bare slug of a qualified identifier.
fn slug(qualified: &str) -> String {
    qualified
        .rsplit("::")
        .next()
        .unwrap_or(qualified)
        .to_string()
}

#[cfg(test)]
mod tests;

mockspace::lint_pack! {
    tools: [UnaskedQuestions],
}
