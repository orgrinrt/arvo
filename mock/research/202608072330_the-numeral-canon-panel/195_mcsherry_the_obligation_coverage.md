# 195. What the canon owes its consumers, and what it currently pays

## The gates

**Canon gate: passed.** Op at `181` sets the bar this measures: the canon must be "exhaustive enough that
a full design and then a full impl of everything can be done based on it". `mockspace.toml` declares
`obligation` on `ruling` and on `proposal` and states the counting rule in the field description. Nothing
here writes another namespace, another field, or a row.

**Test gate: passed, 66 passing, no ignores.** I read the bodies of the arms touching this surface. There
is no check on obligation edges beyond slug resolution, so **the checker constrains nothing about whether
an edge is true**, and every judgement below is mine and is argued rather than enforced.

---

## 1. The table

```
$ 195_probes/obligation_coverage.sh

  OBLIGATION                                           STATE     REACHED BY
  set_operations_over_a_fixed_size_bit_set             NOTHING   -
  ordering_a_directed_acyclic_graph                    NOTHING   -
  a_sparse_adjacency_a_plan_can_be_built_on            NOTHING   -
  a_spectral_partition_of_a_dependency_graph           NOTHING   -
  a_cost_dynamic_program                               NOTHING   -
  a_platform_sized_unsigned_integer_at_an_api_position NOTHING   -
  an_exact_width_container_a_consumer_can_alias_and_pin met       ingest_is_the_consumers_and_the_c_abi...
  a_content_hash                                       NOTHING   -
  debug_output_from_every_numeral_shape                NOTHING   -
  a_build_flag_that_changes_float_semantics            NOTHING   -
  composition_contracts_above_the_numeral              proposed  five proposals

    met 1   proposed 1   nothing 9   of 11
```

**Met is one and it is one half of one.** `proposed` is one, now carrying five proposals where it carried
two. Nine are reached by nothing, and the count is worse than it reads, because five of the nine are the
algorithm surfaces op's own `I11` calls arvo's main selling point:

> our main selling point are the algo crates that hilavitkutin, vehje, pretty much every single repo and
> project I have, downstream, use. As well as the contracts for things that compose to bigger units than
> just numerals alone.

Both halves of that sentence are obligations. **The second is the one obligation with five proposals
against it. The first is five obligations with nothing.**

**The counting rule is not mine and it is the line the whole report turns on.** `mockspace.toml` on the
field: a proposal alone "does not meet one: a proposal is proposed rather than met, and reporting it
otherwise closes a gap op has never seen." So a single number here would be four different lies depending
on which way you rounded, and there is no honest way to write "the canon covers two of eleven".

The counter has a control. Planting one ruling edge and one proposal edge against two obligations that
have neither moves both into the right columns: `met 2 proposed 2 nothing 7`.

## 2. What I wired, and why each

**Four edges.** One ruling, three proposals.

**`ruling::ingest_is_the_consumers_and_the_c_abi_is_where_it_ends_up` → `an_exact_width_container_a_consumer_can_alias_and_pin`.**
The obligation names two halves for itself, in its own `why`: "the demand is both halves: the exact width
in memory and a conversion the consumer writes at the boundary." Op answers the second directly:

> Data entering from outside the program is the consumer's boundary, not arvo's. Everything written here
> ends up in a C ABI sooner or later, and the writer handles it by defining their APIs with arvo's shapes
> and generics. **arvo may ship casting and conversion helpers** and may not use them in place of the
> consumer.

That is the conversion-at-a-consumer-boundary half, decided, by op, with what arvo ships for it named.
**The exact-width-container half is reached by nothing**, and section 3 says so. I wired it because the
obligation itself splits into two and one is answered; I would not defend reading the edge as the whole
obligation discharged, and the schema gives me no way to say half.

**Three to `composition_contracts_above_the_numeral`**, joining the two from `187`:

- **`the_format_concept_carries_three_things_upward_and_compositions_owe_their_own_laws`.** The strongest of the five and it should probably lead them. Its `because` and the obligation's `need` turn on the same word: the obligation asks for contracts "so that what a chain of operations guarantees is **expressible at all**", and the proposal's reason is that a layer not handed the width algebra, the named adaptation and the exactness predicate "**cannot state its own error behaviour at all**". It then names what a composition owes on top. That is the contract, enumerated.
- **`a_composed_expressions_region_is_never_inherited_from_its_parts`.** A rule governing what may be concluded about a unit bigger than a numeral, with an exact counterexample rather than an illustrative one: unsigned saturating addition associates on every triple at eight bits and its composition with the matching subtraction fails on 82.7484 percent at the same coordinates.
- **`configuration_is_not_composition_and_a_composite_is_a_primitive`.** `two_experts`, reached blind twice. "every contract written for a primitive applies to a composite unchanged", plus the two things a construction carries of its own.

