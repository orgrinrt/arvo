# 45. What each claim rests on

**Member:** Xavier Leroy. I wrote file 10 (the certification pass that produced the four evidence
bins and established that the model-width transfer rests on the forbidden-feature bans), file 28
(the identity contract, one of the two files that argued D69, and the origin of the overflow-band
sentence file 44 just corrected), and file 38 (the evidence-class ledger this dispatch extends).
Two of the three defects this dispatch descends from live in sentences I wrote or carried, which
is the correct reason the backfill is mine: the discipline that failed was my discipline, applied
at the wrong granularity, and the repair should be run by the person whose taxonomy had the blind
spot. The habit of mind is unchanged from file 10: a verification claim is only as good as the
statement it actually proves, and the statement lives in the definitions, which here means that
"checked" is meaningless until the record says checked *against what*.

**Gate:** run before this work, myself. `cargo test --workspace` from `mock/`: 654 passed, 0
failed, 9 ignored, summed per binary with an explicit sed over every `test result:` line rather
than trusted from a headline, matching files 41 through 44 exactly. Test bodies read in the
surfaces my claim set touches: `mock/crates/arvo/tests/fixed_point_div.rs` in full (real
assertions with hand-derived expected raws, deliberate container-overflow setups, the one
catalogue red correctly marked, matching file 43's account), and the nine compile-fail pairs in
`mock/crates/arvo/tests/ui/` (counted: eight `no_multiplicative_identity*` pairs plus
`no_signed_identity_on_unsigned`, matching my own file 38's count). No tautologies, no helping
setup found in what I read. Canon gate: `40_consolidation_three.md` and `44b_op_checkpoint_ten.md`
in full before any work; the dispatch executes a call op made at `44b` ("Adopted, including the
backfill"), so the work itself is canon-licensed by the checkpoint that ordered it. Nothing below
overturns a ratified call. One note on scope provenance: file 44's own section 7 recommended
backfilling only the already-identified sentences; op's `44b` widened that to the whole current
consolidation, and the wider scope is what this file executes, because the ratified call outranks
the member's narrower proposal.

**What I read:** `40_consolidation_three.md` in full, twice, the second time claim by claim for the
backfill itself. `41`, `42`, `43`, `44` in full, plus `44b`. My own `38` section 1 reread at the
source rather than from memory, because the bins I am extending are quoted below and a
recall-based restatement of a four-line taxonomy is exactly the failure class my own file 38
documented in others (`38:158-159`, three restatements of a four-line trait, none matching).
`10` section on the transfer basis, at the source. Spot reads where a grounding forced them:
`33:265-277` through file 44's re-verification rather than re-derived (44 rebuilt the probe fresh;
I did not repeat a rebuild two files later), `36`'s and `41`'s perimeter sections through files 42
and 44's independent checks. `ls` of the review directory once: 44 numbered deliverables plus
probe directories before this one.

**What I verified myself, separated from what I reasoned or inherited.** Verified fresh in this
dispatch: the test suite and the test bodies above; the toolchain identity (`rust-toolchain.toml`
pins `nightly-2026-05-28`, `rustc --version` inside the repo resolves it to `1.98.0-nightly
(57d06900f 2026-05-27)`, `rustc -vV` host `aarch64-apple-darwin`); the design-surface grep, with a
correction to the recorded command (section 4.4: `crates/` does not exist at the repo root, the
shipped crates live under `mock/crates/`, and `grep -rln "Adjustment\|Bias\|Numeral" mock/crates/
--include="*.rs"` returns nothing, exit 1, which is the honest empty result the conclusion always
needed); and the `FullRange\|UTerm\|AddWidth` grep backing `40:511`, also empty against
`mock/crates/`. Inherited with the inheriting stated: every compiled result cited from files 27
through 44 is taken at its own file's word where a later independent file already re-checked it,
and taken as single-checked where none has, and the claim set below marks which. Reasoned: the
registry design in section 1, the two design answers in section 2, and every status judgement in
section 3 that is not itself a compile.

## 0. The verdict, stated first

**The backfill is done, is section 3, and is written so the next consolidation can absorb it as
its section 1 annotation layer rather than transcribe it.** Every load-bearing claim in `40`
section 1, as amended by files 41 through 44 and by op's tenth checkpoint, carries a grounding
drawn from a closed registry (section 1), per member where the claim is a conjunction, and marked
`unknown` where no derivation could be found rather than guessed.

**On the first design question: `grounded on: <decision>` is necessary and not sufficient, and
the backfill itself is the demonstration.** Roughly half of the claims in the consolidation rest
on no ratified decision at all. The E0275 composition wall, the SIGBUS, every compile-cost number,
every instruction count and every byte-identical-erasure result rest on the toolchain pin, the
host target and the build-flag discipline; every bounded-exhaustion result rests on the model
width and transfers to real widths only through the forbidden-feature bans, which my file 10
established and `40:30-32` already states at the taxonomy level without ever attaching it to a
claim. A `grounded on:` field whose vocabulary is only decisions cannot record what those claims
rest on, and the next pin bump (a deliberate workspace act by standing policy, so it will happen)
is a coordinate change of exactly the D69 kind with exactly the same unqueryable blast radius. The
repair is small: the field stays as adopted, and its vocabulary widens from decisions to
*grounds*, of which ratified decisions are one of four kinds. This keeps the intent of op's call
(the next overturn's blast radius is a grep) and widens the vehicle to cover the overturns that
are not checkpoint-shaped.

**The evidence class and the grounding are orthogonal axes, and they are not independent fields.**
File 38's bin records *how* a claim was checked; the grounding records *against what*. But the bin
mechanically determines a mandatory minimum grounding: every bin-2 claim is grounded on the model
width, the forbidden-feature list and the pin, by what bin 2 means; every bin-3 claim is grounded
on the pin, the host and the build shape, by what bin 3 means. So those grounds need never be
hand-written per claim: they attach by bin, once, in the registry. The hand-written residue is the
decision-shaped grounds, which is exactly the part Ringer's proposal named, plus the per-member
splitting for conjunctions. This division of labour is what makes the convention cheap enough to
survive.

**On the second design question: greppability requires a closed vocabulary, and the repo already
ships the checking mechanism; nobody had connected it.** A grounding written as free prose ("the
rational coordinates", "D69", "the overturned width parameterisation") is three spellings of one
ground and a grep finds one of them. The registry in section 1 fixes slugs, and the syntax is
fixed to one line shape. That is the honest tier for research prose, which is ungated. The
mechanism tier exists the day these claims graduate into `*.md.tmpl` design documents: mockspace's
reference syntax (`.claude/rules/reference-syntax.md`) provides registry namespaces with
snake_case slugs whose references are resolved and checked at render, "a reference that points
nowhere is reported rather than rendered as something that looks fine." A `decision::` (or
`ground::`) namespace with one row per registry entry makes every grounding an existence-checked
reference, and a slug is never renamed or reused by that rule file's own discipline. What the
mechanism cannot do is also stated plainly in section 2.2, because a mechanism whose perimeter is
unstated is the exact failure this review keeps finding: it cannot detect an *unwritten* grounding.
That residual is discipline, and the per-bin defaults shrink it to the decision residue.

**The backfill caught things, as the dispatch predicted it would.** Six findings, section 4. The
two with teeth: the `Int` tier of the ratified encoding has an empty grounding set, because its
one consumer (integer `Bias`) was removed by the rational-bias repair the same checkpoint ratified
(4.1); and every "by construction" claim about the value-unique tower is currently conditional on
the seal, which `44b` explicitly demoted to a follow-up task, so the bin-1 status of the tower's
uniqueness is grounded on an implementation task that has not landed in the composing tree, and
file 42's compiled attack is the proof that the condition is load-bearing rather than formal (4.2).
Neither contradicts a ratified call; both are exactly what a grounding field exists to make
visible.

## 1. The ground registry

The closed vocabulary. A grounding names one or more slugs from this table and nothing else; a new
ground adds a row here first, with its rung, and then gets cited. Slugs are never renamed, never
reused, never numbered-only (per `reference-syntax.md`'s own registry-row rule). Each row carries
its provenance rung, because the workspace's provenance ladder applies to grounds exactly as it
applies to everything: a claim grounded on an op-ratified decision and a claim grounded on a
panel-settled shape are both queryable, and they are not equally settled.

### 1.1 Ratified decisions (rung: op-ratified, the governing rung)

| slug | decision | where ratified |
|---|---|---|
| `d16` | derived laws safe, asserted laws `unsafe impl` | pre-review decision set, carried `40:248-249` |
| `d38` | the `arvo-num-systems` crate ships | op, carried `40:209-212` |
| `d39` | membership through algebraic structure; held, not overturned | op, `30b`, held again `40:212-214` |
| `d47` | the algebra ladder goes as deep as the theory does | op, carried `40:382-384` |
| `d51` | law verdicts by blanket construction, not per type | op, carried `40:248-249` |
| `d58` | decimal is a proof case, not an afterthought | op, carried `40:78-79` |
| `d69` | identity parameterised in mathematical coordinates; precision and exponent bounds primitive, width derived | op, `30b`, `40:48-55` |
| `vu` | numeral encodings are value-unique as types | op, `34b`, `40:428-432` |
| `widening-out` | `Widening` leaves `Lowering` | op, `39b`, `40:585-587` |
| `growth-out-of-key` | `Growth` leaves the law key | op, `39b`, `40:588` |
| `finest-view` | the finest-view lattice replaces the three-relation fork | op, `39b`, `40:589` |
| `enc` | the value-unique encoding replaces the width chain, rational `Bias` included | op, `44b` ("The encoding is ratified") |
| `seal-owed` | the `Pos`/`Nat` seal is an implementation task, owed, not gating the shape | op, `44b` |
| `div-held` | file 43's division shape recorded, not adopted; operation surface waits | op, `44b` |
| `grounding` | every claim carries `grounded on:`; the backfill is part of it | op, `44b` |

### 1.2 Settled shapes (rung: panel-settled; presumed correct, not op-ratified as a named call)

These are load-bearing grounds that other claims rest on, settled across multiple files and
carried through consolidations without a checkpoint line of their own. They are grounds because
claims derive from them; their rung says a future member may still overturn them with evidence,
which a ratified decision does not invite.

| slug | shape | settled by |
|---|---|---|
| `round-first` | round on the unbounded grid first, classify second | 28, recompiled 31, re-verified against IEEE 754-2019 clause 7 by 39; `40:167-180` |
| `crossing` | the section-retraction triple (identity on values, idempotence on data, injectivity derived) | 30/31, `40:146-162` |
| `sign-split` | `SignDomain` on `Numeral`, `SignIndexing` on `Encoding` | 30/31, `40:87-94` |
| `lowering-charter` | `Lowering` changes no value; no law may read a datum-level fact | 30, carried unchanged, `40:128-135` |
| `const-fn-key` | a law's key is a `const fn` parameter list; unnameable facts fail `E0425`/`E0433` | 26's mechanism, applied by 34/37, `40:130-135` |
| `two-safety` | interior safety and total safety are two conditions serving two promises | 34, `40:335-344` |
| `bias-rational` | `Bias` is a signed, gcd-normalised rational | found by 39, built by 41, hardened by 42, absorbed into `enc` at `44b` |

### 1.3 Physical grounds (rung: facts about the environment; change by act, not by argument)

| slug | ground | identity, verified this dispatch |
|---|---|---|
| `pin` | the toolchain every compile and measurement ran on | channel `nightly-2026-05-28`, resolving to `rustc 1.98.0-nightly (57d06900f 2026-05-27)`; the hash is the ground, the channel is its name (see finding 4.5) |
| `host` | the target every instruction count, `csel` observation and erasure result is a fact about | `aarch64-apple-darwin`, never previously named in any file including my own 38 (finding 4.6) |
| `flags` | the build-shape discipline (`-Cno-prepopulate-passes` for axis legibility, shipping shape for codegen quality) | `40:525-532`, file 34's corrected discipline |
| `model` | bounded exhaustion at a model width, with the width stated per claim | `10`, `40:30-32`; exhaustion refused at nine bits on this pin |
| `ffl` | the forbidden-feature list, specifically the `specialization` and `TypeId` bans, as the transfer basis from `model` to real widths | `10`, `unstable-features.md`; without it every `model` claim is a fact about eight bits and nothing else |
| `tree` | the shipped source at the current commit, for claims about what exists today | verified by grep against `mock/crates/` this dispatch |

### 1.4 The attachment rules

Written once here so the claim set does not repeat them four hundred times.

1. Every bin-2 claim (bounded exhaustion) is grounded on `model + ffl + pin` by definition of the
   bin. The claim set below writes only its *additional* grounds.
2. Every bin-3 claim (measured) is grounded on `pin + host + flags` by definition of the bin. Same
   convention.
3. Every bin-1 claim whose content is a compiler refusal (`E0271`, `E0275`, `E0308`, `E0425`,
   `E0433`, the SIGBUS) is additionally grounded on `pin`, because a refusal is a fact about this
   solver until a compile-fail test pins it, and the four owed codegen regression tests
   (`40:662-665`) have refusal-side siblings that also do not exist yet.
4. A conjunctive claim grounds per member. A member with no findable derivation is written
   `unknown`, never inherited from a sibling. This is file 44's second addition, applied.
5. The syntax is one line, machine-stable: `grounded on: <slug>(, <slug>)*`. The overturn query is
   `grep -rn "grounded on:.*\b<slug>\b" *.md`.

## 2. The two design questions

### 2.1 Is `grounded on: <decision>` the right axis?

The evidence class answers "is there an artifact behind this sentence, and of what strength." The
grounding answers "what must remain true for the artifact to still be about the current design."
File 44 argued they are orthogonal and I concur, with one sharpening from having now walked all of
them: they are orthogonal *axes* but not independent *fields*, because the evidence class implies
a mandatory floor of physical grounds (rules 1 through 3 above). Writing those by hand per claim
would be four hundred repetitions of the same three slugs, which is how a convention dies; deriving
them from the bin makes the hand-written content exactly the part that varies, which is the
decision and settled-shape residue.

The widening from `<decision>` to the four ground kinds is not a quibble, and I want to state the
argument in the terms of op's own call rather than around it. The call's intent (`44b`): "the next
overturn's blast radius is a grep rather than a hand sweep." The next overturn is not guaranteed
to be checkpoint-shaped. The pin bump is a standing, planned workspace act; it invalidates, at a
stroke, the exact-instruction-count claims, the compile-cost table, the two solver-behaviour
findings (the E0275 wall and its SIGBUS corroboration, the most pin-fragile results in the
review), and potentially the erasure results, while leaving every decision untouched. Under the
narrow vocabulary that event has no slug, so its blast radius is a hand sweep, which is the exact
condition the field was adopted to end. The same holds for a target change (nothing measured here
says anything about x86_64) and for any future change to the forbidden-feature list, whose second
job as the model-transfer basis my file 10 established and which `unstable-features.md` now
records: relax either ban and every `model`-grounded claim in this review loses its transfer to
real widths simultaneously. The registry gives each of these one slug and therefore one grep.

One boundary case the backfill surfaced and the axis should absorb rather than special-case:
claims about the *shipped tree* (the rewrite-cost-near-zero claim at `40:511`, file 34's
`TotalOrd` reclassification, the census greps). Their ground is `tree`, at a commit. They go stale
by commit, not by decision, and the review has already once carried a tree claim through four
files as a copied verification command (finding 4.4). `tree` is a ground like any other and its
claims re-verify by re-running the named command against the directory that exists.

### 2.2 What makes the field actually queryable?

Three tiers, honestly bounded.

**Tier one, now, in research prose: closed slugs plus fixed syntax plus grep.** This is a
convention, not a mechanism, and I say so plainly. Its enforcement is that consolidations are
written by members under this review's citation discipline, and a consolidation is the one
document type here that gets systematically reread. The failure mode is a misspelled or free-prose
slug; the registry table being short (28 rows today) and the syntax being one fixed shape makes
the misspelling greppable itself (`grep -n "grounded on:" | grep -v` the alternation of known
slugs finds every stray). That is a checkable convention, which is the most a set of ungated
markdown files supports, and pretending otherwise would be claiming a mechanism I do not have.

**Tier two, when claims graduate to `*.md.tmpl`: the mockspace registry.** The repo's reference
syntax already provides exactly the needed machinery: a registry namespace with snake_case slug
rows, `{{ ground::d69 }}`-shaped references resolved at document generation, a dangling reference
*reported* rather than silently rendered, and `{{ sourcesof(x) }}` for provenance queries. Under
that tier, overturning a decision means editing its registry row, and every document referencing
it is findable by the renderer's own resolution pass. This is the design's own preference for
mechanisms over disciplines, satisfied by machinery that already ships; the only new thing is the
namespace declaration and the rows. I recommend this as the landing shape when the spec leaves the
research directory, and note that nothing about tier one has to be reworked to get there: the
slugs are the rows.

**Tier three, in probes: the header line.** A probe file whose result grounds a claim carries
`//! grounded on: <slugs>` in its header, beside the hypothesis statement `cl-claim-sketch-
discipline.md` already requires. Cheap, greppable across `*_probes/` with the same command, and it
puts the grounding at the artifact rather than only at the prose that cites the artifact. Files
41 through 44 already write informal versions of this sentence; this fixes its shape.

**The perimeter, stated.** No tier detects an unwritten grounding. A member who derives a claim
from D69 and writes no field has produced exactly the defect class this dispatch was sent to
empty, and no grep finds it. Two things shrink that residual to tolerable: the per-bin defaults
mean the physical grounds can never be *forgotten* (they attach by bin), and the consolidation
author, who must re-derive per-member status anyway under file 44's conjunction rule, is
positioned to catch a missing decision ground at the one point the claim gets rewritten. That is
the honest statement of what this buys: it converts blast-radius discovery from archaeology to
grep *for every claim that carries the field*, and it makes the field cheap enough that the
claims which matter will carry it. It is not a dependency tracker, and building one is neither
possible in this medium nor, on the evidence of three defects all caught within one or two files
of a member actually looking, necessary.

**On the constraints paragraph in the brief:** no type-level mechanism is proposed, deliberately.
The grounding is metadata about claims, and the claims are prose; the two places the type system
already does this work are the compile-fail tests (a refusal pinned so a loosened bound cannot
silently restore an illegal state) and the sealed tower (the closed-world hypothesis as a private
supertrait), and both are already in the design. Encoding the review's own bookkeeping into the
artifact would put the map inside the territory. The constraints are satisfied vacuously, and
saying so is cheaper than an ingenious unnecessary mechanism.

## 3. The backfilled claim set

Format per rule 1.4: bins abbreviated C (by construction), X (bounded exhaustion), M (measured),
R (reasoned without artifact); grounds listed are *additional* to the bin's mandatory floor.
Checked-by column: the file(s) whose artifact establishes it, with `+` marking independent
re-verification by a later member. Status: `current`, `current, single-checked`, `conditional`,
`corrected (nn)`, `open`, or `unknown grounding`.

### 3.1 What a number is (`40:39-55`)

| claim | bin | grounded on | checked by | status |
|---|---|---|---|---|
| `Number<N, S>` is an integer k plus a type-level injection into rationals (plus `Specials`) | R (definition) | `d69` | 27, 28 | current |
| precision and exponent bounds primitive, width derived | ratified | `d69` is this claim | 27+28 independent, op `30b` | current |
| the file-26 off-by-one against hardware formats was the parameterisation pointing the wrong way | R | `d69` | 27, 28 | current |

### 3.2 The identity contract (`40:57-104`)

| claim | bin | grounded on | checked by | status |
|---|---|---|---|---|
| the four-member `Numeral` (`Radix`/`Precision`/`Exponent`/`Domain`) | C/X | `d69`, `d58` (radix generality) | 30, +31, +34 | current |
| `Implicit` classify: five branchless instructions | M | `sign-split` | 32 | current, single-checked; `host`-specific (`csel`) |
| `Ranged` with `Specials` classify: six, no shared runtime flag | M | `sign-split` | 32 | current, single-checked |
| `SignDomain`/`SignIndexing` split | X | `d69`, `sign-split` is this claim | 30, +31 | current |
| `SC_SAT_SYM` is not a saturation mode (identical clamp, `-8` vs `-7`) | X | `sign-split` | 30, +31 independent | current |
| nesting stands on the `Underflow` argument alone; BFP withdrawn as evidence | R | `d69` | 30 proposed, 31 corrected | current (the withdrawal is the current state) |

### 3.3 Encoding inside Lowering (`40:106-143`)

| claim | bin | grounded on | checked by | status |
|---|---|---|---|---|
| `Lowering` changes no value; laws cannot name it (`E0425`/`E0433` at the point of use) | C | `lowering-charter`, `const-fn-key`, `pin` (refusal, rule 3) | 26's mechanism, 34/37 applied | current |
| trivial `Canonicalisation` adds zero instructions; trivial `FieldLayout` byte-identical to hand-rolled extractor | M | `lowering-charter` | 32 | current, single-checked |
| richer `Canonicalisation`: seven instructions against two, branchless, for the simplest real collapse | M | `lowering-charter` | 32 | current; whether *every* instance stays branchless is open (`40:670-673`) |

### 3.4 The crossing contract (`40:145-162`), per member

| member | bin | grounded on | checked by | status |
|---|---|---|---|---|
| `decode ∘ encode = id` on values | X | `crossing`, `d69` | 30, +31 recompiled | current |
| `encode ∘ decode` idempotent on data | X | `crossing` | 30, +31 | current |
| `encode ∘ decode = id` iff injective, derived boolean | C | `crossing` | 30, +31 | current |

### 3.5 The quantiser (`40:164-205`)

| claim | bin | grounded on | checked by | status |
|---|---|---|---|---|
| round-first, classify-second; three attributes agree with standard-independent oracles, both signs | X | `round-first`, and `round-first` is re-grounded on IEEE clause 7 text directly by 39 | 28, +31, +39 | current |
| dither is a zero-state extra argument | C | `round-first` | 29 | current |
| dither does not compose with `Refuse` as stated | X | `round-first` | 30, +31 | current; the confine-or-gate fork is open, op's |
| shaping is a scan, not a fold; `fold_compensated` beside `fold` | R | no ratified ground; rests on the naming argument plus the standing scheduler-is-hilavitkutin's call | 30 | current, R-bin; the counter-reading is carried in `40:200-205` |

**The overflow band, per member (the sentence that caused this dispatch), `40:178-180`:**

| member | bin | grounded on | checked by | status |
|---|---|---|---|---|
| same-format addition: empty | X | `round-first` | 28, +30/31 | current |
| multiplication: inhabited, roughly half of pairs | X | `round-first` | 30/31, +33 reproduced exactly | current |
| division: per-format-triple; empty at same precision, inhabited once precisions decouple | X | `round-first`, `d69` (the original blanket claim was derived in dyadic coordinates and went stale under D69; 43's correction is native to the ratified coordinates) | 43 | corrected (43) |
| mixed-format addition: per-format-triple; inhabited except the dividing-quantum degeneracy | X | `round-first`, `d69` | 44 | corrected (44); closed-form status of the degeneracy condition open (`44:393-399`) |
| "every float operation" | none | **unknown** | nobody, ever | unknown grounding. No derivation exists anywhere in 44 files; the member entered at `28:229-231` on the strength of its siblings, two of which have since needed correction. Recommendation: strike the member from the sentence entirely rather than carry it marked unverified. A claim with no findable derivation is not a weak claim, it is not a claim, and the next consolidation should not inherit it in any form other than the open item it already carries (the `Specials`-carrying model-float check, `43:319-322`). |

### 3.6 Membership (`40:207-241`)

| claim | bin | grounded on | checked by | status |
|---|---|---|---|---|
| membership licenses only the exact widening family; quantised laws come from the ladder | R (two independent readings) | `d38`, `d39`, `d69` | 27+28 independent, 30/31 concurring | current |
| `ExactWindow` gates on `Specials = None` (`∞ * 0` lands in no window) | X | `d39` | 30 | current, single-checked |
| finest-inhabited-system reading of D39 | R | `d39`, plus the topic file's own text | 39 only | candidate; explicitly awaiting a second independent read (`40:678-681`), and under the two-expert rule nothing builds on it until that read exists |

### 3.7 The algebra (`40:243-332`)

| claim | bin | grounded on | checked by | status |
|---|---|---|---|---|
| a law is a `const fn` whose parameters are its key, verdict the finest view | ratified | `finest-view`, `d51`, `d16`, `const-fn-key` | 37, op `39b` | current |
| view-set downward closure and join closure, hence unique finest view (3-bit model, nine views) | X | `finest-view` | 37 | current, single-checked at the model; pullback argument is the generalisation, R-bin |
| `Hot`-signed and sub-threshold `Precise` at incomparable lattice points | X | `finest-view` | 37 | current |
| law equality is the canonical quotient | X | `crossing` (the `Canonical` member is the quotient's definition) | 34, carried 37/38 | current |
| `TotalOrd` induces a datum-level order | X against source | `tree` | 34 | current at this commit; the level-annotation sentence is open, op's |
| `IS_EXACT` alone does not trivialise the grade; `IS_EXACT` with `Total<Op>` does | X (8-value model) | `finest-view` | 38 (mine), motivated by 36's type-level anticipation | current; now *inhabited* by 43's `div_floor`/`rem` if adopted, which would move the guard from prospective to load-bearing (`div-held`: not yet) |
| direction enters the key iff the exact result can leave the operand lattice | X | `d69`, `bias-rational` | 33, +44 rebuilt the probe fresh and confirmed it was rational-general from the start | current; the strongest verified survivor of the coordinate change, and 44 section 2's disambiguation of `33:270`'s English ("bias-value integer for this numeral", not "`Bias` type integer") is owed a sentence in the next consolidation |
| a regrouping publishes the generator classes it fails to preserve; tolerance is transfer, never waiver; understating refuses (`E0308`), overstating compiles | C | `finest-view`, `pin` (refusal), and the declared-not-computed shape is grounded on the `generic_const_exprs` ban (`ffl`, its second appearance outside transfer: a design shape forced by a forbidden feature) | 37 | current |
| mechanism price: 0.130 ms and 907 bytes per composition against 0.193 and 1854 | M | `finest-view` | 37 | current, single-checked; neighbour to, not answer for, the open real-consumer bench |
| evaluation strategy of a refusing operand's sibling | R | standards tilt strict (39); undecided | measured to change grade, no verdict | open, op's sentence |

### 3.8 The fold (`40:334-357`)

| claim | bin | grounded on | checked by | status |
|---|---|---|---|---|
| interior safety and total safety are two conditions, two promises | X | `two-safety`, `d69` (stated in radix-free value coordinates) | 33 stated, 34 split | current |
| biased MAC accumulator: zero-bias numeral, four-monomial gcd | X | `d69`, `bias-rational` | 34, and 42's three-rational gcd generalises the machinery it needs; 43's remainder numeral lands on it independently | current, triply corroborated |
| at interior safety all three grade components agree at once | X | `finest-view`, `two-safety` | 37 | current |

### 3.9 The multiplicative half (`40:359-384`)

| claim | bin | grounded on | checked by | status |
|---|---|---|---|---|
| biased product numeral: `bias = B1*B2`, `adjustment = gcd(A1A2, A1B2, A2B1)`, collapsing to the shipped rule at zero bias | X | `d69`, `bias-rational` (the formula's own algebra is what forced `bias-rational`, `40:474-476`) | 31, 33 at arity three with negative control; type-level halves now built by 41 (bias) and 42 (adjustment) | current |
| distributes-is-Monotone, split by totality and by IEEE's two lattice families | X | `finest-view` | 33, 34 split | current |
| no shipped preset is a dioid over `(max, +)`; the rung is derived, reports "no" with the failing axiom | X | `d47` | 33 | current |

### 3.10 The removals (`40:386-426`)

| claim | bin | grounded on | checked by | status |
|---|---|---|---|---|
| `Widening`'s three instances decompose into primitive choice, return numeral, `StoredWidth` | ratified | `widening-out` | 35, corroborated by 39's standards test (MATLAB `SpecifyPrecision` *requires* the removal) | current |
| the three-way symbol fold (direct, composite, widening multiply identical) at native and multi-limb width | M | `widening-out` | 35, +38 (my probe 1, on the new encoding) | current; the owed codegen regression test still does not exist |
| `Growth` leaves the key; `Op::IS_EXACT` makes the contradiction unstatable | ratified/C | `growth-out-of-key` | 35 | current |
| the Lattner gap becomes unstatable | C | `widening-out`, `growth-out-of-key` | 35 | current |
| `Growth` leaves `Policy` entirely | R | standards corroboration (39), no compile | 35 argued | open, tick 3, op's; file 38 already returned 37's premature upgrade of this to the R bin, and it stays there |

### 3.11 The value-unique encoding (`40:428-479`, ratified at `44b`), per tier

| member | bin | grounded on | checked by | status |
|---|---|---|---|---|
| `Pos`/`Nat` uniqueness by induction, no normalisation operator anywhere | C | `enc`, `vu`, and **the induction's closed-world hypothesis is the seal**, so: `seal-owed` | 36, +41 rebuilt, +42 | **conditional** (finding 4.2): by-construction status holds in the sealed copy nobody composes with and fails in the composing copy, by 42's compiled attack; `44b` says the seal does not gate the shape, and the grounding field is where that condition must stay visible until the seal lands |
| `Adjustment` coprimality enforced where observed (`E0271`); `Reduced` normalising alias | C | `enc`, `pin` (refusal) | 36, +41 | current, same seal condition (42's fabricated-`Pos` attack reaches it) |
| `Bias`: `BZero`/`BPos`/`BNeg`, rational, sealed, MATLAB witness representable, unreduced refused | C | `enc`, `bias-rational` | 41, +42 hardened one layer down | current, same seal condition for the layer below it |
| `Int ::= Z0 | Zpos | Zneg` | C (built) | **no current ground** | 36 | unknown grounding, finding 4.1 |
| the composition wall: `Reduce` as a bound diverges (`E0275`); spelled-out chain composes (`BiasMulGeneric`); recursion-limit raise crashes rustc (SIGBUS) | C/M | `pin`, and nothing else: the most pin-fragile results in the review, and the first grep the next pin bump owes is `grounded on:.*\bpin\b` | 41 found, 42 corrected the boundary and built the trait 41 said could not exist | current as corrected by 42; 42's own residual (`Reduce` diverges unprojected, a fresh identical trait does not) recorded, unresolved |
| prices: gcd 5.08, reduction 12.07, dyadic 0.50 (36); bias magnitude 13.61, full 19.10, dyadic ~1.55 (41); seal free, generic trait free (42) | M | `enc` | 36, 41, 42, mutually consistent at overlapping points within stated noise | current; every figure is a `pin + host` fact and none is a bench |

### 3.12 The assembled table and the tree (`40:481-536`)

| claim | bin | grounded on | checked by | status |
|---|---|---|---|---|
| rewrite cost near zero: no shipped source names `Adjustment`, `Numeral`, `FullRange`, `UTerm`, `AddWidth` | tree-grep | `tree` | 34 through 39, 41 through 44, re-verified this dispatch **with the command corrected** (finding 4.4) | current at this commit |
| identity-contract compiled path byte-identical to bare `wrapping_add` under the shipping build shape | M | `flags` (this claim exists *because* the flag discipline was corrected) | 32 corrected by 34's lesson | current, single-checked; the strongest erasure result and the fourth owed regression test |
| downstream contract, six-crate split, Stage G boundary | R/ratified | carried from 26 unchanged | 26 | current |

## 4. What the backfill caught

### 4.1 `Int` is a ratified tier with an empty grounding set

The ratified table (`40:487-488`) lists `Int ::= Z0 | Zpos<P> | Zneg<P>` with the comment "biases,
corrected to a normalised rational (39)". Read that comment against what was actually built and
ratified: the correction it cites is the one that *removed* `Int`'s only consumer. File 41's
`Bias` is `BZero | BPos<N, D> | BNeg<N, D>` over `Pos` pairs with sign carried by the constructor,
not by `Int` (`41:101-131`); file 42's hardening and `44b`'s ratification cover that shape. No
other member of the identity contract is `Int`-typed: precision, widths and exponent bounds are
`Nat` (`40:492`, `40:505`), and the one prospective consumer, the exponent-becomes-a-type fork, is
explicitly unopened (`40:690-691`). So the grounding query for `Int` returns nothing: it was
grounded on `Bias = Int`, which file 39 falsified and files 41/42 replaced, and the tier survived
into the ratified table as the spelling of a dead assumption.

This is the fourth instance of the dispatch's defect class, in the mildest possible form: not a
false claim, but a mechanism whose reason-to-exist moved out from under it, invisible because
nothing queried what it rested on. Two honest resolutions, and the choice is op's: label `Int` in
the table as forward-provision for the exponent fork, grounded on that open item by name, so the
next reader knows exactly what has to close for it to earn its place; or drop it from the ratified
statement until the fork opens, at zero cost, since dropping an unconsumed sealed tier breaks
nothing (verified trivially: nothing else in any probe imports it in a bounded position; 41 and 42
build `Bias` without it). I lean to the drop, because a ratified table is the one place the design
should carry nothing it cannot ground, and the encoding is three lines away in the audit trail if
the fork ever wants it. Suggested, not ruled.

### 4.2 Every "by construction" claim about the tower is conditional on `seal-owed`, and the field should say so

`44b` ratified the encoding and demoted the seal to "an implementation task, not part of the
ratified shape". That call stands and nothing here reopens it. But the grounding record has to
carry its consequence precisely, because the uniqueness induction's *hypothesis* is the closed
world the seal provides (`38:76-79`, my own ledger entry: "the seal is the perimeter and without
it the guarantee is void one crate away"), and file 42 compiled the attack that makes the point
concrete rather than formal: in the copy of the tower everything actually composes with, a foreign
`Pos` with a fabricated `Gcd` reaches a `Bias`-bounded position today (`42_probes/probe_2b`). So
the correct entry for the tower's bin-1 claims is not "current" but "conditional on `seal-owed`",
which is what section 3.11 writes. The day the seal lands in the composing source, one grep
(`grounded on:.*\bseal-owed\b`) finds every claim that silently strengthens from conditional to
unconditional, and 42's probe 2b flips to the regression test that keeps it there. This is the
grounding field doing exactly what it was adopted to do, in the forward direction: not only "what
went stale" after an overturn, but "what completes" after an obligation is discharged.

### 4.3 The float member of the overflow band should be struck, not carried

Section 3.5's table records it as `unknown grounding`, and I want the recommendation on the record
separately because it is the one place the backfill changes what the next consolidation should
*say* rather than annotate. A member with no derivation anywhere in 44 files, whose two nearest
siblings each needed correction when finally checked, is not "unverified"; it was never a claim.
Carrying it marked-unverified still lends it the sentence's authority. The open item that replaces
it already exists and is better scoped (the `Specials`-carrying model-float check, `43:319-322`,
which also gates the IEEE cause-split and reification sentences). Strike the member, keep the open
item.

### 4.4 A verification command was copied through four files and does not reproduce as recorded

Files 41 (`41:14`), 42 (`42:13-15`), 43 (`43:20-22`) and 44 (`44:16-18`) each record running
`grep -rln "Adjustment\|Bias\|Numeral" crates/ --include="*.rs"` "from the repo root" and finding
nothing. There is no `crates/` at the repo root; the shipped crates live at `mock/crates/`
(verified: `ls` at the root shows `docs`, `mock`, `target` and four files). Run as recorded, from
the stated directory, the command errors on a nonexistent path (exit 2) rather than returning an
empty match (exit 1, which file 42 explicitly records), so the recorded command-plus-cwd pair does
not reproduce; the plausible reading is that the runs happened from `mock/`, where `crates/`
exists, and the cwd prose is what drifted. The *conclusion* is true: I ran the corrected command
against `mock/crates/` this dispatch and it is genuinely empty (exit 1), likewise the
`FullRange|UTerm|AddWidth` grep behind `40:511`, and my own file 38 carries the same path shape
(`38:94`, `crates/arvo/tests/ui/` for what is actually `mock/crates/arvo/tests/ui/`), so I am in
the lineage I am correcting.

Small, and exactly on this dispatch's theme: a verification command is itself a claim, its
grounding is a cwd and a tool, and four files inherited one member's spelling as each file's own
fresh check, which is the conjunction-compression failure applied to evidence instead of to
content. The conclusion held by luck of the right kind (the check was real, its transcription was
wrong). The fix costs nothing: the canonical command is `grep -rln "Adjustment\|Bias\|Numeral"
mock/crates/ --include="*.rs"` from the repo root, expected exit 1, and the next member should run
that, not the inherited line.

### 4.5 The pin's identity is recorded inconsistently between the review and the workspace rule

Every probe in this review records `rustc 1.98.0-nightly (57d06900f 2026-05-27)`, and I verified
that is what the repo's `rust-toolchain.toml` pin (`nightly-2026-05-28`) actually resolves to on
this machine. The workspace rule (`workspace.md`, Rust toolchain section) states the same channel
is `1.98.0-nightly (cced03bfd)`. One channel date names one release; both hashes cannot be it.
The review's figure is the measured one, confirmed here and stated consistently across at least
five files; the workspace rule's hash is prose, and by this workspace's own provenance discipline
the measured record outranks it. Reported as an out-of-scope drift observation per standing
instructions: someone should reconcile `workspace.md`, and the registry (section 1.3) names the
pin by the hash rustc reports, because the hash is what the claims are grounded on and the channel
date is merely its name, which a bump edits.

### 4.6 The target was never named, and every instruction-count claim is a fact about it

`csel` is an AArch64 instruction. File 32's branchless-classify counts, the erasure results and
the fold measurements are facts about `aarch64-apple-darwin` (verified: `rustc -vV` host), and no
file in the review, my own 38 included ("this target", `38:137`, unnamed), states the target
identity anywhere. Nothing measured here says anything about x86_64, where the analogous
instruction is `cmov` and the vectorisation heuristics differ, and the stack's consumers will
build there. The registry row `host` (section 1.3) fixes this going forward; the four owed codegen
regression tests, when built, should be cfg-scoped or matrix-run per `arvo-always-optimal-
internals.md`'s Kind-1 discipline, which the design already mandates for exactly this reason.

## 5. What this file does not decide

**Whether `Int` drops from the ratified table or stays as labelled forward-provision** (4.1) is
op's, with my lean stated and not ruled. It is also two-expert-shaped: this file is the first
read, and a second member should form its own from `40:487-488`, `41:101-131` and the fork at
`40:690-691` before anything edits the table.

**Whether the registry's settled-shape rung (1.2) should exist at all**, or whether every ground
must be either op-ratified or physical, is a legitimate objection I considered and decided
against unilaterally resolving: without the middle rung, claims resting on `round-first` or the
`crossing` triple have no expressible ground until op ratifies those shapes by name, which would
convert the backfill into a request for a dozen new checkpoint lines. The middle rung with its
provenance marked seemed the honest compromise. A second reader may disagree; the registry is one
table and cheap to re-rung.

**The closed-form status of 44's dividing-quantum condition, the float model, the sibling-
evaluation sentence, `Precise`'s combinator surface, tick 3, and the four (now five, with 43's
fusion candidate) codegen regression tests** all stand exactly as open as `44b` left them; the
backfill annotates them and moves none.

**Retrofitting the `grounded on:` field into files 01 through 44 themselves** is not done and
should not be: the consolidation is the carried surface, the numbered files are the audit trail,
and annotating the trail would be re-deriving forty files' claims a second time for no query
anyone will run against superseded prose. The backfill covers what the next consolidation
inherits, which is what op's call names ("a member walks the current consolidation").

**Whether tier two (the mockspace registry namespace) lands now or at spec-graduation** is a
packaging call. Nothing in tier one has to change to adopt it later; the slugs are the rows.

## 6. Standing

The claim set in section 3 is the deliverable: every load-bearing claim in the third
consolidation, grounded per member against a closed registry, with the four physical grounds
attached by evidence bin rather than by hand, and with `unknown` written where the derivation
does not exist rather than where it was merely tedious to find. The dispatch predicted the
backfill would catch things and it did, in the milder register the review has earned by now:
no ratified call is touched, but a ratified table carries one tier grounded on a removed
assumption, the tower's by-construction claims are conditional on an obligation the same
checkpoint deferred, one conjunction member turns out to have never been a claim at all, and the
review's own verification record contains a four-file-deep inherited command that does not
reproduce as written. The two design questions come back answered in the direction the evidence
forced rather than the direction the adopted wording implied: the field is right and its
vocabulary is too narrow by three ground kinds, the pin bump being the overturn the narrow form
cannot query; and queryability is a closed vocabulary now, an existence-checked registry
reference the day the claims graduate into templates, with the one thing no tier can do (detect
an unwritten field) stated as the perimeter rather than assumed away. Each finding in section 4
carries the command that re-verifies it. The next consolidation can take section 3 as its
annotation layer and sections 1 and 2 as the convention's statement; nothing in either needs
transcription, which was the shape this dispatch asked for.
