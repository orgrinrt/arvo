//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! Two arms over `mock/registry/ruling.toml`, each with the case that must fail
//! shown failing before either number counts.
//!
//! Arm A re-measures what the dispatch handed me: 86 rows, 52 at `rung =
//! "stated"`, of those 50 carrying a `quote` and 2 not, split 33 process, 10
//! ruling, 6 intent, 2 deferral, 1 refusal. A brief's count is a claim like any
//! other and gets opened rather than taken.
//!
//! Arm B is the one worth building. Every row's `quote` says it is verbatim,
//! and every row's `provenance` names the file it came from. So the quote text,
//! whitespace normalised, has to occur in that file. A quote that dropped an
//! attributing clause, or gained a comma that turns a relayed claim into an
//! asserted one, fails this and nothing else in the tree looks for it.
//!
//! The controls are the point. Arm A is run against a planted registry whose
//! counts are known by construction and must be reproduced exactly. Arm B is
//! run twice over the real corpus: once as written, and once with one row's
//! quote mangled by a single word, which must move the miss count by exactly
//! one. An arm that cannot report a miss is not an instrument, and this one
//! reported none on its first run, which is exactly when a control earns its
//! keep.
//!
//! The repository root is argv[1]. Nothing here names a path outside the tree
//! it is handed, because a probe that reaches into somebody's checkout keeps
//! working on the wrong subject and says nothing about which tree it read.
//!
//! Build and run:
//!   rustc -O p1_the_stated_rows_and_their_quotes.rs -o /tmp/p1 && /tmp/p1 ../../../..

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

// -- the smallest parser that reads this file, and no more ------------------------

#[derive(Default, Clone)]
struct Row {
    fields: BTreeMap<String, String>,
}

impl Row {
    fn get(&self, k: &str) -> Option<&str> {
        self.fields.get(k).map(String::as_str)
    }
    fn id(&self) -> &str {
        self.get("id").unwrap_or("<no id>")
    }
}

/// Rows of one namespace out of a mockspace registry file.
///
/// Handles the two string shapes this file actually uses, single line `k =
/// "v"` and the triple-quoted block, plus single line arrays. It does not
/// handle anything else, and it does not need to: a parser that accepts more
/// than the corpus contains is a parser nobody can check.
fn parse(text: &str, namespace: &str) -> Vec<Row> {
    let header = format!("[[{namespace}]]");
    let mut rows: Vec<Row> = Vec::new();
    let mut cur: Option<Row> = None;
    let mut block: Option<(String, String)> = None;

    for line in text.lines() {
        if let Some((key, acc)) = block.as_mut() {
            if line.trim_end() == "'''" {
                let (k, v) = (key.clone(), acc.clone());
                if let Some(r) = cur.as_mut() {
                    r.fields.insert(k, v);
                }
                block = None;
            } else {
                acc.push_str(line);
                acc.push('\n');
            }
            continue;
        }

        // Any table header closes the row in hand, and only the one we want
        // opens a new one. Closing on the namespace header alone was the first
        // shape of this and control A caught it: a following `[[proposal]]`
        // left `cur` open, so its fields landed on the last ruling and flipped
        // that row's rung. The row count stayed right the whole time, which is
        // why the count alone would not have found it.
        if line.trim_start().starts_with("[[") {
            if let Some(r) = cur.take() {
                rows.push(r);
            }
            if line.trim_end() == header {
                cur = Some(Row::default());
            }
            continue;
        }
        let Some(rest) = line.strip_prefix(|c: char| c.is_ascii_lowercase()).map(|_| line) else {
            continue;
        };
        let Some((key, value)) = rest.split_once(" = ") else {
            continue;
        };
        if !key.chars().all(|c| c.is_ascii_lowercase() || c == '_') {
            continue;
        }
        if value == "'''" {
            block = Some((key.to_string(), String::new()));
            continue;
        }
        let v = value
            .trim()
            .trim_start_matches('"')
            .trim_end_matches('"')
            .to_string();
        if let Some(r) = cur.as_mut() {
            r.fields.insert(key.to_string(), v);
        }
    }
    if let Some(r) = cur.take() {
        rows.push(r);
    }
    rows
}

// -- arm A: the counts the dispatch asserted --------------------------------------

struct Census {
    total: usize,
    by_rung: BTreeMap<String, usize>,
    stated_by_kind: BTreeMap<String, usize>,
    stated_with_quote: usize,
    stated_without_quote: Vec<String>,
}

