# Where the zero-denominator condition is enforced, and what the canon asks of that

Read at `67ce00d8`, on `rustc 1.98.0-nightly (57d06900f 2026-05-27)`, the pinned toolchain.
Probes and their raw output are committed beside this file at
`mock/research/202609021406_phase_denominator_locus_a_probes/`, with `run.out` holding every arm.

**Canon gate: passed.** Checked against `mock/registry/*.toml`, which `mockspace.toml:31` declares as
`canon_paths`. Nothing in the assigned question asks for work the canon forbids, and the question is
well posed, unlike the agenda row's framing of it, which is dealt with in section 6.

**Test gate: passed with one finding.** `cargo test --workspace --all-targets` is green at 44 passing,
2 ignored, both ignored ones carrying a catalogue reason: one naming two open registry questions,
`question::is_the_rounding_vocabulary_complete_at_six` and
`question::which_tie_direction_an_unqualified_nearest_names`, the other an unbuilt second packing
rule. That is the shape `catalogue-edge-cases-as-tests.md` asks for, and I read the bodies rather
than the names. `cargo test --workspace --doc` is green at 4 plus 5
compile-fail. The finding is in section 5.4: the phase's refusal is pinned by a `compile_fail`
doctest and by no `trybuild` case, in a suite where the neighbouring condition gets both, and the
design says in as many words which of the two pins the diagnostic.

**One methodological note, because it nearly cost the gate.** The first suite run was written
`timeout 3000 cargo test ...`. `timeout` is not on macOS, so the pipeline returned exit 0 having run
nothing, and reported green. That is `never-discard-stderr-on-a-check.md` exactly, and the only thing
that caught it was reading the output rather than the exit code.

## 1. The short answer

**The canon requires the condition to be caught at compile time, forbids it being caught at run time,
and names no site.** One row bears on it and it is ratified. The choice the agenda row poses, between
a const refusal on the constructor and the predicates' totality, is a choice between two constructions
the canon deliberately does not pick between, and there is a row saying so in terms.

**Where the canon does bind, the code does not meet it.** A format declaring a zero phase denominator
reaches a produced, running binary on four paths, one of which is the applied map that every value
goes through. Measured, with controls, in section 4.

**And the design says the opposite of that, in one sentence, at `DESIGN.md.tmpl:738`.** The sentence is
false against the crate it governs.

## 2. What the canon says, quoted

### 2.1 The one row that bears on the locus

`ruling::never_a_runtime_check_and_one_lowered_path`, `key = "I15"`, `rung = "ratified"`, ratified by
op himself in the `ratification` block on that row:

> Runtime code can exist. Its branching should be done as much as possible with const-time ifs erased
> by monomorphisation, const solving and ultimately the backend. Never any runtime checks, ever:
> invalids are caught at compile time and unused paths are cleared when lowered. **Unless there is a
> justifiable reason that is blessed by a lead designer.**

That row's own `note` splits the sentence into two strengths, and the split matters here: "as much as
possible" governs the branching, "never, ever" governs the checks. So what binds is a **time** and a
**prohibition**. It says the invalid is caught while compiling and never while running. It does not
say by which construction, in which function, or at whose call.

Note the row carries no `ratified_by` field, unlike the format spine rows. Its `ratification` block
opens "Ratified by op on being asked whether this was `stated` or `in_force`", so the stamp is his and
the field's absence is a shape difference in the namespace rather than a weaker tier. Worth saying,
because a reader filtering the registry on `ratified_by = "op"` misses this row entirely.

### 2.2 The rows that say the canon does not choose the construction

`ruling::the_canon_does_not_police_what_shape_a_law_takes`, `key = "I16"`, `rung = "stated"`,
`kind = "refusal"`, in op's own words:

> Monomorphisation and const solving should lead everything to go through one lowered path, that's it.
> So if a law is a law, it should be expressed so that it actually works, be it typestate or const
> expressions or whatever. We shouldn't police what kind of laws there are or what shapes they take.

Its `instead` field states the bar: "The requirement on a law's expression is functional rather than
structural: it must actually work, meaning it reaches one lowered path."

And its `note` reaches this case almost by name. The measurement behind that question was that four
const-time constructions give four different guarantees, and op's answer left the measurement standing
as "a caution for whoever picks one rather than a rule about which constructions exist".

`ruling::there_is_no_universal_answer_take_the_win_and_gate_it`, `rung = "stated"`:

