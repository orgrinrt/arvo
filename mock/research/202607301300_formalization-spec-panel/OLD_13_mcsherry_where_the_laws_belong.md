# 13: Where the laws belong, read from the scheduler end

**Reviewer:** Frank McSherry (incremental and parallel execution lens: what actually gets reordered,
what the reordering costs, and what a from-scratch baseline says about it).

**What I read.** `11_current_shape_draft.md`, `12_lattner_fresh_read.md`, `12b_op_checkpoint_four.md`,
and no other numbered file in this directory, per the brief. Then the two rules the question collides
with, `arvo-toolbox-not-policer.md` and `panels-argue-the-intent-not-the-wording.md`. Then the thing
nobody in this review has read: hilavitkutin's shipped scheduling and work-unit contracts
(`crates/hilavitkutin-api/src/{work_unit,context,store,record_op,dispatch_codegen}.rs`,
`crates/hilavitkutin/src/{plan,dispatch,thread,resource,scheduler}/`), its integration tests under
`crates/hilavitkutin/tests/`, its committed bench artifacts under `mock/benches/`, and
`mock/DESIGN.md.tmpl`. On the arvo side, `arvo-graph/src/{rank,path,spanning}.rs`,
`arvo-comb/src/dp.rs`, `arvo-spectral/src/power.rs`, and `arvo-strategy/src/identity.rs`. I listed the
directories around all of it.

**What I verified by compiling, running, or reading shipped artifacts**, as distinct from what I
reasoned about. Everything in the first list is a measurement or a source fact; everything else in
this file is argument, offered as directions rather than rulings, and I hold more than one reading
where I have one.

- **arvo's suite: 654 passed, 0 failed, 9 ignored.** `cargo test --workspace` in `arvo/mock`. This
  reproduces file 12's figure exactly.
- **hilavitkutin's suite does not run at all, and has not.** `cargo test --workspace` in
  `hilavitkutin/mock` fails before compiling anything: `crates/hilavitkutin-api/Cargo.toml:17` says
  `mockspace.workspace = true`, and the `[workspace.dependencies]` table at `mock/Cargo.toml:29-57`
  has no `mockspace` key. Cargo cannot load the workspace. This is committed state at
  `67fd5db2`, not a local artifact; the only dirty files are generated `docs/`. 189 `#[test]`
  functions in the engine crate alone are currently reporting nothing. I return to this in section 8,
  because it changes the standing of every claim in this review about what hilavitkutin does.
- **`WorkUnit::COMMUTATIVE` is a hand-declared, unverified, unconsumed flag.** Declared at
  `hilavitkutin-api/src/work_unit.rs:84` (`const COMMUTATIVE: Bool = Bool::FALSE`), copied at
  `hilavitkutin/src/plan/project.rs:156` into `PlanInputs.commutative`
  (`plan/inputs.rs:52`), copied again at `plan/mod.rs:422` into `UnitMeta.commutative`
  (`plan/unit.rs:31`). Those are the only four occurrences in the whole tree. Nothing reads it.
- **The shipped parallel merge is a concatenation in record order, and it does no arithmetic.**
  `MergeAccums::merge_accums` at `hilavitkutin/src/resource/bindings.rs:736-769` memmoves each core's
  live prefix forward in core-index order; core `c` owns `[c*per, (c+1)*per)` (`bindings.rs:753`).
  `gate2_accumulator.rs:1-8` states the contract it is held to: the parallel result "must be
  byte-identical to the single-core `run()` append: same values, same order."
- **`ConvergenceBuffer::combine` takes an unbounded combiner and an unrelated init.**
  `hilavitkutin/src/resource/accumulator.rs:50-58`:
  `pub fn combine(&self, init: T, combine: fn(T, T) -> T) -> T`, left-folding all `N` slots
  (`accumulator.rs:53`, `while i < N`) whether or not a core wrote them. No law bound, and no relation
  between `init` and the buffer's constructor `zero` (`accumulator.rs:29`).
