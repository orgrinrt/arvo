//! Whether every lint in the pack asks the questions the helpers beside this
//! file exist to answer, and whether the scans those questions rest on read
//! anything at all.

use std::collections::BTreeSet;

use mockspace::{LintMode, LintPack};

use super::{
    SEVERITY_ASKED,
    declared_lint_files,
    lint_sources,
    namespaces_declaring,
    registered_repo_lints,
    repo_root,
    severity_after_config,
};

#[test]
fn a_lint_naming_the_namespaces_it_reads_agrees_with_the_schema() {
    // The guard `refusal_owes_an_instead.rs` says exists and, until this was
    // written, did not. That file hand-writes the namespaces it reads, because
    // nothing a lint is handed carries a field declaration, and its own
    // paragraph warns that a written list goes stale in silence. The sentence
    // claiming a guard already caught that was the shape
    // `a-claim-of-totality-names-what-enforces-it.md` names: a claim of
    // coverage naming an enforcer that was not there.
    //
    // What is compared is the `instead` field, since that is the half of the
    // schema the lint's own documentation says decides which namespaces it
    // reads. The `kind`-with-`refusal` half is not compared: a value set is
    // written as an array across several lines and reading it would need the
    // parser this cannot have. Said here rather than left for somebody to
    // discover.
    let declared = namespaces_declaring("instead")
        .expect("mockspace.toml declares at least one registry namespace");
    assert!(
        declared.len() >= 2,
        "the scan found {} namespace(s) declaring `instead`, which means the file's shape \
         moved and this is measuring nothing: {declared:?}",
        declared.len()
    );

    let file = repo_root().join("mock/lints/refusal_owes_an_instead.rs");
    let text =
        std::fs::read_to_string(&file).unwrap_or_else(|e| panic!("read {}: {e}", file.display()));
    let written: BTreeSet<String> = declared
        .iter()
        .filter(|ns| text.contains(&format!("\"{ns}\"")))
        .cloned()
        .collect();
    assert_eq!(
        written, declared,
        "`refusal_owes_an_instead.rs` reads {written:?} and the schema declares `instead` \
         on {declared:?}. A namespace the schema gained is one the lint does not check, and \
         nothing else would say so."
    );
}

#[test]
fn the_schema_scan_can_report_a_disagreement_rather_than_only_agreement() {
    // The control on the guard above. It compares two sets built from two
    // files, and a scan returning nothing would make them agree perfectly. This
    // asks the scan for a field no namespace declares, which has to come back
    // empty while the scan itself still reports having read the file.
    assert_eq!(
        namespaces_declaring("no_namespace_declares_this_field"),
        Some(BTreeSet::new()),
        "the scan read the file and found no namespace declaring a field nothing declares"
    );
    let declared = namespaces_declaring("instead").expect("the file reads");
    assert!(
        !declared.contains("mechanism"),
        "the scan attributes a field to the namespace that declares it rather than to \
         every namespace in the file: {declared:?}"
    );
}

#[test]
fn every_declared_lint_is_registered_under_the_name_its_file_carries() {
    // `assert_registered` is called *by* a lint's own test module, so it says
    // the caller is registered and can say nothing about a file the scan missed
    // or a lint whose name drifted from its file.
    let declared = declared_lint_files();
    assert!(
        declared.len() >= 4,
        "the scan found {} lint files, which means it is not reading the tree",
        declared.len()
    );
    let registered: BTreeSet<String> = registered_repo_lints().into_iter().collect();
    let missing: Vec<&String> = declared.difference(&registered).collect();
    assert!(
        missing.is_empty(),
        "declared in `mock/lints/` and not in the pack the engine is handed: \
         {missing:?}. The pack carries: {registered:?}"
    );
}