**The first and second say the same thing from two sides** and neither cites the other: one says compositions owe their own laws, the other says a composed region is never inherited so the composition needs its own derivation. Worth recording as a convergence the corpus has not noticed it has.

## 3. The nine, and what each is nearest

**The distinction the brief asks for, "nobody wrote it down" against "nobody did the work", separates
these nine cleanly, and it does not fall where I expected.**

### 3.1 Five where nobody did the work, and the corpus reaches them only sideways

`set_operations_over_a_fixed_size_bit_set`, `ordering_a_directed_acyclic_graph`,
`a_sparse_adjacency_a_plan_can_be_built_on`, `a_spectral_partition_of_a_dependency_graph`,
`a_cost_dynamic_program`.

Searched over all twelve registry files and every narrative field, with terms taken from each
obligation's own `need` sentence written down before the search ran. **`Fiedler`, `topological order`,
`sparse`, `adjacency`, `CSR`, `Laplacian` and `content hash` occur in no registry row anywhere except
the obligations that ask for them.** The control plants a record carrying four of those phrases and it
surfaces under all of them, so the zeros are about the corpus.

The near misses are all false and each is a different word doing double duty:

- **`bandwidth`** hits once, in `which_carrier_the_packing_claim_is_about`, where it is memory bandwidth.
- **`union` and `intersection`** hit eighteen times, every one of them the union of demand sets in a join semilattice or the intersection of shape families and convergence sets. None is a bit-set operation.
- **`budget`** hits nine times and every one is the **const-eval** budget, not a cost budget for grouping work.
- **`partition of`** hits three times, partitioning a rounding axis and an arithmetic column.

**What is genuinely near, and it is `191`'s finding rather than mine, is that the corpus reaches these
constantly as test instruments and never as its own subject.** `retirement.toml`'s
`dl_interior_wrapping_with_a_reserved_absorbing_top` measures shortest path on a DAG wrong under a
proposed overflow policy and kills the policy on that evidence, naming min-plus and the tropical algebras.
`dl_gate_algorithm_crates_on_addassoc` retires gating the graph, combinatorial and spectral crates on an
associativity fact. Both are about the algorithm surface. **Neither delivers any of it**, and neither
could carry an edge in any case: `retirement` has no `obligation` field, which is a schema fact worth
knowing before somebody counts these as reach.

**So the answer for these five is: nobody did the work, and the work has never been the subject.** Not a
filing problem. `191` section 0.3 states it exactly, and I confirmed it independently from the other
direction: I started at the obligations and arrived at the same emptiness.

### 3.2 Two where nobody wrote it down, and the material is close

**`a_platform_sized_unsigned_integer_at_an_api_position`.** Two rows are near and neither delivers.

Op's `the_operating_constraints_are_intents_and_rules` requires "public API positions using the stack's
own primitives rather than bare integers, floats, bool or **usize**". That **presupposes** such a
primitive and mandates its use. It is a stronger reason the obligation must be met and it is not a
meeting of it, and I declined the edge on that ground.

`each_choice_in_the_sequence_has_an_owner_and_a_resolution_time` says "a platform-width numeral is a
target-indexed family of formats whose exclusion grounds apply only to dependence that survives to
runtime". That is a real advance: it removes the objection that would have excluded such a type from the
format concept, since `a_format_is_identified_by_its_ambient_domain_and_its_representable_set` says a
value set depending on other data "is not a format but storage". **What it establishes is that arvo may
have one**, and it says nothing about unsignedness, about a public error position, or about the errno and
GetLastError range the obligation names. Necessary condition, not delivery.

**And the question is open in the registry**: `what_a_platform_width_type_is` (Q26) asks what kind of
thing it is, with four options, `decider = "panel"`. I reported in `187` that its first option is written
in the format proposal's words while the binding-time proposal disputes the application. **That dispute is
what stands between this obligation and an answer**, and it is the cheapest of the nine to close.