- **`AccumType` and `MergeOp` are closed enums of blessed operations plus `Custom`.**
  `hilavitkutin/src/plan/fiber.rs:70-85` and `95-110`: Sum, Min, Max, Xor, All/And, Any/Or, Custom.
  `HeadTailConvergence` (`fiber.rs:146-165`) is never constructed anywhere outside its own `Default`
  and the re-export at `plan/mod.rs:43`.
- **`arvo-graph`'s ranking is a max-plus recurrence, not a fold over `+`.** `rank.rs:70-84` and
  `path.rs:65-81`: the reduction is `max` over neighbours; `+` is applied once per node with its
  grouping pinned by the graph. The one genuine sequential fold over `+` in arvo is
  `arvo-spectral/src/power.rs:71`, `sq_sum = sq_sum + ns[k] * ns[k]`, over a float type.
- **Zero algebra-ladder names exist in arvo source** (`Magma`, `Semigroup`, `Monoid`, `AddAssoc`,
  `Combine<`: zero hits under `crates/`), confirming file 12. But `arvo-strategy/src/identity.rs:51`
  already ships `pub const trait Identity<Op> { const IDENTITY: Self; }`, and `arvo-graph/src/path.rs:37`
  and `spanning.rs:57` already bound on `Identity<Additive>`. The ladder's bottom rung is shipped and
  in use.
- **Three probes, in `13_probes/`**, all compiled with `rustc -O` and run. Results in sections 3, 4
  and 6.
- **A committed bench already answers the COST question and nobody cited it.**
  `hilavitkutin/mock/benches/fold_strategy_n1024_findings.md`, with variant sources under
  `benches/variants/fold_{sequential,paired,quad}/`. Numbers in section 7.

**One housekeeping note.** `13_probes/01_stability_vs_associativity.rs` already existed, untracked,
written by something before me. It does not terminate: it enumerates two length-12 nondecreasing
sequences over 8 values, which is C(19,7) squared, about 2.5 billion maps at 128 bytes each. I
replaced it. Its question was the right question.

## 0. The premise I was handed, checked before I used it

File 12 section 5a says the algebra ladder's first clients are arvo's algorithm crates, and that if
`arvo-graph` bounds on it, "a signed `Warm` edge weight in `upward_rank` would be refused, because
clamping is not translation-stable" (12, lines 238-241). That sentence contains a claim about what
`upward_rank` does with addition, and it is wrong.

`upward_rank` does not fold weights. `rank.rs:84` is `rank[node_i] = if any { w + best } else { w }`,
where `best` is the running maximum over already-computed successor ranks (`rank.rs:68-82`). The
reduction is over `max`. Addition is applied exactly once per node, and the grouping of the resulting
nested sum is fixed by the graph's shape. Nothing is ever regrouped. `path.rs:81` is the same
statement for `longest_path`, and `arvo-comb/src/dp.rs` is a matrix-chain DP whose composition points
are likewise pinned by the interval structure.

So associativity of `+` is not what makes those answers come out, and an `AddAssoc` bound would not
be refusing something they depend on. It would be refusing something they do not use. That does not
make the collision file 12 names go away. It relocates it, and as section 3 shows, it inverts which
strategies are on which side of it.

I want to be careful about how far this cuts. File 12's finding that the law machinery's first
downstream contact is the algorithm crates is correct and important; it is the reason this dispatch
exists. What is wrong is the mechanism it attributes to them, and the mechanism turns out to matter.

## 1. What hilavitkutin actually does when it partitions work

Four facts, all from source, because the picture in the draft and in file 12 is of a scheduler that
reassociates arithmetic and that is not the one that shipped.

**It splits into contiguous ranges and never reorders them.** `gate2_convergence.rs:5-8` describes the
shipped shape: "head+tail convergence splits that one trunk's record range across all cores (each
walks a ceil-sized slice; the union covers `[0,N)` with no gap or overlap)". `RecordRange`
(`dispatch_codegen.rs:333-341`) has exactly three variants, `Full`, `Head { mid_slot }`,
`Tail { mid_slot }`, and the enum is explicitly closed with no `Custom` fallback.