#[test]
fn every_registered_lint_has_tests_in_the_file_that_declares_it() {
    // A lint with no test module at all passes every other check here by not
    // being asked, since every other check is called by a lint's own tests.
    // That is a real state and nothing else catches it.
    //
    // **It is not the check that would have caught the line-limit lint**, which
    // is what the first version of this comment claimed. At `c8d40758` that
    // lint had a test module and no gate arm, so this check would have passed
    // on the commit its story named. Measured, after a reviewer said so. The
    // one that catches that is
    // `every_registered_lint_asks_whether_it_reaches_the_gate_at_all` below,
    // and the two together are the whole of it.
    // Read through `lint_sources`, which knows a repo lint may be declared by a
    // tool crate as well as by a file under `mock/lints/`. Joining the path here
    // instead is how the tool-declared one would pass this by not being found.
    let untested: Vec<String> = lint_sources()
        .into_iter()
        // The brace is load-bearing. Without it `mod tests_whatever` reads as a
        // test module, which is how the first version of this passed its own
        // mutation: a module renamed out of the way still matched. A tool crate
        // keeps its tests in a `mod tests;` beside the lint, so the declaration
        // rather than the body is what is looked for there.
        .filter(|(_, text)| !text.contains("mod tests {") && !text.contains("mod tests;"))
        .map(|(name, _)| name)
        .collect();
    assert!(
        untested.is_empty(),
        "registered with no test module in its own file: {untested:?}. Every \
         other check here is called by a lint's own tests, so a lint with none \
         passes all of them by not being asked."
    );
}

#[test]
fn every_registered_lint_asks_whether_it_reaches_the_gate_at_all() {
    // A test module is not the same as a test module that asks the three
    // questions above. A predicate can be exhaustively covered on planted rows
    // and still never run: registered under a name the config does not key on,
    // declared `OFF` and overridden nowhere, or reporting at a severity nobody
    // chose. Each of those leaves every other arm in its file passing.
    //
    // Kamu checks this by reading a separate fixture-tree suite. There is no
    // such directory here and there is not going to be one, so the property is
    // checked against the lint files themselves: the helpers are the three
    // questions, and a file naming none of them has asked none of them. The
    // third question is what severity a finding carries, and it is answered by
    // either of the helpers in `SEVERITY_ASKED`: one asserting the findings
    // block, or one asserting a severity the lint names, which is how a lint
    // that warns on purpose says so.
    const OWED: [&str; 2] = ["assert_registered", "assert_not_declared_off"];

    // **A lint declared by a tool crate cannot call any of them**, because they
    // live here and it is a different crate. Two of the three are answered by
    // this test instead, directly and for every registered lint whatever
    // declares it: it is in the pack the engine is handed, which is the whole of
    // what `assert_registered` establishes, and it does not declare itself off,
    // which is `assert_not_declared_off`. What is left is whether a finding
    // blocks, which needs an input only that crate can build, so its own tests
    // are required to name the severity they assert.
    let mut pack = LintPack::default();
    crate::__mockspace_collect_lints(&mut pack);
    let mut unasked: Vec<String> = Vec::new();
    for (name, text) in lint_sources() {
        let lint = pack
            .repo_lints
            .iter()
            .find(|l| l.name() == name)
            .expect("the name came out of the pack");
        assert!(
            !lint.default_severity().is_off(),
            "`{name}` declares itself off, so it never runs and its predicate is dead code \
             however good it is"
        );
        if text.contains("mockspace::lint_pack!") {
            let beside = repo_root()
                .join("mock/tools")
                .join(&name)
                .join("src/tests.rs");
            let tests = std::fs::read_to_string(&beside).unwrap_or_default();
            if !tests.contains("Severity::HARD_ERROR") {
                unasked.push(format!(
                    "{name}: its own tests never assert what a finding's severity is, and \
                     the helpers here are in another crate"
                ));
            }
            continue;
        }
        let mut missing: Vec<&str> = OWED
            .iter()
            .copied()
            .filter(|helper| !text.contains(helper))
            .collect();
        if !SEVERITY_ASKED.iter().any(|helper| text.contains(helper)) {
            missing.push("assert_findings_block or assert_findings_carry");
        }
        if !missing.is_empty() {
            unasked.push(format!("{name}: {missing:?}"));
        }
    }
    unasked.sort();
    assert!(
        unasked.is_empty(),
        "a registered lint's own file never asks whether it reaches the gate: \
         {unasked:?}. Its predicate may be perfect and nothing establishes the \
         engine runs it, which is the state a whole round went by without \
         noticing."
    );
}

