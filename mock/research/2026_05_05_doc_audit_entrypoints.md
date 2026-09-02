# Arvo entrypoint documentation audit, 2026-05-05

**Date:** 2026-05-05.
**Scope:** `README.md` (repo root, Tier 1), `mock/DESIGN.md.tmpl` and `mock/PRINCIPLES.md.tmpl` and `mock/WORKFLOW.md.tmpl` (Tier 3, render to `docs/DESIGN.md` / `docs/PRINCIPLES.md`), and the rendered `docs/*` Tier 1 output. Companion to the per-crate audit at `mock/research/2026_05_05_doc_audit.md` (sub-agent dispatched separately).
**Method:** Manual read pass against documentation-writing + writing-style + vocabulary + ai-agent-framing workspace rules.

## Summary

Repo-root `README.md` is in good shape after the 2026-05-04 readme-format pass: stand-alone identity, no Tier 1 leakage, no banned vocabulary, the writing-style rules respected. The `mock/` root tmpls are clean of em-dashes (PR #56 swept them) and of explicit posturing, but they carry pervasive **Tier 3 leakage**: round-number references (round 202604271346, 202604280034, 202604300826, 202605021200, 202604301700, 202605031748), `mockspace` and `cargo mock` mentions in `mock/WORKFLOW.md.tmpl` and `mock/DESIGN.md.tmpl`, and explicit `mock/design_rounds/` cross-references. These render verbatim into `docs/DESIGN.md` and `docs/PRINCIPLES.md` (the public Tier 1 output) and constitute the largest cleanup surface for arvo. The rendered `docs/*OVERVIEW.md` (per-crate) carry the same pattern at scale: every file has the auto-gen header naming `mockspace (mock)` and `cargo mock`, plus polka-dots line-number cross-references in arvo-bitmask, arvo-comb, arvo-graph, arvo-sparse OVERVIEW. The auto-gen header is a workspace-wide finding (the mockspace tool generates it; arvo cannot fix it locally without a mockspace change). The polka-dots references are local cleanup work.

## Per-file findings

### README.md (repo root)

Clean. The single-line tagline reads as identity, the "What it is" prose stands alone, no leakage, no banned vocabulary. No findings.

### mock/DESIGN.md.tmpl

- **Round-number Tier 3 leakage, line 21.** `5. **Const-trait-driven.** Per round 202604271346, every trait whose methods are sensibly callable in const context is pub const trait`. The round reference renders into `docs/DESIGN.md` for strangers. Replace with the design rationale itself ("Every trait whose methods are sensibly callable in const context is `pub const trait`. ...") without the round id.
- **Round-number Tier 3 leakage, line 28.** `Per round 202604271346, arvo splits into a layered set of small crates with a downstream facade.` Same fix: drop the round reference.
- **Round-number Tier 3 leakage, line 42.** `arvo-refit | Re-export gateway... Renamed from arvo-narrow in round 202604301700.` The rename history belongs in a Tier 2 history doc, not in the public crate-topology table.
- **Round-number Tier 3 leakage, lines 55-56.** `arvo-narrow-contracts during round 202604280034 and gained Widen<T> during round 202604300826`. Same fix.
- **Mockspace leakage, line 115-116.** `on the mockspace self-bench. The bench bin orchestrates variant cdylibs through the mockspace-bench-harness orchestrator + worker`. The bench discipline is a Tier 2 contributor concern; surfacing `mockspace-bench-harness` to a Tier 1 reader is leakage.
- **Mockspace leakage, line 125.** `Bench discipline lives in mockspace's bench docs; arvo's role here`. Move to Tier 2 (move the paragraph to `mock/research/`, or strip from the rendered output).
- **Mockspace leakage, line 131.** `Per-crate DESIGN under crates/*/DESIGN.md.tmpl.` Names a `.tmpl` extension; pure Tier 2 vocabulary in Tier 3 prose.
- **Mockspace leakage, lines 132-136.** `design_rounds/` is named explicitly, with reference to `mockspace state machine`. All Tier 2.

### mock/PRINCIPLES.md.tmpl

- **Round-number Tier 3 leakage, line 3.** `## Topology (round 202604271346)` heading. The heading itself contains the round id.
- **Round-number Tier 3 leakage, line 10.** `arvo-narrow-contracts during round 202604280034 and gained Widen<T> during round 202604300826`. Same as DESIGN.md.tmpl.
- **Round-number Tier 3 leakage, line 11.** `during round 202604300826), arvo-mask-contracts, arvo-numeric-` paragraph wrap.
- **Round-number Tier 3 leakage, line 17.** `was renamed from arvo-narrow in round 202604301700`. Rename history; belongs Tier 2.
- **Round-number Tier 3 leakage, line 34.** `Per round 202604271346 D-6, every trait whose methods are sensibly callable in const context is pub const trait`.
- **Round-number Tier 3 leakage, line 82.** `### No .0 field-access on arvo primitives (round 202605021200)` heading.
- **Round-number Tier 3 leakage, line 85.** `As of round 202605021200, every arvo numeric primitive`.

### mock/WORKFLOW.md.tmpl

- **Mockspace leakage, line 18.** `**Current phase: design.** The mockspace validates design via cargo`. The first sentence after the heading names mockspace and `cargo`. Tier 3 leakage.
- **Mockspace leakage, line 20.** `implementations yet. The goal is every DESIGN.md.tmpl agrees with`. `.tmpl` named in Tier 3 prose.
- **Mockspace leakage, line 25.** `3. Regenerate. cargo mock to produce generated docs.` `cargo mock` in Tier 3.
- Note: the WORKFLOW.md.tmpl is the most Tier-2-shaped of the three mock-root tmpls; consider whether it should render to a Tier 1 surface at all. If it must render publicly, rewrite it to focus on what consumers can do with the crate and move the contributor workflow to a Tier 2 file. If it does not render publicly (verify what `cargo mock` produces from this template), no fix needed.

### docs/DESIGN.md (rendered Tier 1)

- **Auto-gen header em-dash + leakage, line 2-7.** `AUTO-GENERATED — DO NOT EDIT DIRECTLY ... Generated by: mockspace (mock) ... To regenerate: cargo mock`. The em-dash on line 2 is a writing-style violation; the body is Tier 1 leakage. **This is not arvo-local fixable**: the auto-gen header is generated by the mockspace renderer itself. Recorded as a workspace-wide finding to surface to mockspace upstream. Local mitigation is to suppress or override the header during render if the renderer supports it; check mockspace docs.
- **Inherited round-number leakage from mock/PRINCIPLES.md.tmpl content** (verifiable on a `cargo mock` regeneration after the .tmpl fixes above land). Same items 1-6 listed under PRINCIPLES.md.tmpl above will disappear when those tmpl fixes land + `cargo mock` runs.

### docs/PRINCIPLES.md (rendered Tier 1)

- **Stale rendered output.** Has em-dashes at lines 2, 69, 164, 165, 181, 216, 302 even though `mock/PRINCIPLES.md.tmpl` was swept clean by PR #56. The fix is `cargo mock` regeneration after the round-number cleanup lands.

### docs/ARVO_*_OVERVIEW.md (rendered Tier 1, per-crate)

- **Auto-gen header on every file.** Same as docs/DESIGN.md, lines 2-7. Workspace-wide finding.
- **Polka-dots cross-references, multiple files.** `docs/ARVO_BITMASK_OVERVIEW.md:303-305`, `docs/ARVO_COMB_OVERVIEW.md:112,162-165`, `docs/ARVO_GRAPH_OVERVIEW.md:30,224-227`, `docs/ARVO_SPARSE_OVERVIEW.md:24,113,135,176-179`. These render from `mock/crates/*/DESIGN.md.tmpl` and `mock/crates/*/BACKLOG.md.tmpl`. The references to "polka-dots T3 decision #67" / "T5 L1022-1041" are Tier 1 leakage of a maintainer-private repo not visible to strangers. The per-crate audit (sub-agent task) covers these in detail; flagged here to surface the cross-cutting pattern.

## Cross-cutting patterns

1. **Round-number references everywhere.** The pattern `round YYYYMMDDHHMM` appears 7+ times across `mock/DESIGN.md.tmpl` and `mock/PRINCIPLES.md.tmpl`. The reader on crates.io cannot resolve these. The cleanup is uniform: replace each with the design rationale itself, or drop entirely.
2. **Mockspace + cargo mock + .tmpl + design_rounds vocabulary in Tier 3 templates.** Pervasive in `mock/WORKFLOW.md.tmpl` and the deployment / bench sections of `mock/DESIGN.md.tmpl`. These render to Tier 1.
3. **Auto-gen header is a workspace-wide finding.** The `Generated by: mockspace (mock) ... To regenerate: cargo mock` header is produced by the mockspace renderer. Suppressing or rewording it requires a mockspace change. Either surface to mockspace upstream as an issue, or document a per-repo override if one exists.
4. **Polka-dots historical cross-references in rendered output.** Per-crate concern, scoped to the sub-agent's findings. The cross-cutting fix: workspace-wide ban on Tier 1 references to polka-dots / saalis / loimu / stellar-heritage / clause-dev (already in `documentation-writing.md`); cleanup on each occurrence.

## Suggested topic-file scope

The arvo entrypoint leg of the workspace doc round wants to address:

1. **Strip round-number references from `mock/DESIGN.md.tmpl` and `mock/PRINCIPLES.md.tmpl`.** Replace each with the design rationale or drop. Affects ~7 sites in DESIGN.md.tmpl, ~7 in PRINCIPLES.md.tmpl.
2. **Strip mockspace / cargo mock / .tmpl / design_rounds vocabulary from `mock/WORKFLOW.md.tmpl` and the bench / deployment sections of `mock/DESIGN.md.tmpl`.** Move contributor-only content to a Tier 2 location (`mock/research/contributors.md` or similar) if it must be preserved.
3. **Decide WORKFLOW.md.tmpl's rendering fate.** Verify what `cargo mock` produces from it. If it renders to `docs/WORKFLOW.md` (Tier 1), rewrite the body to focus on consumer-facing workflow (how to use the crate, version compatibility). If it renders only to `mock/WORKFLOW.md` (Tier 2), no rewrite needed.
4. **Per-crate audit (sub-agent dispatched).** Findings will land at `mock/research/2026_05_05_doc_audit.md` separately; merge with this file at synthesis time.
5. **Auto-gen header workspace-wide finding.** Surface to mockspace upstream as a separate issue (not arvo-local). Note in topic file but defer the fix.

The arvo doc round wants one topic file with one doc CL covering items 1-3. The src CL is unnecessary for arvo unless the per-crate audit surfaces rustdoc-on-pub-items drift; that goes into a separate src CL the topic file references but does not write yet.
