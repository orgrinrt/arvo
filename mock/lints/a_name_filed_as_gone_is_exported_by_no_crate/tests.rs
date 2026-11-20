//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! Tests for `a-name-filed-as-gone-is-exported-by-no-crate`.

use std::path::PathBuf;

use super::*;
use crate::canon_lint_testkit::{
    assert_findings_block_at,
    assert_not_declared_off,
    assert_registered,
    ctx_at,
    plant,
    planted_tree,
    repo_root,
    through_view,
    view,
};

/// A supply file whose gone list holds `names`, one to a line.
fn supply(names: &[&str]) -> String {
    let mut out = String::from(
        "pub const ARVO_SHIPPED: &[&str] = &[\n    \"Width\",\n];\n\n\
         /// Names consumers write as arvo's.\npub const ARVO_GONE: &[&str] = &[\n",
    );
    for name in names {
        out.push_str(&format!("    \"{name}\",\n"));
    }
    out.push_str("];\n\npub const NOTKO: &[&str] = &[\n    \"Maybe\",\n];\n");
    out
}

/// A mock directory with that supply file and one crate whose `lib.rs` is `lib`.
fn planted(what: &str, names: &[&str], lib: &str) -> PathBuf {
    let dir = planted_tree(what);
    plant(&dir, SUPPLY, &supply(names));
    plant(&dir, "crates/arvo-thing/src/lib.rs", lib);
    dir
}

fn found(dir: &Path) -> Vec<LintError> {
    through_view(dir, |v| check(v, dir))
}

#[test]
fn a_gone_name_a_crate_reexports_is_a_finding() {
    let dir = planted(
        "reexport",
        &["Cap", "Bits"],
        "pub use bits::{Bits, Bytes};\n",
    );
    let found = found(&dir);
    assert_eq!(found.len(), 1, "one gone name ships: {found:?}");
    assert!(found[0].message.contains("`Bits`"), "{}", found[0].message);
    assert!(
        found[0].message.contains("arvo-thing"),
        "{}",
        found[0].message
    );
    assert_eq!(found[0].line, 8, "the line of the name in the gone list");
}

#[test]
fn a_gone_name_a_crate_declares_at_its_root_is_a_finding() {
    let dir = planted("declared", &["Cap"], "pub struct Cap(u32);\n");
    assert_eq!(found(&dir).len(), 1, "a declaration exports the name too");
}

#[test]
fn the_near_misses_are_no_finding() {
    // A longer name, a private import, a comment and a string do not export it.
    let lib = "pub use thing::Capacity;\nuse inner::Cap;\n// pub use old::Cap;\n\
               pub const NOTE: &str = \"Cap\";\n";
    let dir = planted("near", &["Cap"], lib);
    let found = found(&dir);
    assert!(found.is_empty(), "no line exports `Cap`: {found:?}");
}

#[test]
fn a_gone_list_nothing_exports_is_no_finding() {
    let dir = planted("clean", &["Cap", "UWire"], "pub use bits::Bits;\n");
    assert!(found(&dir).is_empty());
}

#[test]
fn the_reading_finds_the_list_it_judges() {
    // A lint that reads nothing reports nothing, so the control is that the
    // shipped list parses to names and the crates to sources.
    let mock = repo_root().join("mock");
    let names = through_view(&mock, |v| {
        gone(&crate::code_only_source::without_comments(v, mock.join(SUPPLY)).expect("supply.rs"))
    });
    assert!(names.iter().any(|(_, name)| name == "Cap"), "{names:?}");
    let libs = through_view(&mock, |v| libs(v, &mock));
    assert!(
        libs.iter().any(|(krate, _)| krate == "arvo-bits"),
        "the crates are read"
    );
}

#[test]
fn the_shipped_tree_files_no_shipped_name_as_gone() {
    let mock = repo_root().join("mock");
    let found = through_view(&mock, |v| check(v, &mock));
    assert!(found.is_empty(), "{found:?}");
}

#[test]
fn a_finding_blocks_at_commit() {
    let dir = planted("blocks", &["Bits"], "pub use bits::Bits;\n");
    let registry = view(&[], &[]);
    assert_findings_block_at(
        &ANameFiledAsGoneIsExportedByNoCrate,
        ctx_at(&dir, &registry),
    );
}

#[test]
fn it_is_registered_and_not_declared_off() {
    assert_registered(NAME);
    assert_not_declared_off(&ANameFiledAsGoneIsExportedByNoCrate);
}
