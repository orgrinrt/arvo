# 255. The primitive, established: what the canon can carry now, and what it may not

The establish phase for the primitive, written at `origin/dev` `b34d7a3c` on
`research/the-primitive-surface-establish`, with its instruments in `255_probes/` and their
output committed under `255_probes/out/`.

**One thing about who is writing, before anything else.** The candidate at `161`, the
formalisations at `114` and `160`, and the revision at `164` are all this persona's. So
whatever I say below about whether `161` survives is the author's own reading and is not a
second instance of anything; the brief's item 2 wants a non-Leroy read after this one, and I
say so here rather than letting the file's length stand in for independence. What I can do
honestly is re-derive against the registry as it now stands, open every piece of evidence I
lean on, and write the rows in the schema's own shape so the coordinator has something to
gate rather than something to interpret.

## 0. The two gates

### 0.1 Canon gate: aligned, and the brief's frame corrected before use

Checked against: `ruling::the_panel_finishes_the_canon_without_him` (`ratified`, by op),
`ruling::two_experts_converging_is_a_ratification_and_the_coordinator_holds_the_gate`
(`ratified`, by op), `ruling::an_ack_is_not_a_ratification`, `ruling::the_format_spine_is_canon`
(`ratified`, both), `ruling::behaviour_is_stated_per_declared_signature_and_the_premise_dissolves`
(`ratified`, experts), `ruling::the_numeric_door_carries_the_coordinate_set_and_the_two_type_bound_is_not_canon`
(`ratified`, experts), `ruling::the_operating_constraints_are_intents_and_rules` (`in_force`),
`topic::the_primitive`, the eight `proposal` rows and twelve `retirement` rows on that topic,
`question::what_a_datum_stands_for`, and `mockspace.toml`'s `canon_paths` and its `ruling`
schema block. The work I was sent for is the establish phase the panel-conduct rulings call
for, and it fills no reserved question; section 5 says which those are and leaves them.

**The brief's two verified claims both check** (`255_probes/out/08_the_briefs_claims.txt`):
`ruling.where(topic=the_primitive).count()` is `0`, and the operating-constraints row carries
`rung = in_force`, `key = I14`. The `ruling` namespace holds 96 rows: 32 `ratified`, 1
`in_force`, 58 `stated`, 5 `open`.

**One compression in the brief I decline to inherit, and it shapes everything below.** The
brief presents `109` through `165` and `251` through `252` as one thread that "reopens". They
are two questions wearing one word. `topic::the_primitive` asks what a primitive *is*: its
`what` field reads "a value set with a realisation map over a declared operation set, and
what makes two of them the same primitive". `251` and `252` were dispatched on what the
ratified canon says about a *surface*: which stack-owned types a consumer writes at the
public positions five `obligation` rows describe, and where such types would sit. `252` does
not contain the string `the_primitive` at all (`grep -c` returns 0). `251` names the topic
exactly once as a locus, at its section 3.3, and declines in its own words to derive a shape
for it because it had read the one-expert definition row. Neither file attacks, restates or
cites a single clause of `161`'s statement. So "does `161` survive `251` and `252`" has a
short answer, given in section 3, and the establish work the brief actually needs is two
establishments joined at one seam, which section 1 names.

**The state the work builds on has moved since `251` and `252` were written**, and I
checked what moved rather than inheriting their base. Commit `099acaf5`, merged after both,
landed three closed design rounds, one of them `202609021714_topic.the-primitives-a-consumer-names-instead-of-a-bare-number.md`
on exactly the surface question. Its topic file says locus and count "go through two
independent readings before anything is written against them". Its source changelist
repaired `width.rs:14`, so `252`'s finding 7.1 is closed at this base and should not be
re-reported. And the same merge wrote a locus into the public principles page that its own
topic file says may not be decided yet. That is a design-tier misalignment, not a defect in
my assignment, and it is section 7's first finding rather than a reason to return early:
the design tier is waiting on precisely the canon candidate this file is.

### 0.2 Test gate: run whole, bodies read, not decorative, and one hole named

`cargo test` from `mock/`, nothing filtered, at `b34d7a3c`: `arvo-format` 115 unit, 10
`compile_fail`, 13 `matlab_fi_parity`, 4 doctests run and 5 `compile_fail` doctests;
`arvo-placement` 21; `arvo-strategy` 10. **178 passing, 3 ignored, 0 failing.** The three
ignored are catalogue-reds naming their gap in the `#[ignore]` reason:
`matlab_fi_parity.rs:278` on a MATLAB mode outside the ratified six,
`arvo-placement/src/tests.rs:164` on a converse independence one packing rule cannot show,
and `the_ratio_coordinate.rs:438` on a euclidean carry.

Scans over `mock/crates/`: zero `assert!(true`, zero `assert_eq!` with syntactically
identical operands (a `perl` pattern over every `.rs`, committed nowhere because it returned
nothing and a zero from a pipeline is a claim about the pipeline; the positive control was
`assert_eq!(h, m` in the earlier bench corpus, which 165 already opened). Bodies read:
`arvo-format/src/tests/obligations.rs` in full at its header, whose four open contracts each
carry a construction that compiles and is wrong, kept permanently; `the_identity.rs` at its
`Grid` instrument, which exists because the shipped points pin at least two of the four axes
the cancellation depends on and a law asserted through them alone would be measured at one
point of three axes. That is a suite that knows what its own sample cannot see, and I have
nothing to refuse on.

**The hole `252` named is still there and I add one beside it.** Nothing in the suite
asserts the position rule, and nothing asserts what the public principles page tells a
consumer to write. The second is now `255_probes/p02_the_principles_locus/`, a probe rather
than a test, because a test in `mock/crates/` needs a round in a tree another agent is
editing and a lint on the principles page belongs in `mock/lints/`, which I was told not to
touch. It is named as owed in section 7.

`timeout` was not used anywhere, and every command whose result I believe had its stderr
folded in.

## 1. Two questions, one seam

**The primitive** (topic five, `109` through `165`) is the concept: what a primitive is,
when two are the same, what the type must carry, what the type owes the denotation, what a
refinement is, what composition is. Its candidate is `161`, revised at `164`, checked at
`165`, and ported into the registry as eight `proposal` rows by whoever ran the port (their
provenance cites `161` and `164` by line).

**The primitive surface** (`251`, `252`) is the supply side of I14's fifth bullet: the rule
that public API positions carry the stack's own primitives is `in_force`, five consumer
requests describe positions that need one, and both files were asked what the ratified rows
say about that and where it sits.

**The seam.** Both files establish, by different instruments, that arvo ships nothing that
holds a number: `251` const-asserts all four shipped format points zero-sized at eleven
instantiations with two controls (`251_probes/p01`, rebuilt by me at this base,
`out/01_p01_rebuild.txt`, exit 0), and `252` reads the same fact off the derive census
("the numeral layer has nothing to cover yet, there being no numeral"). What is missing is
exactly what `161`'s clause 5 describes at its degenerate point: a value whose lens focus is
the sole occupant of its allocation, which "the language supplies a standalone type" for.
`252`'s word for the tier is *numerals*, "values of a declared format"; `251`'s is *the value
layer*. **The concept has a candidate with zero ratified rows; the surface has a ratified rule
demanding what the concept's unratified clause describes.** Establishing the one without the
other leaves the design tier where it is now, which is inventing a locus in a principles
table.

