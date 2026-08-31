//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! The panel's catalogues are TOML that nothing reads, so nothing catches a
//! broken one.
//!
//! Several sit under the panel directory and one under `mock/registry_catalogue/`.
//! None is inside `canon_paths`, so the engine never opens them, no other check
//! in this crate names them, and `cargo mock` walks past them. A stray quote in
//! any one would sit there indefinitely, and the file it breaks is the one a
//! later seat reads instead of re-sweeping the archive by hand. That is the whole
//! point of a catalogue and it is exactly what a silent parse failure destroys.
//!
//! **The bar here is about readability, not content.** The catalogues disagree
//! with each other on purpose: one sorts op's statements by what became of them,
//! one sorts panel files by whether their content was absorbed, one sorts
//! measured claims by whether they can be carried. Pinning a shared vocabulary
//! would force one shape onto all of them and would be this crate inventing a
//! schema nobody asked for. So what is asserted is what every catalogue must
//! satisfy whatever it is about: it parses, its rows can be named, no row claims
//! an identifier another row already claims, and a number meaning a confidence is
//! a number in the range a confidence can take.
//!
//! **Both of the first two arms were wrong when first written, and each was
//! corrected by a catalogue rather than by argument.** The first demanded `id`,
//! which the file-keyed catalogues do not carry and should not. The second then
//! demanded uniqueness of whatever key was found, which broke on a catalogue that
//! deliberately splits one panel file into three rows with three different
//! verdicts. Both corrections narrowed the assertion onto the property and away
//! from the spelling, which is the only direction a test may be loosened in.

use std::fs;
use std::path::PathBuf;

use arvo_checks::repo;
use toml_edit::{DocumentMut, Item, Value};

/// The keys a catalogue names its rows by.
///
/// Three spellings of one property. `id` where the row is a statement and the
/// slug is its name; `path` and `file` where the row IS a file and the path is
/// its name. All three are identities and none is wrong.
const IDENTITY_KEYS: &[&str] = &["id", "path", "file"];

/// The identity key that also promises uniqueness, and it is the only one.
///
/// `file` and `path` name a subject rather than a row. A catalogue may
/// legitimately carry one file three times, once per section group, when the
/// groups have different verdicts, and one does: splitting it is better work than
/// flattening three dispositions into one row would have been. `id` is a slug
/// somebody minted to be cited, so two rows answering to it is a real defect.
const UNIQUE_KEY: &str = "id";

/// Every committed catalogue, found rather than listed.
///
/// Listed, this goes stale the first time somebody adds one, and the whole
/// reason these files exist is that the next seat adds another. So the walk is
/// over the tree and the population is measured rather than pinned.
fn catalogues() -> Vec<PathBuf> {
    let mut out = Vec::new();
    let roots = [
        repo().join("mock/research/202608072330_the-numeral-canon-panel"),
        repo().join("mock/research/202608072330_the-numeral-canon-panel/catalogue"),
        repo().join("mock/registry_catalogue"),
    ];
    for root in roots {
        let Ok(entries) = fs::read_dir(&root) else {
            continue;
        };
        for e in entries.flatten() {
            let p = e.path();
            if p.extension().is_some_and(|x| x == "toml") {
                out.push(p);
            }
        }
    }
    out.sort();
    out
}

/// The name a report should use for a catalogue.
fn name(p: &PathBuf) -> String {
    p.file_name().unwrap_or_default().to_string_lossy().into()
}

/// Walk every array-of-tables row in a document, whatever the table is called.
///
/// The catalogues use different keys: `[[item]]` and `[[entry]]` both appear, and
/// nothing says a later one has to pick either. Keying on one would make this
/// check silently cover nothing the day somebody picks a different word, which is
/// the failure mode this crate keeps finding elsewhere: an arm keyed on a string
/// that matched nothing, reporting clean over a corpus it never reached.
fn rows(doc: &DocumentMut) -> Vec<(String, &toml_edit::Table)> {
    let mut out = Vec::new();
    for (key, item) in doc.iter() {
        if let Item::ArrayOfTables(arr) = item {
            for t in arr.iter() {
                out.push((key.to_string(), t));
            }
        }
    }
    out
}

/// Read every catalogue, skipping the ones the parse arm already owns.
fn parsed() -> Vec<(PathBuf, DocumentMut)> {
    catalogues()
        .into_iter()
        .filter_map(|p| {
            let text = fs::read_to_string(&p).ok()?;
            let doc = text.parse::<DocumentMut>().ok()?;
            Some((p, doc))
        })
        .collect()
}

#[test]
fn every_committed_catalogue_parses() {
    let found = catalogues();
    assert!(
        !found.is_empty(),
        "no catalogue found at all. Either they moved or this walk is looking in \
         the wrong place, and an empty population passing silently is the thing \
         this test exists to not do."
    );
    let mut broken = Vec::new();
    for p in &found {
        let text = match fs::read_to_string(p) {
            Ok(t) => t,
            Err(e) => {
                broken.push(format!("{}: unreadable: {e}", name(p)));
                continue;
            },
        };
        if let Err(e) = text.parse::<DocumentMut>() {
            broken.push(format!("{}: {e}", name(p)));
        }
    }
    assert!(
        broken.is_empty(),
        "a catalogue does not parse, so nobody can read it and nothing said so: {broken:#?}"
    );
}

