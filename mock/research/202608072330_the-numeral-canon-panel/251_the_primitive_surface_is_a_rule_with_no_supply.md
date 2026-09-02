# 251. The primitive surface is a rule with no supply, and four of the five rows are not the shape of the hole

Locus A, derived independently. Written before opening anything by the other reader on this question.

## 0. The two gates, and what they returned

**Canon gate: aligned.** Checked against the `ruling` namespace read whole, the two rows on
`topic = operating_constraints`, the four stamped by `ruling::the_format_spine_is_canon`, and
`mockspace.toml`'s `[primitive-introductions]`. The question I was sent is a question the canon
permits: it asks what the ratified rows say about a surface, and it does not presume the surface
exists. It also does not presume the five rows are correct, which matters, because four of them are
not.

**Test gate: passed, and it is a real pass rather than a green one.** `cargo test --workspace
--all-targets` from `mock/` is green: 13 in `arvo-format`'s parity suite, 21 in `arvo-placement`,
10 in `arvo-strategy`, plus the doctests and the `trybuild` cases, with two catalogued reds carried
honestly as `#[ignore = "catalogue: ..."]` naming what they wait on. 781 assertions across the three
crates. I scanned for the shapes that disqualify a suite: zero assertions with identical operands,
zero `assert!(true)`, and nine distinct `the_control_*` arms that exist to fail if the thing they guard
stops discriminating. `arvo-format/src/tests/obligations.rs:14` splits each obligation into a runtime
verdict and a build refusal and explains why the tool that can see each differs, which is the
opposite of decorative.

One narrowness I confirm rather than report, because
`ruling::the_numeric_door_carries_the_coordinate_set_and_the_two_type_bound_is_not_canon` already
carries it: `the_format_inventory_admits_a_member_this_crate_does_not_know_about`
(`arvo-format/src/tests/the_inventory.rs:49`) asserts the ratified open-inventory clause using a
`Ternary` whose radix is `Radix::DECIMAL` and which borrows the crate's own quantum and slot range.
The ruling's wording is exact and I add nothing to it.

**A first run of the suite reported success and had not run.** `timeout` is not a command on this
host, so `timeout 900 cargo test` exited 0 having compiled nothing. Recorded because the exit code
was zero and the output was one line, which is what a passing filtered run also looks like.

## 1. The short answer

**The canon licenses the surface. It does more than license it: the surface is already a rule, and
the rule is in force.** `ruling::the_operating_constraints_are_intents_and_rules` carries at
`rung = "in_force"`, `key = "I14"`, an enumeration whose fifth entry is that public API positions use
the stack's own primitives rather than bare integers, floats, `bool` or `usize`. Op's verbatim on it
is that these are "very explicitly also arvo intents and rules" and "not to be questioned".

**And the canon says nothing whatever about what the primitives are.** `ruling.where(topic=the_primitive)`
returns zero rows. Eight proposals sit under that topic, at `one_expert` through `three_or_more`, and
none is ratified. So there is a ratified prohibition on bare primitives at API positions and no
ratified statement of what goes there instead.

That is the whole finding, and everything below is either its evidence or its consequences.

## 2. What the canon does say, part by part, with what each rests on

### 2.1 The rule exists and is in force

`ruling::the_operating_constraints_are_intents_and_rules`, `rung = "in_force"`. Its `says` enumerates
five constraints; `INTENTS.md` I14 spells them as bullets, and the fifth is the one this question is
about. `rulings-with-no-verbatim` reports it as resting on his words.

**Read the third bullet before assuming a conflict.** "No platform dependency" is glossed in I14 as
"no `std::thread`, `std::time`, `std::fs`, `std::net`". It is about std facilities, not about
platform-width types. I went looking for a tension between that bullet and
`obligation::a_platform_sized_unsigned_integer_at_an_api_position` and there is none. Saying so
because the tension is the obvious read and it is wrong, and I would have shipped it if I had
reasoned from the `says` field rather than opening the source it names.

### 2.2 Where a primitive may be introduced is ratified, and it is one crate

`mockspace.toml:1418` carries `[primitive-introductions] arvo-format = ["numeric"]`, one entry, with
the comment recording that the eight before it named the deleted tree and that one of them silently
re-exempted a new crate that reused a dead one's name.

