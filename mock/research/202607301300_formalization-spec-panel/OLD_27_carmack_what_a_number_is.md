# 27. What a number is, and what the identity side must say to name one

**Member:** John Carmack. First-principles systems lens: reduce the thing to what it actually is,
refuse the parts that do not earn their place, and compile the question wherever compiling is cheaper
than arguing.

**Gate:** run before this work. The full mock workspace suite: 654 passed, 0 failed, 1 ignored, and
the one ignore is a correctly catalogued gap (`crates/arvo/tests/fixed_point_div.rs:111`, tracked #5).
The test bodies nearest this file's subject were read, not counted: `crates/arvo/tests/identity_laws.rs`
is the honest post-audit full-matrix shape, and its own module doc records why sampling was the prior
failure. No tautologies found in the sampled surface. Alignment checked against the ratified calls this
file touches: D38 and D39 (op, 2026-07-29, the number-systems topic), D48/D50/D51/D52 (op, the spelling
and derivation calls), and the consolidation's statement that op's calls go stale when something better
surfaces. Where this file argues against a ratified call it says so in place and leaves the call where
it belongs.

**What I read:** `26_consolidation_two.md` in full;
`mock/design_rounds/202607301200_topic.the-formalization-spec.md` in full;
`mock/design_rounds/202607300800/202607291900_topic.the-number-systems-crate.md` in full;
`mock/design_rounds/202607300800/202607300400_topic.the-formats-named-and-the-taxonomy-revisited.md` in
full; `crates/arvo/src/ufixed.rs` and `ifixed.rs` (declaration sites only); the identity-laws and
width-probe test files. No other panel file.

**What I compiled or measured:** the full test suite (above); three probes in `27_probes/`
(one WORKS, two fail with exactly the predicted errors, and both failures are results, not obstacles;
see `27_probes/OUTCOMES.md`). **Everything else in this file is reasoned, and is marked as such where
the distinction matters.**

## 0. A brief-breaking check, first

The brief and six panel members describe `arvo-num-systems` as "a whole layer nobody in this review
has read, which declares the number systems themselves". Checked: **it does not exist**. There is no
crate, no source, no `Natural` anywhere in the tree
(`grep -rn "Natural" mock/crates/ --include="*.rs"` returns nothing, matching the topic's own claim at
`202607291900_topic.the-number-systems-crate.md:9`). What exists is one 125-line ratified topic file
recording D38 and D39. The "unread layer" is twenty minutes of reading, and the consolidation's framing
of it as the "cheapest, most repeatedly-flagged open item" (`26_consolidation_two.md:661-666`) is right
for the wrong reason: it is cheap because it is a decision, not a layer. Six members flagged it as a
cost input to type-level-set mechanisms; there is no cost to discover, because nothing is built. What
there is to do is design it, which is this file.

## 1. What a number is here, stated flat

Strip everything away and a value of `Number<N, S>` is this: **an integer k, drawn from a finite
integer interval, together with a type-level rule that injects k into a set of rationals (plus,
for floats, a handful of data that are not rationals at all).** The numeral is the injection rule.
Everything else in the design is about what happens when an operation's exact result has no preimage
under that injection (`Policy`) and what the bytes cost (`Lowering`).

Two consequences fall straight out of stating it this way, and both do work below.

First, the numeral has exactly two jobs: name the **set** (which rationals are representable) and name
the **indexing** (which k maps to which member). Those are different jobs. IEEE 754 separates them as
format versus encoding, and the spec's own D58 cites exactly that split
(`202607301200_topic.the-formalization-spec.md:70-75`) and assigns encoding to `Lowering`. Hold that
thought for section 2, because the axis set does not practice what D58 preaches.

Second, every representable value in every expressible numeral is rational. Binary quantum, finite
width: the values live in the dyadic rationals, or in ℚ for a `FullRange` quantum, and the num-systems
topic already derived this by hand (`202607291900:97-108`). Nothing inhabits ℝ, nothing ever will in
this substrate, and that is a fact to build on rather than an embarrassment: it is what makes the
membership question decidable from type parameters alone.

## 2. The identity contract is parameterised by encoding, and that is where its standing defects come from

The `Numeral` contract's five members are `ExponentForm`, `Adjustment`, `Bias`, `Sign`,
`LogicalWidth` (`202607301200:40-46`). Look at the float branch: `Stored<const BITS: Width, U>` names
the **width of the exponent field** (`202607301200:94-96`), and D69 says the significand "derives by
subtracting the exponent field and the sign bit" from `LogicalWidth` (`202607301200:117-121`). Both of
those are encoding facts. The set of representable float values is determined by
(radix, precision, emin, emax); the exponent field's bit count and the significand's stored bit count
are how an interchange encoding packs that set into a word.

The design parameterised the set by the encoding and then derived the set-parameters back out, and the
derivation is wrong in exactly the way the consolidation already lists as unresolved: "the significand
derivation is off by one against real hardware, missing the hidden leading bit; the reserved exponent
codes and the exponent's own encoding bias are unnamed" (`26_consolidation_two.md:649-652`). That item
is not a bug to patch. It is the parameterisation reporting that it points the wrong way. The hidden
bit is not an off-by-one to fix; it is an encoding trick that has no business appearing in the identity
at all, and it appears there only because identity was written in encoding coordinates.

**The proposal: precision is the primitive, width is derived.** Reasoned, not compiled, but the
arithmetic is checkable by hand at every named format:

The numeral's uniform primitives become radix, **significand precision P** (a count of significand
digits), an exponent form, and a sign. Fixed point: `UFixed<I, F>` has P = I + F and a constant
exponent of -F. Floating: binary64 is P = 53, exponent ranging over [-1022, 1023] with gradual
underflow. Storage width is then a `Lowering`-side derivation: P for unsigned fixed; 1 + P for two's
complement signed fixed; sign + exponent-field + (P - 1) for a hidden-bit interchange encoding, with
the exponent field's own width, its encoding bias, and its reserved codes all derived alongside, on the
encoding side where D58 already put BID and DPD.

