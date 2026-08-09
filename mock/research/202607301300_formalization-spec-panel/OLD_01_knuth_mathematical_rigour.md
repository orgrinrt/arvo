# Panel 01: mathematical rigour

**Persona:** Donald Knuth, mathematical rigour lens. First member; no prior panel files existed.
**Date:** 2026-07-30

**What I read in full:** the spec (`202607301200_topic.the-formalization-spec.md`), the talk
(`202607301100_topic.the-formalization-talk.md`, all 1848 lines), both sketch FINDINGS
(`202607300500_format-as-exponent-function/`, `202607300600_law-markers-derived-over-composition/`),
the panel brief, and the governing panel rule. **What I read in part:** the inherited-state file
(decision definitions D16, D36, D38, D45, D47-D52 located and read in place), and source at the lines
the talk cites (`arvo/src/ufixed.rs:35`, `arvo/src/float.rs:30-41`, `arvo/src/markers.rs:55-62`,
`arvo-strategy/src/axes.rs:85-116`, `arvo-strategy/src/arith.rs:80-90`). **Gates:** the full suite ran
green, 654 passed, 0 failed, 122 binaries. I read the bodies of the tests on the touched surface;
`arvo/tests/identity_laws.rs` is a genuine whole-matrix law suite with the compile-fail refusals
pinned under `tests/ui/`, and `fixed_point_mul.rs:174` / `fixed_point_div.rs:38` honestly pin the
mul-floor against div-trunc asymmetry the talk reports. No tautological tests found on this surface.
The brief's factual claims about current source checked out everywhere I probed; I found no false
premise, and I proceed.

One caution on the green: per the workspace's own rule, this suite is green because it describes the
shipped design, and the spec changes that design. Nothing below is contradicted by the suite; nothing
below is confirmed by it either.

I have separated what I verified (hand computation, source read, or checked against a standard's
stated definition) from what I reasoned about. Each finding says which it is.

---

## 1. The unsigned associativity blanket impl is false. Verified by hand.

The spec's strongest claim is the faithfulness derivation, and its unsigned half is wrong as written.

Spec lines 210-213:

```rust
// unsigned addition can only leave the range above, so one end is
// unreachable and the rule is truncated addition whatever it does there
impl<A: Resolution, B: Resolution> AddAssoc for ((A, B), Unsigned) {}
```

