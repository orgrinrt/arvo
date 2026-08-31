//! A spike. It answers three questions and nothing else.
//!
//! 1. Can the proposed grammar `<axis>: <span>: <warrant>` be read by an
//!    extension of the split the shipped arm already does, without reparsing a
//!    single committed entry? (The append-only rule turns on that "without".)
//! 2. Do the arms the grammar makes possible fire on planted inputs, including
//!    the case each one exists to refuse?
//! 3. What do they say about the committed canon today?
//!
//! Not shipping code. Every name, arity and field order here is scaffolding
//! chosen to reach the check. The result that transfers is what it establishes,
//! never how it is written.

use arvo_checks::{citation, load, parse, predicate, provenance, shape, Registry};
use std::path::PathBuf;

/// The registry, by a path relative to this manifest rather than through
/// `canon()`. `canon()` derives its path from CARGO_MANIFEST_DIR by popping two
/// components, which is right for `mock/checks` and wrong for anything nested
/// deeper, and it fails by returning an empty registry rather than by erroring.
fn canon() -> Registry {
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../../registry");
    assert!(dir.is_dir(), "the registry is not at {}", dir.display());
    let reg = load(&dir).expect("the registry parses");
    assert!(!reg.rows.is_empty(), "the registry loaded no rows, so every arm below is vacuous");
    reg
}

// -------------------------------------------------------------------------------------------------
// the grammar
// -------------------------------------------------------------------------------------------------

/// The closed set of warrant tokens. Two, because the third state is the
/// absence of a token and the notation may not grow a fourth.
const TOKENS: &[&str] = &["proof", "exhaustive"];

/// Clauses that assert a warrant without naming a mechanism.
///
/// The weakest arm here by a long way, and the one that carries the same defect
/// `shape.rs` documents at length about its own retired word list: a phrase list
/// cannot tell a mechanism from a restatement. It is kept because it costs
/// nothing and catches the laziest form, and it is reported as advisory rather
/// than as the thing that makes the marker honest.
const BARE_RELABELS: &[&str] = &[
    "by construction",
    "structural",
    "it is a proof",
    "proved",
    "trivially",
    "obvious",
    "follows",
    "no width appears",
];

#[derive(Debug, PartialEq, Eq)]
struct Entry<'a> {
    slug: &'a str,
    span: &'a str,
    /// `(token, clause)` where a second colon was written.
    warrant: Option<(&'a str, &'a str)>,
}

/// Split one predicate entry under the proposed grammar.
///
/// The first colon separates the axis, exactly as the shipped arm does. A
/// second colon, where one exists, separates the span from the warrant. The
/// warrant's first word is the token and the rest of it is the clause.
fn read(entry: &str) -> Option<Entry<'_>> {
    let (slug, rest) = entry.split_once(':')?;
    let (span, warrant) = match rest.split_once(':') {
        Some((span, w)) => {
            let w = w.trim();
            let (token, clause) = w.split_once(',').unwrap_or((w, ""));
            (span, Some((token.trim(), clause.trim())))
        },
        None => (rest, None),
    };
    Some(Entry {
        slug: slug.trim(),
        span: span.trim(),
        warrant,
    })
}

fn span_is_any(span: &str) -> bool {
    span.split(|c: char| !c.is_alphanumeric() && c != '_')
        .any(|w| w == "any")
}

// -------------------------------------------------------------------------------------------------
// the arms
// -------------------------------------------------------------------------------------------------

#[derive(Debug, PartialEq, Eq)]
struct Finding {
    kind: &'static str,
    at: String,
    says: String,
}

fn f(kind: &'static str, at: &str, says: String) -> Finding {
    Finding { kind, at: at.to_string(), says }
}