#[test]
fn every_catalogue_row_can_be_named() {
    let mut anonymous = Vec::new();
    for (p, doc) in parsed() {
        for (key, t) in rows(&doc) {
            let named = IDENTITY_KEYS
                .iter()
                .any(|k| t.get(k).and_then(|i| i.as_str()).is_some());
            if !named {
                anonymous.push(format!(
                    "{}: a [[{key}]] carrying none of {IDENTITY_KEYS:?}",
                    name(&p)
                ));
            }
        }
    }
    assert!(
        anonymous.is_empty(),
        "a catalogue row cannot be cited, which makes it prose in a table: {anonymous:#?}"
    );
}

#[test]
fn no_two_rows_claim_the_same_identifier() {
    let mut duplicated = Vec::new();
    for (p, doc) in parsed() {
        let mut seen: Vec<String> = Vec::new();
        for (_, t) in rows(&doc) {
            let Some(id) = t.get(UNIQUE_KEY).and_then(|i| i.as_str()) else {
                continue;
            };
            if seen.iter().any(|s| s == id) {
                duplicated.push(format!("{}: `{UNIQUE_KEY} = {id}` appears twice", name(&p)));
            }
            seen.push(id.to_string());
        }
    }
    assert!(
        duplicated.is_empty(),
        "two rows share an identifier, so a citation of it names both and resolves \
         to whichever a reader happens to find first: {duplicated:#?}"
    );
}

#[test]
fn a_confidence_is_a_number_between_zero_and_one() {
    let mut wrong = Vec::new();
    for (p, doc) in parsed() {
        for (_, t) in rows(&doc) {
            let Some(v) = t.get("confidence") else {
                continue; // optional: a catalogue need not score its rows
            };
            let who = IDENTITY_KEYS
                .iter()
                .find_map(|k| t.get(k).and_then(|i| i.as_str()))
                .unwrap_or("<unnamed>");
            match v.as_value() {
                Some(Value::Float(f)) => {
                    let n = *f.value();
                    if !(0.0..=1.0).contains(&n) {
                        wrong.push(format!("{}: `{who}` has confidence {n}", name(&p)));
                    }
                },
                Some(Value::Integer(i)) => {
                    let n = *i.value();
                    if !(0..=1).contains(&n) {
                        wrong.push(format!("{}: `{who}` has confidence {n}", name(&p)));
                    }
                },
                _ => wrong.push(format!(
                    "{}: `{who}` has a confidence that is not a number",
                    name(&p)
                )),
            }
        }
    }
    assert!(
        wrong.is_empty(),
        "a confidence outside zero to one is a scale error, and it reads as a \
         plausible score rather than as a mistake: {wrong:#?}"
    );
}

/// The controls, and they are the whole reason to trust the four above.
///
/// Each plants the defect the matching arm looks for and requires the reader to
/// find it. Without these, a walk that silently found no rows would report every
/// catalogue clean, which is the shape this crate has already caught twice: an
/// arm keyed on a string that matched nothing, reporting clean over a corpus it
/// never reached.
#[test]
fn the_readers_find_the_defects_they_are_looking_for() {
    // Control 1: a document that does not parse.
    assert!(
        "[[item]]\nid = \"unclosed\n".parse::<DocumentMut>().is_err(),
        "the parser accepted an unterminated string, so the parse arm cannot fail"
    );

    let doc: DocumentMut = r#"
[[item]]
claim = "no identity on this one"

[[item]]
id = "twice"

[[item]]
id = "twice"

[[item]]
file = "same.md"

[[item]]
file = "same.md"
"#
    .parse()
    .expect("the control document parses");
    let r = rows(&doc);
    assert_eq!(r.len(), 5, "the walk did not find all five rows: {r:#?}");

    // Control 2: the naming arm sees the row carrying no identity at all.
    let anonymous = r
        .iter()
        .filter(|(_, t)| {
            !IDENTITY_KEYS
                .iter()
                .any(|k| t.get(k).and_then(|i| i.as_str()).is_some())
        })
        .count();
    assert_eq!(
        anonymous, 1,
        "exactly one planted row carries no identity, and the naming arm must see it"
    );

    // Control 3: an `id` duplicate is caught and a `file` duplicate is not. That
    // asymmetry is the distinction the two arms rest on rather than an accident.
    let ids: Vec<_> = r
        .iter()
        .filter_map(|(_, t)| t.get("id").and_then(|i| i.as_str()))
        .collect();
    let unique = {
        let mut u = ids.clone();
        u.sort_unstable();
        u.dedup();
        u.len()
    };
    assert_eq!(
        ids.len() - unique,
        1,
        "the planted duplicate `id` pair is not being counted, so the uniqueness \
         arm would pass over a real one"
    );
    let files: Vec<_> = r
        .iter()
        .filter_map(|(_, t)| t.get("file").and_then(|i| i.as_str()))
        .collect();
    assert_eq!(
        files.len(),
        2,
        "two rows share a `file` and both must survive, because a catalogue may \
         split one file into several rows on purpose"
    );

    // Control 4: the walk is not keyed on the word `item`.
    let other: DocumentMut = "[[finding]]\nid = \"under-another-name\"\n"
        .parse()
        .expect("the control document parses");
    let r = rows(&other);
    assert_eq!(
        r.len(),
        1,
        "a catalogue using a different table name is invisible to the walk, which \
         would make every arm above pass over nothing: {r:#?}"
    );
    assert_eq!(r[0].0, "finding");

    // Control 5: the confidence band rejects and accepts what it should.
    for bad in [9.5_f64, -0.1, 1.5] {
        assert!(
            !(0.0..=1.0).contains(&bad),
            "the band accepts {bad}, so the confidence arm cannot fail"
        );
    }
    for good in [0.0_f64, 0.55, 0.95, 1.0] {
        assert!(
            (0.0..=1.0).contains(&good),
            "the band rejects {good}, which is a legitimate score"
        );
    }
}
