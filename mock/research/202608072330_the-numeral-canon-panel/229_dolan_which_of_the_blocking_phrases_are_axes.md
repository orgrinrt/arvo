# 229. Which of the blocking phrases are axes, and which are wearing a predicate's clothes

Twelve readings, covering thirty-five of the phrase spellings the ranking lists.
`dimension.toml`, in the header paragraph beginning "The set still moves only", says it moves on two
independent readings. This is one of them and it has not seen the other, which is the point; if a second
exists I have not read it and did not look.

I did not edit `dimension.toml`. Nothing below is a declaration.

---

## 0. The two gates

**Canon gate: passed.** Checked against `mock/registry/*.toml`, which `mockspace.toml:31` declares as
`canon_paths`. The question I was sent is a judgement the canon explicitly reserves to two readers
(same paragraph) and asks for a reading rather than an edit. Two ratified rows bear on the answer
and I use both against the brief rather than around it: `ruling::the_warrant_is_a_token_and_a_clause_on_the_values_side`
(a `ruling` row, `rung = "ratified"`) and
`ruling::behaviour_is_stated_per_declared_signature_and_the_premise_dissolves` (likewise ratified). One of
them closes the brief's second question outright and one of them settles a phrase in the first. Neither was
named in the brief.

**Test gate: run, with one part of it that does not terminate.** Recorded in `229_probes/test_gate.out`.

- `cargo test --workspace` at `mock/` refuses: the manifest is virtual with `members = []`. Expected and
  deliberate, per that file's own header. There is no source tier here.
- **`cargo mock test` did not finish.** It descends into `mock/benches/variants/` and builds each bench crate's
  tests in debug. `bench_bitpack_write_contend_shared` ran twenty-five minutes at about 200% CPU without
  completing and I stopped it. A second copy of the same binary, from a different clone of this repository,
  had already accumulated 132 minutes of CPU before this session opened and was still running when I
  finished, so it is not this worktree. The cause is readable rather than mysterious:
  `mock/benches/variants/bitpack-write-contend-shared/src/stress.rs:99` runs 3000 four-thread concurrency
  trials, with 1000 more at `:121` and 500 at `:88`, unoptimised.
- **The lint pack, which is the surface this work touches: 540 passed, 0 failed, 13 ignored.** I read the
  thirteen. Every one is `#[ignore = "catalogue: ..."]` naming its own gap, and all thirteen sit in the six
  source-side `CrateLint`s that have no source left to lint. That is the catalogue discipline working rather
  than a suite hiding behind an attribute.
- `cargo mock --lint-only` over the real rows: 695 rows, schema check passed, all lints passed.

One thing in that suite fails the gate on its own terms and I will say so rather than leave it in a
footnote. `stress.rs:97-112`, `naive_kernel_corruption_rate_under_real_concurrency`, asserts nothing. Its
own comment argues the case, and the argument is honest: a scheduler-dependent rate is not something a test
should threshold. The conclusion does not follow. What it is is a measurement that prints to stderr, and a
measurement belongs on the bench harness where its number is captured, not in a `#[test]` where the number
goes to a stream nobody reads and the 3000 trials are charged to everybody who runs the suite. It is also
the single largest contributor to the wall time above. The control beside it,
`naive_kernel_never_corrupts_when_the_split_is_aligned` at `:118`, is a real test and should stay.

---

## 1. What reproduces, and what the two instruments each measure

Both committed outputs reproduce byte for byte, run in my own directory against the same corpus. The diffs
are in `229_probes/reproduction.txt`.

- `span_verdicts.sh`: identical. 4 portable, 60 blocked, of 64 spans.
- `unblock_value.sh`: identical, including the ranking's order.
- `axis_census.sh`: **not** identical, and correctly so, because it reads the live `dimension.toml`. The
  committed run says 16 declared axes and 19 of 115 keys declared; today it says 22 and 21 of 115. The
  delta is exactly `radix` and `ambient domain`, both of which the census now marks `declared`.

So the ranking is reproducible and the counts in `dimension.toml`'s header paragraph beginning "A later
seat measured" were true when written. Two of them
are now stale in a way nobody will be told about, which is section 6.

**The two instruments count different populations and the difference matters for what a declaration buys.**
`unblock_value.out` counts *predicate spans*, 64 of them, from eight files. `blocked_inventory.tsv` counts
*statement clauses*, 65 of them, from twelve. The committed probe row
`probe::the_inventory_of_clauses_that_could_not_be_written` says in its `establishes` field that its
tally is 24 no-axis, 16 no-region, 23 ported, 1 refused, 1 folded. I re-derived that from the file rather
than quoting the row: 24, 16, 23, 1, 1, total 65. It reproduces (`229_probes/blocked_by_family.out`, control
B1).

