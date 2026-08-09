# 98. How complete is the canon: the core is covered and mostly attacked, the open list is small and sorted, and the remaining ignorance is at the periphery, named

Donald Knuth, file 98. I wrote file 01 (the mathematical rigour of the founding identity), file 39
(whether the normalised encodings still represent the numbers), and file 62 (the verification
backlog). This file answers the lead designer's own waking question: how far along the design and
spec are, how full and complete the canon is. Not a progress report; an assessment of coverage,
worked against the criterion the dispatch states and I adopt as the measure throughout: **a canon is
complete when a competent implementer can build the thing from it without asking a question the
canon does not answer.**

## What I read

`91_consolidation_nine.md` in full, the standing base. Every deliverable since it in full: `92`,
`93` (through its factual-check section; its adoption I took from `95b` and `97`), `94` (through its
gates and verdict), `95b`, `97`. Under the dispatch's wider licence: `79b` in full, `86b` in full,
the section skeletons of `68b`, `70b`, `74b`, `77b`, `82b` (headings and ruling lines, grepped, not
full reads; each ruling I use below is also carried in a consolidation's section 2, so no judgement
rests on the skim alone), `72_giesen_the_unexamined_ground.md` sections 1 and 2 in full (the
review's own coverage map, which this dispatch is largely a re-run of, twenty-six files later), and
`78_consolidation_eight.md` at sections 1.18, 1.19, and the external-images passage (`78:495-556`).
One `ls` of the panel directory, current through `97_probes`. One `ls` of `mock/design_rounds/`:
the formalization round's three topic files are the design record's live tail, and the panel
directory is where the content lives. Shipped source in two places, both factual checks recorded
below, neither read for meaning.

## Gates

Canon gate, run fresh from the repo root: `grep -rln "Adjustment\|Bias\|Numeral" mock/crates/
--include="*.rs"` and the same with `FullRange\|UTerm\|AddWidth`, both exit 1, empty. This dispatch
is the lead designer's own question, put directly; the work is licensed. HEAD is `4232447` (the
file 97 commit).

Test gate: `cargo test --offline --workspace` from `mock/`, summed across every `test result:`
line: **155 binaries, 672 passed, 0 failed, 9 ignored.** That is 97's 672 passed across two more
binaries than 97's 153, and six tests and six binaries past the standing 666/149 at `91:43-44`. The
drift is attributed, not absorbed: the concurrent dispatch that owns `mock/benches/` and the
manifest this round has uncommitted work in the tree (`git status`: modified `mock/Cargo.toml`,
`mock/benches/*`, untracked `bitpack-footprint-*` artifacts), which is the same attribution file 97
made and which has grown by two binaries since 97 ran. On the committed tree the standing count
holds. The one disqualifying test on record, `arvo-tensor/tests/capacity.rs:14-18`, I re-read at
source this session: `const CAP: Cap = cap(N)` at `src/capacity.rs:48`, asserted against `cap(3)`
in the test, still `assert_eq!(cap(3), cap(3))` after one substitution, still counted in the green
total, now **twenty-one files** since it was flagged. `95b` ruled the disposition (op's word, one
trivial commit outside the panel); I add only the count. Toolchain `rustc 1.98.0-nightly (57d06900f
2026-05-27)`, `aarch64-apple-darwin`, confirmed inside the tree. No probes were needed; this
dispatch's evidence is the record itself, checked by grep and read, and the gates above. Everything
below separates what I verified (a command, a line, a re-read) from what I reasoned, per claim.

---

## 0. The answer, stated first

**The canon's core is covered, and most of the core has survived attack, not merely been written
down.** By the implementer test, the chapters standing in consolidation nine (sections 1.1 through
1.27) answer what a number is, how it is identified, encoded, quantised, folded, multiplied,
divided, stored, crossed to bytes, digested, displayed, parsed, printed, and refused, and they
answer it with the terms defined, the failure cases classified, and the refusals located at
declaration sites. An implementer building the numeral tower, the four presets, the fixed-point and
float models, the byte image, and the digest would, on my reading, not hit a question the record
does not answer, with the specific exceptions itemised in section 1.

**The open list is small relative to the covered area, and it sorts cleanly into three kinds**:
roughly ten mechanical artifacts (single compiles, citations, probe re-runs, each already specified
by name), roughly ten bounded design items (second reads and one derivation, each with a named
closing artifact), and roughly ten decisions that are the lead designer's own, which are not gaps
in the canon but decisions awaiting their decider, each with prepared alternatives. The arithmetic
is in section 3.