> Again, we don't need to settle for one universal solution, it's the anti-pattern I've already named.
> Case by case. [...] Take the win where it applies, gate it out from where it does not. No single
> one-fits-all solutions, it's impossible

`ruling::the_predicate_is_whatever_is_available_at_const_time`, `rung = "stated"`, fixes the
admissible category as const-availability rather than as any particular construction.

**Together these say the silence in 2.1 is deliberate rather than a gap.** A `const fn` constructor
that refuses, and a trait-const obligation forced at a use site, are both const-time constructions.
The canon admits both, ranks neither, and one row explicitly refuses to be asked which.

### 2.3 What the canon says about the phase itself, which is less than it looks

`ruling::the_format_spine_is_canon`, `rung = "ratified"`, `ratified_by = "both"`, ratifies
`proposal::membership_of_the_representable_set_is_one_affine_predicate`. That proposal makes the phase
a coordinate of the one membership predicate. It states **no condition on the pair at all**.

`dimension::phase` describes the coordinate as "the offset of a format's grid from zero, in units of
the quantum at magnitude zero, **as the rational the ratified affine predicate carries**", with the
grammar `phase = <p/q>`. A pair over zero is not a rational, so the word does bear on the question.
It is the closest the canon comes to declaring the pair inadmissible, and it is not close enough to
rest anything on: `dimension` rows carry no `rung`, they declare the axes a finding's region is stated
over, and the file's own header opens by saying the set is incomplete.

`proposal::the_concept_is_closed_and_the_inventory_is_open`, ratified through the same spine row, says
"a new one earns admission by supplying the concept's obligations rather than by amending the canon".
That makes admission a check the candidate passes. It does not say where the check fires.

### 2.4 Where the canon is silent, said plainly

- **It does not say a zero phase denominator is inadmissible.** No row states the condition. The
  nearest thing is the word "rational" in a `dimension` row's `what`, and that namespace carries no
  standing at all.
- **It does not choose between a constructor refusal and a contract obligation.** Two `stated` rows say
  it declines to.
- **It does not say which functions must force an obligation**, nor that a format must be refused on
  every path that reads it.
- **It does not say whether the refusal must be visible to `cargo check` or only to `cargo build`.**

Silence on the first is the one worth escalating. Under
`ruling::a_thing_that_constrains_the_work_and_cannot_be_designed_away_is_canon`, `rung = "stated"`,
in op's words:

> if it's a rule that can't be avoided or designed away, a law, or something that constraints the work
> of the design, it's canon

By that test the condition looks like canon. A format whose phase names no position has no
representable set, so every predicate the ratified spine defines is being asked about nothing, and the
condition cannot be designed away without dropping the phase coordinate the spine requires. **It is
currently stated only in the design, at `DESIGN.md.tmpl:733`.** I am one instrument and this is a
judgement rather than a measurement, so I file it as a gap for the panel and do not close it. A second
independent read is owed before anything is written into the registry on it.

I checked for an existing row covering it before saying that. `question::is_admission_a_predicate_or_a_location`
(`Q30`) sounds like this question and is not: it asks whether admission returns a verdict or a
coordinate, in the `the_number_system` topic. `question::what_the_admission_contract_asks_a_candidate_to_expose`
(`Q29`) asks what a candidate exposes, not where the check fires. Nothing in `question.toml` asks
where an obligation is enforced.

## 3. So what does the canon require the design to state?

**One thing, and the design does state it.** Under I15 the design must place the catch at compile time
and must not place it at run time. `DESIGN.md.tmpl:733` states the condition, and `format.rs:227`
implements it as a const, which is compile-time. On the requirement as stated, the design complies.

**The canon requires nothing about the constructor.** A design that refuses in `Phase::of` and a design
that refuses in `Format::ADMITTED` both satisfy I15, and I16 forbids the canon from preferring one.
Any answer of the form "the canon requires the refusal to sit at X" is inventing a rule, and asking
which of the two should govern is the shape op has refused three times in one sitting.

**What the canon does require, and where the design breaks it, is coverage rather than locus.** I15's
sentence is "invalids are caught at compile time" without a qualifier. Once the design declares a
condition at `DESIGN.md.tmpl:733`, a declaration violating it is an invalid, and I15 says it is caught.
Section 4 measures that it is not, on four paths.

## 4. The measurement

