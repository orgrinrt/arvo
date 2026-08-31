//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! The half of a predicate entry nothing reads.
//!
//! `checks/src/predicate.rs` checks the slug side of `<slug>: <values>` and
//! says so in its own module docs, deliberately, because the value grammars
//! differ per axis. This probe asks what that costs, on two questions the
//! warrant-marker work ran into sideways and which are separable from it.
//!
//! A1  Two `dimension` rows declare a spelling **not admissible** in their own
//!     `grammar` field, in bold, and nothing parses a grammar field. Does the
//!     corpus write an inadmissible spelling anyway?
//!
//! A2  A values side can bind more than one thing. `dimension.toml`'s whole
//!     argument for a closed axis vocabulary is that an undeclared axis
//!     "silently converts the strongest negative statement in the notation into
//!     a shrug". An axis packed inside another axis's values side is undeclared
//!     and invisible at once, because the slug that carries it is legitimate.
//!     How many entries bind more than one thing, and is any of the extra
//!     bindings undeclared?
//!
//! Neither question is the warrant question. Both are reported under the
//! standing instruction to name unlicensed mechanisms found outside the brief.
//!
//! Run:
//!     rustc -O values_side_admissibility.rs -o /tmp/vsa
//!     /tmp/vsa <registry-dir>
//!
//! With no argument it runs the planted control only.

use std::collections::BTreeSet;
use std::env;
use std::fs;

const PREDICATE_FIELDS: &[&str] = &["predicate", "holds", "fails"];

