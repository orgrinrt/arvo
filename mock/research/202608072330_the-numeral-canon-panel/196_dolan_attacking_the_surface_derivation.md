# 196. Attacking the surface derivation

Phase two of the unit opened by `191`. This attacks `191` and owes it replacements.

The short form. `191`'s data is almost entirely correct and its central inference is wrong, and the
gap between those two sentences is one instrument. It measured whether `35`'s and `43`'s **figures**
occur in `mock/registry/` and concluded that the canon "records what the surface above the numeral
cannot be and says nothing about what it is". The registry's own `provenance` field says a figure was
never the transmission path: no row cites any member derivation's figures, rows cite consolidations
and anchors, and `35` and `43` between them are referenced by **seventy-six** panel files, sixteen of
which are direct provenance targets including the three highest-volume ones. `35` section 3.7 is in
the canon as a **law**, with a `holds` region, a `fails` region, a witness from two later independent
models, and a note naming `35:311` as the line where the qualifier was lost. A file cited by the canon
as the origin of a defect the canon corrects is not a file the port dropped.

And the fold-and-chain conflict that `191` reports as unreconciled, and that my brief asked me to
write the reconciling sentence for, **is already written**:
`proposal::the_multiplicative_guard_grows_linearly_and_the_saving_is_adaptation_fusion`, `two_experts`,
predicated over `chain_length` and `operation`, wired by an `answers` edge to Q11, whose `says` ends
"**so any accumulator statement derived from a capacity is an additive-only mechanism**". That is the
partition. There is no contradiction to dissolve. `191` did not find it because it searched for the
phrase `trip count`, which is the vocabulary of its own hypothesis, rather than for the declared axis
the corpus writes the result over.

What survives of `191` is sharper than what it claimed, and I set it out in section 3. What does not
survive is section 2.1's inference, section 2.3 entire, and one of the three options it puts to op.

---

## 0. What broke

Checked before the lens work, per the standing first task.

### 0.1 In my brief: the defect-class count

> "This corpus has eleven recorded classes of instrument defect and the last four were each caught by
> a control rather than by reading the code."

**It is ten.** `192_fog_the_rest_of_the_instruments.md:42` writes "the ten defect classes", and
`mock/checks/tests/a_probe_reads_the_tree_it_sits_in.rs:6` opens "The tenth instrument-defect class,
and the quietest of the ten." No file anywhere in the panel says eleven or names an eleventh; the
only occurrence of "eleven" in the three files after `191` is `192:145` counting probe files in
`47_probes`.

The second clause is not wrong so much as unsupported in the form given. The corpus does carry a
sentence of that shape, but it is about one instrument rather than about four classes:
`probe.toml:96`, "Three, all in earlier versions of this instrument, all found by adding the control
and none visible from reading the code." Nothing states a property of "the last four classes".

Small, and it is the same class `190` records five instances of: a count carried into a brief without
opening the artifact. It cost nothing here because the number was not load-bearing for anything I did.

### 0.2 In my brief: what it told me `191` says

The brief renders `191` section 2.3 as "A fold and a chain want opposite operators, discriminated by
the runtime trip count. A cross-topic proposal and `35` section 3.1 are both right and nothing
reconciles them." That is a faithful summary of `191`. It is not a faithful summary of `35`, and
carrying it forward is how the error propagates: see 2.3 below, where `35`'s own section **title**
contains the disjunct `191` drops and `35`'s own arm E compiles the shape `191` says a fold cannot
have.

### 0.3 In `191`: three claims that hold, checked because they are load-bearing

Stated first so the disagreements below are not read as a general attack on the file's care.

- **"Ten of the eleven obligations are reached by no row anywhere."** Holds. `grep -rn '^obligation = '
  mock/registry/*.toml` returns exactly two lines, `proposal.toml:214` and
  `proposal-the-later-topics.toml:457`, and both carry the same slug.
- **"tarina names arvo on zero lines."** Holds. `grep -rn arvo` over the whole clone excluding `.git`
  returns 0.
- **Both tarina quotations are verbatim.** `DESIGN.md:435-438` and `DESIGN.md:188-191` in the clone
  beside this one, opened rather than trusted.

### 0.4 In `191`: the figure instrument measures figure travel and is read as fact travel

The bound is not stated in `191`'s section 8 table, and it is the bound everything in 2.1 rests on.
`191_probes/which_of_35s_figures_survived.sh` asks "does the string occur anywhere in
`mock/registry/`". A result re-established later by a different seat, with a different instrument, at
different widths, reads ABSENT on that arm and is present in the canon. That is not a hypothetical
here: it is what happened to `35` section 3.7, and section 2.2 below opens the row.

### 0.5 In `191`: "the registry contains two results about composition"

Section 2.3's opening sentence. `196_probes/the_composition_surface_by_axis_not_by_phrase.sh` ARM 4
counts rows carrying `topic = "the_chain"`: **6 in `proposal.toml`, 1 in
`proposal-the-later-topics.toml`, 1 in `law-the-later-topics.toml`, 9 in `question.toml`, 20 in
`retirement.toml`.** Thirty-seven. Read charitably as "two results bearing on the operator question",
it is still wrong, because one of the rows it omits is the one that settles the operator question.

---

## 1. What I read, and what I did not

**Exhaustively.** `191` whole and its seven probe scripts and outputs. `190`. `35` sections 2, 3.1,
3.2, 3.3, 3.4, 3.6, 3.7, 3.8, 3.9, 3.10 and its probe listing; `35_probes/p1_fold_cannot_widen.rs`
arms D and E at source. `mock/registry/obligation.toml` whole. `mock/registry/law.toml` rows
`distributivity_of_multiplication_over_addition`,
`distributivity_of_multiplication_over_subtraction`, `the_saturating_exponent_absorbing_identity`,
`rounding_retraction_is_the_identity`. `mock/registry/proposal.toml` rows `absorption_decides_associativity_of_a_clamped_reduction`, `chain_laws_split_by_whether_a_lifting_theorem_exists`, `chain_accuracy_cannot_be_served_by_an_operator_closed_over_its_operand_type`, `a_coherent_reduction_needs_no_accumulator`,
`an_incoherent_clamped_addition_needs_the_exact_sum_width_less_one_bit` and `the_multiplicative_guard_grows_linearly_and_the_saving_is_adaptation_fusion`. `question.toml` Q11 whole. `retirement.toml` rows `dl_gate_algorithm_crates_on_addassoc` and `dl_interior_wrapping_with_a_reserved_absorbing_top`. `mockspace.toml`'s
`retirement` and `ruling` namespace schemas. `mock/checks/src/predicate.rs` and `src/shape.rs` whole,
and every test body in `mock/checks/tests/`. `hilavitkutin/mock/DESIGN.md.tmpl` lines 46 to 52 and
`hilavitkutin/mock/crates/hilavitkutin/DESIGN.md.tmpl` steps 1 to 13. `tarina/DESIGN.md` at the two
cited spans.

**Not read.** The panel's other files except those named above and `184`, `187`, `192`, `193`, `194`
at their headings. `43` at all, deliberately: `191`'s claim about it is a claim about the registry
rather than about `43`, and section 2.1 below tests it against the registry. `35` sections 4 to 10.
The design_rounds tree. **The deleted crate tree, deliberately.** One of my probes' arms surfaced a
build-artefact path naming a cargo checkout of it; I excluded `/target/` from that arm and did not
open the file, and I flag the hazard in section 6.

**Cloned already, read read-only, no writes.** `hilavitkutin`, `tarina`.

---

## 2. The attack

### 2.1 The transmission path was never a figure, and the corpus says so in a field

`191` section 2.1 is the file's own nomination for "the finding I would put in front of op before
anything else". It is:

> "the panel looked at it twice, and the canon retains the two negative results and none of the
> positive ones."

The registry carries `provenance` on every row, typed `ref[]` and required. So the corpus states
mechanically where each of its sentences came from, and that is a better instrument than a string
search for the same question.
`196_probes/how_35_and_43_actually_reached_the_canon.sh`, controls passing:

- **ARM 2.** Neither `35` nor `43` nor either probe directory appears in any provenance. **`191` is
  right about this**, and the arm was built so the opposite outcome would print.