## 2. The agreement ledger: `251` against `252`

### 2.1 What each instrument actually varied

A convergence is over the intersection of what two instruments reached, not over the names
of their dimensions, so the table comes before the items.

| dimension | `251` | `252` | intersection |
|---|---|---|---|
| `ruling.toml` | all 96 rows, four topics opened in full | all 96, every row touching seven keyword areas opened | the whole namespace, both |
| `obligation.toml` | all 16 | all 16 | both |
| `topic.toml` | in full | all 20 | both |
| `question.toml` | two rows plus keyword sweep | every `id` listed, four opened | `252` wider; the reserved-call finding lives only where `252` reached |
| `proposal*.toml` | the primitive-topic rows by query plus sweeps | ids by vocabulary, one opened | `251` wider on the primitive topic |
| `retirement.toml` | sweep plus `r161_r13` | three opened | neither in full |
| `law*`, `dimension`, `probe`, `strategy` | not read | not read | **absent in both** |
| the panel corpus | none | none, then `235` in an appendix after the blind commit | **absent in both**, including `161` |
| shipped source | type declarations of all three crates, `lib.rs`, `width.rs`, one lint, one tool | `lib.rs` of all three, `width.rs`, `DESIGN.md.tmpl` door section, the ui test | both read the declarations; only `251` asserted their size |
| consumer trees | four, at named oids, via `the-positions`: 193 positions | none; every consumer sentence taken from a row's `quote` | **`251` only** |
| the shipped tree via `the-positions` | run | run: 27 positions, all one crate | both, different corpora |
| compile probes | zero-size of points; the consumer's `USize` field; `Debug` into a caller buffer | containment of `generic_const_exprs` and `adt_const_params` across four shapes | disjoint |
| `mockspace.toml` | the `[primitive-introductions]` table | the table and the layer-hierarchy prose above it | `252` wider |

So the two agree over the ruling and obligation namespaces read whole, and over the shipped
declarations. Anything below that rests on one file's instrument is stated as one instance.

### 2.2 The items

Form per item: **separately** (both derived it, blind), **one alone** (only one file reached
it), **inherited** (one took it from a row or a rule both read), **contested**.

**A1. The rule exists, is `in_force`, and is the only canon statement of the demand.**
Separately. Both from the same row, `ruling::the_operating_constraints_are_intents_and_rules`,
and both note it is the sole `ruling` row containing `usize`. `252` adds a qualification
`251` does not: op's `quote` on that row does not contain the primitives clause, which
reaches the `says` from the list he confirmed wholesale. I checked: the quote reads "No std,
no alloc, all that is explicitly already in place" and the fifth bullet is in `INTENTS.md`'s
list under it. The row stands as it is, per `ruling::the_intent_is_not_every_clause_of_the_quotation`,
and `mock rulings-with-no-verbatim` is the instrument neither ran against it; nor did I.

**A2. `in_force` binds.** Separately, and against `235`, which read it as `stated`. `252`
opens the schema's own `rung` declaration; `251` calls the rule "in force" throughout. The
schema says `in_force` is "where the workspace and this repo's own lints enforce it
independently of convergence", which is a different claim from `stated` and a stronger one.
Both files are right and `235` is wrong, and `252` names why: the generated agent
instructions list `ratified` and `stated` and stop (section 7, F4, re-measured).

**A3. The canon says nothing about what the primitives are.** `251` alone, from
`ruling.where(topic=the_primitive)` returning zero. `252` never queried the topic. Not
contested; unreached by one side.

**A4. The coordinate set is ratified and is not what the five rows ask for.** Separately.
`251`: "the door is about the declaration, and all five rows are about values". `252`: the
coordinate set is "the one kind the canon has actually ratified" and the five rows miss it.
Same fact, opposite emphasis, both from the door ruling.

**A5. arvo ships no type that holds a number.** `251` by instrument (p01, with a mutation
shown to fire), `252` by census. Two instances on the fact; one on the measurement.

