# Typing the algorithm crates: what the design's oldest consumers need, and what they get today

Frank McSherry, file 55. I wrote file 13, on where the laws belong. Forty-one files have landed since
and I have assumed none of my earlier statements still hold.

**What I read, stated precisely rather than flatteringly.** `49_consolidation_four.md` in full as the
standing base and `53b_persona_checkpoint_twelve.md` in full. Files 50 through 54 I read by section
heading plus the sections bearing on my question: file 53's sections 5 and 6 (which revive file 04's
finding and hand me this dispatch), file 54's section 5, file 52's section 4. I grepped all four for
every mention of the algorithm crates, of `Capacity`, of `FromConstant`, and of iteration, and got
zero hits in all four, which is itself worth recording: **nothing in the last five deliverables touches
the surface this one is about.** Behind the consolidation I read only where it compresses a derivation
I needed: file 04 section 3 (my question's origin), file 26 sections 1.6 and 1.7 (Stage G, the boundary
this dispatch is about, and its three already-compiled crossings), file 33 sections 6.1 through 6.4
(the `Monotone` atom and the dioid result), and `47_probes/probe_3` plus `48_probes/probe_2` as the
fold surface I have to write consumer code against. I `ls`ed the panel directory once, at the start.
Files 05 through 25, 27 through 32 and 34 through 48 I touched only by grep, and I say so because the
curated-reading convention in `panels-argue-the-intent-not-the-wording.md` means my coverage of the
argument's history is the consolidation's, not the transcript's.

**What I compiled against what I reasoned.** Ten probe artifacts in `55_probes/`, all on the pinned
toolchain from inside the repo tree, plus two temporary tests written into the mock workspace, run,
and removed. Sections 1, 2, 4 and 5 rest on compiled or executed evidence and say which. Section 3's
`Monotone` recommendation is reasoned from file 33's derivation plus my own runtime measurement;
section 6's `TotalOrd` fork is reasoned only. Every runtime number below is a value a test binary
printed, not a duration; nothing here belongs in a bench harness and nothing here claims a speed.

**Gates.** `cargo test --workspace` from `mock/`: 654 passed, 0 failed, 9 ignored, summed from the
per-binary `test result:` lines rather than a headline, identical to every file from 41 through 54, and
re-run identical after my two temporary tests were removed. On the test-reading half, stated exactly:
I read four of the twenty-eight test files in these four crates in full (`arvo-graph/tests/rank.rs`,
`arvo-spectral/tests/fiedler.rs`, `arvo-spectral/tests/common/mod.rs`,
`arvo-spectral/tests/matrix_bounds.rs`) and swept all twenty-eight for the disqualifying patterns:
per-file assert counts against test counts (no file has fewer asserts than tests), `assert!(true)`,
self-comparison, and calls whose result is discarded. Three `let _ =` sites turned up and all three
are legitimate, two being `#[should_panic(expected = "...")]` with an exact message and one a
commented existence check. Nothing tautological. Section 5 is what the reading did find. The pin resolves to `rustc 1.98.0-nightly (57d06900f 2026-05-27)`, host
`aarch64-apple-darwin`, from inside the tree; file 52's warning reproduces on this machine, where a
bare `rustc --version` from `/tmp` reports stable `1.94.0`.

---

## 0. The question's frame was backwards, and the correction is the finding

File 04 asked what happens to `Precise` at the algorithm-crate boundary, and answered that a
refusing composition does not satisfy `Add<Output = Self>` and is therefore exiled. Fifty files of
machinery later the question reads as: how does the design let `Precise` back in.

That is the wrong question, and I can show it is the wrong question rather than argue it.

I wrote the correct expected value for `upward_rank` on a four-node chain of weight-100 nodes. The
answer is `[400, 300, 200, 100]`. Against the type the function returns, `C::Array<W>` with
`W = UFixed<8, 0, Hot>`, **it does not compile**:

```
error: literal out of range for `u8`
  --> crates/arvo-graph/tests/zz_rank_overflow.rs:32:31
   |
32 |     assert_eq!(r[1].to_raw(), 300, "rank[1] should be 300");
   |                               ^^^
   = note: the literal `300` does not fit into the type `u8` whose range is `0..=255`
```

There is no correct expected value to write down, which is a statement about the signature rather
than about the test. Widening the comparison enough to make it expressible, the two shipped presets
give this (`55_probes/OUTCOMES.md`, both temporary tests reproduced there in full):

