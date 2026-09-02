# The `ADMITTED` blocks do not answer Q29. Four of the nine answer Q31, which is worse

Seat q29b. Derived from `mock/registry/*.toml` at `7fed7b5932f31e77c8b288bd8aabbe93ec3b40aa`,
which is `canon_paths` in `mockspace.toml:32`. Written before reading any other file in
`mock/research/202608072330_the-numeral-canon-panel/` on this subject; the reconciliation at
the end was appended after, and the commit ordering is the evidence for that.

## 0. The brief is broken, and it breaks on the sentence the fork rests on

The brief states:

> `proposal::the_concept_is_closed_and_the_inventory_is_open` is cited in doc comments at
> `ambient.rs:89`, `quantum.rs:226`, `slots.rs:141` [...] It is cited inside none of the four
> `ADMITTED` docs.

**Three of those three citations are inside an `ADMITTED` doc comment.** Not near one. Inside
the doc comment of the const itself, in the paragraph that names it:

- `mock/crates/arvo-format/src/ambient.rs:81` opens `/// What an implementor owes, checked
  rather than asked for.` and runs unbroken to `const ADMITTED: () = {` at
  `mock/crates/arvo-format/src/ambient.rs:159`. The citation at
  `mock/crates/arvo-format/src/ambient.rs:89` sits in that block and its sentence is
  `This is that check`.
- `mock/crates/arvo-format/src/quantum.rs:222` opens the same doc comment, the citation is at
  `mock/crates/arvo-format/src/quantum.rs:226`, the const is at
  `mock/crates/arvo-format/src/quantum.rs:317`, and the sentence is again `This is that check`.
- `mock/crates/arvo-format/src/slots.rs:137` opens it, the citation is at
  `mock/crates/arvo-format/src/slots.rs:141`, the const is at
  `mock/crates/arvo-format/src/slots.rs:210`, and the sentence is `This is that check.`

So the design tier does not merely sit next to the ratified proposition. **It names the
proposition and asserts, three times, in its own words, that the assert block is the check that
proposition calls for.** The brief offered the absence of that citation as the textual room in
which reading (b) could stand. There is no such room. Everything else in the shipped-state
paragraph checks out: four blocks at `ambient.rs:159`, `quantum.rs:317`, `slots.rs:210`,
`format.rs:227`, nine assertions between them (1, 2, 5, 1), `format.rs` citing nothing, and the
proposal reaching canon through `ruling::the_format_spine_is_canon`.

One more, small: the brief cites `question::Q29`. `Q29` is that row's `key`; its `id` is
`what_the_admission_contract_asks_a_candidate_to_expose`, and `question::Q29` resolves to
nothing. I use the slug below.

I do not stop here, because the brief's own escape applies: the falsity does not make the
question unanswerable, it removes one of the two readings and sharpens what is left. What
follows answers the question the canon poses rather than the one the brief posed.

## 1. Verdict, in four sentences

**Reading (a) is false.** The `ADMITTED` blocks do not answer
`question::what_the_admission_contract_asks_a_candidate_to_expose`, because that row asks about
**exposure** and not one of the nine assertions asks a candidate to expose anything; every one
reads a coordinate the trait already required. What the four contracts ask an implementor to
expose is settled separately and **ratifiedly** by
`ruling::the_numeric_door_carries_the_coordinate_set_and_the_two_type_bound_is_not_canon`.

**Reading (b) is false as posed**, because it presumes the nine are one kind of condition. They
are not: five are conditions under which the ratified parameterisation denotes a set at all, and
**four are conditions about what this implementation's carriers can hold**.

**Those four are the finding.** They fold a hosting clause into the same const as a concept
clause, which is option one of `question::one_word_or_two_for_is_a_number_system`, an open row.
The design tier picked a side of an open canon question and shipped it as a build-time refusal
that binds every outside implementor.

**The canon does not decide the (a)/(b) fork as the brief poses it**, because the ratified
sentence the code cites was stamped across the seam between two topics and the ruling that
stamped it does not say which side it lives on. That ambiguity is handed back in section 6, with
what would close it.

## 2. What the canon actually settles about exposure

`ruling::the_format_spine_is_canon`, `rung = "ratified"`, `ratified_by = "both"`, stamps four
propositions. Two of them fix what a format instance has to hand over.

