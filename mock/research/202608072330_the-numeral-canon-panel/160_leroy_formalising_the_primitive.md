# 160. Formalising the primitive

My job here is construction rather than attack. The ninth unit converged, in its reply round, on a
shape nobody has stated exactly: **types as the degenerate case of lenses** (`157:358-362`, S-8,
conceded and adopted outright by `159:169-175`), and **adequacy as two obligations of different
kinds** (`157` sections 3.2 to 3.4, its two most consequential probes rebuilt byte-for-byte at source
by `158`). This file plugs the holes in that shape, determines the bounds each clause holds in, and
states the thing exactly with its predicates.

Two of the holes turned out to be real defects in the converged text, and both are repaired here with
a compiled instance rather than reported: S-8's degeneracy condition is insufficient as worded
(section 2), and S-14's completeness obligation, taken as written, rejects the refinement parameters
the realisation-map topic requires (section 3). Neither defect touches the conclusion it sits inside;
both would have shipped into a canon candidate as stated.

**This file is not the canon candidate.** That is a later dispatch. Everything below is a suggestion;
op decides, and per I12 an opinion given before the experts converge is an ack.

---

## 0. The two gates

### 0.1 Canon gate: passed

Checked against `INTENTS.md` read in full, including its normative "How to read an entry" section,
and against `RULES.md` read in full.

The assignment is licensed: I14 (IN FORCE, `INTENTS.md:268-297`) requires that public API positions
use the stack's own primitives, which presupposes a determinate account of what one is, and I11
(`INTENTS.md:190-197`) makes the base and the contracts above it the library's stated purpose.
Nothing formalised below touches the RATIFIED rung; I13 (`INTENTS.md:214`) is used as an instrument
throughout and argued with nowhere. Everything I formalise is agent output on the presumed-wrong
rung, which is exactly what a formalisation dispatch exists to work over.

**One thing named rather than resolved, because it is op's.** The container premise, `156` item 1:
whether footprint is in the operation set the design ships. `157` F157-4 measured that a container
observation splits every identity class it swept, `159` section 3 mapped which of `154`'s options
each branch closes, and nothing below closes it. Every clause of the statement that the premise
reaches is written conditionally and marked, per the brief's instruction.

### 0.2 Test gate: passed, at 123 across 13, and it is the tenth count

The suite-bearing surface is `mock/benches/variants/`'s thirteen `-shared` crates; `mock/crates/` is
empty by design. Run crate by crate at `--release`, with `bitpack-write-contend-shared` serialised
per the standing instruction, and that crate otherwise untouched: its hang and its soundness bug are
handled elsewhere.

```
12 crates:  9+12+6+5+3+6+1+3+11+7+15+30 = 108
bitpack-write-contend-shared (serial)  =  15
total                                  = 123, all passing
```

`160_probes/run_test_gate.sh`, output at `160_probes/gate_release.out`. The script carries its own
negative control, declared in its header before the run: a crate whose invocation produces no
parseable pass count prints `MISSING OR ZERO` rather than reading as green, and the control line at
the bottom (a nonexistent crate) fires. That control exists because `157` section 0.2 found its own
first sweep measuring nothing and exiting 0, and the class does not deserve a third instance.

**What I read and what I did not.** My surface is the panel's own claims rather than new crate code,
so I read no new test bodies; `154` scanned all 123 mechanically, `155` read
`warm-container-shared`'s fifteen in full, and `157` read `bitpack-write-contend-shared`'s fifteen.
Where a claim below leans on a crate's source, the lines were reread at source by `158` section 1.1
and I reopened the two that carry weight myself: `warm-container-shared/src/lib.rs:187` (`pub trait
Carrier: Copy + 'static`) and `:279-283` (the five native impls). That is a bounded reliance and I
name it: if the three mechanical scans were wrong, my gate inherits the error.

Proceeding.

---

## 1. The statement, exactly

Written to compose with `112` section 9 (`112:898-945`) rather than to replace it: clauses this unit
did not touch are carried by reference, clauses it changed are rewritten here, and clauses it
produced are new. The propagation clauses belong to the realisation-map topic, whose authoritative
ledger is `122`'s per `AGREEMENTS.md` section 9; they are not restated here, because restating a
ledger I did not read would be a fresh compression with no checker.