**A6. The five rows are not a decomposition, and the cut runs by kind rather than by
consumer.** Separately, and this is the item most worth a row. `251` section 4.6 gives four
groups: the rule, the value layer, inventory entries, the containment constraint. `252`
section 5.5 gives five kinds: coordinate types, numerals, placement facts, position rules,
build properties. Intersect over the kinds each actually derived, not over the word "cut":
**position rule** (both), **numeral or value layer** (both), **build property** (both).
`252` alone adds coordinates and placement facts, each governed by a ratified row neither
file disputes. `251` alone adds that inventory entries are not canon work under the
closed-concept clause; `252` says the same of the numeral tier ("which primitives exist is
the wrong question about this tier by construction"), so that one is separately reached
under different words. The design round at `099acaf5` says the five "are one feature", which
is the same conclusion, and is inherited: it landed after both files.

**A7. Row one is a rule quantified over positions, not a type, and its `gap` is stale.**
Separately. Both ran `the-positions`; `251` over four consumer trees (193 positions, 15 on a
foreign boundary, 1424 `lint:allow` sites), `252` over the shipped tree (27 positions, all
constructor or accessor of the introducing crate's own types). I re-ran the second at
`b34d7a3c`: 27, fn-param 14, fn-return 13, scalar 27 (`out/04_the_positions.txt`).
Different corpora, so these are two facts rather than two instances of one, and both make
"nothing has enumerated those" false.

**A8. Row two states a mechanism its own `why` disowns.** Separately, from the row alone,
and both name `obligation::a_build_flag_that_changes_float_semantics`'s `gap` as the
precedent. That two blind readers reached for the same precedent is what makes this the
cleanest convergence in the pair.

**A9. Row two's mechanism is in tension with the ratified canon, and which row it violates is
not agreed.** Contested in the interesting sense: `252` reaches the tension through the
dissolution ruling's "never of the machine carrier"; `235`, which `252` reconciles against,
reaches it through I14's "no platform dependency" and through the storage clause of the
ratified format-identity proposal; `251` reaches no tension at all and says in terms that
the obvious read of the platform-dependency bullet is wrong, because I14 glosses it as
`std::thread`, `std::time`, `std::fs`, `std::net`. I opened `INTENTS.md` I14: `251` is right
about the gloss. So the platform-dependency route is closed and the carrier route stands at
one instance. **Intersection: a tension exists with one ratified row; the row is
`behaviour_is_stated_per_declared_signature_and_the_premise_dissolves`; and it narrows
`question::what_a_platform_width_type_is` without closing it.**

**A10. Row three's word "container" is on the wrong side of a ratified dissolution, and the
need survives the rewording.** Separately. Both cite the dissolution and the ingest ruling.
**Where they part**: `252` says the 28-bit signature "is a format, expressible today as a
point of the shipped parameterisation" and the row "needs no new concept"; `251` says the
value layer is absent and row three is one of the three rows that are that one hole. By
instrument `251` is right: a point is zero-sized (A5, rebuilt), and a consumer holding a
hash needs to hold it. `252` did not run the size and does not contradict it. **Resolved by
the measurement, not by the argument: row three needs the numeral too.**

**A11. Row four's stated reason is false.** `251` alone, by a working `#![no_std]`
implementation writing `Debug` into a fixed buffer with two controls (`251_probes/p04`).
`252` reaches half of it: eighteen `derive` attributes all include `Debug`, so the
coordinate layer prints, the marker layer does not, and the numeral layer has nothing to
print. `252` does not say the no-alloc reason is false; it says the caller-buffer half "is
met by none of it and is addressed nowhere". Not contested; `251` went further and built it.

**A12. Row five is the one that is right, and it is a property of the implementation.**
Separately on the kind. Then disjoint: `251` says it is misfiled, a frame constraint of
I14's shape sitting in the demand ledger; `252` measured what the row calls unmeasured, that
containment of `generic_const_exprs` is position-dependent, a `where` bound contained and a
return-type expression not, and that `adt_const_params` is contained at the declaring crate
(`252_probes/`, opened: seven output files, the README's table matches them). Both are one
instance each and neither disputes the other.

**A13. Op's const-generic exception is canon nowhere.** Separately. `251` O1: it bounds an
in-force rule and lives one tier below it, in `obligation::a_primitive_for_every_position_a_bare_number_would_take`.
`252` 5.3: it is load-bearing rather than ergonomic, because at the const generic parameter
position a stack-owned type demands a gate in the declaring crate, so the exception is what
makes row five satisfiable, and the connection was already in the `bound` of
`question::what_the_numeric_introduction_door_may_carry_out`. I traced the quote to its
source: kolli's `mock/design_rounds/202607261745/202607270230_topic.arvo-everywhere-no-bare-usize.md`
at `origin/dev`, under a blockquote headed "Op:", and it is quoted in this panel at `200`.
Two instances that it belongs at the ruling tier; the row is in section 4.

**A14. Silence on debug output and on unstable containment.** Separately, with different
greps and positive controls, and both over the governing tier. I re-ran it per file
(`out/03_silence.txt`): over `ruling.toml` alone every pattern returns zero and `usize`
returns one; over the whole registry `nightly` returns 26, `feature gate` 12,
`generic_const_exprs` 9, in `dimension`, `law-the-later-topics`, `obligation`, `probe`,
`proposal` and `retirement`. **So the honest predicate is: silent at the `ruling` tier;
named at the demand, probe and proposal tiers; and never about output**, since the two
`fmt` hits are the Debug obligation's own `need` and `keywords`.

**A15. Reserved against silent, for the platform-width type.** Contested in effect. `251`
files the platform-sized numeral as "silent at the ruling tier"; `252` files it as
**reserved**, governed by `question::what_a_platform_width_type_is`, `decider = panel`,
`answered` empty. Both statements are true over the tier each measured, and they license
opposite things: silence lets a design derive inside the intent with two agreements, a
reservation does not. `question` rows are canon under `canon_paths`, so `252`'s predicate is
the one that reaches, and it governs. Re-queried at this base: unanswered
(`out/05_questions.txt`). The same holds for `question::the_width_surface_crossing`, which
`252` reads as governing the alias half of row three and `251` did not open.

**A16. The canon fixes no crate.** `252` alone, by grep over `ruling.toml` for "layer" and
"crate", finding one architectural boundary and no layout. `251` asserts a locus, "a crate
above `arvo-format` and `arvo-placement` that does not exist", citing the coordinator's
one-expert topic-layering proposal as its weakest-tier corroboration. These do not conflict:
`251` says where the design should put it, `252` says the canon does not say. Section 6.

### 2.3 What neither reached

`law.toml`, `law-the-later-topics.toml`, `dimension.toml`, `probe.toml`, `strategy.toml`;
the panel corpus, `161` above all; any consumer design document at source; whether
hilavitkutin's 1375 `USize` occurrences are a live need or a fossil of the deleted tree;
whether the fifteen foreign-boundary positions are arvo's; and the count of coordinate
types, which both correctly left where the door ruling reserved it. A finding in `probe.toml`
about a primitive would be invisible to both files, and I did not read it either; section 11.

### 2.4 What the pair establishes at two instances

Six things, and only these: A1, A2, A4, A6 over its three-kind intersection, A8, A13, A14
over the `ruling` tier. Everything else in `251` and `252` is one instance and is cited below
as such.

## 3. What happened to the candidate at `161`

### 3.1 It was not refuted at `165`

`165` is an independent check by a member who took no part in the unit. Its verdict, quoted:
"The revision is sound where it repairs the two signatures' findings." It rebuilt the lens
degeneracy probe, the two-branch certificate, the `cfg` soundness hole and the container
premise sweep byte-for-byte, and reran the thirteen-clause conditionality sweep independently
and matched `164`'s table exactly. Its one severe finding is a miscount in `157` F157-5 that
`164`'s R17 inherited, "three of six" written as "four of six"; the candidate's own text never
carries the number, and the registry row `retirement::r164_r17_rounding_at_zero_fraction_is_observable`
now reads "three separate and three do not", so the correction landed. **Not refuted.**

### 3.2 It is neither superseded nor refuted by `251` and `252`

Section 1 already says why: neither file cites a clause of it. The nearest either comes is
`251` section 5 naming `proposal::a_primitive_is_a_value_set_with_one_realisation_map` as
"the nearest thing" at one expert and declining to manufacture a second derivation having
read it, which is the correct posture and is not an attack. What `251` and `252` add to the
topic is the demand: a ratified rule with no supply, and four consumer requests that are one
hole, which is what the topic's clause 5 describes at its degenerate point. **That is
pressure on the topic to be established, not evidence against what it says.**

### 3.3 What did move it, all of it after `165`

Three ratified or answered things, none of which existed when `161` and `165` were written,
and each of which touches `161`'s section 6, "what only op decides":

- **`156` item 1, the container premise, is answered**, by
  `ruling::behaviour_is_stated_per_declared_signature_and_the_premise_dissolves`, ratified
  on two independent instances. The premise was malformed as a binary and both branches are
  false. `question::the_container_premise`'s `answered` carries it. `161` section 6 item 1
  is void; so is the "conditional on `156` item 1" marker on clause 4 and the disjunctive
  forms `164` put on clauses 2, 6, 9 and 10.
- **Q65 is answered**, by op, on 2026-08-31, as
  `ruling::a_proof_and_a_bounded_range_get_markers_the_notation_lacked`. `161` section 6
  item 2 is void.
- **Ratification is the panel's**, per `ruling::the_panel_finishes_the_canon_without_him`.
  `161` section 6 item 3 is void.

And two one-expert repairs from `210`, landed as proposal rows on the topic:

- `proposal::the_realisation_is_not_part_of_denotational_identity` restores clause 6's first
  sentence with one word added. The carrier is a coordinate of representational sameness,
  which clause 2 already assigned memory reinterpretation to, so the "footprint-observable
  branch" was a statement about the neighbouring relation and never a rival reading of the
  sentence. The row `supersedes` the omission in
  `an_axis_the_realisation_map_does_not_read_is_not_a_type_parameter`, whose second and
  third parts stand.
- `proposal::a_completeness_obligation_quantifying_over_inputs_misses_a_nullary_witness`
  refuses clause 9's wording, independently of the premise: a witness there is an input and
  `size_of` takes none, so the clause refuses a pair a consumer separates in one call. The
  `instead` states the obligation over contexts in the declared signature including the
  ambient observations the host supplies; the G1 control shows the genuinely spurious pair
  stays refused under the wider set.

`AGREEMENTS.md` section 12 still says "the container premise is op's" and "Q65's marker
question is his too". Both sentences are stale, and section 7 lists it.

### 3.4 The verdict, and the corrected statement

**Kept, and corrected in four clauses.** `161` named the parts well and went wrong only in
detail, and the detail it went wrong in was carrying a premise as a fork that a ratified
ruling later dissolved. Rewrite cost is real and the tiebreaker, and here there is nothing on
the other side of it: no file in the corpus proposes a different account of the primitive.
The eight registry rows already carry the statement; what changes is the marker text on
four of them and the stale section 6. The clauses that move, written out in full because a
delta is not a candidate:

> **2.** Its **identity** is that structure up to denotation-preserving isomorphism,
> relative to the declared operation set. Of the three sameness relations, nominal,
> representational, denotational, each licensing a different operation (assignment, memory
> reinterpretation, rewriting), only the denotational one is a congruence under composition,
> which is why it and only it licenses substitution inside a composite. The clause is
> parametric in the operation set; its extension at any signature is whatever that signature
> separates, and the two figures the corpus measured are two parameter values, both correct.

> **4.** The **signature is part of the definition**: how many primitives exist is not well
> posed until the operation set is fixed, and the unit of definition is a family closed under
> the operations rather than one carrier alone. The identity a signature induces is
> determined by the **reach** of its terms into the realisation map's domain; it is monotone
> in that reach, saturates when the reach is the whole domain, and a full literal reaches
> saturation at depth one, so a design that can write a literal needs no closed operation set
> for stable identity. This holds over signatures whose operations are functions of the value
> set and the realisation map. Every operation arvo declares is such a function, stated over
> the declared width and never over the machine carrier; the host's layout observation is
> ambient rather than declared, is not in the signature, and so identity saturates at the
> declared signature set. The conditional this clause used to carry is discharged.

> **6.** The realisation is **not part of denotational identity and is emphatically part of
> the surface**: the carrier is a coordinate of representational sameness, licensing memory
> reinterpretation and nothing about substitution inside a composite, and a consumer may ask
> for the storage-minimising placement, with denotational sameness licensing the substitution
> underneath that choice. An axis the realisation map does not read **must not** be a type
> parameter; an axis the arm selection reads **may** be one, because weakening repairs it and
> weakening is free. The cost of two names for one primitive is a property of where the
> spellings meet: nothing at a monomorphic site, one threaded parameter at a polymorphic
> signature, and no repair at a homogeneous container, which is why a spurious parameter's
> whole cost lands on the storage path this design protects.

> **9.** The type owes the denotation **adequacy**, and adequacy is two obligations of
> different kinds plus an order. **Soundness**: the denotation factors through what the type
> carries, over every build; it is structural, needs no enumeration, and is not enforceable
> by a signature nor by anything that inspects one build, so its residual obligation is a
> restriction on what the realisation-map call path may read, checkable as a property of a
> call graph. **Completeness, up to weakening**: every pair of distinct shipped
> instantiations is either separated by some context in the declared signature, the
> signature including the ambient observations the host language supplies and cannot
> withhold, or connected by a weakening in exactly one direction; a pair with neither is a
> spurious split and is refused. A nullary observation is a context and needs no adding. The
> obligation is **per pair of shipped instantiations**, not per axis, because an axis can be
> read at some instantiations and not at others. The axis classification, two directions
> spurious, one refinement, zero declared semantics, is this same obligation stated per axis.

Clause 10's phrase reads "the realisation map's whole domain" without a branch note, which
was the internal reading and is the one that stands. Clauses 1, 3, 5, 7, 8, 11, 12 and 13
are unchanged from `161` section 4 and are not restated.

**Every one of those four rewrites is my reading of how a ratified ruling and two one-expert
rows land on my own text.** Clause 4 and clause 6 follow the ruling's `says` and `210`'s
argument nearly word for word. Clause 9's wording adopts `210`'s `instead`, which is one
expert. None of this is promoted below, and the reason is in the next subsection.

### 3.5 The registry's standings are right, and they are lower than `161`'s rungs

`161`'s ledger used four rungs, one of which was CONVERGED: attacked, replied, author agreed.
The `proposal` schema's `standing` uses a stricter test, "each deriving before reading the
other", and under it three of the topic's rows that `161` called CONVERGED are `one_expert`:
the definition itself, the lens with its sole-occupancy criterion, and the type-parameter
refusal. The port was right to demote them. A reply that agrees after reading is inheritance
however good the argument, and the schema's own text says no quality of argument makes it
count. So what the topic holds at two instances or better is exactly three rows:

| row | standing | the instances |
|---|---|---|
| `the_realisation_map_is_one_map_with_two_regions` | `three_or_more` | `110` section 2 blind; `63` C1 from two cold arrivals; `112` F112-4 on a separately built licence-side instrument |
| `configuration_is_not_composition_and_a_composite_is_a_primitive` | `two_experts` | `110` section 4 blind; `154` section 7 blind in a later sitting from a different instrument set |
| `membership_in_the_type_and_identity_are_two_criteria` | `two_experts` | `109` section 11 and `110` section 3, the split stated independently in both phase twos |

Those three are what section 4 promotes. The definition row stays at one expert, and **I am
not a second instance of it**: I composed its wording at `112`'s suggestion and carried it
through three files. The lens row stays at one expert for the same reason. A non-Leroy blind
derivation of the definition is the single most valuable dispatch left on this topic, and it
is what would let the keystone be promoted with the three satellites already standing.

### 3.6 One seam I flag and do not close

The dissolution ruling says identity "saturates at the declared signature set" and that the
footprint is observable only through an observation "the design does not own". `210`'s
clause 9 repair widens the completeness witness set to include exactly that observation. Read
together they are consistent, and the consistent reading is this: **denotational identity is
over the declared signature, which excludes the layout observation; adequacy's completeness
is over every context a consumer can write, which includes it wherever it exists, that is at
sole occupancy.** So two shipped instantiations differing only in carrier are one primitive
and two types, separated at sole occupancy by `size_of` and by nothing at a shared
placement. That is one expert's reading of two texts, mine, and it is the sentence a design
will want first. It is recorded as O2 in section 10 rather than promoted.

## 4. The ruling rows

### 4.1 Why five and not more

Under `ruling::two_experts_converging_is_a_ratification_and_the_coordinator_holds_the_gate`
the experts propose and the coordinator judges whether "the reasoning, the evidence and the
stated region are enough". Three proposals on the topic meet the convergence bar (3.5). Two
things about the surface stand at two blind instances and belong at the ruling tier (A13,
A6). That is five rows. Everything else on either question is one instance, reserved, or
silent, and a row written on any of it would be the drift this panel exists to refuse.

**These are candidate rows, written in the schema so they can be admitted or refused as
written.** I have not edited `mock/registry/`: `canon_paths` refuses it while the panel is
open, the coordinator holds the gate, and on the two rows that make a call about what the
canon permits, R4 and R5, I am the first read and a second is owed.

### 4.2 The rows

Topic `the_primitive`, three promotions. The `promotion` field is written in the door
ruling's shape and is the coordinator's to adopt or rewrite.

```toml
[[ruling]]
id = "the_realisation_map_is_one_map_with_two_regions_is_canon"
kind = "ruling"
rung = "ratified"
ratified_by = "experts"
promotion = """
Three derivations and a fourth instrument. `110` section 2 reached it blind in the first \
sitting; `63` C1 states it earlier from two cold arrivals, `55` and `60`; and `112` F112-4 \
built the licence-side instrument that distinguishes the claim from its rival: a magnitude \
bound switches off the completion and not the rounding, a grid bound the reverse, which is \
what two regions of one map predict and what two independent axes do not. The proposal \
carries `three_or_more`, the strongest standing on its topic, and no file in either sitting \
attacked it.

The intersection is the map's structure and nothing else. None of the instances states a \
width, and the proposal is `normative`, a definition rather than a claim about behaviour at \
a width, so no region is carried and none is owed.
"""
topic = "the_primitive"
says = "The realisation map that takes an exact result back into a primitive's value set is one map with two regions: rounding between grid points, and completion outside the representable range. They are never two mechanisms."
because = "A bound on magnitude switches off the completion and not the rounding, and a bound on the grid does the reverse, which is what two regions of one map predict and two independent axes would not; and three derivations that did not read each other arrived at the same structure."
ratifies = ["the_realisation_map_is_one_map_with_two_regions"]
note = "The definition this map sits inside, that a primitive is a value set together with one such map over a declared operation set, stands at one expert and is not promoted here. Promoting a clause of a definition ahead of the definition is deliberate: the clause has three instances and the definition's composed wording has one, and the schema's test is arrivals rather than argument."
provenance = [
  "panel::202608072330_the-numeral-canon-panel::161_leroy_the_canon_candidate_for_the_primitive::#1-1-the-denotation",
  "panel::202608072330_the-numeral-canon-panel::255_leroy_the_primitive_established::#4-2-the-rows",
]
keywords = ["realisation map", "two regions", "rounding", "completion", "one map", "grid", "range", "primitive"]

[[ruling]]
id = "configuration_is_not_composition_is_canon"
kind = "ruling"
rung = "ratified"
ratified_by = "experts"
promotion = """
Two blind arrivals from different instrument sets. `110` section 4 and its F10 reached the \
distinction and the closure in the first sitting; `154` section 7 reached the same two \
things in the ninth unit from footprint and container-relativity probes, before reading \
`110`, and `157` section 1.3 classified it as a genuine convergence with the shared bench \
corpus named rather than hidden. The transformer half, that borrowing the base's rule for a \
construction's predicate is unsound where borrowing it for equality is not, is measured by \
`110` F12, `112` F112-13 and F112-14, and `110` R6's p11 on a separately built instrument \
reproducing `112`'s three figures.

What is promoted is the distinction, the closure and the two-things-of-its-own clause. The \
fibration frame `154` offered beyond them is one expert's and is not in the proposal.
"""
topic = "the_primitive"
says = "Configuration is not composition. Choosing a format, a system or a strategy fills in a record; composition is a construction taking an algebra to an algebra; and a composite is a primitive under the same definition, so one concept serves and every contract written for a primitive applies to a composite unchanged. A construction carries two things of its own, a predicate on its base and a transformer for its base's refinements, and equality transports through a construction for free where a predicate never does."
because = "Borrowing the base's rule for a construction's predicate is unsound where borrowing it for equality is not, which is what makes the two halves different obligations rather than one; and the distinction was reached blind twice from different instruments."
ratifies = ["configuration_is_not_composition_and_a_composite_is_a_primitive"]
obligation = ["composition_contracts_above_the_numeral"]
provenance = [
  "panel::202608072330_the-numeral-canon-panel::161_leroy_the_canon_candidate_for_the_primitive::#1-5-naming-cost-composition-chains",
  "panel::202608072330_the-numeral-canon-panel::255_leroy_the_primitive_established::#4-2-the-rows",
]
keywords = ["configuration", "composition", "construction", "composite", "algebra", "transformer", "predicate on the base", "equality transports"]

[[ruling]]
id = "membership_and_identity_are_two_criteria_is_canon"
kind = "ruling"
rung = "ratified"
ratified_by = "experts"
promotion = """
Two instances, both first-sitting, the split stated independently in each member's phase \
two: `109` section 11 from the membership side, grounding const-availability in the scope \
op's own instruction gives, and `110` section 3 from the identity side, with the two phase \
twos each naming the split without having read the other's. The row was split from the \
const-availability entailment precisely so this standing would be visible: the entailment, \
that a parameter left runtime forces the check the design forbids, is `154`'s alone and is \
not in this row.

Normative, so no region: it is a criterion the design is told to apply.
"""
topic = "the_primitive"
says = "Membership in the type and identity of the primitive are two criteria answering two questions, and a design needs both. What must be const-available decides membership; what is preserved up to denotation-preserving isomorphism decides identity; and neither answers the other's question."
because = "An axis can be const-available and carry no denotational content, and an axis can be identity-bearing and be decidable at runtime, so a design collapsing the two either puts something in the type that nothing reads or leaves out something that changes the answer."
ratifies = ["membership_in_the_type_and_identity_are_two_criteria"]
provenance = [
  "panel::202608072330_the-numeral-canon-panel::161_leroy_the_canon_candidate_for_the_primitive::#1-3-adequacy",
  "panel::202608072330_the-numeral-canon-panel::255_leroy_the_primitive_established::#4-2-the-rows",
]
keywords = ["membership", "identity", "two criteria", "const-available", "denotation", "isomorphism", "type"]
```

Topic `operating_constraints`, one row carrying op's words and one promotion.

```toml
[[ruling]]
id = "the_const_generic_parameter_is_the_one_excepted_position"
kind = "intent"
rung = "in_force"
topic = "operating_constraints"
says = "One position is excepted from the rule that public API positions carry the stack's own primitives: the type of a const generic parameter may be a bare host integer, for a smoother and more ergonomic API, and even there only where the alternative is genuinely painful."
quote = '''
We want arvo there. No bare usize other than in const generics for smoother and more ergonomic api, and
even there, only when truly painful otherwise.
'''
because = "His own reason is ergonomics at the instantiation site, twice bounded: the position, and inside the position only where the ceremony would otherwise show up at every call."
note = """
Said about `kolli-api`, in a design round of that repository, on whether the contracts \
crate keeps the bare `usize` it shipped with. The generalisation from one crate to the \
stack is the workspace rule's rather than a sentence of his; the workspace type-surface \
rule carries it as the one excepted position and the lints pass it, which is why the rung \
is `in_force` rather than `stated`.

The panel has a second reason he did not give, and it is recorded here as the panel's: at \
the const generic parameter position a stack-owned type demands `adt_const_params` in the \
declaring crate, measured by the shipped ui test and by `252` arm B, so the exception is \
also what lets a consumer declare a format with no feature gate of its own. That reason \
does not widen the exception; it explains why it is load-bearing rather than cosmetic.

Until this row existed the exception lived only in the `need` of \
`obligation::a_primitive_for_every_position_a_bare_number_would_take`, one tier below the \
rule it bounds. Two blind readers found it there independently.
"""
provenance = [
  "panel::202608072330_the-numeral-canon-panel::200_mcsherry_re_deriving_the_demand_side::#5-two-consumers-were-never-read-and-both-say-things",
  "panel::202608072330_the-numeral-canon-panel::251_the_primitive_surface_is_a_rule_with_no_supply::#2-5-silence-stated-plainly",
  "panel::202608072330_the-numeral-canon-panel::252_kiselyov_the_primitive_surface_locus::#5-3-rows-one-and-five-are-one-constraint-seen-from-two-sides-and-neither-row-knows-it",
]
keywords = ["const generic", "exception", "bare usize", "position", "ergonomic", "painful", "I14", "adt_const_params"]

[[ruling]]
id = "the_primitive_surface_is_cut_by_kind_and_the_demand_rows_are_a_sample"
kind = "ruling"
rung = "ratified"
ratified_by = "experts"
promotion = """
Two readings, blind and separately dispatched, committed before either opened the other. \
`251` cut the five rows into four groups from the rulings and from four consumer trees at \
named oids; `252` cut them into five kinds from the rulings and the shipped tree. The tier \
is over the intersection of the kinds each derived: a rule over positions, a value of a \
declared format, and a property of the build. `252` alone names the coordinate set and \
placement facts as further kinds, each governed by a ratified row the other file cites for \
other purposes and does not dispute. `251` alone measured that the shipped format points \
hold no value, at eleven instantiations with a mutation shown to fire, rebuilt at this base; \
`252` reached the same absence by census and did not run the size.

Both reached, from the obligation namespace's own header, that the five rows claim no \
completeness and no partition, so reading them as a decomposition is a category error; the \
design round that landed after them calls the five one feature, which is the same \
conclusion inherited rather than a third instance.

Region: the five rows as they stand at `b34d7a3c`. A sixth consumer request lands in one of \
the kinds or names a kind this row lacks, and either is a finding rather than a refutation.
"""
topic = "operating_constraints"
says = "The consumer requests describing the primitive surface are a demand-side sample of where a stack-owned type is wanted, and never a decomposition of what to build. The surface is cut by kind of thing, along the axis the ratified rows already run: a rule over positions, which is in force; the coordinate set a declaration is written in, which is ratified; the numerals that are values of a declared format; the placement of those values, which is ratified; and properties of the build, on which the canon is silent. The kind no ratified row states and no shipped type holds is the numeral, and it is one thing rather than one per request."
because = "The obligation namespace is read from outside the canon on purpose and says of itself that an absence there means nobody enumerated it; a cut derived from it inherits whichever consumers happened to be read, and the axis the canon is organised along is invisible from there. Two blind readers reached the same axis from the rulings."
note = "Where the numeral sits across crates is not fixed by this row or by any other: the canon fixes one boundary, between what is stated over the declared signature and what is a placement fact, and no crate. Which kind a platform-width value is remains reserved to `question::what_a_platform_width_type_is`, and the crossing between a written width and its type-level form to `question::the_width_surface_crossing`; this row names the kinds and does not assign either of those to one."
provenance = [
  "panel::202608072330_the-numeral-canon-panel::251_the_primitive_surface_is_a_rule_with_no_supply::#4-6-the-cut-the-canon-implies",
  "panel::202608072330_the-numeral-canon-panel::252_kiselyov_the_primitive_surface_locus::#5-5-the-cut-the-canon-implies",
  "panel::202608072330_the-numeral-canon-panel::255_leroy_the_primitive_established::#2-2-the-items",
]
keywords = ["primitive surface", "decomposition", "kind", "numeral", "value layer", "coordinate set", "placement", "position rule", "build property", "obligation", "sample"]
```

### 4.3 The rows I refuse to write, each with why

- **A definition-of-primitive ruling.** `proposal::a_primitive_is_a_value_set_with_one_realisation_map`
  is `one_expert` under the schema's test, its wording is mine, and no second blind
  derivation exists. Writing it would be one persona ratifying itself.
- **A lens ruling.** `proposal::the_lens_degenerates_to_an_ordinary_value_at_sole_occupancy`
  is `one_expert`: the third failure direction was found by an instrument built after
  reading the first. It is the clause the surface most needs, and needing it is not a
  standing.
- **A ruling naming a locus for the numeral.** Both files agree the canon fixes no crate;
  the canon-is-intent rule says a crate name is design-tier; section 6.
- **Anything answering Q26, Q9, Q4 or Q28.** Reserved; section 5.
- **A debug-output ruling or a containment ruling.** Silent at the governing tier; a design
  derives these inside the intent with two agreements, and a ruling written now would be a
  panellist legislating.
- **A row restating I14's fifth bullet with the exception folded in.** That is the duplication
  `251` 4.1 named as the defect in obligation row one, moved up a tier. R4 bounds the
  existing row and cites it; it does not restate it.
- **A ruling closing the ingest row's blocked promotion.** `question::what_the_ingest_row_claims_and_what_turns_on_it`
  exists for it and is unanswered.

### 4.4 The gate, stated

R1 to R3 rest on instances already in the registry; what the coordinator judges is whether
the promotion paragraphs state the evidence and region well enough. R4 and R5 each make a
call about what the canon permits, that a consumer-repository quote of op's belongs at the
ruling tier, and that a reading rule over the demand side is canon rather than bookkeeping.
**On both I am the first read and a second is owed**, from a persona that has not written on
either question: not Kiselyov, who wrote `252`, `154`, `238` and `235`; not whoever wrote
`251`, whose seat is unnamed in the panel's handles.

## 5. Reserved, silent, and answered since `161`

### 5.1 Reserved: the canon holds the call open, and a design may not fill it

Each is a `question` row with `decider = panel` and an empty `answered`, re-queried at this
base (`out/05_questions.txt`), or a ratified ruling refusing in its own words.

- **`question::what_a_platform_width_type_is`** (Q26). Governs
  `obligation::a_platform_sized_unsigned_integer_at_an_api_position` and nothing else does.
  Narrowed by A9 to a hinge on one phrase of `ruling::the_format_spine_is_canon`, "a constant
  of the type", read per compilation or per declaration; not closed.
- **`question::the_width_surface_crossing`** (Q9). Governs the alias-and-pin half of
  `obligation::an_exact_width_container_a_consumer_can_alias_and_pin`; its own `bound` quotes
  the position obligation as governing.
- **`question::what_a_datum_stands_for`** (Q4), on `topic::the_primitive` itself. Op refused
  to bound its option set. `161`'s clause 1 says "value set" and is parametric in what a
  member of that set is, so it does not decide Q4; a promotion of the definition row would
  have to say so in its note, which is one more reason it is not promoted here.
- **`question::arbitrary_width_demands_in_the_canon`** (Q28), adjacent, on `canon_form`.
- **`question::what_transfers_from_a_model_width`** (Q53). Every enumerative claim under
  `161` is at model widths and carries no transfer argument; `161` section 5 item 1 said so
  and it is still true.
- **The count of coordinate types**, reserved by the door ruling in its `says`: "How many
  types that is, this ruling does not say." Not a question row; a ruling refusing.
- **`question::what_the_ingest_row_claims_and_what_turns_on_it`**, which holds the ingest
  ruling's promotion blocked and with it the only `obligation` edge the ruling namespace has.

### 5.2 Silent: measured, and silence is not permission

Over `ruling.toml`, with `usize` as the control returning one: `fmt` 0, `debug output` 0,
`nightly` 0, `feature gate` 0, `generic_const_exprs` 0, `printab` 0
(`out/03_silence.txt`). So at the governing tier the canon says nothing about **debug output
from a numeral** and nothing about **whether arvo's unstable machinery reaches a consumer**.
Both are named at the demand tier (`obligation`), the instrument tier (`probe`) and the
proposal tier, which is where a design will find the material to derive from. A third
silence: **the locus**, no crate and no layer named anywhere in `ruling.toml` (`252`, one
instance, and consistent with the canon-is-intent rule).

What silence licenses is narrow. The design tier may derive an answer inside the intent,
put it through two independent agreements, and record which rows it derived from. `251`'s
reading that I4's mimicry clause reaches Debug at one preset is a reading and says so; it is
the kind of thing such a derivation would start from.

### 5.3 Answered since `161`, so no longer anyone's to reserve

The container premise (`question::the_container_premise`, answered by ratified ruling); Q65
(`question::what_a_proof_marker_is_against_a_measurement`, answered by op); the door
(`question::what_the_numeric_introduction_door_may_carry_out`, answered by ratified ruling,
with the positive shape left to `proposal::the_introduction_doors_bound_is_a_position_rule_not_a_count_of_types`
at one expert). `161` section 6 named the first two as op's; both are closed and the section
is void in full.

## 6. The locus is not the canon's to fix, and what the design tier has instead

`252` established that the canon names no crate (A16) and one boundary: declared-signature
facts against placement facts, which the shipped tree already realises as `arvo-format`
against `arvo-placement`. `251` established that nothing shipped holds a value (A5) and
proposed a crate above both. The design round at `099acaf5` says locus and count go through
two readings first. **All three are right and none of them is a canon question.** A crate
name does not survive a rewrite; a boundary does, and the canon has stated the boundary.

What the design tier has to derive from, once the rows above are gated: the boundary, the
lens (clause 5, one expert, so the design cites it as a proposal), the numeral tier of R5,
and the demand in `251`'s `the_positions.out`, which says what roles the 193 positions
actually carry: truth 64, count 27, capacity 21, identity 15, version 10, code 6, index 6.
That distribution is worth more to a design than any of the five obligation rows, because it
was measured over four trees rather than read off two documents. It is one instance and
unpredicated on which consumer commit; the file carries the oids.

## 7. Findings outside the question, with what each costs

**F1. The public principles page names a locus its own round says may not be named, and the
locus is false.** `mock/PRINCIPLES.md.tmpl:220`, rendered at `docs/PRINCIPLES.md:229`,
tells a consumer that where a bare `u8..u128` or `i8..i128` would sit they write `UFixed` or
`IFixed`, and that both live in `arvo-format`. Compiled as a consumer
(`255_probes/p02_the_principles_locus/`): `use arvo_format::IFixed;` is refused with
`E0432`, "no `IFixed` in the root", and `IFixed` appears in zero files under
`mock/crates/*/src` (`out/07_principles_page.txt`); a field of `arvo_format::points::UFixed<32, 0>`
asserted four bytes wide is refused with `E0080`, because `UFixed` there is declared at
`arvo-format/src/lib.rs:105` as a unit struct, a format point holding no value. The control
with `Width` in the same field builds. So the page sends a consumer to a type that does not
exist and to one that cannot hold the number. **This landed in the same merge, `099acaf5`,
whose round topic says the locus goes through two independent readings before anything is
written against it.** The round's changelists do not touch the principles page; the page was
edited beside them, outside the phase gate because root templates are not gated, and the
edit decided in a table what the round refused to decide in prose. That is the design tier
writing a locus the canon does not fix, in a document a stranger reads, with a type name the
tree does not have. It should come out, and the row that replaces it should say "not built
yet" in the same words the `usize` and `f32` rows already use. The `design-doc-source-mismatch`
lint reads `DESIGN.md.tmpl` and not this page, which is why nothing refused it; extending it
to the principles table is the lint that is owed, in `mock/lints/`, by whoever holds that
tree.

**F2. `mockspace.toml:500-508` describes a deleted crate tree in the present tense**, naming
`arvo`, `arvo-bits`, `arvo-hash` and five consumers and assigning `USize`, `Cap`, `Bits<N>`
and `ContentHash` to them. `252` found it and it is still there at `b34d7a3c`. It is the only
document in the repository that appears to answer the locus question, it answers it about a
tree that is gone, and the table twenty lines below it was repaired while the prose was not.
Not canon, and wrong where it sits.

**F3. `AGREEMENTS.md` section 12 is stale in three sentences.** "The container premise is
op's": answered by ratified ruling. "Q65's marker question is his too": answered by op.
"The authoritative ledger is `164`'s, which supersedes `161` clause by clause": the registry
port cites `161` by line for six of its eight rows and `164` for two, and the disjunctive
clause texts `164` introduced are the ones the dissolution makes unnecessary. A reader
starting from the ledger index is sent to the wrong file and told two closed questions are
open.

**F4. The generated agent instructions list two of the four rung tiers.** `mock/agent/MAIN.md.tmpl`
at "What binds, in order" names `ratified` and `stated`; `in_force` occurs zero times in the
file and `open` is not in the list (`out/06_main_tiers.txt`, control `stated` at 4). The
omitted tier carries the widest constraint in the registry and is the one this whole
question turns on, and `235`'s misreading of it is the predictable result. `252` found this;
I re-measured it. The repair is in the template, not in any row.

**F5. `252`'s finding 7.1 is repaired at this base.** `width.rs:14` no longer says "Two types
and no more"; the source changelist of round `202609021714` took it out and cites the
ratified ruling. Recorded so nobody re-reports it from `252`.

**F6. A ratified question's `bound` leans on an unratified row.** `251` 2.5:
`question::what_the_numeric_introduction_door_may_carry_out` uses
`obligation::the_unstable_machinery_does_not_reach_a_consumer` in its `bound` to rule out its
own second option. The answering ruling refused all four options, so nothing downstream is
wrong; the shape is worth a lint and is not one.

**F7. The widest obligation has no edge from the ruling that states it.** `252` 7.3:
`ruling::the_operating_constraints_are_intents_and_rules` carries no `obligation` edge, so
`mock obligation-coverage` reports row one as reached by nothing. One edge in the whole
`ruling` namespace, on a `stated` row whose promotion is blocked. The edge is the
consolidation's to add, with the coverage figure restated afterwards.

**F8. Every consumer quotation in `obligation.toml` cites a branch and no commit.** `251`
4.7. `tools/the-positions` refuses to run on an unresolvable ref and says why; the rows
beside it do not meet the standard the tool enforces. Sixteen rows, mechanical.

## 8. What I settled, what I moved, what I could not

**Settled.** That `251` and `252` are on the surface and `161` on the concept, joined at
clause 5's degenerate point. That the pair converges on six things at two instances (2.4)
and on nothing else. That `161` was not refuted at `165`, is untouched by `251` and `252`,
and is moved only by a ratified ruling and two one-expert repairs that landed after both.
That three proposals on the topic meet the convergence bar and five candidate rows can be
written without filling a reserved call. That the principles page is false on both halves of
one row, compiled.

**Moved.** The reading of `251` and `252` from "the thread reopens" to two questions. Row
three of the five from `252`'s "needs no new concept" to `251`'s "needs the numeral", by
instrument. The platform-width row from `251`'s "silent" to `252`'s "reserved", by which
tier each measured. `161`'s section 6 from three items to none. `161`'s CONVERGED rungs to
the schema's `one_expert` where the second party read before deriving, accepted rather than
argued.

**Could not.** Supply a second instance for the definition or the lens, being their author.
Close the seam in 3.6 at more than one reading. Reach `probe.toml`, `law.toml`,
`dimension.toml` or `strategy.toml`. Separate hilavitkutin's live need from its fossil. Say
whether `in_force` is the right rung for R4 rather than `stated`; the schema's words fit and
I have written why, and it is the kind of call a second read should make on its own before
reading mine.

## 9. Carried forward unchanged, and from whom. Count: fourteen

1. The four format-spine propositions, `ruling::the_format_spine_is_canon`, op and experts.
2. The dissolution and its four propositions, `ruling::behaviour_is_stated_per_declared_signature_and_the_premise_dissolves`, seats `210` and `225`. I did not rerun its sweeps.
3. The door ruling and its reserved count, seats `238` and `239`.
4. I14 and its five bullets, op via `85` and `INTENTS.md`.
5. The ratification model, `ruling::two_experts_converging_is_a_ratification_and_the_coordinator_holds_the_gate`, op.
6. The handover, `ruling::the_panel_finishes_the_canon_without_him`, op.
7. `165`'s verdict on the revision, and its byte-for-byte rebuilds, Chlipala.
8. `210`'s two repairs and their five-pair table, Dolan. I read the section and did not rerun p2 or p3.
9. `251`'s p02, p03 and p04 outputs, the unnamed seat. Opened, not rerun; p01 rerun.
10. `252`'s four containment arms, Kiselyov. Opened the README and output listing, not rerun.
11. `the-positions`' method and its 193 over four trees, its author; the 27 over the shipped tree rerun by me.
12. The three instances behind each of R1 to R3, as `161`'s ledger states them and as no later file contested.
13. The kolli quotation's attribution to op, from the round file's own "Op:" header at `origin/dev` and from `200`.
14. That `IFixed` was a type of the deleted tree, from `mockspace.toml:500` and from the principles page's own earlier row, which I did not verify in git history and do not need to.

## 10. Options opened, and what closes each

**O1. Promote the definition and the lens.** Closed by one non-Leroy blind derivation of
what a primitive is from the ratified rows and the intents, committed before reading `161`
or any row on the topic. If it arrives at a value set with one realisation map over a
declared operation set and a lens degenerating at sole occupancy, both rows go to two
instances and the topic's keystone can be written. If it arrives elsewhere, that is the
finding.

**O2. The saturation-against-completeness seam (3.6).** Closed by a second reading of the
two texts, formed before reading mine, grounded in the ruling's `says` and `210`'s `instead`.
Two readings agreeing make it a note on R-promotion of clause 9 later; disagreeing, it goes
back to the two seats.

**O3. `in_force` or `stated` for R4.** Closed by the second read on R4 saying which the
schema's words fit, and by one line from whoever owns the lint configuration confirming the
const-generic position is passed by the numeric lints today, which the workspace rule
asserts and I did not run.

**O4. Whether R5 is canon or bookkeeping.** Closed by the coordinator's gate: the test is
whether a design derived without it would build five things. The round at `099acaf5`
suggests not, since its topic already calls them one feature; the principles page in the
same merge suggests yes, since it assigned a locus per row.

**O5. The principles page.** Closed by taking the `UFixed` and `IFixed` row to "not built
yet" and by a lint over the principles table of the shape `design-doc-source-mismatch`
already has. Two acts, both design-tier, neither mine.

**O6. Q26's hinge.** Carried from `252` O1 with `235`'s addition attached: whether "a
constant of the type" reads per compilation or per declaration, and that the storage option
would need a concept the canon does not have. Two independent readings of one phrase.

**O7. The numeral's locus.** Not opened as a canon option, because it is not one. Opened as
design work: derive from the boundary, the lens as a proposal, R5's kinds, and the role
distribution in `251_probes/the_positions.out`; two designers, compared on the consumer-facing
shape of a 47-bit dense value and a 13-bit packed one, which is `161`'s X2 and is still the
right discriminator.

## 11. Coverage, bounded

**Read in full:** `251`, `252`, `161`, `165`, `164` sections 2 and 3, `210` section 5, the
`ruling` rows named in 0.1 and their fields, the eight `proposal` rows on the topic, the
twelve `retirement` rows on the topic by id, the five `obligation` rows, nine `question`
rows, `topic::the_primitive`, `INTENTS.md` I14, `AGREEMENTS.md` section 12, the three
design-round files of `202609021714`, the `ruling` and `proposal` schema blocks in
`mockspace.toml`, `mock/agent/MAIN.md.tmpl` at the tier list, `arvo-format/src/tests/obligations.rs`
header and `the_identity.rs` header and instrument, `width.rs:8-20`, `DESIGN.md.tmpl:330-365`.

**Read in part:** `160` by its headings only, since it is mine and `165` verified it; `200`
at the quotation; kolli's round file at the quotation and the paragraph after it; `252_probes/README.md`
and its output listing; `251_probes/` p01 source, p02 source and stderr, `the_positions.out`
header.

**Ran:** the whole suite twice; `255_probes/run` twice, second run after correcting the
arm paths to relative; `the-positions` on the shipped tree; nine registry queries beyond
the script's.

**Not read, and each could move something above:** `109`, `110`, `111`, `112`, `114`,
`154`, `155`, `157`, `158`, `159`, `162`, `163` this dispatch, taken through `161`'s ledger
and `165`'s check, so every instance count in 3.5 and in R1 to R3 is one compression deep
and is the first thing a second read should open. `235` other than through `252`'s appendix.
`225`, `238`, `239` other than through their rulings. `probe.toml`, `law*.toml`,
`dimension.toml`, `strategy.toml`. Any consumer design document at source.

**Predicates.** Registry and tree facts: `base = b34d7a3c`. Compile arms: `toolchain =
nightly-2026-05-28, edition = 2024, crate_type = lib, std = none`, and no runtime dimension
is claimed. The silence finding: `namespace = ruling`, and it is stated as not holding over
the registry as a whole. The position count: `tree = .@HEAD at b34d7a3c, kinds = fn-param,
fn-return, carrier = scalar`. The convergence claims in 2.4: over the intersection in 2.1
and no wider.

## 12. The one sentence

The primitive has a candidate that survives its check and every ruling since, three of
whose clauses now stand at two instances and can be promoted; the surface has a rule in force
demanding a numeral that the candidate's own unratified lens clause describes and nothing in
the tree supplies; the canon reserves what a platform-width value is and how a width crosses
into a type, is silent on printing and on containment, fixes no crate, and the one document
that tried to fix one this week sent consumers to a type that does not exist.