What this buys, concretely:

1. **The unresolved `Stored`-versus-IEEE item dissolves rather than being fixed.** binary64 is
   `Ranged<53, -1022, 1023, Gradual>` exactly, no off-by-one, because the hidden bit never enters. The
   x87 80-bit extended format (explicit leading bit, precision not derivable from width by the
   interchange formula) becomes expressible for free, which the current shape cannot do at all: it is
   a second encoding of the same kind of set, which is D58's own BID/DPD argument replayed.
2. **D69 is overturned, and the spec predicted it.** "LogicalWidth is primitive and not derivable" is
   true only inside the encoding-first parameterisation. The provenance note at `202607301200:359`
   lists exactly this claim among "the most suspect things here". It was right to.
3. **The shipped facade already agrees.** `IFixed`'s own declaration says "The sign bit is implicit:
   logical width is `1 + I + F`" (`crates/arvo/src/ifixed.rs:37-40`). The user-facing parameters I and
   F are magnitude-precision coordinates; the width is computed from them today, at the declaration
   site, in shipped source. Making width the primitive axis means every alias expansion computes width
   from precision anyway and the float branch then re-derives precision from width, wrongly. The
   primitive should be the thing the aliases already hold. Rewrite cost of this inversion against the
   shipped tree is therefore near zero on the fixed side: the parameters do not move, only the axis
   they expand into.
4. **The exact-product story is untouched.** Section 1.5's "widths add, quanta multiply" becomes
   "precisions add, exponents add", which is the same typenum-style adder on the same numbers
   (`26_consolidation_two.md:227-243`). Nothing in the multiplicative half moves.

The counter-reading, carried honestly: D65's stated virtue is that IEEE defines its interchange
formats by total width, and "both derive from the exponent field's width" is how the standard's own
interchange-format table is written (`202607301200:81-86`). If the design's job were only the
interchange formats, encoding-first parameterisation matches the reference text. But the design's own
test is MATLAB, IEEE and SystemC jointly, extended formats exist inside IEEE itself, and the
consolidation's unresolved item is the measured cost of the encoding-first choice. I hold the
precision-first reading as the stronger one and note that the interchange table survives as a set of
`conv-ieee754` aliases computing field widths in the other direction, which is one const fn.

