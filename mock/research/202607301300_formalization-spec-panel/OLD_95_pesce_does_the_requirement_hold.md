# 95. Does the requirement hold: one of the three is the separation requirement working, and the other two are the requirement nobody has ever performed

Angelo Pesce, file 95. I wrote file 30 (the identity half assembled), file 65 (pricing the L0
migration), and file 82 (the stretch assembled). This file is the last before a checkpoint, so it is
written to be read by the lead designer rather than by the next member, and it tries to close more
than it opens.

Two jobs. Attack the three one-pass files since the ninth consolidation, and then answer whether the
separation requirement adopted at `86b` actually catches the three new instances this stretch
produced, or needs something it does not have. I will give the second answer first, because it turns
out to change what the first job is worth: **the three instances are not three tests of one
requirement. One of them is that requirement working exactly as designed. The other two are a
different requirement, adopted at the same checkpoint, which the ninth consolidation claims to have
applied to everything it absorbs and demonstrably did not, because it has no moment at which anybody
performs it.** The response to "the requirement is insufficient" is a new requirement. The response
to "the requirement was never run" is a clause naming when it runs. The evidence supports the second,
and it is a fraction of the cost.

**What I read.** `91_consolidation_nine.md` in full, the standing base, twice: once for content, once
against its own citations. `92_spj_the_perimeter_second_reads.md`,
`93_lattner_the_zero_divisor.md`, `94_muratori_what_a_name_promises.md`, each in full, and each
one's probe sources rather than its OUTCOMES file where a claim of mine rests on them
(`92_probes/probe_2`, `92_probes/probe_4`, `94_probes/probe_3`). `90b_persona_checkpoint_
twentytwo.md` in full, which set all three and is a persona's rather than op's, so every ratification
in it is a line for him to strike. One `ls` of the panel directory, current through `94_probes`.
Behind those, targeted reads to check one claim each before reasoning from it: `78:405-445` (both
ratified preset tables, read at source rather than through file 94's transcription), `89:475-486`
(how the separation requirement was actually applied when it first fired), and the workspace rule
`what-you-can-observe-is-what-you-guaranteed.md` for its item 1.

**Gates.** Canon gate, fresh from the repo root: `grep -rln "Adjustment\|Bias\|Numeral" mock/crates/
--include="*.rs"` and the same with `FullRange\|UTerm\|AddWidth`, both exit 1, empty. Test gate:
`cargo test --offline --workspace` from `mock/`, summed across every `test result:` line, **149
binaries, 666 passed, 0 failed, 9 ignored**, matching `91:43-44`. I read bodies rather than names in
the surface my probes touch: `mock/crates/arvo-tensor/tests/capacity.rs:14-18` against
`mock/crates/arvo-tensor/src/capacity.rs:48`, and it substitutes to `assert_eq!(cap(3), cap(3))`,
exactly as registry item 14 and file 94 section 0 both say. It is counted in the 666. It is out of
this panel's scope to delete and I am not going to pretend otherwise, but I will say the thing file
94 said and say it once more so it is on the record twice: a fabricated pass carried through nineteen
files of writing about verification discipline is a standing embarrassment, and "waiting for the
implementation phase's first honest red commit" (`91:957-958`) is a reason to delay a deletion that
takes one line. Toolchain `rustc 1.98.0-nightly (57d06900f 2026-05-27)`, `aarch64-apple-darwin`,
confirmed inside the tree immediately before the first compile.

**What is compiled, what is reasoned, and which numbers are which.** Four probes in `95_probes/`,
built and run fresh this session, commands and stdout verbatim in `95_probes/OUTCOMES.md`. Probes 1,
2 and 3 execute; probe 1b is a compile-fail and its `E0080` text is quoted in full. Every count in
this file is a compile outcome, an executed program's stdout, or a `grep` whose command is written
next to it. **No bench was run.** The orchestrator's artifact-destroying defect is fixed at `5dae109`
(a section filter, 25 lines to `mock/benches/src/main.rs` and nothing else, verified by
`git show --stat`), but the by-reference input path is not in that commit, so the one measurement
`Cold`'s footprint intent actually needs is still unbuildable and no timing claim appears below.

**The method constraint, applied.** I read shipped source in three places: the capacity test and its
source (a factual check of a claim the record already makes), the two ratified preset tables at
`78:409-441` (a research artifact, not the tree), and `mock/benches/src/main.rs` (a factual check of
file 94's staleness finding). Every judgement below survives deleting all three.

---

## 0. The verdict, stated first

**On the three one-pass files.** All three are good work and all three have a defect of the shape
this review keeps finding, which is not a coincidence about their authors. File 92's door amendment
is tested at the one instantiation where its claim holds, and the instantiation where it fails is
compiled in the same file, in a different section, for a different argument; I have put both under
one program (`95_probes/probe_1`). File 93's verdict is right and its ground is one clause weaker
than a ground already ratified in the sentence it cites and half-quotes; and its own repair collides
with its own `0/0` closure. File 94's replacement test cannot fail in the hands of the person running
it, which is the defect it correctly identifies in the parity suite it strikes, and its prose
contradicts its own probe on the preset it leans on. Three cross-file collisions land besides:
file 92's vocabulary narrowing creates precisely the naming divergence file 94 then defines a test to
catch, file 93's deletion test is the procedure for a ratified sentence rather than a new one, and
file 94's artifact strike removes one artifact where its own probe 2 supplies the reason to add
another.

**On the requirement.** It catches one of the three cleanly, half of another, and none of the third,
and that is the requirement working rather than failing, because the two it does not catch are not
sampling failures and it is a rule about sampling. Both of them are caught by the
definitional-completeness line adopted in the same checkpoint. That line has fired **zero** times in
advance across the corpus, because nothing names who performs it or when, and three findings have
been taken from it in hindsight this stretch at the cost of roughly one file each. **The design does
not need a third requirement. It needs the two it has to have a moment, and one word added to one of
them.** The one thing genuinely uncovered, a model whose domain is complete and whose subject is
wrong, is not requirement-shaped at all, and minting a requirement against it would be paying on
every file forever for a failure no per-file check finds.

---

## 1. Cheap factual checks, performed before reasoning from anything

Six, and four of them needed something.

**`91:12-13` is false as performed.** The consolidation names as its own deliverable "the
definitional-completeness line and the separation requirement applied to everything this document
absorbs". Applied to its own ratifying text, the line fails at three sentences, all checkable by
grep:

- `91:612-615` (the mutation repair, ratified as a working shape at `90b`) uses **"safe surface"** and
  **"raw accessor"**. `grep -rn "safe surface" *.md` returns fourteen hits across files 87, 88, 90b,
  91 and 92; `grep -rn "safe surface is\|safe surface means\|safe surface denotes" *.md` returns
  none. `raw accessor` has eleven hits and no definition. Both terms are undefined and neither is
  named open.
- `91:846-850` (the refined naming principle, ratified at `90b`) uses **"designated"**. Four hits
  across `90b` and `91`, no definition. This is the ambiguity file 94 section 4 found by walking the
  name set, three files later, and it was a grep away on the day the sentence landed.

Two of the three files under review found two of these three by attack. The line would have found
all three by a mechanical read of its own document.

**File 93 cites `91:158-159` and quotes the weaker half of it.** It writes: "The guard that patrols
this boundary is 'no law may read `Lowering`' (`91:158-159`)", then shows that guard satisfied
vacuously (`93:206-211`). Read at source, `91:158` is: "`Lowering` changes no value; `Encoding`,
nested inside it, may change which datum carries a value. No law may read `Lowering`." The first
clause is not vacuous and it forbids the Door cell outright, with no deletion test and no new
sentence. Section 2.4 below.

**File 94's prose contradicts file 94's probe on `Cold`.** Its probe output, quoted in its own file
at `94:155-158`, reports `Cold` as differing by number kind on out-of-range and on `Door`, and
`Precise` as differing on `Door` alone. Its prose at `94:172-174` then says "`Cold` and `Precise`
agree on every cell a spot-check reads (in-range direction, out-of-range), differing only on
`Door`". Read at source, `78:414` gives `Cold`'s fixed-point out-of-range as clamp and `78:437` gives
its float out-of-range as far point, and `94_probes/probe_3:102,109` models `Clamp` and `FarPoint` as
distinct variants. The probe is right and the prose is wrong. The finding survives with a smaller
coincidence surface: one confirming preset, not two.

**Three things reproduce.** The test count, the canon gate, and file 94's staleness finding about the
bench harness (`5dae109` closes the section-filter half of the closing artifact at `91:1025-1027` and
not the by-reference input path). I re-derived the preset-table facts file 94 rests on from
`78:409-441` rather than from its transcription, and they hold: `Hot` is `TowardNegative` on fixed
point and `ToEven` on float, `Warm` is doubled on fixed point and minimum on float.

*Grounded on: settled shapes (`91:12-13`, `91:158`, `91:612-615`, `91:846-850`, `94:155-174`,
`78:409-441`), measured (the four greps and `git show --stat 5dae109`, commands in
`95_probes/OUTCOMES.md`), reasoned (the readings, mine).*

---

## 2. Attacking what is one-pass

### 2.1 File 92's door amendment is tested at the instantiation where it holds, and its counterexample is in the same file

File 92's binding amendment is that a `NicheCarrier` lowering's mutable door is typed at the niche
member, and that "every store through it is a safely-constructed `NonZeroU16`, which cannot be zero,
so the soundness obligation returns to the type system and vanishes from the caller entirely"
(`92:296-299`). It is compiled at `92_probes/probe_4`, whose carrier is:

```rust
struct Biased(NonZeroU16);                    // 92_probes/probe_4:19
fn value(self) -> u16 { self.0.get() - 1 }    // 92_probes/probe_4:25
```

`Biased` carries no domain bound. `value` is total over every inhabitant, so the modelled numeral's
domain **is** the niche's full inhabitant set and the door is domain-total by construction. The
amendment was checked at the one instantiation where its distinction is vacuous.

File 92 compiled the divergent instantiation itself, in section 1.3, for the unrelated argument that
the audited entry over-collects: "a bounded numeral of 2^13 values biased into NonZeroU16 leaves
(2^16 - 1) - 2^13 = 57343 inhabitants with no decode" (`92_probes/probe_2:40-41`). It never carried
that back to the door. Under one program (`95_probes/probe_1`, both sweeps walking all 65,535
inhabitants, no sampling):

```
A (domain fills the niche): CARD=65535 inhabitants=65535 decoded=65535 orphaned=0     door_domain_total=true
B (bounded numeral, 2^13):  CARD=8192  inhabitants=65535 decoded=8192  orphaned=57343 door_domain_total=false
B: after one safe store through the niche-typed door, decode() = None (carrier raw = 60000, debias = 59999)
```

Zero `unsafe` constructs in the file. Nothing here is undefined behaviour and file 92's soundness
claim is untouched: the door is Rust-safe at both instantiations. What separates them is whether the
door can place the carrier on an inhabitant the numeral's own decode has no answer for, which is the
unenforced domain side-condition statement 0's hardening forbids at the fields level (`80:99-102`).
The niche-carrying case is the one file 84's construction is actually for, since the whole motivation
was "store a **bounded** numeral's datum shifted by one" (`91:581-582`).

**The repair is file 92's own, applied one section further.** Section 1.3 moved three of the audited
entry's four facts out of the trusted tier and into declaration-site const equations, including this
exact cardinality fact. It belongs on the door for the identical reason. Compiled
(`95_probes/probe_1b`):

```
error[E0080]: evaluation panicked: assertion failed: CARD == NICHE_INHABITANTS
note: the above error was encountered while instantiating `fn Bounded::<8192>::typed_mut`
```

Same error class as the level-ordering refusal at `83_probes/probe_3`. The granularity is the part
worth keeping: the refusal fires while instantiating **the door**, not while declaring the type, so
`Bounded<8192>` stays a declarable, constructible, decodable numeral and only its mutable door is
refused. That is the correct place for it, because the numeral is not what is at fault.

So the two sentences that survive together, rather than the one file 92 offered:

> A `NicheCarrier` lowering's mutable door is typed at the niche member, which returns the soundness
> obligation to the type system unconditionally. It is additionally domain-preserving exactly where
> the numeral's domain cardinality equals the carrier's inhabitant count, which is a const equation
> over type parameters and refuses at the door's own instantiation. Two properties, one door, and
> only the first is unconditional.

**On whether forbidding the integer-typed door is policing.** It will be asked, because this design
rejected dropping `div` as the policer posture one section over (`91:282-284`). It is not. The
toolbox rule's own test is whether the consumer is the one who knows the answer
(`arvo-toolbox-not-policer.md`, decision test 1), and its own boundary is a **legitimate** consumer
choice. A door whose violation is undefined behaviour has no consumer who owns the consequence,
because the optimiser owns it. Policing refuses a choice whose cost the consumer pays; this refuses a
choice whose cost nobody can pay. One sentence in the ratifying text keeps the two apart for the next
reader.

*Grounded on: settled shapes (`92:296-299`, `92_probes/probe_2:40-41`, `92_probes/probe_4:19-33`,
`80:99-102`, `91:581-582`, `91:113-121` the pricing pillar), compiled and run (`95_probes/probe_1`,
`probe_1b`), reasoned (the two-property split and the policing boundary, mine, offered as
suggestions).*

### 2.2 File 92's vocabulary narrowing creates the exact naming divergence file 94 then builds a test to catch

File 92 narrows the vocabulary to the `NonZero` family, closed and enumerated, deleting "and whatever
else std documents" (`92:158-163`). I agree with the narrowing and I think its stated reason is
narrower than its conclusion. What disqualifies `char` is not that char is unusable in principle; its
inhabitant count is not a power of two either, so field-shrinking cannot express it and a niche
construction is genuinely needed there too. What disqualifies it is that **the one audited entry's
sentence is shaped for a single excluded run at zero**, which is file 92's own "one entry per shape,
never one entry per phrase" (`92:162-163`). Those two sentences are in mild tension as written: one
fixes the vocabulary's extension to a list, the other says a second shape earns a second entry.

Follow the second, which is the better one, and the collision appears. If a second shape earns a
second entry, it earns a second vocabulary, and the trait named `NicheCarrier` is then the vocabulary
of one shape wearing the name of the genus. File 94's test, written two files later and swept over
the consolidation's trait table (which does not carry a proposed name), asks what a reader holding
one token concludes: `NicheCarrier` says "carries a niche" and denotes "is a `NonZero`". That is a
divergence, at exactly the shape file 94 catalogued, and file 92 created it while repairing something
else. Neither author could see it: file 92 precedes file 94, and file 94 swept ratified names.

**One token closes it: `NonZeroCarrier`.** The genus name stays available for the day a second shape
earns its entry, and the sealed-vocabulary machinery, the routes, the error classes, and the single
audited sentence all transfer unchanged. This is the cheapest finding in the file and I put it here
because it is the kind that gets lost between two good deliverables.

*Grounded on: settled shapes (`92:132-163`, `94:107-118` the reader-side test, `91:258-268`),
reasoned (the tension and the rename, mine, offered as a suggestion).*

### 2.3 File 92's per-level quantification is right, is now compiled, and does not terminate

The fourth bullet of file 92 section 2.1 is offered as prose: "A column whose safe surface hands out
`&mut [u8]` of its backing bytes, an API every storage crate is tempted to ship, reopens the gap at
column granularity, tail-group padding included, with no per-element accessor anywhere in sight"
(`92:247-250`). It is right, and a counterexample to a structural theorem is worth more as a program
than as a sentence, so I built it (`95_probes/probe_2`, `Layout::Bitpacked` at stored width 5, twelve
elements, one whole group plus a partial tail, four container padding bits at column granularity per
`91:566-569`):

```
unsafe blocks in this file: 0
per-element reads agree after the write: true
digests agree: false
theorem as worded (no raw accessor below the FIELDS' width): satisfied by this type
theorem quantified per byte-owner (no accessor below the COLUMN's write granule): violated by bytes_mut
```

`grep -cE "unsafe (\{|fn|impl|trait)"` returns 0. So the adopted theorem's wording is satisfied by a
type that violates the theorem, with no niche, no transmute, no per-element accessor, and no `unsafe`
anywhere. File 92's amendment holds.

**What it lacks is a terminating clause, and the omission is not cosmetic.** Stated per byte-owner
per level, the recursion asks who owns the column's bytes, and the honest answer under this design is
a consumer's arena, which arvo does not own and cannot make a theorem about. Without a named
outermost level the statement reads as an infinite regress in the ratifying text, and the first
reader to notice will either weaken it or extend it into the consumer's code, both of which are
worse than saying where it stops. The clause:

> The per-level statement runs from the fields' extent to the outermost level arvo constructs. Beyond
> that, the bytes belong to the consumer's own allocation and the postcondition is the consumer's,
> stated once in the trusted-base vocabulary at the point arvo hands them over, never asserted as a
> theorem about code arvo did not write.

**And there is a further question the amendment raises and does not settle, which I think is the more
interesting one.** File 92 states the perimeter in prose because a perimeter is what the workspace
rule states in prose (`what-you-can-observe-is-what-you-guaranteed.md`, item 1). This design's
standing preference is the opposite: a claim that can refuse should refuse. So I checked whether the
perimeter can be a bound (`95_probes/probe_3`), with the strongest form the language permits, an
`unsafe trait` whose documented contract is the whole perimeter:

```
marker satisfied by both: true true
Honest    value=7 raw=0x0007
Dishonest value=7 raw=0xe007
the marker compiled, the perimeter did not hold, and rustc said nothing
```

The clean compile is the result. Nothing on the permitted feature set sees a field's visibility: no
`TypeId`, no reflection, no full `specialization`, and `min_specialization` dispatches on types
rather than on their declarations. A derive macro could check the syntactic half (this struct has no
`pub` field) because it is handed the item, and cannot check the transitive half (no method anywhere
hands out a reference below the granule) because it is handed one item.

That is a negative result and it is load-bearing for section 3: **the mutation theorem's domain is
prose by necessity rather than by preference**, which is precisely why it falls under the
definitional-completeness line and not under the harness. A design that could have made it a type
would owe an explanation for not having; this one does not.

*Grounded on: settled shapes (`92:225-254`, `91:566-569`, `what-you-can-observe-is-what-you-
guaranteed.md` items 1-4, `harness-the-type-system.md` the ladder), compiled and run
(`95_probes/probe_2`, `probe_3`), reasoned (the terminating clause and the negative result's
consequence, mine, offered as suggestions).*

### 2.4 File 93's verdict is right and stands on a weaker sentence than the one already ratified

I take file 93's verdict on the Door as correct, and the four compiled or silicon-read facts behind
it are the best evidence in this stretch: one target defines a value and does not distinguish the
cells, one defines a trap, a third defines a different value, and the IR takes all three back
(`93` section 2). A safe total operation delivering target-varying values from identical operands is
not something this design has anywhere else, and calling that a smuggle is right.

What I want to change is where the verdict rests. File 93 identifies the patrolling guard as "no law
may read `Lowering`", shows it satisfied vacuously (every division law is conditioned on a nonzero
divisor, so no law quantifies over the cell), and offers a new sentence, the deletion test, as the
repair (`93:194-203`). Read `91:158` at source and it is two clauses:

> `Lowering` changes no value; `Encoding`, nested inside it, may change which datum carries a value.
> No law may read `Lowering`.

File 93 quotes the second and finds it vacuous. The **first** is not vacuous and it is not a guard on
laws, it is a statement about `Lowering` itself. A cell whose content is 0 on ARM, all bits set on
RISC-V, and a trap on x86 is `Lowering` changing a value, and section 1.3 has forbidden it since file
40, ratified, unamended, sitting in the same document twenty lines below the sentence file 93 cites.

This is not a correction of the verdict. It is a correction of what the design owes, and it changes
the answer to "does this need new spec text" from yes to no:

> The deletion test is not a new rule. It is the decision procedure for section 1.3's ratified first
> clause, `Lowering` changes no value, which had no procedure and was therefore checkable only by
> someone who already suspected the answer. State the cell with the `Lowering` deleted. If a
> value-layer sentence remains, a member of the resolution vocabulary, a constant derived from the
> numeral's parameters, or a parameter the consumer names, the `Lowering` implemented a stated value.
> If nothing remains, the `Lowering` authored one, which section 1.3 already forbids.

That is the shape this review has preferred every time an assembly has proposed a fifth rule: the
four already assign homes, and what is missing is a check that a home was tested. File 93's own
vacuity observation is the reason the check is needed, and its own sentence is the check. Only its
framing as new content needs to go.

*Grounded on: ratified (`91:158`, section 1.3, unchanged since file 40), settled shapes
(`93:194-216`, `93` section 2 in full), reasoned (the citation correction and the reframing, mine,
offered as a suggestion).*

### 2.5 File 93's `div_or` collides with file 93's own `0/0` closure

Section 5 of file 93 does two things that do not fit together, and I think its author would agree on
being shown it.

It closes the `0/0` row by declaring the operation **partial** there: "a numeral with no NaN has no
honest total answer to a question whose every answer is equally wrong, so the operation is partial at
that input at every preset" (`93:313-316`). Then it resolves `Hot`'s cell by giving the consumer a
**total** form: "the total form over a possibly-zero divisor takes the fallback as a parameter the
consumer names, `div_or`-shaped" (`93:329-331`).

For a NaN-less fixed-point numeral those are the same input set. `div_or(x, 0, f)` either answers `f`
at `x = 0`, in which case `0/0` is not partial and clause 3 is violated, or it does not, in which
case `div_or` is not total over a possibly-zero divisor and does not do the job it was introduced
for. The collision is small and it is exactly the kind that survives into spec text.

**It is also informative rather than merely a slip**, because the derivation that produced it is the
good part of file 93. The solution-set reading distinguishes two cells at the same divisor: empty
with a determinate direction, and no privileged value. A single fallback collapses a distinction the
derivation just paid to establish. So:

> The fallback's arity is the arity of the failure taxonomy at that input, which is two. A total
> division over a possibly-zero divisor names a directional fallback (the empty-with-direction cell,
> where the design's own resolution row would otherwise apply) and an indeterminate fallback (the
> `0/0` cell, where nothing is privileged). A consumer naming one value for both is collapsing the
> two cells, which the toolbox posture permits and the ratifying text should say out loud rather than
> let a one-parameter signature imply.

One further note on file 93's own wording, because it is the property it correctly identified as
dangerous. It describes the consumer-named cell as "law-irrelevant by construction (it is the
consumer's own constant)" (`93:334`), offered as a virtue, three sections after establishing that a
value with no law over it "can be authored anywhere without tripping anything" (`93:210-211`). Both
are true and the difference is real, since a consumer parameter survives the deletion test and a Door
cell does not. But "law-irrelevant" is the wrong thing to present as the reason it is safe. The
reason is that its author is named in the program.

*Grounded on: settled shapes (`93:210-211`, `93:288-335`), reasoned (the collision and the arity
sentence, mine, offered as suggestions).*

### 2.6 File 94's replacement test cannot fail in the hands of the person running it

File 94's finding is correct and I want to keep all of it. Its class test is keyed on the compiler
when the observer is a reader; the six divergences it enumerates are real, one of them compiled; and
the wording repair at section 4 ("named in the record") is right and is a grep away from being
forced, as section 1 above shows.

Its **replacement rule** is where I part company. It reads: "A name's class is decided at an
instantiation where its denotation and its behavioural reading diverge" (`94:261-264`), where the
behavioural reading is "what a competent reader who has not read the panel concludes from the token"
(`94:107-108`). File 94 names this as its own softest joint (`94:487-489`), which is honest, and I
think it is worse than soft. It is the same defect as the parity suite file 94 strikes two sections
later: **it cannot fail in the hands of the person running it**, because the only reader available to
the author of a ratifying text is the author, who has read the panel. A test whose verdict is a
judgement about a hypothetical stranger's inference is not a check, it is a mood, and a mood written
into a requirement is paid on every file forever at whatever the day's mood costs.

**The finding survives without the rule, and it survives better.** Three mechanical tests, each a
grep or a table comparison, find all six of file 94's divergences and quantify over no readers:

1. The name appears in more than one ratified table or definition with different content.
2. The name's correct reading requires a fact stated on a different axis or in a different section.
3. The name shares a token with an external standard, format, or algorithm whose scope exceeds the
   design's claim.

Against file 94's own list: `Hot`/`Warm`/`Cold`/`Precise` fail (1), compiled at `94_probes/probe_3`
and re-derived here from `78:409-441`. `StoredWidth::Minimum` fails (2), documented by the review
having read it the other way for two files. `Folded<N>` fails (2), since the count's identity lives
in section 1.14 and not at `Folded`'s definition. `IeeeSpecials`, `E4M3` and `IeeeDefault` fail (3).
`Fnv1a`, `quantise`, `TotalOrd` and the namesake aliases pass all three, matching file 94's own clean
column. Same six, no house style, checkable by the person writing the ratifying text at the moment of
ratification.

**And then the three tests turn out not to be a new instrument.** Test 1 is "this term has two
definitions", test 2 is "this term's definition is elsewhere", test 3 is "this term's definition is
in someone else's document". All three are the definitional-completeness line, reaching one token
further out than it currently reaches: the line governs the terms in a definition and not the term
being defined. One word fixes that, in section 4 below.

Three smaller attacks on the same file.

**The `Cold` sentence.** Section 1 above: its prose contradicts its own probe. The coincidence
surface is `Precise` alone. The finding stands; the sentence does not.

**The artifact strike is half a repair.** File 94 strikes the parity suite from `IeeeDefault`'s
artifact list, leaving two, on the ground that it cannot fail on the deployment claim (`94:311-317`).
That is right. But its own probe 2 found the receipt's mask missing FZ16 on this host, which means
the receipt has **no negative control**: nothing in the list checks that the receipt fails when the
environment is wrong. Strike one and add one. A perturbation arm (set the control register, run,
assert the receipt reports failure) is cheap, it is the artifact probe 2's finding calls for, and it
verifies the receipt rather than the deployment, which is the honest thing for it to verify. Two that
can fail is better than three where one cannot; two that can fail plus one demonstrating that they do
is better still.

**The zero-mask clause stops one step short.** File 94's receipt design is good and its emitted-code
work is the right shape: the field set on the type, the fold in const position, one instruction to
close the hole. Its last clause says a target that cannot express a field "declares a zero mask for
it, which is a claim that the target cannot check the field, never a claim that the field is
satisfied" (`94:441-442`). Then the receipt's verdict is no longer a boolean, and a consumer writing
`assert!(receipt())` gets `true` on a target that checked nothing. The pricing pillar's own clause
finishes it: a numeral whose correctness depends on a field the target declares a zero mask for
should refuse at declaration, not pass a receipt. That is one const equation and it is the difference
between a declared hole and a silent one.

*Grounded on: settled shapes (`94:107-118`, `94:155-174`, `94:255-293`, `94:311-317`, `94:436-446`,
`78:409-441`, `91:113-121`), compiled (`94_probes/probe_3` re-read at source, `94_probes/probe_2`'s
output as file 94 reports it), reasoned (the three mechanical tests, the negative control, the
refusal, mine, offered as suggestions).*

---

## 3. The question: does the separation requirement catch the three?

### 3.1 What the requirement is for, stated before it is judged

> A claim about a distinction is checked at an instantiation where the distinction is nonvacuous, and
> every model states what it separates. (`91:136-137`, adopted at `86b`)

Two halves, and they do different work. The first is a rule about **sampling**: where you check.
The second is a rule about **disclosure**: what a model says about itself. Both address one failure,
the one file 86 catalogued seven times: a wrong subject survives review because at the instantiation
everyone reaches for, the wrong subject and the right one coincide.

It has fired in advance exactly once, and the firing is documented rather than claimed. File 89
applied it deliberately to the review's own record and it produced the counting collision: "file 37's
probe and file 43's probe both looked like measurements of the same object, and neither stated what
it separated. Had either carried the sentence `86b` now requires, the collision would have been
visible in July" (`89:479-482`). That is a real hit, by the second half, on a question four
dispatches had declined as malformed.

### 3.2 The three instances, one at a time

**Instance A, the mutation theorem's unstated domain.** A structural theorem, real, quantified over
nothing anyone wrote down, so its perimeter omitted two entry points (a public field, a safe `union`
field write) and one of its terms needed replacing under one layout (`92` section 2.1).

**The requirement does not catch this, and it should not be asked to.** There is no distinction here
and there is no sampling. The claim is universal over a set, and the set has no name. Ask the
requirement's own question, "at what instantiation is the distinction nonvacuous", and there is no
distinction to instantiate: a perimeter claim is not "X differs from Y", it is "for all doors, P".

**The definitional-completeness line does catch it, mechanically, from the text alone.** "Safe
surface" and "raw accessor" are terms in a ratified sentence and are defined nowhere in the corpus
(section 1, grep-verified). The line adopted at `90b` says those two terms are either defined or
named open in the ratifying text. They are neither, in a sentence ratified at the same checkpoint,
carried into a consolidation that claims to have applied the line to everything it absorbs. Had the
line been performed, file 92's section 2.1 would have been a definition exercise on the day the
repair landed rather than a second read three files later. And the workspace's own perimeter rule is
the same instrument at a different address: item 1 is literally "name the observation surface: every
public field, accessor, `Deref`, `From`, `Into`, iterator item ... that set is the perimeter". The
design already imports that rule for observation. Nobody asked whether the write side owed the same
sweep.

Section 2.3's negative result matters here: the domain cannot be a type on the permitted feature set
(`95_probes/probe_3`), so it has to be a written sentence, so the line is the right instrument and
there is no better one available.

**Instance B, the Door smuggle.** A proposed resolution placing a semantic decision on the one axis
the design says carries none, with a deletion test offered as the distinguishing sentence.

**The requirement catches this, by its second half, and file 93 says so itself**: "the one-target
model (`probe_5` CLAIM D, this host) is precisely the instantiation at which the wrong subject (the
ISA's constant) and the right one (a stated value) coincide, and the second target is where they
separate" (`93:212-215`). `89_probes/probe_5` was written three files after the requirement was
adopted. Had it carried the disclosure sentence, it would have read "this model separates nothing on
the axis this claim is quantified over", and the claim was quantified over targets. That is the
requirement working, missed because nobody ran it, not because it is insufficient.

I would add one thing file 93 does not: the reason nobody thought to vary the target is that the
target was not visibly an axis of the claim. "The target's own divide instruction defines the answer"
reads as a statement about a mechanism, not as a universally quantified statement over targets. That
is a definitional problem feeding a separation problem, and it is the pattern in all three.

**Instance C, the naming class test.** A classification keyed on whether the compiler checks a claim,
when the thing classified is read by a person holding one token out of five.

**Half.** The preset-name divergence is caught: file 94's own probe is the nonvacuous instantiation
and it says so (`94:169-172`). The wrong-observer keying is not caught, and this is the honest
residue of the whole question. File 90's sweep ran over **every** name (`90:136-183`). The sampling
was complete. The disclosure was made: the model states its subject, "the compiler checks the claim".
The subject was simply wrong. No rule about sampling finds a complete model with a wrong subject, and
saying otherwise would be the hindsight this file is supposed to refuse.

**But the repair is not a new instrument either**, per section 2.6: the three mechanical tests that
find file 94's six divergences are the definitional-completeness line reaching the name itself. A
name defined in two ratified tables with different content is a term with two definitions, which
fails the line for the same reason a term with none does.

### 3.3 The count, and what it says

One hit by the separation requirement, working as designed, missed only because it was not performed.
One and a half misses, both of them caught by the definitional-completeness line, which has never
been performed at all. One genuine residue, a complete model with a wrong subject, which no per-file
check finds.

**So the three instances do not test what the dispatch framing suggests they test.** They are not
three probes of one requirement's reach. They are one probe of the separation requirement (passed,
unrun) and two of the definitional-completeness line (failed, never run), plus a residue that belongs
to neither.

That reframing is worth more than the verdict, because it changes the action. "The requirement is
insufficient" points at a third sentence. "The requirement was never run" points at a clause naming
when it runs, and the second is what the evidence supports.

### 3.4 What each requirement costs, over years, which is the question I was sent to weigh

A requirement is the cheapest thing in a design to state and one of the most expensive to keep. It is
paid on every file, by every author, forever, and it is paid whether or not it finds anything. So the
honest test is not "would this have caught the bug", which everything catches in hindsight. It is
"what does it cost per file, how often has it fired in advance, and what happens on the files where
it fires on nothing".

**The separation requirement.** Cost per file: one question, "at what instantiation is this
distinction nonvacuous", plus one sentence per model. Fired in advance once, on a question four
dispatches had declined, and the finding it produced was the largest of its stretch. On files where
it fires on nothing it costs a sentence. It earns its keep at a wide margin and I would keep it if it
never fired again.

**The definitional-completeness line.** Cost per file: a grep over the terms in whatever the file
ratifies. Fired in advance zero times, because nothing says who runs it or when. Three findings have
been taken from it in hindsight this stretch, at roughly a file each, and one of those files (92) is
excellent work that would have been unnecessary in its section 2.1 had the line run on `90b`'s own
text. It is not overpriced. **It is underpriced, and the correct response to an underpriced check is
to charge it, not to add another.**

**A third requirement against the wrong-subject failure.** Cost per file: unclear, because nobody has
been able to state it as a check. File 94 tried, and what came out quantifies over readers and cannot
fail in the hands of its own author. That is the tell that this failure is not requirement-shaped.
What has actually found wrong subjects in this review is not a check, it is a **dispatch from the
other chair**: file 47 wrote consumer code from outside and found the design unwriteable; file 94
read the names from outside and found the class test keyed wrong; file 93 ran the claim on a second
target. Those are roles, and roles are how a panel is composed, not how a design is stated. Minting a
requirement for them would put a permanent per-file tax against a failure the tax does not detect,
which is the worst trade in this whole ledger.

### 3.5 The one thing that is missing, and it is not a requirement

Both existing requirements are stated as properties of artifacts and neither names a moment or an
owner. Compare the table-diff obligation, which has both, in the same document: "**The table-diff
obligation, executed on this document by its own author before it stands**" (`91:55`). That one gets
performed, every consolidation, and it is reported. The two requirements get cited.

That is the whole difference, and it is one clause.

---

## 4. What the design is, in the form the next consolidation could take close to verbatim

Offered, not ruled, and owed a second read like everything one-pass in this stretch.

**On the two requirements, one clause added to their own statement.**

> Both requirements are performed by the author of the ratifying text, on that text, before it
> stands, in the same shape and at the same moment as the table-diff obligation. The
> definitional-completeness line's performance is a list of the terms checked and their dispositions
> (defined here, defined at a cited position, named open). The separation requirement's performance
> is, per model the text relies on, the axis the model separates on and the two instantiations at
> which it separates. A ratifying text that does not carry both performances has not been checked,
> whatever it cites.

**One word in the definitional-completeness line, and one clause behind it.**

> When a structure is ratified, every term in its definition, **including the name being defined**,
> is either defined or named open in the ratifying text. A name defined twice with different content
> is defined nowhere.

That second sentence produces file 94's preset finding, its `StoredWidth` finding and its `Folded<N>`
finding mechanically, and its three mechanical name tests are the same line at three addresses: the
term has two definitions, the term's definition is in another section, the term's definition is in
someone else's document. File 94's six relocated boundary sentences (`94:277-287`) remain its
deliverable, and four of the six already exist in the corpus and only need moving to the definition
the reader reaches. **No new naming rule is needed. The class-two half of the refined principle
stands as adopted, with "designated" replaced by "named in the record" per file 94 section 4.**

**On the mutation theorem, file 92's amendment with its domain terminated.**

> The theorem is stated per byte-owner and per level, mirroring statements 0, P and C. The safe
> surface of a level's byte owner (the carrier at `Dense`, the column group at `Bitpacked`) exposes
> no public field, no `DerefMut` to the representation, no foreign-bytes constructor outside
> statement C's named obligation site, and no accessor below that level's own write granule.
> Whole-value replacement and interior mutability are inside the safe surface and preserve
> canonicality by move semantics. A Rust `union` anywhere in the chain would be a safe raw door and is
> excluded by name. The statement runs from the fields' extent to the outermost level arvo
> constructs; beyond that the bytes are the consumer's allocation and the postcondition is a
> trusted-base entry at the hand-over, never a theorem about code arvo did not write. "Byte owner"
> and "write granule" are defined at the statement; "safe surface" is defined as the set of
> expressions reaching a level's bits without `unsafe`.

**On the niche door, file 92's amendment with its second property named.**

> A `NicheCarrier` lowering's mutable door is typed at the niche member, which returns the soundness
> obligation to the type system unconditionally; an integer-typed door onto a niche carrier is
> forbidden outright, because the identical door shape whose violation costs decorrelation on a
> padded carrier costs undefined behaviour on a niche carrier with no diagnostic on the violating
> store. The niche-typed door is additionally domain-preserving exactly where the numeral's domain
> cardinality equals the carrier's inhabitant count, which is a const equation over type parameters
> and refuses at the door's own instantiation rather than at the numeral's declaration. Two
> properties, one door, and only the first is unconditional. Forbidding the integer-typed door is not
> the policer posture: the toolbox line refuses a legitimate consumer choice, and a door whose
> violation is undefined behaviour has no consumer who owns the consequence.

**On the vocabulary, one token.**

> The vocabulary is named `NonZeroCarrier` and is the `NonZero` family, closed and enumerated. A
> second niche shape earns a second vocabulary with its own name and its own audited entry, never
> admission under this one, because the audited sentence is shaped for a single excluded run at zero
> and is false or meaningless of any other shape.

**On the Door, file 93's verdict with its ground corrected.**

> A `Lowering` may be an input to a design-time derivation and may implement a stated value. It may
> never author one, which section 1.3's first clause has said since file 40. The deletion test is the
> decision procedure that clause never had: state the cell with the `Lowering` deleted, and if a
> value-layer sentence remains (a resolution-vocabulary member, a constant derived from the numeral's
> parameters, or a parameter the consumer names) the placement is legitimate and the `Lowering`'s
> remaining question is cost. If nothing remains, the cell's content lives on the axis defined to
> carry none. The `x/0` cell fails the test on four compiled or silicon-read facts, the resolution is
> a consumer-named fallback, and the Door keeps its two legitimate roles: input to derivations, and
> implementation selector for stated cells.

**On the fallback's arity.**

> The fallback's arity is the arity of the failure taxonomy at that input, which is two: a directional
> fallback for the empty-with-direction cell and an indeterminate fallback for `0/0`. A consumer
> naming one value for both collapses two cells the derivation distinguishes, which the toolbox
> posture permits and the ratifying text states rather than lets a one-parameter signature imply.

**On the receipt, file 94's design with its refusal.**

> An environment type's denotation is a per-target field set, and the receipt is a fold over it
> computed in a const position rather than written per target. A zero mask declares that the target
> cannot check the field, never that the field holds, and a numeral whose correctness depends on a
> field the target cannot check refuses at declaration rather than passing a receipt. The artifact
> list carries the parity suite against the arithmetic claim only, and carries a perturbation arm
> demonstrating that the receipt fails when the environment is wrong.

---

## 5. What I leave open

- **Everything in section 4 is one-pass, including the parts that repair other people's one-pass
  work.** The clause on the two requirements is the one I would most want a second read on, because
  it is the one that costs something on every file forever and I am the person proposing it. The
  attack surface: whether "performed by the author on their own text" is a check at all, given that
  the author of a ratifying text is exactly the person least able to see its undefined terms. My
  answer is that a grep does not care who runs it, which is the whole reason to prefer a mechanical
  check to a judgement, but that answer deserves someone hostile to it.
- **The wrong-subject residue has no mechanism and I have not proposed one.** I have said it is
  caught by composition rather than by rule, and named three instances where it was. Whether that is
  a satisfying answer or an admission is genuinely arguable, and it is op's read, not mine.
- **`NonZeroCarrier` versus `NicheCarrier` is a token and I may be over-reading it.** It is offered
  because file 94's test, which I largely accept, says it is a divergence, and because the cost of
  changing it now is one token against a rename later.
- **File 92's cohort instance of the mutation gap is still uncompiled**, as file 92 itself notes
  (`92:381-384`), and my per-level statement covers it in words only.
- **The `Cold` footprint bench is now blocking three separate things.** It is the designated verifier
  under the naming principle's second clause (file 94 section 4), it is the only artifact that could
  price `Cold`'s own intent, and it is the one measurement the pricing pillar says decides designs.
  `5dae109` closed the section-filter half. The by-reference input path is the whole remaining
  blocker and it is a mockspace-side fix. I would put it above every design item on this list, because
  it is the only one where the review is currently unable to find out whether it is wrong.
- **I did not run a bench and I did not price anything in time.** Every number in this file is a
  compile outcome, a program's stdout, or a grep, on `aarch64-apple-darwin` under the pinned nightly,
  and the file says which at each claim.

Only op's calls are final, and even those go stale. Everything above is evidence and suggestion.

*Grounded on: ratified (`91:55` the table-diff obligation's shape, `91:128-134` the
definitional-completeness line, `91:136-143` the separation requirement, `91:158` section 1.3,
`91:113-121` the pricing pillar, `90b` throughout), settled shapes (`89:479-482`, `90:136-183`, `92`
sections 1.2, 1.3, 2.1, 2.3, `93` sections 1, 2, 3, 5, `94` sections 3, 4, 5, 6, 7, `78:409-441`,
`what-you-can-observe-is-what-you-guaranteed.md` item 1, `arvo-toolbox-not-policer.md` decision test
1), compiled and run (`95_probes/probe_1`, `probe_1b`, `probe_2`, `probe_3`, commands and stdout in
`95_probes/OUTCOMES.md`, all fresh this session inside the tree), measured (the four corpus greps,
`git show --stat 5dae109`, the workspace test run), verified at source
(`mock/crates/arvo-tensor/tests/capacity.rs:14-18` against `src/capacity.rs:48`, `92_probes/probe_2`,
`92_probes/probe_4`, `94_probes/probe_3`, all read rather than trusted from their outcomes files),
reasoned (sections 2 and 3 throughout, and every sentence in section 4, mine, offered as suggestions
and not as rulings).*
