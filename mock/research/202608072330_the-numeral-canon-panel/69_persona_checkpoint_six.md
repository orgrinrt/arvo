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

**~~No crossing between systems preserves operations~~** (`67`, p2, exhaustive at 4 bits).
**RETRACTED by `71`, verified at the source by the coordinator.** `67:252` states the universal "No
crossing preserves operations at 100%" while measuring only three of the five coordinates its own section
2 names. `71`'s p1 measures all five: it reproduces every one of `67`'s eight numbers exactly (192, 111,
136, 80, 192, 101, 108, 1) and finds the two unmeasured indices, re-encoding and re-housing, preserve the
value-level operation at **256/256** (`71_probes/p1_output.txt:66-68`).

`67`'s own K4 survives; the universal at `67:252` and the register line at `67:566` do not. The repair is
two words. This checkpoint repeated the universal without checking its quantifier, which is the same
failure `67` section 1 catalogues three prior instances of, and this is the fourth.

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
and must not be read as architecture.

**Second correction, from `73`, verified. This is the third overclaim about the same repair, and the
second sat inside the correction of the first.** The sentence here previously said the two
pure-architecture tables were "removed outright". Only the layer dependency table was. The
intent-to-crate cookbook rows were still present at `.claude/rules/cookbook.md:136-137` and
`mock/agent/rules/cookbook.md.tmpl:119` when `73` checked.

Now actually done, with the count rather than an adjective, which is what the previous two claims
lacked: **five dead table rows dropped** from the cookbook template, and a grep across all five affected
generated files returns `CLAUDE.md` 0, `cargo.md` 0, `cookbook.md` 5, `implementation.md` 15,
`type-surface.md` 10.

**Thirty references therefore remain, deliberately.** They sit inside prose carrying discipline that
survives the redesign (trait-first thinking, exact-width thinking, the ban on bare primitives at API
positions), and deleting them would remove the rule along with the dead names. The banner is the
mitigation for those; removal was only ever right for the two tables that carried nothing else.

The workspace rule written after the first overclaim says: fix the class, then run the grep that would
have found every instance, and put the count in the fix. The second overclaim was written without running
it. The count above is that grep.

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


## Update after `71` and `72`: the fork put to op is smaller than it first read

`71` found that the endpoints do not determine a crossing: exactly two functions keyed on ordering, both
well typed, so only a canon sentence can break the tie. Stated that way it sounds like op owes an ordering
rule for every multi-coordinate crossing.

`72` measured the boundary and it is much narrower. Reproducing `71`'s control exactly (2 distinct
functions, 30/256 agreement), it then finds **widening gives 1 distinct function at 16/16**, and
**narrowing restricted to in-range values gives 1 function at 16/16**. Every divergent value is lossy
(`72_probes/p2_order_dependence_is_confined_to_loss.out:6-22`, verified by the coordinator).

**So the obligation is "name an order for every lossy crossing", not for every multi-coordinate one.** The
widening composites the panel actually relies on, accumulator entry, promotion, and `60`'s window, need no
sentence from op at all.

`72` also strengthened `71`'s own result rather than only conceding to it. `71`'s two refuting rows are
**constructional, not empirical**: the operation reads neither the encoding nor the housing, so at those
indices it compares a computation with itself. `72` proved it by mutation, giving the target an encoding
that maps all sixteen values onto pattern zero and still reading 256/256
(`72_probes/p1_the_refuting_rows_are_constructional.out:6-8`). That licenses the word **never** in `71`'s
X2, which a 4-bit sweep alone could not, and retires a number from circulation.

`72` accepts the two-word repair to `67:252` and proposes a stronger true universal in its place: **a
crossing preserves the value-level operation exactly when it moves no coordinate that operation reads,
which is at the encoding and at the container and nowhere earlier.**

It locates one place `71` over-reads, with citations: `71:415-417` says the three roles differ at indices
4 and 5 "and nowhere else", against `OPTIONS.md:991-994` carrying an undetermined index-2 difference and
`65:188-192` filing Precise's compute role as already carrying extent. The criterion survives; it is three
questions rather than one.

And `72` refuted its own biconditional with its own probe, keeping it with a closed form for the fourteen
exceptions.
