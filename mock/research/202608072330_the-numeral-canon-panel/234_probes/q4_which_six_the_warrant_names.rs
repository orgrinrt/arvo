// q4. `228`'s finding F5 carries the warrant
//
//     rounding: rounding any: exhaustive, all six named modes
//
// `ruling::the_warrant_is_a_token_and_a_clause_on_the_values_side` defines that
// token as "`exhaustive` for a span walked in full, with the clause naming the
// domain the span is the whole of". So the clause "all six named modes" is a
// claim about which six the run covered, and it is checkable against two files
// that are both on disk.
//
// This reads the ratified six out of the registry row that fixes them, reads the
// six the instrument actually swept out of the instrument's own source, and
// diffs the two sets. It asserts nothing about which set is right; it reports
// whether they are the same set, which is what the clause asserts.
//
// Neither list is typed in here. Both are parsed, so this cannot drift from
// either file, and if either parse comes back with a count other than six the
// probe fails rather than comparing two sets it did not read.
//
// THE CASES THAT MUST FAIL, stated before the run:
//
//   C1  the parsers must each find exactly six names. A parser that finds none
//       reports two empty sets as equal, which is the shape that would make this
//       probe agree with `228` for the worst possible reason.
//   C2  a planted list that is the ratified six must diff to nothing, and a
//       planted list one name off must diff to exactly that name. Without both,
//       the differ is not known to be able to report either answer.
//
// Run from this directory, so the two relative paths resolve against the tree
// this probe sits in rather than against somebody else's clone:
//
//   rustc --edition 2024 -O -o q4 q4_which_six_the_warrant_names.rs && ./q4

use std::collections::BTreeSet;
use std::path::Path;

const RULING: &str = "../../../registry/ruling.toml";
const INSTRUMENT: &str = "../228_probes/p3_retraction_over_the_whole_vocabulary.rs";

/// Every backtick-delimited token on a line.
fn backticked(s: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut it = s.split('`');
    let _ = it.next();
    let mut take = true;
    for part in it {
        if take {
            out.push(part.to_string());
        }
        take = !take;
    }
    out
}

/// The ratified vocabulary, read out of the ruling's `says` field.
fn ratified_six(text: &str) -> BTreeSet<String> {
    let mut out = BTreeSet::new();
    for line in text.lines() {
        if line.starts_with("says = ") && line.contains("rounding mode vocabulary is") {
            // The sentence ends at the first full stop after the list.
            let after = line.split("vocabulary is").nth(1).unwrap_or("");
            let sentence = after.split(". ").next().unwrap_or("");
            for t in backticked(sentence) {
                if !t.trim().is_empty() {
                    out.insert(t);
                }
            }
        }
    }
    out
}

/// The modes the instrument swept, read out of its `MODES` array and its own
/// `name` function, so the spellings are the instrument's rather than mine.
fn swept(text: &str) -> BTreeSet<String> {
    let mut variants: Vec<String> = Vec::new();
    for line in text.lines() {
        if line.trim_start().starts_with("const MODES") {
            let inner = line
                .split('[')
                .nth(2)
                .and_then(|s| s.split(']').next())
                .unwrap_or("");
            for v in inner.split(',') {
                let v = v.trim();
                if !v.is_empty() {
                    variants.push(v.to_string());
                }
            }
        }
    }
    let mut out = BTreeSet::new();
    for v in &variants {
        // `Variant => "spelling",` inside the instrument's own name function.
        let needle = format!("{v} => \"");
        let hit = text
            .lines()
            .find(|l| l.trim_start().starts_with(&needle))
            .and_then(|l| l.split('"').nth(1))
            .map(|s| s.to_string());
        match hit {
            Some(s) => {
                out.insert(s);
            }
            None => {
                out.insert(format!("<{v}: no spelling found>"));
            }
        }
    }
    out
}

fn show(s: &BTreeSet<String>) -> String {
    s.iter().cloned().collect::<Vec<_>>().join(", ")
}