> A format is identified by its ambient domain and its representable set, and that set is a
> constant of the type. Membership in it is one affine predicate over one parameterisation, of
> which integers, fixed point, scaled integers and floats are points.

`proposal::membership_of_the_representable_set_is_one_affine_predicate` names the
parameterisation exactly:

> Membership is one predicate over one parameterisation: an affine slot function, a quantum per
> magnitude and a phase, of which integers, fixed point, scaled integers and floats are points.

That is the shipped exposure list, one for one. `Slots` is the affine slot function's range and
declared width, `Quantum` is the quantum per magnitude as `BASE + SLOPE * i` over `MAGNITUDES`,
`Format::PHASE` is the phase, and `Ambient` is the ambient domain of the identity clause. The
design tier derived a trait shape from a ratified parameterisation. **That is the chain working,
not drift.**

And it is not merely derivable, it is already ruled on.
`ruling::the_numeric_door_carries_the_coordinate_set_and_the_two_type_bound_is_not_canon`,
`rung = "ratified"`, `ratified_by = "experts"`, answering
`question::what_the_numeric_introduction_door_may_carry_out`:

> What the door carries out is the coordinate set of the ratified parameterisation, spelled in
> types the stack owns.

Its `note` enumerates the ten coordinates as `PHASE_NUM`, `PHASE_DEN`, `RADIX`, `SIGNED`, `BASE`,
`SLOPE`, `MAGNITUDES`, `MIN`, `MAX` and `ARITY`. **So the exposure question for these four
contracts is closed by a ratified ruling, in terms that share no vocabulary at all with
`question::what_the_admission_contract_asks_a_candidate_to_expose`.**

Compare that row's option space, quoted from `mock/registry/question.toml`:

> The standing list prefixed with the reduction's two law verdicts [...]
> The same, plus the ambient domain's own law inventory, plus a third verdict for the retraction
> [...]
> Admission relative to a consumer-supplied ambient domain, the candidate exposing only its
> representable set and its reduction [...]

Law verdicts, a law inventory, a retraction verdict, a reduction. **None of those is a coordinate
in the shipped door, and none of the nine assertions is about any of them.** The candidate in
that row is a number system exposing evidence about its algebra. The candidate in the shipped
traits is a format declaring the coordinates of a grid. Reading (a) requires those to be one
contract, and the canon files them under two topics: `topic::the_number_system`, whose `what` is
"What counts as a number system arvo can carry, and what a candidate has to expose to be admitted
as one", and `topic::the_format`, whose `what` is "What a numeral's format is: the widths it
declares, its signedness, and how a declared format relates to the container underneath it".

**Finding 1.** The four contracts' required associated items are the coordinate set of the
ratified parameterisation and are licensed by
`ruling::the_numeric_door_carries_the_coordinate_set_and_the_two_type_bound_is_not_canon` and
`proposal::membership_of_the_representable_set_is_one_affine_predicate`. They do not answer
`question::what_the_admission_contract_asks_a_candidate_to_expose` and are not drift.
*Holds for: the registry at `7fed7b59`; the four contracts in `arvo-format/src/{ambient,quantum,slots,format}.rs`; radix any; ambient_domain any; phase any; total_width any; signedness any; strategy any; threads = 0; target_features none.*

## 3. The nine assertions are not one thing

Written out, with what each actually reads:

| # | Site | Reads | Kind |
|---|---|---|---|
| 1 | `ambient.rs:161` | `RADIX.is_positional()`, which is `RADIX >= 2` | denotation |
| 2 | `quantum.rs:319` | `ranges_over_a_magnitude(MAGNITUDES)`, which is `MAGNITUDES >= 1` | denotation |
| 3 | `quantum.rs:323` | `reach_is_representable(BASE, SLOPE, MAGNITUDES)` | **carrier** |
| 4 | `slots.rs:212` | `MIN.index() <= MAX.index()` | denotation |
| 5 | `slots.rs:216` | `WIDTH.count() >= 1` | denotation |
| 6 | `slots.rs:220` | `WIDTH.count() <= 62` | **carrier** |
| 7 | `slots.rs:229` | `(MAX - MIN) as i128 < i64::MAX as i128` | **carrier** |
| 8 | `slots.rs:234` | `(MAX - MIN) as i128 < 1i128 << WIDTH.count()` | coherence |
| 9 | `format.rs:229` | `PHASE.denotes()` | denotation |

