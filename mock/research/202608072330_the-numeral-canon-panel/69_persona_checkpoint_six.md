# 69. Checkpoint six: unit three, first half

Written by the coordinator after `65` through `68`, the first four files of the number-systems unit.
Nothing here is evidence and nothing here is a decision. It states what the four established, what they
disagree about, and the two questions that are op's rather than the panel's.

## Two questions for op, and the first one is about his own sentence

### Q-A. Which verb is "validate"

The acceptance criterion is op's: **"have the typestate derive the matching container and numeral
representations, then validate, and erase."**

The two blind cold derivations read "validate" differently and **neither noticed**. `65` reads it as
compile-time and per-type: the typestate checks a declaration once, at monomorphisation. `66` reads it as
a runtime per-datum predicate: each incoming value is checked against the representable set. `68` located
the divergence and points out that shared premises cannot explain it, because both files read the same
intents and the same rules. The word itself carries both.

This is not a naming quibble, and that is why it is here rather than in the register. The two readings
have **different machine cost**, measured on the pin and committed in `68_probes/`:

- Under the compile-time reading the typed interior operation is **symbol-aliased to the bare one**.
  The emitted assembly contains the line `_add_trusted = _add_bare` (`68_probes/p4_asm_grep.txt`). Not
  nearly free. The same symbol.
- Under the runtime reading the shape carries `tst` and `csel` residue that does not go away
  (`68_probes/p4_validate_residue.s`).

So which verb was meant decides whether the typestate costs nothing or costs instructions per datum.

`68` also notes that op's earlier Q1 enumeration covers only the compile-time acts, so **the runtime
ingest door is a fourth thing nobody has put to him**. It may be that both are wanted, at different
places, in which case the criterion names two steps rather than one and the canon should say so.

### Q-B. Are the long-standing constraints op's intents, or inherited assumptions

`67` reports, and the coordinator verified by grep, that **none of the following appears anywhere in
`INTENTS.md`**, which holds I1 through I12:

`#![no_std]`, no `alloc`, const sizes with no runtime growth, monomorphisation as the dispatch, no `dyn`,
no `TypeId`, no `std::any`, no platform dependency.

Every one is asserted only in agent-authored places: the workspace rules, arvo's generated agent
instructions, and the panel's own brief at `00_brief.md:155-156`, which lists them among what is fixed.

**The erasure half of the acceptance criterion is argued, across this whole unit, from the absence of
`dyn` and `TypeId`.** If that constraint is inherited rather than intended, the erasure argument rests on
unratified ground, and it is the argument most of this unit's structure hangs from.

The coordinator introduced the same error into arvo's generated agent instructions while fixing them for
a different instance of the same class, calling these constraints op's intents in a section header. That
is corrected, and the correction says plainly where they actually come from.

## What the four files established

**The concept's components are a dependent sequence, not a tuple** (`67`, p1). Compiles gate-free on the
pin. The dependency is enforced rather than asserted: attaching an encoding declared over one identity to
a term at another is refused with `E0271`. The completed term names only its last component and projects
the rest, and four `const` size assertions cover erasure.

**A law contract is decided at the pair (identity, adaptation)** and reads neither encoding nor
container, and is **undecided by the identity alone** (`67`, p1_neg_b: `E0277` at the same ambient domain,
representable set, encoding and container, with only the adaptation moved). This is the sharpest thing
the unit has produced about where strategy sits, and it fills the gap `68` section 7 names as carrying
zero probe instances anywhere in the panel.

**No crossing between systems preserves operations** (`67`, p2, exhaustive at 4 bits). Every crossing is
total and preserves values or patterns at 100%, and not one preserves operations at 100%. The file keeps
a prediction its own output refuted, with the corrected closed form beside it.

**`63`'s C2 separates shared parameters from per-element ones** (`67`, p3), which is not what it appears
to say. Applied mechanically, block floating point yields 8 distinct representable sets and fails; a
packed run at four strides yields 1 and passes; a self-contained float of the same arithmetic passes. So
the test is not "has a scale factor" against "does not".

