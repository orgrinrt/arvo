# The verification backlog: what rebuilds, what holds, and what the primary texts say

Donald Knuth, file 62. I wrote file 01 (mathematical rigour) and file 39 (does it still represent
them), and the second of those matters below in an awkward way: one item on this backlog is a second
read of a reading I myself produced, which the two-expert convention does not permit me to supply,
and I say so where it arises rather than quietly confirming my own work.

**What I read.** `58_consolidation_five.md` in full, the standing base, and the three deliverables
since it: `59_fog_the_lowering_door.md`, `60_dolan_value_or_datum.md`, `61_amin_the_notation_
vehicle.md`. An `ls` of the panel directory: files `00` through `61` plus probe directories, nothing
after `61`. Behind the consolidation, because this dispatch is precisely a set of checks of
derivations it compresses: `08_fog_the_union_and_what_it_costs.md` sections 5 and 11 with the whole
of `08_probes/` including its `README.md`; `57_aaltonen_the_measurement_debt.md` section 2.3;
`54_probes/` (the seal series and its support modules); `55_mcsherry_typing_the_algorithm_crates.md`
sections 1 and 2 with `55_probes/probe_2b`; my own file 39's membership section against the round
topic it cites (`mock/design_rounds/202607300800/202607291900_topic.the-number-systems-crate.md`);
and, in the shipped tree, `arvo-strategy/src/lib.rs`, `arvo-strategy/src/container.rs`, and
`arvo/src/lib.rs`. Outside the tree, two primary documents: the OCP OFP8 specification Revision 1.0
and IEEE Std 754-2019 clause 5.2, extracts and provenance in `62_probes/primary_sources.md`.

**Gates.** Canon gate: the numeral tower still has no shipped source, reproduced fresh from the repo
root, `grep -rln "Adjustment\|Bias\|Numeral" mock/crates/ --include="*.rs"` and the same with
`FullRange\|UTerm\|AddWidth`, both exit 1, empty. Nothing in this dispatch touches a shipped crate:
every experiment on shipped source ran on a copy outside the tree, and everything committed lands
under `62_probes/`, which the phase gate does not cover. Test gate: `cargo test --offline
--workspace` from `mock/`, summed per binary across 130 binaries: **658 passed, 0 failed, 9
ignored**, identical to files 59 through 61. The tests in the surface this dispatch touches are the
probe re-runs themselves, and their bodies are read and reported below, not counted.

**Compiled, measured, and reasoned, kept apart.** Sections 1, 2, 3 and the seal half of section 4
are compiled; every command, verbatim error, and timing is in `62_probes/OUTCOMES.md`. The width
sweep's numbers are **compile-time wall clocks** (`/usr/bin/time -p` around a single `rustc`
invocation, `--emit=metadata`, no codegen), on `rustc 1.98.0-nightly (57d06900f 2026-05-27)`,
`aarch64-apple-darwin`, Apple M1. The instruction counts in section 1 are static counts from emitted
asm at `-C opt-level=3`, with no timing claimed from them. Section 5 is a reading of primary
documents; nothing in it is a compile. There is no runtime measurement anywhere in this file. One
procedural fact worth its sentence: the first build of the copied-out crate ran without naming the
pin and resolved to **stable**, failing with `E0554`, which is the exact trap the brief warned
about; every command thereafter names `+nightly-2026-05-28` explicitly and the error is preserved in
the outcomes file as the specimen it is.

## 1. The width-ceiling citation rebuilds, and the exhibit under `unreproducible` collapses

This was the load-bearing item, and the answer splits into two findings, the second larger than the
question asked.

### 1.1 The claim rebuilds

The claim, as `unstable-features.md` cites it: exhaustive const-eval validation of the translation-
stability witness costs about 28 seconds at eight bits and rustc refuses at nine under
`#[deny(long_running_const_eval)]`, which grounds the rule's argument that model-width validation is
the only form available and that the `specialization`/`TypeId` bans are verification infrastructure.

