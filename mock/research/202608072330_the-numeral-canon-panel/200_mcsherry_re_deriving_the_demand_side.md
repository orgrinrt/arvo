# 200. Re-deriving the demand side from statements of need

*Renumbered from 199 by the dispatcher. Two seats ran in parallel and both took
the next free number, so both deliverables and both probe directories landed as
199. Nothing collided by filename and the merge was clean, which is exactly why
it needed catching: the number, not the file, is what every later citation says.
Numbers are assigned in the brief from now on.*

## The gates

**Canon gate: passed.** `184` names this sweep as owed, the obligation namespace's own header says why
it has to come from outside the canon, and op's `181` bar is what it serves.

**Test gate: passed, 94 passing, no ignores.** I read the arms over the surface I touch, including the
new `precondition_for` walk and its two controls. `preconditions_from_a_namespace_that_cannot_establish_one`
is the right shape: the walk reads a fixed list, so an edge from anywhere else would contribute nothing
and the report would read clean, and that is exactly the case it plants.

---

## 1. The headline, and it is one measurement

**No obligation cites a consumer. Ten cite `184`. Three cite op.**

```
$ 200_probes/whose_words_is_each_obligation.sh
  consumer 0   op 3   summary 10   none 0
```

**And that is by design rather than by carelessness**, which is the part that matters. `184` says so in
its third sentence: "It exists to be cited: the `obligation` rows in the registry point here, **and a
citation into another repository is not resolvable from this one**." `mockspace.toml` declares one
reference root, `panel`, over `mock/research`. There is no way to address a consumer.

So the namespace whose entire justification is being read from outside the canon **has an agent's
summary welded into the one place its independence depends on**, because the reference system cannot
express the citation it needs. Every wording error found in these rows entered at that seam: the
spectral one, the cost-DP one, and the third this pass found. Three seats, three errors, one mechanism.

**This is not a criticism of `184`.** It did the reading, it named its own gaps, and it said in terms
what it had not swept. It is a criticism of a shape that puts a paraphrase where a quotation has to go
and then makes the paraphrase the only citable thing.

## 2. Where the 2106 came from

**Found, at exactly one scope, and it is the wrong one.**

```
184 landed at c962834d
SCOPE                                             OCCURRENCES  FILES
mock/research/.../panel/*.md                      1440         155
mock/research/.../panel/          (all types)     2100         446
reported by 184:                                  2106         436
```

The panel directory counted over **every file type**, at the commit `184` landed. The residual is 6 and
10, and the file-count digits are transposed. At that scope the files carrying hits are 166 `.rs`, 155
`.md`, 45 `.py`, 27 `.out`, 24 `.txt`, and the rest smaller.

**So the figure that justified "the majority of the enumeration will come from this corpus" counts probe
sources and outputs.** More of its files are Rust than prose. The prose corpus was real and about
two-thirds the size advertised, and section 4 is what is actually in it.

## 3. Per obligation: is there a statement of need

**Read from the consumers, cloned into this worktree rather than reached for in a sibling.**

**Survives, and keeping it is the result.** `a_platform_sized_unsigned_integer_at_an_api_position`,
`an_exact_width_container_a_consumer_can_alias_and_pin`, `a_content_hash`,
`a_build_flag_that_changes_float_semantics`. Each has a consumer statement saying what it needs and why:
OS error codes carried as an arvo value in a fixed enum with no heap payload; a runtime hash as
`arvo_hash ContentHash` with the rkyv archived form a bare `u32`; one cfg the build wrapper emits.
**Four of thirteen were derived correctly and nothing about them should move except their provenance.**

**Survives with its need understated.** `set_operations_over_a_fixed_size_bit_set`. The consumer says
`AccessMask` is `Mask64` at up to 64 stores or `Mask256` at up to 256, that step 1 builds the DAG from
`AccessMask` **overlap** and step 12 is dirty propagation, and, load-bearing and absent from the row:
"**All scheduler ops = single-instruction bitwise.**" That is a lowering requirement stated by the
consumer and the obligation does not carry it. Meanwhile **union and difference appear nowhere**;
overlap, which is intersection, is what is asked for.

**Survives, with one item in the wrong row.** `ordering_a_directed_acyclic_graph`. Steps 2, 3 and 4 each
carry a stated purpose in the consumer's own words: linearise or reject as `PlanError::Cycle`, upward
rank to a critical path, waist detection to phase boundaries. But its fourth item, the
bandwidth-reducing reordering, comes from the dependency sentence's "arvo-graph for DAG / RCM", and the
per-crate design puts RCM at **step 5, attributed to arvo-sparse**. The consumer disagrees with itself
and the row inherited the half that was a crate list.