**`an_exact_width_container_a_consumer_can_alias_and_pin`, the other half.** The conversion half is met
above. Nothing states the container: `exact width` and `exactly declared` return zero, and `alias` returns
only rows about type aliases in the width-surface question and retirements about preset markers. The
nearest is `the_lens_degenerates_to_an_ordinary_value_at_sole_occupancy`, which says the realisation is a
placement of bits with a carrier, an offset and a width, and degenerates to a standalone type exactly
where its focus is the sole occupant of its allocation. **That is the shape of the thing the obligation
wants**, described as a realisation rather than as a container a consumer may name and alias.

### 3.3 Two where the corpus has a position and the position is not a delivery

**`debug_output_from_every_numeral_shape`.** Zero hits on `debug output`, `core::fmt`, `no_std` outside a
topic row. `no alloc` hits op's operating constraints, which is **the constraint the obligation must be
met under** rather than a meeting of it: the obligation exists precisely because no-alloc closes the
ordinary route. Nobody wrote it down and nobody did the work.

**`a_build_flag_that_changes_float_semantics`.** Zero hits on `cfg`, `fast math` and `float semantics`.
This one is different from every other unmet obligation, and the difference should reach op rather than
sit here: **the corpus's position on it is against.** The obligation's own `gap` records the measurement,
that one source one flag apart gives one type name two policies and a compile-time check cannot catch it,
and `184` declines to endorse the surface while recording the dependency. `191` section 3.3 proposes an
unwritten row saying a bit-exact reproducibility demand and a build flag are in conflict, and that both
are obligations **from the same consumer**: this one and `a_content_hash`.

That is a genuine internal inconsistency in the demand side rather than a gap in it, and no row says so.
It is the one place where "the canon does not meet this" may be the right outcome rather than a shortfall,
and the canon currently has no way to record that, because an obligation has no state and the only thing
that can speak about it is a row that meets it.

### 3.4 One where the material may exist and nobody has looked

**`a_content_hash`.** `hash` returns one hit, a bench note. `digest` returns one retirement,
`dl_datum_keyed_digest_masks_to_the_fields_width`, about a digest masking to a field's width in the
deleted crate tree. `stable across` returns nothing.

I am marking this one differently from the other eight because **the deleted tree had one**, per the
consumer's own account in `184`: a 28-bit value aliased by the consumer and pinned to 32 bits on disk. So
the question is not whether the work was done but whether anything about it survived the tier being
nuked, and I could not establish that from the registry. Somebody with the crate history can.

## 4. Obligations I think are wrongly worded

Three, and one of them matters.

**`a_content_hash` is not an obligation as written.** "A hash of content, stable across sessions" names
no width, no collision property, no domain, and no consumer-visible type. It is a category. Every other
row here states a property something could be checked against; this one could be met by anything that
returns bytes. Its `why` is better than its `need` and carries the real content: the consumer keys a
persistence cache on it, so what is owed is stability across sessions **and** whatever collision bound a
cache key needs, which nobody has stated.

**`a_build_flag_that_changes_float_semantics` states a mechanism where every other row states a need.**
The obligation namespace's own field description says "The need, never the mechanism that serves it", and
this row's `need` is a cfg. The need underneath is the consumer's, and `184` names it: the build system
wants to select float semantics for the whole compilation. Written as a need it would be servable in ways
a cfg is not, and the panel's measured objection is specifically to the cfg. **As worded, the obligation
cannot be met except by the thing the corpus argues against.**

**`a_spectral_partition_of_a_dependency_graph` may be scoped wrong rather than worded wrong**, and this
is `191` section 4's finding, which I checked and agree with. The consumer's design says "Spectral
partitioning via an **engine-local symmetric Laplacian** over arvo-spectral's k-way partitioning": it
builds the Laplacian itself and asks arvo only for the partitioning. The obligation as written asks arvo
for "a Fiedler partition, splitting a dependency graph into groups", which is the whole step including the
graph-shaped part the consumer says it does. **If that is right the obligation overstates what is owed**,
and what remains is an eigensolver needing `Sqrt` and `Recip` that nothing else in the demand side wants.

## 5. Whether the eleven are the right eleven

**No, and `184` says so about itself**: its sweep read three consumer repositories at one level, and "this
panel's own corpus was not swept, and it is where the majority of the enumeration will come from."

`191` section 0.4 measured what the level cost: vehje's top-level design names arvo on 3 lines and its
eleven per-crate designs name it on 19 more, and `arvo::UWire<N>` is filed by `184` as a gap with "no
statement of what it is for" while two consumers state the use and the same mechanism at length. `191`
proposes five obligations from that material and I am not going to restate its list as mine.

