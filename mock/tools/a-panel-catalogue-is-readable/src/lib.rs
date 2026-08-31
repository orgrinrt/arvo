//! The panel's catalogues are TOML that nothing reads, so nothing catches a
//! broken one.
//!
//! Several sit under the panel directory and one under `mock/registry_catalogue/`.
//! None is inside `canon_paths`, so the engine never opens them, and `cargo mock`
//! walks past them. A stray quote in any one would sit there indefinitely, and
//! the file it breaks is the one a later seat reads instead of re-sweeping the
//! archive by hand. That is the whole point of a catalogue and it is exactly what
//! a silent parse failure destroys.
//!
//! **The bar here is about readability, not content.** The catalogues disagree
//! with each other on purpose: one sorts op's statements by what became of them,
//! one sorts panel files by whether their content was absorbed, one sorts
//! measured claims by whether they can be carried. Pinning a shared vocabulary
//! would force one shape onto all of them and would be inventing a schema nobody
//! asked for. So what is refused is what every catalogue must satisfy whatever it
//! is about: it parses, its rows can be named, no row claims an identifier
//! another row already claims, and a number meaning a confidence is a number in
//! the range a confidence can take.
//!
//! **Two of those were wrong when first written, and each was corrected by a
//! catalogue rather than by argument.** The first demanded `id`, which the
//! file-keyed catalogues do not carry and should not. The second then demanded
//! uniqueness of whatever key was found, which broke on a catalogue that
//! deliberately splits one panel file into three rows with three different
//! verdicts. Both corrections narrowed the assertion onto the property and away
//! from the spelling, which is the only direction a check may be loosened in.
//!
//! # A lint, and why it is declared from a tool crate
//!
//! The contract's four questions put this at question two: there are states it
//! refuses, and each is one edit to one file. It takes no argument and it has a
//! pass line at zero, so it is neither shape of tool.
//!
//! **It cannot live under `mock/lints/` anyway.** Its first refusal is a
//! catalogue that does not parse, and nothing short of a parser can establish
//! that. The generated pack's manifest is engine-written, so a lint file reaches
//! `std` and the lint-rules crate and can add nothing to it. A tool crate has
//! its own manifest and `lint_pack!` registers a `RepoLint` from one, so the
//! check gates exactly like the others while reaching the reader it needs.
//!
//! That is a gap in the lint contract rather than a third kind of check, and it
//! is said here rather than worked around silently. The honest fix upstream is a
//! way for a repository to declare a dependency for its own lint pack.

use std::path::{Path, PathBuf};

use mockspace::{Lint, LintError, RepoContext, RepoLint, Severity};
use toml_edit::{DocumentMut, Item, Value};

const NAME: &str = "a-panel-catalogue-is-readable";

/// The keys a catalogue names its rows by.
///
/// Three spellings of one property. `id` where the row is a statement and the
/// slug is its name; `path` and `file` where the row **is** a file and the path
/// is its name. All three are identities and none is wrong.
const IDENTITY_KEYS: [&str; 3] = ["id", "path", "file"];

/// The identity key that also promises uniqueness, and it is the only one.
///
/// `file` and `path` name a subject rather than a row. A catalogue may
/// legitimately carry one file three times, once per section group, when the
/// groups have different verdicts, and one does: splitting it is better work
/// than flattening three dispositions into one row would have been. `id` is a
/// slug somebody minted to be cited, so two rows answering to it is a real
/// defect.
const UNIQUE_KEY: &str = "id";

/// Where the catalogues sit, relative to the mock directory.
///
/// Directories rather than filenames, because listing the files goes stale the
/// first time somebody adds one, and the whole reason these exist is that the
/// next seat adds another.
const ROOTS: [&str; 3] = [
    "research/202608072330_the-numeral-canon-panel",
    "research/202608072330_the-numeral-canon-panel/catalogue",
    "registry_catalogue",
];

pub struct APanelCatalogueIsReadable;

impl Lint for APanelCatalogueIsReadable {
    fn name(&self) -> &'static str {
        NAME
    }

    fn default_severity(&self) -> Severity {
        Severity::HARD_ERROR
    }
}

impl RepoLint for APanelCatalogueIsReadable {
    fn check_repo(&self, ctx: &RepoContext) -> Vec<LintError> {
        check(ctx.mock_dir)
    }
}

