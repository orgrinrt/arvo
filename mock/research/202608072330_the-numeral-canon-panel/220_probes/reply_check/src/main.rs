//! Re-measuring 219's numbers, and testing two readings of the corpus.
//!
//! Not shipping code. Every question here is one 219 answered with its own
//! scanner; the point of asking again through a different parser is that an
//! agreement then counts as two instruments rather than as one method run twice.
//!
//! Q1 to Q3 reproduce. Q4 to Q6 are the attacks.

use arvo_checks::{load, predicate, Registry, Row};
use std::collections::BTreeMap;
use std::path::PathBuf;

fn canon() -> Registry {
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../../registry");
    assert!(dir.is_dir(), "the registry is not at {}", dir.display());
    let reg = load(&dir).expect("the registry parses");
    assert!(
        !reg.rows.is_empty(),
        "no rows loaded, so every count below is vacuous"
    );
    reg
}

/// Whether a values side writes the universal for its axis.
///
/// Deliberately the loose reading: any standalone `any` token. That is what 219
/// counted and reproducing a number means counting the same thing.
fn is_universal(values: &str) -> bool {
    values
        .split(|c: char| !c.is_alphanumeric() && c != '_')
        .any(|w| w == "any")
}

fn is_bounded(values: &str) -> bool {
    !is_universal(values) && !values.trim().is_empty()
}

/// Every predicate entry grouped by the row it sits on.
fn by_row(reg: &Registry) -> BTreeMap<String, (&Row, Vec<(String, String)>)> {
    let mut out: BTreeMap<String, (&Row, Vec<(String, String)>)> = BTreeMap::new();
    for (row, _field, entry) in predicate::entries(reg) {
        let Some((slug, values)) = predicate::split(entry) else {
            continue;
        };
        out.entry(row.addr())
            .or_insert_with(|| (row, Vec::new()))
            .1
            .push((slug.to_string(), values.to_string()));
    }
    out
}

