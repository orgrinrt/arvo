//! Rulings recording op's authority with no words of his behind them.
//!
//! `says` is required and `quote` is not, which inverts the trust order: a row
//! can carry somebody's restatement of him, pass every schema check, and be
//! mechanically indistinguishable from one that was invented. The first port of
//! this corpus landed four of exactly that shape, and one of them governs when
//! anything becomes canon at all, its only record being an agent's sentence
//! reporting which option he took.
//!
//! # A tool rather than a lint, and the finder's own words say why
//!
//! It was carried across as a lint at first and that was wrong. The check this
//! came from documented itself, in its own voice: **"Not an error in the row.
//! Sometimes the corpus genuinely holds no verbatim, and the row is the best
//! available record of a real call. What the finding says is that the hole is in
//! the corpus and somebody should know where. A row that has a reason states it
//! in `note`, and this reports it anyway, because a note is prose and this is a
//! list."**
//!
//! That is the contract's third question answered: an inventory with no pass
//! line, which is `no-failing-case`. The evidence it was already one is that the
//! check guarding it pinned six row names rather than asserting empty, and a
//! pinned list of names is the invented threshold the contract says is worse
//! than no gate, because people defend numbers. Whether a hole in the corpus can
//! be filled is a judgement about what the corpus holds, which nobody can make
//! from the row.
//!
//! **What is not in scope, by construction.** A `proposal` carries no `quote`,
//! because there are no words but the panel's and `says` holds them, so reading
//! that namespace would report the namespace rather than a hole. And a ruling
//! whose `ratified_by` is `experts` never passed through him: the experts
//! propose, the coordinator gates, and such a row carries a `promotion`
//! recording that judgement, so there is no verbatim to have lost.

use mockspace::tool::{ArgSpec, NotALint, Outcome, Tool, ToolContext, ToolReport};

/// The namespace whose rows claim his authority.
const RULING: &str = "ruling";

/// The ratification route that never passed through him.
const NOT_HIS: &str = "experts";

pub struct RulingsWithNoVerbatim;

impl Tool for RulingsWithNoVerbatim {
    fn name(&self) -> &'static str {
        "rulings-with-no-verbatim"
    }

    fn description(&self) -> &'static str {
        "rulings resting on somebody's restatement of op rather than on his words"
    }

    fn not_a_lint(&self) -> NotALint {
        NotALint::NoFailingCase
    }

    fn args(&self) -> &'static [ArgSpec] {
        &[ArgSpec {
            name:        "slug",
            required:    false,
            description: "report one ruling in full, by its slug, before quoting it",
        }]
    }

    fn help(&self) -> &'static str {
        "With no argument: every `ruling` row that sets `says` and carries no \
         `quote`, so what stands behind it is somebody's restatement of him \
         rather than his words. A row here is not a defect. Sometimes the corpus \
         genuinely holds no verbatim and the row is the best available record of \
         a real call, and the reason for that sits in `note` where a reader has \
         to go and read it.\n\n\
         What the list is for is knowing where the holes are. Two repairs close \
         one: quote the source, or write into `note` that the corpus holds no \
         verbatim and what it holds instead. Which of the two applies is a \
         judgement about what the corpus holds, which is why nothing here \
         gates.\n\n\
         A ruling whose `ratified_by` is `experts` is out of scope: the experts \
         propose and the coordinator gates, he is not in it, and the row carries \
         a `promotion` recording that judgement instead."
    }

    fn run(&self, ctx: &ToolContext<'_>) -> ToolReport {
        let rows = ctx.registry.rows_in(RULING);
        if rows.is_empty() {
            return ToolReport::inconclusive(
                "no `ruling` rows are declared, so this says nothing about whether any \
                 of them rests on his words",
            );
        }
        match ctx.args.first().copied() {
            Some(slug) => self.one(ctx, rows, slug),
            None => self.all(ctx, rows),
        }
    }
}

/// Whether a row rests on somebody's restatement rather than on his words.
pub fn has_no_verbatim(ctx: &ToolContext<'_>, q: &str) -> bool {
    if ctx.registry.field(q, "ratified_by") == Some(NOT_HIS) {
        return false;
    }
    ctx.registry
        .field(q, "quote")
        .is_none_or(|v| v.trim().is_empty())
}

impl RulingsWithNoVerbatim {
    fn all(&self, ctx: &ToolContext<'_>, rows: &[String]) -> ToolReport {
        let holes: Vec<&String> = rows.iter().filter(|q| has_no_verbatim(ctx, q)).collect();
        let total = rows.len();
        if holes.is_empty() {
            return ToolReport::reported(
                format!("every one of the {total} rulings carries his words."),
                total,
            );
        }
        let mut s = format!(
            "{} of {total} rulings rest on somebody's restatement of him.\n\n",
            holes.len()
        );
        for q in &holes {
            let slug = q.rsplit("::").next().unwrap_or(q);
            let reason = ctx
                .registry
                .field(q, "note")
                .map(str::trim)
                .filter(|n| !n.is_empty())
                .map(|n| n.chars().take(90).collect::<String>());
            match reason {
                Some(n) => s.push_str(&format!("  {slug}\n      note: {n}\n")),
                None => s.push_str(&format!("  {slug}\n      no note either\n")),
            }
        }
        s.push_str(
            "\nA row here is not a defect and nothing fails. It says the hole is in the \
             corpus and where to look. Two repairs close one: quote the source, or write \
             into `note` that the corpus holds no verbatim and what it holds instead. The \
             ones already carrying a note are the ones somebody has looked at.\n",
        );
        ToolReport {
            outcome: Outcome::Clean { examined: total },
            output:  s,
        }
    }

    fn one(&self, ctx: &ToolContext<'_>, rows: &[String], key: &str) -> ToolReport {
        let Some(q) = rows
            .iter()
            .find(|q| *q == key || q.rsplit("::").next() == Some(key))
        else {
            return ToolReport::inconclusive(format!(
                "no `ruling` row matches `{key}`, so this is a statement about the spelling \
                 rather than about the canon. `rulings-with-no-verbatim` with no argument \
                 lists every slug."
            ));
        };
        let mut s = format!("{}\n", q.rsplit("::").next().unwrap_or(q));
        s.push_str(&format!(
            "\n  rests on his words: {}\n",
            if has_no_verbatim(ctx, q) { "no" } else { "yes" }
        ));
        for field in ["rung", "ratified_by", "says", "quote", "note", "provenance"] {
            if let Some(v) = ctx.registry.field(q, field)
                && !v.trim().is_empty()
            {
                s.push_str(&format!("\n  {field}:\n    {}\n", v.trim()));
            }
        }
        ToolReport {
            outcome: Outcome::Clean { examined: 1 },
            output:  s,
        }
    }
}

#[cfg(test)]
mod tests;

mockspace::lint_pack! {
    tools: [RulingsWithNoVerbatim],
}