**Does not survive, and it is the third of its kind.** `a_sparse_adjacency_a_plan_can_be_built_on` asks
arvo for "a compressed sparse representation of a dependency graph, sized at compile time, that stays
the fastest shape at every graph size". The consumer builds it:

> DependencyGraph **uses** CSR backing (`row_offsets` + `col_indices` + `edge_kinds`) per Topic 9 axis B,
> bench-locked canonical at every N (no threshold). **It converts to a bidirectional CSR** (forward
> adjacency plus a pre-computed transpose) **to feed the arvo-sparse structural steps**.

**Identically to the spectral case just corrected**: the consumer constructs the representation and asks
arvo for the analyses over it. What it wants from arvo-sparse is steps 5 and 6, the RCM renumber
permutation and the block-diagonal connected-component partition with Dulmage-Mendelsohn fine
decomposition and dead column elimination. "Bench-locked canonical at all N" is the consumer describing
its own choice of backing, and the row reads it as a demand on arvo.

**Already corrected, and one note on the correction.** `a_cost_dynamic_program` was reworded off
`arvo-comb::bin_pack`. That string occurs **once in the whole consumer, in a `BACKLOG.md.tmpl`**, which
is a wish list rather than a design. The live per-crate design's step 8 says a width gate picks between
the spectral former and the greedy one. The rewording is better than what it replaced and its provenance
is weaker than a design.

**Has no consumer behind it at all.** `debug_output_from_every_numeral_shape` is sourced from an arvo
sketch's own FINDINGS, which is inside the canon. It may well be a real need; it is not a demand-side
row, and it is one of the two `184` itself files under a heading that says so.

**Op-sourced and correct.** `composition_contracts_above_the_numeral`,
`a_primitive_for_every_position_a_bare_number_would_take`,
`the_surface_expressible_as_contracts_before_anything_implements_it`. Op is not outside the canon and
never was, so citing `INTENTS` is the right citation and not a defect.

## 4. What the corpus sweep found, and it is a negative result

**The panel's own corpus does not contain the enumeration `184` expected it to.**

Filtering 1590 occurrences of `consumer` across 165 markdown files down to need-shaped language leaves
78 lines, and **almost every one is the panel reasoning about a hypothetical consumer rather
than a consumer stating anything**: "whether any consumer wants the value keying", "a consumer wanting
both on one column", "whether any consumer needs to abstract over schedules". Those are the panel's own
open questions with the word `consumer` in them.

The handful that are genuine are the panel **deriving** a need on a consumer's behalf, and two are worth
the reading. `42` names three consumers and what each wants from a law layer, and proposes the
canon-shaped sentence for it. `67` derives that "the tropical consumer needs the adaptation laws, and
every reduction that has them supports min-plus, at every width, without a sweep."

**Both belong in `proposal` and neither belongs in `obligation`**, because an obligation is the
consumer's own terms and these are the panel's. That is the same distinction my `197` drew from the
other side, and the sweep confirms it: **the corpus is full of derived requirements and empty of stated
ones.** So the fifth thing I told the next reader was worth doing and the answer is that there is
nothing there. That is a real result and it retires an owed job rather than opening one.

## 5. Two consumers were never read, and both say things

**`kolli` names arvo on 96 lines across 17 files under `mock/`.** `184` recorded it as a gap: "kolli
names arvo nowhere in its design". `191` reported the gap resolved: "kolli names arvo on zero lines
anywhere under `mock/`, **measured with a control**." Both are wrong. The files include kolli's
top-level `mock/DESIGN.md.tmpl`, four per-crate designs, and a design round whose filename is
`202607270230_topic.arvo-everywhere-no-bare-usize.md`. Every one landed 2026-07-27, before either seat
looked.

**A control that reports zero on a tree with 96 matching lines did not fire**, and nothing in `191`'s
account would tell a reader that. I cannot say from here which of the two directions it failed in.

What kolli states, quoted:

> A width is an exact-width arvo value, so the bound is stated rather than commented, and a subtraction
> that would go negative saturates instead of wrapping a cramped screen into an enormous one.

> The numerics come from arvo, the workspace's numeric substrate, and every crate here takes it. **arvo's
> unstable machinery stays inside arvo, so a crate naming these types needs no feature gates of its own.**

**And it carries a quote of op's that the obligation derived from him does not have.** In that design
round, on whether the contracts crate keeps its bare `usize`:

> We want arvo there. **No bare usize other than in const generics for smoother and more ergonomic api,
> and even there, only when truly painful otherwise.**

`a_primitive_for_every_position_a_bare_number_would_take` carries no exception. **Op stated one**, twice
bounded, in a consumer repository, and the demand-side sweep never saw it because that consumer was
recorded as silent.

