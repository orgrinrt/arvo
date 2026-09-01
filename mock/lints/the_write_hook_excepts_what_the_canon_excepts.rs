//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! The write hook and the canon have to agree about which positions take a bare
//! primitive, and nothing was checking that they did.
//!
//! The hook is a `PreToolUse` guard on Write and Edit, so it fires before any
//! lint in this pack gets consulted. A position it denies is one an agent cannot
//! write, whatever the lints and the rules go on to permit, and the disagreement
//! presents as nothing at all: the deny reads as the discipline working.
//!
//! It has happened. The canon excepts the type of a const generic parameter, in
//! op's own words at
//! `obligation::a_primitive_for_every_position_a_bare_number_would_take`. The
//! hook denied it anyway, on every one of the const generic parameters this
//! repository's own source carries, none of which holds an escape comment. The
//! lint pack had already been corrected to permit the position and a workspace
//! rule had been written to instruct agents to use it, and the hook contradicted
//! both from a file neither of those changes touched.
//!
//! **What this asserts is behaviour rather than text.** The table below is fed
//! to the hook's own scanner and the verdict compared, so a rewrite that reaches
//! the same answers passes and one that reads correctly while denying does not.
//! The excepted rows are half of it; the denied rows are the controls, and
//! without them a hook that permits everything would pass.
//!
//! **A lint rather than a tool.** There is no state of this repository in which
//! the gate an agent writes through may disagree with the canon, so this refuses
//! rather than reports.
use std::path::Path;
use std::process::{Command, Stdio};

use mockspace::{Lint, LintError, RepoContext, RepoLint, Severity};

use crate::canon_rows::finding;

pub fn repo_lint() -> Box<dyn RepoLint> {
    Box::new(TheWriteHookExcepts)
}

/// The lint's own name, used in its findings and keyed by `[lints.<name>]`.
const NAME: &str = "the-write-hook-excepts-what-the-canon-excepts";

/// The hook's template, which is the tracked half.
///
/// **The generated copy under `.claude/hooks/` is what actually runs, and it is
/// the wrong thing for this to read.** Generation happens after the lint pass in
/// the same invocation, so a lint reading the generated file is one run behind
/// its own template, and a lint that refuses on what it reads there can never be
/// satisfied: the run that would rewrite the file stops at the refusal. That was
/// this lint's first shape and it deadlocked the repository on the first try.
///
/// So the template is read and run with a stub standing in for the helpers the
/// engine splices in. What that costs is the scope check and the payload
/// extraction, which belong to mockspace rather than to this repository and are
/// not what the canon has anything to say about. What it buys is a check over
/// the file a change is actually made to, with no generated state involved.
const HOOK: &str = "mock/agent/hooks/no-bare-primitive-guard.sh.tmpl";

/// The marker the engine replaces with its own helper definitions.
const HELPERS: &str = "{{HOOK_HELPERS}}";

/// The helpers, as much of them as running the scanner needs.
///
/// `_extract` hands back the probe's own path and line, `_scope_or_allow` passes
/// because the probe names a path inside the guarded tree by construction, and
/// the two verdicts print a word this can read. Everything after that in the
/// template is the scanner, which is the part this lint is about.
const STUB: &str = "\
_extract() { case \"$1\" in file_path) printf '%s' \"$PROBE_PATH\" ;; content) printf '%s' \
                \"$PROBE_LINE\" ;; *) printf '' ;; esac; }\n\
_scope_or_allow() { return 0; }\n\
allow() { echo __ALLOW__; exit 0; }\n\
deny() { echo __DENY__; exit 0; }\n";

/// Where the runnable copy of the template is staged, under generated output.
///
/// Inside `mock/target`, which is gitignored, so a run leaves nothing in the
/// tree. Rewritten on every check rather than cached, because the template is
/// the thing under test and a cached copy would answer for an older one.
const SCRATCH: &str = "mock/target/the-write-hook-probe.sh";

/// A file path inside the guarded tree, which the hook needs before it scans.
///
/// Its scope check compares against the repository root and returns early for
/// anything outside it, so a probe naming a path elsewhere is allowed for a
/// reason that has nothing to do with what it holds. That allow reads exactly
/// like the exception working, which is the trap this constant exists to avoid.
const GUARDED: &str = "mock/crates/arvo-format/src/lib.rs";