/// Every arm that reads one entry at a time.
fn per_entry(reg: &Registry) -> Vec<Finding> {
    let mut out = Vec::new();
    for (row, field, entry) in predicate::entries(reg) {
        let Some(e) = read(entry) else { continue };
        let at = row.addr();

        // The arm that was drafted as "a span carrying a colon" is arm 1 wearing
        // another name, which the reachability control found: a second colon is
        // either a warrant or a colon inside a span, and in both cases the tail
        // has to be a declared token or the entry is refused. So there is no
        // separate arm and there never was one. `e.span` cannot contain a colon
        // by construction of `read`, which is why the drafted arm could not fire.
        debug_assert!(!e.span.contains(':'), "read() splits at the second colon");

        let Some((token, clause)) = e.warrant else { continue };

        // Arm 1. The token set is closed the way the axis set is closed.
        if !TOKENS.contains(&token) {
            out.push(f("warrant-is-not-a-known-token", &at,
                format!("`{field}` writes the warrant token `{token}` on `{}`, which is not \
                         one of {TOKENS:?}. A warrant nobody declared is a word, and a word \
                         is what the row-level one already was.", e.slug)));
            continue;
        }

        // Arm 2. The clause is the obligation. A token with no clause is the
        // relabel the ruling names, written in the new notation instead of the
        // old one.
        if clause.is_empty() {
            out.push(f("warrant-has-no-clause", &at,
                format!("`{field}` writes `{token}` on `{}` and names no construction. \
                         The test is to name what makes the axis unable to enter, and a \
                         token alone does not.", e.slug)));
            continue;
        }

        // Arm 3, advisory.
        let lower = clause.to_ascii_lowercase();
        if BARE_RELABELS.iter().any(|b| lower.trim_end_matches('.') == *b) {
            out.push(f("warrant-clause-is-a-bare-relabel", &at,
                format!("`{field}` writes `{token}` on `{}` with the clause `{clause}`, \
                         which asserts the warrant instead of naming a mechanism.", e.slug)));
        }

        // Arm 4. `exhaustive` claims a bounded whole, `any` claims no bound.
        // The ruling's own words: neither a sample nor a universal.
        if token == "exhaustive" && span_is_any(e.span) {
            out.push(f("exhaustive-over-an-unbounded-span", &at,
                format!("`{field}` writes `exhaustive` on `{}` whose span is `{}`. A whole \
                         bounded range is neither a sample nor a universal; `any` is the \
                         universal.", e.slug, e.span)));
        }
    }
    out
}

/// Arm 5, which reads the row rather than the entry: `exhaustive` says a span is
/// the whole of a domain, and the domain is a container. The axis for that is
/// already declared, so the obligation is on the existing vocabulary rather than
/// on a second place to name a container.
fn exhaustive_without_a_container(reg: &Registry) -> Vec<Finding> {
    let mut out = Vec::new();
    for row in reg.of("proposal").chain(reg.of("law")) {
        let all: Vec<&String> = row.list("predicate").iter()
            .chain(row.list("holds").iter())
            .chain(row.list("fails").iter())
            .collect();
        let names_a_container = all.iter()
            .filter_map(|e| read(e))
            .any(|e| e.slug == "container");
        for entry in &all {
            let Some(e) = read(entry) else { continue };
            if e.warrant.map(|(t, _)| t) == Some("exhaustive") && !names_a_container {
                out.push(f("exhaustive-names-no-container", &row.addr(),
                    format!("`{}` is claimed exhaustive over `{}` and the row names no \
                             `container`. `W in 1..=64` is the whole of a u64 and a sample \
                             of a u128.", e.slug, e.span)));
            }
        }
    }
    out
}

/// Arm 7, the census turned into a finder. A row asserting the strongest warrant
/// word the schema has, at row level, where no entry of its predicate carries a
/// per-axis one.
fn a_proof_asserted_only_at_row_level(reg: &Registry) -> Vec<Finding> {
    let mut out = Vec::new();
    for row in reg.of("proposal") {
        if row.get("sentence_kind") != Some("theorem") {
            continue;
        }
        let any_proof = row.list("predicate").iter()
            .filter_map(|e| read(e))
            .any(|e| e.warrant.map(|(t, _)| t) == Some("proof"));
        if !any_proof {
            let widths: Vec<String> = row.list("predicate").iter()
                .filter_map(|e| read(e))
                .filter(|e| e.slug == "total_width" || e.slug == "integer_width")
                .map(|e| e.span.to_string())
                .collect();
            let w = if widths.is_empty() { "ABSENT".to_string() } else { widths.join(" / ") };
            out.push(f("a-proof-asserted-only-at-row-level", &row.addr(),
                format!("`sentence_kind` is `theorem` and no predicate entry carries a \
                         `proof` warrant. Its width span is `{w}`.")));
        }
    }
    out
}

// -------------------------------------------------------------------------------------------------
// the planted cases, each of which must fail
// -------------------------------------------------------------------------------------------------

struct Case { name: &'static str, toml: &'static str, expect: &'static [&'static str] }