**`tarina` names arvo on 41 lines and was never read by anybody.** It states that arvo's algebra "is
still the intended destination. It is not adopted until its two-layer shape is understood well enough to
adopt correctly", and reports why: "arvo carries **two** grade counts at two layers... Tarina adopted one
and did not notice there were two." **That is a consumer reporting that arvo's design misled it**, which
is a statement about legibility and lands directly on op's `181` bar.

It also states, independently and in its own canon: "A fold may be reordered or partitioned exactly
where its stage's monoid is commutative. Where it is not, the fold is ordered and stays sequential."
**That is the reassociation precondition I filed in `197` from `35`'s measurement, reached from the
demand side by a consumer that has never read this panel.** It says commutative where `35` measured
associativity, and the difference is worth somebody's attention rather than mine.

## 6. What changes

**One row is wrong in the way two already corrected ones were.** `a_sparse_adjacency_a_plan_can_be_built_on`
should ask for the structural analyses over a CSR the consumer supplies, not for the CSR. Three of five
plan-chain obligations have now been found to be the crate list rather than the need, which is a rate
worth stating plainly: **the plan-chain half of the demand side was derived from one sentence of a
dependency list, and the majority of it did not survive being checked against the consumer.**

**One row understates and one misfiles.** The bit-set row is missing "all scheduler ops = single-instruction
bitwise" and carries two operations nobody asked for. The ordering row's fourth item is the consumer's
step 5, which the consumer attributes elsewhere.

**One obligation is owed an exception clause from op's own words**, section 5.

**Two candidate obligations, quoted rather than paraphrased**, and I am naming rather than filing them
for the reason section 7 gives:

- **arvo's unstable machinery does not leak, so a consumer naming its types needs no feature gates of its own.** Consumer: kolli, stated at `mock/DESIGN.md.tmpl:110`. Checkable, load-bearing, and nothing in the registry is about it.
- **arvo's algebra is legible enough that a consumer adopting it does not adopt half of it by accident.** Consumer: tarina, which did exactly that and says so at `mock/research/canon/03-algebra.md:236`.

**And four rows change in nothing but provenance**, which is a result: the demand side was not wholly
wrong, it was wrong in one identifiable half, and rewrite cost is real.

## 7. The fix has a shape, and the schema already has the pattern

**An obligation must carry the consumer's words, not a pointer to a file that carries a paraphrase.**

The `ruling` namespace already solves this exact problem: `says` holds a faithful restatement and
`quote` holds the verbatim, marked internal because a table cell is the wrong shape for a paragraph. It
exists because a row claiming somebody's authority with only somebody else's restatement behind it is
"mechanically indistinguishable from one that was invented", and `rulings_with_no_verbatim` is the arm
that reports it.

**`obligation` has no `quote` and no such arm**, and it has now produced three inventions of precisely
that kind. A `quote` field, plus the consumer file and line in a note, would make every one of these
rows checkable against the consumer **without needing a cross-repo citation the reference system cannot
express**. The words travel even though the address does not.

**I have not filed the two new obligations, or repaired the sparse one, for that reason.** Filing them
today means putting a verbatim quotation into `why`, which is what the spectral row did and is the thing
that went wrong. The schema change is small, it mirrors a pattern already in the file, and it wants a
second reader; then ten rows get their quotation and two get written properly.

## 8. Whether they should exist, which the brief put in scope

**Unchanged from `197` and I read `I11` again rather than my own summary of it.** Op names the algorithm
crates as the main selling point and the composition contracts beside them. The five should exist.

**What this pass establishes is narrower and is the position the brief said was available**: they exist,
and three of five were derived from a crate list rather than from a need. **Existence and derivation are
different questions and the answers are different.** Of the ten drawn from consumers, four are right as written, two are right and
understated, three name a crate where the consumer named a need, and one has no
consumer behind it at all.

## 9. What I did not do

Wrote no registry row: no obligation added, none reworded, no provenance repointed. Section 7 is why,
and it is a schema call rather than a row call. Did not read the 2080 arvo-naming lines in
hilavitkutin's `mock/` in full; I read the plan chain, the four numeral-facing crates and the build
crate, which is where the thirteen come from, and 2080 lines is its own dispatch. Did not chase which
direction `191`'s kolli control failed in. Cloned the four consumers into this worktree's ignored build
directory rather than reaching into the sibling clone, which is what `197` said it would do differently.

**And my own classifier shipped the defect it exists to find.** Its first version matched `hilavitkutin`
as a substring, which occurs inside the anchor text `#what-hilavitkutin-asks-for` on a citation whose
target is `184`, and it reported 8 of 13 obligations as consumer-cited. The true figure is zero. **The
wrong answer was the flattering one**, it was one grep from being believed, and the script now carries
the defect in its header rather than a corrected number with no history.
