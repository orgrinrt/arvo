# The lowering door, threaded and compiled, and what a decimal quantum costs

Agner Fog, file 59. I wrote file 08 (the union and what it costs) and file 50 (the float model). File
57 found that one of file 08's artifacts, the five-shape instruction table, cannot be rebuilt from
what is committed, and the review adopted `unreproducible` as a ground state for it (58:709-716). I
take that as the marked ground state it is, and nothing in this file leans on that table.

**What I read.** `58_consolidation_five.md` in full, which the standing instruction makes my only
required reading from the review, and an `ls` of the panel directory (files `00` through `58` plus
their `NN_probes/` directories; nothing has landed after `58`). Behind it, for derivations it
compresses rather than states: `mock/benches/variants/quantiser-fadd-shared/src/{lib,model.rs}` (my
own file 50 model as file 57 vendored it, which the decimal kernel repair generalises),
`57_aaltonen_the_measurement_debt.md` sections 1.1 to 1.3 (how the bench that produced 58:829-834 is
wired, since this dispatch's numbers are read against it), and `mock/benches/{Cargo.toml,bench.toml,
src/main.rs}`. In the shipped tree: `arvo-strategy/src/lib.rs` and `arvo-strategy/src/container.rs`,
because the strategy axis this dispatch threads is a shipped thing with shipped meanings and I am not
entitled to invent what its four markers mean.

**Gates.** Canon gate: the design surface here is unbuilt, reproduced fresh from the repo root,
`grep -rln "Adjustment\|Bias\|Numeral" mock/crates/ --include="*.rs"` and the same with
`FullRange\|UTerm\|AddWidth`, both exit 1, empty. What I did touch in the tree is
`mock/benches/`, which is bench infrastructure and not per-crate source, plus one line of
`mock/Cargo.toml`'s member list; the same boundary file 57 established and committed under. Test
gate: `cargo test --offline --workspace` from `mock/`, summed per binary, **658 passed, 0 failed, 9
ignored**. That is file 57's 655 plus the three correctness tests this dispatch adds (section 3.2).
I read the bodies of the tests in the surface I touched, which is the two quantiser bench crates;
`quantiser-fadd-shared`'s single test cross-checks against the silicon bit-for-bit over the exact
distribution its own `build_input` generates, at all six of the sweep's `PCT` values rather than a
sample of them, and asserts the checked count. It is a real test. Mine are described in 3.2 and one
of them found a defect in its own oracle before it found anything else.

**Compiled, measured, and reasoned, kept apart.** Sections 1 and 2 are compiled: seven probes in
`59_probes/`, every error text reproduced verbatim in `59_probes/OUTCOMES.md`. Section 3.3 is
measured, twice, through the bench harness, with the CSVs and meta committed. Section 3.1 and 3.2's
two derived facts are reasoned and then checked by a test. Where I attribute a runtime cost to a
mechanism I say whether the attribution is measured (the disassembly in 3.4) or derived from the
sweep's own shape (the fixed-against-marginal split in 3.4), and I do not let the second wear the
first's clothes.

**One correction to the brief I was handed, before anything else.** The dispatch says the strategy
marker "selects which lowering door a float operation takes". After threading it, that is not what
happens and cannot be. The strategy selects a **default lowering**; the door is that lowering's own
member. The distinction is not pedantry: it is the difference between a shape that compiles and one
that is refused twice over by the language (section 2.1), and it is what lets a consumer name a
control state arvo has no business knowing.

## 1. Where the door belongs, and why the answer was already ratified

The consolidation frames the door as an open question about a new mechanism. It is neither. The
design has an axis whose defining property is exactly the door's defining property, and the match is
tight enough to be a derivation rather than a placement.

> `Lowering` changes no value; `Encoding`, nested inside it, may change which datum carries a value.
> No law may read `Lowering`; a law's key is a `const fn` parameter list and `Lowering` is not a
> parameter. (58:165-167)

Read the hardware door against that sentence. A hardware lowering under a pinned environment changes
no value, which is the whole content of 58:798-806's derivation that an *unpinned* one is not a
lowering at all. And a door that a law could read would be a door that changed a law-visible fact,
which is the same thing as changing a value. So:

**The door is a `Lowering` member, and it is one because `Lowering` is the axis no law may read.**
The property that makes the door safe and the property that defines the axis are the same property.
There is no fourth axis to mint, no `Policy` member to add, and no new rule. This is the same move
section 1.16 already made for flush-to-zero: FTZ left `Numeral` for `Quantisation` because it changes
no representable set, and the door stays out of `Numeral` and `Policy` for the mirror reason.

*grounded on: `settled shapes` (58:165-167, the ratified `Lowering` definition), plus the compile in
section 1.1.*

### 1.1 The rule is enforced by the language, not by discipline, and the receipt does not weaken it

This matters more now than before the door existed, because a receipt has to be *readable* by a build
layer, which means the door needs a const face, and a const face is exactly the shape that could leak
into a law's key. Probe 1 gives it one: a `const fn` off the lowering type returning
`Option<(Rounding, bool, bool)>`, four scalars, no machinery.

Probe 4 attacks it. Two ways to make a law's holding depend on the lowering, once through the door
type and once through the receipt's own const face. Both refused, and the const-face route is refused
twice:

```
error: generic parameters may not be used in const operations
49 | impl<N: Numeral, L: Lowering> AddCommutes<N> for (Witness, [(); L::HAZARDOUS_FACE as usize]) {}
   = help: add `#![feature(generic_const_exprs)]` to allow generic const expressions