const CASES: &[Case] = &[
    Case {
        name: "an unmarked entry is read exactly as it is read today",
        toml: r#"
[[dimension]]
id = "total_width"
what = "w"
[[proposal]]
id = "unmarked"
sentence_kind = "measured"
predicate = ["total_width: W in 3..=7"]
"#,
        expect: &[],
    },
    Case {
        name: "a warrant token nobody declared is refused",
        toml: r#"
[[dimension]]
id = "total_width"
what = "w"
[[proposal]]
id = "invented_token"
sentence_kind = "measured"
predicate = ["total_width: W any: handwave, it seemed fine"]
"#,
        expect: &["warrant-is-not-a-known-token"],
    },
    Case {
        name: "a proof token with no construction is refused",
        toml: r#"
[[dimension]]
id = "total_width"
what = "w"
[[proposal]]
id = "bare_token"
sentence_kind = "measured"
predicate = ["total_width: W any: proof"]
"#,
        expect: &["warrant-has-no-clause"],
    },
    Case {
        name: "a proof whose clause only asserts proofhood is reported",
        toml: r#"
[[dimension]]
id = "total_width"
what = "w"
[[proposal]]
id = "relabel"
sentence_kind = "measured"
predicate = ["total_width: W any: proof, by construction"]
"#,
        expect: &["warrant-clause-is-a-bare-relabel"],
    },
    Case {
        name: "exhaustive over an unbounded span is refused",
        toml: r#"
[[dimension]]
id = "total_width"
what = "w"
[[dimension]]
id = "container"
what = "c"
[[proposal]]
id = "exhaustive_any"
sentence_kind = "measured"
predicate = ["total_width: W any: exhaustive, every width", "container: container = u64"]
"#,
        expect: &["exhaustive-over-an-unbounded-span"],
    },
    Case {
        name: "exhaustive with no container named is refused",
        toml: r#"
[[dimension]]
id = "total_width"
what = "w"
[[proposal]]
id = "exhaustive_no_container"
sentence_kind = "measured"
predicate = ["total_width: W in 1..=64: exhaustive, every width the container admits"]
"#,
        expect: &["exhaustive-names-no-container"],
    },
    Case {
        name: "a span carrying a colon is refused",
        toml: r#"
[[dimension]]
id = "toolchain"
what = "t"
[[proposal]]
id = "colon_in_span"
sentence_kind = "measured"
predicate = ["toolchain: rustc 1.98.0: nightly"]
"#,
        // the second colon reads as a warrant, so both arms speak: the tail is
        // not a token, and that is exactly the ambiguity arm 6 exists to name.
        expect: &["warrant-is-not-a-known-token"],
    },
    Case {
        name: "a theorem row with a per-axis proof is not reported",
        toml: r#"
[[dimension]]
id = "total_width"
what = "w"
[[proposal]]
id = "honest_theorem"
sentence_kind = "theorem"
predicate = ["total_width: W any: proof, addition at a common scale performs no rescale so no width enters"]
"#,
        expect: &[],
    },
    Case {
        name: "a theorem row over a three-width sweep is reported",
        toml: r#"
[[dimension]]
id = "total_width"
what = "w"
[[proposal]]
id = "sweep_wearing_a_theorem"
sentence_kind = "theorem"
predicate = ["total_width: W in 3..=7"]
"#,
        expect: &["a-proof-asserted-only-at-row-level"],
    },
];

fn all(reg: &Registry) -> Vec<Finding> {
    let mut v = per_entry(reg);
    v.extend(exhaustive_without_a_container(reg));
    v.extend(a_proof_asserted_only_at_row_level(reg));
    v
}

