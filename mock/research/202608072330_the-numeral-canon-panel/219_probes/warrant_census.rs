//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! What the committed predicate corpus looks like on the axes the warrant
//! marker would touch, and whether a prefix marker can be parsed at all.
//!
//! Built for seat 219. Five questions, each with the case that must fail
//! stated before the run, because a census whose every answer is possible is
//! not an instrument.
//!
//! Q1  How many predicate entries are there, and how many end in the bare
//!     token `any`? `any` is the one token every axis grammar spells the same
//!     way, so it is the only cross-axis hinge a checker can hold without a
//!     per-axis parser. If it is spelled several ways the hinge does not exist.
//!
//! Q2  How many rows carry an `any` entry AND a fixed-or-range entry together?
//!     A row-level marker (reusing `sentence_kind`) can only say one thing per
//!     row. Every such row is a row a row-level marker cannot describe. If the
//!     count is zero, the per-axis claim is unfounded and the cheap answer
//!     wins.
//!
//! Q3  Do any values sides begin with `swept`, `construction` or `exhaustive`?
//!     A prefix marker is unparseable if a real value already starts with one.
//!
//! Q4  How many entries name a whole-container range, `1..=64` or `0..=63`?
//!     The ruling's third state. If nothing writes one the state is
//!     anticipatory rather than observed, which is worth saying out loud.
//!
//! Q5  How many entries would a `swept`-may-not-carry-`any` arm fire on today?
//!     That is the retroactive blast radius, and it decides whether the arm can
//!     be a clean-slate assertion or has to be a ratchet.
//!
//! Run:
//!     rustc -O warrant_census.rs -o /tmp/warrant_census
//!     /tmp/warrant_census <registry-dir>
//!
//! With no argument it runs the planted control only, which is the mode that
//! proves the scanner can report a non-zero.

use std::collections::BTreeMap;
use std::env;
use std::fs;

/// The array fields that hold a predicate. Named, not discovered: a walk over
/// every `string[]` would read `keywords` too, which is the mistake the checks
/// crate documents having avoided.
const PREDICATE_FIELDS: &[&str] = &["predicate", "holds", "fails"];

/// The three tokens seat 219 proposes to reserve.
const WARRANTS: &[&str] = &["swept", "construction", "exhaustive"];

#[derive(Debug, Default)]
struct Row {
    namespace: String,
    id: String,
    sentence_kind: String,
    entries: Vec<(String, String)>, // (field, entry)
}

/// A deliberately small TOML reader: this corpus writes one row per `[[ns]]`
/// header, one field per line, and arrays either inline or one element a line.
/// A full parser would be better and is what the checks crate has; this is a
/// probe and says so.
fn scan(text: &str) -> Vec<Row> {
    let mut rows: Vec<Row> = Vec::new();
    let mut in_array: Option<String> = None;

    for raw in text.lines() {
        let line = raw.trim();

        if let Some(field) = in_array.clone() {
            if line.starts_with(']') {
                in_array = None;
            } else if let Some(v) = quoted(line) {
                if let Some(r) = rows.last_mut() {
                    r.entries.push((field, v));
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
                    // inline array
                    let body = rest.rsplit_once(']').map(|(a, _)| a).unwrap_or(rest);
                    for part in split_inline(body) {
                        rows.last_mut()
                            .unwrap()
                            .entries
                            .push((field.to_string(), part));
                    }
                } else {
                    in_array = Some(field.to_string());
                }
                break;
            }
        }
    }
    rows
}

/// The first double-quoted run on a line. Adequate here: no predicate entry in
/// this corpus contains an escaped quote, and the control below plants one that
/// would be visible if that changed.
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

/// True when the values side is the universal for its axis. Every declared
/// grammar spells it as a trailing bare `any`, which is what Q1 checks rather
/// than assumes.
fn is_any(values: &str) -> bool {
    values == "any" || values.ends_with(" any")
}

fn is_whole_container_range(values: &str) -> bool {
    values.contains("1..=64") || values.contains("0..=63")
}

