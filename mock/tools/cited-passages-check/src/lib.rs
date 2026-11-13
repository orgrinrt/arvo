//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! Whether the passages a reader quoted are in the lines it cited.
//!
//! A reader that classifies rows against a source cites where it read: a block
//! per row, a `path:lines` it says it read, and the passage it says it found
//! there. The passage is the one part of the block somebody can check without
//! rereading the source, and a block whose passage is not in its cited lines is
//! a reading nobody can trust. The first version of this was a perl script
//! (`mock/research/202611131500_claim-the-rows/method/verify.pl`); this is the
//! same check as a tool, so it is asked of any reader's output rather than copied.
//!
//! # The block shape it reads
//!
//! ```text
//! ## namespace::slug
//! class: <anything>
//! evidence: path/to/file.md:120-140
//! passage: "a quoted run" and prose, "another quoted run"
//! ```
//!
//! A block opens at a line starting `## `, and its slug is the first word after.
//! `evidence:` may repeat, one `path:a-b` or `path:a` each. `passage:` runs to the
//! next line opening with a `word:` key or to the end of the block.
//!
//! # What it says of each quoted run
//!
//! A run is a double-quoted span of the passage. With no quotes the whole passage
//! is one run. A run is folded (quote marks, markdown marks, line continuations
//! and whitespace flattened, case dropped) and split at an ellipsis, so
//! `"a ... b"` is two segments that each have to be found.
//!
//! A segment is then one of four things: in the cited lines, elsewhere in a cited
//! file (the reader found the words and cited the wrong place), nowhere in any
//! cited file, or too short to judge. Short is under [`MIN_JUDGEABLE`] characters
//! folded, because a run that short occurs by chance in any file of any size, so a
//! hit on it says nothing and the tool names it rather than counting it. A block
//! is `MISS` where any segment is nowhere, `FAR` where any is only elsewhere,
//! `OK` where all judgeable segments are in the lines, and `UNJUDGED` where none
//! could be judged.
//!
//! # Not a lint
//!
//! The outcome is a fact about one reader's output, which is not in the tree
//! being checked, and the repair is rereading rather than an edit. It reports.

use std::path::Path;

use mockspace::tool::{ArgSpec, NotALint, Purpose, Tool, ToolContext, ToolReport};

/// The folded length under which a segment cannot say anything.
pub const MIN_JUDGEABLE: usize = 20;

/// The cited lines are widened by this many on each side unless told otherwise.
pub const DEFAULT_MARGIN: usize = 0;

pub struct CitedPassagesCheck;

impl Tool for CitedPassagesCheck {
    fn name(&self) -> &'static str {
        "cited-passages-check"
    }

    fn description(&self) -> &'static str {
        "whether the passages a reader quoted are in the lines it cited"
    }

    fn purpose(&self) -> Purpose {
        Purpose::Check(NotALint::NoFailingCase)
    }

    fn args(&self) -> &'static [ArgSpec] {
        &[
            ArgSpec {
                name:        "output",
                required:    true,
                description: "the file holding the reader's blocks",
            },
            ArgSpec {
                name:        "root",
                required:    false,
                description: "where a relative `evidence:` path resolves from; the repository root",
            },
            ArgSpec {
                name:        "margin",
                required:    false,
                description: "lines either side of a citation that still count as in it; 0",
            },
        ]
    }

    fn help(&self) -> &'static str {
        "Reads a reader's blocks (`## ns::slug`, `evidence: path:lines`, \
         `passage: \"...\"`) and says for each quoted run whether it is in the \
         cited lines, elsewhere in the cited file, or nowhere, and names the runs \
         too short to judge.\n\n\
         One line per block, in the order read, then \
         the short runs, then the totals. Nothing here fails: a reading whose \
         passage is not where it says is a reading to redo, which is the reader's \
         work and not an edit."
    }

    fn run(&self, ctx: &ToolContext<'_>) -> ToolReport {
        let Some(output) = ctx.args.first() else {
            return ToolReport::inconclusive(
                "no reader output given. Pass the file holding the blocks as the first argument.",
            );
        };
        let text = match std::fs::read_to_string(output) {
            Ok(t) => t,
            Err(e) => {
                return ToolReport::inconclusive(format!("cannot read `{output}`: {e}"));
            },
        };
        let root = ctx.args.get(1).map_or(ctx.repo_root, |r| Path::new(*r));
        let margin = ctx
            .args
            .get(2)
            .and_then(|m| m.parse().ok())
            .unwrap_or(DEFAULT_MARGIN);
        let results = verify(
            &text,
            &|path| std::fs::read_to_string(root.join(path)).ok(),
            margin,
        );
        if results.is_empty() {
            return ToolReport::inconclusive(format!(
                "`{output}` holds no block opening with `## `, so there is nothing to check. \
                 That is a fact about the file's shape and not about the reader."
            ));
        }
        report(&results)
    }
}