**The remaining genuine ignorance is at the periphery, and it is named, not diffuse**: five or six
crate-level subjects from the review's own file-11 blind-spot map that no panel file has ever
examined (shape, geometry, platform, the float packaging boundary, the predicate concept, the PRNG
half of pseudorand), plus one operation family (elementary functions past division) whose failure
kind is classified but whose correctly-rounded story is underived. Section 4 sizes them against the
historical range: the last time this review opened an unexamined area, the outcomes ran from
"closed on inspection, no action" (signed zero) to three full consolidation sections (the
external-images cluster). That range is the honest error bar on any completion estimate.

**One structural caveat governs everything above.** The last two stretches' ratifications, the
counting split, the division dissolution, the entire perimeter block, and the requirements' owner
and moment, are **persona-decided** (`90b`, `95b`), every one strikeable on op's word. The
completeness reported here is conditional on the morning read this file is written for: confirm
`90b` and `95b` and the core stands as described; strike either and sections 1.14 or 1.22 of the
consolidation reopen.

---

## 1. The implementer's questions, one by one

The dispatch names the questions a competent implementer must be able to answer from the record. I
take each, with a verdict from the three-valued scale the dispatch sets: answered, partially
answered, not answered. Per the dispatch's warning, "partially" is applied strictly: a ratified
answer whose terms are undefined is not an answer, and the record's own definitional-completeness
line (adopted `90b`, given its owner and moment at `95b`) exists because that kept happening.

### 1.1 What a number is, and how to spell one

**Answered, and survived attack.** The founding identity (fixed point and float as one
formalisation differing in the exponent function) has stood unchanged since file 40 through nine
consolidations (`91:147-149`). The identity contract (`Radix` sealed as `Rad<P>`, `Bias` and
`Adjustment` sealed, value-unique, gcd-normalised, `91:153-154`) and the four-member `Numeral`
contract with the assembled trait table (`91:698-745`) are the spelled-out answer. Spelling: the
notation vehicle and the `NumeralFace` layer (`91:480-481` unchanged; `78:336-339`, all three
residuals from the sixth consolidation closed), with parse specified as `quantise ∘
rational-of-digits` and print as the shortest correctly-rounded round trip, both compiled at model
scale (`78:496-517`). Attack history: the tower's seal has been attacked by enumerated introduction
route twice (file 46, and again at the niche vocabulary in file 92), the encoding's
representational completeness once (my own file 39), and the value-uniqueness theorem re-read twice
this stretch (`91:184-195`).

### 1.2 What operations exist, what each returns, and what happens when a result does not fit

**Partially answered, and the partial region is exactly namable.** What does not fit is the
best-covered half: the three-kind failure taxonomy (`91:420-429`) is ratified, the `Resolution`
axis is read as a totalisation axis, every preset has a definite `OverRange` row (`91:293-294`),
and division's `x/0` is now resolved by dissolution with the fallback's arity fixed at two
(`95b:41-73`), subject to 97's still-one-pass slot-domain sentence (`97:352-379`). Per-operation
coverage: the fold (1.8), the multiplicative half (1.9, files 24/25), division (1.13, file 43
re-verified this stretch plus 93/95b), `quantize` in both forms (1.16), parse and print
(`78:495-517`), comparison (`TotalOrd` split, 1.16). Each of these states its result numeral, its
growth class or accumulator width, and its grade sites.

