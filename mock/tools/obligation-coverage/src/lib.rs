//! What reaches each obligation, which is the bar the canon is measured against.
//!
//! Op's condition for reviewing the canon is that it be exhaustive enough that a
//! full design and then a full implementation can be done from it. The
//! obligations are the demand side written from outside the canon, so what
//! reaches each one is the only measurement of that bar the canon cannot make
//! come out right by agreeing with itself.
//!
//! **It lived in a shell probe whose own header called it a net rather than a
//! test**, and its figures were relayed upward three times, twice to op, one of
//! them wrong. A net is the right instrument for finding candidates by keyword
//! and the wrong one for a number: it matched prose, so it counted a row that
//! mentioned an obligation as a row that answered one. This walks the typed
//! edges instead, which is what the schema put there.
//!
//! # A tool rather than a lint, and the four questions say which
//!
//! It needs no argument from a person, so it is not `takes-a-question`. There is
//! a state it should refuse, three of them, and each is a lint of its own:
//! `an-obligation-edge-names-a-row`,
//! `an-obligation-edge-comes-from-a-tiered-namespace` and
//! `a-precondition-comes-from-a-namespace-that-can-establish-one`. What is left
//! after those is the coverage itself, and **there is no pass line on it.**
//!
//! An obligation nothing answers is not a defect: it is the state of unfinished
//! work, and most of the population sits there legitimately while the canon is
//! being written. Gating on a count would invent a deadline nobody set, and an
//! invented threshold is worse than no gate because people defend numbers. So
//! this is `no-failing-case`, and what it does is make the population visible
//! and ordered so the reading happens on purpose rather than whenever somebody
//! opens the file.
//!
//! # The three things it reports and why none of them is arithmetic
//!
//! **The tier each obligation has reached**, from the typed edges alone. A
//! ruling names it and op has been in the loop, which is the only tier that is
//! an answer. A proposal names it and op has not seen that proposal, which the
//! obligation file says in as many words is proposed rather than met. A
//! retirement names it and nothing else does, which means a route was tried and
//! closed: strictly worse than proposed and strictly better than nothing, and
//! the distinction the `retirement.obligation` field exists to make visible.
//!
//! **Preconditions, counted separately and never folded into the tier.** A
//! precondition says the obligation has a dependency, which leaves it further
//! from met rather than nearer. The arithmetic temptation is real: an obligation
//! with four known preconditions looks better attended than one with none, and
//! it is worse off.
//!
//! **The pair a reader most needs**, which the registry could not express before
//! the field existed: an obligation answered by nothing that also carries an
//! established precondition.

use std::collections::BTreeMap;

use mockspace::RegistryView;
use mockspace::tool::{ArgSpec, NotALint, Outcome, Tool, ToolContext, ToolReport};

/// The namespace holding the demand side.
const OBLIGATION: &str = "obligation";

/// How far an obligation has got, from the typed edges alone.
///
/// The order is the ranking: a later variant is never reported where an earlier
/// one holds.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Reach {
    /// A ruling names it. Op has been in the loop, so this is the only tier that
    /// is an answer.
    Met,
    /// A proposal names it, and op has not seen that proposal.
    Proposed,
    /// Only retirements name it: a route toward it was tried and closed.
    RouteClosed,
    /// Nothing names it at all.
    Nothing,
}

impl Reach {
    /// The word used in a report.
    pub fn word(self) -> &'static str {
        match self {
            Self::Met => "met",
            Self::Proposed => "proposed",
            Self::RouteClosed => "route-closed",
            Self::Nothing => "nothing",
        }
    }
}

/// The namespaces carrying a typed `obligation` field, and what a row there
/// means.
///
/// Read off the schema rather than guessed. A namespace gaining the field and
/// not appearing here would be silently ignored and the coverage would read
/// better than it is, which is why a lint of its own refuses exactly that case.
const EDGES: [(&str, Reach); 3] = [
    ("ruling", Reach::Met),
    ("proposal", Reach::Proposed),
    ("retirement", Reach::RouteClosed),
];

/// The namespaces that can establish a precondition, and therefore carry the
/// edge.
const PRECONDITION_SOURCES: [&str; 2] = ["law", "proposal"];

/// The slug half of a `namespace::slug`.
fn slug(qualified: &str) -> &str {
    qualified.rsplit("::").next().unwrap_or(qualified)
}