`ruling::the_numeric_door_carries_the_coordinate_set_and_the_two_type_bound_is_not_canon` is ratified
on two blind instances and says what that door carries out: "the coordinate set of the ratified
parameterisation, spelled in types the stack owns." It says explicitly that how many types that is
stays open, because the two derivations disagreed and neither the canon nor a third reading settles
it.

**The door is about the declaration, and all five rows are about values.** A consumer's OS error code
is not a coordinate of a parameterisation. So the ratified answer about the door does not reach the
question I was sent, and reading it as though it does is the error available here.

### 2.3 The concept is closed and the inventory is open

`ruling::the_format_spine_is_canon` stamps four propositions, among them
`the_concept_is_closed_and_the_inventory_is_open` and
`membership_of_the_representable_set_is_one_affine_predicate`, under which integers, fixed point,
scaled integers and floats are points of one predicate rather than four kinds of thing.
`arvo-format/src/lib.rs:82` ships all four as worked instances and says in as many words that none is
privileged by being there.

**This is what decides the decomposition question in section 4.** A new numeral joins by implementing
`Format`. It does not need a canon row, and asking the canon for one per consumer use case is asking
it to reopen a clause it closed.

### 2.4 There is no container, and that word is in one of the five rows

`ruling::behaviour_is_stated_per_declared_signature_and_the_premise_dissolves` is ratified and says
the container premise is malformed as a binary and both branches are false: every operation is a
function of the declared width and never of the machine carrier, and the footprint is observable
through exactly one observation, at sole occupancy only.

### 2.5 Silence, stated plainly

- **Debug output: silent.** I grepped `ruling`, `proposal`, `law` and `proposal-the-later-topics` for
  `Debug`, `core::fmt`, `fmt`, `formatting` and `printab`. Every hit is the debug build profile, in
  `ruling::the_overflow_panic_is_permitted_and_bounded` and its correction. Nothing is about output.
- **A platform-sized numeral: silent at the ruling tier**, and touched once at `one_expert` by
  `proposal::each_choice_in_the_sequence_has_an_owner_and_a_resolution_time`, which argues a
  platform-width numeral is a target-indexed family whose exclusion grounds reach only dependence
  surviving to runtime.
- **Containment of unstable features: silent, and the silence is load-bearing.**
  `question::what_the_numeric_introduction_door_may_carry_out` uses
  `obligation::the_unstable_machinery_does_not_reach_a_consumer` in its `bound` to rule out its own
  second option, so an unratified obligation row is doing work a ratified question turned on. The
  ruling that answered that question refused all four options anyway, so nothing downstream is wrong;
  the load-bearing use of an unratified row is the thing to see.
- **Op's own const-generic exception: not canon anywhere.** The clause bounding I14's fifth bullet,
  "No bare usize other than in const generics ... and even there, only when truly painful otherwise",
  is op's, quoted in `obligation::a_primitive_for_every_position_a_bare_number_would_take`. I grepped
  every `ruling` row for "const generic": one hit, inside the `note` of the door ruling, describing
  somebody's proposal. **An op-stated bound on a ratified rule is living one tier below the rule it
  bounds**, in the namespace the ladder calls presumed wrong.

## 3. Where the surface would sit, and the answer is a crate that does not exist

### 3.1 arvo ships no value layer, measured

`251_probes/p01_the_value_layer_is_absent` compiles against `arvo-format` and asserts, as const
evaluation, that all four shipped points are zero sized at eleven instantiations across the four
families. Two controls sit beside it: the coordinate newtypes are not zero sized, and each is exactly
its host width. Without the first control the check would pass against a crate that made everything a
marker. **I mutated one assertion to `== 4` and watched `E0080` fire**, then restored and rebuilt, so
the instrument is known to be able to fail.

So `Integer<32>` declares which values exist and holds none of them. Every value-carrying public type
in the three crates is one of three things and no numeral is among them:

- a coordinate of a declaration: `Width(u32)`, `Bool(bool)`, `Slot(i64)`, `SlotCount(i64)`,
  `Exponent(i32)`, `Magnitude(u32)`, `MagnitudeCount(u32)`, `Radix(u32)`, `Arity(u32)`;
- an intermediate of the adaptation: `Fraction`, `Exact`, `Dither`;
- a derived placement: `Placement`.

### 3.2 The consumer's own position, compiled