What is missing, verified by grep across the corpus: **`Sqrt` and `Recip` have no chapter.** They
appear in the failure taxonomy as Kind 2 generators (`91:425-426`) and nowhere else since file 72
flagged exactly this with stated low confidence (`72:73`: the correctly-rounded story for
irrational results, the table-maker's dilemma at arbitrary radix and precision, "surfaced in no hit
I read"). My grep this session confirms nothing has landed since: the only post-72 hits are the
taxonomy sentences (files 84, 91, 93 in failure-kind context). And there is no closure sentence: no
ratified text says "these are all the operations," so an implementer cannot distinguish "Sqrt is
deferred" from "Sqrt was forgotten." That closure sentence costs one line and the
definitional-completeness line arguably already demands it of whatever text ratifies the operation
surface. The Sqrt/Recip chapter itself may be small: the design may legitimately scope them as
non-correctly-rounded with a stated licence and a named verifier, which is one decision plus one
derivation, but that is my estimate, not a finding.

### 1.3 What the laws are, what they are keyed on, and what a published fact means

**Answered, with one held call that is op's own.** The largest hole this review ever found in its
own laws, the grade ratified as an algebra over an undefined generator set, is closed: the site
count and the moved count are two facts at two layers, `Folded<N>` is unambiguously the site count,
the moved count rides beside the value behind `Door = Quantised`, and multiplicities live in the
monoid under an idempotent join (`91:331-408`, two-read confirmed, persona-ratified). Law keying at
the value layer, the fold's interior-safety conditions, and the digest's own grouping-invariance
law (reusing the multiplicative half's exponent-offset argument, `91:220-233`) are all stated. The
one live piece: **division's grading axis is op's own, held since checkpoint ten** (`95b:75`), a
decision awaiting its decider, not a gap.

### 1.4 How a value is stored, how it crosses to bytes, comes back, and what is guaranteed

**Answered; the strongest area of the canon, and the most attacked.** Three width levels and one
declared axis (`91:531-543`), three maps with three statements (0, P, C; `91:544-577`), zero
inter-value padding as a theorem under `Bitpacked` (`91:563-570`), the digest contract at two
stopping points (`91:628-661`), the mutation perimeter quantified per byte-owner and per level with
the door-typing rule (`95b:108-127`), and the niche vocabulary sealed, narrowed to the `NonZero`
family, its trusted base shrunk to one sentence (`95b:110-118`). The guarantee's own scope is
stated rather than implied: same-process, same-build-target, cross-target portability explicitly
out of design scope with its constituents named const-derivable (`91:684-687`). Nearly all of this
block is now two-read material; the residue is 97's four one-pass findings (section 3 below) and
the persona-tier caveat from section 0.

### 1.5 How the presets differ and why

**Answered, on op's own checkpoints, which matters given this area's history.** Both preset tables
ratified at `70b` after the `68b` regression voided two rows and forced re-derivation from op's
verbatim statements; `StoredWidth`'s reading forced three ways from ratified material (`91:500-515`);
the hardware-reachability theorem with a now-checkable precondition (`91:517-520`); the bitpack
price corrected on a rebuilt harness and ratified at `82b` (1.50x and 1.29x, not 4.6x). Two open
pieces, both named: `Hot`'s default float environment (op's own, held three times), and **`Cold`'s
footprint intent, which no artifact yet prices**, the bandwidth-contention bench being the one open
item the record has twice ranked top of queue (`95b:151-155`) and which the concurrent dispatch's
uncommitted tree suggests is being built as I write. Until it lands, the canon asserts `Cold`'s
reason to exist on the workspace rule's authority rather than on a measurement, and the record says
so out loud rather than hiding it, which is the honest state.

### 1.6 What the crates are, what each owns, and which may depend on which

**Partially answered, and this is the widest genuine gap.** The eleven-row taxonomy has survived
since file 74 with five rows substantially enriched this stretch (`91:755-774`), the bottom-carrier
crate proposal is two-read complete, and the dependency direction is inherited from the design
round's layer discipline. But the rows are, by the consolidation's own standing practice, flagged
suggestions awaiting op's per-row confirmation (`91:772-774`, open item at `91:1028-1029`), and,
more materially, **the review wrote its own blind-spot map at file 11 and has still not exhausted
it**: `arvo-shape`, `arvo-geom`, `arvo-platform`, `arvo-float`'s packaging boundary, the unified
predicate concept, and `arvo-pseudorand`'s PRNG half remain panel-unreviewed (`11:44-56` per
`72:52-60`; `91:769-770` confirms nothing this stretch touched their dispositions, and
`91:766` marks pseudorand "still panel-unreviewed as a crate in its own right"). Section 4 sizes
these. An implementer of the *numeric core* does not need them; an implementer of *arvo* does.

### 1.7 What the type system refuses and why

**Answered, and this is the area where "survived attack" is strongest.** Eleven seal firings, each
an enumerated-route attack with distinct compiler error classes; the declaration-site refusals
(level ordering `E0080`, empty-capacity trait bound, niche totality, no-wrap, the coverage
assertion on the print buffer); the forbidden integer door on niche carriers; and the compiled
refusal that a moved count cannot inhabit `Folded<N>` (`E0435`), which is itself the evidence for a
design ruling. The verification mandate (`79b:53-57`) will convert these into compile-fail pins in
the implementation phase, and the record already names which pins are owed.