/// The entries of a list field.
///
/// The engine joins a `string[]` with `", "` before a tool sees it, so the split
/// is that separator. Every field read here holds slugs, which carry no comma,
/// so nothing is lost.
fn list<'a>(reg: &'a RegistryView, q: &str, field: &str) -> Vec<&'a str> {
    reg.field(q, field)
        .map(|v| {
            v.split(", ")
                .map(str::trim)
                .filter(|e| !e.is_empty())
                .collect()
        })
        .unwrap_or_default()
}

/// Every obligation with how far it has got and what got it there.
pub fn reach(reg: &RegistryView) -> BTreeMap<String, (Reach, Vec<String>)> {
    let mut out: BTreeMap<String, (Reach, Vec<String>)> = reg
        .rows_in(OBLIGATION)
        .iter()
        .map(|q| (slug(q).to_string(), (Reach::Nothing, Vec::new())))
        .collect();

    for (ns, tier) in EDGES {
        for q in reg.rows_in(ns) {
            for named in list(reg, q, OBLIGATION) {
                let Some(entry) = out.get_mut(named) else {
                    continue; // a slug naming no obligation is a lint's report
                };
                entry.0 = entry.0.min(tier);
                entry.1.push(q.clone());
            }
        }
    }
    out
}

/// Preconditions somebody has established for each obligation.
///
/// **Never a tier and never counted as coverage.**
pub fn preconditions(reg: &RegistryView) -> BTreeMap<String, Vec<String>> {
    let mut out: BTreeMap<String, Vec<String>> = reg
        .rows_in(OBLIGATION)
        .iter()
        .map(|q| (slug(q).to_string(), Vec::new()))
        .collect();
    for ns in PRECONDITION_SOURCES {
        for q in reg.rows_in(ns) {
            for named in list(reg, q, "precondition_for") {
                if let Some(entry) = out.get_mut(named) {
                    entry.push(q.clone());
                }
            }
        }
    }
    out
}

/// How many obligations sit at each tier.
pub fn tally(reg: &RegistryView) -> BTreeMap<&'static str, usize> {
    let mut out = BTreeMap::new();
    for tier in [
        Reach::Met,
        Reach::Proposed,
        Reach::RouteClosed,
        Reach::Nothing,
    ] {
        out.insert(tier.word(), 0);
    }
    for (_, (tier, _)) in reach(reg) {
        *out.entry(tier.word()).or_insert(0) += 1;
    }
    out
}

pub struct ObligationCoverage;

impl Tool for ObligationCoverage {
    fn name(&self) -> &'static str {
        "obligation-coverage"
    }

    fn description(&self) -> &'static str {
        "what reaches each obligation, by tier, with the preconditions against it"
    }

    fn not_a_lint(&self) -> NotALint {
        NotALint::NoFailingCase
    }

    fn args(&self) -> &'static [ArgSpec] {
        &[ArgSpec {
            name: "slug",
            required: false,
            description: "report one obligation in full, by its slug",
        }]
    }

    fn help(&self) -> &'static str {
        "With no argument: every obligation, by the tier the typed edges put it \
         at, with the rows that got it there. Met means a ruling names it and op \
         has been in the loop. Proposed means a proposal names it and he has \
         not seen that proposal, which is proposed rather than met. \
         Route-closed means only a retirement names it: a way to it was tried \
         and is known not to work, which is not the same as nobody having \
         looked.\n\n\
         Preconditions are reported beside the tiers and never folded into \
         them. A precondition is a dependency somebody established, so it \
         leaves an obligation further from met rather than nearer, and an \
         obligation with four of them and no answer is the worst-placed row \
         here rather than the best-attended one.\n\n\
         Nothing here fails. An unanswered obligation is the state of \
         unfinished work rather than a defect, and gating on a count would \
         invent a deadline nobody set. What is a defect is an edge naming no \
         row, an edge from a namespace the tiering does not know, and a \
         precondition from a namespace that cannot establish one: three lints \
         refuse those at every gate."
    }

    fn run(&self, ctx: &ToolContext<'_>) -> ToolReport {
        let rows = ctx.registry.rows_in(OBLIGATION);
        if rows.is_empty() {
            return ToolReport::inconclusive(
                "no `obligation` rows are declared, so there is no demand side to \
                 measure against and this says nothing about whether the canon is \
                 exhaustive",
            );
        }
        match ctx.args.first().copied() {
            Some(slug) => self.one(ctx, slug),
            None => self.all(ctx),
        }
    }
}