**Its accumulator merge is a concatenation, not a reduction.** `merge_accums`
(`resource/bindings.rs:736-769`) walks cores in index order and memmoves each core's live prefix
forward. Core index order is record order (`bindings.rs:753`). The operation is list concatenation:
associative, not commutative, identity the empty list, and it composes partials in exactly the order a
single thread would have produced them.

**Its self-imposed correctness contract is equality with the from-scratch single-thread answer.**
`gate2_accumulator.rs:1-8`, quoted above, and the test does check it: it runs the single-core path
first, captures values and order into a reference buffer (`gate2_accumulator.rs:166-174`), then runs
the parallel path and compares (`182-196`). That is the right oracle and I want to say so plainly,
because it is the same discipline an incremental engine lives or dies by. The parallel answer equals
the sequential answer, at every position, or the parallel path is wrong no matter how fast it was.

**It performs no arithmetic reassociation at all today.** Contiguous ranges, disjoint column writes,
ordered concatenation for accumulators. `ConvergenceBuffer` is exported (`resource/mod.rs:10`) and used
by nothing but its own test file. `HeadTailConvergence`, `MergeOp` and `AccumType` are plan-stage
records that nothing constructs. `WorkUnit::COMMUTATIVE` is declared, copied twice, and read by
nobody.

The consequence for the question I was sent: **hilavitkutin does not currently need associativity,
and the reason is not that it is careful about it. The reason is that the part which would need it is
not built.** Any argument that says "the law belongs one layer up, where the reordering happens"
should know that today, one layer up, the reordering does not happen.

## 2. The dichotomy is order versus grouping, and file 12 draws it in the wrong place

File 12's dissolution (12, lines 245-249) reads: "a sequential fold with a fixed traversal order is a
well-defined function without associativity. Associativity is required only when the reduction order
is unspecified, which in this stack means parallel reduction, which is hilavitkutin's layer."

The first sentence is true. The second does not follow, and probe 03 shows it is false about the
shape hilavitkutin actually ships.

Associativity is not a statement about order. It is a statement about **grouping**. Chunking a
sequence into contiguous pieces, folding each piece, and combining the partials in piece order
preserves the order of the elements perfectly and still changes the grouping, from
`((((e ⊕ x0) ⊕ x1) ⊕ x2) ⊕ x3)` to `((e ⊕ (x0 ⊕ x1)) ⊕ ((e ⊕ x2) ⊕ x3))`. Under a non-associative
operation those are different values, and a documented traversal order does nothing about it.

Probe 03 confirms this on the shipped shape. Signed saturating addition, sequence `[-4, -4, -4, 1]`
over a representable range of `[-4, 3]`, cut after the first element: the single-thread answer is
`-3`, the chunked in-order answer is `-4`. Nothing was reordered. The contract at
`gate2_accumulator.rs:1-8` is violated by a two-way contiguous split with an in-order merge.

So the line between "needs nothing" and "needs a law" does not fall between sequential and parallel.
It falls between **one accumulator** and **more than one accumulator**, and the second of those
arrives long before threads do. Section 7 has the number.

## 3. The law the algorithm crates need is not associativity, and the presets sort the opposite way

If `arvo-graph`'s ranking is a max-plus recurrence, what does its correctness argument rest on? The
recurrence computes `best[v] = w[v] + max over predecessors p of best[p]`. For that to be the maximum
over paths, you need the extension to distribute over the reduction:

```
w + max(a, b) == max(w + a, w + b)                                        (D)
```

which over a total order is monotonicity of `+` in its second argument. Probe 02
(`13_probes/02_what_the_algorithm_crates_need.rs`) checks (D) and plain associativity exhaustively
over a representable range of `[-4, 3]`, and then checks the DP against its own stated specification
("the maximum path weight ending at any node", `path.rs:23`) over all 64 DAGs on 4 nodes crossed with
625 weight vectors. Verbatim output:

| arith | `+` associative | `+` distributes over max (D) |
|---|---|---|
| Wrap (`Hot`) | yes | **NO** at `(-4, -4, 0)` |
| Saturate (`Warm` / `Cold`) | **NO** at `(-4, -4, 1)` | yes |
| SubstituteZero | NO | NO |
| Exact | yes | yes |

