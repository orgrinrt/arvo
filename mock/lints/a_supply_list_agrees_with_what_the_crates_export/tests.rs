//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! Tests for `a-supply-list-agrees-with-what-the-crates-export`.

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

/// A supply file in the shape rustfmt gives the real one: the gone list on one
/// line closing inline, a third list after it, and a function full of string
/// literals after that, so a reading that runs past the list's end picks them up.
fn supply(shipped: &[&str], gone: &[&str]) -> String {
    let quoted = |names: &[&str]| {
        names
            .iter()
            .map(|n| format!("\"{n}\""))
            .collect::<Vec<_>>()
            .join(", ")
    };
    format!(
        "pub const ARVO_SHIPPED: &[&str] = &[\n    // arvo-thing\n{}];\n\n\
         /// Names consumers write as arvo's.\n\
         pub const ARVO_GONE: &[&str] =\n    &[{}];\n\n\
         pub const NOTKO: &[&str] = &[\"Just\", \"Outcome\"];\n\n\
         pub fn supplier(name: &str) -> Option<&'static str> {{\n    \
         if ARVO_SHIPPED.contains(&name) {{ Some(\"arvo\") }} else {{ Some(\"notko\") }}\n}}\n",
        shipped
            .iter()
            .map(|n| format!("    \"{n}\",\n"))
            .collect::<String>(),
        quoted(gone),
    )
}

/// A mock directory with that supply file and one crate whose `lib.rs` is `lib`.
fn planted(what: &str, shipped: &[&str], gone: &[&str], lib: &str) -> PathBuf {
    let dir = planted_tree(what);
    plant(&dir, SUPPLY, &supply(shipped, gone));
    plant(&dir, "crates/arvo-thing/src/lib.rs", lib);
    dir
}

fn found(dir: &Path) -> Vec<LintError> {
    through_view(dir, |v| check(v, dir))
}

fn listed(dir: &Path, item: &str) -> Option<Vec<String>> {
    through_view(dir, |v| {
        let path = dir.join(SUPPLY);
        list(v.text(&path)?, v.rust(&path)?, item)
    })
    .map(|names| names.into_iter().map(|(_, name)| name).collect())
}

#[test]
fn a_list_closing_inline_is_read_to_its_own_end_and_no_further() {
    let dir = planted(
        "inline",
        &["Width"],
        &["Cap", "UWire"],
        "pub use w::Width;\n",
    );
    assert_eq!(
        listed(&dir, GONE),
        Some(vec!["Cap".to_string(), "UWire".to_string()])
    );
    assert_eq!(listed(&dir, SHIPPED), Some(vec!["Width".to_string()]));
}

#[test]
fn a_gone_name_in_a_vertical_pub_use_is_a_finding() {
    // rustfmt splits a long `pub use` one name to a line, so no line holding
    // the name begins with `pub`.
    let lib = "pub use bits::{\n    Bits,\n    Bytes,\n};\npub use w::Width;\n";
    let dir = planted("vertical", &["Width"], &["Cap", "Bits"], lib);
    let found = found(&dir);
    assert_eq!(found.len(), 1, "one gone name ships: {found:?}");
    assert!(found[0].message.contains("`Bits`"), "{}", found[0].message);
    assert!(
        found[0].message.contains("arvo-thing"),
        "{}",
        found[0].message
    );
    assert_eq!(found[0].line, 8, "the line of the name in the gone list");
    assert_eq!(found[0].finding_kind, Some("a-name-filed-as-gone-ships"));
}

#[test]
fn a_gone_name_a_crate_declares_or_aliases_at_its_root_is_a_finding() {
    for (what, lib) in [
        ("declared", "pub struct Cap(u32);\npub use w::Width;\n"),
        (
            "aliased",
            "pub use other::Capacity as Cap;\npub use w::Width;\n",
        ),
        ("module", "pub mod Cap {}\npub use w::Width;\n"),
        (
            "own-self",
            "pub use outer::Cap::{self};\npub use w::Width;\n",
        ),
    ] {
        let dir = planted(what, &["Width"], &["Cap"], lib);
        assert_eq!(found(&dir).len(), 1, "{what}: the root exports `Cap`");
    }
}

#[test]
fn what_does_not_reach_a_consumer_is_no_export() {
    // A longer name, a private import, a restricted one, a name replaced by an
    // alias, a comment, a string, and a public name inside a private module.
    let lib = "pub use thing::Capacity;\nuse inner::Cap;\npub(crate) use inner2::Cap;\n\
               pub use thing::Cap as Lid;\n// pub use old::Cap;\n\
               pub const NOTE: &str = \"Cap\";\nmod slots { pub struct Cap; }\n\
               pub use w::Width;\n";
    let dir = planted("near", &["Width"], &["Cap"], lib);
    let found = found(&dir);
    assert!(
        found.is_empty(),
        "nothing exports `Cap` at the root: {found:?}"
    );
}