`251_probes/p02_the_consumer_position_has_no_type` takes
`hilavitkutin-linking/src/error.rs:30` at `313d427aeae1f5f4c98678a49f1f8aaea84fdd55`, which reads
`LoadFailed { platform_code: USize }`, and asks arvo for the type. The refusal is committed with its
stderr:

```
error[E0432]: unresolved import `arvo_format::USize`
  |     ^^^^^^^^^^^^^^^^^^ no `USize` in the root
```

**With a positive control on the same rustc invocation**, `the_position_control.rs`, identical in
shape with `Width` in the field: exit 0, empty stderr. So the `E0432` is a fact about `USize` and not
about how I called the compiler. My first attempt at that control used `-o /dev/null` and failed with
a temp-dir error, which would have read as a second refusal; it is redone with `--out-dir`.

The probe also pins that range is not what refuses this. `Width` would hold a `GetLastError` value on
range alone. What refuses it is that `Width` counts bits.

### 3.3 So the locus, stated as precisely as I can

**The surface belongs in a crate above `arvo-format` and `arvo-placement` that does not exist, whose
topic the canon has already named `the_primitive`, and in which zero rows are ratified.**
`proposal::the_topics_form_a_stack_a_frame_and_the_canons_own_machinery` puts the primitive on the
realisation map and the number system, which agrees; it is `one_expert` and it is the coordinator's
own derivation, so it is corroboration of the weakest kind and I cite it as such.

`mock/Cargo.toml:25` has three members. There is no `arvo` facade crate, which is what every consumer
imports from.

### 3.4 What that costs the consumers now, measured with a control

`251_probes/p03_the_consumer_break_surface/measure.sh`, reading the object store at named refs, never
a working tree, with output committed beside it:

| repo | ref | commit | files | occurrences of `USize` |
|---|---|---|---|---|
| hilavitkutin | `origin/dev` | `313d427a` | 75 | 1375 |
| kolli | `origin/dev` | `c7b549e2` | 11 | 108 |
| vehje | `origin/main` | `5da3d105` | 3 | 5 |

89 files and 1488 occurrences, plus 90 more in hilavitkutin across `ISize`, `Cap`, `Bits`, `UFixed`,
`IFixed`, `FastFloat`, `Mask64` and `Mask256`. The script carries a control name no crate defines and
it reads 0 in every tree; a non-zero there would void the table.

**An earlier run of that measurement returned zero everywhere and I nearly believed it.** The loop
used `set -- $pair`, which is bash, and this shell is fish, so the repo and ref variables were empty
and every `git -C ""` ran against the wrong directory. It is in this file because the zero was
plausible, quiet, and in the direction that would have made the finding smaller.

## 4. The decomposition is wrong in four of five places, and the canon implies a different cut

Measured with the repository's own instrument before I argue anything: `mock obligation-coverage`
over the five reports **`tier: nothing` for four of them**, and for the fifth,
`an_exact_width_container_a_consumer_can_alias_and_pin`, a single naming from
`ruling::ingest_is_the_consumers_and_the_c_abi_is_where_it_ends_up` at `rung = "stated"`, which is
the weakest rung that is not absence.

### 4.1 `a_primitive_for_every_position_a_bare_number_would_take` is a ruling wearing an obligation's clothes

Its `need` is I14's fifth bullet. Under the provenance ladder a `ruling` at `in_force` governs and an
`obligation` is demand-side enumeration read from outside the canon, so the same sentence now exists
at two tiers and a reader can cite the weaker one. **The row's only content beyond the ruling is op's
const-generic exception**, which section 2.5 shows is canon nowhere.

That inverts what the row is for. The part that duplicates a ruling is redundant; the part that is
unique is the part that should have been a ruling and is not.

**Its `gap` is right and is now answerable.** It says the obligation "is not satisfied by counting the
primitives that exist: it is satisfied by the positions, and nothing has enumerated those."
`mock the-positions` has since enumerated them, and I ran it over four consumer trees at named
commits (`251_probes/the_positions.out`): **193 positions**, 178 free and 15 on a boundary where the
width is an operating system's or a foreign ABI's. By role: truth 64, unclassified 33, count 27,
capacity 21, identity 15, version 10, code 6, index 6, bit-width 5, opaque-bits 4, real 1, stride 1.
It also reports **1424 `lint:allow` sites in shipped consumer source** turning the rule off, 861 of
them on `no-bare-numeric`, every one naming a task. The gap sentence is stale and the row does not
say so.