struct Census {
    entries: usize,
    any_entries: usize,
    any_spellings: BTreeMap<String, usize>,
    mixed_rows: Vec<String>,
    reserved_collisions: Vec<String>,
    whole_container: Vec<String>,
    swept_any_hits: Vec<String>,
    kind_of_any_rows: BTreeMap<String, usize>,
    /// Q6: slug sides that are not a bare `[a-z_]+` slug. The slug side is the
    /// only part of an entry a checker already parses, so whether a marker can
    /// live there at all turns on whether it is lexically clean today.
    dirty_slugs: Vec<String>,
    /// Q7: which axes write the universal as a bare `any` with the axis word
    /// left off, since a marker placed on the values side has to survive that.
    bare_any: BTreeMap<String, usize>,
}

fn census(rows: &[Row]) -> Census {
    let mut c = Census {
        entries: 0,
        any_entries: 0,
        any_spellings: BTreeMap::new(),
        mixed_rows: Vec::new(),
        reserved_collisions: Vec::new(),
        whole_container: Vec::new(),
        swept_any_hits: Vec::new(),
        kind_of_any_rows: BTreeMap::new(),
        dirty_slugs: Vec::new(),
        bare_any: BTreeMap::new(),
    };

    for row in rows {
        let mut has_any = false;
        let mut has_bounded = false;

        for (field, entry) in &row.entries {
            c.entries += 1;
            let Some((slug, values)) = split_entry(entry) else {
                continue;
            };
            let addr = format!("{}::{}::{field}::{slug}", row.namespace, row.id);

            // Q6. A marker can only live on the slug side if the slug side is
            // lexically clean, so that is measured rather than assumed.
            if slug.is_empty() || !slug.chars().all(|ch| ch.is_ascii_lowercase() || ch == '_') {
                c.dirty_slugs.push(format!("{addr} => slug side is `{slug}`"));
            }
            // Q7. A marker on the values side has to survive the axis word
            // being left off, so the bare spelling is counted separately.
            if values == "any" {
                *c.bare_any.entry(slug.to_string()).or_default() += 1;
            }

            if is_any(values) {
                c.any_entries += 1;
                has_any = true;
                *c.any_spellings.entry(values.to_string()).or_default() += 1;
                // Q5: unmarked defaults to `swept`, so every unmarked `any`
                // entry is what a swept-may-not-carry-any arm fires on.
                if !WARRANTS.iter().any(|w| values.starts_with(w)) {
                    c.swept_any_hits.push(addr.clone());
                }
            } else {
                has_bounded = true;
            }

            if WARRANTS.iter().any(|w| {
                values == *w || values.starts_with(&format!("{w} "))
            }) {
                c.reserved_collisions.push(format!("{addr} => {values}"));
            }

            if is_whole_container_range(values) {
                c.whole_container.push(format!("{addr} => {values}"));
            }
        }

        if has_any && has_bounded {
            c.mixed_rows
                .push(format!("{}::{}", row.namespace, row.id));
            *c.kind_of_any_rows
                .entry(if row.sentence_kind.is_empty() {
                    "(none)".to_string()
                } else {
                    row.sentence_kind.clone()
                })
                .or_default() += 1;
        }
    }
    c
}