Counterexample, worked at width 8, so `M = 255`, with `OverRange = SubstituteZero` (SystemC's
`SC_SAT_ZERO`, which the spec's own vocabulary carries at line 141):

```
a = 255, b = 255, c = 1

(a (+) b) (+) c :  255 + 255 = 510 > 255  ->  0;    0 + 1  = 1        ->  1
a (+) (b (+) c) :  255 + 1   = 256 > 255  ->  0;    255 + 0 = 255     ->  255
```

`1 != 255`. Unsigned addition under substitute-to-zero is not associative, and the impl quantifies
over every `Resolution` at the reachable end. The prose justification ("the rule is truncated
addition whatever it does there") is right about the unreachable *lower* end and silently wrong about
the reachable upper one: it assumes any resolution at the top preserves associativity, which holds
for modular reduction, for clamping, and for refusal, and fails for substitution.

For completeness I checked the other unsigned cases by hand, because the blanket impl is *right*
about them and that is worth knowing:

| OverRange resolution | unsigned addition associative? | why |
|---|---|---|
| `ReduceModulo` | yes | congruence of `+` mod `2^N` |
| clamp (`TowardNegative`) | yes | `min(x + y, M)` is the truncated-addition monoid; monotone, operands nonnegative |
| `Refuse` | yes, under Kleene equality | operands nonnegative, so `a + b <= a + b + c`; both groupings are defined exactly when the total fits |
| `SubstituteZero` | **no** | the counterexample above |

Note the `Refuse` row needs a definition the spec does not give: what associativity *means* for a
fallible operation. Kleene equality (both sides refuse, or both return and agree) makes the row true
and makes the spec's signed-trap counterexample at line 220 well-posed. Without a stated equality for
the fallible case, `AddAssoc` is not a well-formed predicate over compositions whose `Fallibility` is
`Outcome`. State it.

## 2. The law markers are keyed on the wrong index set. Verified against the spec's own text.

Both blanket impls (spec 212, 215) condition on `((A, B), Signedness)` and on nothing else. But the
spec elsewhere establishes two facts that make that key too coarse:

First, floating addition rounds, and rounded addition is not associative regardless of overflow
resolution. The talk knows this in its best section: fast-math reassociation is unsafe *because* IEEE
addition is not associative, `StrictFloat` gets no `AddAssoc`, and `FastFloat`'s becomes an
`unsafe impl` promise (talk 1300-1314). Yet the derived impls as written would hand a `Stored`
numeral with wrapping unsigned semantics a safe `AddAssoc` by blanket. The two parts of the spec
contradict each other. The impls need a numeral-side condition, at minimum `ExponentForm = Implicit`
(uniform spacing makes same-format addition exact, so only the range ends act), or the key becomes
the whole composition as D51's original wording had it.

Second, a biased numeral's addition does not even close (spec 963-972 region of the talk, and D68's
own `AddClosed`), so `AddAssoc` over a biased numeral is a statement about an operation that does not
exist. The key must exclude `Bias != Zero`, or `AddAssoc` must be bounded on `AddClosed`.

This is the brief's "is an assertion hiding in a constructor" question answered in the affirmative,
in an unexpected place: the derivation is real, but its index set silently encodes the assumption
"fixed point, unbiased", which is an assertion about the numeral that appears nowhere in the bound.

## 3. "Faithful" is not well-formed as defined. Reasoned, with a proposed repair.

Spec 204-208 defines the load-bearing property: a recovery is faithful "when it leaves the operation
the operation of an algebraic structure on the representable set", with modular reduction in and
clamping, substituting, refusing out.

Taken literally this is vacuous. Any total recovery leaves the operation a closed binary operation on
the representable set, and a set with a closed binary operation is a magma, which is an algebraic
structure. So everything total is faithful and the definition does not cut.

Taken as intended (the structure, presumably the group `Z/2^N Z`), it cuts too hard, and finding 1
shows the cost: unsigned clamping addition IS the operation of an algebraic structure on the
representable set, namely the commutative monoid `([0, M], min(x + y, M))`, with identity 0. It is a
perfectly respectable structure (it is the truncated-addition monoid, the finite cousin of the
tropical semiring's additive part). By the definition as written, unsigned clamping is faithful. The
spec says it is not. The spec's *conclusions* about which compositions fold are then reached partly
by the definition and partly by the separate unsigned blanket impl, and the seam between them is
exactly where the `SubstituteZero` error slipped in.

The property that actually does the work is one identity, and I propose it as the replacement. Let
`phi` be the recovery map from exact results onto the representable set (for the total resolutions;
`Refuse` under Kleene equality). Then the recovered operation is associative if and only if

```
phi(phi(x) + c) = phi(x + c)      for every exact sum x and representable c
```

which says `phi` absorbs its own application under translation. Call it translation stability. It is
the special case of "laws transfer along a congruence-compatible retraction", and it sorts every case
above correctly, including the ones the current framing gets wrong:

| recovery, domain | stable? | witness |
|---|---|---|
| mod `2^N`, either signedness | yes | congruence: `phi(x + c)` depends only on `x mod 2^N` |
| clamp, unsigned | yes | monotone, and translations by `c >= 0` cannot cross the lower bound |
| clamp, signed | no | `x = 200` over `M = 127`, `c = -100`: `phi(27) = 27`, `phi(100) = 100` |
| substitute zero, unsigned | no | finding 1's counterexample restated: `phi(510) = 0`, then `0 + 1`, against `phi(511)` |
| refuse, unsigned | yes (Kleene) | definedness of both groupings coincides by monotonicity |
| refuse, signed | no (Kleene) | the spec's own `(127 + 1) + (-1)` example |

Each row is a two-line lemma, checkable per `(Resolution, Signedness)` pair, and the two blanket
impls become one impl bounded on a `TranslationStable` marker whose instances carry those lemmas.
That is a genuinely derived shape in D16's sense, where the current "Faithful = ReduceModulo" is a
one-row table wearing a definition's clothing.

A second reading, for the panel to weigh rather than for me to rule on: keep "faithful" but define it
as "phi is a homomorphism from `(Z, +)`", which picks out modular reduction uniquely and cleanly (it
is the quotient by a congruence, and laws transfer along surjective homomorphisms, which is why wrap
preserves associativity, commutativity, and distributivity all at once). Then unsigned clamping's
associativity is derived from a *different* property (monotone retraction plus one-sided domain), and
the two properties together replace the current pair of impls. This costs one more marker and states
more mathematics; the single translation-stability identity is more economical but proves only
associativity, so multiplication and distributivity (D2, open) will need it re-instantiated per
operation. Either way, the current definition should not survive as written.

## 4. The in-range/out-of-range boundary is drawn in the wrong place, or in no place. Verified against IEEE 754's overflow definition.

The five situations (spec 129-134) classify an exact value as between neighbours, on a midpoint, or
"past either end" of the range. The spec never says where "past the end" begins, and the natural
reading, past the largest representable value MAX, disagrees with every standard the conventions
would alias.

IEEE 754 detects overflow *after* rounding as if the exponent range were unbounded: an exact result
in `(MAX, MAX + q/2)` under round-to-nearest is not an overflow, it rounds to MAX. SystemC and MATLAB
behave the same way for fixed point, because the quantisation to the lattice happens before the range
check. Under the spec's carving read literally, such a value is "past the top of the range" and the
`OverRange` resolution fires. The difference is observable and large:

- `Precise` (`OverRange = Refuse`) would refuse results every standard quietly rounds to MAX.
- `Hot` (`OverRange = ReduceModulo`) would wrap a value at `MAX + 0.3q` to near the bottom of the
  range where every standard returns MAX.

The repair is standard and cheap to state: the map is round-first on the extended lattice (the
numeral's lattice continued past the range), then resolve the range. Under that composition the five
situations are exact, the boundary sits at the last midpoint, and the aliases in `conv-ieee754` and
`conv-systemc` mean what their vendors mean. Without it, D67's falsifiability test is being run on
spelling rather than on semantics: an alias can *name* `roundTiesToEven` and still not compute what
IEEE computes. I recommend D67's test be restated as behavioural (the alias reproduces the vendor's
result on the boundary cases: last midpoint, first over-range midpoint, tie at MAX), because those are
precisely the inputs where a nominally correct alias set goes wrong.

## 5. The Direction/Resolution subtyping admits undefined points in one direction while refusing them in the other. Reasoned from the spec's own vocabulary.

The hierarchy `Direction: Resolution` (spec 137-147) encodes "everything usable between neighbours is
usable at the range ends". The refusal the spec celebrates, `ReduceModulo` at a midpoint position
failing to compile, guards the converse and is correct. But the encoded direction is the false half:

- `TowardPositive` at `OverRange` names a neighbour that does not exist. So do `AwayFromZero`, and
  arguably `ToEven` / `ToOdd` (one neighbour; "the even one" degenerates to clamp or to nothing
  depending on parity of MAX).
- The spec's line 163-164 claims "the combinations no standard names are the non-monotone ones".
  False: `(_, _, _, TowardPositive, _)` is not non-monotone, it is undefined. Non-monotone points are
  strange but computable; these have no value to return.

Two repairs, both compatible with the derived-law machinery. Per-position capability traits
(`UsableAtOverRange`, implemented by `TowardNegative`, `TowardZero`, `ReduceModulo`,
`SubstituteZero`, `Refuse`; the mirror set at `UnderRange`), which keeps the shared vocabulary and
refuses the undefined points exactly as `ReduceModulo`-at-midpoint is refused today. Or define the
semantics of an off-grid direction as clamping to the one existing neighbour, which makes every point
total but makes four spellings of saturation, and the law derivation must then treat them as equal,
which is more aliasing than the vocabulary needs. I lean to the first but state both per the panel
rule.

## 6. `Fallibility` is asserted, violating the spec's own D16 discipline. Verified against the spec text; repair reasoned.

Spec 149-157 puts `type Fallibility<T>: ConstTry<Output = T>` on `Quantisation` as a declared member,
while spec 190-193 states the governing rule: a derived property cannot lie, an asserted one is a
promise. `Fallibility` as declared can lie in both directions: a quantisation carrying `Refuse` at
`OverRange` can declare `Just<T>` (unsound, the refusal has no channel), and one with no `Refuse`
anywhere can declare `Outcome<T, _>` (sound but false, every call site unwraps for nothing).

It is derivable with the machinery already in the design, no negative reasoning required: put
`type CanRefuse: TruthMarker` on `Resolution` (`Refuse` alone says yes), fold the five members with a
type-level or (a four-impl table on marker pairs, the same shape as the law tuples), and let
`Fallibility` be a projection of the fold. Then `Precise` is fallible *because* its `OverRange` is
`Refuse`, in the type system rather than beside it. Given that the spec's one-line summary of its own
contribution is "derived rather than declared", this member should not be the exception.

## 7. Radix is used everywhere and declared nowhere. Verified against the spec text.

Every identity formula is radix-parameterised: the quantum is `Adjustment * radix^exponent`
(spec 105-109), `FullRange<F>` is `radix^F / (radix^F - 1)` (spec 107), the value map is
`Adjustment * radix^exponent * k + Bias` (spec 112). No axis carries the radix; the ten-axis table
(talk 1627-1636) has no row for it. Meanwhile D58's proof case for the numeral/lowering split is
decimal64 (spec 73-75), a radix-10 format the ten axes cannot express at all.

Nothing is unsound here, but one of two things should be said out loud. Either the radix is fixed at
two, in which case write it into `Numeral`'s contract as a stated constant and demote the decimal64
citation to an analogy (it currently reads as an expressibility claim, and D67's test would fail it),
or the radix is an eleventh identity axis, which D54's test supports (changing it changes the
representable set) and which `conv-flocq` would eventually want anyway, Flocq being radix-generic. I
do not rule; I note that the middle position, formulas in terms of a radix no type carries, is the
one position that cannot be right.

## 8. `Stored<BITS, U>` does not determine an IEEE format, and the precision derivation is off by one. Verified against the binary32 layout.

Spec 81-96 and 117-119: precision and minimum exponent "derive from the exponent field's width", and
the significand "derives by subtracting the exponent field and the sign bit". For binary32 that
subtraction gives `32 - 8 - 1 = 23`. IEEE binary32's precision is 24. The missing bit is the hidden
leading bit, which exists only under IEEE's normalization convention, and that convention is not any
of the ten axes. Three separate conventions are being silently inherited:

1. The hidden bit. x87 double-extended is the standing counterexample: an explicit integer bit, so
   two real formats with identical field widths and different representable sets. Identity, by D54's
   own test, and not expressible.
2. The reserved exponent codes. All-ones for infinities and NaNs, all-zeros for subnormals and zero.
   Without the reservation, `Stored<8, Gradual>` has a different emax than binary32 and no specials.
   The spec waves at specials once ("adds the specials that inhabit nothing", spec 200) with no
   mechanism anywhere in the axes.
3. The exponent encoding bias (127 for binary32). Arguably `Lowering` (it is an encoding of the same
   values), in which case fine, but then note the name hazard: `Numeral::Bias` (the affine origin
   offset, D68) and IEEE's "biased exponent" are unrelated concepts one word apart, in a crate family
   that will alias IEEE vocabulary. One of them should not be called bias at the public surface.

Consequence: the claim "fixed point and floating point differ only in where the exponent lives" is
true in Flocq's value-set sense (FIX against FLX/FLT as exponent functions, which the 0500 sketch
verified, including the FTZ threshold arithmetic, `emin + prec - 1 = -126`, which I checked) and
false as a claim about *formats people ship*, which differ additionally in normalization and special
values. The unification is real and worth keeping; its perimeter should be stated: `Stored` expresses
Flocq-style idealised floating formats, and `conv-ieee754` needs either two more identity axes
(significand convention, specials) or an honest note that its aliases denote the finite part of each
format. Run D67's test on this axis the way the spec ran it on MATLAB's bias, and it fails the same
way; the spec found the bias gap and did not find this one because nobody wrote the IEEE column out
to the same depth.

## 9. `FullRange<F>` misbehaves at its boundary. Verified by arithmetic.

`FullRange<1>` is `radix^1 / (radix^1 - 1) = 2/1 = 2`, a power of the radix. So a `FullRange<1>`
numeral is dyadic, and the membership derivation keyed on `Adjustment = Unit` (spec 199-201) misses
it: sound (no false claim) but incomplete (a true dyadic membership not derived), which matters
because the spec's selling line is "cannot lie", and the honest statement is "cannot lie, and can
fall silent, here is where". `FullRange<0>` is `1/0` and must be unconstructable. The talk's own
Mersenne detour knew this boundary ("odd and greater than one for every `F > 1`", talk 1016-1017);
the spec dropped the bound when the constructor was renamed. Restore it: `F >= 2`, stated on the
constructor, with the `F = 1` case documented as the reason. 1-bit UNORM is a real degenerate format
(values 0 and 1) and lands in dyadic territory where the equality bound will not find it.

## 10. The affine characterisation is per-exponent, not per-numeral. Reasoned.

"The value of a stored integer k is `Adjustment * radix^exponent * k + Bias`" (spec 112) reads as if
the representable set were the image of one affine map. For `Implicit` numerals it is, and everything
downstream that leans on it (lattice, coset, closure) is sound. For `Stored` numerals the exponent
varies per value, so the set is a union of lattices, one per exponent, and two consequences the
uniform case never has: representations may be non-canonical (`m * 2^e = 2m * 2^{e-1}` where both
fit, and for any radix above two the cohort problem is worse), and the "lattice generated by the
quantum" language stops applying globally. Nothing in the spec is wrong because of this, but the
derivations that quantify over "the stored integer" (membership, range, closure) should be checked
per exponent form before being written as blanket impls, or they inherit finding 2's disease at a
second position.

## 11. When does quantisation fire? Unstated, and the laws depend on it. Reasoned.

The spec never says whether the quantisation map applies per operation or on store. The most coherent
reading, each binary operation returns a `Number` of the same type so the map fires per operation
with `Growth` governing the intermediate *within* one operation, makes the law derivation well-posed,
and I recommend stating exactly that sentence. But then spec 266, "`Cold` pays a compare and select
on every store", should say on every operation, and the `Warm` narrative in the talk (compute wide,
"narrowing on store", talk 289-290) is the *other* semantics, deferred quantisation, under which a
signed clamping fold is observably associative because it clamps once at the end. These two semantics
give different answers for the same composition, and the difference is precisely the one the law
markers exist to police. Pick one, per composition or globally, and put the choice in the spec; a
`Growth` or `Lowering` reading that quietly changes *when* the policy fires is a policy axis wearing
a lowering costume, which is D54's test failing an axis the table does not list.