### 1.8 What the design promises, what it merely assumes, and where the boundary runs

**Answered as a mechanism; partially answered as an artifact.** The provable-versus-trusted
sentence is ratified for the design's own text (`91:620-626`), and every trusted-base entry I can
find is named where it is introduced: the hand-laid `unsafe impl Crosses` entries (`80:104-108` via
`91:172-176`), the niche vocabulary's one unreachability sentence (`95b:116-118`), the raw door's
postcondition, the consumer hand-over entry (97's terminating clause), `IeeeDefault`'s deployment
residual with its three artifacts (`91:853-867`), and the standing toolchain trust 97 made
explicit. What does not yet exist is **the assembled register**: one place an implementer reads the
complete trusted base as a list. The entries are recoverable by reading sections 1.4, 1.12, 1.22,
and 1.27, which is precisely the kind of recoverability the review has learned not to trust
(distributed statements of one thing are where its two-organs defects lived). Assembling it is a
consolidation-ten-sized task, not a design task, and I suggest it as one. Additionally, 97's
cannot-check versus cannot-provide reversal (`97:291-346`) sits exactly on this boundary and is
one-pass; until it is adjudicated, the boundary's exact position at uncheckable environment fields
is stated two ways in the record.

---

## 2. Coverage is not settledness: the tiers, distinguished mechanically

The dispatch asks that answered be separated from answered-and-survived, and the record supports
doing it mechanically rather than by impression. Three live tiers:

| Tier | What puts a claim there | Core content currently on it |
|---|---|---|
| Op-ratified, post-ratification attack survived | An op checkpoint plus a later adversarial read that reproduced or corrected it | The founding identity, the seal (46 then 92), statements 0/P (80 then 82/87/92), both preset tables (re-derived at 70 after the 68b regression), the bitpack price (rebuilt at 81/82), quantize (84 then 85/86), division's surface (43 re-verified, then 93/95/97) |
| Persona-ratified (`90b`, `95b`), strikeable | A checkpoint op delegated while asleep | The counting split, division's dissolution and two-arity fallback, the naming principle, the entire perimeter block, the requirements' owner and moment |
| Panel-converged, checkpoint-silent | Two independent reads, no checkpoint yet | Capacity's full resolution (`91:819-821`, explicitly awaiting op's word), the bottom-carrier crate, 97's confirmed repairs |

The reason the separation earns its space is the overturn record, which I re-checked at the cited
lines rather than recalling: `74b` ratified a bitpack multiple wrong by a factor of three and a
`Bitpacked` reading op himself reversed at `77b`; `68b` voided two preset rows a prior stretch had
carried as settled; `67b`'s naming principle died at `90b`; `90b`'s own division instinct died at
`95b` eight files later; and consolidation nine's claim to have performed the completeness line was
found false at three of its own sentences (`95b:11-14`). **Ratification is a provenance fact, not a
correctness fact**, and the record's honest strength is that its second-read convention keeps
finding these, at an accelerating rate (the persona checkpoint notes the second reader being caught
by the third as "the discipline working"). For the completeness question this cuts both ways:
coverage is higher than the open list suggests, and certainty is lower than the ratification count
suggests, with the two-read tier the reliable floor.

---

## 3. The arithmetic: the open list, sorted and counted

Everything below is drawn from `91` section 4, updated by what `92` through `97` closed, opened, or
reversed. Per `82b`'s discipline every item carries a named closing artifact in the record; I
restate the sort, not the artifacts.

**Closed since consolidation nine** (verified against the closing files): the `NicheCarrier` seal's
second read, the mutation repair's second read and combined case, division's `x/0` fork, the naming
refinement's second read, the x86 receipt form, the bench orchestrator's overwrite defect (fixed at
`5dae109`, verified by 92), and five of `95b`'s working shapes second-read by 97 (the terminating
clause, the rename, "named in the record", the relocation, the requirement-performance clause,
though 97's repairs to two of them are themselves new one-pass material).

