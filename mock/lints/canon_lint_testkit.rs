//! What a canon lint's own tests are built out of.
//!
//! **This file declares no lint.** The engine scans each `mock/lints/*.rs` for
//! `lint()`, `cross_lint()`, `repo_lint()` and `message_lint()`, includes the
//! file as a module either way, and registers only what it found. So a module
//! that defines none of them compiles into the cdylib alongside the lints and
//! reaches nothing, which is what lets every lint here share one set of helpers
//! without any of them depending on another's internals.
//!
//! Everything here is `#[cfg(test)]`, so a release build of the pack carries an
//! empty module.
//!
//! **What is here serves the registry lints**, which read a [`RegistryView`] and
//! nothing else. That view is public and constructible, so such a predicate can
//! be exercised on a planted registry with no fixture tree, no temporary
//! directory and no process.
//!
//! [`RepoContext`] also carries `mock_dir` and `repo_root`, and a lint reading
//! the worktree through those is driven by a planted tree in its own file
//! instead: [`ctx`] hardcodes a `mock_dir` of `.` and cannot drive one. The
//! registry helpers below still apply to it, and [`assert_registered`] and
//! [`assert_not_declared_off`] apply to every lint whatever it reads.
//!
//! Three questions a planted view cannot answer on its own, and the helpers
//! that answer each without a process: whether the engine loads the pack
//! ([`assert_registered`], which calls the generated collector), whether the
//! lint is switched off before its predicate runs ([`assert_not_declared_off`]),
//! and whether a finding actually refuses ([`assert_findings_block`]).
//!
//! **What that leaves uncovered is the shape the loader hands a real predicate.**
//! Kamu answers it with a fixture tree run through the gate. This pack does not
//! have one, so a lint whose predicate disagrees with a real `RegistryView` in
//! some way a hand-built view does not reproduce would pass here. Said plainly
//! rather than left for somebody to discover.

#![cfg(test)]

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

use mockspace::{
    Lint, LintError, LintPack, RegistryView, RepoContext, RepoLint, RowFields, Severity,
};

/// A registry holding exactly the rows named, and the reverse edges named.
///
/// `referrers` is passed rather than derived because deriving it needs the field
/// types, which are configuration. Handing over an empty map would make every
/// row read as referenced by nothing, which is a real state and not one to fall
/// into by omission, so it is spelled at each call site.
pub fn view(rows: &[(&str, &[(&str, &str)])], referrers: &[(&str, &[&str])]) -> RegistryView {
    let mut map: BTreeMap<String, RowFields> = BTreeMap::new();
    for (q, fields) in rows {
        map.insert(
            (*q).to_string(),
            fields
                .iter()
                .map(|(k, v)| ((*k).to_string(), (*v).to_string()))
                .collect(),
        );
    }
    let edges = referrers
        .iter()
        .map(|(q, rs)| {
            (
                (*q).to_string(),
                rs.iter().map(|r| (*r).to_string()).collect(),
            )
        })
        .collect();
    RegistryView::new(map, edges)
}

/// A context carrying that registry and nothing else.
///
/// Every other field is the empty case, and `mock_dir` and `repo_root` are
/// both `.`, which makes this the wrong instrument for a lint that reads the
/// worktree: it would walk whatever directory the test binary happens to be in.
/// Such a lint plants its own tree in its own file and calls its predicate
/// directly, and its real-run arm lives under
/// its own `mod tests`, where the fixture is a tree rather than a registry.
///
/// The empty case is legitimate for the fields it does cover: arvo's crate tree
/// is deliberately empty while the canon is written, so a registry lint here
/// reads the registry or reads nothing.
pub fn ctx<'a>(registry: &'a RegistryView) -> RepoContext<'a> {
    static HERE: OnceLock<PathBuf> = OnceLock::new();
    ctx_at(HERE.get_or_init(|| PathBuf::from(".")), registry)
}

/// The same context with the mock directory pointed at a planted tree.
///
/// **This is what a lint reading the worktree is driven by.** [`ctx`] hardcodes
/// `.`, which makes it the wrong instrument for one of those: it would walk
/// whichever directory the test binary happens to be in. A corpus lint builds
/// its own tree under [`planted_tree`] and hands the path here, so the same
/// predicate the gate runs is the one under test rather than a copy of it.
///
/// `repo_root` is the same path. Nothing here reads both, and pointing them at
/// two different places would be inventing a layout no real run has.
pub fn ctx_at<'a>(mock_dir: &'a Path, registry: &'a RegistryView) -> RepoContext<'a> {
    static CRATES: OnceLock<BTreeSet<String>> = OnceLock::new();
    static DIRS: OnceLock<Vec<PathBuf>> = OnceLock::new();
    static STRINGS: OnceLock<Vec<String>> = OnceLock::new();
    RepoContext {
        mock_dir,
        repo_root: mock_dir,
        all_crates: CRATES.get_or_init(BTreeSet::new),
        src_dirs: DIRS.get_or_init(Vec::new),
        invocation: None,
        canon_paths: STRINGS.get_or_init(Vec::new),
        open_panels: STRINGS.get_or_init(Vec::new),
        registry,
    }
}