| preset | `upward_rank` on the four-chain | true answer |
|---|---|---|
| `Hot` | `[144, 44, 200, 100]` | `[400, 300, 200, 100]` |
| `Precise` | `[255, 255, 200, 100]` | `[400, 300, 200, 100]` |

And on two independent chains, one totalling 400 and one totalling 210, `Hot` ranks the longer path
at **144** and the shorter at **210**. The ordering inverts. Silently. On a four-node graph. In the
crate hilavitkutin reads for plan-stage DAG analysis.

So the finding that reframes file 04: **the exile was never the problem. The admission is.** The
preset the design's fallibility would exile is the one that would have said something; the presets
that compile today return wrong orderings and say nothing, and no amount of letting `Precise` back in
addresses that. File 04's three options were "accept the exile", "panic", and "bifurcate the crates".
All three take for granted that the crates are correct for the presets they do admit. They are not.

*grounded on: tree (`arvo-graph/src/rank.rs:34-88`, `arvo/src/strategy_semantics.rs` preset
semantics as shipped), pin, host. The compile refusal is grounded on `pin` until a compile-fail test
pins it.*

---

## 1. The design's answer, and it is one the design already gives everywhere else

`upward_rank` returns `C::Array<W>`. The result lives in the operand numeral. The design refuses
that shape at every other point it has considered: `mul_full` is `N1 x N2 -> mulnum(N1, N2)`
(`49:269`), the MAC accumulator is a separately computed numeral with its own gcd quantum
(`49:260-265`), and `div_exact` is `N -> divnum(N, C)` (`49:440-442`). A computed result gets a
computed numeral. That rule has simply never been applied to a fold-shaped algorithm, because nobody
typed one.

So the first half of the answer for algorithm consumers, and it is not a new mechanism:

> **A fold-shaped algorithm's result numeral is computed from its operand numeral and its arity, by
> the same kind of numeral-level map the exact-widening family already uses. `foldnum(W, A)` carries
> `W`'s precision plus `ceil(log2 A)`. An algorithm whose result numeral is its operand numeral is
> claiming an exactness it cannot have, and that claim is what makes both shipped presets wrong
> above.**

*Compiled:* `55_probes/probe_4_the_result_numeral_and_the_constant.rs`, three instances (eight-bit
weights over four nodes widen to ten; over sixty-four, to fourteen; sixteen-bit weights over
sixty-four, to twenty-two), with the negative control in `probe_4b` confirming the projected return
type is checked rather than inferred (`E0308`, expected `Num<O<I<O<H>>>>`, found `Num<O<I<I<H>>>>`).

### 1.1 Where the arity comes from, and the fourth firing of the spine rule

The map needs an arity. The good news, and I expected worse: **the arity is already in the
signature.** No simple path in a DAG on `C` nodes visits more than `C` of them, so the rank fold's
arity is bounded by the node capacity, which every one of these functions already carries as
`C: Capacity`. The algorithm does not have to invent a number or ask the caller for one.

The bad news is one line long. `Capacity` exposes its size as `const CAP: Cap`
(`arvo-tensor/src/capacity.rs:24`) and nothing else. Interior safety is a type-level comparison
through the tower's `Cmp`. Stating the obligation from the const fails, and rustc names the forbidden
feature itself:

```
error: generic parameters may not be used in const operations
  --> probe_1b_the_const_capacity_cannot_state_it.rs:57:26
   |
57 |     Hd: InteriorSafety<{ C::CAP - 1 }>,
   |                          ^ cannot perform const operation using `C`
   = help: add `#![feature(generic_const_exprs)]` to allow generic const expressions