## 3. The design's own nesting rule, applied to the rest of the identity

`Underflow` nests inside `Stored` "because a constant exponent has no bottom to fall off and would
have to carry a value meaning the axis does not apply" (`202607301200:98-99`). That is the right rule.
Now apply it to the other two identity members, because it indicts them identically.

The value map that gives `Adjustment` and `Bias` their meaning is the affine map
`Adjustment * radix^exponent * k + Bias` (`202607301200:112-114`), and that map only exists on the
`Implicit` branch. Under a stored exponent the map is per-binade; a global origin shift B breaks it
(no standard has an affine-biased float, and the three-standard test finds no use), and a `FullRange`
quantum under a varying exponent names nothing (MATLAB slope-bias and UNORM are both fixed-point
concepts). So the axis product `ExponentForm x Adjustment x Bias` contains meaningless points, which
is precisely the defect Lattner found in the `Policy x Lowering` product ("unlowerable points with no
compatibility predicate stated anywhere", `26_consolidation_two.md:52-59`), sitting unnamed inside
`Numeral` itself. The consolidation's closure gap (`26:326-331`, adjustment and bias not closed under
multiplication) is downstream of the same structure.

**The structural fix is the one the spec already used once: nest them.**

```rust
pub const trait Numeral {
    type Radix:        Radix;         // section 4
    type Precision:    Precision;     // section 2
    type ExponentForm: ExponentForm;
    type Sign:         Signedness;
}

// the affine parameters live where the affine map exists
pub struct Implicit<const E: Exponent, A: Adjustment, B: Bias>;
// the range, underflow and specials live where an exponent can range
pub struct Ranged<const EMIN: Exponent, const EMAX: Exponent, U: Underflow, SP: Specials>;
```

Four top-level members instead of five, no meaningless points, and `Stored x FullRange` stops being a
case anyone has to gate because the type does not exist. The sorting test (D54) still classifies every
nested member as identity; nesting changes reachability, not classification.

The counter-reading: a flat axis list with a `WellFormed` compatibility predicate keeps the ten-axis
table symmetric and keeps generic bounds on `Adjustment` writable without projecting through
`ExponentForm`. That is real, and if the compatibility-predicate machinery gets built for
`Policy x Lowering` anyway (Lattner's gap needs an answer regardless), riding it is cheaper than
restructuring. I hold nesting as the more honest shape because the spec's own Underflow argument is
exactly this argument, and a rule applied once and skipped twice is worse than either consistent
choice.

## 4. What the identity is missing

**A radix.** Seconding the consolidation's flag (`26:642-647`) and sharpening it with the membership
tie: the finest-set derivation of section 5 is radix-dependent. A Unit-adjustment fixed numeral at
radix r inhabits ℤ[1/r], and containment between ℤ[1/r1] and ℤ[1/r2] is prime-divisor containment, so
radix-16 fixed point inhabits the dyadics (ℤ[1/16] = ℤ[1/2]) while radix-10 does not. Without a radix
axis the membership blanket impls of D61 (`202607301200:195-201`) cannot even be written generically.
One marker type, one instantiated arithmetic (binary), and the decimal proof case D58 leans on becomes
expressible instead of gestured at.

**Specials, as identity.** Nothing in `Numeral` says whether infinities and NaN are representable
data, yet by the design's own sorting test that is identity: change it and the representable set
changes. IEEE requires them; SystemC's `sc_fixed` has none; some DSP float formats ship without Inf.
The natural home is the `Ranged` branch (an `Implicit` exponent generates no specials, the same
nesting argument a third time). And there is a concrete hole this closes, cheap to check in prose:
the consolidation claims the quantisation vocabulary "reproduces every named rounding and overflow
mode in IEEE 754, SystemC and MATLAB's Fixed-Point Designer with no gaps" (`26:48-50`). IEEE's default
overflow behaviour produces **infinity**, and roundTowardZero's produces the largest finite; which one
you get is not expressible in a five-position vocabulary whose range has a finite top and whose
resolutions are directions, modulo, zero and refuse. With Inf as a representable datum the gap closes
from the other side: overflow-to-Inf stops being an out-of-range resolution at all, because the range
has a top element and "past the top" is unreachable, exactly as `Hot`'s wrap makes one end unreachable
in the faithfulness derivation (`202607301200:210-216`). The no-gaps claim is true of the finite
fragment and false of IEEE as shipped on hardware, and the fix is an identity member, not a new
resolution.

**Signed zero, named.** IEEE sign-magnitude carries -0, a datum distinct from +0 that maps to the
same rational. The moment it exists, the injection k -> value is not an injection, data-equality and
value-equality come apart, and every law in the apparatus that compares results (Kleene equality over
what, exactly?) needs to say which it means. The design's machinery is well equipped for this (it
already distinguishes grade-agreement from value-agreement for `Precise`, `26:214-221`); it just needs
the datum/value distinction stated once, at the identity, where it originates. A `Specials` member is
the place.