/// An empty directory nothing else is using, for a lint that reads a tree.
///
/// Keyed on the caller's own name plus the process and thread, so two arms in
/// one binary cannot plant into each other's tree. Removed and recreated on
/// entry rather than on exit, because a test that fails leaves its tree behind
/// and that is the tree somebody then wants to look at.
pub fn planted_tree(what: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "arvo-canon-lint-{what}-{}-{:?}",
        std::process::id(),
        std::thread::current().id()
    ));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("a planted tree");
    dir
}

/// Write one file into a planted tree, creating whatever directories it needs.
pub fn plant(dir: &Path, at: &str, text: &str) {
    let path = dir.join(at);
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).expect("a planted directory");
    }
    std::fs::write(&path, text).expect("a planted file");
}

/// What one lint said about one planted registry, as plain strings.
pub fn findings(lint: &dyn RepoLint, registry: &RegistryView) -> Vec<String> {
    lint.check_repo(&ctx(registry))
        .iter()
        .map(LintError::to_string)
        .collect()
}

/// The names of every repo lint in the pack the engine is handed.
///
/// The pack is filled by `__mockspace_collect_lints`, which the engine generates
/// into this crate from a scan of `mock/lints/*.rs` and calls across the cdylib
/// boundary. Calling it here reads the same registration list from the same
/// generated source, which is the half of "the lint is wired in" that needs no
/// process: a predicate can be perfect and the file can still define its
/// constructor under a name the scan does not recognise, and then nothing ever
/// runs it.
/// Each registered lint paired with the text of the file declaring it.
///
/// Keyed off the registration list rather than off a directory listing, so a
/// file present but unregistered is not silently read as covered, and a lint
/// registered under a name matching no file fails loudly here rather than being
/// skipped.
pub fn lint_sources() -> Vec<(String, String)> {
    let dir = repo_root().join("mock/lints");
    registered_repo_lints()
        .into_iter()
        .map(|name| {
            let file = dir.join(format!("{}.rs", name.replace('-', "_")));
            let text = std::fs::read_to_string(&file).unwrap_or_else(|e| {
                panic!(
                    "`{name}` is registered and {} does not read: {e}. A lint's file is \
                     named after the lint it declares.",
                    file.display()
                )
            });
            (name, text)
        })
        .collect()
}

pub fn registered_repo_lints() -> Vec<String> {
    let mut pack = LintPack::default();
    crate::__mockspace_collect_lints(&mut pack);
    pack.repo_lints
        .iter()
        .map(|l| l.name().to_string())
        .collect()
}

/// Assert this lint is in the pack, under the name it answers to.
///
/// The name matters as much as the presence: `[lints.<name>]` in `mockspace.toml`
/// is keyed on it, and every check that greps a gate's output for a lint's own
/// words is keyed on it too.
pub fn assert_registered(name: &str) {
    let names = registered_repo_lints();
    assert!(
        names.iter().any(|n| n == name),
        "`{name}` is not in the pack the engine is handed, so nothing runs it \
         however well it works. The pack carries: {names:?}"
    );
}

/// Every finding this lint produced blocks at every gate.
///
/// **The severity that decides a refusal is the one the finding carries**, put
/// there by whichever `LintError` constructor the call site used.
/// [`Lint::default_severity`] is not that value, and asserting on it instead
/// pins something the engine reads for one purpose only: whether the lint runs
/// at all. The engine says so in `run_with_overrides`, that each finding keeps
/// the severity its constructor chose "so a declared default is honoured as on
/// or off rather than per gate", and only an explicit `[lints.<name>]` override
/// restamps it.
///
/// Measured rather than read off that comment. Setting `default_severity()` to
/// `ADVISORY` changed nothing: the gate still printed `[error]` and still
/// exited non-zero. Swapping one `LintError::error` for `LintError::warning`
/// turned the refusal off with the declared default untouched, and the arm
/// asserting the declared default stayed green through it.
pub fn assert_findings_block(lint: &dyn RepoLint, registry: &RegistryView) {
    assert_findings_block_at(lint, &ctx(registry));
}