fn main() {
    let mut sound = true;

    for p in [RULING, INSTRUMENT] {
        if !Path::new(p).exists() {
            eprintln!("cannot read {p}: run this from the directory the probe sits in");
            std::process::exit(2);
        }
    }
    let ruling = std::fs::read_to_string(RULING).unwrap();
    let instrument = std::fs::read_to_string(INSTRUMENT).unwrap();

    let canon = ratified_six(&ruling);
    let run = swept(&instrument);

    println!("q4. which six the `exhaustive, all six named modes` clause names");
    println!();
    println!(
        "ratified, from ruling::the_ambiguous_rounding_word_is_retired_for_six_explicit_names"
    );
    println!("  ({}) {}", canon.len(), show(&canon));
    println!("swept, from 228_probes/p3_retraction_over_the_whole_vocabulary.rs `MODES`");
    println!("  ({}) {}", run.len(), show(&run));

    println!();
    println!("== C1 must-fail: each parse must find exactly six ==");
    if canon.len() == 6 && run.len() == 6 {
        println!("  C1 ok: six and six, so the comparison below is between two real sets");
    } else {
        println!(
            "  C1 FAILED: parsed {} ratified and {} swept; a set this probe did not read",
            canon.len(),
            run.len()
        );
        println!("             cannot be compared, and an empty set would compare equal");
        sound = false;
    }

    let missing: Vec<&String> = canon.difference(&run).collect();
    let extra: Vec<&String> = run.difference(&canon).collect();

    println!();
    println!("== the diff ==");
    println!("  ratified but never swept: {missing:?}");
    println!("  swept but not ratified:   {extra:?}");

    println!();
    println!("== C2 must-fail: the differ must report both answers ==");
    let same: BTreeSet<String> = canon.clone();
    let one_off: BTreeSet<String> = canon
        .iter()
        .cloned()
        .filter(|s| s != "stochastic")
        .chain(std::iter::once("away_from_zero".to_string()))
        .collect();
    let d_same: Vec<&String> = canon.difference(&same).collect();
    let d_off: Vec<&String> = canon.difference(&one_off).collect();
    println!("  planted identical list  -> missing {d_same:?}");
    println!("  planted one-name-off    -> missing {d_off:?}");
    if d_same.is_empty() && d_off.len() == 1 {
        println!("  C2 ok: the differ reports nothing on an identical set and exactly the");
        println!("         one name on a set one off, so the diff above is earned");
    } else {
        println!("  C2 FAILED: the differ cannot distinguish the two planted cases");
        sound = false;
    }

    println!();
    println!("== WHAT THIS REFUTES ==");
    if missing.is_empty() && extra.is_empty() {
        println!("Nothing. The two sets agree and `228`'s clause is accurate.");
    } else {
        println!("`228` finding F5's warrant clause, `exhaustive, all six named modes`.");
        println!("The span it carries is `rounding any`, which under");
        println!("`dimension::rounding` spans every value of the axis, and the clause names");
        println!("the domain as the six named modes. The run covered a different six: it");
        println!("substituted {extra:?} for {missing:?}.");
        println!();
        println!("So the one ratified name the sweep never reached is exactly the one that");
        println!("cannot be swept, and the one it did reach is not a value of the axis the");
        println!("ratified vocabulary declares. Under the warrant ruling, `exhaustive`");
        println!("obliges the clause to name the domain the span is the whole of. This");
        println!("clause names a domain the run was not the whole of.");
        println!();
        println!("The neighbouring finding F4 does this correctly on the same row: its");
        println!("clause reads `swept, five ratified deterministic modes and away-from-zero");
        println!("at all 108 cells, stochastic by construction`, which names what ran. F5");
        println!("compresses the same run into a phrase that names something else.");
    }

    println!();
    println!("instrument: {}", if sound { "sound" } else { "UNSOUND" });
    if !sound {
        std::process::exit(1);
    }
}