## 12. Smaller verified items, kept short.

- **`Exact` growth is incoherent for division.** "Keep everything, nothing is dropped" (spec
  170-171) has no meaning for `1/3`; the exact quotient is not a numeral at any width. Division
  always quantises. The Growth vocabulary presumes ring operations; say so, and define what `Exact`
  means for `Div` (presumably: quantise once, directly, no intermediate narrowing).
- **"Widths add, quanta multiply" is the multiplication case only** (spec 168-169). Addition is
  width plus one, quantum unchanged. The sentence generalises from Mul to "combining two values".
- **D69's table conflicts with D71.** The ratified axis table lists `Widening` as `InContainer`,
  `PerOperation` (talk 1635); D71 and the spec add `None` (spec 183-184). One-day drift inside one
  round; the spec should carry the amended instance list against D69 explicitly.
- **The single `Monotone` blanket impl undercovers.** `(TowardNegative, T, TowardPositive)` covers
  the nearest family only; the four constant directed rows (all-`TowardZero` etc.) are also monotone
  and need their own impls, else `Monotone` is itself a sampled law of exactly the kind
  `catalogue-edge-cases-as-tests.md` warns about (talk 1113-1117).
- **Stochastic rounding is excluded by construction and the exclusion is unrecorded.** A3 asked
  where `Stochastic` lives (talk 59); the triple is a deterministic function of position and cannot
  express it; the spec presents the vocabulary with no note. Record the exclusion (a stateful
  resolution breaks the ZST const model, a fine reason) or the extension point. On the same axis, a
  strength worth naming: round-to-odd IS expressible (`ToOdd` at all three positions), and it is the
  classical cure for double rounding, which D2's multiplication work under `Narrowed` growth will
  meet immediately. The vocabulary being able to state the cure is a real point in its favour.
