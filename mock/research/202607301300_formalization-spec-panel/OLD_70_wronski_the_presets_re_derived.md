# 70. The four presets, re-derived against the current axis set, for fixed-point and float

Bart Wronski, file 70. I wrote file 29, the quantisation contract, forty files and two full
consolidations ago. I do not carry that file's conclusions forward unread; I re-checked the two of its
claims that this dispatch leans on (the `Resolution`/`Direction` vocabulary and `Fallibility<T>`'s
position) against the current tree of files rather than against my own memory of writing it.

**What I read.** `68_consolidation_seven.md` in full, `68b_op_checkpoint_sixteen.md` in full,
`69_ringer_the_source_justification_sweep.md` in full, all three required. An `ls` of the panel
directory before starting: files `00` through `69`, sixty-nine numbered deliverables plus checkpoints
and probe directories, nothing landed after `69`. `mock/design_rounds/202607301100_topic.the-
formalization-talk.md` lines 1600 to 1780 for D69 through D73, which is where the governing D71 table
and the intent sentence live; I read this directly rather than through any panel file's paraphrase of
it, because a paraphrase of a design decision is exactly the kind of secondary source this dispatch's
method warns against trusting. Targeted reads at the coordinates my own derivation needed: `35_dolan_
does_widening_collapse.md` in full (the compiled removal of `Widening` and `Growth`), `59_fog_the_
lowering_door.md` in full (the exhibit itself, read as a source of compiled mechanism rather than of
justification, per the paragraph below), `11_current_shape_draft.md` lines 190 to 235 (the `Resolution`
/`Direction` vocabulary's declaration), `63_consolidation_six.md` lines 296 to 322 and `68`'s own lines
368 to 375 (`Underflow` and `Specials`' current standing), and `01_knuth_mathematical_rigour.md` /
`08_fog_the_union_and_what_it_costs.md` for `Fallibility<T>`'s declared shape, which no later file
disturbs.

**Gates.** Canon gate: reproduced fresh from the repo root, `grep -rln "Adjustment\|Bias\|Numeral"
mock/crates/ --include="*.rs"` and the same with `FullRange\|UTerm\|AddWidth`, both exit 1, empty.
Test gate: `cargo test --offline --workspace` from `mock/`, summed per binary, 658 passed, 0 failed,
9 ignored, matching every consolidation since file 65. Toolchain `rustc 1.98.0-nightly (57d06900f
2026-05-27)`, `aarch64-apple-darwin`, resolved from `rust-toolchain.toml`, confirmed by running
`rustc --version` in this session rather than trusting the citation chain.

**What is compiled, what is reasoned, what is a proposal.** Section 6 is compiled: three probes in
`70_probes/`, each outcome reproduced in `70_probes/OUTCOMES.md`, run fresh against the pinned
toolchain in this session. Everything else is reasoning from op's stated intent, from the review's
own settled shapes, and from what a real hardware FPU and a real software quantiser actually do,
marked as such at each step. Where a cell has no honest derivation, I say so and leave it open rather
than filling it plausibly; the review's own pre-D71 table already established the convention I use for
that, a literal `?`.

## 0. The method this dispatch owes, stated before using it

Op's correction and file 69's sweep both bear directly on this dispatch, because the artefact I am
replacing is the artefact the correction named. I adopt file 69's proposed test explicitly, at every
citation to the shipped tree below: **does the row's justification survive if the citation is
deleted and only the design's stated intent remains?** Two consequences follow.

First, `59_fog_the_lowering_door.md`'s section 2.3 table (the "shipped meaning" column, the doc-comment
quotes, the sentence "every row below is derived from what the preset already means for fixed-point
arithmetic in the shipped tree") is void. I do not cite it, quote it, or treat its four rows as a
starting point to adjust. I re-derive from op's intent statements alone.

Second, that file's sections 1, 2.1, 2.2, 2.4 and 2.5 are not void, and I use them. They compile a
mechanism (`LoweringDoor`, `Quantised`, `HostFloat<E: FloatEnv>`, `DefaultLowering<N>: Strategy`,
`HostImplemented`, the refusal-not-fallback posture, the `RANK`-based mixed-strategy resolution) whose
correctness does not depend on what any doc comment says a marker means. Deleting file 59's tree
citations at those sections (`arvo-strategy/src/container.rs:104-112` for the refusal precedent,
`arvo-strategy/src/lib.rs:104-107` for `RANK`'s existence) leaves the derivations standing: E0119's
coherence refusal, E0658's specialisation refusal, E0207's unconstrained-parameter refusal, and the
compiled probes behind them are facts about what the type system does, not claims about what a
comment says a marker is for. This is file 69's own worked distinction (`69:161-174`, quoting `33_lamport_the_laws_restated.md:186-198`, the Lamport
`TotalOrd` case) applied here rather than restated: a citation that corroborates that a mechanism
already exists is licensed; a citation whose deletion deletes the argument is not. I mark every tree
citation below with which kind it is.

## 1. The axis set as it now stands, and where the two removed rows went

D71 (`202607301100_topic.the-formalization-talk.md:1683-1715`) spread the four presets across six
rows: in-range quantisation, out-of-range, growth, stored width, widening, layout. Two of those six no
longer name an axis. `Growth` left `Policy` (`35_dolan_does_widening_collapse.md` section 2, confirmed
in the ratified table at `68:566`, "Growth removed from Policy: RATIFIED"); `Widening` left `Lowering`
entirely (`35` section 1, confirmed at `68:574`, "Widening removed: RATIFIED"). Neither is a renaming.
File 35 compiled both collapses and showed the content is not relocated, it is absorbed by machinery
that already existed for an unrelated reason:

`Widening`'s three old instances (`None`, `InContainer`, `PerOperation`) decompose into which primitive
an operation calls, what numeral type that primitive returns (ordinary `Numeral` typing, the
multiplicative half's `mul_full` machinery), and that returned numeral's own `StoredWidth`/`Layout`.
Measured at native width and at a multi-limb width where a real cost asymmetry exists in principle, all
three fold to identical codegen against the composite form (`35:109-114`, compiled, `35_probes/probe_1`,
`probe_3`).

`Growth` decomposes further: it was never a fact about one numeral's own `Policy` at all, it is a fact
about a relationship between an operand numeral and a result numeral, which belongs on the operation's
signature, not on a unary slot (`35:56-58`, compiled, `35_probes/probe_2`).

The current axis set, per `68:556-587`, is what a preset can still speak to: `Policy::Quantisation`
(unchanged in mechanism since file 11, `pub const trait Quantisation { type UnderMidpoint: Direction;
type OnMidpoint: Direction; type OverMidpoint: Direction; type OverRange: Resolution; type UnderRange:
Resolution; type Fallibility<T>: notko::ConstTry<Output = T>; }`, `11:212-218`), `Lowering::StoredWidth`,
`Lowering::Layout`, and `Lowering::Door` (new since file 59, `type Door: LoweringDoor`). `Numeral`'s own
axes (`Radix`, `Precision`, `Exponent`, `Domain`, and nested inside `Exponent::Ranged`, `Underflow` and
`Specials`) are not preset axes. A preset parameterises `Number<N: Numeral, S>` at `S`, not at `N`; the
numeral's own identity is a separate, orthogonal choice the consumer makes regardless of which preset
they reach for. This matters for what follows: `Underflow` (`Gradual`/`Abrupt`) and `Specials` (the
four-point product) are not something a preset sets, and I do not try to make them one. They bear on
the float derivation below only through a well-formedness question `Quantisation` asks of them, in
section 5.

So the current preset table has four live rows, not six: `Quantisation`'s in-range direction,
`Quantisation`'s out-of-range resolution (over and under, which need not be symmetric), `StoredWidth`,
`Layout`, plus `Door`, which is new. Five, once `Door` is counted. `Fallibility<T>` is not an
independent choice; section 4 below shows it is a derived consequence of whether `Refuse` appears
anywhere in a preset's `Resolution` pair.

## 2. The intent, and nothing else, as the starting point

Op's statement of each preset's identity, unchanged since D71: "Hot is as fast as possible, Cold
stores as small as possible, Precise is the most precise at the price of both storage and compute,
Warm is the compromise that suits most default cases and behaves intuitively"
(`202607301100_topic.the-formalization-talk.md:1659-1661`).

Two further statements, made this session, that govern the two rows D71 left unresolved for float and
that reopen (per `68b`) what a plain re-derivation would otherwise have assumed still held from the
fixed-point reading.

Warm: "I think we should assume that it'll work the same as writing regular old floats would work. If
that takes the hardware door, then that's truly the intent behind it. The intuition is that it works
and behaves as f32 and f64 etc in rust today without any framework on top of it. For hot and precise
and cold, we explicitly lose that intuition for their intended behavior instead" (`68b:62-67`).

Cold: "It should be something between warm and precise. Cold also tells us it's seldom computed or
used, it's on a cold path. It can take more cost than warm, but shouldn't just be precise in disguise.
That's the intent" (`68b:69-73`).

Nothing below cites a doc comment, a shipped default, or a variable name as evidence for what a preset
should mean. Where I use a fact about the shipped tree, it is a fact about a compiled mechanism's
existence or behaviour, marked `tree-fact`, never a fact about a comment's stated purpose.

## 3. Fixed-point, re-derived

Op's checkpoint reopened only the float table (`68b:55-58`, "`Hot` and `Precise` survive re-derivation
... `Warm` and `Cold` were never established", read against the strategy-door table specifically).
D71's six-row fixed-point table was never flagged void; it is op's own ratified decision
(`202607301100:1683`). But two of its rows no longer name a live axis, and the brief's warning applies
here too: I do not carry D71's surviving rows forward as authority, I re-derive them from the same
intent sentence D71 itself cites, and note where the fresh derivation lands on the same answer.

| | `Hot` | `Cold` | `Warm` | `Precise` |
|---|---|---|---|---|
| in-range direction | `TowardNegative` (all three) | nearest, `ToEven` | nearest, `ToEven` | nearest, `ToEven` |
| `OverRange` / `UnderRange` | `ReduceModulo` / `ReduceModulo` | `TowardNegative` / `TowardPositive` (clamp) | `TowardNegative` / `TowardPositive` (clamp) | `Refuse` / `Refuse` |
| `StoredWidth` | minimum | minimum | doubled | doubled |
| `Layout` | dense | bitpacked | dense | dense |
| `Door` | inert (see below) | inert | inert | inert |

**`Hot` is fastest, so its rounding is the one arithmetic gives for free.** An arithmetic right shift
rounds toward negative infinity, not toward zero; that is what the shift instruction does, and Hot's
whole identity is paying for nothing beyond the operation the hardware already performs. `Reduce
Modulo` at both range ends is the same argument at the boundary: wraparound is the native behaviour of
two's-complement addition, `ℤ/2ⁿℤ`, and asking for anything else costs a branch Hot's intent forbids.
Minimum `StoredWidth`, dense `Layout`: no headroom Hot's intent has any use for. This reproduces D71's
own row, and the reproduction is now grounded entirely in "as fast as possible" rather than in the row
D71 stated it followed "without argument"; I have supplied the argument.

**`Precise` is most precise at the price of both storage and compute, so it refuses rather than
silently discards precision, and it pays whatever the exact result needs.** Nearest-`ToEven` in range
because refusing what is already exactly representable would contradict "most precise" for no reason;
`Refuse` at both range ends because clamping or wrapping is exactly the silent discard "most precise
at the price of ... compute" was written to forbid, and a hardware instruction cannot refuse (an FPU
instruction is unconditional; it always returns some bit pattern, and returning one that is wrong by
construction is the thing Precise's whole identity exists to prevent). Doubled `StoredWidth`, dense
`Layout`: matches D71, and the fresh argument is that Precise pays a widen per operation regardless
(the returned numeral of `mul_full` is sized to the true product), so doubling the resting storage
too is what lets a chain of operations accumulate more than one operation's worth of exactness before
a narrow forces a rounding decision, which is squarely "at the price of ... storage" rather than an
unexplained doubling.

**`Warm` and `Cold` both round nearest and clamp, and the reason is the same reason D71 gave, now
derivable without the row it was attached to.** A type nobody reaches for expecting a crash, and that
is not optimising for raw speed either, has no reason to accept truncation bias; nearest-`ToEven` is
the unbiased choice and clamping is the choice that never corrupts a stored value into something far
from the truth. Where they differ is exactly D71's own three remaining rows, restated fresh: `Warm`'s
doubled `StoredWidth` and dense `Layout` are "the compromise that behaves intuitively", a container
wide enough that a single operation never needs an explicit widen call, matching what a naive
hand-rolled fixed-point type would do. `Cold`'s minimum `StoredWidth` and bitpacked `Layout` are "stores
as small as possible", literally; the compare-and-select nearest-`ToEven` still costs something on
every store, and Cold pays it because "seldom computed" (this session's statement) means the type is
willing to spend at store time to avoid drift accumulating silently in a column that is read far more
often than it is written.

**`Door` is inert for fixed-point, and I show why rather than assert it.** A native integer add
instruction and Hot's own quantiser composition (`mul_full` then `ReduceModulo`) compile to the same
instruction; file 35 measured this directly, zero-cost at native width and at a multi-limb width where
a genuine cost asymmetry exists in principle (`35:109-114`, compiled). There is no fixed-point analogue
of a floating-point control register: an integer ALU has no rounding-mode state to declare or verify,
because `ReduceModulo`, `Clamp`, `ToEven` and the rest are properties of the *composition* the
quantiser builds, not properties of a piece of silicon that could disagree with them. So there is
nothing for a `HostImplemented`-style marker to distinguish for a fixed-point numeral: every preset's
effective door is the software composition, and the optimiser folds it to the native instruction for
free wherever that instruction happens to compute the same thing. Building a `HostInt` door
analogous to `HostFloat<E>` would add a type with no work to do; I recommend against it, as a
suggestion rather than a ruling, on exactly that ground.

## 4. `Fallibility<T>` is not a fifth row, it is a fold

`Quantisation::Fallibility<T>: notko::ConstTry<Output = T>` selects which tier of the fallibility
ladder an operation returns through (`11:218,230`, unchanged since; no later file touches this member).
Whether it can be `Just<T>` (infallible) or must carry a real refusing channel is exactly `01_knuth_
mathematical_rigour.md`'s and `05_leijen_fallibility_without_poisoning.md`'s `CanRefuse` fold: `Refuse`
is a `Resolution` whose `CanRefuse` is true, every other member's is false, and the join across
`OverRange` and `UnderRange` (an `Or`) decides `Fallibility<T>`'s tier (`05:88-105`, compiled). So
`Precise` (`Refuse`/`Refuse`) is fallible by construction, and `Hot`, `Cold` and `Warm` (none of whose
`Resolution`s is `Refuse`) are infallible by the identical construction. This needs no separate
derivation from intent once the four rows above are settled; it is downstream of them, not a fifth
choice.

## 5. Float, the harder half

### 5.1 The mechanism I keep, and what makes it keepable

Section 0 already states the test. The concrete mechanism that survives it:

```rust
pub trait LoweringDoor: Sealed { }
pub struct Quantised;
pub struct HostFloat<E: FloatEnv>(PhantomData<E>);

pub trait Lowering { type Door: LoweringDoor; /* Encoding, StoredWidth, Layout unchanged */ }
pub trait HostImplemented: Numeral { }
pub trait DefaultLowering<N: Numeral>: Strategy { type L: Lowering; }
```

(`59:161-173`, compiled, `59_probes/probe_3c`, no `#![feature(...)]` line anywhere). Three facts decide
a door, and they arrive from three places, which is why no projection keyed on any single one can be
total: whether this target's silicon implements the numeral (a target fact, cfg-gated, Kind 1
structural lowering under the always-optimal-internals rule); which control state the deployment
guarantees (a deployment fact arvo cannot know and must not decide); which door the preset prefers
where both exist (the one fact a preset actually owns). This is a description of a compiled trait
shape, not a claim about what any preset's marker is documented to mean, so it survives section 0's
deletion test intact: delete every tree citation in file 59's sections 1 through 2.2 and the E0119 /
E0658 / E0207 refusals, the sealed-carrier diagnostic, and the probes are still there, still refusing
the same way, for the same reason.

**Refusal, not silent fallback, is a design-wide posture, not a preset-specific one, and I hold every
preset to it below.** `arvo-toolbox-not-policer.md` forbids exactly the failure mode a silent hardware-
to-software fallback would be: "default selections that quietly change semantics... auto-resolve to a
more conservative strategy without flagging" is on its explicit ban list. A door that silently drops
from one instruction to a thirteen-to-seventeen-times-slower software path with no diagnostic is that
failure mode, regardless of which preset it happens to. I do not derive this from any preset's own
doc comment (there is none to derive it from); it is a standing constraint on the whole axis, cited by
rule rather than by any marker's stated meaning, and it binds `Warm` exactly as it binds `Hot`.

### 5.2 `Hot`, confirmed, and its full row completed

Op already settled `Hot`'s door: "as fast as possible" is the hardware door (`68b:56`). The rest of the
row follows the same logic as the fixed-point case, at a numeral instead of an integer, with one
change a real FPU forces.

`Hot`'s in-range direction cannot be the fixed-point row's `TowardNegative`. There is no hardware
floating-point instruction that rounds toward negative infinity by default; the one rounding attribute
every general-purpose FPU implements as its default, and the only one reachable without touching a
control register, is round-to-nearest, ties-to-even (`ToEven`). Reaching for anything else on the fast
path is not free, it needs the control register moved, which trades one cost for the receipt-tracking
cost file 59 already declined to make arvo's problem (file 59 section 2.5). So `Hot`'s in-range row for a float
numeral is `ToEven`, not `TowardNegative`; the "free" direction is a property of the instruction set,
not a universal, and the two number kinds disagree about what it is. `Hot`'s `OverRange`/`UnderRange`:
whatever the host FPU actually delivers past the representable finite range, which section 5.3 derives
is the far-direction reading gated on the numeral's own `Specials`. `StoredWidth`: minimum. `Layout`:
dense. Both match the native bit pattern exactly, no bookkeeping, which is what "no framework on top"
means when Hot says it about itself rather than borrowing Warm's language for it.

*Grounded on: `ratified` (`68b:56`, the door assignment itself), `settled shapes` (`59`'s mechanism,
deletion-test-survives per section 0), reasoned (the rounding-mode argument, the `StoredWidth`/`Layout`
argument).*

### 5.3 `Warm`, and the collision resolved

Op's statement is unambiguous about the door: plain Rust `f32`/`f64`, no framework, and if that is the
hardware door then that is the intent (`68b:62-67`). It collides with D71's own `Warm` row on exactly
the axis D71's fixed-point reading never had to answer: a plain hardware float, out of range, does not
`Clamp` to the finite maximum. It produces the signed infinity. Neither `ReduceModulo` (that is Hot's
wraparound, and floats do not wrap), nor `Clamp` (D71's own fixed-point answer), nor `Refuse` (that is
Precise's answer, and a hardware instruction has no refusing channel to route through) is what a plain
`f32 * f32` actually does when the exact product exceeds the finite range. Op names this collision
directly: "Warm explicitly wants the saturate to infinity, unlike the rest. So this is why we have the
strategies. They mean they SHOULD behave different as per their intent."

**No new `Resolution` constructor is needed, and the closed vocabulary already contains the answer.**
`11_current_shape_draft.md` states the mechanism for `Clamp` without naming it as a separate
constructor: "clamping above the range is simply `TowardNegative`, the same marker used between
neighbours" (`11:195-196`), because at the top of the range there is exactly one representable
neighbour, the finite maximum, located below the exact value, and `TowardNegative` is the direction
that always picks the lower of two candidates. That argument depends on there being exactly one
neighbour. Where the numeral's own `Specials` makes the signed infinity a representable point,
`Specials ∈ {InfOnly, IeeeSpecials}`, there are *two* candidates past the finite maximum: the finite
maximum itself, below the exact value, and the signed infinity, above it. That is an ordinary
two-neighbour rounding situation, resolved by the opposite direction from clamp's: `TowardPositive`
at `OverRange`, `TowardNegative` at `UnderRange`, the far-direction reading rather than the near one.
IEEE 754's own overflow rule is exactly this read as a grid extended one hypothetical step past the
finite maximum, with the far endpoint of that step relabelled infinity, which is the same construction
this review already used for `Abrupt`-underflow's own hole (`68:198-199`, "the value-level reading,
the hole `(0, r^EMIN)`, is the meaning"). I did not invent a new shape for this; I am reusing one the
review already validated at the opposite end of the same grid.

**Compiled, three cases, the whole `Specials` product's relevant split.** `70_probes/probe_1_far_
direction_positive.rs`: a numeral whose `Specials` is `IeeeSpecials` or `InfOnly` accepts the far-
direction reading through a `HasInfinity` marker, `OverflowsToFarPoint<N, TowardPositive>`.
`70_probes/probe_1b_negative_control_nospecials.rs` and `probe_1c_negative_control_nanonly.rs`: a
numeral whose `Specials` is `NoSpecials` or `NanOnly` refuses the identical bound, `E0277`, "the trait
bound `NoSpecials: HasInfinity` is not satisfied" and the `NanOnly` mirror, at the exact call site.
All four members of the product are exercised, not a sample of two. This establishes the bound is
expressible under the permitted feature set and refuses exactly where it should; it does not by itself
settle that `Quantisation` should carry this obligation in exactly this shape, which is a design call
for the review, offered per the standing instruction to suggest rather than legislate.

**This makes `Warm`'s `OverRange`/`UnderRange` well-formed only for a numeral whose `Specials` includes
the relevant infinity, and that is a fact worth stating plainly rather than smoothing over.** A `Warm`
numeral declared with `Specials = NoSpecials` or `NanOnly` has nowhere for an out-of-range value to go
under "behaves like plain f32/f64", because plain f32/f64 always has somewhere for it to go. Two
honest readings, and I do not pick between them: either `Warm`'s default numeral shape is constrained
to carry the relevant infinity (which every IEEE binary format already does, so this constrains only
exotic Warm numerals nobody would reach for anyway), or a `Warm` numeral without an infinity in its
`Specials` needs its own `?`, an open cell, until the review decides what "behaves like a plain float"
means for a format that has no plain-float analogue. I lean toward the first reading, because "the
compromise that suits most default cases" already suggests Warm's natural habitat is the standard
IEEE shapes, but I state it as a lean, not a ruling.

**`Warm`'s in-range direction is `ToEven`, independently confirmed rather than merely carried over.**
Round-to-nearest, ties-to-even is not this review's choice for Warm, it is IEEE 754's own default
rounding attribute, the one every conforming implementation uses absent an explicit control-register
change (`IEEE Std 754-2019 §4.3`, cited as a standard, not as arvo's shipped tree). "Behaves as f32 and
f64 do in Rust today" and "nearest, ties to even" name the same fact from two independent directions,
neither derived from the other, which is the strongest kind of agreement available here.

**`Warm`'s door is `HostFloat<E>`, and it takes the same refusal posture `Hot`'s already does, for the
same reason.** The alternative, a silent software fallback for a `Warm` numeral the host does not
implement, is exactly the failure mode section 5.1 already ruled out design-wide. This has one
concrete, checkable consequence for the review to carry forward: the diagnostic text file 59 built for
`Hot`'s refusal names `Warm` as one of the software-quantiser alternatives ("Choose `Warm`, `Cold` or
`Precise`, which lower through the software quantiser at every numeral", `59:190-192`). That sentence
is no longer true once `Warm` also conditions on `HostImplemented`. I flag it as a stale diagnostic
string to correct when a stub exists to correct it, not something to patch here.

**`Warm`'s `StoredWidth` and `Layout` diverge from the fixed-point row, and the divergence is the
sharpest single finding this dispatch produces.** D71's fixed-point `Warm` doubles the container so a
single operation's intermediate never needs a separate wide type. A real FPU does not need this trick
and does not do it: IEEE 754 requires a correctly-rounded result, computed as if with unbounded
intermediate precision and rounded exactly once, and the hardware delivers that property for free,
internally, invisibly to software. Doubling `Warm`'s stored width for a float numeral would add
storage the hardware never asks for and the "no framework on top of it" intuition explicitly forbids;
a plain `f32` is four bytes, not eight, at rest. So `Warm`'s float row is `StoredWidth = minimum`,
`Layout = dense`, matching `Hot` exactly on both axes and diverging from `Warm`'s own fixed-point row
on both. The two number kinds needed the doubling for the same underlying reason, correctly-rounded
intermediates, and only one of them lacks hardware that already gives it away for free.

*Grounded on: `ratified` (`68b:62-67`, the intent quote), `settled shapes` (the `Clamp`-as-`TowardNegative`
reading, `11:195-196`), compiled (`70_probes/probe_1`, `probe_1b`, `probe_1c`), reasoned (the
extended-grid construction, the `StoredWidth`/`Layout` divergence, IEEE 754's default rounding
attribute cited as a standard rather than as arvo's tree).*

### 5.4 `Cold`, and why its door differs from `Warm`'s even though both intents are float-general

"Between warm and precise", "seldom computed... can take more cost than warm", "shouldn't just be
precise in disguise" (`68b:69-73`). Two of these three phrases are comparative, against `Warm` and
`Precise`, so `Cold`'s row is derivable once theirs are settled, not independently of them.

**`Cold`'s door is `Quantised`, and the argument does not repeat "storage minimisation forces it"
without saying why.** `Cold`'s whole identity is minimum storage, and minimum storage for a float
numeral almost never means a standard IEEE binary32 or binary64 shape; it means a reduced-precision or
bitpacked format chosen specifically because it is smaller than the standard widths, exactly the kind
of numeral no general-purpose FPU implements natively. Even in the case where a `Cold` numeral happens
to be a standard width, `Cold`'s own `Layout = bitpacked` already commits every access to a software-
managed pack and unpack step; routing the arithmetic itself through the same software quantiser that
already handles that crossing costs nothing additional on top of a cost `Cold` is already paying, and
buys exact intermediate rounding across the pack/unpack boundary rather than an extra, uncoordinated
rounding step. This is the fixed-point argument's fresh restatement (section 3, `Cold`'s "compare and
select on every store" cost), carried to float on its own terms rather than borrowed from the fixed-
point row unexamined.

**`Cold`'s in-range direction and range resolution are nearest, `ToEven`, and the far-direction reading
where `Specials` supports it, matching `Warm`'s resolved values but for a different reason.** `Cold`
is "between warm and precise", and both of its neighbours resolve out-of-range the same way `Warm`
does once `Warm`'s own `Specials`-conditional infinity reading is in place: `Warm` reaches the far
point because that is what plain hardware floats do; `Precise` refuses outright. `Cold` sits between
them by taking the far-point reading when the numeral supports it (matching `Warm`'s behaviour, not
`Precise`'s refusal, because `Cold`'s intent never says it is willing to fail a computation, only that
it is willing to pay more for one), and by staying `?`, genuinely open, for a numeral whose `Specials`
does not carry the infinity, exactly the open question section 5.3 already flagged for `Warm`, now
inherited rather than independently resolved.

**`Cold`'s `StoredWidth` and `Layout` are minimum and bitpacked, unchanged from the fixed-point row and
needing no new argument.** Storage minimisation is not a number-kind-specific intent; it means the
same thing for a float numeral that it means for a fixed-point one, and nothing about hardware
rounding behaviour bears on how a value sits at rest.

*Grounded on: `ratified` (`68b:69-73`), reasoned entirely (no compiled probe adds anything section 5.3's
already did not establish).*

### 5.5 `Precise`, confirmed, and its full row completed

Op already settled `Precise`'s door: "most precise at the price of both storage and compute" is the
software quantiser (`68b:56`). The structural reason, stated once rather than borrowed from the fixed-
point case: a hardware instruction is unconditional and infallible by construction, it always computes
some bit pattern and returns it; `Precise`'s own identity requires a refusing branch, which needs a
conditional return, which no single instruction offers. This is the same argument file 59's probe 7
reached by measurement (`f32::MAX * 2.0` delivers `inf` on the host, where `Precise` owes a refusal),
now derived from what `Precise` says it is rather than from what one measured instruction happened to
do, which is the stronger of the two grounds and the one that generalises past this one host and this
one instruction.

`Precise`'s in-range direction is `ToEven`, for the identical reason it was in the fixed-point row:
refusing a value that is already exactly representable, or that rounds without ambiguity, contradicts
"most precise" for nothing. `OverRange`/`UnderRange` are `Refuse`/`Refuse`, unconditionally, regardless
of what the numeral's `Specials` carries; unlike `Warm` and `Cold`, `Precise` never needs the far-point
reading at all, because refusing is available to every numeral, with or without a representable
infinity to round toward. This is worth stating as a small point in `Precise`'s favour: it is the one
preset among the four whose out-of-range row needs no well-formedness condition on `Specials`, because
`Refuse` never has to ask what lies past the edge.

`StoredWidth` and `Layout` carry forward from D71 (doubled, dense), on the same reasoning section 3
already gave: a per-operation widen already provides room for a single operation's exactness, and the
doubled resting storage is what lets a chain of operations retain more than one operation's worth of
precision before a narrowing step forces a decision, which is squarely "at the price of ... storage".
A genuinely interesting downstream elaboration, not something this dispatch needs to settle, is that a
doubled-storage float representation is exactly the shape a compensated or double-double technique
uses to buy effective precision beyond a single float's mantissa; whether `Precise`'s doubled storage
should be read that literally is a question for whoever eventually builds the numeral tower's
arithmetic, not for the preset table.

*Grounded on: `ratified` (`68b:56`), reasoned (the structural refusal argument, the `Specials`-
independence point, the `StoredWidth`/`Layout` carry-forward).*

## 6. The hardware-reachability theorem, and why it changes

File 69 flagged this dependency explicitly rather than fixing it: "if that re-derivation changes what
any preset means in a way that changes its rank... the uniformly-`Hot` theorem inherits that ground
shift silently... This is not a defect in what exists today; it is a dependency the preset
re-derivation dispatch should discharge explicitly" (`69:231-243`). This section discharges it.

**`RANK`'s ordering itself survives re-derivation, on independent grounds, and I check this rather than
assume it.** `RANK` resolves a mixed-strategy expression to the more conservative operand, `Precise >
Cold > Warm > Hot` (`arvo-strategy/src/lib.rs:104-107`, cited here as `tree-fact`, an existing
mechanism whose ordering I am about to re-derive independently rather than trust as evidence of
meaning). `Precise` is still the most conservative of the four under the fresh reading above: it is
the only preset that ever refuses. `Hot` is still the least: it is the only preset whose door ever
reaches hardware unconditionally and whose in-range rounding is the cheapest available rather than the
IEEE default. `Cold` ranking above `Warm` still holds: `Cold`'s own fresh intent ("can take more cost
than warm") is a preset volunteering to spend more to protect a stored value, which is what
"conservative" already meant in the old ordering, restated rather than assumed. Nothing in the fresh
derivation above moves any of the four out of the relative order they already had.

**What does change is which cells the ordering routes to hardware, because `Door` assignment changed,
not because `RANK` did.** File 59's own theorem was "the hardware door is reachable only in a
uniformly-`Hot` expression" (`59:247`), true under the void table where `Hot` was the only preset whose
door was `HostFloat<E>`. Under the re-derivation above, `Warm`'s door is `HostFloat<E>` too. A mixed
expression resolves to the more conservative operand's door; the resolved door is hardware exactly
when the resolved strategy is `Hot` or `Warm`, which happens exactly when *both* operands rank at or
below `Warm`, i.e. both are in `{Hot, Warm}` (any operand ranking `Cold` or above forces the resolved
strategy to `Cold` or above, and both of those doors are `Quantised`). That is four cells of the
sixteen-cell matrix, not one: `(Hot, Hot)`, `(Hot, Warm)`, `(Warm, Hot)`, `(Warm, Warm)`. This is a
counting argument over a totally ordered four-element set with two elements on the hardware side of
the door split; it needs no probe, only the ordering (confirmed above) and the door assignment
(sections 5.2 and 5.3). The corrected statement for the next consolidation to carry: **the hardware
door is reachable exactly on the sub-matrix where every operand ranks at or below `Warm`.** Whether
this sub-matrix stays a fixed two-by-two block or grows further is entirely a function of whether any
future preset or any redefinition moves a third preset's door to hardware; nothing about today's four
does.

*Grounded on: `tree-fact` (`RANK`'s existence and ordering, `lib.rs:104-107`), reasoned (the ordering's
survival, the corrected cell count).*

## 7. What is compile-time, what is runtime, and under what

Every axis this dispatch touches, `Quantisation`'s `Direction`/`Resolution` pair, `StoredWidth`,
`Layout`, `Door`, is a type, resolved by the type checker at the call site through
`DefaultLowering<N>: Strategy` and the preset's own trait impl. Monomorphisation is the only dispatch
in play; nothing here reaches a `dyn` or a `TypeId`. The `HostImplemented` bound that decides whether
`Hot` or `Warm` refuse for a given numeral is checked at compile time, per target, cfg-gated (Kind 1
structural lowering); a target that lacks the relevant FPU support refuses to build rather than
silently falling back, which is a compile-time refusal, not a runtime branch. The one runtime-adjacent
surface is the receipt file 59 already scoped narrowly and declined to expand: `pub const fn
receipt<L: Lowering>() -> Option<(Rounding, bool, bool)>`, itself `const`-callable, returning `None`
for the `Quantised` door because a quantiser reads no control state at all. Verifying that receipt
against a live process's actual FP control register, and invalidating it on a write, are both stated
as owed to a build layer arvo does not ship (file 59 section 2.5); nothing this dispatch proposes changes that
boundary or adds runtime cost to it. The three probes in `70_probes/` compiled clean under `--edition
2024` with no `#![feature(...)]` line, on the pinned nightly, `aarch64-apple-darwin`, no other codegen
flags; the mechanism they test needs no unstable feature to express.

## 8. What this dispatch leaves open, and what it hands forward

**Genuinely open, marked `?`, not filled in.** `Warm`'s and `Cold`'s `OverRange`/`UnderRange` for a
float numeral whose `Specials` does not carry the relevant signed infinity (section 5.3, section 5.4).
I lean toward scoping `Warm` (and, following it, `Cold`) to numerals whose `Specials` includes the
infinity, but I state the lean and not a ruling; the alternative, an explicit open cell for the rest,
is the review's own established convention for exactly this situation.

**Flagged forward rather than resolved, because resolving it is a `B3` question ("what a preset is
mechanically", still open per the topic file) rather than a preset-content question.** Whether
`Quantisation`'s declared type is even consulted for a numeral whose `Door` is `HostFloat<E>`. The
hardware instruction determines the result regardless of what `Quantisation` names; a declared type
that is never consulted should still be honest about what would happen if it somehow were, which
argues for `Hot`'s and `Warm`'s declared `Quantisation` matching what the hardware actually does
(section 5.2, section 5.3) rather than an arbitrary placeholder, but whether the trait system should
express "this member is inert under this `Door`" as a first-class fact is squarely a mechanical
question about what a preset *is*, which this dispatch was not asked to settle and does not attempt
to.

**Flagged forward, needing primary-source verification rather than my own recollection.** Whether
IEEE 754's overflow tie-break, at the exact halfway point between the finite maximum and the next
grid step, favours the infinity unconditionally or follows the ordinary ties-to-even rule extended to
that boundary. I believe the standard favours the infinity at the tie, which would make the far-
direction reading's `OnMidpoint` cell a directional constant rather than `ToEven`, but I have not
checked the standard's own text for this session, and the review's own established discipline (the
`E4M3` `Specials` witness, checked against the specification rather than vendor documentation,
`68b:36`) is exactly the discipline this claim needs before it hardens. I state the uncertainty rather
than resolve it plausibly.

**Handed to whoever next touches the shipped-mechanism side of this.** File 59's diagnostic string
naming `Warm` as an unconditional software-quantiser alternative to `Hot`'s refusal (`59:190-192`) is
now stale under this dispatch's re-derivation and needs correcting when a stub exists to correct it.

**Handed to the next consolidation, stated once so it is not restated as settled a fifth time.** File
59's own table (`63:582-586`, standing unchanged through `68`) was already named void by op and is not
restated here in any form; this document's sections 3 and 5 are the replacement, not an amendment to
it. The hardware-reachability theorem's corrected statement (section 6) supersedes `59:247`'s "reachable
only in a uniformly-`Hot` expression" and `59_probes/probe_6`'s single-cell assertion, both of which
were compiled correctly against the table that has since been voided and both of which need re-running
against the corrected door assignment before either is cited as current again.

## 9. The table-diff self-check

Each table above was checked against the prose of the section it sits in and, where a row cites a
tree-fact, against the file and line that established it. The fixed-point table's `Door` row was
checked against section 3's own argument for why it is inert, not asserted separately. The float
section's tables are inline in prose rather than a single combined table, deliberately: three of the
four presets carry a genuine open cell or a cross-axis condition (`Warm` and `Cold`'s `Specials`
dependency) that a flat table cell cannot state honestly without a footnote doing the real work, and a
footnote-heavy table is worse than prose that says the same thing once. The `70_probes/` outcomes file
was checked line by line against the three source files it describes rather than paraphrased from
memory of writing them.

## 10. Verification

`70_probes/probe_1_far_direction_positive.rs`, `probe_1b_negative_control_nospecials.rs`, and
`probe_1c_negative_control_nanonly.rs`, each compiled and run fresh in this session against `rustc
1.98.0-nightly (57d06900f 2026-05-27)`, `aarch64-apple-darwin`, `--edition 2024`, no other flags,
outcomes reproduced verbatim in `70_probes/OUTCOMES.md`. Canon gate and test gate reproduced fresh at
the top of this document. `202607301100_topic.the-formalization-talk.md` lines 1600 to 1780 read
directly for D69 through D73 rather than through any panel file's paraphrase. Every tree citation in
this document is marked `tree-fact`; none is offered as evidence of a preset's meaning, and section 0
states the test I held each one to before it entered the document.
