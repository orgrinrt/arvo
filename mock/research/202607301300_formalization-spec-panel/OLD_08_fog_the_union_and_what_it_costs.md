# Panel 08: the union, and what it costs

**Persona:** Agner Fog, measurement lens. Eighth member; read `01_knuth_mathematical_rigour.md`,
`02_kiselyov_type_level_encoding.md`, `03_jhala_what_is_provable.md`,
`04_torvalds_does_it_earn_its_keep.md`, `04b_op_checkpoint_and_directions.md`,
`05_leijen_fallibility_without_poisoning.md`, `06_muratori_the_consumer_surface.md`,
`06b_op_checkpoint_two.md` and `07_spj_is_the_type_story_sound.md` in full, plus every probe under
`02_probes/`, `05_probes/`, `06_probes/` and `07_probes/`, before starting.
**Date:** 2026-07-30

**What I read in full:** the spec (`202607301200_topic.the-formalization-spec.md`), all nine panel
files, the four probe READMEs, `02_probes/c_computed.rs` and `c4_diag.rs`,
`05_probes/{a_handler,b_carrier_join,c_layout,d_delivery_codegen,e_refusing_through_graph,f_const_falsification}.rs`
by header, `06_probes/{c_nominal_and_modifier,d_verdict_names_the_composition}.rs` line by line,
`07_probes/{a_witness_typestate,b_bounds_collapse}.rs` line by line, the panel brief, and
`panels-argue-the-intent-not-the-wording.md` and `bench-and-sketch-discipline.md`. **What I read in
part:** `arvo-strategy/src/{lib,container,identity,arith_impls}.rs`, `arvo/src/lib.rs`,
`~/.cargo/config.toml`.

**Directory listing done** across `mock/design_rounds/`, `mock/research/`,
`mock/research/sketches/`, `mock/crates/`, `mock/benches/variants/` and the panel directory.
Nothing supersedes the spec. The bench harness has four variants (`fnv1a`, `xxhash3`,
`spectral-bisection`, `structural-decomposition`) and none for compile cost, which is where 02
section 9 and 04 section 6 both said the measurement belonged.

**Gates.** I re-ran the suite: 654 passed, 0 failed, 9 ignored, matching every prior member who ran
it. I did not re-audit test bodies the first six members read in their own hands. One observation
that falls in my lens and belongs in the record: **arvo has no compile-time test of any kind.** Nine
`tests/ui/` fixtures pin what the compiler *says*; nothing pins what compilation *costs*, and this
spec proposes to change compilation cost substantially.

**Separation of evidence.** Sections marked *measured* carry numbers I produced on this machine
(Apple M1, 8 cores, `nightly-2026-05-28`) with the method stated. Sections marked *compiled* were
built or deliberately failed under the same pin, from nine artifacts committed at `08_probes/`.
Sections marked *reasoned* are argument. I carry more than one reading wherever the evidence does not
force one, and I rule on nothing.

**One correction to my own work, up front, because it is the same class of error this panel keeps
finding in others.** My first five baseline readings were 15 to 27 seconds against 04's 6.5, and I
had a paragraph drafted correcting 04. They were wrong. Setting `RUSTFLAGS="-Ztime-passes"` changes
the fingerprint and forces a full dependency rebuild, so I was timing a cold build and calling it an
incremental one. Controlled, 04's number is right. Section 7 has the corrected series. I state this
rather than quietly fixing it, because the harness audit I owe the panel is one I first failed
myself.

---

## 0. What I was asked, and what came back

07 proposed that a union probe replace the spec's first sketch obligation, on the grounds that the
panel holds five proposals that each compiled alone and have never been compiled together. I built
it. It compiles, it runs, and it refuses the right compositions.

It also broke in four places, three of which are invisible to any single proposal and one of which
retracts a claim two members endorsed. That is the value of the exercise and it is worth stating as
the headline rather than the caveat: **a proposal that only works alone is a finding, and three of
the five are in that category to some degree.**

The second half of the dispatch was cost, which nine files had left unmeasured. The short version is
that the design is cheap in the places everyone worried about and expensive in exactly one place
nobody named. Sections 7 through 11.

