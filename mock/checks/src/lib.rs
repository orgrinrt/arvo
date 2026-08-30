//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! What a TOML schema structurally cannot check about the canon.
//!
//! The generated schemas already state required fields, slug shape, types and
//! the closed value sets, and `mock check` runs them through a validator. This
//! crate is for the rest, and the rest is most of what makes a row honest: that
//! a predicate names axes somebody declared, that a citation into a living
//! ledger does not carry a line number, that a row marked as measured has an
//! instrument behind it.
//!
//! # Shape
//!
//! Every arm is a pure function from rows to findings. Nothing here reads the
//! filesystem except [`load`], and nothing mutates anything. That is what makes
//! each arm testable against a planted input as easily as against the committed
//! canon, and **the planted input is not optional**: an arm that has only ever
//! run over a clean canon has returned an empty list and established nothing.
//! Each test file runs its arm both ways.

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

use toml_edit::{DocumentMut, Item, Value};

pub mod citation;
pub mod comments;
pub mod predicate;
pub mod shape;

/// One thing wrong, named so a reader can go and look at it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Finding {
    /// The arm that produced it, in kebab-case, so a report can group by kind.
    pub kind: &'static str,
    /// `namespace::slug`, or the file where the row could not be identified.
    pub at: String,
    /// What is wrong, in a sentence a person can act on.
    pub says: String,
}

impl Finding {
    pub fn new(kind: &'static str, at: impl Into<String>, says: impl Into<String>) -> Self {
        Self {
            kind,
            at: at.into(),
            says: says.into(),
        }
    }
}

/// One registry row, flattened to what an arm needs.
#[derive(Debug, Clone)]
pub struct Row {
    /// The array-of-tables key the row was declared under.
    pub namespace: String,
    /// The slug, from `id`.
    pub id: String,
    /// Which file it sits in, for a report that names a place.
    pub file: PathBuf,
    /// Scalar fields, as their string form.
    pub strings: BTreeMap<String, String>,
    /// Array fields, as their string entries. An array of anything else is
    /// dropped, because nothing here reads one and a silent partial read is
    /// worse than an absence.
    pub lists: BTreeMap<String, Vec<String>>,
}

impl Row {
    /// `namespace::slug`, which is how a row is addressed everywhere else.
    pub fn addr(&self) -> String {
        format!("{}::{}", self.namespace, self.id)
    }

    pub fn get(&self, field: &str) -> Option<&str> {
        self.strings.get(field).map(String::as_str)
    }

    pub fn list(&self, field: &str) -> &[String] {
        self.lists.get(field).map_or(&[], Vec::as_slice)
    }

    /// Whether the field is present at all, in either shape.
    pub fn has(&self, field: &str) -> bool {
        self.strings.contains_key(field) || self.lists.contains_key(field)
    }
}

/// Every row under a registry directory, with the raw text of each file beside it.
#[derive(Debug, Clone, Default)]
pub struct Registry {
    pub rows: Vec<Row>,
    /// Raw file text, kept because one arm is about what a comment claims and a
    /// parsed document has thrown the comments away by the time it is walked.
    pub texts: BTreeMap<PathBuf, String>,
}

impl Registry {
    pub fn of(&self, namespace: &str) -> impl Iterator<Item = &Row> {
        self.rows.iter().filter(move |r| r.namespace == namespace)
    }

    /// The slugs declared in one namespace, for an arm checking a reference
    /// into it.
    pub fn slugs(&self, namespace: &str) -> Vec<&str> {
        self.of(namespace).map(|r| r.id.as_str()).collect()
    }
}

/// The repository root, from the crate's own manifest directory.
///
/// **Read at run time, never through `env!`.** The macro bakes in the directory
/// of whichever tree compiled the binary, and cargo shares one target directory
/// across every worktree cut from a clone. So the first tree to build a check
/// leaves a binary that answers about that tree from then on, in every other
/// tree, with no rebuild and nothing said. Several seats work in worktrees of
/// this repository at once, which is exactly the arrangement that turns that
/// into a wrong answer rather than a curiosity.
pub fn repo() -> PathBuf {
    let mut p = std::env::var_os("CARGO_MANIFEST_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|| std::env::current_dir().expect("a working directory"));
    // `<repo>/mock/checks` is where cargo puts us.
    for _ in 0..2 {
        p.pop();
    }
    p
}

/// The committed canon.
pub fn canon() -> Registry {
    load(&repo().join("mock/registry")).expect("mock/registry is readable")
}

/// Every `*.toml` under a directory, parsed into rows.
///
/// A row's namespace is the array-of-tables key it is declared under and never
/// its path, which is what lets the canon be filed by subject and still be
/// queried by kind. Nested directories are walked.
pub fn load(dir: &Path) -> std::io::Result<Registry> {
    let mut reg = Registry::default();
    walk(dir, &mut reg)?;
    reg.rows
        .sort_by(|a, b| (&a.namespace, &a.id).cmp(&(&b.namespace, &b.id)));
    Ok(reg)
}

/// Parse one document's rows in, for a test planting an input rather than
/// reading a file.
pub fn parse(name: &str, text: &str) -> Registry {
    let mut reg = Registry::default();
    let file = PathBuf::from(name);
    absorb(&file, text, &mut reg);
    reg
}

fn walk(dir: &Path, reg: &mut Registry) -> std::io::Result<()> {
    if !dir.is_dir() {
        return Ok(());
    }
    let mut entries: Vec<_> = fs::read_dir(dir)?.collect::<Result<_, _>>()?;
    entries.sort_by_key(std::fs::DirEntry::path);
    for entry in entries {
        let path = entry.path();
        if path.is_dir() {
            walk(&path, reg)?;
        } else if path.extension().is_some_and(|e| e == "toml") {
            let text = fs::read_to_string(&path)?;
            absorb(&path, &text, reg);
        }
    }
    Ok(())
}

fn absorb(file: &Path, text: &str, reg: &mut Registry) {
    reg.texts.insert(file.to_path_buf(), text.to_string());
    let Ok(doc) = text.parse::<DocumentMut>() else {
        return; // a file that does not parse is the schema check's report to make
    };
    for (namespace, item) in doc.iter() {
        let Item::ArrayOfTables(tables) = item else {
            continue;
        };
        for table in tables.iter() {
            let mut strings = BTreeMap::new();
            let mut lists = BTreeMap::new();
            for (name, value) in table.iter() {
                match value {
                    Item::Value(Value::String(s)) => {
                        strings.insert(name.to_string(), s.value().to_string());
                    }
                    Item::Value(Value::Integer(i)) => {
                        strings.insert(name.to_string(), i.value().to_string());
                    }
                    Item::Value(Value::Boolean(b)) => {
                        strings.insert(name.to_string(), b.value().to_string());
                    }
                    Item::Value(Value::Array(a)) => {
                        let entries: Vec<String> = a
                            .iter()
                            .filter_map(|v| v.as_str().map(str::to_string))
                            .collect();
                        lists.insert(name.to_string(), entries);
                    }
                    _ => {}
                }
            }
            let Some(id) = strings.get("id").cloned() else {
                continue; // the schema reports a row with no id; this arm needs one to name
            };
            reg.rows.push(Row {
                namespace: namespace.to_string(),
                id,
                file: file.to_path_buf(),
                strings,
                lists,
            });
        }
    }
}
