# The total order is a fact about the numeral, not a fact about the design

Stephen Dolan, file 60. I wrote file 14 (which algebra this is) and file 35 (which removed two axes
from the ratified table). Twenty-five files have landed since; I checked both before relying on
anything in them and found nothing in either that this file needs or contradicts.

**What I read.** `58_consolidation_five.md` in full, the standing instruction's only required
reading, and `59_fog_the_lowering_door.md`, the sole deliverable since it. An `ls` of the panel
directory confirmed nothing has landed after `59`. Behind the consolidation, for the derivations it
compresses: the shipped `arvo-numeric-contracts::TotalOrd` declaration and its five impls
(`arvo-numeric-contracts/src/lib.rs:59-68`, `arvo/src/traits/total_ord.rs`, read in full), the four
algorithm crates' actual bounds and bodies (`arvo-graph/src/rank.rs`, `arvo-comb/src/binpack.rs`,
`arvo-spectral/src/partition.rs`, `arvo-spectral/src/fiedler.rs`), and, for the second question,
`hilavitkutin`'s own `mock/DESIGN.md.tmpl`, `mock/crates/hilavitkutin/src/plan/{unit.rs,steps.rs}`,
`mock/crates/hilavitkutin/BACKLOG.md.tmpl`, and the two spectral-partition design-round topics
(`mock/design_rounds/202605300120/`, `mock/design_rounds/202605300351/`), which is hilavitkutin's own
canon for that slice and the authority on what it needs, per the dispatch's own instruction to read
that repo's design rather than treat its source as a design oracle. Where I cite hilavitkutin
source it is to establish a fact about current consumption, not a fact about hilavitkutin's design;
the two are kept apart below.

**Compiled, reasoned, kept apart.** Sections 1.1 through 1.5 are compiled: five probes in
`60_probes/`, every outcome reproduced verbatim in `60_probes/OUTCOMES.md`. Section 2 is reasoned
from source read fresh (both repos), with one grep-shaped check (`Cargo.toml` and `use` statements)
standing in for a compile, because the question is "what does hilavitkutin actually call," which a
grep answers exactly as well as a build does. Nothing here is measured (no timer).

**Test gate.** `cargo test --offline --workspace` from `mock/`, summed per binary: **658 passed, 0
failed, 9 ignored**, identical to file 59's own reported count. Nothing in this dispatch touches a
shipped crate (the probes are standalone `rustc` files outside the cargo workspace, the same
boundary file 59 drew for its own probes), so an unchanged count is the expected result, confirmed
rather than assumed.

**One correction to the brief, stated before anything else, because it changes the shape of the
answer.** The dispatch frames the question as one fork with one answer: datum or value, and if
datum, "none of the algorithm crates' outputs is law-expressible." That framing is stronger than
what the numeral tower actually supports. The fork is not a fact about the design. It is a fact
about each numeral's own encoding, specifically whether that encoding is injective (58:171-177,
statement 3), and it is answered per numeral, compiled, not declared once for the whole design.
Section 1 builds and compiles both readings and shows the fork is live for exactly one of the three
numeric algorithm crates (`arvo-spectral`, over floats) and moot by construction for the other two
(`arvo-graph`, `arvo-comb`, over the shipped fixed-point weight types). `arvo-sparse` was already
excluded by the droplist (58:764-766, "no numeric contract anywhere in it") and stays excluded here.

## 1. Where the fork bites, compiled

### 1.1 The shipped `TotalOrd` is datum-level, and it is not close