**Mechanical, panel-owed** (each a single compile, citation, or re-run, already specified): the
signed halves of division probes 2, 4, 5; the float-division compile against a `Specials`-bearing
numeral; the `foldnum` compile against the real four-member contract (owed since file 78); the
non-default `Canonical` compile; the constructive-extensibility compile (named owed by three files
in succession, still unperformed, and it is the cheapest remaining check on a load-bearing claim,
that the tower is third-party-extensible constructively and not only sealed adversarially); IEEE
clause 7.6 and §5.12 primary-source quotations; the ISA primary citations file 93 opened; the
compile-fail pin for the entry-level totality refusal; the x86 and RISC-V verdict-split receipt
forms. **Count: ten. Aggregate size: days of dispatch work, not stretches**, because each names its
artifact and none has a design question inside it.

**Design-shaped, panel-owed**: second reads on 97's four one-pass findings (the foreclosed-region
retirement, the cannot-check/cannot-provide split with the verdict-side receipt, the per-lowering
keying, the holes-not-shadows slot domain; the first and second of these reverse adopted material
and are the priority per 97's own ranking); the required-field relation derivation (97's one new
open item); the digest chapter's type-level history split (its own dispatch); and the six items
untouched since the eighth consolidation (the reduction firing site and `FullRange`'s survival,
dither versus `Refuse`, `SC_WRAP<n>` with `n_bits > 0`, richer canonicalisation's branchlessness
with cross-word bitpacked extraction, `DatumDeterministic`, and the `Gcd`-coherence question
confined to the bottom carrier); plus the `notko-hlist` binding-time sentence; plus the `Cold`
footprint bench, in flight. **Count: about twelve, of which the six eighth-consolidation items have
sat two stretches without motion**, which is the one place the open-list discipline is aging rather
than closing. None of the twelve blocks the core chapters; several (dither, `SC_WRAP`,
`DatumDeterministic`) look one-file-sized on their face, though this review's history says such
estimates are worth little until the file is dispatched.

**Op's own, decisions awaiting their decider, counted separately as the dispatch instructs**:
`Hot`'s default float environment; division's grading axis; the seven upper vocabulary members'
reading (the two symmetric offers at `91:197-204`); D39's final hardening; capacity's ratification
(two-read complete, nothing further owed from the panel); the eleven crate rows' per-row word; the
workspace perimeter rule's provable-versus-trusted clause (lean recorded); the three
`unstable-features.md` wording edits; the `FromConstant` vehicle; the capacity tautology's deletion
(one commit, outside the panel); and, above all of these in consequence, **confirming or striking
the two persona checkpoints**, which is the single act that moves the largest volume of material
between tiers in one stroke. **Count: ten or eleven. None is a gap in the canon's coverage; every
one has its alternatives prepared and its consequences priced in the record.**

The honest summary arithmetic: against roughly twenty-seven standing chapters (consolidation nine's
sections 1.1 through 1.27, most multi-claim), the panel-side open list is about twenty-two items of
which ten are mechanical, and the decision queue is about ten items of which one (the persona
confirmation) dominates. **The canon is much closer to complete than a raw count of open-list lines
suggests**, because the list's own discipline (every item names its closing artifact) means the
items are small by construction; anything large left on it would be visible as an item with no
artifact, and I found none.

---

## 4. The unexamined ground, named and sized

Where a whole area has never been examined, the dispatch asks which and how large it looks. The
review already owns the method here: file 72 derived a category list from outside its own history
and swept the corpus. Twenty-six files later, my re-check of that table:

Three of its unexamined rows are now the canon's best material: text (parse/print/display, folded
at consolidation eight), the byte boundary (statements P and C, the three levels), and the digest
(the two stopping points). Signed zero was checked and was already closed. That conversion rate is
the strongest single argument that the remaining rows should be swept before the canon is
earmarked, because the same sweep that found them found nothing else at the core: **the review's
own outside-derived category list is, at the numeric core, exhausted.** What remains:

- **Elementary functions past division.** `Sqrt`/`Recip` classified but underived (section 1.2
  above). Plausibly one decision (correctly-rounded or licensed-approximate, with the named
  verifier the naming principle now requires) plus one growth-class derivation. The table-maker's
  dilemma makes the correctly-rounded branch genuinely hard; the licensed branch is cheap. Which
  branch is op's kind of call.
- **The PRNG half of `arvo-pseudorand`.** Uniform sampling over a numeral's value set is a real
  spec question (uniform over values and uniform over data diverge the moment the grid is
  non-uniform, `72:72`), and it now has the digest chapter to stand on. One dispatch, I estimate,
  with the usual error bar.