The three carrier rows say so themselves. `mock/crates/arvo-format/src/slots.rs:221`:

> "declared width is wider than a slot index carries; the count of slots is 2^width and 2^63 does
> not fit a signed 64-bit integer"

`mock/crates/arvo-format/src/slots.rs:230`:

> "slot range spans more indices than a count can carry, so counting it would overflow"

`mock/crates/arvo-format/src/quantum.rs:324`:

> "the step law runs its exponent past what an exponent carries before reaching its largest
> magnitude, so it names no quantum there"

**Each of those three is a sentence about a container in this implementation.** `62` is `i64`.
`i64::MAX` is `i64`. "what an exponent carries" is the `i32` behind `Exponent`. Row 8 is
different and belongs with the denotation rows: it says the two declared coordinates must agree
with each other, which is a property of the declaration and of no carrier.

**The canon carries none of these three bounds.** `grep -rn '\b62\b' mock/registry/*.toml`
returns exactly one hit and it is a line number inside a `provenance` string
(`mock/registry/proposal.toml:1681`). No ratified row bounds a width, a slot count or an
exponent range anywhere.

**Finding 2.** The `ADMITTED` consts hold two kinds of assertion under one name: five conditions
on whether the ratified parameterisation denotes a set, and three conditions on what this
implementation's carriers hold. A reader cannot tell them apart from the const, and nothing in
the canon distinguishes them either.
*Holds for: the registry and the tree at `7fed7b59`; the four `ADMITTED` blocks named above; radix any; ambient_domain any; phase any; total_width any; signedness any; threads = 0; target_features none.*

## 4. The three carrier rows pick a side of an open question

`question::one_word_or_two_for_is_a_number_system` is open, `decider = "panel"`, and asks:

> Does the canon use one word or two for "is a number system" and "can arvo carry it"?

Its first option, verbatim:

> One word, folding the residue clauses into the concept, which makes the concept exclude
> unbounded exact rationals as a matter of mathematics, which is false, and makes every hosting
> clause conditional on unratified constraints inside a sentence about arithmetic.

**That is exactly what `slots.rs:210` is.** Five assertions in one const, three of them saying
what a grid is and two of them saying what an `i64` holds, with nothing separating them and no
canon behind the second pair. The option's stated cost, "every hosting clause conditional on
unratified constraints inside a sentence about arithmetic", is the shipped shape described in
advance.

And the choice binds outward rather than staying inside the crate.
`mock/crates/arvo-format/src/slots.rs:120` tells an outside implementor:

> The trait is open, so another crate may implement it at a range of its own; what such an
> implementor owes is the `ADMITTED` obligation.

The trait is open and `ADMITTED` closes it at 62. `mock/crates/arvo-format/DESIGN.md.tmpl:598`
makes the openness claim in the design tier too:

> **The trait itself is open and is not sealed.** A numeral wanting a slot range that is neither
> of the two shipped shapes has to be able to supply one, and
> `proposal::the_concept_is_closed_and_the_inventory_is_open` is ratified through the format
> spine.

So a ratified open-inventory clause is cited as the licence for a trait whose own admission
obligation refuses, at build time, every range above 62 bits, for a reason the canon never
states. `question::is_the_number_system_inventory_open` carries `answered = "Open."` on the
strength of that same ruling. **An inventory that is open in the canon and bounded at 62 in the
const that admits into it is not the same inventory.**

`proposal::membership_and_hosting_are_two_questions` names the hazard, and I cite it as an
unratified one-expert row rather than as authority:

> Conflating them makes the concept's boundary a fact about one implementation's runtime, which
> would move every time the implementation did.

**Finding 3.** Rows 3, 6 and 7 of the table in section 3 answer
`question::one_word_or_two_for_is_a_number_system` in favour of its first option, inside the
design and code tiers, on a row that is open and whose `decider` is `panel`. This is the drift
the brief went looking for. It is not at Q29 and it is not about exposure; it is a hosting bound
with no canon behind it, welded into the admission obligation and exported to every implementor
by an open trait.
*Holds for: the registry and the tree at `7fed7b59`; `quantum.rs:323`, `slots.rs:220`, `slots.rs:229`; radix any; ambient_domain any; total_width any; signedness any; phase any; threads = 0; target_features none.*