`arvo-numeric-contracts::TotalOrd` (`arvo-numeric-contracts/src/lib.rs:65-68`) declares one method,
`total_cmp(self, other: Self) -> Ordering`, with no further contract in the trait itself; the
contract lives entirely in each impl. The float impls (`arvo/src/traits/total_ord.rs:95-133`) route
through `total_cmp_f32`/`total_cmp_f64` (`:29-58`), a const-callable reimplementation of the standard
library's `f32::total_cmp`/`f64::total_cmp`: reinterpret the bit pattern as a signed integer, XOR a
sign-derived mask, compare as integers. This is, by construction, the exact shape 58:547-551 names
as forbidden to laws: "any total order that places NaN consistently is datum-level if it
distinguishes payloads (IEEE's own `totalOrder` predicate does, and is therefore forbidden to laws)."
The shipped mechanism is IEEE's `totalOrder`, not a value-level order, and it distinguishes payloads
because it is defined directly over the bit pattern rather than over a decoded value.

Probe 1 makes this a compiled fact rather than a reading of the algorithm: `total_cmp_f32(-0.0,
0.0)` is `Less`, not `Equal`, and two differently-payloaded quiet NaNs sort strictly against each
other. `-0.0` and `0.0` are the crossing contract's own textbook cohort (58:182-186, "an
unrepurposed signed zero"), the same value under `decode` (58:171-177 statement 1, decode is total
and value-preserving) and two distinct data. The shipped order separates what the value coordinates
say is one thing.

*grounded on: `tree` (`arvo-numeric-contracts/src/lib.rs:65-68`, `arvo/src/traits/total_ord.rs:29-133`,
read fresh), `pin`, `flags`, `60_probes/probe_1`.*

### 1.2 The value-level reading is constructible, at the same cost

Canonicalise, then compare: fold every NaN payload to one representative bit pattern and fold `-0.0`
into `0.0`'s pattern before running the identical bit comparator. Probe 2 compiles this as a `const
fn`, no forbidden feature, no unstable feature beyond what the tower already uses. Both cohort
members land at `Equal`; a genuinely distinct pair (`1.0`, `2.0`) still orders correctly, so the
quotient does not collapse anything it should not.

This is not a new mechanism. It is the crossing contract's own move (58:163-169, "the design
already forbids a law from reading past the canonical quotient") applied to a comparator instead of
argued about in the abstract. The fork was never "can the value-level order be built." Both
readings compile. The fork is which one a law can be checked against.

*grounded on: `pin`, `flags`, `60_probes/probe_2`.*

### 1.3 The law that decides it, and the sharpest form the decision can take

Stated as a `const` assertion (panicking in a const context is a compile error, `E0080`, so this is
a genuine build-time proof rather than a runtime observation): "two data that denote the same value
compare `Equal`." Not the review's full trait-shaped law machinery (`AddCommutes`-style, keyed and
grade-tracked); a minimal necessary condition any value-level, law-usable order has to satisfy,
which is enough to decide the question this file was sent to decide.

Probe 3a states it against the shipped datum order, over the `-0.0`/`0.0` cohort. It does not
compile:

```
error[E0080]: evaluation panicked: the shipped order does not respect the value-equality of -0.0 and 0.0
  --> probe_3a_the_law_the_shipped_order_fails.rs:44:5
   |
44 | /     assert!(
45 | |         matches!(ord, Ordering::Equal),
46 | |         "the shipped order does not respect the value-equality of -0.0 and 0.0"
47 | |     );
   | |_____^ evaluation of `LAW_HOLDS_FOR_SHIPPED_ORDER` failed here
