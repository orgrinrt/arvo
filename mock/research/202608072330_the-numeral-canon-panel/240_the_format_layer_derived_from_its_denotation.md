# 240. The format layer, derived from its denotation

**Persona:** Leslie Lamport. Specification before implementation; the state machine
before the code; a proof written to find out whether a thing is true rather than to
decorate a thing already believed.

**Position:** a second read of the format topic, dispatched to derive what the
format layer establishes rather than to summarise what the panel said about it.
Written without opening `63`, `64`, `236`, `237`, or the `proposal` namespace on
topic `the_format`. Phase two, below the marked line, is the reconciliation after
reading those.

**Probes:** `240_probes/`, five instruments, sources and outputs committed with
`RUN.md` carrying every build line, every exit code, and the control that had to
fire before each number counted. Two link the shipped `arvo-format` and read its
real associated items. Three are models of the coordinate space, and `RUN.md`
states the bridge between the halves and its bound. One of my own claims was
refuted by my own probe, and both runs are on disk.

**Read for this file:** the `ruling` namespace in full, at
`mock/registry/ruling.toml`, with the ratified rows read verbatim rather than
through anybody's account of them. `dimension`, for the axes a predicate may name.
`topic::the_format`. Then `08`, `55`, `56`, `57`, `58`, `59`, `60`, `61`, `62` in
full. Then `arvo-format`'s source in full and its tests in full, as claims to test.
**Not opened:** `63`, `64`, `236`, `237` and their probe directories, the
`proposal` namespace on topic `the_format`, and every other numbered panel file.

## Gates

**The canon gate passes, in the first of the three situations.** A ratified canon
exists for this topic: `ruling::the_format_spine_is_canon`, at `rung = "ratified"`,
`ratified_by = "both"`. It governs and this file defends it. What I checked the
work against, by query rather than by memory: that row, plus
`ruling::behaviour_is_stated_per_declared_signature_and_the_premise_dissolves`,
`ruling::the_derivation_is_a_placement_and_the_operation_set_is_an_admission_rule`,
`ruling::never_a_runtime_check_and_one_lowered_path`,
`ruling::the_ambiguous_rounding_word_is_retired_for_six_explicit_names`,
`ruling::arms_over_regions_are_the_fundamental_heart` and
`ruling::the_warrant_is_a_token_and_a_clause_on_the_values_side`.

Nothing below proposes work the canon forbids. Everything below that criticises the
shipped tree criticises it **for departing from that ratified row**, which is the
direction the provenance ladder runs. The nine premise files are agent output with
no recorded human decision and are read as such throughout: where one of them
conflicts with the ratified row I take the row.

**The test gate: run, read, and it does not stop the work, but it has two
findings.**

`cargo mock test` over the whole tree is **red**, one of nine trees failing, and it
is honestly red. The bench tree fails on four variants whose manifests inherit an
`arvo` dependency from a workspace root that no longer has one, which is the nuked
crate tree showing through a manifest nobody updated. `RUN.md` carries the four
names. The workspace members are green: `arvo-format` 51 plus 2 compile-fail,
`arvo-placement` 18 plus 1 ignored, `arvo-strategy` 10, and the lint pack 603 plus
13 ignored.

I read the body of every test in `arvo-format/src/tests.rs`, all 601 lines of it,
which is the surface this file is about. **It is a good suite and it is better than
most of what I have seen in this workspace.** The width sweeps genuinely run 1
through 62 rather than the powers of two. The laws that could be tautological are
not: `the_law_separates_the_two_constructions_rather_than_answering_one_way` at
`tests.rs:547` exists precisely so a law stuck at one verdict fails, and
`the_control_the_widths_produce_different_slot_ranges` at `tests.rs:34` is the
control the sweeps rest on. Two predicates that were `matches!` over an enumeration
nothing read were deleted rather than improved, and `overflow.rs:70-74` says so in
the file where they stood. That is the discipline working.

**Two findings against it, and the second is the one that matters.**

**One, minor: a redundant assertion.** `slot_count::<Unsigned<$w>>() == 1i64 << $w`
is asserted at `tests.rs:75` and again at `tests.rs:434`, over the same width list,
in two different tests. One of them is doing the work.

**Two, and this one is the sampled law the file's own header forbids.**
`tests.rs:6` reads "The laws the design names, asserted over the whole matrix
rather than a sample", and `tests.rs:8` reads "Choosing which instantiations to
include is choosing what not to find out". The phase law at `tests.rs:183` samples
four phase numerators, `1`, `1`, `3` and `0`, over a fixed denominator of two.
Every one of them is odd or zero. **The region where the law is wrong is exactly
the even nonzero numerators, and no row reaches it.** Section 3.1 has the
measurement. This is not a case somebody could not have thought of: `Biased`'s
third const parameter is an `i64` and `2` is the second value anyone would try.

Neither finding stops the assigned work, because neither is a fabricated green: the
suite is not lying about coverage it does not have, it is missing a case in a law
it correctly frames. I have said so plainly and moved on.

---

## 0. The verdict, before the argument

**The ratified spine is right, and I could not break it.** I went looking for a
representation the four propositions cannot describe and for a formulation that
describes the same things more simply, and found neither. That is the strongest
support a spine gets, and it is worth more than agreement, because it came from
trying to break it.

**What the format layer establishes, stated once:**

> A format is a pair: an ambient domain, and a finite set of that domain's elements.
> Everything else a format carries is a **presentation** of that set, and a
> presentation is not the format. Arithmetic on the format is a function into the
> set, factored as an exact operation in the domain followed by a total map onto
> the set, and the total map is an object with its own identity and its own laws.
> The concept admits a new instance by supplying the presentation and discharging
> the obligations that make the presentation denote a set at all.

Everything below is the derivation of that sentence, and then what happens when it
is held against the shipped realisation.

**Five things the derivation establishes that the panel's nine files do not say, in
descending order of how much they cost:**

**One. The union-of-shells formulation gets subnormals free, and the knee is an
artifact of the other formulation.** `08` reports the canonical exponent as a
function whose float instance needs a clamp at the bottom, calls gradual underflow
the meet of a fixed grid and a float, and puts to op whether the design wants a
third named shape for it. Measured: it does not. A quantum law affine in the
magnitude, with a slot range that does not vary with magnitude, denotes the IEEE
binary set exactly, subnormal run included, at thirty parameter pairs
(`q3_output.txt` section 1). The constant-step region near zero falls out of the
lowest shell covering everything below the next shell's reach, with no `max`
anywhere. **`08`'s question one has an answer and it is not the one `08` framed.**

**Two. The shipped `Floating` point cannot denote an IEEE set, and the reason is
one code point.** Its slots are `Signed<MANTISSA>`, which is two's complement and
therefore asymmetric under negation. An IEEE binary set is symmetric. Zero exact
matches over 1080 comparisons with the two's-complement range, thirty with a
symmetric one (`q3_output.txt` section 1). This is `62`'s range-symmetry parameter,
which it found moving the induced algebra two rungs, arriving at the denotation
from the other end: it does not only change what laws hold, it changes which set is
denoted.

**Three. A shipped law of the format layer is wrong, over about a third of the
space it quantifies over.** `has_additive_identity` at `format.rs:65` tests
`F::PHASE_NUM == 0`. The phase is `PHASE_NUM / PHASE_DEN`. A numerator that is a
nonzero multiple of the denominator is a whole number of quanta and leaves zero on
the grid. Measured against the shipped crate: five of five even-numerator rows
disagree with the denotation, controls holding on ten rows (`q1_output.txt`).
Measured over a swept model: 7375 of 23800 tuples, 31 per cent (`q2_output.txt`).
The repair is fifteen lines of integer arithmetic and it agrees with the
enumeration on every one of the 23800.