### 4a. A second open row is picked the same way, more weakly

`question::is_admission_a_predicate_or_a_location` is open, and its first option is

> A predicate returning member or not, which discards the coordinate a consumer needs and makes
> every mislocated member read as a non-member.

`is_admissible_ambient`, `is_admissible_quantum`, `is_admissible_format` and
`slots::is_admissible` all return `Bool`. That is the first option, and
`proposal::admission_returns_a_coordinate_rather_than_a_verdict` is the rival, at `one_expert`.

I hold this one **weaker than finding 3** and say why: that row's candidate is a number system,
and nothing forces its "admission" to be the format contract's. Finding 3 does not depend on the
identification, because a width bound is a hosting fact under any reading of the word. This one
does. I record it as a finding to test rather than as a conviction.
*Holds for: the tree at `7fed7b59`; the four verdict functions named; conditional on the number-system and format admission contracts being one contract, which section 6 states is unsettled.*

## 5. What is genuinely licensed, so the report is not one-sided

The mechanism is not itself unlicensed, and saying so is a result.
`proposal::the_concept_is_closed_and_the_inventory_is_open`, ratified, says:

> The canon defines once what a number system is **and what admission requires**; the set of
> admitted instances is open, and a new one earns admission by supplying the concept's
> obligations rather than by amending the canon.

and its `because`:

> Closing the concept and opening the inventory is what makes admission **a check** rather than
> a negotiation.

"What admission requires" is conditions, and "a check" is the form. So a const that checks
conditions is the shape the ratified sentence asks for, and the six denotation-and-coherence
rows are conditions on the ratified parameterisation itself. `ruling::validate_means_all_three_readings`
carries op's own words at `rung = "stated"` and its `note` defines the admissibility reading as
"where the typestate refuses declarations it cannot serve", which is what those six rows do.
`ruling::never_a_runtime_check_and_one_lowered_path` is what forces the check to be a const
rather than a runtime guard, and the docs cite it correctly at
`mock/crates/arvo-format/src/quantum.rs:237` and `mock/crates/arvo-format/src/slots.rs:149`.

**Nothing here proposes deleting `ADMITTED`.** Six of the nine assertions should stay exactly
where they are. What has no canon is the three carrier rows sharing the const with them.

## 6. Where the canon genuinely does not decide, and what would close it

The brief's fork cannot be settled from the canon, and the reason is a specific and checkable
defect in the stamp rather than a gap.

`proposal::the_concept_is_closed_and_the_inventory_is_open` carries `topic = "the_number_system"`.
`ruling::the_format_spine_is_canon` carries `topic = "the_format"` and stamps it as one of four,
restating it in its own `says` as "The concept is closed and the inventory of admitted instances
is open" and glossing it in its `because` as "the closed-concept clause says how a new instance
joins without amending the canon". **In the ruling's framing "a new instance" is a format
instance; in the proposal's own filing the concept is the number system.** One sentence, stamped
once, resolving to two different concepts depending on which field you read.

Every citation in the shipped tree exploits that. The instrument rather than a count, so a
later reader can re-ask it:

```
grep -rn "the_concept_is_closed_and_the_inventory_is_open" mock/crates mock/lints
```

At `7fed7b59` it returns `arvo-placement/DESIGN.md.tmpl:142`,
`arvo-placement/src/tests/the_open_inventory.rs:8`, `arvo-format/DESIGN.md.tmpl:387`,
`arvo-format/DESIGN.md.tmpl:601`, `arvo-format/src/ambient.rs:89`,
`arvo-format/src/quantum.rs:226`, `arvo-format/src/slots.rs:141`,
`arvo-format/src/tests/the_inventory.rs:8` and
`mock/lints/a_contract_coordinate_is_not_a_host_primitive.rs:19`. Every one cites a
`the_number_system` proposition to license a mechanism on the `the_format` contracts, and not one
says it is crossing a topic. The last is a lint, which means the seam is load-bearing for a gate
as well as for prose.

So the canon **does not** answer whether the format contract's obligations and the number
system's admission contract are one thing. If they are one, Q29's option space is wrong, because
none of its three options mentions a coordinate. If they are two, the four `ADMITTED` blocks are
outside Q29 entirely and the citation chain in the tree is loose rather than false.

