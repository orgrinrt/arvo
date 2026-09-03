# Where the zero phase denominator is refused, and what the canon has to say about it

Seat B, answering independently. Probes and their raw output are committed beside this file at
`mock/research/202609021401_probes/locus/`, with `run.sh` as the whole instrument and `output.txt` as
what it printed.

## The two gates, first

**Canon gate: passed.** Checked against `mock/registry/*.toml`, which `mockspace.toml:31` declares as
`canon_paths`, read through `cargo mock query`. Nothing in the question asks for work the canon
forbids. One framing inside the brief's own material does not survive, and the brief already says it is
not a constraint; it is under "the question is malformed" below.

**Test gate: run, green, and it has a hole in exactly the surface this question is about.**
`cargo test --workspace` from `mock/` returns eight result lines, all `0 failed`: 8, 13 of 14, 21 of
22, 10, plus four doctests and five `compile_fail` doctests. Two ignored. I read the bodies rather than
the names in the surface I touch, which is `arvo-format`'s `Phase`, `Format::ADMITTED` and the
predicates over them:

- `mock/crates/arvo-format/tests/compile_fail.rs` holds eight `trybuild` refusals. **None of them is
  the phase denominator.** The eight are the width bound, a host integer at a slot, an index against an
  extent, a phase against a fraction, a law over no magnitudes, a step law off the exponent, a word
  length past the ladder, and an arvo type as a const parameter. `tests/ui/` carries the same eight and
  their `.stderr` files.
- The phase denominator's only refusal in the suite is the `compile_fail` doctest at
  `mock/crates/arvo-format/src/format.rs:183`, and it is a real test rather than a decorative one: the
  control immediately under it at `:206` is the identical shape at a denominator of two and asserts a
  value. A `compile_fail` doctest passes on any compilation failure, so without that control it would
  pass on a typo. The control is there. Credit where it is due.
- **What is asserted nowhere at all**: that the refusal reaches a `const` call site (arms C and G
  below), that an implementor cannot switch it off (arm I), and that `contains` answers for a format
  whose phase does not denote (arm M). Those are fundamental properties of the thing under test, and
  under `the-test-gate` a surface whose fundamentals are asserted nowhere does not describe the thing.

I did not refuse the assigned work over that, and the reason is not politeness. The missing tests are a
gap rather than a fabricated green, nothing in the suite lies about coverage, and the fix does not
belong to the code tier: the design is what is wrong here, and under
`.claude/rules/canon-design-code-chain.md` the design tier moves before the code under it. Writing
those three tests now would pin behaviour the design does not describe, which is the undeclared design
change that rule names. They are owed, they are listed at the end, and the probes beside this file are
where they should be lifted from.

## The brief's facts, checked

Every one holds except a small one worth correcting, because it changes the reachable surface.

- `format.rs:73` is `pub const fn of(num: i64, den: i64) -> Self` and stores both arguments unchanged.
  Confirmed and measured: arm A prints `of(1, 0)` coming back as `(1, 0)`.
- `Phase::halves` at `format.rs:79` fixes the denominator at two. Confirmed.
- `format.rs:227` is `const ADMITTED: () = { assert!(Self::PHASE.denotes().get(), ...) }`. Confirmed.
- `Ambient`, `Quantum` and `Slots` each carry one: `ambient.rs:159`, `quantum.rs:317`, `slots.rs:210`.
  Confirmed.
- `Phase::denotes()` at `format.rs:106` returns a `Bool`. Confirmed.
- **"lines 322, 398 and 433 show `ADMITTED` being forced at use sites" is right about the line numbers
  and wrong about which obligation.** `format.rs:322` and `format.rs:398` force
  `<F as Format>::ADMITTED`. `format.rs:433` forces `<F::Ambient as Ambient>::ADMITTED`, inside
  `radix<F>()`, and says nothing about the phase.

  So `Format::ADMITTED` is forced at **two** sites in the whole crate, both in `format.rs`:
  `cancelling_slot` at `:321` and `has_additive_identity` at `:397`. Nothing else in `arvo-format`
  forces it. `contains` at `:250` does not. `is_admissible_format` at `:241` deliberately does not.
  `step_exponent`, `radix` and `smallest_step_exponent` do not. `apply.rs:355` forces
  `Slots::ADMITTED` through the declared signature and not the format's. That two-site surface is the
  whole mechanism, and it is what arms F, I and M walk through.

