# 241. Admission is a resolution, not a verdict

Seat 241. Cold open on the admission subject of `topic = "the_number_system"`.

**This file is committed in stages and the first commit is partial.** The staging is
deliberate: the pre-read commit is the only checkable evidence that the derivation was
blind, and a stall takes everything uncommitted with it. Each stage says what it is.

## Status of this stage

Stage 1. Brief audit, canon floor, and the central derivation. Probes not yet built.
Nothing under `mock/research/` opened.

## 0. The brief's cheap claims, checked before anything else

Three claims in the dispatch are checkable in seconds and two of them are false. Neither
is fatal to the subject, but one of them changes what I am permitted to answer.

**The brief says the six are open questions. One of them is ratified canon.**
`question::is_the_number_system_inventory_open` (Q20) carries an `answered` field, and it
resolves upward: it points at `ruling::the_format_spine_is_canon`, which is
`rung = "ratified"`, `ratified_by = "both"`, and whose `ratifies` list contains
`proposal::the_concept_is_closed_and_the_inventory_is_open`. That proposal's own `topic`
is `the_number_system`, not `the_format`. So the inventory question is not open, has not
been open since that ruling landed, and was settled by a ratification carrying the lead
designer's stamp as well as expert convergence.

I do not answer Q20. Under the provenance ladder a ratified ruling is defended rather
than weighed, and re-deriving it would produce a second unratified opinion sitting beside
a stamped one, which is how a settled thing gets reopened by accretion. What I do below
instead is *use* it, because it is the strongest floor the subject has.

**The brief says the `bound` field frequently carries a constraint that has already
closed part of the question. Not for these.** Zero of the six carry a `bound` field at
all:

```
is_the_number_system_inventory_open                     bound=0 answered=1
is_admission_a_predicate_or_a_location                  bound=0 answered=0
is_number_system_broad_enough_for_non_magnitude         bound=0 answered=0
are_set_valued_carriers_admitted                        bound=0 answered=0
one_word_or_two_for_is_a_number_system                  bound=0 answered=0
what_the_admission_contract_asks_a_candidate_to_expose  bound=0 answered=0
```

The claim is true of the topic as a whole, where four rows carry one, and false of every
row assigned to me. The instruction to read the bound is good advice generally and had
nothing to find here.

**The sixth question is misnamed in the brief.** The brief calls it "the one asking what
the admission contract asks a candidate to supply". The row is
`what_the_admission_contract_asks_a_candidate_to_expose`, and it asks what a candidate is
asked to *expose*. The difference is not cosmetic and I take it as load-bearing below:
supply is what a candidate hands over, expose is what it makes visible about itself, and
the whole third option of that row turns on the distinction, since a consumer-supplied
ambient domain is a thing supplied *to* the candidate rather than exposed *by* it.

## 1. The floor, quoted rather than remembered

Four ratified sentences bear on this subject, and I take them as given.

**R1, `ruling::the_format_spine_is_canon`, ratified, `ratified_by = "both"`.** A format is
identified by its ambient domain and its representable set; that set is a constant of the
type; membership in it is one affine predicate over one parameterisation, of which
integers, fixed point, scaled integers and floats are points; arithmetic on a format is an
exact operation in the ambient domain composed with a named total adaptation onto the set;
and the concept is closed while the inventory of admitted instances is open.

**R2, `ruling::the_derivation_is_a_placement_and_the_operation_set_is_an_admission_rule`,
ratified, `ratified_by = "experts"`.** In its own words: "the design ships an admission
rule rather than an operation list: an operation is admitted exactly when it is a function
of the declared signature, and where two realisations of one name disagree, the signature
is missing a coordinate." Its `promotion` field says of itself that "the admission rule is
the format spine's closed-concept-open-inventory shape one tier down, which is what makes
it a derivation rather than a new decision."

**R3, `ruling::the_numeric_door_carries_the_coordinate_set_and_the_two_type_bound_is_not_canon`,
ratified, `ratified_by = "experts"`.** What crosses the numeric boundary is "the coordinate
set of the ratified parameterisation, spelled in types the stack owns," and the
parameterisation has ten coordinates. How many types that is, it explicitly does not say.