### 4.2 `a_platform_sized_unsigned_integer_at_an_api_position` names a mechanism its own `why` refuses

Its `id` and `need` say platform-sized. Its `why` says "The consumer states the property rather than
the type: non-negative across both platforms' ranges." Those are different requirements and the row
holds both.

**This is the identical defect `obligation::a_build_flag_that_changes_float_semantics` documents about
itself**, whose `gap` says the row "used to state the mechanism ... which the field description
forbids in as many words: the need, never the mechanism that serves it." That row was caught and
carries its correction. This one is not marked by anything, and `obligation-coverage` reports it
clean, because no check reads a `need` against its own `why`.

The need as the `why` states it is met by any unsigned numeral of 32 bits or more: `errno` is a C
`int` and `GetLastError` returns a `DWORD`. Platform sizing is the consumer's vehicle. Under
`ruling::the_format_spine_is_canon` an unsigned 32-bit numeral is a point of the one parameterisation
and needs no canon row at all.

**Two further facts about the quotation, neither of which the row carries.** The consumer's own doc
comment three lines above the field it quotes says the codes are carried as `arvo::ISize`, signed,
while the field is `USize`; the consumer disagrees with itself. And `USize` is destructured as
`USize(val as usize)` at `hilavitkutin-linking/src/backend/unix.rs:123`, so the shape being asked for
has a public tuple field, which is what `no-public-raw-field` exists to refuse.

### 4.3 `an_exact_width_container_a_consumer_can_alias_and_pin` is on the wrong side of a ratified dissolution

It says container. `ruling::behaviour_is_stated_per_declared_signature_and_the_premise_dissolves`
ratified that the container premise is malformed and that behaviour is stated per declared signature.
`arvo-format/src/lib.rs:12` opens with "A format here is not a container."

**The need survives the rewording and the ratified canon supplies every part of it.** A declared
width is a declared signature; an alias is a type alias over one; and the conversion at a boundary
the consumer controls is what
`ruling::ingest_is_the_consumers_and_the_c_abi_is_where_it_ends_up` already assigns to the consumer,
which is why that is the one ruling naming this row. Only the word is wrong, and the word is what a
later reader builds from.

### 4.4 `debug_output_from_every_numeral_shape` gives a reason that is false, and I built the refutation

Its `why` says "the no-alloc constraint means the ordinary route is unavailable." The ordinary route
is `core::fmt::Debug` rendered into a caller-supplied sink, `core::fmt` is in `core`, and it needs
neither `std` nor `alloc`.

`251_probes/p04_debug_into_a_caller_buffer` is a `#![no_std]` crate with a fixed-array buffer
implementing `core::fmt::Write` and refusing rather than truncating on overflow. Four tests, all
passing: `Width::bits(13)` renders to `"Width(13)"`, `Bool::of(true)` to `"Bool(true)"`,
`Slot::at(-7)` to `"Slot(-7)"`, a four-byte buffer refuses a nine-byte rendering and reports it, and
two different widths render differently. The last two are controls: without them the first would pass
against a sink that dropped writes or a renderer that printed a constant. **arvo-format already
derives `Debug` on 18 of its declarations**, so the route is not merely available, it is in use.

What is genuinely unavailable is the four points, and the reason is section 3.1 rather than no-alloc.
Committed with its stderr:

```
error[E0277]: `Integer<32>` doesn't implement `Debug`
  = help: the trait `Debug` is not implemented for `Integer<32>`
```

**The row also quantifies over an open set.** "Under every strategy" ranges over a set
`ruling::the_strategy_set_is_not_closed_at_four` holds open at `rung = "open"`, so as written the row
can never be discharged. The nearest ratified text that reaches Debug at all is
`ruling::warms_objective_is_the_intuitive_best_choice` (I4), whose mimicry clause would arguably carry
it at one preset. **That is my reading and not canon**, and I mark it so rather than filing it as what
the design says.

Its provenance is the weakest of the five: no consumer states it, and the evidence is a check written
against a deleted tree.

### 4.5 `the_unstable_machinery_does_not_reach_a_consumer` is the one that is right, and it is misfiled