fn report(label: &str, c: &Census) {
    println!("== {label} ==");
    println!("Q1  entries: {}", c.entries);
    println!("Q1  entries whose values are the axis universal: {}", c.any_entries);
    println!("Q1  spellings of the universal seen:");
    for (spelling, n) in &c.any_spellings {
        println!("        {n:>4}  {spelling}");
    }
    println!(
        "Q2  rows carrying a universal AND a bounded entry together: {}",
        c.mixed_rows.len()
    );
    for r in &c.mixed_rows {
        println!("        {r}");
    }
    println!("Q2  sentence_kind of those rows: {:?}", c.kind_of_any_rows);
    println!(
        "Q3  values sides beginning with a reserved warrant token: {} (a collision in the unmarked corpus, a marker in the control)",
        c.reserved_collisions.len()
    );
    for r in &c.reserved_collisions {
        println!("        {r}");
    }
    println!(
        "Q4  entries naming a whole-container range: {}",
        c.whole_container.len()
    );
    for r in &c.whole_container {
        println!("        {r}");
    }
    println!("Q6  slug sides that are not a bare lowercase slug: {}", c.dirty_slugs.len());
    for r in &c.dirty_slugs {
        println!("        {r}");
    }
    println!("Q7  axes writing the universal as a bare `any`: {:?}", c.bare_any);
    println!(
        "Q5  entries a `swept may not carry any` arm fires on today: {}",
        c.swept_any_hits.len()
    );
    for r in &c.swept_any_hits {
        println!("        {r}");
    }
    println!();
}

/// The case that must fail.
///
/// Planted before the real run, with the answers written down here rather than
/// read off the output: five entries, three of them universals, one row mixing
/// a universal with a bounded entry, one values side colliding with a reserved
/// token, one whole-container range, and exactly two `swept`-defaulted
/// universals because the third carries an explicit `construction` marker and
/// must be excluded.
///
/// If any of those comes back different the scanner is not measuring what the
/// real run will be quoted for.
const CONTROL: &str = r#"
[[proposal]]
id = "planted_mixed"
sentence_kind = "argument"
predicate = [
  "total_width: W any",
  "fraction_width: F = 0",
]

[[proposal]]
id = "planted_marked"
sentence_kind = "argument"
predicate = ["total_width: construction W any"]

[[law]]
id = "planted_law"
holds = ["threads: threads any"]
fails = ["container: exhaustive u8"]

[[proposal]]
id = "planted_container_range"
predicate = ["total_width: W in 1..=64"]

[[proposal]]
id = "planted_dirty_slug"
predicate = ["Total Width: W any"]

[[proposal]]
id = "planted_bare_any"
predicate = ["threads: any"]
"#;

fn control() -> bool {
    let rows = scan(CONTROL);
    let c = census(&rows);
    let mut ok = true;
    let mut check = |what: &str, got: usize, want: usize| {
        if got != want {
            println!("CONTROL FAILED: {what}: got {got}, want {want}");
            ok = false;
        }
    };
    check("entries", c.entries, 8);
    check("universals", c.any_entries, 5);
    // Was `1` on the first run and the control fired. The planted data is what
    // was wrong, not the scanner: `planted_law` carries `threads any` in
    // `holds` and a bounded `container` entry in `fails`, so it is a second
    // mixed row by inspection of the fixture above, not by reading the output.
    // Recorded rather than quietly corrected, because a control adjusted to
    // match its own output has stopped being one.
    check("mixed rows", c.mixed_rows.len(), 2);
    check("reserved collisions", c.reserved_collisions.len(), 2);
    check("whole-container ranges", c.whole_container.len(), 1);
    // Q6 and Q7 planted cases. Both arms printed a zero on their first run with
    // nothing in the fixture reaching them, which is a vacuous arm however
    // correct the code behind it. `Total Width` carries a space and capitals,
    // and `threads: any` leaves the axis word off, which is the spelling a
    // values-side marker would have to survive.
    check("dirty slug sides", c.dirty_slugs.len(), 1);
    check("bare `any` spellings", c.bare_any.len(), 1);
    check("swept-defaulted universals", c.swept_any_hits.len(), 4);
    if ok {
        println!("control: passed, and it can fail: every arm above returned a non-zero.\n");
        report("PLANTED CONTROL", &c);
    }
    ok
}

fn main() {
    if !control() {
        eprintln!("the scanner is wrong; the real run below would be uninterpretable");
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
    for path in &files {
        let text = fs::read_to_string(path).expect("registry file");
        rows.extend(scan(&text));
    }
    println!("read {} files, {} rows\n", files.len(), rows.len());
    let c = census(&rows);
    report("COMMITTED REGISTRY", &c);
}