## 1. The union compiles, and here is what it is

`08_probes/a_union.rs`, 63 impl blocks, one crate, no forbidden features
(`#![feature(const_trait_impl)]` only, WATCH-allowed per `unstable-features.md`). It carries all five:

01's translation-stability primitive replacing `Faithful`, as a `[const]` trait method with a
generic `const fn` checker. 02's computed-truth law fold with 07's witness making the check the
typestate. 05's delivery separated from refusing and placed on `Lowering`. 06's nominal constructors
at every consumer-selected position plus four delegating modifiers. 07's graded fallibility with the
declared `Fallibility` projection deleted and every arithmetic body collapsed to one bound. Plus 02
section 5's split parameters, since 04 and 05 both endorsed them.

Five compositions fold and five refuse, the arithmetic runs across three deliveries, and the
classification table the const checker computes is 01's table exactly:

```
ReduceModulo     stable1=true  stable2=true  refuses=false
TowardNegative   stable1=true  stable2=false refuses=false
TowardPositive   stable1=true  stable2=false refuses=false
SubstituteZero   stable1=false stable2=false refuses=false
Refuse           stable1=true  stable2=false refuses=true
```

That last row is the first defect, and it is not cosmetic.

## 2. 07's witness cannot express `Refuse`, and repairing it closes 01 finding 6 at the leaf. Compiled.

`07_probes/a_witness_typestate.rs:37-39` declares the recovery map as

```rust
pub const trait Resolve {
    fn phi(x: i32, min: i32, max: i32) -> i32;
}
```

Total. `Refuse` has no total recovery map; refusing is precisely the absence of a returned value. So
the probe implements `Resolve` for `ReduceModulo`, `Clamp` and `SubstituteZero`
(`a_witness_typestate.rs:85-126`) and for nothing else, and 07's own summary calls the shape "the
answer to Thread C" without noting that **the one resolution the entire fallibility half of this
design rests on sits outside it.** `Precise`'s whole identity is `OverRange = Refuse` (spec:253).

This is also what 01 finding 1 already required and nobody carried forward: 01's own text says the
`Refuse` row "needs a definition the spec does not give: what associativity *means* for a fallible
operation", and answers it with Kleene equality. A total `phi` cannot express Kleene equality
because it has no undefined element.

The repair is small and I compiled it (`a_union.rs:56-95`). Make the map partial:

```rust
pub enum Rec { At(i32), Refused }
pub const trait Resolve { fn phi(x: i32, min: i32, max: i32) -> Rec; }
```

and the stability check compares under Kleene equality (both refuse, or both return and agree). All
five constructors now classify, and the two `Refuse` rows in the table above are 01's hand analysis
reproduced mechanically: stable one-sided (unsigned, by monotonicity), not stable two-sided (01's
`(127 + 1) + (-1)` counterexample).

**And the repair pays a dividend nobody has claimed.** With `phi` partial, "does this rule ever
refuse" is computable from the same definition (`a_union.rs:99-110`):

```rust
pub const fn ever_refuses<R: [const] Resolve>(min: i32, max: i32) -> bool
```

so the *grade* is witnessed alongside the stability markers. 01 finding 6 says `Fallibility` is
asserted where the spec's own D16 demands derivation; 07 section 2 closes that at the fold, by making
the carrier a projection of the computed join. This closes it at the **leaf**: a constructor cannot
declare `Refuses = False` while its own recovery map refuses. Compiled and confirmed to bite in
section 6.

Two readings on how far this generalises. Either every classification a resolution declares should be
witnessed against the one semantic definition, which is a clean rule and is what I built. Or the
grade is different in kind from stability, because stability is a mathematical property of a map
while fallibility is a statement about a signature, and witnessing it conflates the two. I lean to
the first because in this design they are both projections of the same `phi`, but the second is a
real position and the distinction matters if a future resolution refuses for a reason a bounded
domain scan cannot exhibit.

## 3. 02's parameter split and 06's diagnostic result are in tension, and the split loses. Measured.