fn main() {
    let c = canon();
    let entries = predicate::entries(&c);
    let rows = by_row(&c);

    println!("### Q1 (reproduces 219). total predicate entries, and how many write the universal");
    let universals: Vec<_> = entries
        .iter()
        .filter_map(|(r, f, e)| predicate::split(e).map(|(s, v)| (r, *f, s, v)))
        .filter(|(_, _, _, v)| is_universal(v))
        .collect();
    println!("  entries: {}   219 says 527", entries.len());
    println!("  universal entries: {}   219 says 38", universals.len());
    let mut spellings: BTreeMap<&str, usize> = BTreeMap::new();
    for (_, _, _, v) in &universals {
        *spellings.entry(v).or_default() += 1;
    }
    for (v, n) in &spellings {
        println!("      {n:>3}  {v}");
    }

    println!();
    println!("### Q2 (reproduces 219). rows carrying a universal and a bounded entry together");
    let mixed: Vec<&String> = rows
        .iter()
        .filter(|(_, (_, es))| {
            es.iter().any(|(_, v)| is_universal(v)) && es.iter().any(|(_, v)| is_bounded(v))
        })
        .map(|(addr, _)| addr)
        .collect();
    println!("  mixed rows: {}   219 says 25", mixed.len());
    let mut kinds: BTreeMap<&str, usize> = BTreeMap::new();
    for addr in &mixed {
        let (row, _) = &rows[*addr];
        *kinds.entry(row.get("sentence_kind").unwrap_or("(none)")).or_default() += 1;
    }
    println!("  their sentence_kind: {kinds:?}");
    println!("  219 says {{\"(none)\": 7, \"argument\": 7, \"measured\": 7, \"theorem\": 4}}");

    println!();
    println!("### Q3 (reproduces 219). slug sides that are not a bare lowercase slug");
    let dirty: Vec<&str> = entries
        .iter()
        .filter_map(|(_, _, e)| predicate::split(e))
        .map(|(s, _)| s)
        .filter(|s| !s.chars().all(|ch| ch.is_ascii_lowercase() || ch == '_'))
        .collect();
    println!("  dirty slugs: {}   219 says 0", dirty.len());

    println!();
    println!("### Q4 (ATTACK). the whole-domain range, literal against parameterised");
    println!("  219 searched for `1..=64` and `0..=63`, found zero, and concluded the");
    println!("  registry holds no whole-container range at all. The literal search is");
    println!("  right. Here is every range entry, so the class can be judged rather than");
    println!("  the two spellings.");
    let ranges: Vec<_> = entries
        .iter()
        .filter_map(|(r, f, e)| predicate::split(e).map(|(s, v)| (r, *f, s, v)))
        .filter(|(_, _, _, v)| v.contains("..="))
        .collect();
    println!("  range entries: {}", ranges.len());
    let literal = ranges.iter().filter(|(_, _, _, v)| v.contains("1..=64") || v.contains("0..=63")).count();
    println!("  matching 219's two literals: {literal}");
    println!("  ranges whose upper bound is itself an axis or a formula, which is what a");
    println!("  whole domain looks like when the domain is parameterised:");
    let mut parameterised = 0usize;
    for (r, f, s, v) in &ranges {
        let after: String = v.split("..=").nth(1).unwrap_or("").trim().to_string();
        let head: String = after
            .chars()
            .take_while(|ch| ch.is_alphanumeric() || *ch == '_' || *ch == '-' || *ch == ' ')
            .collect();
        let bound = head.trim();
        if !bound.is_empty() && !bound.chars().next().unwrap().is_ascii_digit() {
            parameterised += 1;
            println!("      {} :: {f} :: {s} => {v}", r.addr());
        }
    }
    println!("  parameterised-bound ranges: {parameterised}");

    println!();
    println!("### Q5 (ATTACK). is `construction` already a word in service in the registry?");
    println!("  219 rejects `proof` partly on the ground that it is a claim about the");
    println!("  sentence. The counter-question is whether its own token collides.");
    let mut hits = 0usize;
    for row in &c.rows {
        for (field, text) in &row.strings {
            let low = text.to_ascii_lowercase();
            if low.contains("construction") {
                hits += 1;
                if hits <= 12 {
                    let at = low.find("construction").unwrap();
                    let start = at.saturating_sub(46);
                    let end = (at + 40).min(low.len());
                    println!("      {}::{field} ... {}", row.addr(), &text[start..end].replace('\n', " "));
                }
            }
        }
    }
    println!("  fields whose text contains `construction`: {hits}");
    let ids = c.rows.iter().filter(|r| r.id.contains("construction")).count();
    println!("  row ids containing `construction`: {ids}");

    println!();
    println!("### Q6 (ATTACK). what 219's Arm 1 would cost: does a citation target exist?");
    println!("  Arm 1 requires a `construction` field naming a registry row or a probe.");
    println!("  Where a row carrying an unmarked universal has no outbound reference at");
    println!("  all, its author must first create a target before the warrant is");
    println!("  writable. That is the friction the ruling's `because` blames for the gap.");
    const OUTBOUND: &[&str] = &["law", "answers", "evidence", "obligation", "supersedes", "instead", "gate"];
    let mut with = 0usize;
    let mut without: Vec<String> = Vec::new();
    let mut seen: Vec<String> = Vec::new();
    for (r, _f, _s, _v) in &universals {
        if seen.contains(&r.addr()) {
            continue;
        }
        seen.push(r.addr());
        if OUTBOUND.iter().any(|f| !r.list(f).is_empty() || r.get(f).is_some()) {
            with += 1;
        } else {
            without.push(r.addr());
        }
    }
    println!("  distinct rows carrying an unmarked universal: {}", seen.len());
    println!("  of those, carrying at least one outbound reference: {with}");
    println!("  carrying none, so Arm 1 makes them write a new row first: {}", without.len());
    for a in &without {
        println!("      {a}");
    }

    println!();
    println!("### CONTROL. Every arm above must be able to return something other than its");
    println!("    committed answer, or the reproductions are a fixture agreeing with itself.");
    let planted = arvo_checks::parse(
        "planted.toml",
        r#"
[[dimension]]
id = "total_width"
what = "w"
[[dimension]]
id = "threads"
what = "t"
[[proposal]]
id = "planted_mixed"
sentence_kind = "measured"
predicate = ["total_width: W any", "threads: threads = 1"]
[[proposal]]
id = "planted_dirty"
sentence_kind = "measured"
predicate = ["Total Width: W in 1..=64"]
[[proposal]]
id = "planted_no_outbound"
sentence_kind = "measured"
predicate = ["threads: threads any"]
"#,
    );
    let pe = predicate::entries(&planted);
    let pu = pe
        .iter()
        .filter_map(|(_, _, e)| predicate::split(e))
        .filter(|(_, v)| is_universal(v))
        .count();
    let pd = pe
        .iter()
        .filter_map(|(_, _, e)| predicate::split(e))
        .map(|(s, _)| s)
        .filter(|s| !s.chars().all(|ch| ch.is_ascii_lowercase() || ch == '_'))
        .count();
    let prows = by_row(&planted);
    let pmixed = prows
        .values()
        .filter(|(_, es)| {
            es.iter().any(|(_, v)| is_universal(v)) && es.iter().any(|(_, v)| is_bounded(v))
        })
        .count();
    println!("  planted entries: {} (want 4)", pe.len());
    println!("  planted universals: {pu} (want 2)");
    println!("  planted dirty slugs: {pd} (want 1, where the committed answer is 0)");
    println!("  planted mixed rows: {pmixed} (want 1)");
    assert_eq!(pe.len(), 4);
    assert_eq!(pu, 2);
    assert_eq!(pd, 1, "the dirty-slug arm cannot fire, so its committed zero means nothing");
    assert_eq!(pmixed, 1);
    println!("  controls held: every arm returned a non-committed value on a planted tree.");
}