#[derive(Default, Clone)]
struct Row {
    namespace: String,
    id: String,
    grammar: String,
    entries: Vec<(String, String)>,
}

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
        if let Some(rest) = line.strip_prefix("grammar = ") {
            if let Some(v) = quoted(rest) {
                rows.last_mut().unwrap().grammar = v;
            }
            continue;
        }
        for field in PREDICATE_FIELDS {
            let head = format!("{field} = [");
            if let Some(rest) = line.strip_prefix(&head) {
                if line.contains(']') {
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

/// Bindings in a values side, counted at brace depth zero.
///
/// A binding is ` = ` or ` in `. Inside `{...}` neither counts, because
/// `overflow policy in {wrap, saturate}` is one binding over a set, and a set
/// member may contain a space. Depth-aware rather than a substring count, which
/// is the difference between measuring structure and measuring punctuation.
fn bindings(values: &str) -> Vec<String> {
    let bytes: Vec<char> = values.chars().collect();
    let mut depth = 0usize;
    let mut clause_start = 0usize;
    let mut clauses: Vec<String> = Vec::new();
    for (i, c) in bytes.iter().enumerate() {
        match c {
            '{' => depth += 1,
            '}' => depth = depth.saturating_sub(1),
            ',' if depth == 0 => {
                clauses.push(bytes[clause_start..i].iter().collect());
                clause_start = i + 1;
            }
            _ => {}
        }
    }
    clauses.push(bytes[clause_start..].iter().collect());

    clauses
        .into_iter()
        .map(|c| c.trim().to_string())
        .filter(|c| {
            let d = declaring_part(c);
            !d.is_empty()
        })
        .collect()
}

/// The left-hand side of a clause that binds something, or empty.
///
/// The `` any `` arm is not decoration and was not in the first version. The
/// control caught its absence: the universal is spelled `<axis> any` with no
/// separator at all, so a splitter looking only for `` = `` and `` in `` reads
/// `debug-assertions any` as free text and reports a two-binding entry as one.
/// That is the exact entry this probe was written to find, and the first
/// version could not have found it.
fn declaring_part(clause: &str) -> String {
    for sep in [" = ", " in ", " <= ", " >= ", " > ", " < "] {
        if let Some((lhs, _)) = clause.split_once(sep) {
            return lhs.trim().to_string();
        }
    }
    if let Some(lhs) = clause.trim().strip_suffix(" any") {
        return lhs.trim().to_string();
    }
    String::new()
}

fn normalise(s: &str) -> String {
    s.to_lowercase().replace([' ', '-'], "_")
}

fn main() {
    if !control() {
        eprintln!("scanner is wrong; the real run would be uninterpretable");
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
    run("COMMITTED REGISTRY", &rows);
}

fn run(label: &str, rows: &[Row]) -> (usize, usize, usize) {
    println!("== {label} ==");

    // The declared axis vocabulary, and the axes whose own grammar forbids the
    // universal spelling.
    let mut declared: BTreeSet<String> = BTreeSet::new();
    let mut forbids_any: BTreeSet<String> = BTreeSet::new();
    for r in rows.iter().filter(|r| r.namespace == "dimension") {
        declared.insert(normalise(&r.id));
        if r.grammar.contains("not admissible") {
            forbids_any.insert(normalise(&r.id));
        }
    }
    println!(
        "declared axes: {}, of which forbid the universal in their own grammar: {:?}",
        declared.len(),
        forbids_any
    );

    // Short spellings an axis writes itself as inside its values side. Read off
    // the grammars rather than guessed, so a renamed axis does not silently
    // start producing false positives here.
    let mut aliases: BTreeSet<String> = declared.clone();
    for r in rows.iter().filter(|r| r.namespace == "dimension") {
        for clause in r.grammar.split('`') {
            let lhs = declaring_part(clause);
            if !lhs.is_empty() {
                aliases.insert(normalise(&lhs));
            }
        }
    }

    let mut a1 = 0usize;
    let mut a2_multi = 0usize;
    let mut a2_undeclared = 0usize;

    println!("\nA1  inadmissible universal written anyway:");
    for r in rows.iter().filter(|r| r.namespace != "dimension") {
        for (field, entry) in &r.entries {
            let Some((slug, values)) = split_entry(entry) else {
                continue;
            };
            let slug_n = normalise(slug);
            let universal = values == "any" || values.ends_with(" any");
            if universal && forbids_any.contains(&slug_n) {
                a1 += 1;
                println!("        {}::{}::{field} => `{entry}`", r.namespace, r.id);
            }
        }
    }
    if a1 == 0 {
        println!("        none");
    }

    println!("\nA2  values sides binding more than one thing:");
    for r in rows.iter().filter(|r| r.namespace != "dimension") {
        for (field, entry) in &r.entries {
            let Some((slug, values)) = split_entry(entry) else {
                continue;
            };
            let bs = bindings(values);
            if bs.len() <= 1 {
                continue;
            }
            a2_multi += 1;
            let strays: Vec<String> = bs
                .iter()
                .map(|c| normalise(&declaring_part(c)))
                .filter(|lhs| !lhs.is_empty() && !aliases.contains(lhs))
                .collect();
            let mark = if strays.is_empty() {
                String::new()
            } else {
                a2_undeclared += 1;
                format!("   <== binds undeclared: {strays:?}")
            };
            println!(
                "        {}::{}::{field} [{}] => `{}`{mark}",
                r.namespace, r.id, slug, values
            );
        }
    }
    if a2_multi == 0 {
        println!("        none");
    }

    println!(
        "\nsummary: A1 inadmissible={a1}, A2 multi-binding={a2_multi}, of which bind an \
         undeclared name={a2_undeclared}\n"
    );
    (a1, a2_multi, a2_undeclared)
}

/// The case that must fail, with the answers fixed before the run.
///
/// Three planted rows. `operation` declares its universal inadmissible and one
/// predicate writes it, so A1 must be 1. One values side binds two things and
/// the second name is declared nowhere, so A2 must be 1 multi-binding with 1
/// undeclared. And one entry binds a set containing a comma inside braces,
/// which must NOT count as two bindings: that is the arm that fails if the
/// depth tracking is wrong, and it is the reason this is not a substring count.
const CONTROL: &str = r#"
[[dimension]]
id = "operation"
grammar = "`operation = <op>`. **`operation any` is not admissible**, because the set is open."

[[dimension]]
id = "overflow_policy"
grammar = "`overflow policy = <name>`, `overflow policy in {<set>}`, or `overflow policy any`."

[[dimension]]
id = "build_profile"
grammar = "`debug-assertions = on`, `debug-assertions = off`."

[[proposal]]
id = "planted_inadmissible"
predicate = ["operation: operation any"]

[[proposal]]
id = "planted_packed"
predicate = ["build_profile: emission in {metadata only, full codegen}, debug-assertions any"]

[[proposal]]
id = "planted_set_with_comma"
predicate = ["overflow_policy: overflow policy in {wrap, saturate, clamp}"]
"#;

fn control() -> bool {
    let rows = scan(CONTROL);
    let (a1, multi, undeclared) = run("PLANTED CONTROL", &rows);
    let mut ok = true;
    let mut check = |what: &str, got: usize, want: usize| {
        if got != want {
            println!("CONTROL FAILED: {what}: got {got}, want {want}");
            ok = false;
        }
    };
    check("A1 inadmissible universals", a1, 1);
    check("A2 multi-binding entries", multi, 1);
    check("A2 entries binding an undeclared name", undeclared, 1);
    if ok {
        println!("control: passed, and every arm returned a non-zero, so none is vacuous.\n");
    }
    ok
}
