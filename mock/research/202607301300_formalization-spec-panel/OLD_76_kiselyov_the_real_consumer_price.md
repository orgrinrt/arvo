# 76. The real consumer price: what the shared carrier costs, and the guarantee that decides the fork before the cost does

Oleg Kiselyov, file 76. I wrote files 02, 36 and 54. File 36 built the value-unique tower this dispatch
prices; two of its claims have since been overturned by other members (the `Bias = Int` reading, corrected
at `41:25` and `40:582`, and one further statement overturned at `54:57`), so I treat nothing in my own
earlier files as standing and re-derive below what I need. My subject is staging: which computations
belong at expansion time, which at type-check time, and what each costs the person who did not write
either.

**What I read.** `68_consolidation_seven.md` in full as the standing base, with its two stated warnings
carried (the preset table superseded by file 70's and ratified at `70b`; the lowering-door table void).
`74b_op_checkpoint_eighteen.md` in full, which sets this dispatch, and `74_lattner_the_taxonomy_rechecked.md`
in full, which re-scoped it. `70_wronski_the_presets_re_derived.md` and `75_aaltonen_what_bitpacked_means.md`
by section heading plus their gate paragraphs. Behind those, only where a derivation was load-bearing:
`63:665-679` (the cost model this bench does not replace), `53:150-180` (the named cliff and its profile),
`65_probes/OUTCOMES.md` and `65_probes/probe_9_predicates_under_each_route.rs` in full (the route-Y and
route-Z predicate shapes, which I rebuilt rather than reused), and `74_probes/capacity.rs` and
`74_probes/unify.rs` in full, because section 2 corrects the scope of what they establish. One `ls` of the
panel directory and one of `76_probes/`, `74_probes/`, `65_probes/`, `54_probes/`, `36_probes/` at the
start. Workspace rules leaned on and quoted rather than paraphrased: `unstable-features.md`,
`arvo-toolbox-not-policer.md`, `arvo-compile-time-last.md`.

**Gates.** Canon gate, reproduced fresh from the repo root this session: `grep -rln
"Adjustment\|Bias\|Numeral" mock/crates/ --include="*.rs"` and the same with `FullRange\|UTerm\|AddWidth`,
both exit 1, empty. Test gate, run fresh and summed per binary: `cargo test --offline --workspace` from
`mock/` reports **661 passed, 0 failed, 9 ignored**, which matches `75:28` exactly and is the standing 658
plus the three tests `bench-bitpack-shared` adds; I checked that attribution rather than accepting it
(`grep -c "#\[test\]" mock/benches/variants/bitpack-shared/src/lib.rs` returns 3, and they are the
`column256`/`column4096`/`column16384` round-trips). File 74's gate line at `74:29` carries 658 "matching
`68:64-65`", which was correct when written and is now stale by three; noted so the next reader does not
treat a moving figure as a fixed one. Toolchain confirmed inside the tree: `rustc 1.98.0-nightly
(57d06900f 2026-05-27)`, `aarch64-apple-darwin`; the identical command from `/tmp` reports stable
`1.94.0`, and this cost me a first harness that silently measured the wrong compiler (section 6).

**Test bodies read, in the surface this file touches**, which is the capacity foundation, since the
unification puts it under every consumer: all six files under `arvo-tensor/tests/` in full (408 lines),
plus `arvo/tests/no_multiplicative_identity.rs` and its nine `trybuild` fixtures. One finding, small and
stated for proportion. `arvo-tensor/tests/capacity.rs:14-18`, `dim_cap_is_typed_and_exact`, asserts
`<Dim<3> as Capacity>::CAP == cap(3)` against an impl whose body is `const CAP: Cap = cap(N)`. That is
`cap(3) == cap(3)`: it compares a computed result to the same computation and cannot fail unless `cap` is
not a function. It should be deleted rather than improved, and it is three lines in a file whose other two
tests carry real load (`generic_build_and_walk_is_gce_free` is the shape the whole capacity foundation
exists to make expressible). Everything else in that surface is real, and
`no_multiplicative_identity.rs:9-24` is the best-shaped test in the tree I have read: nine compile-fail
pins, per impl and per strategy, with the file's own comment recording that one case was not a pin because
loosening the signed bound left the suite green.

**What is compiled and measured, against what is reasoned.** Sections 2 through 5 are compiled or measured:
thirteen probe files plus three generators and three timing harnesses in `76_probes/`, every outcome
verbatim in `76_probes/OUTCOMES.md`,
every timing through `hyperfine` with two warmups, every arm carrying a positive control that pins the
computation to a value known independently and a negative control that refuses when the pin is corrupted.
Section 1 (the exit condition) is a judgement, stated before any of it ran. Section 6 (the staging rule)
and section 8 are reasoned from the measurements. Per the method constraint ratified at `70b`, no shipped
source or comment is read as design meaning anywhere below. Two shipped-tree reads appear and both are
factual checks that survive deletion: the consumer census in section 5, which is a count of what consumers
wrote and not a claim about what the design means, and the `arvo-tensor` test bodies above, which the test
gate requires.

## 1. The exit condition, stated before the measurement

This is written before any number in this file existed, because the alternative is choosing the threshold
after seeing the result and calling it a measurement. Three clauses, and they are ordered: a later one is
not reached if an earlier one fails.

**Clause 0, feasibility.** The unification is priced only if it can be expressed. `74b:28-33` adopts one
sealed bottom carrier with `Capacity` kept as a semantic alias over it, and a capacity's entire job is to
name the backing storage for a count. So the clause is: does the shared carrier admit a `type Array<T>`
that is a real contiguous array, under the permitted feature set, with `generic_const_exprs` forbidden. If
not, there is no cost to compare and the fork's premise, not its price, is what needs revising.

**Clause 1, guarantee parity.** The two routes must refuse the same things at the same time, or the
comparison is between different products. The specific obligation is the one the review installed for the
`UFixed<0, F>::ONE` defect: a purely fractional numeral must not have a multiplicative identity, and it
must be refused **under `cargo check`**, because that is where a consumer looks and what a consumer's
editor runs. An arm that refuses only at codegen, or only for the widths someone tabulated, has not
discharged the obligation, and its cost number does not belong beside the other's. This clause exists
because the review has been caught once already by arms that forced different obligations, and because I
was about to be caught by it again in this very file (section 4).

**Clause 2, the cost.** Measured on the crate that declares the numerals and the capacities, as `cargo
check` wall time, with the machinery compiled as a dependency rather than pasted into the same file,
because a per-consumer cost and an amortised one are different quantities and only one of them is what a
consumer pays.

- **Closes to route Z** if, at the consumer's real declaration count, the added cost sits below the
  run-to-run standard deviation of the consumer's existing whole-workspace check, **and** growth is at
  most linear in the number of distinct declarations, **and** the magnitude of the values is free. The
  first threshold is not taste: a cost smaller than the noise of the build it sits inside is not
  perceptible by any consumer at any time, and `arvo-compile-time-last.md:16` licenses far more than that
  ("compile time is the last cost we try to minimise, not the last place we shift work into. It is the
  bucket we pour into, freely, when doing so buys runtime or correctness").
- **Closes to route Y** if the added cost exceeds a quarter of that baseline, **or** if any superlinear
  term appears anywhere inside the consumer-shaped region. A quarter is where an edit-check loop changes
  character rather than merely lengthening. The consumer-shaped region is fixed here, in advance, at up to
  400 distinct numerals and 400 distinct capacities, an order of magnitude past the census, and at
  capacity values up to 2^32, because a column store's capacity is not thirteen.
- **Between the two, it goes to op with both numbers**, because that band is a taste call about a trade
  the ratified rule already licenses in principle.

Per-composition milliseconds appear nowhere in these clauses on purpose. Every prior measurement of this
design reported them and every one was quietly reassuring, because a number with no denominator cannot be
alarming.

*Grounded on: ratified (`74b:28-47`, `arvo-compile-time-last.md:16`), settled shapes (`68:703-710`,
`74:162-171`), reasoned (the thresholds and the region, chosen before measuring).*

## 2. Clause 0 fails for the spelling that was ratified, and the probe that cleared it never tested this

**The situation, from ratified text only.** `74b:26-43` adopts the unification on the finding at `74:63`
and `74:110-119` that the design holds "two type-level natural encodings of one concept". That finding is
true and I am not disputing it. What it understates is that the two differ in **kind**, not in spelling,
and the difference in kind is the whole price. The tower's is inductive and value-unique, ratified at
`44b` and stated at `68:549-556` as `Nat ::= Z | Pz<P>`, `Pos ::= H | O<P> | I<P>`: a number is a
recursive type. The capacity side's is what `unstable-features.md:74` records op's own migration as
producing, in its own words, "the capacity is a TYPE ... so no `cap_size` expression sits in type position
and no `Cap` const generic appears": a type whose parameter is an array-length const. Read the two
ratified sentences side by side and they describe a recursive encoding and a const-parameter wrapper.
Unifying them is not choosing between two spellings; it is replacing every capacity in the workspace with
a recursive type and then asking that recursive type for an array length.

**Written the obvious way, it does not compile, and rustc names the forbidden feature.**
`76_probes/a1_naive_unification.rs` is the shared sealed carrier, `Capacity` as one blanket alias over it
per op's condition, and `type Array<T> = [T; <N as Nat>::VAL]`:

```
error: generic parameters may not be used in const operations
  --> a1_naive_unification.rs:68:26
   |
68 |     type Array<T> = [T; <N as Nat>::VAL];
   |                          ^ cannot perform const operation using `N`
   |
   = help: add `#![feature(generic_const_exprs)]` to allow generic const expressions
```

The help names `generic_const_exprs`, which `unstable-features.md:74` forbids outright, with op's own date
on it (2026-07-28), and forbids on the reasoning that the sanctioned successor exists. So the successor
gets its turn. `76_probes/a2_escapes_refused.rs` takes the const-block form and a `min_generic_const_args`
`type const` associated item; both refuse, with rustc suggesting the whole chain be made `type const`.
`76_probes/a3_type_const_all_the_way.rs` does exactly that, following the compiler's own advice down to
the inductive step, and gets:

```
error: complex const arguments must be placed inside of a `const` block
  --> a3_type_const_all_the_way.rs:30:29
   |
30 |     type const VAL: usize = 2 * P::VAL;
```

and `a3b` adds the suggested const block and gets the identical error. `min_generic_const_args` cannot
express `2 * P::VAL`, which is the entire content of a binary inductive natural. This confirms, on this
pin and for this shape, the finding `unstable-features.md` records from the 2026-05-29 sketch ("a `type
const` RHS cannot use a generic parameter"), which was recorded for a different shape and which I checked
rather than carried.

**Why the feasibility probe that cleared the unification did not see this, stated precisely and without
blame.** `74_probes/capacity.rs:8-10` declares the capacity trait as

```rust
pub trait Capacity {
    const SIZE: u128;
}
```

and `74_probes/unify.rs` proves the payoff through `<... as Capacity>::SIZE == 13`. Both probes are
correct and both establish exactly what they say: the **number** unifies, the seal survives the crate
split, the two domains' names become one type. What is absent from the probe is the associated type the
capacity domain exists for. The conclusion at `74:144-151`, "the whole load-bearing path", is wider than
the path compiled, and the missing segment is the one that does not compile. This is the same shape as the
convention `68` adopted two stretches ago after the uncited sketch: a universal claim about a mechanism
owes a compile of the part that carries the weight.

*Grounded on: ratified (`unstable-features.md:74`, `44b` via `68:549-556`, `74b:26-43`), compiled
(`76_probes/a1`, `a2`, `a3`, `a3b`, four refusals with rustc's own diagnostics), settled shapes
(`74_probes/capacity.rs:8-10`, read as an artifact of this review rather than as shipped design). The
argument survives deleting every shipped-source citation: two ratified sentences describing encodings of
different kinds, and a compiler that refuses the composition, is the whole of it.*

## 3. Two constructions that do express it, and the one the design should take

The novelty posture distinguishes what is impossible from what nobody has done. Section 2 establishes that
nobody can write the naive spelling. It does not establish that the unification is unavailable, and both
routes below compile with zero feature gates.

**Construction one: derive the storage structurally.** `76_probes/b1_structural_array.rs`. The array stops
being a const expression and becomes a type-level function of the encoding, which is the identical move
the design already made once at `65_probes/probe_4`, where the container bucket stopped being a const fn
and became three type-level functions. `O<P>` doubles its child's storage, `I<P>` doubles it and adds one
slot, `#[repr(C)]` makes the nest layout-identical to a flat array, and `as_slice` recovers the flat view.
It compiles, no gates, and the layout law holds over eight capacities at three element types of different
size and alignment, twenty-four compile-time assertions.

It costs one `unsafe` at the bottom of the design, which I state rather than bury, and that cost is what
makes the discharge interesting. The precondition is not asserted in a list of capacities someone
remembered; it is an inline const block **inside `as_slice`**, evaluated per monomorphisation, so a
capacity a downstream crate invents cannot reach the cast without the check running.
`76_probes/b1c_perimeter_control.rs` is that claim under attack: corrupt the odd-arity node, name a
capacity no assertion list mentions, reach it only through the door, and the build refuses with
`evaluation of <Pz<I<I<O<H>>>> as Capacity>::as_slice::<u32>::{constant#0} failed here`.
`76_probes/b1b` is the ordinary negative control and it fires at exactly four of eight capacities, and
exactly the four whose encodings contain `I`, so the law discriminates per constructor rather than firing
on anything.

**Construction two, and it is the cheaper and the more principled: split by layer.**
`76_probes/b2_split_by_layer.rs`. The count is the shared carrier. The array grammar is not.

This is the design's own layer-keying rule, applied to a place nobody has applied it: *a fact is keyed on
the coarsest layer whose identity its truth depends on* (`68`, section on the three design rules). A
count's arithmetic, ordering, comparison and every law quantified over it depend on the **value**, so they
key on the shared `Nat`, and that is exactly the unification op ratified. The array grammar `[T; K]`
depends on nothing but the carrier, and `K` is a language-level array-length const, which is the one place
the grammar forces a literal. Keying the grammar on the value is the rule's own named failure, in the same
way `74:196-203` found the clamp target was being keyed on the wrong layer.

So `Slot<N, const K: usize>` pairs the two, and the two cannot disagree: `agrees::<N, K>()` is checked in
an inline const block at the only construction door, and `76_probes/b2b_disagreement_refused.rs` puts a
13-count behind a 12-slot array and gets `capacity's declared length disagrees with its value`.

The introduction route is host-staged: the declaration macro emits the reduced encoding and the literal
length together, already agreeing, so neither is computed by the type checker at any use site. That is not
a workaround; it is the correct binding time for a fact the consumer states rather than derives.

**Which to take.** Construction two, and I would say so even if it were the slower one. It adds no
`unsafe` to the bottom of the design, it needs no layout argument, it puts `Capacity`'s two halves on the
layers the design's own rule assigns them, and it leaves `[T; K]` as the plain array every consumer and
every debugger already understands. Construction one is worth keeping in the record as the answer if a
future requirement makes the paired literal unavailable, and worth keeping because its discharge-at-the-
door pattern is reusable and is used again in section 4.

*Grounded on: compiled (`76_probes/b1`, `b1b`, `b1c`, `b2`, `b2b`, all zero-gate, with negative controls),
settled shapes (the layer-keying rule at `68`, `65_probes/probe_4`), reasoned (the recommendation between
the two).*

## 4. Clause 1 fails for route Y, three ways, and this is what decides the fork

This section is the one I nearly got wrong, in exactly the way the dispatch warned about. I built a fourth
arm, `ys`, that stages route Y's predicate the way section 3 stages the capacity's length: the declaration
macro decides `I > 0` at expansion time and the type carries a sealed witness, so `OneRepresentable` costs
one impl instead of a table. It compiles, it is fast, and I had it in the results table beside route Z
before I attacked it. It should not have been there.

**Route Y candidate one: the two-dimensional impl table.** This is the expression `65_probes/probe_9`
names as the only one available. It refuses correctly, at type-check. What it costs is a policy the
substrate is forbidden to set. Measured, at the real consumer profile, 14 numerals:

| table ceiling | impls | `cargo check` equivalent |
|---:|---:|---:|
| 16 | 136 | **build fails**, 3 errors: the census's widths 27, 28 and 64 are not in the table |
| 32, 48 | 528, 1176 | **build fails**, 1 error each: width 64 is not in the table |
| 64 | 2080 | 267 ms |
| 96 | 4656 | 840 ms |
| 128 | 8256 | 2190 ms |
| 160 | 12880 | 4988 ms |
| 192 | 18528 | 9780 ms |
| 256 | 32896 | **30031 ms** |

The bottom row was measured rather than extrapolated, after an extrapolation from the measured exponent
predicted 29 s. The exponent in the impl count climbs from 1.42 to 1.86 across the sweep, so the cost is
roughly quadratic in the table and roughly quartic in the ceiling.

The top three rows are the finding, not the bottom one. **The ceiling is not free to choose.** At 16, 32
and 48 the build fails on widths the census says a consumer already writes, so 64 is a floor forced by
real code. And `arvo-toolbox-not-policer.md:60` is explicit: "No bit-width cap below the largest container
the substrate is willing to dispatch through." The substrate dispatches past 128. So route Y's predicate
is priced on a number arvo is not permitted to pick small, and at the 512 the sweep's own exponent implies
it is roughly six minutes per build, per consumer, forever.

**Route Y candidate two: stage the predicate.** `76_probes/c_ys_attack_1_only.rs` forges the affirmative
witness on `Num<0, 8, OneYes, Hot>`, which is precisely the `UFixed<0, F>::ONE` defect re-offered through
the staged door. The result, and it is the sharpest thing in this file:

```
$ rustc --edition 2024 --crate-type=lib --out-dir out c_ys_attack_1_only.rs
error[E0080]: evaluation panicked: one-witness disagrees with the widths

$ rustc --edition 2024 --crate-type=lib --emit=metadata --out-dir out c_ys_attack_1_only.rs
$ echo $?
0
```

**Caught at `--emit=link`. Not caught at `--emit=metadata`.** A monomorphisation-time const assertion is
evaluated when a function is instantiated for codegen, and `cargo check` does not instantiate. So under
the command a consumer runs on every save, a purely fractional numeral claims a multiplicative identity
and nothing complains. Route Z's `OneRepresentable` is a trait bound and refuses at type-check, which is
the difference clause 1 exists to catch. The seal does its job on the other attack (a downstream marker
offered as a witness is refused with `MyOwnYes: Sealed is not satisfied`), so this is not a hole in the
sealing discipline; it is a hole in the binding time, and it is a hole staging cannot close, because
`I > 0` on a const parameter has no type-check-time expression.

**Route Y candidate three, which nobody has named and which I owed the posture: let the consumer emit the
impl.** If the declaration macro emits `impl HasOne for UFixed<13, 0, Hot>` only when the integer part is
nonzero, the table exists but is linear in what the consumer declared, refuses at type-check, and needs no
ceiling at all. It is the right idea and the orphan rule kills it. `76_probes/d_lib.rs` and
`d_consumer.rs`, two crates, the library exporting the macro and the consumer calling it:

```
error[E0117]: only traits defined in the current crate can be implemented for types defined outside of the crate
  = note: impl doesn't have any local type before any uncovered type parameters
  = note: define and implement a trait or new type instead
```

A numeral whose parameters are all consts contains no local type, so only arvo may write its impls, and
only the consumer knows which numerals it will declare. The two facts are incompatible. rustc's own note
states the remedy: "define and implement a trait or new type instead", which is to make the widths types,
which is route Z.

**So route Y has no expression of the guarantee that is simultaneously correct at check time,
consumer-extensible, and not priced on a ceiling arvo is forbidden to choose.** That is stronger than
`68:704-705`'s "leaves one predicate with no honest expression", and it is stronger in the way that matters:
the three candidates are enumerated and each is killed by a compiler diagnostic rather than by an argument.

*Grounded on: ratified (`arvo-toolbox-not-policer.md:60`), compiled (`76_probes/c_ys_attack`,
`c_ys_attack_1_only`, `d_lib`+`d_consumer`, three refusals and one silent pass), measured (the ceiling
sweep, `76_probes/results.csv`), settled shapes (`65_probes/probe_9`).*

## 5. Clause 2, measured: linear, magnitude-free, and below the noise of the build it sits in

The consumer is not a synthetic sweep. Its numerals are the ones the design's heaviest real consumer
writes, counted this session across the workspace:

```
$ grep -rhno 'Uint<[^>]*>|Int<[^>]*>|UFixed<[^>]*>|IFixed<[^>]*>' \
    hilavitkutin/mock/crates --include='*.rs' | sort | uniq -c | sort -rn
```

which gives widths 1, 2, 3, 4, 5, 6, 7, 11, 14, 16, 27, 28 and 64, one purely fractional at (0, 16), and
strategies `Hot`, `Warm` and `Cold`, across 40 sites. Capacities come from the same tree, where
`grep -rn 'Cap<\|Capacity<\|Cap::\|cap_size' hilavitkutin/mock/crates --include='*.rs'` returns 107 and
`grep -rn ': Capacity\b'` returns a further 135 bound sites, which are the ones the unification puts the
carrier under. Both counts are zero in vehje and kolli.

Every arm compiles the same program: the same numerals, the same pairwise arithmetic,
the same fold, the same capacity-generic container, and the same function generic over a numeral **and** a
capacity at once, which is the site staging cannot reach because neither width is known there.

Every `z` arm pins each computed width to the value the generator knows independently
(`const _: () = assert!(W3 == 4);`), so a solver that skipped the fold would fail to compile rather than
report a flattering number; `76_probes/gen/nc_z.rs` corrupts one pin and the build refuses with
`assertion failed: W3 == 5`. The measurement is of work that happened.

**Count sweep**, n distinct numerals and n distinct capacities, `--emit=metadata`, mean of five runs after
two warmups, milliseconds:

| n | none of it | route Y staged | route Z staged | route Z |
|---:|---:|---:|---:|---:|
| 14 (the census) | 54 | 58 | 71 | **70** |
| 50 | 56 | 65 | 94 | 96 |
| 100 | 69 | 82 | 147 | 144 |
| 200 | 89 | 124 | 237 | 238 |
| 400 | 125 | 181 | 443 | 438 |
| 800 | 208 | 311 | 890 | 873 |

Route Z's doubling ratios from n=50 are 1.50, 1.65, 1.84, 1.99, converging on exactly linear, with the
sub-linear behaviour at small n being fixed overhead rather than a discount. **There is no cliff anywhere
in the region fixed in section 1, nor at twice its far edge.**

**It does not amortise into the dependency, and that had to be measured separately.** Compiling the
machinery and the consumer as one file is not the consumer situation, since arvo is a dependency compiled
once. `76_probes/split_bench.sh` splits the same program at the machinery boundary, builds the machinery
as an rlib, and times only the consumer crate against it: 66, 107, 151, 248, 459 ms at the same counts,
within noise of the single-file figures. A type-level natural is instantiated at the **naming** site, so
the consumer pays for its own declarations and the library's compilation buys it nothing. That is the
honest framing and it is the less flattering one.

**The magnitude of a capacity is free, which is the axis the unification newly puts under every container
and which a count sweep cannot see.** A column store's capacity is 65536, not 13, and under the
unification that is a seventeen-deep type. Thirty-two distinct capacities per point, each pinned to its
own value:

| value near | 2^4 | 2^8 | 2^12 | 2^16 | 2^20 | 2^24 | 2^28 | 2^32 | 2^40 | 2^48 | 2^56 | 2^62 |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| ms | 69 | 67 | 63 | 63 | 65 | 62 | 70 | 67 | 71 | 74 | 75 | 72 |

Flat to sixty-two bits of depth, with the negative control confirming the chains are genuinely evaluated.
The binary encoding is logarithmic and its constant is under the noise floor. Combining the axes changes
nothing: 400 numerals and 400 capacities near 2^16 together cost 462 ms against 438 ms at census-sized
capacities.

**The denominator.** `cargo check --offline --workspace --all-targets` in `arvo/mock`, after touching the
facade crate root, is **6.35 s +- 0.09** over three runs.

So at the consumer's real declaration count the entire shared carrier costs **16 ms on a 6350 ms build,
which is 0.25%, and is one sixth of that build's own run-to-run standard deviation.** Clause 2's first
threshold was "below the noise of the build it sits inside", chosen before measuring, and the result is
not near it; it is a factor of five inside it.

**Flags, because this review has twice found a headline that was measuring flags.** Route Z at n=400,
metadata: default solver 438 ms, `-Znext-solver` 415 ms, `--emit=link` 872 ms against a base of 489 ms.
Route Y at ceiling 128: default 2204 ms, `-Znext-solver` 2626 ms, `-Zthreads=1` 2217 ms. Every result
survives every flag tried, and route Y is slightly **worse** under the next-generation solver, which is
the direction that would have rescued it if any were going to.

*Grounded on: measured (`76_probes/results.csv`, `split_results.csv`, `deep_results.csv`, all with
positive and negative controls), settled shapes (the census, counted this session; it is a count of
consumer text, not a reading of design meaning).*

## 6. The staging answer, which is a negative result, and the rule it yields

The dispatch asks whether the move that paid earlier is available here: an earlier file measured that
moving reduction to the host was several times cheaper than making the type checker do it.

**It is available and it buys nothing.** Arms `z` and `zs` are indistinguishable at every point of the
sweep (70/71, 96/94, 144/147, 238/237, 438/443, 873/890), and the difference between them is precisely
whether the width sum is folded by the trait solver at each declaration or emitted already reduced by the
declaration site. I expected a gap and there is none, and the reason is worth more than the number would
have been.

The rule, and I offer it as spec text because it explains both results rather than just this one:

> **Stage the reduction whose cost is superlinear in the value; do not bother staging one that is
> logarithmic.** A binary inductive natural's addition is linear in the number of **bits**, so a
> width of sixty-four is a seven-node encoding and its sum is a handful of solver steps that the type
> checker's own overhead swallows. A rational's reduction needs a gcd, whose type-level cost is
> superlinear in the operands, and that is where the review's measured cliff lives. The two look like
> the same operation ("compute a number at type level") and are a factor of roughly three hundred apart
> per distinct site (`63:670-679`'s 143 ms against this file's 0.51 ms). The axis that separates them is
> the cost curve of the computation, not the fact that it happens in types.

The corollary the design should carry: **staging is a treatment for a diagnosis, not a policy.** Applying
it everywhere costs macro machinery and a second introduction route at every declaration site, and buys
nothing wherever the underlying computation is cheap. Section 3's capacity macro stages the array length
not for cost but because the length is a fact the consumer **states** rather than derives, and the type
checker has no way to derive it at all. Different reason, same mechanism, and the spec should say which
reason applies where.

*Grounded on: measured (the `z` against `zs` columns, `76_probes/results.csv`), settled shapes (`63:670-679`
for the gcd-side cost model and `53:162-171` for the cliff), reasoned (the rule and its corollary).*

## 7. What this does not overturn

A favourable number is exactly the kind of result that gets read wider than it was taken, so the boundary
is stated rather than left to inference.

**The named cliff stands, untouched.** `63:670-679` and `53:162-171` price one hundred distinct arbitrary
rational compositions at 14.3 s and four hundred at 63.7 s. That is `Bias` composition through a
type-level gcd. This file prices **width and capacity naturals**, whose type-level operation is binary
addition and comparison. The two are different computations with different curves, and section 6 says why.
Nothing here licenses removing that cliff from the attempt list, and a reader who takes "the shared
carrier is linear and free in magnitude" as "type-level number machinery is cheap in this design" has
generalised past every measurement in this file.

**This file prices the carrier, not the whole facade migration.** `68:687-702`'s estimate of one to three
weeks for the facade gate is about edits and decisions, and is untouched. What changes is that one of the
four decisions is now settled on evidence.

**One arm in my results table is measuring a weaker guarantee, and stays in the table with that stated.**
Route Y staged (`ys`) is genuinely cheaper than route Z at every point, and it is cheaper because section
4 shows it does not refuse under `cargo check`. Its column is retained so a later reader can see the size
of the discount a missing guarantee buys: at the far end of the sweep it adds 103 ms over the baseline
where route Z adds 665 ms, so the guarantee costs 562 ms at 800 distinct numerals and 4 ms at the
consumer's real count. Neither figure makes the discount worth taking.

## 8. The verdict, and what it hands forward

**For the next consolidation, in provenance form.**

*The facade fork closes to route Z, and it closes on the guarantee before it closes on the cost. Route Y
has no expression of the `OneRepresentable` guard that is simultaneously correct under `cargo check`,
extensible by a consumer, and not priced on a width ceiling `arvo-toolbox-not-policer.md:60` forbids arvo
to set: the impl table refuses correctly but costs 30.0 s at a 256 ceiling and fails outright below 64,
where the census's own widths already sit; a host-staged witness is cheap but is caught only at
`--emit=link` and passes `cargo check` silently, re-opening the `UFixed<0, F>::ONE` defect at the command
consumers actually run; and a consumer-emitted per-declaration impl is refused by the orphan rule
(`E0117`), whose own note names route Z as the remedy. All three compiled (`76_probes/c`, `d`, and the
ceiling sweep). Route Z's cost, measured against the widths and capacities the heaviest real consumer
actually writes and with the machinery compiled as a dependency, is 16 ms on a 6.35 s +- 0.09 whole-
workspace check, which is 0.25% and one sixth of that build's own run-to-run standard deviation; growth is
exactly linear in distinct declarations out to 800, twice the far edge of the region fixed before
measuring; and the magnitude of a value is free to sixty-two bits of encoding depth. Every figure survives
`-Znext-solver`, `-Zthreads=1` and `--emit=link`.*

*The unification ratified at `74b` needs one amendment before it is buildable. Its obvious spelling, the
shared carrier answering for the backing array, has no expression under the permitted feature set: rustc
names `generic_const_exprs`, which `unstable-features.md:74` forbids, and `min_generic_const_args` cannot
express the inductive step `2 * P::VAL` even with the const block its own diagnostic suggests (four
compiled refusals, `76_probes/a1` through `a3b`). The feasibility probe that cleared the unification
declared the capacity trait as `const SIZE: u128` alone (`74_probes/capacity.rs:8-10`) and so never
reached the associated type the domain exists for. The amendment is the layer-keying rule applied to the
capacity: the count keys on the shared carrier, which is the unification op ratified and it holds; the
array grammar keys on the lowering side as a literal paired with the count, with their agreement checked
in an inline const block at the one construction door (compiled, zero gates, `76_probes/b2`, with the
disagreeing pair refused at `b2b`). A second construction exists and is recorded as the fallback: the
storage derived structurally from the binary encoding under `repr(C)`, layout-identical to a flat array,
its one `unsafe` discharged per monomorphisation inside the only door rather than by a list of
capacities someone maintained (`76_probes/b1`, with the perimeter control at `b1c` refusing a capacity no
assertion names).*

*One spec sentence on binding time follows from the measurement: stage a type-level reduction whose cost
is superlinear in the value, and do not stage one that is logarithmic. Route Z's width sum, staged and
unstaged, is indistinguishable at every point of the sweep, because binary addition is linear in bits; the
review's measured cliff is a gcd, which is not. Staging is a treatment for a diagnosis rather than a
policy, and where it is used for a different reason, as the capacity's array length is because the length
is stated rather than derived, the spec should say which reason applies.*

**Open, and op's.** Whether the capacity's array grammar sits where section 3 puts it, since it amends a
ratification made one file ago. Whether construction one is kept in the spec as a recorded fallback or
dropped. The tautological test at `arvo-tensor/tests/capacity.rs:14-18`, which should be deleted rather
than improved and which is not mine to delete.

**For the exploring rhythm, one dispatch-sized item this raised and did not resolve.** Section 4's third
candidate died on `E0117` because a numeral parameterised only by consts contains no local type. That is a
general fact with a general consequence nobody has stated: **under route Z a downstream crate can mint its
own numerals and its own capacities over the shared vocabulary and inherit every blanket predicate with no
impl of its own, and under route Y it cannot mint anything at all.** The measurement in section 5 already
exercises the route-Z half incidentally, through the split build. What is owed is the honest converse: a
foreign crate that declares a numeral the tower has never seen, and an attack showing it can extend the
tower's instances while remaining unable to extend its vocabulary. `72:71` and `74:313-317` each named this
compile as owed; it is now the last unchecked half of the fork's own argument.

## 9. Self-check and verification

Every table above was checked line by line against `76_probes/results.csv`, `split_results.csv` and
`deep_results.csv`, re-read at the moment of writing rather than from memory of running them. Every quoted
diagnostic was copied from a run in this session, not paraphrased. Every `file:line` citation into the
panel and into the workspace rules was re-grepped this session: `68:549-556`, `68:703-710`, `74:63`,
`74:110-119`, `74:144-151`, `74:162-171`, `74:196-203`, `74:313-317`, `74b:26-47`, `75:28`, `63:670-679`,
`53:162-171`, `72:71`, `unstable-features.md:74`, `arvo-toolbox-not-policer.md:60`,
`arvo-compile-time-last.md:16`, `74_probes/capacity.rs:8-10`, `arvo-tensor/tests/capacity.rs:14-18`,
`arvo/tests/no_multiplicative_identity.rs:9-24`, all confirmed.

The dispatch's own factual premises were checked rather than assumed. "Nobody has ever compiled a real
consumer against this machinery": confirmed, no prior file's probes contain a consumer-shaped compile, and
`65_probes/probe_9` is the closest and is a shape check with two type aliases. "One of them was recently
found not to call the entry points everyone assumed": that is file 55's finding about the algorithm crates,
and it is the reason section 5's census counts what consumers wrote rather than what the design expects
them to write.

The exit condition in section 1 was written and its thresholds fixed before any arm was generated, and the
consumer-shaped region was fixed at 400 by 400 before the sweep ran; the sweep went to 800 by 800
afterwards specifically so the region's far edge would not be its own boundary.

Two traps the dispatch named, and what happened at each. **Arms forcing different obligations**: I built
`ys` and had it in the results table before attacking it, and section 4 is the correction; the arm stays,
labelled, because the size of the discount a missing guarantee buys is itself information. **A bench
measuring its own methodology**: the first harness wrote its generated source to `mktemp -t`, which lands
outside the repository tree, where `rustc` resolves to stable `1.94.0` and refuses every gate. It produced
an empty results file rather than a wrong one, which is luck rather than design, and the harness now
writes inside the tree and the pin is confirmed in `76_probes/OUTCOMES.md`.

Every design conclusion survives deleting its tree-adjacent citations, checked sentence by sentence
against `70b`'s deletion test. Section 2's finding is that two ratified sentences describe encodings of
different kinds and a compiler refuses their composition. Section 4's is three compiler diagnostics
against three constructions, none of which is shipped. Section 5's census is a count of consumer text,
which is evidence about what to measure and carries no claim about what the design means.