It is not a request for a type, its own header says so, and it is the only one of the five that adds
something no ruling carries: a bound on what arvo may use internally, stated by a consumer as a
condition of adopting arvo at all. It bounds a live ratified question. Nothing reaches it.

**What is wrong is where it sits.** It is a constraint on every arm, which is the shape of I14 rather
than the shape of a consumer's want, and it is filed on `topic = "binding_time"` in a namespace read
from outside the canon. Its `gap` says "Nothing in the registry is about this, and it is not implied
by anything that is." That is a canon-shaped hole being tracked in the demand-side ledger.

### 4.6 The cut the canon implies

Four groups, and only two of them are canon work:

1. **The rule.** I14's fifth bullet, already ratified and in force, plus op's const-generic exception,
   which belongs beside it and is not there. **Canon work, and it is one row's worth.**
2. **The value layer.** There is no type that holds a number. This is `topic = the_primitive`, zero
   ratified rows, and it is what four of the five requests actually need. **Canon work, and it is the
   whole of what is missing.**
3. **Inventory entries.** A 32-bit unsigned numeral for an error code, a 28-bit one for a content
   hash. Under `the_concept_is_closed_and_the_inventory_is_open` these join by implementing `Format`,
   and admission is a check rather than a negotiation. **Not canon work at all**, and giving each a
   row reopens a clause the canon closed.
4. **The containment constraint.** A bound on every arm's implementation, of I14's kind.

**So the five rows cut by consumer vocabulary where the canon cuts by parameterisation.** Three of
them are one hole and two inventory entries; one is a duplicate of a ruling carrying an orphaned
exception; one is a frame constraint in the wrong ledger. The count five is an artifact of which
consumer documents were read, which is exactly what `obligation.toml`'s own header says the namespace
is for, and it is why the namespace is the wrong place to answer this question from.

### 4.7 One thing wrong with all five, and with the namespace

**Every quotation cites a branch and none cites a commit.** The rows say "on `dev`". A branch moves.
`tools/the-positions/src/lib.rs:32` states the methodology the same repository already holds itself
to, and the reason it gives is this namespace's own history: an earlier pass "reported that consumer
at zero and called the figure controlled. It was reading a single-branch clone." The tool refuses to
run on an unresolvable ref; I hit that refusal and it is why my tables carry oids. The rows quoting
consumers do not meet the standard the tool beside them enforces.

## 5. What I settled, what I moved, and what I could not

**Settled.**

- The canon licenses the surface and in fact requires it, at
  `ruling::the_operating_constraints_are_intents_and_rules`, `in_force`.
- The canon says nothing about what the primitives are: zero ratified rows on `topic = the_primitive`.
- arvo ships no value layer. Const-asserted at eleven instantiations with two controls and a mutation
  showing the instrument fires (p01).
- The consumer position named by one of the five rows does not compile, with a positive control on the
  same invocation (p02).
- The Debug row's stated reason is false. Refuted by a working no-alloc implementation with two
  controls (p04).
- Debug does not reach the four points, and the reason is the absent value layer rather than no-alloc
  (p04, `E0277`).
- The demand side is 193 positions over four trees at named commits, and 1424 consumer `lint:allow`
  sites currently turn the rule off.

**Moved.**

- `a_primitive_for_every_position_a_bare_number_would_take` from an obligation to a duplicate of a
  ruling whose only unique content, op's const-generic exception, is canon nowhere.
- `a_platform_sized_unsigned_integer_at_an_api_position` from a type request to a mechanism stated
  where a need belongs, with the row's own `why` as the witness.
- `an_exact_width_container_a_consumer_can_alias_and_pin` from a container request to a declared-width
  request the ratified spine already answers.
- `debug_output_from_every_numeral_shape` from a no-alloc problem to a value-layer problem.
- The decomposition from five consumer wants to four groups, two of which are canon work.

**Could not.**

- **How many types the coordinate set is.** Ratified as open by the door ruling on two disagreeing
  derivations. I did not attack it: it is a different question, my dispatch is about values and the
  door is about the declaration, and going there would be sideways rather than down.
- **What the value layer's shape is.** I establish that it is absent and where it belongs. I do not
  propose one. `proposal::a_primitive_is_a_value_set_with_one_realisation_map` stands at `one_expert`
  and is the nearest thing; a second independent derivation is what it wants and manufacturing one
  here, having read it, would be confirmation rather than corroboration.