Eleven arms, one binary each so a refusal is attributable, each built with `cargo check` and
`cargo build` separately, each run. `run.sh` is the driver, `run.out` the committed output.

| Arm | Construction | `cargo check` | `cargo build` | Ran and printed |
|---|---|---|---|---|
| P1 | `Phase::of(1, 0)` alone | accepted | accepted | `1/0 denotes=false` |
| A1 | control: `of(1,2)` through `has_additive_identity` | accepted | accepted | `false` |
| A2 | `of(1,0)` through `has_additive_identity` | **accepted** | **refused** | did not run |
| A3 | `of(1,0)` declared, `PHASE` read directly | accepted | accepted | `PHASE = 1/0` |
| A4 | `of(1,0)` through `adapt`, the applied map | accepted | accepted | `slot 3` |
| A5 | `of(1,0)` through `contains` | accepted | accepted | `true` |
| A6 | `of(1,0)` through `smallest_step_exponent` | accepted | accepted | `0` |
| A7 | `of(1,0)` forced in a `const` item | **refused** | refused | did not run |
| A8 | control: `of(1,2)` in the same `const` item | accepted | accepted | `false` |
| A9 | shipped identity at a whole-step phase | accepted | accepted | `true` |
| A10 | `Phase`'s declared unwrap doors | accepted | accepted | `7`, `3` |

The refusals in A2 and A7 carry the obligation by name, so they fail for the right reason rather than
incidentally:

```
error[E0080]: evaluation panicked: a phase denominator of zero names no position on the grid, so
the phase does not denote a value the set could contain
   |  evaluation of `<shared::Broken as arvo_format::Format>::ADMITTED` failed here
```

A1 and A8 are the controls that say a refusal is a property of the denominator and not of the harness:
the identical construction at `of(1, 2)` builds and answers in both shapes.

### 4.1 `Format::ADMITTED` has two forcing sites, not three

The brief says lines 322, 398 and 433 force `ADMITTED`. Two of them force `Format::ADMITTED`:
`format.rs:322` in `cancelling_slot` and `format.rs:398` in `has_additive_identity`. `format.rs:433`
is inside `radix` and forces `Ambient::ADMITTED` only. `apply.rs:355` forces `Slots::ADMITTED` through
`S::Format` and not the format's own. Grep, across the whole crate:

```
format.rs:322:    let () = <F as Format>::ADMITTED;
format.rs:398:    let () = <F as Format>::ADMITTED;
```

That is the entire enforcement surface. Every other public function taking an `F: Format` reads the
declaration without ever forcing what it owes.

### 4.2 The four paths a non-denoting format takes to a running binary

- **A4 is the serious one.** `adapt::<Signature<Broken, Adapt<HalfEven, Saturate>>>` is the applied
  map, `apply.rs:349`, the path a value actually takes. It returns slot 3. Its own rustdoc at
  `apply.rs:345` says "every position returns a slot the format admits", and the format admits
  nothing, because it has no representable set. The comment at `apply.rs:352` explains that the
  `Slots::ADMITTED` line is there because "reading `MIN` and `MAX` does not force it on its own, so
  without this line the completion would work over a range that merely arrived rather than one that
  was admitted". The identical reasoning applies to the format and the line is not there.
- **A5 contradicts a law the design already wrote down.** `contains::<Broken>(Slot::at(3), Magnitude::at(0))`
  returns `true`. `contains` is the membership predicate the ratified spine names, and it answers that
  coordinates name a member of a set with no members. The design states this exact law for the
  neighbouring condition at `DESIGN.md.tmpl:248`: "**A law over no magnitudes admits nothing, and every
  function that answers a membership question agrees about that.** The one that reads only the slot
  range is the one that got to disagree." That is the same sentence, one coordinate over, and nobody
  wrote its phase half.
- **A6**: `smallest_step_exponent::<Broken>` answers `0`.
- **A3**: the declaration reaches a running binary and prints its own `1/0`. No forcing site exists on
  that path and none can, because an associated const default is evaluated only where it is named.

**Region.** `holds for: phase = 1/0 with the shipped `Broken` declaration, ambient_domain = BinaryRationals,
radix = 2, quantum = Constant<0>, slots = Signed<8>, toolchain = nightly-2026-05-28 as pinned,
build_profile = cargo dev default, threads = 1, target_features any.` A5 and A6 do not read the phase
at all, so they generalise over every phase; A3 and A4 were run at one declaration each and are
existence claims, which is all they need to be, since one path reaching a binary refutes a universal.

