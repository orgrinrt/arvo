//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! Every position across the stack where a host primitive sits, by kind.
//!
//! `obligation::a_primitive_for_every_position_a_bare_number_would_take` demands
//! a primitive of arvo's own for every public API position across the stack that
//! would otherwise carry a bare integer, float, `bool` or `usize`, with the type
//! of a const generic parameter excepted. Its own `gap` says what is missing:
//!
//! > It is not satisfied by counting the primitives that exist: it is satisfied
//! > by the positions, and nothing has enumerated those.
//!
//! This enumerates them. It is the demand side made countable, and it is
//! deliberately not the answer to what arvo should supply: what a position wants
//! is decided by what it means, and that judgement is a person's.
//!
//! # Why a tool and not a lint
//!
//! There is no failing case here and there could not be. The stack's own
//! `no-bare-numeric` already gates the same material and blocks a commit on it;
//! a second gate would be that lint twice. What this answers is the question the
//! gate cannot: given that the rule refuses these, **what are they**, grouped so
//! that a reader can see which groups want one primitive between them.
//!
//! It takes a question because the trees it reads are other repositories, whose
//! checkout location this repository cannot know, and because a ref is not
//! optional here.
//!
//! # Every read names a ref, and that is the whole methodology
//!
//! The obligation's `gap` records that the clause bounding it was found in a
//! consumer design round which two prior passes had recorded as saying nothing,
//! and that one of those "reported that consumer at zero and called the figure
//! controlled. It was reading a single-branch clone."
//!
//! So nothing here reads a working tree. `corpus` goes to the object store with
//! a ref named on the command line, and the report prints the commit each ref
//! resolved to, because a count over a corpus is a claim about which corpus and
//! a ref moves.

pub mod allows;
pub mod corpus;
pub mod kinds;
pub mod report;
pub mod role;
pub mod supply;
pub mod walk;

use mockspace::tool::{ArgSpec, NotALint, Outcome, Tool, ToolContext, ToolReport};

use crate::corpus::TreeSpec;
use crate::kinds::Found;

pub struct ThePositions;

