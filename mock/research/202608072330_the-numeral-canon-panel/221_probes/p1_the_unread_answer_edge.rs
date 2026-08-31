// p1: which answer edges into `question` does anything actually read?
//
// WHY THIS RUNS. `mock/checks/tests/a_settled_question_does_not_sit_in_the_queue.rs`
// exists because a question op already answered renders as open and goes back
// into his queue. Its own module doc measures the fields nothing reads:
// `decider` 0, `bound` 0, `unblocks` 0, `answers` 0. It then repairs exactly one
// of them, `ruling.answers`, by building a reverse index in `answered_by()`.
//
// `answered_by()` iterates `reg.of("ruling")` and nothing else. The `proposal`
// namespace has an `answers` field too, and 99 + 23 proposal rows use it. So the
// check that exists to stop a settled question reading as open reads one of the
// two namespaces that settle questions.
//
// This probe measures the size of what is left unread, and separates out the
// part that is not merely a strong proposal but a proposition op has RATIFIED,
// which reaches a question through `ruling.ratifies -> proposal -> proposal.answers`.
// Nothing walks that path either.
//
// THE THREE RULES COMPARED.
//   RULE_A  what the shipped check sees: `answered` non-empty, or a settled
//           phrase in `note` / `bound`, or some `ruling.answers` naming it.
//           Transcribed from SETTLED_PHRASES / PROSE_FIELDS / ANSWER_FIELD /
//           answered_by() in that file.
//   RULE_B  RULE_A, plus any `proposal.answers` naming it.
//   RULE_C  RULE_A, plus a `proposal.answers` naming it where that proposal is
//           in the `ratifies` list of a ruling at `rung = "ratified"`.
//
// WHAT MUST FAIL, declared before the run. Four controls, and the run is void
// if any reports other than its required verdict.
//   C1  Over the committed canon, RULE_A must see a non-empty settled set. If
//       the scanner cannot read `ruling.answers` at all then every question
//       looks unsettled under RULE_A and the gap below is an artifact of the
//       reader. The shipped check's own floor arm asserts >= 4; this asserts
//       the same thing about this scanner.
//   C2  On a planted registry carrying ONLY a `ruling.answers` edge, RULE_A
//       must catch it. Without this, RULE_A is a rule that sees nothing and
//       the comparison is vacuous.
//   C3  On a planted registry carrying NO `proposal.answers` edge at all,
//       RULE_B minus RULE_A must be empty. Without this, RULE_B is reporting
//       something other than the proposal edge.
//   C4  On a planted registry where a proposal answers an unanswered question
//       and no ruling does, RULE_A must MISS it and RULE_B must CATCH it. This
//       is the case that must fail for the finding to exist at all: if RULE_A
//       catches it there is no gap and this probe counts nothing.
//
// SCOPE. The committed registry at `mock/registry/*.toml`, read through a
// line-oriented scanner rather than a TOML parser, plus four planted inputs.
// The scanner's adequacy is exactly what C1 tests.

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

// Transcribed from the shipped check, not paraphrased.
const SETTLED_PHRASES: &[&str] = &["recorded as answered", "recorded as closed"];
const PROSE_FIELDS: &[&str] = &["note", "bound"];
const ANSWER_FIELD: &str = "answered";

#[derive(Debug, Clone)]
struct Row {
    ns: String,
    id: String,
    body: String,
}

impl Row {
    fn scalar(&self, key: &str) -> Option<String> {
        // `key = "value"`, `key = '''...'''`, `key = """..."""`.
        let pat = format!("\n{key} = ");
        let hay = format!("\n{}", self.body);
        let at = hay.find(&pat)? + pat.len();
        let rest = &hay[at..];
        for fence in ["'''", "\"\"\""] {
            if let Some(r) = rest.strip_prefix(fence) {
                let end = r.find(fence)?;
                return Some(r[..end].to_string());
            }
        }
        if let Some(r) = rest.strip_prefix('"') {
            let end = r.find('"')?;
            return Some(r[..end].to_string());
        }
        let end = rest.find('\n').unwrap_or(rest.len());
        Some(rest[..end].trim().to_string())
    }

    fn has(&self, key: &str) -> bool {
        self.body.starts_with(&format!("{key} = ")) || self.body.contains(&format!("\n{key} = "))
    }

    fn list(&self, key: &str) -> Vec<String> {
        let pat = format!("\n{key} = [");
        let hay = format!("\n{}", self.body);
        let Some(at) = hay.find(&pat) else {
            return Vec::new();
        };
        let rest = &hay[at + pat.len()..];
        let Some(end) = rest.find(']') else {
            return Vec::new();
        };
        let inner = &rest[..end];
        let mut out = Vec::new();
        let mut it = inner.char_indices();
        while let Some((i, c)) = it.next() {
            if c == '"' {
                let tail = &inner[i + 1..];
                if let Some(j) = tail.find('"') {
                    out.push(tail[..j].to_string());
                    for _ in 0..=j {
                        it.next();
                    }
                }
            }
        }
        out
    }
}