### 4.3 So the design's guarantee sentence is false

`DESIGN.md.tmpl:736-738`:

> **Each fires exactly where the slot range's does**, for the same reason and with the same limit: a
> const evaluated at monomorphisation, so codegen refuses and `cargo check` does not. The guarantee is
> that an inadmissible declaration cannot reach a produced binary, and it can reach a passing check.

An inadmissible declaration reaches a produced binary four ways, and in three of them it produces an
answer somebody could act on. The true sentence is narrower and is checkable: *a declaration violating
`Format::ADMITTED` cannot reach a produced binary through `cancelling_slot` or `has_additive_identity`,
and reaches one through every other path.*

This is `a-claim-of-totality-names-what-enforces-it.md` in its plainest form. The paragraph quantifies
over declarations, the mechanism under it covers two functions, and nothing in the file names which.

## 5. Does the design stand in the relation the chain requires

Measured against `.claude/rules/canon-design-code-chain.md` as it sits in this worktree. **Partly. It
passes both rules that file calls enforceable and fails the reproduction property in four named
places.**

### 5.1 What passes

**It declares its canon**, at `DESIGN.md.tmpl:3-12`, naming eight `ruling` slugs and two `proposal`
slugs. The rule: "Every design document must declare the canon it relates to."

**Every declared slug resolves.** Checked one by one against `mock/registry/ruling.toml` and
`mock/registry/proposal.toml`; all ten found. The rule: "Naming a canon that does not exist is a hard
failure." It is not being violated here.

**Nothing under `archive/` is named**, and no `mock/canon/archive/` exists in the tree to name.

### 5.2 The source contradicts the design about the binding time, and the design is right

`format.rs:179-181`, the rustdoc on `Format::ADMITTED`:

> **It fires at codegen, not at `cargo check`**, so `cargo build` refuses and `cargo check` does not,
> which is why the predicates below stay total and answer a zero denominator rather than assuming it
> was refused.

`DESIGN.md.tmpl:255-259`:

> **Where an obligation is forced decides which tool can see its refusal**, and that is the axis rather
> than a property of either tool. An obligation is a const and a const is evaluated where it is used.
> Forced from a runtime call it is reached at codegen, which `cargo check` skips, so a doctest, which
> builds a binary, is what catches it. Forced in a `const` item it is reached at check time, and a
> `trybuild` case both sees it and pins the exact diagnostic, which a `compile_fail` doctest does not.

**The design says it depends on the forcing site. The source says it is a property of the obligation.**
A7 settles it: `const _REFUSED: Bool = has_additive_identity::<Broken>();` is refused by `cargo check`,
with the phase assertion named in the diagnostic, and A8 shows the same const item at `of(1, 2)`
accepted. So the design's relative claim is right and the source's absolute one is wrong, and the
source's sentence bolds the false half.

That is not a stylistic quibble. The clause "which is why the predicates below stay total" makes a
false premise load-bearing for a design decision stated in the same paragraph. The predicates should
stay total, and the reason is the one the design gives, not this one.

### 5.3 Two things in the code the design does not say

`the-canon-design-code-chain.md`: "nothing may appear in code that is not in the design", and its
reproduction test, "Nuke the code and lose nothing: the design says what the code was."

- **`Phase` has two declared unwrap accessors and the design says one.** `DESIGN.md.tmpl:447-449`:
  "Each of these is `repr(transparent)` where it wraps one value, carries a private field, and **has
  exactly one declared unwrap accessor**". `Phase` declares `numerator()` at `format.rs:87` and
  `denominator()` at `format.rs:96`, and A10 calls both. Worse than a miscount: `format.rs:93-95`
  argues the case in the source, "The second unwrap door. Two, rather than one, because the pair is
  what the coordinate is and handing back a single number would mean dividing." **That is a design
  decision written into the leaf tier**, which is the failure that rule names in bold as "an undeclared
  design change wearing the leaf tier's freedom". The argument may well be right. It belongs at
  `DESIGN.md.tmpl:448`, and the design changes before the code under it does.