- **The MV-chain claim is right, with a policy condition attached.** UNORM-encoded `UFixed<0, F>`
  closed at 1 is the Łukasiewicz chain with `2^F` elements, and arvo's half-open dyadic `[0, 1)` is
  not, verified against the MV axioms: the strong disjunction needs the top element and the
  involution `1 - x` needs 1 representable. But the MV *operation* is truncated (saturating)
  addition, so the structure is a property of the (numeral, policy) pair, not of the numeral; under
  `Hot`'s wrap the same numeral is `Z/2^F` instead. The composition-keyed derivation supports this
  fine; the prose files it under numeral-derived membership, which is one contract too low.
- **The Flushed underflow has a dual reading worth one decision.** FTZ as identity (subnormal values
  do not exist, Flocq's reading, the spec's choice) against FTZ as operation behaviour (results are
  flushed but a subnormal bit pattern can be loaded, the hardware reading with DAZ). For a substrate
  whose `Cold` columns hold raw bit patterns, the identity reading makes a loadable pattern a
  non-value, and something must say what deserialising it means. Not an error; an unstated boundary.

## 13. What I verified and found solid, so the panel does not re-litigate it.

The quantum decomposition arithmetic: `1/255 = (256/255) * 2^-8`, adjustment inside `[1, 2)`,
checked. The bias-against-adjustment independence argument (D68): an affine map is not determined by
either half, and the UNORM one-liner (no bias lands both `k = 0` on 0 and `k = 255` on 1) checks
exactly, spec 110-114 and talk 1489-1494. Underflow as identity: FLX/FLT/FTZ are three value sets,
Flocq's reading, and the 0500 sketch's threshold numbers are correct. The rounding triple covering
IEEE's five and SystemC's seven: I walked all twelve rows, every one is a triple, including
`SC_RND` (ties toward positive) and MATLAB's trap trio, and the talk's warning that only MATLAB's
`Convergent` matches IEEE's default is correct. The signed-trap non-associativity example
(`(127 + 1) + (-1)`) is correct under Kleene equality. Unsigned *saturating* associativity, which a
reader might doubt, is in fact true (finding 1's table). The `LogicalWidth` non-derivability argument
(D69) is sound on all three routes; my finding 8's off-by-one does not touch it, only its
"significand derives" corollary. And the overall three-contract sort under D54's test is, for the
nine axes I checked it against, clean; my quarrels above are with the derivations and boundaries, not
with the identity/policy/lowering cut itself, which I think survives this review stronger than the
derivations built on it.

## 14. Where the mathematics suggests a cleaner foundation than the spec reached. Reasoned; offered, not ruled.

The spec has independently reinvented, in pieces, a structure the literature has had since the
1970s: Kulisch's theory of computer arithmetic defines a machine operation as
`a (op') b = round(a op b)` where `round` is a monotone idempotent projection from the exact
structure onto the representable subset, and derives which laws survive from three axioms on the
projection (idempotence on the subset, monotonicity, sign symmetry). That is exactly the spec's
"quantisation is one map, laws are derived from its properties", with fifty years of worked
consequences attached, including precisely the associativity transfer questions of findings 1-3.

Restated in that frame, the design gets simpler and the errors above become unwriteable:

1. A `Quantisation` is a monotone idempotent map from the extended lattice onto the representable
   set, possibly partial (`Refuse`). The five members are its normal form, and `Monotone` stops
   being a derived afterthought and becomes the axiom most compositions satisfy.
2. The round-then-resolve composition of finding 4 is Kulisch's definition verbatim, which settles
   the boundary question by citation rather than by invention.
3. The law derivation conditions on projection properties (congruence for wrap, monotone retraction
   for clamp, translation stability as the common test), not on a hand-sorted `Faithful` marker.

I am not proposing the spec adopt Kulisch's notation or scope; his rounding is total and two-sided
and arvo's is richer at the ends. I am proposing the spec cite the frame, adopt the
projection-with-properties shape for `Quantisation`, and let the named properties carry the law
derivation. The alternative reading, that the spec's five-position vocabulary is *better* than the
projection frame because it is finitely enumerable and const-dispatchable, is genuinely arguable and
a later lens (encoding, compile cost) may prefer it; the two are not exclusive, since the projection
properties can be derived per five-tuple. What should not survive either way is a law derivation
whose central predicate is defined by example.

---

**Summary for the next member.** The three-contract decomposition holds up. The derivations built on
it do not, in five places: the unsigned law impl is false (SubstituteZero counterexample, finding 1),
the law key omits the numeral and contradicts the spec's own float analysis (finding 2), the
faithfulness definition does not cut (finding 3, with a replacement identity), the range boundary
diverges observably from every standard the conventions alias (finding 4), and `Fallibility` is
asserted where the spec's own rule demands derivation (finding 6). Radix, the hidden bit, and the
specials are three identity-relevant facts no axis carries (findings 7, 8). None of this argues for a
different shape; all of it argues that the spec's strongest section, "what is derived rather than
declared", is currently its weakest, and that the sketch obligations in the spec (its section
"Sketch obligations", items 2 and 3) would not have caught findings 1, 2, or 4, because they test
that the machinery compiles and refuses, not that the mathematics it encodes is true. A sketch
compiling a false blanket impl passes. The obligations should gain hand-worked counterexample cases,
starting with the ones in this file.
