//! What op has not blessed yet, batched so he is asked once rather than often.
//!
//! The `ruling` schema already draws the line this reports on. `ratified` needs
//! both halves, the experts converging and him blessing that convergence.
//! `stated` is his direction and an ack rather than a ruling. `open` is where he
//! has explicitly not settled it. So a `stated` row is somebody's record of
//! something he said, standing in for a ruling it is not, and the gap between
//! the two is invisible to every reader who does not open the row.
//!
//! Op, setting this up: *"all of it is a statement rather than a full ruling
//! until you run them by me. Which should be a workflow in itself: Anything that
//! is listed as a statement but not confirmed and blessed as a ruling from me,
//! some tool should list those and you should run them by me with ask user tool
//! when there's enough to make it a batch of more than 4."*
//!
//! **A tool rather than a lint.** A `stated` row is not a defect. It is the
//! normal resting state of a thing he said once and has not been asked to rule
//! on, and most of the corpus will sit there for a long time legitimately.
//! There is no failing case to gate on, and picking a count that blocks would
//! invent a deadline nobody set. What the tool does is make the population
//! visible and ordered, so the asking happens on purpose rather than whenever
//! somebody happens to open the file.
//!
//! # What it reports
//!
//! **With no argument, the batch.** Every row awaiting a ruling, ordered by how
//! much is waiting on it, with the count and whether that count has passed the
//! four op named. What it deliberately does not do is pick which four to ask:
//! ordering is a suggestion and the dispatcher reads the rows.
//!
//! **With a key, one row in full**, which is what you read before putting it to
//! him: his words if the row carries them, what the row says on top of those
//! words, and what cites it, because a row three others depend on is a different
//! question from one nothing has been built on.

use mockspace::tool::{ArgSpec, NotALint, Outcome, Tool, ToolContext, ToolReport};

/// The namespace carrying op's statements.
const RULING: &str = "ruling";

/// The field naming how far a row has got. Its own name is a problem: `rung` is
/// on the workspace's banned list, which says to name the position outright
/// instead. Renaming a schema field is a registry migration rather than this
/// tool's business, so it is read as spelled and reported here instead.
const STANDING: &str = "rung";

/// The value meaning he said it and has not ruled on it.
const AWAITING: &str = "stated";

/// The batch size op named. Below it, hold; at or above, ask.
///
/// Not a threshold this tool enforces. `AskUserQuestion` takes four questions at
/// most, so a batch is asked in rounds and this is the point at which a round is
/// worth spending rather than a point at which anything fails.
const BATCH: usize = 4;

pub struct AwaitingARuling;

impl Tool for AwaitingARuling {
    fn name(&self) -> &'static str {
        "awaiting-a-ruling"
    }

    fn description(&self) -> &'static str {
        "rows op stated but never ruled on, and whether there are enough to ask"
    }

    fn not_a_lint(&self) -> NotALint {
        NotALint::NoFailingCase
    }

    fn args(&self) -> &'static [ArgSpec] {
        &[ArgSpec {
            name: "key",
            required: false,
            description: "report one row in full, by its slug, before putting it to him",
        }]
    }

    fn help(&self) -> &'static str {
        "With no argument: every `ruling` row still at `stated`, ordered by how \
         many other rows cite it, so the ones carrying the most weight are asked \
         first. A `stated` row is his direction recorded as an ack; it is not a \
         ruling and nothing downstream may treat it as one.\n\n\
         With a slug: that row alone, in full. His verbatim if the row carries \
         it, the derived prose written on top of it, and what cites it. Read this \
         before asking, because the question to put to him is usually the gap \
         between the quote and the rest of the row rather than the row's own \
         subject.\n\n\
         The count is a prompt, not a gate. Nothing here fails, and a row may sit \
         at `stated` indefinitely without that being wrong."
    }

    fn run(&self, ctx: &ToolContext<'_>) -> ToolReport {
        let rows = ctx.registry.rows_in(RULING);
        if rows.is_empty() {
            return ToolReport::inconclusive(
                "no `ruling` rows are declared, so nothing is awaiting one and this \
                 says nothing about whether op has been asked",
            );
        }

        let waiting = self.waiting(ctx, rows);

        match ctx.args.first().copied() {
            Some(key) => self.one(ctx, rows, key),
            None => self.batch(&waiting, rows.len()),
        }
    }
}

