//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! Two more arms, both aimed at defects I first found by reading and then had
//! no way to be sure I had found all of.
//!
//! **Arm C, an edge that runs backwards in time.** A row at `rung = "stated"`
//! that claims to `supersede` or `correct` a row at `rung = "ratified"` is
//! saying an unratified statement overrides one op has since stamped. Both
//! rows may be his and both may be right; the edge is still the wrong way
//! round, because the ratification is the later act. Found twice by hand,
//! which is exactly the count that makes you want a sweep.
//!
//! **Arm D, how far a `says` travels from the `quote` it sits on.** Arm B in
//! `p1` established that every quote occurs verbatim in its source. That is a
//! check on the quote and says nothing about the restatement built on it. This
//! arm counts the content words in `says` that appear nowhere in `quote`, and
//! then asks, of those, how many appear elsewhere in the file the provenance
//! names.
//!
//! The split is the useful part. A word absent from the quote and absent from
//! the source is the panel's own vocabulary, which is ordinary and expected,
//! since naming the intent is a second act. A word absent from the quote and
//! **present in the source** is the shape worth reading: the `says` is drawing
//! on a sentence of his that the `quote` was clipped short of, so a reader
//! diffing the row against its own evidence finds a claim the evidence does
//! not carry, while the source would have carried it.
//!
//! This arm ranks and does not judge. It cannot: a shared word may be a
//! coincidence of two people writing about one subject, and stemming would
//! trade one guess for another. What it is for is turning "I found three" into
//! a list somebody read all of.
//!
//! Build and run:
//!   rustc -O p2_how_far_a_says_travels_from_its_quote.rs -o /tmp/p2 && /tmp/p2 ../../../..

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

// -- shared with p1, kept whole here so this file runs on its own -----------------

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
    fn list(&self, k: &str) -> Vec<String> {
        self.get(k)
            .map(|v| {
                v.trim()
                    .trim_matches(|c| c == '[' || c == ']')
                    .split(',')
                    .map(|s| s.trim().trim_matches('"').to_string())
                    .filter(|s| !s.is_empty())
                    .collect()
            })
            .unwrap_or_default()
    }
}

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
        if line.trim_start().starts_with("[[") {
            if let Some(r) = cur.take() {
                rows.push(r);
            }
            if line.trim_end() == header {
                cur = Some(Row::default());
            }
            continue;
        }
        let Some((key, value)) = line.split_once(" = ") else {
            continue;
        };
        if key.is_empty() || !key.chars().all(|c| c.is_ascii_lowercase() || c == '_') {
            continue;
        }
        if value == "'''" {
            block = Some((key.to_string(), String::new()));
            continue;
        }
        let v = value.trim().trim_matches('"').to_string();
        if let Some(r) = cur.as_mut() {
            r.fields.insert(key.to_string(), v);
        }
    }
    if let Some(r) = cur.take() {
        rows.push(r);
    }
    rows
}

