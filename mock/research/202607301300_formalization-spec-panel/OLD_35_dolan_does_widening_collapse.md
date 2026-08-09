# 35. Does Widening collapse

**Member:** Stephen Dolan. I wrote file 14 on the algebraic half, much earlier; the material has moved
a long way since, the identity half is new ground to me, and I do not carry that file's conclusions
forward unexamined. My own habit of mind is the one this dispatch asks for directly: a design that
looks like it needs a whole extra piece of machinery is usually solving a problem a smaller, already-
present mechanism already answers, and the job is to find which. That is what MLsub was: subtyping
inference had a reputation for needing constraint sets and bespoke closure algorithms until polarity
showed the general rule was smaller than every special case in the literature. The same question is in
front of me twice in one dispatch, at two different axes, and I compiled both before trusting either.

**Gate:** `cargo test --workspace` from `mock/`: 654 passed, 0 failed, 9 ignored, exactly matching file
34's own count, which itself matched files 31 through 33. I re-ran the negative greps myself rather
than trusting the citation chain: `grep -rn "Monotone\|Magma\|AddAssoc\|Distributes\|Associative"
crates/ --include="*.rs"` returns nothing, no `arvo-algebra-contracts` directory exists, and
`TotalOrd`'s declaration at `arvo-numeric-contracts/src/lib.rs:59-68` reads exactly as file 34 quoted
it. Canon gate: `30b_op_checkpoint_seven.md` and `34b_op_checkpoint_eight.md` are the governing calls
for this dispatch; both read in full. Nothing below overturns a D-numbered call or either checkpoint.
The two axis-table questions I was sent to answer are exactly the two the consolidation and file 34
hold open (`26:52-59`, `26:393-403`, `34:389-417`); I am answering a question the ratified table
itself leaves open, and I say so at each point rather than presenting it as settled by anyone but op.

**What I read:** `26_consolidation_two.md` in full. `30b_op_checkpoint_seven.md` and
`34b_op_checkpoint_eight.md` in full. `34_giesen_the_three_halves_assembled.md` in full.
`31_arntzen_settling_the_identity_contract.md` section 4 in full, per the brief. I reached into `33`
for the exact key-table row text (`33:235-243`) and the funnel citation (`33:279-282`), and into `26`
for the two accumulator-sufficiency lines (`26:157-158`, `26:269-271`) that the multiplicative half
already built and that my argument leans on. `ls` of the directory, once, before starting; 34 numbered
files plus their probe directories, matching the brief's own count.

**What I compiled or measured, separated from what I reasoned.** Three artifacts in `35_probes/`, each
summarised in `35_probes/OUTCOMES.md`, against the workspace pin (`rustc 1.98.0-nightly (57d06900f
2026-05-27)`, confirmed with `rustc --version`, the identical pin file 34 used). Probe 1 reuses
`25_probes/05_composed_exact_product.rs`'s width-arithmetic machinery, unmodified in substance (I
re-verified it compiles and runs before extending it), to build all three of the old `Widening`
instances with no axis at all, and checks both value agreement (functional shape) and instruction
identity (codegen shape, `-C opt-level=3`, no LTO). Probe 2 is a structural, exhaustive-by-construction
demonstration that `Growth` is derivable from the operation and can be made unrepresentable-as-
independent rather than merely observed-as-redundant. Probe 3 repeats probe 1's codegen check at a
width beyond one native register, where a genuine cost asymmetry exists in principle (schoolbook
multiplication needs three limb-products for a truncated result against four for the full product),
specifically to stress-test the claim past the case where "it's obviously free." All three compile
clean; the OUTCOMES table states exactly which build shape produced which number, per file 34's own
corrected lesson about `-C lto=fat` on an unlinked `--emit=asm` build silently deferring the vectoriser
(`34:71-97`). Sections 1 through 3 below are compiled or measured except where marked; section 4 is
reasoning built on those results.

## 0. The verdict, stated first