Each clause below is tagged **[argument]** or **[sweep]**, which is the distinction Q65 records the
notation as lacking a marker for. I am not settling Q65; I am saying which kind each claim is, in
prose, so that a later pass can translate once op rules on the marker.

> **1. A primitive is a value set together with one realisation map taking an exact result back into
> it, over a declared operation set.** Its identity is that structure up to denotation-preserving
> isomorphism. A law is read off it and never declared. [Carried unchanged from `112:904-906`,
> resting on `110`, `63` C1, `90` R1, `109` P2; three instruments on the law half.]
>
> **2. The realisation is a lens: a placement `(carrier, offset, width)` of the value's bits within
> a carrier allocation.** The lens **degenerates to a value exactly where its focus is the sole
> logical occupant of its carrier allocation**: padding is permitted, sharing is not. At a degenerate
> point the language supplies a standalone `Sized` type; everywhere else the primitive is reached
> through its carrier, and no `Sized`-bounded contract ranges over it. Whether a placement has a
> standalone name is a property of the target's addressing, never of the primitive, so the canon
> states the reason and not the arity. [S-8 with its condition repaired, section 2; the
> addressing-not-primitive clause is `157` S-6 carried unchanged. Argument plus compiled instances.]
>
> **3. Weakening a declared refinement is free at every point of the declared range, including the
> packed end, and tightening is a compile-time refusal naming the instantiation.** [The dense half
> is `111` F111-12; the packed half is section 4.3's probe, new here. Sweep at the widths named in
> the predicates.]
>
> **4. Adequacy is the obligation the type owes the denotation, and it is two obligations of
> different kinds, plus an order.**
>
> **Soundness**: the denotation factors through what the type carries, **over every build**. It is
> structural, needs no enumeration, and is not enforceable by a signature, nor by anything that
> inspects one build; the residual obligation is a restriction on what the realisation map may read,
> checkable as a property of a call graph. [`157` sections 3.2 and 3.7, `159` F159-2's sharpening,
> `109:649-651` widened per S-21. Argument, with the compiled violation at
> `157_probes/p8_soundness_is_not_enforced/`.]
>
> **Completeness, up to weakening**: every pair of distinct shipped instantiations is either
> **separated by one witness**, or **connected by a weakening in exactly one direction**. A pair
> with neither, connected both ways and separated by nothing, is a spurious split, and the
> certificate's job there is to refuse. [The repair of S-14, section 3. Sweep at the model width for
> the direction count; the witness half is closed-form over `W in 1..=64`.]
>
> **5. The axis classification and the adequacy obligation are one thing at two granularities**, and
> the per-pair form is the one that discharges, because an axis can be read at some instantiations
> and not at others. [`157` sections 3.4 and 3.6, second-read by `158` section 2; F157-11 carried.]
>
> **6. The three verdicts of the classification age differently as the operation set grows.** A
> separation, once reachable, never becomes unreachable, so **declared semantics is stable**. A
> refinement pair shares its realisation map, so no growth of the operation set ever separates it,
> and **refinement is stable**. Only **spurious is provisional**: growth can convert it to declared
> semantics and nothing can convert it back. Eliminating a spurious axis from the parameter list,
> as opposed to gating arms on its inertness, is therefore licensed only where the verdict holds at
> the **largest operation set the design will ever admit**, which with a full literal is the
> realisation map's whole domain. At the shipped set, a two-direction verdict is a licence the
> resolver may take under a predicate, not a reclassification of the axis. [New as a statement;
> assembled from `110` F5/F6, `111` section 6, and `108:827`'s licence clause. Argument, section 4.1.]
>
> **7. The identity an operation set induces is determined by the reach of its terms into the
> realisation map's domain; it is monotone in that reach, saturates when the reach is the whole
> domain, and a full literal reaches saturation at depth one.** This holds **over operation sets
> whose members are functions of the value set and the realisation map**. An observation of the
> container is outside that class and splits every class it touches, so whether identity saturates
> at the literal is decided by `156` item 1 and is not decided here. [S-10 and S-11 carried
> unchanged with the premise inline; F157-4's measurement behind the premise. Argument, with the
> sweep at `157_probes/p4_output.txt`.]

**Permanence.** Every sentence survives a rewrite in another language or decade. None names a
container width, a marker, a type parameter, a crate, or a count. Clause 2 names `Sized`, which is a
statement about what the clause is *about*, the boundary between a language's addressable values and
everything else, and the sentence survives translation to any language with the same boundary.

**Equivalence.** Two teams implementing this produce units that behave the same on what matters: a
sole-occupant placement is an ordinary value and a shared placement is reached through its carrier;
weakening never costs and tightening never compiles; no pair of shipped types is connected both ways
and separated by nothing; and no axis is deleted from the surface on the evidence of the shipped
operation set alone. They differ on the lens's spelling, the sugar at the degenerate point, and the
boundary shape at the wall, which is exactly the residue O-C still holds (section 5.1).

---

## 2. The lens, exactly, and S-8's condition repaired

### 2.1 What S-8 says and where it under-specifies

`157:358-362`, verbatim in its operative clause: "a primitive's realisation is always a lens
`(carrier, position)`; where the position is const-zero and the carrier is one machine word, the
lens is an identity and the thing is a value."

The stated degeneracy condition, position const-zero in one machine word, is **insufficient**, and
the counterexample is the first element of any packed column: offset zero, one machine word, and not
a value, because thirteen sibling bits share the carrier and any reference to the element is a
reference to an allocation the siblings live in. It is also **over-strict in the other direction**
if "identity" is read literally: `154`'s `Dense13` is a `u16` holding 13 logical bits and 3 bits of
padding, and it is unambiguously a value while its lens is a mask rather than an identity.

Neither reading is what `157` meant, which is visible from its own S-1 ("at the native end the
carrier is one machine word and the lens is the identity, so `Bool` stays `Bool`"), but a canon
candidate compresses the operative sentence and not the surrounding intent, and the operative
sentence as written admits the packed column's first element. That is precisely the class of defect
a formalisation dispatch exists to catch before the compression happens.

### 2.2 The repaired condition: sole occupancy

> A lens `(carrier, offset, width)` **degenerates to a value exactly where its focus is the sole
> logical occupant of its carrier allocation.** Padding is permitted; sharing is not. Offset zero
> and exact fit are each neither necessary nor sufficient: a padded sole occupant is a value, and a
> zero-offset shared occupant is not.

`160_probes/p2_lens_degeneracy/lens.rs`, with its case-that-must-fail declared in the header before
the run and firing (`lens_control.err`: `E0080` at `Lens64::<60, 13>::IN_CARRIER`, "lens focus
leaves the carrier", naming the instantiation). What the run establishes (`lens_run.out`):

- The padded sole occupant is an ordinary value: 16 bits for 13 logical, referenceable, `Sized`.
- The zero-offset shared occupant's only standalone form carries its carrier: 8 bytes, pointer-sized,
  not `ceil(13/8) = 2`, and the sibling's bits are observable through the same reference, which is
  the perimeter fact that makes it not a value rather than a stylistic point.
- The degenerate lens and the sole-occupant value agree on all 8192 thirteen-bit values,
  0 disagreements, so the degeneracy is an equality of observations and not an analogy.

**What is cited rather than redone.** That no Rust type has exactly 13 bits is `154` F6
(`154_probes/p2_fibre/`, the committed `E0080` refusal), widened by `159:225-230` to
`W any where W mod 8 != 0` on the size-in-bytes argument. So at every width off a byte multiple, a
value is a *padded* sole occupant or it is not a value, which is why the discriminator has to permit
padding to cover the declared range at all.

**F160-2. The lens degenerates to a value on sole occupancy of the carrier allocation, not on
position zero; an out-of-carrier focus is refused at compile time naming the instantiation.**
`holds for: W = 13, carrier in {u16 sole-occupant, u64 shared}, offset in {0, 13}, F = 0,
signedness = unsigned, toolchain = nightly-2026-05-28, edition 2021, opt-level = 3, threads = 1 for
the run; the compile-time refusal is what rustc accepts and carries threads any on that argument;
target features any for the size_of facts (sizes are ABI, not instruction selection)`. The
discriminator itself is an argument over the language's addressing model, holding wherever
allocations are byte-addressed; the probe is its instance at one width. Evidence:
`160_probes/p2_lens_degeneracy/`.

### 2.3 The lens statement is invariant under the container premise, which is why it can be stated now

`159` section 3 establishes that `156` item 1 splits O-A against O-B and closes O-D. The lens
formulation is untouched by the premise, and the argument is short enough to state exactly:

The lens describes the **form** of the realisation, where the bits rest. The container premise
decides whether that form is part of **identity**: under footprint-observable, two lenses over one
`(V, R)` are two primitives; under footprint-internal, one. Either way every realisation is a lens
and the degeneracy condition reads only the placement, so clause 2 holds on both branches, and only
clause 7's saturation is conditional. That is what makes S-8 formalisable before op rules, and it is
the reason `159` could adopt it while explicitly declining to decide O-A against O-B. [Argument;
nothing to sweep, since the claim is about which clauses read which premise.]

---

## 3. Adequacy, exactly, and the hole in S-14

### 3.1 The defect, located in the converged text

`157:695-701`, S-14's replacement sentence, operative clause: "**Completeness** holds when every
pair of distinct parameter assignments is separated by some input, and a separating witness
discharges one pair at any width."

Set that beside the classification rule the same file proves is the adequacy condition per axis
(`112:934-937`): an axis with **one** direction admitting a total denotation-preserving map "is a
refinement and **may be a parameter**, with the map as its weakening."

A refinement pair is never separated by any input. Structurally: the two assignments share the
realisation map by definition, so no term over any operation set evaluates differently under them,
at any width, ever. Measured: `111:1175-1176`, 1753 declaration pairs changing the selected arm and
zero changing an answer, with the moved-observable-axis control in the tens of thousands. So under
S-14 as written, every refinement parameter fails completeness, and the certificate S-16 proposes
("the gap is the set of carried axes with no separating witness... a design can assert it is empty")
is an assertion no design with refinement parameters can ever pass. The realisation-map topic's
entire mechanism is refinement parameters. **S-14 and the classification rule contradict each other
in the two files' own words, one section apart, and neither noticed**, which is the same shape `157`
section 3.4 found between `111` and `112` one round earlier.

### 3.2 The repair: completeness up to weakening, and the certificate has three outcomes

> **Completeness, up to weakening.** For every pair of distinct shipped instantiations, one of:
>
> - a **separating witness** exists: one input on which the two denote differently. The pair is a
>   real semantic distinction and both names stay. Discharged in `O(1)` per pair at any width.
> - a **weakening exists in exactly one direction**: a total map, identity on the representation,
>   refused at compile time in the other direction. The pair is a refinement pair; two names for one
>   restricted-and-unrestricted view of one denotation, and the order is the repair.
> - **neither**: the pair is connected both ways and separated by nothing. Two names for one
>   denotation with no order between them, which is the spurious split, and the certificate
>   **refuses**.

`160_probes/p1_two_branch_certificate/cert2.rs`, both controls declared in the header before the
run. The run (`cert2_run.out`):

```
P1 declared-semantics pair : witness=true  directions=0
P2 refinement pair         : witness=false directions=1
P3 spurious pair           : witness=false directions=2
witness-only conflates P2 and P3 : true
policy_separates_every_width()   : true
```

The `witness-only conflates` line is the defect of 3.1 as a compiled fact: S-14's scheme returns the
same verdict for the pair that must be carried and the pair that must be refused. The two-branch
scheme separates all three, inside `const` items, and the case that must fail fails:
`cert2_control.err` is `E0080` on `assert!(pair_is_admissible(M, SAT, M, SAT))` under
`--cfg carry_spurious`. The closed-form witness half (add the maximum to one) closes a
`while w <= 64` loop in one `const` item, reproducing `157` F157-6's construction on an
independently written model.

**F160-1. A witness-only completeness certificate cannot distinguish a refinement pair from a
spurious pair, and the two-branch certificate (witness, or weakening in exactly one direction)
classifies declared-semantics, refinement and spurious pairs correctly, at const time, with the
spurious case a compile failure.** `holds for: model W = 6 for the direction count (exhaustive over
the extents), W in 1..=64 for the closed-form witness (the whole domain of the u64 model; Q65's
third state, and I use the reading "exhaustive over the container's domain" rather than any), F = 0,
signedness = unsigned, overflow policy in {sat, wrap}, refinement = one-sided [0, b], operation =
add, arity = 2, term depth = 1 for the commuting check, toolchain = nightly-2026-05-28, edition
2021; the verdicts are what rustc accepts and carry threads any and target features any on the
exact-arithmetic argument`. Evidence: `160_probes/p1_two_branch_certificate/`. The conflation half
is structural (shared `R` admits no witness, argument); the classification half is the sweep.

**Bounds on the instrument, named rather than discovered later.** The commuting check is depth one;
this model's separation structure is depth-one-complete because its only separating witness is
depth-one, and the probe claims nothing past that. The direction count's collapse to extent
inclusion holds for equal value sets, where denotation preservation forces the identity map; with
distinct value sets the map is not forced and the count must be computed as a search, which nothing
here does. And the model collapses the unread rounding parameter into literal identity of
assignments, which is faithful to what an unread parameter is to `R` and is the thinnest possible
spelling of it.

### 3.3 What this changes in the carried sentences, stated so the candidate can compress it

- **S-14's soundness half is carried unchanged.** The factoring formulation, its per-pair scope, and
  the predicate-on-the-certificate clause all stand.
- **S-14's completeness clause is replaced** by 3.2's three-outcome form. The witness discharge and
  its at-any-width property are preserved inside the first outcome.
- **S-16 is restated**: the measurable gap between nominal and denotational identity is the set of
  carried pairs with **neither** witness **nor** one-directional weakening, and that set being empty
  is what a design can assert and the compiler check.
- **S-17's monotonicity claim survives for the witness branch and gains a caveat for the scheme**:
  growing the operation set only adds witnesses, so a separation never degrades; but a spurious
  verdict can convert to a separation (section 4.1), so the *refusal* branch is the one that must be
  evaluated at the maximal set. The certificate never has to be redone for pairs already separated,
  which is what S-17 wanted.

---

## 4. Bounds

### 4.1 The three verdicts age differently, and only spurious is provisional

[Argument.] Fix a pair of instantiations and grow the operation set from `S` to `S' ⊇ S`, members
restricted to functions of `(V, R)` per clause 7's premise.

- **A witness is a term, and a term over `S` is a term over `S'`.** So separation is preserved under
  growth: verdict "declared semantics" is stable. [This is `157` S-17's own argument, carried.]
- **A refinement pair shares `R`.** No term over any `S` evaluates the two differently, so no growth
  produces a witness, and the one-directional weakening is a fact about the extents, which the
  operation set does not touch. Verdict "refinement" is stable.
- **A spurious verdict is a statement about reach**: the two `R`s differ nowhere the current terms
  reach. Growth extends reach, so the verdict can flip to "declared semantics", and by witness
  preservation it can never flip back. `110` F6's rounding-at-`F = 0` case is exactly this flip, at
  the moment a non-grid literal joins.

So the certificate's refusal branch is licensed to **delete the axis from the surface** only where
the two-direction verdict holds at the largest operation set the design will ever admit, which with
a full literal is `R`'s whole domain, and the test for that is the one `110` P5 already built: probe
`R` over the whole line. At the shipped set, a two-direction verdict licenses **gating**, under a
predicate, per `108:827`'s "a licence the resolver may take under a predicate over the chain, not a
reclassification of the axis". This connects the certificate to the degeneracy machinery, which
neither `157` nor `112` had done, and it is the sentence that keeps the certificate from repeating
`110` P4's canonicalisation hazard one layer up.

### 4.2 Soundness's residual obligation, carried with its sharpening

Carried unchanged, counted in section 6: soundness is not enforceable by a signature, because `cfg`
is in scope inside a `const fn` body (`157` F157-13, compiled, rebuilt by `158`), and not
enforceable by anything that inspects one build, because every single build satisfies I15 completely
while the denotation moves between builds (`159` F159-2, which is the sharper form and arrives from
the intent side). The residual obligation is stateable in one sentence and is a lint's shape, not a
type's: **nothing on the realisation-map call path reads `cfg`, a module-level constant, or any
input not in its parameter list.** `109:649-651`'s target-independence clause is this with two
qualifiers dropped, per `157` S-21, and the attribution correction stands: the clause is `109`'s,
not `154`'s (`159` section 2).

### 4.3 Weakening is free at the packed end, which closes the one region clause 3 had no instance in

`111` F111-12 compiled weakening-as-identity for a dense carried range. The packed end had no
instance, and the packed end is the region I17 (`INTENTS.md:363-380`) forbids deprioritising, so the
clause either needed the instance or needed its predicate to exclude the protected region. The
instance: `160_probes/p3_packed_weakening/packed_weaken.rs`, a bitpacked column of 13-bit elements
(no element type anywhere in it, elements are `(column, index)`) carrying a declared per-element
bound as a const parameter.

- Weakening `PackedCol<100>` to `PackedCol<200>` changes no bit and no address: 64 of 64 elements
  read back identically through both types, and the reference survives unchanged
  (`packed_weaken_run.out`).
- Tightening is refused at compile time naming the instantiation:
  `packed_weaken_control.err` is `E0080` at `weaken_ref::<200, 100>`, "weakening must not tighten
  the bound", under `--cfg control`, declared as the case that must fail before the run.

**One declared check could not be performed as planned, and I record it rather than absorb it.** The
probe header declared a symbol-aliasing check on the emitted body. The committed assembly
(`packed_weaken.s`) contains **no body for `weaken_ref` at all**: the optimiser folded every call
away entirely, `#[inline(never)]` notwithstanding, which is consistent with the identity claim and
stronger than aliasing, but is not the evidence the header promised. The run-level facts above are
what the finding rests on, and the assembly is committed as the record of the absence.

**F160-3. Weakening a declared bound on a bitpacked column is the identity on representation and
address, and tightening is a compile-time refusal naming the instantiation.** `holds for: W = 13,
N = 64 elements, container = [u64], declared bound one-sided [0, b] with b in {100, 200}, F = 0,
signedness = unsigned, toolchain = nightly-2026-05-28, edition 2021, opt-level = 3, target =
aarch64-apple-darwin host default, threads = 1 for the run; the refusal is what rustc accepts and
carries threads any on that argument`. Evidence: `160_probes/p3_packed_weakening/`.

### 4.4 What the certificate costs

**Unpriced, and the word is used deliberately.** `157` Q157-E stands exactly as it left it: nothing
on `mock/benches/` measures a const-evaluation budget, my `rustc` invocations here are ad-hoc quick
spikes with no substance as far as magnitude goes, and the per-pair obligation's compile-time cost
over a realistic instantiation count is a harness question with three arms already named (no
certificate, per-axis, per-pair; the two-branch scheme adds a fourth arm to that family, per-pair
with the direction count). Nothing below depends on the magnitude.

---

## 5. What I could not formalise, and why each wall is where it is

### 5.1 The boundary shape at the wall: O-C's residue stands, narrower, and needs a designer

`159:176-185` narrowed O-C to: the sentence says enough about the wall (every implementer meets it,
because it is a fact about the target) and not enough about **the shape at the wall**: a column type
with index accessors and a borrowed `PackedRef<'a, W>` differ at the boundary in ways a consumer
observes, storability, aliasing, lifetime. I attacked this and could not close it, and the reason is
worth exact statement: the equivalence test (`RULES.md`'s two-teams form) quantifies over
**behaviour at the consumer boundary**, and the lens formalisation deliberately says nothing about
that boundary, because saying something would be the concrete spelling the canon may not carry. So
the wall is not a missing formalisation; it is the line between canon and design, and the
discriminator remains the one S-5 named and nobody has run: hand clause 2 to two designers, ask each
for the consumer-facing shape of a 13-bit packed column and a 47-bit dense value, and compare
behaviour. That needs a second designer, not a probe, and I concede it forward exactly as `154` and
`159` did.

### 5.2 The container premise: op's, and every dependent clause is marked

Clause 7's saturation, the identity-relativity of the container, O-A against O-B, and O-D's
dissolution all hang on `156` item 1. `159` F159-3 mapped the dependency exactly and nothing here
extends it. What this file adds is only the invariance argument of 2.3: the premise does not reach
clauses 1 through 6, so the candidate can compress those now and hold clause 7 conditional.

### 5.3 The notation's third state: Q65 stays open, and this file names its own reading

Three predicates in this file quantify `W in 1..=64` by closed form or a const loop. Under the
notation as ratified that spelling is a fixed set; under `159`'s reading it is the whole domain of
the container. I used the whole-domain reading and said so at each site. Whether a proof carries a
different marker from a measurement is Q65's cheaper closing option and it is op's, not mine; the
[argument]/[sweep] tags in section 1 are this file's interim compliance, not a proposal to settle
the marker.

### 5.4 The direction count at distinct value sets

The two-branch certificate's direction count collapses to extent inclusion only where the two value
sets coincide and the identity map is forced. Across distinct value sets (a widening pair, a radix
pair at `F > 0`) the map is not forced and existence is a search. I did not build that search, and
the certificate's classification half is therefore bounded to same-value-set pairs, which is where
every pair the unit argued about lives. The general count is a real hole for whoever formalises the
widening story, and it is named here so it is inherited as an obligation rather than rediscovered.

---

## 6. What I am carrying forward unchanged, and from whom, with the count

**Eleven items, from six authors.**

1. `112:904-906`'s first clause, the primitive as value set plus one realisation map over a declared
   operation set. Clause 1, untouched.
2. `112:934-937`'s classification rule, carried as the per-axis face of adequacy. Clauses 4 and 5.
3. `157` section 3.2's soundness half, the factoring formulation. Clause 4.
4. `157` section 3.6 / F157-11, the obligation is per pair, with region-dependence as the reason.
   Clause 5.
5. `157` F157-13's conclusion plus `159` F159-2's sharpening, soundness needs a lint and no
   single-build inspection can enforce it. Clause 4 and section 4.2.
6. `157` S-6, the arity and standalone-name question is a property of the target's addressing and
   fails the permanence test as canon content. Clause 2's closing sentence.
7. `157` S-10/S-11, the reach theorem with its premise, conditional exactly as stated. Clause 7.
8. `157` S-17's witness-monotonicity argument, carried inside 4.1's first bullet.
9. `111` F111-12, weakening as identity at the dense end. Clause 3's first half.
10. `159` F159-1's corrected count, three instruments not five behind `154` F6, and its
    generalisation that "N bench crates agree" is worth much less than N in this corpus; carried so
    the candidate does not resurrect the five. And the settlement it does not touch: the
    `Copy + 'static` bound is a proof `155`'s instrument could not reach the packed end, which is
    stronger than any count and is what clause 2's non-degenerate half actually rests on.
11. `110`'s definitional-versus-reachability machinery in `111` section 6's one-notion-two-extents
    form, which 4.1 restates through the certificate rather than replaces.

**Amended: 2.** S-8's degeneracy condition (section 2) and S-14's completeness clause with S-16's
gap assertion (section 3). Both amendments are in my file, not theirs, per the rule that a predicate
and a claim are never widened or repaired in place.

**Refuted: 0. Withdrawn of my own: 0**, and I note what `157` noted in the same position: a file
that withdraws nothing has either been lucky or has not built an instrument that could embarrass it.
Mine embarrassed me once, at P3's declared aliasing check, which could not be performed as planned
and is recorded as such rather than quietly replaced (section 4.3).

---

## 7. What only op decides, listed so the candidate inherits the list

1. **`156` item 1**, the operation set the design ships, and with it whether footprint is
   observable. Decides clause 7's saturation, O-A against O-B, and whether the primitive count is
   container-relative. Restated by Q157-A; nothing in this unit closes it and nothing here does.
2. **Q65's marker question**, whether a proof carries a different marker from a measurement. This
   file's [argument]/[sweep] tags are interim practice, not a settlement.
3. **Ratification of any of this.** Everything here is at most TWO EXPERTS on the clauses `158`
   second-read, ONE EXPERT on the two repairs, and the candidate that compresses it is a later
   dispatch's.

---

## 8. Coverage, bounded honestly

**Read in full, this dispatch:** `INTENTS.md`, `RULES.md`, `154` (both phases), `155` (both
phases), `157`, `158`, `159`, `109` (both phases), `113`, `OPTIONS.md` Q52, Q53, Q64, Q65,
`AGREEMENTS.md` sections 6 through 9 plus its heading map, `156` item 1, and `110`, `111`, `112`,
`114` at the sections named inline: `110` phase one in full with its findings block and R0 through
R8; `111` sections 0 through 10, 18, 26 and its findings block; `112` sections 1 and 9 with
F112-1's block and its section map; `114` sections 0, 1 and 8.

**Not read:** `115` through `152` beyond what the above quote; `122`'s ledger itself, which is why
the propagation clauses are pointed at rather than restated; `63`, `74`, `90`, `106`, `108` beyond
the cited lines; everything numbered 1 through 107 except `109`'s account of it; every probe
directory except the files named inline; `PRIOR_CALLS.md`, `PERSONA_CALLS.md`, `DROPLIST.md`,
`HANDLES.md`, `seed/`, `archive/`.

**Which sections would move if something I leaned on were wrong.**

- Section 3.1's structural half rests on "a refinement pair shares `R` by definition". If the
  design's refinement is ever allowed to change `R` (a declared bound that alters saturation
  behaviour inside the extent, say), the no-witness-ever argument fails and the three-outcome
  certificate loses its middle branch's stability. Nothing in the corpus proposes such a refinement,
  and `112`'s definition ("read only by the arm selection") excludes it; the exposure is to a future
  design change, not to a present file.
- Section 4.1's stability argument quantifies over operation sets whose members are functions of
  `(V, R)`. Under the footprint-observable branch of `156` item 1 the quantification is over a
  smaller class than the shipped set, and the "declared semantics is stable" bullet would need
  re-derivation for observation members; flagged rather than resolved, same as everything else that
  premise touches.
- Section 2's discriminator leans on the byte-addressed-allocation model of the target. A target
  with sub-byte addressing would move the boundary between the degenerate and shared cases; clause
  2's closing sentence is written so the canon text survives that, and the probe's predicate names
  the target.
- The test gate leans on three prior members' mechanical scans for the tautology check, named in
  0.2.

**Reproduction.** All three probes' generated outputs were re-run after this file was written and
diffed against the committed copies: `cert2_run.out`, `lens_run.out`, `packed_weaken_run.out` all
reproduce byte for byte; the two `E0080` controls reproduce with identical messages. The gate's
timing-bearing output is not expected to reproduce byte for byte; its pass counts do.

**Citations, checked by opening them.** `160_probes/citecheck.out`: every `file:line` in this
document opened and its content read, with the two deliberate wrong-citation controls firing.

**What I settled.** That S-8's degeneracy condition is sole occupancy, with the compiled instance
and the refusal (F160-2). That S-14's completeness clause as written rejects every refinement
parameter, that a witness-only certificate cannot tell a refinement from a spurious pair, and that
the two-branch form classifies all three pair kinds at const time with the spurious case refusing
to compile (F160-1). That weakening is free at the packed end, closing clause 3's protected-region
gap (F160-3).

**What I moved.** The certificate machinery, from unconnected to the degeneracy extents: the
elimination licence now has its maximal-set condition and its shipped-set licence form (4.1). The
lens, from an account in a reply to a statement with a discriminator, an invariance argument under
op's premise, and its instances committed.

**What I could not.** Sections 5.1 through 5.4: the boundary shape needs a second designer, the
container premise is op's, the notation's marker is op's, and the direction count at distinct value
sets is unbuilt and named as inherited work. I found no way to close any of the four from inside
this dispatch, and each carries what would close it.

**Unlicensed mechanisms.** Checked for and none found that is not already on the record: the
untracked bench lockfiles are `110`'s report, the unfailable stress test's placement is `154`'s,
and the write-contend soundness bug is `154`'s and `157`'s, being handled separately per the brief.
Nothing new observed in this dispatch's surface.