**One sentence worth having in the spec, found by walking this:** a NaN with a payload is a refusal
cause carried in-band. The float branch's fallibility is not `Outcome`-shaped, it is value-shaped;
IEEE built the effect monad into the data. That is why `FastFloat` can be total where `Precise` is
fallible, and stating it makes the D30 non-NaN witness (`202607300400:132-134`) the boundary where the
in-band effect is discharged back into the type-level one.

## 5. The number-system layer: what membership can and cannot mean over a quantised substrate

D38 and D39 are op's calls and the layer exists. The question worth a panel file is what it can
honestly say, because one sentence in D39 will produce a broken implementation if it ships as written.

### 5.1 Inclusion is not a homomorphism, and the design already measured that

D39: "a consumer bounding on `Real` gets the field operations because that is what being real means,
not because a separate bound was also written" (`202607291900:61-67`). Under the topic's own
"inhabits" precision (`:80-84`), membership is a statement about **values**. It cannot deliver
operations, because no shipped policy's in-numeral operation is the ambient set's operation:

`Warm`/`Cold` saturating addition is not ℕ's addition (it disagrees whenever the true sum leaves the
range). `Hot`'s wrapping addition is ℤ/2^n's addition, a different structure that is not in the tower
at all. `Precise`'s addition is a partial restriction. The recovery map phi of the consolidation's
own classification (`26:77-88`) is precisely the measure of inclusion's failure to be a homomorphism,
and the measured law inversions (wrapping associative but not distributive over max, saturating the
reverse, `26:126-137`) are the empirical face of the same fact. If "bounding on a set gets you the
set's operations" ships literally, someone will derive associativity and distributivity for a numeral's
own quantised ops from `Field` on the ambient set, and that derivation is **measurably false** by the
panel's own numbers.

### 5.2 What membership does license, exactly

There is one operation family for which inclusion into the ambient set IS a homomorphism: the exact,
widening family. `mul_full` and exact addition are literally the ambient ring's operations restricted
to a finite window, with the result landing in a different window and totality guaranteed by width
growth (`26:227-243`). No phi fires, so nothing breaks, so every ambient law transfers for free. That
is not a coincidence; it is what "exact" means.

So the honest content of the number-system layer is this, and it is more useful than D39's sentence,
not less:

**`Inhabits<S>` licenses the exact operation family of S, and nothing else. The quantised in-numeral
operations get their laws from the ladder, keyed per (composition, operation, accumulator), exactly as
section 1.4 already concluded.** The two-fact split even explains the interior-safety theorem
(`26:149-164`) instead of sitting beside it: a wide-accumulator fold is lawful because its interior is
computed **in the ambient structure**, where associativity is inherited through the now-honest
homomorphism, and quantisation fires once at the root, where a map applied once to a
grouping-independent argument cannot depend on the grouping. The number-system layer is the type-level
name for where the exact intermediates live. The MAC's accumulator holds a value of ℤ[1/2] that no
numeral in the composition can represent; today that fact is expressed as a width inequality
(`26:283-287`); with the layer it has a name, and the name is what a bound can carry across the
Stage G boundary.

### 5.3 The lattice reading does not survive contact, and I compiled the wreckage

Two probes, both in `27_probes/`, both small, both decisive.

