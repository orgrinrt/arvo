//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! Rendering, and the one thing the parse cannot reach: a position written in a
//! design rather than in source.

use std::collections::BTreeMap;

use crate::kinds::{family, Carrier, Found, Position};
use crate::role;

/// Positions written in a design document's fenced Rust.
///
/// **Two trees here carry designs and no source at all**, and every tree here
/// carries designs describing surface that is not written yet. A walk over `.rs`
/// answers those at zero, and the obligation covers a position that "sits **or
/// would sit**", so a zero there would be the half of the demand nobody has
/// built yet, reported as absent.
///
/// # Parsed, after a line scan was tried and was wrong
///
/// The first version of this read fenced lines and classified them by what they
/// started with. It was wrong in the way a line scan is always wrong, and the
/// corpus showed it immediately: `pub struct PoolFrame<const MAX_CORES: usize,
/// const MAX_PHASES: usize>` came back as two struct fields, when both are const
/// generic parameters and op excepted that position by name. `pub trait
/// ByteEmitter: Push<u8> + BulkPush<u8> {}` came back as two more, when both are
/// type arguments in a supertrait bound.
///
/// So a fence is handed to the same walk the source gets, and the classification
/// is the same one. tree-sitter recovers from a fragment rather than refusing it,
/// which is what makes this work on the half of a design's fences that are not
/// whole items.
///
/// # What it still cannot do
///
/// **Visibility is not judged.** Everything a design writes in a fence is the
/// surface it is describing, and a fragment usually carries no `pub` because the
/// prose around it already said so. So every position from a fence is taken as
/// public, which over-counts a design that shows a private helper and is the
/// direction that reports more demand rather than less.
pub fn design_positions(tree: &str, path: &str, text: &str) -> Vec<Found> {
    let mut out = Vec::new();
    for (first_line, block) in rust_fences(text) {
        for mut row in crate::walk::walk(tree, path, &block) {
            row.line += first_line;
            row.owner = if row.owner.is_empty() {
                "<design>".to_string()
            } else {
                row.owner.clone()
            };
            row.public = true;
            row.shipped = true;
            out.push(row);
        }
    }
    out
}

/// Every Rust fence in a markdown document, with the line its body starts on.
///
/// An unlabelled fence counts as Rust. In this corpus that is what they are, and
/// the alternative is dropping the design positions in every document whose
/// author did not label a block.
fn rust_fences(text: &str) -> Vec<(usize, String)> {
    let mut out = Vec::new();
    // Two pieces of state, not one. Tracking only the rust fences means the
    // closing marker of a `toml` block reads as an opening marker with an empty
    // language, which counts as rust here, and from there every fence in the
    // document is inverted. That is not hypothetical: it swallowed the one
    // unlabelled fence in the suite's own fixture and the suite caught it.
    let mut inside = false;
    let mut is_rust = false;
    let mut first_line = 0usize;
    let mut body: Vec<&str> = Vec::new();

    for (idx, line) in text.lines().enumerate() {
        let trimmed = line.trim_start();
        if trimmed.starts_with("```") {
            if inside {
                if is_rust {
                    out.push((first_line, body.join("\n")));
                }
                inside = false;
                is_rust = false;
                body.clear();
            } else {
                let lang = trimmed.trim_start_matches('`').trim();
                inside = true;
                is_rust = lang.is_empty()
                    || lang.starts_with("rust")
                    || lang.starts_with("rs")
                    || lang.starts_with("ignore")
                    || lang.starts_with("no_run");
                first_line = idx + 1;
            }
            continue;
        }
        if inside && is_rust {
            body.push(line);
        }
    }
    // An unterminated fence at the end of a document is still a fence.
    if inside && is_rust {
        out.push((first_line, body.join("\n")));
    }
    out
}