06 section 4 makes the strongest ergonomic claim in the panel: "ten axes are free in the error
surface, and the spec's diagnostic problem is entirely an artifact of one implementation choice".
Its evidence is `06_probes/c_nominal_and_modifier.rs`, which renders
`Number<Fix<13, 3, Signed>, LayoutOf<OverRangeOf<Warm, Refuse>, Bitpacked>>` complete and
untruncated.

That probe uses a **fused** strategy parameter, `Number<N, S>` (`c_nominal_and_modifier.rs:131-134`).
02 section 5, endorsed by 04 section 8 ("Cheap, no consumer-visible cost, do it") and by 05 section
10, proposes splitting it into `Number<N, P, L>`. Under the split, the same preset expression appears
**twice** in every rendered type, because one alias fills both positions.

I measured it with a controlled variant: `08_probes/d_fused_parameter_control.rs` is
`a_union.rs` mechanically fused back to two parameters, same axes, same derivation, same
`Proves<C>`. Rendered length of the composition in the `Proves<...>` help line, and whether rustc
truncated:

| consumer case | fused | split | fused render | split render |
|---|---|---|---|---|
| `IFixed<13, 3, Warm>` | 47 chars | 74 chars | complete | complete |
| `UFixed<13, 3, OverRangeOf<Warm, SubstituteZero>>` | 92 chars | 169 chars | complete | **truncated, long-type file** |
| `IFixed<13, 3, LayoutOf<OverRangeOf<Warm, Refuse>, Bitpacked>>` | 117 chars | 214 chars | truncated | truncated |

The split costs about 1.8x in rendered length and moves the truncation boundary down by one modifier
level. Under the union, three of four deliberately-wrong consumer cases spill to a long-type file
(`08_probes/f_error_surface.rs`). Under fusion, two of three render whole.

So 06's claim needs a boundary attached: **ten axes are free in the error surface for a bare preset,
and not for a modified one under split parameters.** 06 measured with fusion; the panel then endorsed
a split that costs exactly the thing 06 had just established was recoverable.

## 4. And the split does not deliver what it was endorsed for. Compiled.

This is the retraction. 02 section 5's argument is not primarily about parameters; it is about what
the type system enforces. Its words: "Every law impl then mentions `N` and `P` and cannot mention
`L`, which makes the spec's own sentence a typing fact rather than a review note." 05 section 10
built on it: "With `Number<N, P, L>` the impl header cannot mention `L` and the invariant is typed.
With the fused parameter it is prose."

That is false, and `08_probes/c_split_does_not_bind.rs` is eleven lines proving it:

```rust
impl<N: Numeral, P: Policy, L: Lowering> AddAssocIllegal for Number<N, P, L>
where
    L::Layout: IsDense,
{
}
```

Compiles clean. `L` is a parameter of the impl, so of course a law can name it. The split makes the
violation *visible* in the impl header, which is worth something to a reviewer, and it makes it no
less writable. That is the same review discipline 02 said the split would replace, at a measured
cost of 1.8x rendered type length and one modifier level of legibility.

Two readings, and I hold them at different strengths. The one I lean to: what actually types the
invariant is a sealed input trait, `trait LawInput` implemented only for the `(N, P)` pair, so a law
impl is written against something that structurally has no lowering member to reach. That was 02's
own second option, which it named and dismissed as "still discipline"; it is not, because a sealed
trait with no `L` in scope cannot mention `L`. The weaker reading: the visual salience is genuinely
worth 1.8x, since a reviewer scanning impl headers is the mechanism that actually catches this today
and the diagnostic cost only lands when a consumer is already in a failure. I do not find that
persuasive, because the failure is exactly when legibility matters, but it is arguable.

Either way the panel should stop citing the split as a typing guarantee. It is a naming convention
with a measured cost.

## 5. 05's delivery result has a precondition nobody named, and it costs 8x when it fails. Measured.

05 section 5 measures that a fallible return doubles every intermediate, and section 6 measures that
the absorbing-bottom delivery emits a branchless ten-instruction loop body against the sum type's
eleven with two exits. Both are correct as measured. Both are conditional on something neither the
probe nor the spec's axis set records.