**Four. The shipped adaptation is a total map onto the representable set for the
constant-quantum family and is not one for the other family.** `adapt` returns an
`i64` slot. The representable set is indexed by a slot and a magnitude. At
`MAGNITUDES = 1` the slot determines the member; above it, it does not, and
`apply.rs` never reads the magnitude, in 267 lines, with `MAGNITUDES` appearing
zero times and the word `magnitude` twice, both in doc comments. On
`Floating<5,0,8>` one slot names eight different members (`q5_output.txt`). This is
the ratified factoring's second half realised on half its domain.

**Five. The format's coordinates carry no obligation, where the slot range's do,
and 56 per cent of the coordinate space denotes a set no numeral convention wants.**
`Slots::ADMITTED` at `slots.rs:93` refuses an inverted range, a zero width, an
uncountable span and a width that cannot address its range. There is no
`Format::ADMITTED`. Over 16728 swept coordinate tuples, 9376 denote a set whose
gaps are not powers of one ratio: shapes like `[1, 4, 5, 8]`. The obligation that
separates the two is decidable from coordinates the `Format` trait already carries
and it agrees with the enumeration on all 16728, with three mutants caught
(`q4_output.txt`).

**And one thing I got wrong, kept with its refutation.** I claimed every set the
parameterisation reaches is a single geometric ladder. My own probe refuted it, 93
of 240 tuples, and the refutation is what produced finding five and the narrower
true statement in section 2.4. Both runs are on disk as
`q3_output.v1_integer_ratios_only.txt` and `q3_output.v2_ladder_claim_refuted.txt`.

---

## 1. The derivation

I do not derive a format from what a numeral looks like. I derive it from the
smallest thing that has to be written down before any statement about arithmetic on
it is even a statement. That is the discipline: state the problem before describing
the solution, and the problem here is what a sentence about a number's behaviour is
quantified over.

### 1.1 The object, at the top of the hierarchy

Take a sentence a consumer wants to be true. "Adding these two values gives that
one." For it to be a proposition rather than a noise, four things must already be
fixed.

**Which values exist.** Without a set, "that one" names nothing and the sentence
has no truth value.

**Where the exact answer lives.** Addition of two members of a finite set of
rationals is not a member of that set in general. So the sentence is not about an
operation on the set; it is about an operation somewhere larger, and that larger
place has to be named, because "the exact answer" means different things in the
rationals and in a finite ring.

**What happens to the exact answer.** It is generally not in the set, and something
brings it back, and that something is where every interesting property lives.

**Nothing else.** That is the claim, and it is the load-bearing one.

So:

> **A format is `(D, V)`**, an ambient domain and a finite `V ⊆ D`.
> **An operation on a format is `ρ ∘ f`**, where `f` is exact in `D` and `ρ: D → V`
> is total.

`ρ` is a parameter of the operation and not of the format, which is why one format
carries many operations and why two declarations differing only in `ρ` are two
declared signatures over one format. The shipped crate has this exactly right at
`adapt.rs:63-77`, and `tests.rs:364` pins it.

### 1.2 Why identity is the set and not the presentation

This is the step the panel's four-choice model leaves open and `56` section 2.2
correctly named as unstated.

`V` is finite, so it can be written down many ways: as a phase and a step and a
count, as a low value and a high value and a step, as a slot range under a quantum
law, as a literal list. Each of those is a **presentation**. The question "is
identity the set or the presentation" is not a matter of taste, and here is the
argument.

**Suppose identity were the presentation.** Then two presentations of one set are
two formats. An operation declared over one is not an operation over the other. A
consumer holding a value of the first and wanting the second must convert, and the
conversion is `ρ` applied to a value already in `V`, which is the identity map. So
the design would carry a conversion that provably does nothing and cannot see that
it does nothing, at every pair of presentations of every set. That is not a
performance complaint; it is that the design would be unable to state a true
sentence about its own values.

**Suppose identity is the set.** Then the presentation is a compression of the
format for the compiler's benefit, two presentations of one set are one format, and
the conversion between them does not exist as an object because there is nothing to
convert.

The second is what the canon says, in the ratified row's own words: "A format is
identified by its ambient domain and its representable set". So the question is
settled, and the interesting question is what it obliges, which is section 3.2.

**One consequence worth stating separately, because it is not obvious.** `55`
section 1 makes the encoding `E` a component of the format and orders it after `Q`,
and `56` section 2.2 objects that this leaves equality ambiguous. Under the set
identity, `E` is not a component at all: two encodings of one set are one format,
which is exactly what `55`'s own probe 3 was built to show for two's complement
against offset binary. `55` reached the right answer about the probe and then wrote
`E` into the tuple anyway. The ratified row settles it in the set's favour, and the
shipped crate agrees: `arvo-format` has no encoding coordinate, and the placement
lives one crate over.

### 1.3 What `ρ` must be, and what it must not be

`ρ` is total onto `V`, which is the whole of what the factoring demands. Three
things follow immediately and one does not.

**Totality forbids divergence.** A panic is not a member of the slot the factoring
names, because a diverging map is not a map onto `V`. `overflow.rs:10-17` says
exactly this and is right.

**Totality does not require a single mechanism.** Nothing in the factoring says `ρ`
is one map rather than a composition, so "one map with two regions" and "two
composed maps" are the same object described twice, and the design may pick either
wording. What it may not do is let the two disagree about the composition order,
which `apply.rs:12-16` fixes as rounding then completion, correctly, because a
position may round onto a slot outside the range and never the other way.

**Totality says nothing about the laws.** This is the point where the panel's most
expensive result lands. Whether `ρ` is monotone, distance-minimising, absorbing, or
a homomorphism is not part of being an adaptation. `55` proposed expelling wrap
from the slot on the grounds that it fails the properties the other members share;
`56` measured that the members fail each other's properties in both directions with
all four cells of the two-by-two inhabited; `57` established that the property
deciding associativity is absorption and that `42`'s bound-counting sentence is
false under both readings of its own quantifier. **The derivation says why the
two-by-two had to be inhabited**: the slot is defined by totality alone, so no law
beyond totality is shared by construction, and any expectation of a shared law was
an expectation about the members somebody happened to name rather than about the
slot.

So the correct statement of the adaptation's tier is the weakest one: **`ρ` is
total onto `V`, and every further law is a per-`ρ` fact carrying its own region.**
That is `ruling::arms_over_regions_are_the_fundamental_heart` at the adaptation
layer, and I arrive at it from the definition rather than from the measurements.

### 1.4 What the format concept must expose for a chain claim to exist

`60` derived, blind, that a chain is a composition of exact operations plus a
schedule of adaptation points, and that a format concept closing its operations
over the format cannot state the composite-exactness claim at all. I second the
statability half from the definition rather than from a probe, and the argument is
one line.

`ρ ∘ f` for a single operation is stateable with `V`, `D` and `ρ`. A claim about a
composite, that `ρ(f(g(x, y), z))` equals something, quantifies over the
intermediate `g(x, y)`, which is an element of `D` and not of `V`. **A concept
whose operations have signature `V × V → V` has no name for that intermediate**, so
the sentence has no expressible form. This is not a limitation to be worked around;
it is what "closed over the format" means.

The shipped crate is on the right side of this and it is worth saying, because it
is the single best decision in the tree. `apply::Exact` at `apply.rs:26-35` carries
a position as `slot + num/den`, exactly, as a rational rather than as an
approximation. That is the intermediate, named, in the format's own coordinates,
with the remainder exact so that a tie is representable rather than being whatever
the host's arithmetic produced. **A tie rule is testable because of that decision
and would not be otherwise.** I would keep it whatever else changes.

---

## 2. What follows, and what does not

Four theorems. Each is stated so it can be wrong, and each carries the instrument
that established it.