**The row-level instrument is the one that answers "what would declaring this buy", and it has not been
re-scored since the four declarations landed.** I did that, with a negative control that clears zero when
the declared set is emptied (control B3). **Five of the twenty-four blocked clauses are cleared by
`ambient_domain`, `radix`, `accumulator_width` and `operand_window`. Nineteen are not.** The five are
`122:397`, `122:425`, `136:320`, `136:355` and `146:452`.

**None of those five has been written.** That is a separate measurement and it is the one that changed my
view of what a declaration is worth. Across the 527 predicate entries in the four predicate-bearing files,
19 distinct axes are named. Three declared axes are named by nothing at all: `access_pattern`,
`ambient_domain` and `occupancy` (`229_probes/axis_usage.txt`, `warrant_usage.out`). `ambient_domain` was
declared as "the single largest blocker in the corpus's unwritable predicates, sole blocker of four spans
and present in eleven" (`dimension::ambient_domain`, `moves`) and it appears in `proposal.toml` exactly
once, inside a row id, as
part of the slug `a_format_is_identified_by_its_ambient_domain_and_its_representable_set`. **Declaring
an axis does not write the rows.** Whoever proposes the next declaration
should say who is doing the port, because the last round of four bought five clearable clauses and zero
written ones.

*holds for: this registry at commit `0e71955b`; every count above re-derived from the files rather than
quoted from a row. Not a claim about any other corpus.*

---

## 2. The ranking's method, tested

The instrument is sound for what it claims and has three limits, two of which its author states and one of
which it does not.

**Stated by its author and correct.** The comma splitter protects `{}`, `[]` and `()` and not commas inside
prose values, so three spans block only on `or unsigned with signed`, which is the tail of the single value
`signedness = signed, or unsigned with signed intermediates`. The footer of `span_verdicts.sh` names the
three spans and says the honest portable count is 5 of 64 rather than 4. I confirmed this independently from
the span text in `229_probes/context_all.out` before reading the footer. **That phrase is a tokenisation
artifact and not a key.** It sits at rank three by sole-blocker count, which is where a reader who trusts the
column ordering would start.

**Stated and correct.** The coverage/region split in `span_verdicts.sh:11-18` is one seat's judgement,
written out. It is conservative in the right direction: anything not on the coverage list blocks rather than
being silently dropped.

**Not stated, and it is the one that would mislead a reader.** The grouping into families in the second
table is a regex assignment written by hand, and two of its rows are wrong.

- `fraction` is grouped under *the stochastic-coupling parameters* because the family regex matches the
  substring. Its three occurrences are `fraction = 1/3` in `132 #9`, `136 #7` and `138 #2`, which is the
  stochastic rounding member's fractional part, so the grouping happens to land right for the wrong reason.
- `carriers 8..=24 > and 120` is grouped under *the term and declaration shape* by the `carriers` alternative.
  It is the container axis, and it is already declared.

Neither changes the ordering. Both mean the family table is a reading and not a measurement, and it is
presented in the same typeface as the measurement above it.

**And the census's own mapping table is stale in the direction that understates what the declarations
bought.** `axis_census.sh:103` still maps `accumulator_width` to `declared as container`, and `:104` maps
`toolchain` and `edition` to `declared as build_profile, loosely`. Both are their own rows now. Re-running
the census reports 21 declared where the true figure against the live registry is higher.

---

## 3. The readings

Format per phrase: the verdict, what in the corpus it rests on, and what would change my mind. Ranked by
where the phrase sits in `unblock_value.out`, not by how confident I am.

### 3.1 `domain`, 11 spans, rank 2. Already an axis. Closed, and the ranking should stop showing it

`ambient_domain` was declared from this exact rank and its `keywords` field includes
`domain`. The census still reports it undeclared because it matches slugs exactly and `domain` is not
`ambient_domain`. Nothing to decide.

**What would change my mind:** a span writing `domain` to mean something other than the mathematical domain
the values live in. I read all eleven; none does.

### 3.2 `domain closed under negation` and its variants, 6 + 1 + 1 spans. Values, not axes

`domain closed under negation`, `ambient domain closed under negation`, `ambient domain closed under
negation for the saturating arm`, `ambient domain one-signed`, `one-signed`, `domain containing a complete
residue system and the`. Every one is a constraint on which domain, written as prose after the axis word.
They are the values side of `ambient_domain`, and the splitter turns each into its own key because the key
extractor cuts at the first ` = `, ` in `, ` any` or ` >= ` and these carry none of them.

**This is the largest single source of inflation in the ranking.** Eight of the ninety-odd undeclared
phrases are one axis's values.

**What would change my mind:** a case where two of these must be stated together on one row, which the
once-per-axis lint would refuse. `122 #6` writes `ambient domain closed under negation for the saturating
arm` alone, so the qualification is inside the value rather than beside it, and one entry holds it.

### 3.3 `declarations` (7) and `restrictions` (2). One axis, and it is `operand_window`, whose grammar cannot spell it

