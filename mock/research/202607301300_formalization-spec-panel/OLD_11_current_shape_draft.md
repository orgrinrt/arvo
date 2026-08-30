# The current shape: arvo's numeral, policy and lowering split

A design round restructured arvo's numeric core, replacing four separate types (`UFixed`, `IFixed`,
`FastFloat`, `StrictFloat`) with one type composed over three contracts, ten axes, and a set of
mathematical properties computed rather than hand-declared per type. That restructuring produced a
draft spec. A ten-member review panel then spent a day taking the spec apart, largely by compiling
things against it rather than by reading it, and the shape changed substantially under that pressure:
several derivations were shown outright wrong, one whole load-bearing mechanism was shown to prove a
theorem nobody needed while never checking the one that mattered, and the panel's own early answers to
that were themselves each found broken by whoever compiled them next.

This document states where that leaves the design right now, on the assumption that everything the
panel converged on can in fact be built. It separates what the lead designer (op) actually decided in
this round, what the panel converged on but op explicitly left open for further work, and what remains
broken, unresolved, or simply never reached. It is written to stand alone: a reader who has seen none
of the design round or the panel transcripts should be able to audit it from this text and the arvo
source tree.

Two things are worth holding while reading it. First, only op's calls are final in this design, and
even those are understood to go stale the moment something better surfaces; every "banked" item below
is one day old at the time of writing and is stated as the current shape, not as a permanent one.
Second, where a downstream consumer's existing code is cited as evidence, it is evidence of what arvo
lacked at the moment that code was written. It is not evidence of what the design should provide, and
it is specifically not license to preserve current behaviour for its own sake: the design's standing
instruction is to ask what a consumer would ideally deal with, not what they currently work around.

## 1. What this covers, and what it does not

The design round that produced this spec did much more than restructure the numeric core. In two days
it also relocated roughly a dozen other concerns: a dimensional and capacity foundation, a spatial
geometry crate, a number-systems crate, a platform-primitives crate, the storage container crate, a
bitfield macro crate, the float-wrapper crate's packaging boundary, a unified predicate concept, a
hashing and pseudorandom family, and the shared heterogeneous-list crate two other repos had been
independently reinventing. All of that was settled, in the same round, in the topic files that precede
the spec this document is about. None of it was reviewed by the panel. The panel's brief scoped it to
one artifact, the numeral/policy/lowering spec, and every one of its ten members stayed inside that
scope.

The table below is the whole taxonomy the design round touched, with each row marked by how much
scrutiny it has actually received. "Settled, untouched" means a decision was made and nothing has
stress-tested it since. "This document" is the numeral/policy/lowering restructuring covered below.

| Area | What was decided | Reviewed by the panel |
|---|---|---|
| Numeral, Policy, Lowering, the ten axes, `Number<N, S>` | Three contracts replace four numeric families | Yes, in depth: this document |
| `arvo-capacity` (`Cap`, `Capacity`, `Dim<N>`) | Lifted out from under the container crates | No |
| `arvo-shape` (rank, per-axis extents) | Rank is a cons-list of capacities, spans both value and bit domains | No |
| `arvo-geom` (`Point`, `Orthotope`/`Rect`, `Affine`) | Named; rotation grounded on rotors and motors; curve representation left to a future bench | No |
| `arvo-num-systems` (naturals through p-adics) | Membership defined through algebraic structure, by inhabitance not equality | No, though this document's format work changes what a numeral inhabits |
| `arvo-platform` (`USize`, `Bool`, `NUSize`) | Split out of the old storage crate | No |
| `arvo-container` (the renamed storage remnant: `Bits<N, S, Sign>`, width conversions) | Absorbed refit, gained a saturation-limits contract | No, despite being exactly what the new `Lowering` contract governs from above |
| `arvo-bitfield` | Given its own crate against a future proc-macro upgrade | No |
| `arvo-float` (packaging boundary for `FastFloat`/`StrictFloat`) | Kept as packaging rather than a mathematical claim | Only the boundary argument, not the crate's contents |
| The predicate concept (typestate predicate, homed in notko) | Two existing predicate families unified | No |
| `arvo-pseudorand` (hash, PRNG, noise as one family) | Redistributed along algebraic and purpose axes | No |
| `notko-hlist` (the shared cons-list crate) | Extracted, with a `Cardinal` counting trait | No, despite sitting in the same conceptual neighbourhood (folding, algebraic structure) as this document's algebra ladder |