### 2.1 The union of shells denotes a float, subnormals included

**The construction.** Under the shipped presentation, `V` in units of the quantum
at magnitude zero is

```
V = { φ + s · r^(σ·m)  :  s ∈ [a, b],  m ∈ [0, M) }
```

with `r` the ambient radix, `σ` the quantum law's slope, `M` its magnitude count,
`[a, b]` the slot range and `φ` the phase. It is a **union of `M` arithmetic
progressions over one index range**, geometrically spaced.

**The claim.** With `φ = 0`, `σ = 1`, `r = 2` and `[a, b] = [-(2^(p-1) - 1),
2^(p-1) - 1]`, this set equals the IEEE binary set at precision `p` over `M`
exponents, subnormals included and nothing named.

**The evidence.** `q3` section 1, thirty exact matches over 1080 comparisons, with
the IEEE side computed from Flocq's `generic_format` written as integer
divisibility so nothing rounds: `k` is representable exactly when `2^max(0,
bitlen(|k|) - p + 1)` divides `k`. The correspondence is `mantissa = p` and
`span = p + M - 1`. Worked witness at `q3_output.txt`: mantissa 3, exponents 3,
against `p = 3`, span 4, both sets printed in full and equal.

**Why the knee is not needed.** Flocq's `φ(x) = max(emin, mag(x) - p + 1)` states
membership as a function **of the value**, so the exponent has to be clamped at the
bottom or the smallest magnitudes get an exponent below the format's own floor. The
union states membership as an existential **over an index**, and the lowest shell,
at the smallest step over the whole slot range, covers everything below the next
shell's reach. `q3` section 2 prints it: at mantissa 3, exponents 4, the gaps read
`[8, 4, 4, 2, 2, 1, 1, 1, 1, 1, 1, 1, 1, 2, 2, 4, 4, 8]` and the run of the
smallest gap has length 8, centred on zero. **That run is the subnormal region and
no line of the law mentions it.**

**What this does to `08`.** `08` section 4.2 measures that gradual underflow is the
meet of a fixed grid and a float, twice, once by enumeration and once through the
trait solver, and both measurements are right. Its reading of what that costs is
what I disagree with: it concludes the design either names a third point off its
two curves or admits the canonical exponent as a function. **Neither, under the
union.** The knee is not a shape the parameterisation has to reach; it is a shape
the other formulation has to write down. `08`'s question two, "is gradual underflow
understood as the meet", has the answer yes and the follow-on "so what does a
fourth point cost" does not arise.

### 2.2 The two's-complement slot range cannot denote a symmetric set

**The claim.** No instantiation of the shipped `Floating` point denotes an IEEE
binary set.

**The argument, which is one sentence.** `Signed<B>` has `MIN = -2^(B-1)` and
`MAX = 2^(B-1) - 1`, so `V` contains `a · r^(σ(M-1))` and does not contain its
negation. An IEEE binary set is closed under negation. A set closed under negation
is not equal to one that is not.

**The evidence.** `q3` section 1: zero exact matches over 1080 comparisons with the
two's-complement range, against thirty with the symmetric one. The worked witness
prints the difference as a single element: `-16` is in the union and not in the
IEEE set, and nothing is in the IEEE set and not in the union.

**Why this matters more than one code point.** `62` established, from the algebra
side, that the range's symmetry under negation moves the induced multiplicative
structure from "not a semigroup" to "a commutative monoid" without touching the
policy, the width or the scale, and called it a property of `Q` alone. This is the
same parameter reached from the denotation: it decides which set is denoted, and
therefore, under section 1.2, **which format it is**. The two seats are looking at
one coordinate through two windows, and neither the dimension namespace nor the
`Format` trait names it.

### 2.3 The adaptation's codomain is the slot range, not the set

**The claim.** `apply::adapt` is a total map onto `V` exactly when `M = 1`.

**The argument.** `adapt` has return type `i64` and its body reads `MIN`, `MAX`,
the rounding mode and the overflow policy. A member of `V` is determined by a slot
**and** a magnitude. At `M = 1` the magnitude is forced and the slot determines the
member. At `M > 1` it does not.

**The evidence.** `q5_output.txt`. Four one-magnitude formats, worst slot ambiguity
exactly one, which is the control. Three many-magnitude formats at 3, 5 and 8. And
the worked case: `Exact::on_grid(1000)` under a saturating adaptation returns slot
`3` for `Floating<3,-2,3>`, and slot `3` names `3·2^-2`, `6·2^-2` and `12·2^-2`.
The format's largest member is the third of those. **A saturating adaptation that
returns the first has saturated to a quarter of the maximum**, and nothing in the
returned value distinguishes the three.

**What I am not claiming.** I am not claiming the crate computes a wrong number
today, because nothing downstream consumes `adapt`'s output for a `Floating`
format: `arvo-format` has no consumer in the tree above it. I am claiming the
ratified factoring's second half is realised for one of the two families the
ratified first half names, and that the gap is in the signature rather than in the
arithmetic. `apply.rs:229` returns an `i64` where the factoring wants a member, and
a member of a magnitude-indexed set is a pair.

**And the repair is not obviously "return a pair".** A slot plus a magnitude is a
redundant coordinate: `q5` measures eight coordinate pairs denoting one value at
`Floating<5,0,8>`. So the honest options are to return a normalised pair, to return
the value in the format's own exact rational coordinates, or to restrict the
adaptation surface to `M = 1` and say so. I have not established which, and section
6 puts it to the seats that own the realisation map.

### 2.4 The reach: what the parameterisation cannot denote

**My first claim, refuted by my own probe.** I claimed every reachable set is a
single geometric ladder, on the reasoning that the affine law fixes one ratio.
`q3_output.v2_ladder_claim_refuted.txt`: 93 of 240 tuples denote a set whose
distinct gaps are not powers of one ratio. Shapes like `[1, 4, 5, 8]` at radix 2
slope 3, and `[1, 2, 3]` at radix 3 slope 1. **The shells do not tile.** Where a
shell stops, the next one takes over at a join that is aligned only under a
condition nothing enforces, and where it is not aligned the union carries a gap
that is no power of anything.

**The narrower true statement, established by asking the reach question directly.**
A target set is named and the whole coordinate space is searched for a tuple
denoting it. `q3` section 4: the float target, shell exponents `[0, 1, 2]`, is
reached at `(radix 2, slope 1, magnitudes 3, slots [-3, 3])`, which is the control.
Two tapered targets, shell exponents `[0, 1, 3]` and `[0, 2, 3]`, are reached by
nothing.

> **The shell exponents a format denotes are an arithmetic progression**, because
> the quantum law is affine in the magnitude. A shell ladder whose exponents are
> not equally spaced has no coordinates.

That is the exact sense in which a tapered numeral is outside the parameterisation,
and it is narrower and more useful than the gap-shape claim it replaces. It also
agrees with `08` section 3.3 from a different instrument: `08` measured every posit
configuration to have canonical exponent slopes drawn from `{0, 1, 2}` and
concluded no intersection closure of the two named families reaches it. My route is
that the exponent sequence is an arithmetic progression by construction, so more
than one slope has no presentation, whatever closure is taken.

**Whether that is a defect is not mine to say and I decline to say it.** The canon
says the inventory is open and the concept closed. A tapered numeral joins the
inventory by supplying the concept's obligations, and the obligations do not
mention an affine quantum law: that is `arvo-format`'s presentation, not the
canon's. So a posit joins by implementing `Quantum` with a non-affine law, which
the shipped `Quantum` trait forbids by carrying `BASE` and `SLOPE` as its whole
content. **The narrowing is in the crate and not in the canon**, and naming which
tier it lives at is the useful part.

---

## 3. What the shipped layer establishes, and what it asserts