#[test]
fn a_shipped_name_no_crate_exports_is_a_finding() {
    // A public type under a private module is out of every consumer's reach.
    let lib = "pub use w::Width;\nmod slots { pub struct Unsigned; }\n";
    let dir = planted("unexported", &["Width", "Unsigned"], &["Cap"], lib);
    let found = found(&dir);
    assert_eq!(found.len(), 1, "{found:?}");
    assert!(
        found[0].message.contains("`Unsigned`"),
        "{}",
        found[0].message
    );
    assert_eq!(
        found[0].finding_kind,
        Some("a-name-filed-as-shipped-is-not-exported")
    );
}

#[test]
fn a_shipped_name_exported_any_way_a_root_can_is_no_finding() {
    let lib = "pub use a::{\n    b::{self},\n    Width as Wide,\n};\npub trait Bool {}\n";
    let dir = planted("shipped", &["b", "Wide", "Bool"], &["Cap"], lib);
    let found = found(&dir);
    assert!(found.is_empty(), "{found:?}");
}

#[test]
fn a_name_through_a_public_module_ships_inline_in_a_file_and_in_a_nested_file() {
    // `arvo_format::slots::Unsigned` and `arvo_strategy::presets::Hot` are this
    // shape: a consumer names them by a longer path, and they are there.
    let lib = "pub mod presets { pub struct Hot; }\npub mod slots;\npub mod deep;\n";
    let dir = planted(
        "modules",
        &["Hot", "Unsigned", "Inner", "Wrapped"],
        &["Cap"],
        lib,
    );
    plant(
        &dir,
        "crates/arvo-thing/src/slots.rs",
        "pub struct Unsigned;\npub mod inner;\n",
    );
    plant(
        &dir,
        "crates/arvo-thing/src/slots/inner.rs",
        "pub struct Inner;\n",
    );
    plant(
        &dir,
        "crates/arvo-thing/src/deep/mod.rs",
        "pub use x::Wrapped;\n",
    );
    let found = found(&dir);
    assert!(found.is_empty(), "{found:?}");
}

#[test]
fn a_gone_name_a_public_module_declares_is_a_finding() {
    let lib = "pub mod bits;\npub use w::Width;\n";
    let dir = planted("gone-in-module", &["Width"], &["Bits"], lib);
    plant(&dir, "crates/arvo-thing/src/bits.rs", "pub struct Bits;\n");
    assert_eq!(found(&dir).len(), 1, "`Bits` ships through `bits`");
}

#[test]
fn a_supply_file_without_its_lists_is_a_finding() {
    let dir = planted_tree("missing");
    plant(&dir, SUPPLY, "pub const NOTKO: &[&str] = &[\"Just\"];\n");
    plant(&dir, "crates/arvo-thing/src/lib.rs", "pub use w::Width;\n");
    let kinds: Vec<_> = found(&dir).into_iter().map(|f| f.finding_kind).collect();
    assert_eq!(kinds, vec![
        Some("a-supply-list-is-not-found"),
        Some("a-supply-list-is-not-found")
    ]);
}

#[test]
fn the_reading_of_the_shipped_tree_takes_its_lists_and_nothing_past_them() {
    // A lint that reads nothing reports nothing, and one that reads too much
    // judges names that are not on the list. The control is both: the lists
    // parse to names, and none of the third list's names or of the strings
    // after it are among them.
    let mock = repo_root().join("mock");
    let gone = listed(&mock, GONE).expect("ARVO_GONE");
    assert!(gone.iter().any(|name| name == "Cap"), "{gone:?}");
    for stray in ["Just", "Maybe", "Outcome", "Lent", "arvo", "gone", "notko"] {
        assert!(
            !gone.iter().any(|name| name == stray),
            "`{stray}` read: {gone:?}"
        );
    }
    let shipped = listed(&mock, SHIPPED).expect("ARVO_SHIPPED");
    assert!(shipped.iter().any(|name| name == "Width"), "{shipped:?}");
    let crates = through_view(&mock, |v| crates(v, &mock));
    let bits = crates
        .iter()
        .find(|(krate, _)| krate == "arvo-bits")
        .expect("arvo-bits is read");
    assert!(bits.1.contains("Bits"), "{:?}", bits.1);
}

#[test]
fn the_shipped_tree_agrees_with_its_supply_lists() {
    let mock = repo_root().join("mock");
    let found = through_view(&mock, |v| check(v, &mock));
    assert!(found.is_empty(), "{found:?}");
}

#[test]
fn a_finding_blocks_at_commit() {
    let dir = planted(
        "blocks",
        &["Width"],
        &["Bits"],
        "pub use bits::Bits;\npub use w::Width;\n",
    );
    let registry = view(&[], &[]);
    assert_findings_block_at(
        &ASupplyListAgreesWithWhatTheCratesExport,
        ctx_at(&dir, &registry),
    );
}

#[test]
fn it_is_registered_and_not_declared_off() {
    assert_registered(NAME);
    assert_not_declared_off(&ASupplyListAgreesWithWhatTheCratesExport);
}