The two laws are complementary across the presets. The one strategy the draft's `AddAssoc` admits for
signed values, `Hot`, is exactly the one that breaks the law `arvo-graph` depends on; the ones
`AddAssoc` refuses, `Warm` and `Cold`, are exactly the ones that satisfy it.

The end-to-end check says the same thing louder. Under `Wrap`, the DP's answer differs from the
maximum path weight under *both* nestings: at `e=0x44`, `w=[-4,-1,-1,-4]`, the DP returns `-1` and the
specification says `3`. Under `Saturate`, the DP agrees with the forward left-nested path sum on all
40000 cases and disagrees with the other nesting somewhere. Under `Exact` it agrees with both.

Two readings, and I hold both.

**Reading one, the sharp one.** Gating the algorithm crates on `AddAssoc` would be worse than not
gating them, because it would admit the only preset under which `longest_path` returns something that
is not a longest path, and refuse the two under which it does. The mechanism would be pointing at the
wrong thing while looking rigorous, which is the failure mode that costs the most to unwind.

**Reading two, the one that keeps the machinery.** This is not an argument against deriving laws. It
is an argument that the derived set is missing the law these crates use. `Monotone` already exists as
a partial implementation on the rounding side (draft 5.1 notes the four constant-direction rows have
none), and monotonicity of the arithmetic with respect to `TotalOrd` is derivable from the same axes
by the same method. If the ladder grows a `Monotone<Op, Ord>` rung and `arvo-graph` bounds on that
rather than on `AddAssoc`, the collision with `arvo-toolbox-not-policer.md` mostly evaporates on its
own: the refusal lands on `Hot` signed weights, which is a case where the algorithm genuinely
does return the wrong answer, and a refusal that catches a real defect is a different object from a
refusal that catches a strategy choice.

I lean to reading two, and I flag that leaning as a leaning. What I would not do is decide this
without also writing down what the algorithm's specification means, which is section 5.

## 4. Translation stability is sound and it is over-strict by three orders of magnitude

The draft derives its laws from translation stability (draft 3.4): `phi(phi(x) + c) == phi(x + c)` for
every exact `x` and representable `c`. A fold needs associativity of `op(a,b) = phi(a + b)` over
representable `a, b, c`, which is a weaker quantification.

Probe 01 (`13_probes/01_stability_vs_associativity.rs`) enumerates **every** total recovery map that
fixes the representable points, over a signed model with representables `[-2, 1]` and an exact domain
`[-6, 5]`: 65536 maps, which includes wrapping, clamping, substitute-zero, and every resolution nobody
has written down. Verbatim:

```
  maps searched:                65536
  of which monotone:            1   (the whole monotone family is `clamp`)
  translation-stable:           1
  fold-associative:             1024
  associative but NOT stable:   1023   <- refused by the draft, fine for a fold
  stable but NOT associative:   0   <- would make the criterion unsound
```

Two things, and the second is the one I would act on.

**Stability implies associativity on this model, with zero counterexamples.** The criterion is sound
as a sufficient condition. The draft is not certifying something false.

**It is 1024 to 1 over-strict.** One resolution in 65536 is translation-stable in the signed case
(wrapping), and 1024 are genuinely fold-associative. The mechanism refuses 1023 resolutions whose
folds are perfectly well defined. The draft's own consequence at 3.5, "Only `Hot` folds (has a true
`AddAssoc`) for signed values", is not a fact about which signed arithmetics fold. It is an artifact
of deriving the fold law from a strictly stronger property.

A corollary worth recording separately: my first cut of this probe restricted the search to monotone
maps, and that proved nothing, because a monotone total map fixing `[MIN, MAX]` must send everything
below to `MIN` and everything above to `MAX`. **The monotone family is the single map `clamp`.**
Wrapping and substitute-zero are both non-monotone. Anyone reasoning about "the monotone
quantisers" as a family should know it has one member.

This is a place where I think the design's own method would find its own answer. The draft says
(3.4) that translation stability "sorts every case correctly, including the ones the original wording
got wrong". It does. It also sorts a thousand other cases incorrectly in the conservative direction,
and nobody checked, because the check was only ever run against the five hand-written rows.