07 section 2's graded aggregate makes the carrier `<JoinOf<Q> as CarrierOf>::C<T>`, a function of the
grade and generic in `T` (`07_probes/b_bounds_collapse.rs:62-70, 159`). The bottom's saving depends
on the **numeral** having a spare bit pattern, which `CarrierOf` cannot see. Built the honest
composition and measured it (`08_probes/a_union.rs`, `arith` binary):

```
size_of  sat=2  precise=4  preciseBot=4
```

The bottom delivery costs exactly what the sum type costs, because with no spare pattern the bottom
needs a companion flag. And the codegen is worse than either, `08_probes/e_codegen.rs`,
`-C opt-level=3`, aarch64, loop body only:

| shape | instruction lines | branches |
|---|---|---|
| bare saturating baseline | 8 | 1 (back edge) |
| union, `Warm`, total carrier | 9 | 1 |
| union, `Precise`, sum-type delivery | 8 | 3 (two data-dependent exits) |
| union, `Precise`, bottom in a companion flag | **87** | **6** |
| union, `Precise`, bottom in the numeral's spare pattern | 10 | 1 |

The last row reproduces 05's probe D almost exactly (05: "ten-instruction loop body, no branch but
the back edge", two over the saturating baseline). The fourth row is the same delivery with its
precondition unmet, and LLVM did not vectorise it (zero SIMD registers in the body, checked).

