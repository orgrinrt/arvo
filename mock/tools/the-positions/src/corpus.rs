//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! Reading a tree at a named ref, out of the object store rather than off disk.
//!
//! **The working tree is the wrong input and the obligation's own record says
//! so.** `obligation::a_primitive_for_every_position_a_bare_number_would_take`
//! carries, in its `gap`, that the clause bounding it was found in a consumer
//! design round that two prior passes had recorded as saying nothing, and that
//! one of those passes "reported that consumer at zero and called the figure
//! controlled. It was reading a single-branch clone." A checkout answers for one
//! branch and reports the rest as absent, with nothing in the number saying
//! which branch it was.
//!
//! So every read here names a ref. `git cat-file --batch` streams the contents
//! of many paths through one process, which is what makes a whole-repository
//! walk affordable at several thousand files; `git show` per path is the same
//! answer at one process each.
//!
//! Nothing here writes. `ls-tree` and `cat-file` do not touch a working tree,
//! an index or a ref, so this reads a repository somebody else is working in
//! without moving anything under them.

use std::io::{BufRead, BufReader, Read, Write};
use std::path::Path;
use std::process::{Command, Stdio};

/// One file, as it stands at a ref.
pub struct Blob {
    /// Path relative to the repository root, as the tree spells it.
    pub path: String,
    /// The bytes, decoded lossily. A source file that is not UTF-8 is not a
    /// source file we can parse, and lossy decoding keeps the offsets sane
    /// rather than dropping the file silently.
    pub text: String,
}

/// A tree named on the command line, as `<path>@<ref>`.
#[derive(Debug)]
pub struct TreeSpec {
    pub repo: String,
    pub git_ref: String,
}

impl TreeSpec {
    /// Split `<path>@<ref>`. The separator is the last `@`, so a path
    /// containing one still resolves.
    ///
    /// A spec with no `@` is refused rather than defaulted. A default ref is
    /// exactly the single-branch read this module exists to prevent, and it
    /// would be invisible in the output.
    pub fn parse(raw: &str) -> Result<Self, String> {
        let Some(at) = raw.rfind('@') else {
            return Err(format!(
                "`{raw}` names no ref. Write `<repo-path>@<ref>`, for instance \
                 `../notko@origin/dev`. A tree with no ref is a read of whatever \
                 branch happens to be checked out, which is the failure this \
                 tool is built not to have."
            ));
        };
        let (repo, git_ref) = raw.split_at(at);
        let git_ref = &git_ref[1..];
        if repo.is_empty() || git_ref.is_empty() {
            return Err(format!(
                "`{raw}` has an empty repository path or an empty ref"
            ));
        }
        Ok(Self {
            repo: repo.to_string(),
            git_ref: git_ref.to_string(),
        })
    }

    /// How the tree is named in a report.
    pub fn label(&self) -> String {
        let leaf = Path::new(&self.repo)
            .file_name()
            .map(|s| s.to_string_lossy().to_string())
            .unwrap_or_else(|| self.repo.clone());
        format!("{leaf}@{}", self.git_ref)
    }
}

/// The commit a ref resolves to, so a report says which tree it read.
///
/// A ref moves. A report naming `origin/dev` and nothing else cannot be checked
/// later, and a count over a corpus is a claim about which corpus.
pub fn resolve(repo: &str, git_ref: &str) -> Result<String, String> {
    let out = Command::new("git")
        .args(["-C", repo, "rev-parse", git_ref])
        .output()
        .map_err(|e| format!("could not run git in `{repo}`: {e}"))?;
    if !out.status.success() {
        return Err(format!(
            "`{git_ref}` does not resolve in `{repo}`: {}",
            String::from_utf8_lossy(&out.stderr).trim()
        ));
    }
    Ok(String::from_utf8_lossy(&out.stdout).trim().to_string())
}

/// Every path in the tree at `git_ref` whose name passes `keep`.
pub fn paths(
    repo: &str,
    git_ref: &str,
    keep: impl Fn(&str) -> bool,
) -> Result<Vec<String>, String> {
    let out = Command::new("git")
        .args(["-C", repo, "ls-tree", "-r", "-z", "--name-only", git_ref])
        .output()
        .map_err(|e| format!("could not run git in `{repo}`: {e}"))?;
    if !out.status.success() {
        return Err(format!(
            "could not list `{git_ref}` in `{repo}`: {}",
            String::from_utf8_lossy(&out.stderr).trim()
        ));
    }
    Ok(String::from_utf8_lossy(&out.stdout)
        .split('\0')
        .filter(|p| !p.is_empty())
        .filter(|p| keep(p))
        .map(std::string::ToString::to_string)
        .collect())
}