## 5. A documented order is a real contract about the function and a fiction about the specification

The brief asks whether a documented order is a real contract or a comfortable fiction. It is both,
and the seam between them is where I think the useful thing is.

**About the function, it is a real contract.** `upward_rank` with a stated traversal order is a
well-defined total function of its inputs for any arithmetic at all, associative or not. File 12 is
right about this and it is not a small point.

**About the specification, it is a fiction.** `path.rs:23` says the return is "the maximum path weight
ending at any node". "The weight of a path" is a sum of the weights along it, and under a
non-associative addition that phrase does not denote until you say how it is grouped. Probe 02 shows
this is not hypothetical: under `Saturate` the DP matches the forward left-nested reading everywhere
and diverges from the other reading, so the doc comment names a quantity that has at least two values
and the function computes one of them by accident of implementation.

So the honest form of "magma plus a documented order" is stronger than it sounds. It is: the crate
documents its traversal order, **and** the doc comment stops claiming to compute a quantity whose
definition needs a law the type does not carry. Either the doc says "the maximum, over paths, of the
forward-nested weight sum", which is precise and slightly awkward and true; or the crate bounds on
associativity so that "the path weight" denotes, and then it is policing after all.

That third option is what I think the dichotomy in file 12 is missing. The choice is not
{ bound on the law } versus { document the order }. It is:

1. Bound on the law, and keep the strong specification. Refuses strategies. Policing.
2. Document the order, and weaken the specification to match what is computed. Refuses nothing, and
   costs the crate its ability to state what it does in one line.
3. Document the order, keep the strong specification, and ship a test that the two agree only for the
   compositions where they do. This is the catalogue-the-edge-case discipline applied to a
   specification rather than to a value, and it is the only one of the three that leaves a red mark
   where the gap is.

I would put option 3 in front of op alongside the other two, because it is the one nobody has named
and it is cheap.

## 6. What a scheduler needs, per shape, and one defect the shipped tests cannot see

Probe 03 (`13_probes/03_what_the_scheduler_needs.rs`) models the three merge shapes that exist in
hilavitkutin at three different stages of reality, and checks each against the single-thread answer
over 625 sequences and all 8 partitions of length 4. Verbatim:

```
arith / op           assoc   commut  0 is id   | (a)+(b) order (c) head+tail (b) idle slots
Wrap / Add           yes     yes     yes       | yes          yes          yes
Wrap / Max           yes     yes     NO        | yes          yes          yes
Saturate / Add       NO      yes     yes       | NO           NO           NO
    (a) counterexample: xs=[-4, -4, -4, 1] cuts=[1] seq=-3 par=-4
    (c) counterexample: xs=[-4, -4, -4, 1] seq=-3 par=-4
Saturate / Max       yes     yes     NO        | yes          yes          yes
Exact / Add          yes     yes     yes       | yes          yes          yes
Exact / Max          yes     yes     NO        | yes          yes          yes
```

Every operation in that table is commutative, so it cannot separate the two laws. The probe therefore
also models sequence concatenation directly, which is associative, not commutative, and is exactly
what `merge_accums` performs:

```
    sequential           = [1, 2, 3, 4, 5, 6]
    (a) chunk + in-order = matches   (every cut point checked)
    (c) head+tail        = [1, 2, 3, 6, 5, 4]   DIFFERS
```

That separates them cleanly. **Shape (a), contiguous chunks merged in record order, needs
associativity and an identity and nothing more.** **Shape (c), head+tail with the tail walking
backward (`plan/fiber.rs:159`), additionally needs commutativity**, which is precisely why
`WorkUnit::COMMUTATIVE` gates head+tail eligibility at `plan/fiber.rs:150`. The design's own gating
condition is correct. It is just that the flag it gates on is hand-typed, unverified, and read by
nothing.

Which answers one part of the brief directly. **Associativity is not the single law a scheduler
needs.** It needs a per-shape conjunction, and the shapes hilavitkutin already names want different
members of it:

| shape | associativity | commutativity | identity element |
|---|---|---|---|
| contiguous chunks, in-order merge (shipped) | required | not required | required |
| head+tail, tail reversed (designed) | required | required | required |
| unordered or atomic merge (not present) | required | required | required |
| per-record map, `RecordOp` (shipped) | not applicable | not applicable | not applicable |

