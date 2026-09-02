# 197. What `35` and `43` established, and the one obligation they reach

## The gates

**Canon gate: passed.** Op at `181` says to port the results and shape the registry so it works, which is
what filing a derived result as a row is. `mockspace.toml` declares every field used here. Nothing below
edits `obligation.toml`, and section 5 says why that matters more than usual this time.

**Test gate: passed, 77 passing, no ignores.** I read the bodies of the arms over the surface I touch, in
`mock/checks/src/obligation.rs` and `tests/what_reaches_each_obligation.rs`. **They are better than the
shell probe of mine they replaced**, and one of them is better than it looks: `a_closed_route_does_not_improve_coverage`
exists because the ceiling test's first version capped `nothing` alone, so adding the retirement edge moved
three obligations out of `nothing` and dropped the count by three with no canon progress at all. The
instrument caught a defect in its own gate before anybody else did. That is what a control is for.

---

## 0. The brief's premise is not what my `195` said, and my `195` over-reached in the same area

**Two corrections, and they point opposite ways, so neither cancels the other.**

**The brief attributes to `195` a claim it does not make.** It says my report "said the algorithm surface
is missing filing rather than derivation". `195` says the opposite, in these words:

> **So the answer for these five is: nobody did the work, and the work has never been the subject.** Not a
> filing problem.

The filing claim is `191`'s, about `35` and `43` specifically, and I had not read either file when I wrote
`195`. This is the third brief in this arc to carry a factual claim that did not survive being opened, and
`190` is the write-up of the previous two. **The cheap check is the one that keeps not happening**: one
grep of the file being cited, before the sentence about it is written.

**And `195` over-reached.** What I searched was the registry, twelve TOML files. What I wrote was "nobody
did the work, and the work has never been the subject", which is a claim about the corpus. The corpus was
not searched. **The evidence supported "no registry row reaches it" and I wrote something larger.** Having
now read both files, the larger claim is wrong in an interesting way and right in the way that matters,
which is section 2.

## 1. Which obligations the two files reach, edge by edge

**One of thirteen, and it is the one that was already reached.**

| obligation | `35` | `43` | tier reached |
|---|---|---|---|
| `set_operations_over_a_fixed_size_bit_set` | no | no | nothing |
| `ordering_a_directed_acyclic_graph` | uses a DAG programme as an instrument | no | nothing |
| `a_sparse_adjacency_a_plan_can_be_built_on` | no | no | nothing |
| `a_spectral_partition_of_a_dependency_graph` | no | no | nothing |
| `a_cost_dynamic_program` | no | no | nothing |
| `a_platform_sized_unsigned_integer_at_an_api_position` | no | no | nothing |
| `an_exact_width_container_a_consumer_can_alias_and_pin` | no | no | unchanged |
| `a_content_hash` | no | no | nothing |
| `debug_output_from_every_numeral_shape` | no | no | nothing |
| `a_build_flag_that_changes_float_semantics` | no | no | nothing |
| **`composition_contracts_above_the_numeral`** | **yes, four results** | **yes** | **proposed** |
| `a_primitive_for_every_position_a_bare_number_would_take` | no | no | nothing |
| `the_surface_expressible_as_contracts_before_anything_implements_it` | no | argues the contract is arvo's | nothing |

**The tier of every edge I wrote is `proposed`.** Both files are agent output with no human in the loop, so
nothing in either can reach `met`, and saying so is not a hedge: it is the whole difference between the two
columns.

**The one that looks like a hit and is not.** `35_probes/p5` runs two DAG dynamic programmes and measures
a shortest path returning wrong answers on 407,293,133 of 832,398,764 in-range instances at width 4 under
wrapping. That is real, it is the algorithm layer, and it does not touch `ordering_a_directed_acyclic_graph`:
its own header says both routines run "over a **fixed** topological order". The ordering is an input.
**It measures what a numeral does to an algorithm given an ordering and establishes nothing about producing
one**, which is what that obligation asks for.

**So `195`'s conclusion survives the reading that was supposed to overturn it**, and survives it having been
tested from the other direction: I started at the two files this time rather than at the obligations, and
arrived at the same place.

**What did not survive is the reason I gave.** "Nobody did the work" is wrong. A great deal of work was
done, on exactly the layer above the numeral, with ten committed instruments. It reaches the numeral's
obligations to that layer and it does not reach the layer's own deliverables, and those are different
things that I collapsed.

## 2. What they established that no obligation asks for

**This is the larger half, and the answer to why an enumeration built from consumer documents cannot see
it.**

Fourteen results between the two files, seven from each. None is asked for by any of the thirteen obligations, and the reason
is structural rather than an oversight in the enumeration.

**From `35`:** a fold's operation must be closed or its trip count static; the accumulator is a function of
element width and capacity rather than of operand widths; a fold's seed must be representable and at nine
of sixty-three cells the multiplicative identity is not; a min-plus fold needs an absorbing top and only
saturation supplies one; absorption and monotonicity are two separable properties and min-plus needs both;
a reduction may only be split where the operation is associative; algebraic rewriting is a soundness trade
and the fractional part is what makes it one.