/// The contents of every named path at `git_ref`, in one `cat-file` process.
///
/// A path that does not resolve is skipped rather than failing the run: an
/// `ls-tree` listing cannot produce one, so a miss here means the ref moved
/// under us, which is worth surviving and is reported by the count coming back
/// short.
pub fn read_all(repo: &str, git_ref: &str, paths: &[String]) -> Result<Vec<Blob>, String> {
    if paths.is_empty() {
        return Ok(Vec::new());
    }
    let mut child = Command::new("git")
        .args(["-C", repo, "cat-file", "--batch"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| format!("could not run git in `{repo}`: {e}"))?;

    let mut stdin = child.stdin.take().expect("stdin was piped");
    let query: String = paths.iter().map(|p| format!("{git_ref}:{p}\n")).collect();
    let writer = std::thread::spawn(move || {
        let _ = stdin.write_all(query.as_bytes());
        let _ = stdin.flush();
    });

    let stdout = child.stdout.take().expect("stdout was piped");
    let mut reader = BufReader::new(stdout);
    let mut out = Vec::with_capacity(paths.len());
    let mut idx = 0usize;

    loop {
        let mut header = String::new();
        if reader
            .read_line(&mut header)
            .map_err(|e| format!("reading `cat-file` output: {e}"))?
            == 0
        {
            break;
        }
        let header = header.trim_end();
        if header.is_empty() {
            continue;
        }
        // `<oid> <type> <size>` on a hit, `<name> missing` on a miss.
        let fields: Vec<&str> = header.split(' ').collect();
        if fields.len() < 3 || fields[1] != "blob" {
            idx += 1;
            continue;
        }
        let size: usize = fields[2]
            .parse()
            .map_err(|_| format!("`cat-file` gave an unreadable size in `{header}`"))?;
        let mut buf = vec![0u8; size];
        reader
            .read_exact(&mut buf)
            .map_err(|e| format!("reading a blob body: {e}"))?;
        let mut nl = [0u8; 1];
        let _ = reader.read_exact(&mut nl);
        if let Some(path) = paths.get(idx) {
            out.push(Blob {
                path: path.clone(),
                text: String::from_utf8_lossy(&buf).into_owned(),
            });
        }
        idx += 1;
    }

    let _ = writer.join();
    let _ = child.wait();
    Ok(out)
}

/// Whether a path is Rust source this walk should read.
///
/// `target/` is excluded because a build directory holds generated and vendored
/// source that nobody writes a signature in. Everything else is read, and what
/// it counts *as* is decided by [`is_shipped`] rather than here, so a reader can
/// see the split instead of inheriting somebody's exclusion list.
#[must_use]
pub fn is_rust(path: &str) -> bool {
    path.ends_with(".rs") && !path.split('/').any(|c| c == "target")
}

/// Whether a path is inside a crate's compiled library surface.
///
/// **This is the corpus question and it decides the whole answer.** The first
/// run of this tool put 38,252 of 39,529 public API positions in arvo, which
/// has three crates and 36 source files in them: 1,956 of its 2,254 Rust files
/// are panel probes, 114 are bench variants and 81 are research sketches. Every
/// one of those is a spike by the workspace's own definition, none is anybody's
/// API, and counting them answered a question nobody asked.
///
/// The rule is cargo's own and needs no list: a crate's library is what lives
/// under its `src/`. A `tests/`, an `examples/`, a `benches/` and a probe
/// directory are all compiled separately or not at all, and a `pub fn` in one is
/// public to nobody.
///
/// Kept and reported rather than dropped, because the difference between the
/// two populations is itself a finding: the demand the shipped lint makes is
/// over every line in the repository and the obligation's wording is over the
/// API, and the gap between those two numbers is what that difference costs.
#[must_use]
pub fn is_shipped(path: &str) -> bool {
    let parts: Vec<&str> = path.split('/').collect();
    let Some(src_at) = parts.iter().position(|c| *c == "src") else {
        return false;
    };
    // `src` must be a crate's own, so the component before it is a package
    // directory rather than another `src`. And nothing under a research,
    // sketch, probe or bench tree is shipped whatever it calls its directories.
    if parts[..src_at].iter().any(|c| {
        matches!(
            *c,
            "research"
                | "sketches"
                | "benches"
                | "probes"
                | "target"
                | "examples"
                // A fixture crate under `tests/` has a `src/` of its own, so a
                // rule that only looked for `src` counted `notko-macros`'s
                // trybuild fixture as shipped surface. It is a fixture.
                | "tests"
                | "fixtures"
                | "ui"
                // A tool or a lint is a check on the repository rather than a
                // crate anybody depends on, and neither is published.
                | "tools"
                | "lints"
        ) || c.ends_with("_probes")
    }) {
        return false;
    }
    true
}

/// Whether a path is a design document, which is where a position that does not
/// exist yet is written down.
///
/// A mockspace repository writes its designs as `*.md.tmpl` and generates the
/// `.md` beside them, so taking both counts every position twice. A repository
/// with no mockspace writes a plain `DESIGN.md` and has no template at all, and
/// **two repositories in this stack are in that state**, so a rule of "templates
/// only" reports them at zero and calls it clean.
///
/// `has_templates` is whether the tree contains any `.md.tmpl` at all, which is
/// the cheapest thing that distinguishes the two cases.
#[must_use]
pub fn is_design(path: &str, has_templates: bool) -> bool {
    if has_templates {
        return path.ends_with(".md.tmpl");
    }
    if !path.ends_with(".md") {
        return false;
    }
    // In a repository with no templates, the design is a hand-written document.
    // Prose that is plainly not one is skipped by name rather than by guessing.
    let leaf = path.rsplit('/').next().unwrap_or(path);
    !matches!(
        leaf,
        "README.md" | "CHANGELOG.md" | "CONTRIBUTING.md" | "LICENSE.md"
    )
}