I rebuilt it fresh, under my own name, per the convention `58:713-716` ratifies: the union crate
reassembled from the committed trail (section 1.2 below says why that was possible), and a new
sweep driver per width (`62_probes/drv_3.rs` through `drv_9.rs`), five constructors, one one-sided
stability check each over the span `0..=2^N - 1`, forced by `const` items:

| width | this rebuild | file 8 (`08:445-450`) |
|---|---|---|
| 5 | 0.20s | 0.84s |
| 6 | 0.72s | 2.26s |
| 7 | 2.90s | 8.65s |
| 8 | 11.64s | 28.45s |
| 9 | refused | refused |

The structure reproduces exactly: cost quadruples per bit (3.6x, 4.0x, 4.0x between adjacent
widths), and width nine is refused with the identical diagnostic, `constant evaluation is taking a
long time` under `#[deny(long_running_const_eval)]`. The constant does not reproduce: my wall
clocks are roughly 2.5x to 3x below file 8's at every width, which is unsurprising for a
wall-clock figure measured through a different harness shape (a bare driver `rustc` against a
prebuilt rlib here, against whatever file 8's build wrapped). The two parts of the claim are not
equally durable, and the distinction is exactly what an honest citation should carry: **the refusal
at nine bits is a deny-by-default lint on const-eval step count, deterministic in steps and
therefore machine-independent on this rustc; the 28.45 seconds is one machine's wall clock through
one harness.** The workspace rule's argument rests entirely on the first part, which rebuilds, so
the citation keeps its ground. I would propose one wording adjustment to op, not an edit: the rule
should cite the quadrupling and the step-budget refusal as the durable facts and mark the seconds
figure as a wall clock from one machine, because the seconds are the part a future reader will fail
to reproduce and the refusal is the part that matters.

One small correction to the consolidation on the way past: `58:722-724` says "the sweep generator is
committed." No width-sweep generator is committed; the two committed Python generators in
`08_probes/` sweep the table-versus-projection and monomorphisation questions, not the width
ceiling. What is committed is the crate recipe plus the `stable` function whose span arguments the
sweep varies, and my seven driver files now serve as the generator the sentence believed existed.

*grounded on: `pin`, `host`, `flags` (`--emit=metadata`, no codegen flags); `62_probes/drv_*.rs`,
`62_probes/OUTCOMES.md` section 2.*

### 1.2 File 57's unreproducibility finding is false, and the ground state's exhibit goes with it

To rebuild the sweep I first had to rebuild the union crate, which file 57 had established could not
be done: "neither `spare.rs` nor `fusion.rs` exists anywhere in the panel directory... **This table
cannot currently be reproduced by anyone from what is committed**" (`57:203-208`). The consolidation
adopted `unreproducible` as a fifth ground state with this as its exhibit (`58:709-716`), and op's
loudest-list item 8 (`58:1018-1019`) rests on the same finding.

The reproduction recipe is committed, and it sits in the same directory file 57 searched.
`08_probes/README.md:8-11`, committed in the same commit as file 8 itself (`f1021e9`):

> To reproduce the union: `cargo init --lib union`, edition 2024, pin the toolchain, drop
> `a_union.rs` in as `src/lib.rs`, `b_spare_pattern_decides_delivery.rs` as `src/spare.rs`,
> `c_split_does_not_bind.rs` as `src/fusion.rs`, and the `e`/`f`/`g` files under `src/bin/`.

The modules file 57 declared missing are `b_` and `c_`, present all along under their probe names.
I followed the recipe (`62_probes/rebuild_union.sh`). The crate builds clean in 0.85 seconds,
`g_classification_table` reproduces file 01's five-row table byte for byte, and `e_codegen`
compiles against the rebuilt rlib. The five-shape instruction table itself, the specific artifact
file 57 marked unreproducible, reproduces in structure and in its headline: the companion-flag
delivery is an 82-instruction, **6-branch** function here against file 8's 87-instruction,
**6-branch** loop body (my count is whole-function, a coarser measure; the branch counts are
directly comparable and match on all five rows; full counts in `62_probes/OUTCOMES.md` section 1).

I want to be exact about what file 57 got right and what it got wrong, because the wrongness is
of a specific and instructive kind. Its compiled fact is true: `a_union.rs` alone, built as
`--crate-name=union`, fails with `E0583` on the two module declarations. Its restraint convention
(never reconstruct someone else's artifact and present it as their reproduction) is good and I have
followed it here; everything in section 1.1 is a fresh derivation. What is false is the universal
sentence built on the compiled fact: "cannot be reproduced **by anyone from what is committed**."
That sentence is refuted by a file in the directory the probe ran in, whose first table row is a
column named "Question" and whose opening paragraph is the recipe. The check that would have
prevented it costs one `cat`. And the error then compounded precisely the way this review's own
provenance discipline predicts unratified findings compound: a consolidation minted a new ground
state on the exhibit, a persona checkpoint demanded a targeted re-derivation, op's morning list
carries the worry, and file 59 then applied the `unreproducible` vocabulary to a further artifact.
A universal negative ("nobody can rebuild this") is the single most expensive kind of claim to be
wrong about, because everything downstream inherits it, and it is also the cheapest kind to check,
because one successful rebuild settles it. The review should treat this as the worked example of
why a "cannot" claim owes an exhaustive look at the committed trail before it ships, exactly as a
"cannot compile" claim owes the whole matrix.

What survives for op's list: item 8 (`58:1018-1019`) resolves in the good direction. The ratified
workspace rule does not rest on an unreproducible measurement; the measurement rebuilds, structure
exact, constant machine-shaped. Whether the `unreproducible` ground state itself stays in the
registry is a smaller question than it was: its founding exhibit is gone, but the vocabulary did
real work one file later (file 59's gitignore finding, `59:517-540`, is a genuine instance), so my
suggestion, and it is only that, is to keep the ground and replace its exhibit with the bench-
artifact case.

*grounded on: `tree` (`08_probes/README.md:8-11`, commit `f1021e9`, `57:198-215`, `58:709-716`),
`pin`, `flags`; `62_probes/rebuild_union.sh`, `count_shapes.py`, `OUTCOMES.md` section 1.*

## 2. The forbidden-feature finding holds, compiled at the shipped crates themselves

File 59 section 4.1 reported, from a reduced probe, that `arvo-strategy/src/container.rs:254` does
not compile without `generic_const_exprs`. I checked it the strong way rather than re-running the
reduction: the shipped crate itself, copied out of the tree, the gate line deleted
(`arvo-strategy/src/lib.rs:11`) and nothing else touched, built under the pin against the
unmodified in-tree dependencies.

**Sixteen refusal sites**, every one `error: generic parameters may not be used in const
operations` with rustc's help naming the forbidden feature. With `#![feature(min_generic_const_args)]`
in the gate's place: sixteen refusals again, `complex const arguments must be placed inside of a
const block`, and the const-block escape refuses in turn on my own reduced probe
(`62_probes/probe_min_gca_const_block.rs`) with the help pointing at `generic_const_args`, which
needs `-Znext-solver=globally`, mutually exclusive with the arrangement per the workspace's own
record (`58:146-147`). The same experiment on the facade (`arvo/src/lib.rs:25` deleted): **478
refusal sites.** The facade's core types are the shape itself; `UFixed` is `repr(transparent)` over
`Bits<{ ufixed_bits(I, F) }, S>` (`arvo/src/ufixed.rs:35-36`), a const expression over generic
const parameters in type position, load-bearing at the crate's identity.

So file 59's finding is confirmed by a second independent expert, at the whole-crate grain, and per
the two-expert convention that closes the factual half: **the two remaining `generic_const_exprs`
gates are structural, not stale annotations.** What follows for `unstable-features.md` is a
proposal for op, since feature-table changes are his:

1. The FORBIDDEN row's sentence "everything the stack needs works under the `min_` version" is
   falsified at the two crates it does not cover. It was true of the algorithm crates the
   capacity-as-a-type migration cleaned (`arvo-comb/src/lib.rs:16` and siblings genuinely carry no
   gate); it is not true of the L0 container dispatch or the facade, and the row should scope the
   sentence to the migrated crates.
2. The "DRIFT to remediate" entry reads as comment cleanup ("both still carry the superseded
   WATCH-tier vetting comment above the gate"). The comment is indeed superseded and should go, but
   the entry's cheap reading, delete a stale gate, does not exist: deleting either gate breaks the
   crate at 16 and 478 sites respectively. The entry should state the real remediation, which is
   the spine rule's (`58:85-99`): the computed width becomes a type, which is what `Nat` already is
   in the unbuilt numeral tower and what nothing in the shipped `Bits<N, S, Sign>` surface is. That
   is a migration touching every consumer of `Bits`, `UFixed` and `IFixed`, and whoever budgets it
   should budget for that, not for a one-line diff.
3. Until that migration lands, the rule should state the contradiction it currently implies away:
   the stack's L0 dispatch and its facade are load-bearing on a feature the same rule forbids, with
   op's 2026-07-28 ruling on one side and 494 compiled refusal sites on the other. A rule that
   names its own open violation is trustworthy; one whose drift entry undersells the violation by
   two orders of magnitude of rewrite cost is how the next reader schedules an afternoon for a
   quarter's work.

There is a happier note to attach, and it belongs in the same proposal. The numeral tower's design
is the migration target already built on paper: the tower spells every computed quantity as a type
precisely because this wall exists, and files 50, 54, 55 and 56 compiled that shape at every
position they touched. The remediation of the shipped gates is not new design work; it is the
existing design arriving at L0.

*grounded on: `tree` (`arvo-strategy/src/lib.rs:11`, `arvo-strategy/src/container.rs:254-258`,
`arvo/src/lib.rs:25`, `arvo/src/ufixed.rs:35-36`, all read fresh), `pin`, `flags`;
`62_probes/strip_gate_experiment.sh`, `probe_min_gca_const_block.rs`, `OUTCOMES.md` section 3.*

## 3. The ordinary confirmations

**`foldnum(W, A)`: passes its first independent read, with one wording precision.** My own
derivation, formed before re-reading file 55's: the exact sum of `A` values each drawn from a
`p`-digit numeral is bounded by `A(2^p - 1) < 2^(p + ceil(log2 A))`, so `p + ceil(log2 A)` digits
suffice, and the bound is achieved when `A` is a power of two, so the formula is tight, not merely
sufficient. The signed case gives the identical formula over a two's-complement range. The shape (a
numeral-level map beside `mulnum` and `divnum`) is consistent with the settled machinery, and the
arity-is-the-capacity argument (`55:113-116`) is correct: no simple path in a DAG on `C` nodes
visits more than `C`. The wording precision: "carries `W`'s precision plus `ceil(log2 A)`" should
say the map **adds `ceil(log2 A)` integer digits and leaves the quantum unchanged**, because for a
fixed-point operand the grid must not move, only the range, and "precision plus" is one careless
reading away from scaling the fractional side too. This matches the fold accumulator's own shape at
`58:288-295`, quantum fixed, width grown. Note the accounting: file 55 flagged the vocabulary as
owing two independent reads; this is the **first**. One more is owed, by someone who is neither
file 55's author nor me.

**`Unbounded`: the mechanism passes, the vocabulary owes the carrier-at-birth treatment before it
is settled.** Probe 2b re-runs clean (zero errors), and the coherence argument is exactly right:
`Unbounded` is not a `Pos`, so the two blankets are disjoint by parameter and no specialisation of
any kind is involved. My reservation is about the position, not the mechanism. `InteriorSafety<A>`'s
`A` now quantifies over an open union, `Pos` or `Unbounded` or anything else a third party names
(which merely fails to solve, so nothing is unsound). But the review's own rule, `58:101-104`, says
a closed vocabulary that a guarantee quantifies over owes its seal and its adversary **at birth**,
and arity has just become such a vocabulary: two constructors, `Fin`-like and `Unbounded`, with
interior safety quantified over it. The two-line fix is the tower's own idiom, a sealed `Arity`
kind with the finite constructor wrapping a `Pos`, at the cost of spelling `Fin<P>` where `P` now
sits bare. I recommend adopting `Unbounded` with that condition attached rather than as-is; the
rule being applied is the review's, not mine. Same accounting as above: first independent read
done, second owed.

**The `Rad<P>` seal: all four routes reproduce, and a fifth route I added is refused twice over.**
File 54's probe series re-runs verbatim under the pin: the positive tower compiles; radix one
refuses at the bound (`H: AtLeastTwo is not satisfied`, E0277); the forged-carrier direct impls
refuse on the private supertraits (E0277 x3); the supertrait-reach and blanket routes refuse (E0603,
E0210). The route the four-route enumeration did not name as a route: `AtLeastTwo` is itself a
predicate trait, and if a consumer could implement it for `H`, `Rad<H>` would become legal without
touching any carrier's seal. It cannot: `62_probes/probe_atleasttwo_for_h.rs` refuses on both the
private module (E0603) and the orphan rule (E0117). So the sealed-predicate half of the
construction is as closed as the carriers, which the ratified-table edit was standing on without a
compiled witness. The honest limit stated at `58:370-372` (verified as "every attack found lands in
one of the four," not "four is the whole space") still stands and now covers five.

**The membership reading: I am disqualified from the check the review actually owes, and I say so
rather than perform it.** The item queued at every checkpoint since file 39 is a second independent
read of the finest-inhabited-system candidate (`58:268-269`, `58:1122-1123`). I wrote file 39. My
file said, at the time, "a second member's independent read of the same topic is owed before
anything builds on this paragraph" (`39:373-374`), and the two-expert convention's whole content is
that the second reading is formed by someone who did not form the first. Me re-deriving my own
candidate would be confirmation wearing corroboration's clothes, and a dispatcher counting it as
the owed read would be laundering. What I can honestly contribute, and did: the citation
underneath the candidate verifies. The round topic's precision paragraph says, verbatim at the
position file 39 cited, "the predicate is **inhabits**, not **equals**. `Natural` asserts that
every value of the type is a natural number, not that the type represents all of ℕ"
(`202607291900_topic.the-number-systems-crate.md:80-82`), inside a section whose framing is
recorded as op's; and the structural sibling (report the finest relation that holds rather than
choose one) is file 37's, reached independently. The quote is real, the provenance is as file 39
stated it, and the second read remains owed, **assignable to anyone but me and file 39's other
sources**. That is this item's honest state and I decline to improve on it.

*grounded on: `pin`, `flags` for every compile; `tree` for `202607291900:80-82`;
`62_probes/OUTCOMES.md` sections 4 and 5; the foldnum derivation is reasoned and its tightness
claim is arithmetic a reader can check in the margin.*

## 4. The two secondary-sourced facts, against the primary texts

Both checks were worth making, and they came back in the pattern the review should find
reassuring and then immediately not: both facts **confirm**, and both primary texts turned out to
carry something the secondary sources could not have shown, one a defect in the primary source
itself, one a scope qualifier that narrows the design's own stated deviation.

### 4.1 OFP8: confirmed verbatim, and the primary source contradicts itself where nobody quoted it

The OCP 8-bit Floating Point Specification, Revision 1.0 (approved June 20, 2023), obtained from
OCP's own FP8 repository, states on page 12, prose, nearly in the review's own words:

> The E4M3 format does not represent infinities and uses only two bit patterns for NaN (a single
> mantissa-exponent bit pattern but allowing both values of the sign bit) in order to increase
> emax to 8 and thus to increase the dynamic range by one binade.

Table 1 gives E4M3 bias 7, emax 8, emin -6; Table 2 gives infinities N/A, NaN `S.1111.111`, max
normal 448; E5M2 bias 15, emax 15, infinities and three NaN codes, max normal 57,344. Every figure
the review carried (`58:534-540`, file 50's "raise emax from 7 to 8 and gain a binade") is
confirmed against the primary text, including the phrase "one binade" itself. The `NanOnly`
witness row's E4M3 citation hardens.

Two findings the check produced beyond the confirmation. First, **the primary source is internally
contradictory**: its own definitions section (page 11, section 4.2) gives E4M3 "an exponent bias of
15" and E5M2 "an exponent bias of 7," the two biases transposed, refuted by the document's own
Table 1 and by arithmetic (E4M3's min normal `2^-6` forces bias 7). A reader sent to "confirm
against the PDF" who lands on the definitions section extracts exactly the wrong figures with the
full authority of the primary source behind them. The spec sentence that cites this document should
cite **Table 1, page 13** by name, not the document, and this is a concrete instance of a general
rule worth a line in the consolidation: a primary-source citation owes the position within the
source, because primary sources carry typos with more authority than secondary sources carry
truths. Second, a mis-attribution to correct in the witness table: **the OCP document defines E4M3
and E5M2 only; no FNUZ variant appears anywhere in it.** `58:537`'s row "OCP OFP8 `E4M3`, and its
`FNUZ` variant" attributes `E4M3FNUZ` to a document that does not contain it. The format is real
and the fact used about it (negative-zero datum repurposed as NaN, file 54's injectivity witness)
is right, but its home is the earlier Graphcore/AMD/Qualcomm 8-bit formats proposal and the ONNX
type registry, and the table row should split the citation accordingly.

### 4.2 IEEE 754 clause 5.2: confirmed for the exact case, and the design's deviation is narrower than stated

The review's characterisation (`54:372-374`, carried at `58:594-596`): the standard specifies, per
operation, a preferred exponent for decimal results, "a function of the operation and its operands'
exponents rather than of the result's value," which `Canonical` cannot express because `Canonical`
is a function of the value alone. The standard's actual governing paragraph (754-2019, clause 5.2,
page 30, verbatim in `62_probes/primary_sources.md`):

> For all computational operations except where stated otherwise, if the result is inexact the
> cohort member of least possible exponent is used to get the maximum number of significant
> digits. If the result is exact, the cohort member is selected based on the preferred exponent
> for a result of that operation, a function of the exponents of the inputs.

And the standard's own definition scopes the term itself: "the value of the exponent q which best
reflects the quanta of the operands **when the result is exact**."

So the characterisation is correct about the exact-result branch and silently generalises it. The
standard's selection rule has two branches, and the **inexact** branch selects by "least possible
exponent," which for a given value and precision is a function of the result's value alone, which
is to say: **a `Canonical`-expressible rule.** It is, in fact, one of the two natural cohort rules
file 54 already compiled as `Canonical` candidates (`58:580-584`, "largest significand with the
smallest exponent"). Three consequences, each a sharpening of the deviation sentence the spec owes:

1. **A decimal `Ranged` numeral whose `Canonical` picks least-possible-exponent conforms to clause
   5.2 for every inexact result.** The deviation is confined to exact results whose preferred
   exponent differs from the canonical choice, plus the two operations (quantize,
   roundToIntegralExact) that deliver the preferred exponent unconditionally. The spec sentence at
   `58:607-611` ("does not represent clause 5.2's preferred exponents") should say that, because
   as written it concedes a strictly larger non-conformance than the design has.
2. **For multiplication on `Implicit` decimal numerals, the design delivers the standard's
   preferred exponent by construction.** The preferred exponent for multiplication is
   `Q(x) + Q(y)`, and `mulnum` computes exactly the exponent sum at the type level (`58:139-141`,
   compiled by files 50 and 54). The design's compile-time quantum is not merely "strictly
   stronger" in the abstract, as file 54 argued; on this operation it is the standard's own rule,
   discharged statically. The same holds for division's `Q(x) - Q(y)` against the held `divnum`
   shape. That convergence deserves a sentence in the spec, because it turns "not conformant" into
   "conformant by construction exactly where the exponent is a type."
3. **Quantize is special in the standard's own text and should be named as such, not folded into
   the general deviation.** Clause 5.2's scoping sentence: "Except for the quantize operation, the
   value of a floating-point result... is never dependent on the representation or encoding of an
   operand." Quantize is the one operation whose value semantics are datum-dependent, so the
   design's value-valued operation surface cannot express it at all, at any `Canonical`. That is a
   different and honest sentence from the cohort-selection deviation, and conflating them
   overstates the second while understating the first.

The check file 54 asked for ("a member with the standard should do the same," `54:413-414`) is done;
both figures harden; and the deviation the design carries is smaller and better-shaped than the
sentence currently claiming it.

*grounded on: the two primary documents as quoted, with position-level provenance, in
`62_probes/primary_sources.md`; `settled shapes` (`58:580-584`, `58:594-611`) for what they are
checked against. Nothing in this section is a compile.*

## 5. What survives as shape, for the next consolidation to take directly

**The width-ceiling citation stands**, re-derived: cost quadruples per bit, refusal at nine bits by
a step-budget lint, machine-independent; the seconds figure is one machine's wall clock and should
be marked so wherever it is quoted, including in `unstable-features.md` (proposal to op).

**The `unreproducible` ground state loses its founding exhibit.** File 8's five-shape table and its
width sweep both rebuild from the committed trail by following `08_probes/README.md:8-11`; file
57's "cannot be reproduced by anyone from what is committed" is refuted by a file in the directory
its probe ran in. Op's loudest-list item 8 resolves: no ratified rule rests on an unreproducible
measurement. The ground state's vocabulary still has one genuine instance (file 59's bench-artifact
finding) and can keep it as the exhibit if op keeps the ground at all.

**The two shipped `generic_const_exprs` gates are structural**, 16 and 478 compiled refusal sites,
second-expert-confirmed on file 59 section 4.1; the `unstable-features.md` drift entry undersells
the remediation by the full size of the spine-rule migration, and the proposed rewording in section
2 goes to op with the note that the numeral tower is already the migration's target shape.

**`foldnum` adopted-in-shape with the quantum-unchanged wording; `Unbounded` adopted-in-mechanism
with the sealed-`Arity` condition; each now carries one of its two owed independent reads.** The
`Rad<P>` seal's ratified-table edit now stands on a re-run plus a fifth compiled route (the
predicate's own seal). The membership second read remains owed and must go to a member who is not
me.

**Both secondary-sourced facts harden into primary-sourced ones**, with three spec-text
sharpenings: cite OCP Table 1 by position (the document's definitions section transposes the
biases); split the FNUZ attribution out of the OCP row; and restate the clause 5.2 deviation as
exact-results-only, with the inexact branch conformant under a least-possible-exponent `Canonical`
and multiplication conformant by construction on `Implicit` numerals.

## 6. Droplist addition

A universal unreproducibility claim grounded on compiling one file in isolation: file 57's
`57:203-208`, refuted by the committed recipe at `08_probes/README.md:8-11` and by the rebuild in
`62_probes/rebuild_union.sh`. The restraint half of the convention (a fresh derivation under one's
own name, never a reconstruction presented as someone else's build) survives and was applied here;
the "nobody can" half of any such claim owes an exhaustive read of the committed trail first,
because it is the cheapest universal to refute and the most expensive to be wrong about.

## 7. Open, and I am not closing them

Whether the `unreproducible` ground state stays in the registry now that its exhibit is gone (op's,
since the checkpoint that adopted it was persona-decided). The second independent reads of
`foldnum` and `Unbounded`, one each still owed. The membership second read, owed to a member who is
not me. The loop-body-grain reproduction of the five-shape table under `-C codegen-units=1` (my
counts are whole-function; the table is five functions in one compilation unit, which is exactly
the flag-sensitive shape file 52 named, and nobody has swept it now that sweeping is possible).
Whether `Canonical = least possible exponent` should be the design's stated decimal default given
that it buys clause 5.2 conformance for every inexact result, which is a design call this file's
evidence informs and does not make.