impl Tool for ThePositions {
    fn name(&self) -> &'static str {
        "the-positions"
    }

    fn description(&self) -> &'static str {
        "every position in a tree where a host primitive sits, by grammatical kind and semantic role"
    }

    fn not_a_lint(&self) -> NotALint {
        NotALint::TakesAQuestion
    }

    fn args(&self) -> &'static [ArgSpec] {
        &[
            ArgSpec {
                name: "tree",
                required: true,
                description: "a tree to read, as `<repo-path>@<ref>`; repeatable",
            },
            ArgSpec {
                name: "--kind",
                required: false,
                description: "list every position of one grammatical kind in full",
            },
            ArgSpec {
                name: "--role",
                required: false,
                description: "list every position of one semantic role in full",
            },
            ArgSpec {
                name: "--api-only",
                required: false,
                description: "tally only the positions the obligation is about",
            },
            ArgSpec {
                name: "--everything",
                required: false,
                description: "list every occurrence, not only the ones at a public API position",
            },
        ]
    }

    fn help(&self) -> &'static str {
        "Reads each named tree at the named ref, out of the object store rather \
         than off disk, and reports every occurrence of a host primitive with the \
         position it occupies.\n\n\
         A tree is written `<repo-path>@<ref>`, for instance `../notko@origin/dev`, \
         and the argument repeats. The ref is required and has no default: a \
         default would be a read of whatever branch is checked out, and the \
         obligation this serves records that exact reading producing a zero that \
         was then called controlled.\n\n\
         Two axes are reported. The grammatical kind comes off the parse and says \
         whether an outside caller or implementor has to write the type. The \
         semantic role is inferred from the identifier the position is attached \
         to and says what the position means, which is what decides which \
         primitive could serve it; it is a reading rather than a measurement and \
         the report says so.\n\n\
         `--kind <kind>` or `--role <role>` lists one group in full, with file \
         and line, which is what a reader needs before deciding what a group \
         wants. `--api-only` drops the interior and const-generic-parameter \
         positions, which is the obligation's own wording; the default keeps them, \
         because the shipped lint refuses interior occurrences too and the \
         difference between the rule as written and the rule as enforced is worth \
         being able to see."
    }

    fn run(&self, ctx: &ToolContext<'_>) -> ToolReport {
        let mut specs: Vec<TreeSpec> = Vec::new();
        let mut want_kind: Option<String> = None;
        let mut want_role: Option<String> = None;
        let mut api_only = false;
        let mut everything = false;

        let mut args = ctx.args.iter().copied();
        while let Some(arg) = args.next() {
            match arg {
                "--api-only" => api_only = true,
                "--everything" => everything = true,
                "--kind" => want_kind = args.next().map(std::string::ToString::to_string),
                "--role" => want_role = args.next().map(std::string::ToString::to_string),
                other if other.starts_with("--") => {
                    return ToolReport {
                        outcome: Outcome::Inconclusive {
                            reason: format!("unknown flag `{other}`"),
                        },
                        output: String::new(),
                    };
                }
                other => match TreeSpec::parse(other) {
                    Ok(spec) => specs.push(spec),
                    Err(why) => {
                        return ToolReport {
                            outcome: Outcome::Inconclusive { reason: why },
                            output: String::new(),
                        };
                    }
                },
            }
        }

        if specs.is_empty() {
            return ToolReport {
                outcome: Outcome::Inconclusive {
                    reason: "no tree named. Write `<repo-path>@<ref>`, repeatable.".to_string(),
                },
                output: String::new(),
            };
        }

        let mut found: Vec<Found> = Vec::new();
        let mut allows: Vec<allows::Allow> = Vec::new();
        let mut heads: Vec<(String, String, usize)> = Vec::new();
        let mut designs: Vec<(String, usize, usize)> = Vec::new();

        for spec in &specs {
            let head = match corpus::resolve(&spec.repo, &spec.git_ref) {
                Ok(h) => h,
                Err(why) => {
                    return ToolReport {
                        outcome: Outcome::Inconclusive { reason: why },
                        output: String::new(),
                    };
                }
            };
            let label = spec.label();

            let rust_paths = match corpus::paths(&spec.repo, &spec.git_ref, corpus::is_rust) {
                Ok(p) => p,
                Err(why) => {
                    return ToolReport {
                        outcome: Outcome::Inconclusive { reason: why },
                        output: String::new(),
                    };
                }
            };
            let blobs = match corpus::read_all(&spec.repo, &spec.git_ref, &rust_paths) {
                Ok(b) => b,
                Err(why) => {
                    return ToolReport {
                        outcome: Outcome::Inconclusive { reason: why },
                        output: String::new(),
                    };
                }
            };
            let read = blobs.len();
            for blob in &blobs {
                found.extend(walk::walk(&label, &blob.path, &blob.text));
                if corpus::is_shipped(&blob.path) {
                    allows.extend(allows::allows_in(&label, &blob.path, &blob.text));
                }
            }

            // The design side: a position that does not exist yet is written in
            // a template, and a template is where the whole of two consumers
            // lives. Counted separately, because a fenced signature is a
            // proposal rather than a shipped position.
            // Whether the tree writes its designs as templates has to be
            // established before deciding what a design file is, because a
            // repository with no mockspace writes a plain `DESIGN.md` and two
            // in this stack are in that state.
            let has_templates =
                corpus::paths(&spec.repo, &spec.git_ref, |p| p.ends_with(".md.tmpl"))
                    .map(|v| !v.is_empty())
                    .unwrap_or(false);
            let design_paths = match corpus::paths(&spec.repo, &spec.git_ref, |p| {
                corpus::is_design(p, has_templates)
            }) {
                Ok(p) => p,
                Err(why) => {
                    return ToolReport {
                        outcome: Outcome::Inconclusive { reason: why },
                        output: String::new(),
                    };
                }
            };
            let design_blobs =
                corpus::read_all(&spec.repo, &spec.git_ref, &design_paths).unwrap_or_default();
            let mut design_hits = 0usize;
            for blob in &design_blobs {
                let hits = report::design_positions(&label, &blob.path, &blob.text);
                design_hits += hits.len();
                found.extend(hits);
            }
            designs.push((label.clone(), design_blobs.len(), design_hits));
            let _ = has_templates;
            heads.push((label, head, read));
        }

        if found.is_empty() {
            return ToolReport {
                outcome: Outcome::Inconclusive {
                    reason: "no host primitive found in any named tree. Every tree in this \
                             stack contains at least one, in its const generic parameters if \
                             nowhere else, so an empty result is the instrument failing rather \
                             than a corpus that is clean."
                        .to_string(),
                },
                output: String::new(),
            };
        }

        let examined = found.len();
        let output = report::render(
            &found,
            &allows,
            &heads,
            &designs,
            api_only,
            everything,
            want_kind.as_deref(),
            want_role.as_deref(),
        );
        ToolReport {
            outcome: Outcome::Clean { examined },
            output,
        }
    }
}

#[cfg(test)]
mod tests;

mockspace::lint_pack! {
    tools: [ThePositions],
}