**Tropical distributivity holds iff the reduction is monotone**, biconditional in 6 of 6 cells with both
sides observed true and false (`67`, p4). One cell of six serves both consumer classes, and it is exactly
nonnegative shortest path's shape.

**What the pipeline actually certifies, itemised** (`68`). The four verbs have four evidential statuses.
"Derive" is declared-and-checked rather than computed, in both probes. "Validate" is the ambiguity above.
"Erase" decomposes three ways and this unit touches one: `65` establishes layout erasure, which is close
to a language tautology, and **`66`'s erase arm establishes nothing**, its `erase` being
`raw.reverse_bits()` at `66_probes/derive_validate_erase_pipeline.rs:95`, verified by the coordinator at
the source.

**A mutant with a deliberately over-declared window compiles clean**, `EXIT=0`
(`68_probes/p3_mutant.stderr`). Validation constrains declarations only from below; the guarantee is
carried by round-trips through the maps rather than by the declaration.

**Stored bits are not self-describing** (`68`). One format hosts many systems, so interchange validity is
conventional and the system identification travels out of band. That completes `65`'s role model: compute,
storage and interchange differ in **who re-establishes the invariant**.

**The model-width ceiling, re-established inside this panel** rather than cited from the closed one: the
9-bit exhaustive const check is refused by `deny(long_running_const_eval)`.

## What they disagree about

**Q18, where the adaptation lives**, now has evidence rather than three assertions. `67`'s p1_neg_b shows
a law contract turning on the adaptation with everything else held fixed, which is a fact any of the three
positions must accommodate and which none of them predicted.

**Q19, the level hierarchies.** `68` argues both the three-level and five-level cuts **undercount the
bottom tier**, because neither survives Cold packing, where the container answer is a per-value and
per-aggregate pair rather than a level. Unresolved.

## Two miscitations found and verified

`66:520-523` attributes to `63` section 8 a sentence that is verbatim `65:511`. `grep -c -i "interchange"`
against `63` returns **0**. Its "two independent sources" is one source cited twice, so the three-role
model is ONE EXPERT adopted as corroborated. One-word fix, and the coordinator confirmed the grep.

`66:60-68` built a cross-check on arvo's generated crate table, which described the removed tree in the
present tense. `65` attacked it, `67` reached the same conclusion independently, `68` seconds it. **Three
readers.** The generated instructions were fixed at their source and the dead lint scopes are gone.

**Correction, from `70`, verified.** That sentence as first written overclaimed. Only `MAIN.md.tmpl` had
been rewritten; four further generated files still described the removed tree in the present tense
(`.claude/CLAUDE.md:137-140`, `.claude/rules/cookbook.md:127-128`, `.claude/rules/cargo.md:42-45`,
`.claude/rules/implementation.md:89`), and a fifth, `type-surface.md`, carried ten more. Forty-nine dead
references across six templates survived the first pass, because the coordinator fixed the file the
reports named rather than grepping for the class.

Now fixed: each affected rule carries a banner saying its crate names name a tree that no longer exists
and must not be read as architecture, and the two pure-architecture tables (the layer dependency table and
the intent-to-crate cookbook rows) are removed outright, since neither carried surviving discipline.

The workspace rule that predicts this exact failure was in the same directory throughout, at
`canon-design-code-chain.md:70`: "A lower tier that survives a change above it becomes a claim about
something that no longer exists. It still gets read, and it still gets defended, because it is concrete
and detailed and looks authoritative next to the abstract statement that replaced it." 

## Evidence discipline across the unit, stated because it is one-directional

`68` committed 16 artifacts, source and transcript for every instrument. `67` committed 5 instruments
with outputs, after a stall that nearly lost all of them uncommitted. `65` committed 4 with two
transcripts, one of which was generated from an uncommitted scratch path and has been regenerated against
committed source. `66` committed **zero** output transcripts for any of its four probes; all were
recovered by rerun by `68` and all counts reproduce, so nothing is void.

`66`'s Python probe also hardcodes its headline count as a print literal, and two of its four Rust tests
are tautologies under `the-test-gate.md`.