fn census(rows: &[Row]) -> Census {
    let mut by_rung: BTreeMap<String, usize> = BTreeMap::new();
    let mut stated_by_kind: BTreeMap<String, usize> = BTreeMap::new();
    let mut stated_with_quote = 0usize;
    let mut stated_without_quote: Vec<String> = Vec::new();

    for row in rows {
        let rung = row.get("rung").unwrap_or("<unset>").to_string();
        *by_rung.entry(rung.clone()).or_default() += 1;
        if rung != "stated" {
            continue;
        }
        let kind = row.get("kind").unwrap_or("<unset>").to_string();
        *stated_by_kind.entry(kind).or_default() += 1;
        match row.get("quote") {
            Some(q) if !q.trim().is_empty() => stated_with_quote += 1,
            _ => stated_without_quote.push(row.id().to_string()),
        }
    }
    stated_without_quote.sort();
    Census {
        total: rows.len(),
        by_rung,
        stated_by_kind,
        stated_with_quote,
        stated_without_quote,
    }
}

// -- arm B: does the quote occur in the file its provenance names? ----------------

/// Collapse every run of whitespace to one space, dropping markdown blockquote
/// markers on the way.
///
/// Two things differ between the registry's copy and the panel file's. The
/// wrapping, because each is wrapped to its own column width, so the two agree
/// on the words and disagree on where the newlines fall. And the `> ` a panel
/// file puts in front of every line of his it quotes, which is markup rather
/// than anything he said.
///
/// Leaving the markers in reported 70 misses of 79, which read like a corpus
/// full of fabricated quotes and was entirely this function. That is the run
/// the control was built for.
fn flatten(s: &str) -> String {
    s.lines()
        .map(|l| l.trim_start().trim_start_matches('>').trim_start())
        .collect::<Vec<_>>()
        .join(" ")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}
/// `panel::<dir>::<file>::#anchor` to `mock/research/<dir>/<file>.md`.
///
/// Returns `None` for a provenance naming something other than a panel file,
/// which is not a defect: some rows cite `INTENTS` or `PRIOR_CALLS`, and those
/// resolve the same way, while anything else this arm simply does not reach and
/// says so rather than counting it either way.
fn provenance_file(root: &Path, prov: &str) -> Option<PathBuf> {
    let inner = prov.trim().trim_matches(|c| c == '[' || c == ']' || c == '"');
    let first = inner.split("\", \"").next()?;
    let parts: Vec<&str> = first.split("::").collect();
    if parts.len() < 3 || parts[0] != "panel" {
        return None;
    }
    let p = root
        .join("mock/research")
        .join(parts[1])
        .join(format!("{}.md", parts[2]));
    p.exists().then_some(p)
}

struct Fidelity {
    checked: usize,
    unreachable: Vec<String>,
    misses: Vec<String>,
}

fn fidelity(root: &Path, rows: &[Row]) -> Fidelity {
    let mut checked = 0usize;
    let mut unreachable = Vec::new();
    let mut misses = Vec::new();
    let mut cache: BTreeMap<PathBuf, String> = BTreeMap::new();

    for row in rows {
        let Some(quote) = row.get("quote").filter(|q| !q.trim().is_empty()) else {
            continue;
        };
        let Some(prov) = row.get("provenance") else {
            unreachable.push(format!("{} (no provenance)", row.id()));
            continue;
        };
        let Some(path) = provenance_file(root, prov) else {
            unreachable.push(format!("{} ({prov})", row.id()));
            continue;
        };
        let body = cache
            .entry(path.clone())
            .or_insert_with(|| flatten(&fs::read_to_string(&path).unwrap_or_default()));
        checked += 1;
        if !body.contains(&flatten(quote)) {
            misses.push(format!(
                "{} -> {}",
                row.id(),
                path.file_name().unwrap_or_default().to_string_lossy()
            ));
        }
    }
    Fidelity {
        checked,
        unreachable,
        misses,
    }
}

// -- the controls, which run before anything is reported --------------------------

