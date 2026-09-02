# 68. What the derive-validate-erase pipeline certifies, and what it merely names

**Author lens:** Leroy. Semantic preservation, trusted bases, what a verification claim actually
quantifies over.
**Position:** fourth file of the number-systems unit's first half, after the two cold derivations
(`65`, `66`). `67` is being written concurrently and its file did not exist when this one was
finished; its probe directory exists and I deliberately did not open it, because citing another
member's evidence without their argument is how attributions drift. Everything below is written
without `67`, and the checkpoint reader should weigh that.
**Probes:** `68_probes/`, committed with this file. p1/p1b/p2/p2b are verification reruns of `65`'s
and `66`'s instruments; p3, p4, p5 are new. All on the pinned `nightly-2026-05-28`
(`68_probes/p0_toolchain.txt`), zero feature gates anywhere.

**The assigned question.** Both cold derivations build a pipeline from op's acceptance criterion
("have the typestate derive the matching container and numeral representations, then validate, and
erase") and both claim it mechanically achievable, with a committed probe each. What does that
pipeline actually establish, what does it assume, where does the trusted base begin, and what is a
consumer entitled to conclude at runtime once nothing survives erasure to witness validity.

## 0. Gates

**Canon gate: passes, situation two.** No canon exists; `mock/canon/` is absent, `mock/crates` is
empty by the declared mutation order, and this panel is writing the first canon. Nothing binds but
op's intents in `INTENTS.md`, all of which I re-read before writing. Nothing below settles anything.

**Test gate: no suite exists to run.** The substitute is the probe discipline, and I applied it to
the probes in my surface by reading every test and assertion body in `65_probes/` and `66_probes/`
before the assigned work. Findings are in section 1 and are proportionate: these are spikes,
presumed flawed by rule, and the defects found are named for what they cost, not prosecuted as if
they were a shipping suite.

**Reported, per the standing instruction, outside the question.** `65` section 0 reported the
generated agent instructions describing the nuked tree in the present tense; commit `59cc2a94`
(08:38) acted on it and regenerated most of the surface. Residue remains: `.claude/CLAUDE.md:128-131`
still prohibits, in the present tense, imports between crates that do not exist ("Do not put
arithmetic fielded structs in `arvo-bits`", "Do not import `UFixed` / `IFixed` directly in
`arvo-graph`"). Dead-tree vocabulary presented as current instruction, one regeneration short of
fixed.

## 1. Verification of the unit's evidence, before any argument

Per the panel's rule that a probe is cited for what it proved and presumed flawed, I re-ran
everything both files rest on rather than trusting either file's account of its own instruments.

**`65`'s probe reproduces, and one evidence defect is found and repaired.** The clean compile
reproduces on the pin (`68_probes/p1_rerun_65_clean.txt`, exit 0, same toolchain hash). The
committed negative-case transcript, however, names `/tmp/negcase.rs` at line 368: **the E0277
evidence was generated from a scratch file outside the repository**, not from the committed source,
whose commented negative line sits at 376. That is exactly the class the evidence rule names: a
transcript whose source nobody can diff. The remedy is recovery, not escalation, and it is done:
`68_probes/p1b_65_negative_enabled.rs` is the committed source with the one line enabled (diff
committed in the file header's derivation, one line changed), and `p1b_65_negative.stderr` is the
E0277 against it, line numbers matching the committed text. The claim was true; it is now citeable.

**`66`'s probes reproduce, and none of their claimed outcomes had been committed.** `66`'s prose
claims a clean compile, four passing tests, and three Python results with specific counts
(1152 of 4913, 81 strings onto 31 values, the 2^-10 fixed-versus-float error). **No transcript of
any of these was committed**; `66_probes/` holds four source files and zero outputs. Rerun on the
pin: the Rust probe compiles clean as a lib and as a test binary, all four tests pass
(`68_probes/p2_rerun_66.txt`), and all three Python probes reproduce their claimed counts exactly
(`68_probes/p2b_rerun_66_python.txt`). Recovered, so the claims stand; but a file whose evidence
discipline is otherwise careful shipped every one of its measured claims uncommitted, and had any
number drifted, nothing on disk could have said so.

**Two structural defects inside `66`'s instruments, named plainly.**
`66_probes/associativity_check.py:31-39` contains a dead function (`check_associative`) whose body
is `if False` scaffolding that computes nothing and is never called; the live check
(`exhaustive_associativity`) is real. Worse, the printed CONCLUSION at lines 97-102 **hardcodes
"1152 of 4913" as a string literal** rather than printing the measured count, while the asserts
only require `len(fails) > 0`. Had the measurement produced any other nonzero count, the probe
would have exited green while printing a false number. The rerun shows the literal happens to match
the measurement, so no harm landed, but this is precisely the shape the dispatch warns about: a
conclusion attached to a probe rather than computed by it. And in
`66_probes/derive_validate_erase_pipeline.rs`, two of the four tests are tautologies:
`hot_accepts_full_range_and_erases_identically` (lines 119-124) asserts that an identity function
guarded by a validate that is literally `true` returns its input, which cannot fail; and the
positive half of `cold_accepts_and_erases_to_a_different_representation_than_hot` (line 135)
asserts the pipeline's output equals `raw.reverse_bits()` where the pipeline's erase **is**
`reverse_bits`, the same computation on both sides. The real content of that test is the
`assert_ne` and the reject arm. In a spike these are scaffolding, not a fabricated suite, and I
weight them as such; but `66`'s prose cites "all four of its tests pass" as part of its doability
claim, and two of the four are incapable of failing. The doability evidence is the compile and the
reject arm, not the test count.

**Both cold derivations are cold in the checkable sense.** `65` phase one at `4c4353a1` (08:29)
precedes its phase two at `6f162ba0` (08:35); `66` phase one at `4a856b0c` (08:30) precedes its
reconciliation. And `66`'s protocol violation is on the record with its repair: an unrelated commit
swept phase-two forward-pointers into phase one, and `45582bdc` (08:39) restored phase one to its
committed text. I diffed `66`'s phase one at HEAD against `4a856b0c` through the divider: identical.
The audit trail here is honest.

## 2. The four verbs, sorted into their bins

The pipeline is one sentence and four verbs, and the four have four different evidential statuses.
Sorting them is the assigned work, so here is the sort, each with what checks it and what carries it.

### 2.1 "Derive" is established as *declared and checked*, not as *computed*

In both probes the association from demand to representation is written by hand: `65`'s
`impl DeriveStorage for WindowM3To12 { type Storage = Biased4InC8; }` is an author-supplied pairing,
and `66`'s two `Derive<S>` impls likewise. Nothing in this unit computes "the matching container"
from a demand; the matching is asserted per instance and then **validated** per instance, which is a
legitimate architecture (a checked table rather than a verified function) but a different theorem.
A verified derivation function is proved once and covers every instance; a checked table is only as
good as the per-instance validation, and section 2.2 shows what that validation can and cannot see.
Both files say this honestly at the model scale; `65` explicitly excludes the general
width-to-container projection and correctly routes it to the container-derivation unit's ground
(the kind boundary, `53` section 3.3). No inflation here, only a distinction the canon must keep:
**op's verb "derive" is discharged in this unit's evidence as "declare, then validate the
declaration's consequences", and the computed form exists only where the container unit built it.**

### 2.2 "Validate" is two different verbs, and the two probes silently disagree on which

This is the unit's largest unnamed fork, and neither cold derivation noticed it in reconciliation,
because each checked the other's hierarchy and neither checked the other's *verbs*.

**`65`'s validate is entirely compile-time.** Every check is a `const` assertion: coverage,
round-trips, the carry-save law, the law inventory, all discharged before any program runs, per
type, quantified over the model space. Its own section 4 says so: "At compile time, because
everything being validated is a compile-time fact."

**`66`'s validate is a runtime predicate over bits.** `fn validate(raw) -> bool`, called per datum,
in a pipeline function that branches on it and returns `Option`. Its admissibility criterion
("decidable from the container's bits and the static type parameters alone") is *derived from* that
runtime reading: under `65`'s reading no such criterion is needed, because no datum is ever checked
at runtime.

Both readings are legitimate and they answer different questions. The compile-time verb validates
**the derivation** (op's Q1 enumeration, all three of whose parts, admissibility, usage,
self-validation, are compile-time acts). The runtime verb validates **a datum at an ingest
boundary**, which is not among Q1's three parts at all. Section 4 argues the two are not rivals but
role-keyed: the runtime door is mandatory at exactly one place, and only there. For the checkpoint:
**op's "then validate" has been read two ways by two blind instruments over one premise set, which
is evidence the criterion's word is ambiguous, and the canon must either split the verb or key it
by boundary.** Two files sharing every premise and diverging here is the one kind of divergence
shared premises cannot explain away.

**What the compile-time validation can actually see: one direction.** New evidence,
`68_probes/p3_mutant_overdeclared_window.rs`: `65`'s probe with one lie introduced. The storage
representation's declared window is widened to [-100, 100] (its encode/decode functions still cover
exactly [-3, 12]) and its `REDUNDANT` flag is flipped to `true`. **The entire validation suite
compiles clean** (`p3_mutant.stderr`: empty, exit 0). The coverage check `covers(rep, demand)` can
only fail when a declaration *understates*; an overstated declaration passes vacuously, and
`REDUNDANT` is read by nothing anywhere. What actually carries the guarantee in `65`'s probe is the
round-trip check, because it quantifies over the demand window **through the executable maps** and
never consults the declaration. The lesson is canon-shaped and I offer it as such: **a
representation's declared properties are worth exactly nothing unless validation runs through the
maps; validation of declarations against declarations is paper checking paper.** This is the
declarations-nothing-constrains failure from the workspace's own test-gate rule, observed live
inside the unit's best probe, and it did not matter here only because the probe's round-trip
happened to do the real work.

### 2.3 "Erase" has three parts, and this unit's evidence touches one of them

The erasure claim decomposes exactly as the panel's own Q1 material already records (`OPTIONS.md`
Q1, the clause-four decomposition): **layout** erasure, **dispatch** erasure, **operation** erasure.

**`65` establishes layout erasure and nothing else**, and layout erasure is nearly a language
tautology: `repr(transparent)` guarantees the size identity as a matter of Rust semantics, so
`size_of::<Num<R>>() == 1` is a regression pin on a language guarantee, not new evidence. Worth
having, worth nothing more. `65`'s probe also cannot exhibit the establishment story at all: `Num`
has no constructor and is never constructed, so the probe's erased type is a type with no values
and no operations, the degenerate case where erasure is free by vacuity.

**`66` establishes nothing about erasure, and its "erase" is not erasure.** Attack, with the
citations: `66_probes/derive_validate_erase_pipeline.rs` wraps no value in any type. Its
`type Container = u8` is bare, its `pipeline` takes `u8` and returns `Option<u8>`, and no
`repr(transparent)` newtype exists in the file, so there is no value-level typestate anywhere to
discard. Its `erase` for `Cold` is `reverse_bits` (line 96), a runtime re-encoding that changes
every bit, directly contradicting the method's own doc comment two lines up ("changes no bits, only
what the type system remembers about them"). And its pipeline is a runtime branch on `validate`,
which is not erasure but the opposite: the typestate lowered into a check the machine executes.
The probe genuinely proves what its better half claims, that one shape under per-strategy impls can
route to two different encodings gate-free, and `66`'s carried conclusions do not actually depend
on the erase arm. But the file's doability sentence ("the derive-validate-erase pipeline ... is
mechanically buildable today") is one third unsupported by its own instrument, and a consolidator
citing `66` for the erase verb would be citing a probe for something it does not contain.

**Operation erasure is a per-instance property, demonstrated here at one instance and refuted
elsewhere at another.** New evidence, `68_probes/p4_validate_residue.rs` with the emitted assembly
in `p4_asm_grep.txt`, on the pin at `-O`: an interior operation over a compile-time-validated
newtype (`add_trusted`) compiles to **the identical symbol** as the same operation over bare bytes;
the assembler output line is `_add_trusted = _add_bare`, the compiler aliasing the two because the
bodies are indistinguishable. A compile-time-validated constant folds to `mov w0, #123` with the
assert discharged during compilation and absent from the code. Against that, the runtime-validate
shape (`add_runtime_validated`, `66`'s pipeline in miniature) carries its check into the object
code as `orr`/`tst`/`csel`: not a branch on this target, but three instructions of residue per
ingest that the compile-time door does not pay. This is a qualitative existence demonstration and
an ad-hoc quick spike with no substance for any how-much question; nothing here is priced. What it
pins: the two readings of "validate" in 2.2 differ *observably in the object code*, so the fork is
real at the machine level, not vocabulary. And the general caution stands from the panel's own
record: `51` found the existing erasure evidence quantified over one width and breaking at
W >= 18, with the typed arm emitting *worse* code than its hand-written twin. Operation erasure is
checked per instance or it is assumed.

### 2.4 The quantifier: everything exhaustive is at model width, and the ceiling is now this panel's own

Every exhaustive law check in this unit runs at 4 bits (`65`) or over [-8, 8] (`66`). Transfer
upward rests on "uniformity of construction", which `65` states as an assumption in so many words.
Two facts bound that assumption, one re-established here, one inherited:

**The ceiling is forced by the toolchain, not chosen by the panel.** New evidence,
`68_probes/p5_const_eval_ceiling.rs`: the identical exhaustive signed-saturating associativity
count, cfg-keyed by width. At w = 6 (262,144 triples) it compiles in 3.1 seconds (`p5_w6.txt`). At
w = 9 (134M triples) **rustc refuses after 4.7 seconds under `deny(long_running_const_eval)`, on by
default** (`p5_w9.txt`, the diagnostic committed). The lint is allowable, so this is a default
refusal rather than an absolute wall, but allowing it means switching off the compiler's own guard
and paying an unpriced and rapidly growing compile cost per law per width. The prior panel's
record of the same ceiling (quadrupling per bit, refused at nine) was inherited through a workspace
rule; it is now re-established inside this panel's own probe set. **Model-width validation is
forced, and therefore the transfer assumption is load-bearing, not decorative.**

**And uniformity of construction does not by itself carry the transfer.** The workspace's own
droplist records a compiled counterexample: a property true at eight bits and false at nine, with
no forbidden feature, through const-tag container dispatch (`DROPLIST.md`, the two-mechanism
enumeration row). So the honest form of the transfer claim is conditional: model-width validation
transfers to shipped widths **provided no law-relevant path dispatches on width**, and that proviso
is a construction discipline stated in prose, checked by nothing. It belongs on the trusted-base
list, and it is on it below.

## 3. The trusted base, itemised

A proof does not eliminate trust; it relocates and shrinks it, and the value of the pipeline is
measured by how small and how explicit this list is. As of this unit, a canon sentence reading
"the typestate derives, validates, and erases" rests on:

1. **rustc's const evaluator and trait solver**, the checker discharging every `const` assertion
   and every E0277 refusal. Trusted, not proved, and its evaluation budget is itself a gate (2.4).
2. **The `repr(transparent)` layout guarantee.** Language semantics, trusted; the size assertions
   are regression pins on it.
3. **The ban list holding** (`dyn`, `TypeId`, full `specialization`), which is what makes
   monomorphisation the dispatch. Trusted, and *known insufficient* for width transfer on its own
   (the droplist's third-mechanism counterexample).
4. **The model-width transfer proviso**: no law-relevant path keyed on width. Prose discipline,
   unchecked, with a recorded counterexample class. The single most load-bearing unchecked item.
5. **Validation-through-the-maps rather than through declarations.** Where validation reads
   declared constants, it checks paper against paper and cannot see an overstatement (p3). The
   probes' guarantees hold because their round-trips run the real functions; a shipping design must
   make that the rule, not the accident.
6. **The construction perimeter of every erased type.** Private fields, doors that establish
   membership, and no path to a value that skips them. `65`'s probe has no doors at all, so this
   entire obligation is currently discharged nowhere in the unit.
7. **Rust's soundness boundary**: any `unsafe` anywhere in the shipped program, and every FFI
   crossing, is outside every guarantee the typestate states.
8. **Transcript-to-source correspondence.** Broken once in this unit (`65`'s `/tmp/negcase.rs`
   transcript), repaired here; the class stays on the list because a transcript of an uncommitted
   file is trust wearing evidence's clothes.
9. **Operation erasure per instance.** Demonstrated at trivial instances (p4's symbol aliasing),
   refuted at a nontrivial one (`51`), unpriced everywhere.

Items 1, 2, 7 are permanent residents of any Rust design's base. Items 4, 5, 6, 9 are the ones a
canon can shrink, by demanding the checks that discharge them, and naming them is what makes that
demand possible.

## 4. What a consumer is entitled to conclude at runtime, and what they are not

The assigned question's sharpest clause: if validity is decided before erasure and nothing survives
erasure to witness it, what does a consumer hold at runtime?

**The entitlement, stated as the conditional it is.** If (i) every construction of a value of
erased type T passes through a door that establishes membership, (ii) every operation producing T
preserves membership, as validated at the model width and transferred under item 4's proviso,
(iii) no unsafe code anywhere in the program forges a T, and (iv) the compiler correctly implements
const evaluation, monomorphisation and `repr(transparent)`, **then** every value of T observable at
runtime satisfies T's membership predicate and was produced by a validated operation, and observing
it costs nothing beyond the operation itself (p4). That is a real and valuable theorem, and it is a
theorem about **reachable values of a type inside one process**, quantified over the construction
perimeter, conditional on four antecedents each of which sits on the trusted-base list and each of
which has a recorded failure mode in this workspace: open perimeters (the what-you-can-observe
rule's worked case), width-keyed dispatch (the droplist), unsafe forging (Rust's boundary), and the
compiler itself.

**Not entitled, one: no runtime validity test exists unless the design deliberately retains one.**
The membership predicate erased with the types. A consumer holding eight bits cannot ask "is this a
valid numeral of R", because the question's subject no longer exists in the program, unless the
door is shipped as a function. This is where `66`'s runtime verb has its correct and only home: an
**ingest door** at each boundary where bits arrive without their establishing history, namely
deserialization, FFI, and packed-boundary reads (where `16`'s record shows what an assumed door
does: a data-dependent blindness that returns the right answer whenever the truncated bits happened
to be zero). Inside the process, between doors, the compile-time verb governs and no check runs.
The two verbs of 2.2 are therefore both needed, keyed by boundary, and neither subsumes the other.

**Not entitled, two: stored bits are not self-describing, so interchange validity is conventional.**
`65`'s sharpest sentence, the format does not determine the system, has a runtime corollary neither
file drew: after erasure, the byte 0x0F is simultaneously a valid numeral of Z/256, of a checked
window, of a bounded chain, of GF(2)^8 and of a Boolean lattice, and **witnesses none of them**. A
stored value is evidence of nothing without its type, and the type does not travel with the bits.
Within a process the type context is carried by the program text and re-established at every
monomorphisation, so storage-role canonicity (bit equality is value equality) is meaningful. The
moment bits cross an interchange boundary, the system must be re-established by agreement, out of
band: a schema, a protocol, a convention. So the three roles `65` proposes differ precisely in
**who re-establishes the invariant**: compute, the compiler, per monomorphisation; storage, the
type system, within the process; interchange, an external agreement plus a mandatory runtime door.
Canonicity at interchange (`65`'s candidate 7) is necessary and not sufficient; the other half of
the interchange representation is the system identification, and it is not made of bits the
typestate controls. I offer this as the role model's completion, ONE EXPERT, and it dissolves the
2.2 fork into structure rather than adjudicating it.

**Not entitled, three: any cost claim.** Operation erasure is per-instance (2.3), and nothing in
this unit is priced.

**For the checkpoint, flagged rather than resolved:** op's Q1 answer enumerates three validation
parts and all three are compile-time acts. The runtime ingest door is a fourth thing his
enumeration does not contain, it is forced by erasure itself the moment bits cross a boundary, and
nobody has asked him whether "then validate" was meant to cover it. That question is now sharp
enough to ask.

## 5. Does erasure constrain which number systems are admissible?

The dispatch names this live. `66` derived one clause (validity decidable from bits plus static
type, nothing environmental) from its runtime reading. Deriving from the erasure requirement
itself, with no commitment to either reading of validate, three clauses fall out, because "no
runtime residue" quantifies over three different things a value could otherwise drag along:

- **Information residue**: a value at rest carries only its container's bits, so a system's
  per-value runtime information must be bounded by a const container. Excludes unbounded carriers
  at rest (exact rationals with growing terms, arbitrary-precision integers, unbounded
  continued-fraction depth); bounded windows of each remain admissible. `66` reached this via
  no_alloc; it also follows from erasure alone.
- **Dispatch residue**: which operation runs is decided at monomorphisation, so a system whose
  operation selection depends on runtime facts about the value (dynamic radix, dynamically shaped
  numerals) is out.
- **Consultation residue**: validity and correctness are functions of (type, bits), so a system
  whose legality consults the environment (locale, configuration, clock) is out. `66`'s clause.

These three are exactly arvo's standing bans, no alloc, no dyn/TypeId, no environmental
dependence, restated at the value level. **The erase clause is not a new constraint on the design;
it is the semantic content of the ban list, and the bans are the erase clause enforced in the
vocabulary.** Offered as a candidate canon observation, ONE EXPERT. Two boundaries of it, stated so
it is not over-read: erasure constrains **representations and formats**, not law inventories; any
finite system whatsoever can be hosted, and no law set is excluded. And chain-scale demands that
look like unbounded state (I7's accuracy across chains) are re-admitted by `60`'s window mechanism,
whose capacity is static, which is the same criterion applied at the schedule level rather than an
exception to it. One genuine consequence for the open ring: a system is admissible exactly when its
*ingest predicate* is writable as a pure function of (type parameters, bits), which is a sharper
admission clause than "has a validity predicate" because it is placed where section 4 shows the
predicate must actually run.

## 6. The hierarchies: same upper cut, and both undercount the bottom

`65` cuts three levels (system, representation, format), `66` cuts five (number, system, scheme,
format, container), and `65`'s reconciliation table maps them onto each other and onto unit two's
(D, Q) plus realisation. I re-derived the mapping and it holds above the bottom tier; the residues
`65` names (which layer the word "system" points at, where "numeral" lands) are naming questions
for op, correctly filed. One attack neither file made, and it lands on both:

**Both hierarchies are value-centric, and the container tier fails on the panel's own strongest
aggregate result.** `66`'s level five defines container as the physical housing of *a value's*
bits, whose change leaves numeral meaning intact; `65`'s format pins a representation to widths,
field positions, alignment. But the container-derivation unit established, at TWO EXPERTS, that
`Cold` **has no standalone value form at all**: packing is a statement about how a run composes,
the derivation answers a per-value question and a per-aggregate question ({carrier, stride}), and
the load type for one packed element is a third derived quantity that is neither
(`OPTIONS.md`, the derivation's-outputs section). For a packed element there is no per-value
container to occupy a hierarchy level: its bit position is a property of the aggregate, cycling
through phases, and "the container" is two answers keyed differently. So the bottom of both ladders
is not a level but a pair of questions, and a canon that adopts either five-level or three-level
wording as written will be silently wrong about the storage role's flagship case. The repair is
small and available: keep the shared upper structure both files derived, and state the bottom tier
in the derivation unit's own two-question form rather than as a level. This composes with, rather
than contests, op's criterion: his sentence names "container" in the singular and "representations"
in the plural, and the unit's evidence says the container answer is itself plural one level down.

## 7. Attacks and supports, with rungs kept honest

**Supported, as a third read with its own derivation (not a third independent instance; I read
everything first and say so):** the number/numeral/system separation; the format-does-not-determine-
the-system sentence, which section 4 extends to its runtime corollary; the concept-closed-inventory-
open shape with the admission contract; redundancy as role-keyed (the carry-save law check in
`65`'s probe is real, exhaustive, and has an independent oracle path; verified by rerun).

**Attacked and standing:** `66`'s erase arm (2.3: not erasure, contradicts its own doc comment,
and the file's doability sentence over-covers it). `66`'s tautological test arms and hardcoded
conclusion string (section 1). The evidence gaps in both files (uncommitted outputs; the `/tmp`
transcript), all repaired by recovery per the workspace rule, none escalated.

**Attacked once before, seconded now, which makes it two:** `65` phase two's attack on
`66:60-68`'s use of the generated crate table as a "genuine cross-check". I second it from the
mutation-order rule directly: the crate table describes the dead tier, a hierarchy "predicting" a
dead design's layering is corroborated by nothing, and the paragraph should be struck or
downgraded to "consistent with the prior attempt". `66`'s hierarchy needs no help; it stands on
op's own two nouns.

**A claim the unit carries that has zero probe instances, said so nobody inherits an inflated
rung:** "the strategy is a parameter of the correctness relation" (I9, `65` candidate 3, `66`'s
identity result, unit two's C4). The semantic evidence is real: policy provably changes the law
inventory, measured exhaustively many times over. But no probe in this unit instantiates *strategy
selecting the correctness relation*: `65`'s probe contains no strategy at all (its two impls are
keyed by **role**, storage and compute), and `66`'s probe's strategy selects the **encoding**
(`reverse_bits`), which is realisation, the one layer unit two established strategy-independent for
identity. So the probes sit beside the claim without instancing it, and one of them mechanically
instances something the claim's own framework files elsewhere. The claim's support is I9's words
plus the policy-changes-laws measurements plus an inference connecting them, which is a fine rung
to hold, and it should be written down as that rather than as probe-backed.

## 8. Fits against the register

**Kills nothing.** Fits and additions:

- **Q1** fits well and gains the fourth validation moment: the runtime ingest door, forced by
  erasure at interchange and packed boundaries, outside op's three compile-time parts, flagged for
  him in section 4. It also gains p3's direction result: the self-validation part is real only when
  it runs through the maps; declaration-against-declaration checking is vacuous in the overstating
  direction.
- **Q4's soundness-or-bestness fork** gains a sibling at the validation layer: soundness of a
  declared derivation is checkable per instance (round-trip through the maps); "matching" as
  bestness is a computed-derivation claim this unit has no evidence for either way.
- **The derivation's-outputs section** gains section 6: both new hierarchies undercount its
  two-question bottom tier, which is corroboration for the per-value/per-aggregate framing from a
  direction that was not looking for it.
- **Q16 sense two** and the interchange role gain section 4's corollary: interchange validity is
  conventional; the system identification travels out of band or the stored bits mean nothing.
- **The trusted-base framing** (section 3) is offered to the eventual consolidator as the honest
  form of any "no caveats left" sentence, per the same shape Q1 already carries from `17`: an
  enumerated explicit base, never a verdict of none.

## 9. Candidate canon sentences

Each ONE EXPERT, offered to the consolidation, phrased to survive the permanence test:

1. **Validation is two acts keyed by boundary.** At derivation, compile-time, per type: the
   declared representation is checked through its maps, exhaustively at a model width. At ingest,
   runtime, per datum: bits arriving without their construction history pass a membership door
   that is a pure function of the type's parameters and the bits. Between doors, nothing checks
   and nothing needs to.
2. **Validation validates maps, not declarations.** A property a representation declares about
   itself is canon-relevant only where a check ties it to the encode and decode maps; a declared
   constant nothing reads is a comment with a type.
3. **Erasure's guarantee is a conditional, and the canon states its base.** What survives erasure
   is the theorem that reachable values of a type satisfy its predicate, conditional on a closed
   construction perimeter, on validated operations, on the transfer proviso, and on the language's
   own guarantees; the canon lists these, because a guarantee whose perimeter is unstated is not
   one.
4. **Stored bits are not self-describing.** One format hosts many systems, so after erasure a
   value witnesses its system only through its type; interchange therefore carries the system
   identification out of band, and canonicity at interchange is necessary but not sufficient.
5. **The erase clause and the ban list are one statement.** No information residue, no dispatch
   residue, no consultation residue: bounded const containers, monomorphic operation selection,
   validity from (type, bits) alone. This admits every finite system and constrains only how one
   may be represented.

## 10. Coverage, bounded honestly

**Read end to end:** `INTENTS.md`, `RULES.md`, `65` (both phases), `66` (both phases), `63`, `64`,
`OPTIONS.md` in full, `DROPLIST.md` in full, `00_brief.md` not re-read this dispatch (its
acceptance-criterion quote is taken from the dispatch and from `65`/`66`, which agree on it).
**Read at the source:** every file in `65_probes/` and `66_probes/`, including every test and
assertion body. **Not read:** `67` (does not exist; its probe directory exists and was deliberately
not opened), files `01` through `62` except as reached through `63`/`64`/`OPTIONS.md`/`DROPLIST.md`;
every statement here about `16`, `17`, `51`, `53`, `60` is sourced to the register or the
consolidation and inherits their errors if any.

**Re-ran:** `65`'s compile (clean), `65`'s negative case against committed source (E0277,
repairing the `/tmp` transcript gap), `66`'s Rust probe (lib, test build, four tests), `66`'s three
Python probes (all counts reproduce). **Built:** the overdeclaration mutant (p3), the residue
demonstration (p4), the const-eval ceiling (p5). All committed in `68_probes/` with transcripts.

**Not done, and what it leaves open:** no attack on the H1/H2 frame (out of this unit's scope and
still the panel's most attack-worthy ONE EXPERT claim); no probe of the ingest door for a packed
boundary (section 4 argues it is mandatory; nobody has built one; `16`'s data-dependent blindness
says what a wrong one looks like); no pricing of anything, and every cost-flavoured remark here
says so; and the strategy-selects-correctness claim still has no probe instance, which is the
cheapest constructive item the unit's second half could build: one probe where a strategy
parameter selects among reduction members over one fixed (D, Q), refusing an algorithm bound when
the selected member loses the law. That would put I9's attachment point on the same footing as
everything else in this unit, and nothing currently occupies the slot.

**Nothing here settles anything.** The mode is explore; this file goes to the checkpoint beside
`65`, `66`, and `67` when it lands, and sections 4, 6 and 7 are what the unit's second half should
argue with.