- `DESIGN.md.tmpl:87` to `:93` is the paragraph beginning "The phase coordinate holds the pair it was
  declared with". **It says nothing about a zero denominator and nothing about where any condition is
  enforced.** Its subject is normalisation: that the pair is not reduced, that the sign is not moved,
  and that "the two pairs that have no normalisation are answered rather than reinterpreted", which
  names the `i64::MIN` overflow pairs and not the zero denominator.

  **The design is not silent, though. It is loud, 340 lines further down**, and the brief's pointer
  lands one paragraph short of the sentence that answers it. `DESIGN.md.tmpl:431` to `:436`:

  > **So the phase's condition sits on the contract that reads it, not on its constructor.** A
  > denominator of zero names no position on the grid, `Phase::denotes` is that predicate written over
  > the coordinate, and `Format::ADMITTED` refuses it where the coordinates are declared together,
  > which is the shape `Slots`, `Quantum` and `Ambient` already carry. An invariant a constructor can
  > only hold by discarding a value it was handed is not an invariant worth holding there.

  The source comment at `format.rs:64` to `:71` is a restatement of that paragraph, not a punt the
  design failed to cover. So the shape of the problem is not "the code did something the design does
  not mention". It is that the design says where the condition sits, says it clearly, and then
  misdescribes what putting it there actually buys.

## The answer, part one: where the canon requires it enforced

**The condition is canon. The locus is not, and one of op's own statements says the canon will not
supply one.**

### The condition is canon by entailment, not by any row that states it

No row states "the phase denominator is nonzero". Searched the whole registry, with a positive control
so the zeroes mean something: `zero denominator` returns 0 hits and `does not denote` returns 0, while
`phase` returns 80, `affine predicate` returns 8 and `denominator` returns 4, those four sitting in
`probe`, `proposal` and `retirement` prose about arithmetic and rounding, none about a condition on
a coordinate. A case-sensitive search for `ADMITTED` over `mock/registry/*.toml` returns **zero**:
the canon does not name the mechanism at all.

What does carry the condition is entailment from the ratified spine.
`ruling::the_format_spine_is_canon` (`rung = ratified`, `ratified_by = both`) ratifies
`proposal::membership_of_the_representable_set_is_one_affine_predicate`, which says membership is

> one predicate over one parameterisation: an affine slot function, a quantum per magnitude and a
> phase

and `dimension::phase` calls that coordinate

> The offset of a format's grid from zero, in units of the quantum at magnitude zero, **as the rational
> the ratified affine predicate carries.**

A pair whose denominator is zero is not a rational, so it is not a phase, so a `Format` declaring one
declares no member of the parameterisation the spine ratified. That is where the condition comes from
and it is the only place it comes from. It is a consequence of a ratified row rather than a sentence
anybody wrote, which is worth saying plainly, because a design paragraph citing "the canon requires a
nonzero denominator" would be citing a row that does not exist.

### The locus is not canon, and I16 says the canon declines to supply one

