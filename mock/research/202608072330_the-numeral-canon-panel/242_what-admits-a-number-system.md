# 242. What a number system is, and what admits a candidate

Seat 242. Cold open on the admission subject: `is_the_number_system_inventory_open`,
`is_admission_a_predicate_or_a_location`, `is_number_system_broad_enough_for_non_magnitude`,
`are_set_valued_carriers_admitted`, `one_word_or_two_for_is_a_number_system`, and
`what_the_admission_contract_asks_a_candidate_to_expose`.

Written blind. The first commit on this branch landed the premise check and the governing rows
before anything else, and this file was finished before anything under `mock/research/` was
opened. Section 9 lists every path I opened. The reconciliation is section 10 and is later.

## 0. The brief's premises, checked before anything else

Three claims. Two hold, one does not.

**Holds.** `mockspace.toml` declares `canon_paths = ["mock/registry/*.toml"]`, so the registry is
the canon and there is no canon prose directory. `mock/crates/` holds exactly three crates:
`arvo-format`, `arvo-placement`, `arvo-strategy`.

**Holds.** `question.where(topic=the_number_system).count()` returns 18, and all six named rows
resolve once `what_the_admission_contract_asks_a_candidate_to_supply` is corrected to its real
id, `..._to_expose`. The brief misnames it. Minor.

**Does not hold.** The brief says: "Read each row's own `asks`, `options` and `bound`; the `bound`
field frequently carries a constraint that has already closed part of the question." **Not one of
the eighteen `the_number_system` rows carries a `bound` field.** 24 of the registry's 105 question
rows carry one; none is in this topic.

Worth more than a correction, because the field that does that job exists under another name.
`is_the_number_system_inventory_open` carries an **`answered`** field recording that
`ruling::the_format_spine_is_canon` already settles it. Two of the eighteen have one: that row and
`adaptation_in_identity_or_realisation`. A member taking the brief at its word would look for a
constraint on all six, find none on any, and conclude nothing prior bears on them. One of the six
is already closed and a second, I will argue in section 6, is closed by a sentence nobody has
recorded against it.

I flag it and continue rather than stopping, because the false premise is about **where to look**
and not about what is true: it frames no answer, and the right field is one query away. Had it
asserted a conclusion I would have stopped.

## 1. Two things I found that are outside the question, reported because the standing instruction says to

**A provenance hole in the canon's own top tier.** `ruling.where(rung=ratified).count()` is **32**.
`ratified_by=op` is 7, `=experts` is 5, `=both` is 2. **Fourteen of thirty-two ratified rulings
name a ratifier and eighteen do not.** My brief says `ratified` governs and is defended rather
than weighed, and that `ratified_by` distinguishes op's stamp from a two-expert convergence. Both
cannot hold of a row at `ratified` with no `ratified_by`: it claims the governing tier while
stating no human and no convergence was in the loop, which is the exact condition this workspace
uses to presume an artifact wrong. Two readings, and I do not resolve between them:

- The field is optional and its absence marks rows ported from prose before the convention.
  Distinguisher: whether every ratified row lacking it has a pre-registry `provenance`.
- The field is load-bearing and eighteen rows are defective. Distinguisher: any lint or check that
  reads `ratified_by`, or any such row whose `because` claims a convergence the field omits.

**The dimension namespace cannot express the region of any canon-tier finding.** All 24 declared
axes are about numeric computation: widths, rounding, strategy, threads, radix, alignment. There
is no axis for which revision of the registry a claim was measured against. So every finding in
this panel that is *about the canon* is unpredicatable in the declared grammar, and the predicate
discipline's "absent holds nowhere" makes that a real gap rather than a stylistic one. I state
registry claims against a commit below and mark that the grammar does not sanction it.

**And one process slip of my own, stated because it is the kind of thing I would report in
somebody else.** I reached for a one-line `python3` heredoc to edit this file mid-draft.
`no-python.md` bans that absolutely, for anything written from now on. No output of that command
survives in the tree; the file was rewritten from a shell heredoc. Recording it rather than
quietly not mentioning it.

## 2. What governs, quoted rather than remembered

`ruling::the_format_spine_is_canon`, `rung = ratified`, `ratified_by = both`, four propositions
ratified as one shape. Two are load-bearing here:

> A format is identified by its ambient domain and its representable set, and that set is a
> constant of the type. Membership in it is one affine predicate over one parameterisation, of
> which integers, fixed point, scaled integers and floats are points. Arithmetic on a format is an
> exact operation in the ambient domain composed with a named total adaptation onto that set, and
> the adaptation is a first-class object with its own laws. **The concept is closed and the
> inventory of admitted instances is open.**