Both collapse, and neither is a renaming. `Widening`'s entire distinguishing content decomposes into
three mechanisms that already exist for unrelated reasons: which primitive is named, what numeral type
that primitive returns (ordinary `Numeral` typing, built by the multiplicative half to widen products),
and that numeral's own `StoredWidth`/`Layout`. Nothing new receives the job; three pre-existing
mechanisms jointly cover it, measured to cost nothing extra at native register width and, more
surprisingly, at a multi-limb width where a real asymmetry exists in principle. `Growth` decomposes the
same way, and further: it was never really an axis of a single numeral's `Policy` at all. Growth
describes a relationship between an operand numeral and a result numeral (does this operation widen or
not), which is a fact about an operation's signature, not a fact about one numeral's own identity. That
is a sharper reason for its removal than "the operation name already determines it" (file 34's reading,
confirmed and kept, `34:393-403`): it says Growth was the wrong *kind* of thing to be a `Policy` axis in
the first place, sorted onto a unary slot for a fact that is really relational. `Quantisation` does not
share this defect (its rounding and overflow policy is a genuine unary property of the destination
numeral, fixed regardless of source), which is why it survives where `Growth` does not.

Together, the two collapses do not merely resolve the Lattner gap (`26:52-59`) with a compatibility
predicate; they make it unstatable. `Growth::Exact` paired with `Widening::None` cannot even be spelled
in the resulting vocabulary, because neither name is an axis instance anymore. Section 4 gives the
resulting `Lowering` and `Policy` shapes in the form the next consolidation could take almost verbatim.

## 1. Widening collapses (compiled)

### 1.1 The claim, and why the multiplicative half already contains its refutation