**From `43`:** a composition is a binding-time distinction rather than a container; its defining boundary is
capacity against count with `len <= capacity` as its own invariant; no derivation reads the numeral's grid,
so those coordinates may be held at run time; the operations disagree with the derivations and the split is
exact, addition reading neither the adjustment nor the canonical exponent while multiplication reads the
exponent and the bias is in neither set; compositions nest and the nest must be flattened before the
derivation runs, sound and not tight at one bit on 1201 of 4096 two-level shapes; a numeral is not a
degenerate composition; and the capacity invariant does not survive lowering.

**Why the demand side is blind to all of it.** An obligation is a consumer need in the consumer's terms.
The consumer asks for a topological sort. What it actually needs first is that the numeral's addition be
associative in the cells it runs in, because op's `I10` says arvo will use however many cores it detects and
splitting a reduction changes the association order. **No consumer document says that, because the consumer
does not know it.** It is a fact about the interaction between the two layers, and it is discoverable only
by deriving it, which is what these two files did.

**So the gap is not in the enumeration and cannot be closed by enumerating harder.** It is that the schema
has no way to say *this obligation cannot be met unless that proposal holds*. `ordering_a_directed_acyclic_graph`
is unmeetable, for a consumer running on the cores `I10` promises, unless the addition under it is
associative; the registry can hold both sentences and cannot join them. **That is the missing edge, and it
is the fourth time across four dispatches I have wanted a relation the schema does not have** (partial
answers, bears-on, half-met, and now precondition).

## 3. What I filed

**Five probe rows and four proposals.** Ten instruments sat behind the two files with **no probe row naming
a single one of them**, so every result in both was unciteable: an `evidence` field drawn from either had
nothing to point at, which is why two of `35`'s fourteen figures reaching the registry was never a selection
anybody made.

Each probe row carries the case that had to fail, and each of the five genuinely has one:

- `a_widening_operation_cannot_be_folded`: four positive arms compile clean in the same run, which is what locates the wall at the trip count rather than at the widening.
- `which_sign_and_policy_cells_survive_reassociation`: one counter reports zero in three cells and a majority in the fourth.
- `the_top_absorbs_under_saturation_and_never_under_wrapping`: the same column reports 0 of 63 and 63 of 63, and a second arm moves independently of the first.
- `a_dag_dynamic_programme_returns_wrong_answers_on_in_range_instances`: an instance counts only where the exact answer and every intermediate fit in range, so a disagreement cannot be range exhaustion; and three of four routine-and-policy cells report zero.
- `no_derivation_reads_the_numerals_grid`: three negative compile arms, each of which must fail and does, including one showing the sameness relation is not vacuous.

Four proposals drawn from them, each predicated over every dimension that could move it, each wired to
`composition_contracts_above_the_numeral`. Two carry `threads any` on an argument rather than a sweep, and
both say so in `note`: the refusal is a type-check outcome that precedes execution, so it cannot vary with
a thread count. **A reader wanting that dimension measured should treat it as unmeasured**, which is the
honest form and not a hedge.

**Obligation coverage does not move, and that is the result rather than a miss.** Measured by forcing the
ceiling test to print:

```
11 obligations are answered by nothing, against a ceiling of 11.
{"met": 1, "nothing": 8, "proposed": 1, "route-closed": 3}
```

Identical before and after. **Four measured results landed and the demand side did not notice**, because
all four reach the one obligation that was already reached.

## 4. What I did not file, and why each

**`35` 3.5, monotonicity.** Measured at 33 of 33 cells under saturation and failing at 33 of 33 under
wrapping to 33.07 percent of triples. It is half of what a min-plus computation needs and the filed
absorption row names it as a gap. I left it because it wants its own probe row over `p2`/`p2b` and the
absorption row would then need its predicate rewritten to the intersection of three instruments. **A row
each is the right shape and this pass did not have the second one.**

**`43` s3, the flattening.** Exhaustive over 4096 two-level shapes with a negative control that fires at
6502 overflows one bit narrower, which is a good instrument. Its result is about nesting rather than about
what the layer above needs from the numeral, and I could not decide whether it belongs under `the_chain` or
wants the topic `191` proposed. **Left rather than filed under a topic I picked to make it fit.**

**`43` p7, the assembly read.** Its own `RUN.md` says "A qualitative assembly read. Not a bench, no timing,
and the file says so in its own header." So it cannot carry a magnitude, and the interesting figure, 58
assembly lines against 94, is exactly a magnitude. **Filing it would have put a number into the registry
that its own author declined to price**, and the honest row would be `uncontrolled` carrying no figure,
which says less than the file already says.

## 5. The spectral obligation was widened past what the consumer says, and the justification cites a table that does not exist

**This is outside the question I was sent for and the brief asked for it anyway, so here it is without
softening.**