/// Every catalogue under one mock directory, in name order.
pub fn catalogues(mock_dir: &Path) -> Vec<PathBuf> {
    let mut out = Vec::new();
    for root in ROOTS {
        let Ok(entries) = std::fs::read_dir(mock_dir.join(root)) else {
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

/// Every array-of-tables row in a document, whatever the table is called.
///
/// The catalogues use different keys: `[[item]]` and `[[entry]]` both appear,
/// and nothing says a later one has to pick either. Keying on one would make
/// this cover nothing the day somebody picks a different word, which is the
/// failure this corpus keeps producing: an arm keyed on a string that matched
/// nothing, reporting clean over a corpus it never reached.
pub fn rows(doc: &DocumentMut) -> Vec<(String, &toml_edit::Table)> {
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

/// One finding, at the severity that blocks every gate.
fn finding(kind: &'static str, at: &Path, message: String) -> LintError {
    let mut e = LintError::error("catalogue".to_string(), 0, NAME, message);
    e.path = Some(shown(at));
    e.finding_kind = Some(kind);
    e
}

fn shown(p: &Path) -> String {
    p.file_name().unwrap_or_default().to_string_lossy().into()
}

/// The four refusals, over one mock directory.
pub fn check(mock_dir: &Path) -> Vec<LintError> {
    let found = catalogues(mock_dir);
    if found.is_empty() {
        // Not a clean pass. Three roots and none of them holds a catalogue means
        // either they moved or this walk is looking in the wrong place, and an
        // empty population passing silently is the thing this exists to not do.
        return vec![finding(
            "no-catalogue-found",
            Path::new("mock"),
            format!(
                "no catalogue was found under any of {ROOTS:?}. Either they moved or this \
                 walk is reading the wrong tree, and the three checks below are about to \
                 report clean over nothing."
            ),
        )];
    }

    let mut out = Vec::new();
    let mut parsed: Vec<(PathBuf, DocumentMut)> = Vec::new();
    for p in found {
        let text = match std::fs::read_to_string(&p) {
            Ok(t) => t,
            Err(e) => {
                out.push(finding(
                    "a-catalogue-does-not-read",
                    &p,
                    format!("cannot be read: {e}. Nobody can read it and nothing else says so."),
                ));
                continue;
            }
        };
        match text.parse::<DocumentMut>() {
            Ok(doc) => parsed.push((p, doc)),
            Err(e) => out.push(finding(
                "a-catalogue-does-not-parse",
                &p,
                format!(
                    "does not parse: {e}. Nothing else opens this file, so a stray quote \
                     sits here indefinitely and the seat that reads it instead of \
                     re-sweeping the archive gets nothing."
                ),
            )),
        }
    }

    for (p, doc) in &parsed {
        let mut seen: Vec<String> = Vec::new();
        for (key, t) in rows(doc) {
            let named = IDENTITY_KEYS
                .iter()
                .find_map(|k| t.get(k).and_then(|i| i.as_str()));
            let Some(who) = named else {
                out.push(finding(
                    "a-catalogue-row-cannot-be-named",
                    p,
                    format!(
                        "a `[[{key}]]` row carries none of {IDENTITY_KEYS:?}, so it cannot be \
                         cited, which makes it prose in a table."
                    ),
                ));
                continue;
            };
            if let Some(id) = t.get(UNIQUE_KEY).and_then(|i| i.as_str()) {
                if seen.iter().any(|s| s == id) {
                    out.push(finding(
                        "two-rows-claim-one-identifier",
                        p,
                        format!(
                            "`{UNIQUE_KEY} = \"{id}\"` appears twice, so a citation of it \
                             names both and resolves to whichever a reader happens to find \
                             first."
                        ),
                    ));
                } else {
                    seen.push(id.to_string());
                }
            }
            if let Some(wrong) = confidence_out_of_range(t) {
                out.push(finding(
                    "a-confidence-is-out-of-range",
                    p,
                    format!(
                        "`{who}` has {wrong}. A confidence outside zero to one is a scale \
                         error, and it reads as a plausible score rather than as a mistake."
                    ),
                ));
            }
        }
    }
    out
}

/// What is wrong with a row's confidence, where anything is.
///
/// `None` covers both the absent field, since a catalogue need not score its
/// rows, and a value inside the range.
fn confidence_out_of_range(t: &toml_edit::Table) -> Option<String> {
    let v = t.get("confidence")?;
    match v.as_value() {
        Some(Value::Float(f)) => {
            let n = *f.value();
            (!(0.0..=1.0).contains(&n)).then(|| format!("confidence {n}"))
        }
        Some(Value::Integer(i)) => {
            let n = *i.value();
            (!(0..=1).contains(&n)).then(|| format!("confidence {n}"))
        }
        _ => Some("a confidence that is not a number".to_string()),
    }
}

#[cfg(test)]
mod tests;

mockspace::lint_pack! {
    repo_lints: [APanelCatalogueIsReadable],
}
