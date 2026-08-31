//! Can a checker separate a named mechanism from a relabel without a blocklist?
//!
//! The blocklist arm has to anticipate every bad phrase, which is the defect
//! `checks/src/shape.rs` documents about its own retired word list. This inverts
//! it: rather than forbidding phrases, require that the clause names something
//! **outside the vocabulary of warranting**. A relabel is built entirely from
//! that vocabulary; a mechanism is not, because a mechanism is about the thing.
//!
//! The vocabulary that gets stripped is small and closed by construction: it is
//! the words this notation uses to talk about predicates and proofs, plus the
//! axis grammar. That is a set somebody can finish writing, which a blocklist of
//! bad phrases is not.
//!
//! Not shipping code. It answers one question: does the test separate the real
//! warrants this corpus already contains from the relabels a lazy author writes.

/// Words the notation uses to talk about itself, plus the axis grammar and
/// ordinary stop words. A clause built only from these has said nothing about
/// the thing it warrants.
const NOTATION_VOCABULARY: &[&str] = &[
    // warranting
    "proof", "proved", "proves", "construction", "constructed", "structural",
    "structurally", "trivially", "trivial", "obvious", "obviously", "follows",
    "holds", "exhaustive", "exhaustively", "swept", "sweep", "measured",
    "argument", "free", "clear", "evident", "immediate", "given",
    // the axis grammar and axis names
    "any", "in", "all", "every", "w", "f", "i", "n", "s", "width", "widths",
    "fraction", "integer", "signedness", "signed", "unsigned", "arity",
    "operation", "container", "threads", "rounding", "alignment", "radix",
    "toolchain", "strategy", "axis", "axes", "predicate", "region", "span",
    "value", "values", "range", "domain", "set", "bounded", "whole",
    // stop words
    "the", "a", "an", "of", "to", "and", "or", "is", "are", "be", "it", "its",
    "this", "that", "for", "by", "on", "at", "as", "with", "no", "not", "so",
    "cannot", "can", "does", "do", "enter", "enters", "entering", "here",
    "there", "which", "because", "since", "from", "into", "over", "under",
];

/// The content tokens a clause carries: what is left after the notation's own
/// vocabulary is removed.
fn content(clause: &str) -> Vec<String> {
    let mut seen: Vec<String> = Vec::new();
    for raw in clause.split(|c: char| !c.is_alphanumeric() && c != '_') {
        if raw.is_empty() {
            continue;
        }
        let w = raw.to_ascii_lowercase();
        if NOTATION_VOCABULARY.contains(&w.as_str()) {
            continue;
        }
        if w.chars().all(|c| c.is_ascii_digit()) {
            continue;
        }
        if !seen.contains(&w) {
            seen.push(w);
        }
    }
    seen
}

/// The threshold, set to one after the first run measured the separation at
/// zero against two. Three was my guess before measuring and it misclassified
/// the shortest real clause; the populations separate at any floor in (0, 2],
/// and one is the safest point in that interval.
const FLOOR: usize = 1;

struct Sample { verdict: &'static str, source: &'static str, clause: &'static str }

/// Every clause is from the committed corpus or is a relabel a lazy author
/// would write. Nothing here is invented to make the test look good, and the
/// borderline rows are kept in rather than dropped.
const SAMPLES: &[Sample] = &[
    // --- real warrants, all from the committed corpus -------------------------
    Sample { verdict: "mechanism", source: "predicate span, proposal.toml",
        clause: "the equalities being decided at compile time" },
    Sample { verdict: "mechanism", source: "predicate span, proposal.toml",
        clause: "the refusal being a type-check outcome that precedes execution" },
    Sample { verdict: "mechanism", source: "predicate span, proposal.toml",
        clause: "the splits being computed rather than executed on lanes" },
    Sample { verdict: "mechanism", source: "ruling::the_additive_and_absorption_verdicts_are_canon promotion",
        clause: "addition at a common scale performs no rescale" },
    Sample { verdict: "mechanism", source: "159_kiselyov_reply F6",
        clause: "size_of is denominated in bytes so every Rust type's bit size is a multiple of eight" },
    Sample { verdict: "mechanism", source: "159_kiselyov_reply F1",
        clause: "a width the compiler cannot know forces a range check, because the compiler's ignorance is the premise" },
    Sample { verdict: "mechanism", source: "dimension.toml access_pattern",
        clause: "a compile-time result is decided before any thread exists" },
    Sample { verdict: "mechanism", source: "ruling note, the rounding asymmetry",
        clause: "addition at a common scale performs no rescale so no rounding step exists" },
    // --- relabels -------------------------------------------------------------
    Sample { verdict: "relabel", source: "invented", clause: "by construction" },
    Sample { verdict: "relabel", source: "invented", clause: "structural" },
    Sample { verdict: "relabel", source: "invented", clause: "it is a proof" },
    Sample { verdict: "relabel", source: "invented", clause: "trivially, the width cannot enter" },
    Sample { verdict: "relabel", source: "invented", clause: "obvious from the argument" },
    Sample { verdict: "relabel", source: "invented", clause: "holds for any width" },
    Sample { verdict: "relabel", source: "invented", clause: "the argument is width-free by construction" },
    Sample { verdict: "relabel", source: "invented", clause: "proved for all widths" },
    // --- the hard cases, kept in on purpose -----------------------------------
    Sample { verdict: "relabel", source: "invented, the adversarial one",
        clause: "the width cannot enter the argument by construction of the proof" },
    Sample { verdict: "mechanism", source: "the shortest real one I could find",
        clause: "no rescale occurs" },
];

fn main() {
    println!("### the test: strip the notation's own vocabulary, require {FLOOR} distinct content tokens");
    println!();
    let mut wrong = 0usize;
    let mut worst_mechanism = usize::MAX;
    let mut best_relabel = 0usize;
    for s in SAMPLES {
        let c = content(s.clause);
        let passes = c.len() >= FLOOR;
        let called = if passes { "mechanism" } else { "relabel" };
        let ok = called == s.verdict;
        if !ok {
            wrong += 1;
        }
        if s.verdict == "mechanism" {
            worst_mechanism = worst_mechanism.min(c.len());
        } else {
            best_relabel = best_relabel.max(c.len());
        }
        println!("  [{}] {:>9} -> {:<9} n={}  {:?}", if ok { "ok" } else { "MISS" },
                 s.verdict, called, c.len(), c);
        println!("        {:?}", s.clause);
        println!("        source: {}", s.source);
    }
    println!();
    println!("### separation");
    println!("  quietest mechanism: {worst_mechanism} content tokens");
    println!("  loudest relabel:    {best_relabel} content tokens");
    println!("  misclassified:      {wrong} of {}", SAMPLES.len());
    if worst_mechanism > best_relabel {
        println!("  the two populations are separated, and any floor in ({best_relabel}, {worst_mechanism}] works");
    } else {
        println!("  the populations OVERLAP, so no floor separates them and the test is a heuristic");
        println!("  rather than a discriminator. That is the honest reading and it is what to report.");
    }

    println!();
    println!("### CONTROL. The stripper must actually strip, and must not strip everything.");
    let all_vocab = content("by construction the width holds for any axis");
    let all_content = content("rescale denominated lanes");
    println!("  a clause of pure notation vocabulary -> {all_vocab:?} (must be empty)");
    println!("  a clause of pure content            -> {all_content:?} (must be 3)");
    assert!(all_vocab.is_empty(), "the stripper is not stripping");
    assert_eq!(all_content.len(), 3, "the stripper is eating content");
    println!("  controls held.");
}