His condition travels with it, in his own words on the row:

> I stamp them but all of these are subject to being changed if the experts themselves end up
> disagreeing or finding a better solution or wording or bound.

`proposal::the_concept_is_closed_and_the_inventory_is_open` carries the operative sentence, and
note its topic is **`the_number_system`**, not `the_format`: it is my topic's proposition, ratified
through the format ruling.

> The canon defines once what a number system is and what admission requires; the set of admitted
> instances is open, and a new one earns admission by supplying the concept's obligations rather
> than by amending the canon.

`ruling::the_numeric_door_carries_the_coordinate_set_and_the_two_type_bound_is_not_canon`,
`ratified`, `experts`: the door carries out "the coordinate set of the ratified parameterisation,
spelled in types the stack owns", and explicitly does **not** say how many types that is.

Two open deferrals sit on my topic, both his, both `rung = open`:
`the_family_question_wants_the_comparison_first` and
`his_instinct_on_one_family_is_not_to_be_acted_on`. The second records an instinct for one family
and instructs that it not be acted on. I take it at its word and use it as evidence in neither
direction.

## 3. The parameterisation is ten coordinates, and I counted them in source

`arvo-format` carries exactly ten associated items across four traits:

| trait | coordinates |
|---|---|
| `Ambient` | `RADIX`, `SIGNED` |
| `Quantum` | `BASE`, `SLOPE`, `MAGNITUDES` |
| `Slots` | `MIN`, `MAX`, `WIDTH` |
| `Format` | `PHASE_NUM`, `PHASE_DEN` |

An `impl Format for X` fixes all ten. That is what a candidate does to join.

## 4. The finding: admission is a check over three of the ten coordinates and a declaration over seven

The ratified clause says a candidate "earns admission by supplying the concept's obligations", and
that closing the concept while opening the inventory "is what makes admission a check rather than
a negotiation". `Slots::ADMITTED` mechanises exactly that, and says so in its own doc, citing the
proposal by id. It is the right shape. It is also the **only** one: `Ambient`, `Quantum` and
`Format` carry no obligation at all.

I built `242_probes/admission/` to find out what that costs. Five arms, one per invocation, each
built and run rather than checked, because the one obligation that exists fires at codegen and
`cargo check` skips it. `output.txt` is committed beside the source.

| arm | what it declares | result |
|---|---|---|
| positive | all ten coordinates, from outside the crate, at values no shipped point uses | **built and ran** |
| `phase_den_zero` | `PHASE_DEN = 0`, a phase of one over zero | **built and ran** |
| `radix_one` | `RADIX = 1` | **built and ran** |
| `magnitudes_zero` | `MAGNITUDES = 0`, so `contains` is false everywhere | **built and ran** |
| `inverted_slots` | `MIN = 40`, `MAX = -40` | **refused at codegen**, named message |

The last row is the instrument's own control and it is why the other four mean anything: without a
refusal the probe could not detect one, and every "built" would be worthless. It refuses at
`slots.rs:94` with `slot range is inverted: its lowest index exceeds its highest, so it admits
nothing`.

**The positive arm is the good news and it is worth stating on its own.** An outside crate wrote
radix 7, a slope-two quantum the crate ships no family for, an asymmetric slot window `[-11, 20]`
that is neither `Signed<N>` nor `Unsigned<N>` and whose span is not a power of two, and a half-step
phase. Every derived quantity resolved and the obligation forced clean. The ratified open-inventory
clause is **true in fact and not only in prose**, and it is now checkable rather than asserted.

**The three that built are the finding.** Each is a ratified coordinate with nothing enforcing it:

- **`MAGNITUDES = 0` admits a format with an empty representable set**, and `has_additive_identity`
  returns **true** of it. The crate says a set with no members contains zero. The ratified spine
  identifies a format *by* its representable set; a format whose set is empty is not identified by
  it, and the predicate that is supposed to say what is in the set says everything is out while
  admission passes. This is the worst of the three, because the failure is inside the ratified
  sentence rather than beside it.
- **`PHASE_DEN = 0` is admitted**, against its own doc's "One for an unbiased grid, two for the
  half-step bias. Never zero." And the reason nothing enforces it is worse than an oversight:
  **`PHASE_DEN` is read by no function anywhere in the crate.** It is written by four shipped
  points and one test and read by none. That is `the-test-gate.md`'s "declarations nothing
  constrains" exactly: a constant a type declares about itself that no code reads can be given any
  value and everything still compiles. Ask what value would make a law fail, and the answer is
  none.
