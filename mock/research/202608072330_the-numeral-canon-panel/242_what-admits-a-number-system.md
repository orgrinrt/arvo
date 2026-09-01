# 242. What a number system is, and what admits a candidate

Seat 242. Cold open on the admission subject: `is_the_number_system_inventory_open`,
`is_admission_a_predicate_or_a_location`, `is_number_system_broad_enough_for_non_magnitude`,
`are_set_valued_carriers_admitted`, `one_word_or_two_for_is_a_number_system`, and
`what_the_admission_contract_asks_a_candidate_to_expose`.

**PARTIAL COMMIT.** This is the derivation in progress, committed before it is finished and
before anything under `mock/research/` has been opened, because the pre-read commit is the only
checkable evidence of blindness. Later commits on this branch extend it. The reconciliation
section is written after the first read of `mock/research/` and is marked as such.

## 0. The brief's premises, checked before anything else

Three claims in my brief. Two hold, one does not.

**Holds.** `mockspace.toml` declares `canon_paths = ["mock/registry/*.toml"]`, so the registry is
the canon and there is no canon prose directory. `mock/crates/` holds exactly three crates:
`arvo-format`, `arvo-placement`, `arvo-strategy`.

**Holds.** `question.where(topic=the_number_system).count()` returns 18, and all six of my named
rows resolve, once `what_the_admission_contract_asks_a_candidate_to_supply` is corrected to its
actual id, `what_the_admission_contract_asks_a_candidate_to_expose`. The brief misnames it. Minor.

**Does not hold, and I am not working around it.** The brief says: "Read each row's own `asks`,
`options` and `bound`; the `bound` field frequently carries a constraint that has already closed
part of the question." **Not one of the eighteen `the_number_system` rows carries a `bound`
field.** 24 of the registry's 105 question rows carry one; zero of them are in this topic. So the
instruction to read `bound` on these rows is an instruction to read a field that is not there.

This is worth more than a correction, because the field that *does* do the job named exists and
is a different field. `is_the_number_system_inventory_open` carries an **`answered`** field
recording that `ruling::the_format_spine_is_canon` already settles it. That is the mechanism the
brief was reaching for, under another name, and only two of the eighteen rows have it: the
inventory one, and `adaptation_in_identity_or_realisation`. A member who took the brief at its
word would have gone looking for a constraint on all six, found none on any, and concluded that
nothing prior bears on them. One of the six is in fact already closed.

I flag it and continue rather than stopping, because the false premise is about where to look
rather than about what is true: it does not frame the answer, and the correct field is one query
away. Had it asserted a *conclusion* I would have stopped.

## 1. A provenance hole in the canon's own top tier, reported because I found it

Outside my question, per the standing instruction.

`ruling.where(rung=ratified).count()` returns **32**. Of those, `ratified_by=op` is 7,
`ratified_by=experts` is 5, `ratified_by=both` is 2. **Fourteen of thirty-two ratified rulings
name a ratifier. Eighteen do not.**

My brief tells me `rung = "ratified"` governs and is defended rather than weighed, and that
`ratified_by` distinguishes op's stamp from a two-expert convergence. Both halves cannot be true
of a row at `ratified` with no `ratified_by`: it claims the governing tier while stating no human
and no convergence was in the loop. Under this workspace's own provenance ladder, an artifact
that does not state a human was in the loop is presumed wrong. So eighteen rows are
simultaneously the top tier and, by the ladder that defines the top tier, unratified.

I do not know which reading is right and I am not going to resolve it from one seat. The two
readings, and what would distinguish them:

- **The field is optional and its absence means the older ported rows predate the convention.**
  Distinguisher: whether every ratified row lacking `ratified_by` has a `provenance` pointing at
  a pre-registry source, and every one carrying it points at a panel file.
- **The field is load-bearing and its absence is a defect in eighteen rows.** Distinguisher: any
  registry check or lint that reads `ratified_by`, or any row whose `because` claims a
  convergence the field does not record.

Either way it is a finding for whoever combs the registry, not for me to fix.

## 2. What governs my subject, quoted rather than remembered

`ruling::the_format_spine_is_canon`, `rung = ratified`, `ratified_by = both`, topic `the_format`.
It ratifies four propositions as one shape, and two of the four are load-bearing here:

> A format is identified by its ambient domain and its representable set, and that set is a
> constant of the type. Membership in it is one affine predicate over one parameterisation, of
> which integers, fixed point, scaled integers and floats are points. Arithmetic on a format is
> an exact operation in the ambient domain composed with a named total adaptation onto that set,
> and the adaptation is a first-class object with its own laws. **The concept is closed and the
> inventory of admitted instances is open.**

Op's condition travels with it, in his words on that row:

> I stamp them but all of these are subject to being changed if the experts themselves end up
> disagreeing or finding a better solution or wording or bound.

Two open deferrals sit on my topic and both are his, both `rung = open`:

- `ruling::the_family_question_wants_the_comparison_first`: whether the numeral space is one
  family or several wants a written comparison first, stating for each candidate what becomes
  derivable, what has to be named, what the canon must say that it otherwise would not, and what
  it costs a consumer.
- `ruling::his_instinct_on_one_family_is_not_to_be_acted_on`: he records an instinct for one
  family and instructs that it not be acted on. The row's own note: a member citing that instinct
  as a ruling is making the previous panel's mistake with his words instead of its own.

I take the second at its word and do not use the instinct as evidence in either direction.

## 3. (in progress)

The derivation continues in later commits on this branch.

## Paths opened during the blind phase, so far

- `mockspace.toml`
- `mock/registry/question.toml`, `mock/registry/ruling.toml` (via `cargo mock query`, plus one
  `grep -n` for the misnamed id and one `awk` over `question.toml` to establish the `bound` count)
- `ls mock/registry/`, `ls mock/crates/`
- Nothing under `mock/research/`. Nothing under `mock/design_rounds/`. No other branch, and no
  `git log` beyond my own commits.
