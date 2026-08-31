//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! Seat 219's reply instrument: what seat 220's material does to seat 219's numbers.
//!
//! Written after reading `220_orchard_the_marker_spelling.md`, which is why it
//! is a separate file rather than an edit to `warrant_census.rs`. The census is
//! the blind record and stays as committed, defects included; this is the
//! correction, and keeping them apart is the difference between a corrected
//! measurement and a rewritten one.
//!
//! R1  My census called a span a universal when it equals `any` or ends in
//!     ` any`. 220 quotes two committed spans of the form
//!     `threads any, <clause>`, which are universals with a trailing prose
//!     note and which my rule cannot see. How many did I miss?
//!
//! R2  My multi-binding arm splits clauses on a comma at brace depth zero. A
//!     span joining two bindings with a bare ` and ` has no comma. How many did
//!     I miss?
//!
//! R3  220's section 3 tabulates the eight `sentence_kind = "theorem"` rows
//!     against their width spans, and that table is its load-bearing empirical
//!     claim. Reproduced here from a separately written reader, because a claim
//!     two instruments agree on is worth more than a claim I checked by eye.
//!
//! R4  220's section 10 says the parameterised whole domain
//!     `fraction_width: in 0..=W-1` has "five instances". Its own committed
//!     probe `p1_predicate_census.out` line 102 says three. Which is it.
//!
//! Run:
//!     rustc -O reply_to_220.rs -o /tmp/r220 && /tmp/r220 <registry-dir>
//!
//! With no argument it runs the planted control alone.

use std::collections::BTreeMap;
use std::env;
use std::fs;

const PREDICATE_FIELDS: &[&str] = &["predicate", "holds", "fails"];

#[derive(Default, Clone)]
struct Row {
    namespace: String,
    id: String,
    sentence_kind: String,
    entries: Vec<String>,
}

fn scan(text: &str) -> Vec<Row> {
    let mut rows: Vec<Row> = Vec::new();
    let mut in_array = false;
    for raw in text.lines() {
        let line = raw.trim();
        if in_array {
            if line.starts_with(']') {
                in_array = false;
            } else if let Some(v) = quoted(line) {
                if let Some(r) = rows.last_mut() {
                    r.entries.push(v);
                }
            }
            continue;
        }
        if line.starts_with("[[") && line.ends_with("]]") {
            rows.push(Row {
                namespace: line[2..line.len() - 2].to_string(),
                ..Row::default()
            });
            continue;
        }
        if rows.is_empty() {
            continue;
        }
        if let Some(rest) = line.strip_prefix("id = ") {
            if let Some(v) = quoted(rest) {
                rows.last_mut().unwrap().id = v;
            }
            continue;
        }
        if let Some(rest) = line.strip_prefix("sentence_kind = ") {
            if let Some(v) = quoted(rest) {
                rows.last_mut().unwrap().sentence_kind = v;
            }
            continue;
        }
        for field in PREDICATE_FIELDS {
            let head = format!("{field} = [");
            if let Some(rest) = line.strip_prefix(&head) {
                if line.contains(']') {
                    let body = rest.rsplit_once(']').map(|(a, _)| a).unwrap_or(rest);
                    for part in split_inline(body) {
                        rows.last_mut().unwrap().entries.push(part);
                    }
                } else {
                    in_array = true;
                }
                break;
            }
        }
    }
    rows
}

fn quoted(s: &str) -> Option<String> {
    let start = s.find('"')? + 1;
    let end = s[start..].find('"')? + start;
    Some(s[start..end].to_string())
}

fn split_inline(body: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut rest = body;
    while let Some(start) = rest.find('"') {
        let after = &rest[start + 1..];
        let Some(end) = after.find('"') else { break };
        out.push(after[..end].to_string());
        rest = &after[end + 1..];
    }
    out
}

fn split_entry(entry: &str) -> Option<(&str, &str)> {
    let (a, b) = entry.split_once(':')?;
    Some((a.trim(), b.trim()))
}

/// The rule `warrant_census.rs` shipped with.
fn is_any_strict(values: &str) -> bool {
    values == "any" || values.ends_with(" any")
}

/// The corrected rule. A universal followed by a prose note is still a
/// universal, so the token is looked for at a clause boundary rather than only
/// at the end of the string.
///
/// `anywhere` is the trap and is why this is not a substring test: one committed
/// span reads `no feature gates anywhere, main arm at exit 0`, which contains
/// ` any` and asserts no universal at all.
fn is_any_loose(values: &str) -> bool {
    clauses(values).iter().any(|c| has_any_token(c))
}

