//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! Rendering, and the one thing the parse cannot reach: a position written in a
//! design rather than in source.

use std::collections::BTreeMap;

use crate::kinds::{family, Found, Position};
use crate::role;

/// Positions written in a design template's fenced Rust.
///
/// **Two consumers here have designs and no source at all**, so a walk over
/// `.rs` alone answers those at zero and calls it clean, which is the shape of
/// reading the demand side wrong. A fenced signature is a position that would
/// sit, which is exactly what the obligation says it covers.
///
/// Line-oriented rather than parsed, deliberately: a design's fence is often a
/// fragment that does not parse as an item, and a parser that recovers from that
/// reports the fragment's shape rather than its author's. What is looked for is
/// narrow and stated: a host primitive inside a fenced block, on a line that
/// carries a signature marker.
pub fn design_positions(tree: &str, path: &str, text: &str) -> Vec<Found> {
    let mut out = Vec::new();
    let mut in_fence = false;
    let mut rust_fence = false;
    for (idx, line) in text.lines().enumerate() {
        let trimmed = line.trim_start();
        if trimmed.starts_with("```") {
            if in_fence {
                in_fence = false;
                rust_fence = false;
            } else {
                in_fence = true;
                let lang = trimmed.trim_start_matches('`').trim();
                rust_fence = lang.is_empty() || lang.starts_with("rust") || lang.starts_with("rs");
            }
            continue;
        }
        if !in_fence || !rust_fence {
            continue;
        }
        let is_signature = trimmed.starts_with("pub fn ")
            || trimmed.starts_with("fn ")
            || trimmed.starts_with("pub const ")
            || trimmed.starts_with("const ")
            || trimmed.starts_with("pub type ")
            || trimmed.starts_with("type ")
            || trimmed.starts_with("pub ")
            || trimmed.contains("-> ")
            || trimmed.contains(": ");
        if !is_signature {
            continue;
        }
        let scan = strip_after_line_comment(trimmed);
        for prim in every_primitive(&scan) {
            out.push(Found {
                tree: tree.to_string(),
                path: path.to_string(),
                line: idx + 1,
                position: if trimmed.starts_with("pub const ") || trimmed.starts_with("const ") {
                    Position::TraitConst
                } else if trimmed.contains("-> ") {
                    Position::FnReturn
                } else if trimmed.starts_with("pub fn ") || trimmed.starts_with("fn ") {
                    Position::FnParam
                } else {
                    Position::StructField
                },
                primitive: prim,
                name: declared_name(&scan),
                owner: "<design>".to_string(),
                public: true,
                shipped: true,
            });
        }
    }
    out
}

fn strip_after_line_comment(s: &str) -> String {
    match s.find("//") {
        Some(i) => s[..i].to_string(),
        None => s.to_string(),
    }
}

/// Every host primitive name occurring as a whole token in a line.
fn every_primitive(line: &str) -> Vec<String> {
    let mut out = Vec::new();
    let bytes = line.as_bytes();
    let mut i = 0usize;
    while i < bytes.len() {
        if !bytes[i].is_ascii_alphanumeric() && bytes[i] != b'_' {
            i += 1;
            continue;
        }
        let start = i;
        while i < bytes.len() && (bytes[i].is_ascii_alphanumeric() || bytes[i] == b'_') {
            i += 1;
        }
        let word = &line[start..i];
        // A token preceded by a digit or an underscore is a literal suffix or
        // part of a longer name, and neither is a position.
        let prev_ok = start == 0 || !matches!(bytes[start - 1], b'0'..=b'9' | b'_');
        if prev_ok && family(word).is_some() {
            out.push(word.to_string());
        }
    }
    out
}

/// The identifier a fenced signature line declares, for the role reading.
fn declared_name(line: &str) -> String {
    for kw in [
        "pub fn ",
        "fn ",
        "pub const ",
        "const ",
        "pub type ",
        "type ",
    ] {
        if let Some(rest) = line.trim_start().strip_prefix(kw) {
            let name: String = rest
                .chars()
                .take_while(|c| c.is_ascii_alphanumeric() || *c == '_')
                .collect();
            if !name.is_empty() {
                return name;
            }
        }
    }
    // A field line: `name: Type,`
    if let Some(colon) = line.find(':') {
        let head = line[..colon].trim().trim_start_matches("pub ").trim();
        if !head.is_empty() && head.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') {
            return head.to_string();
        }
    }
    String::new()
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
        kept.iter().copied().filter(|f| f.is_demand()).collect()
    };
    let scope = if everything {
        "every occurrence"
    } else {
        "public API positions"
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