**What would close it, and it is one row rather than a panel.** A `ruling` saying which concept
`proposal::the_concept_is_closed_and_the_inventory_is_open` closes. Two readings are available
and both are cheap to state: that it closes one concept of which a format is the representable-set
half, in which case Q29's options need rewriting to include the coordinate set; or that the
format concept and the number-system concept are two closed concepts each with its own admission,
in which case the shipped citations want a second slug and Q29 is untouched by any of this.
**The call is not mine and I have not made it.** Under
`ruling::two_experts_converging_is_a_ratification_and_the_coordinator_holds_the_gate` it needs a
second independent reading before it is anything.

**Finding 4.** `ruling::the_format_spine_is_canon` stamps a `the_number_system` proposition inside
a `the_format` ruling without saying which concept the stamped sentence closes, and every citation
of it under `mock/crates` and `mock/lints` crosses that seam. No open row asks the question, so it is not merely
unsettled, it is untracked.
*Holds for: the registry at `7fed7b59`; `ruling::the_format_spine_is_canon` and `proposal::the_concept_is_closed_and_the_inventory_is_open`; the nine citation sites the grep above returns. Holds over no numeric axis, per `question::can_a_claim_about_the_canons_own_structure_carry_a_region`, which is open on whether a claim of this kind can carry a region at all; threads = 0; target_features none.*

## 6a. And the obligation nobody is guarding, measured

While reading the surface for the test gate I checked what the `ADMITTED` conditions are held by.
The answer is measured rather than argued: `mock/research/202609021238_probes/`, committed with
this file.

**Delete four of the five assertions in `Slots::ADMITTED` and the entire workspace stays green.**
Baseline at `7fed7b59` is 165 passed, 0 failed, 2 ignored across
`cargo test --workspace --all-targets` and `--doc`. With `WIDTH.count() >= 1`,
`WIDTH.count() <= 62`, the `i64::MAX` span bound and the `1i128 << WIDTH` bound removed, it is
165 passed, 0 failed, 2 ignored. Not one test, doctest or `trybuild` case changes colour. The
mutant, the diff and both raw logs are in the probe directory.

The mechanism is a duplication. The five conditions are written twice: once in the const at
`mock/crates/arvo-format/src/slots.rs:210` and again, as a separate expression, in
`is_admissible` at `mock/crates/arvo-format/src/slots.rs:249`. Every test calls the copy.
`MIN <= MAX` is the only condition with a build-refusal case, the `compile_fail` doctest at
`mock/crates/arvo-format/src/slots.rs:166`, and `tests/ui/` holds cases for the two `Quantum`
conditions and none for any `Slots` condition. `SpanTooWide` and `WidthTooNarrow` in
`mock/crates/arvo-format/src/tests/the_inventory.rs` are good constructions aimed at the copy.

**This is not a small thing next to findings 1 to 4, it is the same thing one tier down.** The
design tier says the obligations are "checked rather than requested"
(`mock/crates/arvo-format/DESIGN.md.tmpl:615`) and every doc comment on the const says the check
is what makes admission a check rather than a negotiation. The check that carries that claim is
four fifths unguarded, and the two conditions with no canon behind them, the ones finding 3 is
about, are both in the unguarded four. So the state is that a bound the canon never stated is
enforced on every implementor and nothing in the repository would notice if it vanished.

**And the design tier says the width bound is not a check.**
`mock/crates/arvo-format/DESIGN.md.tmpl:589` heads the section "The bound, which is the set of
impls rather than a check inside one", and
`mock/crates/arvo-format/DESIGN.md.tmpl:596` says "There is no guard inside a function, because a
guard is a thing that can be deleted and an absent impl is not." The bound is nonetheless a guard
inside a const at `mock/crates/arvo-format/src/slots.rs:220`, and the mutation is the
demonstration that it is exactly the thing that can be deleted, with nothing noticing. The design
predicted the failure mode and the code shipped it anyway. Under `the-canon-design-code-chain`
this is the design and the code disagreeing, and the design is the oracle.

The same document already caught this class once, at
`mock/crates/arvo-format/DESIGN.md.tmpl:609`: "the same sentence appeared three times in two
files and each reading of it was a claim that something was total when nothing was checking." The
repair was a check. Nobody then checked the check.

