# 191. The surface above the numeral, derived cold

Cold derivation, first phase of a unit, written to be attacked.

My dispatch asked what the canon must state about the surface above a single numeral, on the
premise that a hundred and eighty files have gone into what a numeral is and not one sentence
reaches the algorithm surface. That premise is false in three separate places, and one of the
three is false in a way that changes what the unit should do. So section 0 comes first.

The short form of what I found. The derivation the brief says is missing has been done twice,
at length, with committed instruments, by `35` and `43`. What is missing is not the reasoning.
It is that the port kept those two files' **retirements** in full and dropped almost every
positive result they established, so the registry now records what the surface above the numeral
cannot be and says nothing about what it is. That is a filing defect rather than a research gap,
and it is the more dangerous of the two, because a corpus that has argued something and lost the
argument's conclusion reads exactly like a corpus that never argued it.

---

## 0. What broke in the brief

Three claims, checked before anything else. Two are wrong on the numbers and one is wrong about
the corpus.

### 0.1 "Five of the eleven are reached by no row anywhere." It is ten of eleven.

Under the schema, `refsto(obligation::<slug>)` reports rows carrying an `obligation` edge to that
slug (`mockspace.toml:930`). Two rows in the entire registry carry one:

```
proposal.toml:211                       obligation = ["composition_contracts_above_the_numeral"]
proposal-the-later-topics.toml:454      obligation = ["composition_contracts_above_the_numeral"]
```

Both point at the same slug. **Ten of the eleven obligations are reached by no row anywhere**, and
the one that is reached is reached by paper rather than by a ruling, which `187` says itself at
`187:164`.

The brief's "five" is `187` section 4's number read through the wrong instrument. `187` measured
something else and was careful about it: `187_probes/obligation_reach.sh` nets rows whose `says`
**mentions any of an obligation's own keywords**, and its own header calls it "a net, not a test".
Five obligations came back with no substantive keyword hit. That is a claim about vocabulary, not
about edges. Collapsing the two understates the gap by a factor of two and, worse, points the unit
at the five plan-chain obligations when the actual reach problem covers the numeral-facing ones
too: `a_platform_sized_unsigned_integer_at_an_api_position` has fifteen keyword hits and zero
edges.

### 0.2 The word list is `187`'s with one word added, and the search was scoped to three files

`187:158-160` writes: "The word 'Fiedler' appears in no `says` in the registry. Neither does
'topological', 'CSR', or 'content hash'." Four words. The brief has five, adding "adjacency".

The addition happens to be true and the scope is not. `187`'s instrument reads the `says` field in
`ruling.toml`, `proposal.toml` and `proposal-the-later-topics.toml`. Six namespaces were never
searched, and `retirement` is one of them. Re-run over every namespace and every narrative field
(`191_probes/reach_every_namespace.sh`, control planted and firing):

```
a_spectral_partition_of_a_dependency_graph   2 non-self hits, both in retirement.toml
ordering_a_directed_acyclic_graph            9 non-self hits, in retirement, question, ruling, proposal, probe
a_content_hash                               4 non-self hits, all in retirement.toml
a_sparse_adjacency_a_plan_can_be_built_on    0
set_operations_over_a_fixed_size_bit_set     0
```

The word "spectral" occurs four times in `retirement.toml`, in two rows whose subject is the
algorithm crates and the laws they need.

### 0.3 "Not one sentence reaches these." The corpus reaches them repeatedly, and always sideways

This is the one that matters. The registry contains, today:

- `retirement.toml:1145`, which measures **shortest path on a directed acyclic graph** wrong on
  12.6 percent of 622 million in-range instances under a proposed overflow policy, names
  **min-plus** and **the tropical algebras** explicitly, and kills the policy on that evidence.
- `retirement.toml:42`, which retires gating the graph, combinatorial and spectral crates on an
  associativity fact, on the ground that associativity and the distributivity those algorithms need
  are different laws that invert across the same presets, and names **a monotonicity marker over
  addition** as the atom the gate was reaching for.
- `retirement.toml:532`, which retires a design-wide verdict about the algorithm crates' outputs as
  "correct only for the spectral crate".
- `retirement.toml:483`, which finds the accuracy-first preset's exile from the algorithm crates was
  never wrong and locates the defect in **the crates' own return type**.
- `law.toml:218`, whose witness measures that "saturating addition and **min** admit some" compatible
  total orders where wrapping addition admits none. `min` is the tropical additive operation.
- `question.toml:609`, **Q33**, whose first option reads in full: "Fixed: the concept describes one
  arithmetic ... and the tropical semiring the algorithm crates compute in is described by the
  algorithm crates. Cost: the named selling point computes in something the canon does not cover."

That last one is the brief's own dichotomy, already written down. The brief says "Either the canon's
scope excludes them and nobody wrote that down, or the work went somewhere else." **It is written
down.** It is one arm of an open panel question, with its cost stated in the same sentence, and it
has been sitting in the registry since the options register was ported.

**What is true, and what the brief was reaching for, is narrower and sharper.** Every row above
carries a numeral topic: `overflow_policy`, `algebraic_laws`, `the_number_system`, `the_format`.
`191_probes/where_the_algorithm_results_are_filed.sh` checks this across twelve terms with a
positive and a negative control, and the only row in the corpus that reaches the algorithm surface
and carries `topic = "arvo_identity"` is an obligation, which is the demand side rather than the
canon side. **No row in any namespace has the surface above the numeral as its subject.** The
corpus reaches it constantly, as a test instrument for numeral questions, and never turns round to
face it.

### 0.4 And one more, about `184` rather than about the brief

`184:106` and `184:133` record `arvo::UWire` as named by vehje "with no statement of what it is for"
and file it as a gap. Two consumers state the use at length:

> A future arvo primitive `UWire<N>` (fixed backing for exactly `N` wire bits, no `Strategy` axis)
> would collapse the second tier back into the first.
> `hilavitkutin/mock/crates/hilavitkutin-extensions/DESIGN.md.tmpl:58`

> A future `arvo::UWire<N>` primitive (see arvo BACKLOG) provides fixed-width wire-stable newtypes
> without a `S: Strategy` axis and flips the bare sites.
> `vehje/mock/crates/vehje-runtime-abi/DESIGN.md.tmpl:90`

Both give the reason: `UFixed<I, F, S>` is transparent over a backing whose width comes from the
strategy and is therefore not pinned to a C integer width, so an FFI descriptor cannot accept it.
That is a demand with a stated mechanism and a stated cost, reached independently by two consumers,
and it is currently filed as an absence.

The cause is scope. `184` says it read hilavitkutin at `mock/crates/*/DESIGN.md.tmpl` and vehje at
`mock/DESIGN.md.tmpl` only. `191_probes/the_demand_side_below_the_top_level.sh` counts what that
missed: vehje's top-level design names arvo on 3 lines and its eleven per-crate designs name it on
19 more. Its two controls behave (kolli zero, notko zero).

---

## 1. What I read, and what I did not

**Exhaustively.** `184`, `187`, `181`, `INTENTS.md` I10 through I17 at source, `mock/registry/`
whole for the searches in section 0 and by row for every citation I make. `35` sections 1 through 8
and its probe listing. `43` section 0 and its section headings. `hilavitkutin/mock/DESIGN.md.tmpl`
plan-chain section, `hilavitkutin/mock/crates/hilavitkutin/DESIGN.md.tmpl` steps 1 to 13,
`hilavitkutin-api`, `-extensions`, `-linking`, `-persistence` at every arvo-naming line.
`vehje/mock/crates/*/DESIGN.md.tmpl` at every arvo-naming line. `kolli` and `tarina` whole for arvo.

**Not read.** The panel's own 180 files apart from `35`, `43`, `184`, `187`, `181`. `35`'s and
`43`'s probe sources; I cite their conclusions from their files and their probe directories are
committed, which I checked. The design_rounds tree. The deleted crate tree, deliberately, per my
brief and per `the-canon-design-code-chain.md`; the one place I record something from it is `35`'s
own quotation of two trait bounds, and I mark it as `35` marked it.

**Cloned to read.** `tarina`, which `184` names as unread and as the consumer most likely to demand
something nobody enumerated.

---

## 2. The derivation

### 2.1 The work exists twice, and the port kept the corpses

`35_mcsherry_what_the_layers_above_need_from_the_numeral.md` is 884 lines and is a cold derivation
of exactly my question. `43_rompf_what_a_composition_is.md` is 934 lines and is a cold derivation of
the other half of I11. Between them they answer the dispatch. What reached the registry is the
question.

I measured it. `191_probes/which_of_35s_figures_survived.sh` takes fourteen figures from `35`
sections 3.4, 3.5 and 3.5a and asks whether each string occurs anywhere under `mock/registry/`,
with three controls: two figures known present, one known absent, and one of `35`'s own figures as
the proof that a `35` figure *can* survive.

```
ABSENT   63 of 63       absorption holds at every cell under saturation
ABSENT   0 of 63        absorption holds at no cell under wrapping
ABSENT   33 of 33       monotonicity holds at every cell under saturation
ABSENT   33.07          worst-case triples under wrapping
ABSENT   5,414,255      shortest path wrong, w=3, wrapping
ABSENT   407,293,133    shortest path wrong, w=4, wrapping
ABSENT   736,300,800    in-range instances, longest path, w=4
ABSENT   832,398,764    in-range instances, shortest path, w=4
ABSENT   45.4           percent wrong, w=3
ABSENT   48.9           percent wrong, w=4
present  12.6           reserved top, percent still wrong
present  560 of 2176    reserved top, monotonicity failures
ABSENT   680 of 2176    wrapping monotonicity failures
ABSENT   78.2           reserved top, instances
```

**Two of fourteen survived, and both are inside `retirement.toml:1145`, the row that kills the shape
`35` proposed.** Every figure establishing the positive requirement is gone: the absorption split,
the monotonicity split, and the whole end-to-end table with its in-range control. What the canon
now knows about the tropical algorithms is that one hybrid does not work.

Same instrument on `43`, at `191_probes/which_of_43s_figures_survived.sh`. **Zero of seven
survived.** That run also caught its own defect and I have kept the transcript: version one used a
bare substring grep and reported `4096`, `58` and `94` present, all three false, matching a law's
"4096 triples", the digits inside `12,582,912`, and the file citation `94:887`. The rewritten arm
prints the surrounding text for every hit so a reader can see it.

Of `43`'s concepts, one survives: "per-aggregate", inside
`proposal.toml:443`, `each_choice_in_the_sequence_has_an_owner_and_a_resolution_time`. The
capacity-against-count boundary, the `len <= capacity` invariant, the flattening result and the
assembly read are all absent.

**This is the finding I would put in front of op before anything else in this file.** Not "the panel
never looked at the algorithm surface", which is false, but "the panel looked at it twice, and the
canon retains the two negative results and none of the positive ones." A reader of the registry
today can learn that a reserved absorbing top fails and that gating on associativity is wrong. They
cannot learn that saturation absorbs at 63 of 63 cells and wrapping at 0 of 63, which is the fact
those two rows are both consequences of.

### 2.2 The six needs are not six subjects, and they are not one either

My brief asks what a bit set, a graph ordering, a sparse adjacency, a partition, a cost program and
a hash have in common. Read the plan chain for **what kind of number flows through each step**
rather than for the algorithm's name, and it splits five ways.
`hilavitkutin/mock/crates/hilavitkutin/DESIGN.md.tmpl` steps 1 to 13:

**Class A, pure index and set combinatorics.** Steps 1, 2, 4, 5, 6, 11, 12, 13. Build the graph from
mask overlap, topologically sort and renumber, detect waists, reorder for bandwidth, decompose into
blocks, classify columns, propagate dirty masks, classify shapes. Nothing here is arithmetic in the
numeral's sense. The values are node identifiers, offsets, counts and set memberships. What these
steps want from arvo is a **bounded index whose width is a capacity** and **set operations over a
fixed-size bit set**, and the consumer states the cost it expects for the second: "All scheduler ops
= single-instruction bitwise" (`hilavitkutin-api/DESIGN.md.tmpl:156-157`).

**Class B, semiring arithmetic over weights.** Step 3, upward rank into a critical path, which is
max-plus. Step 8's grouping and the cost dynamic program, which is a budgeted optimisation over
min-plus or a knapsack. These have laws, and the laws they need are the ones `35` measured:
absorption of a top and monotonicity of addition. Neither is associativity, which is what
`retirement.toml:42` says the abandoned gate got wrong.

**Class C, real-valued numerical linear algebra.** Step 7 only. A symmetric Laplacian and a Fiedler
vector. Convergence, conditioning, and a determinism question none of the other classes has.

**Class D, ordinary numeral arithmetic the consumer writes by hand.** Step 9, and it is worth
quoting because it is the only step in the chain arvo's canon currently describes:

> `window = (L1_usable / Σ write_sizes).clamp(MIN_MORSEL, MAX_MORSEL) & !3`

A division, a sum, a clamp and an alignment mask. This is a numeral expression and nothing more.

**Class E, a mixing function, and it is the one that breaks the pattern.** The content hash.
`hilavitkutin-persistence/DESIGN.md.tmpl:193` names it: a 28-bit FNV identity. FNV is
multiply-and-xor in a wrapping ring folded to a width that is not a multiple of a byte. So it
wants three things at once: **wrapping multiplication, deliberately**, which is the policy class B
needs excluded, measured at 45.4 and 48.9 percent wrong answers; **xor**, which `law.toml:218`
groups with wrapping addition as admitting no translation-invariant order at all; and an **exact
28-bit width**, which is arvo's identity and the reason the consumer reached for arvo at all.

And its correctness criterion is not arithmetic. The obligation says "stable across sessions", so
what "right" means for class E is **bit-exact reproducibility across builds**, and nothing else.

**That puts two of the eleven obligations in direct tension, and no row says so.**
`a_content_hash` needs the same bytes to hash the same in every build.
`a_build_flag_that_changes_float_semantics` is a cfg a consumer's build system sets under which
arvo's semantics change between builds. Its own `gap` field already records that a compile-time
check cannot catch the hazard, because each build emits one lowered path and satisfies the check
while the denotation differs. The two obligations come from **the same consumer**, and one of them
is the mechanism by which the other fails. That is not an argument that either is wrong. It is a
conflict the demand side contains and the canon has never been shown.

I want to bound this honestly: the cfg is a float flag and FNV is integer, so the two do not
collide today at the operations either one names. What collides is the shape. A consumer that
keys a cache on a computation and a build system that changes what a computation means are in
conflict for reasons that have nothing to do with which operations are involved, and a canon that
admits the second owes a statement about which surfaces the first may key on.

**So the answer to "what do the six have in common" is: three of them have nothing in common with
the numeral at all.** The bit set, the ordering and the sparse adjacency are class A. They ask arvo
for a bounded index and a mask, and the entire numeral canon, every law, every rounding mode, every
overflow policy, every strategy, is inapplicable to them, because there is no arithmetic to have a
policy about.

That is the structural fact I would build the canon's answer on, and it has a consequence the brief
did not anticipate: **the canon owes the classification before it owes any contract.** A canon that
says "arvo ships graph ordering" has said nothing useful, because the ordering step and the rank
step want different things and one of them wants nothing the numeral concept has.

**Two readings, and what would distinguish them.** The first is that A, B, C, D and E are five
different obligations wearing one consumer's crate names, and the canon owes five sentences. The
second is that A and D are one obligation (both are numeral use, one degenerate) and B and C are one
obligation (both are algebras the numeral must be a member of), and E is neither, so the canon owes
three. The distinguisher is whether a bounded index is a numeral: if the index type is
`UFixed<N, 0, S>` then class A is degenerate numeral use, and if it is a separate concept with no
strategy axis then it is not. That is Q26's territory (`what_a_platform_width_type_is`) and section
2.4 argues it is not a numeral.

### 2.3 The two composition contracts have opposite requirements, and the axis is a binding time

I11 asks for "the composition contracts for units bigger than a numeral". The registry contains two
results about composition and **they contradict each other on the operator**, and nothing reconciles
them.

`proposal.toml:860`, `chain_accuracy_cannot_be_served_by_an_operator_closed_over_its_operand_type`,
standing `cross_topic`, which is the strongest this panel produces: three separately dispatched
topics computed it independently and none cites the other two. Chain accuracy **requires a
non-closed operator** and the intermediate width grows linearly in chain length.

`35` section 3.1, from a contract test that does not compile: a fold's operation **must be closed**,
because a loop-carried accumulator has exactly one type. Four formulations of a widening fold refused
by rustc with the identical diagnosis, against two positive controls where the arity is a
compile-time fact. `35:97`: "**The boundary is the runtime trip count and nothing else.**"

Both are right and they are not in conflict, but the sentence that dissolves the conflict is
nowhere. **A chain and a fold are two compositions with opposite requirements on the operator, and
what separates them is whether the length is known at compile time.** A chain is a bounded
expression: it can widen at every step and adapt once at the end, which is what
`proposal.toml:204` calls a schedule of adaptation points. A fold is a loop over an aggregate whose
count is dynamic: it cannot widen at all, so it needs a closed operation and a separately sized
accumulator, and `43` locates the same boundary from the other side as capacity against count.