```

This is the spine rule (`49:59-72`) firing a fourth time, after the fold's grade (`49:66-68`), the
`Ranged` exponent bounds (`49:552-563`, compiled by file 50), and `Implicit`'s single exponent (file
54 section 5), and the shape is identical every time: a quantity that is computed and then has to
appear in a type is a type. Worth one sentence more than that, because the four are not four of a
kind: the first three are all members of the `Numeral` contract, found by files 47, 48, 50 and 54
working inward on one object. **`Capacity` is the first application outside it**, to a type that is
not a number and belongs to a different crate at a different layer, and it fired anyway. A rule that
holds where it was derived is a description; a rule that holds one layer away is a rule.

> **`Capacity` owes a `Pos` face. The size is computed by whoever declares the container and then has
> to appear in a type, so it is a type; the `usize` the array-length grammar needs is projected out
> of it.**

*Compiled:* `probe_1_the_arity_of_a_rank.rs` compiles clean with the face added, `probe_1b` is the
refusal without it.

Two honest costs, because the arrow only goes one way and I will not pretend otherwise.

**The const generic cannot be removed.** `[T; N]` is the language's array-length grammar and
`[T; <Pz<P> as Nat>::VAL]` is a const expression in type position, which is the forbidden feature. So
`Dim` carries both spellings, `Dim<const N: usize, P: Pos>`, which is the decorrelation risk the
review already names elsewhere (`48_probes/probe_2:64-66` keeps `BITS` as a const for exactly this
reason). The mitigation is a forced const assertion, and I checked that it fires rather than
assuming it:

```
error[E0080]: evaluation panicked: assertion failed: N as u64 == <Pz<P> as Nat>::VAL
   |
48 |     const AGREES: () = assert!(N as u64 == <Pz<P> as Nat>::VAL);
   |     evaluation of `<DimBoth<63, O<O<O<O<O<O<H>>>>>>> as DimAgrees>::AGREES` failed here
