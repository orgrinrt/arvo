//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! What reaches each obligation, which is the bar the canon is measured against.
//!
//! Op's condition for reviewing the canon is that it be exhaustive enough that a
//! full design and then a full implementation can be done from it. The
//! obligations are the demand side written from outside the canon, so what
//! reaches each one is the only measurement of that bar that the canon cannot
//! make come out right by agreeing with itself.
//!
//! **It lived in a shell probe whose own header called it a net rather than a
//! test**, and its figures were relayed upward three times, twice to op, one of
//! them wrong. A net is the right instrument for finding candidates by keyword
//! and the wrong one for a number: it matched prose and so counted a row that
//! mentioned an obligation as a row that answered one. This walks the typed
//! edges instead, which is what the schema put there.

use std::collections::BTreeMap;

use crate::{Finding, Registry};

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
    ///
    /// **Not met.** The obligation file says so in as many words: a proposal is
    /// proposed rather than met, and reporting it otherwise closes a gap op has
    /// never seen.
    Proposed,
    /// Only retirements name it: a route toward it was tried and closed.
    ///
    /// Strictly worse than `Proposed` and strictly better than `Nothing`, and it
    /// is the tier the `retirement.obligation` field exists to make visible. The
    /// obligation is open, and one way to it is known not to work.
    RouteClosed,
    /// Nothing names it at all.
    Nothing,
}

impl Reach {
    /// The word used in a report, and in the ceiling test's message.
    pub fn word(self) -> &'static str {
        match self {
            | Self::Met => "met",
            | Self::Proposed => "proposed",
            | Self::RouteClosed => "route-closed",
            | Self::Nothing => "nothing",
        }
    }
}

/// The namespaces carrying a typed `obligation` field, and what a row there means.
///
/// Read off the schema rather than guessed: a namespace gaining the field and not
/// appearing here would be silently ignored, which is why the arm below reports
/// exactly that case.
const EDGES: &[(&str, Reach)] = &[
    ("ruling", Reach::Met),
    ("proposal", Reach::Proposed),
    ("retirement", Reach::RouteClosed),
];

/// Every obligation with how far it has got and what got it there.
pub fn reach(reg: &Registry) -> BTreeMap<String, (Reach, Vec<String>)> {
    let mut out: BTreeMap<String, (Reach, Vec<String>)> = reg
        .of("obligation")
        .map(|r| (r.id.clone(), (Reach::Nothing, Vec::new())))
        .collect();

    for (ns, tier) in EDGES {
        for row in reg.of(ns) {
            for slug in row.list("obligation") {
                let Some(entry) = out.get_mut(slug.as_str()) else {
                    continue; // a slug naming no obligation is the arm below
                };
                entry.0 = entry.0.min(*tier);
                entry.1.push(row.addr());
            }
        }
    }
    out
}

/// An `obligation` edge naming no obligation row.
///
/// The engine reports this too, so the arm is here for the tests rather than for
/// the gate: a rename or a typo has to fail somewhere the suite can see it, and
/// a check that only the engine performs is one the suite cannot plant a control
/// for.
pub fn obligation_edges_naming_nothing(reg: &Registry) -> Vec<Finding> {
    let known: Vec<&str> = reg.slugs("obligation");
    let mut out = Vec::new();
    for (ns, _) in EDGES {
        for row in reg.of(ns) {
            for slug in row.list("obligation") {
                if !known.contains(&slug.as_str()) {
                    out.push(Finding::new(
                        "obligation-edge-names-nothing",
                        row.addr(),
                        format!(
                            "names obligation `{slug}`, which is not a row. An obligation slug is \
                             an address and landed prose cites it, so a rename orphans every \
                             citation to it and this edge with them."
                        ),
                    ));
                }
            }
        }
    }
    out
}

/// A namespace carrying an `obligation` edge that the tiering above does not know.
///
/// **The control on the instrument itself.** `reach` partitions by namespace from
/// a fixed list, so a namespace gaining the field would contribute nothing and
/// the coverage would read better than it is, silently and in the flattering
/// direction. This is the case that has to fail.
pub fn obligation_edges_from_an_untiered_namespace(reg: &Registry) -> Vec<Finding> {
    let mut out = Vec::new();
    for row in &reg.rows {
        if !row.has("obligation") {
            continue;
        }
        if EDGES.iter().any(|(ns, _)| *ns == row.namespace) {
            continue;
        }
        out.push(Finding::new(
            "obligation-edge-from-an-untiered-namespace",
            row.addr(),
            format!(
                "carries an `obligation` edge from `{}`, which the coverage measurement does not \
                 tier. It would be counted as nothing at all, so coverage would read better than \
                 it is. Decide what a row there means for an obligation and add it.",
                row.namespace
            ),
        ));
    }
    out
}

/// Obligations reached only by a retirement, reported so they are not read as open-and-untouched.
///
/// Not a defect and not a finding against anybody. It is the distinction the
/// field was added for, surfaced where a reader will meet it.
pub fn obligations_whose_only_route_is_closed(reg: &Registry) -> Vec<Finding> {
    reach(reg)
        .into_iter()
        .filter(|(_, (tier, _))| *tier == Reach::RouteClosed)
        .map(|(id, (_, by))| {
            Finding::new(
                "obligation-reached-only-by-a-closed-route",
                format!("obligation::{id}"),
                format!(
                    "is named by {} retirement(s) and by nothing that could answer it: {}. The \
                     obligation is open and one way to it is known not to work, which is not the \
                     same as nobody having looked.",
                    by.len(),
                    by.join(", ")
                ),
            )
        })
        .collect()
}

/// How many obligations sit at each tier.
pub fn tally(reg: &Registry) -> BTreeMap<&'static str, usize> {
    let mut out = BTreeMap::new();
    for tier in [Reach::Met, Reach::Proposed, Reach::RouteClosed, Reach::Nothing] {
        out.insert(tier.word(), 0);
    }
    for (_, (tier, _)) in reach(reg) {
        *out.entry(tier.word()).or_insert(0) += 1;
    }
    out
}