- **Whether hilavitkutin's 1375 `USize` occurrences are a live need or a fossil.** Both readings fit:
  the consumer wrote them against the deleted tree, and it also still needs an error code carrier.
  Nothing in the corpus separates the two and I could not separate them from source alone. This is the
  one place I would want a different instrument, and I name it in section 6.
- **Whether the 15 foreign-boundary positions are arvo's at all.** The tool reports them as positions
  where "arvo can wrap one of those; it cannot choose it", which is a reading rather than a
  measurement, and I did not check it.

## 6. Options I open, each with what closes it

**O1. Op's const-generic exception is canon nowhere.** It bounds a ratified in-force rule and lives in
an obligation row. *Closed by* a `ruling` row carrying the exception with its verbatim, which needs
only his existing quote and no new judgement, since I14 is already his. It is bookkeeping rather than
a decision, which is why it has stayed missing.

**O2. `debug_output_from_every_numeral_shape` should be rewritten or retired.** Its reason is refuted
and it quantifies over an open set. *Closed by* either a retirement with a replacement stating the
need over the value layer, or an edit to the `why` plus a bound on the strategy quantifier. Which of
the two is a question about the namespace's mechanics, not about the finding: I do not know whether
`obligation` rows may be edited in place, and `obligation::a_build_flag_that_changes_float_semantics`
suggests they are, since it was reworded and records that it was.

**O3. `a_platform_sized_unsigned_integer_at_an_api_position` states a mechanism.** *Closed by* the same
act that closed it on the float-flag row: reword the `need` to the property, leave the slug alone
because it is an address, and add the `gap` saying so. That row's own `gap` is the template and says
in terms why the slug stays.

**O4. The word container in `an_exact_width_container_a_consumer_can_alias_and_pin`.** *Closed by* one
reword to declared width, citing the ratified dissolution.

**O5. `the_unstable_machinery_does_not_reach_a_consumer` may be on the wrong side of the artifact
split.** It is a frame constraint in a demand ledger. *Closed by* two experts agreeing from quoted
canon on whether a consumer-stated constraint on arvo's internals is an obligation or a candidate for
the operating-constraints topic. **I am the first read and a second is owed.** I do not resolve it,
because the canon does not settle it: I14 enumerates five constraints and this is not among them, and
nothing says the enumeration is closed.

**O6. Whether the obligation namespace should carry inventory entries at all.** Under the ratified
open-inventory clause a new numeral joins by a check, so a row per consumer numeral duplicates the
inventory. *Closed by* two experts agreeing on whether "the concept is closed and the inventory is
open" reaches the demand side or only the format contract. **First read, second owed.**

**O7. Obligation quotations cite a branch and not a commit.** *Closed by* resolving each quoted ref
once and recording the oid, which is what the repository's own tool does and refuses to run without.
Sixteen rows, mechanical.

**O8. Is the value layer one crate or several?** Not opened by me as a proposal, only named: the
question exists and I have no instrument for it. *Closed by* a derivation that starts from the
coordinate set the door ruling ratified and asks what a value carrying a member of a declared set has
to hold, rather than from what consumers named.

## 7. Carried forward unchanged, and from whom

Eighteen, none of them re-derived, all cited above at the point of use.

| # | Carried | From |
|---|---|---|
| 1 | `ruling::the_operating_constraints_are_intents_and_rules` (I14) and its five bullets | op, via `85` and `INTENTS.md` |
| 2 | `ruling::the_numeric_door_carries_the_coordinate_set_and_the_two_type_bound_is_not_canon` | seats 238, 239, ratified by experts |
| 3 | `ruling::the_format_spine_is_canon` and its four stamped proposals | op and the experts, `213` |
| 4 | `ruling::behaviour_is_stated_per_declared_signature_and_the_premise_dissolves` | seats 210, 225 |
| 5 | `ruling::arms_over_regions_are_the_fundamental_heart` (I13) | op, `213` |
| 6 | `ruling::warms_objective_is_the_intuitive_best_choice` (I4) | op, `38` |
| 7 | `ruling::never_a_runtime_check_and_one_lowered_path` (I15) | op, `85` |
| 8 | `ruling::ingest_is_the_consumers_and_the_c_abi_is_where_it_ends_up` | op, `88` |
| 9 | `ruling::an_ack_is_not_a_ratification` (I12), as the rule for reading every row above | op, `01` |
| 10 | `ruling::the_intent_is_not_every_clause_of_the_quotation` | op |
| 11 | `ruling::the_strategy_set_is_not_closed_at_four` (I1), for the open-set argument in 4.4 | op, `39` |
| 12 | `question::what_the_numeric_introduction_door_may_carry_out`, its `bound` and its `answered` | the panel |
| 13 | `proposal::the_introduction_doors_bound_is_a_position_rule_not_a_count_of_types` | seat 238, one instance |
| 14 | `proposal::a_primitive_is_a_value_set_with_one_realisation_map` | one expert |
| 15 | `proposal::the_topics_form_a_stack_a_frame_and_the_canons_own_machinery`, for the locus in 3.3 | the coordinator, one instance, weakest tier |
| 16 | `obligation::a_build_flag_that_changes_float_semantics`'s `gap`, as the template for 4.2 and O3 | whoever repaired that row |
| 17 | `tools/the-positions`, its whole method and its 193 | its author |
| 18 | `tools/obligation-coverage`, the five tier readings | its author |