**What I can add, from the registry rather than from the consumer documents, is two op-sourced obligations
nobody has written.** Both come from rulings already in the canon, which is the strongest provenance
available and is the same route `composition_contracts_above_the_numeral` took from `I11`.

**Twelfth: a stack primitive for every public-API position where a consumer would otherwise write a bare
integer, float, bool or usize.** Op's `the_operating_constraints_are_intents_and_rules`, at
`rung = "in_force"` and lint-enforced, requires exactly that of consumers. **The obligation list carries
one instance of it and not the rule.** `a_platform_sized_unsigned_integer_at_an_api_position` is the
`usize` case; there is no obligation for the `bool` case or the float cases, and `grep -c 'bool'
obligation.toml` returns 0 while the stack's own vocabulary lists `Bool`, `FastFloat` and `StrictFloat`.
A rule in force whose consequences are enumerated once out of four is a demand side that will keep
producing surprises.

**Thirteenth: arvo's surface must be expressible as trait contracts that hold their shape before any
implementation exists.** Op's `the_trait_contract_structure_is_a_primary_paradigm`:

> The trait contract based structure is a primary paradigm we uphold in future too. This allows for the
> shapes to stick even when the impls are wip, like hilavitkutin currently has

**This is a demand on arvo with a stated reason and a named live instance, and it is not in the obligation
list.** It is also, unlike every other row here, directly checkable against op's `181` bar: if the canon's
surface is contracts, a full design follows from it, and if it is not, the bar is not met however many
other obligations are. I declined to wire this ruling to
`composition_contracts_above_the_numeral` in `187` and I decline again, for the same reason: it says how
arvo expresses things, not that composition contracts exist. **What it wants is an obligation of its own.**

Both are op's words rather than a consumer's, and `obligation.consumer` admits `any` for exactly that
case, with `composition_contracts_above_the_numeral` as the precedent.

## 6. Two things I could not settle

**Whether a `retirement` reaching an obligation should be able to say so.** Three retirements are the
nearest thing in the corpus to three of the unmet obligations and none can carry the edge, because the
field is on `ruling` and `proposal` only. I do not think that is obviously wrong: a retirement records
that a route was closed, which is not a delivery. But it means `refsto` cannot distinguish an obligation
the corpus has never considered from one it has considered and closed a route to, and those want
different work. **Schema question, not mine.**

**Whether `an_exact_width_container_a_consumer_can_alias_and_pin` should read as met.** I wired the
ruling and the counter now says `met 1`. One reader will take that as the obligation discharged and
another will read section 2 and find half of it standing. **The count is honest only with the prose beside
it**, which is the third time across three dispatches that I have wanted a partial edge and not had one.
If the answer is that obligations should be split until each is atomic, this one splits cleanly along the
line its own `why` already draws.

## 7. What I would tell the next reader

1. **Take the table to op with `I11` beside it.** Five of the nine unmet are the algorithm surfaces he calls the main selling point. Q33's first option already writes down the scope arm ("the tropical semiring the algorithm crates compute in is described by the algorithm crates. Cost: the named selling point computes in something the canon does not cover"). **He has never been shown that this is where the canon stands.**
2. **Q26 is the cheapest unmet obligation to close.** Section 3.2. One dispute between two proposals, both already written, and the platform-width obligation moves.
3. **`191`'s five obligations, and my two.** Seven candidate rows, none written, all from material already in hand.
4. **Reword `a_content_hash` and `a_build_flag_that_changes_float_semantics` before anybody tries to meet them.** Section 4. The first cannot be checked and the second can only be met by the thing the corpus argues against.
5. **The build-flag conflict has no home.** Section 3.3. Two obligations from one consumer are in conflict and nothing in the registry can say so.

## 8. What I did not do

Wrote only the `obligation` field, on `ruling.toml`, `proposal.toml` and `proposal-the-later-topics.toml`.
Added no row, changed no other field, and did not touch `obligation.toml` where section 4 says three rows
are wrongly worded. Did not wire `arvo_is_a_library_and_the_value_composes_on_top` to anything, because
`184` derived four obligations from it and a row reported as meeting the obligation it generated is a
citation loop, which is the same call I made in `187`. Did not sweep the panel's own corpus for consumer
statements, which `184` names as owed and which is where most of the missing enumeration is.
`docs/PROPOSAL.md` and `docs/RULING.md` are in the commit because `cargo mock` regenerates them from the
registry.