The old axis (`11:165`, unchanged through file 30/31's identity contract at `31:347-352`) had three
instances: `None` ("Hot needs no intermediate room at all"), `InContainer` (the widened value fits in
the original numeral's own over-allocated storage), `PerOperation` (a fresh, wider carrier per call).
Consolidation's own text already recorded that three of the four presets treat it as *derived* from
`(Growth, StoredWidth)`, and only `Precise` carried it as an independent choice (`26:54-57`). That
already meant three quarters of the axis's instance space was doing no independent work before this
file touched it.

The multiplicative half (consolidation section 1.5, file 33 section 3.3, file 34 section 2.6) settled,
independently of this question, that every exact intermediate is a real, named `Numeral`: `mul_full`'s
return type is the product numeral (`I1+I2, F1+F2`, computed by a type-level ripple-carry adder over
zero-sized width types, verified compiling and correct at realistic scale, `26:245-261`), and a fold's
accumulator is an explicit, checked-sufficient numeral parameter (`26:278-286`). Once that is true,
"where does the widened value live" has an answer that does not need a separate axis to give: it lives
in whatever numeral type the operation that produced it returns, exactly as it would for any other
value. The axis was answering a question that only existed because the old design had not yet built the
machinery that makes every intermediate a first-class type. It is not that a trick routes around
`Widening`; the multiplicative half's own settled result already obsoleted it, one file before anyone
asked whether it had.

### 1.2 All three old instances, with no axis, compiled (`35_probes/probe_1`)

- **`None`** (Hot): `hot_mul_direct`, a native operation with no software-visible wide value at all.
- **`InContainer`**: not tested as a separate case because it was never a fact about an *operation*; it
  is a fact about the *original* numeral's own `StoredWidth::DoubleLogical`, unrelated to whether that
  numeral's arithmetic happens to widen. It survives unchanged, correctly re-scoped: a container's own
  over-allocation is a `StoredWidth` fact about that container, full stop, not a fact smeared across a
  separate "where does headroom for this op come from" vocabulary.
- **`PerOperation`** (Precise): `precise_mul_widens`, which is exactly `mul_full`'s own return type with
  nothing else named. "Per operation" was always the plain fact that `mul_full` is called once per call
  site and returns a fresh type each time; there was never a decision here beyond "do not call
  `quantize` yet."

Measured, `-C opt-level=3`, no LTO (probe 1's codegen shape, file 34's corrected shape A): `hot_mul_direct`,
the composite `hot_mul_via_full_then_quantize` (a real `mul_full` call into the named product numeral,
then `quantize_wrap` narrows it back), and `precise_mul_widens` all compile to **the same symbol**
(`_hot_mul_via_full_then_quantize = _hot_mul_direct`, `_precise_mul_widens = _hot_mul_direct`), one
instruction (`mul x0, x1, x0; ret`). LLVM folds all three before the codegen shape even reaches the
assembler with three distinct bodies. This is the consolidation's own fold-detection mechanism observed
directly (`26:399-407`), applied to multiplication rather than addition, and it says the axis bought
zero codegen distinction that the compiler was not already going to produce on its own from the
composite form.

### 1.3 Stress-testing past the case where it is obviously free (`35_probes/probe_3`)

The native-width result above is not automatically representative. At a width beyond one native
register, computing the exact product genuinely costs more instructions than computing a truncated
result directly: for two 2-limb operands, schoolbook multiplication needs three limb-products for the
low limbs of a truncated result and four for the full product. If `Widening` were doing real cost work
anywhere, this is where it would show, because "always compute the exact product via `mul_full`, then
throw half of it away" has a real, non-hypothetical cheaper alternative at this width.

Measured (128-bit operands, a genuine 256-bit intermediate via hand-written schoolbook limb arithmetic,
shipping-shaped build with the composite inlinable): `warm_mul_via_full_then_quantize_128` and
`hot_128_direct` (`u128::wrapping_mul`, a real truncating hardware-adjacent multiply) both compile to
four instructions (`umulh`, two `madd`, `mul`), the same shape up to commutative operand order. The
optimiser eliminates the `hi_hi` limb-product and its carries once it can see through the composition,
recovering exactly the three-limb-product form. Also tried, and kept in the probe as a negative control
per this review's own standing practice: the same composition with `mul_full_256` marked
`#[inline(never)]` (a check-build-shaped, axis-legible variant) does not fold; it pays a real call, a
stack frame, and a spilled return value, 24 lines against 7. This is file 34's own axis-legibility-
versus-codegen-quality lesson (`34:110-118`), independently reproduced at a different operation and a
different width: the two questions want different build shapes, and asking one with the other's flags
gets a wrong answer for a methodological reason, never a design one.

I state the honest limit of this evidence in the OUTCOMES file rather than only here: this is a positive
result on the current compiler, at two widths, one hand-written multi-limb shape. It is not a formal
guarantee that every width and every target folds this cleanly, and it carries the identical epistemic
status the consolidation already assigns its own multi-limb carry-chain finding, "a dependency on an
optimiser heuristic holding, not a guarantee" (`26:452-457`). The right response is the one that finding
already recommends: one codegen test per question class, pinned as a regression check, so a future
toolchain regression is caught rather than silently eaten as a preset's performance.

### 1.4 What it costs the four presets

None of the four presets loses anything. Concretely, with no `Widening` axis anywhere:

- **Hot**: `Mul` narrows immediately (`mul_full` then `quantize` with `ReduceModulo`). Measured
  zero-cost fold at native width (1.2) and at multi-limb width (1.3). No change in behaviour, no change
  in cost, one fewer axis to instantiate at the call site.
- **Warm / Cold**: identical shape to Hot with a different `Quantisation` resolution; same measured
  fold. Nothing about these presets' expression changes.
- **Precise**: does not call `quantize` inside a fold interior at all; the accumulator numeral *is* the
  product or sum numeral, sized by the mechanism consolidation section 1.5 already built for a different
  reason (`const { assert_accumulator_sufficient::<N>(...) }`, `26:278-286`), independently of any
  `Widening` instance. `PerOperation` never needed its own vocabulary; it needed the accumulator-
  sufficiency check the multiplicative half had already shipped.

What genuinely changes is upstream of the presets: whoever writes a strategy's `Mul`/`Add`
implementation now writes it as a straight-line sequence of named calls (`mul_full`, optionally
`quantize::<Src, Dst>`) instead of writing (or generating) an impl keyed on a `Widening` instance. This
is strictly less code to generate, not more: the old shape needed the `(Growth, StoredWidth)`-derives-
`Widening` line to be true for three presets and a separate, undocumented case for the fourth; the new
shape needs no derivation rule at all, because there is nothing left to derive.

### 1.5 The renaming check, explicitly

The specific risk I was sent to watch for: does the axis reappear inside whatever absorbed it, so the
design gained nothing. It does not, and I can say precisely why not. `Widening`'s content splits three
ways, and each destination already existed for an unrelated reason before this file:

1. **Which primitive is called** (`mul_full` versus a composite ending in `quantize`). This is ordinary
   function naming, present in the multiplicative half since file 25/33, not new vocabulary invented to
   receive the axis.
2. **What numeral type that primitive returns.** This is the `Numeral`/`AddWidth` machinery, built by
   the multiplicative half to make width arithmetic type-level, for a reason that has nothing to do with
   `Widening` (it exists so `mul_full` can be one generic function over every width pair, `26:252-256`).
3. **That numeral's own `StoredWidth`/`Layout`.** These are the unchanged `Lowering` axes, present since
   file 11, describing container shape for any numeral, widened or not.

None of these three is a disguised `Widening`. Each was independently motivated, independently built,
and independently priced before this question was ever asked. The axis is not relocated; its job was
already being done, redundantly, by machinery that arrived after it was drafted.

## 2. Growth leaves the key, and leaves Policy (compiled, and one structural argument beyond what was asked)

### 2.1 The narrow question, confirmed and sharpened (compiled, `35_probes/probe_2`)

File 34's reading: the key's operation slot already determines quantiser presence; carrying `Growth`
beside it stores the same fact twice (`34:393-403`). File 33's own key table agrees with itself in a way
worth quoting directly, because it is the cleanest evidence available and I did not have to build
anything to find it: the `Growth` row states its entire content as "decides whether a quantiser is
present at all between the exact operation and the result" (`33:241`), and the row two above it, "the
result numeral," states that `mul_full` maps into a different numeral than its operands, so the law
needs the result named, "never for widening operations" (`33:238`). Those two rows already say the same
thing about the same fact from two directions: knowing the result numeral (and hence which primitive
produced it) already tells you whether a quantiser fired. `Growth` was never elided from the key
(`33:241`'s own "never" column) precisely because nobody had noticed the "result numeral" row already
made it redundant.

Probe 2 turns this from an observation into a structural fact. The old shape (`Growth` as a free-
standing enum, paired with an operation tag by convention) type-checks a pairing the design can never
build: `law_holds_old(true, GrowthOld::Narrowed)` compiles and returns `false`, but nothing in the type
system stopped the call, which is exactly the "carries too much... discipline, not mechanism" gap
consolidation section 1.4 already names for a different key parameter (`26:182-186`). The new shape
binds `Growth`'s content as an associated const on the operation marker itself (`Op::IS_EXACT`), and
there is no slot left in which to spell a mismatch. This is stronger than "the two facts happen to
agree everywhere tested"; it is "the vocabulary to disagree does not exist." That is the right target
for a key discipline that consolidation itself says should "err toward key inflation deliberately"
(`26:184-186`): not adding more parameters to check against each other, but removing the parameter that
had nothing independent to check.

### 2.2 Beyond the asked question: Growth was never a fact about one numeral

This is my own addition, and I hold it with less certainty than 2.1 because it goes past what the
checkpoint scoped ("Growth leaving the law key", `34b:22-23`) into whether `Growth` belongs as a
`Policy` axis at all, which touches the ratified table more than the narrow question does. I state the
argument plainly because the checkpoint's own posture asks for an attempt at exactly this kind of thing,
and because ruling on the two questions separately risks leaving a redundant axis standing for no
reason once its sole job (informing the key) is gone.

`Policy` and `Lowering` are components of `S` in `Number<N: Numeral, S: Policy + Lowering>`, bound to
*one* numeral. Every other `Policy`/`Lowering` axis genuinely is a fact about that one numeral in
isolation: `Quantisation` says what a single numeral's own resolve/classify/round map does to an exact
value trying to land in *its* representable set, regardless of where that exact value came from (any
source, one fixed destination policy). `StoredWidth` and `Layout` say how *this* numeral's own container
is shaped. `Growth` was trying to say something structurally different: whether *an operation applied
to* this numeral widens or narrows its result, which is a fact about a relationship between an operand
numeral and a result numeral, i.e. about an operation's signature, never about one numeral's own
identity. A unary type parameter was carrying a binary (at minimum) fact. Once every such operation is
named directly (`mul_full`, `add_exact`, `quantize::<Src,Dst>`, per section 1), the relational fact lives
exactly where a relational fact belongs, in the operation's own signature, and there is no remaining
question for a per-numeral axis to answer. This is not the same argument as 2.1's key-redundancy (which
would survive even if `Growth` were correctly unary and merely duplicated by the operation name); it is
a claim about which *kind* of thing `Growth` always was, and it is the sharper of the two reasons to
remove it, because it explains why the redundancy could not have been designed around by picking a
different key spelling. A relational fact cannot be made to sit correctly on a unary axis by any choice
of vocabulary; it has to move to the operation.

I have not compiled this half. It is a type-shape argument, checkable in principle by asking whether any
consumer-facing operation exists, or could exist, whose growth behaviour genuinely cannot be read off
from (which primitive, which target numeral), and I did not find one, but I did not exhaustively search
arvo's operation surface for one either. I hold it as the stronger reading, not as settled beyond what
2.1 already settled by compiling.

### 2.3 What it costs the presets

Nothing beyond what section 1.4 already costs, because `Growth`'s removal and `Widening`'s removal are
the same removal seen from two axes: a strategy's `Mul`/`Add` impl already had to name its call
sequence once `Widening` was gone; naming that sequence is simultaneously the entire content `Growth`
used to carry. There is no separate accounting to do.

## 3. The two questions interact exactly as the checkpoint said they would

Ruling on `Widening` alone, leaving `Growth` in place as an unused-but-present `Policy` axis, would have
left a redundant axis standing with no argument for its own removal (its sole content, per 2.1, was
about quantiser presence in the key; once that key question moves to the operation, `Growth` has nothing
left to be a fact about). Ruling on `Growth` alone, leaving `Widening` untouched, would have left the
Lattner gap exactly where it was, since the gap is a claim about a pairing of instances from both axes.
Settling both together, as the checkpoint asked: the specific uninhabited point, `Growth::Exact` paired
with `Widening::None` (`26:52-59`), is not resolved by a compatibility predicate. It is **unstatable**,
because neither name exists as an axis instance in the resulting vocabulary. That is a stronger outcome
than the consolidation's own framing anticipated (a predicate that rules the pairing out); there is
nothing left to rule out.

## 4. The settled shape, stated for the next consolidation

```rust
pub const trait Lowering {
    type Encoding:    Encoding;
    type StoredWidth: StoredWidth;
    type Layout:      StorageLayout;
    // Widening removed. Every exact intermediate is a Numeral, named in
    // the term (mul_full's return type; a fold's accumulator parameter),
    // with its own StoredWidth and Layout. Nothing is left for a separate
    // axis to describe; section 1 of file 35 is the argument and the
    // measurement.
}

pub const trait Policy {
    type Quantisation: Quantisation;
    // Growth removed. Growth described a relationship between an operand
    // numeral and a result numeral (does an operation widen), which is a
    // fact about an operation's signature, never about one numeral's own
    // identity; it belongs to the named operation (mul_full, add_exact,
    // quantize::<Src, Dst>), not to S. Quantisation survives unchanged:
    // it is a genuine unary fact about this numeral's own destination
    // policy, independent of source.
}
```

Nine axes remain of the original ten (`26:39-42`), not the eight a naive count might suggest, because
`Quantisation` was always described separately from the nine in consolidation's own accounting
(`11:150-152`); the count that matters is two removed from the `Numeral`/`Policy`/`Lowering` trio,
leaving `ExponentForm`, `Precision` (per D69, `30b:9-16`), `Domain`, `LogicalWidth` on `Numeral`;
`Quantisation` alone on `Policy`; `Encoding`, `StoredWidth`, `Layout` on `Lowering`. The Lattner gap
(`26:52-59`) is closed by removal rather than by a predicate, per section 3. The key discipline of
consolidation section 1.4 gains a stronger form where it applies (2.1): binding a derived fact to the
operation marker as an associated item, rather than threading it as a co-equal parameter, makes an
inconsistent pairing unrepresentable rather than merely unexercised, which is worth carrying to the
other key slots consolidation already flags as never-elided (`Quantisation`'s resolutions, the
accumulator numeral) the next time one of them is built rather than reasoned about.

## 5. What this file does not decide

The `Growth`-leaves-`Policy` argument (2.2) is reasoned, not compiled, and I hold it as the stronger
reading rather than as settled to the standard 2.1 met. Probe 1 and probe 3's codegen results are
measurements on this pin, at these widths, not a formal guarantee; I recommend, and have not built, the
one-codegen-test-per-question-class regression check consolidation's own multi-limb finding already
calls for (`26:452-457`), now covering the composite mul-then-quantize fold at native and multi-limb
width alongside the existing carry-chain and add-fold tests. Everything file 34 left open and did not
route through me (the relation-ladder fork for `Precise`, the `TotalOrd` level annotation, D39's honest
content, the dither-versus-`Refuse` choice, the type-level gcd's price) stands exactly as open as file
34 left it; I did not touch any of it.

## 6. Standing

Both table-touching questions get a settled answer rather than a held reading, per the checkpoint's own
instruction (`34b:19-20`). Nothing here overturns a D-numbered call, `30b`, or `34b`; where I answer a
question the ratified table leaves open, I say so at each point, and the removal is a recommendation for
op's ratification, not a unilateral change to the table. Section 2.2 is the one place I went past the
scoped question, and I have marked its confidence accordingly rather than folding it into section 1's
compiled certainty. The Lattner gap that has stood as the consolidation's oldest open structural item
(`26:52-59`, carried since file 12) is closed in this file, by the same move both times: a fact that
looked like it needed its own type-level vocabulary turned out to already be expressible, in full, by
machinery the multiplicative half had built for an unrelated reason. That is the smallest mechanism that
is actually correct, and it was smaller than what was already there.
