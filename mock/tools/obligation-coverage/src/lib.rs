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
//! # Authority is the rung, and the namespace is not a proxy for it
//!
//! The namespace a row sits in says what kind of claim it is. **It does not say
//! whether the claim governs**, and whether it governs is the whole of what a
//! coverage tier is about.
//!
//! `ruling` carries four rungs and one of them is canon. The schema's own
//! words: `ratified` where it is canon, `in_force` where this repo's lints
//! enforce it independently of convergence, `stated` where it is his direction
//! and an ack rather than a ruling, `open` where he has explicitly not settled
//! it. **A tier taken from the namespace reads all four as met**, so op's
//! standing instruction that a stated entry is not to be written as clear cut
//! and settled is broken by the instrument that reports on it.
//!
//! It is wrong in the other direction too. `proposal.obligation` is documented
//! as the obligations a proposal would meet **if it were stamped**, and
//! `ruling.ratifies` is the stamp. So a proposal a ratified ruling names there
//! has been stamped, and its obligation edges carry that ruling's authority.
//! Reading the naming namespace alone files it as proposed forever.
//!
//! **The registry has already recorded this class of miss on the neighbouring
//! field.** `question::adaptation_in_identity_or_realisation` says of its own
//! answer that nothing was decided to close it, that the edge was already in
//! the registry and nothing followed it, because it runs `ruling.ratifies` into
//! `proposal.answers` and every reader of the settled set read `ruling.answers`
//! alone. The same unfollowed hop, one field over.
//!
//! **Both defects were live on the committed canon when this was written.** The
//! single row the namespace tiering called met was reached by a ruling at
//! `rung = "stated"` whose own note records op declining to bless it, so no
//! obligation was reached by a ratified ruling at all; and the one obligation a
//! ratified ruling does reach, through the stamp, was filed as proposed.
//!
//! # The ladder, and the one comparison the canon does not make
//!
//! Strongest first: `ratified`, `in_force`, `stated`, `proposed`, `unsettled`,
//! `route-closed`, `nothing`. Four are the schema's own words for a rung. The
//! other three stand over something the schema spells differently or does not
//! spell at all, and `Reach::word` says which is which per word.
//!
//! `ratified` over everything is the provenance ladder rather than a reading: a
//! ruling at that rung governs and nothing else does. `stated` over `proposed`
//! is the ladder again, op's own direction over an agent's claim.
//!
//! **`in_force` over `stated` is this file's reading and two shipped things
//! here disagree about it.** The schema lists the four values in that order and
//! declares no ranking over them, so the list is not the argument it reads as.
//! `mock/lints/no_ruling_supersedes_one_that_outranks_it.rs:58` ranks the same
//! two the other way round, weakest first as `open`, `in_force`, `stated`,
//! `ratified`, on the ground that `in_force` is a process call nobody stamped.
//! Deciding between them is a canon reading and takes two independent
//! agreements; until somebody does that, this is a call rather than a
//! derivation and is marked as one.
//!
//! **`proposed` over `unsettled` is this file's reading and nothing in the
//! canon makes it.** The ladder is how far an obligation has got toward being
//! met; a proposal is argued content that would meet it once stamped, and a
//! ruling at `open` is op declining to settle, which contributes none. Nothing
//! rests on it today, since no ruling at `open` names an obligation. It is one
//! line to move if the reading is wrong.
//!
//! **`unsettled` also holds a ruling whose rung this cannot read**, which is
//! the pessimistic direction on purpose: a rung the tool does not know must
//! never read stronger than it is. The rung is printed verbatim beside the row
//! either way, so `open` and unreadable are distinguishable on the line even
//! though they share a tier.
//!
//! # The three things it reports and why none of them is arithmetic
//!
//! **The tier each obligation has reached**, from the typed edges and the rung
//! together, with the row that got it there and, where that row is a stamped
//! proposal, the ruling that stamped it. The two-hop path is printed rather
//! than collapsed, because `ratified` on a proposal-sourced line claims a
//! ruling's authority and a reader has to be able to see which ruling.
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

/// The namespace carrying `rung` and `ratifies`.
const RULING: &str = "ruling";

/// The namespace a ruling stamps.
const PROPOSAL: &str = "proposal";

/// The namespace recording a route tried and closed.
const RETIREMENT: &str = "retirement";

/// The one rung at which a ruling governs and at which its stamp is a stamp.
const RATIFIED: &str = "ratified";

/// What is printed for a ruling carrying no readable rung.
///
/// `rung` is `required = true` in the schema, so this should be unreachable on
/// a loaded registry. It is rendered rather than assumed away because the tier
/// it produces is the weakest one, and a reader owed an explanation of why a
/// row landed there is owed the reason rather than a blank.
const NO_RUNG: &str = "(absent)";