fn main() {
    let mut bad = 0usize;

    println!("### 1. the split, against every committed entry");
    let c = canon();
    let entries = predicate::entries(&c);
    let mut reparsed = 0usize;
    let mut unreadable = 0usize;
    for (_, _, entry) in &entries {
        match read(entry) {
            None => unreadable += 1,
            Some(e) => {
                if e.warrant.is_some() {
                    reparsed += 1;
                    println!("  REPARSED: {entry}");
                }
            },
        }
    }
    println!("  entries: {}", entries.len());
    println!("  entries the new grammar reads differently from the old one: {reparsed}");
    println!("  entries neither grammar can split: {unreadable}");
    if reparsed != 0 {
        println!("  FAIL: the grammar is not append-only; it rewrites a committed entry.");
        bad += 1;
    }

    println!();
    println!("### 2. the planted cases");
    for case in CASES {
        let reg = parse("planted.toml", case.toml);
        let got: Vec<&str> = all(&reg).iter().map(|f| f.kind).collect();
        let ok = got.len() == case.expect.len()
            && case.expect.iter().all(|k| got.contains(k));
        println!("  [{}] {}", if ok { "ok" } else { "FAIL" }, case.name);
        println!("        expected {:?}", case.expect);
        println!("        got      {got:?}");
        if !ok {
            bad += 1;
        }
    }

    println!();
    println!("### 3. CONTROL. Every arm must be reachable, or a green run means nothing.");
    let reached: std::collections::BTreeSet<&str> = CASES.iter()
        .flat_map(|c| all(&parse("planted.toml", c.toml)))
        .map(|f| f.kind)
        .collect();
    let arms = [
        "warrant-is-not-a-known-token",
        "warrant-has-no-clause",
        "warrant-clause-is-a-bare-relabel",
        "exhaustive-over-an-unbounded-span",
        "exhaustive-names-no-container",
        "a-proof-asserted-only-at-row-level",
    ];
    for a in arms {
        let hit = reached.contains(a);
        println!("  [{}] {a}", if hit { "reached" } else { "NEVER FIRES" });
        if !hit {
            bad += 1;
        }
    }
    println!("  note: the arm drafted as `a-span-carrying-a-colon` was deleted. The control above");
    println!("        found it could never fire: `read` splits at the second colon, so a span");
    println!("        never contains one, and the case it was drafted for is refused by the");
    println!("        token arm instead. One arm, not two.");

    println!();
    println!("### 4. the committed canon under these arms");
    let found = all(&c);
    let mut by_kind: std::collections::BTreeMap<&str, Vec<&Finding>> = Default::default();
    for f in &found {
        by_kind.entry(f.kind).or_default().push(f);
    }
    for (kind, fs) in &by_kind {
        println!("  {kind}: {}", fs.len());
        for f in fs {
            println!("      {} :: {}", f.at, f.says);
        }
    }
    if by_kind.is_empty() {
        println!("  nothing, which given the census is the finder having stopped working");
        bad += 1;
    }

    println!();
    println!("### 5. what an empty registry does to the arms that already ship");
    println!("    Found by pointing this spike's `canon()` at the wrong directory. `load`");
    println!("    returns Ok(empty) for a path that is not a directory, so `canon()`'s");
    println!("    `.expect(\"mock/registry is readable\")` never fires and every arm goes");
    println!("    quiet at once. Reported, not fixed: the shipped crate is code tier and");
    println!("    the round is at TOPIC.");
    let empty = parse("empty.toml", "");
    let arms: &[(&str, fn(&Registry) -> Vec<arvo_checks::Finding>)] = &[
        ("predicate::undeclared_dimensions", predicate::undeclared_dimensions),
        ("predicate::repeated_dimensions", predicate::repeated_dimensions),
        ("shape::predicate_disagrees_with_the_sentence_kind", shape::predicate_disagrees_with_the_sentence_kind),
        ("shape::measured_without_evidence", shape::measured_without_evidence),
        ("shape::rulings_with_no_verbatim", shape::rulings_with_no_verbatim),
        ("shape::rows_with_no_keywords", shape::rows_with_no_keywords),
        ("citation::citations_with_no_target", citation::citations_with_no_target),
        ("provenance::standing_claims_more_arrivals_than_it_cites", provenance::standing_claims_more_arrivals_than_it_cites),
    ];
    let mut quiet = 0usize;
    for (name, arm) in arms {
        let n_empty = arm(&empty).len();
        let n_canon = arm(&c).len();
        println!("  {name}: empty registry -> {n_empty}, committed canon -> {n_canon}");
        if n_empty == 0 {
            quiet += 1;
        }
    }
    println!("  arms that say nothing about an empty registry: {quiet} of {}", arms.len());
    println!("  CONTROL: at least one arm must speak about the committed canon, or this");
    println!("           section is comparing two silences.");
    let speaks = arms.iter().filter(|(_, a)| !a(&c).is_empty()).count();
    println!("  arms that speak about the committed canon: {speaks}");
    if speaks == 0 {
        println!("  FAIL: no arm speaks about the real canon, so the comparison is vacuous");
        bad += 1;
    }

    println!();
    println!("### exit: {}", if bad == 0 { "every case behaved" } else { "SOMETHING FAILED" });
    std::process::exit(i32::from(bad != 0));
}