I add the fourth row because `RecordOp` (`hilavitkutin-api/src/record_op.rs:34-42`) is the one place
the engine already draws this boundary correctly: it is documented as pure, per-record, "no
cross-record state", and `record_op.rs:14-15` says explicitly that "a unit that carries cross-record
state (an accumulator) does not implement `RecordOp`". The engine already knows that accumulation is
the case that needs a contract. It just has not written the contract.

**The identity column is where a defect is sitting right now.** `ConvergenceBuffer::combine`
(`accumulator.rs:50-58`) folds all `N` slots unconditionally, whether or not a core wrote them, and
unwritten slots hold the constructor's `zero` (`accumulator.rs:31`). Nothing requires `zero` to be the
identity of the combiner, and `combine` takes its `init` as a separate argument with no relation to
it. Probe 03's direct reproduction:

```
    slots = [-3, -1] (two live, two never written)
    combine(0, max) = 0   true max of the live slots = -1
```

Four slots, two cores ran, signed payload. The answer is `0`, a value that appears nowhere in the
data.

The shipped tests cannot see this. `resource_accumulator.rs` has four test functions.
`accumulator_slot_constructs` (line 15) asserts a field equals the constructor argument.
`convergence_buffer_default_is_zero` (line 21) asserts the same for the buffer.
`convergence_buffer_combine_addition` (line 29) uses `wrapping_add`, and
`convergence_buffer_combine_max` (line 39) uses `max` over `T = u32` with values 5, 2, 9, 1, for which
0 genuinely is the identity. Every combiner tested is associative, commutative, and has 0 as its
identity at that payload type. This is setup that helps: the path that breaks is never entered, and
the assertions are real, and the four tests together do not describe `combine` at all. There is no
test that a merged result equals the single-thread result, which is the property `combine` exists to
provide and the one `gate2_accumulator.rs:1-8` states for the sibling mechanism.

I am flagging that as a test-gate failure rather than refusing this dispatch over it, because the gap
is the subject I was sent to examine and refusing to examine it would destroy the finding. But any
dispatch that goes on to *implement* against `ConvergenceBuffer` should fix the suite first, and
should note that it currently cannot even run it (section 8).

## 7. COST: the bench already exists, it says the reordering pays on one thread, and nobody cited it

The brief asks whether a law-gated parallel reduction should have to beat the obvious sequential one
before it earns its machinery. The answer is that this workspace already ran that comparison, through
the proper harness, and committed the artifacts.

`hilavitkutin/mock/benches/fold_strategy_n1024_findings.md`, three variants under
`benches/variants/fold_{sequential,paired,quad}/`, 160 samples per variant, function-under-test
medians:

| variant | median | vs `fold_paired` | throughput |
|---|---|---|---|
| `fold_sequential` (one accumulator) | 110 ns | +40.8 ns (+58.5%), CI [+40, +42] | 9.309 Gops/s |
| `fold_paired` (two accumulators) | 70 ns | base | 14.670 Gops/s |
| `fold_quad` (four accumulators) | 55 ns | -16.2 ns (-23.2%), CI [-17, -15] | 18.755 Gops/s |

Four accumulators beat one by a factor of two, at N=1024, **on a single thread**. The mechanism is
instruction-level parallelism: the sequential variant's doc comment names it, "with a single
accumulator, the loop is latency-bound at ~3 cycles per iteration"
(`benches/variants/fold_sequential/src/lib.rs:3-6`).

That reframes the whole placement question, and it is the reason I do not think "the reordering
happens in hilavitkutin" survives contact.

**Splitting one accumulator into four is a regrouping, and it is worth 2x before any thread exists.**
It is exactly the transformation `arvo-always-optimal-internals.md` licenses arvo to perform inside
its own bodies without asking anyone: "internals unwrap to whatever is most optimal", "asm
microkernels when benchmarks show a hand-rolled inner loop beats the trait-method composition". A
four-way split accumulator in an arvo hot loop is squarely inside that licence, it changes the answer
under any non-associative arithmetic, and no layer above arvo is anywhere near it. So the claim that
the reordering only happens one layer up is false at the level of a single unrolled loop, and it is
false in a direction where arvo has already granted itself permission.