```

It fires at **use**, not at declaration, because an associated const nothing touches is not
evaluated. A `Capacity` impl whose two spellings disagree survives until someone folds with it. The
cheap fix is to put the reference on a path every consumer already takes: `Capacity::filled` and
`Capacity::from_fn` are called by every one of the four crates' entry points, and one `let () =
Self::AGREES;` in each closes it. I have not compiled that variant; it is a one-line prediction from
a compiled mechanism, and it is the kind of thing to check rather than believe.

**`Pos` has no zero.** A zero-capacity container has no `Dim`. That is arguably correct (a zero-node
DAG has no fold to be safe about) and `arvo-comb/src/dp.rs:49` handles `cap_size(N::CAP) == 0`
explicitly today, so it is a real narrowing of `Capacity`'s domain and somebody should decide it
rather than discover it.

### 1.2 One of the four crates is not a numeric consumer, and a second is the same defect twice

The dispatch names four crates and calls them "a larger surface". Three of them are; the fourth is
not, and saying so shrinks the work.

**`arvo-sparse` has no numeric contract at all.** Its value parameter is bounded `W: Copy` on the
storage type and `W: Copy + Default` on its impls (`arvo-sparse/src/csr.rs:40, 80`), and nothing
else, anywhere in the crate. `rcm_reorder`, `block_diagonal` and `dulmage_mendelsohn` are structural:
they read adjacency bit patterns and permute indices, and they never compute on a stored value.
Grepping the crate for `arvo::traits`, `Add<`, `Mul<`, `TotalOrd` or `FromConstant` returns nothing.
So the design's answer for `arvo-sparse` is **nothing**, and that is a finding rather than an
omission: it is why the `AddAssoc` droplist entry names three crates and not four (`49:896-898`),
which I read as a deliberate distinction rather than a slip.

The arithmetic a sparse matrix does live one layer up, in `arvo-spectral`'s `SparseLaplacian`
operator, where the bound is the spectral one and section 2 applies unchanged.

**`arvo-comb`'s `bin_pack` is section 1's defect a second time, with a consequence worth naming.**
Its bound is `W: Add<Output = W> + TotalOrd + Copy + FromConstant`
(`arvo-comb/src/binpack.rs:44`), identical to `upward_rank`'s; it accumulates an affinity score over
every other item into `Array<W, N>`, the operand numeral (`binpack.rs:52-63`), so its arity is again
the capacity; and it then compares an accumulated bin load against the caller's capacity in that
same numeral. The failure modes differ by preset and both are real. Wrapping makes a load read small,
so the bin **over-fills**, which for hilavitkutin's fiber grouping means too much work in one fiber.
Saturating makes it read at maximum, so the bin closes early and **under-fills**. Same signature
defect, same fix, different consequence, and neither is announced.

*Reasoned from reading the two crates; not separately compiled, because compiling it would be a third
demonstration of the mechanism probe 3b already established, which is corroboration rather than work.*

---

## 2. The fixpoint: what a grade means across iterations nobody counted

This is the half my dispatch flagged as mine, and it turns out better than I expected.

`power_iteration` and `fiedler_vector` take `iterations: USize`, a runtime value
(`power.rs:38-40`, `fiedler.rs:54-58`). Real solvers are worse: they run until convergence, so the
count is a function of the data. The design says nothing about what a published grade means across
an unbounded number of steps, and its droplist kills the naive shape outright ("growing an
accumulator's own type on every iteration of a runtime-bounded loop: cannot work in principle",
`49:912-913`).

The droplist entry is right and it is not the whole story. Three results.

**The published grade is trip-count independent, for an algebraic reason rather than a fortunate
one.** The grade lattice's join is idempotent, so `G join G join ... join G = G` for any number of
terms at all, including a number nobody knows. File 48 checked commutativity, associativity, identity
and absorption over the whole matrix; it did not check idempotence, and idempotence is the one law a
fixpoint needs. I checked it over the whole four-point carrier at widths one through four, in both
associations (because a scheduler may group the steps either way and the report must not depend on
that), plus the sixteen seed-then-four-steps cells.

*Compiled:* `probe_2_the_grade_of_a_fixpoint.rs`, clean. The `Same` gadget it uses is not
tautological: asserting `J<Faithful, RefusalsTransferred> = Faithful` fails with `E0277`, checked
before relying on it.

**Interior safety is not trip-count independent, and here the droplist is exactly right.** The arity
of an unnormalised accumulating iteration is `trips * step_arity`, and multiplying two const
parameters in a bound is `generic_const_exprs` again (`probe_2c`, section 1). Lifting the trip count
to a `Pos` type does compile, which is worth stating precisely because it means the door is not shut:
it costs the trip count becoming compile-time knowledge, which for a convergence loop is exactly the
wrong price, since "iterate until converged" means the count is a function of the data. The
signature is written out in `probe_2b` so the price is visible rather than asserted.

**What closes the gap is a property of the algorithm, not of the number.** A step that renormalises
has a per-step bounded arity, so its interior safety is the per-step obligation section 1 already
supplies, and the trip count drops out. Both shipped spectral routines renormalise every step
(`power.rs:74-81`, `fiedler.rs:130-142`), which is why they work today and why nobody noticed the
question. "This step renormalises" is derivable from no numeral, policy or lowering; it is something
the algorithm's author knows and the number cannot. The design has the vocabulary for exactly that
already: D16, safe impl when derived, `unsafe impl` when asserted (`49:220-222`).

So the second half of the answer for algorithm consumers:

> **A fixpoint's published grade is the join of its seed's grade with its step's, independent of the
> trip count, because the grade lattice's join is idempotent. Its interior safety is the step's, and
> only when the step is contractive: a step whose output range is bounded by its input range has a
> per-step arity the capacity already bounds. Contractiveness is not derivable from any numeral, so
> it is an `unsafe impl` under D16, and it is the first consumer-side asserted fact the review has
> found, as against the operand-side ones D16 was written for.**
>
> **An iteration whose trip count is a function of the data has an unbounded arity. The comparison
> is made total by one marker and one blanket: `Unbounded` is not a `Pos`, so `impl<Hd: Pos>
> InteriorSafety<Unbounded> for Hd { type Out = Unsafe; }` coexists with the `Pos` blanket with no
> specialisation of any kind, and the fixpoint case gets a grade rather than an error.**

*Compiled:* `probe_2b`, clean, including the coherence result that the two blankets coexist without
`min_specialization`.

### 2.1 The grade a fixpoint publishes is at the top of the lattice, and it is still actionable

I was asked to say so if the grade a caller receives turns out uninterpretable, on the grounds that
machinery serving nobody is machinery to delete. It does not turn out that way, and I checked rather
than assumed, because "it is fine" is the answer I would most expect myself to reach for.

An unbounded iteration publishes the pessimistic grade. Under refusing resolutions that is
`RefusalsTransferred` and the remedy is to read the refusal; under wrapping resolutions it is
`EventsTransferred` and the remedy is not to trust the magnitude. Both remedies are real consumer
distinctions, and the shipped code already draws the line they need: `fiedler.rs:24-26` states that
"only the sign pattern of the result is meaningful for `spectral_bisection`; magnitude is
L2-normalised after the final step". A transferred event multiset does not move a sign. So
`spectral_bisection` is **correct at `EventsTransferred`** and can say so in its bound, while a
consumer reading magnitudes (an algebraic-connectivity estimate, say) is not, and its bound is what
stops it.

*Compiled:* `probe_2b` bottom section, sign-reading consumer clean; `probe_2c` sections 2 and 3, the
magnitude-reading consumer refused against both a wrapping and a refusing solver.

That is a grade doing work: it separates two consumers of the same function that today are
indistinguishable, and it does it at the type level with no runtime cost. I would not have predicted
it; the top of a lattice is usually where information goes to die.

---

## 3. Which fact these algorithms actually need, and the droplist entry that stops one word short

File 33 derived that the graph and DP algorithms use strictly less than the dioid structure their
textbook account reaches for: they use **monotonicity of the weight operation**, one atom, which
"holds for the two presets under which those algorithms are correct, and it fails for the one under
which they are not" (`33:558-561`). The droplist carries the consequence: gating the three numeric crates on
`AddAssoc` "admits the one preset whose recurrences return wrong answers and refuses the two that
compute correctly" (`49:896-898`).

My probe 3b is the first compiled instance of file 33's derivation in consumer code, and it separates
the presets exactly as predicted. Wrapping addition is not monotone: `200 + 200 = 144 < 200`. That is
precisely why `Hot`'s ordering **inverted** (144 for the weight-400 path against 210 for the
weight-210 path). Saturating addition is monotone: `Precise` produced `[255, 255, ...]`, which is
wrong in value but **never inverted**, only degraded to a tie. One atom, two presets, two distinct
failure modes, both visible on a four-node graph.

So the droplist entry is correct about `AddAssoc` and it stops one word short of the fact that should
be there instead. Nobody connected it to file 33's atom. (The entry names three crates, not four,
and section 4.1 below says why that is right.) The connection:

> **`Monotone<Add>` is the fact the ordering-returning algorithms need, and it is the fact the
> droplist's `AddAssoc` entry was reaching past. It is not a gate on all four crates, because the
> value-returning and ordering-returning contracts are different: the widened result numeral of
> section 1 gives the value exactly and needs no monotonicity at all, while an algorithm that returns
> in the operand numeral needs `Monotone` to keep its ordering honest.**

That is two named entry points, not one gated one, which is the design's own
`fold`-beside-`fold_sequential` idiom applied a third time (`49:544-546`), and it stays inside the
toolbox rule rather than policing `Hot` out of existence. It is also the honest reading of what
`upward_rank` promises: its own doc calls rank "a generic longest-path estimate" (`rank.rs:3`), and
an estimate whose ordering can invert is not an estimate of anything.

*Reasoned from file 33's derivation plus my own runtime measurement; the `Monotone` bound itself is
not compiled here, because file 26's move 1 already compiled the mechanism for exactly this shape
(`26:476-482`) and re-deriving it would be corroboration rather than work.*

### 3.1 On idempotent semirings, since my dispatch asked

The max-plus and min-plus structures under `longest_path` and `matrix_chain_dp` are idempotent
semirings, and file 33 is right that no shipped preset is one, for two independent reasons at
saturating addition (associativity fails, and the bottom element does not annihilate, witness
`sat(-8, 3) = -5`, `33:549-551`).

I have nothing to overturn there and one thing to add. File 33 scoped the fix to `Specials`: negative
infinity is a genuine identity for `max` and a genuine annihilator for `+`, which is why the textbook
algebra is stated over the extended reals (`33:571-575`). That is correct, and it means the
idempotent-semiring rung becomes non-empty exactly when the float model lands, which the checkpoint
already ranks first (`53b`, call 5, item 1). Whoever builds it should know that these four crates are
its first consumer and that the consumer is already written: `longest_path` grounds roots at their own
weight (`path.rs:81`) precisely because it has no bottom element to ground them at, and
`matrix_chain_dp` carries a whole parallel `Matrix<Bool, N>` reachability table
(`dp.rs:62, 99-113`) that exists for the same reason. **Both are hand-rolled substitutes for an
annihilator the numeral cannot supply.** A `Specials`-carrying numeral deletes the reachability
matrix outright, which is a rare thing in this review: a mechanism the design would remove rather
than add.

*Reasoned, from reading the two bodies. Not compiled; the numeral it needs does not exist yet.*

---

## 4. `FromConstant` is a partial map declared total, and it has a live defect in the tree

Every one of the four crates initialises with `W::from_constant::<{USize(0)}>()`. `arvo-spectral`
also asks for one and two (`fiedler.rs:71-72, 165`). The trait is
`fn from_constant<const C: USize>() -> Self` (`arvo-numeric-contracts/src/lib.rs:85-88`), with the
constant a parameter of the **method**, so no bound can mention it and no impl can be absent for one
value of it. The implementation is `from_raw((C as $ctype) << $f)`
(`arvo/src/traits/from_constant.rs:40`), with no check.

Measured on the shipped tree, and this one is not about the algorithm crates at all:

```
raw = 19660800, max raw for 24 logical bits = 16777215
thread '...' panicked at ...:15:5:
raw 19660800 exceeds the numeral's own bit width 16777215
```

`UFixed<8, 16, Hot>::from_constant::<300>()` puts a bit pattern into a `repr(transparent)` container
that the type says cannot exist. That is a perimeter breach in the sense of
`what-you-can-observe-is-what-you-guaranteed.md`, in shipped source, reachable from a public trait.

This is the `Identity` finding generalised. The review already established that `UFixed<0, 8>::ONE`
held raw zero because `1 << F` does not fit at `I == 0`, and fixed it by removing the impl. That is
the same defect at the constant one, and `FromConstant` is every other constant. The current tree is
accidentally safe at `I == 0` only because the fractional impls were never instantiated there, and
the macro block's own comment invites filling them in: "specific instantiations can be added as
needed without an API break" (`from_constant.rs:99-101`). The next person who adds `I=0` ships the
bug.

> **The fix is one const parameter moved from the method to the trait.
> `trait FromConstant<const C: USize>` costs no unstable feature, because a bare const parameter is a
> standalone argument rather than an expression. The impl set becomes the representable set, an
> algorithm crate writes the constants it uses into its own where-clause, and an unrepresentable
> constant is `E0277` at the call site instead of a wrong number at runtime.**

*Compiled:* `probe_4`, clean, with the refusals in `probe_4b`. The diagnostic is good without any
tuning at all:

```
error[E0277]: the trait bound `Q0_15: FromConstantKeyed<2>` is not satisfied
help: the trait `FromConstantKeyed<2>` is not implemented for `Q0_15`
      but trait `FromConstantKeyed<0>` is implemented for it