- **ARM 1.** Ninety-seven distinct numbered panel targets *are* cited. The list includes member
  derivations (`109_bellard`, `154_kiselyov`, `168_mcsherry`, `47_wingo`, `97_dolan`, `07_orchard`),
  attacks (`108_lamport`), checks (`17_leroy`), and probe directories from files `06`, `07`, `08`,
  `40`, `47`, `56`, `57`, `60`, `62`, `63`, `73`, `80`, `82`, `93`, `94`, `97`, `98` and upward.
  **Files earlier than `35` are cited. Files later than `35` are cited. Probe directories on both
  sides are cited.** So "member files are not cited" is not the rule; `35` and `43` are the
  exception.
- **ARM 3, which is the one that matters.** Seventy-six panel files reference `35` or `43`. Sixteen
  of those carriers are themselves registry provenance targets:

| carrier | ->35 | ->43 | registry citations of the carrier |
|---|---|---|---|
| `DROPLIST` | 2 | 0 | 97 |
| `OPTIONS` | 12 | 7 | 76 |
| `63_spj_consolidation_the_format_concept` | 7 | 2 | 46 |
| `106_giesen_consolidation_the_strategy_axis` | 2 | 0 | 39 |
| `90_giesen_consolidation_derived_algebraic_laws` | 1 | 0 | 38 |
| `173_leroy_the_canon_candidate_for_the_chain` | 0 | 16 | 18 |
| `74_giesen_consolidation_the_number_system_concept` | 1 | 0 | 28 |
| `146_leroy_the_canon_candidate_for_the_strategy_object` | 0 | 5 | 13 |
| `53_leroy_consolidation_the_container_derivation` | 7 | 5 | 1 |
| `172_leroy_formalising_the_chain` | 0 | 4 | 1 |
| `168_mcsherry_the_chain_derived_cold` | 4 | 20 | 1 |
| `97_dolan_the_strategy_space_attacked` | 20 | 0 | 2 |
| `AGREEMENTS`, `136_leroy`, `138_leroy`, `108_lamport`, `47_wingo` | | | |

And outside the cited set, `42_willsey_the_law_layer` references `35` **forty-six** times,
`44_arntzen_the_two_outputs_re_derived` references `43` twenty-three times.

**So the shape is: member derivation to consolidation or anchor to registry row.** A figure survives
that path only when a consolidator chose to quote it, which is a fact about compression. `191`
measured compression loss and reported canon ignorance.

**The honest version of `191`'s finding, which is real and which I keep**: `35` and `43` are the two
substantial member derivations in this panel with **no direct citation of any kind**, neither file nor
probe directory, while forty-seven other probe directories are cited by row. That is worth a question.
It is not evidence that their results were dropped.

### 2.2 `35` section 3.7 is in the canon as a law, and the canon cites `35` by line

`191` sampled fourteen figures from `35` sections 3.4, 3.5 and 3.5a. `35` states **ten** requirements,
3.1 through 3.10. `196_probes/which_of_35s_ten_sections_reached_the_registry.sh` widens the sample,
same instrument shape, four controls including `191`'s own two positives and a reproduction arm that
confirms `63 of 63` is absent so we are reading the same tree.

The interesting row is 3.7. Its figures (`6 of 33`, `87.5`) come back one absent and one a false
positive in an unrelated row, which my probe now records against itself. And the result is in the
canon anyway, at `law.toml:145`:

```
id = "distributivity_of_multiplication_over_addition"
holds = [ W in 3..=8, F = 0, unsigned, policy in {wrap, saturate}, {add, mul}, arity in {2,3},
          threads = 1, target features any ]
fails = [ W in {6,7}, F = 0, signed, saturate, {add, mul}, arity = 3, threads = 1, tf any ]
witness = "At signed saturating and `F = 0`, two independently written models measure the law
           failing: `93_probes/p7` gives 47.72% of triples at `W = 7`, and `97_probes/p2` gives
           34.52% at `W = 6`. The mechanism is that a one-sided clamp is a congruence and a
           two-sided one is not."
```

And the note:

> "`F = 0` is necessary and it is not sufficient, and the unqualified form of this sentence is false.
> The strategy consolidation records that this is the **third** time the qualifier has been lost from
> the sentence in this panel's own history: it appears unqualified at **`35:311`** where it
> originated, at `94:887`, and in the standing workspace rule `arvo-always-optimal-internals.md`,
> which was a live licence to emit a wrong rewrite until it was corrected during that unit."

`35:311` reads, verbatim: "**Multiplicative associativity and distributivity hold exactly at `F == 0`
and fail everywhere else.**"

**The canon read `35` closely enough to cite the line number where its sentence went wrong, and to
correct it with two later models `35` did not have.** Against that, "the port dropped almost every
positive result" cannot stand.

### 2.3 The fold-and-chain conflict does not exist, and its reconciliation is written and wired

This is section 2.3 of `191` and question 3 of my brief. My brief asked me to write the sentence or
show why it cannot be written. Neither: **it is written.**

**First, the misquote that generates the conflict.** `191` renders `35` as:

> "`35` section 3.1, from a contract test that does not compile: a fold's operation **must be
> closed**, because a loop-carried accumulator has exactly one type."

`35`'s section 3.1 is titled: "**A fold's operation must be closed, or its trip count must be
static**." The disjunct is dropped in the paraphrase, then reintroduced two paragraphs later as
`191`'s own reconciliation. That is recoverable. What is not is the arm table `35` prints directly
under that title, which `191` quotes as "eight arms with two positive controls" without reading arm E:

```
D  closed op in a loop over a runtime-length slice             compiles
E  closed op into a separately named wider accumulator         compiles
```

And `35_probes/p1_fold_cannot_widen.rs:253-276` at source:

```rust
pub fn arm_e(xs: &[Num<W1>]) -> Num<W4> {
    let mut acc: Num<W4> = Num::new(0);
    for x in xs { let widened: Num<W4> = (*x).widen_into(); acc = acc.cadd(widened); }
    acc
}
```

The elements are `Num<W1>`. The accumulator is `Num<W4>`. **That operator is not closed over its
operand type.** It is closed over the accumulator type, which is a different quantification, and
the slug of the row `191` cites is `chain_accuracy_cannot_be_served_by_an_operator_closed_over_**its
operand type**`. The two sentences say the same thing about different type variables and `191` reads
them as opposites because the word "closed" is unquantified in one of them.

**Second, the reconciliation, already in the registry.** `proposal::the_multiplicative_guard_grows_linearly_and_the_saving_is_adaptation_fusion` (`proposal.toml:1305` today, and see 2.8 on why that number is not the citation):

```
id       = the_multiplicative_guard_grows_linearly_and_the_saving_is_adaptation_fusion
kind     = finding      sentence_kind = measured      standing = two_experts    topic = the_chain
says     = "For multiplication no bounded closed form exists: the exact guard grows linearly in fold
            length at `(n-1)F` bits with no logarithmic closed form. ... So any accumulator statement
            derived from a capacity is an additive-only mechanism."
predicate= [ W = 4, F = 3, both signednesses, operation = mul, chain length in 2..=5,
             rounding in {truncate, floor, round to nearest even} ]
answers  = ["what_a_numeral_guarantees_to_a_fold"]
because  = "... The second parameterised the rounding rule, predicted the fusion from the schedule
            algebra in the probe header before running, and refuted the constant at fold length five"
```

`answers` points at **Q11**, which `question.toml:220` records as "Added from `35`, which calls it the
most valuable single item it found", and whose note reads: "`60` later undercuts the
accumulator-derivation option as an additive-only mechanism, since the multiplicative fold needs
linear growth with no closed form against **addition's width-plus-log-of-capacity**."

That last clause is `35` section 3.2's formula, `acc_width(W, C) = W + ceil(log2 C)`, named in the
canon, as the thing the multiplicative case is being contrasted against.

**Third, what the axis actually is.** Not the trip count. Put the four predicated rows beside each
other, which is what `196_probes/the_composition_surface_by_axis_not_by_phrase.sh` ARM 2 does by
reading the declared axis `chain_length` rather than by guessing a phrase:

| row | operation | chain_length | what the width is a function of |
|---|---|---|---|
| `chain_accuracy_cannot_be_served_by_an_operator_closed_over_its_operand_type` | mul | any | position, linearly, no closed form |
| `the_multiplicative_guard_grows_linearly_and_the_saving_is_adaptation_fusion` | mul | 2..=5 | position, `(n-1)F`, no closed form |
| `a_coherent_reduction_needs_no_accumulator` | add | 2..=8 | nothing; the format width suffices |
| `an_incoherent_clamped_addition_needs_the_exact_sum_width_less_one_bit` | add | 2..=8 | a bound, the exact-sum width less one |