error[E0207]: the type parameter `L` is not constrained by the impl trait, self type, or predicates
45 | impl<N: Numeral, L: Lowering<Door = Quantised>> AddCommutes<N> for Witness {}
```

E0207 is the mechanism, and it is unconditional: a law trait keyed on the numeral alone leaves any
lowering parameter unconstrained, and rustc refuses the impl before reaching any question of whether
the law is true. The forbidden-feature wall is a second, independent one on the const-face route,
which is `unstable-features.md`'s own "the forbidden list is verification infrastructure, not only
hygiene" section turning up at a position nobody had checked.

So the ratified "no law may read `Lowering`" needs no enforcement mechanism designed for it. It has
one, and adding the receipt's const face does not open a hole.

*grounded on: `pin`, `flags` (`rustc --edition 2024`, no other codegen flags), `59_probes/probe_4`.*

## 2. Threading `S: Strategy` through, and what happens to the presumptive table

The consolidation's per-preset assignment (58:820-822) is flagged reasoned rather than compiled, every
row of it. I threaded it. **Every row's content survives. Every row's justification is replaced, and
three of the four stop being choices at all.** One row gains a condition it did not have, and one of
the four turns out to have been justified by a non-sequitur that happened to reach the right answer.

### 2.1 The shape the table implies, read literally, is closed twice over

Read literally, "`Hot` carries the receipt-carrying hardware lowering" is a projection from the
strategy alone. It cannot be total, because a `Ranged` numeral with `p = 11`, `emin = -14`,
`emax = 15` and `Underflow = Abrupt` is a legal numeral of this design that no silicon implements,
and it still has to lower somehow. The obvious repair is a software fallback refined where the numeral
is host-implemented. Probe 2a:

```
error[E0119]: conflicting implementations of trait `DoorFor<_>` for type `Hot`
30 | impl<N: Numeral> DoorFor<N> for Hot {
   | ----------------------------------- first implementation here
34 | impl<N: Numeral + HostFormat> DoorFor<N> for Hot {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ conflicting implementation for `Hot`
```

That is section 1.10's structural theorem, verbatim, at a second position: two impls disagreeing on
the answer for the same generic domain, refused by coherence before correctness is reached. The
review derived that theorem for `Growth`-in-`Policy` and stated it generally ("no operation
expressible in this type system's dispatch discipline can have policy-dependent growth", 58:334-335).
The door is the first independent instance, and it confirms the general form rather than the
particular one.

Probe 2b runs the same shape under `min_specialization`, the only specialisation door the workspace
permits, and gets two refusals for the price of one:

```
error[E0658]: specialization is experimental
27 |     default type Out = Quantised;

error: cannot specialize on trait `HostFormat`
30 | impl<N: Numeral + HostFormat> DoorFor<N> for Hot {
```

`min_specialization` does not reach an associated type at all, and it does not specialise on an
ordinary trait. The only opener is full `specialization`, forbidden. So the fallback-plus-refinement
door is shut in the safe language and shut again in the one permitted unsafe-ish extension.

*grounded on: `pin`, `flags`, `59_probes/probe_2a`, `probe_2b`.*

### 2.2 The shape that survives, and it needs no feature gate at all

Three facts decide a door and they arrive from three different places, which is precisely why no
projection keyed on one of them can be total:

- **whether this target's silicon implements the numeral.** A target fact. Belongs to the numeral,
  cfg-gated per target, which is Kind 1 structural lowering under the always-optimal-internals rule.
- **which control state the deployment guarantees.** A deployment fact. arvo cannot know it and must
  not decide it.
- **which door the preset prefers where both exist.** A strategy fact, and the only one of the three
  the strategy owns.

```rust
pub trait LoweringDoor: Sealed { }
pub struct Quantised;
pub struct HostFloat<E: FloatEnv>(PhantomData<E>);   // E has no default

pub trait Lowering { type Door: LoweringDoor; /* Encoding, StoredWidth, Layout unchanged */ }

/// A target fact about a numeral. Sealed; cfg-gated per target. Absence is the
/// mechanism, not an error condition.
pub trait HostImplemented: Numeral { }

/// What a strategy actually selects.
pub trait DefaultLowering<N: Numeral>: Strategy { type L: Lowering; }

impl<N: Numeral + HostImplemented> DefaultLowering<N> for Hot  { type L = HostLowering<N, IeeeDefault>; }
impl<N: Numeral>                   DefaultLowering<N> for Warm { type L = SoftwareLowering<N>; }
impl<N: Numeral>                   DefaultLowering<N> for Cold { type L = SoftwareLowering<N>; }
impl<N: Numeral>                   DefaultLowering<N> for Precise { type L = SoftwareLowering<N>; }

pub type DoorOf<S, N> = <<S as DefaultLowering<N>>::L as Lowering>::Door;
```

One impl per strategy. No second impl, so there is nothing for probe 2a's E0119 to overlap with:
`Hot`'s partiality lives in a **bound**, not in a competing impl. Probe 3c compiles this with no
`#![feature(...)]` line anywhere, and probe 3d shows the refusal:

```
error[E0277]: this target's floating-point unit does not implement the numeral `Ranged11Abrupt`
    = note: The `Hot` preset lowers a float operation to the host instruction, which exists only for
      the numerals the target provides (binary16/32/64 on aarch64-apple-darwin). Choose `Warm`,
      `Cold` or `Precise`, which lower through the software quantiser at every numeral, or choose a
      numeral the host implements.
help: the following other types implement trait `HostImplemented`
 76 | impl HostImplemented for Binary32 {}   `Binary32`
 79 | impl HostImplemented for Binary64 {}   `Binary64`
```

**Refusal, not fallback, and the shipped tree already made that call.** `BitsContainerFor`'s absent
`Project` impl is what makes `Uint<100, Warm>` a compile error pointing at a named alternative rather
than a silent widening (`arvo-strategy/src/container.rs:104-112`). The door inherits that posture and
should: a silent fallback from the hardware door to the software one is a **13x to 17x** cost change
(58:829-836) delivered without telling anyone, which is the policer posture wearing a helpful face.
The presumptive table, read literally, implies exactly that fallback; it should not.

The exhaustive "the following other types implement" list is rustc's own, unprompted, and it is
exhaustive because the marker is sealed. That is file 56's seal-as-free-diagnostic dividend
(58:362-368) reproduced at a position nobody had tested, and it is the second time in this review that
sealing a carrier turned out to be the diagnostic as well as the mechanism.

*grounded on: `pin`, `flags`, `59_probes/probe_3c`, `probe_3d`; and `tree` for the
`BitsContainerFor` precedent.*

### 2.3 The corrected per-preset table

Every row below is derived from what the preset already means for fixed-point arithmetic in the
shipped tree, carried across to float by the same reading. That is a different kind of statement from
the presumptive table's, and it is why three of the rows stop being preferences.

| preset | shipped meaning | door | why, and is it a choice |
|---|---|---|---|
| `Hot` | "Arithmetic is wrapping. Single instruction per op" (`arvo-strategy/src/lib.rs:110-114`) | `HostFloat<E>` where the numeral is host-implemented, **refusal otherwise** | The only genuine choice of the four, and it is `Hot`'s identity: one instruction per op is the hardware door in the float reading exactly as it is in the fixed-point one. Conditional on host-implementedness, which the presumptive table did not have. |
| `Warm` | "Container is 2x the logical bit width. A single add / sub / mul of two values within their logical range cannot overflow the container" (`:118-124`) | `Quantised` | **Forced.** `Warm`'s entire identity is that the intermediate is wider than the format, which is the exact-then-round posture. A hardware `fadd` rounds in-format. There is no `Warm` reading of the hardware door. |
| `Cold` | "Minimum container, bitpacked. Arithmetic widens to 2x before operating, narrows back on store" (`:128-131`) | `Quantised` | **Forced**, and the presumptive justification was a non-sequitur. "Follows the semantics-first side" derives an arithmetic-lowering fact from nothing; `Cold`'s own doc comment derives it from something. Widen, operate exactly, narrow back **is** the round-first quantiser in miniature. |
| `Precise` | "Arithmetic is saturating: overflow clamps to logical min/max rather than wrapping" (`:135-139`) | `Quantised` | **Forced**, and measured. Probe 7: `f32::MAX * 2.0` delivers `inf` on this host, where `Precise` owes the largest finite magnitude. A door that delivers a different value is not a lowering under 58:798-806 at any pinning. |

The one control state under which IEEE itself saturates is a directed rounding mode, and it saturates
only on the side rounding moves toward, so it is not `Precise`'s two-sided clamp either. I state that
as reasoning from the standard's overflow clause rather than as something I measured.

The reading that makes all four rows fall out at once, and which I would put in a consolidation
directly: **`Hot` is the only preset whose meaning is about the instruction. The other three are
about the intermediate, and an operation whose meaning is about its intermediate cannot be lowered to
an instruction that has none.** The door assignment was never a table of preferences; it is what the
four markers already said, read at a numeral that happens to be a float.

*grounded on: `tree` (`arvo-strategy/src/lib.rs:110-114`, `:118-124`, `:128-131`, `:135-139`, read fresh),
`host` + `pin` for probe 7's measurement, `settled shapes` (58:798-806) for the lowering definition.*

### 2.4 What a mixed-strategy expression does, which the table never addressed

Every consumer surface has expressions whose operands disagree on strategy, and the presumptive table
says nothing about them. Nothing new is needed. arvo resolves cross-strategy operations by
`Strategy::RANK`, "higher is more conservative", `Precise > Cold > Warm > Hot`
(`arvo-strategy/src/lib.rs:104-107`). `Hot` is the **lowest** rank and, by 2.3, the only preset whose
door is the hardware one. So:

> **The hardware door is reachable only in a uniformly-`Hot` expression.**

A mixed expression cannot silently acquire hardware semantics, because resolution moves away from
`Hot` by construction and every other preset's door is the quantiser. Probe 6 compiles the whole
sixteen-cell matrix and asserts on the whole of it rather than a sample: `hardware_cells == vec![
("Hot", "Hot", "Hot")]`, which fails if any other cell reaches hardware or if that one stops. One of
sixteen.

This is a rank ordering that shipped long before anyone asked the door question, delivering a safety
property the door question needed. It is the review's "keep the shape where it serves" instruction
paying out: rewrite cost zero, and the property is stronger than anything a new rule would have got.

*grounded on: `pin`, `flags`, `59_probes/probe_6`; `tree` for the rank ordering.*

### 2.5 The line I did not cross, and where it is

The dispatch warns that a threading which grows receipt plumbing into arvo has crossed a line the
review drew deliberately, and 58:944-948 states the two receipt families as owed to a build layer
rather than to arvo. Concretely, here is the whole of arvo's side, from probe 1:

```rust
pub const fn receipt<L: Lowering>() -> Option<(Rounding, bool, bool)>
where L::Door: DeclaresEnv { <L::Door as DeclaresEnv>::DECLARED }
```

A `const fn` returning four scalars off a type. `None` when the door is the quantiser, because a
quantiser reads no control state. That is a **declaration**, and everything downstream of it belongs
to somebody else: reading it off a monomorphised call site, checking it against the deployment,
invalidating it when something writes the FP control register, deciding what to do when it does not
hold. arvo ships none of that and should ship none of it.

**What the design needs back from the build layer, stated so it is designed rather than noticed
later.** Three obligations, and each is a thing arvo cannot do from inside:

1. **Verify the declared state against the deployment.** The receipt is a claim about the process's
   FP control register, and arvo has no visibility into what any other translation unit does to it.
   The build layer, which sees the whole link, is the only thing that can.
2. **Invalidate process-wide on a write.** 58:810-812 already says this. It is a whole-program
   property, so it is a whole-program layer's job.
3. **Refuse rather than fall back.** If clause 1 fails, the correct action is a build failure, not a
   silent substitution of the quantiser. Section 2.2's refusal posture is the compile-time half of
   the same rule and the build layer owes the link-time half.

The one cheap thing arvo may offer, and I agree with 58:812-813 that it should, is a debug-build
assertion comparing live control state against the declared one: three instructions of cfg-gated
inline assembly, Kind 1 structural lowering, no harness. That is a tool, not a policy, and it is
opt-out by being debug-only.

## 3. The radix-ten quantiser: what was radix-two shaped, and what it costs

58:845-846 and 58:1079-1080 both name this unbuilt and unmeasured, with the kernel's "shift alignment
is radix-two-shaped" as the reason. It is, in exactly three places, and none of them is the design.

### 3.1 Three shifts, and the design sentence that never mentioned two

The design's own statement of the quantiser is radix-general already:

> A `Ranged` numeral denotes the union, over `e` in `[EMIN, EMAX]`, of the grids with quantum
> `radix^(e - p + 1)` restricted to `[radix^e, radix^(e+1))` (58:220-222)

`radix`, not two. What is radix-two shaped is my own file 50 model, in three places:

| place | radix-two form | radix-`R` form |
|---|---|---|
| grid selection | `floor_log2`, via `leading_zeros` | `floor_log_r`, via `ilog(R)` |
| alignment and rounding | `<<` and `>>`, tie threshold `1 << (s-1)` | multiply or divide by `R^k`, tie compared as `2*lost` against `R^s` |
| carry-out renormalisation | `m >>= 1` | `m /= R` |

The repair is `mock/benches/variants/quantiser-radix-shared/src/rmodel.rs`, `R` as a const generic so
each variant monomorphises to its own radix and neither pays a runtime branch. The exponent function
`quantum_exp` needed no change at all, which is the finding hiding inside the mechanical work: **the
part of the quantiser that carries the design is already radix-general, and the part that was
radix-two shaped is the arithmetic kernel underneath it.** Section 1.17's conclusion, that "the design
was built to express this before anyone checked that it did", extends from the representation to the
quantiser.

### 3.2 Two facts the radix-two model could not have shown

**A tie is only reachable at an even radix.** A tie is `2 * lost == R^s`. For odd `R`, `R^s` is odd
and the left side is even, so no exact tie exists at any `s`, and every tie-breaking rule the
`Quantisation` axis offers is **vacuous** at radix three, five, seven or any other odd radix. Radix
ten is even, so decimal keeps its ties and the tie-breaking resolution is live there. This is a
degree of freedom the design exposes which has no inhabitants over half its own radix axis, and it is
worth a sentence in the spec because a consumer reading "the tie-breaking rule is yours to pick" at
radix three is being offered a choice with one outcome. Checked rather than asserted, over every
reachable `s` at radix three and radix ten (`an_odd_radix_has_no_representable_tie`).

**The carry-out renormalisation stays exact at every radix.** When it fires, `m == R^p` exactly, so
`m / R` loses nothing. The radix-two `m >>= 1` was not exploiting binaryness, it was exploiting that
the carry lands on a power of the radix, and that is radix-general.

**Three tests, and one of them found a defect in its own oracle first.** The radix-two
instantiation of the generalised kernel is checked bit-for-bit against the silicon over the bench's
own input distribution at all four swept spreads and 32 seeds each, 32,768 operations, zero
mismatches: that is the regression check that generalising did not break the path file 50 validated
over 41,380,159 operations. The radix-ten path has no silicon to check against on any target this
workspace pins, so it is checked against the **definition** instead, with an independent exact-integer
oracle asserting the delivered significand is the nearest decimal32 grid point and that ties went to
even, same coverage, 32,768 operations. That oracle panicked on its first run with an integer
overflow, and the cause was mine and instructive: near-total cancellation drops a result several
decades **below** either operand's own grid, so `q < s` is reachable and neither side of the
comparison may assume it is the coarser one. The kernel handled it; my check did not. I record it
because a reader should know the decimal oracle was wrong once before it was right.

*grounded on: `pin`, `tree` (the three tests are committed at
`mock/benches/variants/quantiser-radix-shared/src/lib.rs`), and the odd-radix statement is a
derivation checked by a test rather than a measurement.*

### 3.3 The measurement

A real bench under the harness, per `bench-and-sketch-discipline.md`, not a probe with a timer. Three
new crates (`quantiser-radix-shared` holding the kernel and the Routine, `quantiser-radix2` and
`quantiser-radix10` as the cdylib variants), wired into `mock/Cargo.toml`'s members,
`mock/benches/Cargo.toml`, `mock/benches/bench.toml` as
`[bench.decimal-quantiser-radix-sweep]`, and `mock/benches/src/main.rs`'s `routine_for_n`. Run:

```
cargo build --offline --release -p bench-quantiser-radix2 -p bench-quantiser-radix10 -p arvo-benches
cd mock/benches && ../target/release/arvo-benches
```

**Why the two variants are comparable, stated because it is the load-bearing methodological choice.**
Both consume the identical operand stream: 256 pairs of `(neg, mag, exp)` with `mag` drawn from
`[10^6, 10^7)`. That range is simultaneously a normalised seven-digit **decimal** significand and an
exactly-representable twenty-four-bit **binary** one, since `10^7 < 2^24`, so every operand is an
exact value of both formats and neither variant starts from an unrepresentable input. Exponents are
drawn in **grid steps**, so a pair `SPREAD` apart is `SPREAD` grid steps apart in whichever radix
reads it. Same `quantize`, same `exact_add`, same `floor_log_r`, monomorphised at `R = 2` and
`R = 10`. `SPREAD` is the swept axis because it is what decides how much alignment work an exact add
does, which is the step that was radix-two shaped.

`aarch64-apple-darwin`, Apple M1, `rustc 1.98.0-nightly (57d06900f 2026-05-27)`, `--release`, no
other codegen flags. 256 ops per call, matching the subnormal sweep's `N` so the per-op figures are
read off the same amortisation of the FFI boundary as 58:829-834's.

| exponent spread | radix 2 (binary32) ns/op | radix 10 (decimal32) ns/op | ratio |
|---|---|---|---|
| 0 | 16.43 | 29.25 | 1.78x |
| 2 | 17.23 | 32.13 | 1.87x |
| 8 | 20.80 | 37.68 | 1.81x |
| 20 | 24.66 | 42.00 | 1.70x |

Medians, second run. The first run gave 16.14 / 28.93, 17.18 / 32.37, 19.55 / 38.28, 24.84 / 43.23
for the same cells: every ratio reproduces within 0.1x and the largest single-cell move is 6%. CVs are
2.7% to 4.8% except one cell of the first run (radix 10 at spread 2, CV 24.3%) which did not recur.
Both runs flagged lag-1 autocorrelation in the 0.6 to 0.8 band, which the harness reads as warm-up
drift; I report the medians rather than the means for that reason and note that the ratio is what
this bench is for, and a drift affecting both variants of a pass affects it far less than either
absolute.

**Three readings, and they are different claims.**

1. **Decimal32 against binary32, both software.** 1.7x to 1.9x. This is the consumer-facing number
   and it is not a radix isolation: the two real formats have different precisions because that is
   what the standards say.
2. **Decimal32 against a native `fadd`.** Using 58:829-834's 1.20 ns/op hardware anchor, measured on
   this host through this harness at this `N`: **24x at spread 0, 35x at spread 20**. Against that
   table's software binary32 figure of 19.82 ns/op the radix-two column here is 16.4, close enough to
   be a consistency check and not close enough to be a reproduction; the distributions differ (no
   subnormals here, and no `f32` bit-level decode and re-encode either), so I state it as a
   consistency check and nothing stronger.
3. **The design question the number was for.** A decimal quantiser costs about twice a binary one,
   both in software, and the design has already accepted the software quantiser as the default
   lowering for every preset but one. **Decimal representability is therefore not nominal.** It costs
   what the design's own default already costs, doubled, on a path a `Hot` consumer was never going
   to take anyway, because no target this workspace pins implements decimal in hardware and section
   2.2's refusal is what a `Hot` decimal consumer meets.

*grounded on: `pin`, `host`, `flags`; artifacts `mock/benches/decimal-quantiser-radix-sweep_n{0,2,8,
20}.{csv,meta.json}` and the four `_findings.md`, committed.*

### 3.4 Where the decimal cost lives, and how much of it is reducible

Two attributions, and they are different kinds of statement.

**Measured, by disassembly.** `quantised_add` monomorphised at each radix, from the release cdylibs:

| variant | instructions in `quantised_add` | calls to `__udivti3` |
|---|---|---|
| radix 2 | 261 | 1 |
| radix 10 | 807 | 3 |

3.1x the static instruction count and three times the 128-bit-division libcalls. The static count
overstates the runtime ratio, as it always does, because not all of it is on the hot path; the
direction and the mechanism are what it establishes.

**Derived from the sweep's own shape, not measured directly.** Across spread 0 to 20 the radix-two
column rises 8.2 ns and the radix-ten column rises 12.8 ns, so the marginal cost per grid step of
alignment is about 0.41 ns and 0.64 ns respectively: a 1.6x radix penalty on the alignment term. But
the **fixed** term at spread 0 already differs by 12.8 ns, which is larger than the entire alignment
range. So most of the decimal penalty is not in the alignment loop at all. It is in the two things
that happen once per operation regardless of spread: finding the value's decade, and dividing by a
power of ten. The first is `ilog(10)` where radix two gets a `clz`; the second is `__udivti3`.

**The reducible part, named as an absence to attempt rather than a limitation to report**, per the
lead designer's novelty posture. The divisor is `R^s` with `s` bounded by the format's precision, so
it is drawn from a set of at most `P` values known at compile time. A table of precomputed reciprocals
turns all three `__udivti3` calls into multiply-high sequences, and decimal32's `P` is seven, so the
table is seven entries. Nothing about that is research; it is the standard constant-divisor
transformation applied to a divisor the type system already bounds. **So 1.7x to 1.9x is an upper
bound on the decimal penalty, not a floor**, and a spec sentence saying "decimal costs roughly twice
binary" should say "measured at roughly twice, with the dominant term a division by a compile-time-
bounded constant that no implementation has yet strength-reduced".

I did not build the reciprocal table. It is an optimisation of an implementation the design does not
yet have, and building it now would be optimising ahead of the thing being optimised.

*grounded on: `pin`, `host`, `flags` for the disassembly (`objdump -d --disassemble-symbols=<mangled>`
on the release cdylibs); the marginal-against-fixed split is derived from the section 3.3 table and is
marked derived.*

## 4. Two things outside the question, reported because they should be

### 4.1 The shipped strategy dispatch is load-bearing on a forbidden feature

I nearly carried `BitsContainerFor`'s Pattern-C const-tag dispatch forward as the door's precedent,
on the reasoning that the mechanism is shipped, ratified and free. It is shipped. It is not free.

`arvo-strategy/src/container.rs:254-258`:

```rust
const impl<const N: u16, Sign: Signedness> BitsContainerFor<N, Sign> for Hot
where Picker: Project<{ tag_hot_cold(N) }, Sign, { bytes_for_u16(N) }, Hot>
```

A const expression over a generic const parameter, in a bound. Probe 5 reduces it to essentials with
no arvo dependency and compiles it three ways:

| gate | outcome |
|---|---|
| none | `error: generic parameters may not be used in const operations`, help points at `generic_const_exprs` |
| `#![feature(min_generic_const_args)]` | `error: complex const arguments must be placed inside of a` `const` `block` |
| `#![feature(generic_const_exprs)]` | compiles, with the incomplete-feature warning |

Only the forbidden feature admits it, and `arvo-strategy/src/lib.rs:11` carries that gate today.
`unstable-features.md` already lists that line as drift to remediate, on the reading that the gate is
a stale annotation. It is not stale. It is **structural**: the shipped four-strategy container
dispatch does not compile without it, and the sound successor refuses the shape outright rather than
merely lacking a convenience.

This belongs in the live-defect registry as a new entry, and I state the repair honestly rather than
cheaply. The shape that replaces it is section 2.2's: the computed quantity becomes a type. That is
the spine rule (58:85-99) firing at a fifth or sixth position depending on which file's count you
take, and here it means the **width** would have to become a type, which is what `Nat` already is in
the numeral tower and what nothing in the shipped `Bits<N, S, Sign>` surface is. Rewrite cost is
large and touches every consumer of `Bits`. I am not proposing it in this file; I am recording that
the cheap reading of the drift entry ("delete a stale gate") is wrong, so that whoever picks it up
budgets for the real thing.

*grounded on: `tree` (`arvo-strategy/src/lib.rs:11`, `container.rs:254-258`, read fresh), `pin`,
`flags`, `59_probes/probe_5`.*

### 4.2 A new instance of the decoder-ring ceiling, at a position nobody had tested

58:1108-1109 leaves open whether `on_unimplemented`'s interpolation is safe against every carrier the
design ships. It is worse than unswept in one shape. Probe 3a carries the attribute on the tag
carrier and probe 3b carries it additionally on the outer trait, naming `{N}`. The two outputs are
byte-identical: **when the refusal fires on a bound over a projected associated type, rustc reports
the innermost unsatisfied bound and the outer trait's attribute is never rendered at all.** Dead text.

The consequence is a design rule, not a diagnostic tweak: the message must be written on the carrier
whose bound actually fails, in terms that make sense without naming the thing the consumer wrote. The
`required for` note recovers the numeral, so nothing is lost, but a spec sentence promising "the error
names your numeral" is false in this shape and true in probe 3c's, which is one more reason to prefer
the marker-on-the-numeral form over the projected tag.

*grounded on: `pin`, `flags`, `59_probes/probe_3a`, `probe_3b`, `probe_3d`.*

### 4.3 Every bench artifact this review has produced is discarded by a gitignore

`mock/benches/.gitignore` is six lines and the first three are `*.csv`, `*.meta.json`, `*_findings.md`.
So:

```
$ git ls-files mock/benches/ | grep -cE "findings|meta.json|csv"
0
```

**Not one bench artifact has ever been committed in this repository.** File 57's commit
(`d6ab342`, "bench: quantiser-vs-fadd subnormal sweep, dylib-shape fix") landed the bench code and
none of its numbers. The four benches that predate it are in the same state, and my own would have
been too.

This is drift against two things at once. `bench-and-sketch-discipline.md`, which this repo generates
into its own `.claude/rules/`, gives the artifact trail as the reason a bench belongs in the harness
at all: "every run emits the CSV + meta + findings artifact trail that makes the result reproducible,
auditable, and citable". And the workspace carries an op-stated rule that bench history is tracked and
never gitignored. The ignore defeats the rule the repo ships to justify the harness it runs.

The consequence lands on this review directly, and on the very decision this dispatch was sent to
compile. 58:824-841's table is the number the strategy-door assignment is supposed to spend, it is
grounded on `pin`, `host`, `flags`, and **its artifacts are not in the tree**. By the review's own
freshly adopted vocabulary that is `unreproducible` (58:709-716), applying to the newest measurement
in the review rather than the oldest, one file after the ground state was minted for file 08's table.

**I re-ran it, which is the check the vocabulary exists to prompt.** Same host, same pin, same
harness, same manifest, this dispatch's own run:

| PCT subnormal | 58:829-834 software | this run | 58:829-834 hardware | this run | 58's ratio | this run |
|---|---|---|---|---|---|---|
| 0 | 19.82 | 19.92 | 1.20 | 1.47 | 16.5x | 13.5x |
| 25 | 19.84 | 18.72 | 1.19 | 1.47 | 16.7x | 12.7x |
| 50 | 17.40 | 17.25 | 1.41 | 1.38 | 12.4x | 12.5x |
| 100 | 15.85 | 14.15 | 1.20 | 1.38 | 13.2x | 10.2x |

The software column reproduces within 1% to 11%, which is a real reproduction. The hardware column
moves 15% to 25%, which is unsurprising for a single `fadd` measured near the harness's own
resolution floor, and it is what drags the ratio. **So the consolidation's "13x to 17x, at every point
of the sweep" is tighter than the measurement supports; "roughly an order of magnitude, ten to
seventeen across two runs" is what two runs say.** Nothing built on that number changes: the software
quantiser is expensive enough that the door is worth having, which is the only load it was carrying.
File 57's own restraint about the falling-with-subnormal-fraction pattern was right, and this run
reproduces that pattern too.

**The repair, which I made rather than filed.** The three artifact patterns are removed from
`mock/benches/.gitignore` and every artifact this run produced is committed, labelled as this run's.
I did not commit them under file 57's name: they are a fresh measurement of its bench, not a
reproduction of its build, and 58:713-716 forbids presenting one as the other. The four pre-existing
benches' artifacts land as this run's too, for the same reason.

*grounded on: `tree` (`mock/benches/.gitignore`, the `git ls-files` count, commit `d6ab342`), and
`pin`, `host`, `flags` for the re-run.*

## 5. What this file leaves for the next one

**Carried forward as shape, for a consolidation to take directly:** section 1's derivation that the
door is a `Lowering` member because `Lowering` is the axis no law may read; section 2.2's
`DefaultLowering<N>` projection with host-implementedness as a sealed marker on the numeral, refusal
rather than fallback; section 2.3's corrected table, whose content is the presumptive table's and
whose four justifications are new; section 2.4's uniformly-`Hot` theorem; section 2.5's three
build-layer obligations; section 3.1's radix-general kernel and section 3.2's odd-radix vacuity.

**Droplist additions.** The door as a projection from the strategy alone, with a software fallback
refined where the numeral is host-implemented: refused by coherence (E0119) and, separately, by
`min_specialization` twice, and the only opener is a forbidden feature. And: `Cold`'s door justified
as "follows the semantics-first side", which reaches the right answer from a storage fact that does
not imply it; replaced by `Cold`'s own widen-operate-narrow meaning, which does.

**One thing the next consolidation should fold in rather than treat as mine.** Section 4.3's
re-measurement changes a printed table (58:829-834) and its headline sentence (58:836). That is a
ratified-table-adjacent edit of the same kind the persona checkpoints have been flagging, and it
should go on the loudest-for-op list rather than stay in a member file.

**Open, and I am not closing them.** Whether `Hot`'s default environment should be `IeeeDefault` at
all, or whether a preset should decline to name one and force the consumer to pick a lowering
explicitly the moment a float meets `Hot`; I compiled the first and believe it is right on
toolbox grounds (a default that is the standard's own default is not a policy), but it is a call, it
is one expert's, and it wants a second read. Whether the `HostImplemented` marker set is cfg-gated
per target inside arvo or supplied by the build layer, which is the same locus question section 2.5
answers for the receipt and I did not ask for the marker. Whether the reciprocal-table strength
reduction in 3.4 actually lands the decimal ratio near 1.2x or near 1.6x, which is a measurement
nobody can take until there is an implementation to take it against. And decimal's own residuals from
58:1085-1090 are untouched by this file: the `InfOnly` witness, the two secondary-source figures owed
the file-39 treatment, and the `u64` readout ceiling.

**One thing I want to flag for op specifically**, since section 2 edits a table the persona checkpoint
marked as needing his eye (58:1014). The mechanism it adopted survives the compile intact. What
changes is that `Hot`'s row is conditional and the other three rows are not choices, so the item in
front of him is smaller than it was: not "is this assignment right" but "is `IeeeDefault` the right
default environment for `Hot`, and is refusal the right answer when `Hot` meets a numeral the host
does not implement". The second of those I believe the shipped tree already answered
(`arvo-strategy/src/container.rs:104-112`) and the first I do not think anyone but him should settle.