The tree is a claim to be tested. Here is what testing it found. Two of the five
are wrong today and three are gaps between the ratified sentence and its
realisation.

### 3.1 `has_additive_identity` answers on the numerator, not on the phase

`format.rs:65-67`:

```
pub const fn has_additive_identity<F: Format>() -> bool {
    F::PHASE_NUM == 0 && slot_in_range::<F::Slots>(0)
}
```

Its doc, `format.rs:53-62`, says "A zero phase puts zero on the grid at slot zero,
provided the slot range admits it. A nonzero phase takes it off, and takes one off
with it: every exact sum then lands half a step away from every grid point".

**The phase is `PHASE_NUM / PHASE_DEN`**, declared as such at `format.rs:31-42`, in
units of the quantum at magnitude zero. A numerator that is a nonzero multiple of
the denominator is a phase of a whole number of quanta, which shifts the grid by
whole steps and leaves zero on it whenever the corresponding slot is in range.

**Measured against the shipped crate**, `q1_output.txt`: `Biased<7,-2,2>` has phase
`2/2 = 1`, `has_additive_identity` returns `false`, and zero is in the set at slot
`-1`. Four more rows the same. The ten control rows, phases `0/1`, `0/2`, `±1/2`
and `3/2`, all agree, so the enumerator is not simply disagreeing with everything.

**Measured over the coordinate space**, `q2_output.txt`: 7375 of 23800 tuples, 31
per cent, with the first witness at phase `-7/1`, which is not exotic. Three of the
four shipped points use `PHASE_DEN = 1`, and at `PHASE_DEN = 1` **every** nonzero
phase is integral, so every one of them is a case the predicate gets wrong.

**The doc's second clause is also wrong on the same rows.** "Every exact sum then
lands half a step away from every grid point" is `56` section 5.2's result, which
holds at `φ = 1/2` and is false at `φ = 1`: an integral phase shifts the grid
without changing it, so exact sums land on it exactly as they do at `φ = 0`. The
prose imported a half-step fact as a nonzero-phase fact.

**The repair, checked.** `q2` section 1: for each magnitude, solve `s · r^(σm) =
-PHASE_NUM/PHASE_DEN` and ask whether `s` is an integer in range. Fifteen lines,
integer arithmetic, a bounded loop, the shape a `const fn` over the associated
items takes. **Zero disagreements with the enumeration over all 23800 tuples**,
with the shipped predicate caught on 7375 of them by the same instrument, so the
instrument can tell the two apart.

### 3.2 The coordinates are not quotiented by the set they denote

`ruling::the_format_spine_is_canon` identifies a format by its ambient domain and
its representable set. The shipped `Format` is a trait, so format identity is type
identity, which is finer.

**Measured**, `q1_output.txt` section 3. Two `Format` impls over
`BinaryRationals`: one at phase `0/1` over an outside `Slots` impl with range
`[-7, 8]`, one being `Biased<4,0,2>`, phase `2/2` over `Signed<4>` at `[-8, 7]`.
Different phase, different slot range, and the same sixteen values, printed in
full. The control, `Integer<4>` against the first, comes out different, so the
comparison is not degenerate.

**This is two definitions of one concept**, and the mathematician's answer to that
is old and is not a matter of taste: a concept is defined once, and the agreement
between two definitions is a theorem somebody has to prove and maintain. Here
nobody has, and the cost is concrete. Under
`ruling::the_derivation_is_a_placement_and_the_operation_set_is_an_admission_rule`
an operation is admitted exactly when it is a function of the declared signature,
and "where two realisations of one name disagree, the signature is missing a
coordinate". **Two presentations of one set are two signatures whose realisations
of every operation agree**, which is the opposite condition, and the admission rule
has nothing to say about it.

**The repair, checked.** `q2` section 2. For the constant-quantum family the set is
an arithmetic progression, so three comparisons decide equality: the count, the
lowest value in absolute units, and the step, with the singleton case separated
because one point has no step. **Zero disagreements over 142884 ordered pairs**,
with the offset-dropping mutant caught on 19698 of them.

**And the size of the gap, measured rather than asserted:** 378 coordinate tuples
in that pool denote 163 distinct sets, so 215 of them are a second name for a set
another tuple already names. **57 per cent of the presentation space is alias.**

**What I do not claim.** I have not extended the decision procedure past the
constant-quantum family. For the magnitude-indexed family the set is a union of
progressions and equality is a harder question I did not solve; section 7 says so.

### 3.3 There is no `Format::ADMITTED`, and 56 per cent of the coordinate space is ragged

`Slots::ADMITTED` at `slots.rs:93-119` is the right idea done well: an implementor
outside the crate can write a slot range, the trait says what it owes, and the
obligation is checked by the compiler rather than requested in a comment. The file
even says why the bound is an absent impl rather than a check, at `slots.rs:16-21`,
and it is right about that too.

**The format has no such obligation, and it needs one for the same reason.** The
coordinates can disagree with each other exactly as the slot range's could. Where a
shell's reach does not join the next shell's cleanly, the union carries a gap that
is no power of the ratio, and the denoted set is not a numeral of any convention:
it is a ragged union.

**The obligation, derived.** Let `R = radix^SLOPE`. Let `d_up` be the distance from
`MAX` to the least multiple of `R` strictly above it, and `d_down` the distance
from `MIN` to the greatest multiple of `R` strictly below it. The shells tile
exactly when both are powers of `R`. At `MAGNITUDES = 1` or `R = 1` there is one
shell and nothing to join.

**The evidence**, `q4_output.txt`: 16728 coordinate tuples at radices 2, 3, 5 and
10, slopes 0 to 4, magnitude counts 1 to 5, slot ranges over `[-12, 0] × [0, 12]`.
The oracle, computed from the enumerated gaps and knowing nothing about the
condition, calls 7352 clean and 9376 ragged. **The derived obligation differs from
the oracle on zero tuples.** A mutant admitting everything is caught on 9376; a
mutant checking only the top join is caught on 1264, with its first witness
printed, so the second half of the condition is exercised rather than assumed.

**56 per cent of the swept coordinate space is ragged.** A `Format` impl denoting
such a set compiles, is admitted, and nothing anywhere says otherwise, while a
`Slots` impl with an inverted range is refused at codegen with a named message.
**The two coordinates are checked at one tier and unchecked at the tier above it**,
and the check at the tier above is the same kind of thing: integer arithmetic over
associated items, const-evaluable, no runtime anything.

### 3.4 The quantum law's own coordinates have no declared dimension

Every finding here about the magnitude count, the quantum slope or the phase is a
finding whose controlling parameter has no row in the `dimension` namespace. The
namespace carries `integer_width`, `fraction_width`, `total_width`, `signedness`,
`radix`, `ambient_domain`, `overflow_policy`, `rounding`, `operation`, `arity`,
`chain_length`, `container`, `alignment`, `access_pattern`, `target_features`,
`threads`, `strategy`, `accumulator_width`, `toolchain`, `build_profile`,
`operand_window`, `occupancy`, `association` and `leaf_aliasing`. Twenty-four rows,
and not one of them is the shell count, the slope, or the phase.

`fraction_width` stands in for the constant family's exponent, since `F = -BASE`
there. Nothing stands in for the other two.

**Under `ruling::a_predicate_lists_only_what_holds` and the notation it descends
from, an axis a predicate does not name is one the finding does not hold along at
all.** So every finding in this file about the magnitude-indexed family is a
finding that, written in the notation as it stands, **claims to hold nowhere**,
because the axis that separates the two families cannot be written. That is a
defect in the notation rather than in the findings, and it is the same shape as the
gap `ruling::a_proof_and_a_bounded_range_get_markers_the_notation_lacked` records
and repairs for the width universal: the authors wrote nothing because there was no
honest spelling, and the notation read that as the strongest negative.