/// Arm A against a registry whose answer is known by construction.
///
/// Two stated rows, one with a quote and one without, one ratified row, and a
/// row of a different namespace that must not be counted at all. Any of those
/// four coming out wrong means the census is measuring something else.
fn control_census() {
    const PLANTED: &str = r#"
[[ruling]]
id = "stated_with_words"
kind = "ruling"
rung = "stated"
quote = '''
the strategy set is not closed at exactly four
'''

[[ruling]]
id = "stated_without_words"
kind = "process"
rung = "stated"

[[ruling]]
id = "already_ratified"
kind = "intent"
rung = "ratified"
quote = '''
cold is not to be deprioritised
'''

[[proposal]]
id = "not_a_ruling_at_all"
kind = "ruling"
rung = "stated"
"#;
    let rows = parse(PLANTED, "ruling");
    let c = census(&rows);
    assert_eq!(c.total, 3, "the proposal leaked into the ruling namespace");
    assert_eq!(c.by_rung.get("stated"), Some(&2));
    assert_eq!(c.by_rung.get("ratified"), Some(&1));
    assert_eq!(c.stated_with_quote, 1);
    assert_eq!(c.stated_without_quote, vec!["stated_without_words"]);
    assert_eq!(c.stated_by_kind.get("process"), Some(&1));
    assert_eq!(c.stated_by_kind.get("ruling"), Some(&1));
    assert_eq!(c.stated_by_kind.get("intent"), None, "a ratified row was counted as stated");
    println!("control A: census reproduces a planted registry exactly");
}

/// Arm B has to be able to report a miss, and it reported none on its first
/// run over the real corpus.
///
/// A run that can only return zero is not a measurement, so one real row's
/// quote is mangled by a single word and the miss count must move by exactly
/// one. Mangling is done on the parsed rows rather than on disk, because a
/// probe that edits the corpus to test itself has changed the thing it was
/// measuring.
fn control_fidelity(root: &Path, rows: &[Row]) -> usize {
    let clean_run = fidelity(root, rows);
    let clean = clean_run.misses.len();
    let already_missing: Vec<&str> = clean_run
        .misses
        .iter()
        .map(|m| m.split(" -> ").next().unwrap_or(""))
        .collect();

    // Mangle a row that currently matches, never one that already misses.
    // The first shape of this took the first reachable row and happened to
    // take one of the misses, so the count did not move and the control read
    // as the arm being blind when it was the choice of row.
    let mut mangled: Vec<Row> = rows.to_vec();
    let mut planted = false;
    for row in mangled.iter_mut() {
        let Some(q) = row.get("quote").map(str::to_string) else {
            continue;
        };
        if q.trim().is_empty()
            || provenance_file(root, row.get("provenance").unwrap_or("")).is_none()
            || already_missing.contains(&row.id())
        {
            continue;
        }
        let target = row.id().to_string();
        row.fields
            .insert("quote".into(), format!("{q} and one word he never said"));
        println!("control B: mangled `{target}`, which matches as written, by one clause");
        planted = true;
        break;
    }
    assert!(planted, "no matching row was reachable to mangle, so the control never ran");

    let dirty = fidelity(root, &mangled).misses.len();
    assert_eq!(
        dirty,
        clean + 1,
        "mangling one quote did not move the miss count by one, so the arm cannot report a miss"
    );
    println!("control B: miss count moved {clean} -> {dirty} on one mangled clause");
    clean
}
fn main() {
    let root = PathBuf::from(
        std::env::args()
            .nth(1)
            .expect("give the repository root as the first argument"),
    );
    let path = root.join("mock/registry/ruling.toml");
    let text = fs::read_to_string(&path).expect("could not read the ruling namespace");
    let rows = parse(&text, "ruling");

    control_census();
    let expected_misses = control_fidelity(&root, &rows);
    println!();

    let c = census(&rows);
    println!("== arm A, the census ==");
    println!("rows: {}", c.total);
    for (rung, n) in &c.by_rung {
        println!("  rung {rung}: {n}");
    }
    println!("stated rows carrying a quote: {}", c.stated_with_quote);
    println!("stated rows carrying none:    {}", c.stated_without_quote.len());
    for id in &c.stated_without_quote {
        println!("    {id}");
    }
    for (kind, n) in &c.stated_by_kind {
        println!("  stated, kind {kind}: {n}");
    }

    let f = fidelity(&root, &rows);
    println!();
    println!("== arm B, quote against the file its provenance names ==");
    println!("quotes checked against a panel file: {}", f.checked);
    println!("quotes the arm could not reach:      {}", f.unreachable.len());
    for u in &f.unreachable {
        println!("    {u}");
    }
    println!("quotes not found in their source:    {}", f.misses.len());
    for m in &f.misses {
        println!("    {m}");
    }
    assert_eq!(
        f.misses.len(),
        expected_misses,
        "the reported run disagrees with the controlled run"
    );
}
