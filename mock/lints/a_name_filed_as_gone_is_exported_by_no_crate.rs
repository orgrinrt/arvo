//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! Lint: a name `the-positions` files as gone from arvo is exported by no
//! crate's `lib.rs`.
//!
//! `mock/tools/the-positions/src/supply.rs` keeps `ARVO_GONE`, the names
//! consumers still write as arvo's that arvo does not have, and its report
//! counts a position carrying one as pointing at nothing. The list is written by
//! hand, so it goes stale the moment a crate ships one of those names again, and
//! the report then files a met obligation as an unmet one. `Bits` and
//! `BitAccess` stayed on the list a whole tier after they shipped, with a test
//! asserting it.
//!
//! What this reads: every string in the `ARVO_GONE` array, and every
//! `crates/<crate>/src/lib.rs`, with comments and literals blanked. A name is exported where
//! a line of a `lib.rs` opens with `pub` and names it as a whole word: a
//! `pub use`, or a `pub` struct, trait, enum, type, const, fn or mod declared
//! there. What it does not read is a name exported from a module's own file and
//! reached by its path, such as `arvo_format::slots::Unsigned`: the list is of
//! names a consumer imports from a crate's root, and a module path is not one.

use std::path::Path;

use mockspace::{Lint, LintError, RepoContext, RepoLint, Severity, View};

pub fn repo_lint() -> Box<dyn RepoLint> {
    Box::new(ANameFiledAsGoneIsExportedByNoCrate)
}

/// The lint's own name, used in its findings and keyed by `[lints.<name>]`.
const NAME: &str = "a-name-filed-as-gone-is-exported-by-no-crate";

/// Where the list lives, under the mock directory.
const SUPPLY: &str = "tools/the-positions/src/supply.rs";

/// The array whose strings are the names filed as gone.
const GONE: &str = "pub const ARVO_GONE";

struct ANameFiledAsGoneIsExportedByNoCrate;

impl Lint for ANameFiledAsGoneIsExportedByNoCrate {
    fn name(&self) -> &'static str {
        NAME
    }

    fn default_severity(&self) -> Severity {
        Severity::HARD_ERROR
    }
}

impl RepoLint for ANameFiledAsGoneIsExportedByNoCrate {
    fn check_repo(&self, ctx: &RepoContext) -> Vec<LintError> {
        check(ctx.view(), &ctx.mock_dir)
    }
}

/// One finding for each name on the gone list that a crate's `lib.rs` exports.
fn check(view: &View, mock_dir: &Path) -> Vec<LintError> {
    let supply = mock_dir.join(SUPPLY);
    let Some(text) = crate::code_only_source::without_comments(view, &supply) else {
        return Vec::new();
    };
    let names = gone(&text);
    let libs = libs(view, mock_dir);
    let mut found = Vec::new();
    for (line, name) in &names {
        for (krate, source) in &libs {
            if exports(source, name) {
                found.push(finding(*line, name, krate));
            }
        }
    }
    found
}

/// The strings of the `ARVO_GONE` array, each with its one-based line.
fn gone(text: &str) -> Vec<(usize, String)> {
    let mut out = Vec::new();
    let mut inside = false;
    for (at, line) in text.lines().enumerate() {
        if !inside {
            inside = line.trim_start().starts_with(GONE);
            continue;
        }
        if line.trim_start().starts_with("];") {
            break;
        }
        let mut rest = line;
        while let Some(open) = rest.find('"') {
            let after = &rest[open + 1 ..];
            let Some(close) = after.find('"') else { break };
            out.push((at + 1, after[.. close].to_string()));
            rest = &after[close + 1 ..];
        }
    }
    out
}

/// Every crate's `lib.rs` under the mock directory, comments and literals
/// blanked, by crate.
fn libs(view: &View, mock_dir: &Path) -> Vec<(String, String)> {
    let crates = mock_dir.join("crates");
    let Some(entries) = view.list(&crates) else {
        return Vec::new();
    };
    let mut out = Vec::new();
    for entry in entries.entries() {
        let lib = crates.join(entry.name()).join("src/lib.rs");
        if let Some(source) = crate::code_only_source::code_only_at(view, &lib) {
            out.push((entry.name().to_string(), source));
        }
    }
    out
}

/// Whether a line of `source` opening with `pub` names `name` as a whole word.
fn exports(source: &str, name: &str) -> bool {
    source
        .lines()
        .map(str::trim_start)
        .filter(|line| line.starts_with("pub ") || line.starts_with("pub("))
        .any(|line| crate::code_only_source::contains_whole_word(line, name))
}

fn finding(line: usize, name: &str, krate: &str) -> LintError {
    let mut error = LintError::error(
        "mock".to_string(),
        line,
        NAME,
        format!(
            "`{name}` is filed in `ARVO_GONE` as a name arvo no longer has, and `{krate}` \
             exports it from its `lib.rs`, so the report counts a position carrying it as \
             pointing at nothing when it points at a shipped type. Move it to `ARVO_SHIPPED` \
             and change the test asserting it is gone."
        ),
    );
    error.finding_kind = Some("a-name-filed-as-gone-ships");
    error.path = Some(SUPPLY.to_string());
    error
}

// The generated lint crate loads this file itself through `#[path]`, at the
// name the lint is collected by, so the tests travel with it.
#[cfg(test)]
#[path = "a_name_filed_as_gone_is_exported_by_no_crate/tests.rs"]
mod tests;
