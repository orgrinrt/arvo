# 46. The carrier collision, attacked

**Date:** 2026-08-09. **Persona:** Stephen Dolan. **Mode:** explore, do not settle (`00_brief.md`, `04`,
`28`). **Position in the unit:** third file on one topic (`44`, `45`, this file), per the cadence
correction in `RULES.md`. My job is to attack or support `45` with my own derivation, not to summarise
it.

**Status: COMPLETE.**

## 0. Gates

### 0.1 Canon gate

No ratified canon exists to defend or diverge from. The fixed material is `01`, `04`, `28`, `INTENTS.md`,
the workspace discipline, and the acceptance criterion. `45`'s question, whether the container
derivation's output count is forced by arithmetic or is an artifact of Rust's type system, is squarely
inside that fixed material: the acceptance criterion is the ratified text `45` and `44` both trace, and
whether it names one output or two is the exact question. Nothing below proposes anything the
forbidden-feature list excludes. **Gate: passes.**

### 0.2 Test gate

There is no suite. `mock/crates` is empty. Evidence is probes, opened and, where cheap, re-derived by
hand rather than trusted from prose.

## 1. Reading order, stated so the rung claims below are checkable

Per the brief: `INTENTS.md`, `00_brief.md`, `RULES.md`, then `45` in full and its probes directly, then
`44` in full, then `15` and `16` directly, then `OPTIONS.md`'s "derivation's outputs" section, opened at
the current line numbers rather than trusted from any account of it.

This order does **not** meet `RULES.md`'s strict bar for an independent derivation (derive before
reading the predecessor). I read `45`'s claims first, then went to `15` and `16` to check them against
the primary text myself, rather than deriving cold. Where I confirm a claim already carried at TWO
EXPERTS (the two-output count itself, established independently by `15` and `16` before either read
`45`), my confirmation is a third read, not a third independent instance, and I say so rather than
claiming a rung I have not earned. Where I find something `45` gets wrong, that is an attack, and an
attack does not need independence to be worth having: it needs to be correct, and it carries its own
citation.

One thing worth recording before the content: `OPTIONS.md`'s "derivation's outputs" section (lines
703-778, opened directly) and `00_brief.md`'s acceptance-criterion line (line 145) have **both already
been edited** to carry `44`'s and `45`'s corrections, between `45`'s file being written (00:26) and my
reading it. `00_brief.md:145` now reads "the matching container and numeral **representations**" in the
plural, where `45` itself grepped it minutes earlier and found it still singular (`45:386-392`). This
means the register is already running ahead of the two-expert convergence this dispatch exists to
supply; worth naming so a later reader does not mistake "the register already says this" for "this was
checked."

## 2. My own derivation of the core claim, checked against `15` and `16` directly

**The question, restated as I would state it, independent of `45`'s formalization but landing on the
same shape.** A derivation `Derive : (Strategy, Width, Sign) -> ?` has to hand a downstream site enough
to (a) pick the machine type an operation lowers to, and (b) lay out an aggregate of the value. Is one
output (call it carrier) enough for both, or does the second requirement force a second output that is
not a function of the first?