I searched for the positive statement of this. It exists in the registry as **keywords only**:
`trip count` appears in `question.toml:222` (Q11's keyword list), in `retirement.toml:114`
(`dl_growing_an_accumulator_type_per_iteration`, the negative half), and in one unrelated row about
a static-length lever's noise floor. No `says`, no `claim`, no `statement` carries it.

`35:99-102` says what that absence costs, and I would put this sentence in front of op verbatim:

> So the design's width algebra is available in expressions and unavailable inside every loop in the
> layer above. That is not a defect in the algebra. It is where the algebra's domain ends, and a
> canon describing the algebra without describing that boundary **will be read as describing
> something the algorithm crates can use.**

**This is what a composition contract is, and it is the answer to the first question my brief
asks.** Not a trait, not a container. It is a statement of which of two staging regimes a unit is
in, because that decides whether widening is available, and everything else follows.

### 2.4 The strategy axis does not survive above the numeral

My brief asks what the strategy axis means above the numeral. Three regions, and each has an
existing measurement or an existing consumer statement behind it.

**Where there is no arithmetic, the axis has no content.** `hilavitkutin-api/DESIGN.md.tmpl:915`
types each scheduling-hint discriminant as "the narrowest arvo `UFixed<N, 0, Hot>` alias that fits
the axis", and the table's first row is `Urgency = UFixed<2, 0, Hot>` over four marker types with
discriminants 3, 2, 1, 0. There is no operation on an urgency. `Hot` there is what the alias table
offered, not a weighting anybody made. Every class A quantity is in this region.

**At a wire boundary the axis must be absent, not chosen.** Both `UWire` statements in section 0.4
say it in the same words: the strategy is what makes the backing width unpinned, and a `#[repr(C)]`
descriptor cannot accept the variability. So the demand is not for a strategy that happens to give a
fixed width. It is for a numeral-shaped thing **with the axis removed**.

**In the tropical algebras one strategy is excluded by measurement.** `proposal.toml`,
`no_total_join_exists_over_the_observable_axes_so_the_operation_reports`, in its `because` at line 887,
measures that "the absorbing top is a law only a lossy policy has, so an accuracy-first policy is not
the top of the law order and no choice of inventory rescues it", and `retirement.toml:483` closes the
matter: the accuracy-first preset's exile from the algorithm crates "was never wrong". Class B cannot
select `Precise`. Not by preference, by absence of the law it needs.

**So the honest general statement is a negative one and it is checkable:** the strategy axis is a
property of an operation's realisation, and above the numeral it applies exactly where an operation
happens. Where the composition carries indices it is vacuous, where the composition crosses a wire
it is harmful, and where the composition computes in a tropical algebra it is partial.

I want to flag that this is the section I am least sure of, because all three regions are established
from a single consumer plus existing registry rows, and none of the three has been attacked. The
second reading available is that the axis does apply everywhere and what varies is only which member
is admissible, which would make the wire case a missing member rather than a missing axis. What
would distinguish them: whether a consumer ever wants two different strategies on the same wire type,
which nothing I read answers.

### 2.5 A runtime check op's I15 forbids, and whether it can be removed

`hilavitkutin/mock/crates/hilavitkutin/DESIGN.md.tmpl:52`, step 2:

> A registration whose `PlanDims` declares more phases or trunks than the fixed-width `PhaseId` /
> `TrunkId` can name is rejected up front as `PlanError::PhaseCapacityExceedsIdWidth` ... so an
> over-wide dims fails loudly instead of silently wrapping ids past the addressable range.

Both quantities are compile-time constants. `INTENTS.md` I15 is op's, `STATED`:

> Never any runtime checks, ever. We catch invalids on compile time, and unused paths we clear out
> when lowered. Period.

**So the heaviest consumer's algorithm surface violates op's own intent, at exactly the place where
arvo does not supply the concept that would make the check compile-time.** That is a demand nobody
enumerated and it follows from putting two of op's statements together rather than from anything a
consumer wrote.

I did not stop at reporting it. `191_probes/p1_capacity_against_id_width.rs` models a bounded index
and a capacity in miniature, the way `35_probes/p1` models the width algebra, and asks which
spellings refuse. Six arms, three required to compile and three required to be refused.

The first run reported three arms refused and looked right, and it was wrong: `-o /dev/null` makes
rustc fail creating a temporary beside its output, so every arm died for the wrong reason. **The
only thing that caught it was the base arm, which must compile and did not.** The transcript is
`p1_v1_devnull.out` and I have kept it, because a probe whose arms all expect refusal cannot tell a
refusal from a broken invocation.

The corrected run, `rustc 1.98.0-nightly (57d06900f)`:

```
BASE   fitting capacities                     COMPILED   as required
B      monomorphic, 300 into 8 bits           REFUSED    E0080, evaluation panicked
C-bad  generic, called at 300 / 8             REFUSED    E0080, at PlanC::<8, 300>::_CHECK
D      the where-clause spelling              REFUSED    generic parameters may not be used
                                                         in const operations, names
                                                         generic_const_exprs
F      associated-const gate, called          REFUSED    E0080
```

Arm D reproduces `35`'s section-6 droplist entry at a different spelling with the same diagnostic,
which is a second instance of a result that had one.

Then the case that had to fail before any of that counted. An inherent associated const is evaluated
where it is used, so I named the invalid instantiation three ways without touching the const:

```
G1     bad type returned from a function      COMPILED   *** the mechanism is a landmine ***
G2     bad type behind a type alias           COMPILED
G3     bad type held in a struct field        COMPILED
```

**The const-assertion route is not a refusal.** `PlanC::<8, 300>` is a nameable, returnable, storable
type and nothing complains until somebody forces the constant.

So I attacked that. `191_probes/p2_definition_site_refusal.rs`: const comparison of two const
parameters is what rustc refuses, and trait resolution over types is not, so carry the capacity as a
**type** in binary and make "fits in W bits" a structural relation the trait solver decides. No const
arithmetic anywhere, no feature gate.

```
H1  capacity 200 in 8 bits, positive control   COMPILED   as required
H3  capacity 300 in 9 bits, positive control   COMPILED   as required  (measures width, not size)
H2  capacity 300 in 8 bits, in a signature     REFUSED    E0277, B1<E>: FitsIn<Z> not satisfied
H4  capacity 300 in 8 bits, only in a field    REFUSED    E0277, same
H5  capacity 300 in 8 bits, only aliased       COMPILED   *** left red on purpose ***
H6  the alias then held in a field             REFUSED    E0277
H7  the alias then used in a signature         REFUSED    E0277
```

**H4 is the case `p1`'s G3 could not reach.** A type-level relation refuses at every position that
uses the type, including one that only names it in a field.

H5 is the residue and I have left the arm red rather than relabelling it. A bare `type` alias
compiles, because Rust does not check bounds on aliases at all: that is `type_alias_bounds` and it is
general to aliases rather than anything about this relation. H6 and H7 bound it: the moment the alias
is used the refusal arrives with the same diagnostic. So it is a deferral to the use site, not a hole.

**The composition, which is the deliverable rather than a winner:**

| mechanism | refuses at | costs |
|---|---|---|
| const assertion in an inherent const | only where the constant is forced | nothing, and it is a landmine: `p1` G1-G3 |
| type-level relation over binary naturals | every position that uses the type | the capacity is a type rather than a const, so a consumer writing `300` writes nine type constructors, and trait-solver depth at large capacities is unmeasured |
| const comparison in a `where` clause | nothing, refused terminally | a forbidden feature |

**What this establishes for the canon.** The bounded index concept is expressible with a
definition-site refusal, gate-free, on the pinned toolchain, so the canon may commit to the intent.
What it may not do is commit to the const-parameter spelling, which is the obvious one and is the one
that does not work.

**What it does not establish, and I want this on the record rather than buried.** I did not measure
compile time, trait-solver depth, or behaviour at capacities near the pointer width. `35` section 8
flags the same for its own `ceil(log2)` and calls it unpriced; mine is unpriced for the same reason
and by the same standard. And this is an ad-hoc compile spike, not a bench: it establishes that a
refusal is expressible and it prices nothing.

### 2.6 What the algorithm surface asked of the numeral, when it existed

`35:62-63` records the two trait bounds the shipped algorithm crates carried, and marks them as
corroboration it is deliberately not counting because they are one dead artifact. I repeat the mark
and use them only for the one thing they can support, which is a scale:

```
arvo-graph/src/path.rs:36      W: Add<Output = W> + TotalOrd + Copy + FromConstant
arvo-spectral/src/power.rs:44  F: Add + Mul + Sqrt + Recip + TotalOrd + Copy + FromConstant
```

Every operation closed, no widening anywhere. Seven distinct bounds across the whole surface.

I raise it because of the asymmetry it names, and the asymmetry survives the artifact being dead.
The numeral canon has fourteen laws in `law.toml`, six more in `law-the-later-topics.toml`, twenty
topics and 543 rows. **The surface I11 calls the selling point asked for seven things, and the
canon has a measured statement about one of them**, the total order at `law.toml:218`. There is no
law about `min`, none about `max`, none about monotonicity, none about absorption as a fold
requirement rather than as an exponent identity, and none about `Sqrt` or `Recip` at all.

That is not an argument that the canon is wrong. It is the shape of the gap stated in the canon's own
units, and it is why I think the answer here is small rather than large: the demand side is seven
bounds and two staging regimes, not a library of algorithms.

---

## 3. Which rows I think the canon owes

Stated as rows because my brief asks for that shape. **I am not writing any of them and none of this
is settled.** Each carries what kind of sentence it is and the region it holds in, or why it has none.

### 3.1 In `topic`

**One new topic.** Every algorithm-surface result in the corpus is currently filed under a numeral
topic, which is what makes them invisible (section 0.3). `arvo_identity` is about what arvo is and
`the_chain` is about composition beyond a single operation; neither is about the demand the algorithm
layer places on the numeral.

- `the_layer_above`, or whatever it ends up called. What it is about: what a composition over
  numerals requires of the numeral, and which of those requirements the numeral concept can carry.
  **No predicate**; a topic is a subject.

I flag the cost, because it is real: adding a topic re-files existing rows, and re-filing a row is a
change to a document later rows were written against. The cheaper alternative is to leave the rows
where they are and add the edges, which section 3.5 covers. I do not know which is right and I think
it is the unit's call rather than mine.

### 3.2 In `proposal`, the statements the port dropped

Four, each recovering a positive result the registry lost. All four are `sentence_kind = "measured"`
and each carries its instrument, which is committed.

- **`saturation_supplies_the_absorbing_top_and_wrapping_supplies_none`.** The top absorbs under
  saturation at 63 of 63 cells and under wrapping at 0 of 63. Instrument
  `35_probes/p4_identities_and_absorption.rs`. Predicate, from `35`'s own sweep and no wider:
  `total_width` and `fraction_width` over the 63 swept cells, `overflow_policy in {wrap, saturate}`,
  `operation = add`, `threads = 1`.
- **`addition_is_monotone_under_saturation_and_not_under_wrapping`.** Holds at 33 of 33 cells under
  saturation, fails at 33 of 33 under wrapping, to 33.07 percent of triples, both signednesses.
  Instruments `35_probes/p2_laws.rs` and `p2b_laws_signed.rs`.
- **`min_plus_needs_absorption_and_monotonicity_and_max_plus_needs_neither`.** The end-to-end DAG
  table with its in-range control. Shortest path wrong on 45.4 percent at width 3 and 48.9 percent at
  width 4 under wrapping, zero under saturation; longest path correct under both at 736 million
  in-range instances. Instrument `35_probes/p5_algorithm_end_to_end.rs`. **This is the row that makes
  `retirement.toml:1145` legible**, because that retirement is a consequence of it.
- **`the_two_properties_are_two_and_the_policy_names_neither`.** `35` section 3.5a's conclusion:
  a policy question asks which of wrap or clamp, and what the layer above needs to know is whether the
  top absorbs and whether addition is monotone, which are separable and one is buyable without the
  other. This is the one that bears on Q6.

### 3.3 In `proposal`, the statements neither derivation wrote down

- **`a_fold_and_a_chain_want_opposite_operators_and_the_axis_is_the_trip_count`.**
  `sentence_kind = "normative"`, and it is the reconciliation of `proposal.toml:860` with `35`
  section 3.1. A composition whose length is a compile-time fact may use a widening operation and
  adapt once; a composition whose count is dynamic may not, and needs a closed operation with a
  separately determined accumulator. **No predicate on width or policy**, because the mechanism is
  the type system rather than the arithmetic; the evidence is a compiled refusal at eight arms with
  two positive controls, `35_probes/p1`.
  I would put this first of everything in section 3. It is the sentence `35:99-102` says the canon
  cannot omit without being misread.

- **`a_bit_exact_reproducibility_demand_and_a_build_flag_are_in_conflict`.**
  `sentence_kind = "normative"`, and it is section 2.2 class E. A consumer keying a cache on a
  computation and a build system changing what a computation means are in conflict whatever
  operations either one names, and both are already obligations from the same consumer.
  `a_content_hash` and `a_build_flag_that_changes_float_semantics`, which the row would carry as
  edges to both. **No predicate**: it is a statement about two obligations rather than about an
  arithmetic region, and it is derived rather than measured, which its `sentence_kind` should say.
  What it does not do is settle either one. `184` already declines to endorse the cfg surface and
  I decline with it; what I am adding is that the demand side is not merely un-enumerated here, it
  is internally inconsistent, and a canon serving both without saying so serves neither.

### 3.4 In `obligation`, the rows `184` did not reach

Five, from the per-crate documents section 0.4 shows were not read. Written as needs, no crate names.

- **`a_wire_stable_container_with_no_strategy_axis`.** A container of exactly N bits whose backing is
  pinned to a C integer width, usable in a `#[repr(C)]` descriptor. Consumers: hilavitkutin and vehje,
  independently, each with the same stated reason and each carrying `lint:allow` annotations at every
  site until it lands. **This replaces `184`'s gap entry.**
- **`a_bounded_index_whose_width_is_a_capacity`.** An index type whose declared width names how many
  values it can carry, where declaring a capacity above it is refused. Consumer: hilavitkutin, which
  states the need as a runtime error today. It is the one obligation that op's I15 makes mandatory
  rather than merely wanted.
- **`arithmetic_available_in_a_const_context_over_a_recursive_type`.**
  `Cons<H, T>::LEN = USize(T::LEN.0 + 1)`, "callable in const contexts via the const-arith surface"
  (`hilavitkutin-api/DESIGN.md.tmpl:522`). Not the same as const construction: it is arithmetic
  through a recursion whose depth is the aggregate's.
- **`named_quantities_at_public_positions`.** `ByteCount`, `Alignment`, `CoreCount`, `Nanoseconds`
  (`hilavitkutin-api/DESIGN.md.tmpl:971`). A units demand. Nothing in the registry is about units and
  I have no view on whether it is arvo's; section 4.
- **`a_semiring_a_graph_algorithm_can_compute_in`.** Op's, derived rather than quoted, and the one
  I11 implies that no consumer document states, because the consumers state it as five crate names.
  Q33 is its open form.

**And one correction rather than a new row.** `184`'s kolli gap resolves: kolli names arvo on zero
lines anywhere under `mock/`, measured with a control. And its tarina gap resolves in the opposite
direction from what it expected; section 4.

### 3.5 In edges, which is the cheapest thing in this file

Ten of eleven obligations are reached by nothing (section 0.1) and several are reachable now. At
minimum: `retirement.toml:1145` and the three proposals in 3.2 reach
`a_spectral_partition_of_a_dependency_graph` and `a_cost_dynamic_program` through class B;
`law.toml:218` reaches both; `proposal.toml:204` and `860` reach
`composition_contracts_above_the_numeral` alongside the two that already do.

I have not written the edge list because I have not opened both sides of every pair, and `187`
section 5 is a list of exactly the edges somebody asserted without doing that. Whoever writes them
should read both rows first, and the count they produce is the honest measure of how far the corpus
reaches, which is currently one.

---

## 4. What is not arvo's

Three things, and one of them is the most interesting single artifact I found.

**Class C, the spectral step, is probably not arvo's, and the consumer already says so.** Step 7
(`hilavitkutin/mock/crates/hilavitkutin/DESIGN.md.tmpl:64-67`) reads: "Spectral partitioning via an **engine-local symmetric Laplacian** over arvo-spectral's k-way
partitioning". The consumer builds the Laplacian itself and asks arvo only for the partitioning. A
Laplacian is a graph-shaped object, its construction is domain knowledge about what the graph means,
and the engine has that and arvo does not. What is left for arvo is an eigensolver, which needs
`Sqrt` and `Recip` that nothing else in the demand side wants, and a convergence criterion that is a
numerical-analysis question rather than a numeral one. I would put the whole of class C outside and
say so, and I would expect that to be argued with.

**Units are not obviously arvo's.** `ByteCount`, `Alignment`, `CoreCount`, `Nanoseconds` are four
quantities from one domain. A nanosecond is not a numeral property. The reading I would defend is
that arvo owes the mechanism by which a consumer builds a named quantity that is transparent over a
numeral, and owes none of the four names. The reading against it is that `USize` and `Cap` are
already exactly that and arvo already ships them, so the line is somewhere else than where I put it.

**And tarina, which `184` predicted would demand something nobody enumerated.** It names arvo on zero
lines. What it says instead, under a heading called "What is deliberately not being built":

> No exotic dependencies in the engine. Plain Rust, `serde`, ordinary integers. A character sheet has
> no numeric requirements that a specialised arithmetic library uniquely serves, and the engine's
> value is that it is small enough to read and reuse.
> `tarina/DESIGN.md:435-438`

**A consumer op's I11 counts among "pretty much every single repo and project I have" has examined
arvo and declined it, with a reason.** Worth reading carefully, because the reason is about the
numeral and the same document states a requirement that is not:

> Provenance is not decoration. Every number on the derived sheet must be able to name the features
> that produced it, both because a player will ask why their AC is what it is, and because a pack
> that is later disabled has to withdraw exactly its own contributions.
> `tarina/DESIGN.md:188-191`

That is a demand about **what a computation carries**, not about what a value is. It is I11's second
half stated by a consumer that had never heard it, and arvo offered nothing that spoke to it.

I do not think this is an obligation, and I want to be careful not to overclaim it: tarina wants
attribution over a derivation tree, arvo would at most want a schedule of adaptation points, and
those are not the same object. What it is, is the sharpest available evidence that I11's second half
is real and unserved. **The one consumer that evaluated arvo and said no, said no on the axis where
arvo has nothing, and the panel has spent its time on the axis where arvo has 543 rows.**

---

## 5. Alternatives I considered and rejected

Eight, each with what closed it. The list is the part of this file most likely to save the next
member time.

**1. Derive the algorithm surface from the deleted crate tree's design documents.** They are
recoverable and the consumers cite them by name as their contract, so it is tempting. Closed by my
brief and by `the-canon-design-code-chain.md`: the tier is declared dead and reading it reattaches
what had to be detached. The residue, which is real: **both consumers cite `arvo-graph
DESIGN.md.tmpl` and `arvo-bitmask DESIGN.md.tmpl` as "foundations contract" sources**
(`vehje-schedule/DESIGN.md.tmpl:44-45`) and those citations now resolve to nothing. Whatever the
canon says here has to be enough for them to re-derive what they were promised.

**2. Treat the six needs as six obligations and write six canon sentences.** Rejected by section 2.2:
three of the six ask nothing of the numeral concept, so three of the six sentences would be about a
different subject wearing the same list. Writing them anyway would put graph vocabulary into a canon
whose subject is numerals, which is the failure `a-homeless-document-is-a-design-problem.md`
describes.

**3. Treat them as one obligation, "arvo ships the algorithm crates".** Rejected because it is the
crate decomposition again with the names filed off, and because `retirement.toml:42` already measured
what happens when you treat graph, combinatorial and spectral as one thing with one gate: the gate
admits the preset that computes wrong and refuses the two that compute right.

**4. Answer "should arvo ship the algorithm surface" yes or no.** Refused on arrival, and my brief
says why. `never-ask-which-single-rule-governs.md` names the shape, and op has sent it back three
times. Both arms are right somewhere: class A wants a bounded index arvo should ship, class C wants
an eigensolver it probably should not.

**5. Propose a law marker for monotonicity, which is what `retirement.toml:42` names as the
replacement atom.** Closed by `proposal.toml:592`: an author-written law marker is a declaration
checked by nothing, measured at two policies declaring one associativity marker where one declaration
is false and the licensed consumer returns a different answer on 16,268 of 65,536 vectors. The repair
in that same row is the route: compute the permission from the policy's own map, which makes a false
instantiation a const-evaluation error. So the atom is right and the marker spelling is dead, and
Q25 is where the naming sits.

**6. Propose the bounded index with a const capacity parameter and a const assertion.** Built it,
measured it, killed it: `p1` arms G1 to G3, section 2.5. It compiles for the invalid instantiation at
three positions. I would not have found this by reasoning; the arms found it.

**7. Extend `35`'s reserved-absorbing-top shape rather than abandoning it.** `retirement.toml:1145`
records the reopening condition as "a use case needing the absorbing top and the additive inverse but
not monotonicity". I looked for one in the plan chain and did not find it: every class B step is a
relaxation and every relaxation rests on monotonicity. So the retirement's condition stands unmet
from this side too, which is a second instance for it rather than a new finding.

**8. Write the demand-side sweep of the panel's own corpus, which `184` names as owed.** I did not,
and it is still owed. It is 2106 occurrences of "consumer" across 436 files by `184`'s count, and it is
a unit rather than a section. What I did instead was read the two files that answer this question
directly, and section 2.1 is the argument that those two are where most of the answer already is.

---

## 6. What is genuinely op's

Three, in the `question` shape. Everything else in this file is the panel's or is derivable.

**Q-A. Is the surface above the numeral in scope for this canon, and if so at which of four
classes does it stop?**

- *Options.* (1) The canon covers what the numeral owes a composition and stops there; the algorithm
  layer is described where it is implemented, which is Q33's first arm and carries Q33's stated cost,
  "the named selling point computes in something the canon does not cover". (2) The canon covers
  classes A, B and D, so a bounded index, the tropical laws and ordinary numeral arithmetic, and
  excludes class C's numerical linear algebra; the consumer's own design already builds the Laplacian
  itself, which is evidence for the line falling here. (3) The canon covers all four, and owes a
  convergence and determinism statement nothing in the corpus has instruments for.
- *What answering it unblocks.* Whether a topic gets added, whether Q33 can be closed by the panel,
  and whether class C's demands enter the obligation namespace at all. Nothing else in section 3
  depends on it: the rows in 3.2 and 3.3 are owed under every option.
- *Why it is his.* It is a scope call on what arvo is, which is I11's territory, and I11 is `STATED`
  rather than ratified.

**Q-B. `I11` names two things. Is the second one this arc's work, or the next one's?**

Op's words at `ruling.toml:452` carry a clause the panel has not weighed: "But we need this base to
work, to build the bigger things." That is an ordering statement and it may mean the composition
contracts are deliberately after the numeral rather than beside it.

- *Options.* (1) The composition contracts are part of the finished canon he reviews, so this arc
  owes them and the unit continues. (2) They are the next canon, and what this one owes is the
  numeral plus an explicit statement of where its domain ends, which is section 3.3's row and nothing
  more. (3) Neither, and the base clause is about implementation order rather than canon order, so
  the canon owes both now and the code owes them in sequence.
- *What answering it unblocks.* The size of everything in section 3, and whether the four recovered
  proposals in 3.2 are the unit's whole job or its opening.
- *Why it is his.* He set the bar at `181` that the canon be exhaustive enough for a full design and
  a full implementation to follow. Whether "everything" includes the layer above is a reading of his
  own sentence and I will not make it for him.

**Q-C. A consumer's plan chain rejects a statically-known over-wide capacity at run time, which I15
forbids. Is closing that arvo's obligation?**

- *Options.* (1) Yes, and arvo owes the bounded index concept, which section 2.5 establishes is
  expressible gate-free with a definition-site refusal, at the cost of the capacity being a type
  rather than a const. (2) Yes in principle and no in this canon, because the consumer's check is the
  consumer's and I15 binds arvo's own surfaces rather than its consumers'. (3) No: a plan
  registration is data arriving at run time, the check is at an ingest boundary, and I15's own
  closure of Q-A refused an ingest-boundary check as a design option, so the consumer's shape is
  already wrong for a reason that has nothing to do with arvo.
- *What answering it unblocks.* Whether `a_bounded_index_whose_width_is_a_capacity` is an obligation
  or is out of scope, and it is the one obligation in 3.4 that is derived from his intents rather
  than quoted from a consumer.
- *Why it is his.* It asks how far I15 reaches, and I15 is his.

---

## 7. What I could not determine

**Whether a bounded index is a numeral.** Section 2.2's two readings turn on it and I have an
argument in 2.4 rather than a measurement. Q26 is the register's form of it.

**What the type-level relation in `p2` costs.** Trait-solver depth, compile time, and behaviour at
capacities near the pointer width are all unmeasured. `35` flagged the same for its own `ceil(log2)`
and called it unpriced; mine is unpriced on the same terms, and pricing it needs the bench harness
rather than another compile spike.

**Whether the units demand is arvo's.** Section 4 gives both readings and no discriminator.

**Whether the class A / B / C / D split survives a second consumer.** It is derived from one
consumer's thirteen-step chain, cross-checked against vehje's three needs, which fall in A and B and
therefore cannot test C or D. A consumer in a different domain might split it differently, and
tarina, the obvious candidate, declined arvo.

**Whether adding a topic is right.** Section 3.1 states the cost and does not resolve it.

**And the sweep `184` names as owed is still owed**, and nothing in this file substitutes for it.
I read two of the 180 files and I chose them because they answer this question directly. The other
178 may hold more.

---

## 8. Probes

All under `191_probes/`, committed as they ran, each with the case that had to fail.

| probe | what it establishes | controls |
|---|---|---|
| `reach_every_namespace.sh` | ten of eleven obligations have no edge; the plan-chain obligations are reached by retirement rows the earlier instrument could not see | a planted row carrying phrases from the two zero obligations, which surfaces under both |
| `where_the_algorithm_results_are_filed.sh` | every algorithm-surface result carries a numeral topic; `laplacian`, `eigen` and `critical path` are genuinely absent | `saturat` must hit, 54 rows; `zzz_no_such_term` must miss |
| `which_of_35s_figures_survived.sh` | two of `35`'s fourteen figures reached the registry, both inside the row that killed its proposal | `476` and `897` present; `12.6` present, proving a `35` figure can survive; `999999999` absent |
| `which_of_43s_figures_survived.sh` | zero of `43`'s seven reached it | as above, plus every hit printed with its context after version one reported three false positives on short decimals; `_v1_substring.out` kept |
| `the_demand_side_below_the_top_level.sh` | vehje's per-crate designs carry 19 arvo-naming lines `184` did not open | kolli must be zero, notko must be zero, and a terminator line, after version one died on an unmatched glob and printed two of five consumers; `_v1_truncated.out` kept |
| `p1_capacity_against_id_width.rs` + `p1_run.sh` | the const-assertion route refuses only where the constant is forced | the base arm must compile, and it is what caught `-o /dev/null` making every arm fail for the wrong reason; `p1_v1_devnull.out` kept |
| `p2_definition_site_refusal.rs` + `p2_run.sh` | a type-level relation refuses at every position that uses the type, gate-free | H1 and H3 must compile, H3 at one more bit than H2 so the relation measures width rather than size; H5 left red with H6 and H7 bounding it |

Three of the seven caught a defect in their own first version. I have kept all three transcripts,
because in each case the thing that exposed the defect was a control and not a reading, and the
version-one output looked correct.

**One note on the tooling, for whoever writes the next probe here.** nutshell never sets `$0` to the
script path, under a shebang or under `nutshell <file>`; it is always the interpreter. So the
`here="$(cd "$(dirname "$0")" && pwd)"` idiom used elsewhere in this panel resolves into nutshell's
own bin directory. Mine walk up for `mockspace.toml` instead. And an unmatched glob is an error here
rather than a literal, which under `set -e` truncates a run silently.