/// What kind of row an obligation edge came from.
///
/// A named kind rather than a tier beside the namespace, because a `ruling`
/// contributes its rung and a `proposal` contributes whether a ratified ruling
/// stamped it, and neither of those is a property of the namespace. Keeping it
/// an enum makes the walk's match exhaustive, so a namespace added to `EDGES`
/// cannot be tiered by a fallback arm nobody looked at.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Edge {
    Ruling,
    Proposal,
    Retirement,
}

/// The namespaces carrying a typed `obligation` field.
///
/// Read off the schema rather than guessed, and the same three the
/// `an-obligation-edge-comes-from-a-tiered-namespace` lint refuses anything
/// outside of. A namespace gaining the field and not appearing here would be
/// silently ignored and the coverage would read better than it is, which is
/// why that lint exists and why the two lists are kept in the same order.
const EDGES: [(&str, Edge); 3] = [
    (RULING, Edge::Ruling),
    (PROPOSAL, Edge::Proposal),
    (RETIREMENT, Edge::Retirement),
];

/// The namespaces that can establish a precondition, and therefore carry the
/// edge.
const PRECONDITION_SOURCES: [&str; 2] = ["law", "proposal"];

/// How far an obligation has got, from the typed edges and the rung.
///
/// The order is the ranking: a later variant is never reported where an earlier
/// one holds. What each position rests on is in the module documentation, ladder
/// section, including the one comparison nothing in the canon makes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Reach {
    /// A ruling at `rung = "ratified"` names it, or a proposal such a ruling
    /// stamped names it. The only tier that is met.
    Ratified,
    /// A ruling at `rung = "in_force"` names it. Enforced by this repo's lints
    /// independently of convergence, which is not the ratification route.
    InForce,
    /// A ruling at `rung = "stated"` names it. His direction, and an ack rather
    /// than a ruling, so it binds without being canon.
    Stated,
    /// A proposal nothing has stamped names it. Proposed rather than met, which
    /// is the line the schema draws in as many words.
    Proposed,
    /// A ruling names it at `rung = "open"`, where he has explicitly not settled
    /// it, or at a rung this cannot read. Neither settles anything.
    Unsettled,
    /// Only retirements name it: a route toward it was tried and closed.
    RouteClosed,
    /// Nothing names it at all.
    Nothing,
}

impl Reach {
    /// The word used in a report.
    ///
    /// Four of the seven are `rung` values spelled as the schema spells them, so
    /// a reader holding a report can grep `mockspace.toml` for the word and land
    /// on the field description that defines it.
    ///
    /// **Three cannot be grepped for and each names what it stands over.**
    /// `unsettled` stands over the rung the schema spells `open`, plus any rung
    /// this cannot read at all, which is why it is not simply spelled `open`.
    /// `route-closed` stands over the namespace the schema spells `retirement`,
    /// and keeps the corpus's word rather than the schema's because the tier was
    /// named in `mock/research/202608072330_the-numeral-canon-panel/` and a
    /// report that stops using it stops matching the research it mechanises.
    /// `nothing` is this file's own word for absence and no schema word stands
    /// behind it.
    pub fn word(self) -> &'static str {
        match self {
            Self::Ratified => "ratified",
            Self::InForce => "in_force",
            Self::Stated => "stated",
            Self::Proposed => "proposed",
            Self::Unsettled => "unsettled",
            Self::RouteClosed => "route-closed",
            Self::Nothing => "nothing",
        }
    }

    /// Whether anything constructive reaches it.
    ///
    /// A match over every tier rather than a chain of not-equals, and the
    /// difference is not cosmetic: the not-equals form named the two tiers to
    /// exclude, so a tier added later joined the unanswered side silently and
    /// an obligation that had just gained an answer would have been reported as
    /// having none.
    pub fn answered(self) -> bool {
        match self {
            Self::Ratified | Self::InForce | Self::Stated | Self::Proposed => true,
            Self::Unsettled | Self::RouteClosed | Self::Nothing => false,
        }
    }
}

/// Every tier, strongest first, in one place.
const TIERS: [Reach; 7] = [
    Reach::Ratified,
    Reach::InForce,
    Reach::Stated,
    Reach::Proposed,
    Reach::Unsettled,
    Reach::RouteClosed,
    Reach::Nothing,
];

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

/// A ruling's rung as written, or `(absent)`.
fn rung<'a>(reg: &'a RegistryView, q: &str) -> &'a str {
    reg.field(q, "rung")
        .map(str::trim)
        .filter(|r| !r.is_empty())
        .unwrap_or(NO_RUNG)
}

