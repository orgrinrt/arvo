//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! Lint: the supply lists of `the-positions` agree with what the crates export.
//!
//! `mock/tools/the-positions/src/supply.rs` keeps two hand-written lists.
//! `ARVO_SHIPPED` is the names of types and items an arvo crate ships, and
//! `ARVO_GONE` the names consumers still write as arvo's that no crate has. The
//! report counts a position carrying a gone name as pointing at nothing, so a
//! list that drifts from the tree files a met obligation as unmet, or an unmet
//! one as met. `Bits` and `BitAccess` stayed on the gone list a whole tier
//! after they shipped, with a test asserting it.
//!
//! Two findings, one per direction: a name on `ARVO_GONE` that some crate
//! exports, and a name on `ARVO_SHIPPED` that no crate exports. A supply file
//! that holds neither list is a finding too, so a moved or renamed list cannot
//! pass by being read as empty.
//!
//! Both sides are read from the parse. A list is the string literals inside the
//! value of the `const` item of that name, and nothing past that item's end. A
//! crate exports what its `lib.rs` declares `pub`, and what every module it
//! reaches through a `pub mod` declares `pub`, inline or in the module's file:
//! each name a `pub use` brings in (under its alias where it has one, and the
//! module itself for a `self` in braces), however rustfmt lays the braces out,
//! and the name of a `pub` struct, enum, union, trait, type, const, static, fn
//! or mod. So `arvo_format::slots::Unsigned` and `arvo_strategy::presets::Hot`
//! ship: a consumer can name them, by a longer path than it may write today.
//! `pub(crate)` and the other restricted forms export nothing, and neither does
//! anything under a private module. A `pub use` of a glob names nothing this
//! lint can see, so a name that ships only through a glob reads as unexported,
//! and a module file placed by a `#[path]` attribute is not followed.

use std::collections::BTreeSet;
use std::path::Path;

use mockspace::tree_sitter::{Node, Tree};
use mockspace::{Lint, LintError, RepoContext, RepoLint, Severity, View};

pub fn repo_lint() -> Box<dyn RepoLint> {
    Box::new(ASupplyListAgreesWithWhatTheCratesExport)
}

/// The lint's own name, used in its findings and keyed by `[lints.<name>]`.
const NAME: &str = "a-supply-list-agrees-with-what-the-crates-export";

/// Where the lists live, under the mock directory.
const SUPPLY: &str = "tools/the-positions/src/supply.rs";

/// The `const` holding the names filed as gone.
const GONE: &str = "ARVO_GONE";

/// The `const` holding the names filed as shipped.
const SHIPPED: &str = "ARVO_SHIPPED";

/// The item kinds whose `name` field is what a `pub` on them exports.
const NAMED_ITEMS: [&str; 9] = [
    "struct_item",
    "enum_item",
    "union_item",
    "trait_item",
    "type_item",
    "const_item",
    "static_item",
    "function_item",
    "mod_item",
];

struct ASupplyListAgreesWithWhatTheCratesExport;

impl Lint for ASupplyListAgreesWithWhatTheCratesExport {
    fn name(&self) -> &'static str {
        NAME
    }

    fn default_severity(&self) -> Severity {
        Severity::HARD_ERROR
    }
}

impl RepoLint for ASupplyListAgreesWithWhatTheCratesExport {
    fn check_repo(&self, ctx: &RepoContext) -> Vec<LintError> {
        check(ctx.view(), &ctx.mock_dir)
    }
}

/// One finding for each gone name a root exports, each shipped name no root
/// exports, and each list the supply file does not hold.
fn check(view: &View, mock_dir: &Path) -> Vec<LintError> {
    let supply = mock_dir.join(SUPPLY);
    let (Some(text), Some(tree)) = (view.text(&supply), view.rust(&supply)) else {
        return Vec::new();
    };
    let crates = crates(view, mock_dir);
    let mut found = Vec::new();
    match list(text, tree, GONE) {
        Some(gone) => {
            for (line, name) in &gone {
                for (krate, exports) in &crates {
                    if exports.contains(name) {
                        found.push(gone_but_exported(*line, name, krate));
                    }
                }
            }
        },
        None => found.push(missing(GONE)),
    }
    match list(text, tree, SHIPPED) {
        Some(shipped) => {
            for (line, name) in &shipped {
                if !crates.iter().any(|(_, exports)| exports.contains(name)) {
                    found.push(shipped_but_unexported(*line, name));
                }
            }
        },
        None => found.push(missing(SHIPPED)),
    }
    found
}