**The split is the operation, not the composition shape.** A multiplicative chain and a multiplicative
fold behave the same; an additive fold has a closed form and an additive chain has the same closed
form with the length in place of the capacity. Whether the count is static changes only whether you
write `n` or a capacity `C`; it does not change whether a closed form exists.

**Fourth, why `191`'s search could not find any of it.** It reports: "`trip count` appears in
`question.toml:222` (Q11's keyword list), in `retirement.toml:114`, and in one unrelated row. No
`says`, no `claim`, no `statement` carries it." ARM 1 of my probe reproduces that exactly: two
occurrences, both in `keywords`, zero in any claim-bearing field. **The search is correct and it is a
search for the vocabulary of `191`'s own hypothesis.** A reconciliation written over `operation` and
`chain_length` is invisible to it and indistinguishable from one nobody wrote.

### 2.4 The demand side is the deleted crate tree with the names filed off

`184` opens by stating its method: "every row below is written as a need and none of them names an
arvo crate. Where a consumer's document names only the crate and never the use, no row is written and
the gap is recorded at the end." Every row honours the letter of that.

Its source for the plan-chain half is one sentence, which `184` quotes,
`hilavitkutin/mock/DESIGN.md.tmpl:50`:

> "Each step names its foundations crate dependency: `arvo-bitmask` for set ops, `arvo-graph` for
> DAG / RCM, `arvo-sparse` for CSR DependencyGraph (bench-locked canonical at all N), `arvo-spectral`
> for the Fiedler partition step, `arvo-comb` for the cost DP."

`184` then writes "Five needs." `196_probes/are_the_plan_chain_obligations_the_crate_list.sh` ARM 2,
with a splitter shown by its control to match an invented `arvo-zzz for ...` clause and therefore not
keyed on the five real names:

```
arvo-bitmask` for set ops                    set_operations_over_a_fixed_size_bit_set
arvo-graph`   for DAG / RCM                  ordering_a_directed_acyclic_graph
arvo-sparse`  for CSR DependencyGraph        a_sparse_adjacency_a_plan_can_be_built_on
arvo-spectral` for the Fiedler partition     a_spectral_partition_of_a_dependency_graph
arvo-comb`    for the cost DP                a_cost_dynamic_program
```

**Five clauses, five obligations, in order.** The canon's demand side, on its plan-chain half, is one
sentence of a consumer's dependency list with the crate names struck out. That is the decomposition my
brief told me not to reason from, sitting in the namespace built to be independent of it.

**And one of the five is a wrong paraphrase.** ARM 3. The obligation reads:

> `need = "A dynamic program over costs, deciding how to group work under a budget."`

Every statement of that crate's purpose in the consumer, build artefacts and imported prior art
excluded:

```
research/seed/foundations.md:24   | arvo-comb | Matrix-chain ordering DP, greedy constrained
                                    interval grouping, two-level greedy bin-packing |
research/seed/plan.md:106         Step 8, fiber grouping. Greedy at 10 or fewer ops
design_rounds/202605101036/...:31 8. Fiber grouping: greedy or matrix chain DP for >10 ops
research/2026_05_05_doc_audit.md:193  `matrix chain` here is the algorithm name
```

Matrix-chain ordering minimises scalar multiplications over parenthesisations. **There is no budget in
it.** The budget in the plan chain is step 9, morsel sizing, `window = (L1_usable / Σ write_sizes)
.clamp(...)`, which is a formula and a different step, and which `191` files as class D. The
obligation welds step 8's algorithm to step 9's budget and calls the weld one need. It is a
paraphrase, of a crate name, and it is wrong.

`191` then builds its class B on it: "Step 8's grouping and the cost dynamic program, which is a
budgeted optimisation over min-plus or a knapsack." Matrix-chain ordering is neither. "knapsack" is in
the obligation's `keywords` and in no consumer document I found.

### 2.5 One stage falls through both, and it is the only stage that is a strategy

ARM 4 walks the twelve canonical stages against `191` section 2.2's class assignments.

```
 1 build_dag        A      7 spectral         C
 2 topo_sort        A      8 fiber group      B
 3 upward_rank      B      9 morsel size      D
 4 waist            A     10 phase config     -- UNASSIGNED
 5 RCM              A     11 column classify  A
 6 block-diag       A     12 dirty masks      A
```

Step 10 is in none of `191`'s five classes. It is also the one stage the dependency sentence names no
crate for, which is why `184` has no obligation for it either. **Both files inherited the same blind
spot from the same sentence**, and the sentence is the deleted tree's.

What step 10 is, in the consumer's own words:

```
research/seed/execution.md:143   Per-phase config selection picks among the grouper's MAX_FUSE,
                                 BALANCED, and MAX_SPLIT configs independently per phase
research/202606061000_...:15     one of the per-phase STRATEGY modes (MAX_FUSE / BALANCED /
                                 MAX_SPLIT / sequential)
design_rounds/202604200227/...:23  `PhaseStrategy`, per-phase config (MAX_FUSE / BALANCED / ...)
```

**A selection among a named finite set of policies, made per phase, called a strategy by the consumer,
with a type named `PhaseStrategy`.** That is the shape of arvo's strategy axis, in the consumer's plan
chain, at the one step nobody has a row or a class for. And `191` section 2.4 concludes, without it,
that "the strategy axis does not survive above the numeral".

I want to bound this honestly, because it is an analogy and not a measurement. `PhaseStrategy` selects
a *grouping policy*, not an arithmetic realisation, so it is not arvo's `Strategy` and I am not
claiming it is. What it is, is a compile-time-or-frame-time selection among named alternatives whose
members trade the same axis arvo's markers trade, sitting in the same chain, and it is the one datum
that would test `191`'s section 2.4 rather than illustrate it. Nobody has looked at it.

### 2.6 Three words that mean two things each, and one of them is the word the surface needs

Not a `191` defect. A canon defect `191`'s subject walks straight into.

- **absorption.** Twenty-nine occurrences. At least three senses. (i) The tropical absorbing top,
  `TOP + x == TOP`, which is `35` 3.4 and what a min-plus fold needs: present in
  `retirement::dl_interior_wrapping_with_a_reserved_absorbing_top` and as an option at `question.toml:77`. (ii) Reduction absorption, "reducing
  before combining agrees with reducing after", `proposal::absorption_decides_associativity_of_a_clamped_reduction`, a `measured` `two_experts` row
  over 4248 and 7744 configurations. (iii) The saturating exponent identity `x^d == x^(d+1)`,
  `law::the_saturating_exponent_absorbing_identity`. A reader searching the canon for the property the algorithm surface needs lands on
  (ii), which is a different predicate with a different subject.
- **retraction.** `35` 3.9 uses it for withdrawing a contribution, needing an additive inverse. The
  canon uses it for `round ∘ embed = id`: `law::rounding_retraction_is_the_identity`,
  `topic = "rounding"`. Section 3.4 below is why this one is expensive.
- **closed.** Unquantified in `35`'s section title and quantified in that row's slug, which
  is the whole of 2.3.

**These are worse than absence.** An absent term reports zero and prompts a question. A colliding term
reports twenty-nine hits, all irrelevant, and reports them to a reader who then believes the canon
covers the subject.

### 2.7 The predicate discipline reaches three fields in two namespaces

`mock/checks/src/predicate.rs:29-33`:

```rust
const PREDICATE_FIELDS: &[(&str, &str)] = &[
    ("proposal", "predicate"), ("law", "holds"), ("law", "fails"),
];
```

And `shape.rs:249`, `predicate_disagrees_with_the_sentence_kind`, iterates `reg.of("proposal")` alone.
The `retirement` namespace's schema (`mockspace.toml:804-853`) has no `predicate`, no `sentence_kind`,
no `evidence` and no `standing` field to carry.

That is correct for a retired *claim*, which is wrong and has no region. It is not correct for the
*measurement that killed it*, which is a real result with a real region and which has nowhere else in
the row to go. `196_probes/the_unpredicated_measurement_store.sh`, four controls including a
words-as-numbers negative that makes the count a lower bound: **9 of 176 retirement rows carry a
quantified figure in `why`**, against 17 of 94 proposal rows carrying one in `says`/`because`, of
which 15 carry a predicate.

Nine is smaller than I expected and I report it as measured rather than as the crisis I went looking
for. But one of the nine is `dl_interior_wrapping_with_a_reserved_absorbing_top`, and its `why`
carries, unpredicated, in a namespace with no field for a region:

> "It absorbs correctly, at zero of sixteen failures, matching saturation. It still gets shortest path
> on a directed acyclic graph wrong on 12.6 percent of 622 million in-range instances, because
> interior wrapping destroys monotonicity, at 560 of 2176, and **the min-plus algebra needs absorption
> and monotonicity both**."

The bolded clause is `35` section 3.5a's conclusion and one of the two rows `191` proposes to write.
It is in the canon, in the one namespace where under I13 an absent axis means the claim holds nowhere.

**And the tropical half genuinely did not reach anything else.** Searching for the claim rather than
the figure: `monoton` occurs 30 times, and outside `keywords` every occurrence is in `retirement.toml`
or in probe control prose. **No `law` and no `proposal` asserts monotonicity of addition under any
policy.** `retirement::dl_gate_algorithm_crates_on_addassoc`, in its `replacement` field, names "the monotonicity marker over addition" as "the atom the gate was
reaching for" and nothing supplies it.

So the accurate statement about `35`, replacing "two of fourteen figures survived", is a five-way
split by **how** each section reached the canon:

| `35` section | how it reached the canon |
|---|---|
| 3.7 ring laws | a `law` row, corrected, citing `35:311` as the origin of the defect |
| 3.2 accumulator from capacity | Q11 option 3, plus `proposal::the_multiplicative_guard...` which `answers` it |
| 3.8 per-aggregate | Q11's neighbour and `question.toml:180`, as options |
| 3.4, 3.5, 3.5a tropical | one retirement's `why`, and `question.toml:77` as an option |
| 3.1, 3.3, 3.6, 3.9, 3.10 | not at all |


### 2.8 Every panel file citing the registry by line is citing a moving target, and nothing checks it

`191` cites the chain result as `proposal.toml:860`. At the commit that added `191` that was exactly
right: `git show 78a907b6:mock/registry/proposal.toml | sed -n '858,862p'` prints the `[[proposal]]`
header with the id on the next line. **Eleven registry commits have landed since**, and the row's id
now sits at `proposal.toml:872`. Line 860 today is inside the neighbouring row, about compile-time
strategy selection leaving no residue, which is a different claim on a different topic.

The corpus already knows this hazard in the other direction. `mock/checks/src/citation.rs` and
`tests/no_line_citation_into_a_living_ledger.rs` refuse a **registry row** citing a moving line in a
ledger, and the test's own header states the reasoning: "a line citation into a file that is still
being written resolves forever and points at different text after every edit above it."

**The reverse direction is unguarded and is the busier one.** `panel` is frozen, so a registry row
citing `63_spj_consolidation...::640` is honest forever. The registry is not frozen; it is the most
actively edited tree in the repository. Every panel file that writes `proposal.toml:NNN`, and they all
do, has written a citation that decays silently. It does not fail, it does not warn, and it resolves
to a plausible neighbouring row, which is the worst available failure mode.

The remedy is available and costs nothing, because the ids exist and are stable: cite
`<namespace>::<slug>`, which is the form `mockspace.toml` already uses for `refsto(obligation::<slug>)`.
Slugs are unique per namespace; globally there are exactly three collisions (`overflow_policy` and
`rounding` each as a dimension and a topic, and `the_container_premise` twice), which is why the
namespace prefix is part of the form rather than decoration.

I have written this file's registry citations that way, with a line number only where it helps a
reader find the row today.

---

## 3. Replacements, addressed to `191`

Six, ordered by what I think they are worth. Each says what it replaces.

### 3.1 Replace section 2.1's inference: the finding is orphaning, not dropping

**Replaces** "the port kept those two files' retirements in full and dropped almost every positive
result they established."

**With** this, which your own data supports and which is smaller and sharper: *`35` and `43` are the
only substantial member derivations in this panel that no registry row cites, by file or by probe
directory, while forty-seven other probe directories are cited by row. Their results reached the canon
through consolidations and ledgers, at least three of them demonstrably: `35`'s ring laws as
`law::distributivity_of_multiplication_over_addition`, `35`'s capacity formula as Q11's third option
and as the contrast in `proposal::the_multiplicative_guard...`, and `43`'s composition vocabulary
through `173_leroy_the_canon_candidate_for_the_chain`, which cites it sixteen times and is itself
cited by eighteen registry rows.*

**Why it is worth more.** "Results were dropped" prescribes writing four new proposals. Orphaning
prescribes something cheaper and more useful: **read the two files against the rows that descend from
them and record which of their results the descent lost**, which is the compression check this
workspace already has a procedure for, pointed at a pair nobody ran it on. Four of `35`'s ten sections
are candidates and I name them in 2.7's table.

### 3.2 Replace section 2.3 entirely: the axis is the operation, and the sentence exists

**Replaces** "A fold and a chain want opposite operators, discriminated by the runtime trip count",
and the proposed row `a_fold_and_a_chain_want_opposite_operators_and_the_axis_is_the_trip_count`.

**With** the statement the registry already carries and one it does not:

*Carried.* `proposal::the_multiplicative_guard_grows_linearly_and_the_saving_is_adaptation_fusion`:
"any accumulator statement derived from a capacity is an additive-only mechanism." `two_experts`,
predicated, `answers`-wired to Q11. **Do not write a row that restates it.**

*Not carried, and this is the row I would write instead.* The additive half's positive statement, which
exists in `35` section 3.2 with two independent instruments and reached the canon only as a question
option:

- **`the_additive_accumulator_width_is_the_element_width_plus_the_log_of_the_bound`.**
  `sentence_kind = "measured"`. A fold over at most `C` values of width `W` needs
  `W + ceil(log2 C)`. Two instruments, opposite directions: `35_probes/p7_accumulator_from_capacity.rs`
  compiles it as a two-input contract with an associated accumulator type, with the derived widths
  checked by const assertion against arithmetic and negative controls showing one bit narrower is not
  enough, and `35_probes/p8_log2ceil_without_a_table.rs` supplies `ceil(log2)` inductively in three
  impls over a positive-binary representation with no table, checked at forty hand-written values,
  with a mutation arm dropping the increment that fails 33 assertions. And `20_fog_what_the_benches_already_know.md:208-213`, which I opened rather than relaying from `35`, reports the
  bench crate's own interior-safety predicate as the identical formula with the crossover at arity 8
  "to the row", and the arm picking the narrowest satisfying accumulator "at or near best at every
  arity". **Predicate**: `operation = add`; capacity `C` in 1..=65536 for the existence result and
  1..=64 for the sufficiency sweep; `total_width: W in 1..=24`; `threads = 1`; `target_features:
  rustc 1.98.0-nightly (57d06900f)` for the compiled half. **Gap**: unpriced, by `35`'s own statement
  and mine; the scale check to 65536 is an existence result and nothing measured how long it takes.

**And a correction to the axis, which is the part I would most like argued with.** The discriminator
between a widening composition and a non-widening one is not whether the trip count is static. It is
**whether the result width is indexed by position or bounded by a capacity**, and those come apart in
both directions:

- A dynamic count still admits widening, once, from a static bound. That is `35` 3.2 and it is the
  whole content of the row above. Under your formulation it is impossible.
- A static count does not always admit per-step widening in practice, because arm C measured three and
  four elements. A statically known count of ten thousand unrolls to ten thousand distinct types, and
  what that costs is unmeasured by anybody. Under your formulation it is licensed without qualification.

`35`'s own title carries the disjunction ("closed, **or its trip count must be static**") and its arm
E carries the counterexample to the first half. I would put the axis as position-against-bound and
keep the trip count as the sufficient condition it is, rather than as the axis.

### 3.3 Replace the class taxonomy's justification, and add the class it is missing

**Replaces** section 2.2's five classes as a partition of the demand, and its two readings (five
obligations or three).

**With** two things. First, the honest provenance: *the six needs are not six readings of the plan
chain. Five of them are the five clauses of one sentence of the consumer's dependency list, in order,
with the crate names struck out, and one of the five (`a_cost_dynamic_program`) paraphrases its crate
wrongly: the consumer's `arvo-comb` does matrix-chain ordering, greedy interval grouping and bin
packing, and the budget the obligation attaches to it belongs to a different step.* So the question
"what do the six have in common" is asking what five clauses of a dependency list have in common, and
the answer to that is that they name five crates.

Second, the class the sentence cannot see: **step 10, per-phase config selection among `MAX_FUSE`,
`BALANCED` and `MAX_SPLIT`, which the consumer's own documents call a strategy and once had a type
named `PhaseStrategy` for.** No crate is named for it, so `184` has no obligation for it and your
taxonomy has no class for it. It is the one step in the chain whose shape is a selection among named
policy alternatives, which is the shape of the thing your section 2.4 concludes does not survive above
the numeral.

I am not claiming `PhaseStrategy` is arvo's `Strategy`; it selects a grouping policy rather than an
arithmetic realisation. I am claiming it is the only datum in the chain that could **test** 2.4 rather
than illustrate it, and that it went missing for a reason that has nothing to do with what it is.

### 3.4 Replace the tarina reading: it is not I11's second half, it is `35` section 3.9's missing consumer

**Replaces** "That is a demand about what a computation carries, not about what a value is. It is
I11's second half stated by a consumer that had never heard it... I do not think this is an
obligation."

**With** a much stronger claim, and I think this is the most valuable single thing in my file.

Read tarina's sentence again, at `DESIGN.md:188-191`, and the operative clause is line 190:

> "a pack that is later disabled has to **withdraw exactly its own contributions**."

Now `35` section 3.9, which is titled "Retraction: I went looking for the requirement and found the
opposite":

> "I expected **retraction** to matter: an aggregate maintained under updates needs an additive
> inverse to **withdraw a contribution**, and `p2` measures that wrapping preserves it at 33 of 33
> cells while saturation loses it at 33 of 33, up to 49.6%.
>
> ... I read the downstream engine ... Its incremental machinery is incremental skip, not incremental
> aggregate ... The one running aggregate is an exponential moving average, and an EMA is the
> canonical structure you reach for precisely because it does not need an inverse ...
>
> So the requirement is not established ... a contract distinguishing a **monoid** (maintainable only
> by recomputation or a tree) from a **group** (maintainable in place) costs one named structure
> today, and a downstream engine wanting incremental maintenance later cannot add it without changing
> every contract. **I flag it and claim nothing.**"

**`35` measured tarina's requirement, in tarina's vocabulary, named the structure that expresses it,
named the retrofit cost, went looking for a consumer, read one engine, found none, and withdrew.**
tarina is the consumer. It maintains a derived sheet under pack enable and disable, which is
maintenance in place, which needs a group.

So tarina is not evidence about I11's second half in the abstract. It is the missing half of a
withdrawn measurement, and it makes that measurement's requirement live. And it is an obligation,
which you declined to claim:

- **`an_aggregate_maintainable_under_withdrawal`.** A composition whose contributions can be removed
  individually without recomputing the whole, so a contributor that is later disabled withdraws
  exactly its own share. Consumer: tarina, stating it as a hard requirement for a user-visible reason.
  **What it costs arvo**: the composition contract must distinguish a monoid from a group, because
  only the second is maintainable in place.

**And it lands on the overflow axis, hard.** `35` p2: retraction holds at 33 of 33 cells under
**wrapping** and fails at 33 of 33 under **saturation**, up to 49.6%. Everything else in this corpus
that reaches the algorithm surface wants saturation, because the tropical absorbing top is a law only
a lossy policy has. **So arvo has two consumers whose structural requirements select opposite overflow
policies, for reasons neither can trade away**, and that is a region split rather than a preference,
which is exactly the shape I13 says the work is made of.

`35`'s figure `49.6` is absent from the registry, measured by my ten-section probe. So is any `law` or
`proposal` asserting monotonicity of addition, checked the same way and reported in 2.7.

### 3.5 Replace one of the three questions to op: Q-C option 3 is already closed by the consumer

**Replaces** Q-C's third option: "No: a plan registration is data arriving at run time, the check is
at an ingest boundary, and I15's own closure of Q-A refused an ingest-boundary check as a design
option, so the consumer's shape is already wrong for a reason that has nothing to do with arvo."

**Because it is not true of this consumer.** `PlanDims` is a trait with associated types, not runtime
data. `hilavitkutin/mock/research/202605301130_numeric-position-convention.md:317-327`:

```rust
pub trait PlanDims { ... }
pub struct DefaultPlanDims;
impl PlanDims for DefaultPlanDims { type Units = Cap64; type Stores = Cap64; ... }
// Scheduler<Cfg, WuVals, Vals, M, D: PlanDims = DefaultPlanDims>
```

The declared capacities are associated types on a type parameter of the scheduler. They are compile
time, and `Cap64` is already an arvo capacity type. So your section 2.5's premise is stronger than you
stated it and option 3 is refuted by the consumer's own design.

**Putting a closed option to op is the thing this workspace forbids most specifically.** Two options
remain and they are a real fork; I restate them in section 5.

### 3.6 Replace the topic proposal in 3.1 with an edge proposal, on the evidence in 2.7

**Replaces** "One new topic. `the_layer_above`", offered with its cost and with "I do not know which
is right".

**With**: do not add a topic. The reason your section 0.3 gives for wanting one, that every
algorithm-surface result carries a numeral topic and is therefore invisible, is a **search** problem
and the corpus has a search field, `keywords`, plus a typed edge vocabulary that is measurably
unused (two `obligation` edges in 577 rows). The four rows in 2.3's table are all `topic =
"the_chain"`, which is a perfectly good subject for what a composition requires of a numeral, and
re-filing rows is a change to a document later rows were written against, which is the cost you
correctly named.

What the corpus is short of is not a topic. It is edges. Your section 3.5 already says so and then
declines to write them for a good reason. I would keep that decline and narrow the job: **five edges,
each with both rows opened**, and I name them in section 5 as work rather than as a question.

---

## 4. Alternatives I considered and rejected

Eleven, with what closed each. The list is the part of this file most likely to save the next member
time.

**1. Attack `191`'s figure counts directly.** Re-ran `191_probes/which_of_35s_figures_survived.sh`
and my own ten-section arm reproduces its `63 of 63` absence exactly. Closed: the data is right. The
attack had to move to what the data licenses.

**2. Argue the retirement namespace is a hole in the predicate discipline, at scale.** Built
`the_unpredicated_measurement_store.sh` expecting a large store of unpredicated measurements in
`retirement.why`. Measured **9 of 176**. Closed as a scale claim; kept as a single-row claim, because
one of the nine is the row carrying the min-plus requirement. Reporting the 9 rather than the crisis I
went looking for is the honest outcome and the probe's `NEG-WORD` control says the 9 is a lower bound.

**3. Argue "absorption" is a vocabulary collision so severe the canon misleads.** It is a collision,
three senses, and I keep it at 2.6. Closed as a headline: I could not show anybody was actually
misled by it, and a collision with no demonstrated victim is a tidiness complaint. `191` itself was
not misled; it never searched the word.

**4. Argue the obligation rows' "twelve-step plan chain" contradicts the design's thirteen steps.**
Checked: `hilavitkutin/mock/DESIGN.md.tmpl:46` is headed "Plan algorithm chain (12 steps + step 13
FiberShape classification)". The consumer says both and is consistent. Closed as a contradiction, and
it is how I found the twelve canonical stages that ARM 4 walks, which is where step 10 fell out.

**5. Build a compile spike extending `191`'s `p2` type-level relation to price it.** `191` correctly
records trait-solver depth and compile time as unmeasured. I did not build it: pricing needs the bench
harness, an ad-hoc compile spike would produce a number with no standing, and the question is
unpriced either way. Saying it is unpriced costs nothing and is true.

**6. Read `43` and check `191`'s "zero of seven" myself.** Closed by scope and by 2.1: the claim is
about the registry, and the registry answers it. Reading `43` would have told me what `43` says, which
is a different question, and `168_mcsherry_the_chain_derived_cold` and `173` have already read it
twenty and sixteen times respectively.

**7. Propose the monoid-against-group contract as a canon sentence.** Tempting, because `35` 3.9
hands it over ready. Closed: it is a design choice about what a composition contract distinguishes,
one consumer has now been found to want the group half, and `35` explicitly declined to claim it. It
belongs in section 5 as a question with the new evidence attached, not as a row I write.

**8. Argue the whole obligation namespace should be rebuilt from consumer prose rather than from
dependency lists.** Closed as scope creep. `184` names the demand-side sweep as owed, `191` agrees it
is still owed, and it is a unit rather than a section of an attack. What I add is a reason it matters
more than either said: the current rows are a crate list, so the sweep is not a widening of the demand
side, it is the first reading of it.

**9. Report the deleted crate tree as recoverable from a consumer's cargo checkout.** One arm of my
crate-list probe surfaced `~/.cargo/git/checkouts/arvo-*/mock/crates/arvo-comb/src/dp.rs` through
hilavitkutin's `mock/target/`. I excluded `/target/` from the arm and did not open the file. Closed as
a finding I will not build on, and flagged as a hazard in section 6: the tier that had to be detached
for canon work is one grep away in every consumer clone that has built.

**10. Write the five edges.** Closed by `191`'s own reasoning, which I agree with: `187` section 5 is
a list of edges somebody asserted without opening both sides, and doing it again quickly would be the
same defect. It is work, named in section 5, not a thing to do in an attack's margins.

**11. Argue that `retirement` should gain a `predicate` field.** Closed by deriving it: a retired
claim is wrong and has no region, so a predicate on it would be a category error of exactly the kind
`shape.rs`'s `REGIONLESS` handling exists to prevent. The measurement that killed the claim is the
thing wanting a home, and it already has one, `probe`, with `establishes` and `control` fields. So the
repair is an edge from the retirement to a probe row, not a new field.

---

## 5. What is genuinely op's, and what is only work

Two questions, down from `191`'s three, and I say why the third is not one.

**Q-D. Does arvo's composition contract distinguish a monoid from a group?**

- *Options.* (1) It does, so a composition states whether its contributions can be withdrawn
  individually, at the cost of one named structure in the contract vocabulary and of a distinction
  every composition must then declare. (2) It does not, so every aggregate is maintained by
  recomputation or by a tree, at the cost that a consumer wanting maintenance in place cannot express
  it and cannot be given it later without changing every contract. (3) It does for addition and not
  in general, which is where the measurement actually falls: retraction holds at 33 of 33 cells under
  wrapping and fails at 33 of 33 under saturation, so the distinction is real only where the policy
  admits it.
- *What answering it unblocks.* Whether `an_aggregate_maintainable_under_withdrawal` enters the
  obligation namespace, and whether `35`'s section 3.9 measurement gets filed or stays withdrawn.
- *Why it is his.* `35` states the asymmetry and it is the reason this cannot wait: one named
  structure today against changing every contract later. And it is a choice about what arvo is for,
  which is I11's territory.
- *What is new since `35` declined it.* A consumer. tarina states the requirement in `35`'s own words,
  had never seen `35`, and is one of the repositories I11 names.

**Q-E. `I11` names two things. Is the second one this arc's work, or the next one's?**

Kept from `191` unchanged, because I could not improve it and because it is genuinely his. `191` has
it as Q-B with three options and I would put it forward as written. The only thing I add is that the
answer now costs less than `191` thought: if the composition contracts are this arc's, section 2.3
shows the operator half is already carried and section 3.2 shows the additive half is one row.

**And Q-C is not op's.** `191` puts three options to him about whether closing the consumer's runtime
capacity check is arvo's obligation. Option 3 is refuted by the consumer's own source, per 3.5. The
remaining two are "arvo owes the bounded index" and "I15 binds arvo's surfaces rather than its
consumers'", and that is a scope question about I15's reach which is his. I would ask it as one
sentence rather than as a fork with a dead arm, and I would attach `191`'s `p1`/`p2` result to it,
which is that the concept is expressible with a definition-site refusal, gate-free, at the cost of the
capacity being a type rather than a const.

**Not questions, work.** Five edges, each with both rows opened before it is written:
`retirement::dl_interior_wrapping_with_a_reserved_absorbing_top` and
`law::distributivity_of_multiplication_over_addition` to
`obligation::a_spectral_partition_of_a_dependency_graph` and
`obligation::a_cost_dynamic_program`; `proposal::the_multiplicative_guard...` and
`proposal::chain_accuracy_cannot_be_served_by_an_operator_closed_over_its_operand_type` to
`obligation::composition_contracts_above_the_numeral`. And one correction:
`obligation::a_cost_dynamic_program`'s `need`, which does not describe what its consumer asked for.

---

## 6. What I could not determine, and two hazards

**Whether the orphaning of `35` and `43` lost anything beyond the four sections in 2.7's table.**
Answering it is a compression check over two files against the rows that descend from them, which is a
dispatch and not a section.

**Whether step 10's `PhaseStrategy` bears on arvo's strategy axis at all.** I have an analogy and a
reason to look. Nobody has looked, and the reason nobody has is a sentence in a dependency list.

**What the corpus's "absorption" collision has actually cost.** Three senses, twenty-nine occurrences,
no demonstrated victim. It is a hazard I can name and not a defect I can price.

**Whether the additive accumulator row's region is right.** I took `W in 1..=24` and `C in 1..=64`
from `35`'s description of `p8`'s sufficiency sweep and `C` to 65536 from `p8_scale_check.rs`, both as
`35` reports them. I did not run either probe. Whoever writes the row runs them.

**Hazard one: the deleted tier is one grep away.** Any consumer clone that has built carries the
deleted arvo crates under `mock/target/` and in `~/.cargo/git/checkouts/`. A probe grepping a consumer
for "arvo" reaches them without deciding to. Mine did; I excluded `/target/` and did not open
anything. A later probe that does not exclude it will read the dead tier and report about it, and
nothing in its output will say so.

**Hazard two: every line citation into the registry decays.** 2.8. It is not hypothetical, it has
already happened to `191` within eleven commits, and the check that would catch it exists and points
the other way.

---

## 7. Probes

All under `196_probes/`, committed as they ran, each with the case that had to fail before its result
counts.

| probe | what it establishes | controls |
|---|---|---|
| `the_unpredicated_measurement_store.sh` | 9 of 176 retirement rows carry a quantified figure in `why`, a field in a namespace with no region, against 17 of 94 proposal rows of which 15 carry a predicate | POS-REAL the row `191` names must match; POS-PLANT a planted figure must match; NEG-PLANT a pure argument must not; NEG-WORD numbers as words must not, which makes the count a lower bound |
| `the_composition_surface_by_axis_not_by_phrase.sh` | eleven rows carry a `chain_length` predicate and they partition by `operation`, not by composition shape; 37 rows carry `topic = "the_chain"` | CTRL-REPRO must reproduce `191`'s zero claim-bearing hits for "trip count" or the two files disagree on data; CTRL-POS a planted predicate must be found; CTRL-NEG a lookalike slug must not; CTRL-COL added after v1 |
| `which_of_35s_ten_sections_reached_the_registry.sh` | `191` sampled three of `35`'s ten sections, all from the algebra half; the composition half was not sampled | `191`'s own POS-A and POS-B, a NEG, and REPRO requiring `63 of 63` absent so the two files read the same tree |
| `how_35_and_43_actually_reached_the_canon.sh` | 97 numbered panel targets are cited by provenance and `35` and `43` are not; 76 panel files reference them and 16 of those carriers are cited, including the three highest-volume | POS the most-cited consolidation must be read out of provenance; NEG a nonexistent slug must not appear; DISC the arm prints the outcome that would refute the reading |
| `are_the_plan_chain_obligations_the_crate_list.sh` | five obligations map one-to-one in order onto five clauses of one dependency sentence; `a_cost_dynamic_program` does not describe what its crate does; step 10 is unassigned by `191` and uncrated by the consumer | POS-1 the twelve stages must parse; POS-2 the sentence must be found; NEG the clause splitter must match an invented `arvo-zzz` clause so ARM 2 is not circular; DISC ARM 3 prints every purpose statement so the rewrite reading is refutable |

**Three of the five caught a defect in their own first version and the transcripts are kept.**

- `the_composition_surface_by_axis_not_by_phrase.sh` split its rows by `grep -i mul` over the whole
  line, which matched every row whose id contains "accu**mul**ator", filing an additive result under
  multiplication. Transcript `_v1_accumulator_matched_mul.out`. The fix splits on the operation column
  and CTRL-COL now pins it.
- `which_of_35s_ten_sections_reached_the_registry.sh` reported `87.5` present for section 3.7. **It is
  a false positive**, in `proposal.toml`'s row about composed add-and-subtract associativity under box
  lifting, whose own keywords carry `21.98`. A three-character decimal is not distinctive. The probe
  prints context for every hit, so the instrument was honest and I read the count instead of the
  context for one draft. Recorded in the probe's own closing text rather than tuned away.
- `how_35_and_43_actually_reached_the_canon.sh` and the ten-section arm both died silently under
  `set -euo pipefail` on a grep that matched nothing, and the second died again on a `[ ... ] && [ ...
  ] && continue` whose false first test killed the script. Both are the documented neighbour hazards
  and both produced an empty section that looked like a zero rather than like a crash.

**One note for the next probe here, added to `191`'s.** `191` records that nutshell never sets `$0` to
the script path. Two more: a `grep` that matches nothing kills a pipeline under `pipefail`, so guard
every count with `|| true` and know that an empty arm and a zero arm look identical in the output; and
an `&&` chain as a bare statement makes the whole script's exit status depend on its first test, so a
guard clause written as `[ a ] && [ b ] && continue` terminates the run whenever `a` is false.

**Paths.** Every probe resolves the repository by walking up for `mockspace.toml` and resolves the
consumer clones from the repository's parent. None names a home directory, per the tenth
instrument-defect class.

**And two defects in this file rather than in its probes, both caught by verifying my own citations
after drafting and both of the exact class this file attacks.**

- I wrote `retirement::dl_the_associativity_gate_on_the_algorithm_crates` as the slug for the row that
  retires gating the algorithm crates on associativity. **No such slug exists.** I had constructed it
  from the row's `claim` text rather than reading its `id`. The row is
  `retirement::dl_gate_algorithm_crates_on_addassoc`. A fabricated slug is worse than a stale line
  number, because it resolves to nothing and therefore reads as a citation nobody bothered to check
  rather than as one that decayed.
- I wrote that seventeen probe directories are cited by registry rows. **It is forty-seven**, counted
  with `grep -rho '"panel::...::[0-9]*_probes' mock/registry/*.toml | sort -u | wc -l`. I had counted
  the distinct directories visible in one screen of a sorted list rather than running the count. The
  error made my own finding weaker than it is: `35` and `43` are not two exceptions among seventeen,
  they are two among forty-seven.

Both were found by opening every citation in the draft before committing it, which took a few minutes
and is the whole of the remedy. Neither would have been caught by reading the file.

---

## 8. The blocker in 3.2, attacked

Section 3.2 proposed a row on `35`'s accumulator formula and section 6 said "whoever writes the row
runs [`p7` and `p8`]". Reporting a blocker and leaving it is not a deliverable, so I ran them, and the
run found something neither `35` nor `191` records.

### 8.1 `35`'s two probes were never composed, and the composition is the wall `191` hit

`35` supports `acc_width(W, C) = W + ceil(log2 C)` with `p7`, which builds the `SumAccum` derivation,
and `p8`, which supplies `ceil(log2)` inductively with no table. `35` says of the first: "`p7`'s first
version was inadmissible and I say so before anyone else has to. Its `ceil(log2)` was one impl per
capacity, which is exactly the enumeration `SETTLED.md:110` refuses."

**The committed `p7.out` is still that version.** Its arm-5 diagnostic prints the enumeration:

```
help: the following other types implement trait `Log2Ceil`:
          Cap<1024>  Cap<16>  Cap<1>  Cap<256>  Cap<2>  Cap<3>  Cap<4>  Cap<8>
```

And `p8` never mentions `SumAccum`. So "expressible gate-free" is the conjunction of two artifacts
using **different capacity representations**: `p7` a const generic `Cap<const K: usize>`, `p8` a
type-level binary `One / Twice<N> / TwiceP1<N>`. Joining them needs a map from a const to a type,
which is the direction Rust refuses, and **that is the same wall `191` section 2.5 hit from the other
side.** `191` reached it going from a const capacity toward a definition-site refusal; `35` reached it
going from an inductive log toward a usable derivation. Neither noticed the other was there.

`196_probes/p3_composing_p7_and_p8.rs` and `p3b_const_to_type_bridge.rs`, `rustc 1.98.0-nightly
(57d06900f)`, every arm landing on its required verdict:

| arm | shape | required | got |
|---|---|---|---|
| A1 | `SumAccum` keyed on the binary capacity, folded over a slice | COMPILE | COMPILE |
| A2 | derived widths asserted against integer arithmetic at capacities 1, 2, 3, 4, 5, 7, 16, 256, plus a tightness control | COMPILE | COMPILE |
| A2m | the `TwiceP1` recurrence's increment dropped | REFUSE | REFUSE, at C3 and C5 |
| B1 | a blanket bridge `Cap<K>` to the binary type | REFUSE | REFUSE, "generic parameters may not be used in const operations" |
| B2 | array storage `[T; C::VAL as usize]` | REFUSE | REFUSE, same diagnostic |
| C1 | both a const `K` and a capacity type `C`, instantiated as `Both<Num<N4>, 4, C256>` | COMPILE | **COMPILE** |
| C2a | the agreement asked for by associated-const equality, no feature | REFUSE | REFUSE, wants `min_generic_const_args` |
| C2b | the same with `#![feature(associated_const_equality)]` | REFUSE | REFUSE, **E0557 feature has been removed** |

**A2m is the case that had to fail** and it fails at exactly the odd capacities, which is where the
dropped increment changes the answer. Without it every COMPILE above would mean only that nothing was
checked.

**C1 is `191`'s landmine, reproduced from `35`'s side.** `Both<Num<N4>, 4, C256>` says the storage
holds four and the capacity is 256, nothing relates them, and it is a nameable, storable, returnable
type that compiles. `191` found the same shape at its `p1` arms G1 to G3 and called it a landmine; it
is the same landmine and it arrives whenever a const and a type are both carried with no relation.

**And C2b moves a droplist entry, without invalidating one.** `191`'s arm D and `35`'s
section-6 droplist both record `generic_const_exprs` as the refused route, and the registry's
droplist rows name `generic_const_exprs` and `min_generic_const_args` and nothing else. I grepped
`associated_const_equality` across `mock/registry/` and the whole panel: it appears nowhere, so no
row is wrong today and I am not reporting one. What is worth recording is that the spelling
somebody would reach for next, having been refused by the other two, **has been removed from the
toolchain** rather than being unstable, and its diagnostic redirects to `min_generic_const_args`,
which `retirement::dl_capacity_unification_naive_spelling` already records as unable to express
the inductive doubling step.

**One thing that had to be split out and is a result of its own.** The C2 bound cannot share a file
with the other arms. `C: PosVal<VAL = { K as u64 }>` is gated at **parse** time, so merely writing it
under a `#[cfg]` that is off makes every other item in the file unbuildable. That is why `35`'s `p7`
and `191`'s `p1` could each stay gate-free while the composition of the two cannot be tested inside
either, and it is a trap for the next person who tries.

### 8.2 So drop the const. The storage derives from the capacity type too

B1, B2 and C1 all exist for one reason: the storage wanted `[T; K]`. Nobody asked whether the storage
could come from the same induction as the width. `196_probes/p4_storage_from_the_capacity_type.rs`:

```
One          ->  Slot<T>
Twice<N>     ->  Pair<N::Shape, N::Shape>
TwiceP1<N>   ->  Pair<Pair<N::Shape, N::Shape>, Slot<T>>
```

Three impls, pairwise disjoint by the same construction that makes `Log2Ceil` three impls, holding
exactly `PosVal::VAL` slots by construction, with **no array length and no arithmetic in a type
position**.

| arm | required | got |
|---|---|---|
| S1 the storage exists at capacities 1, 2, 3, 4, 5, 7, 13, 16 | COMPILE | COMPILE |
| S2 slot count and `size_of` of the laid-out shape both equal the capacity | COMPILE | COMPILE |
| S2m the odd constructor forgets its extra slot | REFUSE | REFUSE, at C3, C5, C7, C13 |
| S3 a structural fold over the shape into the `p3`-derived accumulator | COMPILE | COMPILE |
| S4 a capacity with no storage row, used | REFUSE | REFUSE, `NoRow: Store<u8>` not satisfied |
| S5 the shape's size and alignment against a flat array, at `u8` and at `u64` | COMPILE | COMPILE |

**S2 is deliberately not two declarations checking each other.** The slot count is asserted against
`core::mem::size_of` of the shape the compiler actually laid out, so a shape with a missing slot is a
smaller type and the arithmetic catches it. S2m proves it: with the extra slot dropped, both the count
assertion and the `size_of` assertion fail, at every odd capacity and at none of the even ones.

**S5 is the result I did not expect.** The nested `Pair` tree is `repr(Rust)`, so its layout is
unspecified and the odd nesting `Pair<Pair<X, X>, Slot<T>>` is where padding would appear.
`size_of::<Shape>() == C * size_of::<T>()` and `align_of::<Shape>() == align_of::<T>()` hold at all
eight capacities, four of them odd, for a one-byte element and an eight-byte one. **The derived shape
is size- and alignment-identical to `[T; C]` on this toolchain.**

| arm | required | got |
|---|---|---|
| S6 a capacity-3 store constructed and folded | COMPILE | COMPILE |
| S6b a capacity-2 shape offered where capacity 3 is wanted | REFUSE | REFUSE, E0308 |
| S7 the same, built as a binary and **executed** | exit 0 | exit 0, printing `= 9, accumulator width = 4` |
| S7m the same binary with the recurrence mutated | must not build | does not build |

**S6, S6b and S7 exist because a retirement demanded them.**
`retirement::dl_feasibility_probe_compiled_the_load_bearing_path` retires a prior capacity probe's
claim with: "the probe declared the capacity trait as a bare const and never reached the associated
array type the domain exists for." Naming a type in a signature normalises it and does not prove a
value of it can be built, so S6 builds one, S7 runs the fold and asserts the answer against
arithmetic, and S6b offers a capacity-2 shape where capacity 3's is wanted and is refused. **S7m is
what makes S7's exit-0 mean anything**: with the recurrence mutated the binary does not build at all,
because the const assertions fail before `main` exists.

Before that, `s6_value_is_right` was a function returning a bool that nothing evaluated, which is the
shape the test gate calls a test that asserts nothing. It compiled under every arm and proved nothing.
S7 is that defect repaired rather than described.

**And the neighbouring retirement is not this construction.**
`retirement::dl_capacity_unification_naive_spelling` retires "the shared carrier answering directly
for the **backing array**", refused four ways "citing the forbidden generic-const-expressions feature
and, behind the compiler's own suggested successor, the inductive doubling step, which the restricted
successor cannot express either." That is `2N` as an **array length**. `p4` never builds an array
length: the doubling is `Pair<S, S>`, a structural recursion on types, and no const arithmetic appears
anywhere in it. So this is not the retired claim re-proposed, it is a construction reaching the same
place by not needing the thing that was refused. I checked that before building on it rather than
after.

### 8.3 What this composes to, with the costs stated in both directions

**The const never has to exist.** With the storage derived, there is nothing for a const to disagree
with, so B1's missing bridge, B2's refused array length and C1's landmine all stop being problems
rather than being solved. That is the shape worth having: the fork disappears rather than being
decided.

**What it costs, and none of it is free:**

- **The consumer writes the capacity as a binary type.** `C16` is `Twice<Twice<Twice<Twice<One>>>>`.
  This is `191`'s own recorded cost for its `p2` and it is the same cost here.
- **The capacity must be named once at each fold.** The shape does not determine it: `Pair<Slot<T>,
  Slot<T>>` is the storage for capacity 2 and is structurally a sub-shape of the storage for 4, so
  inference reports E0283. One turbofish per fold, not per element. Recorded in the probe at the line
  where it bit.
- **The storage is a nested tree, not a slice.** It is the same size and alignment as `[T; C]` and it
  is **not** contiguous-by-guarantee, cannot be indexed by an integer, and cannot be handed to
  anything wanting `&[T]`. **An algorithm crate wanting random access cannot use this shape**, and
  most of the plan chain's class A wants exactly that. So this is an arm over a region, not a
  replacement.
- **Unpriced.** Compile time, trait-solver depth, and behaviour at capacities near the pointer width.
  `35` flagged the same for its own `ceil(log2)` and called it unpriced; this is unpriced on the same
  terms, and pricing it needs the bench harness rather than another compile spike.

**The composition, which is the deliverable rather than a winner:**

| capacity carried as | width derivation | storage | refuses a mismatch at | costs |
|---|---|---|---|---|
| a const `K` alone | enumerated per capacity, `p7` as committed | `[T; K]`, natural | nothing; there is no mismatch to have | the enumeration a ratified rule refuses |
| a const `K` and a type `C`, unrelated | inductive, gate-free | `[T; K]`, natural | **nothing**, and the type compiles wrong: `p3` C1 | a landmine, `191`'s G1-G3 at a second site |
| a const `K` and a type `C`, related | inductive | `[T; K]` | would refuse at the definition site | unavailable: `p3b` C2a and C2b, and the feature is removed |
| a type `C` alone, slice storage | inductive, gate-free | `&[T]` with a runtime length | the capacity, not the length | the length is unchecked, which is where a runtime check would go back in |
| a type `C` alone, derived storage | inductive, gate-free | `Pair` tree, same size and alignment as `[T; C]` | every position that names the shape | the binary spelling, one turbofish per fold, no indexing, no slice interop |

### 8.4 What this changes in section 3.2, and what it opens

**The row I proposed in 3.2 is writable and its region is narrower than I wrote it.** The formula and
the inductive `ceil(log2)` are established; what is established **gate-free as one artifact** is the
derivation over a capacity carried as a type. Whoever writes the row states that, and does not write
"expressible gate-free" unqualified, because on the obvious spelling with a const capacity it is not.

**And it opens one thing that is the panel's rather than op's**, so I state it as work with a
decision procedure rather than as a question: *a bounded aggregate's capacity is a type, a const, or
both, and the three differ in what they refuse rather than in what they express.* The evidence for
picking is in the table above; the missing input is a measurement of what the type spelling costs at
compile time, which is a bench and not another compile spike. **Do not put this to op as a fork.**
Both arms are right in their own region: class A of the plan chain wants indexable storage and
therefore a const, and a fold wanting a definition-site refusal wants a type. That is two arms and a
predicate, which is what I13 says the work is.

Probes for section 8, committed with their logs and every arm's required verdict printed beside what
it got:

| probe | what it establishes | the case that had to fail |
|---|---|---|
| `p3_composing_p7_and_p8.rs` + `p3_run.sh` | `35`'s two accumulator probes compose only with a type-carried capacity; the const-to-type bridge, the type-to-array-length map and the both-carried agreement are refused; carrying both unrelated compiles wrong | A2m, the recurrence's increment dropped, which fails at C3 and C5 and nowhere else |
| `p3b_const_to_type_bridge.rs` + `p3b_run.sh` | the associated-const-equality bridge is refused, and the feature has been **removed** rather than being unstable | C2b, which had to be reachable: if it had compiled the finding would be "gated" rather than "unavailable" |
| `p4_storage_from_the_capacity_type.rs` + `p4_run.sh` | the storage derives from the capacity type by the same three-impl induction, with slot count tied to `size_of` of the laid-out shape, and packs identically to `[T; C]` at eight capacities and two element alignments | S2m, the odd constructor forgetting its slot, which fails at C3, C5, C7 and C13 |

**Three of my own defects in these three, all caught by a control or by a base arm that had to
compile, all recorded here rather than tuned away.**

- `p3`'s base arm failed on the first run with `E0658: associated const equality is incomplete`,
  pointing at a line inside a `#[cfg]` that was off. Diagnosis: the bound is parse-gated. Fix: split
  it into `p3b`. **The base arm is what caught it**, exactly as `191` records for its own `-o
  /dev/null` run, and had the base arm not been required to compile I would have read six refusals as
  six results.
- `p4`'s `FoldInto` was parameterised by the accumulator where it needed the capacity, so the compiler
  asked for `Num<Su<...>>: Log2Ceil`, which is nonsense. Caught by the base arm again.
- `p4`'s fold then failed with `E0283`, which I first read as a defect and which is a **finding**: the
  derived shape does not determine its capacity, so the capacity has to be named at the fold. It is
  recorded as a cost in 8.3 and as a comment at the line where it bit.