/// What a block says about where it read, and what was found.
#[derive(Debug, PartialEq, Eq)]
pub struct BlockResult {
    pub slug:      String,
    pub class:     Option<String>,
    pub verdict:   Verdict,
    pub cited:     Vec<String>,
    pub in_lines:  usize,
    pub elsewhere: usize,
    pub nowhere:   usize,
    /// The folded text of each segment too short to judge.
    pub short:     Vec<String>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Verdict {
    Ok,
    /// Found, but not in the cited lines.
    Far,
    /// Not in any cited file.
    Miss,
    /// Nothing long enough to judge.
    Unjudged,
    /// No `evidence:` or no `passage:`.
    NoEvidence,
    /// An `evidence:` that is not `path:lines`.
    NoParse,
    /// A cited file that cannot be read.
    NoFile,
}

impl Verdict {
    fn label(self) -> &'static str {
        match self {
            Verdict::Ok => "OK",
            Verdict::Far => "FAR",
            Verdict::Miss => "MISS",
            Verdict::Unjudged => "UNJUDGED",
            Verdict::NoEvidence => "NOEVID",
            Verdict::NoParse => "NOPARSE",
            Verdict::NoFile => "NOFILE",
        }
    }
}

/// Every block of `output`, checked against the files `read` returns.
///
/// `read` maps a cited path to its text, or `None` where it cannot be read, so a
/// test supplies files without a disk.
pub fn verify(
    output: &str,
    read: &dyn Fn(&str) -> Option<String>,
    margin: usize,
) -> Vec<BlockResult> {
    split_blocks(output)
        .into_iter()
        .map(|block| check_block(&block, read, margin))
        .collect()
}

/// The text of each block, one per `## ` line.
fn split_blocks(output: &str) -> Vec<String> {
    let mut blocks: Vec<String> = Vec::new();
    for line in output.lines() {
        if line.starts_with("## ") {
            blocks.push(String::new());
        }
        if let Some(b) = blocks.last_mut() {
            b.push_str(line);
            b.push('\n');
        }
    }
    blocks
}

/// The value of the first `key:` line of a block, trimmed.
fn field(block: &str, key: &str) -> Option<String> {
    block.lines().find_map(|l| {
        l.strip_prefix(key)
            .and_then(|r| r.strip_prefix(':'))
            .map(|v| v.trim().to_string())
    })
}

/// The passage: from `passage:` to the next line opening with a `word:` key.
fn passage_of(block: &str) -> Option<String> {
    let mut lines = block.lines();
    let first = lines.find_map(|l| l.strip_prefix("passage:"))?;
    let mut out = first.to_string();
    for l in lines {
        if opens_a_key(l) {
            break;
        }
        out.push('\n');
        out.push_str(l);
    }
    Some(out)
}

/// Whether a line opens with `word:`, which is where the next field of a block
/// begins.
fn opens_a_key(line: &str) -> bool {
    let Some((key, _)) = line.split_once(':') else {
        return false;
    };
    !key.is_empty() && key.chars().all(|c| c.is_ascii_alphabetic() || c == '_')
}

/// A cited range: a path, and the 1-based first and last line.
fn parse_evidence(e: &str) -> Option<(String, usize, usize)> {
    let (path, rest) = e.split_once(':')?;
    let rest = rest.split_whitespace().next()?;
    let (a, z) = match rest.split_once('-') {
        Some((a, z)) => (a.parse().ok()?, z.parse().ok()?),
        None => {
            let a: usize = rest.parse().ok()?;
            (a, a)
        },
    };
    (!path.is_empty() && a >= 1 && z >= a).then(|| (path.to_string(), a, z))
}

/// Text with the differences a quotation survives folded away: typographic
/// quotes to plain, a TOML line continuation, a markdown quote marker at a line
/// start, emphasis and code marks, whitespace runs, and case.
pub fn fold(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let joined = s.replace("\\\n", "");
    for line in joined.lines() {
        let line = line.trim_start();
        let line = line
            .strip_prefix('>')
            .map_or(line, |l| l.strip_prefix(' ').unwrap_or(l));
        out.push_str(line);
        out.push(' ');
    }
    let plain: String = out
        .chars()
        .filter(|c| !matches!(c, '*' | '_' | '`'))
        .map(|c| {
            match c {
                '\u{2018}' | '\u{2019}' => '\'',
                '\u{201C}' | '\u{201D}' => '"',
                c => c,
            }
        })
        .collect();
    plain
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .to_lowercase()
}