Three honest caveats on that bench, because it is not quite the number one wants.

**The variants do not compute the same function.** FNV-1a is not associative and `fold_quad` finishes
with `acc0 ^ acc1 ^ acc2 ^ acc3`, which is a different hash. The bench prices the *shape* of breaking
a dependency chain. It does not price a law-preserving regrouping against its sequential equivalent,
which is the number the design question actually wants.

**End to end, the spread nearly vanishes.** Under the harness's shared realistic workload the same
three variants land at 2260 ns, 2299 ns and 2213 ns, a total spread under 4%. The 2x on the function
under test is 4% in the surrounding program at this size. That is the COST discipline pointing the
other way, and it should be read next to the first number rather than instead of it.

**Nothing has ever benched a threaded reduction against the single-thread baseline in this stack.**
There is no such variant under `benches/variants/`, and there could not be a meaningful one yet,
because the threaded reduction is not built. Which is the COST answer in its plainest form: **the
parallel reduction has not yet shown it beats one competent thread, because it does not exist, and
the law machinery is being designed for it in advance.** I would want that number before the ladder's
parallel rungs are sized, and I would expect it to be unflattering at the record counts hilavitkutin's
own examples use.

## 8. The state this leaves the relocation question in

I was asked to confirm, refute, or find the dichotomy wrong. My read is the third, in a way that
mostly preserves file 12's destination and discards its reasoning.

**What I think is right in file 12.** The laws' first downstream contact is the algorithm crates, and
nobody had traced it. The collision with `arvo-toolbox-not-policer.md` is real and is not dissolved by
restating either side. The layer where a contract is enforced should be the layer that can violate it.

**What I think is wrong.** The mechanism ("`upward_rank` folds") is not what the source does
(section 0). The criterion ("order is unspecified" versus "order is documented") is the wrong axis,
because grouping and order are different and contiguous chunking preserves order while changing
grouping (section 2). And the destination ("hilavitkutin, where the reordering actually happens") is
not where the reordering happens: it happens in any multi-accumulator loop, on one thread, inside
arvo's own licensed internals, and it is worth 2x there (section 7).

**What I would put in front of op instead of a relocation.** Not one place for the laws, but a split
by who can violate which:

- **Facts about the arithmetic belong in arvo**, because only arvo knows them: is `+` associative,
  is it monotone with respect to `TotalOrd`, does it have an identity, does it distribute over `max`.
  These are properties of a composition and the design's derivation machinery is the right way to
  compute them. Note that the bottom rung already ships and is already consumed
  (`arvo-strategy/src/identity.rs:51`, `arvo-graph/src/path.rs:37`).
- **Requirements on the arithmetic belong wherever a regrouping is performed**, which is a set that
  includes hilavitkutin's merge shapes *and* arvo's own unrolled accumulators. A crate that regroups
  states which of the facts it needs; a crate that does not regroup states nothing and refuses
  nothing.
- **The specification question is separate from both** and is currently unowned (section 5).

Under that split, `arvo-graph` refuses nothing, because it does not regroup, and its doc comment gets
honest about which nesting it computes. `merge_accums` states associativity and identity.
`ConvergenceBuffer::combine` states associativity and identity and stops taking an unrelated `init`.
Head+tail additionally states commutativity, which is what `plan/fiber.rs:150` already says in prose.
And `arvo`'s own four-way accumulator split, wherever it lands, states associativity like everyone
else, which means the discipline applies to the crate that wrote it rather than only to its
consumers.