- **The search bound is in the code and not in the design.** `format.rs:268` declares
  `MAGNITUDE_SEARCH_BOUND` and `format.rs:287` `SCALING_WIDTH = Width::bits(127)`, with a derivation in
  the rustdoc for why a bounded search returns what an unbounded one would. The design says only that
  it is "a bounded loop inside a `const fn`", at `DESIGN.md.tmpl:197`. Two implementers reading the
  design would both write a bounded loop and would not both write a bound that is provably sufficient,
  which is the design acceptance test in that rule failing. Weaker than the first finding, because a
  reader could argue the derivation is implementation, and I do not think that survives: the claim that
  the bounded search agrees with the unbounded one is a property of the answer rather than of the code
  that computes it.

### 5.4 The refusal is pinned by the weaker of the two instruments the design names

`tests/compile_fail.rs` holds eight `trybuild` cases, including
`a_law_over_no_magnitudes_is_refused.rs`, which forces `Quantum::ADMITTED` in a `const` item and pins
the exact diagnostic in a committed `.stderr`. **There is no such case for the phase.** Its refusal is
pinned only by the `compile_fail` doctest at `format.rs:183`, and `DESIGN.md.tmpl:259` says in as many
words that a `compile_fail` doctest does not pin the diagnostic.

A `compile_fail` doctest passes when the snippet fails to compile for any reason at all. Rename a type
in that snippet and it still passes. A7 shows the `trybuild` shape is available for this condition and
produces the named assertion at check time, so the missing case is an omission rather than an
impossibility.

**And the design does not leave the choice open.** `DESIGN.md.tmpl:261-262`, the sentence directly
under the passage quoted in 5.2:

> So both shapes ship and neither is redundant: the doctests cover a declaration reached only through a
> running program, and `tests/ui/` covers the const-bound form with its `E0080` committed beside it.

Both shapes ship. For the phase condition one does. That makes this a design non-compliance rather
than a preference about test tooling, and it sits in the same file, four sections earlier, as the
paragraph explaining why it matters.

`a-hand-check-becomes-a-test-every-time.md` says my A7 should become that test. I have not written it,
because the brief confines me to `mock/research/`, and the probe is committed so the next reader has
the construction rather than a sentence about it.

### 5.5 The verdict on question two

The design satisfies the two rules that file says "hold now". It fails the reproduction property in
four places: one design sentence false against its own code (4.3), one source paragraph contradicting
the design and wrong (5.2), and two things in the code the design does not say (5.3).

**The repair order is fixed and is not the obvious one.** Three of the four are the code deviating and
the code is what gets rewritten. **4.3 is the design being wrong about the world**, so the design
sentence is narrowed first and the code under it rewritten after, which is the mutation order that
file states. Fixing the code to make the design's sentence true is not available, because A3 cannot be
caught by any forcing site: an associated const default is evaluated only where it is named, and a
declaration nobody names is named nowhere.

## 6. The agenda row's framing is the shape op has refused, three times

The brief quotes the workspace agenda row
`phase-of-stores-a-zero-denominator-the-doc-comment-calls-impossible` as framing this "as a choice
between a const refusal on the constructor and the predicates' totality being the whole answer".

**That framing is dead on arrival and the canon says why.**
`ruling::there_is_no_universal_answer_take_the_win_and_gate_it` records op naming this shape, in a
`note` that says "He notes it is the third time a question of this shape had been put to him in one
sitting". `ruling::the_canon_does_not_police_what_shape_a_law_takes` records the same refusal one
topic over, against a question whose three options were "the strongest const-time construction only;
any const-available construction; or leave it to the design tier".

The agenda row is agent output with no human in its loop and it reproduces, for a fourth time, the
question he has already answered by refusing. The answer is the one he gave: take the win where it
applies and gate it out from where it does not. A constructor refusal and a contract obligation are
not rivals for one slot; they refuse at different times, in different tools, over different scopes,
and `never-ask-which-single-rule-governs.md` is a workspace rule that names this failure by name.

The dispatch question is not the agenda's question and is well posed, which is why this file has an
answer rather than a refusal.

## 7. Findings the brief did not ask for

Reported under the standing instruction, and neither is inside the assigned question.

### 7.1 Two registry rows carry a claim about the shipped source that stopped being true

`proposal::the_additive_identity_is_decided_by_the_phase_being_a_whole_multiple_of_the_quantum`,
in its `note`:

> **The shipped `has_additive_identity` implements the superseded claim.**

`question::how_the_stamped_spine_rows_phase_clause_gets_corrected` (`Q67`), in its `note`:

> The shipped code does not wait on this. `has_additive_identity` contradicts the affine predicate the
> same row states one sentence earlier