/// [`assert_findings_block`] against a context the caller built.
///
/// A lint reading the worktree needs its planted tree in the context, which
/// [`ctx`] cannot carry, so it calls this with one from [`ctx_at`]. Same
/// assertion, same reason, and the two share a body so a change to what
/// "blocks" means cannot reach one and miss the other.
pub fn assert_findings_block_at(lint: &dyn RepoLint, ctx: &RepoContext) {
    let found = lint.check_repo(ctx);
    assert!(
        !found.is_empty(),
        "nothing was found, so this says nothing about what a finding carries"
    );
    for e in &found {
        assert_eq!(
            e.severity,
            Severity::HARD_ERROR,
            "`{}` reported `{}` at a severity that does not block every gate",
            e.lint_name,
            e.message
        );
    }
}

/// The lint is not declared off.
///
/// The other half of [`assert_findings_block`], and the one the declared
/// default really governs: a lint whose `default_severity()` is `OFF`, and
/// which no `[lints]` section names, is skipped before its predicate runs.
/// Fourteen lints upstream shipped in exactly that state, declaring `OFF` and
/// stamping `HARD_ERROR` on findings nothing ever produced.
///
/// Asserted as "not off" rather than as one exact value, because that is the
/// whole of what the engine consults here and pinning more would be this test
/// deciding something the engine does not read. **Blocking is not the stronger
/// version of this**, however much it reads like one: the severity that decides
/// a refusal is the one the finding carries, which is what
/// [`assert_findings_block`] measures, and a declared default of `ADVISORY`
/// changes nothing at any gate.
pub fn assert_not_declared_off(lint: &dyn Lint) {
    assert!(
        !lint.default_severity().is_off(),
        "`{}` declares itself off, so it never runs and its predicate is dead \
         code however good it is",
        lint.name()
    );
}

/// The repository this pack was built from.
///
/// **Walked at run time rather than taken from `env!`.** The macro bakes the
/// directory of whichever tree compiled the binary, the pack builds under
/// `mock/target/mockspace-lints`, and one target directory is shared across
/// every worktree cut from a clone, so a baked path answers about whichever
/// tree built first, in every other tree, silently.
pub fn repo_root() -> PathBuf {
    let mut dir = std::env::current_dir().expect("a working directory");
    loop {
        if dir.join("mockspace.toml").is_file() {
            return dir;
        }
        assert!(dir.pop(), "no mockspace.toml above the working directory");
    }
}

/// Every `mock/lints/*.rs` declaring a lint, as the name it should answer to.
///
/// The file name is the source, kebab-cased. A lint whose declared `name()`
/// disagrees with its file is caught by the same set difference that catches
/// one which never registers, and both are the same defect to a reader looking
/// for it by either.
///
/// **The needle is built rather than written**, because the engine's own scan
/// over these files is textual and matched this line the first time it was
/// spelled out, so a file that declares no lint declared one and the cdylib
/// stopped compiling. It is the scan reporting exactly what it looks for, and
/// the reason this test can be written at all.
fn declared_lint_files() -> BTreeSet<String> {
    let needle = format!("pub fn {}()", "repo_lint");
    let dir = repo_root().join("mock/lints");
    std::fs::read_dir(&dir)
        .unwrap_or_else(|e| panic!("read {}: {e}", dir.display()))
        .flatten()
        .map(|e| e.path())
        .filter(|p| p.extension().is_some_and(|x| x == "rs"))
        .filter(|p| std::fs::read_to_string(p).is_ok_and(|t| t.contains(&needle)))
        .filter_map(|p| p.file_stem().and_then(|s| s.to_str()).map(str::to_owned))
        .map(|s| s.replace('_', "-"))
        .collect()
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
    let dir = repo_root().join("mock/lints");
    let untested: Vec<String> = registered_repo_lints()
        .into_iter()
        .filter(|name| {
            let file = dir.join(format!("{}.rs", name.replace('-', "_")));
            // The brace is load-bearing. Without it `mod tests_whatever` reads
            // as a test module, which is how the first version of this passed
            // its own mutation: a module renamed out of the way still matched.
            !std::fs::read_to_string(&file).is_ok_and(|t| t.contains("mod tests {"))
        })
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
    // declared `OFF` and overridden nowhere, or reporting warnings that refuse
    // nothing. Each of those leaves every other arm in its file passing.
    //
    // Kamu checks this by reading a separate fixture-tree suite. There is no
    // such directory here and there is not going to be one, so the property is
    // checked against the lint files themselves: the three helpers are the
    // three questions, and a file naming none of them has asked none of them.
    const OWED: [&str; 3] = [
        "assert_registered",
        "assert_not_declared_off",
        "assert_findings_block",
    ];

    let mut unasked: Vec<String> = Vec::new();
    for (name, text) in lint_sources() {
        let missing: Vec<&str> = OWED
            .iter()
            .copied()
            .filter(|helper| !text.contains(helper))
            .collect();
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