/// The universal token, matched as a whole word.
///
/// **Not a suffix test and not a substring test, and the control enforces both.**
/// A suffix test misses `threads any for the compile-time artifacts`, which is a
/// real committed span and is a universal. A substring test counts
/// `no feature gates anywhere`, which is a real committed span and is not.
/// Splitting on whitespace and comparing the whole token is the only rule that
/// gets both right, and I did not have it in the census.
fn has_any_token(clause: &str) -> bool {
    clause.split_whitespace().any(|t| t.trim_matches(',') == "any")
}

/// Clause boundaries: a comma at brace depth zero, or a bare ` and `.
///
/// The ` and ` arm is the one my census lacked. Two committed spans join two
/// bindings on one axis with `and` and no comma at all.
fn clauses(values: &str) -> Vec<String> {
    let chars: Vec<char> = values.chars().collect();
    let mut depth = 0usize;
    let mut start = 0usize;
    let mut parts: Vec<String> = Vec::new();
    for (i, c) in chars.iter().enumerate() {
        match c {
            '{' => depth += 1,
            '}' => depth = depth.saturating_sub(1),
            ',' if depth == 0 => {
                parts.push(chars[start..i].iter().collect());
                start = i + 1;
            },
            _ => {},
        }
    }
    parts.push(chars[start..].iter().collect());

    let mut out = Vec::new();
    for p in parts {
        let mut rest = p.as_str();
        while let Some(idx) = rest.find(" and ") {
            out.push(rest[..idx].trim().to_string());
            rest = &rest[idx + 5..];
        }
        out.push(rest.trim().to_string());
    }
    out.into_iter().filter(|c| !c.is_empty()).collect()
}

fn binds(clause: &str) -> String {
    for sep in [" = ", " in ", " <= ", " >= ", " > ", " < "] {
        if let Some((lhs, _)) = clause.split_once(sep) {
            return lhs.trim().to_string();
        }
    }
    // A universal binds its axis with no separator at all, which is the case
    // the census's splitter could not see.
    if has_any_token(clause) {
        let lhs: Vec<&str> = clause
            .split_whitespace()
            .take_while(|t| t.trim_matches(',') != "any")
            .collect();
        return lhs.join(" ").trim().to_string();
    }
    String::new()
}

struct Out {
    strict: usize,
    loose: usize,
    missed_any: Vec<String>,
    multi_comma: usize,
    multi_with_and: usize,
    missed_multi: Vec<String>,
    theorem: Vec<(String, String)>,
    param_domain: BTreeMap<String, usize>,
}

fn run(label: &str, rows: &[Row]) -> Out {
    let mut o = Out {
        strict: 0,
        loose: 0,
        missed_any: Vec::new(),
        multi_comma: 0,
        multi_with_and: 0,
        missed_multi: Vec::new(),
        theorem: Vec::new(),
        param_domain: BTreeMap::new(),
    };

    for row in rows {
        let mut width = "ABSENT".to_string();
        for entry in &row.entries {
            let Some((slug, values)) = split_entry(entry) else {
                continue;
            };
            let addr = format!("{}::{}", row.namespace, row.id);

            if slug == "total_width" {
                width = values.to_string();
            }
            if slug == "fraction_width" && (values.contains("..=W") || values.contains("..=k")) {
                *o.param_domain.entry(values.to_string()).or_default() += 1;
            }

            let s = is_any_strict(values);
            let l = is_any_loose(values);
            if s {
                o.strict += 1;
            }
            if l {
                o.loose += 1;
            }
            if l && !s {
                o.missed_any.push(format!("{addr} [{slug}] => {values}"));
            }

            // R2. Comma-only clause splitting against comma-or-`and`.
            let comma_only: Vec<String> = {
                let chars: Vec<char> = values.chars().collect();
                let mut depth = 0usize;
                let mut start = 0usize;
                let mut parts = Vec::new();
                for (i, c) in chars.iter().enumerate() {
                    match c {
                        '{' => depth += 1,
                        '}' => depth = depth.saturating_sub(1),
                        ',' if depth == 0 => {
                            parts.push(chars[start..i].iter().collect::<String>());
                            start = i + 1;
                        },
                        _ => {},
                    }
                }
                parts.push(chars[start..].iter().collect());
                parts
            };
            let n_comma = comma_only.iter().filter(|c| !binds(c).is_empty()).count();
            let n_all = clauses(values).iter().filter(|c| !binds(c).is_empty()).count();
            if n_comma > 1 {
                o.multi_comma += 1;
            }
            if n_all > 1 {
                o.multi_with_and += 1;
            }
            if n_all > 1 && n_comma <= 1 {
                o.missed_multi.push(format!("{addr} [{slug}] => {values}"));
            }
        }
        if row.sentence_kind == "theorem" {
            o.theorem.push((row.id.clone(), width));
        }
    }

    println!("== {label} ==");
    println!("R1  universals, census rule: {}", o.strict);
    println!("R1  universals, corrected rule: {}", o.loose);
    println!("R1  missed by the census rule: {}", o.missed_any.len());
    for m in &o.missed_any {
        println!("        {m}");
    }
    println!("R2  multi-binding, census rule (comma only): {}", o.multi_comma);
    println!("R2  multi-binding, corrected rule (comma or `and`): {}", o.multi_with_and);
    println!("R2  missed by the census rule: {}", o.missed_multi.len());
    for m in &o.missed_multi {
        println!("        {m}");
    }
    println!("R3  sentence_kind = theorem rows and their width span: {}", o.theorem.len());
    for (id, w) in &o.theorem {
        println!("        {id}  =>  {w}");
    }
    println!("R4  parameterised whole-domain fraction spans:");
    for (span, n) in &o.param_domain {
        println!("        {n:>3}  {span}");
    }
    println!();
    o
}