- **`RADIX = 1` is admitted.** The ratified predicate is `phase + slot * quantum(magnitude)` with
  the quantum a power of the radix, so at radix one every quantum is 1 whatever the exponent, the
  magnitude axis carries no information, and the affine grid degenerates to the slot range. Nothing
  in the crate computes `radix^exponent` yet, so today this is a latent collapse rather than a
  wrong answer, and I state it at that strength rather than higher.

I want to be precise about what this does and does not impeach. It does **not** impeach the
ratified spine: the spine is a claim about what a format *is*, and the probe agrees with it. It
impeaches the sentence "admission is a check rather than a negotiation" as a description of what
is built, and it tells Q29 something its own option list does not contain.

**Two smaller things the probe turned up.** `is_admissible`, the free verdict function whose whole
documented purpose is letting a wrong construction "live permanently in a test rather than in a
scratch file somebody deletes", is not re-exported at the crate root, while `slot_count` and
`slot_in_range` declared beside it are. Reachable as `arvo_format::slots::is_admissible`, so this
is an omission rather than an inaccessibility, but it is the one function an outside implementor
most needs. And `the_format_inventory_admits_a_member_this_crate_does_not_know_about`, the only
shipped test asserting the ratified open-inventory clause, declares a `Ternary` that borrows the
crate's own `DecimalRationals`, `Constant<-1>` and `Signed<3>` and writes **two of the ten
coordinates**. Its own assertion is `radix::<Ternary>() == 10`, so the type is named for a radix it
does not have. My positive arm is what that test should be.

## 5. Q30, and why it is one word doing two jobs

Q30 asks whether admission is a predicate or a location. Measured, the shipped tree has both, at
two tiers the question does not separate:

- **Value tier.** `contains::<F>(slot, magnitude) -> bool`. A predicate over the elements of an
  already-located set. The canon has **already ratified this**: "Membership in it is one affine
  predicate over one parameterisation."
- **Candidate tier.** `impl Format for X` fixes ten coordinates. A location.

And the candidate tier is not free to be a predicate, because the two ratified clauses jointly
force it. An **open inventory** means membership cannot be an enumeration. A **closed concept**
means the thing membership is checked against is fixed. A fixed thing that classifies an open set
of instances without enumerating them **is a coordinate system**. So "the concept is closed and the
inventory is open" has one joint model and that model is a location.

So Q30's answer is **both, at two tiers**, and the question reads hard because one word spans them.
Its own third option, "a location for membership and a predicate for hosting", is close and splits
on the wrong axis: the real split is candidate-tier against value-tier, not membership against
hosting.

**One stated cost of the location option is overtaken.** The row says a location "presumes a
coordinate count the canon may not want to commit to". The canon has committed:
`ruling::the_numeric_door_carries_the_coordinate_set_and_the_two_type_bound_is_not_canon` is
ratified by experts and says the door carries out the coordinate set of the ratified
parameterisation. What that ruling leaves open is the number of **types**, which is a different
count from the number of **coordinates**, and the second is ten in shipped, tested code. A reader
conflating them will read the ruling as leaving the coordinate count open, and it does not.

The row's own note supports this from the other side: "Most of the disputed cases, a Gray code,
two's complement and a stride, are coordinate choices rather than rejections." Under a location
that is the expected shape rather than an awkwardness, because a location has nothing to reject
with. Its stated discriminator, whether any canon sentence must quantify over non-members, I could
not close: I found no such sentence, and an absence I searched for by grepping the `says` fields of
`ruling` and `proposal` is weak evidence at best, so I do not claim it.

## 6. Q22, which I think a ratified sentence already answers

The spine ratifies: "The representable set is a constant of the type: **a value set that depends on
other data is not a format but storage**."

An interval value is a pair of grid points and its endpoints are runtime data, so the value set it
denotes depends on other data, so it is storage. That reading answers Q22 "scoped out" from a
sentence already ratified, in the same way Q20 was answered, and no `answered` field on the row
records it.

But the other reading is real and I will not pretend otherwise:

- **Reading A.** An interval *value* is a pair over a grid. Intervals are **composed of** formats
  rather than being one. Scoped out.
- **Reading B.** The *format* is "intervals over grid G", which is a constant of the type. Admitted,
  and the carrier is a set of sets.