fn provenance_file(root: &Path, prov: &str) -> Option<PathBuf> {
    let inner = prov
        .trim()
        .trim_matches(|c| c == '[' || c == ']' || c == '"');
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

// -- arm C: a supersession that runs backwards in time ----------------------------

struct BackwardEdge {
    from: String,
    from_rung: String,
    field: String,
    to: String,
    to_rung: String,
}

/// Every `supersedes` and `corrects` edge, with the rung at each end.
///
/// Reports the edges pointing from an unratified row at a ratified one, and
/// separately any edge naming a row that is not in the namespace at all, which
/// is a different defect and would otherwise hide inside the same sweep.
fn backward_edges(rows: &[Row]) -> (Vec<BackwardEdge>, Vec<String>) {
    let rung: BTreeMap<&str, &str> = rows
        .iter()
        .map(|r| (r.id(), r.get("rung").unwrap_or("<unset>")))
        .collect();
    let mut backward = Vec::new();
    let mut dangling = Vec::new();

    for row in rows {
        let from_rung = row.get("rung").unwrap_or("<unset>");
        for field in ["supersedes", "corrects"] {
            for target in row.list(field) {
                let Some(to_rung) = rung.get(target.as_str()) else {
                    dangling.push(format!("{}.{field} -> {target}", row.id()));
                    continue;
                };
                if from_rung != "ratified" && *to_rung == "ratified" {
                    backward.push(BackwardEdge {
                        from: row.id().to_string(),
                        from_rung: from_rung.to_string(),
                        field: field.to_string(),
                        to: target,
                        to_rung: (*to_rung).to_string(),
                    });
                }
            }
        }
    }
    (backward, dangling)
}

// -- arm D: the distance between a says and its quote -----------------------------

/// Words too common to carry a claim.
///
/// Deliberately short. A longer list would suppress hits, and suppressing a
/// hit is the failure this arm exists to avoid; a false positive costs a
/// reader ten seconds and a false negative costs the finding.
const STOP: &[&str] = &[
    "about",
    "after",
    "again",
    "against",
    "already",
    "also",
    "always",
    "another",
    "anything",
    "because",
    "been",
    "before",
    "being",
    "below",
    "between",
    "both",
    "cannot",
    "could",
    "does",
    "doing",
    "done",
    "each",
    "either",
    "else",
    "enough",
    "even",
    "ever",
    "every",
    "everything",
    "from",
    "further",
    "give",
    "given",
    "have",
    "having",
    "here",
    "himself",
    "into",
    "itself",
    "just",
    "keep",
    "less",
    "like",
    "made",
    "make",
    "makes",
    "many",
    "might",
    "more",
    "most",
    "much",
    "must",
    "name",
    "named",
    "names",
    "naming",
    "neither",
    "never",
    "nothing",
    "only",
    "other",
    "others",
    "over",
    "rather",
    "same",
    "several",
    "should",
    "since",
    "some",
    "something",
    "such",
    "than",
    "that",
    "their",
    "them",
    "then",
    "there",
    "these",
    "they",
    "thing",
    "things",
    "this",
    "those",
    "through",
    "under",
    "until",
    "upon",
    "very",
    "what",
    "whatever",
    "when",
    "where",
    "whether",
    "which",
    "while",
    "with",
    "within",
    "without",
    "would",
    "your",
    "yours",
];

fn words(s: &str) -> BTreeSet<String> {
    s.to_lowercase()
        .split(|c: char| !c.is_ascii_alphabetic())
        .filter(|w| w.len() >= 4 && !STOP.contains(w))
        .map(str::to_string)
        .collect()
}

struct Travel {
    id: String,
    absent_from_quote: usize,
    of_those_in_source: Vec<String>,
}

fn travel(root: &Path, rows: &[Row]) -> Vec<Travel> {
    let mut out = Vec::new();
    let mut cache: BTreeMap<PathBuf, BTreeSet<String>> = BTreeMap::new();

    for row in rows {
        let (Some(says), Some(quote)) = (row.get("says"), row.get("quote")) else {
            continue;
        };
        if quote.trim().is_empty() {
            continue;
        }
        let qw = words(quote);
        let absent: BTreeSet<String> = words(says).difference(&qw).cloned().collect();

        let src = row
            .get("provenance")
            .and_then(|p| provenance_file(root, p))
            .map(|path| {
                cache
                    .entry(path.clone())
                    .or_insert_with(|| words(&fs::read_to_string(&path).unwrap_or_default()))
                    .clone()
            })
            .unwrap_or_default();

        let mut shared: Vec<String> = absent.intersection(&src).cloned().collect();
        shared.sort();
        out.push(Travel {
            id: row.id().to_string(),
            absent_from_quote: absent.len(),
            of_those_in_source: shared,
        });
    }
    out.sort_by(|a, b| {
        b.of_those_in_source
            .len()
            .cmp(&a.of_those_in_source.len())
            .then(b.absent_from_quote.cmp(&a.absent_from_quote))
    });
    out
}

// -- controls ---------------------------------------------------------------------

/// Arm C against planted edges whose verdicts are known by construction.
///
/// Four cases and every one has to come out right: the backward edge is
/// reported, the forward edge is not, an edge between two unratified rows is
/// not, and an edge naming nothing lands in the dangling list rather than
/// being silently dropped or counted as backward.
fn control_edges() {
    const PLANTED: &str = r#"
[[ruling]]
id = "stated_one"
rung = "stated"
supersedes = ["ratified_one"]

[[ruling]]
id = "ratified_one"
rung = "ratified"
corrects = ["stated_two"]

[[ruling]]
id = "stated_two"
rung = "stated"
corrects = ["stated_one"]

[[ruling]]
id = "stated_three"
rung = "stated"
supersedes = ["no_such_row"]
"#;
    let rows = parse(PLANTED, "ruling");
    let (backward, dangling) = backward_edges(&rows);
    assert_eq!(backward.len(), 1, "expected exactly the backward edge");
    assert_eq!(backward[0].from, "stated_one");
    assert_eq!(backward[0].to, "ratified_one");
    assert_eq!(dangling, vec!["stated_three.supersedes -> no_such_row"]);
    println!("control C: one backward edge of four, the forward and the flat one left alone");
    println!("control C: the edge naming nothing is reported as dangling, not as backward");
}

/// Arm D against three rows whose numbers are known by construction.
///
/// It needs a real file to intersect against, and uses a fixture committed
/// beside this probe rather than a panel file, so the control cannot start
/// passing for the wrong reason because somebody edited a panel file. The
/// fixture names both words it carries and says why.
fn control_travel(root: &Path) {
    const FIXTURE: &str = "panel::202608072330_the-numeral-canon-panel/218_probes::control_fixture";
    let planted = format!(
        r#"
[[ruling]]
id = "says_is_inside_the_quote"
says = "the strategy set is not closed"
quote = '''
the strategy set is not closed at exactly four
'''
provenance = ["{FIXTURE}"]

[[ruling]]
id = "says_reaches_the_source"
says = "the strategy supersession"
quote = '''
the strategy set is not closed at exactly four
'''
provenance = ["{FIXTURE}"]

[[ruling]]
id = "says_reaches_nowhere"
says = "the strategy zzqqxx"
quote = '''
the strategy set is not closed at exactly four
'''
provenance = ["{FIXTURE}"]
"#
    );
    let rows = parse(&planted, "ruling");
    assert!(
        provenance_file(root, &format!("[\"{FIXTURE}\"]")).is_some(),
        "the control fixture is not where the control expects it, so nothing below means anything"
    );
    let t: BTreeMap<String, Travel> = travel(root, &rows)
        .into_iter()
        .map(|x| (x.id.clone(), x))
        .collect();

    assert_eq!(
        t["says_is_inside_the_quote"].absent_from_quote, 0,
        "a says drawn wholly from its quote must travel no distance"
    );
    assert_eq!(t["says_reaches_nowhere"].absent_from_quote, 1);
    assert!(
        t["says_reaches_nowhere"].of_those_in_source.is_empty(),
        "a word in no source was counted as drawn from one"
    );
    assert_eq!(t["says_reaches_the_source"].absent_from_quote, 1);
    assert_eq!(
        t["says_reaches_the_source"].of_those_in_source,
        vec!["supersession".to_string()],
        "a word present in the source and absent from the quote was not caught"
    );
    println!("control D: 0 / 1-absent-nowhere / 1-absent-in-source all reported as planted");
}

fn main() {
    let root = PathBuf::from(
        std::env::args()
            .nth(1)
            .expect("give the repository root as the first argument"),
    );

    let text = fs::read_to_string(root.join("mock/registry/ruling.toml"))
        .expect("could not read the ruling namespace");
    let rows = parse(&text, "ruling");

    control_edges();
    control_travel(&root);
    println!();

    let (backward, dangling) = backward_edges(&rows);
    println!("== arm C, supersession and correction edges ==");
    println!("edges naming a row that is not there: {}", dangling.len());
    for d in &dangling {
        println!("    {d}");
    }
    println!(
        "edges from an unratified row at a ratified one: {}",
        backward.len()
    );
    for e in &backward {
        println!(
            "    {} ({}) {} {} ({})",
            e.from, e.from_rung, e.field, e.to, e.to_rung
        );
    }

    let stated: Vec<Row> = rows
        .iter()
        .filter(|r| r.get("rung") == Some("stated"))
        .cloned()
        .collect();
    println!();
    println!("== arm D, stated rows ranked by how far the says travels ==");
    println!(
        "(words in `says` absent from `quote`, and how many of those the source file carries)"
    );
    for t in travel(&root, &stated) {
        if t.of_those_in_source.is_empty() {
            continue;
        }
        println!(
            "  {:>2} of {:>2}  {}",
            t.of_those_in_source.len(),
            t.absent_from_quote,
            t.id
        );
        println!("           {}", t.of_those_in_source.join(" "));
    }
}