The values are `one-sided`, `one-sided exhaustive`, `two-endpoint exhaustive at arity 2 and sampled at
arity 3`, `a uniform magnitude bound on every component`, and `upper bounds in {1, 3, 7}`. The surrounding
prose at `119:212` reads them as declared extents on the operands: "wherever every declared lower bound is
at or above zero, which properly contains the one-sided declarations `112` swept".

That is `dimension::operand_window`, whose `what` reads "whether the operands are declared to lie
within a restricted sub-range of the format's representable domain". **So this is not a new axis and the
repair is not a new row.** It is that the existing row's `grammar` at `:253` offers exactly two values,
`declared non-negative` and `full range`, and the corpus writes at least four more. A row wanting to say
`two-endpoint` has a declared axis and no admissible spelling on it.

**One thing to separate before porting.** These phrases fuse a region with a coverage statement:
`one-sided exhaustive and two-endpoint exhaustive at arity 2 and sampled at arity 3` says both which windows
the claim is about and how much of each was walked. The region half goes on `operand_window`; the coverage
half is now expressible, since `ruling::the_warrant_is_a_token_and_a_clause_on_the_values_side`
gives `exhaustive` in its `says` as a token with a clause naming the domain the span is the whole of.
That ruling is ratified and the split it enables is exactly this one.

**What would change my mind:** an instance where the declared window and the operand restriction are
independent, so a claim needs both. I did not find one in the nine spans.

### 3.4 `term shapes` (6) and `term` (2). An axis, missing, and I would declare it

This is the strongest positive I have.

`term = a left-nested fold` in `119 #11` and `122 #8` is a region: the claim is about a left-nested
association and not about any other. `term shapes = every term at 2 and 3 leaf slots over the signature in
play` is that same axis quantified over a set, with a sampled tail (`120 of 2025 sampled at 4` in `119 #8`)
that is coverage.

**It is measured, at fixed everything else, and the witness is committed.**
`82_probes/p2_output.txt:27-31`:

```
section 3: length-8 fold, left against balanced tree, width 8 signed, 4,000,000 samples
  declared [-128, 127] (straddles)     divergent   2544825 of 4000000 (63.6206%)
      witness [-126, 54, -45, -99, -29, 15, -36, 28] left=-100 tree=-128
  declared [0, 127] (non-negative)     divergent         0 of 4000000 (0.0000%)
```

Same width, same fraction width, same signedness, same overflow policy, same operation, same arity, same
chain length. The only thing that moves is how the eight operands are associated, and the answer moves with
it on 63.6% of samples. One witness would have been enough; four million is more than enough.

**No declared axis carries it.** `chain_length` is how many operations, and both arms of that measurement
have the same chain length. `arity` is operands per operation. `operation` is which operation. There is
nowhere to put "left-nested" today.

**And it is the axis every associativity law is quantified over.** `82:305-307` writes the lifting candidate
as "every parenthesisation of a fold over operands from `[LO, HI]` agrees". An associativity law is exactly
the claim that the answer does not depend on this axis, so a law stating that has to be able to name the
axis it is universal over. Under the absence rule, an associativity row that names nothing here says it
holds nowhere a term shape exists, which is nowhere a chain exists, which is the strongest negative in the
notation applied to the corpus's most-cited family of results.