fn scan(text: &str) -> Vec<Row> {
    let mut out: Vec<Row> = Vec::new();
    let mut cur: Option<(String, String)> = None;
    for line in text.lines() {
        let t = line.trim();
        if t.starts_with("[[") && t.ends_with("]]") {
            if let Some((ns, body)) = cur.take() {
                push(&mut out, ns, body);
            }
            cur = Some((t[2..t.len() - 2].to_string(), String::new()));
        } else if let Some((_, body)) = cur.as_mut() {
            body.push_str(line);
            body.push('\n');
        }
    }
    if let Some((ns, body)) = cur.take() {
        push(&mut out, ns, body);
    }
    out
}

fn push(out: &mut Vec<Row>, ns: String, body: String) {
    let row = Row {
        ns,
        id: String::new(),
        body,
    };
    let id = row.scalar("id").unwrap_or_default();
    out.push(Row { id, ..row });
}

fn load(dir: &Path) -> Vec<Row> {
    let mut files: Vec<PathBuf> = std::fs::read_dir(dir)
        .unwrap_or_else(|e| panic!("cannot read {}: {e}", dir.display()))
        .filter_map(|e| e.ok().map(|e| e.path()))
        .filter(|p| p.extension().is_some_and(|x| x == "toml"))
        .collect();
    files.sort();
    assert!(!files.is_empty(), "no registry files under {}", dir.display());
    let mut out = Vec::new();
    for f in files {
        out.extend(scan(&std::fs::read_to_string(&f).unwrap()));
    }
    out
}

struct Reg {
    rows: Vec<Row>,
}

impl Reg {
    fn of<'a>(&'a self, ns: &'a str) -> impl Iterator<Item = &'a Row> {
        self.rows.iter().filter(move |r| r.ns == ns)
    }

    /// Exactly what the shipped check computes.
    fn answered_by_ruling(&self) -> BTreeMap<String, Vec<String>> {
        let mut out: BTreeMap<String, Vec<String>> = BTreeMap::new();
        for r in self.of("ruling") {
            for q in r.list("answers") {
                out.entry(q).or_default().push(r.id.clone());
            }
        }
        out
    }

    fn answered_by_proposal(&self) -> BTreeMap<String, Vec<String>> {
        let mut out: BTreeMap<String, Vec<String>> = BTreeMap::new();
        for p in self.of("proposal") {
            for q in p.list("answers") {
                out.entry(q).or_default().push(p.id.clone());
            }
        }
        out
    }

    /// Propositions a ruling at `rung = "ratified"` names in `ratifies`.
    fn ratified_propositions(&self) -> BTreeMap<String, String> {
        let mut out = BTreeMap::new();
        for r in self.of("ruling") {
            if r.scalar("rung").as_deref() == Some("ratified") {
                for p in r.list("ratifies") {
                    out.insert(p, r.id.clone());
                }
            }
        }
        out
    }

    /// RULE_A. Transcribed from the shipped check.
    fn rule_a(&self) -> BTreeSet<String> {
        let by_ruling = self.answered_by_ruling();
        let mut out = BTreeSet::new();
        for q in self.of("question") {
            let carries = q
                .scalar(ANSWER_FIELD)
                .is_some_and(|t| !t.trim().is_empty())
                && q.has(ANSWER_FIELD);
            let phrase = PROSE_FIELDS.iter().any(|f| {
                q.scalar(f).is_some_and(|t| {
                    let l = t.to_lowercase();
                    SETTLED_PHRASES.iter().any(|p| l.contains(p))
                })
            });
            if carries || phrase || by_ruling.contains_key(&q.id) {
                out.insert(q.id.clone());
            }
        }
        out
    }

    fn questions(&self) -> BTreeSet<String> {
        self.of("question").map(|q| q.id.clone()).collect()
    }
}

fn root() -> PathBuf {
    // Resolved from this file's own location, never from a checkout path.
    let here = Path::new(file!())
        .parent()
        .map(|p| p.to_path_buf())
        .unwrap_or_else(|| PathBuf::from("."));
    let mut p = std::fs::canonicalize(&here).unwrap_or(here);
    for _ in 0..4 {
        p = p.parent().unwrap().to_path_buf();
    }
    p
}

fn planted(text: &str) -> Reg {
    Reg { rows: scan(text) }
}