/// The quoted runs of a passage, typographic quotes read as plain; the passage
/// itself where it holds none.
fn runs_of(passage: &str) -> Vec<String> {
    let plain: String = passage
        .chars()
        .map(|c| if matches!(c, '\u{201C}' | '\u{201D}') { '"' } else { c })
        .collect();
    let parts: Vec<&str> = plain.split('"').collect();
    // An even count of parts is an odd count of quotes: the last opened no run.
    let closed = if parts.len().is_multiple_of(2) { parts.len() - 1 } else { parts.len() };
    let runs: Vec<String> = parts[.. closed]
        .iter()
        .skip(1)
        .step_by(2)
        .map(|s| (*s).to_string())
        .filter(|s| !s.trim().is_empty())
        .collect();
    if runs.is_empty() { vec![plain.trim().to_string()] } else { runs }
}

/// A folded run split at each ellipsis into the segments that are each looked
/// for, so a quotation that elides its middle is not a miss for eliding it.
fn segments_of(run: &str) -> Vec<String> {
    fold(run)
        .replace('\u{2026}', "...")
        .split("...")
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect()
}

fn check_block(block: &str, read: &dyn Fn(&str) -> Option<String>, margin: usize) -> BlockResult {
    let slug = block
        .lines()
        .next()
        .and_then(|l| l.strip_prefix("## "))
        .and_then(|l| l.split_whitespace().next())
        .unwrap_or("")
        .to_string();
    let class = field(block, "class");
    let evidence: Vec<String> = block
        .lines()
        .filter_map(|l| l.strip_prefix("evidence:"))
        .map(|v| v.trim().to_string())
        .collect();
    let blank = |verdict| {
        BlockResult {
            slug: slug.clone(),
            class: class.clone(),
            verdict,
            cited: evidence.clone(),
            in_lines: 0,
            elsewhere: 0,
            nowhere: 0,
            short: Vec::new(),
        }
    };
    let Some(passage) = passage_of(block) else {
        return blank(Verdict::NoEvidence);
    };
    if evidence.is_empty() {
        return blank(Verdict::NoEvidence);
    }
    let mut cited = Vec::new();
    for e in &evidence {
        let Some((path, a, z)) = parse_evidence(e) else {
            return blank(Verdict::NoParse);
        };
        let Some(text) = read(&path) else {
            return blank(Verdict::NoFile);
        };
        let lines: Vec<&str> = text.lines().collect();
        let lo = a.saturating_sub(1 + margin);
        let hi = (z - 1 + margin).min(lines.len().saturating_sub(1));
        let window = if lines.is_empty() || lo > hi {
            String::new()
        } else {
            lines[lo ..= hi].join("\n")
        };
        cited.push((fold(&window), fold(&text)));
    }

    let (mut in_lines, mut elsewhere, mut nowhere) = (0, 0, 0);
    let mut short = Vec::new();
    for run in runs_of(&passage) {
        for segment in segments_of(&run) {
            if segment.chars().count() < MIN_JUDGEABLE {
                short.push(segment);
            } else if cited.iter().any(|(w, _)| w.contains(&segment)) {
                in_lines += 1;
            } else if cited.iter().any(|(_, whole)| whole.contains(&segment)) {
                elsewhere += 1;
            } else {
                nowhere += 1;
            }
        }
    }
    let verdict = if nowhere > 0 {
        Verdict::Miss
    } else if elsewhere > 0 {
        Verdict::Far
    } else if in_lines > 0 {
        Verdict::Ok
    } else {
        Verdict::Unjudged
    };
    BlockResult {
        slug,
        class,
        verdict,
        cited: evidence,
        in_lines,
        elsewhere,
        nowhere,
        short,
    }
}

fn report(results: &[BlockResult]) -> ToolReport {
    let mut s = String::new();
    for r in results {
        s.push_str(&format!(
            "{:<8} {} {}  {}  in={} far={} miss={} short={}\n",
            r.verdict.label(),
            r.class.as_deref().unwrap_or("-"),
            r.slug,
            r.cited.first().map_or("-", String::as_str),
            r.in_lines,
            r.elsewhere,
            r.nowhere,
            r.short.len()
        ));
    }
    let with_short: Vec<&BlockResult> = results.iter().filter(|r| !r.short.is_empty()).collect();
    if !with_short.is_empty() {
        s.push_str(&format!(
            "\nToo short to judge (under {MIN_JUDGEABLE} characters folded), so a hit on them \
             would say nothing:\n"
        ));
        for r in with_short {
            for q in &r.short {
                s.push_str(&format!("  {}: \"{q}\"\n", r.slug));
            }
        }
    }
    let count = |v: Verdict| results.iter().filter(|r| r.verdict == v).count();
    s.push_str(&format!(
        "\n{} block(s): ok={} far={} miss={} unjudged={} noevidence={} noparse={} nofile={}\n",
        results.len(),
        count(Verdict::Ok),
        count(Verdict::Far),
        count(Verdict::Miss),
        count(Verdict::Unjudged),
        count(Verdict::NoEvidence),
        count(Verdict::NoParse),
        count(Verdict::NoFile),
    ));
    ToolReport::reported(s, results.len())
}

#[cfg(test)]
mod tests;

mockspace::lint_pack! {
    tools: [CitedPassagesCheck],
}