note: required by a bound in `lambda_max_bound`
```

It names the missing constant, the constant that is available, and which bound wanted it. That is
better than anything the numeral-notation thread has managed for `E0308` (`49:600-603`), and for a
reason worth carrying: the mismatch is expressible as a bound rather than as a type equality, which
is the same lever file 47 found for the caller contract (`47_probes/probe_6`). **Every place the
design can turn a numeral mismatch into a bound, it gets a readable error for free.** That may be the
general form of the decoder-ring open item, and it is cheaper than a diagnostic layer.

---

## 5. What reading the tests found

Nothing tautological and nothing I would delete; the sweep is described in the gates paragraph above.
Two findings, both of the setup-that-helps kind rather than the fabricated-green kind, which is the
harder kind to see and the reason the gate asks for bodies rather than counts.

**Every weight in `arvo-graph/tests/rank.rs` is a single digit.** The values are 1, 5, 2 and 7
(`rank.rs:32, 49, 95`) against a `u8` container, and the assertions are exact and correct. The
breaking path is simply never entered: nothing in the file, or in `path.rs`'s tests, sums past 7. My
probe 3b is the same file's shapes with the weights raised to 100 and 200, and it takes two tests to
find an inverted ordering. That is the definition of setup that helps, and the region it leaves
unnamed is the whole subject of sections 1 and 3.

**`arvo-spectral` has ten test files and none of them uses an arvo numeral.** All of them run on
`TF`, a test-local newtype over bare `f32` (`arvo-spectral/tests/common/mod.rs:21`), whose own
comment gives the reason: "Test crates cannot impl the arvo traits on `f32` directly (orphan rule),
so a local newtype is the minimal path". The reason is true about `f32` and does not apply to
`FastFloat`, which arvo ships with every trait the crate requires.

I formed the hypothesis that the crate's bound had no shipped inhabitant at all, which would have
been a much larger finding, and **refuted it by compiling it**: `FastFloat<f32>` satisfies the whole
conjunction and both `fiedler_vector` and `power_iteration` run on it (temporary test, 2 passed,
removed; reproduction in `OUTCOMES.md`). What survives is smaller and still real: the crate has two
shipped inhabitant families, its own tests use neither, and so the numeric behaviour of the L3 crate
is unexercised at any type the substrate ships. Given that its bound reaches `Recip`, which is
division, which is held (`49:423-426`), that gap is where the held item's consequences would first
show up.

---

## 6. Three things I did not settle, stated so nobody records them as settled

**The `TotalOrd` fork is decided by these crates, and I am not deciding it.** The open item is one
sentence (`49:844-845`): datum-level, and therefore forbidden to laws, or value-level with one NaN
class placed consistently. Max-selection over weights is the core operation of `upward_rank`,
`downward_rank`, `longest_path` and `matrix_chain_dp`; all four call `total_cmp` in their inner loops.
If `TotalOrd` is datum-level then none of these four crates' outputs is a law-expressible claim, and
the whole of sections 1 through 3 is stated about something the algebra cannot see. That is a much
larger consequence than "a one-sentence fork nobody has picked" suggests, and it is the reason to
pick it. I have no compiled evidence bearing on which way, so I state the stake and stop.

**Whether the widened result numeral is what a consumer wants.** hilavitkutin reads `upward_rank`'s
output to order work units. Widening the result numeral makes the values exact and changes the
returned type, so every consumer's storage widens with it. That is the right default by the design's
own lights, and it is a real cost that a consumer with sixty-four-node graphs and eight-bit weights
pays as six bits per node forever. The two-door shape in section 3 is my proposal for it and it is a
proposal, not a ruling; the door that returns in the operand numeral is exactly the one that needs
`Monotone`, so the two questions are one question.

**Whether `Capacity`'s narrowing is acceptable.** Section 1.1's `Pos` face has no zero. I flagged it
rather than resolved it.

And one thing I want to be counted honestly: **section 1's `foldnum` and section 2's `Unbounded`
marker are additions to the design's vocabulary, and both are mine.** They are compiled, they are
small, and they are shaped like mechanisms the design already has, which is exactly the condition
under which an agent's own call is most likely to look right and be wrong. Two independent reads,
please, before either lands in a consolidation.

---

## 7. The paragraph a consolidation could take

> **Algorithm consumers.** A fold-shaped algorithm's result numeral is computed from its operand
> numeral and its arity by a numeral-level map, `foldnum(W, A)`, the same shape `mulnum` and `divnum`
> already use; an algorithm returning in its operand numeral is claiming an exactness it does not
> have. The arity is the container's capacity, which the signature already carries, and `Capacity`
> therefore owes a `Pos` face alongside its array-length const, the fourth firing of the spine rule
> after the grade and the two exponents; the two spellings are held together by a forced const
> assertion on a path every consumer takes. A fixpoint's published grade is the join of its seed's
> with its step's and is independent of the trip count, because the grade lattice's join is
> idempotent; its interior safety is the step's, and only where the step is contractive, which no
> numeral derives and which is therefore an `unsafe impl` under D16. An iteration whose trip count is
> a function of the data has arity `Unbounded`, against which every finite headroom is unsafe, and the
> resulting top-of-lattice grade is actionable rather than decorative: it separates a consumer that
> reads signs from one that reads magnitudes, at the type level. `Monotone<Add>` is the fact an
> ordering-returning algorithm needs, which is the atom the droplist's `AddAssoc` entry was reaching
> past; the value-returning door needs no monotonicity because the widened numeral gives the value
> exactly, so these are two named entry points rather than one gate. And `FromConstant` is keyed on
> its constant, so representability is a where-clause rather than a hope.

*grounded on: spine (`49:59-72`), d16, tree (`arvo-graph/src/rank.rs`, `arvo-spectral/src/power.rs`,
`arvo-spectral/src/fiedler.rs`, `arvo-comb/src/dp.rs`, `arvo-tensor/src/capacity.rs:24`,
`arvo-numeric-contracts/src/lib.rs:85-88`, `arvo/src/traits/from_constant.rs:40`), pin, host. The
`Monotone` half is grounded on file 33's derivation plus this file's runtime measurement and carries
no compile of its own. The `foldnum` map and the `Unbounded` marker are this member's own calls and
owe two independent reads.*