impl AwaitingARuling {
    /// Every row still at `stated`, with how many other rows cite it.
    ///
    /// Ordered by that count, descending, then by slug so the output is stable
    /// across runs and a diff of two reports means something.
    fn waiting(&self, ctx: &ToolContext<'_>, rows: &[String]) -> Vec<(String, usize)> {
        let mut out: Vec<(String, usize)> = rows
            .iter()
            .filter(|q| ctx.registry.field(q, STANDING) == Some(AWAITING))
            .map(|q| (q.clone(), cited_by(ctx, q)))
            .collect();
        out.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
        out
    }

    fn batch(&self, waiting: &[(String, usize)], total: usize) -> ToolReport {
        if waiting.is_empty() {
            return ToolReport::reported(
                format!(
                    "nothing is awaiting a ruling. all {total} rows are ratified, in \
                     force, or explicitly open."
                ),
                total,
            );
        }

        let mut s = String::new();
        s.push_str(&format!(
            "{} of {} rows are his direction recorded as an ack, not a ruling.\n\n",
            waiting.len(),
            total
        ));

        for (q, cites) in waiting {
            let slug = q.rsplit("::").next().unwrap_or(q);
            match cites {
                0 => s.push_str(&format!("  {slug}\n")),
                1 => s.push_str(&format!("  {slug}   (1 row cites it)\n")),
                n => s.push_str(&format!("  {slug}   ({n} rows cite it)\n")),
            }
        }

        s.push('\n');
        if waiting.len() >= BATCH {
            s.push_str(&format!(
                "That is past the {BATCH} op named, so a round of asking is owed. \
                 Read each with `awaiting-a-ruling <slug>` first: the question is \
                 usually the gap between his quote and the prose written on top of \
                 it, not the row's subject.\n\n\
                 Ordering is by what cites a row, which is a suggestion. Nothing \
                 here says the top four are the right four."
            ));
        } else {
            s.push_str(&format!(
                "Fewer than the {BATCH} op named, so these hold until more \
                 accumulate. Asking one at a time is what the batch exists to \
                 stop."
            ));
        }

        ToolReport {
            outcome: Outcome::Clean { examined: total },
            output: s,
        }
    }

    fn one(&self, ctx: &ToolContext<'_>, rows: &[String], key: &str) -> ToolReport {
        let Some(q) = rows
            .iter()
            .find(|q| *q == key || q.rsplit("::").next() == Some(key))
        else {
            return ToolReport::inconclusive(format!(
                "no `ruling` row matches `{key}`, so this is a statement about the \
                 spelling rather than about the canon. `awaiting-a-ruling` with no \
                 argument lists every slug."
            ));
        };

        let standing = ctx.registry.field(q, STANDING).unwrap_or("(unset)");
        let mut s = format!(
            "{}\n\n  standing: {standing}\n",
            q.rsplit("::").next().unwrap_or(q)
        );

        if standing != AWAITING {
            s.push_str(
                "\n  Not awaiting a ruling. Reported in full anyway, because asking \
                 about a row that is already settled is worth catching before it \
                 reaches him.\n",
            );
        }

        for field in [
            "topic", "quote", "says", "because", "instead", "note", "gap",
        ] {
            if let Some(v) = ctx.registry.field(q, field)
                && !v.trim().is_empty() {
                    s.push_str(&format!("\n  {field}:\n    {}\n", v.trim()));
                }
        }

        let citers = citers(ctx, q);
        s.push('\n');
        match citers.len() {
            0 => s.push_str(
                "  Nothing cites it. Cheapest to ask about and cheapest to be wrong \
                 about, because nothing has been built on it yet.\n",
            ),
            n => {
                s.push_str(&format!(
                    "  {n} row(s) cite it, and inherit whatever it is wrong about:\n"
                ));
                for c in &citers {
                    s.push_str(&format!("    {c}\n"));
                }
            }
        }

        ToolReport {
            outcome: Outcome::Clean { examined: 1 },
            output: s,
        }
    }
}

/// How many rows depend on this one through a typed field.
fn cited_by(ctx: &ToolContext<'_>, q: &str) -> usize {
    ctx.registry.referrers(q).len()
}

/// Which rows depend on this one, by qualified key.
///
/// The engine's own reverse edges, which are typed: a row reaches this one
/// through a declared reference field rather than by happening to spell its slug
/// in prose. Substring matching over `note` and `gap` was the first shape of
/// this and it is wrong in both directions, since a slug that is a common phrase
/// matches rows that do not depend on it, and a row referencing this one under a
/// different spelling is missed entirely.
fn citers(ctx: &ToolContext<'_>, q: &str) -> Vec<String> {
    ctx.registry.referrers(q).to_vec()
}

#[cfg(test)]
mod tests;

mockspace::lint_pack! {
    tools: [AwaitingARuling],
}