**Finding 5.** Four of the five conditions in `Slots::ADMITTED` are covered by no test, doctest or
`trybuild` case, because the conditions exist twice and the suite reads the copy. Measured by
mutation, with the baseline and mutant runs committed.
*Holds for: the tree at `7fed7b59`; `slots.rs:216`, `slots.rs:220`, `slots.rs:229`, `slots.rs:234`; toolchain = the pin in `rust-toolchain.toml`; build_profile = dev; threads = 0; target_features none.*

## 7. What is owed, in order

1. **Split the carrier rows out of the `ADMITTED` consts** at `quantum.rs:323`, `slots.rs:220`
   and `slots.rs:229`, or get a ruling that puts a hosting bound inside admission. Either is
   fine; what is not fine is the current state, where an open question is answered by a const
   nobody looked at.
2. **File `question::one_word_or_two_for_is_a_number_system`'s shipped answer as a finding on
   that row**, so the next reader of the row knows the design tier already picked.
3. **File the topic-seam question** from section 6, with its two readings.
4. **Give the four unguarded `Slots` conditions a build-refusal case each**, in `tests/ui/`, in
   the shape `a_law_over_no_magnitudes_is_refused.rs` already uses, so a condition cannot be
   deleted silently. Or collapse the const and the verdict onto one expression, which removes
   the class rather than testing around it.
5. **Nothing about Q29.** It is untouched by the shipped tree and should stay open on its own
   three options.

## 8. Gates

**Canon gate: ambiguous on the assigned question, misaligned on something adjacent.** Handed back
in section 6 rather than resolved. The misalignment is finding 3 and is reported rather than
fixed, because fixing it is a design-tier change under `the-canon-design-code-chain` and the
round for it is not mine to open.

**Test gate: passes.** `cargo test --workspace --all-targets` at `7fed7b59` is 156 passed, 0
failed, 2 ignored, both catalogue-reds naming the question that closes them.
`cargo test --workspace --doc` is 9 passed, 0 failed. The 8 `trybuild` cases in
`arvo-format/tests/compile_fail.rs` include the `ADMITTED` refusals. I read the bodies of
`mock/crates/arvo-format/src/tests/obligations.rs` (258 lines) and
`mock/crates/arvo-format/src/tests/the_inventory.rs` in full rather than their names. They are
real: every verdict arm carries a control that the law admits the shipped set, a separator
asserting a shipped instance and a wrong one get different answers, and the wrong constructions
are kept in the file rather than deleted. `every_admitted_width_has_a_coherent_range` sweeps all
62 widths rather than a sample. `the_declared_width_is_read_rather_than_recovered` carries a
comment naming the tautology its own previous cut was, a constant against the literal its
definition set, and the replacement is not one. I found nothing to delete. What I found instead
is in section 6a: the tests are good and they are pointed at a copy of the conditions rather than
at the conditions, which no reading of the bodies would have shown and only the mutation did.

One thing worth carrying: `ruling::the_numeric_door_carries_the_coordinate_set_and_the_two_type_bound_is_not_canon`'s
`note` already records that `the_format_inventory_admits_a_member_this_crate_does_not_know_about`
asserts the open-inventory clause inside the one crate exempt from the lints it is about. That is
still true at `7fed7b59` and it is the same shape as finding 3: a claim about openness made where
nothing can refuse it.

## 9. Reconciliation, appended after reading the prior panel files

Everything above is as committed at `5eeb39d2`, before I opened `241`, `242`, `246` or `247`.
The commit ordering is the evidence and it is the only kind available. I then read those four on
this subject. Nothing above changes; what follows says which of my findings has a prior instance
and which does not, because that decides the tier each sits at.

**Finding 2 has a prior instance and is now at two.** `246` section 5.2 extracted the same five
`Slots::ADMITTED` conditions and classified them the same way, two hosting and three
well-formedness, with four controls including a not-flagged one. Its rows are my rows 4, 5, 6, 7
and 8 and we agree on every classification. **It reached that on `Slots` alone.** My instance
widens it to nine assertions over four contracts and adds `quantum.rs:323` as a third carrier
row, which is on a contract `246` did not look at. Two instances, different instruments,
different scopes, agreeing about the intersection.