- **`arvo-shape`, `arvo-geom`, `arvo-platform`, `arvo-float`'s packaging boundary, the unified
  predicate concept.** Design-round dispositions the panel has never reviewed (`11:44-56`). These
  are consumers and packagings of the core rather than extensions of it, so the likely outcome
  per row is closer to signed zero (checked, closed) than to the external-images cluster (three
  sections), but the file-11 table has now been carried unexhausted through eighty-seven files,
  and the cheapest way to make this section of my assessment false is to run the sweep.
- **The integer-saturating SIMD lane residue** (`72:70`), known-open since file 20, unmoved.

What I did not find, and looked for: any unexamined area *inside* the core chapters. The categories
a number's whole life needs (computed, written, read, stored, loaded, compared, hashed, displayed,
generated, extended) each now have either a chapter or a named open item. The extension row's
constructive half is the one core claim still resting on an unperformed compile, and it is on the
mechanical list above.

---

## 5. The two requirement performances, on this text, before it stands

**The definitional-completeness line, performed.** Terms this file introduces, with dispositions:
*mechanical item* (an open item whose named closing artifact is a single compile, citation, or
re-run, defined section 3), *design-shaped item* (one whose artifact requires a judgement or
derivation, section 3), *op decision* (an item whose closing artifact is op's own word per `91`
section 4's own labels, section 3), *unexamined area* (a subject with zero panel deliverables,
established by the `11:44-56` map plus file 72's sweep method plus my greps, section 4),
*coverage* and *settledness* (section 2, distinguished by provenance tier and by presence of a
post-ratification adversarial read). The implementer-completeness criterion itself is the
dispatch's, quoted verbatim at the top, not coined here. No vocabulary member is defined; this file
defines no types. Checked by grep over this file's starred and quoted terms.

**The separation requirement, performed.** This file's one model is the three-valued verdict scale
of section 1. What it separates: *answered* from *answered with undefined terms*, nonvacuous at the
grade (the generator table was ratified for forty files while "event" was undefined; before file 89
the scale would have read "answered" and been wrong, after it the same chapter reads "answered" and
is right), and *covered* from *settled*, nonvacuous at the bitpack multiple (covered and ratified
at `74b`, wrong by 3x; settled at `82b`). At any chapter with no history of term-gaps or overturn,
the scale's distinctions are vacuous and my verdicts there are correspondingly weaker evidence,
which is why section 2 lists the tier of every core chapter rather than letting section 1's
verdicts stand alone.

**The honest limit of both performances**, inherited from 97's finding: they verify that my terms
are placed and my scale has content; they do not verify that my verdicts are correct. This file is
one reader's assessment of an 800-plus-page record, built from the consolidations, the deliverables
since, the checkpoints, and targeted greps, not from a full re-read of ninety files. Where a
chapter's status I report rests only on a consolidation's "unchanged" line (radix ten, the cost
model, the algorithm crates, the notation residuals), I have verified the line exists and traces,
not re-derived the chapter, and I say so here rather than let the verdict table imply otherwise.

## 6. What this file leaves open

This assessment is one-pass, like everything else on its first day. Its most consequential
judgements, that the core has no unexamined interior and that the open list contains nothing large
in disguise, are exactly the kind that a second reader with a different category list could break,
and the cheap way to attack them is file 72's method with fresh search terms rather than mine. Its
most useful successor artifacts, in order: op's confirmation or striking of `90b` and `95b` (which
moves more material than any dispatch can), the file-11 sweep of the five periphery rows, the
operation-surface closure sentence, and the assembled trusted-base register.

Only op's calls are final, and even those go stale. Everything above is evidence and suggestion.

*Grounded on: ratified (`70b`, `74b`, `77b`, `82b`, `86b`, `79b`, and the persona-tier `90b`/`95b`
as marked throughout), settled shapes (`91` sections 1.1-1.27 and 4, `92`, `93`, `94`, `95b`, `97`,
`72:52-75`, `78:336-556`, `11:44-56` via `72`), measured (the canon greps and the workspace test
run, commands inline, run fresh this session), verified at source
(`arvo-tensor/tests/capacity.rs:14-18` against `src/capacity.rs:48`, HEAD `4232447`, the
uncommitted manifest state, attribution only), reasoned (the verdicts of section 1, the tier
assignments of section 2, the sort and counts of section 3, the sizings of section 4, mine, offered
as an assessment and not as a ruling).*