**Upward membership cannot discriminate.** Inhabits is upward-closed along the tower: everything that
inhabits ℕ inhabits ℤ, ℚ, ℝ and every exotic rung above. So `Inhabits<Real>` is satisfied by every
finite numeral ever expressible here and can never select anything
(`probe_a_finest_system_dispatch.rs`, `check_real_is_vacuous`, compiles with both model numerals).
D38's motivating example, "when a value is in ℝ do this, when it is not ℝ but ℤ do that"
(`202607291900:20-23`), is not writable as membership bounds at all, in two independent ways: "not ℝ"
needs a negative bound the feature set does not have, and the naive positive transcription as two
blanket impls distinguished by `Numeral<System = _>` equality bounds **fails to compile with E0119**
(`probe_b_marker_lattice_divergence.rs`; associated-type-equality where-clauses do not participate in
coherence, and `min_specialization` does not apply because the impls are incomparable). That compile
failure is the finding: the lattice-of-marker-traits reading of D39 has no expressible divergence
mechanism under this workspace's constraints.

**The finest-set reading works whole, and compiled first try.** A numeral carries one finest system
as a projection (`type System`), derived from the axes by the same macro table that already feeds
integrality (`202607301200:224-228`), never free-assigned. Upward membership is one blanket impl over
a `ContainedIn` order stated on the marker ZSTs. Divergence is written per system marker, a closed
set, one impl per rung, no coherence pressure, selected through `N::System`, resolving in const
context. Refusal quality is better than anything hand-written: bounding on `Inhabits<Zint>` and
passing a fractional numeral yields `` the trait bound `Dyadic: ContainedIn<Zint>` is not satisfied ``
with the compiler volunteering which containment does hold (`probe_a2_downward_refusal.rs`, E0277 as
intended). This is the shape I would build.

### 5.4 The family markers and the membership layer are one mechanism wearing two names

D73 derives `IntegerLike`, `FractionLike`, `FloatLike` from the exponent form through a macro-expanded
integrality table (`202607301200:224-228`). The finest-system derivation conditions on the same table:
integral and unsigned is ℕ, integral and signed is ℤ, fractional Unit-adjustment is ℤ[1/r], other
exact quantum is ℚ. These are the same derived fact at two granularities, and the taxonomy file
already walked to the edge of saying so: "if they are derivable, D51 says they should be derived, and
the placement question changes into whether they exist at all" (`202607300400:115-120`). Finish the
thought: **one derivation, the finest system; the family markers become blanket impls over it**
(`IntegerLike` for `System ∈ {ℕ, ℤ}`) or disappear into it. Two independently maintained derivations
of one fact from one table is the kind of duplicated mechanism that ships one subtly wrong copy, and
this workspace has a rule with exactly that name.

### 5.5 The tower's far end, honestly

The exotic rungs (ℂ, ℍ, 𝕆, surreal, hyperreal, p-adic) cost a ZST and some order rows each, and as
finest-set classifications they are uninhabitable by any numeral the axes can express, while as upward
memberships they are vacuous. Ship them as op called (D38's "they ship even if nothing uses them" is
cheap and the vocabulary is fixed by mathematics), but build **zero** proof machinery for them, and
write one constraint down that walking them surfaces: the entire `Policy` contract presumes the
ambient set is a chain. Midpoints, directions, clamping, the five positions, all of it is
total-order vocabulary. ℚ_p and everything from ℂ upward are unordered, so a numeral whose finest
system is unordered cannot take the `Quantisation` contract as designed. That belongs in the spec as a
bound (`N::System: TotallyOrdered`, a ladder fact carried by the system marker), because it is the
honest scope statement of the whole ten-axis machine: it is a machine for ordered systems, and the
unordered rungs are vocabulary awaiting composite numerals (a pair of Numbers as a Gaussian-dyadic
ℂ-inhabitant is a real future design, whose system is then derivable from its components; one
paragraph of forward provision, no more).

