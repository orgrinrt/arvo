# 201. Is the canon exhaustive enough, and what a designer could not do from it

## The gates

**Canon gate: passed.** The standard is op's own, at `181`: he will not review until the canon is
exhaustive enough that a full design and then a full implementation can be done from it. That is the
thing measured below, and it is measured with the canon's own fields rather than against my taste.

**Test gate: passed, 98 passing, no ignores.** I read the arms over the surface I audit. The coverage
ratchet has been rewritten since I last saw it and the rewrite is right: it bounds **what is answered**
rather than what is outstanding, and its comment says why in terms I would not improve on, that a flat
ceiling on the outstanding count "fails the moment somebody reads a consumer nobody had read and writes
down what it asks for, which is the demand side working exactly as intended". `a_closed_route_does_not_improve_coverage`
is a real control and it caught a real defect.

**One thing in it is loose.** The `ANSWERED` constant's doc says "reached by a ruling or a **ratified**
proposal". The code counts `met + proposed`, and no proposal in this canon is ratified. The assertion
message is accurate where the doc comment is not, so nothing is broken; what it produces is the sentence
in section 1.2.

---

## 1. The answer

**The bar is not met, and it is not close.** What a designer could not do from this canon is the
ordinary case rather than the exception, and the reason is not that rows are missing. It is that almost
nothing in it has been settled by anybody.

### 1.1 The measurement, in four numbers

```
$ 201_probes/how_much_of_the_canon_is_settled.sh

  rulings          75
    of those, process (how the panel runs, not what arvo is)  37
    at rung ratified 1   in_force 1   stated 68   open 5
  proposals        103
    ratified by a ruling (`ratifies` edges in the whole canon)  0
    normative, i.e. a design decision rather than a measurement  62
    resting on one expert  71
```

**Zero of a hundred and three.** Not one thing the panel established has op's stamp on it. Sixty-two of
them are `normative`, which the schema defines as "imposed rather than established" and is the mark a
design decision carries. Seventy-one rest on one expert.

**And thirty-seven of the seventy-five rulings are `process`**: how the panel is run, not what arvo is.
Of the thirty-eight that are about arvo, **one is ratified and one is in force**. The rest are `stated`,
which op's own standing instruction says is "an ack rather than a ruling" and is "not to be written as
clear cut and settled".

### 1.2 Two claims in the brief that do not survive checking

**"15 obligations, 2 answered."** The two are one `met` and one `proposed`. `mockspace.toml` on that
field: a proposal "does not meet one: a proposal is proposed rather than met, and reporting it otherwise
closes a gap op has never seen." **Reporting the sum upward is that collapse**, and it is going upward to
the person the sentence exists to protect. The honest line is `met 1, proposed 1, route-closed 3,
nothing 10`.

**"Eight rows now carry the consumer's verbatim words."** Twelve do. The three without are
`debug_output_from_every_numeral_shape`, `composition_contracts_above_the_numeral` and
`the_surface_expressible_as_contracts_before_anything_implements_it`, which are exactly the three with no
consumer document behind them, so the absence is correct and the count is not. Sixth failed brief claim
this arc, and the cheapest to have checked.

## 2. What the canon itself says cannot be written

**The `question` namespace has a field for exactly this.** `unblocks` is "what becomes writable once it
is answered", so an open question carrying one is a sentence this corpus wrote about itself naming
something a designer cannot write. No judgement of mine is involved.

```
$ 201_probes/what_cannot_be_written.sh

  79 questions:  4 answered   29 proposed   46 open
  carrying an unblocks: 18,  of those still open: 15
```

**Fifteen, and twelve of them are op's.** By decider: twelve `op`, two `panel`, one `measurement`.

That is the answer to the question the brief actually asked. **The missing thing is overwhelmingly not a
row somebody can write today.** Three are: `is_the_role_set_closed`,
`does_a_named_policy_selection_survive_above_the_numeral`, and
`which_reassociated_arm_a_law_licenses`, which is a harness run rather than an argument.

The twelve group into roughly six conversations:

- **What a numeral is**: `mixed_numeral_addition`, `arithmetic_column_one_axis_or_two`, `the_width_surface_crossing`, `does_precision_count_the_sign_digit`.
- **The strategy axis**: `which_operation_set_the_design_ships`, `which_units_a_weighting_is_expressed_in`.
- **Rounding**: `why_the_default_rounding_position_is_chosen`, `the_rounding_mode_vocabulary`.
- **What a canon sentence may be**: `may_the_canon_carry_an_unpredicated_proposition`, `is_the_rounding_candidates_pairing_section_canon_or_design`.
- **The container premise**, alone.
- **Whether the observability principle becomes an intent**, alone.

**So the canon is not one dispatch of writing away from op's bar. It is about six sittings with op away
from being able to start.** That is a better position than a long list of rows nobody has written, and it
is not the position the coverage numbers suggest.

## 3. Where the bar is met, stated as a region

**One topic, and it is real.**

```
holds for: topic = operating_constraints, op rows = 1 at rung in_force, open questions = 0,
           unratified proposals = 0
```

`the_operating_constraints_are_intents_and_rules` is `in_force`, enforced by lints independently of
anything the panel converged on, and it says what it says: no std, no alloc, const sizes with no runtime
growth, monomorphisation with no dyn and no TypeId, no platform dependency, and the stack's own
primitives at public API positions. **Zero questions sit under that topic.** A designer knows exactly
what binds them and an implementer can check it mechanically. That region passes op's bar today.