Nothing below should be read as a statement about any of the untouched rows. Where this document
mentions one in passing (the container crate's relationship to `Lowering`, for instance), that is a
noted gap, not a finding.

## 2. The environment this design has to fit

Three standing constraints from outside this round bound every mechanism described below, and several
of the panel's findings turn on them directly.

arvo is `#![no_std]`, ships no `alloc`, and every size is const at the type level: no runtime growth,
no heap. Dispatch is monomorphisation only. There is no `dyn`, no `TypeId`, no virtual call anywhere in
the design; every choice a consumer makes is a type parameter, and the compiler generates one
specialised code path per concrete instantiation.

The workspace forbids two unstable Rust features outright: `generic_const_exprs` (a const expression
computed from a generic parameter, used in type position) and full trait specialisation. Both are
judged unsound with no prospective fix. A narrower, sound successor to each is allowed:
`min_generic_const_args` and `min_specialization`, plus `const_trait_impl` and `adt_const_params`,
which this design leans on throughout for `pub const trait` declarations and for zero-sized marker
types deriving `ConstParamTy`. Two live `#![feature(generic_const_exprs)]` gates remain in the crates
this round touches (`arvo/src/lib.rs`, `arvo-strategy/src/lib.rs`), both flagged as drift against the
current rule and both needing to go as part of this restructuring; one of the panel's findings, noted
below, ties their removal directly to a diagnostic repair.

And arvo's own standing design principle is that it exposes tools and never polices a consumer's
choice: a strategy marker such as `Cold` (bitpacked, minimum storage) is not an edge case to
deprioritise, it is close to the reason the substrate exists, and nothing in this design should
silently refuse a combination of axes a consumer might have a real reason to want.

## 3. The banked shape

Everything in this section is op's call, made in the design round, incorporating the panel's
mathematical and mechanical corrections where those corrections were verified and not since
overturned. It is written in the present tense, as the design stands, not as a sequence of decisions.

### 3.1 The three contracts and the one type

Four numeric families become one. `UFixed`, `IFixed`, `FastFloat` and `StrictFloat` are names for four
compositions of a single generic type, and what differentiates a composition is where its exponent
lives, not which struct it is.

```rust
/// What the number IS: which values are representable at all.
pub const trait Numeral {
    type ExponentForm: ExponentForm;
    type Adjustment:   Adjustment;
    type Bias:         Bias;
    type Sign:         Signedness;
    type LogicalWidth: Width;
}

/// What is returned when a result does not land in the numeral.
pub const trait Policy {
    type Quantisation: Quantisation;
    type Growth:       Growth;
}

/// What it costs to hold and to compute. Changes no answer.
pub const trait Lowering {
    type StoredWidth: StoredWidth;
    type Widening:    Widening;
    type Layout:      StorageLayout;
}

pub struct Number<N: Numeral, S>(..) where S: Policy + Lowering;

pub type UFixed<const I, const F, S = Warm>
    = Number<Fixed<I, F, Unsigned>, S>;
```

A strategy marker such as `Warm` supplies both `Policy` and `Lowering` at once; there is no separate
bundle trait between the marker and the two contracts it implements. The parameter list stays at two
(`N`, `S`), matching what the design round settled directly. A panel proposal to split `S` into two
parameters (`Number<N, P, L>`) was tried, endorsed by two members on the grounds that it would make the
"a law may never read a `Lowering` member" invariant a typing fact, and then shown by a later member to
deliver neither promise: an eleven-line counterexample compiles a law impl that names the `Lowering`
member cleanly under the split, and the split costs roughly 1.8 times the rendered type length in error
messages, moving the point where a composition's error spills to a "long type written to file" note one
modifier level sooner. The fused, two-parameter form is what currently stands; a real typed guarantee
against the same problem exists (section 5.2) but is not this parameter split, and is not yet wired in.

The axis a consumer decides by changing an axis, and only ever changes cost, never the values a numeral
can represent or the arithmetic performed: change it and ask whether the set of representable values
changed. If yes, it is identity (`Numeral`). If the same values are representable but the arithmetic
differs, it is policy (`Policy`). If neither changed and only the cost did, it is lowering (`Lowering`).
This is the sorting test the whole axis table below is built against; the panel found one further
question the test does not ask (section 4.2's third sort), which is currently unresolved rather than
overturned.

The decomposition itself, tested against nine of the ten axes by three independent panellists with no
disagreement, is the one part of the design that survived scrutiny cleanly: the derivations built on
top of it needed real repair, the cut itself did not.

### 3.2 The ten axes

Nine of the ten are listed here with their instances; the tenth, quantisation, is described separately
in 3.3 because of its internal structure.

| Contract | Axis | Instances |
|---|---|---|
| `Numeral` | `ExponentForm` | `Implicit<const EXPONENT: Exponent>` (the exponent is fixed in the type: constant spacing, fixed point), `Stored<const BITS: Width, U: Underflow>` (the exponent is stored per value, in a field of the given width: spacing that grows with magnitude, floating point) |
| `Numeral` | `Adjustment` | `Unit` (the quantum is a pure power of the radix, every ordinary numeral), `FullRange<const F: Width>` (the quantum is `radix^F / (radix^F - 1)`, not a power of the radix; this is what makes a UNORM-style encoding, all-ones landing exactly on 1.0, expressible) |
| `Numeral` | `Bias` | `Zero` (every ordinary numeral), `Offset<..>` (the affine origin is shifted) |
| `Numeral` | `Sign` | `Unsigned`, `Signed` |
| `Numeral` | `LogicalWidth` | the total bit count; primitive, not derivable from anything else on `Numeral` or from `Lowering` |
| `Policy` | `Growth` | `Exact` (keep the whole exact intermediate; widths add, quanta multiply, nothing is dropped), `Narrowed<W: IntermediateWidth, A: Anchor>` (keep `W` bits anchored at one end; anchoring at the least significant end preserves the quantum and shrinks the range, anchoring at the most significant end preserves the range and coarsens the quantum) |
| `Lowering` | `StoredWidth` | `Minimum`, `DoubleLogical` |
| `Lowering` | `Widening` | `None`, `InContainer`, `PerOperation` (naming where intermediate headroom comes from; `None` was added because `Hot` needs no intermediate room at all) |
| `Lowering` | `Layout` | `Dense`, `Bitpacked` |

`Underflow` is not counted as a top-level axis. It is nested inside `Stored`, because a constant
exponent has no floor to fall off and there is nothing for it to mean at every other numeral. Its three
instances, taken from the formal literature this design is grounded in (Flocq, the Coq library behind
CompCert's floating-point reasoning): `Unbounded` (no floor, no subnormals), `Gradual` (subnormals,
IEEE 754's own choice), `Flushed` (abrupt, flush to zero). Number-system membership (which of the
naturals, integers, dyadic rationals, or rationals a numeral's values inhabit) is likewise not an axis:
it is derived from the numeral, described in 3.3.

The value of a stored integer `k` under a numeral is the affine map `Adjustment * radix^exponent * k +
Bias`. `Adjustment` and `Bias` cannot be folded into each other: one changes the spacing between
representable values, the other moves the origin, and an affine map is not determined by either half
alone (worked example: UNORM8's values are `k/255`; an adjustment factor of `256/255` lands `k = 0` on
0 and `k = 255` on exactly 1, and no bias alone can do both, since matching one endpoint with a bias
displaces the other).

### 3.3 Quantisation, in full

`Quantisation` is one `Policy` axis with real internal structure. It is the single map from an exact
arithmetic result onto the numeral's representable set, and it replaces what earlier drafts treated as
two separate axes (rounding and overflow): both are the same kind of decision, in-range and
out-of-range halves of one map, which is what the field calls a quantizer (an analogue-to-digital
converter quantises and clips, and both are quantisation).

An exact result sits in one of five situations relative to the representable set: strictly between two
representable neighbours below their midpoint, exactly on the midpoint, strictly between them above the
midpoint, past the top of the range, or past the bottom. The first three have two neighbours to choose
between, so a rule for them is a direction. The last two have only one neighbour, so a rule there has
three further options: take the one neighbour that exists (still a direction, and the reason "clamp"
needs no name of its own: clamping above the range is simply `TowardNegative`, the same marker used
between neighbours), return something unrelated to where the value was, or refuse.

```rust
pub const trait Resolution {}
pub const trait Direction: [const] Resolution {}

// usable both between neighbours and, some of them, at a range end
pub struct TowardNegative;  pub struct TowardPositive;
pub struct TowardZero;      pub struct AwayFromZero;
pub struct ToEven;          pub struct ToOdd;

// meaningless between neighbours; only make sense where one side is absent
pub struct ReduceModulo;    pub struct SubstituteZero;
pub struct Refuse;

pub const trait Quantisation {
    type UnderMidpoint: Direction;
    type OnMidpoint:    Direction;
    type OverMidpoint:  Direction;
    type OverRange:     Resolution;
    type UnderRange:    Resolution;
    type Fallibility<T>: notko::ConstTry<Output = T>;
}
```

This single vocabulary reproduces every named rounding and overflow mode in IEEE 754, SystemC (IEEE
1666) and MATLAB's Fixed-Point Designer as a row of the triple or a pair, with no gaps needing their own
name: a rounding mode is any (below, on, above) triple; an overflow mode is any (above-range,
below-range) pair. `Saturate` is `(TowardNegative, TowardPositive)` at the range members, which states
what saturation is rather than asserting it. The refusal this design is proudest of is real and checked:
`ReduceModulo` is a `Resolution` but not a `Direction`, so writing it at a midpoint position (where
there is no modulus to reduce by) fails to compile.

`Fallibility<T>` selects which tier of arvo's fallibility ladder an operation returns through: `Just<T>`
for a total quantisation, `notko::Outcome<T, _>` where `Refuse` can fire. This member is declared by
hand in the spec text as written; whether it should instead be computed is one of the open threads in
section 5.2, along with what actually delivers a refusal at runtime.

### 3.4 What is derived rather than declared

The design's central claim is that a set of mathematical facts about a composition (which number
system its values inhabit, whether its addition is associative, what kind of number it presents as) are
computed from the ten axes rather than hand-declared per type. This section states the mechanism as it
currently stands, folding in the panel's verified repairs.

**Number-system membership.** An empty fractional part with an unsigned domain gives the naturals; the
same signed gives the integers; a power-of-two quantum (`Adjustment = Unit`) with no bias gives the
dyadic rationals; any other exact quantum gives the rationals; a floating numeral adds the specials
(infinities, NaN) that inhabit no number system at all. The `Adjustment = Unit` check is an
associated-type equality, not a comparison, which keeps it inside the feature set this workspace
allows. It has one known gap (`FullRange<1>` reduces to exactly `2/1`, a power of the radix, so it is
secretly dyadic, and the equality check as stated misses it because it checks the type rather than the
value the type computes) that is not yet closed; see section 5.1.

**Algebraic laws.** A recovery rule (a member of `Resolution`) is *faithful* when it leaves an operation
the operation of a real algebraic structure on the representable set. The design's original wording for
this ("leaves the operation the operation of an algebraic structure") was shown to not actually cut
anything (any total recovery is technically some structure) and to disagree with itself on a concrete
case (unsigned clamping addition genuinely is the operation of a real structure, the truncated-addition
monoid, yet the design calls it unfaithful). The replacement, adopted through the panel review, is
**translation stability**: a recovery map `phi` is stable when `phi(phi(x) + c) == phi(x + c)` for every
exact sum `x` and every representable `c`, under Kleene equality for the refusing case (both sides
refuse, or both sides return and agree). This identity sorts every case correctly, including the ones
the original wording got wrong: modular reduction is stable under both signs (it is a congruence);
unsigned clamping is stable (monotone, and a nonnegative translation cannot cross the lower bound);
signed clamping is not (a value at 200 over a max of 127, translated by -100, disagrees depending on
which stage the clamp happens at); substituting zero on overflow is not stable, unsigned or signed, and
this specific case (a hand-verified counterexample: two maximum values summed, then a small increment,
against the same in the other grouping) is what first showed the original wording was actually false as
written, not merely imprecise. `Refuse` needs the map to be *partial* rather than total for this to be
well-formed at all (refusing is the absence of a returned value, not a value refusal maps to), and once
made partial, both refusing rows resolve correctly by the same identity: refusal is stable one-sided
(unsigned) by monotonicity, and unstable two-sided (signed), the concrete counterexample being that
`(127 + 1) + (-1)` refuses while `127 + (1 - 1)` returns 127, so the two groupings genuinely disagree.

The law itself is then a computed truth value rather than a hand-partitioned pair of blanket
implementations. The original two-implementation shape (one for the unsigned case, one bounded on
faithfulness for the signed case) hits a real ceiling: as soon as a third true fact needs stating (for
example, "wrapping addition folds regardless of signedness, because it is always the group operation of
a cyclic group"), the two original impls and the new one become three implementations, none more
specific than any other, and Rust's coherence check refuses all three as conflicting, with no escape
under this workspace's permitted features (`min_specialization` only orders a linear chain of
specificity; it does not help here, and full specialisation is forbidden outright). The fix that avoids
this ceiling entirely: each `Resolution` constructor states its own lemmas as associated truth-valued
members (does it stay stable one-sided, does it stay stable two-sided), a type-level fold combines them,
and one implementation conditions the algebraic structure on the fold's result being true. This
generalises cleanly: a new resolution constructor that omits its lemma members fails to compile at its
own definition site (`E0046`), so nothing can enter the design without answering the question, and the
law key also needs to exclude a biased numeral (folding a numeral whose `Bias` is not `Zero` does not
even close under addition, so the law's key includes an `AddClosed` condition gated on `Bias = Zero`)
and, per one of the panel's mathematical findings, needs a numeral-side condition too (rounded addition,
which any `Stored` floating numeral performs, is not associative regardless of overflow handling; the
current two-fact key omits this and would hand a floating composition a false `AddAssoc` by construction
unless the key is widened to include `ExponentForm`).

**The type-family markers.** `IntegerLike`, `FractionLike` and `FloatLike` derive from `ExponentForm`
via a macro-expanded integrality table (the equality-bound cure applies here too, since "fractional"
otherwise needs an inequality on a generic const parameter, which this workspace's permitted features do
not allow). `BoolLike` is dropped from the family entirely: `Bool` has no numeral and is not a `Number`,
and grouping it with the other four was never more than filing by proximity.

**The representable range.** Not a separate declared contract. It derives from `LogicalWidth`, `Sign`
and the quantum, all already carried by the numeral. What the earlier, now-retired separate contract was
protecting survives and gets sharper: the numeral's own range is an identity fact, the carrier's
physical range is a lowering fact, and they differ exactly when the stored width is doubled.

**`Deterministic`.** A blanket marker keyed on the whole composition, making a previously prose-only
qualification structural: the property holds for one composition, and two consumers on different
strategies not agreeing with each other was never a promise this design makes.

**`ConstantTime`.** A blanket marker that reports rather than requests: a consumer can check whether the
composition they picked happens to be constant time, cannot demand it, and an internals change adding a
data-dependent early exit for speed can silently withdraw the property from a composition that used to
have it. This is the accepted trade, matching arvo's standing rule that internal implementation is free
to be whatever is fastest. It is not in the same family as the algebraic properties above: those are
facts about a mathematical structure the type parameters name, and this is a claim about generated
machine code for a specific target, which lives entirely outside anything the type system can see and
can change without any source edit at all (a toolchain bump, a different target). It belongs with
`Deterministic` in a fourth, weaker category described in section 5.3, discharged only by a runtime
measurement in `mock/benches/`, never a type-level fact.

### 3.5 The presets

Pre-1.0, nothing depends on the presets' current meanings, so each of the four is redefined from what
its name states as intent rather than from what it happens to do today: `Hot` is as fast as possible,
`Cold` stores as small as possible, `Precise` is the most precise at the price of both storage and
compute, `Warm` is the compromise most consumers want by default.

| | `Hot` | `Cold` | `Warm` | `Precise` |
|---|---|---|---|---|
| in-range | truncate | nearest, ties to even | nearest, ties to even | nearest, ties to even |
| out-of-range | reduce modulo | clamp | clamp | refuse |
| growth | narrowed to operand | exact | exact | exact |
| stored width | minimum | minimum | doubled | doubled |
| widening | none | per operation | in container | per operation |
| layout | dense | bitpacked | dense | dense |

Three consequences follow and are worth stating plainly rather than leaving to discovery. Only `Hot`
folds (has a true `AddAssoc`) for signed values: clamping and refusing are both unfaithful under the
translation-stability identity, so the marker chosen for speed is the only one whose signed folds the
type system permits, reading backwards until it is remembered that wrapping is exactly the arithmetic of
the cyclic group `ℤ/2ⁿℤ` and every other resolution is a deviation from a group. `Cold` pays a compare
and select on every operation, deliberately: a bitpacked column that wraps silently corrupts a stored
value rather than an intermediate, and that price is accepted. `Precise` is fallible: its arithmetic
returns through the refusing branch of the fallibility projection and call sites unwrap, which is the
price of "most precise" stated in the type rather than in a comment.

Two of the four preset redefinitions are silent value changes under an unchanged spelling (`Warm` and
`Cold` move from wrap to clamp; `Hot`'s division rounding changes), and this is the shape of migration
this workspace's own discipline treats as acceptable exactly once, pre-1.0, provided it is a single
loud, audited event rather than a quiet one: the concrete-value assertions that currently pin the
shipped semantics are the only place the old behaviour is stated precisely, and flipping them, test by
test, in the same change that flips the implementation, is the audit obligation this redefinition
carries. At the one real downstream consumer measured (twenty-two of thirty-one `arvo`-typed call sites
in a sibling crate default to `Warm`), the redefinition happens to correct a documented claim the
consumer's own comments already made but the shipped behaviour did not honour, though this is a
coincidence of that particular consumer and not a general mitigation.

### 3.6 Conventions

Every established industry convention ships as an optional, off-by-default feature containing type
aliases over the abstraction and nothing else: `conv-ieee754`, `conv-systemc`, `conv-matlab`,
`conv-amd-vitis`, `conv-flocq`. arvo ships one structure and does not decide which vocabulary a consumer
already thinks in.

The design's own falsifiability test on this abstraction: if a convention's mode cannot be written as an
alias over the ten axes, the abstraction is not general enough. This has already found and closed one
real gap (MATLAB's slope-and-bias scaling was inexpressible until `Bias` was added as a member). As
written, the test only checks that the vocabulary has an available slot to fill, which is necessary but
not sufficient; section 5.1 covers what it misses and the fix that has not yet landed.

### 3.7 Crates

One crate per contract, plus the strategy crate holding only the four presets and nothing else it used
to carry (the container projection, the arithmetic dispatch, and everything else that had accumulated
under the `arvo-strategy` name over several prior rounds moves out).

| Crate | Holds |
|---|---|
| `arvo-numeral` | `Numeral`, `ExponentForm`, `Adjustment`, `Bias`, `Underflow`, and their markers |
| `arvo-policy` | `Policy`, `Quantisation`, `Resolution`, `Direction`, `Growth`, and their markers |
| `arvo-lowering` | `Lowering`, `StoredWidth`, `Widening`, `StorageLayout`, and their markers |
| `arvo-strategy` | `Hot`, `Cold`, `Warm`, `Precise`, and nothing else |
| `arvo-numeric` | `Number<N, S>`, the semantic aliases, the `conv-*` alias sets |
| `arvo-algebra-contracts` | the algebraic ladder and the law markers |

The algebra ladder is declared to the depth the mathematics goes, not to the depth arvo's own numerals
currently reach, on the reasoning that a vocabulary fixed by mathematics cannot be got wrong in a way
that later needs undoing. `Combine<Op>` is renamed `Magma<Op>`, the precise term for a set with a binary
operation and no law claimed, with laws attached as separate markers so a structure is a magma plus the
laws it happens to satisfy. Declaring an unused rung costs nothing and is not the same commitment as
implementing one: the obligation that every implementation be sketched and benched before it ships
attaches to implementations, which are claims about arvo, not to bare declarations, which are not.

### 3.8 What does not change, and what does

`UFixed<13, 3, Warm>`, `Uint<13>` and `Bits<13, Hot>` all still read exactly as themselves at the point
a consumer writes them; the composition each expands to is internal to the alias. This input-side
compatibility claim held up under direct testing.

What changes is behaviour, not spelling, and it is stated in 3.5 above rather than repeated here.

### 3.9 What has actually been measured

Very little of this design's cost was measured before the panel; most of what follows is new.

Compile time, controlled: an L0-level source touch (equivalent to touching `arvo-strategy` today)
recompiles twenty crates in roughly 5.2 to 5.9 seconds through this workspace's compiler cache wrapper,
5.1 to 5.3 seconds without it (the wrapper cannot help an incremental source-touch cache miss and costs
about 12 percent on exactly that operation, an unrelated finding worth an afternoon on its own).
Coherence checking (the pass that finds conflicting trait implementations) is 59 percent of the current
facade crate's own compile time and about a quarter of the current strategy crate's, driven by how hard
each individual overlap check is rather than by how many implementations exist.

Encoding choice for the per-width and per-exponent tables this design needs (an explicit impl-per-row
table versus a typestate projection): the table's coherence cost is quadratic in row count, the
projection's is flat, and at 512 rows the table costs roughly eleven times the projection in coherence
alone. In absolute terms both are negligible next to the facade's own baseline, so the choice between
them should be made on grounds of uniformity and diagnostic quality, not compile time.

Monomorphisation cost: roughly 5.2 milliseconds of compile time per distinct composition a consumer
actually uses, and zero additional symbols in the shipped binary across a 400-times sweep, because every
instantiation inlines away completely. A representative accumulation loop under this design's full
machinery (ten axes, the derived law, a witnessed classification, a computed fallibility carrier) was
measured at one machine instruction per element more than a hand-written baseline doing the identical
thing with none of that machinery. This is squarely the trade the workspace's compile-time policy exists
to license: pay compile time freely when it buys runtime or correctness, and here it buys the latter for
close to nothing.

Diagnostics: today, before any of the ten axes exist, the two aliases documented as the intended
consumer-facing spelling (`arvo::Fixed`, `arvo::Signed`) are used zero times in the one real downstream
consumer measured; the dominant spelling there is a different existing alias (`Uint<N, S>`), used at
more than twice the rate of the raw struct form. rustc expands type aliases in error messages, so an
alias's spelling survives at the point a failure is reported (the caret) only when the failing trait
carries a `#[diagnostic::on_unimplemented]` attribute; without it, or when the failure is one hop removed
(a where-clause on a derived implementation rather than the named trait itself), the message shows the
fully expanded composition, which for this design's compositions is long enough to spill into a
"long type written to file" note. In the specific environment the real downstream consumer already runs
in (the forbidden `generic_const_exprs` feature enabled there today), the render is worse again: an
anonymous const item with no visible width at all. Section 5.1 covers the proposed fix and what it costs
under the fused-versus-split parameter question.

Layout: a fallible return doubles the size of every intermediate value in flight, regardless of whether
the error payload carries information, because the discriminant needs somewhere to live; this holds
whether the value is two bytes or sixteen. Where rustc can see a spare, unused bit pattern in the value's
own representation, the refusal is free (zero additional bytes); the mechanism that would let this
design declare such a pattern explicitly is compiler-internal and closed off by this workspace's feature
rules, so the free case only happens when a preset's own storage choice happens to leave room, which is
true for exactly one preset (`Precise`, one of only two stored at doubled width) and false for the rest.
Section 5.2 covers this in full, since it decides how expensive a fallible `Precise` composition actually
is.

## 4. Proposed but not accepted

Three threads were explicitly kept open by op through the panel's mid-run checkpoints, each with a
specific instruction to keep iterating rather than to stop at the first working answer. None of the
three is settled. What follows is the current best answer on each, and what is still missing from it.

### 4.1 Thread A: the consumer surface

The question: given that this design's diagnostic story is currently just aliases, and that aliases only
survive at the point of failure under specific conditions (section 3.9), what is the best available
consumer-facing surface, not merely a surface that beats plain aliases.

Three candidate shapes were built and measured. Plain type aliases (the spec's current text) are cheap
and, as measured, genuinely broken in the failure case: the alias is destroyed the moment the failing
trait lacks the diagnostic attribute, or the failure sits one where-clause away from the named trait.
Concrete newtype faces (a `repr(transparent)` wrapper struct per family, sharing one internal
composition, with impls forwarded by macro) recover the spelling of the numeral half of a composition
(the fixed-point width and fractional bits) but not the policy or lowering half, which is the half a
consumer actually varies day to day; a face's own generic parameters still carry the axes structurally,
so the full five-axis-plus tail still renders. The strongest measured result is a third shape: nominal
constructors at every position a consumer selects, combined with small per-axis "modifier" types that
delegate every member except the one being changed. Under this shape, ten axes render for free in an
error message, provided every value a consumer can select is reached through a *named* type rather than
a raw structural parameter list, because rustc prints the type arguments a consumer applied but not the
associated types those arguments project to. A consumer who wants exactly one axis different from a
preset writes, for example, `IFixed<13, 3, OverRangeOf<Warm, Refuse>>`, and the resulting error names
exactly that composition, with the nine untouched axes absent from the message rather than spelled out.

This result carries three unresolved costs. It was measured under the fused two-parameter `Number<N, S>`
form; under the three-parameter split explored and later abandoned (section 3.1), the same technique
costs roughly 1.8 times more rendered length and truncates one modifier level sooner, so the diagnostic
win and the parameter-count question are coupled rather than independent. The computed-truth-value law
mechanism (section 3.4) needs its own small repair to stay legible under this shape: bounding a law on a
computed boolean produces an error naming `False` rather than naming the composition that failed, and the
fix (wrapping the verdict in a marker parameterised by the composition it is about, so the diagnostic
attribute has something to name) is a four-line addition, verified, not yet folded into the spec text.
And the modifier types do not canonicalise: two different orderings of the same set of changes
(`LayoutOf<OverRangeOf<Warm, Refuse>, Bitpacked>` against the same composition built the other way round)
are the same composition with different spellings and different rendered error text, which is unresolved
and matters for anything that compares error snapshots textually.

A finding outside the assigned question, ranked highest by more than one reviewer independently: a real
downstream consumer's actual, present-day code (four bounded-identifier newtypes wrapping arvo aliases,
used to address arrays) needs twenty separate lint-escape workarounds across four categories, none of
which this restructuring's ten axes touch at all, because they are about arithmetic and this gap is about
identifiers and indexing. The four categories: deriving a const-generic array size from a numeral's
width; converting a typed identifier to a raw array index and back in both directions; displaying the raw
index respecting the numeral's own fixed-point scale; and computing an addressable-count bound
(`2^width`) from the width, which arvo already knows internally as an associated constant but exposes in
a form the consumer cannot reach, so the consumer re-derives and hand-syncs it with a comment asking
future readers to keep it in sync by hand. Whether this belongs inside this restructuring (a design
already being written could cheaply add the missing surface) or in a separate topic entirely is
unresolved.

### 4.2 Thread B: fallible arithmetic

The question: `Precise` refuses out-of-range results by design, and op wants that kept. What is the best
possible shape for fallible arithmetic in a `no_std`, no-`alloc`, monomorphisation-only substrate, and
what does the best shape unlock rather than merely what the current shape costs.

A first finding corrected an over-broad early claim: one generic arithmetic function body *can* serve
both a total and a fallible composition without duplicating the body, provided the resolution rule
constructs its own answer (in whichever carrier type it needs) rather than the calling body constructing
a refusal generically. This matters because arvo cannot implement its own operator traits on a foreign
type such as `notko::Outcome` (Rust's orphan rule forbids it), so any design where the calling body tries
to build a refusal directly hits a wall the panel initially mistook for a fundamental limit rather than a
consequence of one specific shape.

With two range positions (over and under), the return type any composition needs is the join of the two
resolutions' own carrier choices, connected by a lift where they differ; this is the same shape effect
systems use for combining independently-installed handlers, and it scales cleanly to a future third or
fourth effect (a divide-by-zero refusal, say) without redesign.

The sharpest reframe of the thread: whether a refusal *arrives* as a checked sum type (returning through
`Just`/`Outcome`, per-operation control flow, short-circuiting), as an absorbing "bottom" value carried
inside the numeral's own spare bit pattern, or as an accumulated sticky flag read once at the end, is by
this design's own axis-sorting test a `Lowering`-level choice: the representable set and the mathematical
function computed are identical across all three, and only the cost and the shape of the call site
differ. Under this reading, `Precise` can keep refusing exactly as designed while a consumer separately
picks how the refusal travels. A refusing composition delivered as an absorbing bottom was run, unmodified,
through the existing generic graph-ranking algorithm crate and produced correct results, settling once at
the boundary rather than forcing every call site to unwrap.

This reframe carries three real costs, none yet resolved. It has a precondition nobody stated until it
was measured: the absorbing-bottom delivery only costs nothing (branchless, two extra machine
instructions over a plain saturating baseline) when the numeral's own storage happens to have spare,
unused bit-pattern room, which is only reliably true for `Precise` among the four presets; where there is
no spare room, the identical delivery mechanism costs eight times more instructions and doubles the
value's size anyway, because a companion flag has to be threaded alongside it instead. A delivery that
propagates a bottom through addition must also propagate it correctly through *selection* (a min or max
comparison), and a naive total-ordering comparison silently discards it, which is precisely the defect
IEEE 754-2008 shipped in its own min/max functions before the 2019 revision fixed it; at least one of
this design's own generic algorithm crates performs exactly this kind of selection today and would need a
propagating comparison contract, not a plain one, before the bottom delivery is safe to adopt there. And
under the sum-type delivery specifically, a refusing operation's short-circuit is measurably not constant
time (two data-dependent branch exits per element in the compiled code, against none for the bottom
delivery), which means the `ConstantTime` derived marker (section 3.4) is currently keyed on data that
does not decide it: delivery decides it, and delivery is not one of the ten axes.

A related, structurally converging finding from three independent directions (layout cost, generated
code shape, and Rust's own orphan rule) is that whichever delivery mechanism is chosen, the carrier
holding a refusal should be arvo's own sealed type, with a single `settle()`/`observe()` accessor as its
only door; a prototype built during the review exposes its bottom-carrier's fields publicly today, which
defeats every guarantee the surrounding machinery establishes the moment a consumer reads the raw field
instead of going through the accessor.

A fourth, distinct dissolution of the exile question was proposed and not tested against a real
algorithm crate: a locally-installed handler at the call site, selected by a turbofish and free under
monomorphisation, letting a consumer accumulate under one policy and settle once at a chosen boundary,
independent of both the ambient type-level policy and the delivery question above.

Growth interacts with delivery in a way nothing currently addresses: under a `Narrowed` (rather than
`Exact`) growth policy, a single operation can produce two refusal opportunities, one when the wide exact
intermediate is narrowed and a second on the final result, and none of the carrier-join machinery
described above has been extended to a two-site case.

Finally, the spec's currently-declared `Fallibility<T>` member (a hand-written associated type,
section 3.3) is a plausible candidate for replacement by a computed shape: fallibility as a *grade*
(a two-point lattice, no-refusal below refusal), computed once as the join over an operation's actual
firing sites and consumed through one blanket implementation, so every arithmetic function carries a
single bound rather than restating the join by hand. This dissolves the hand-declared version's "can lie"
problem and removes a soundness dependency the current shape has on a fact about `notko` that arvo does
not own and has never pinned as a compile-fail test (specifically, that a particular fallibility-crate
type does not implement a certain conversion; if `notko` ever added that conversion for its own reasons,
arvo's soundness argument here would silently stop holding). This computed shape is not yet reflected in
the spec text, and, per Thread C below, its own leaf-level checking turned out to have a serious gap of
its own.

### 4.3 Thread C: leaf truth, and whether the check can be the typestate

The question, in op's own framing: the type machinery in this design delivers totality (every case must
be answered) and coherence (no two answers may contradict), but never the *truth* of a leaf mathematical
fact someone hand-typed, such as "this recovery rule is translation-stable." Op asked for a shape where
checking a leaf fact against reality *is* the typestate, rather than a mechanism bolted alongside it.

This thread went through several shapes in sequence, each repaired or overturned by whoever compiled it
next, and the current state is the fourth. The first proposal was bounded, compile-time brute-force
checking: since every leaf fact in question (associativity, translation stability, membership) is, once
a width is fixed, a statement in ordinary bounded integer arithmetic, a small `const` block can check it
exhaustively at build time against a hand-written reference function, with no solver dependency and no
new unstable feature. This was sketched but not built.

The second built it, and went further: the recovery rule itself becomes a `[const]` trait method,
generic over a small integer model, and a generic const function checks the translation-stability
identity through the trait bound rather than against a hand-copied restatement. The enforcement runs at
two points, an eager check inside the crate that declares each rule (fires early, names the constructor
directly) and a direct check inlined in the single generic entry point every arithmetic operation passes
through (fires at monomorphisation, cannot be silenced by an implementor overriding a default). This was
presented, and accepted by three subsequent reviewers in a row, as the answer to Thread C: the checked
function was believed to be the same function the runtime arithmetic actually called.

The third pass found a real gap in that shape before accepting it further: the checked recovery function
was declared *total* (it always returns a value), and `Refuse` cannot be expressed by a total function,
since refusing is precisely the absence of a returned value. The fix, verified, is to make the function
partial and compare under Kleene equality, which mechanically reproduces the entire hand-derived
classification table from section 3.4, including both refusal rows.

The fourth pass, compiling the checking machinery and the actual arithmetic pipeline together for the
first time in the review's history, found the far larger gap the second pass's claim had actually missed
entirely: the checked function and the function the runtime pipeline executed were two independently
authored pieces of code that never touched each other at any point. The witness verified only that a
declared classification tag agreed with its own private copy of the recovery rule; the real arithmetic
pipeline's own refusal-handling code called an unrelated, hand-written delivery function that never
consulted the checked rule at all. Under this construction, a composition declared and verified as
wrapping (`Hot`) silently *clamped* instead at runtime, passing every check the design performed, because
nothing in the design had ever asked whether the two definitions agreed. This is the single sharpest
finding produced anywhere in this review: totality, coherence, and a compiled, passing exhaustive witness
can all hold at once while the code that actually runs disagrees with what was proven, because what was
proven was never connected to what runs.

The current, fifth-pass repair, the most recent and most load-bearing state of this thread, closes that
gap by making the checked function and the executed function the *same text*, monomorphised twice rather
than authored twice: the recovery rule becomes one `[const]` generic function, parameterised over the
numeric payload representation, instantiated once at a small model width (where the const evaluator
checks it exhaustively at compile time) and again at a composition's real width (where it is the literal
code that runs). A second, smaller obligation is checked alongside it at the model width, confirming that
the whole surrounding pipeline (widen, apply the rule, narrow back to the numeral) agrees with the rule
itself under Kleene equality, which catches the one authored surface the single-definition move does not
already make unwritable: the carrier implementation. Verified: the earlier disconnection case now
reproduces the correct answer at real width; a carrier written to deliberately lie is refused at compile
time; and, measured through generated machine code rather than argued, the checked reference path and a
hand-written baseline for the wrapping case are not merely equally fast, the compiler proved them
identical and emitted the same machine-code symbol for both.

This shape has not been wired end to end with a separate, independently verified closure for the
Thread B/Policy-versus-Lowering question (section 5.2 covers that closure); has not been tried against
arvo's real storage representation (`Bits<N, S>`-shaped values), only against a small integer model; and,
given that this is the fourth shape in a sequence where each of the prior three had a hole the next
reviewer found only by actually compiling the previous one, should be read with the same suspicion rather
than as settled. What it currently establishes, precisely, and what remains trusted rather than checked,
is recorded as a ledger in section 5.4.

## 5. Open, broken, or unanswered

### 5.1 Mathematical and derivation gaps not yet repaired in the spec text

No axis names the radix. Every formula in the design (the quantum, the affine value map, `FullRange`'s
definition) is written in terms of a radix no axis carries, and the design's own proof case for its
identity/policy/lowering split (that IEEE 754 specifies two distinct bit encodings for one decimal
format) is a radix-ten example the current axis set cannot express and, separately, arvo has no
executable arithmetic for at any radix other than two. Either the radix is fixed at two and the decimal
proof case is demoted to an analogy, or an eleventh identity axis is added; the middle position (formulas
written generically over a radix nothing carries) is the one position that cannot be right, and it is
where the design currently sits.

`Stored<BITS, U>` does not determine a real IEEE format, and the significand derivation is off by one
against real hardware. "The significand derives by subtracting the exponent field and the sign bit" gives
23 bits for a 32-bit exponent field of 8, where real binary32 precision is 24; the missing bit is the
hidden leading bit, a normalisation convention that exists only under IEEE's own convention and is not
named by any of the ten axes. Two further conventions are silently inherited and unnamed: the reserved
exponent codes that produce infinities, NaN, subnormals and zero (the spec gestures at this with no
mechanism), and the exponent's own encoding bias, which collides in name with the unrelated
`Numeral::Bias` affine-origin member and should be renamed before any IEEE convention alias set ships.
The design's headline claim, that fixed point and floating point differ only in where the exponent lives,
is true of Flocq's idealised value sets and false as a claim about the formats real hardware ships.

Related to the above: arvo's floating-point types are sealed wrappers around hardware `f32`/`f64`, with
LLVM fast-math flags as the only lever, and arvo does not (and per its own design principles should not)
implement software floating-point arithmetic. So the fully general `Stored<BITS, U>` axis is at once
inexpressive of real shipped formats (the paragraph above) and has no operative caller beyond the two
hardware widths. Whether to ship the general parameterised form now, or ship only the two-point
operational axis actually used and park the general form as an unbuilt promissory note until a genuine
storage-format consumer (a half-float or UNORM column in a renderer, plausible but currently unscheduled)
needs it, is unresolved.

`FullRange<F>`'s boundary is unguarded. `FullRange<1>` reduces to exactly `2/1`, a power of the radix, so
it is secretly dyadic, and the membership check (an equality on the `Adjustment` type name) misses this
true membership because it never inspects the value the constructor computes. `FullRange<0>` divides by
zero and must be made unconstructable. An earlier, unpublished stage of this design carried an explicit
`F >= 2` bound on this constructor for exactly this reason; the published spec text dropped it.

The affine value formula is stated as if one map covers a whole numeral, which holds for `Implicit`
(fixed-point) numerals but not `Stored` (floating) ones, where the exponent varies per value and the
representable set is really a union of lattices, one per exponent, with real consequences (the same
value has more than one representation, and the problem worsens for any radix above two) that none of
the derivations quantifying over "the stored integer" (membership, range, closure) currently account for.

The spec never states whether quantisation fires per operation (each binary operation returns a fresh,
already-quantised value) or is deferred (compute wide, narrow only on the eventual store). The two
readings give different associativity answers for the identical composition: a signed clamping fold is
associative if the clamp happens once at the very end and is not if it happens after every intermediate
step, which is exactly the fact the whole law-derivation machinery exists to police. This is unresolved,
and whichever choice wins changes one already-stated consequence (that `Cold` pays a compare-and-select
"on every store" needs to read "on every operation" under the per-operation reading).

`Exact` growth has no coherent meaning for division: the exact quotient of two representable values is
generally not itself expressible at any finite width (a third has no terminating fixed-point form), so
division always quantises regardless of the growth setting, and what `Exact` should mean specifically for
division (most likely: quantise once, directly, with no intermediate narrowing at all) is undefined.

The one shipped `Monotone` law implementation only covers the "nearest, with some tie rule" family of
rounding rows; the four constant-direction rows (always toward zero, always toward positive infinity, and
so on) are also monotone by the same reasoning and have no implementation, which is exactly the kind of
gap this workspace's own edge-case discipline exists to catch and record as a red test rather than leave
silent.

Stochastic rounding is excluded by construction (the quantisation triple is a deterministic function of
position; a stateful resolution would break the zero-sized-marker const model this whole design rests on)
and the exclusion is nowhere recorded in the spec text. On the credit side of the same axis, round-to-odd
*is* expressible under the current vocabulary and is the field's own classical cure for double rounding,
which the still-unwritten multiplication work under `Narrowed` growth will meet immediately; this is a
real strength worth keeping visible.

The boundary between "in range" and "out of range" needs to sit at the last representable midpoint after
rounding on the numeral's own lattice extended past its stated bound, not at the raw numeral maximum, to
match how IEEE 754, SystemC and MATLAB all actually behave: a value just past the maximum, close enough
to round to it, quietly rounds *to* the maximum under every one of those standards rather than triggering
overflow handling. Without this fix, the convention-alias adequacy test (below) is checking spelling
rather than arithmetic.

The convention adequacy test itself ("if a convention's mode cannot be written as an alias, the
abstraction is not general enough") currently only checks that the vocabulary has an available slot to
fill, which found the one real gap it has found so far (MATLAB's bias) but says nothing about whether the
alias, once it type-checks, computes the same numbers the vendor's own format computes at the boundary
case above. The stronger, not-yet-written version checks each alias against a small set of already-
published vendor reference values (the last representable midpoint before the maximum, the first
over-range midpoint, a tie exactly at the maximum, one subnormal case, one specials case); this is cheap
specifically because the correct answers are already published and need no derivation, and the same table
would double as the porting document for a consumer arriving from any of the aliased conventions.

A mechanical gap in the convention design: Cargo unifies feature flags across an entire dependency graph,
so if two crates in one build each enable a different convention (say `conv-matlab` and `conv-ieee754`)
and both define a flat, non-namespaced name such as `Nearest`, the build breaks for a consumer who only
asked for one of the two. The alias sets need to live in per-convention modules rather than as flat
re-exports; this is not yet in the spec text.

### 5.2 Structural and mechanism gaps

Whether a mechanism exists that actually prevents a law from being conditioned on a `Lowering` member was
tested directly, since the fused-versus-split parameter question (section 3.1) was originally proposed on
exactly this promise. The finding: a crate boundary matching the current crate table (section 3.7) does
make the *derivation itself* provably independent of `Lowering` (the symbol simply has no referent in a
crate with no dependency edge to the lowering crate), and Rust's ordinary orphan rule already prevents
any unrelated third-party crate from injecting a conflicting law. Neither of those was ever the hard
part. The hard part, verified directly: the one crate that legitimately has to own the physically real
`Number` type (because that type's own definition needs `Lowering` in scope, to determine its real byte
layout) can still write a law implementation conditioned on a `Lowering` member, and no crate-boundary
shape prevents this, because the crate capable of writing that implementation at all is, by construction,
a crate where `Lowering` has methods a where-clause can name. A shape that closes this completely does
exist and was verified in isolation: prove the law about a distinct, purely phantom type
(`LogicalNumber<N, P, L>`) whose own definition places no bound on `L` at all, so no where-clause
anywhere, honest or not, has anything to condition on. This closes the gap independent of whether any
crate split exists at all. Its cost, not yet paid anywhere: the type a law is proven about becomes
distinct from the type that actually holds bytes and depends on `Lowering` for its layout, and connecting
the two (so that the physically real `Number<13, 3, Warm>` is provably the same type the law was proven
about, with no forwarding step that could reopen the gap) is a real design exercise nobody has completed.
A macro-based attempt to route around this failed for a general, structural reason: Rust does not imply a
struct's own bounds into every implementation that targets it, so any forwarding site, wherever it lives,
is unavoidably a place where `Lowering` must be nameable, which rules out an entire family of attempted
shortcuts before anyone spends further time on them.

Growth's interaction with delivery (a `Narrowed` intermediate can produce two refusal sites in one
operation, once on narrowing and again on the result) is unaddressed by any of the carrier-join machinery
built so far, which only ever modelled a single refusal site per operation.

The prototype fallible carrier built during the review exposes its internal fields publicly. Any code
reading the raw field bypasses every guarantee the surrounding checking machinery establishes; this needs
to become a sealed, arvo-owned type with a single accessor as its only door, which independently is also
where three separate arguments (byte layout, generated code shape, and the orphan rule) already converge.

arvo's current fallibility soundness argument, in one of the proposed shapes, rests on a fact about
`notko` (that a specific conversion is not implemented for `notko::Just`) that arvo does not own and has
never pinned as a compile-fail test; if `notko` ever adds that conversion for reasons internal to its own
design, the argument would silently stop holding with nothing anywhere to notice. (The computed-grade
shape from section 4.2 makes this dependency disappear structurally rather than needing a pin; which
shape ships decides whether the pin is still owed.)

Multiplication, and the distributivity law pairing addition and multiplication into a ring, are entirely
untested. Every worked proof in this whole design round, including everything the panel checked, covers
addition only. Rounding interacts with multiplication's associativity in a way it does not with
addition's, and this is expected to be where the work first gets genuinely hard.

Whether one generic arithmetic function body can serve every strategy at once (an explicit obligation the
spec itself names) remains the least-tested corner of the whole design; the deepest nested where-clauses
and the most axis-projection layers measured anywhere in this review occur exactly here, and no full
worked example against the real design's shape (rather than a small model) exists yet.

Whether the numeral's own type should be a small, closed, nominal constructor set (best for the
diagnostics work in section 4.1) or must stay open to arbitrary structural composition (needed only if a
future convention needs a numeral shape none of the closed set can express) is unresolved. The working
assumption behind the diagnostics proposal, that a closed set is safe because conventions only ever add
alias sets and never a genuinely new numeral shape, is stated as an assumption and has not been checked
against every convention this design intends to support.

### 5.3 Design questions never closed

What the composition type is actually called, and its exact parameter order and defaults, separate from
the fused-versus-split question in section 3.1.

What a preset (`Hot`, `Warm`, `Cold`, `Precise`) is mechanically: a plain type alias over one fixed
composition, as the spec's own text currently implies, or a nominal marker type from which axes are
projected. The diagnostic and modifier work in section 4.1 needs the second reading (a modifier has to
delegate *from* something), but this has not been formally decided against the first.

What `arvo-numeric` ends up containing once the numeral, policy and lowering definitions themselves move
out to their own crates: whether it becomes a pure semantic-alias-and-convention crate, and what, if
anything, it needs to declare beyond `Number<N, S>` itself.

Whether `arvo-num-systems` (a crate decided in the round immediately before this one, defining membership
of the naturals, integers, and so on through algebraic structure) now depends on this design's format
concept or the reverse, given that the format concept fully absorbed what an earlier, now-superseded
proposal called "format." This has not been re-examined since the three-contract shape replaced that
earlier proposal.

### 5.4 What the design currently proves, stated as a ledger, and what remains trusted

Once the section 4.3 repair is treated as landed, every claim in this design sorts into one of four bins,
and the value of the whole apparatus is measured by how small and how explicit the trusted bin stays.

Machine-checked, by construction, meaning a violation is not merely wrong but does not compile at all: a
value substituted on a refusal path where the refusal constructor takes no payload argument to substitute
with; a law reading a `Lowering` member, under the phantom-type closure described in section 5.2; a
classification member omitted by a new resolution constructor; two contradictory law claims for the same
composition.

Machine-checked by bounded exhaustion at a small model width, meaning a violation fails the build rather
than failing to type-check: a declared classification checked against the recovery rule; the executed
arithmetic checked against the same recovery rule (this is the row whose earlier absence was the
Thread C finding); the fallibility grade checked against the rule's own actual refusal behaviour.

Trusted, named explicitly, with nothing beneath it left to check against. The five hand-written recovery-
rule bodies themselves, roughly twenty-five lines total: there is nothing to derive them from, and
careful hand review, already done once during this process, is the only available check. The statement of
the checked identities themselves (translation stability, Kleene equality, the preservation equation),
roughly thirty lines: a wrong statement of a theorem certifies the wrong thing everywhere it is used, and
this is every verified system's irreducible core, not specific to this design. The observation and settle
functions, which define the perimeter every guarantee in the design lives inside; anything that exits a
carrier by another route (the prototype's public fields, noted above) is a hole in every theorem the
carrier is meant to carry, not a style objection. The per-width primitive storage operations (widening,
wide addition, comparison, narrowing), which are small, enumerable, and already the best-tested code in
the tree via the existing suite, but are genuinely untouched by any of this review's new checking
machinery. The width-uniformity transfer argument itself, that a fact checked exhaustively at three or
four bits holds at every width arvo actually supports: this stays prose forever, is never mechanical, and
is backed by exactly one runtime guard (a panic naming what it guards, should the argument ever be
wrong). The workspace's own standing bans on trait specialisation and on `TypeId`: this design's
transfer argument above depends on every checked function being unable to ask "which width am I running
at," which those two bans are what actually guarantee; this dependency is real and was not previously
written down anywhere, including in the rule that states the bans themselves. And, unavoidably, that
rustc's compile-time evaluator and its code-generation backend agree with each other and are each
individually correct, which is outside this design's perimeter entirely and always will be.

Validated per artifact, a bin expected to grow over time: every future hand-optimised arithmetic
implementation (inline assembly, target-specific intrinsics), which the compile-time checking machinery
described above cannot see at all, since none of it is evaluable at compile time. Each one needs its own
validation test (exhaustive at small widths, sampled at large) shipped in the same change that adds it.
Zero such implementations exist on this surface today.

Promised, dischargeable only by runtime measurement, never a type-level fact at all: `Deterministic` and
`ConstantTime`. Already confirmed broken in one concrete direction: a refusing composition delivered as a
checked sum type cannot honestly claim `ConstantTime`, on any axis, because delivery decides it and
delivery is not one of the ten axes at all.

One boundary worth stating precisely, because it bounds how far the whole checking apparatus can ever
reach: it cannot be stated, in this language, as a type-level theorem that "the executed code always
agrees with the specification for every input", because that quantifies over values in a type, which is
dependent typing and Rust does not have it. What can be built, and is what section 4.3's current shape
does build, is a structural argument (one definition serves as both the checked and the executed code)
backed by a bounded exhaustive check at a small representative width, plus a stated, unmechanised argument
that the small width's behaviour transfers to every real width. That combination is real, useful, and
should never be described in stronger terms than this.