The external literature agrees the mechanism is textbook rather than exotic: saturating fixed-point addition
is not associative once a partial sum clamps, so the evaluation order decides the result
([Wikipedia, saturation arithmetic](https://en.wikipedia.org/wiki/Saturation_arithmetic),
[Wikipedia, fixed-point arithmetic](https://en.wikipedia.org/wiki/Fixed-point_arithmetic)). That is
corroboration of the mechanism and not a second instance of the finding.

**What would change my mind:** if `chain_length` were intended to carry the shape as well as the length. It
is not; `dimension::chain_length`'s `what` says "How many operations run before a result is observed", which is a count.
Or if somebody shows the corpus never states a result at one shape and not another. It does, at 63.6%.

*holds for: signedness = signed, W = 8, F = 0, operation = add, overflow policy = saturate, arity = 2,
chain length = 8, operand window in {full range, declared non-negative, declared non-positive, [-1, 127]},
threads = 1, toolchain = whatever built `82_probes/p2`. Established as an existence claim (a witness is
printed), so the failure carries; the 63.6% rate does not carry past that cell.*

### 3.5 `F_exact`, `F_intermediate`, `F_final`, `staged narrowing`, 6 spans in the family. A real region the notation structurally cannot hold

The claim is `132`'s: "The nearest members make a staged narrowing depend on its staging" (`132:329-330`).
Its predicate is `F_exact in {4, 5}; F_intermediate in {2, 3}; F_final in {1, 2}` (`132:340`), and `136 #2`
writes the same thing as one phrase, `staged narrowing F_exact = 4, F_intermediate = 2, F_final = 0`.

**This one is not a missing row. It is a shape the notation cannot express, and there is a committed law row
that says so about itself.** `law::double_rounding_is_innocuous_at_an_intermediate_width_between_f_and_2f`
carries `fails = ["fraction_width: any"]`, and its
own `gap` at `:289-297` says that region "claims a failure only where no width, no signedness, no operation
and no container exist, which is not what the source means". The claim is about an intermediate width
strictly between `F` and `2F`. It has three fraction widths and the notation gives it one.

**Three entries on `fraction_width` is not available.** `mock/lints/a_predicate_names_an_axis_once.rs` is a
hard-error `RepoLint` and its own header says two entries for one axis "is neither of the two positive states
and is not absence either". So the repair cannot be "write `fraction_width` three times".

Two shapes are available and I prefer the second.

- **Three rows**: `intermediate_fraction_width` and `final_fraction_width` beside `fraction_width`. Cheap,
  and it does not generalise: `146:452`'s neighbour writes `accumulator width in {W, W+1, W+2, 2W}; fold
  length = 3` for what its prose calls an *accumulation schedule*, and `151:367` is blocked on the same
  thing. Two more rows there, and the next staged computation wants two more again.
- **One row whose value is the schedule**: `narrowing_schedule = 4 -> 2 -> 0`, `narrowing_schedule = direct`,
  `narrowing_schedule any`. It is what `136 #2` already wrote as one phrase before the splitter cut it, it
  keeps `fraction_width` meaning the declared format's `F`, and the same row shape absorbs the accumulation
  schedule.

**The general statement, which is worth more than either option.** The notation gives one value per axis per
claim. A staged computation has a *sequence* of values on one axis, and that is not a fourth region state,
it is a different arity of value. Three separate blocked clauses turn on it (`132:320`, `146:452`, `151:367`).

**What would change my mind:** somebody showing that `accumulator_width` was meant to carry a schedule
already. Its `what` says "How wide the intermediate a chain accumulates into is",
singular, and `146`'s own prose distinguishes the width from the schedule, so I read it as one value.

### 3.6 `overflow limit read at the declared width`, 1 sole blocker, 3 spans. Not an axis, and a ratified ruling already says so

The phrase is the container premise written as a coordinate. **The premise was dissolved.**
`ruling::behaviour_is_stated_per_declared_signature_and_the_premise_dissolves` (rung
`ratified`) says at `:1470`: "every operation the design declares is a function of the declared width and
never of the machine carrier, so arithmetic and encoding are stated over the declared width."

A dimension with one admissible value is a constant. Reading the overflow limit anywhere but the declared
width is not a situation arvo can be in, so nothing varies and no claim can be true at one value and false
at the other.

**This is the one place where the answer is worth money immediately.** `146:444`, clause 5.5c, "a lossless
storage container adds no answer function", is blocked in `blocked_inventory.tsv` by that phrase **and by
nothing else**. Every other phrase in its predicate maps onto a declared axis. So that clause is writable
today, with no declaration at all, by dropping a phrase that a ratified ruling has since made universal. The
inventory was taken before the ruling landed and nobody re-scored it.

**What would change my mind:** a committed row needing to state a live region on the container-stated branch.
The ruling's own `promotion` field discusses that branch (the MATLAB `fi` divergence), but it discusses it as
a refuted alternative, and the registry has a `retirement` namespace for exactly that. A refuted design is
not a region.

### 3.7 `discharge check` (2), `decision procedure` (2), `equality read` (1). Methods, not regions

`discharge check = root under a homomorphism and per node otherwise`, `decision procedure = exact vertex
enumeration over rationals`. These say how the check was carried out. They belong on the probe row, whose
`establishes` field already says a probe shows what it shows at the shapes it checked, or in the
`Argument kind` tag the spans already carry beside them.

**What would change my mind:** a claim that is true under one discharge check and false under another. That
would not be a region, it would be a bug in one of the checks.

### 3.8 `assignment set` (2) and `assignments` (1). A product of declared axes written as one phrase

`assignment set = rounding {floor, toward zero} x overflow {wrap, saturate both, saturate high only} x
intermediate {stepwise, exact}` (`146 #2`). Three factors, and all three are declared: `rounding`,
`overflow_policy`, and `intermediate {stepwise, exact}` which is `accumulator_width in {declared,
unbounded}`, spelled out as `accumulator width = unbounded` in the sibling span `146 #4`.

It ports by writing its factors as separate entries. It is a compression, not an axis.

**What would change my mind:** if the claim were about the assignment *space* rather than about a point in
it, in a way that a per-factor predicate cannot say. `146 #5`'s "the same assignment set at W = 4" is a
back-reference and not that.

### 3.9 `observation sets` (3). A genuine candidate, and it exposes an ambiguity in `operation`

`observation sets = every subset of {add, subtract, multiply, multiply-add, multiply-subtract}` is not the
same statement as `operation in {add, sub, mul}`. The claims in `146` §5.2 and §5.3 are about
*distinguishability*: whether two assignments can be told apart by a consumer, which depends on which
operations that consumer may call. A consumer holding only `add` cannot distinguish two policies differing
on `mul`.

**So `operation in {add, mul}` carries two incompatible readings and the registry does not say which.** For
a law it means "holds at each of add and mul", elementwise. For a distinguishability claim it must mean
"holds when the observer has both", over the set as one object. `dimension::operation`'s `grammar` gives it and its `moves`
`:121` says "A law is a statement relating operations", which is the elementwise reading, and 73 committed
entries name `operation` under it.

I cannot settle whether `observation_set` should be its own axis or whether `operation`'s values side should
carry the distinction, and I do not think one reading can. What I can say is that the ambiguity is live and
that it is a defect in a declared row rather than a gap in the set.

**What would change my mind:** a rule somewhere saying which reading `in {...}` takes. I grepped
`dimension.toml`, `ruling.toml` and the two lints and found none.

### 3.10 `coupling` (5). A candidate I would declare, on weaker evidence than 3.4

`coupling in {comonotone, independent}` and `coupling = any point of the Fréchet interval`. This is the
dependence structure between the rounding errors of different elements under stochastic rounding: whether
one random draw is shared across a vector or each element draws its own. It decides whether summed error
variance grows linearly or quadratically, which is the subject of `132` §5.6.

**It is a design choice an implementation sits at**, which is `accumulator_width`'s own stated criterion at
`dimension::accumulator_width`'s `moves`: "a design choice an implementation sits at a value of, which
is what makes it an axis
rather than a parameter of whichever run measured it". It is orthogonal to `rounding`: two implementations
both at `rounding = stochastic` differ in coupling. And the corpus already writes it in two of the
notation's three states, a fixed set and an `any`, which is what an axis looks like in use.

**Why weaker than 3.4.** I have not opened a measurement showing a claim true at one coupling and false at
another; I have the corpus asserting it. The variance law's dependence on coupling is textbook, but textbook
is not this corpus.

**What would change my mind:** finding that the stochastic rounding member is not shipping, in which case
this is an axis of a design that does not exist. `132` §5.8 and `138` §2 treat it as live.

### 3.11 The cost-model family. Split, and the split is the finding

`unblock_value.out` puts "the cost-model population" at 25 spans, second largest. It is not one thing.

- **`selector` (3 spans): an axis.** Values `linear` and the augmented Chebyshev form. `146:473`
  reports the plain selector picking a dominated arm at 11 of 41 weights and the augmented one at 0, so a
  claim is false at one value and true at the other. And it is a live design fork:
  `question::which_selector_does_the_design_ship_linear_or_augmented_chebyshev`
  asks "Does the strategy object's selector ship as a plain linear weighting, which reaches every hull
  vertex, or as an augmented Chebyshev form". `reference point` and `augmentation coefficient` are the
  augmented form's own parameters and belong on this axis's values side, not beside it.
- **`baseline` (2 spans): an axis.** `146:530` states the no-op condition at `baseline uniform across
  coordinates` and its prose at `:531-532` says every non-uniform baseline tried moved a selection. True at one
  value, false at others, and `:511` says the design declares which units a weighting is expressed in, so a
  shipped design sits at a value.
- **`cost coordinates` (7 spans): an axis, against the header's own parenthetical.**
  `dimension.toml`'s header, in the paragraph beginning "The test is what the value indexes", names "which
  cost coordinates it collected" as a parameter, on the ground that
  "nothing about the world is different at five arms rather than six". The corpus disagrees with that in
  use. `146:521` writes `cost coordinates any; arms any; weights any positive` with **Argument kind:
  order-preservation theorem**, which is the widest positive state plus a warrant; `146:525` writes
  `cost coordinates = 3; arms = 7`; `146:492` writes `cost coordinates in {2, 3}`. That is all three
  notation states on one phrase. A pure run parameter is not written `any` with a theorem behind it, it is
  simply not written. And `151:360-363` says "three cost coordinates is what `139` proposes", so it is a design
  choice, and `146:477` constrains what may count as one at all.
- **`arms` (6 spans): I decline, and the reason is not confidence.** The same evidence applies, but `arms`
  is a *cardinality* and the object is the arm set, which `146 #7` writes out in full as `arm set = {fused
  by widening, fused by partial products, stepwise by shift, stepwise by partial products}`. Two designs
  with seven arms each can differ in which seven. Declaring `arms` would declare a summary statistic and
  leave the thing it summarises unnamed, and the second declaration is the expensive one because the set is
  append-only. The real candidate is `arm_set`, and whether a region over a *set of lowerings* is
  expressible in a notation whose values are spans over scalars and named modes is a question I cannot
  answer from the corpus.
- **`weight grid resolution` (3 spans): coverage, not an axis.** `1/24`, `1/40 sampled every seventh
  point`, `1/12 on the 2-simplex`. Nothing about the world is different at 1/12 and 1/40; only how much of
  the weight simplex the run walked. This is the phrase the header's parameter test was written for, and it
  fits it exactly.
- **`cost tables drawn uniformly from integers 1..20`, `120 tables per cell`, `200 independent target
  pairs`, `no exact duplicate arms`: coverage.** Already on the `COVERAGE` list in `span_verdicts.sh`, and
  correctly.

**So the family is one coverage phrase, one statistic, and three axes, and the ranking prints it as one row
of 25.** That is the cost of the hand-written family regex.

### 3.12 `element count` (3) and `keying` (1) / `keying axis` (2). I cannot settle these

`element count any` appears three times, always beside `fraction = 1/3` and `coupling`. In `132 #9` the
claim is about summed-error variance over a set of elements, which reads as a fold length and would be
`chain_length`. In `132 #10`'s neighbourhood the same topic is about decorrelation across positions in one
cell, where the count is how many values share a placement, which is `occupancy`-adjacent and is not a fold
length at all. **The corpus uses the phrase for both and I could not find a sentence that decides it.**

`keying in {none, value, position}` is how a stochastic member's randomness is derived, and `keying axis =
one-dimensional` is the dimensionality of that key space. Both look like design choices. I have one span
each and no measurement showing a claim moving along either, so I would be guessing.

**What would settle them:** for `element count`, a single sentence in `132` §5.6 or `138` §2 saying whether
the elements are summed or merely co-resident. For `keying`, `136` section 8, which `136 #8` cites by name
and which I did not read closely enough to be worth a verdict.

---

## 4. The compile-time item, which I think is already closed and closed by a ratified row

The brief says a separate open item records that the notation has no region for a result that holds at
compile time only, so such a finding is written in a form that says it holds nowhere. I tested that against
the registry rather than against the sentence.

**The premise is false as stated, and the true situation is worse in a different way.**

**First, the notation does provide the region, and it was provided after the sentence naming the gap was
written.** `ruling::the_warrant_is_a_token_and_a_clause_on_the_values_side` (rung
`ratified`) gives a predicate entry the form `<axis>: <span>: <token>, <clause>`, with three tokens and no
fourth. One of them is `construction`, defined at `:1614` as "for an axis that cannot enter the argument at
all, with the clause saying what makes it unable to". That is precisely what a compile-time result needs:
`threads: any: construction, <the staging fact that keeps the thread count out>`.

The gap this closes is named in the corpus in exactly those terms. `dimension::access_pattern`'s `note` says a correctness
claim untouched by the access pattern writes `access pattern: any` "with the structural argument as the
warrant, exactly as a compile-time result writes `threads any`". `220:83-85` quotes that sentence back and
says of it: "That sentence names the thing, requires it, and has nowhere to put it." It has somewhere to put
it now.

**Second, nobody has used it.** Across the 527 committed predicate entries, **zero** carry `swept`,
`construction` or `exhaustive` on the values side (`229_probes/warrant_usage.out`, with a control that the
matcher finds entries at all: 527 matched, and 527 is the same number that ruling's own `note` reports from its
own independent count). So the mechanism exists, is ratified, and is unexercised.

**Third, and this is the part that is genuinely open: compile-time rows are not writing a false region, they
are writing none.** `proposal::a_folds_compile_time_refusal_is_the_staging_boundary_reporting_its_own_position`
is the corpus's clearest compile-time finding and it carries no `predicate` field at
all. It is filed `sentence_kind = "normative"`, and `mock/lints/a_region_agrees_with_the_sentence_kind.rs`
excuses `normative` and `definition` from carrying a region, on the correct ground that an imposition is not
established anywhere.

That escape is documented by a row that took it and said so.
`proposal::the_topics_form_a_stack_a_frame_and_the_canons_own_machinery` records in
its own note that it was "**Filed `normative` after being written `argument`, and the correction is worth
stating because it is forced rather than chosen**", because "**None can express a region over the canon's own
contents**, so a reasoned structural claim has no way to say where it holds".

**So the real mechanism is: a claim whose region the vocabulary cannot express gets refiled as an imposition,
and `normative` means "imposes rather than establishes".** A derived finding is being recorded as a decree
because there is nowhere to put its region. `mock/lints/an_imposition_rests_on_no_instrument.rs` guards the
obvious half of this and says why at its `:15-19`: filing a measured claim as an imposition "silently widens
it from the model width it was established at to everywhere, and it does that without touching the predicate,
where the widening would have shown up". It fires on a `normative` row carrying an `evidence` key. It cannot
fire on one that names its instruments in `because` prose and carries no `evidence` key.

I tried to measure that population and **my first arm failed its own control**, which is recorded rather than
repaired: `229_probes/normative_escape_first_attempt.out`. N2 required the one row known to be in the class
and the arm did not find it, because that row's `because` names a coordinator's derivation rather than an
instrument. So arm one measures a different set (21 rows filed under a region-free kind whose `because` reads
as resting on instruments, none carrying `evidence`), and I wrote a narrower arm two for the class itself,
with three passing controls: **7 rows in the registry say in their own prose that their region could not be
written** (`229_probes/normative_escape2.out`). That is a self-report and therefore a floor. A row that made
the same move without saying so is invisible to that arm and to everything else.

### What I would do about it, and it is not an axis

**The repair is not a new `dimension` row and not a new value.** Three things, in order of how sure I am.

1. **Write the compile-time rows with the ratified warrant marker.** `threads: any: construction, <clause>`,
   `target_features: any: construction, <clause>`, and so on for every machine axis the staging argument
   keeps out. This needs nobody's permission; it is ratified and unused. The one cost is real and stated at
   that ruling's `says`: a `construction` entry obliges the row's `evidence` to name an instrument that varied
   that axis and found no movement. For a staging claim that instrument is cheap (run it at two thread
   counts, two feature sets, and report no movement) and nobody has run it.
2. **The residue is `normative` doing duty for a third thing.** That same row says so plainly: either
   the vocabulary grows an axis over the canon's own structure, or the two region-free kinds are permanently
   covering a kind nobody named. That is a `sentence_kind` question rather than a `dimension` question, and
   it is not mine.
3. **The class generalises past compile time and
`ruling::the_additive_and_absorption_verdicts_are_canon` already names it in its `note`**: "the notation
has no
   inapplicable state and absence means the finding holds nowhere that dimension exists", written about an
   additive row that lists no `rounding` because addition at a common scale does not round. That is the same
   shape and the same answer: `rounding: any: construction, addition at a common scale never reads the
   scale`. An inapplicable axis is not a fourth region state, it is the widest positive state plus a clause
   saying why. Op's mechanism keeps its three.

*holds for: this registry at commit `0e71955b`. The warrant-token count is over the four predicate-bearing
files named in `canon_rows.rs:120-124` and nothing else. The 7-row figure is a floor over self-reporting rows
and is not a census.*

---

## 5. One thing the header says that I think is half wrong, and it matters for the bar

The header paragraph beginning "Extending the set does not reach backwards" corrects an earlier reading
and the correction is right: declaring an axis does not
reach backwards, because a predicate's absence quantifies over the world rather than over this file's
contents, so a row written before a declaration was always exactly as narrow as it now reads.

**That is true when the phrase declared is genuinely an axis. It is false when it is not, and the file draws
no distinction.**

If a phrase is declared that does not index a situation the world can be in, then every committed row that
omits it now says it holds nowhere that non-dimension exists, and there was no such narrowness before,
because the thing was not a dimension of anything. Declaring created it, retroactively, across 527 entries.
And it cannot be taken back: `:33-36` makes the set append-only, because deleting or renaming a row "turns a
written span into an unparseable one and a written absence into nonsense".

So the cost is asymmetric. **A correct declaration is free and a wrong one is permanent and retroactive.**
That asymmetry is the actual reason for the two-reading bar at `:38-39`, and the header states the bar
without stating the reason, having just spent nine lines arguing that declaring is free. A reader who takes
those nine lines at face value will conclude the bar is ceremony.

It also bears directly on 3.11: it is why I decline `arms` rather than taking it, and why I would take
`term_shape` (measured, with a witness) before `coupling` (asserted, not measured here).

---

## 6. Defects outside the question, reported because they were in front of me

**`dimension.toml`'s header claims a check exists and none does.** It says so twice, once in the opening
paragraphs and once in the block introducing the later rows: the set is append-only and
"a check pins it". I grepped `mock/lints/` and `mock/tools/` for `append`: zero hits, with a positive control
on a word that is there (`keywords`, three files) so the instrument is known to work. There is no lint, no
tool, and no schema arm enforcing append-only on `dimension.toml`. The sentence in the file says the opposite
of the situation, twice, in the one file whose whole subject is that a wrong sentence in a registry comment
is never contradicted by anything.

**`dimension.toml`'s header carries two stale counts, in the blind spot the count lint documents about
itself.** The file has 22 rows. One sentence says "Fifteen of the sixteen rows below cite that one file": fifteen
still do, and there are 22. Another says "The four below were not in the sweep that produced this file" and
seven rows sit below it. `mock/lints/no_registry_comment_counts_its_own_rows.rs:112` requires digits, and its
own doc at `:109` says "A spelled-out number is not caught, which is stated rather than hidden: this is a
tripwire on the shape that actually recurs, and a reader who writes 'seven rows' in words has gone out of
their way." Nobody went out of their way. The two stale counts are in the file that exists to say counts go
stale, spelled in the one form the tripwire does not see, and `cargo mock --lint-only` reports all lints
passed.

`proposal::the_topics_form_a_stack_a_frame_and_the_canons_own_machinery` has a third, in digits this
time: "All twenty-one declared dimensions". There are 22.
It escapes for two independent reasons: it is in a `note` field rather than a `#` comment, and its noun is
`dimensions`, which is not in the lint's `COUNTED` list.

**`183_probes/axis_census.sh` prints a claim that is now false.** Its C2 control message at `:119` reads
"radix: PASS, present as a bare key and no dimension row declares it". A `dimension` row declares it. The
arm only checks presence, so it still passes and still prints the false half.

**`mock/benches/variants/bitpack-write-contend-shared/src/stress.rs:97`** as covered in section 0.

None of these is in scope for the question I was sent. All five are cheap and all five are in files somebody
will quote.

---

## 7. Summary, and what a second reader should attack first

Twelve phrases, and the shape of the answer is that the ranking's top is mostly not axes.

| phrase | spans | verdict |
|---|---|---|
| `domain` and its seven value-variants | 19 | `ambient_domain`, declared. Values, not axes |
| `declarations`, `restrictions` | 9 | `operand_window`, declared. Its grammar cannot spell the values |
| `term shapes`, `term` | 8 | **An axis. Missing. Measured with a witness. I would declare it** |
| `F_exact` / `F_intermediate` / `F_final` / `staged narrowing` | 6 | A real region the notation cannot hold: one axis whose value is a schedule |
| `cost coordinates` | 7 | An axis, against the header's own parenthetical |
| `arms` | 6 | Declined. A cardinality; the object is `arm_set` and I cannot say if it is expressible |
| `coupling` | 5 | A candidate axis, on assertion rather than measurement |
| `selector` (+ `reference point`, `augmentation coefficient`) | 3 | An axis. The two others are its values |
| `observation sets` | 3 | Candidate, and it exposes a live ambiguity in `operation`'s `in {...}` |
| `overflow limit read at the declared width` | 3 | **Not an axis. A ratified ruling made it universal, and dropping it writes clause `146:444` today** |
| `or unsigned with signed` | 3 | A tokenisation artifact. Not a key |
| `assignment set`, `assignments` | 3 | A product of declared axes written as one phrase |
| `discharge check`, `decision procedure`, `equality read` | 5 | Methods. They belong on the probe row |
| `weight grid resolution` | 3 | Coverage |
| `baseline` | 2 | An axis |
| `element count`, `keying`, `keying axis` | 6 | **Could not settle. Named what would settle each** |

The three I would put to a second reader in this order, because they are where I am most likely to be wrong
and where being wrong is most expensive:

1. **`term_shape`.** The measurement is unambiguous and the naming is not. Whether the axis is the
   association shape of a fold, or the shape of an arbitrary term over a signature (which is what `119`'s
   `term shapes = every term at 2 and 3 leaf slots` is about), changes the grammar and the two may not be
   one axis. I think they are, and I have not proved it.
2. **`cost coordinates`.** I am contradicting the header's own worked example of a parameter. The corpus's
   usage is on my side and the header's test is arguably on its own. If a second reader disagrees, this goes
   to the human rather than to a third expert.
3. **The schedule shape.** One axis with a sequence value against several scalar axes is a design call about
   the notation, not a lookup, and the once-per-axis lint makes it consequential.

And one thing that needs no second reader, because it is a deletion rather than a declaration: clause
`146:444` is blocked by a phrase a ratified ruling has already retired, and porting it costs nothing.

---

## What I did not reach

I searched the web once, for prior art treating association shape as a stated dimension rather than as a
known hazard. What came back confirms the mechanism is standard and none of it names it as a coordinate a
claim is stated over, so it is corroboration of the physics and not a second instance of the finding. I did
not search for prior art on the cost-model axes or on staged narrowing, and I did not use the `web-search`
skill's separate budget at all.

I did not read `136` section 8, which is what would settle `keying`. I did not read the twelve governing
files end to end; I read every predicate span in all twelve, through the extractor, and the prose around the
twenty-odd spans I quote.

I did not look for the second reading of this question and do not know whether one exists.

Sources for the external corroboration in 3.4:
[Saturation arithmetic](https://en.wikipedia.org/wiki/Saturation_arithmetic),
[Fixed-point arithmetic](https://en.wikipedia.org/wiki/Fixed-point_arithmetic).

---

## A disclosure, and one that arrived after everything above was written

**A second seat is working this question and I have not read a word of it.** I learned it exists from a
process listing while waiting for a commit hook: a worktree at `.worktrees/axes2` is writing `230_probes`,
and its commit subject scrolled past. I did not open its directory, its file or its probes, and I will not.

Everything above was derived and committed before that. The probes landed at `0e71955b` and this file was
finished against them; the only thing I have seen of the other seat is a commit subject that names an
instrument rather than a verdict on any phrase. So this reading is uncontaminated, and I am declaring the
brush rather than leaving somebody to wonder.

**And it is why nothing could be committed for half an hour.** Five bench processes were running across
four worktrees, every one of them `bench_bitpack_write_contend_shared`, at 43 to 207 minutes of CPU each.
Seven cargo invocations held the shared package-cache lock, and every mockspace pre-commit hook on this
machine builds the lint pack in release before it will let a commit through. So the hung bench test from
section 0 is not a slow suite, it is **a machine-wide commit stall**: one `cargo test` that does not
terminate blocks every other agent's commit in every other clone, and the only sign is a `git commit` that
sits there.