/// The strings of the top-level `const` named `item`, each with its one-based
/// line, or `None` where the file declares no such item.
fn list(text: &str, tree: &Tree, item: &str) -> Option<Vec<(usize, String)>> {
    let src = text.as_bytes();
    let root = tree.root_node();
    let mut cursor = root.walk();
    let found = root.named_children(&mut cursor).find(|node| {
        node.kind() == "const_item"
            && node
                .child_by_field_name("name")
                .is_some_and(|name| node_text(name, src) == item)
    })?;
    let mut out = Vec::new();
    if let Some(value) = found.child_by_field_name("value") {
        strings(value, src, &mut out);
    }
    Some(out)
}

/// Every string literal under `node`, without its quotes, with its line.
fn strings(node: Node, src: &[u8], out: &mut Vec<(usize, String)>) {
    if node.kind() == "string_literal" {
        let quoted = node_text(node, src);
        let inner = quoted
            .strip_prefix('"')
            .and_then(|s| s.strip_suffix('"'))
            .unwrap_or(quoted);
        out.push((node.start_position().row + 1, inner.to_string()));
        return;
    }
    let mut cursor = node.walk();
    for child in node.named_children(&mut cursor) {
        strings(child, src, out);
    }
}

/// What every crate exports, by crate.
fn crates(view: &View, mock_dir: &Path) -> Vec<(String, BTreeSet<String>)> {
    let crates = mock_dir.join("crates");
    let Some(entries) = view.list(&crates) else {
        return Vec::new();
    };
    let mut out = Vec::new();
    for entry in entries.entries() {
        let src = crates.join(entry.name()).join("src");
        let lib = src.join("lib.rs");
        if view.is_file(&lib) {
            let mut names = BTreeSet::new();
            file_exports(view, &lib, &src, &mut names);
            out.push((entry.name().to_string(), names));
        }
    }
    out
}

/// The names one module file exports, and the modules it declares `pub`.
/// `dir` is where the file's own `mod x;` declarations are found.
fn file_exports(view: &View, file: &Path, dir: &Path, out: &mut BTreeSet<String>) {
    if let (Some(text), Some(tree)) = (view.text(file), view.rust(file)) {
        module_exports(view, tree.root_node(), text.as_bytes(), dir, out);
    }
}

/// The names the `pub` items of one module body export, following every
/// `pub mod` into its body or its file.
fn module_exports(view: &View, body: Node, src: &[u8], dir: &Path, out: &mut BTreeSet<String>) {
    let mut cursor = body.walk();
    for item in body.named_children(&mut cursor) {
        if !is_plain_pub(item, src) {
            continue;
        }
        if item.kind() == "use_declaration" {
            if let Some(argument) = item.child_by_field_name("argument") {
                use_names(argument, src, None, out);
            }
            continue;
        }
        if !NAMED_ITEMS.contains(&item.kind()) {
            continue;
        }
        let Some(name) = item.child_by_field_name("name") else {
            continue;
        };
        let name = node_text(name, src);
        out.insert(name.to_string());
        if item.kind() != "mod_item" {
            continue;
        }
        let inner = dir.join(name);
        match item.child_by_field_name("body") {
            Some(body) => module_exports(view, body, src, &inner, out),
            None => {
                let flat = dir.join(format!("{name}.rs"));
                let nested = inner.join("mod.rs");
                if view.is_file(&flat) {
                    file_exports(view, &flat, &inner, out);
                } else if view.is_file(&nested) {
                    file_exports(view, &nested, &inner, out);
                }
            },
        }
    }
}