/// Every line the hook is asked about, with the verdict the canon requires.
///
/// The `true` rows are the exception: the type of a const generic parameter, in
/// each shape the position is written in, including the one broken over several
/// lines where the annotation sits alone. The `false` rows are the controls, and
/// they carry the two discriminations that are easy to lose: an item constant is
/// a different position and is not excepted, and a line holding both an excepted
/// parameter and an ordinary bare type is still refused for the second.
const TABLE: &[(bool, &str)] = &[
    (true, "pub struct Signed<const BITS: u32>;"),
    (true, "impl<const B: u32, const F: i32> T for S<B, F> {}"),
    (true, "fn f<const N: usize>(x: Bool) -> Bool { x }"),
    (true, "    const BITS: u32,"),
    (true, "pub struct Buf<const N: usize = 8>;"),
    (false, "pub struct Handle(u32);"),
    (false, "const WIDTH: u32 = 32;"),
    (false, "pub const MAX: u64 = 9;"),
    // The two the table did not discriminate, both found by a reviewer running
    // the real scanner rather than reading it. An item constant whose
    // initialiser sits on the next line looks exactly like a parameter alone in
    // a broken generic list, and was excepted; and a trait's associated const
    // is the shape this whole round is about, was already refused correctly,
    // and nothing held it there.
    (false, "    const MAX: u64"),
    (false, "    const RADIX: u32;"),
    (false, "fn f<const N: usize>(x: u32) -> Bool { }"),
    (false, "let n = x as u32;"),
    (false, "pub struct Sized(u128);"),
];

struct TheWriteHookExcepts;
impl Lint for TheWriteHookExcepts {
    fn name(&self) -> &'static str {
        NAME
    }

    fn default_severity(&self) -> Severity {
        Severity::HARD_ERROR
    }
}
impl RepoLint for TheWriteHookExcepts {
    fn check_repo(&self, ctx: &RepoContext) -> Vec<LintError> {
        let root = ctx.mock_dir.parent().unwrap_or(&ctx.mock_dir).to_path_buf();
        check(&root)
    }
}

/// The verdict, over a repository root.
///
/// Split from the trait impl so a test can point it at a tree it built, and so
/// the one place that decides is one function.
fn check(root: &Path) -> Vec<LintError> {
    let Ok(template) = std::fs::read_to_string(root.join(HOOK)) else {
        return vec![finding(
            NAME,
            Some("the-hook-is-not-there"),
            format!(
                "`{HOOK}` cannot be read, so what an agent is allowed to write cannot be \
                 established. This reports rather than passing, because a missing gate and \
                 a correct one are the same silence."
            ),
        )];
    };
    if !template.contains(HELPERS) {
        return vec![finding(
            NAME,
            Some("the-hook-carries-no-helper-marker"),
            format!(
                "`{HOOK}` does not carry `{HELPERS}`, so this cannot stand anything in for \
                 the engine's helpers and cannot run the scanner. Either the marker moved \
                 or the file is not the hook this reads."
            ),
        )];
    }

    let Some(runnable) = stage(root) else {
        return vec![finding(
            NAME,
            Some("the-hook-could-not-be-staged"),
            format!("`{SCRATCH}` could not be written, so the scanner could not be run."),
        )];
    };

    let target = root.join(GUARDED);
    let mut wrong = Vec::new();
    for (excepted, line) in TABLE {
        match verdict(&runnable, &target, line) {
            None => {
                return vec![finding(
                    NAME,
                    Some("the-hook-did-not-answer"),
                    format!(
                        "`{HOOK}` returned nothing readable for `{line}`. A gate that cannot \
                         be asked what it does is not one this can check."
                    ),
                )];
            }
            Some(denied) if denied == *excepted => wrong.push((*excepted, *line, denied)),
            Some(_) => {}
        }
    }

    wrong
        .into_iter()
        .map(|(excepted, line, _)| {
            if excepted {
                finding(
                    NAME,
                    Some("an-excepted-position-is-denied"),
                    format!(
                        "the hook denies `{line}`. The type of a const generic parameter is \
                         the position the canon excepts, at \
                         `obligation::a_primitive_for_every_position_a_bare_number_would_take`, \
                         and the hook fires before any lint, so this is what an agent can \
                         actually write however the lints are configured."
                    ),
                )
            } else {
                finding(
                    NAME,
                    Some("a-refused-position-is-allowed"),
                    format!(
                        "the hook allows `{line}`. The exception reaches the type of a const \
                         generic parameter and no further, so this line is one the canon \
                         still refuses and the gate has stopped refusing it."
                    ),
                )
            }
        })
        .collect()
}