**I am not adding a dimension row.** The namespace is append-only and there is a
lint enforcing it, `the_axis_set_is_append_only`, which I saw pass 603 tests. The
finding is that three axes the format layer turns on are unnameable, and section 5
writes my predicates with that stated in the clause rather than pretending
otherwise.

### 3.5 What the shipped layer gets right, said plainly

A reader could take the four sections above for a verdict on the crate, and that
would be wrong. **Most of it is the best-argued source in this repository**, and
three decisions in it are ones I would defend against a rewrite.

**No carrier is reachable from the crate, and the dependency edge is what makes it
true** rather than a comment asking for it (`lib.rs:19-22`). That is
`ruling::behaviour_is_stated_per_declared_signature_and_the_premise_dissolves`
enforced by construction, which is the only way a claim of that shape is worth
anything.

**`Exact` carries the remainder as an exact fraction** (`apply.rs:26-35`), so a tie
is representable and a tie rule is testable. Section 1.4 says why that is the
decision that makes chain claims expressible at all.

**Two predicates were deleted rather than improved**, and `overflow.rs:70-74` and
`rounding.rs:99-102` say so where they stood: they were `matches!` over an
enumeration nothing read, so their tests reached a declaration and stopped. Someone
applied the declarations-nothing-constrains check to their own crate and acted on
it. That is rare and it should be said.

---

## 4. The three repairs, and what each costs

Stated as what they are: derived, checked against an oracle, and unratified.

**Repair one, the additive-identity predicate.** Replace `PHASE_NUM == 0` with the
existential over magnitudes. Fifteen lines, no allocation, const-evaluable, zero
disagreements over 23800 tuples. **Cost: fifteen lines and a `radix.pow` in a const
loop.** It also fixes the doc's second clause, which should say that a phase not
an integral multiple of the quantum takes the identity off the grid, rather than
that a nonzero one does.

**Repair two, denotation equality for the constant family.** Three comparisons,
zero disagreements over 142884 pairs. **Cost: a `const fn` taking two `Format`
type parameters.** What it buys is that the design can state, and check, that two
presentations are one format, which section 1.2 shows it currently cannot. What it
does not cover is the magnitude-indexed family, and offering it for half the
inventory is a real limitation rather than a rounding error.

**Repair three, `Format::ADMITTED`.** The tiling obligation, in the shape
`Slots::ADMITTED` already takes, forced by every function that reads the
coordinates. Zero disagreements over 16728 tuples, three mutants caught. **Cost: an
associated const on the trait and one `let () = F::ADMITTED;` in each reader.**
What it buys is that 56 per cent of the coordinate space stops compiling, which is
a large number and is the point: those tuples denote sets nobody wants and the
design currently cannot say so.

**One caution on all three, and it is the reason they are offered rather than
proposed.** `ruling::the_canon_does_not_police_what_shape_a_law_takes` exists, and
`arvo` is a toolbox rather than a policer. An obligation refusing 56 per cent of a
coordinate space is a policy about what a numeral is, and I have derived that the
refused sets are not numerals of any convention I know. **That is not the same as
establishing that no consumer wants one**, and the canon's own shape, closed
concept and open inventory, is a warning against my kind of tidiness. So repair
three is the one I hold most loosely, and section 6 puts it to the seats rather
than asserting it.

---

## 5. The findings, with their predicates

Written per `ruling::a_predicate_lists_only_what_holds` and
`ruling::the_warrant_is_a_token_and_a_clause_on_the_values_side`. An axis listed
with a span holds across it; an axis listed with a value holds only there; an axis
absent holds nowhere.

**Three axes this file turns on have no dimension row**, per section 3.4: the
quantum law's magnitude count, its slope, and the phase. Where a finding is
predicated on one of them I write the region in the clause of a warranted entry
rather than inventing an axis, and I say in the clause that the axis is undeclared.
A reader gating an arm on one of these has nothing in the notation to gate on, and
that is the honest report.

### F1. The union of shells denotes the IEEE binary set exactly

```
holds for:
  ambient domain = the rationals
  radix = 2
  signedness = signed
  total_width: W in 2..=5: exhaustive, the mantissa widths swept, and the
    exponent count 1 to 5 crossed with them
  rounding any: construction, no rounding occurs; the claim is set equality
    between two membership predicates and no value is adapted anywhere
  overflow policy any: construction, same reason
  threads any: construction, the sets are computed from integer divisibility
    with no shared state
  target features any: construction, same reason
  toolchain: rustc = 1.98.0-nightly (57d06900f), edition = 2024
  build_profile: opt level = 2, debug-assertions = off
  occupancy any: construction, no value is placed anywhere in the claim
evidence: 240_probes/q3, section 1, 30 exact matches of 1080 comparisons
condition: the slot range is symmetric under negation, which is an axis the
  dimension namespace does not declare
```

### F2. No two's-complement slot range denotes a set closed under negation

```
holds for:
  ambient domain = the rationals
  radix = 2
  signedness = signed
  total_width: W in 2..=5: exhaustive over the same crossing as F1
  rounding any: construction, no adaptation occurs
  overflow policy any: construction, same
  threads any: construction, same
  target features any: construction, same
  toolchain: rustc = 1.98.0-nightly (57d06900f), edition = 2024
  build_profile: opt level = 2, debug-assertions = off
  occupancy any: construction, same
evidence: 240_probes/q3, section 1, 0 of 1080
```

### F3. `has_additive_identity` disagrees with the denotation on an integral nonzero phase

```
holds for:
  ambient domain in {the rationals, the rationals at radix ten}
  radix in {2, 10}
  signedness in {signed, unsigned}
  integer_width: I in 1..=13: swept, the widths the probes instantiate
  fraction_width: F in -2..=8: swept, as the constant family's exponent
  rounding any: construction, the predicate reads no rounding mode and no
    adaptation is applied
  overflow policy any: construction, same
  threads any: construction, the predicate is a const fn over associated items
  target features any: construction, same
  toolchain: rustc = 1.98.0-nightly (57d06900f), edition = 2024
  build_profile: opt level = 2, debug-assertions = off
  occupancy any: construction, no value is placed
evidence: 240_probes/q1 against the shipped crate, 5 of 5 with 10 controls
  agreeing; 240_probes/q2 over the coordinate space, 7375 of 23800
condition: the phase numerator is a nonzero multiple of the denominator, which
  is an axis the dimension namespace does not declare
```

### F4. The repaired predicate agrees with the denotation everywhere swept

```
holds for:
  ambient domain in {the rationals, the rationals at radix ten}
  radix in {2, 10}
  signedness in {signed, unsigned}
  total_width: W in 3..=5: exhaustive, over the seven slot ranges the pool
    carries, which are not all width-shaped and are listed in the source
  rounding any: construction, the predicate reads no mode
  overflow policy any: construction, same
  threads any: construction, a const fn over associated items
  target features any: construction, same
  toolchain: rustc = 1.98.0-nightly (57d06900f), edition = 2024
  build_profile: opt level = 2, debug-assertions = off
  occupancy any: construction, no value is placed
evidence: 240_probes/q2, section 1, 0 of 23800, with the shipped predicate
  caught on 7375 by the same instrument
```

### F5. `apply::adapt` is a map onto the representable set only at one magnitude

```
holds for:
  ambient domain = the rationals
  radix = 2
  signedness in {signed, unsigned}
  total_width: W in 3..=8: exhaustive over the seven formats instantiated
  rounding = half_even
  overflow policy = saturate
  operation in {adapt}
  arity = 1
  threads any: construction, `adapt` is a const fn reading associated items
  target features any: construction, same
  toolchain: rustc = 1.98.0-nightly (57d06900f), edition = 2024
  build_profile: opt level = 2, debug-assertions = off
  occupancy any: construction, the claim is about a return type rather than a
    placement
evidence: 240_probes/q5 against the shipped crate, ambiguity 1 on all four
  one-magnitude formats and 3, 5, 8 on the three others
condition: the quantum law's magnitude count, which is an axis the dimension
  namespace does not declare. The one-magnitude half of the claim is what
  `rounding` and `overflow policy` above are fixed at; whether it holds at the
  other five modes and the other two policies is unmeasured
```