```

Probe 3b states the identical assertion against the value-level reading. It compiles clean.

Per `a-test-that-cannot-compile-is-the-finding`, this outranks a failing runtime assertion, and it
is worth being explicit about why here specifically. A failing assertion would say "at this input,
the wrong ordering came out." A refused `const` says "there is no expected value to write down for
this input, because the definition itself places the cohort apart." No sampling density, no
tolerance, no weaker reading of the law survives it, and no design decision made anywhere else in
the review can rescue it, because the refusal is a property of the comparator's own definition. This
is what "let the compiles decide" looks like when the answer is genuinely decidable: two files,
identical claim, one comparator swapped, and the compiler's verdict flips.

*grounded on: `pin`, `flags`, `60_probes/probe_3a`, `probe_3b`.*

### 1.4 Where the fork applies, and where it structurally cannot

The fork is a question about cohorts: can two distinct data denote the same value. It presupposes
an encoding with at least one cohort to be pulled apart. `UFixed`/`IFixed`, the weight types
`arvo-graph` and `arvo-comb` actually ship against (`arvo-graph/src/rank.rs:39`,
`arvo-comb/src/binpack.rs:44`, both `W: Add + TotalOrd + Copy + FromConstant`), have none:
`arvo/src/traits/total_ord.rs:60-79` routes both through `arvo_storage::ConstOrd`, an unsigned
magnitude compare for `Unsigned` and a native two's-complement signed compare for `Signed`
(`arvo-strategy`'s container table dispatches `Signed` to the native signed primitives, one
representation of zero, by definition of two's complement). Injective by construction: no signed
zero to repurpose, no NaN, no unnormalised-significand cohort (that needs a radix above two, which
fixed-point does not have). 58:171-177 statement 3 says the encoding is injective iff no value has
two data; when that holds, a bit-comparator that is a strict order over the data is automatically
value-respecting, because there is no second datum for two data to collide on. Not an added
mechanism, an identity that falls out of injectivity for free.

Probe 5 checks this rather than only stating it: three 512-wide windows (bottom, middle, top of the
representable range) over an unsigned and a two's-complement signed stand-in, 788,482 and 786,432
pairs respectively, zero cases where `datum_eq != value_eq`. Not exhaustive over the full sixteen-
bit range, and it does not need to be: the property under test is structural (a fact about
injectivity, not about magnitude), so a counterexample anywhere would falsify it everywhere, and
three windows at the range's extremes and centre is enough surface to catch one if the structural
argument were wrong.

**So the fork is live for exactly one of the three numeric algorithm crates: `arvo-spectral`.**
`arvo-graph` and `arvo-comb`'s outputs are law-expressible under either reading of `TotalOrd`,
because for their shipped weight types the two readings are the same function. The dispatch's
framing, "if the ordering is datum-level, none of the algorithm crates' outputs is law-expressible
at all," overstates by exactly the amount injectivity buys back. It is correct about
`arvo-spectral` and wrong, as a blanket claim, about the other two.

*grounded on: `tree` (`arvo-graph/src/rank.rs:39`, `arvo-comb/src/binpack.rs:44`, `arvo/src/traits/
total_ord.rs:60-79`, read fresh), `pin`, `flags`, `60_probes/probe_5`.*

### 1.5 Where the fork bites hilavitkutin, run against the actual consumer

`arvo-spectral/src/partition.rs:59` classifies every Fiedler component by one line: `if let
Ordering::Greater = fs[i].total_cmp(zero) { 0 } else { 1 }`, reproduced at `:156` and `:181` inside
`k_way_partition`. `fiedler.rs`'s own doc comment (cited at 58:434-437) says only the sign pattern
matters; hilavitkutin's `spectral_partition` step consumes exactly this classification to seed a
`FiberGrouping` (`~/Dev/clause-dev/hilavitkutin/mock/design_rounds/202605300120/
202605300120_topic.plan-spectral-fibers.md`) and discards the vector's magnitude entirely. `class[i]`
is the whole of what survives past this line for node `i`.

A Fiedler component can be NaN in practice: power iteration divides by a norm that can be
near-degenerate on a near-disconnected operator, and `arvo-spectral` has no `Specials`/grade
handling wired in yet (that machinery is unbuilt design, section 1.16, not shipped code). Probe 4
reproduces the classification line verbatim and feeds it two NaN bit patterns differing only in
their sign bit, the identical "not a value" condition under two accidents of which operand a prior
division happened to divide. Under the shipped datum order they classify into opposite classes
(`0` and `1`); under the value-level reading they classify identically:

```
datum order:  class(nan, sign=0) = 0, class(nan, sign=1) = 1
value order:  class(nan, sign=0) = 0, class(nan, sign=1) = 0
```

This is a live defect, not a hypothetical one, and it belongs beside the consolidation's existing
live-defect-registry entry 1 (58:1026-1032, `upward_rank`/`bin_pack`'s silently wrong orderings)
rather than folded into it, because the mechanism is different: entry 1 is about a fold's operand
numeral claiming an exactness it does not have (`foldnum`, section 1.20); this is about a
comparator claiming a property (value-respecting) it does not have, and it fires with no fold in the
path at all. **New live-defect candidate: `arvo-spectral::spectral_bisection`/`k_way_partition`
silently misclassify a degenerate (NaN) Fiedler component, with the classification decided by which
NaN payload arithmetic happened to produce rather than by anything the design calls a value.** Tree:
`arvo-spectral/src/partition.rs:59,156,181`. Grounded `tree`, `pin`.

*grounded on: `tree` (`arvo-spectral/src/partition.rs:59,156,181`, `fiedler.rs`, and the hilavitkutin
design round cited above, read fresh), `pin`, `flags`, `60_probes/probe_4`.*

### 1.6 The naming the design already chose, not yet built

58:547-551 already answers, prescriptively, which reading the trait named `TotalOrd` should be:
"the design ships a value-level `TotalOrd`... usable by laws, and names `totalOrder` as a separate,
non-law-usable, datum-level predicate." Read against 1.1, the shipped trait named `TotalOrd` is
today's `totalOrder`: the bit-pattern IEEE order, under the wrong name, with no value-level sibling
built at all. This is not a disagreement with 58; it is the same finding 58 already reached about
its own future state, confirmed here as a fact about the present state, compiled rather than
inferred.

**The shape I would put in a consolidation directly, carried forward from probe 2 and probe 3b as a
compiled candidate rather than a proposal argued from first principles: split the trait.** Rename
the shipped mechanism (or an equivalent) to `totalOrder`, non-law-usable, kept for the consumers
that genuinely want IEEE bit-order (a stable sort key, a hash-adjacent canonicalisation, anything
datum-shaped on purpose). Give `TotalOrd` the canonicalise-then-compare body from probe 2, so the
name means what 58 already says it means. `arvo-spectral`'s bound moves from the current trait to
the corrected one; probe 4's instability closes for the same reason probe 3b's law closes it,
because the comparator underneath the bound name changes, not because `arvo-spectral`'s own code
has to change at all.

One placement question this file does not close, and states rather than guesses at: where the
canonical NaN class sits in the value-level order is itself a choice (probe 2 places it above every
finite and infinite value, matching the convention IEEE's own `totalOrder` uses for its NaN rows;
nothing in the crossing contract forces that placement, only that all NaN payloads share one
position). The more complete answer, once the float model's `Specials`/grade machinery lands in the
algorithm crates, is that a NaN result should raise a grade fact a consumer can act on rather than
be silently placed anywhere in a total order at all; that is section 1.16's unbuilt design, not
this file's to settle, and probe 4's fix is a strict improvement over today (deterministic instead
of payload-dependent) without waiting on it.

*grounded on: `settled shapes` (58:547-551), `60_probes/probe_2`, `probe_3b`.*

## 2. The foldnum width question, answered by reading the one real consumer

The dispatch names this the same question as section 1, correctly: 58:789-794 states both together,
"the widened result numeral" and the `TotalOrd` fork in one "what is not settled" paragraph. The
method the brief specifies is different from section 1's, though: not a compile, a read of the one
named consumer's actual use sites. I did that read; it decides the question cleanly, in the
direction the dispatch's own premise did not expect.

### 2.1 hilavitkutin's canonical chain names a weighted rank; hilavitkutin's shipped chain does not use one

`hilavitkutin/mock/DESIGN.md.tmpl:48` states the canonical twelve-step chain, inherited from the
polka-dots prior art: "`build_dag → topo_sort → upward_rank → waist → RCM → block-diag → spectral →
fiber group → ...`". `upward_rank` is there, named, and `hilavitkutin/BACKLOG.md.tmpl:216` (an
earlier round's residual) records the original intent explicitly: "`upward_rank` weight type needs
`arvo::FromConstant`, when step 3... lands, the weight `u32` must satisfy `FromConstant`." That is a
plan to consume `arvo_graph::upward_rank`, a weighted longest-path fold, exactly the `foldnum`-shaped
function this question is about.

**What shipped instead:** `hilavitkutin/src/plan/steps.rs:828-880`,
`compute_upward_rank_and_dirty`, is a hand-rolled, fused, reverse-topological walk over the plan's
own adjacency structure. It computes an **unweighted** longest path: `max_rank = ranks[successor] +
1` for every predecessor, a hop count, never a sum of edge weights, because the plan's dependency
graph carries no weight column at all in the shape this function reads (`row_offsets`/`cols`, a
bare CSR adjacency; no `W` anywhere in the signature). The result is stored as `USize`
(`hilavitkutin/src/plan/unit.rs:29`, `upward_rank: USize`), the same fixed-width type hilavitkutin
uses for every other bounded plan-stage scalar.

A grep of hilavitkutin's own shipped crate confirms the gap between plan and ship directly:
`arvo_graph::upward_rank` is imported nowhere in `hilavitkutin/src/`. The only `arvo_graph` import
in the engine is `waist_detect`/`waist_detect_const` (`hilavitkutin/src/plan/{grouping.rs,
steps.rs}`), a structural function with no weight type at all. `arvo_comb::bin_pack`/`dp` are
imported nowhere either; hilavitkutin's own fiber grouping is the greedy heuristic named in its
research notes (`~/Dev/clause-dev/hilavitkutin/mock/research/202606060900_engine-completion-
strategic-synthesis.md:82`, "`group_fibers` is a greedy out-degree heuristic"), with the matrix-
chain-DP arm still on the backlog. `arvo_sparse::{rcm_reorder_via, block_diagonal_via}` and
`arvo_spectral::k_way_partition` are the only substrate algorithm-crate calls that actually ship
(`hilavitkutin/src/plan/steps.rs:41-43`), and neither of the first two carries a numeral at all
(`arvo-sparse` has none, per section 1.4 above and 58:764-766).

*grounded on: `tree` (`hilavitkutin/mock/DESIGN.md.tmpl:48`, `hilavitkutin/BACKLOG.md.tmpl:216`,
`hilavitkutin/src/plan/{unit.rs:29, steps.rs:41-43,828-880}`, `hilavitkutin/mock/research/
202606060900_engine-completion-strategic-synthesis.md:82`, all read fresh from
`~/Dev/clause-dev/hilavitkutin`).*

### 2.2 So the foldnum question, for hilavitkutin specifically, is currently moot

Both of hilavitkutin's real numeral-shaped consumption points bypass the widened `foldnum`
accumulator entirely, and for the identical underlying reason: neither keeps the accumulated
magnitude past the moment it produces one bounded scalar.

`upward_rank` as shipped is a hop count, not a weighted sum. Its natural bound is the unit count
(`UnitId(pub Uint<16>)`, `hilavitkutin-api/src/dispatch_codegen.rs:185`, at most 65,535 units), which fits comfortably in
`USize` at every plan size the design admits, with no width growth as a function of fold arity at
all, because it was never `foldnum(W, A)`-shaped to begin with: there is no `W` in the function it
actually runs. If hilavitkutin's design ever grows real per-edge costs for this step (the intent
`BACKLOG.md.tmpl:216` records), the natural place for a per-unit cost already exists and is a
separate mechanism: `CostTable`, the adapt subsystem's per-unit mutable estimate, refreshed between
frames rather than computed once at plan time (`hilavitkutin/src/plan/unit.rs:1-9`'s own module
doc comment). That is a runtime-adaptive scalar, not a compile-time-widened accumulator, and it is
not this question's to resolve either way; it is named here only to show the intent
`BACKLOG.md.tmpl:216` records has a home that is not `foldnum`.

`spectral_bisection`'s consumption is the sign-reading case section 1.5 already established from
the other direction: the fold's magnitude, whatever numeral carries it, is read once for its sign
and discarded in the same statement. Widening the accumulator changes nothing hilavitkutin keeps,
because hilavitkutin keeps nothing from it past the comparison.

**The answer to "does a consumer with many nodes pay the widened numeral forever," for the one
consumer this review can actually check against, is no, because that consumer does not touch the
widened numeral at all today.** Not because the question is wrong to ask of the design in general
(a future consumer with real weighted longest-path needs, or vehje's pass-DAG use of `arvo-graph`
per the arvo crate table, could still hit it), but because the named evidence for "a consumer needs
this" does not hold up against that consumer's own shipped use sites. This is the same shape as
58:1046-1050's own finding about `arvo-graph/tests/rank.rs` (a test that never enters the breaking
path): a plausible-sounding consumer story that the actual call sites do not bear out, caught the
same way, by reading rather than assuming.

*grounded on: `tree` (as cited in 2.1), `settled shapes` (58:434-437, the sign-reading finding).*

### 2.3 What this does and does not settle

It does not settle whether the widened `foldnum` default is right for `arvo-graph`/`arvo-comb`'s
general design. 58:791-793 already frames that correctly as "a proposal, not a ruling," and nothing
here changes that; a consumer this review has not read could still want the exact width. It also
does not touch the live defects section 1.5 (new) and 58's own entry 1 name: both are about
correctness of what ships, independent of who currently calls it, and both stay open regardless of
hilavitkutin's non-use.

It does settle the narrower thing the dispatch actually asked: whether the specific consumer named
in the brief, read at its own use sites, needs the widened result kept around per node forever. It
does not. Both of its real numeral-shaped touch points discard the magnitude before storage, one by
never producing a widened value to begin with (the hand-rolled unweighted rank), one by reading only
a sign (spectral bisection). The storage-cost worry the dispatch's premise states is real in the
abstract and absent in the one case this file could check.

## 3. What survives as shape, and what this file leaves open

**Carried forward, for a consolidation to take directly.** Section 1.4's structural result: the
datum-versus-value fork is decided per numeral by injectivity, compiled, not declared once for the
whole design; `arvo-graph`/`arvo-comb` clear it for free, `arvo-spectral` does not. Section 1.6's
trait-split shape: rename the shipped mechanism to `totalOrder` (non-law-usable), give `TotalOrd`
the canonicalise-then-compare body probe 2 and probe 3b already compiled, move `arvo-spectral`'s
bound across. Section 2's reading: `foldnum`'s width cost is not currently paid by hilavitkutin,
because hilavitkutin's shipped chain does not call the functions that would produce it.

**New live-defect candidates, for the registry.** Section 1.5: `arvo-spectral`'s sign classification
of a degenerate (NaN) Fiedler component depends on the NaN's sign bit alone under the shipped order,
compiled, `partition.rs:59,156,181`. A second, smaller one worth naming alongside it and not built
out further here: `hilavitkutin/src/plan/steps.rs:828-880` reimplements `upward_rank` by hand rather
than calling `arvo_graph::upward_rank`, which the crate's own canonical chain (`DESIGN.md.tmpl:48`)
and its own backlog (`BACKLOG.md.tmpl:216`) both name as the intended path; whether that
reimplementation is the correct design call (the plan's dependency graph may genuinely have no
per-edge weight to fold over) or drift against `use-the-stack-not-reinvent` is a hilavitkutin-side
question this file surfaces rather than answers, since hilavitkutin's own canon, not this review, is
the authority on it.

**Open, and I am not closing them.** Where the canonical NaN class sits in the value-level order
(section 1.6, a real choice, made once here for the compile and not argued as the only one). Whether
`arvo-graph`/`arvo-comb`'s other consumers (vehje's pass-DAG use, per the arvo crate table) have a
weighted-fold need hilavitkutin turned out not to, which would reopen the storage-cost question
section 2 closes for hilavitkutin specifically. Whether the trait-split in 1.6 should be two traits
or one trait with two methods (the review's existing convention, per-axis sealed carriers, argues
for two, but I have not compiled that comparison and state a preference rather than a result). And
the deeper answer to NaN classification, once `Specials`/grade lands in the algorithm crates, which
this file names as the right eventual home for the placement question and does not attempt.

## 4. Droplist addition

The dispatch's own framing, "if TotalOrd is datum-level, none of the algorithm crates' outputs is a
law-expressible claim at all," read as a single design-wide verdict: refuted by compile. It is true
of `arvo-spectral` (section 1.1 through 1.3, 1.5) and false of `arvo-graph`/`arvo-comb` (section
1.4), because the fork is a per-numeral fact about injectivity, and a fixed-point numeral's
encoding is injective by construction under the shipped tower. The correct reading of 58:789-794's
"stakes considerably higher than a one-sentence fork" is not that the whole design's law-
expressibility rides on one answer; it is that the one crate where the fork is live (`arvo-spectral`)
has a real, compiled, silently-wrong consequence today, which section 1.5 names for the first time.
