//! What `mockspace.toml` and `mock/lints` declare, read as text for the tests that
//! ask whether the two agree.

#![cfg(test)]

use std::collections::BTreeSet;

use super::repo_root;

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
pub fn declared_lint_files() -> BTreeSet<String> {
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

/// The namespaces `mockspace.toml` declares a field of this name on.
///
/// A text scan rather than a parse, because the generated pack's manifest is
/// engine-written and no lint file can add a TOML reader to it. The shape it
/// relies on is the one the file carries and the engine's own loader relies on
/// too: an array-of-tables header opens a namespace, `key = "..."` on the line
/// after it names the namespace, and every `name = "..."` after that belongs to
/// that namespace until the next namespace header.
///
/// Returns `None` where the scan found no namespace at all, which means the
/// shape moved and the scan is measuring nothing rather than reporting a clean
/// disagreement.
pub fn namespaces_declaring(field: &str) -> Option<BTreeSet<String>> {
    let text = std::fs::read_to_string(repo_root().join("mockspace.toml")).ok()?;
    let wanted = format!("name = \"{field}\"");
    let mut seen_any = false;
    let mut here: Option<String> = None;
    let mut out = BTreeSet::new();
    for line in text.lines() {
        let line = line.trim();
        if line == "[[registry.namespace]]" {
            here = None;
            seen_any = true;
        } else if here.is_none()
            && let Some(rest) = line.strip_prefix("key = \"")
            && let Some(key) = rest.strip_suffix('"')
        {
            here = Some(key.to_string());
        } else if line == wanted
            && let Some(ns) = &here
        {
            out.insert(ns.clone());
        }
    }
    seen_any.then_some(out)
}