**A9 measures both false.** At `Phase::of(1, 1)` with `Constant<0>` and `Signed<8>`, a nonzero phase,
the shipped `has_additive_identity` returns `true`, which is the corrected claim. The control arm at
`of(1, 2)` returns `false`, so the function is discriminating rather than answering one way.

The dates say what happened and nothing was done wrong. The registry's last touch before this read is
`5ae69731` at `2026-09-02 00:04:21`; the fix is `da2f9d23`, "fix: the identity turns on a whole
multiple of the quantum", at `2026-09-02 00:06:35`, an ancestor of `67ce00d8`. **Both sentences were
true when written and false two minutes later.** That is the class
`a-claim-of-totality-names-what-enforces-it.md` calls a claim about live data: correct when measured,
carrying no instrument a reader can re-run, and drifting silently. The repair is not an edit in place
to either row; Q67's own `bound` explains why editing a stamped row is the option "that happens by
default when nobody chooses". What is owed is that Q67's `note` name the commit, so a reader can check
rather than believe.

### 7.2 The brief's own framing of the design is incomplete, and in the direction that matters

The brief says `DESIGN.md.tmpl` "has a paragraph on the phase coordinate at lines 87 to 93" and asks
what it does and does not say about where a condition is enforced. That paragraph says nothing about
enforcement, correctly.

**It is not the design's paragraph on this.** `DESIGN.md.tmpl:431-436` states the locus decision
outright:

> **So the phase's condition sits on the contract that reads it, not on its constructor.** A
> denominator of zero names no position on the grid, `Phase::denotes` is that predicate written over
> the coordinate, and `Format::ADMITTED` refuses it where the coordinates are declared together, which
> is the shape `Slots`, `Quantum` and `Ambient` already carry. An invariant a constructor can only hold
> by discarding a value it was handed is not an invariant worth holding there.

And `DESIGN.md.tmpl:733-742` states the condition, the binding time and the totality consequence.

A reader who checked only lines 87 to 93 would report that the design is silent on the locus and that
the source comment is the only statement of it, which is the presumed-wrong tier. That report would be
wrong, and it would be wrong in the direction of manufacturing a design gap. The design states the
decision, states the reason, and states the consequence. Its defect is the guarantee sentence in 4.3,
not silence.

I do not know whether the line range was a slip or a deliberate narrowing. Either way,
`expert-dispatch-defends-the-canon.md` puts checking a brief's factual claims on the dispatcher, and
this one is checkable with one grep.

## 8. What I could not reach

- **No external sources were needed and none were searched.** The question is about this repository's
  canon, design and source, and all three are in the tree.
- **I did not read anything under `mock/research/` dated today**, per the brief. Six such entries
  exist and are unopened, so if any of them already establishes part of this, I have restated it. My
  probes are my own and were built before any of this was written.
- **Whether the condition belongs in the registry is a judgement I did not close.** Section 2.4 states
  it as a gap owed a second independent read, per the two-expert rule.
- **I did not price the repair.** Adding a `Format::ADMITTED` forcing line to `contains`,
  `smallest_step_exponent`, `step_exponent` and `adapt` is four lines, and whether it costs anything at
  monomorphisation is unmeasured. It is unpriced and I am calling it that rather than asserting it is
  free.

## 9. What a fix would have to touch, in the order the chain requires

Not a proposal to act on now, and the design tier is not mine to edit from a research file. Written
down because a precise statement of what is unproven is worth more than a vague one.

1. **`DESIGN.md.tmpl:738`**: narrow the guarantee to what a use-site obligation can deliver, and say
   which functions carry it. The sentence to beat is the one already in the file at line 247, the
   every-function-agrees law, written for the phase condition rather than only the magnitude one.
2. **`DESIGN.md.tmpl:448`**: state that `Phase` carries two unwrap doors and why, moving the argument
   out of `format.rs:93`.
3. **`DESIGN.md.tmpl:197`**: state that the bounded search agrees with the unbounded one, which is a
   property of the answer and belongs in the design.
4. **Then the code**, nuked and rewritten from the changed design: the forcing lines, and
   `format.rs:179-181`'s rustdoc corrected to the design's relative claim.
5. **A `trybuild` case for the phase refusal**, in the shape A7 already compiles, pinning the
   diagnostic the way the magnitude case does.

Item 4 does not close A3, and nothing available closes A3. That is the honest residue: a declaration
nobody names is refused nowhere, and the design's sentence has to admit it.