/// Which proposals a ratified ruling has stamped, and which ruling stamped each.
///
/// Keyed by the proposal's slug, because `ratifies` holds slugs. **The rung is
/// checked here rather than taken on trust**: a stamp from anything below
/// `ratified` is a hard error at the gate, under
/// `an-unratified-ruling-stamps-a-proposal`, and a measurement that assumed the
/// gate had run would report the proposal as canon on exactly the row the gate
/// exists to catch.
pub fn stamps(reg: &RegistryView) -> BTreeMap<String, Vec<String>> {
    let mut out: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for q in reg.rows_in(RULING) {
        if rung(reg, q) != RATIFIED {
            continue;
        }
        for named in list(reg, q, "ratifies") {
            out.entry(named.to_string()).or_default().push(q.clone());
        }
    }
    out
}

/// Every obligation with how far it has got and what got it there.
///
/// The second half of each entry is what the report prints under the tier: the
/// qualified row, plus the rung for a ruling and the stamping ruling for a
/// stamped proposal.
pub fn reach(reg: &RegistryView) -> BTreeMap<String, (Reach, Vec<String>)> {
    let stamped = stamps(reg);
    let mut out: BTreeMap<String, (Reach, Vec<String>)> = reg
        .rows_in(OBLIGATION)
        .iter()
        .map(|q| (slug(q).to_string(), (Reach::Nothing, Vec::new())))
        .collect();

    for (ns, edge) in EDGES {
        for q in reg.rows_in(ns) {
            let (tier, by) = match edge {
                Edge::Ruling => {
                    let r = rung(reg, q);
                    let tier = match r {
                        RATIFIED => Reach::Ratified,
                        "in_force" => Reach::InForce,
                        "stated" => Reach::Stated,
                        _ => Reach::Unsettled,
                    };
                    (tier, format!("{q}   (rung = {r})"))
                }
                Edge::Proposal => match stamped.get(slug(q)) {
                    Some(by) => (
                        Reach::Ratified,
                        format!("{q}   (stamped by {})", by.join(", ")),
                    ),
                    None => (Reach::Proposed, q.clone()),
                },
                Edge::Retirement => (Reach::RouteClosed, q.clone()),
            };
            for named in list(reg, q, OBLIGATION) {
                let Some(entry) = out.get_mut(named) else {
                    continue; // a slug naming no obligation is a lint's report
                };
                entry.0 = entry.0.min(tier);
                entry.1.push(by.clone());
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
    for tier in TIERS {
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
        "what reaches each obligation, by the rung that reaches it, with the preconditions against it"
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
        "With no argument: every obligation, by the tier the typed edges and the \
         rung put it at, with the rows that got it there.\n\n\
         The tier is the authority of what reaches it, never the namespace the \
         row sits in. `ratified` is the one tier that is met: a ruling at that \
         rung governs, and so does a proposal such a ruling stamped through \
         `ratifies`, which is what a stamp is for. `in_force` is enforced by \
         this repo's lints without having gone through convergence. `stated` is \
         his direction and an ack rather than a ruling. `proposed` is a \
         proposal nobody has stamped, which is proposed rather than met. \
         `unsettled` is a ruling at `open`, where he explicitly did not settle \
         it, or one whose rung could not be read. `route-closed` means only a \
         retirement names it: a way to it was tried and is known not to work, \
         which is not the same as nobody having looked.\n\n\
         A ruling's rung is printed beside it and a stamped proposal names the \
         ruling that stamped it, so a `ratified` line reached through the stamp \
         can be checked rather than taken.\n\n\
         Preconditions are reported beside the tiers and never folded into \
         them. A precondition is a dependency somebody established, so it \
         leaves an obligation further from met rather than nearer, and an \
         obligation with four of them and no answer is the worst-placed row \
         here rather than the best-attended one.\n\n\
         Nothing here fails. An unanswered obligation is the state of \
         unfinished work rather than a defect, and gating on a count would \
         invent a deadline nobody set. What is a defect is an edge naming no \
         row, an edge from a namespace the tiering does not know, a \
         precondition from a namespace that cannot establish one, and a stamp \
         from a ruling that is not ratified: four lints refuse those at every \
         gate."
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
        for tier in TIERS {
            s.push_str(&format!(
                "  {:<13} {}\n",
                tier.word(),
                counts.get(tier.word()).copied().unwrap_or(0)
            ));
        }
        s.push_str(
            "\n`ratified` is the only tier that is met. The rest are degrees of not yet, \
             ordered by\nhow far each has got, and a ruling's rung is printed beside it.\n",
        );

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
            .filter(|(_, (tier, _))| !tier.answered())
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