I hold one alternative reading and it is not weak. If the derived-law machinery cannot state
monotonicity and distributivity as cleanly as it states associativity, then the ladder is not
actually the vocabulary these consumers need, and a smaller, hand-written per-crate contract would
serve them better at a fraction of the machinery. The draft's own justification for declaring the
ladder to full mathematical depth (3.7, "a vocabulary fixed by mathematics cannot be got wrong in a
way that later needs undoing") is a claim about the vocabulary, not about whether the consumers speak
it. Probe 02 is one piece of evidence that they speak a different dialect. I would not settle that
without trying to derive `Monotone` and `Distributes` by the same method and seeing whether they come
out as cleanly.

**And a scope condition on all of it.** Op's checkpoint calls this "a relocation across two
repositories touching the engine's scheduling contracts" (12b, lines 42-44). The engine side of that
is currently in a state where nothing can be verified: **hilavitkutin's workspace does not load.**
`crates/hilavitkutin-api/Cargo.toml:17` inherits a `mockspace` dependency that
`mock/Cargo.toml:29-57` does not declare. Cargo refuses before compiling. So 189 engine test
functions, including every accumulator, convergence and parallel-plan test this file cites, have been
reporting nothing at HEAD. Every statement I have made about hilavitkutin is a statement about source
text that I read, and I have marked it as such. I would fix the manifest and get a real colour out of
that suite before any decision is taken about moving a contract into it, on the same reasoning file 12
gives for gating the crate moves on the verification spine (12, section 7): packaging that calcifies
around an unverified mechanism is the expensive mistake, and an unrunnable suite is a stronger version
of unverified than the one that argument was made about.

## 9. Unlicensed or out-of-scope things I noticed and am reporting anyway

- **`AccumType` and `MergeOp` (`plan/fiber.rs:70-110`) are the policing shape, in hilavitkutin.** A
  closed enum of six blessed operations plus `Custom`. The six are exactly the commutative monoids
  someone thought of; `Custom` has no contract at all. Whatever is decided about arvo's ladder, this
  is a substitute for a law expressed as a list of names, and it will refuse the seventh operation a
  consumer wants for no reason a consumer can act on. `arvo-toolbox-not-policer.md`'s "no hardcoded
  limits, anywhere" is written about arvo, and the reasoning transfers without modification.
- **`WorkUnit::COMMUTATIVE` (`work_unit.rs:84`) is a consumer-typed claim about mathematics with no
  check and no consumer.** It is the exact thing the draft's Thread C exists to prevent: a leaf fact
  someone hand-typed, trusted, never verified. It is also, right now, dead weight, since nothing reads
  it (`project.rs:156`, `plan/mod.rs:422` copy it, nothing consumes it).
- **`arvo-spectral/src/power.rs:71` is the one real sequential fold over `+` in arvo**, and it is over
  a float type (the bound at `power.rs:45-50` requires `Sqrt` and `Recip`). Floating-point addition is
  non-associative for every composition, signed or not, `FastFloat` or `StrictFloat`. An `AddAssoc`
  gate on the algorithm crates would refuse `arvo-spectral` outright, at every strategy, which is a
  louder version of the section 3 inversion and is worth stating separately because file 12's
  discussion of spectral (12, lines 254-259) is about `TotalOrd` and bottom propagation, not about
  this.
- **The `ConvergenceBuffer` identity defect is invisible to the parallel-equals-sequential oracle.**
  Probe 03's main table shows `Max` passing every shape even though 0 is not its identity, because the
  reference fold starts from the same wrong `init`. Only comparing against the mathematical intent
  catches it. That is a real limitation of the engine's own chosen oracle and it should be written
  down next to the oracle, at `gate2_accumulator.rs:1-8`.

## What I did not do

I ran no new bench. The COST numbers in section 7 are read out of committed artifacts produced by the
proper harness; `bench-in-bench-harness-never-sketches.md` forbids putting a timing loop in a probe,
and the number I actually want (a law-preserving regrouping against its sequential equivalent, and a
threaded reduction against one thread) does not exist as a variant and would have to be added under
`mock/benches/` as real work rather than smuggled in here.

I did not attempt to derive `Monotone` or `Distributes` from the ten axes, which is the single piece
of evidence that would settle section 8's alternative reading, and which I think is the next thing
worth someone's compile time.

I did not touch the hilavitkutin manifest. It is one line in `mock/Cargo.toml` and it is not mine to
land inside a review dispatch, but it is the cheapest unblocking piece on this whole surface and
somebody should take it before the next dispatch reads that tree.