`ruling::the_canon_does_not_police_what_shape_a_law_takes` (`rung = stated`, op's own words, key I16):

> I do not think I get the framing. Monomorphisation and const solving should lead everything to go
> through one lowered path, that's it. So if a law is a law, it should be expressed so that it actually
> works, be it typestate or const expressions or whatever. We shouldn't police what kind of laws there
> are or what shapes they take. The law is defined as makes sense and is applicable in each situation
> on a case by case basis.

Its `because` field records why the question that produced it went nowhere: "The question was asking
the canon to do the policing that arvo's toolbox posture forbids." Its `instead` field is the operative
half: "The requirement on a law's expression is functional rather than structural: it must actually
work, meaning it reaches one lowered path."

So the canon's answer to "constructor or contract" is that it is not the canon's to say. What the canon
does impose is a functional test, and the functional test is the thing this design fails, which is part
two.

### What the canon does require of the result

Three rows, and each bounds the answer without choosing a locus.

**`ruling::never_a_runtime_check_and_one_lowered_path`** (`rung = ratified`, op, key I15):

> Runtime code can exist. Its branching should be done as much as possible with const-time ifs erased
> by monomorphisation, const solving and ultimately the backend. Never any runtime checks, ever:
> invalids are caught at compile time and unused paths are cleared when lowered. **Unless there is a
> justifiable reason that is blessed by a lead designer.**

The row's own `note` splits it correctly: "as much as possible" governs the branching, "never, ever"
governs the checks. This rules out one of the two shapes the agenda row offers, and it is worth
measuring rather than asserting, so arm J does. A plain `const fn` carrying an `assert!` refuses at a
`const` call site and **panics at run time**, because a `const fn` is also an ordinary function and
`Phase::of` is `pub`. Arm J passes `cargo check` and `cargo build` and exits 101 when run, with its
control at a denominator of two returning first. That is the runtime check I15 forbids, arriving
through the door a `pub const fn` leaves open, and it is a canon-grounded reason the plain constructor
assert is the wrong shape. It is not a reason the constructor is the wrong place, which arm K settles
separately.

**`ruling::validate_means_all_three_readings`** (`rung = stated`, op):

> Usage, Admissibility, Self-validation, All that makes sense.

Its `note` records what admissibility means: "the typestate refuses declarations it cannot serve". So
his direction is that the declaration is refused, that this is one of three readings rather than the
whole answer, and that the challenge bar for dropping a reading is his phrase "truly not worth it".
Nobody has argued that bar for this condition. A design resting the whole answer on one refusal site is
narrower than the direction, and narrower in a way arms F, I and M make observable.

**`ruling::the_work_is_predicated_arms_composed`** (`rung = ratified`, op, key I13) is why the question
as the agenda row puts it has no answer of the shape it asks for. It rejects a universal solution "by
premise" and requires arms over regions where a predicate holds. "A const refusal on the constructor"
against "the predicates' totality being the whole answer" is a request for one rule over a category,
which is the shape I13 refuses. Both options are right somewhere. The regions are in the table below,
and the composition is at the end.

### So: the question is malformed as a canon question, and the canon closes it

The agenda row `phase-of-stores-a-zero-denominator-the-doc-comment-calls-impossible` frames this as a
choice between a const refusal on the constructor and the predicates' totality being the whole answer.
Under I13 that is the wrong shape, and under I16 the canon has already declined to pick. The row is
agent-written and the brief says so; this is not a complaint about the row's existence, it is the
answer to the question the row poses. **The canon requires the condition, requires that it be caught at
compile time with no runtime check, and leaves the construction to the design tier under a functional
test.** Where the canon is silent is on the locus, and silence is not permission to leave the functional
test failing.

## The answer, part two: what the canon requires the design to state

**That it works, and where.** Two rows carry it, and the design fails both in the same paragraph.

`ruling::the_work_is_predicated_arms_composed` requires that "every finding must be predicated,
including universal ones, so that where it holds is said exactly rather than assumed". A design
sentence claiming a guarantee is a finding in that sense, and two of them here are unpredicated
universals that are false in three measured regions.

`ruling::the_canon_does_not_police_what_shape_a_law_takes` supplies the test the construction has to
pass: it "should be expressed so that it actually works". The construction the design picked does not,
in the sense that matters, and the registry already says which sense.

### The design contradicts itself about its own mechanism, and the correct version is the one it states first

`DESIGN.md.tmpl:255` to `:258` states the mechanism exactly right, and every arm below reproduces it:

> **Where an obligation is forced decides which tool can see its refusal**, and that is the axis rather
> than a property of either tool. An obligation is a const and a const is evaluated where it is used.
> Forced from a runtime call it is reached at codegen, which `cargo check` skips, so a doctest, which
> builds a binary, is what catches it. Forced in a `const` item it is reached at check time, and a
> `trybuild` case both sees it and pins the exact diagnostic.

Then `DESIGN.md.tmpl:683` to `:685` says the opposite, as a property of the verb:

> Each obligation is a const evaluated at monomorphisation, so it fires during codegen. `cargo build`
> refuses. `cargo check` does not, because it skips codegen. So the guarantee is that an inadmissible
> range or an inadmissible law cannot reach a produced binary, and it can reach a passing check.

And `:737` to `:738` repeats it for this obligation specifically:

> **Each fires exactly where the slot range's does**, for the same reason and with the same limit: a
> const evaluated at monomorphisation, so codegen refuses and `cargo check` does not. The guarantee is
> that an inadmissible declaration cannot reach a produced binary, and it can reach a passing check.

Both halves of that are measurably false, and the source comment at `format.rs:179` to `:181` carries
the same false pair into the crate's rustdoc: "It fires at codegen, not at `cargo check`, so
`cargo build` refuses and `cargo check` does not."

- **"`cargo check` does not" is false.** Arms C and G refuse at `cargo check`, exit 101, because the
  call site is a `const` item and a `const` item is evaluated at check time. Arm G's const item is
  never read and it refuses anyway, so this is not about liveness.
- **"cannot reach a produced binary" is false, three ways.** Arm F declares the inadmissible format,
  reads `PHASE` straight off the impl, forces nothing, and prints `1/0` out of a binary that built
  clean. Arm I adds one line, `const ADMITTED: () = ();`, and gets `has_additive_identity` to answer
  out of a binary. Arm M asks `contains` for coordinate membership of that format and gets `true`.

### The design already holds the principle that refutes its own sentence, one contract over

`DESIGN.md.tmpl:801` to `:805`, under the heading "The adaptation forces the admission obligation":

> `adapt` reads a slot range from the declared signature. **Reading an associated const does not force
> the contract's obligation on its own**, so `adapt` forces it, and the range the completion works over
> is one that has been admitted rather than one that merely arrived.

That is arm F's entire mechanism, correctly stated, and correctly acted on for `Slots`. It is not acted
on for `Format`: `contains` at `format.rs:250` reads the format's coordinates and forces nothing, which
is exactly the case the paragraph is about. So the defect is not that nobody thought of this. It is
that the reasoning was applied to one contract and not to the neighbouring one, and the guarantee
sentence was written as though it had been applied everywhere.

### The registry already names this construction, and calls it what it is

`proposal::the_licensed_category_is_const_available_and_four_constructions_bind_at_four_times`
(`standing = one_expert`, topic `binding_time`, evidence
`probe::the_four_const_available_constructions_bind_at_four_times`, whose own `standing` is `sound`):

> Four constructions bind at four times: a crate-level const and a structural trait bound refuse on
> dead code under metadata-only emission; an inline const in a non-generic function refuses at code
> generation; and **a const assertion in a generic's associated const refuses only where the
> instantiation is reached, so a wrong declaration in an unreached public function compiles clean at
> every setting.**

`Format::ADMITTED` is that fourth construction exactly. And the row's `because` field names the failure
in advance:

> The reachability-dependent construction is the one that matters: **it is not a banned category, it is
> a construction that fails the functional test when it is used for a library-wide claim**, and the
> specification that makes the split safe is one sentence, that a reachability-conditioned permission
> quantifies over reached instantiations while a library claim must sit at the unconditional level.

"An inadmissible declaration cannot reach a produced binary" is a library-wide claim. It is stated over
a reachability-conditioned construction. The registry said that fails the functional test before this
design was written, and it is the same functional test I16 imposes.

**Standing, stated honestly.** That row is a `proposal` at `one_expert` and no ruling ratifies it, so it
is not canon and I am not citing it as canon. What it is, is a prior independent arrival at the same
mechanism over a different object: its instrument swept four construction shapes across emission
profiles and debug-assertion settings for law permissions, mine walks one construction across call-site
kinds and verbs for a format obligation. I ran the arms before I found the row, so this is a second
instance rather than a reading of the first. The two agree about the intersection of their dimensions
and not the union: the intersection is the reachability behaviour of the associated-const construction
under metadata-only emission and under full codegen, and that is the whole of what the pair supports.
Neither of us varied threads or target features on this claim.

## The measurements

Eighteen arms, every one under `cargo check` and `cargo build`, and run where a build survived.
Toolchain `rustc 1.98.0-nightly (57d06900f 2026-05-27)`, `cargo 1.98.0-nightly (fbb61be30 2026-05-26)`,
target `aarch64-apple-darwin`, profile `dev`. Instrument
`202609021401_probes/locus/run.sh`, raw output `202609021401_probes/locus/output.txt`, sources under
`src/bin/`. `stderr` is folded into `stdout` throughout, because a refusal is the result here and a
discarded `stderr` turns every refusal into a silent pass.

| arm | what it is | check | build | run |
|---|---|---|---|---|
| A | `Phase::of(1, 0)`, nothing else | 0 | 0 | stores `(1, 0)`, `denotes=false` |
| H | the same pair in a `const` item, no `Format` | 0 | 0 | `1/0` |
| B | `Format` declaring it, run-time call forcing `ADMITTED` | **0** | **101** | refused |
| C | the same, called from a read `const` item | **101** | 101 | refused |
| G | the same, called from an unread `const` item | **101** | 101 | refused |
| D | the same, verdict form only (`is_admissible_format`) | 0 | 0 | `verdict=false`, control `true` |
| E | control: the identical shape at denominator two | 0 | 0 | builds and answers |
| F | the same declaration, `PHASE` read directly, nothing forced | 0 | 0 | **`1/0` out of a binary** |
| I | the same plus `const ADMITTED: () = ();` | 0 | 0 | **answers out of a binary** |
| M | the same, asked `contains` | 0 | 0 | **`in-range=true`, control `false`** |
| J | a `const fn` constructor carrying `assert!`, run-time call | 0 | 0 | **panics, exit 101** |
| J2 | the same constructor, `const` call site | 101 | 101 | refused |
| K | denominator as a const generic, good instantiation | 0 | 0 | builds and runs |
| K2 | the same, bad instantiation from a non-`const` function | 0 | **101** | refused |
| L | a blanket-impl obligation, well-formed declaration | 0 | 0 | forces and builds |
| L2 | the same against a declaration that wrote `ADMITTED` | **101** | 101 | refused |
| L3 | the same, from a run-time call site | 0 | **101** | refused |
| L4 | the second `impl Admits` an implementor would need | **101** | 101 | `E0119` |

Every arm carries the case that had to fail, and the ones that decide anything have a control that
fired. E is B minus the denominator and it builds, so B's refusal is the denominator and not the shape,
the crate or the toolchain. D asserts the control declaration comes back `true` through the same
verdict, so `false` is not what that function says regardless of input. M asserts an out-of-range slot
comes back `false`, so `contains` is not answering `true` to everything. J prints its denominator-two
control before the panic, so the panic is the denominator. L's control forces the blanket obligation on
a well-formed declaration and builds, so L2 and L3 are not a trait that refuses everything. L4 is the
negative control on the word "cannot": the disarming impl is `E0119`, a coherence error, rather than a
language rule quoted from memory.

Two arms deserve their diagnostics rather than a table cell.

Arm I, in full, is the four lines of an ordinary impl plus one:

```
const PHASE: Phase = Phase::of(1, 0);
const ADMITTED: () = ();
```

and it prints `I: verdict=false has_additive_identity=false` from a binary that `cargo build` produced
without a word. `ADMITTED` is a defaulted associated const, `DESIGN.md.tmpl:300` says so and says an
implementor "supplies it by supplying coordinates that meet it and never writes it out", and nothing
anywhere stops the implementor writing it out. The party the obligation constrains owns the check for
the thing it constrains, inside the same impl block. **This registry has already named that exact
class**, at `retirement::dl_fidelity_licence_as_a_trusted_marker_trait`: "the implementor writing the lie also
controls the check for the lie inside the same impl block". It was named there about a permission
witness, it was retired for it, and the same shape is load-bearing in shipped source here.

Arm M is the one I would put in front of anybody who thinks this is a corner. `contains` is the crate's
own spelling of the ratified membership predicate, its module doc at `format.rs:19` to `:24` says so,
and it answers `true` for a coordinate of a format whose phase does not denote, in a binary, having
forced nothing. Whether that answer is wrong depends on a question the design does not settle: whether
coordinate membership means anything when the coordinate-to-value map is undefined. It is defensible
that it does, because `contains` quantifies over slots and magnitudes rather than over values. It is
not defensible that the design does not say which, while claiming the inadmissible declaration cannot
reach a binary at all.

## The design against the source, under the chain rule

`.claude/rules/canon-design-code-chain.md` asks four things of this relation. I checked each.

**Does the design declare the canon it relates to?** Yes, and well. `DESIGN.md.tmpl:1` to `:11` names
eight `ruling` slugs and two `proposal` slugs. **All ten resolve.** I ran each through
`cargo mock query`; none is a name pointing at nothing and none is archived. The rule calls naming a
canon that does not exist "a hard failure on every gate", and this design passes it cleanly.

**Does anything appear in the code that is not in the design?** Yes, one thing, and it is the failure
the rule singles out as the most common. `Format::ADMITTED` is overridable by the implementor and an
override disarms the obligation entirely. The design says at `:300` that an implementor "never writes
it out", which describes the well-behaved case and is not a statement that the other case is refused,
because it is not refused. Arm I is behaviour in the code that the design does not describe, in the one
place where the design's guarantee lives. Under the rule that is an undeclared design change wearing
the leaf tier's freedom, and it does not stop being one for having arrived by omission rather than by
edit.

**Does the design misdescribe the code?** Yes, at `:685`, at `:738`, and in the rustdoc at
`format.rs:179` to `:181` that repeats them. The design's own `:255` to `:258` is correct and its own
`:801` to `:805` holds the principle that refutes the guarantee sentence. So this is an internal
contradiction rather than a design that was simply wrong: two sections say the mechanism is the force
site, two say it is the verb, and the ones that say the verb are the ones a reader arrives at, because
they sit in the section headed by the obligations.

**Does it hold the reproduction property?** No, and this is the part I would fix first. The rule's
acceptance test is that "two implementers, reading it independently, produce working implementations of
the same thing". **The design never says which functions force which obligation.** It says the
obligations exist, it says where an obligation is forced decides what sees it, it names one force site
in prose for `adapt`, and it stops. A second implementer reading this design could force
`Format::ADMITTED` inside `contains` and produce a crate that refuses arm M where this one answers it,
and both implementations would satisfy every sentence in the document. The force-site set is the whole
observable content of the guarantee, and it is the one thing the design leaves to the implementer.

That is a design defect with a cheap fix and it is not a code defect. The code is a faithful
transcription of what the design says; the design says too little and, where it says more, says two
incompatible things.

## The composition, which is what I13 asks for instead of a winner

Four constructions, four regions, and the honest answer is that the design wants more than one of them.
Each was built and measured rather than reasoned about.

**1. The defaulted associated const, which is what ships.** Refuses at check from a `const` call site
(C, G), at build from a run-time call site (B), and not at all from a declaration nothing forces (F, M)
or from an implementor who writes the const (I).

Predicate: `holds for: toolchain = nightly-2026-05-28, target = aarch64-apple-darwin, profile = dev,
verb in {check, build}, call site in {const item read, const item unread, run-time fn}, implementor
writes ADMITTED = no, threads = 1`.

Keep it. It is not wrong, it is narrower than the design claims, and the fix is the claim.

**2. The blanket impl over `F: Format`, which closes the disarm.** `trait Admits { const OK: (); }` with
one `impl<F: Format> Admits for F`, and the force sites read `<F as Admits>::OK` instead. Refuses the
disarmed declaration at check from a `const` site (L2) and at build from a run-time site (L3), matching
construction 1 on the verb axis, and the disarming impl is `E0119` (L4) rather than a no-op.

Predicate: same as above with `implementor writes ADMITTED = yes` added to the region, `threads = 1`.

**This is a strict improvement over what ships and it costs one trait and one blanket impl.** It does
not narrow the open inventory: an outside crate implementing `Format` gets `Admits` from the blanket,
which is the same reason a second impl is refused. Measured, not argued: L's control forces `OK` on a
well-formed declaration and builds, and L4's `E0119` is the mechanism.

What it does not close is F and M. Nothing that is a const forced at a call site can, because the whole
point of F is that there is no call site.

**3. The denominator as a const generic on the constructor, which closes the run-time door.**
`PhaseK::of::<DEN>(num)` with an inline `const { assert!(DEN != 0) }`. Refuses at build from an
ordinary non-`const` function (K2), and there is no run-time value of `DEN` for a caller to route
around it. Contrast arm J, the plain `const fn` assert, which passes both verbs and panics at run time,
which I15 forbids.

Predicate: `holds for: toolchain = nightly-2026-05-28, target = aarch64-apple-darwin, profile = dev,
verb in {check, build}, call site in {run-time fn, const item}, threads = 1`.

**This is the one that could close F and M**, because it makes the non-denoting pair unconstructible
rather than unusable, and a `Format` cannot then declare one. What it costs is real and I am not
hiding it: it moves the denominator out of the value domain into the const-generic domain, which
changes `Phase`'s spelling at every call site, and `ruling::the_numeric_door_carries_the_coordinate_set_and_the_two_type_bound_is_not_canon`
records that the const generic parameter position is where the coordinate types already do not reach.
The design's own reason for the current spelling, at `:431` to `:436`, is that "an invariant a
constructor can only hold by discarding a value it was handed is not an invariant worth holding there",
and that reason is exactly right about arm J and does not reach arm K, where nothing is discarded
because nothing invalid can be handed over. So the design's argument against the constructor is sound
against the construction it had in mind and untested against this one.

I am not proposing it. I am recording that the argument in the design closes one door and leaves this
one open, and that it measures.

**4. The totality of the predicates, which is not an enforcement and is not useless.** `denotes`,
`is_whole_multiple`, `is_admissible_format` and the guards inside `cancelling_slot` and
`has_additive_identity` answer rather than diverging. Arm D is where that earns its keep: the verdict
form is the one call site at which a zero denominator is meant to be observable, and it is, correctly,
with its control.

But **the guards inside the two forcing functions are dead in every binary that can exist**, and this
follows from the arms rather than from reading. `has_additive_identity` forces `ADMITTED` at
`format.rs:398`, so a run-time call site refuses at build (B) and a `const` call site refuses at check
(C, G). The only way to reach the guard's answer is arm I, where the implementor has already disarmed
the obligation. `DESIGN.md.tmpl:740` justifies the totality as covering "the check-time evaluation an
obligation cannot reach", and arms C and G show the obligation does reach check-time evaluation. So the
stated reason for that branch is wrong even though the branch is worth keeping, and the real reason is
the one arm I exhibits: it is what stops a disarmed declaration from dividing by zero.

That is a better argument for the totality than the design's own, and it is the argument the design
should make. If construction 2 lands, the disarm goes and the branch becomes genuinely unreachable, at
which point keeping it is a judgement rather than a necessity.

## What is owed

Stated as work rather than as a complaint, and none of it is mine to do from inside `mock/research/`.

- **The design tier moves first.** `DESIGN.md.tmpl:685` and `:738` state an unpredicated universal that
  is false in three measured regions. `format.rs:179` to `:181` repeats it in rustdoc, which ships. The
  correct statement is already in the same document at `:255` to `:258` and needs carrying down.
- **The design owes the force-site set**, because it is the whole observable content of the guarantee
  and two implementers cannot reproduce this crate without it.
- **The design owes a sentence on the override**, because `ADMITTED` is defaulted and arm I is one
  line.
- **The design owes a sentence on `contains`**, saying whether coordinate membership is meaningful for
  a format whose phase does not denote. Either answer is fine; the silence is not, because arm M
  answers `true` today and nothing says whether that is the intent.
- **Three tests are owed once the design says what they should assert**: the check-time refusal (arms
  C, G), the override (arm I), and `contains` on a non-denoting format (arm M). Lift them from
  `202609021401_probes/locus/src/bin/`. The first two want `trybuild` cases under `tests/ui/` with the
  `E0080` committed beside them, which is what `DESIGN.md.tmpl:261` to `:262` says that mechanism is
  for and which the phase denominator does not currently have.
- **`proposal::the_licensed_category_is_const_available_and_four_constructions_bind_at_four_times`
  stands at `one_expert` and now has a second instrument on it**, over a disjoint object, reached
  before its author read the row. Whether that is worth a second standing is not mine to decide; the
  instrument is committed and the intersection is stated above.

## What I could not reach

- **Whether the canon means to say anything about the locus at all.** I searched every namespace with
  controls and found nothing, and `ruling::the_canon_does_not_police_what_shape_a_law_takes` reads as
  deliberate rather than as a gap. But it is `rung = stated` rather than ratified, and it was said about
  laws rather than about contract obligations. Extending it to obligations is my reading and is marked
  as mine. Under the two-expert rule this is a first read and a second is owed.
- **Whether arm M's `true` is a defect or correct behaviour.** It turns on what `contains` quantifies
  over, and the design does not settle it. I have not resolved it and I am not licensed to.
- **Debug-assertion settings, target features, threads, and the release profile.** Every arm ran at
  `profile = dev`, single-threaded, with default features. Nothing here claims anything outside that.
- **Nothing outside the repository was needed**, so the search-path tiers went unused.

## Blindness

Blind against everything under `mock/research/` dated today and against every other worktree in the
workspace; I opened neither and did not look for the other seat's branch. **Not blind against the
registry**, which the brief names as the governing canon and instructs me to query, and not blind
against `mock/crates/arvo-format/`, which is the object. The registry's last touch in my base is
`83212d7e`, a prose edit dropping banned words, which is an ancestor of my head. The probes were built
and run before I found
`proposal::the_licensed_category_is_const_available_and_four_constructions_bind_at_four_times`, and the
commit ordering on this branch is what says so.