impl ObligationCoverage {
    fn all(&self, ctx: &ToolContext<'_>) -> ToolReport {
        let reached = reach(ctx.registry);
        let pre = preconditions(ctx.registry);
        let counts = tally(ctx.registry);
        let total = reached.len();

        let mut s = format!("{total} obligations.\n\n");
        for tier in [
            Reach::Met,
            Reach::Proposed,
            Reach::RouteClosed,
            Reach::Nothing,
        ] {
            s.push_str(&format!(
                "  {:<13} {}\n",
                tier.word(),
                counts.get(tier.word()).copied().unwrap_or(0)
            ));
        }

        s.push_str("\nBy obligation, weakest first:\n\n");
        let mut ordered: Vec<(&String, &(Reach, Vec<String>))> = reached.iter().collect();
        ordered.sort_by(|a, b| b.1.0.cmp(&a.1.0).then_with(|| a.0.cmp(b.0)));
        for (id, (tier, by)) in ordered {
            let deps = pre.get(id).map_or(0, Vec::len);
            let mark = match deps {
                0 => String::new(),
                1 => "   (1 precondition against it)".to_string(),
                n => format!("   ({n} preconditions against it)"),
            };
            s.push_str(&format!("  {:<13} {id}{mark}\n", tier.word()));
            for who in by {
                s.push_str(&format!("                  {who}\n"));
            }
        }

        let closed: Vec<&String> = reached
            .iter()
            .filter(|(_, (tier, _))| *tier == Reach::RouteClosed)
            .map(|(id, _)| id)
            .collect();
        if !closed.is_empty() {
            s.push_str(&format!(
                "\n{} obligation(s) are named only by a retirement: {closed:?}. The \
                 obligation is open and one way to it is known not to work, which is not \
                 the same as nobody having looked, and reads identically on a flat list.\n",
                closed.len()
            ));
        }

        let stuck: Vec<&String> = reached
            .iter()
            .filter(|(_, (tier, _))| *tier != Reach::Met && *tier != Reach::Proposed)
            .filter(|(id, _)| pre.get(*id).is_some_and(|on| !on.is_empty()))
            .map(|(id, _)| id)
            .collect();
        if !stuck.is_empty() {
            s.push_str(&format!(
                "\n{} obligation(s) are answered by nothing and carry an established \
                 precondition: {stuck:?}. Each is further from met than an obligation \
                 nobody has looked at, rather than nearer.\n",
                stuck.len()
            ));
        }

        ToolReport {
            outcome: Outcome::Clean { examined: total },
            output: s,
        }
    }

    fn one(&self, ctx: &ToolContext<'_>, wanted: &str) -> ToolReport {
        let reached = reach(ctx.registry);
        let Some((tier, by)) = reached.get(wanted) else {
            return ToolReport::inconclusive(format!(
                "no `obligation` row matches `{wanted}`, so this is a statement about the \
                 spelling rather than about the canon. `obligation-coverage` with no \
                 argument lists every slug."
            ));
        };
        let pre = preconditions(ctx.registry);
        let mut s = format!("{wanted}\n\n  tier: {}\n", tier.word());
        let q = format!("{OBLIGATION}::{wanted}");
        for field in ["what", "says", "why", "note"] {
            if let Some(v) = ctx.registry.field(&q, field)
                && !v.trim().is_empty()
            {
                s.push_str(&format!("\n  {field}:\n    {}\n", v.trim()));
            }
        }
        s.push('\n');
        match by.len() {
            0 => s.push_str("  Nothing names it.\n"),
            _ => {
                s.push_str("  Named by:\n");
                for who in by {
                    s.push_str(&format!("    {who}\n"));
                }
            }
        }
        if let Some(on) = pre.get(wanted)
            && !on.is_empty()
        {
            s.push_str(&format!(
                "\n  {} established precondition(s), which leave it further from met \
                 rather than nearer:\n",
                on.len()
            ));
            for who in on {
                s.push_str(&format!("    {who}\n"));
            }
        }
        ToolReport {
            outcome: Outcome::Clean { examined: 1 },
            output: s,
        }
    }
}

#[cfg(test)]
mod tests;

mockspace::lint_pack! {
    tools: [ObligationCoverage],
}
