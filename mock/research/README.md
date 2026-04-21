# arvo research

Source material backing the arvo designs. Research notes are NOT gated by the writing-style lint — files under this directory (and its subdirectories) are archival and preserved verbatim from their origins where applicable.

## `imported-from-polka-dots/`

Primary sources copied verbatim from `~/Dev/polka-dots/`. The polka-dots workspace is where arvo's numeric substrate was originally researched and ratified.

- `rust-nightly-features-for-type-constraints.md` — nightly-only features that arvo leans on (const-generic expressions, ConstParamTy, specialization, etc.). Grounds arvo's nightly-by-default stance.
- `design_rounds/202603241200_topic.arvo-strategy-markers.md` — origin of the Hot/Warm/Cold/Precise strategy markers. Rank ordering, cross-strategy resolution.
- `design_rounds/202603241300_topic.arvo-strategy-aliases-and-open-questions.md` — follow-up on strategy semantic aliases, open questions left for later rounds.
- `design_rounds/202603241400_topic.arvo-crate-dependency-revision.md` — L0/L1/L2 dependency direction decisions.
- `design_rounds/202603241500_changelist.doc.md` — doc CL that closed the strategy/alias/dependency trilogy.

## `imported-from-saalis/`

Primary sources copied verbatim from `~/Dev/saalis/`. Saalis ran a separate primitives-design round that overlaps arvo's substrate concerns.

- `design_rounds/202603170810/` — primitives-design round (topic + doc CL + src CL, all locked in the source workspace). Historical context for how arvo's primitive surface converged.

## Additional references not copied here

Broader arvo-adjacent material (the bit-level contracts discussion, the `arvo-types-only` lint design, the fixed-point-first identity text) remains in `~/Dev/polka-dots/` and is already synthesised into `mock/PRINCIPLES.md.tmpl` and the agent rules. Port on demand as specific questions arise.