**A discriminator neither the row nor its note has.** Under B, `contains` must answer whether a
*pair* is in the set, and the ratified affine predicate has exactly **one** slot coordinate. So
reading B cannot be expressed without widening the ratified parameterisation and reading A can be
expressed today with nothing added. That is checkable by counting coordinates, where the row's own
stated discriminator ("whether certified accuracy should be expressible inside the system concept
or beside it") is a preference nobody can gate on.

I lean A and say plainly it is a lean, and I state its cost rather than burying it. The spine's own
`because` says the adapt-exact factoring exists because "the error of a computation is the
composition of adaptation errors, and the adaptation error is a property the format can state".
Under A a format can state its adaptation error and a pair carrying a *certified* bound cannot be a
format, so certified accuracy lands outside the vocabulary. Whether that costs a real consumer
anything is a question I have no instrument for.

## 7. Q21 and Q33 are one question, and it is not about breadth

Q21's own note says the two should be read as one and that its stated discriminator was measured
not to cut. Here is a different angle.

**Ask whether the two-element Boolean algebra can be given ten coordinates.** It can, trivially:
radix 2, unsigned, `BASE = 0`, `SLOPE = 0`, `MAGNITUDES = 1`, slots `0..=1`, width 1, phase `0/1`.
That is `Unsigned<1>` over an unsigned radix-2 ambient and the crate admits it today. So the
**representable set** half of the concept is already broad enough for the non-magnitude case and
has been since the spine was ratified. The two-element set is an ordinary point of the affine
predicate.

What excludes Boolean algebra is not the set, it is the **ambient domain**: GF(2) under xor is not
the rationals restricted to `{0, 1}`, because the operations differ. So Q21 is not a question about
the concept's breadth at all. It is a question about whether `Ambient` names a *domain* or an
*algebra*, which is Q33 wearing Q21's clothes, and reading them as one is right for a sharper
reason than the note gives.

**And measured, Q33's option list is missing the shipped answer.** It asks whether the ambient
operation family is "fixed at addition and multiplication, or a parameter", which presupposes the
operations are stated somewhere. They are stated **nowhere**: `Ambient` carries `RADIX` and
`SIGNED` and nothing else, and no trait in the crate carries an operation family. So the ratified
sentence "Arithmetic on a format is an exact operation in the ambient domain" names something the
type surface does not carry. Same shape as section 4's three: a ratified clause whose subject is
absent from the types.

Two readings, and the evidence does not force one:

- **(a) The operation family is a further coordinate on `Ambient`.** Boolean, GF(2) and the tropical
  semiring become ordinary members and Q21 dissolves. Cost: the ratified door ruling names "the
  coordinate set of the ratified parameterisation", so widening it moves a ratified boundary and
  the door's contents change.
- **(b) It belongs on `Operation` / `DeclaredSignature` in `adapt.rs`**, which already exists and
  already carries `ARITY`. Cost: two systems with the same ten coordinates and different arithmetic
  become one format, contradicting the spine's own `because`, "the same representable set under two
  ambient algebras is two formats", which is the sentence justifying carrying the ambient at all.

What would distinguish them is whether any consumer needs one generic algorithm spanning two
operation families. I could not measure it; see section 8.

## 8. Q31, where I have one thing to add and one concession

The row's note already argues the distinguisher cuts, and that option 1 makes the concept "exclude
unbounded exact rationals as a matter of mathematics, which is false". I agree two words is close
to forced and will not restate the note. Two things it does not carry.

**The word "obligation" is already spent.** The registry has an `obligation` namespace of sixteen
rows meaning "things arvo must be able to do": `set_operations_over_a_fixed_size_bit_set`,
`a_content_hash`, `a_cost_dynamic_program`. The ratified admission clause uses the same word for a
different thing, "supplying the concept's obligations", and `Slots::ADMITTED`'s doc calls its
assertions "what an implementor owes". Whichever two words Q31 picks, using "obligation" for the
admission half collides with a live namespace. That is a hard constraint on the answer and no
option in the row carries it.

**"Number system" is a borrowed phrase that already means something arvo ratified out.** The
ordinary definition treats a number system as a set of digit values plus an interpretation rule
mapping digit sequences to values, that is, the encoding is part of the definition. The spine
ratifies the opposite: "Adaptation choice and encoding are realisation ... and not part of
identity." So arvo's concept is not what the phrase denotes elsewhere, and using it unqualified
tells a reader something the canon has ruled out. That argues for two words more strongly than the
hosting argument does, because it is a defect in the *first* word rather than in the second.

**Concession.** I reached for "a number system" and "carried" as the pair, and "carried" is wrong:
`arvo-placement` exists and its own lib doc says "where the bits go is `arvo-placement`", so
"carried" already leans at placement and would confuse the two tiers this whole file is about. I do
not have a better second word. Someone should pick it who has read the retirement namespace, which
I did not.

## 9. What I settled, and the shape I think the subject actually has

Consolidating, and this is the part I would most want attacked.

**Four of the six read hard because one word spans two tiers, and it is the same missing
distinction each time.**

| question | the two tiers it spans |
|---|---|
| Q30 | value-tier membership against candidate-tier admission |
| Q21 | set-tier breadth against domain-tier operations |
| Q22 | member-tier (an interval value) against family-tier (intervals over G) |
| Q29 | what a candidate declares against what it owes |

That is not four hard questions. It is one missing distinction seen four times, and **the place it
belongs is Q19**, `are_the_level_hierarchies_the_same_cut`, which is in my topic and outside my
six. Its two options are three levels (system, representation, format) and five (number, system,
representation scheme, format, container).

So my honest answer to "derive the subject as a whole" is: **the admission subject does not close
until the level cut does**, and I would rather say that than manufacture six answers over a
distinction nobody has made. Q20 is closed by ratification; Q22 is closed by a ratified sentence
nobody has recorded against it; Q30's shape falls out of the two ratified clauses jointly; Q21 and
Q33 collapse into one question about `Ambient`; Q29 gains a fourth option below. None of that needs
the level cut. Choosing *which tier the phrase "number system" names* does, and Q31 cannot be
answered without it either, because a word is chosen for a tier.

**The fourth option for Q29, which is my main constructive contribution.**

The three recorded options all argue about what a candidate **declares**. There are three things an
admission contract can ask and every recorded option mixes only the first two:

1. What the candidate declares. All three options.
2. What the candidate must prove **relating its declarations to each other**. Option 2's third
   verdict, partially.
3. What must be true of the declarations for **the ratified predicate to mean anything at all**.
   Nobody's option.

Category 3 is what the probe measured missing, and `Slots::ADMITTED` is a worked precedent for it
sitting in the tree already. Three obligations follow, each derived from a ratified sentence rather
than from taste:

- **`Ambient`: `RADIX >= 2`.** From "a quantum per magnitude": at radix one the quantum is 1 at
  every exponent and the magnitude axis is inert.
- **`Quantum`: `MAGNITUDES >= 1`**, and `BASE + SLOPE * (MAGNITUDES - 1)` not overflowing `i32`.
  From "identified by its ambient domain and its representable set": the empty set identifies
  nothing, and `has_additive_identity` currently lies about it.
- **`Format`: `PHASE_DEN != 0`**, and plausibly `> 0` so a phase has one spelling. From the
  coordinate's own doc, which today is a totality claim with nothing behind it, which is the exact
  class `Slots`'s doc says it was written to stop being.

**And this recasts the self-ambient collapse rather than answering it.** Option 1's sufficiency is
refuted because a candidate can name its own computed algebra as its ambient domain and satisfy the
list verbatim. Option 2 catches that with a third verdict; option 3 makes it unstateable. But
notice the collapse and the radix-one case are **the same defect**: the ambient domain is a
declaration with two constants and zero obligations, so a candidate can declare an ambient that is
not a domain, whether by naming radix one or by naming itself. **The missing half is the same
half.** That does not hand me the obligation that rules out self-ambience, and I say so in section
10; what it does is say the option list is arguing on the wrong axis, and the right axis has a
worked precedent twenty lines away in `slots.rs`.

**Prior art, which corroborates the concept and locates arvo's three generalisations.** Uniform
affine quantization is standardly three parameters, scale, zero-point and bit-width. arvo's
parameterisation is the same three generalised on each axis: scale becomes magnitude-indexed, which
is what buys floating point; zero-point becomes an exact rational, which is what buys the half-step
phase; bit-width becomes an explicit `MIN`/`MAX`, which is what buys the asymmetric window my
positive arm built. So the affine predicate is a recognised shape and the closure claim is not
eccentric. Separately, FLoPS formalises low-precision formats in three tiers, bit encodings, an
algebraic model, and a value-set semantics, with bijections between them. arvo puts encoding
*outside* identity where FLoPS keeps it as a tier with a bijection to identity. Different call,
same recognised tiering, and it is independent support for section 9's claim that the subject wants
a level cut rather than more admission argument.

## 10. What I could not do

- **The ambient-domain obligation that would rule out the self-ambient collapse.** I attacked it and
  failed. I have that the collapse and radix-one are one defect, which relocates the problem to a
  place with a precedent and does not solve it. It probably wants somebody who can say what makes a
  domain a domain without enumerating domains, which is the same shape as the whole closed-concept
  problem one tier down.
- **Q33's remaining half**, whether any consumer needs one generic algorithm spanning two operation
  families. `workspace.md` names `loimu` as the downstream real-workload reference and it is not in
  this clone. I did not clone it, because a consumer survey is a different dispatch from this one.
- **Q30's stated discriminator.** Whether any canon sentence must quantify over non-members. I
  grepped `says` across `ruling` and `proposal` and found none, and an absence established that way
  is not evidence, so I claim nothing.
- **The retirement namespace**, which bears on Q31's word choice and which I did not open. My
  concession in section 8 stands on that.
- **Search I could not reach**: I ran two web searches and no more. I did not reach the FLoPS paper
  itself, only its abstract via search results, so I cite it for the tiering it is described as
  having and not for anything inside it.

## 11. Findings, each with its predicate

The `dimension` namespace has no axis for a claim about the canon, so the registry claims are
predicated on the commit instead and that is outside the declared grammar. Said rather than
smuggled.

- **The open inventory is real from outside the crate.** An outside crate wrote all ten
  coordinates and every derived quantity resolved.
  `rustc = 1.98.0-nightly (57d06900f 2026-05-27)`, `edition = 2024`, `debug-assertions = on`,
  `radix = 7`, `ambient domain = the rationals`, tree at `b6350453`.
- **Three of the ten coordinates carry an admission obligation and seven carry none.** Established
  by construction, one arm per coordinate, with a refusing positive control.
  `rustc = 1.98.0-nightly (57d06900f 2026-05-27)`, `edition = 2024`, `debug-assertions = on`,
  `radix in {1, 7}`, tree at `b6350453`.
- **`MAGNITUDES = 0` admits an empty representable set and `has_additive_identity` returns true of
  it.** Same predicate as above.
- **`PHASE_DEN` is written by five impls and read by no function in the crate.** Established by
  construction from a full-tree grep plus the `phase_den_zero` arm building.
  `rustc = 1.98.0-nightly (57d06900f 2026-05-27)`, `edition = 2024`, `debug-assertions = on`,
  tree at `b6350453`.
- **The suite is 81 passing and 1 ignored across three crates, all green.** `cargo test
  --workspace`, `rustc = 1.98.0-nightly (57d06900f 2026-05-27)`, `edition = 2024`,
  `debug-assertions = on`, tree at `b6350453`.
- **The only shipped test of the ratified open-inventory clause writes two of ten coordinates and
  is named for a radix it does not have.** Registry-and-source claim, tree at `b6350453`.
- **No `the_number_system` question row carries a `bound` field; two carry `answered`.** Registry
  claim, tree at `b6350453`.
- **Eighteen of thirty-two ratified rulings name no ratifier.** Registry claim, tree at `b6350453`.
- **`is_admissible` is not re-exported at the crate root while `slot_count` and `slot_in_range`
  are.** Source claim, tree at `b6350453`.

Nothing above is claimed at `threads` anything, and none of it was measured with threads, so by
the standing reading it holds only where threads do not exist. That is correct for every one of
them: they are compile-time and registry facts.

## 12. Paths I opened during the blind phase

- `mockspace.toml`, `mock/Cargo.toml`
- `mock/registry/*.toml` via `cargo mock query`, plus one `grep -n` for the misnamed question id,
  one `awk` over `question.toml` for the `bound` count, and one `grep -h` for the namespace list
- `mock/crates/arvo-format/src/{lib,format,ambient,quantum,slots,adapt,width}.rs`,
  `mock/crates/arvo-format/src/tests.rs`, `mock/crates/arvo-format/Cargo.toml`
- `ls mock/registry/`, `ls mock/crates/`, `find mock/crates -name '*.rs'`
- `cargo test --workspace`
- My own `242_probes/admission/`
- Two web searches, listed in section 10

**Nothing under `mock/research/` and nothing under `mock/design_rounds/`. No other branch. No
`git log` beyond my own two commits and `git rev-parse` on `HEAD` and `origin/dev`.**


---

# Reconciliation, written after reading `mock/research/`

Everything above was committed at `64ab711e` before this directory was opened. This section is
later and says what I found, what I would change, and what I would not.

## What I would withdraw as new, and keep as a second instance

**`PHASE_DEN` is read by no function in the crate.** Seat 237 found this first, at
`237_the_format_proposals_against_the_ratification_gate.md:459-465`, and found it the same way and
filed it under the same rule, down to the sentence "ask what value would make it fail, and the
answer is none". I withdraw any claim of priority. What survives is that it is now **two
independent instances**: 237 reached it by grepping the crate for uses, I reached it by compiling a
`PHASE_DEN = 0` impl from outside and watching every law pass. Different instruments, same fact,
and under the three-instances preference that is worth recording as a count rather than as a
discovery.

**The obligation exists at `Slots` and at no tier above it.** Seat 240 reached this independently,
at `240_the_format_layer_derived_from_its_denotation.md:572`: "The two coordinates are checked at
one tier and unchecked at the tier above it", and its `q4` probe calls its own proposal "the
format-level analogue of `Slots::ADMITTED`, which the crate does carry at the slot level". Same
structural finding, and again a different instrument: 240 swept 16728 coordinate tuples against a
raggedness oracle with two mutants; I compiled four negative controls with one refusing positive
control. **Two instruments, disjoint methods, same conclusion.**

They are complementary rather than competing, and the ordering matters. 240's obligation is that
the denoted set be a clean geometric ladder rather than a ragged union. Mine are that the
coordinates denote a set at all. **Mine are prior**: a format at `MAGNITUDES = 0` has no set to be
ragged, so 240's oracle has nothing to range over, and a format at `RADIX = 1` has `R = radix^SLOPE
= 1`, which makes "a power of `R`" true of exactly one distance and 240's tiling condition
degenerate. Whoever writes the format-level obligation wants both, with the well-formedness half
checked first. I did not verify the radix-one degeneracy against 240's oracle and state it as
arithmetic rather than as a measurement.

## What I would add, because it is new and it corrects a prior seat

**Seat 237's proposed repair does not survive the value that motivated it.** 237 writes, at `:464`:

> The correct predicate is `PHASE_NUM % PHASE_DEN == 0 && slot_in_range(0)`, and writing it is what
> would make `PHASE_DEN` load-bearing.

`242_probes/fix_check/` measures what that predicate does at `PHASE_DEN = 0`, which is the value
237's own finding says compiles today. Two arms, with the control that the predicate must evaluate
and separate the two shipped phases first, which it does.

- **In a const context** it does not return a verdict. It fails const evaluation:
  `attempt to calculate the remainder of 1_i64 with a divisor of zero`. That refuses, which is the
  right outcome, but by an arithmetic error rather than by a named obligation, which is the
  distinction `Slots::ADMITTED`'s own doc goes to some length to preserve.
- **Called at runtime it is worse, and this is the finding.** `has_additive_identity` is a free
  `pub const fn`, so it is also an ordinary function, and const evaluation never looks at a call
  that is not in a const context. `runtime_call.rs` builds clean and **panics at run time**:
  `attempt to calculate the remainder with a divisor of zero`. That is a runtime check on a lowered
  path, which `ruling::never_a_runtime_check_and_one_lowered_path` (ratified) forbids.

So the repair and the obligation are **ordered, and 237 proposed the second without the first**.
`PHASE_DEN != 0` has to be an admission obligation before `PHASE_NUM % PHASE_DEN` may appear
anywhere, or the change converts a silent unconstrained declaration into a runtime panic, which is
a worse state than the one it fixes. I do not read this as a defect in 237's work: it found the
class, filed it correctly, and proposed a repair in one clause at the end of a long file. The
repair simply has a precondition, and the precondition is the thing my probe was built to find.

**`MAGNITUDES = 0` and `RADIX = 1` appear nowhere in this directory.** Greps for `MAGNITUDES = 0`,
`empty representable set`, `RADIX = 1`, `radix of one` and `radix 1` return my own files and
nothing else. I offer them as new, with the caveat that a grep for a phrase is weak evidence of
absence and I did not read all 442 files.

Of the two, **`MAGNITUDES = 0` is the one I would put in front of a canon reader**, because its
failure is inside a ratified sentence rather than beside it: the spine identifies a format *by* its
representable set, the set is empty, and `has_additive_identity` returns true of it. A format with
no members that reports containing zero is not a marginal case of the concept, it is the concept
returning a wrong answer about itself.

## What I would change in my own derivation

**Section 6 on Q22 was written without knowing the consolidation had answered it, and I would now
put it more strongly rather than less.** `74_giesen_consolidation_the_number_system_concept.md:749`
records Q22 with both options live and says of the scoped-out reading that `65` withdrew its
version "in favour of the composition filing carried from unit two ... which is one read of that
filing, not a closure; the register entry stays live". Fair at the time.

But **the sentence I rely on was ratified afterwards.** `ruling::the_format_spine_is_canon` carries
"a value set that depends on other data is not a format but storage", and its provenance is seat
213; the consolidation is seat 74. So the governing sentence postdates the consolidation by well
over a hundred seats, and nobody has applied it to the row. That is the shape
`a-governing-claim-is-applied-where-it-hurts` names: a ratified claim gets applied to the gaps it
closes and not to the standing claims it falsifies, and the falsified ones announce nothing because
they read as settled.

I still do not resolve Q22, and my reason is unchanged and now sharper. The ratified sentence bears
on **reading A** (an interval value is a pair of runtime data, hence storage) and does not by itself
kill **reading B** (the format is "intervals over grid G", a constant of the type). My discriminator
stands and is the contribution: **the ratified affine predicate has exactly one slot coordinate**,
so reading B cannot be expressed without widening the ratified parameterisation and reading A costs
nothing today. That is checkable by counting, where the register's own stated discriminator is a
preference. The consolidation's `67`-derived note that "a point composition and a shared-parameter
aggregate are different things, and intervals are the first kind" points the same way from a
different direction, which I did not have and which strengthens A.

**Section 5 on Q30 stands and I would add one thing.** The consolidation files Q30 as "As appended"
with no derivation, so there is no prior position to reconcile against. The part I would now press
harder is the cost claim: the row says a location "presumes a coordinate count the canon may not
want to commit to", and
`ruling::the_numeric_door_carries_the_coordinate_set_and_the_two_type_bound_is_not_canon` is
ratified and commits to the coordinate set. Seat 238 and seat 239 are the two instances behind that
ruling and they disagree about the number of **types**, which the ruling says explicitly it does not
resolve. A reader who takes "the count is open" from that ruling and applies it to Q30's coordinate
count has moved a disagreement about types onto a question about coordinates. **The ten is not in
dispute in either seat.**

## What I would not change

Sections 0, 1, 3, 4, 7 and 8 stand as written. Section 9's claim, that four of the six read hard
because one word spans two tiers and the place that belongs is Q19, I still hold, and I now hold it
with one piece of support I did not have: the consolidation's own treatment of Q23 says the unit
"reshaped the question", that the closure question is "well formed for realisation roles and
malformed for a mixed set". That is the same shape as my claim about the other four, arrived at
independently on a question outside my six, by an author who did not frame it as a tier problem.
Whether it is the same problem is worth somebody attacking rather than my asserting.

## Predicates on the reconciliation's own findings

- **Seat 237's proposed phase predicate fails const evaluation at `PHASE_DEN = 0` and panics at run
  time when called outside a const context.** Established by construction, two arms plus a
  separating control. `rustc = 1.98.0-nightly (57d06900f 2026-05-27)`, `edition = 2024`,
  `opt level = 3`, `debug-assertions = off`, tree at `27ac9476`.
- **`MAGNITUDES = 0` and `RADIX = 1` are not named in this panel directory.** Absence claim, from
  five greps over 442 files at tree `27ac9476`, and weak on that account.
- **Two independent instruments converge that the admission obligation exists at `Slots` and at no
  tier above it**: seat 240's 16728-tuple sweep and seat 242's compile-and-run controls. The
  intersection is the tier claim. They do not intersect on which obligation belongs above it, and
  nothing here ratifies a count.

## Paths I opened after the blind commit

`ls` of the panel directory; `240_probes/q4_output.txt` and the head of
`240_probes/q4_the_obligation_the_format_does_not_carry.rs`; lines 455-470 of `237_...gate.md`;
lines 744-772 and greps of `74_giesen_consolidation_the_number_system_concept.md`; grep hits across
the directory for `PHASE_DEN`, `ADMITTED`, `MAGNITUDES = 0`, `RADIX = 1`, `set-valued`, `two tiers`
and the obligation-namespace phrases. I did not read `65`, `66`, `OPTIONS.md`, or the bulk of the
442 files, and my absence claims are bounded by that.