So the honest statement of 05's finding is stronger and narrower than 05 gave it: **the absorbing
bottom is the cheapest delivery where the numeral has a spare pattern and the most expensive of the
four where it does not**, by 8 instructions per element and a factor of two in bytes. 05 noticed the
structural alignment ("`Precise` is the only preset whose out-of-range resolution is `Refuse`, and it
is one of the two whose `StoredWidth` is `DoubleLogical`") and filed it as a hypothesis it might be
"a coincidence of this particular preset table". It is not a coincidence and it is not a preset fact:
it is a precondition on the delivery axis.

I built the carrier that reads it (`08_probes/b_spare_pattern_decides_delivery.rs`), keyed on
`(L::StoredWidth, L::Layout)`, and it resolves:

```
Precise (DoubleLogical, spare exists): Answer<u16> = 2 bytes
Hot (Minimum, no spare):               Answer<u16> = 4 bytes
```

The consequence for the sorting question 05 opened and 07 sharpened: the carrier now reads **three**
`Lowering` members, and it is the return type of every arithmetic operation. So the spec's line 54,
"What it costs to hold and to compute. Changes no answer", is true of the value and false of the
type. 07 named this as "a third sort D54 never asks about, did the type of the interaction change".
The measurement says that third question is not optional, because getting it wrong is an 8x codegen
difference in a hot loop, which is exactly the cost `arvo-compile-time-last.md` does not sanction.

## 6. Three smaller composition results, all compiled.

**Any table keyed on a preset name breaks under 06's modifiers.** My first spare-pattern table was
keyed on the lowering type (`impl SpareRule for (Fix<I,F,S>, Warm)`), which is how a person naturally
writes a per-preset fact. `DeliveredAs<Precise, AsBottom>` has no impl, even though it delegates
every member, and the error is `the trait bound (Fix<13,3,Unsigned>, DeliveredAs<Precise, AsBottom>):
SpareRule is not satisfied`. Modifiers are transparent to **projections** and opaque to **impls keyed
on the type**. The repair is to key every derived fact on projected members and never on a preset
name, which worked. That is a general rule the modifier proposal needs and does not state, and it is
the sort of thing that would be discovered one crate too late.

**The door check survives the union and names the wrong thing.** I reproduced 07's worst case inside
the union: flipped `Refuse`'s declared grade to `False`, added 07 probe a6's disarming
`const WITNESS: () = ();`, and deleted the eager forcing const per a3. The door caught it:

```
error[E0080]: evaluation panicked: this resolution's declared classification
              disagrees with its own recovery map
  evaluation of `<Number<Fix<13, 3, Unsigned>, Precise, Precise> as Arith>::over::<u16>::{constant#0}`
```

Three errors for three compositions using the one lying rule, and **none of them names `Refuse`**. In
07's standalone probe the door was `resolve::<R>` and the error named `SubstituteZero`; in the union
the door is a method of the aggregate, so it names the composition. 07's two-site discipline is
therefore genuinely two-site rather than belt-and-braces: the eager per-constructor const is the only
site that names the constructor, and with N compositions the door alone gives N errors pointing at
the wrong thing.

**What composed without incident**, so the panel does not re-litigate it: the law derivation resolves
through modifiers, stacked two deep; `Proves<C>` fires with its remediation note intact under the
union's real bound chain; the graded aggregate collapses every arithmetic body to a single
`C: Arith` bound even with the delivery on `Lowering`; and the law impl omitting the `L` bound is
genuinely L-independent, so 02's *intent* holds even though its mechanism does not.

## 7. The baseline, corrected, and a harness defect that affects every number in this panel. Measured.

04 section 6 gives the panel its only cost anchor: "A `touch` of `arvo-strategy/src/lib.rs` recompiles
twenty crates; `cargo check --workspace` completes in 6.5 seconds."

Twenty crates is right. 6.5 seconds is right, and I nearly published a correction saying it was not,
for the reason in this file's header.

But the number is measured through a proxy nobody controlled for. `~/.cargo/config.toml` sets
`rustc-wrapper = "sccache"` globally, so every `cargo check` in this workspace runs through sccache,
and sccache's server is currently in a broken state on this machine: `sccache -s` returns
`Failed to send data to or receive data from server. Mismatch of client/server versions?`, and one of
my builds died mid-flight with `sccache: encountered fatal error: failed to zip up compiler outputs`.

Controlled, five runs each, L0 touch, `cargo check --workspace`:

| configuration | steady-state |
|---|---|
| wrapper on, as everyone measured | 5.81, 5.88 seconds |
| wrapper off (`RUSTC_WRAPPER=""`) | 5.16, 5.25 seconds |

The wrapper costs about 12% on an incremental source touch, which is the operation it cannot help
with, because a touched source file is a cache miss by construction. That is a finding about the
maintainer's edit-compile loop independent of anything in this spec, and it is worth an afternoon
regardless of what the round decides.

**A separate correction to 04's method, not its conclusion.** 04 reports "roughly one hundred impls
across `arith_impls.rs`, `container.rs` and `identity.rs` (grep counts: 37, 44, 25)". Those three
numbers are reproduced exactly by `grep -c "impl"`, which counts any line containing the substring,
including doc comments and prose. Counting impl-block openers
(`grep -cE "^\s*(pub )?(const )?impl\b"`) gives 0, 11 and 4 for those three files: `arith_impls.rs`
contains **zero** impl blocks and 32 macro invocations. The crate has 83 hand-written impl blocks and
135 macro invocations across all files. The order of magnitude survives; the method should not be
cited, and a bench that quotes "before: 100 impls" against a post-expansion "after" would be
comparing two different things.

## 8. Where the time actually goes. Measured.

`-Ztime-passes`, isolated per crate, touching only that crate:

| crate | total | `type_check_crate` | `coherence_checking` | `MIR_borrow_checking` |
|---|---|---|---|---|
| `arvo-strategy` | 2.364s | 1.090s | 0.604s | 0.571s |
| `arvo` (facade) | 2.600s | 1.844s | 1.527s | not in top five |

Coherence checking is 59% of the facade's own compile and about a quarter of `arvo-strategy`'s. It is
the single largest identifiable pass in the facade, and the facade has **91 impl blocks**. So arvo's
coherence cost is not driven by impl count; it is driven by the difficulty of each overlap check,
which is where const-generic bounds, the `Project<TAG, Sign, BYTES, S>` dispatch and the const-trait
machinery all land.

That matters because coherence is the pass the spec's additions land in: blanket law impls over
multi-axis tuples, per-width and per-exponent macro-expanded tables, ten axes composed. Section 9
prices the table shape; section 10 prices the instantiation.

**A hypothesis I tested and could not support, stated because a later member should not assume it.**
I expected the `generic_const_exprs` drift that 02, 03, 04, 06 and 07 all flag on rule-compliance
grounds to also be the dominant coherence cost, which would make its removal a compile-time win
rather than a compliance cost. I built a synthetic reproducing the shipped `Picker:
OneRepresentable<{ tag_one(I) }>` pattern against the equivalent associated-const projection, swept
32 to 256 sites, and the difference was in the noise. My synthetic does not reproduce arvo's actual
bound structure faithfully enough to settle it. The experiment that would: `cargo install
measureme-tools`, run `-Zself-profile` on the facade, and read the per-query breakdown. Nobody has
done that and it is twenty minutes.

## 9. The table-versus-projection asymptotics, priced. Measured.

02 section 9 argued that the two encodings differ asymptotically rather than by a constant, and that
benching whichever gets written first "measures a choice rather than informing it". It is right about
the shape. `08_probes/h_gen_table_vs_projection.py` sweeps both, with a consumer site per row so the
solver actually runs:

| rows | table, coherence | projection, coherence | table, total | projection, total |
|---|---|---|---|---|
| 32 | 0.009s | 0.005s | 0.090s | 0.070s |
| 64 | 0.010s | 0.003s | 0.079s | 0.040s |
| 128 | 0.015s | 0.004s | 0.093s | 0.049s |
| 256 | 0.032s | 0.007s | 0.117s | 0.124s |
| 512 | 0.129s | 0.012s | 0.240s | 0.132s |

The table's coherence is quadratic (256 to 512 doubles the rows and quadruples the time); the
projection's is flat. At 512 rows the table costs 10.75x the projection in coherence and 1.8x in
total.

The practical reading cuts the other way from the asymptotic one, and both should be carried.
Asymptotically 02 is right and the projection is the correct shape. Practically, 0.24 seconds at 512
rows is nothing next to the facade's 2.6, so **the per-width table the spec proposes at D73 is not
where arvo's compile time is going to go**, and a round that adopts the projection encoding for
compile-cost reasons is adopting it for the wrong reason. The right reasons are 02's own (uniform,
gate-free, linear in derivations) and 06 section 7's (an associated const in a const-fn body computes
the significand with no type position at all, which I confirmed independently in the union at
`a_union.rs:399-406`).

## 10. What a composition costs, and what it costs at runtime. Measured.

`08_probes/i_gen_monomorphisation_sweep.py`, K distinct compositions each calling the union's `add`,
release build, symbol count from `nm -U`:

| distinct compositions | `monomorphization_collector_graph_walk` | total | symbols in the binary |
|---|---|---|---|
| 1 | 0.019s | 0.885s | 747 |
| 10 | 0.064s | 0.254s | 747 |
| 40 | 0.195s | 0.477s | 747 |
| 100 | 0.554s | 1.087s | 747 |
| 200 | 1.055s | 1.748s | 747 |
| 400 | 2.097s | 3.347s | 747 |

Linear, about **5.2 milliseconds of compile time per distinct composition**, and **zero symbols**.
The count does not move by one across a 400x range, because every instantiation inlines away
completely. Ten axes, four modifiers, three deliveries, the witness, the graded carrier and the
computed law derivation all cost exactly nothing in the shipped binary.

Set that against the emitted code in section 5: the union's `Warm` accumulation is **one instruction
per element over a hand-written saturating baseline**. One. For a type carrying ten axes, a witnessed
classification, a computed law and a graded carrier.

That is the answer to whether this is the trade `arvo-compile-time-last.md` sanctions, and for
everything except the delivery axis the answer is yes, emphatically. The rule's own words are that
compile time "is the bucket we pour into, freely, when doing so buys runtime or correctness". Five
milliseconds per composition and 12% of a second per hundred is pouring into that bucket at a rate
the rule explicitly licenses.

The one thing that does cost runtime is the delivery choice, and it costs it only when its
precondition fails (section 5). That is not a compile-time trade at all; it is a design axis whose
bad configuration is 8x on a hot loop, and it should be gated by the type rather than left to a
preset table.

**What a real bench is owed, named rather than approximated.** The instruction sequences above are
the artifact and I take no timing from them. The throughput consequence of one branchless instruction
against two data-dependent loop exits, over the four deliveries, on a realistic accumulation, is a
bench. It belongs at `mock/benches/variants/delivery-shapes/` under the harness per
`bench-and-sketch-discipline.md`, with one cdylib per delivery so no cross-variant inlining
contaminates it, and it can be written before `arvo-policy` exists. A second bench is owed on the
encoding question of section 9 if anyone wants the practical reading confirmed at arvo's real width
range rather than my synthetic's.

## 11. The witness at the composition's actual width is not available. Measured.

07 section 1.4 leaves one question explicitly open and explicitly unpriced: "since the door knows the
concrete composition, it could run the check at the composition's **actual** width whenever the span
is small enough for the const-eval budget... I did not measure it and say so."

Measured. The stability check is quadratic in the span, so it is O(4^N) in the width. Build time for
five constructors, one check each:

| width | max value | build |
|---|---|---|
| 3 bits | 7 | 0.53s |
| 5 bits | 31 | 0.84s |
| 6 bits | 63 | 2.26s |
| 7 bits | 127 | 8.65s |
| 8 bits | 255 | 28.45s |
| 9 bits | 511 | refused |

Four times per bit, exactly as the complexity predicts, and at 9 bits rustc stops it:

```
error: constant evaluation is taking a long time
  = note: `#[deny(long_running_const_eval)]` on by default
```

arvo's widths run to 128 natively and past 256 through `WideBits`. So the actual-width witness has a
hard ceiling at 8 bits, and at 7 bits it already costs more than the entire arvo workspace check
(section 7). It should be written into the round as unavailable rather than as an option to consider,
because it is the sort of thing a later reader will try.

The fixed representative-width form is free by comparison. The whole union crate, five witnessed
constructors, ten axes, four presets, four modifiers, three deliveries, the aggregate and the law,
builds in **0.15 to 0.21 seconds**, with `coherence_checking` at 0.004s and `type_check_crate` at
0.005s. So 07's two-site discipline at small widths costs nothing measurable, and 03's
width-uniformity argument remains the thing that carries the generalisation, exactly as 03 said and
for a reason 03 could not have known: there is no width at which brute force replaces it.

Two readings on what to do with the ceiling. Either the representative widths are fixed at 3 and 4
bits and the uniformity argument is written next to them in prose, which is cheap and honest and is
what I built. Or the check is restructured to sample rather than exhaust, which restores larger widths
at the cost of the property that makes it worth having, since a sampled check over the widths where
the law already holds is precisely the failure mode
`a-test-that-cannot-compile-is-the-finding.md` names. I lean hard to the first and think the second
should be named and rejected in the round so nobody proposes it later.

## 12. Engagement with the prior seven, kept short

**01's translation-stability primitive.** It is the one proposal in the union that composed with
everything without a single adjustment, and it absorbed the `Refuse` extension for free (section 2).
I would say plainly what the panel has been implying: 01 finding 3 is the load-bearing repair in this
whole review, because the witness mechanism, the grade computation and the law fold are all
instantiations of it, and none of them has a form until "faithful" is replaced by something a `const
fn` can compute.

**02's computed-truth encoding.** Survives the union intact. Its section 5 parameter split does not
(sections 3 and 4). I note the asymmetry because 02's file is strong and the one part of it the panel
adopted most readily is the part that does not hold up.

**03's bounded falsification.** Fully vindicated and now priced. Its own second reading, that the
oracle is a second place to be wrong, was answered by 07 (the oracle becomes the shipping map) and my
section 2 extends that to the grade. Its section 5 notko-absence finding: 07 says the graded shape
removes the dependency structurally, and I confirm it in the union, since no body anywhere names
`ConstFromResidual`. The pin 03 and 04 asked for is still worth writing, for any other surface that
leans on the absence.

**04's economics.** Its baseline is correct and I nearly said otherwise; its impl-count method is not
(section 7). Its section 6 impression about three-in-the-morning debugging is the one thing I would
now downgrade: 06 probe D discharged the four-frames-deep worry and my union confirms the error is
reported at the outermost concrete instantiation. What replaces that worry is smaller and more
specific, which is that the door check names compositions rather than the rule that lied (section 6).

**05's delivery reframe.** The claim survives, the measurement behind it does not survive
generalisation, and the difference matters (section 5). I would put its section 6 `ConstantTime`
finding higher than it put it: my numbers say a refusing composition under the sum-type delivery has
two data-dependent exits per element and one under the bottom delivery has none, so the property
genuinely inverts on an axis the ten do not contain.

**06's nominal constructors.** The strongest ergonomic result in the panel and it needs the boundary
in section 3 attached. Its section 9 domain-newtype finding is outside my lens; op's `06b` correction
governs it and I have nothing to add except that the twenty escapes are still a measurement of
something, whatever it is a measurement of.

**07's union proposal.** Correct call, and the reason is now on the record with numbers: three of the
five proposals broke on contact with each other, and every break was invisible from inside the
proposal that caused it.

## 13. What I did not get to

The `-Zself-profile` per-query breakdown of the facade's 1.5 seconds of coherence, which would settle
section 8's unsupported hypothesis. Twenty minutes with `measureme-tools` installed.

Whether the spare-pattern precondition survives `Cold`'s bitpacked layout, which 05 also flagged as
unfinished. My table asserts `(Minimum, Bitpacked) -> no spare`, which is right for a field exactly as
wide as its logical width and wrong the moment a bitpacked field is padded. That row wants deriving
rather than declaring, and by section 2's own rule it wants a witness.

The four `.stderr` fixtures of `08_probes/f_error_surface.rs` as committed `tests/ui/` cases, which is
what 04 section 2 asked for and what would turn the truncation measurements of section 3 into a gate
rather than a paragraph. Half a day, and it is the artifact that decides the fused-versus-split
question by evidence rather than by my table.

Whether the graded aggregate's `where` clause is re-checked per obligation at a cost that grows with
the surface, which 07 flagged against its own proposal. My union has one arithmetic function; arvo
will have dozens.

And the delivery bench named in section 10, which is the only remaining timing claim in this panel
that nobody is entitled to make yet.

---

**Summary for the next member.** The union compiles and all five proposals are in it, with four
defects, three of them invisible from inside any single proposal. 07's witness mechanism cannot
express `Refuse` at all, because its recovery map is total and refusing is the absence of a returned
value; making the map partial admits it, reproduces 01's whole classification table mechanically
including both `Refuse` rows, and closes 01 finding 6 at the leaf by making the fallibility grade a
projection of the same `phi` (section 2, compiled). 02 section 5's parameter split conflicts with 06
section 4's diagnostic result and loses on measurement, costing 1.8x rendered type length and one
modifier level of legibility against a controlled fused variant, so 06's "ten axes are free in the
error surface" holds for a bare preset and not for a modified one (section 3, measured); and the
split does not deliver the typing guarantee it was endorsed for, since a law conditioned on
`L::Layout` compiles clean under it (section 4, compiled, eleven lines). 05's delivery result has an
unnamed precondition the graded aggregate structurally cannot see: with the numeral's spare pattern
the bottom is 2 bytes and 10 branchless instructions, without it 4 bytes and 87 instructions with six
branches, so the delivery axis is conditional on two `Lowering` members and its bad configuration is
the only thing in this design that costs runtime (section 5, measured). Any derived fact keyed on a
preset name breaks under 06's modifiers, and the door check names compositions rather than the rule
that lied, which is why 07's two-site discipline is genuinely two-site (section 6). On cost: 04's 6.5
second baseline is right and my first five readings contradicting it were my own harness error;
sccache is a global rustc wrapper on this machine, is in a broken server state, and costs 12% on
exactly the incremental touch it cannot cache (section 7). Coherence is 59% of the facade's compile
at 91 impl blocks, so it is difficulty per check and not impl count (section 8). The per-width table
is quadratic and the projection flat, but both are negligible at arvo's range, so the encoding should
be chosen for 02's and 06's reasons and not for compile time (section 9). A distinct composition
costs about 5.2ms of compile time and **zero symbols**, and the union's arithmetic is one instruction
per element over a hand-written saturating baseline, which is the trade `arvo-compile-time-last.md`
exists to license (section 10). And 07's open question is answered: the witness at the composition's
actual width quadruples per bit, costs 28 seconds at 8 bits, and is refused by
`#[deny(long_running_const_eval)]` at 9, so it is unavailable at arvo's widths and should be recorded
as unavailable rather than as an option (section 11). I rule on nothing; op decides.