#[test]
fn a_lint_that_splits_its_tests_out_is_read_whole() {
    // The arm above asks each registered lint's own source whether it
    // establishes that the engine runs it. A lint past the file-size limit puts
    // its tests in `mock/lints/<stem>/tests.rs`, and reading the entry file
    // alone reported that lint as never having asked, which is a fact about
    // where the `#[path]` points. So the composition is asserted here: the text
    // holds what the sibling file holds, and the entry file does not hold it,
    // which is what makes this a statement about the composition rather than
    // about the lint.
    let stem = "half_up_carries_its_note";
    let entry = std::fs::read_to_string(repo_root().join("mock/lints").join(format!("{stem}.rs")))
        .expect("the entry file of a lint this repository ships");
    let whole = lint_sources()
        .into_iter()
        .find(|(name, _)| name == "half-up-carries-its-note")
        .map(|(_, text)| text)
        .expect("the lint is registered");
    assert!(
        !entry.contains("assert_registered"),
        "the entry file has taken the helper back, so this says nothing about the composition"
    );
    assert!(
        whole.contains("assert_registered"),
        "the sibling file's tests are not in what the source of a lint reads as"
    );
}

/// The ledger lint's severity as this repository's `mockspace.toml` leaves it,
/// layered over the lint's own default, blocks at every gate.
///
/// The default alone is not enough. `[lints.catalogue-ignore]` keyed `commit`,
/// `build` or `push` to `"off"` or `"warn"` turns the gate off while the lint's
/// own `HARD_ERROR` still reads as blocking, and every other arm keeps passing.
#[test]
fn catalogue_ignore_runs_at_a_blocking_severity_at_every_gate() {
    let mut pack = LintPack::default();
    crate::__mockspace_collect_lints(&mut pack);
    let lint = pack
        .repo_lints
        .iter()
        .find(|l| l.name() == "catalogue-ignore")
        .expect("`catalogue-ignore` is in the pack the engine is handed");
    let config = std::fs::read_to_string(repo_root().join("mockspace.toml"))
        .expect("the repository's mockspace.toml");
    let severity = severity_after_config(&config, "catalogue-ignore", lint.default_severity());
    for mode in [LintMode::Commit, LintMode::Build, LintMode::Push] {
        assert!(
            severity.is_blocking(mode),
            "`catalogue-ignore` does not block at {mode:?} once `mockspace.toml` is applied, so \
             an ignore the ledger does not resolve would pass that gate"
        );
    }
}

/// The cases the arm above must be able to see: each gate keyed down in turn is
/// caught, and a table for another lint, a comment, or no table at all is not.
#[test]
fn control_the_config_overlay_sees_each_gate_turned_down_and_nothing_else() {
    use mockspace::Severity;
    let blocks = |cfg: &str| {
        let s = severity_after_config(cfg, "catalogue-ignore", Severity::HARD_ERROR);
        [LintMode::Commit, LintMode::Build, LintMode::Push].map(|m| s.is_blocking(m))
    };
    for (key, at) in [("commit", 0), ("build", 1), ("push", 2)] {
        for level in ["off", "warn"] {
            let cfg = format!("[lints.catalogue-ignore]\n{key} = \"{level}\"\n");
            let got = blocks(&cfg);
            let mut want = [true; 3];
            want[at] = false;
            assert_eq!(
                got, want,
                "`{key} = \"{level}\"` turns exactly that gate down"
            );
        }
    }
    assert_eq!(
        blocks("[lints.catalogue-ignore]\ncommit = \"off\" # temporarily\n"),
        [false, true, true],
        "a trailing comment is not part of the value"
    );
    assert_eq!(
        blocks("[lints.catalogue-ignore]\nledgers = \"x\"\n"),
        [true; 3]
    );
    assert_eq!(blocks("[lints.other]\npush = \"off\"\n"), [true; 3]);
    assert_eq!(
        blocks("[lints.catalogue-ignore]\npush = \"off\"\n[lints.other]\n"),
        [true, true, false]
    );
    assert_eq!(blocks(""), [true; 3]);
}

/// Every repo lint in the pack, the shared pack's included, is on: none declares
/// itself off, which is the half of the severity question `lint_sources` cannot
/// ask of a lint with no source here.
#[test]
fn no_repo_lint_in_the_whole_pack_declares_itself_off() {
    let mut pack = LintPack::default();
    crate::__mockspace_collect_lints(&mut pack);
    let off: Vec<&str> = pack
        .repo_lints
        .iter()
        .filter(|l| l.default_severity().is_off())
        .map(|l| l.name())
        .collect();
    assert!(off.is_empty(), "declared off, so never run: {off:?}");
}