`246` section 5.1 supports it from a direction I did not take: it greps every non-test source
file of `arvo-format` for `usize`, `isize`, `target_pointer_width`, `cfg(target` and pointer-sized
`size_of` and gets zero, with a positive control finding one in the test files. So the carrier
bounds are indexed by this crate's chosen types and by no target. That is the sharpest support
finding 3 has and it was measured before I got here.

**Finding 4 has a prior instance of the handback and not of the reason.** `247` section 8, O3, is
the brief's fork verbatim, down to the wording, and it was already returned as "ambiguous under
the canon gate and handed back rather than decided". So the brief I was given is O3 re-dispatched.
`247` names as what would close it "two readings of Q29's `asks` against `Ambient::ADMITTED`'s own
doc". **I disagree about the locus and that is my contribution here.** Reading the const's doc
against Q29's `asks` cannot close it, because the ambiguity is not in either of those texts: it is
in `ruling::the_format_spine_is_canon` stamping a `the_number_system` proposition and glossing it
as being about format instances. Two careful readers of the doc and the row will still disagree,
because the sentence they are both reading resolves to two concepts. What closes it is a ruling
saying which concept the stamped sentence closes, which is a different act by a different party.

`241` section 0 saw the same mismatch and used it as a floor rather than as a defect: "That
proposal's own `topic` is `the_number_system`, not `the_format`." It is stated there as a
parenthesis in an argument about Q20 and nothing was filed.

**Finding 1 has no prior instance and it is what kills reading (a).** Six panel files cite
`ruling::the_numeric_door_carries_the_coordinate_set_and_the_two_type_bound_is_not_canon`.
`241` carries it as floor item R3. `242` uses it twice, both times against Q30's stated cost that
a location "presumes a coordinate count the canon may not want to commit to", correctly observing
that the ruling commits to the coordinate set and leaves only the type count open. **Nobody points
it at Q29.** `246` section 6 on Q29 says in full: "Reserved, two standing answers, both at one
instance, neither cited. I add nothing and I did not attack it."

That is the gap. Q29 asks what the admission contract asks a candidate to expose; a ratified
ruling already says what the format contracts carry out, in a vocabulary of coordinates that
shares nothing with Q29's three options. Reading (a) needs those two to be one contract, and the
canon has answered one of them and left the other open, which is only possible if they are two.
**One instance, mine, and it wants a second reader before it is anything.**

**Finding 3 is a sharpening rather than a second instance.** `246` reached "the count is three
rather than two" and then explicitly declined to go further: "I do not propose a third word ...
where the third belongs is open." Its finding is taxonomic. Mine is that
`question::one_word_or_two_for_is_a_number_system`'s first option, folding hosting into the
concept, is not merely a live option: it is implemented, at
`mock/crates/arvo-format/src/slots.rs:220`, and it binds every outside implementor of an open
trait at build time. **The difference matters for what is owed.** A taxonomy finding wants a
vocabulary decision. An implemented answer to an open row wants the row to say so, because the
next reader of that row will otherwise weigh two options one of which has already shipped.

**Finding 5 has no prior instance anywhere.** Nobody has measured what the `ADMITTED` conditions
are held by. `246` classified them, `241` characterised them, `247` counted them, and none asked
whether deleting them turns anything red. The answer is that four of the five in `Slots::ADMITTED`
are held by nothing at all.

**One stale figure travelled into my brief and is worth naming.** `247` O3 says "the tree enforces
three obligations". At `7fed7b59` there are four, because `Ambient::ADMITTED` landed in the round
at `mock/design_rounds/202609011813/` after `247` was written. The brief I was given carries the
updated count, nine assertions across four blocks, without saying that the row it inherits its
fork from was written against three. Nothing in my answer turns on it, and a later reader
comparing the two files would otherwise read the difference as a disagreement.

**Coverage, bounded.** I read `241` sections 0 and 1 and its heading outline, `246` sections 5.1,
5.2 and 6 in full, `247` section 8 O3 and section 9, and `242` at its three citations of the door
ruling. I did not read `243`, `244`, `245`, `248`, `249`, or the bodies of `241` sections 2
through 11. So a prior instance of finding 1, 3 or 5 could exist in what I did not open, and my
"no prior instance" claims are bounded by that list rather than by the corpus. The greps behind
them are not: `grep -ln "the_numeric_door_carries_the_coordinate_set" *.md` over the whole panel
directory returns the six files named above and I checked how three of them use it.