/// Whether the item carries a bare `pub`, the one visibility that reaches a
/// consumer of the crate.
fn is_plain_pub(item: Node, src: &[u8]) -> bool {
    let mut cursor = item.walk();
    let found = item
        .children(&mut cursor)
        .find(|child| child.kind() == "visibility_modifier")
        .is_some_and(|vis| node_text(vis, src) == "pub");
    found
}

/// The names one `use` clause brings in. `owner` is the last segment of the path
/// in front of the braces the clause sits in, which a `self` there names.
fn use_names(node: Node, src: &[u8], owner: Option<&str>, out: &mut BTreeSet<String>) {
    match node.kind() {
        "identifier" => {
            out.insert(node_text(node, src).to_string());
        },
        "self" => {
            if let Some(owner) = owner {
                out.insert(owner.to_string());
            }
        },
        "scoped_identifier" => {
            if let Some(name) = node.child_by_field_name("name") {
                if name.kind() == "self" {
                    if let Some(path) = node.child_by_field_name("path") {
                        out.insert(last_segment(path, src).to_string());
                    }
                } else {
                    out.insert(node_text(name, src).to_string());
                }
            }
        },
        "use_as_clause" => {
            if let Some(alias) = node.child_by_field_name("alias") {
                out.insert(node_text(alias, src).to_string());
            }
        },
        "scoped_use_list" => {
            let owner = node
                .child_by_field_name("path")
                .map(|path| last_segment(path, src));
            if let Some(list) = node.child_by_field_name("list") {
                use_names(list, src, owner, out);
            }
        },
        "use_list" => {
            let mut cursor = node.walk();
            for child in node.named_children(&mut cursor) {
                use_names(child, src, owner, out);
            }
        },
        // A glob names nothing that can be read here; the module doc says so.
        _ => {},
    }
}

/// The last segment of a path: `c` of `a::b::c`, `a` of `a`.
fn last_segment<'a>(path: Node, src: &'a [u8]) -> &'a str {
    match path.child_by_field_name("name") {
        Some(name) => node_text(name, src),
        None => node_text(path, src),
    }
}

fn node_text<'a>(node: Node, src: &'a [u8]) -> &'a str {
    core::str::from_utf8(&src[node.start_byte() .. node.end_byte()]).unwrap_or("")
}

fn at(line: usize, kind: &'static str, message: String) -> LintError {
    let mut error = LintError::error("mock".to_string(), line, NAME, message);
    error.finding_kind = Some(kind);
    error.path = Some(SUPPLY.to_string());
    error
}

fn gone_but_exported(line: usize, name: &str, krate: &str) -> LintError {
    at(
        line,
        "a-name-filed-as-gone-ships",
        format!(
            "`{name}` is filed in `{GONE}` as a name arvo no longer has, and `{krate}` \
             exports it, at its root or through a public module, so the report counts a \
             position carrying it as pointing at nothing when it points at a shipped type. \
             Move it to `{SHIPPED}` and change any test asserting it is gone."
        ),
    )
}

fn shipped_but_unexported(line: usize, name: &str) -> LintError {
    at(
        line,
        "a-name-filed-as-shipped-is-not-exported",
        format!(
            "`{name}` is filed in `{SHIPPED}` as a name an arvo crate exports, and no crate \
             exports it, at a root or through a public module, so the report counts a \
             position carrying it as met when it points at nothing a consumer can import. \
             Move it to `{GONE}`, or make it public in the crate that holds it."
        ),
    )
}

fn missing(item: &str) -> LintError {
    at(
        1,
        "a-supply-list-is-not-found",
        format!(
            "`{SUPPLY}` declares no top-level `const {item}`, so this lint has nothing to hold \
             against what the crates export. Restore the list, or move this lint to \
             wherever it went."
        ),
    )
}

// The generated lint crate loads this file itself through `#[path]`, at the
// name the lint is collected by, so the tests travel with it.
#[cfg(test)]
#[path = "a_supply_list_agrees_with_what_the_crates_export/tests.rs"]
mod tests;
