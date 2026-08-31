//! What op has not blessed, and what it takes to promote one now that he has
//! stopped blessing things.
//!
//! **The premise this was built on is spent.** It existed to batch rows so he
//! was asked once rather than often, four at a time, which is the workflow he
//! described when he set it up. He has since handed the canon to the panel:
//! *"you don't need me anymore. I've given all I can, the canon should be
//! solvable and fully fillable without me from now on with all that I've already
//! said."* So there is nobody to batch to, no count at which a round of asking
//! is owed, and a row here moves by two experts independently agreeing on it
//! with the coordinator gating that.
//!
//! What survives the change is everything the tool actually does. The population
//! is still worth seeing, still worth ordering by what depends on it, and still
//! worth reading one row at a time before touching it. Only the destination
//! moved.
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
//! **With no argument, the whole population.** Every row awaiting a ruling,
//! ordered by how much is waiting on it, and marked where another row has
//! retired it. What it deliberately does not do is pick which to promote:
//! ordering is a suggestion and the coordinator reads the rows.
//!
//! **Retirement is the only blocked state it can see**, because it is the only
//! one with a field. A row op was asked to bless and declined, and a row
//! deliberately held, both look exactly like ready ones here. Seat 218 found
//! three that must not be promoted and only this one was derivable.
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
    fn waiting(&self, ctx: &ToolContext<'_>, rows: &[String]) -> Vec<(String, usize, Option<String>)> {
        let mut out: Vec<(String, usize, Option<String>)> = rows
            .iter()
            .filter(|q| ctx.registry.field(q, STANDING) == Some(AWAITING))
            .map(|q| (q.clone(), cited_by(ctx, q), retired_by(ctx, q)))
            .collect();
        out.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
        out
    }

    fn batch(&self, waiting: &[(String, usize, Option<String>)], total: usize) -> ToolReport {
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

        for (q, cites, retired) in waiting {
            let slug = q.rsplit("::").next().unwrap_or(q);
            let mark = match retired {
                Some(by) => format!("   RETIRED by {by}, do not promote"),
                None => String::new(),
            };
            match cites {
                0 => s.push_str(&format!("  {slug}{mark}\n")),
                1 => s.push_str(&format!("  {slug}   (1 row cites it){mark}\n")),
                n => s.push_str(&format!("  {slug}   ({n} rows cite it){mark}\n")),
            }
        }

        s.push('\n');
        let retired = waiting.iter().filter(|(_, _, r)| r.is_some()).count();
        if retired > 0 {
            let (count, verb) = match retired {
                1 => ("One".to_string(), "is"),
                n => (n.to_string(), "are"),
            };
            s.push_str(&format!(
                "{count} of those {verb} marked retired above and not a candidate \
                 for anything. They are listed because a row another row has \
                 killed still reads as ready on a flat list, which is how one \
                 gets promoted.\n\n"
            ));
        }
        s.push_str(
            "These are not queued for op. He has handed the canon to the panel, so \
             a row here is promoted by two experts independently agreeing on it and \
             the coordinator gating that, and the batch of four he once named is \
             spent. Read each with `awaiting-a-ruling <slug>` before promoting it: \
             the work is usually the gap between his quote and the prose written on \
             top of it, not the row's subject.\n\n\
             Ordering is by what cites a row, which is a suggestion and not a \
             ranking. And retirement is the only blocked state this can see: a row \
             he was asked to bless and declined, or one deliberately held, looks \
             exactly like a ready one here, because neither has a field to be \
             written in.",
        );

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

        let later = op_files_after(ctx, q);
        if !later.is_empty() {
            s.push_str(
                "\n  Op files that postdate this row. Read them before asking about it:\n",
            );
            for f in &later {
                s.push_str(&format!("    {f}\n"));
            }
            s.push_str(
                "  A later file of his own may already settle this, restate it better, or\n  \
                 supersede it. Asking about a row he has already settled spends the one thing\n  \
                 a batch exists to save.\n",
            );
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

/// The field naming rows this one retires.
const SUPERSEDES: &str = "supersedes";

/// Whether another ruling has retired this row.
///
/// Candidates come from the engine's typed reverse edges rather than from a
/// scan, so a row reaches this one through a declared reference field. The
/// `supersedes` read then says which kind of edge it is, because a referrer may
/// point here through any of several fields and only one of them means dead.
///
/// Worth having because a retired row is indistinguishable from a live one on a
/// flat list, and the list is what a coordinator works top to bottom. Seat 218
/// found three rows on it that must not be promoted, of which this is the state
/// the registry can already answer for itself.
fn retired_by(ctx: &ToolContext<'_>, q: &str) -> Option<String> {
    let slug = q.rsplit("::").next().unwrap_or(q);
    ctx.registry
        .referrers(q)
        .iter()
        .find(|who| {
            ctx.registry
                .field(who, SUPERSEDES)
                .is_some_and(|dead| dead.split(|c: char| !(c.is_alphanumeric() || c == '_')).any(|named| named == slug))
        })
        .map(|who| who.rsplit("::").next().unwrap_or(who).to_string())
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

/// Op's own panel files that postdate the row's own provenance.
///
/// **The check that was missing when four rows he had settled repeatedly were
/// put to him anyway.** His reply: *"Most of these have been well established
/// and blessed by me earlier several times. This should not be something to ask
/// me, I've been clear"*. Panel file `95` is his, carries two of those four in
/// its addendum, and nothing in the tool pointed at it.
///
/// Ordering is by the leading number in the filename, which is the panel's own
/// sequence, against the leading number in the row's `provenance`. A file that
/// predates the row is where the row came from and is not news; one that
/// postdates it may settle, restate or supersede it.
///
/// **This reports rather than gates.** A later file of his often has nothing to
/// do with the row, so a hit is a thing to read and not a refusal, and there is
/// no threshold here to tune.
fn op_files_after(ctx: &ToolContext<'_>, q: &str) -> Vec<String> {
    let Some(after) = ctx
        .registry
        .field(q, "provenance")
        .and_then(panel_number)
    else {
        // No provenance, or none this can order. Reporting every op file would
        // be noise dressed as diligence, so it reports none and says nothing.
        return Vec::new();
    };

    let dir = ctx.mock_dir.join("research");
    let mut out: Vec<(u32, String)> = Vec::new();
    let Ok(panels) = std::fs::read_dir(&dir) else {
        return Vec::new();
    };
    for panel in panels.flatten() {
        let Ok(files) = std::fs::read_dir(panel.path()) else {
            continue;
        };
        for f in files.flatten() {
            let name = f.file_name().to_string_lossy().to_string();
            if !name.ends_with(".md") || !is_op_file(&name) {
                continue;
            }
            if let Some(n) = leading_number(&name)
                && n > after {
                    out.push((n, name));
                }
        }
    }
    out.sort();
    out.into_iter().map(|(_, n)| n).collect()
}

/// The panel-file number a provenance string points at.
///
/// A provenance is `panel::<dir>::<file>::<anchor>`, so the number wanted is the
/// one leading the third segment. Taking the first number in the whole string
/// would take the panel directory's timestamp instead, which is the same for
/// every row and would order nothing.
fn panel_number(provenance: &str) -> Option<u32> {
    provenance
        .split("::")
        .nth(2)
        .and_then(leading_number)
}

fn leading_number(s: &str) -> Option<u32> {
    let digits: String = s.chars().take_while(char::is_ascii_digit).collect();
    digits.parse().ok()
}

/// Whether a panel filename is one of op's own files rather than one about him.
///
/// The convention is `<number>[<letter>]_op_<slug>.md`, so `op` has to be the
/// segment straight after the number. `contains("_op_")` is the obvious spelling
/// and it is wrong: it takes `207_mcsherry_op_material_in_the_dead_panel.md`,
/// which is a member's file *about* his material and carries none of his words.
///
/// That matters more than a tidy listing. This whole check exists to make a
/// short list somebody will actually read, and a list padded with files that are
/// not his is one nobody reads, which puts it back where it started.
fn is_op_file(name: &str) -> bool {
    let rest = name.trim_start_matches(|c: char| c.is_ascii_digit());
    let rest = rest.strip_prefix(|c: char| c.is_ascii_alphabetic()).unwrap_or(rest);
    rest.starts_with("_op_")
}