### F6. The shell exponents a format denotes are an arithmetic progression

```
holds for:
  ambient domain in {the rationals, the rationals at radix ten}
  radix in {2, 3, 10}
  signedness in {signed, unsigned}
  total_width: W in 3..=5: exhaustive over the five slot ranges the search
    carries
  rounding any: construction, no adaptation occurs in a set-equality search
  overflow policy any: construction, same
  threads any: construction, same
  target features any: construction, same
  toolchain: rustc = 1.98.0-nightly (57d06900f), edition = 2024
  build_profile: opt level = 2, debug-assertions = off
  occupancy any: construction, no value is placed
evidence: 240_probes/q3, section 4, two tapered targets reached by nothing over
  a search of 3 radices, 5 slopes, 6 magnitude counts and 110 slot ranges, with
  the float control reached
```

### F7. The tiling obligation decides raggedness from the coordinates alone

```
holds for:
  ambient domain in {the rationals, the rationals at radix ten}
  radix in {2, 3, 5, 10}
  signedness in {signed, unsigned}
  total_width: W in 1..=4: exhaustive, the slot ranges swept are every pair in
    [-12, 0] x [0, 12] rather than width-shaped, and the widths named here are
    the widths those ranges correspond to
  rounding any: construction, no adaptation occurs
  overflow policy any: construction, same
  threads any: construction, integer arithmetic over associated items
  target features any: construction, same
  toolchain: rustc = 1.98.0-nightly (57d06900f), edition = 2024
  build_profile: opt level = 2, debug-assertions = off
  occupancy any: construction, no value is placed
evidence: 240_probes/q4, 0 of 16728, with an admit-everything mutant caught on
  9376 and a top-join-only mutant caught on 1264
```

### F8. The presentation space over-counts the sets it denotes

```
holds for:
  ambient domain = the rationals
  radix = 2
  signedness in {signed, unsigned}
  total_width: W in 1..=5: exhaustive over the seven slot ranges the pool
    carries
  fraction_width: F in -1..=1: exhaustive, as the constant family's exponent
  rounding any: construction, no adaptation occurs
  overflow policy any: construction, same
  threads any: construction, same
  target features any: construction, same
  toolchain: rustc = 1.98.0-nightly (57d06900f), edition = 2024
  build_profile: opt level = 2, debug-assertions = off
  occupancy any: construction, no value is placed
evidence: 240_probes/q2, section 2, 378 tuples denoting 163 sets, and a
  three-comparison decision procedure agreeing with enumerated equality on all
  142884 ordered pairs
condition: the constant-quantum family. The magnitude-indexed family is not
  covered and the decision procedure is not claimed for it
```

**What is deliberately absent from every predicate above: `chain_length`,
`container`, `alignment`, `access_pattern`, `accumulator_width`, `association`,
`leaf_aliasing`, `operand_window` and `strategy`.** Nothing here was established
under any value of any of them, so under the notation these findings hold nowhere
those axes exist. For `container`, `alignment`, `occupancy` and `access_pattern`
that is arguably the point rather than a limitation: this crate has no carrier and
the ratified container ruling puts the placement one tier down. For `strategy` it
is a real gap, and I did not measure it.

---

## 6. What I put to the seats, for the resumption

**To `08`.** Your question one, whether the canonical exponent is a member of the
design or its two values are, is put as a fork between a two-point sample and a
function space. `q3` section 1 says there is a third reading you did not have: the
union over an index, where the exponent is affine and the knee falls out of the
lowest shell rather than out of a clamp. Thirty exact matches against
`generic_format` including the subnormal run. Does that dissolve your question two,
and does it change what admitting a fourth point costs, given that the third point
you priced is no longer a point?

And your section 4.1 collision with "no enumeration, ever" was about carrying the
canonical exponent as a type-level list. Under the union the exponent is two
integers and a count, so there is no list to carry and no collision to price. Would
you withdraw the collision, or is there a shape it survives in that I have not
seen?

**To `55` and `56`, on the equality question `56` section 2.2 opened and `55` never
answered.** It is settled by the ratified row, which says the ambient domain and
the representable set, so `56`'s identity-plus-realisation split is the right
reading and `55`'s tuple is not. What neither of you has is that the equality is
**decidable** for the constant family in three comparisons, measured at zero
disagreements over 142884 pairs, and that the shipped coordinates over-count the
sets by 57 per cent. Does that change what either of you wants the canon sentence
to say, given that "identity is the set" is now a thing the design can check rather
than a thing it asserts?

**To `57` and `61`, on absorption.** Nothing here touches your result and I second
its framing from the definition rather than from an instrument: the adaptation slot
is defined by totality alone, so no law beyond totality is shared by construction,
and `56`'s two-by-two had to be inhabited. The question I have is `61`'s
domain restriction. You established that absorption and coherence coincide exactly
when the operand box is a subset of `Q`, and grounded the restriction in the claim
that every stored value of a numeral type is already an element of `Q` by
construction. **Section 2.3 is a case where that is not obviously true**: `adapt`
returns a slot, and a slot at `M > 1` does not name an element of `Q`. Does your
restriction survive on a magnitude-indexed format, or is it a constant-family
result the way so much of this unit's material turned out to be?

**To `62`.** Your range-symmetry parameter is bigger than your file claims. You
found it moving the induced multiplicative structure two rungs without touching
policy, width or scale, and called it a property of `Q` alone. Section 2.2 finds
the same parameter deciding **which set is denoted**, so under the ratified
identity clause it decides which format it is, not only which laws hold on it.
`Signed<MANTISSA>` therefore cannot denote an IEEE set at any parameters, zero of
1080. Would you take that as a second instrument on your symmetry finding, and does
it change what you would want Q11's structure-naming option to carry, given that
the parameter is now upstream of the algebra rather than beside it?

**To `59` and whoever dispatches next.** Your P4 said the signed cell was empty and
`62` filled it. There is a second empty cell nobody has named: **every one of this
unit's results is about the constant-quantum family.** `55`'s probes, `56`'s
two-by-two, `57`'s absorption sweep, `58`'s fraction boundary, `61`'s wrap ring,
`62`'s cube: all of them are plain integer or fixed-point arithmetic at one scale,
which is `M = 1`. Not one file measures a magnitude-indexed format, and section 2.3
says the shipped adaptation is not even a map onto the set there. **The unit built
and validated its law layer in one of the two families the ratified spine names.**

**To whoever holds the realisation map**, given
`ruling::the_derivation_is_a_placement_and_the_operation_set_is_an_admission_rule`.
The admission rule says an operation is admitted when it is a function of the
declared signature, and that a disagreement between two realisations of one name
means a missing coordinate. Section 3.2 is the mirror case the rule does not cover:
two signatures whose realisations of every operation **agree**, because they
present one set. Is that a missing clause in the admission rule, or is it outside
it because the rule is about operations rather than about formats?

---

## 7. What I could not determine

**Denotation equality for the magnitude-indexed family.** The set is a union of
progressions and I did not find a decision procedure. The obvious route, normalise
both to a canonical presentation and compare, needs a canonical form I do not have,
and the tiling obligation of section 3.3 is a necessary condition for a canonical
form to exist rather than a sufficient one. This is the largest hole in section 4
and it is why repair two is offered for half the inventory.