This also answers the dependency direction the taxonomy left open (`202607300400:110-114`): the tower
and its order rows are upstream vocabulary depending only on the ladder (D39's stated edge, and now
with content: each system marker carries its ambient structure's ladder facts, which is where
"complete ordered field" lives without any numeral ever claiming it). The numeral crate depends on the
num-systems crate because `Numeral` projects `System`. Membership derivation lives with the numeral,
where the axes are visible. Format does not depend on membership; membership names what format
determines.

### 5.6 The "harder half" mostly evaporates, and D16 already says why

Op flagged the static proof machinery as where the design effort goes ("contracts without proofs
would be marker traits a consumer could implement wrongly", `202607291900:73-84`). Applying D16's own
dichotomy (`202607301200:190-193`) dissolves most of it. For anything expressed as a `Numeral`,
membership is **computed** from the axes: a blanket impl conditioned on structure cannot lie and needs
no proof obligation at all, which is D61 as already specced. The only place an assertion enters is a
foreign type that is not a `Numeral`, and for an opaque foreign type no machinery expressible under
these constraints can verify a for-all-values claim; that is an `unsafe impl` carrying a contract,
per D16, optionally backed by the same model-width exhaustive witness pattern the recovery maps
already use when the foreign type can enumerate. The crate is therefore small: markers, order rows,
one blanket derivation, one unsafe door. The real design effort goes into the derivation table, which
is shared with integrality anyway (5.4), so it is paid once.

## 6. What a consumer writes

Nothing new on the common path, which is the right answer and already ratified: `UFixed<13, 3>` and
the preset names survive unchanged (D48, `202607301200:315-318`). The composition is what the alias
expands to, and under section 2 the expansion holds precision coordinates the alias parameters already
are.

A consumer who diverges on domain writes one extra bound and one impl per system they handle, in the
probe A shape: behaviour on the marker, selection through `N::System`, refusal with a diagnostic that
names the containment that holds. A consumer who never diverges writes nothing and pays nothing. An
algorithm crate behind Stage G keeps bounding on operations; the system bound crosses that boundary
only when the algorithm genuinely cares about domain rather than laws, which the panel's own
measurements suggest is rare (laws attach to regrouping combinators, not to membership,
`26:121-137`). That is the honest answer to "what does the layer buy that a bound on operations does
not": classification, divergence, diagnostics, and a name for the exact-ops codomain. Not laws. Never
laws.

## 7. Summary of proposals, and what each costs

1. **Invert the float identity to set-first**: precision, emin, emax primitive; width, hidden bit,
   exponent-field encoding derived on the `Lowering` side. Dissolves the unresolved `Stored`/IEEE item
   and D69 (which the spec's own provenance note pre-flagged as suspect). Cost: the spec's axis table
   and the conv-ieee754 alias derivation; near-zero on the fixed side because the shipped parameters
   are already precision-shaped.
2. **Nest `Adjustment` and `Bias` into `Implicit`**, by the spec's own Underflow argument; `Numeral`
   drops to four members and the meaningless axis-product points stop existing. Counter-reading (flat
   plus `WellFormed`) carried in section 3.
3. **Add `Radix` and `Specials` (including signed zero) to the identity**; the IEEE overflow-to-Inf
   hole in the no-gaps claim closes as a side effect.
4. **Correct D39's operative sentence** before it ships: membership licenses the exact operation
   family only; quantised operations get laws from the ladder, keyed as section 1.4 already keys them.
   The interior-safety rule becomes the bridge between the two, not a third mechanism.
5. **Build membership as one finest-system projection plus a tower order with marker dispatch**
   (probe-verified in both directions), merge the D73 family markers into it, ship the exotic rungs
   inert, state the `TotallyOrdered` scope bound, and route foreign types through a D16 unsafe door.
6. **Dependency direction**: ladder, then num-systems, then numeral. Resolves the taxonomy's open
   question.

The one place I am pointing away from where the brief pointed me: the interesting problem was not
whether the number-system layer should exist (op called it, and built as 5.2 through 5.6 it earns its
keep at a few hundred lines), but that the **identity side is parameterised in encoding coordinates**,
which is upstream of the membership derivation, upstream of the unresolved IEEE item, and upstream of
the radix gap. Fix the coordinates first; the membership crate is then a weekend, and I have already
compiled its skeleton.