fn main() {
    println!("### p1. which answer edges into `question` does anything read?");
    let dir = root().join("mock/registry");
    println!("### registry: {}", dir.display());
    let reg = Reg { rows: load(&dir) };
    println!(
        "### rows: {} question, {} ruling, {} proposal",
        reg.of("question").count(),
        reg.of("ruling").count(),
        reg.of("proposal").count()
    );
    println!();

    let mut void = false;

    // --- C1 ----------------------------------------------------------------
    let by_ruling = reg.answered_by_ruling();
    let ok = by_ruling.len() >= 4;
    println!("CONTROLS");
    println!(
        "  C1  RULE_A sees a non-empty ruling edge over the committed canon   {:>9}  required=>=4      got {}",
        if ok { "as required" } else { "*** VOID ***" },
        by_ruling.len()
    );
    void |= !ok;

    // --- C2 ----------------------------------------------------------------
    let p2 = planted(
        r#"
[[question]]
id = "q_ruling"
asks = "?"

[[ruling]]
id = "r"
rung = "ratified"
answers = ["q_ruling"]
"#,
    );
    let c2 = p2.rule_a().contains("q_ruling");
    println!(
        "  C2  a planted ruling edge is caught by RULE_A                      {:>9}  required=caught",
        if c2 { "as required" } else { "*** VOID ***" }
    );
    void |= !c2;

    // --- C3 ----------------------------------------------------------------
    let p3 = planted(
        r#"
[[question]]
id = "q_open"
asks = "?"

[[question]]
id = "q_shut"
asks = "?"
answered = "yes"

[[ruling]]
id = "r"
rung = "stated"
"#,
    );
    let a3 = p3.rule_a();
    let b3: BTreeSet<String> = p3
        .answered_by_proposal()
        .keys()
        .cloned()
        .filter(|q| p3.questions().contains(q))
        .collect();
    let c3 = b3.difference(&a3).count() == 0;
    println!(
        "  C3  no proposal edge means RULE_B adds nothing                     {:>9}  required=empty",
        if c3 { "as required" } else { "*** VOID ***" }
    );
    void |= !c3;

    // --- C4, the case that must fail --------------------------------------
    let p4 = planted(
        r#"
[[question]]
id = "q_by_proposal"
asks = "?"

[[proposal]]
id = "p"
standing = "two_experts"
answers = ["q_by_proposal"]

[[ruling]]
id = "r"
rung = "ratified"
ratifies = ["p"]
"#,
    );
    let a4 = p4.rule_a();
    let b4: BTreeSet<String> = p4.answered_by_proposal().keys().cloned().collect();
    let c4 = !a4.contains("q_by_proposal") && b4.contains("q_by_proposal");
    println!(
        "  C4  a proposal-only edge is MISSED by RULE_A and caught by RULE_B  {:>9}  required=missed+caught",
        if c4 { "as required" } else { "*** VOID ***" }
    );
    void |= !c4;
    println!();

    if void {
        println!("*** A CONTROL DID NOT REPORT ITS REQUIRED VERDICT. NOTHING BELOW COUNTS. ***");
        std::process::exit(1);
    }

    // --- the measurement ---------------------------------------------------
    let questions = reg.questions();
    let a = reg.rule_a();
    let by_prop = reg.answered_by_proposal();
    let ratified = reg.ratified_propositions();

    let unanswered: BTreeSet<String> = questions
        .iter()
        .filter(|q| !a.contains(*q))
        .cloned()
        .collect();

    let mut b_only: Vec<(String, Vec<String>)> = Vec::new();
    let mut c_only: Vec<(String, Vec<(String, String)>)> = Vec::new();
    for q in &unanswered {
        if let Some(ps) = by_prop.get(q) {
            b_only.push((q.clone(), ps.clone()));
            let rat: Vec<(String, String)> = ps
                .iter()
                .filter_map(|p| ratified.get(p).map(|r| (p.clone(), r.clone())))
                .collect();
            if !rat.is_empty() {
                c_only.push((q.clone(), rat));
            }
        }
    }

    println!("MEASUREMENT");
    println!("  questions declared                                    {}", questions.len());
    println!("  settled under RULE_A (what the shipped check sees)    {}", a.len());
    println!("  reading as open under RULE_A                          {}", unanswered.len());
    println!("  of those, named by a `proposal.answers` edge          {}", b_only.len());
    println!("  of those, backed by a RATIFIED proposition            {}", c_only.len());
    println!();

    println!("THE RATIFIED-BACKED SET (a question op has answered, reading as open)");
    for (q, rat) in &c_only {
        for (p, r) in rat {
            println!("  question::{q}");
            println!("      <- proposal::{p}");
            println!("      <- ruling::{r}  (rung = ratified)");
        }
    }
    println!();

    println!("THE WIDER SET (proposal edge, unread)");
    for (q, ps) in &b_only {
        println!("  question::{q}");
        for p in ps {
            println!("      <- proposal::{p}");
        }
    }
    println!();

    // Dangling edges: a proposal answering something that is not a question.
    let mut dangling: Vec<(String, String)> = Vec::new();
    for (q, ps) in &by_prop {
        if !questions.contains(q) {
            for p in ps {
                dangling.push((p.clone(), q.clone()));
            }
        }
    }
    println!("REFERENTIAL INTEGRITY ON THE UNREAD EDGE");
    println!("  `proposal.answers` entries naming no declared question: {}", dangling.len());
    for (p, q) in &dangling {
        println!("    proposal::{p} -> question::{q}  (declared nowhere)");
    }
}