/// The whole report.
pub fn render(
    found: &[Found],
    heads: &[(String, String, usize)],
    designs: &[(String, usize, usize)],
    api_only: bool,
    everything: bool,
    want_kind: Option<&str>,
    want_role: Option<&str>,
) -> String {
    let kept: Vec<&Found> = found
        .iter()
        .filter(|f| !api_only || f.is_demand())
        .collect();

    let mut s = String::new();
    s.push_str("the corpus\n");
    for (label, head, files) in heads {
        s.push_str(&format!("  {label:<44} {head}  ({files} rust files)\n"));
    }
    for (label, files, hits) in designs {
        if *files > 0 {
            s.push_str(&format!(
                "  {label:<44} {files} design documents, {hits} fenced positions\n"
            ));
        }
    }
    s.push_str(&format!(
        "\n{} occurrences in all, {} of them at a public API position.\n",
        found.len(),
        found.iter().filter(|f| f.is_demand()).count()
    ));
    if api_only {
        s.push_str("Reporting the public API positions alone.\n");
    }

    // A listing is over the demand set unless asked otherwise. The tool exists
    // to answer what arvo owes, and a listing that opened with several thousand
    // probe interiors buried the answer under the corpus.
    let listable: Vec<&Found> = if everything {
        kept.clone()
    } else {
        kept.iter()
            .copied()
            .filter(|f| f.is_demand())
            .filter(|f| f.carrier.is_a_number())
            .filter(|f| matches!(family(&f.primitive), Some("numeric" | "truth")))
            .collect()
    };
    let scope = if everything {
        "every occurrence"
    } else {
        "the demand"
    };
    if let Some(kind) = want_kind {
        return listing(
            &listable,
            |f| f.position.token() == kind,
            &format!("kind `{kind}`, over {scope}"),
        );
    }
    if let Some(want) = want_role {
        return listing(
            &listable,
            |f| role::of(&f.name, &f.owner, &f.primitive) == want,
            &format!("role `{want}`, over {scope}"),
        );
    }

    // The composition, first and unconditionally. A corpus is what decides the
    // answer, and the first run of this tool was over one where 87 percent of
    // the files were panel probes. Nothing in the totals said so.
    s.push_str("\nwhere the occurrences are, by directory\n");
    s.push_str("  `shipped` is a crate's own `src/`. Everything else is a test, an example,\n  a bench variant or a research probe, and is public API to nobody.\n\n");
    let mut where_from: BTreeMap<String, (usize, usize)> = BTreeMap::new();
    for row in found {
        let key: String = row.path.split('/').take(3).collect::<Vec<_>>().join("/");
        let slot = where_from.entry(format!("{} {key}", row.tree)).or_default();
        slot.0 += 1;
        if row.shipped {
            slot.1 += 1;
        }
    }
    let mut ranked: Vec<(&String, &(usize, usize))> = where_from.iter().collect();
    ranked.sort_by(|a, b| b.1 .0.cmp(&a.1 .0));
    s.push_str(&format!(
        "  {:<62} {:>7} {:>8}\n",
        "tree and directory", "all", "shipped"
    ));
    for (key, (all, shipped)) in ranked.iter().take(14) {
        s.push_str(&format!("  {key:<62} {all:>7} {shipped:>8}\n"));
    }
    if ranked.len() > 14 {
        let rest: usize = ranked.iter().skip(14).map(|(_, v)| v.0).sum();
        let rest_shipped: usize = ranked.iter().skip(14).map(|(_, v)| v.1).sum();
        s.push_str(&format!(
            "  {:<62} {rest:>7} {rest_shipped:>8}\n",
            format!("({} more directories)", ranked.len() - 14)
        ));
    }

    s.push_str("\nby grammatical kind\n");
    s.push_str("  every kind is listed, including the ones at zero, because a kind\n  nobody found is a result about the stack rather than about the walk.\n\n");
    s.push_str(&format!(
        "  {:<22} {:>7} {:>7}  {}\n",
        "kind", "all", "shipped", "an outside crate writes this type"
    ));
    for position in Position::all() {
        let all = kept.iter().filter(|f| f.position == *position).count();
        let public = kept
            .iter()
            .filter(|f| f.position == *position && f.public && f.shipped)
            .count();
        s.push_str(&format!(
            "  {:<22} {all:>7} {public:>7}  {}\n",
            position.token(),
            if position.is_api() { "yes" } else { "no" }
        ));
    }

    s.push_str("\nby carrier, over the public API positions\n");
    s.push_str("  what the primitive is wrapped in, off the parse rather than off a name.\n  a pointer target and a slice element are the unit of memory or of a byte\n  string; no numeral replaces either, so they are the positions arvo is not\n  the answer to.\n\n");
    let api_pre: Vec<&&Found> = kept.iter().filter(|f| f.is_demand()).collect();
    s.push_str(&format!(
        "  {:<12} {:>7}  {}\n",
        "carrier", "count", "a number is what is meant"
    ));
    for carrier in Carrier::all() {
        let n = api_pre.iter().filter(|f| f.carrier == *carrier).count();
        s.push_str(&format!(
            "  {:<12} {n:>7}  {}\n",
            carrier.token(),
            if carrier.is_a_number() { "yes" } else { "no" }
        ));
    }

    s.push_str("\nby semantic role, over the public API positions\n");
    s.push_str("  a reading off the identifier, not a measurement. `--role <name>`\n  lists one in full so it can be checked.\n\n");
    let api: Vec<&&Found> = kept.iter().filter(|f| f.is_demand()).collect();
    s.push_str(&format!(
        "  {:<16} {:>7}  {}\n",
        "role", "count", "the primitives it is written in"
    ));
    for name in role::all() {
        let rows: Vec<&&&Found> = api
            .iter()
            .filter(|f| role::of(&f.name, &f.owner, &f.primitive) == name)
            .collect();
        let mut prims: BTreeMap<&str, usize> = BTreeMap::new();
        for row in &rows {
            *prims.entry(row.primitive.as_str()).or_default() += 1;
        }
        let spelling: Vec<String> = prims.iter().map(|(p, n)| format!("{p}x{n}")).collect();
        s.push_str(&format!(
            "  {:<16} {:>7}  {}\n",
            name,
            rows.len(),
            spelling.join(" ")
        ));
    }

    // The cross-tabulation, which is what separates the demand from the total.
    // Neither axis alone says it: `&str` is a reference and is out of the
    // obligation's wording, `*mut u8` is in the wording and is not a number.
    s.push_str("\nfamily against carrier, over the public API positions\n\n");
    s.push_str(&format!("  {:<12}", "carrier"));
    for fam in ["numeric", "truth", "textual"] {
        s.push_str(&format!(" {fam:>9}"));
    }
    s.push_str("     a number\n");
    for carrier in Carrier::all() {
        s.push_str(&format!("  {:<12}", carrier.token()));
        for fam in ["numeric", "truth", "textual"] {
            let n = api_pre
                .iter()
                .filter(|f| f.carrier == *carrier && family(&f.primitive) == Some(fam))
                .count();
            s.push_str(&format!(" {n:>9}"));
        }
        s.push_str(&format!(
            "     {}\n",
            if carrier.is_a_number() { "yes" } else { "no" }
        ));
    }

    // What the obligation is actually over, with every filter applied and each
    // one named. A number with no predicate is a number nobody can check.
    let demand: Vec<&&&Found> = api
        .iter()
        .filter(|f| f.carrier.is_a_number())
        .filter(|f| matches!(family(&f.primitive), Some("numeric" | "truth")))
        .collect();
    s.push_str("\nthe demand\n");
    for line in [
        "a position is one arvo owes a primitive for when all five hold: an outside",
        "crate writes the type, the item is reachable, the file is a crate's `src/`,",
        "the primitive is a number or a truth rather than text, and it is not the",
        "unit of a pointer or a slice. Each is reported above on its own, so the",
        "subtraction can be checked rather than taken.",
    ] {
        s.push_str(&format!("  {line}\n"));
    }
    s.push('\n');
    s.push_str(&format!("  {} positions\n\n", demand.len()));
    let mut by_role: BTreeMap<&str, usize> = BTreeMap::new();
    for row in &demand {
        *by_role
            .entry(role::of(&row.name, &row.owner, &row.primitive))
            .or_default() += 1;
    }
    let mut ranked_roles: Vec<(&&str, &usize)> = by_role.iter().collect();
    ranked_roles.sort_by(|a, b| b.1.cmp(a.1));
    for (name, count) in ranked_roles {
        let prims: BTreeMap<&str, usize> = demand
            .iter()
            .filter(|f| role::of(&f.name, &f.owner, &f.primitive) == *name)
            .fold(BTreeMap::new(), |mut m, f| {
                *m.entry(f.primitive.as_str()).or_default() += 1;
                m
            });
        let spelling: Vec<String> = prims.iter().map(|(p, n)| format!("{p}x{n}")).collect();
        s.push_str(&format!(
            "  {:<16} {count:>5}  {}\n",
            name,
            spelling.join(" ")
        ));
    }

    // The other half of the fraction. A count of host primitives says nothing
    // about how far the obligation is from met without the count of positions
    // that already went through the stack.
    s.push_str("\nthe supply, at the same bar\n");
    for line in [
        "the same walk over the same positions, keeping the stack's own type names",
        "instead of the host's. The list is hand-written and is printed below, so a",
        "name missing from it undercounts this side and makes the obligation look",
        "further from met than it is.",
    ] {
        s.push_str(&format!("  {line}\n"));
    }
    s.push('\n');
    let supply: Vec<&&Found> = kept.iter().filter(|f| f.is_supply()).collect();
    let mut by_supplier: BTreeMap<&str, usize> = BTreeMap::new();
    for row in &supply {
        *by_supplier.entry(row.supplier.unwrap_or("?")).or_default() += 1;
    }
    for (who, count) in &by_supplier {
        s.push_str(&format!("  {who:<16} {count:>5}\n"));
    }
    let host = demand.len();
    let gone = by_supplier.get("gone").copied().unwrap_or(0);
    let live: usize = by_supplier
        .iter()
        .filter(|(k, _)| **k != "gone")
        .map(|(_, v)| *v)
        .sum();
    let total = host + gone + live;
    if total > 0 {
        s.push('\n');
        for line in [
            format!("{host} host, {gone} naming an arvo crate or type that is not on"),
            format!(
                "arvo's `dev`, and {live} resolving. So {}% of the positions carrying",
                host * 100 / total
            ),
            format!(
                "any of the three still name the host, and a further {}%",
                gone * 100 / total
            ),
            "name arvo and reach nothing. Counting the second group as supply, which".to_string(),
            "one undivided arvo list does, is what makes the obligation look nearly".to_string(),
            "met when half of what meets it does not exist.".to_string(),
        ] {
            s.push_str(&format!("  {line}\n"));
        }
    }
    s.push_str("\n  which names are counted as the stack's:\n");
    let mut seen: BTreeMap<&str, usize> = BTreeMap::new();
    for row in &supply {
        *seen.entry(row.primitive.as_str()).or_default() += 1;
    }
    let used: Vec<String> = seen.iter().map(|(n, c)| format!("{n}x{c}")).collect();
    s.push_str(&format!("    in use   {}\n", used.join(" ")));
    let unused: Vec<&str> = crate::supply::every_name()
        .into_iter()
        .filter(|n| !seen.contains_key(n))
        .collect();
    s.push_str(&format!(
        "    on the list and reaching nothing   {}\n",
        unused.join(" ")
    ));

    s.push_str("\nby primitive family, over the public API positions\n\n");
    let mut fams: BTreeMap<&str, usize> = BTreeMap::new();
    for row in &api {
        if let Some(f) = family(&row.primitive) {
            *fams.entry(f).or_default() += 1;
        }
    }
    for (name, count) in &fams {
        s.push_str(&format!("  {name:<16} {count:>7}\n"));
    }

    s.push_str("\nby tree, over the public API positions\n\n");
    let mut trees: BTreeMap<&str, usize> = BTreeMap::new();
    for row in &api {
        *trees.entry(row.tree.as_str()).or_default() += 1;
    }
    for (name, count) in &trees {
        s.push_str(&format!("  {name:<44} {count:>7}\n"));
    }

    s
}

fn listing(kept: &[&Found], keep: impl Fn(&Found) -> bool, what: &str) -> String {
    let rows: Vec<&&Found> = kept.iter().filter(|f| keep(f)).collect();
    let mut s = format!("{} positions at {what}\n\n", rows.len());
    for row in &rows {
        s.push_str(&format!(
            "  {}:{}:{}  {:<20} {:<10} {} {}\n",
            row.tree,
            row.path,
            row.line,
            row.position.token(),
            row.primitive,
            if row.owner.is_empty() {
                "-"
            } else {
                row.owner.as_str()
            },
            if row.name.is_empty() {
                "-"
            } else {
                row.name.as_str()
            },
        ));
    }
    s
}
