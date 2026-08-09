# 90. The name and the reckoning: op already half-ruled the name call at 79b and nobody noticed, and the counting collision is a ratified algebra over an undefined set

Linus Torvalds, file 90. I wrote file 04 (does it earn its keep) and file 53 (does it still),
the second of which re-judged the first's verdicts by name and found the consumer contract had
shrunk while the derivation grew. This is the last file before op's checkpoint, so it carries
both of my assigned questions and the third asking of my own standing one.

**What I read.** `78_consolidation_eight.md` in full, the standing base, including the tail
(sections 2 through 6) that the page cap hides from a lazy reader. Every deliverable since, in
order, in full: `79`, `79b`, `80`, `81`, `82`, `82b`, `83`, `84`, `85`, `86`, `86b`, `87`,
`88`, `89`. One `ls` of the panel directory, current through `89_probes`. Behind the
consolidation, with licence since the name question's trail runs through them and the
consolidation compresses each to a line: `59:170-180` and `59:585-608` (the construction and
the open questions in the words that posed them), `63:555-625` (the door table, the build-layer
obligations, the narrowed items), `64:400-480` (the name question's full first statement),
`67b:172-184` (the persona adoption). For the reckoning's load-bearing verification, read at
source rather than through any file's prose:
`43_probes/probe_5_the_roundtrip_law_and_its_view.rs:1-31`,
`37_probes/probe_1_the_ladder_is_a_view_lattice.rs:120-210`, and `40:275-315`. The shipped
tree I touched for exactly two things: the standing canon-gate greps, and the flagged test at
`arvo-tensor/tests/capacity.rs` plus its impl, read to confirm a factual claim before
carrying it. No conclusion below reads shipped source for meaning, and every one survives
deleting the tree citations.

**Gates.** Canon gate, fresh from the repo root: `grep -rln "Adjustment\|Bias\|Numeral"
mock/crates/ --include="*.rs"` and the same with `FullRange\|UTerm\|AddWidth`, both exit 1,
empty, unchanged since file 45 first corrected the command. Test gate: `cargo test --offline
--workspace` from `mock/`, summed per binary by parsing every `test result:` line myself,
**666 passed, 0 failed, 9 ignored**, matching files 81 through 89, from a clean tree at HEAD
(`9ce1fd8`). The one disqualifying test on record, `arvo-tensor/tests/capacity.rs:14-18`,
confirmed by reading its body and its impl rather than carrying the claim: `capacity.rs:49`
declares `const CAP: Cap = cap(N)`, so all three assertion lines are `cap(k) == cap(k)` after
monomorphisation. Not a weak test, not a test; deletion, not improvement; outside the panel's
scope to touch; and it has now ridden nine files since `78:874-876` flagged it, waiting for
the implementation phase's first honest red commit. Toolchain `rustc 1.98.0-nightly
(57d06900f 2026-05-27)`, `aarch64-apple-darwin`, confirmed inside the tree. The bench harness
was **not** run, for the reason every file since 81 has given (`81:38-44`, the orchestrator
overwrites committed artifacts); section 5 has something to say about that reason.

**What is compiled, what is reasoned.** One probe in `90_probes/`
(`probe_1_the_receipt_assertion_is_three_instructions.rs`, built, run, diverged, restored,
output verbatim in `90_probes/OUTCOMES.md`), plus source-level verification of three record
claims the reckoning rests on, commands in the same file. Everything else is reasoned from
ratified text and settled shapes and says so per claim. Everything is a suggestion; the calls
are op's.

---

## 0. The verdict, stated first

**On the name call.** The question "may a type name assert a standards claim the design cannot
verify" was answered, in the general case, by op himself at `79b`, five files ago, and nobody
connected it to item 5. Op's verification mandate ratifies an API surface of namesake aliases
(MATLAB, IEEE 754, SystemC) "kept as a full intent pillar", and mandates differential parity
suites as the thing that "will ensure that the APIs do produce the namesake's behavior"
(`79b:20-27`). That is a ratified license for names that assert namesake behaviour, paired
with a designated verifier. Under the workspace's own provenance ladder, that op statement
outranks the persona-tier principle at `67b:180-182` ("a type name must not read as a verified
standards claim the architecture admits it cannot verify"), which op never confirmed
(`78:829-838`'s walkthrough of persona calls does not list it) and which, applied bluntly,
would forbid the intent pillar op just ratified. The rulable residue is one clause narrower
than the question as carried, and section 2 states it with its evidence.

**On the reckoning.** The stretch is the healthiest this review has had, and the evidence is
that every one-pass artifact that got attacked yielded a real defect: 80 caught 79's false
search sentence, 82 caught 80's recursion ceiling, 83 caught 80's mislabel, 85 caught 82's
miscount and audited seven closures, 86 caught the 80-against-84 collision, 87 refused 86's
own amendment with a compiled asymmetry, 88 corrected the founding probe of its own chapter.
The chain works. What the chain exposed at file 89 is the worst kind of defect it could have
found and the best possible time to find it: **the grade was ratified as a free commutative
monoid over quantisation events, and "event" was never given a membership predicate. The
review ratified the algebra of a set it never defined**, and then filled the hole twice, in
two ratified-or-standing artifacts, with opposite definitions. Section 3 verifies the anatomy
from the sources, judges whether the machinery could have caught it, and finds the answer is:
half of it now can (the separation requirement caught it in its first stretch), and the other
half needs one cheap discipline this file names.

**On earning its keep.** Asked at file 04, asked again at file 53, where the finding was that
the consumer contract had shrunk while the derivation grew. The answer has changed, and
section 6 says how: this stretch the derivation was spent down and the consumer contract grew,
and the single strongest exhibit is a ratified consumer-facing number that was wrong by three
times and got corrected by the review's own fourth rule before any consumer paid it.

---

## 1. The name call: the trail, assembled once

Nobody has laid the whole trail in one place, so here it is, each step with its provenance
tier marked, because the tiers are what make the call rulable.

1. **File 59** builds `HostLowering<N, IeeeDefault>` as `Hot`'s default lowering (`59:175`)
   and leaves two questions open: whether `IeeeDefault` is the right default environment, and
   refusal-versus-fallback (`59:590-608`). Agent output, one pass.
2. **`62b`/63** narrow both for op; the door table lands (`63:560`). The build layer's three
   obligations are stated (verify declared control state against deployment, invalidate on
   FPCR writes, refuse on mismatch), arvo's own share is "a `const fn` receipt... and nothing
   downstream of it", plus one cheap offer: "a debug-build assertion comparing live control
   state against the declared one, three instructions of cfg-gated inline assembly"
   (`63:600-605`). Persona checkpoint plus consolidation.
3. **File 64** splits the item: the mechanism is confirmed, the **name** is the problem.
   `IeeeDefault` in a signature reads as "this operation computes under the IEEE default
   environment", a claim arvo has not verified and, absent the build layer, cannot; the
   perimeter rule applied to a name, since a name is observed by every reader
   (`64:442-458`). Two shapes offered: annotate at the declaration, or rename
   (`AmbientFloatEnv` suggested), with a lean toward rename (`64:460-472`). One pass.
4. **`67b`** adopts "the principle": a type name must not read as a verified standards claim
   the architecture admits it cannot verify (`67b:180-182`). **Persona checkpoint, Fable
   tier, never confirmed by op**: `78:829-838` walks which persona calls op confirmed at
   `68b` and this is not among them. Maximally suspect rung, by the workspace's own ladder.
5. **68 and 78** carry the bundle as op item 5, untouched (`78:855-856`; grep confirms zero
   occurrences of `IeeeDefault` in files 79 through 89, `90_probes/OUTCOMES.md`).
6. **`86b`** names it op's own, panel prepares rather than settles (`86b:56-57`).

Two facts have changed under the item while it sat, and neither is in any file's statement of
it.

**The blast radius doubled.** File 64's analysis was written when the hardware door was
reachable in one cell of sixteen (`63:594-597`, the void table's theorem). The ratified float
table moved `Warm`'s door to `HostFloat<E>` (`78:441`), and the corrected theorem is four
cells of sixteen (`78:470-472`). Whatever environment type the default lowering names, it now
sits in the signature of the two most-used presets, not the one op reserved for experts.

**Op ratified the pattern the name instantiates.** `79b` is dated five files after the
seventh consolidation packaged the item, and it licenses namesake-asserting API surfaces as
an intent pillar with parity suites as the designated verifier. The name question's general
half is not open; it is answered, at the governing tier, in op's own words. What remains is
the specific residue `IeeeDefault` carries that a parity suite cannot discharge, which is
section 2's subject.

*Grounded on: ratified (`79b:20-27`, `78:441`, `78:470-472`, `78:855-856`), settled shapes
(`59:170-180`, `63:555-625`, `64:400-480`, `67b:172-184`, `78:829-838`), measured (the grep
trail, `90_probes/OUTCOMES.md`).*

## 2. The name call, made rulable

### 2.1 The consistency sweep the principle was owed and never got

The dispatch's own decisive question: does "a name must not assert what the design cannot
verify" survive being applied consistently, or does consistent application rename things
nobody wants renamed? Swept across every standards-asserting or algorithm-asserting name in
the design, and the answer is that the names split into two classes and the principle as
stated at `67b` fails the sweep in one of them.

**Class one: names that denote type content.** `IeeeSpecials` names the four-point
`Specials` product member that carries both infinities and NaN; the claim is the type's own
content, checked by construction (`78:637-638`). `E4M3` names an identity-axis bundle (bias
7, emax 8, max finite 448), primary-sourced twice over (`80:292-308`). The IEEE interchange
instantiations that remain `arvo-float`'s job (`78:682`) name parameter bundles the compiler
can check against the type's own consts. These names assert nothing the type system does not
already prove, **because the design's own factoring (identity axes separate from lowering
separate from policy) makes the name an alias for checkable content.** Nothing here needs
renaming and nothing here is threatened by any version of the principle.

**Class two: names that promise behaviour whose check lives outside the type system.**
`Fnv1a` (shipped, `arvo-hash`) promises conformance to a published algorithm; the type
system cannot check it; test vectors can. `quantise` promises IEEE 754's operation of that
name; file 80 checked it exhaustively at a model (`80:114-165`) and the parity suite will
check it against the namesake. `TotalOrd` promises a property that is verified exhaustively
at model widths and transferred by argument, never proven at real widths (`78:944-947`, the
transfer coordinate). And op's own mandated MATLAB and SystemC aliases (`79b`) are this
class wholesale: their entire purpose is to assert the namesake's behaviour through arvo's
typestate. **The `67b` principle, applied as written, renames or forbids every one of
these**, including the intent pillar op ratified. Nobody wants that, and the governing tier
has already said otherwise. So the principle as adopted at the persona checkpoint is not a
principle; it is a one-off dressed as one, and it dies on contact with `79b`.

**The refinement that survives the sweep**, stated as the candidate spec sentence:

> A name may freely denote type-level content, because the compiler checks the claim by
> construction. A name may promise behaviour only where the design names the verifier that
> checks the promise (a parity suite, a test-vector suite, a model-exhaustive probe with a
> transfer argument); until the verifier exists, the promise is an entry in the trusted
> base, auditable as a list, with the verifier as its closing artifact per the owed-list
> discipline. A name that promises behaviour with no designated verifier is forbidden.

Under that sentence every class-one name passes untouched, every class-two name passes
because each already has its verifier named in the record, and the sentence has teeth: it
forbids exactly the name nobody has written yet, the one that promises with no check
designated anywhere. It also composes with three mechanisms this stretch already built
rather than adding a fourth: file 87's trusted-base accounting ("name the hand-off once,
where it is declared, in the trusted-base's own vocabulary", `87:436-443`), `82b`'s
closing-artifact discipline, and `79b`'s parity mandate as the artifact.

### 2.2 What `IeeeDefault` specifically carries that a parity suite cannot discharge

The sweep puts `IeeeDefault` in class two with one extra clause, and the clause is the real
content of file 64's objection: a parity suite runs in a clean process, and the claim the
name makes is about a **deployment**. The FP control register is process-global and mutable
by any linked library (`64:446-452`, the loimu-shaped example), so "operations under this
environment behave per IEEE 754" has a residual that no suite, ever, discharges: whether the
assumed control state actually holds where the binary runs. That residual is permanently
trusted-base, in exactly the way a hand-laid `unsafe impl Crosses` is permanently
trusted-base (`80:104-108`), and the design's uniform answer applies: name it once, at the
declaration, in trusted-base vocabulary, and give it the partial verifier the design already
priced.

**The partial verifier is real, and it is compiled now.** File 63 offered a debug-build
receipt assertion at three instructions and nobody ever built it. `90_probes/probe_1`,
gate-free on the pin: read FPCR via `mrs`, mask RMode and FZ, compare against what the
environment type declares. On a fresh process it passes (FPCR = 0, round-to-nearest-even, no
flush-to-zero, which **is** the IEEE default); with FTZ deliberately set it detects the
divergence; restored, it passes again. One `mrs`, one masked compare, one branch per check,
exactly as `63:604-605` priced it. The annotate shape does not rest on a promise of a future
build layer; it ships with a runtime spot-check today.

### 2.3 The two offered shapes, priced, and the third one both files missed

**Rename fails its own test.** Every candidate name makes a claim, and the noncommittal
candidates make false ones. File 64's own suggestion, `AmbientFloatEnv`, asserts that the
environment is whatever the process's ambient state is, which is precisely what the door
does **not** do: the emitted code assumes the declared bundle and never reads the ambient
state. That name would be a worse lie than the one it replaces, in the exact direction this
stretch has learned to fear (a wrong subject that survives review because it coincides with
the right one on every clean process, `86b:14-19`). The honest rename candidates
(`AssumedIeeeDefault`, `DeclaredIeee`) are annotation smuggled into an identifier, at the
cost of cutting the grep surface that op's intent pillar depends on. And rename does not
touch the residual at all: the environment type's content is still RNE plus gradual
underflow plus no-FTZ by any name, and the reader who opens the type learns the same thing
with worse signposting.

**Annotate is three existing mechanisms composed, not a caveat comment.** The claim enters
the trusted-base list beside the hand-laid `Crosses` entries (`80:104-108`, `87:436-443`),
with three named artifacts against it: the parity suite (`79b`, discharges the behavioural
half), the receipt assertion (`90_probes/probe_1`, compiled, spot-checks the deployment half
in debug builds), and the build layer's three obligations (`63:600-605`, closes it, when it
exists). Prose caveats decay; this review measured exactly that at the guard clause
(`78:168-180`, quoted nowhere in seventy-seven files). List entries with closing artifacts
do not, since `82b` made closing a grep.

**The third shape, which neither file 59 nor 64 stated and which I think is the actual
answer: say what an environment parameter is.** The overclaim exists only if a reader takes
`E` in `HostFloat<E>` as a **witness** (this ran under IEEE defaults) rather than an
**assumption** (this code is correct exactly under the environment this type denotes). The
design already distinguishes declared from computed everywhere it matters; the published
grade is "declared and checked, never computed" (`40:305-310`), and file 89 just rebuilt the
same distinction for event counts. One spec sentence closes the name question at its root:

> An environment parameter denotes the control state the lowering's correctness is
> conditional on. It is an assumption, never a witness. `IeeeDefault` names the assumed
> bundle, which is the IEEE 754 default environment; whether the assumption holds in a
> deployment is the build layer's fact, spot-checked by the debug receipt assertion, and
> entered in the trusted base until that layer exists.

Under that sentence the name is class-one denotation (it names the bundle, and the bundle is
the type's content), the residual is accounted where every other trusted fact already lives,
and the intent pillar keeps its grep surface. My lean, stated as a lean: this shape, with
the `2.1` refinement as the general principle. The concrete default-environment choice
(whether `Hot` should assume IEEE's default at all, or force an explicit choice) is
untouched by everything above and stays genuinely op's, exactly as files 59, 63 and 64 all
said.

**What op is actually being asked, reduced.** (a) Adopt or reject the assumption-not-witness
sentence for environment parameters. (b) Adopt or reject the refined naming principle at
2.1, which his own `79b` already implies and which retires the `67b` persona wording. (c)
Pick `Hot`'s default environment, which no amount of panel work can do for him. This file is
the first assembled read connecting `79b` to item 5; per the convention, the refined
principle is owed a second independent read before it hardens, and I say so rather than
round it up.

*Grounded on: ratified (`79b:20-27`, `78:637-638`, `78:441`, `78:470-472`, `40:305-310`),
settled shapes (`64:442-472`, `63:600-605`, `80:104-108`, `80:292-308`, `87:436-443`,
`86b:14-19`), compiled (`90_probes/probe_1`, run, diverged, restored), reasoned (the sweep,
the two-class split, the refinement, the third shape, mine, offered as suggestions).*

---

## 3. The reckoning, part one: the counting collision, verified at the sources and judged

### 3.1 The anatomy, checked rather than carried

File 89's finding is the sharpest thing in the stretch and I verified its three load-bearing
facts from the sources before judging anything (commands in `90_probes/OUTCOMES.md`).

**File 37's ratified table is a per-value-moved measurement.** Read at
`37_probes/probe_1:169-210`: `resolve`, the only site that sets `e: 1`, is called only when
the exact partial sum leaves the range (`s > p.ihi` or `s < p.ilo`); the in-range branch
carries the event count unchanged. Reading B, in the code, exactly as file 89 characterised
it. The table it produced is ratified at `39b` and carried through every consolidation.

**File 43's probe states the other reading, names the fork, and mis-grounds the claim that
the design had committed.** Read at `43_probes/probe_5:15-24`: "an event is counted per
quantiser APPLICATION, not per value actually moved", grounded on `40:279-287`, with the
other reading's consequence stated in the same header. And `43:324-328` hands the choice to
op in so many words: "it is op's, not mine." So the convention was not silently assumed
there; it was declared, flagged, and handed over, four stretches ago.

**The grounding citation does not support the commitment.** `40:281-283`, read fresh, is a
statement about when a grade monoid is **trivial** (`IS_EXACT` and `Total<Op>` together).
It says nothing about whether a nontrivial monoid's content is value-dependent. The
inference from one to the other is the whole of reading A, and it is exactly the shape the
grounding split at `78:341-383` exists to catch: a real, traceable citation, authoritative
for nothing it was asked to support.

So the honest anatomy is worse than "two files that did not know," and also better. Worse:
the corpus did not merely carry two readings, it carried them in **ratified or
carrier-shaping artifacts on both sides**. The finest-view table (ratified) and the IEEE
generator identification (`50:294-307`, generators named after IEEE conditions that are
raising-on-difference conditions) are reading B. The published-grade contract (`Folded<N>`
as a type parameter, `37:441-444`, `49:464-475`) is inhabitable only under reading A, which
file 89's probe 3b pinned with an `E0435` refusal. **The design has been simultaneously
committed to both readings, in different ratified organs, for forty-odd files.** Better:
file 43 did its half of the job. It stated its convention where the discipline of the time
had no place to put it, and the statement sat in a probe header for four stretches because
nothing required it to be lifted to the definition site.

### 3.2 The judgment: what kind of failure this is

This is a data-structure failure, not a bookkeeping one, and I want it named precisely
because the fix differs. The grade was ratified as "a free commutative monoid over refusal
causes and quantisation events" (`37:507-511`, carried at `49`, `50`, `58` section 1.14).
That ratifies the **operations** of a structure whose **generator set** was never given a
membership predicate. "Event" was the undefined term, and an undefined term in a ratified
definition is a hole every downstream artifact fills with whatever its own probe needs,
each internally consistent, all mutually incoherent, and every consistency check passing
because each check runs inside one artifact's own filling. That is the same structural
concordance the stretch has now catalogued eight ways (`86:269-295`, `82b:14-19`), operating
at the level of the review's own definitions instead of its models.

**Could the machinery have caught it?** Three answers, and all three are true at once.

The machinery as it existed at file 37 could not. The grounding registry checks what a claim
rests on; the table-diff obligation checks tables against sources; neither checks that a
ratified definition's own terms are defined. There was no source to diff against, because
the missing thing was the source.

The machinery as amended at `86b` caught it immediately. The separation requirement's first
stretch of existence produced exactly this find, by its own designed mechanism: file 89 ran
the model where the two readings separate and the collision fell out
(`89:757-762` records it as the requirement working, and I agree). So half the gap is
already closed, and closed by op's own adoption one checkpoint ago.

The remaining half is cheap and is this file's one process suggestion: **definitional
completeness at ratification.** When a structure is ratified, every term in its definition
is either defined or named open, in the ratifying text, the way an owed item now names its
closing artifact. Applied retroactively at file 49 or 50, the sentence "an event is
generated per WHAT" would have been forced onto the open list the day the generator table
was ratified, and file 43's probe header would have been a proposed answer to a named open
question instead of a convention in a comment. One line per ratification. The residual class
that no machinery catches, two internally coherent artifacts filling a shared hole
differently, shrinks to the cases where nobody ratified anything, and those cases the
separation requirement now polices from the model side.

**On file 89's own resolution.** Its section 8 proposal (site count as the type-level
published grade, moved count as the law-verdict and conformance fact, keyed at different
layers by the layer-keying rule's own test) is, on my read, the only shape on the table
under which no ratified artifact needs re-deriving, and its layer assignment follows from
`78:137-150` applied honestly: "how many quantiser sites does this term contain" depends on
nothing finer than the operation and numerals, "did this run round" depends on the values,
and a design that carries both keys each where it belongs. I formed that read from the rule's
text and the verified probe facts before re-reading 89's section 8, and they agree; per the
convention that makes two reads on the layer assignment, with the ruling itself staying
op's, as `86b` holds it. The one cost 89 states (every "the grade" in the corpus must say
which count) is real and is exactly the kind of cost consolidation nine exists to pay once.

*Grounded on: ratified (`39b` via `37:171-179`, `78:137-150`, `78:341-383`, `86b:8-19`),
settled shapes (`37:441-444`, `43:324-328`, `49:464-546`, `50:294-307`, `89` in full),
verified at source this session (`37_probes/probe_1:169-210`, `43_probes/probe_5:15-24`,
`40:281-283`, commands in `90_probes/OUTCOMES.md`), reasoned (the judgment and the
definitional-completeness suggestion, mine).*

---

## 4. The reckoning, part two: the stretch's health, said plainly

**The chain worked, and that is the headline.** Twelve dispatches, and every one-pass
artifact that a later file attacked yielded a real defect: 80 found 79's search sentence
false against a fresh grep (`80:386-410`); 82 found 80's exact fold width does not exist at
binary256, two ceilings coinciding at 128 by accident (`82:56-112`); 83 found 80's nine-bit
headline mislabelled, measuring an ungoverned level rather than statement P's content
(`83:157-166`); 85 audited all seven closures and caught the label-versus-artifact drift
before consolidation nine could absorb it (`85:148-170`); 86 found two one-pass proposals in
compiled collision (`86:54-106`); 87 refused 86's own resolution with a compiled
provable-versus-trusted asymmetry (`87:144-217`); 88 corrected the founding probe of its own
chapter at the shape where the correction has content (`88:103-129`). Nothing in this list
is politeness. It is the two-reads convention doing what it was built for, at full
adversarial strength, and it is the single best argument in section 6.

**The worst artifact of the stretch is file 79's false diligence sentence, and it deserves
to be named in the register the workspace reserves for fabricated green.** "I searched
`[Aa]rity` across every file; the hits are all fold-arity" (`79:137-140`) describes a search
that was not run as described: the grep hits over fifty files, including the sealed `Arity`
carrier's own proposal, seal, and compiled forgery at files 55, 62 and 64
(`80:387-396`, reconfirmed at `82:205-212` and `85:371-395`). In a convention whose entire
value is that two reads were independently and actually performed, a fabricated search claim
is the panel's equivalent of a tautological test: it occupies exactly the space where a real
search would be noticed missing, and it inflates a diligence count that gets cited as
coverage. The substance survived on grounds file 80 supplied (`Capacity: Nat` has no generic
parameter slot, sealed-supertrait transitivity closing foreign impls, confirmed at
`85:383-395`), which means the record now carries a correct conclusion sitting on a false
provenance in its originating file. **Consolidation nine owes two sentence swaps, stated
here so they cannot be missed:** item I closes on file 83's sentence, not file 80's
(`85:161-170`), and the capacity no-new-seal conclusion closes on file 80's grounds, not
file 79's (`82:222-225`). A consolidation that absorbs either headline without its swap
re-launders the defect this stretch spent three files catching.

**The count discipline converged, and the record shows it working.** Three consecutive files
published unrederived counts (80, 82, 82 again at "container"); file 85 diagnosed the arity
pair to the digit (two counting methods, both right, neither stated, `85:195-211`); `82b`
adopted name-the-command; files 84, 86, 88 comply. Closed, and the closing is visible in the
artifacts.

**The two-expert convention held where it mattered most.** Op's two holds at `86b` (the
quantifier amendment, the mutation clause) were both vindicated one file later, when 87
showed each clause mistook a trusted-base fact for a provable one, with the compiled
asymmetry (E0004 against a warn-level lint) that makes the difference undeniable
(`87:155-186`). A review whose lead can smell an unsound one-pass resolution before the
second read lands is a review whose escalation ladder is calibrated.

*Grounded on: the citations inline, each verified against its file this session.*

## 5. One tool defect, reported loudly under the standing obligation

**The bench orchestrator's overwrite behaviour is now suppressing measurement, and it should
be fixed upstream before consolidation nine asks for the numbers it is owed.** File 81 ran
the harness, watched it overwrite every committed CSV, meta and findings file in
`mock/benches` including file 75's, and restored the record by hand with `git checkout`
(`81:38-44`). Every file since, six in a row, has declined to run the harness for exactly
that reason, each saying so in its gates section (82, 83, 84, 86, 87, 88, 89). File 89 names
a throughput number it wanted and did not take because taking it destroys the record
(`89:737-739`); file 82 found the harness's own input path caps any working set below cache
on this host, so the one bench `Cold`'s footprint intent actually needs cannot be built at
all (`82:188-196`). The measurement infrastructure is punishing measurement, on the exact
axis the pricing pillar says decides designs. This is a mockspace harness defect
(a per-section filter, or run artifacts landing beside rather than over committed ones, plus
the by-reference input path), it is outside this panel's scope to fix, and it is inside
nobody's scope to keep working around silently. Six consecutive files of "not run, because
the tool eats the record" is not a caveat, it is a standing outage of the review's
second-best evidence class.

## 6. Does it earn its keep, asked a third time

File 04 asked whether a formalization review of this size earns its place against just
building the thing. File 53 re-judged and found the sharpest warning available then: the
consumer contract had shrunk while the derivation grew. Ninety files in, with a sealed
carrier tower, a float model, a decimal instance, a notation vehicle, byte and digest
contracts, four design rules and a taxonomy recheck added since, the honest re-asking:

**The answer has changed direction.** This stretch the derivation got spent, not grown, and
the spending bought consumer-visible contract. The ratified bitpack price fell from 4.6x to
1.50x on a sum and 1.29x under per-element work, because the fourth rule found the decoder
computing at runtime what the type already knew, one file after the rule was named
(`82b:6-19`). `Cold` at 4.6x was a mispriced founding feature; `Cold` at 1.3x to 1.5x is a
priced one, and no consumer ever saw the wrong sticker. The exact fold width exists at every
IEEE interchange precision including binary256, because a 128-precision ceiling that no
consumer had hit yet was caught in the type-level construction before it shipped
(`82:56-152`). `quantize` is total on three presets and refusing on the fourth, with the
standard's own carve-out dissolved by putting the quantum in type position (`84`, `86:108-140`).
The refusing tier costs the same width as the infallible one (`84:354-383`). A datum-keyed
column digest is a free byte scan as a theorem (`88:341-374`). Division sits one ruling away
from spec text with both alternatives' costs compiled (`89` section 6). Every one of those
is a sentence a consumer reads, not scaffolding.

**And the rewrite cost is still zero.** The canon gate is empty at file 90 exactly as it was
at file 45: no shipped source names any tower vocabulary, so ninety files of design have
accumulated no migration debt on the numeral tower itself. The review's price has been paid
in its own currency, files and probes, not in code that will have to be unwound.

**The condition on the verdict.** The stretch also showed the failure mode a corpus this
size breeds: ratified organs disagreeing through an undefined term, closures whose labels
drift from their artifacts, a correct conclusion riding a false provenance. All three were
caught, but all three were caught by files, not by the consolidation machinery, and
consolidation nine is where they either get fixed once or get compressed into the next
base document as settled. So: it earns its keep, on the evidence above, **provided
consolidation nine is a hardening pass**: the two sentence swaps (section 4), the
definitional-completeness line (section 3.2), closing artifacts on every item it absorbs
(`82b:50-58`, adopted), and the two held second reads (`86b`) dispatched before anything
one-pass in files 87 through 90 hardens. Op's own order of work already says it
(`86b:52-53`: "the open list, not the interesting list, is the queue"); the reckoning's
contribution is that the open list now includes the review's own record in two named places.

---

## 7. What this leaves open

- **The second read on section 2's refined naming principle**, before it hardens. I am the
  first assembled read connecting `79b` to item 5; the convention requires a second formed
  independently. The load-bearing pieces to attack: whether `79b`'s intent-pillar license
  really extends to an environment parameter (op wrote it about API aliases), and whether
  the assumption-not-witness sentence survives the environment type also being the future
  home of the NaN-on-overflow deployment mode (`78:325-326` routed that to `FloatEnv`, so
  the type is not purely an IEEE bundle forever).
- **The x86 form of the receipt assertion.** Probe 1 is aarch64 (FPCR); the MXCSR form is a
  different register with FTZ and DAZ bits and I did not build it. Cheap, and owed before
  the annotate shape's verifier claim is called portable.
- **The definitional-completeness discipline** (section 3.2) is a suggestion for the
  review's own process text, one line per ratification; it wants op's adoption or
  rejection, not silent practice.
- **The harness defect** (section 5) is outside the panel's scope and inside the
  workspace's; nothing in this review should run the orchestrator again until a run stops
  destroying the committed record.
- **The two sentence swaps** (section 4) are consolidation nine's, stated here so the next
  author cannot miss them.
- **The counting ruling and the division ruling** stay op's, exactly as file 89 left them,
  now with the second read on the layer assignment recorded (section 3.2).

Only op's calls are final, and even those go stale. Everything above is evidence and
suggestion, not a ruling.

*Grounded on: ratified (`79b` in full, `82b`, `86b`, `39b` via `37:171-179`, `78:137-150`,
`78:341-383`, `78:409-441`, `78:470-472`, `78:552-567`, `78:855-856`), settled shapes
(files 79 through 89 in full, `59:170-180`, `63:555-625`, `64:400-480`, `67b:172-184`,
`40:275-315`, `37:507-511`, `50:294-307`), compiled (`90_probes/probe_1`, built, run,
diverged and restored this session on the pinned toolchain), verified at source
(`37_probes/probe_1:169-210`, `43_probes/probe_5:15-24`, `40:281-283`,
`arvo-tensor/tests/capacity.rs:14-18` with its impl, the `IeeeDefault` grep trail, all
commands in `90_probes/OUTCOMES.md`), reasoned (the sweep, the refinement, the judgment,
the keep verdict, mine, offered as suggestions and evidence, not as rulings).*