I re-ran 17 and 18 rather than quoting their prior outputs, and 17's refusal on an unresolvable ref is
what made me check the refs at all.

## 8. Coverage

**Read in full.** `mock/registry/ruling.toml` (all 96 rows, ids listed and every row on
`topic = operating_constraints`, `the_format`, `the_container_premise` and `the_strategy_axis` opened);
`mock/registry/obligation.toml` (all 16 rows); `mock/registry/topic.toml`; `mockspace.toml`;
`mock/crates/arvo-format/src/lib.rs`, `width.rs`, and the type declarations of every module in the
three crates; `mock/lints/a_contract_coordinate_is_not_a_host_primitive.rs`;
`mock/tools/the-positions/src/lib.rs` and `supply.rs`; `INTENTS.md` sections I14 and I15.

**Read in part.** `question.toml` (the two rows this bears on, and a keyword sweep);
`proposal.toml` and `proposal-the-later-topics.toml` (the rows on `topic = the_primitive` by query,
plus keyword sweeps for usize, bare, primitive, Debug, unstable, platform); `retirement.toml`
(keyword sweep, plus `r161_r13`); `arvo-format/src/tests/obligations.rs` and
`the_inventory.rs` (headers and the flagged test).

**Ran.** The whole suite, twice, the first run void. `mock obligation-coverage` on all five.
`mock the-positions` over four consumer trees, once for the report and three times for role
breakdowns. `mock rulings-with-no-verbatim` on I14. Four probes, committed.

**Not read, and each could change something above.**

- **`law.toml`, `law-the-later-topics.toml`, `dimension.toml`, `probe.toml`, `strategy.toml`**, other
  than keyword sweeps. A law about a primitive's operations would bear on section 4.6's group 2.
- **The panel corpus.** 460 entries; I opened none of the numbered member files. Specifically not
  `161_leroy_the_canon_candidate_for_the_primitive`, which is the canon candidate for the exact topic
  I locate the hole in, and not `238` or `239`, whose ruling I carry from the registry row rather than
  from their files. **This is the largest hole in my coverage** and it is deliberate: the dispatch is a
  blind derivation and those files are where the answer would have been handed to me.
- **The other reader's file on this question.** Not opened. This is committed first.
- **`arvo-placement` and `arvo-strategy` source** beyond their type declarations and test names.
- **The consumer design documents** other than the four lines of `hilavitkutin-linking` quoted, and
  `kolli`'s `mock/DESIGN.md.tmpl` sentence, which I read through the obligation row rather than in
  situ.
- **tarina.** It has zero rust files at `origin/main`, so it contributes nothing to the position count
  and I did not read its canon, which
  `obligation::the_algebra_is_legible_enough_to_adopt_without_adopting_half_of_it` says reports arvo
  misleading a consumer.
- **Whether the 15 foreign-boundary positions are correctly classified.** The tool says the reading is
  off the identifier rather than measured, and I took it.

## 9. The one sentence

Arvo has a ratified rule forbidding bare primitives at public API positions, no ratified statement of
what may go there instead, and nothing in the tree that holds a number; the five rows describe that
one hole four times in the consumers' vocabulary and once in the wrong ledger, and the canon's own
closed-concept clause says three of the five were never canon questions at all.