**The Cold argument, re-derived from `16` directly rather than from any digest of it.** `16` section 2
(`16:126-141`, opened and read by me, not `45`'s account of it) restricts the map to unsigned `Cold`,
widths nine through sixteen. Every one of those eight declarations shares the same sixteen-bit native
container, because sixteen bits is the smallest native rung that holds thirteen bits and also the
smallest that holds sixteen. A derivation whose codomain is the container alone cannot distinguish
`UFixed<9,0,Cold>` from `UFixed<16,0,Cold>` the moment it returns, and that distinction is exactly what
`Cold`'s own stated intent ("aggressively minimises and bitpacks," `I6`) exists to preserve. This
argument uses nothing beyond: `Cold` packs tightly (ratified at the two-name level, `I2`/`I6`), and Rust
has no native integer type of an arbitrary bit width. Neither premise depends on `Precise`, on the
shipped ladder, or on any dead-tree specific. **I confirm this independently: it is sound, and it does
not need anything more than what I just used.**

This is the same argument `44` and `OPTIONS.md`'s own header rung marker already carry at TWO EXPERTS
(`OPTIONS.md:710-713`, `44:380-383`). My reading adds a third confirmation of the same argument, not a
new rung; I flag that honestly per section 1.

**So on the headline claim, "two, not one, independent of `Precise`," I land where `45` lands, and my own
route to it is the Cold argument alone. `45` is right that this half of `OPTIONS.md`'s framing
("blocked on the `Precise` strategy's undecided semantics," the pre-correction text `45` quotes at
`45:73-76`) was wrong: the Cold argument alone already closes the "is one output enough" question, and
it was sitting in the panel, unattributed to anything about `Precise`, before `45`'s dispatch opened.**

## 3. Where I attack: the wide-rung collision borrows its evidence from ground this panel has not settled

This is the substantive disagreement, and it is about `45` section 2.2, not section 2.1.

### 3.1 What `45` actually built, checked by opening the probe myself

`45_probes/p1_wide_rung_collision.rs` (opened directly, not from `45`'s prose) defines two functions,
`warm_bytes` and `hot_bytes`, and asserts, at `W = 256`, that `WideBits<32>` (align 1) and
`AlignedWideBits16<32>` (align 16) have equal size and unequal alignment. The assertions are real, the
file compiles, and the output matches what `45` quotes (`45_probes/p1_wide_rung_collision.out`, read
directly: "40 of 640", "first witness: W = 249", the `W = 256` const-checked block, the `W = 240`
negative control). I have no complaint about the mechanics. `45_probes/p2` reproduces `16`'s own
bit-count carrier representation over the same domain and gets zero, which is also a real, checkable
result and matches `45`'s claim that `16`'s own instrument is blind to the collision `p1` finds.

**The complaint is about what the probe's inputs are entitled to assume.** `p1`'s header states the
model is "REAL carrier types (not bit counts) matching arvo's own documented shape: `WideBits<BYTES>` at
align 1 (Warm/Cold/Precise, N > 128) and `AlignedWideBits16<BYTES>` at align 16 (Hot, N > 128), per the
crate architecture notes" (`45_probes/p1_wide_rung_collision.rs:14-16`). `45`'s prose repeats this:
"arvo's own documented architecture (quoted in the system material every member of this panel receives)
puts `Hot` on `AlignedWideBits16<BYTES>` (align 16) while `Warm`/`Cold`/`Precise` stay on
`WideBits<BYTES>` (align 1)" (`45:101-103`), and section 5.3 calls this "a documented architectural
decision, unrelated to anything under dispute in this panel" (`45:351-352`).

**That characterization is wrong, and the source that refutes it is `15`, the very file `p1` cites for
its numbers.** `15` section 5 (`15:528-556`, opened and read directly, not through `45`'s account):

> One thing about it is a design question rather than a measurement, and it is small: `Hot`'s alignment
> choice is currently a property of the strategy applied at the wide rung only. Whether alignment is a
> fifth axis, or a consequence of the strategy as it is modelled here, is not decided by anything I
> built. I am naming it and not resolving it.

And `15` section 3.7, describing its own probe's assumptions (`15:418-429`), states plainly that the
Hot-pads-to-align-16 rule is "stated in its header as an assumption" and that "the assignment is
replaceable without touching a line of the mechanism, which is the property that makes it safe to leave
open." `15` even flags that the crossover point itself, the width at which Hot switches from a native
rung to the wide byte-buffer arm, is contested: "`SETTLED.md:85` says Warm's crossover at 65 bits is
wrong with a precise reason... Neither is discharged here and neither is contradicted."

**So the file `45` cites for "arvo's own documented shape" is the same file that calls that shape a
replaceable assumption, not a settled architectural fact.** The "system material every member of this
panel receives" that `45` also cites (the arvo repository's generated agent instructions, describing
`WideBits`/`AlignedWideBits16` as the pre-panel crate layout) is a restatement of the pre-panel design,
which is exactly the tree `00_brief.md` names as being nuked and instructs not to cite as evidence about
what is correct (`00_brief.md:165-169`). It is a different file from `mock/crates` itself, but its
content is the same architecture, generated from the same round, and it predates this panel's own
question about whether that architecture is right.

### 3.2 What survives the attack, and what does not

The collision itself, as a **mechanism**, is real and general, and I want to be precise about how much
of `45`'s finding I am actually contesting.

**What survives.** Any design where two strategies pick different alignment policies at some rung will
have this shape: at widths where the byte counts happen to coincide, the pair `(declared width, stride)`
is identical between the two strategies while the carrier types differ, because alignment is a property
of the type and not of the byte count. This is a real, checkable, general fact about types with
different alignment requirements and identical size, and it does not need `Precise` to produce it. I
re-derived this myself, independent of `45`'s specific numbers: pick any two alignments `a1 != a2` and
any byte count `n` that is a multiple of both, and the same shape appears. Nothing about this needs
sixteen specifically, or Hot specifically, or arvo's current crossover rule specifically.

**What does not survive as stated.** The claim that this is **already forced**, that it is "unrelated to
anything under dispute," and the specific count "40 of 640" as a fact about the design rather than a fact
about one unratified model of it. `15`, the source `45` leans on, explicitly holds this open. Whether
`Hot` diverges from `Warm` in alignment at the wide rung at all, and at what width the divergence starts,
are both live questions in this same panel. A design that settled Hot and Warm on the same alignment
policy at the wide rung (a real, undecided possibility, not a strawman) would make `45`'s specific
witness at `W = 256` vanish along with the 40-of-640 count, while leaving the Cold argument in section
2.1 completely untouched, because Cold's forcing needs no cross-strategy alignment divergence at all.

**The honest restatement.** Section 2's title, "two outputs are already forced, before `Precise` is
settled at all," is correct on the strength of section 2.1 alone. Section 2.2 is not a second,
independent, already-settled forcing at the same rung as 2.1. It is a demonstration that **if** the
canon settles on any two strategies diverging in alignment policy at any rung, which is a choice the
strategies are entitled to make and which `Hot`'s stated intent (`I5`, performance) plausibly wants, then
the pair's irreducibility is forced a second, independent way, with zero dependence on `Precise`. That is
a real and useful result, and it belongs in the register as a conditional finding tied to an open axis
(alignment-as-a-strategy-choice), not as an unconditional fact standing beside Cold's.

This matters for what a canon sentence says. "The two-output requirement is forced twice over, by `Cold`
and by `Hot`'s wide-rung alignment" reads as two facts about the design. The accurate sentence is "the
requirement is forced by `Cold` alone, and would be forced a second, independent way by any future
alignment divergence between strategies, which the design has not yet settled." The first sentence would
mislead a reader into treating an open axis as closed evidence for a closed question, which is exactly
the failure mode `RULES.md`'s "reading list needs a slot for the repository" section names from the other
direction: treating unsettled material as though it had already been checked off.

## 4. A second attack: the "widening recovers" check in `p4` is vacuous, and the pigeonhole finding does
not need it

`45_probes/p4_fraction_crosscheck_and_widening_recovers.py`, opened directly (`45_probes/p4_fraction_crosscheck_and_widening_recovers.py:68-77`):

```python
# part (b): the widened computation never rounds the intermediate at
# all, so its "step 1" is the EXACT product x*a, not m. Its final
# rounding is the only rounding it ever does, so it is definitionally
# equal to `once1` / `once2` for the respective x. Check it anyway,
# via the same exact-Fraction machinery, rather than asserting it.
wide1 = round_nearest_fraction(x1 * a * b, quantum)  # no intermediate round
wide2 = round_nearest_fraction(x2 * a * b, quantum)
if wide1 != once1 or wide2 != once2:
    widening_always_matches = False
```

`once1` is computed four lines earlier as `round_nearest_fraction(x1 * a * b, quantum)`
(`45_probes/p4_fraction_crosscheck_and_widening_recovers.py:58`), the **identical expression**. `wide1`
and `once1` are the same Python expression evaluated twice; `wide1 != once1` cannot be true for any
input, so `widening_always_matches` cannot ever become `False`. The comment says as much
("definitionally equal") and then runs the check anyway, which reads as a check but decides nothing: it
is `assert X == X`, not a test of whether a genuinely wider, still-finite intermediate recovers the
once-truncated answer.

`45` reports this as a result: "does the WIDENED computation... match the once-truncated exact reference?
It does, in every case checked, with zero exceptions" (`45:221-223`). That sentence is true and it is
true by construction of the code, not by anything the search discovered. It establishes nothing beyond
what the comment already states in plain language: infinite-precision arithmetic, rounded once at the
end, agrees with itself. A real test of "widening recovers" would model a **finite** wider intermediate
(the way `16_probes/p5_recovery_direction.rs` models Precise-widens as doubling the native rung, which
`45`'s own `p5_third_output_is_mechanically_free.rs` also uses as its model) and check that rounding once
from *that* representation, rather than from an exact rational, matches the once-truncated reference.
Nothing in `p3` or `p4` does this.

**What this does and does not cost the pigeonhole argument.** The existence claim, that no `F`-bit-per-
step rounding rule matches the once-truncated exact answer for every input, is untouched. That claim
rests on the **disagreement counts** (`p3`: 61, 732, 7354, 73461 under round-half-up; `p4`, independently
coded with exact `Fraction` arithmetic and a different comparison method, reproduces the identical
counts), and I re-checked the cross-check myself by re-deriving the first `F=4` witness by hand: `x1 =
8/16`, `x2 = 9/16`, `a = 1/16`. `x1*a = 8/256`, which at `F=4` (quantum `1/16`, i.e. sixteenths of
sixteenths collapsing to sixteenths) rounds to `1/16`; `x2*a = 9/256` also rounds to `1/16`. Both true
step-one products round to the same representable point. That part of `p3`/`p4` is a real, non-vacuous,
correctly cross-checked finding, and I confirm it by hand-checking the arithmetic rather than trusting
the printed output. What is vacuous is only the "widening recovers" half, which `45` states as though it
were a second, separate check earning its own weight, and it is not: it is the same fact the pigeonhole
argument already assumes (an intermediate with enough bits to distinguish `x1` from `x2` does not lose
the information that causes the collision), restated as a tautological code check rather than
established by one.

`45`'s own section 9 already flags, honestly, that "whether a narrower widening... suffices" and "the
exact growth rate for a multiplicative chain" are open (`45:497-504`). I would add: the specific claim
"widening... recovers, in every case checked" should not be cited as evidence toward that open question,
because as coded it cannot distinguish a sufficient finite widening from an insufficient one; it only
confirms that unlimited precision (which nobody is proposing to ship) recovers, which nobody doubted.

## 5. Support: sections 4, 5.1, 5.2, and 6, checked and held

**Section 4, third output mechanically free.** I opened `45_probes/p5_third_output_is_mechanically_free.rs`
directly. The trait `Derive3<R: Rung>` carries three associated consts, one macro produces the
non-widening impls uniformly across `Hot`/`Warm`/`Cold`/`Precise`, and switching `Precise` to the
widening reading is exactly one additional `impl` block gated by `cfg(precise_widens)`, with nothing
else in the file changing. This is a real, minimal, gate-free demonstration and it matches the general
move named in `a-refused-bound-wants-a-trait-not-a-feature.md`: decompose a refused expression into named
associated items rather than trying to compute it inline. I have no attack on this section; it holds.

**Section 5.1, the "type system" half of the dichotomy is narrower than it reads.** I checked this
against `16` directly. `16`'s `p5b_const_to_type.rs` refuses one specific syntactic form (arithmetic on a
bare const, in bound position), naming `generic_const_exprs`. `16`'s own `p6_trait_form_recovers_both.rs`
computes both outputs together, from the original `(Strategy, Width, Sign)` triple, with zero feature
gates. `45` is right that citing the first refusal as evidence that "the pair's irreducibility rests only
on the const-to-type argument" (the pre-correction `OPTIONS.md` language, `44:361-362`) overstates what
the refusal shows: it shows one dead-end route is dead, not that the trait mechanism (which never tried
to go through that route) pays any cost. Confirmed.

**Section 5.2, the "recoverable from stride alone" question is narrower and more hypothetical than it
reads.** I checked the acceptance criterion's placement of erasure directly, per `00_brief.md:145-146`
(now reading, since the file was updated, "it erases on lowering", i.e. at codegen, not at the point
layout facts are derived) and per `15:274-282`, which keys the map on strategy from the start. Given `S`
is retained as a type parameter for the entirety of a numeral's Rust-level existence, any site holding
the numeral's type can re-project `Derive<S, W, Sign>::Carrier` or `::Stride` directly, at zero cost,
rather than needing to recover one from the other. `45` is right that nothing in the panel proposes
dropping `S` early, and that `16`'s `p5b` was testing a scenario (recovering carrier from a bare const
pair with `S` unavailable) nobody has proposed building. This is a genuine, correctly derived narrowing.

**Section 6, is the derivation itself licensed.** I re-checked the grep myself:

```
grep -n "matching container" 00_brief.md
145:bytes, the typestate derives the matching container and numeral **representations**, it validates, and
```

The file now carries the plural, fixed between `45`'s writing and mine (section 1 above). The underlying
point, that the derivation traces to a ratified plural noun rather than being an unlicensed addition,
holds regardless of which state of the file a reader catches it in, and I agree with `45`'s conclusion
that the derivation is not the unlicensed thing here.

## 6. Bearing on the live options, checked against `OPTIONS.md` as it now reads

**The derivation's outputs section.** *Corrects `45`'s correction, does not kill it.* The two-output
result stands, forced by `Cold` alone, independent of `Precise` (section 2). The wide-rung alignment
finding (`45` section 2.2) is real as a mechanism but is evidence for a **conditional** claim ("if
strategies diverge in alignment, the pair is irreducible a second way") rather than an unconditional
second fact standing beside `Cold`'s, because the alignment scheme it borrows its numbers from is a
replaceable assumption by `15`'s own account, not settled architecture. I would restate the blockquote
`45` added at `OPTIONS.md:759-778` to mark this distinction explicitly rather than presenting both
forcings at the same epistemic weight.

**The `Precise`-on-inexact open item.** *Unaffected by either of my attacks.* The pigeonhole existence
argument (section 4 above, the part that is not vacuous) stands, and I confirm it by independent hand
arithmetic on the first `F=4` witness. Section 8's sharpened question for op, combining `18`'s and `35`'s
findings with the pigeonhole result, is a real and well-posed question regardless of the `p4` defect,
since the defect is in a redundant corroborating check, not in the existence proof itself.

**A note for whoever writes the canon sentence about strategy-entitled divergence.** `45` section 5.3's
reframing, "forced by semantics... any strategy entitled to diverge compute-type from storage-footprint
forces the arity past one," is the right general shape and I would keep it, with one adjustment: state
explicitly that `Cold`'s divergence is ratified (`I2`/`I6`) while a hypothetical `Hot`-versus-`Warm`
alignment divergence is not yet a design fact, only a design possibility the strategies are entitled to
choose. The general mechanism (any divergence forces the arity) is a single, unifying, correct claim;
which divergences are actual is a separate, still-open question, and the register should not let the
first make the second look settled.

## 7. What I would add to the register

I am not editing `OPTIONS.md`, `INTENTS.md`, or `00_brief.md`, per my brief.

**A downgrade of the wide-rung alignment forcing from unconditional to conditional**, per section 3.2,
with the citation to `15`'s own "not decided by anything I built" and "stated in its header as an
assumption" attached, so the next reader does not treat borrowed dead-tree numbers as settled evidence.

**A note that `p4`'s "widening recovers" check is vacuous** (`wide1`/`once1` are the identical
expression), per section 4, so it is not cited as progress toward the open question of how much
widening a chain actually needs. The pigeonhole existence result itself is untouched and I hand-verified
its first witness independently.

**A cheap, unclaimed check that would settle the conditional cleanly**: whether the register, once it
does settle whether strategies may diverge in alignment at all (a question that also touches `Q5`
per `44`'s connective-tissue note), should record the wide-rung collision as a **consequence** of that
settlement rather than as a fact standing on its own. If alignment divergence is ruled out entirely (a
live option nobody in this panel has proposed but nobody has closed either), the wide-rung forcing
vanishes and `Cold` alone carries the two-output requirement.

## 8. What I would put to `45` directly, since it will be resumed

Three concrete questions, each answerable and each narrower than my attacks above might read as:

1. Do you agree that `15:553-556` and `15:418-429` state the Hot-align-16 wide-rung rule as an
   unresolved assumption rather than as settled architecture, and if so, would you restate section 2.2's
   "already forced" as conditional on that axis being settled the way your probe assumes?
2. Independent of (1), does the *general* mechanism (any two strategies diverging in alignment force the
   pair's irreducibility, whatever the specific numbers) still belong in the register as a real result,
   worth naming even while conditional? I think yes and would keep it, restated.
3. On `p4`: do you agree `wide1` and `once1` are the same Python expression, and if so, would a genuine
   test need a finite-width model (matching your own `p5`'s doubling model, or `16`'s `p5`) rather than
   exact `Fraction` arithmetic, to say anything about how much widening actually suffices?

## 9. What I could not determine

**Whether a design that keeps Hot and Warm on the same wide-rung alignment is genuinely live**, or
whether some other constraint in the panel (SIMD-friendliness, a bench nobody has run) already rules it
out in practice even though nothing states it as ratified. I did not find such a constraint stated
anywhere in my reading, but my reading was bounded (section 10) and I would not claim I searched
exhaustively for it.

**Whether the specific crossover width `15` flags as contested (`SETTLED.md:85`, Warm's 65-bit crossover)
bears on where the wide-rung collision's 40-of-640 count would land under a different crossover rule.** I
did not build this; it is a cheap extension of `p1` (parameterize the crossover and rerun) and I flag it
rather than guess at the answer.

**Whether a third, genuinely independent, finite-width model of "does widening recover" would find the
same headroom-growth rate `45` speculates about in its own section 9.** I did not build this either; it
is the natural fix to section 4's defect and I have not attempted it.

## 10. Coverage, bounded honestly

**Read end to end, directly, before touching any account of it:** `INTENTS.md`, `00_brief.md`,
`RULES.md`, `45` in full, `44` in full, `15` in full, `16` in full.

**Read at the specific passages cited, by opening the lines or the file:** `OPTIONS.md` lines 695-779
(the derivation's-outputs section as it currently reads, post-`44`, post-`45` edits), `18` lines 465-510
(section 3.4, the refuse-on-inexact figures), `42` lines 190-198 (the guard-digit reference), `35` lines
385-423 (`p10`'s EMA figures), `DROPLIST.md` lines 120-155 (the "partial associativity" retirement,
confirmed a different question).

**Probes opened and checked directly, not trusted from prose:** `45_probes/p1_wide_rung_collision.rs`
and `.out`, `45_probes/p2_p5_style_instrument_is_blind.rs`, `45_probes/p3_search_pigeonhole_witness.py`
and `.out` (hand-verified the `F=4` first witness by arithmetic), `45_probes/p4_fraction_crosscheck_and_widening_recovers.py`
(found the vacuous check at lines 73-77), `45_probes/p5_third_output_is_mechanically_free.rs`,
`45_probes/RUN.md`.

**Not read:** `02` through `43` except as cited above, `CANON_CANDIDATE.md`, `MORNING.md`,
`PERSONA_CALLS.md`, `SETTLED.md`, `seed/` beyond what `15`/`16`/`44` quote of it, the closed predecessor
panel, `archive/`. I did not run any of the Rust probes myself (no toolchain invocation in this
dispatch); I checked them by reading the source and the committed `.out` files and by hand-deriving the
one arithmetic case in section 4. I did not independently re-verify `45`'s "40 of 640" or "0 of 640"
counts by re-running `p1`/`p2`; I checked the logic of both files by reading them and found no defect in
the mechanics, only in what the mechanics are entitled to assume as input.

**Not verified:** whether `43`'s grid-invariance apparatus, which `44` section 6 discusses and which I
did not open, would extend to a genuinely settled alignment axis the way it extends to the carrier and
the fold accumulator. I am relying on `44`'s account of `43` for that connection, same as `44` relied on
it for the same reason.

**One instance of evidence, stated per `RULES.md:116-118`.** My attack on section 2.2 rests on one
citation (`15`'s own stated assumption) read by one reader (me). A second, independent reader confirming
`15` says what I say it says would move this past one instance. My attack on `p4`'s vacuous check is a
direct reading of Python source with no external dependency; I consider this closer to a proof than to
an instance of evidence, since the claim is that two lines of code compute the same expression, which is
checkable by anyone opening the file rather than by anyone building an apparatus.