/// The case that must fail, with every expectation fixed before the run.
///
/// Six planted entries. Two are universals only the corrected rule sees, one is
/// the `anywhere` trap that neither rule may count, one joins two bindings with
/// a bare `and`, one is an ordinary comma multi-binding both rules see, and one
/// theorem row supplies R3 with something to find.
const CONTROL: &str = r#"
[[proposal]]
id = "planted_trailing_note"
sentence_kind = "theorem"
predicate = [
  "threads: threads any, the equalities being decided at compile time",
  "total_width: W in 3..=7",
  "build_profile: no feature gates anywhere, main arm at exit 0",
]

[[proposal]]
id = "planted_and_joined"
predicate = ["threads: threads = 1 for the timed instance and threads any for the artifacts"]

[[proposal]]
id = "planted_comma_multi"
predicate = ["build_profile: emission in {metadata only, full codegen}, debug-assertions any"]

[[proposal]]
id = "planted_plain"
predicate = ["fraction_width: in 0..=W-1"]
"#;

fn control() -> bool {
    let o = run("PLANTED CONTROL", &scan(CONTROL));
    let mut ok = true;
    let mut check = |what: &str, got: usize, want: usize| {
        if got != want {
            println!("CONTROL FAILED: {what}: got {got}, want {want}");
            ok = false;
        }
    };
    // `debug-assertions any` ends the string, so the census rule sees it. The
    // other two universals sit before a clause boundary and it does not.
    check("universals, census rule", o.strict, 1);
    check("universals, corrected rule", o.loose, 3);
    check("universals the census rule misses", o.missed_any.len(), 2);
    // The `anywhere` span must be counted by neither, or the corrected rule is
    // a substring test wearing a clause splitter's clothes.
    check(
        "the `anywhere` trap is counted by neither",
        o.missed_any
            .iter()
            .filter(|m| m.contains("anywhere"))
            .count(),
        0,
    );
    check("multi-binding, comma only", o.multi_comma, 1);
    check("multi-binding, comma or `and`", o.multi_with_and, 2);
    check("multi-binding the census rule misses", o.missed_multi.len(), 1);
    check("theorem rows found", o.theorem.len(), 1);
    check("parameterised spans found", o.param_domain.len(), 1);
    if ok {
        println!("control: passed, and every arm returned a non-zero it could have got wrong.\n");
    }
    ok
}

fn main() {
    if !control() {
        eprintln!("the reply instrument is wrong; the real run would be uninterpretable");
        std::process::exit(1);
    }
    let Some(dir) = env::args().nth(1) else {
        println!("no registry directory given; control-only run complete");
        return;
    };
    let mut rows = Vec::new();
    let mut files: Vec<_> = fs::read_dir(&dir)
        .expect("registry directory")
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| p.extension().map(|x| x == "toml").unwrap_or(false))
        .collect();
    files.sort();
    for p in &files {
        rows.extend(scan(&fs::read_to_string(p).expect("registry file")));
    }
    println!("read {} files, {} rows\n", files.len(), rows.len());
    run("COMMITTED REGISTRY", &rows);
}