**R4, `ruling::the_panel_finishes_the_canon_without_him`, ratified, `ratified_by = "op"`.**
Silence in his corpus is not permission; a question his words do not reach is derived from
the intent inside its spirit and put through two independent agreements.

## 2. The central derivation: admission already has a ratified shape, and it is neither of the two the questions argue over

The six questions read as six independent disputes. They are not. Five of them are one
question asked five times, and the question is: **what kind of object does admission
return?**

R2 answers it, for operations, in ratified text, and the answer is not in any of the
option lists my questions offer. Read the second clause again: *where two realisations of
one name disagree, the signature is missing a coordinate.* That sentence describes what
happens when admission **fails**, and the failure is not a rejection. It is a **demand for
a coordinate**. The candidate is not thrown out of the set; the signature is found to be
underdetermined, and the repair is to add a coordinate until it is not.

So the ratified failure mode of admission, one tier down, is not `false`. It is the *name
of the coordinate that was not fixed*.

That is a third shape, and it subsumes both shapes `is_admission_a_predicate_or_a_location`
offers:

- **Admission is a resolution.** Given a candidate, it returns either a total assignment
  of the ratified coordinate set, or the name of a coordinate the candidate failed to fix.
- A **predicate** is that object composed with "did it succeed". It is a projection, and
  it is lossy in exactly the way the question's first option complains about: it "discards
  the coordinate a consumer needs".
- A **location** is that object composed with "which coordinate", defined only on the
  success branch. It is the other projection, and it is lossy in exactly the way the
  question's second option complains about: it has nothing to say about the candidate that
  did not resolve.

Both options are folds over one structure. The canon should state the structure and let a
predicate and a location be two interpretations of it, because that is the only version
where the two cannot drift: each is derived from the single statement rather than written
twice. Stating either projection as the canon sentence forces the other to be re-derived
by every consumer that needs it, and re-derived differently.

**This is not an import of a foreign idea.** It is what R2 already ratified, read at its
own word, and R2's own `promotion` says the shape descends one tier at a time. My subject
is one tier up from R2's operations. The same shape descending again is a derivation, not a
new decision, and it arrives with R2's ratification behind it rather than needing its own.

### What that costs, said here rather than left to be found

It costs an output type where a truth value would have done, which is exactly the cost the
question's second option names, and it does not dodge that. What it buys back is that the
cost is paid once: the two projections are free afterwards, and the failing case carries
information instead of discarding it.

It also commits the canon to *there being a coordinate set to be underdetermined about*.
R3 ratifies that there is one and refuses to say how many types it is spelled in. The
resolution reading needs the set, not the type count, so it composes with R3's reservation
rather than reopening it.

### The second reading, which I do not think wins but which the evidence does not kill

A resolution is the right object *only if the coordinate set is fixed before admission
runs*. If the ambient domain is itself a coordinate that a candidate may choose freely, and
the operation family with it, then "the signature is missing a coordinate" is not a
diagnosis but an infinite regress: any disagreement between two realisations can be
absorbed by inventing a coordinate, and admission never fails. On that reading admission
must be a predicate over a fixed schema after all, because the predicate is the only shape
that can say no.

What would distinguish them: whether the coordinate set is closed. R3 says the
parameterisation has ten coordinates and the door carries them, which reads as closed, but
R3 is a ruling about a door rather than about the concept, and it explicitly reserves the
type count. `question::is_the_ambient_operation_family_fixed` (Q33) is the row that decides
it, and Q33 is not in my six. **I therefore cannot close the resolution reading, and I do
not.** What I claim is the weaker and still useful thing: if the coordinate set is closed,
the resolution shape is forced by R2 and both of Q30's options are projections of it; and
Q30 cannot be answered without Q33, so the brief's cut of the subject is wrong at that
seam.

## 3. Stage 1 ends here

Continues in the next commit: the remaining four questions under the same reading, the
set-valued case, the one-word-or-two case, and the probes.

## Paths opened during the blind phase, stage 1

- `mockspace.toml`
- `mock/registry/question.toml` (via `cargo mock query` and direct `sed`/`grep`)
- `mock/registry/ruling.toml` (via query and direct `sed`)
- `mock/registry/proposal.toml` (via query)
- `.claude/rules/` (listing only, generated by `cargo mock`)
- `mock/crates/` (listing only, no source read yet)

Nothing under `mock/research/` opened.