**Whether the tiling obligation is what the design wants.** I derived that the
refused sets are not numerals of any convention I know, and that is a claim about
what I know. Section 4's last paragraph says why I hold it loosest.

**Whether F5 holds at the other rounding modes and overflow policies.** `q5` fixes
`half_even` and `saturate`. The argument is about the return type and is
mode-independent on its face, which is exactly the shape of reasoning this panel
has repeatedly found to be a constant-family artifact, so I have written the
predicate at the values I measured rather than at the values I believe.

**What `adapt` should return.** Section 2.3 names three options and establishes
none. A slot plus a magnitude is redundant, eight to one at `Floating<5,0,8>`.

**Whether any of this is priced.** Nothing here is priced. Every number is a count
from a committed probe, no bench harness ran on any of it, and the repairs in
section 4 have compile-time costs I have not measured. `q4`'s obligation runs a
bounded loop and two `pow` calls in a const context, and whether that is free at
the widths the design admits is a question for the harness rather than for me.

**Whether the shipped `Floating` point should exist in its current form.** Section
2.2 establishes it cannot denote an IEEE set. Whether the repair is a symmetric
slot range, a different point, or no point at all until the placement layer has
something to say, is a design call I have not made and would not make alone.

**Anything about `63`, `64`, `236`, `237` or the `proposal` namespace on this
topic.** Not opened before this line. Phase two is below.

---

## 8. Coverage, bounded honestly

**Read in full:** the `ruling` namespace at `mock/registry/ruling.toml`, by query
and at source; `dimension`; `topic::the_format`; the nine premise files `08`, `55`,
`56`, `57`, `58`, `59`, `60`, `61`, `62`; and `arvo-format`'s entire source and
tests, `lib.rs`, `format.rs`, `ambient.rs`, `quantum.rs`, `slots.rs`, `adapt.rs`,
`apply.rs`, `overflow.rs`, `rounding.rs`, `width.rs` in part, and `tests.rs`.

**Not opened:** the withheld list; every other numbered panel file, so every
statement above about `03`, `06`, `07`, `18`, `20`, `25`, `35`, `42`, `43`, `50` or
`55b` is a statement about what one of the nine reports of them; `INTENTS.md`,
`OPTIONS.md`, `DROPLIST.md`, `RULES.md`, `seed/`, `archive/`; `arvo-placement` and
`arvo-strategy` source beyond their test counts.

**Not re-run:** any predecessor's probe. My instruments do not depend on any
predecessor's counts, so there was nothing to regenerate before arguing with, and
where I use a predecessor's number I attribute it to their file rather than
regenerating it. That is a weaker position than `57`, `58`, `61` and `62` took and
I say so rather than implying otherwise.

**Built:** five probes, each with instrument validation that fires and is stated
before the run, all committed with sources and outputs and a `RUN.md`. Two
earlier runs of `q3` are kept on disk, and the second of them **refuted a claim I
had written into the file**, which is why section 2.4 says what it says rather than
what I expected it to say.

**Everything measured here is exact integer and rational arithmetic.** No floating
point appears in any instrument, deliberately, so no comparison in this file can be
an artifact of rounding.

**First-read here, owed a second:** the union-of-shells reproduction of the IEEE set
and the reading of the knee that follows from it; the two's-complement asymmetry
result; the additive-identity defect and its repair; the adaptation's codomain
result; the arithmetic-progression reach statement; the tiling obligation; the
alias measurement. **Seconded here, from a different route:** `62`'s
range-symmetry parameter, reached from the denotation rather than from the algebra;
`60`'s statability argument for why a closed operation signature cannot state a
chain claim, reached from the definition rather than from a probe; `56`'s
identity-plus-realisation split, which the ratified row settles and which I derive
independently in section 1.2.

**Nothing here settles anything.** Two of the eight findings are about a ratified
row's realisation rather than about the row, and the row stands. The three repairs
are derived and checked and unratified, and each needs a second independent
instance before it is anything more.

---

*Phase one ends here. Committed before any withheld file was read. Phase two
follows below and does not edit anything above this line.*

# Phase two: reconciliation, after reading the withheld files

**Appended per the dispatch. Phase one above is untouched and its commit precedes
this one**, so the ordering is checkable rather than asserted. Read for this phase,
in this order: the `proposal` namespace on topic `the_format`, all seven rows in
full; `63` in full; `64`; `236`; `237`; and the probe sources and outputs under
`236_probes/` and `237_probes/`.

## 9. The verdict of the reconciliation

**One of my eight findings is not mine. `237` reached it first, at the same
witness, with the same diagnosis.** Six of the remaining seven appear nowhere in
the withheld material. And the reconciliation turns up one measurement that is
worth more than any of my findings, which is section 9.4.

**F3 is seconded rather than found.** `237_probes/p2_the_shipped_crate_admits_the_counterexample.rs:90`
runs `Biased<4, 0, 2>` as its arm D, under the comment "A phase of a whole quantum,
which is the same lattice as phase zero", and its output reads
`value zero on the grid true` against `has_additive_identity() false`. Its verdict
says it in terms I would not improve: "`has_additive_identity` tests
`PHASE_NUM == 0` rather than whether the phase is a whole multiple of the quantum,
so at a phase of one whole step it reports no additive identity while zero is on
the grid."

That is my F3, exactly, and `237` had it first. **I derived it blind and my
instrument is independent**, so under the panel's own bookkeeping this is a second
arrival on one defect rather than a discovery, and I file it as one. What my probes
add beyond a second instance is the region and the repair: `237` runs one arm, `q1`
runs five shipped-crate rows with ten controls, `q2` runs 23800 model tuples and
puts the defect at 31 per cent of the coordinate space, and `q2` carries the
corrected predicate checked at zero disagreements, which `237` does not have and
did not need for its own question.

**Six findings survive as new.** F1, the union reproducing the IEEE set; F2, the
two's-complement asymmetry; F5, the adaptation's codomain; F6, the reach as an
arithmetic progression; F7, the tiling obligation; F8, the alias count. Checked by
grep over `63`, `64`, `236`, `237` and every probe source under `236_probes/` and
`237_probes/` for `ieee`, `generic_format`, `subnormal`, `MAGNITUDES`, `Floating`,
`tile`, `ragged`, `codomain`, `alias` and `denotation equal`. The only hits are
`63:184`'s inherited sentence that subnormals fall out of a max, and three
occurrences of "symmetric range" in `63` that are `62`'s algebra finding rather than
a denotation one.

## 9.1 What phase one got wrong, and it is a framing rather than a result

**Phase one's section 3.1 says the defect is in the shipped code and not in the
ratified row, and that is right, but I did not check whether the row's own `note`
already says so.** It does. `proposal::membership_of_the_representable_set_is_one_affine_predicate`
carries, in its `note`:

> The necessity of the phase term does not: it is one expert's measurement, plus a
> concession, plus a constructive repair, and the consolidation offers it as an
> argument for stating phase explicitly rather than as a two-expert standing.

So the phase clause was already marked as the weakest part of a ratified row, by
the row itself, before either `237` or I went looking. `237` quotes it and reaches
the same conclusion I did about which tier the defect lives at. **Nothing of mine
is corrected by this; what is corrected is my implicit claim to have located the
tier, which was already located.**

## 9.2 What my derivation supplies that the four unratified rows need

`237`'s closing paragraph is the reason this seat exists, and it is worth quoting
because it specifies the dispatch that produced this file:

> **What unblocks it is one dispatch, and `214` already specified the shape.** A
> cold derivation on the format concept, dispatched at `55` through `62` with the
> registry withheld, deriving from the premises before comparing. That is the only
> route by which the four rows can gain the instance they need, because anybody who
> reads the rows first is a reader.