**One more topic passes in shape and not in content.** `the_predicate_notation` carries the canon's only
ratified row, `the_work_is_predicated_arms_composed`, which tells a designer the *form* of what to write:
arms with const predicates over the regions where something holds, composed, with a universal answer
rejected by premise. That is genuinely enough to know how to write the design. It is not enough to know
what to write, and one of its three questions is still fully open.

**The honest summary of the region: the canon is exhaustive about how a design must be expressed and
constrained, and about nothing that is to be designed.**

## 4. Three topics have nothing from op at all, and they are what a numeral is

```
  TOPIC                      OP-ROWS  PROPOSALS
  the_format                 0        6
  the_primitive              0        6
  the_chain                  0        10
  the_number_system          2        19
  algebraic_laws             1        15
  the_strategy_object        1        10
  the_strategy_axis          15       9
```

**Twenty-two claims across three topics with no op input whatever**, and the three are the format, the
primitive and the chain: what a numeral is, what a primitive is, and what a composition of them is.
`canon_form` and `the_container_premise` also have none, at one claim and zero.

**The strategy axis is the only inversion**, fifteen op rows against nine claims, and it is where op has
actually engaged. **And its founding row is at `rung = "open"` by his own hand**:

> Okay let me just say that the strategy set is not closed at exactly four. These are the ones the last
> panel settled with, and what my amateur ass had written for arvo that we are now redesigning, so it's
> entirely open to discussion and exploration

**Every numeral in this design carries a strategy.** All four rows in the `strategy` namespace stand at
`prior_attempt`, which the schema defines as a name the previous design carried. **So the canon names no
strategy it ships**, and the one parameter that appears on every type in it is an open set by op's
explicit ruling.

There is a written resolution and op has not seen it:
`the_named_strategies_are_points_in_a_product_and_the_flat_set_is_a_slice` says a name is a binding
rather than a member of a closed set, which would make the openness harmless. It is a `proposal`. Like
the other hundred and two, it is proposed.

## 5. What I got wrong in `200`, found by the seat that acted on it

**I wrote that `union` is asked for nowhere in the consumers.** It is: the live design walks "the union
of the fiber's units' write masks", and the imported prior art enumerates "AND for conflict, OR for
union, `== 0` for empty check". The bit-set row now carries union on that evidence and is right to.

**What I actually did was assert an absence over a corpus I had read part of.** I read the plan chain and
the numeral-facing crates carefully and never grepped the consumers for the word, and then wrote a
sentence quantified over all of them. **That is the second time in this arc**: `195` said "nobody did the
work" on the strength of having searched twelve TOML files.

**It is the same defect class this audit keeps finding in everybody**, `184`'s kolli gap and `191`'s
control that reported zero on a tree with ninety-six matching lines included, and the rule is one
sentence: an absence claim carries the search that established it, or it is not a claim. I have now
supplied two instances of it myself and I would weight my own absence claims accordingly.

The row's `gap` field records all of this, which is the corpus working.

## 6. One thing the canon does not license, and it is the second audit to say so

`what_a_proof_marker_is_against_a_measurement` is `decider = "op"`, carries no answering edge, and asks
how the disagreement between the ratified predicate notation and the panel's own findings is resolved.
Its second option is op ruling that a proof carries a different marker from a measurement.

**The schema implements that option.** `sentence_kind` has `theorem`, `measured`, `enumeration`,
`normative` and `argument`. I raised this in `187` section 9, where what I could point at was
one file having marked its own 74 rows. **Every one of the canon's 103 proposals now carries
the mark.** Op has still not been asked, the question still reads as open, and the registry has
taken one side of it in a field every claim in the canon is marked with.

**This is not an argument that the mark is wrong.** It is a good mechanism and I use it. It is that a
question of op's has been answered by the shape of the file, that this is the second audit in a row to
say so, and that every claim written since carries the mark.

## 7. What I would tell the next reader

1. **Take section 2's twelve to op, grouped as six conversations rather than as twelve questions.** They are what stands between this canon and a design, and no amount of panel work closes any of them.
2. **Stop reporting `met + proposed` as one number.** Section 1.2. The schema forbids it in the field description and the collapse is currently travelling upward in briefs.
3. **Section 4 is the thing to put in front of op first if only one thing goes.** The strategy set is open by his own ruling, it is a parameter of every type in the design, and there is a written answer waiting that he has not seen.
4. **Three topics have no op input and carry twenty-two claims.** A canon whose account of what a numeral is rests entirely on unratified single-expert normative rows is not one a second designer would reproduce, which is the acceptance test the workspace states for a canon.
5. **`the_predicate_notation`'s open question is the one to close cheaply**, because the schema has already acted on it and the exposure grows with every row.

## 8. What I did not do

Wrote no registry row, and this dispatch asked for an audit rather than filing. Did not read all 598 rows;
I read the `ruling`, `obligation`, `strategy` and `topic` namespaces whole, and sampled `proposal` and
`question` through the two instruments, both of which walk every row. Did not re-verify the consumer
statements behind the reworded obligations beyond the bit-set one that corrected me, because the seat
that wrote them quoted the consumer with the file, which is the repair I asked for in `200` and it is
there. Did not challenge whether a given row sits in the right namespace, which was in scope and which I
found no instance of worth raising: the namespace split held everywhere I looked.

**And I opened this dispatch by correcting the brief's file number and was wrong**: my last file landed
renumbered as `200` because `199` had been taken by the reply seat, so the brief was right and my
correction was not. It cost nothing because I checked it in the same breath, which is the only reason to
mention it: the check is cheap enough that being wrong in public is the cheap outcome.