/// The template with the helpers stood in for, written where bash can run it.
///
/// Separate from `check` so an arm testing one verdict can reach a runnable copy
/// without going through the whole table, and so the substitution has one home.
fn stage(root: &Path) -> Option<std::path::PathBuf> {
    let template = std::fs::read_to_string(root.join(HOOK)).ok()?;
    let runnable = root.join(SCRATCH);
    std::fs::create_dir_all(runnable.parent()?).ok()?;
    std::fs::write(&runnable, template.replace(HELPERS, STUB)).ok()?;
    Some(runnable)
}

/// Whether the scanner denies one line, or nothing where it could not be asked.
///
/// The line goes in through the environment rather than through the payload the
/// real hook parses, because the stub above is what reads it and the parsing is
/// mockspace's. So a line holding a quote or a backslash needs no escaping and
/// there is no payload that can fail to parse, which is the failure that would
/// otherwise read as an allow on every row at once.
fn verdict(hook: &Path, target: &Path, line: &str) -> Option<bool> {
    let out = Command::new("bash")
        .arg(hook)
        .env("PROBE_PATH", target)
        .env("PROBE_LINE", line)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .output()
        .ok()?;
    let text = String::from_utf8_lossy(&out.stdout);
    if text.contains("__DENY__") {
        return Some(true);
    }
    if text.contains("__ALLOW__") {
        return Some(false);
    }
    None
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use mockspace::Lint;

    use super::{check, verdict, TheWriteHookExcepts, GUARDED, HELPERS, HOOK, NAME, TABLE};
    use crate::canon_lint_testkit::{
        assert_findings_block_at, assert_not_declared_off, assert_registered, ctx_at, plant,
        planted_tree, view,
    };

    /// A tree with a hook template in it, whose scanner is supplied by the caller.
    ///
    /// The helper marker is written in as the engine's template carries it, so
    /// the substitution this lint performs is exercised rather than assumed.
    fn tree_with_hook(what: &str, scanner: &str) -> PathBuf {
        let dir = planted_tree(what);
        plant(
            &dir,
            HOOK,
            &format!("#!/usr/bin/env bash\n{HELPERS}\n{scanner}\nallow\n"),
        );
        plant(&dir, GUARDED, "");
        dir
    }

    /// The runnable copy `check` writes, for an arm calling `verdict` directly.
    ///
    /// The template itself is not runnable: it still carries the helper marker,
    /// and bash on it fails, which `verdict` reports as no answer. Two arms
    /// asserted against the template and got exactly that.
    fn staged(root: &std::path::Path) -> PathBuf {
        super::stage(root).expect("the planted template stages")
    }

    /// A scanner that denies exactly the lines whose content matches a needle.
    fn denies_when_containing(needle: &str) -> String {
        format!(
            "LINE=$(_extract content)\nif printf '%s' \"$LINE\" | grep -qF '{needle}'; then \
             deny x; fi"
        )
    }

    #[test]
    fn the_lint_is_named_and_refuses() {
        let lint = TheWriteHookExcepts;
        assert_eq!(lint.name(), NAME);
        assert_eq!(lint.default_severity(), mockspace::Severity::HARD_ERROR);
    }

    #[test]
    fn a_missing_hook_is_reported_rather_than_passed() {
        let dir = planted_tree("no-hook-at-all");
        let out = check(&dir);
        assert_eq!(out.len(), 1, "one finding for the absent hook");
        assert_eq!(out[0].finding_kind, Some("the-hook-is-not-there"));
    }

    #[test]
    fn a_template_with_no_helper_marker_is_reported() {
        // Without the marker there is nothing to stand the helpers in for, so
        // the scanner cannot run. Reporting that is the only honest answer: a
        // silent pass here would be a lint that stopped checking.
        let dir = planted_tree("no-marker");
        plant(&dir, HOOK, "#!/usr/bin/env bash\nexit 0\n");
        plant(&dir, GUARDED, "");
        let out = check(&dir);
        assert_eq!(out.len(), 1);
        assert_eq!(
            out[0].finding_kind,
            Some("the-hook-carries-no-helper-marker")
        );
    }

    #[test]
    fn a_hook_that_answers_the_table_correctly_passes() {
        // The refused rows share no single needle, so this arm lists them.
        let mut scanner = String::from("LINE=$(_extract content)\ncase \"$LINE\" in\n");
        for (excepted, line) in TABLE {
            if !excepted {
                scanner.push_str(&format!(
                    "  *'{}'*) deny x ;;\n",
                    line.replace('\'', "'\\''")
                ));
            }
        }
        scanner.push_str("esac");
        let dir = tree_with_hook("agrees", &scanner);
        assert!(
            check(&dir).is_empty(),
            "a hook agreeing with the table has nothing to report"
        );
    }

    #[test]
    fn a_hook_that_denies_the_excepted_position_is_caught() {
        // The defect this lint was written for, reproduced: a scanner with no
        // exception denies the parameter along with everything else.
        let dir = tree_with_hook("denies-the-exception", &denies_when_containing("u32"));
        let out = check(&dir);
        let kinds: Vec<_> = out.iter().filter_map(|e| e.finding_kind).collect();
        assert!(
            kinds.contains(&"an-excepted-position-is-denied"),
            "the denied exception has to be reported: {kinds:?}"
        );
    }

    #[test]
    fn a_hook_that_allows_everything_is_caught() {
        // Without this arm the one above passes for a lint that only ever looks
        // at the excepted half, and a gate that refuses nothing would clear it.
        let dir = tree_with_hook("allows-everything", ":");
        let out = check(&dir);
        let denied = TABLE.iter().filter(|(excepted, _)| !excepted).count();
        assert_eq!(
            out.len(),
            denied,
            "every control row has to be reported, not merely one"
        );
        assert!(out
            .iter()
            .all(|e| e.finding_kind == Some("a-refused-position-is-allowed")));
    }

    #[test]
    fn a_hook_that_says_nothing_is_reported_rather_than_read_as_an_allow() {
        // An answer that is neither word is neither verdict. Reading it as an
        // allow is the direction that hides a broken gate.
        let dir = planted_tree("says-nothing");
        plant(
            &dir,
            HOOK,
            &format!("#!/usr/bin/env bash\n{HELPERS}\nexit 0\n"),
        );
        plant(&dir, GUARDED, "");
        let out = check(&dir);
        assert_eq!(out.len(), 1);
        assert_eq!(out[0].finding_kind, Some("the-hook-did-not-answer"));
    }

    #[test]
    fn the_table_carries_both_halves() {
        // A table of one kind cannot fail in the other direction, and the two
        // controls above each rest on there being rows of the opposite sign.
        assert!(TABLE.iter().any(|(excepted, _)| *excepted));
        assert!(TABLE.iter().any(|(excepted, _)| !excepted));
    }

    #[test]
    fn it_reaches_the_pack_the_engine_is_handed() {
        assert_registered(NAME);
    }

    #[test]
    fn it_does_not_declare_itself_off() {
        assert_not_declared_off(&TheWriteHookExcepts);
    }

    #[test]
    fn its_findings_block_every_gate() {
        // A warning here is worth nothing. The hook fires before any lint, so a
        // disagreement that does not block is one an agent keeps hitting while
        // the pack reports clean beside it.
        let root = planted_tree("hook-severity");
        let empty = view(&[], &[]);
        assert_findings_block_at(&TheWriteHookExcepts, &ctx_at(&root.join("mock"), &empty));
    }

    #[test]
    fn a_line_carrying_a_quote_reaches_the_scanner_whole() {
        // The line goes through the environment, so nothing about it has to be
        // escaped. Pinned because the earlier shape built a JSON payload, where
        // a quote produced a document that did not parse and therefore an allow
        // on every row at once.
        let dir = tree_with_hook("quoted-line", &denies_when_containing("\"a\\b\""));
        let target = dir.join(GUARDED);
        let hook = staged(&dir);
        assert_eq!(verdict(&hook, &target, "let s = \"a\\b\";"), Some(true));
        assert_eq!(verdict(&hook, &target, "let s = 1;"), Some(false));
    }

    #[test]
    fn a_template_that_is_not_executable_still_answers_through_bash() {
        // A file's mode is the generator's business and not this lint's, so it
        // is invoked through `bash` rather than executed. Pinned because running
        // it directly would fail and the failure would read as a denied position.
        let dir = tree_with_hook(
            "not-executable",
            &denies_when_containing("nothing-matches-this"),
        );
        let target = dir.join(GUARDED);
        let hook = staged(&dir);
        assert_eq!(verdict(&hook, &target, "pub struct A(u32);"), Some(false));
    }
}