`a_spectral_partition_of_a_dependency_graph` was reworded to ask arvo for "**the Laplacian** over an
adjacency the consumer already holds, the iteration that finds the Fiedler vector, and the bisection and
k-way split over it". Its `gap` justifies the widening:

> Its foundations table gives arvo the Laplacian construction, the power iteration, the Fiedler vector and
> both split forms; it builds the Laplacian from its own adjacency rather than instead of arvo.

**The consumer's design says neither thing.** Read from a committed ref rather than a working tree, because
the clone is on a recovery branch, and read on both refs, which are identical on these lines:

- `mock/DESIGN.md.tmpl`, the foundations sentence: "`arvo-spectral` for **the Fiedler partition step**." One step, named once.
- `mock/crates/hilavitkutin/DESIGN.md.tmpl`: "Spectral partitioning → spectral `FiberGrouping` via an **engine-local symmetric Laplacian** over arvo-spectral's k-way partitioning."

Counted across both refs and both files, with controls:

```
  Laplacian construction   occurrences: 0
  power iteration          occurrences: 0
  both split forms         occurrences: 0
  bisection                occurrences: 0

  control, present:  engine-local           per-crate 1
  control, present:  Fiedler partition step top-level 1
  control, absent:   ZZZ_NOT_IN_ANY_DESIGN  0
```

**Every one of the four things the `gap` attributes to the consumer's table occurs nowhere in the
consumer's documents, and the one thing the consumer does say is that the Laplacian is engine-local**,
which is what `191` section 4 read off the same line and what the rewording overrode.

**Why this is worse than an ordinary wrong row.** The obligation namespace's whole justification, in its own
file header, is that it is "read from outside the canon on purpose: a check that walks the canon can only
report that the canon agrees with itself, so the enumeration has to come from somewhere the canon does not
reach." **An obligation widened by an agent from material the consumer did not write is no longer outside
the canon.** It is the canon talking to itself with a consumer's name on it, and it inflates the one
measurement op's `181` bar is checked by. The row's `provenance` points at `184` rather than at the
consumer, so nothing in the registry lets a reader catch this without leaving the repository.

**It should be narrowed back to what the consumer asks for, and the `gap` sentence deleted rather than
edited.** I did not do it because `obligation.toml` is not mine to write and because a second reader is owed
on a call this consequential.

## 6. The challenge the brief invited, and it fails

**Whether the algorithm surface belongs in arvo at all.** I read `I11` rather than a summary of it:

> our main selling point are the algo crates that hilavitkutin, vehje, pretty much every single repo and
> project I have, downstream, use. As well as the contracts for things that compose to bigger units than
> just numerals alone.

**That is not ambiguous and it is op's own voice.** The algorithm crates are half of what he says arvo is
for. The five obligations should exist, and the challenge I was invited to make does not survive its own
evidence. **What is arguable is scope inside each**, which is section 5's subject and is a question about
one obligation's wording rather than about the five existing.

**One thing does deserve saying about `I11` while it is open.** Its `rung` is `stated`, and `35` and `43`
are, between them, the panel's only sustained work on the second half of that sentence. The first half has
nothing after the whole of this panel and the one before it. **A reader taking the tally at face value would conclude the panel deprioritised
the algorithm crates; the truth is narrower and worse, which is that nobody has ever opened the question.**

## 7. What I would tell the next reader

1. **Section 5, and it should go to op or be narrowed before anything else is built on it.** An obligation carrying a four-item demand the consumer never wrote is a corrupted measurement of the only bar `181` sets.
2. **The missing precondition edge, section 2.** Five obligations are unmeetable under `I10` unless results now sitting in `proposal.toml` hold, and nothing can say so. That is a schema question and it is the one that would make the coverage number mean what people read it as meaning.
3. **`35` 3.5 and `43` s3 want one probe row each**, section 4, and both instruments are committed and controlled. Cheap, and the monotonicity one completes a claim already filed with the gap named.
4. **Two of my four proposals carry `threads any` on an argument rather than a sweep.** Both say so. If that reading is wrong the two rows narrow to `threads = 1` and nothing else about them changes.
5. **Nobody has swept the panel's own corpus for consumer statements**, which `184` names as owed and which is where the rest of the demand side is. `191` found five obligations that way from consumer documents alone; the corpus has 2106 occurrences of the word.

## 8. What I did not do

Wrote `probe.toml` and `proposal.toml` only, additively: 68 and 99 lines inserted, zero deleted, row counts
79 to 84 and 77 to 81. Did not touch `obligation.toml`, `question.toml`, `ruling.toml`, `retirement.toml` or
`topic.toml`, and did not add the topic `191` proposed, because re-filing existing rows is a cost somebody
should choose deliberately. Did not read `35` and `43`'s full citation trails, only the instruments the
filed rows rest on. Read the consumer's design out of the sibling clone in the parent workspace rather than
cloning it into mine, from a committed ref so a working tree could not affect what I saw, and that is the
one thing here I would have done differently with more room.