My brief withheld the `proposal` namespace on this topic and not the `ruling`
namespace, which is the right cut: the ratified rows govern and are required
reading, and the four rows at one arrival are the ones a second instance is owed
on. So what follows is what phase one supplies to each, and I state it narrowly
because a reader inclined to count arrivals will otherwise count too many.

**`the_adaptation_slot_is_derived_and_a_strategy_selects_a_member_per_operation`.**
Phase one section 1.3 derives, from the definition and before reading the row, that
the adaptation slot is defined by totality alone, that no law beyond totality is
shared by construction, and that every further law is a per-member fact carrying
its own region. **That is a second arrival on the derived half**, reached from the
definition rather than from `56`'s two-by-two, and it explains why the two-by-two
had to come out inhabited rather than reporting that it did. It is not a second
arrival on the strategy-selects-a-member half, which I did not derive, and it does
not repair either blocker `236` and `237` name: the measured sentence filed
`normative` with no region is a filing defect my derivation does not touch.

**`the_format_concept_carries_three_things_upward_and_compositions_owe_their_own_laws`.**
Phase one section 1.4 seconds the statability half from the definition: a concept
whose operations have signature `V x V -> V` has no name for the intermediate a
composite claim quantifies over, so the chain clause has no expressible form
against it. **One line, no probe, and independent of `60`**, whose route is the
schedule. `237` refuses the row for welding two clauses by two authors into one,
and I second exactly one of them, which is the clause `64` reads as a chain
sentence rather than a format one. So my arrival is evidence for splitting the row
rather than for ratifying it whole.

**`a_nonzero_phase_leaves_the_representable_set_without_an_additive_identity`.** No
arrival. My F3 is a second refutation of a wider reading, alongside `237`'s, and a
refutation needs no second instance to stand. What I add is that the same
over-reach is in the shipped code at 31 per cent of the coordinate space rather
than at one witness.

**`raw_order_agreement_holds_for_monotone_encodings_not_only_unsigned`.** Nothing.
I did not touch the encoding axis, which section 1.2 argues is outside format
identity and which the shipped crate correctly does not carry.

**And one arrival on a row that is already canon.** Phase one section 1.2 derives
`a_format_is_identified_by_its_ambient_domain_and_its_representable_set` from the
cost of the alternative, before reading it: if identity were the presentation, the
design would carry a conversion that provably does nothing and could not see that
it does nothing. That row is ratified and needs no arrival. **What the arrival is
worth is section 3.2's consequence**, which the row does not have: the shipped
coordinates are not quotiented by the set, so the ratified identity and the shipped
identity are two relations, and the shipped one is finer by 57 per cent.

## 9.3 Where I differ from the consolidation, and it is one sentence

`63` section 3.3 and `C3` both carry "subnormals fall out of a max with no special
case", inherited from `55`'s Flocq framing and repeated in the ratified row's
`says` as "of which integers, fixed point, scaled integers and floats are points".

**The max is not needed and the shipped crate does not have one.** `quantum.rs`
carries `BASE` and `SLOPE` and computes `BASE + SLOPE * magnitude`, with no clamp
anywhere, and `q3` section 1 measures that the resulting union denotes the IEEE
binary set exactly at thirty parameter pairs, subnormal run included. So the
sentence should read that subnormals fall out of the lowest magnitude with no
special case, and the `max` belongs to the presentation `55` and `08` both derived
from rather than to the concept.

**This is a refinement of a ratified row's supporting prose and not a challenge to
the row.** The row says integers, fixed point, scaled integers and floats are
points of one predicate, and my measurement is a second, independent confirmation
that they are, from a presentation neither `55` nor `08` used. `63`'s own hedge on
`C3`, "Equivalence: passes at the model widths probed", is the honest one and my
widths are also model widths.

## 9.4 The measurement that is worth more than any of my findings

`63` consolidates ten member files. `236` and `237` read the seven rows against the
ratification gate. Between them they name the topic's gaps: no second read, one
arrival on four rows, `nearest` outside the ratified six, no `phase` axis, no
`quantum exponent` axis.

**Not one of them names the largest one, and it is measurable in a single command.**

```
grep -rlE 'Indexed|MAGNITUDES|Floating' --include='*.rs' .
```

Three files, in a panel that has committed **741 probe sources**. All three are
mine and all three were written today.

**Every instrument this panel has built on the format topic runs at one
magnitude.** `55`'s p1 through p5, `56`'s q1 through q3, `57`'s p1 through p9,
`58`'s p1 and p2, `60`'s p_a through p_d, `61`'s q1 and q2, `62`'s p1 through p4,
`236`'s p1, `237`'s p1 and p2: plain integer or fixed-point arithmetic at a fixed
scale, which is the constant-quantum family. The ratified spine names two families,
and the topic's whole law layer, its absorption biconditional, its two-by-two, its
congruence argument, its accumulator grades, its fraction boundary and its signed
cube, was built and validated in one of them.

**`59` found the sign asymmetry by asking which cell was empty and `62` filled it.
This is the same question one axis up, and the axis has no dimension row, which is
why nobody asked.** Section 3.4 of phase one is that gap stated from the notation
side; this is it stated from the evidence side, and the two are the same fact.

**And phase one section 2.3 is what makes it urgent rather than merely untidy.**
The shipped adaptation is not a map onto the representable set for that family, so
the question is not whether the law layer's results transfer to it. It is whether
the factoring's second half has been realised there at all, and the answer measured
in `q5` is that it has not.

## 9.5 What I would now put to the two gate seats

**To `236` and `237`, on the same finding.** We have one defect and two
instruments, and `237` had it first. Under the panel's bookkeeping that makes
`has_additive_identity`'s numerator test a two-instance result, with `237`'s single
arm establishing it and my sweep bounding it at 7375 of 23800 coordinate tuples.
Does that clear the bar for a `retirement` row against the shipped predicate, or is
a defect in code outside what the registry records at all? I could not find the
answer in the namespaces I read and I decline to invent one.

**To `237` specifically.** Your section 5.2 says the finding "cannot be written as a
registry predicate today" because two of its six coordinates have no axis, and that
this is "the more useful half". I agree and I have a third instance of the same
shape: **six of my eight findings turn on the magnitude count, the quantum slope or
the phase, and none of the three has an axis.** You name the additive repair as two
`dimension` rows. From where I sit it is three, and the third one, the magnitude
count, is the one separating the two families the ratified spine names, so a
predicate that cannot say which family it holds in cannot state the spine's own
scope.

**To whoever dispatches next.** `237` says the topic needed one cold derivation
with the rows withheld. It has had one. What it now needs is not a third read of
the seven rows: it is an instrument that runs at more than one magnitude, and the
cheapest useful one is `q5` extended past the shipped `Floating` point to whatever
the realisation map decides `adapt` should return.

## 10. Coverage of this phase

**Read in full:** the seven `proposal` rows on topic `the_format`, by query; `63`
in its entirety; `64`; `236`; `237`; `236_probes/p1` and its output;
`237_probes/p1`, `p2`, both outputs and `RUN.md`.

**Not re-run:** `237_probes/p2`. I read its committed output rather than rebuilding
it, which is weaker than the standard `63`, `237` and several members held, and I
say so rather than implying otherwise. My own five reproduce byte-identically after
the commit hook reformatted their sources, which is recorded in `RUN.md`.

**Not opened, still:** every other numbered panel file; `INTENTS.md`,
`OPTIONS.md`, `DROPLIST.md`, `RULES.md`, `seed/`, `archive/`; `63_probes/rerun/`.

**What changed in phase one on reading:** nothing above the line, per the dispatch.
What I would change if I could is the F3 header, which should read "seconding
`237`" rather than reading as a discovery. It is corrected here rather than there.

**Nothing here settles anything.** Six findings at one arrival, one at two, three
repairs derived and checked and unratified, and a gap in the evidence base that is
one grep and is the thing I would act on first.
