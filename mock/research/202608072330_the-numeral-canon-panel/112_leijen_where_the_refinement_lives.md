# 112. Where the refinement lives

Fourth and last member of topic five before the checkpoint, so this file owes closure as well as
content. Two cold derivations landed first, `109` and `110`, blind and in parallel. `111` attacked both
and produced a constructive answer with one thing it could not settle, which it named as the sharpest
attack available on its own section 9. That is my question.

I read the panel before deriving. Nothing in this file earns the TWO EXPERTS rung by independence, and
where I reproduce a predecessor's result I say so rather than counting it twice. What is not a
reproduction is stated as such, and there is a fair amount of it, because the question I was handed had
not been asked before.

One thing about provenance at the top rather than buried in a coverage note. `40` and `73` in this panel
are the same persona as this file. `40` is where the observable-against-unobservable cut entered the
panel, and `108` section 3.1 records that `106` misattributed that definition to `97` when it is `40`'s.
I build on `40`'s definition in section 3, and an agreement between this file and `40` is one approach
applied twice, not two experts.

## 0. The two gates

### 0.1 Canon gate: passed

Checked against `INTENTS.md` read with its own "How to read an entry" section as normative, and against
`RULES.md`. Nothing in I1 through I18 forbids the question, presumes an answer to it, or is contradicted
by anything I build.

Four intents do load-bearing work below and I name them here rather than smuggling them in.

**I13** (`INTENTS.md:214`, the one RATIFIED entry) governs the shape of everything: the answer below is
two arms with two predicates rather than one rule, and section 8 is the third arm arriving from the
composite side.

**I15** (`INTENTS.md:299`), never a runtime check, is what makes the compile-time refusal in section 4
the only admissible form of the obligation a declared refinement introduces. `p4c` is that refusal,
compiled.

**I17** (`INTENTS.md:363`), the storage-minimising path is not deprioritised, is what makes section 7's
result bite rather than being a curiosity. The one place a missed merge has no repair is the storage
boundary, which is the path I17 protects.

**I5** (`INTENTS.md:110`), the speed-first concern may sacrifice soundness for a proven meaningful gain,
bounds what may be done with an undischargeable declaration, and section 12's alternative B says why
nothing here licenses that arm yet.

Every probe stays inside I14's operating constraints. **Zero `#![feature(...)]` gates across all three
Rust probes**, checked with `grep -c '^#!\[feature'` and recorded in `p4_output.txt`. No `dyn`, no
`TypeId`, no `alloc` in the modelled design; `std` appears only to print.

### 0.2 Test gate: passed, at 123 across 13, and it is the eighth count

I ran every test-bearing bench variant crate, per crate, serially. Full transcript with the toolchain
line at `112_probes/p0_test_gate_run.txt`.

```
grep -rnE '^[[:space:]]*#\[test\][[:space:]]*$' --include='*.rs' variants/ | grep -v '/target/' | wc -l
123
```

Per crate: `wide-rung-shared` 30, `bitpack-write-contend-shared` 15, `warm-container-shared` 15,
`bitpack-contend-shared` 12, `satfold-shared` 11, `bitpack-carrier-shared` 9, `warm-clamp-shared` 7,
`bitpack-wide-shared` 6, `bitpack-footprint-shared` 6, `bitpack-plan-shared` 5, `bitpack-shared` 3,
`quantiser-radix-shared` 3, `quantiser-fadd-shared` 1. Sum 123, all passing, zero failures, zero ignores.

**One thing cost me time and is worth writing down for the next member.** The package names carry a
`bench-` prefix that the directory names do not: `cargo test -p satfold-shared` returns "package ID
specification `satfold-shared` did not match any packages", and `-p bench-satfold-shared` is the working
form. My first gate run reported thirteen errors and zero tests, which is the shape of a gate that
passes vacuously if nobody reads the output. Both runs are in `p0_test_gate_run.txt`.

**`wide-rung-shared` runs its 30 tests in 4.38 seconds of test time on this host**, 6.55 wall including
the build. That is a fourth measurement after `110`'s 4.05, `111`'s 4.25 and a third under five seconds,
and it agrees. The 107-second figure a previous brief attributed to that crate is dead and `OPTIONS.md`
already records it as dropped.

**On the surface I touch, the suite is real.** `satfold-shared/src/lib.rs:519` and `:547` are two const
fns that exhaustively sweep associativity for saturating addition and saturating subtraction, and the
second exists to return `false` from the identical sweep so the negative arm is a control rather than a
stub, which its own doc comment says in those words. `warm-clamp-shared/src/lib.rs:1105`'s
`clamping_is_a_retraction_on_non_negative_addition_at_every_swept_width` asserts a mathematical property
across the whole swept matrix rather than at a chosen point, and
`warm-clamp-shared/src/lib.rs:1161`'s `the_shipped_container_fails_interior_safety_at_arities_the_design_expects`
asserts that the shipped container **fails** a property, which is a test that would break if the code got
accidentally better. That is the opposite of the failure this gate exists to catch.

**One nit, `109`'s, which `111` confirmed and I confirm as the third reader.**
`quantiser-radix-shared/src/lib.rs:370` and `:372` assert `p % 2 == 1` twice in two spellings. Redundant
rather than tautological, since either can fail. Not a gate failure.

**What I did not do.** I did not audit the ten crates outside my surface beyond running them. Anyone
reading my gate result as a census is reading a sample as one, and three members have now sampled the
same three crates.

---

## 1. The answer, stated first

> **The refinement is neither of the two things `111` could not choose between.** It is not a coordinate
> of the primitive, and it is not a member of the strategy pair's first component. It is the carrier for
> a licence the previous unit already placed outside component one, and `108` section 7 already ruled
> that taking such a licence is "not a reclassification of the axis" (`108:827`).
>
> **Why it is not a coordinate.** A coordinate, moved, yields another primitive. A refinement moved does
> not yield another primitive, because the restricted carrier is not closed under the operations, so it
> is not an algebra, so under `110`'s own definition there is no primitive there to be a coordinate of.
> `111` measured that and read it as a defect in its hypothesis. It is the proof.
>
> **Why it is not component one.** Component one is defined by changing what the program denotes.
> Measured across every axis and every extent I swept, a discharged declaration changes no denotation at
> all: it changes which arms are available. And component one's stated justification, that consumers
> "must agree about it" (`108:825`), fails for a refinement, because two consumers holding different
> declarations of the same value both compute correctly whenever one weakens to the other.
>
> **What it is instead.** A **grade** over a fixed primitive and a fixed observable assignment: a
> declared restriction on where an operation's arguments land, ordered by weakening, transformed rather
> than preserved by each operation, and read only by the arm selection. `108` needed a const-checkable
> per-chain predicate and put it in the audit trail because the canon sentence could only say the licence
> exists. The refinement is that predicate given a home on the value, which is the one place it can
> survive a storage boundary.

Five further results, each of which breaks something and replaces it.

**`110` F8's "no repair" is wrong at a function boundary and right at a storage boundary**, compiled both
ways, and that is what dissolves `110`'s internal contradiction without either sentence having to lose.

**`111` F111-9's exactness is a property of the term shape it was swept over**, not of the propagated
bound. On a term with a correlated leaf the same rule is conservative on 120 of 136 available licences.
Soundness is unaffected and is zero-unsound on every shape I swept.

**The grade has two parts and each switches off one region of `R`.** A magnitude bound switches off the
completion, a grid bound switches off the rounding, and neither touches the other. That is `110` section
2's "two regions of one map" arriving on the licence side, and it explains why `111`'s two arms carried
two different propagated quantities.

**A declared extent discharges a construction's base predicate**, which is the connection between
`110`'s untouched composite results and `111`'s refinement. And the lifting rule is per construction: the
componentwise rule applied to complex multiplication is **unsound on 26 of 81 pairs**, which is a hazard
the design would otherwise walk into.

**And the propagation rule is not one rule.** The interval rule and an affine rule each recover what the
other loses, neither dominates, and their disjunction is sound and **reaches an enumerating oracle on ten
of ten term shapes swept**. Both are const-computable, both are compiled here with no feature gate, and
that is I13's arm structure appearing inside a mechanism rather than across one.

The rest of this file is that taken apart. Sections 3 to 8 are the working, section 9 is the statement
offered, and sections 10 onward are bookkeeping.

---

## 2. What I kept, and what is a reproduction rather than a finding

Keeping is a result, so the ledger before the disagreements. I went looking for reasons to break each of
these.

**`110`'s realisation map as one map with two regions.** Everything I built assumed it and nothing
needed it split. Section 5 turns it into a prediction that held: if `R` is one map with two regions, a
bound on `R`'s argument should switch off one region at a time, and it does. Fourth instance after `63`
C1, `110` and `111`; not independent, since I read all three.

**`109`'s const-availability criterion for membership, and `110`'s isomorphism criterion for identity.**
`111` section 8 says these answer different questions and both survive. I agree and add nothing except
section 3.1, which uses `110`'s criterion to settle a question `111` asked without noticing its own tool
answered it.

**`109` section 8's non-endomorphism move** (`109:424`). `111` applied it one level down to repair the
extent. Section 8 applies it one level up, to the composite layer, and it works there too. That is now
three levels of the same move and it is `109`'s.

**`111`'s propagated grade**, in the region `111` swept. p2b reproduces zero unsound and zero
conservative on an independent implementation with a conservative counter that fires under mutation.
**This is a reproduction and not a second instance**, because I read `111` first; what section 6 adds is
the boundary, which is new.

**`110` F12's monotonicity predicate for the interval construction.** p5 reproduces its shape on my own
bases, 16 of 16 against 8 of 16 for the no-precondition hypothesis. The agreement of the counts is a
coincidence of how many bases I happened to sweep and should not be read as reproducing its numbers.

---

## 3. Settling `111`'s alternative E: the refinement is neither

`111` section 12, at `111:951`:

> **E. Asking whether the refinement is a strategy.** `106` section 1's first component is "an assignment
> on the axes a consumer can observe, supplied and never derived", which is what a declared range is. If
> the refinement is one of those axes, then the strategy pair already contains it [...] What would decide
> it: whether a declared range is recoverable from the bits. It is not, which is the pair's own criterion
> for component one, so **I lean toward it being one of those axes**.

The lean is wrong, and the reason is visible in the sentence it quotes. Both halves of the question are
wrong: it is not a coordinate either.

### 3.1 The coordinate reading dies on `110`'s own criterion, and `111` had the measurement

`110` section 3 defines identity as denotation-preserving isomorphism over a declared signature. A
coordinate of a primitive is something that, moved with the others held fixed, yields **another
primitive**. So the question "is the refinement a coordinate" reduces to "does a restricted carrier
carry an algebra".

`p1` asks that directly, by testing for a total denotation-preserving map between the restricted thing
and the base, which is forced (denotation preservation determines the map, so nothing has to be hunted)
and is a homomorphism exactly when the two interpretations agree on the smaller carrier.

```
extent v <=  0 (size  1)   count 1   widen True  tighten False closed True   disagree 0/3
extent v <=  1 (size  2)   count 0   widen False tighten False closed False  disagree 1/12
extent v <=  7 (size  8)   count 0   widen False tighten False closed False  disagree 48/192
extent v <= 15 (size 16)   count 0   widen False tighten False closed True   disagree 420/768

  widening direction exists for bounds: [0]
  extents closed under the operations:  [0, 15]
  extents where BOTH hold:              [0]
```

At `W = 4` unsigned saturating under `{add, sub, mul}`, **the only extents carrying an algebra are the
trivial one and the whole carrier**. That is `111` F111-8 arriving from a second instrument, and `111`
recorded it as the falsification of its own hypothesis (`111:626`, "largest sound declared bound: 0").

**`111` read that as a defect to repair. It is the answer to the question `111` asked eight sections
later.** A restricted carrier is not an algebra, so there is no primitive there, so the refinement is not
a coordinate of one. The coordinate reading is not merely unsupported; it is refuted by the measurement
already in the panel.

I would not have this recorded as two instances. It is one measurement seen twice, and the second sight
is what matters rather than the sighting.

### 3.2 The membership reading dies on the clause it quotes, which states a premise as if it were the criterion

`108` section 7's clause, at `108:823-827`:

> It is supplied and never derived, **because** a consumer of a value cannot recover it from the bits,
> **so** every consumer of that value must agree about it. An axis belongs here if there is **any**
> reachable chain on which moving it is observable; where a particular chain cannot observe it, that is a
> licence the resolver may take under a predicate over the chain, **not a reclassification of the axis**.

The sentence has a premise, a conclusion, and then a separate criterion. `111` applied the **premise**
("cannot recover it from the bits") as the test. The premise is necessary and is not sufficient, and
`108` itself knows this: at `108:442` it argues that if the weighting were written at the site then
component one would not be on the value and "every consumer of that value must agree about it" **fails**.
So `108` already treats the agreement requirement as the thing that decides membership, and the
un-recoverability as the reason for it.

The two come apart on exactly the object under discussion, and it is measurable. `p2b` asks whether two
consumers holding different assignments of an axis compute different answers on the values both can hold:

```
  axis moved                grade                    disch   term differs  types merge
  overflow policy           magnitude <= 7            True           0/64        False
  overflow policy           magnitude <= 15          False        120/256        False
  total width 3 vs 4        magnitude <= 3            True           0/16        False
  total width 3 vs 4        magnitude <= 7           False          28/64        False
  signedness                magnitude <= 3            True           0/16        False
  signedness                magnitude <= 7           False          28/64        False
```

A declared refinement is not recoverable from the bits, and consumers holding different ones **agree**,
provided the declaration is discharged. So the refinement satisfies the premise and fails the criterion,
which is precisely what a necessary-but-not-sufficient condition looks like when somebody uses it as a
test.

**And `p4` shows the same thing in the compiler, which is where the difference is spendable.** Widening
a declared bound is one function that compiles to nothing:

```
_cast_radix_10_to_2:  ret
_cast_radix_2_to_10  = _cast_radix_10_to_2
_plain_identity      = _cast_radix_10_to_2
_widen_100_to_200    = _cast_radix_10_to_2
_widen_7_to_255      = _cast_radix_10_to_2
```

Five symbols, one body, and `plain_identity` is the control that makes it an observation rather than a
coincidence. A consumer receiving a value declared `<= 100` may treat it as `<= 200` and is right at no
cost. A consumer receiving a value declared saturating may not treat it as wrapping: the cast exists, and
`p4` measures that it fails to commute with the operation on **120 of 256 pairs**, so it reinterprets
rather than maps. That is the difference between an axis consumers must agree about and one they need
only under-claim about.

### 3.3 The square the pair's definition already contains, with one cell forbidden

The pair's own definition supplies two independent binary distinctions and populates two of the four
cells. Component one is supplied by the consumer and changes what the program denotes. Component two is
resolved by the compiler and preserves it. Written out:

| | changes the denotation | preserves the denotation |
|---|---|---|
| **supplied by the consumer** | component one, the declared semantics | **the refinement** |
| **resolved by the compiler** | forbidden | component two, the weighting |

The third cell is where the refinement lands, and it is the cell the pair never named. It is supplied,
because nothing derives a declaration from the bits, and it preserves the denotation, because a
discharged declaration selects an arm computing the same values.

**The fourth cell is a prohibition rather than an absence**, and naming it that way is the check that the
square is complete. Nothing may silently change a denoted answer without the consumer having asked, which
is I9 read with I15, and is `108`'s own equivalence clause ("nothing that changes a denoted answer is
decided by a timing", `108:869`). I5 is the interesting near-miss and it does not populate the cell: the
speed-first concern may sacrifice soundness, and `110` section 6 already argued the licence must be
"declared and scoped". A declared licence is supplied. It sits in the first column.

**And a square with every cell filled would carry no information, which my own persona established at
`40`.** `40_probes/p1_the_two_by_two_carries_no_information.py` enumerates every assignment of four
labels to a two-by-two and finds that **24 of 24 placements are exact bijections with every cell filled
and none repeated**, so "the four names fill the grid exactly" is `4 = 2 * 2` restated and distinguishes
the observed table from no other. I am not making a claim of that shape and the difference is the point:
this square has **three cells populated and one forbidden**, and the forbidden cell is what carries the
content. If all four were filled the square would be decoration, and `40`'s probe is the reason I checked
before drawing it.

### 3.4 So the refinement is the carrier for a licence `108` already placed outside component one

`108` section 3.1 built exactly this licence and could not put it anywhere durable. Its predicate is
three static bits per operation plus a three-state scan of the chain, checked exhaustively:

```
| setting                                | chains | value tuples | exact | conservative | unsound |
| W=4, containers 4 against 12, length 3 |    729 |        65536 |   701 |           28 |       0 |
```

That instrument reads **the chain's shape**. Mine reads **the values' declared extents**. They decide the
same question, may this arm be substituted, and they have different domains and different costs:

- `108`'s is conservative by 28 of 729 and its conservatism is structural, since no fixed set of
  per-operation bits sees an operation that re-synchronises two diverged accumulators.
- Mine is exact on left-nested single-operation chains with independent leaves and conservative
  elsewhere, by amounts section 6 measures.
- **`108`'s does not survive a storage boundary and mine does.** A chain scan is a property of a chain,
  and at a column there is no chain. A declared extent is on the value.

That last point is the one that decides where the refinement belongs, and it is `109`'s perimeter rule
(`109:376`, "the guarantee's perimeter is the construction site") applied to the licence rather than to
the range: the licence has to be established where the value is made, or it is not available where the
value is used.

So the answer composes with the previous unit rather than adding to it. `108` said a per-chain licence is
not a reclassification of the axis. I am saying the refinement **is** such a licence, given a carrier, so
it is not a reclassification either, and the pair stands unchanged with one clause repaired.

### 3.5 What would decide it against me, stated because I could be wrong

Three things, in order of how much they would cost.

**If a declared refinement ever changes a denoted answer while discharged**, the whole placement moves
and it belongs in component one. I swept for that on six axes and found zero across every discharged
cell in `p2b` and `p4`. The sweep is at `W <= 6` and I have no transfer argument to real widths.

**If the agreement property fails for some axis I did not sweep**, the criterion in 3.2 is not the one
that decides. I swept the overflow policy, the total width, the signedness, the rounding mode and the
fraction grid. I did not sweep a non-uniform value set, which `110` names as its own largest gap and
which is where arvo's float side lives.

**If a design finds it needs the refinement at rest, in the layout rather than in the type**, the
carrier question reopens, because a value's bits then do partly determine it. Nothing here touches that
and `109`'s W1 witness (denotationally same, representationally different) is where it would be attacked.

---

## 4. The direction count, which classifies an axis without a list

`110` F9's rule is "an axis that the value set and the realisation map do not read must not be a type
parameter" (`110:395`). It is right and it distinguishes two cases where there are three, so a design
applying it to a refinement gets the wrong answer: `R` does not read a declared bound, so the rule says
drop it, and `111` F111-12 shows keeping it is free and useful.

The repair is to count rather than to ask a yes-or-no question. For two configurations differing in one
axis, count **how many of the two directions admit a total denotation-preserving map that commutes with
the operations**. The count is 0, 1 or 2 and it is computable by enumeration, since denotation
preservation forces the map.

`p1`, single-axis moves at `W = 4`, `F = 0`, unsigned, saturating, truncating, radix 2 unless the row
says otherwise:

```
  radix 2 vs 3, F = 0             count 2   A<->B    consumers disagree on   0/768
  rounding trunc vs near, F = 0   count 2   A<->B    consumers disagree on   0/768
  radix 2 vs 3, F = 1             count 0   neither  consumers disagree on  30/108
  overflow policy sat vs wrap     count 0   neither  consumers disagree on 416/768
  rounding trunc vs near, F = 1   count 0   neither  consumers disagree on  12/768
  total width 3 vs 4              count 0   neither  consumers disagree on  61/192
  fraction width 0 vs 1           count 0   neither  consumers disagree on  61/192
  signedness unsigned vs signed   count 0   neither  consumers disagree on  89/192
```

and the count agrees with the consumer-disagreement reading on **24 of 24 cells**, with a mutation
confirming the verdict moves when the thing it tests for is removed (forcing the two policies to share
one completion moves the count from 0 to 2).

The reading:

> **Two directions: the axis is spurious.** Two types differing only in it denote the same thing, so what
> is wanted is equality and no language gives it. Do not make it a parameter. This is `110` F9's case and
> `110` F9 is right about it.
>
> **One direction: the axis is a refinement.** Two types differing only in it denote nested sets, so what
> is wanted is not equality but weakening, and weakening is total, free, and refused in the wrong
> direction. This is `111`'s case, and `110` F9 misclassifies it.
>
> **Zero directions: the axis is observable.** No total map either way. It must be a parameter, and
> nothing coerces.

The count is checked against the compiler in `p4`, on three axes carried three ways in one file:

```
  spurious (radix)     : the cast commutes with add on 65536/65536 pairs, both directions exist
  observable (policy)  : the cast commutes with add on   136/256 pairs
  refinement (bound)   : widening is the identity on 101/101 representations, and the tightening
                         is refused before the program exists
```

and the tightening's refusal, from `p4c`, names the instantiation:

```
error[E0080]: evaluation panicked: widening must not tighten the declared bound
   evaluation of `Widen::<Lit<200>, Lit<100>>::CHECK` failed here
```

which is I15's shape: the invalid never becomes a runtime concern because it never becomes a program.
That reproduces `111` F111-12 on an independent construction. It is a reproduction, not a second
instance, because I read `111` first.

**One incidental cost of the refinement carrier, worth a line because a canon reader will meet it.**
`#[derive(Copy)]` on a type with a phantom parameter adds an implicit `B: Copy` bound, which a marker
carrying only an associated const does not satisfy. The impls are hand-written once at the definition
rather than at every use. `p4` carries the note at its own definition site.

---

## 5. The grade has two parts, and each switches off one region of `R`

`110` section 2 argues that overflow and rounding are not two mechanisms but two regions of one map:
rounding acts between grid points, completion acts outside the range. If that is right, and if a grade is
a bound on where `R`'s argument lands, then a grade should switch off one region at a time depending on
what it bounds. `111`'s F111-9 and F111-10 carry two propagated quantities, a magnitude and a fraction
width, and `111` states them as "two arms with two predicates rather than one mechanism wearing two
names". They are one mechanism with two parts, and the prediction is testable.

`p2b`, at `W = 6`, `F = 2`, `mul`, arity 2:

```
  axis moved              grade                     disch   term differs
  rounding trunc vs near  magnitude <= 3, grid 1/4   True         36/169
  rounding trunc vs near  grid 1 (integers)          True           0/16
  overflow policy at F=2  grid 1 (integers)          True           0/16
  overflow policy at F=2  grid 1, grid part only     True        180/256
```

Read the four rows as a two-by-two. A magnitude bound alone does not license the rounding substitution
(36 of 169 differ). A grid bound alone does not license the completion substitution (180 of 256 differ).
Each licenses the other's region. The third row has both parts and licenses both.

So the two arms `111` reports are the two parts of one declaration, and a design carrying only a
magnitude bound has a mechanism that works on exactly half of `R`. That is worth knowing before the
parameter list is chosen, since `110` F8's whole point is that the parameter list is decided once.

**And the licence is denotational, never representational.** The same probe, unsigned against signed at
`W = 4`:

```
  unsigned vs signed, bit patterns whose value differs: 8/16
  unsigned vs signed, arithmetic on a discharged extent: 0/16
```

Half the bit patterns denote different numbers under the two readings, and the arithmetic agrees
completely inside a discharged extent. So a grade licenses substituting the **operation** and never
reinterpreting the **bits**, which is `109` section 10's three-relation lattice with the grade applied to
it: the licence reaches the denotational relation and stops there.

**This refutes an expectation `111` recorded about its own result.** `111` section 9.6: "whether every
axis of `R` has a propagable quantity is untested and I would expect the signedness not to". A magnitude
bound licenses the signedness substitution on a term, measured at 0 of 16 differing on a discharged
extent and 28 of 64 on an undischarged one. It also licenses the total width the same way. The
expectation was reasonable and it does not hold, and the reason it does not is that the grade acts on
`R`'s argument rather than on the axis, so it does not care which axis the argument's fate depends on.

---

## 6. Where `111`'s exactness stops, which nobody had bounded

`111` F111-9 and F111-10 report "zero unsound and zero conservative" over four sweeps, and `OPTIONS.md`
Q52 carries that as "exactly, zero unsound and zero conservative". `p2b` reproduces it: zero and zero
over six settings, with a mutation showing the unsound counter moves from 0 to 120 when the propagation
rule is broken.

**The zero-conservative half is a property of the term shape, not of the rule.** Every sweep in `111` and
every sweep in my `p2b` has one shape: a left-nested chain of one operation over independent leaves. On
that shape the corner rule is tight by construction, because the propagated interval's endpoints are
attained by taking each operand at its own endpoint, so a conservative verdict is impossible before any
arithmetic happens. Reporting the zero without the shape reads as a property of the mechanism, and a
design relying on it needs to know which terms it holds for.

`p3b` sweeps other shapes. The oracle is a rule that enumerates the reachable set, which is exact and is
not available as a const predicate at a real width.

```
CONTROL: independent leaves, one operation, left-nested
  x + y, unsigned sat W=4          unsound 0  conservative   0  exact  256  of  256 extents
  (x + y) + z, unsigned sat W=4    unsound 0  conservative   0  exact 4096  of 4096 extents
  x * y, unsigned sat W=4          unsound 0  conservative   0  exact  256  of  256 extents

THE DEPENDENCY PROBLEM: a leaf mentioned twice
  (x + y) - y, unsigned sat W=4    unsound 0  conservative 120  exact  136  of  256 extents
  x * (y - y), SIGNED sat W=4      unsound 0  conservative  33  exact   31  of   64 extents

MIXED OPERATIONS with independent leaves
  (x + y) - z, SIGNED sat W=4      unsound 0  conservative   0  exact  512  of  512 extents
  (x + y) * z, unsigned sat W=4    unsound 0  conservative 120  exact 3976  of 4096 extents

THE ORACLE
  (x + y) - y, unsigned sat W=4    corner licenses   16/256, reachable-set licenses  136/256
  x * (y - y), SIGNED sat W=4      corner licenses   31/64,  reachable-set licenses   64/64
  (x + y) * z, unsigned sat W=4    corner licenses  385/4096, reachable-set licenses 385/4096
```

Three things fall out, and the third is the one a canon has to carry.

**Unsound is zero on every shape swept**, which is the property that matters. The corner rule
over-approximates, so it can lose an optimisation and cannot license a wrong answer.

**Correlation is the first source of conservatism, and it is large.** On `(x + y) - y` the corner rule
licenses 16 of the 136 extents on which the cheap arm is in fact correct, so it loses **88.2% of the
available licences**. The mechanism is the classical interval-arithmetic dependency problem: the two
occurrences of `y` are treated as independent, so the propagated lower bound goes negative while the
reachable result is always `x`. A tighter rule that tracks correlation recovers all 136.

**Downstream insensitivity is a second source and no node-wise rule recovers it.** On `(x + y) * z` with
`z` declared zero, the inner node genuinely overflows and the final result is zero either way. The corner
rule refuses, the reachable-set oracle also refuses (385 of 4096, identically), and the arms agree
anyway on 120 extents. That conservatism is not about reachability at all; it is about the term's result
not depending on the node, and catching it needs reasoning about the whole term rather than about any
node.

**And a repeated leaf is not sufficient on its own.** `x - x` at signed `W = 4` has conservative 0,
because its propagated interval never leaves the container, so there is nothing to be conservative about.
The condition is a repeated leaf **whose lost correlation crosses a container edge**, which is narrower
than the shape and is why the honest predicate names the shape rather than the syntax.

This does not weaken `111`'s finding. It bounds it, which is what I13 asks of every finding, and the
bound is good news for the design: the shape on which the rule is exact is the fold, which is the shape
`satfold` and `warm-clamp` are both built around.

---

## 6b. Attacking that blocker: the correlation loss is recoverable, and the two rules compose

Naming a blocker and leaving it is not a deliverable, so this section is the attack on section 6's
result. The question: is a correlation-tracking rule expressible under the operating constraints, and does
it recover what the corner rule loses?

**The candidate is an affine grade.** Carry a linear form `c0 + sum ci * ei` with one noise symbol per
declared leaf, rather than an interval. Addition and subtraction are exact on that representation because
they are linear, so two occurrences of one leaf carry the same symbol and cancel. Multiplication of two
non-constant forms is not linear and contributes a fresh symbol, which is the standard affine-arithmetic
treatment and is sound rather than exact. The interval is recovered by summing the absolute coefficients,
so the discharge test is unchanged.

`p7` measures it against the corner rule and against the enumerating oracle, per node in both cases:

```
CONTROL: shapes where the corner rule is already exact
  x + y                 corner   136/256    affine   136/256   oracle   136/256   unsound c=0 a=0
  (x + y) + z           corner   816/4096   affine   816/4096  oracle   816/4096  unsound c=0 a=0
  x * y                 corner    76/256    affine    31/256   oracle    76/256   unsound c=0 a=0

THE CASES THE CORNER RULE LOSES
  (x + y) - y           corner    16/256    affine   136/256   oracle   136/256   unsound c=0 a=0
  x * (y - y)  SIGNED   corner    31/64     affine    64/64    oracle    64/64    unsound c=0 a=0
  (x + y) * z           corner   385/4096   affine   151/4096  oracle   385/4096  unsound c=0 a=0

WHERE THE AFFINE RULE IS ITSELF CONSERVATIVE
  (x+y) * (z+w)         corner   212/256    affine    31/256   oracle   212/256   unsound c=0 a=0
```

**The affine rule recovers the correlation loss completely and loses badly on multiplication.** On
`(x + y) - y` it goes from 16 to 136, which is the oracle exactly. On `x * (y - y)` it goes from 31 to 64,
again the oracle. And on plain `x * y` it drops from 76 to 31, and on `(x+y) * (z+w)` from 212 to 31.

**The mechanism for the losses, named rather than left as a number**, because a design has to know when
not to reach for it. An affine form centres `[0, b]` at `b/2` with radius `b/2`, so it is symmetric about
its centre, and the product of two symmetric forms carries a negative lower bound the interval rule never
had. Affine arithmetic trades sign information for correlation information, and on a non-negative domain
that is a bad trade whenever a multiply is present and a good one whenever a leaf repeats. A mutation
confirms the mechanism: giving the two occurrences of `y` different symbols drops the affine rule back to
16 of 256, matching the corner rule exactly.

**So neither rule dominates, and the deliverable is the composition rather than a winner.** Both rules are
sound on every row, so their disjunction is sound, and `p7b` measures it:

```
  rows swept                                     : 10
  rows where the union is unsound                : 0
  rows where the union reaches the oracle        : 10/10
  rows where affine licenses what corner refuses : 4/10
```

**Two const-computable rules, disjoined, match an enumerating oracle on every one of ten term shapes**,
including a wrapping base and two signed ones. The union never exceeds the larger of the two counts, which
says the licence sets are nested per row rather than complementary within a row, so a design could equally
select per term shape. Disjoining is the cheaper arm because it needs no shape analysis: evaluate both
const predicates and take either.

**And the residue is the annihilation case**, checked directly since section 6 predicted it. On
`(x + y) * z` with `z` declared zero, the union licenses 136 of 256 while the arms agree on all 256, short
by 120. Neither rule reaches it and no node-wise rule can, because the fact is that the term's result does
not depend on the node rather than that the node's range is small.

### 6b.1 It compiles, and the wall on the way there is the one the workspace already names

An expressibility claim nobody compiled is exactly what this panel has been burned by, so `p8` builds it.
The obvious spelling puts the coefficient vector in an associated const array whose length is another
associated const, which needs arithmetic in type position and therefore the forbidden feature.

I reached for a type-level list, which is the right move, and then **put the coefficient in a const
argument**, so `Cons<{ A + B }, ..>` asked for `generic_const_exprs` anyway:

```
error: generic parameters may not be used in const operations
   |     type Out = Cons<{ A + B }, <S as AddC<T>>::Out>;
   = help: add `#![feature(generic_const_exprs)]` to allow generic const expressions
```

That failure is committed rather than deleted, because half-applying the reflex is easy and lands on the
forbidden feature with a clean conscience. The repair is to stop making the coefficient a const argument
at all: **a coefficient is a type carrying an associated const**, so every arithmetic operation happens in
an impl body where arbitrary const expressions are legal, and nothing arithmetic appears in type position.
That is exactly the construction `109` P5 uses for a scalar bound, applied to a vector.

`p8b` compiles that with **zero feature gates**, and cancels in the type: `(x + y) - y` composed through
the grade types resolves to a radius of 7 where `x` alone has radius 7, against the corner rule's
`[-14, 28]`.

**And `p8b`'s gate was wrong, which `p7c` had already established.** Its `Discharges` tests one interval,
the root's, and `p7c` builds a hand witness where a root-only test licenses an arm that computes the wrong
answer: over unsigned saturating `W = 4` with `x` in `[8, 10]`, `y` in `[8, 10]` and `z` pinned at 15, the
root propagates to `[1, 5]` and fits while the intermediate `x + y` propagates to `[16, 20]` and does not,
and the two arms disagree on **9 of 9** tuples inside the declaration.

`p8c` recurses the check over every node, which is expressible for a reason worth stating on its own:
**a composed grade is a composed type, so the structure the check has to walk is the structure the type
already has.**

```
  WIDE declaration, x and y in 0..=14
    x + y            -> [  0,  28]   root-only false   per-node false
    (x + y) - y      -> [  0,  14]   root-only true    per-node false
    corner, same term-> [-14,  28]   licenses  false

  NARROW declaration, x and y in 0..=6
    x + y            -> [  0,  12]   per-node true
    (x + y) - y      -> [  0,   6]   per-node true   radius 3
    corner, same term-> [ -6,  12]   licenses  false
```

The wide row is `p8b`'s defect caught by the fix. The narrow row is the point: **the affine advantage
survives the per-node discipline**, because the corner root keeps both occurrences of `y` and lands at
`[-6, 12]`, whose lower bound is outside the container, while the affine root cancels them and lands at
`[0, 6]`.

And the licence costs nothing at runtime, with both controls aliasing:

```
_affine_gated_diff:                _corner_gated_diff:
	add	w8, w1, w0                     and	w8, w0, #0xff
	sub	w0, w8, w2                     add	w8, w8, w1, uxtb
	ret                                mov	w9, #255
                                       cmp	w8, #255
_bare_diff       = _affine_gated_diff  csel	w8, w8, w9, lo
_general_diff    = _corner_gated_diff  subs	w8, w8, w2, uxtb
                                       csel	w0, wzr, w8, lo
                                       ret
```

The two aliases are what make this a comparison. `bare_diff` is the ungated `(a + b) - c` and
`general_diff` is the ungated saturating chain, and the licensed arm is the first while the refused arm is
the second, so the const gate erases in both directions. Two instructions against seven is an ad-hoc quick
spike as far as magnitude goes and prices nothing.

### 6b.2 One limitation this exposed, which applies to every sweep in this file

`p7b`'s mutation, which asked whether a root-only check is as good as a per-node one, measured **0 unsound
over 4096 extents** and read as the per-node discipline being unnecessary. It could not fire. Every
declared extent in every probe here has the form `[0, b]`, and with every lower bound pinned at zero the
root of an addition chain over a non-negative domain is the widest node, so a root that fits implies every
node fits and the two checks are equivalent by construction.

**So every count in this file is predicated on one-sided extents.** `111` section 12 alternative B names
two-endpoint windows as untested and strictly more expressive, and `82` F6's sign-uniform window actually
uses them. `p7c` builds its witness with a two-endpoint extent for exactly that reason, and it is the only
place in my probes where one appears.

---

## 7. `110`'s contradiction, priced by where the two spellings meet

`110` says both of these about one act:

> A canonicalisation that **splits** where it could have merged costs names and nothing else.
> (`110:282`)

> in a nominally typed language a missed merge is a wall rather than a slow path [...] **And there is no
> repair.** (`110:357`, `110:370`)

`111` section 7 calls this a contradiction and locates the resolution in `110`'s own F9: parameterise by
what `R` reads, and the second spelling never exists. That is right about **what to build** and it does
not price **the act**, which is what the two sentences disagreed about. `110:540` half-notices, saying
the first line is a soundness statement rather than a cost statement and that the compile-time cost is
unmeasured.

**Neither sentence has the right unit.** The cost of a missed merge is not a property of the split. It is
a property of **where the two spellings meet**, and there are three sites with three answers.

**A monomorphic site: zero.** Each spelling has its own call and neither mentions the other. `p4`'s two
arrays go through the same generic function and neither call names the other's spelling.

**A polymorphic site: repairable, and `110` is wrong that it is not.** `110`'s "no repair" is about making
two type constructors applied to different arguments into one type, which is correct and is not the only
way to write one function over both. Abstracting over the parameter works, with no feature gate:

```
  sum_any_radix is ONE generic function and both arrays went through it
  sum_any_policy likewise accepts Obs<Sat> and Obs<Wrap>: 15 and 2
```

and it works for a spurious axis and an observable one alike. The cost is real and bounded: the parameter
viralises into every signature that wants to be written once, and the two remain distinct types.

**A storage site: unrepairable, and this is where the wall actually is.** `p4b`, expected failure:

```
error[E0308]: mismatched types
   |     let _column: [Spur<2>; 2] = [Spur::<2>(1), Spur::<10>(2)];
   |                                                ^^^^^^^^^^^^^ expected `2`, found `10`

error[E0308]: mismatched types
   |     let _mixed: [Ref<Lit<100>>; 2] = [Ref(1, ..), Ref::<Lit<200>>(2, ..)];
   |                                                   ^^^^^^^^^^^^^^^^^^^^^^ expected `100`, found `200`
```

A homogeneous container is one type by construction, so no signature, bound, blanket impl or const
predicate lets two spellings share one array, one slice or one column. Parametric abstraction does not
reach it, because the abstraction is over the container's element type and the container has exactly one.

**And the runtime cost is zero in every case**, which is what makes the pricing entirely about names. In
`p4`'s emitted assembly the spurious cast in both directions, both refinement widenings and the plain
identity control are **five symbols aliased to one body, and that body is `ret`**.

So the resolution of the contradiction is that both sentences are true of different sites, and the arm
structure is:

| where the spellings meet | cost of a missed merge |
|---|---|
| a monomorphic call | nothing |
| a polymorphic signature | one type parameter threaded through it; runtime cost zero |
| a homogeneous container | the program; no in-language repair exists |

**This is where I17 bites.** The one site with no repair is the storage boundary, and the
storage-minimising, aggressively bitpacked column layout is exactly what I17 says is not to be
deprioritised. So a spurious parameter is not a tidiness question; it is a cost that lands entirely on
the path arvo exists for, and `110`'s F9 rule is right for a reason `110` did not name.

**And it sharpens F9 rather than replacing it.** F9 says an axis `R` does not read must not be a
parameter. Section 4's count says which axes those are, and adds that an axis the **arm selection** reads
may be a parameter even though `R` does not read it, because it has a repair.

---

## 8. The composite side, which nobody had pushed on

`111` section 13 records that it "did not attack `109`'s section 8 chain result or `110`'s P7 and P8
composite results at all" and that "the composite side is where nobody has pushed back" (`111:993`). This
is that push, and it is constructive rather than an attack, because I went looking for a connection and
found one.

`110` closes its composite section with a sentence it flags as unexpected (`110:348-350`):

> So `interval` is an arm with a const predicate on its base, and the predicate is monotonicity, which
> wrapping does not have. The same shape as every other arm in this design, arrived at from the
> composition side rather than the rewriting side, and I was not looking for it.

**A wrapping base restricted to an extent on which nothing wraps is exact on that extent, and an exact
operation is monotone.** So `110`'s predicate should be dischargeable by a declaration and not only by a
choice of base. `p5` tests it.

```
P1. Ungraded, reproducing 110 F12's shape
  monotonicity predicts closure on 16/16 bases
  the no-precondition hypothesis agrees on 8/16

P2. Graded: interval closure over a WRAPPING base under a declared extent
  uW3F0wrap  <= 1   propagated fits True   monotone on it True    ill-ordered   0/9
  uW3F0wrap  <= 2   propagated fits True   monotone on it True    ill-ordered  0/36
  uW3F0wrap  <= 3   propagated fits True   monotone on it True    ill-ordered 0/100
  uW3F0wrap  <= 4   propagated fits False  monotone on it False   ill-ordered 23/225

  over wrapping bases: 8 of 40 declared extents give a closed interval construction
  the propagated bound predicts closure: unsound 0, conservative 2, exact 38, total 40
```

**A declared extent discharges a construction's base predicate.** Eight of forty declared extents over a
wrapping base gain a construction the ungraded predicate refuses, and the propagated bound predicts the
gain with zero unsound predictions. So the composite layer and the arm layer are not two mechanisms
needing two designs: the refinement supplies the const predicate the composite layer already wanted, and
`110`'s "same shape as every other arm" is the same shape because it is the same thing.

**And here is the hazard, which I found by attacking my own answer.** If a grade lifts through a
construction, what is the lifting rule? The obvious answer is componentwise, and `p5` shows it is
**unsound**:

```
    product2   extent <= 2: pairwise rule says discharge, unsound  0/81
    complex    extent <= 2: pairwise rule says discharge, unsound 26/81
    dual       extent <= 2: pairwise rule says discharge, unsound  1/81
```

Complex multiplication mixes the components, so the componentwise bound does not bound its result, and a
design carrying a refinement through a composite with the base's rule licenses an arm that computes a
different answer on 26 of 81 pairs. That is the same structural fact `110` F11 reports from the law side,
that the componentwise product preserves its base's law set exactly while the twisted constructions do
not, arriving on the grade side. Two instruments, one fact; and I read `110` first, so it is a second
sighting rather than a second instance.

`p5b` supplies the repair rather than leaving the hazard reported:

```
  base uW3sat  product2   smallest sound rule: componentwise
  base uW3sat  dual       smallest sound rule: twice-componentwise
  base uW3sat  complex    smallest sound rule: NONE of the three fires soundly

  base sW4sat  product2   smallest sound rule: componentwise
  base sW4sat  dual       smallest sound rule: twice-componentwise
  base sW4sat  complex    smallest sound rule: twice-componentwise
```

Read off each construction's multiplication and the rules are derivable rather than guessed: `product2`
needs `b^2` in range, `dual` and `complex` sum two products so they need `2b^2`, and complex's real part
is a **difference**, so over an unsigned base no magnitude bound alone discharges it and none of the
three rules fires soundly. Over a signed base it does.

So the canon sentence is not that a grade transforms through a construction. It is that **a construction
carries its own grade transformer**, alongside the base predicate `110` found, and the transformer is a
joint fact with the base's signedness. That is I13's shape appearing a third time in this unit, after
`111` F111-6 found observability of the rounding mode to be joint with the signedness and the overflow
policy.

---

## 9. The statement, offered

Suggestions. Op decides, and per I12 an opinion given before the experts converge is an ack. This is what
I believe `109`, `110`, `111`, `108`, `106` and `40` jointly support with the corrections above applied,
and it is written to compose with `108` section 7 rather than to replace it.

> A **primitive** is a value set together with one realisation map taking an exact result back into it,
> over a declared operation set. Its **identity** is that structure up to denotation-preserving
> isomorphism. A **law** is read off it and never declared.
>
> A **refinement** is a declared restriction on where an operation's arguments lie. It is **not a
> coordinate of the primitive**, because a restricted carrier is not closed under the operations and so
> carries no algebra for a coordinate to be a coordinate of. It is **not a member of the declared
> semantics**, because moving it changes no denoted answer and because consumers of one value may hold
> different refinements of it and all be correct, which is the property the declared semantics is defined
> by lacking.
>
> A refinement is **transformed by an operation rather than preserved by it**, so it decorates the nodes
> of a derivation rather than being an invariant of a carrier, and each node's licence is checked against
> its own transformed refinement. It is ordered by **weakening**, which is total, is the identity on the
> representation, and whose opposite direction is a compile-time refusal naming the instantiation.
>
> A refinement has **as many parts as the realisation map has regions**, and each part switches off the
> region whose trigger it bounds and no other. A bound on magnitude switches off the behaviour outside
> the range; a bound on the grid switches off the behaviour between grid points.
>
> **How a refinement is propagated is not part of what it is.** A propagation rule is sound when it
> over-approximates the reachable set at every node of the derivation, and no sound rule is uniquely best:
> one loses where a leaf repeats, another loses where quantities multiply. Sound rules **disjoin into a
> sound rule**, so a design carries as many as it can afford to evaluate and licenses when any of them
> licenses. Checking only the derivation's result rather than every node is unsound.
>
> What a refinement licenses is the **substitution of one arm for another on a term**, never the
> identification of two primitives. Two assignments of an observable axis stay two primitives whatever is
> declared about the values flowing through them.
>
> An axis is classified by **how many directions admit a total denotation-preserving map**. Two means the
> axis is spurious and must not be a parameter, because what is wanted there is equality and no
> equality exists. One means it is a refinement and may be a parameter, with the map as its weakening.
> Zero means it is part of the declared semantics and must be a parameter, because nothing coerces.
>
> A **construction on primitives carries two things**: a predicate on its base, and a transformer for its
> base's refinements. Neither is inherited from the base and using the base's own rule for either is
> unsound on the twisted constructions.
>
> The cost of two names for one primitive is **not a property of the split**. It is a property of where
> the two spellings meet: nothing at a monomorphic site, one threaded parameter at a polymorphic one, and
> the program at a homogeneous container, where no repair exists.

**Permanence.** Every sentence survives a rewrite in another language or decade. None names a container,
a width, a marker, a type parameter, a crate, or a count.

**Equivalence.** Three teams implementing this produce units that behave the same on what matters: a
consumer declares a restriction and gets the cheap arm wherever some sound rule they carry proves it; an
undischargeable declaration is a build failure and never a runtime one; a consumer may always under-claim
and never over-claim; no axis nothing reads appears as a parameter; two spellings of one denotation do
not exist; and a composite refuses a lift its own transformer does not license. They differ on how the
refinement is spelled, whether weakening is a method or a coercion, how many parts the refinement has
beyond the two named, and how many primitives ship.

**Where it is weaker than I would like, stated rather than hidden.**

Everything enumerative is at `W <= 6` and I have made **no transfer argument** to any real width.
`unstable-features.md` is explicit that a model-width check needs one and that its own enumeration of the
routes by which behaviour can vary per instantiation is not exhaustive.

The classification in section 4 is measured on six axes over uniform value sets. I did not sweep a
non-uniform spacing, which is `110`'s stated largest gap and is where arvo's float side lives.

The refinement I built and swept carries an upper bound and a grid step. `82` F6's window carries two
endpoints and its sign-uniformity predicate needs both, and I did not measure whether a two-endpoint form
changes any verdict here. `109` section 14 flags the same gap from the range side.

**And nothing here prices anything.** No bench harness ran, I took no timing that decides anything, and
the word for every magnitude in this file is unpriced. The instruction counts and symbol aliases in
section 3.2 and section 7 are structural observations about which bodies exist, and two instructions
against five decides nothing.

---

## 10. Findings, each with its predicate

Per I13 and `RULES.md`. Three conventions stated once.

**Threads.** Everything enumerative ran on one thread and is predicated `threads = 1`. Three findings
carry `threads any`, and all three are compile-time structural results where the argument is stateable
rather than assumed: what rustc accepts, and which symbols it emits for a given source, are not functions
of a runtime thread count.

**Target features.** The model sweeps are exact rational arithmetic whose results no instruction
selection can move, so they carry `target features any` with that as the argument. The assembly findings
carry the host default explicitly, because there the feature set is the thing being read.

**Model width.** Every enumerative finding is at `W <= 6` with no transfer argument, so every predicate
lists its width as a fixed set.

**F112-1. A declared extent carries no algebra at any non-trivial size, so it is not a coordinate of a
primitive.** At `W = 4` under `{add, sub, mul}` the only extents admitting a total denotation-preserving
map to the base are the singleton and the whole carrier, and the only extents closed under the operations
are the same two. `W = 4, F = 0, signedness = unsigned, overflow policy in {sat, wrap}, rounding = trunc,
radix = 2, signature = {add, sub, mul}, extents = every upper bound in 0..=15, threads = 1, target
features any`. `p1_output.txt`. A second instrument for `111` F111-8, read after it, so a reproduction
rather than an instance.

**F112-2. The direction count classifies an axis, and it agrees with the consumer-disagreement reading on
every cell.** Two directions for radix at `F = 0` and rounding at `F = 0`; zero for the overflow policy,
the total width, the fraction width, the signedness, radix at `F = 1` and rounding at `F = 1`; one for a
discharged declared extent. 24 of 24 cells agree, and a mutation moves the verdict. `W in {3, 4}, F in
{0, 1}, signedness in {unsigned, signed}, overflow policy in {sat, wrap}, rounding in {trunc, near},
radix in {2, 3}, signature = {add, sub, mul}, threads = 1, target features any`. `p1_output.txt`.

**F112-3. Under a discharged magnitude bound, two assignments of an observable axis compute the same
answers on a term, and remain two distinct primitives.** Zero of 64 differing for the overflow policy at
bound 7 against 120 of 256 at bound 15; zero of 16 for the total width and for the signedness at bound 3
against 28 of 64 at bound 7; and `types merge` is false in every row. `W in {3, 4}, F = 0, signedness in
{unsigned, signed}, overflow policy in {sat, wrap}, rounding = trunc, radix = 2, operation = add, arity =
2, extents = uniform upper bounds, threads = 1, target features any`. `p2b_output.txt`.

**F112-4. The magnitude part of a declaration switches off the completion and not the rounding; the grid
part switches off the rounding and not the completion.** At `W = 6, F = 2`, `mul`, arity 2: magnitude
alone leaves 36 of 169 rounding disagreements and removes all completion disagreements; a grid bound
alone leaves 180 of 256 completion disagreements and removes all rounding disagreements. `W = 6, F = 2,
signedness = unsigned, overflow policy in {sat, wrap}, rounding in {trunc, near}, radix = 2, operation =
mul, arity = 2, declared grid in {1/4, 1}, threads = 1, target features any`. `p2b_output.txt`.

**F112-5. A discharged declaration licenses the arithmetic substitution and never the reinterpretation of
the bits.** Eight of 16 bit patterns denote different values under unsigned against signed at `W = 4`,
and zero of 16 arithmetic results differ on a discharged extent. `W = 4, F = 0, signedness in {unsigned,
signed}, overflow policy = sat, rounding = trunc, radix = 2, operation = add, arity = 2, extent = 0..=3,
threads = 1, target features any`. `p2b_output.txt`.

**F112-6. The propagated corner bound is sound on every term shape swept and exact only on left-nested
chains of one operation over independent leaves.** Zero unsound in all twelve rows. Conservative 0 on the
control shapes, 120 of 256 extents on `(x + y) - y`, 33 of 64 on `x * (y - y)`, 120 of 4096 on
`(x + y) * z`. `W = 4, F = 0, signedness in {unsigned, signed}, overflow policy in {sat, wrap}, rounding
= trunc, radix = 2, operations in {add, sub, mul}, term shapes as enumerated in the probe, arity in {2,
3}, extents = every upper-bound tuple, threads = 1, target features any`. `p3b_output.txt`.

**F112-7. Two distinct sources of conservatism, and only one is recoverable by a tighter bound.** On
`(x + y) - y` the corner rule licenses 16 of the 136 extents a reachable-set oracle licenses, and the
oracle recovers all of them. On `(x + y) * z` the corner rule and the oracle both license 385 of 4096
while the arms agree on 120 more, which no node-wise rule reaches. Same predicate as F112-6.
`p3b_output.txt`.

**F112-8. A repeated leaf is not sufficient for conservatism.** `x - x` at signed `W = 4` over extents
0..=7 is conservative on 0 of 8, because its propagated interval never leaves the container. `W = 4, F =
0, signedness = signed, overflow policy = sat, rounding = trunc, radix = 2, operation = sub, term =
x - x, extents = upper bounds in 0..=7, threads = 1, target features any`. `p3b_output.txt`.

**F112-9. A missed merge is repairable at a function boundary by abstracting over the parameter, and
unrepairable at a homogeneous container.** One generic function accepts both spellings of a spurious axis
and both assignments of an observable one; an array literal mixing two spellings is `E0308` naming both
const arguments, for a spurious parameter and for a refinement parameter alike. `toolchain =
nightly-2026-05-28, rustc 1.98.0-nightly (57d06900f 2026-05-27), edition 2021, feature gates = none,
threads any, target features any`. `p4_output.txt`, `p4b_expected_failure.txt`. This **corrects `110`
F8**, whose "no repair" is right about type equality and wrong about the function boundary.

**F112-10. Every coercion in the classification compiles to nothing, and the licensed arm loses the
clamp.** Five symbols alias to one `ret` body: both directions of the spurious cast, both refinement
widenings and the plain-identity control. `add_licensed_100_100` is `add w0, w1, w0 ; ret` against
`add_general_u8`'s five instructions with `cmp` and `csel`. `toolchain = nightly-2026-05-28, edition
2021, feature gates = none, target = aarch64-apple-darwin, target features = host default, opt level = 3,
container = u8, declared bounds in {7, 100, 200, 255}, operation = add, arity = 2, threads any`.
`p4_asm_bodies.txt`, `p4_asm.s`.

**F112-11. A tightening is refused at build time naming the instantiation.** `E0080` at
`Widen::<Lit<200>, Lit<100>>::CHECK`. Same toolchain predicate as F112-9, `threads any`.
`p4c_expected_failure.txt`. Reproduces `111` F111-12 on an independent construction.

**F112-12. A declared extent discharges the interval construction's base predicate over a wrapping base,
and the propagated bound predicts it.** Eight of 40 declared extents over wrapping bases give a closed
interval construction; the bound predicts closure with 0 unsound, 2 conservative, 38 exact of 40. `W in
{2, 3}, F in {0, 1}, signedness in {unsigned, signed}, overflow policy = wrap, radix = 2, construction =
interval, operation = add, arity = 2, extents = every upper bound in the carrier, threads = 1, target
features any`. `p5_output.txt`.

**F112-13. A construction's grade transformer is not its base's, and borrowing the base's is unsound.**
The componentwise rule applied to complex multiplication is unsound on 3 of 16 pairs at extent 1 and 26
of 81 at extent 2 over an unsigned base, and on 1 of 81 for dual. `W = 3, F = 0, signedness = unsigned,
overflow policy = sat, radix = 2, constructions in {product2, dual, complex}, operation = mul, arity = 2,
extents in {1, 2, 3}, threads = 1, target features any`. `p5_output.txt`.

**F112-14. The smallest sound transformer differs per construction and is a joint fact with the base's
signedness.** `product2` needs the componentwise rule, `dual` needs twice it, and `complex` needs twice it
over a signed base and has no sound magnitude-only rule at all over an unsigned one. `W in {3, 4}, F = 0,
signedness in {unsigned, signed}, overflow policy = sat, radix = 2, constructions in {product2, dual,
complex}, rules as enumerated in the probe, operation = mul, arity = 2, extents in {1, 2, 3}, threads =
1, target features any`. `p5b_output.txt`.

**F112-17. An affine grade recovers the corner rule's correlation loss completely and loses on
multiplication of non-centred quantities.** On `(x + y) - y` it licenses 136 of 256 against the corner
rule's 16, matching the oracle; on `x * (y - y)` 64 of 64 against 31; and on `x * y` 31 of 256 against 76,
and on `(x+y) * (z+w)` 31 of 256 against 212. Zero unsound for both rules on every row. `W = 4, F = 0,
signedness in {unsigned, signed}, overflow policy in {sat, wrap}, rounding = trunc, radix = 2, operations
in {add, sub, mul}, term shapes as enumerated in the probe, arity in {2, 3, 4}, extents = one-sided upper
bounds over every tuple, threads = 1, target features any`. `p7_output.txt`.

**F112-18. The two rules disjoin into a sound predicate that reaches the enumerating oracle on every term
shape swept.** Ten rows, zero unsound, ten of ten reaching the oracle, and four of ten where the affine
rule licenses something the corner rule refuses. Same predicate as F112-17 with the shapes as enumerated
in `p7b`. `p7b_output.txt`.

**F112-19. The residue the union does not reach is the annihilation case, and it is about the term's
dependence rather than any node's range.** On `(x + y) * z` with `z` declared zero, the union licenses 136
of 256 while the arms agree on 256 of 256. Same predicate as F112-17. `p7b_output.txt`.

**F112-20. An affine grade is expressible under the operating constraints, and cancels in the type.**
Coefficients carried as types with associated consts rather than as const arguments; `(x + y) - y`
composed through the grade types resolves to a radius equal to `x`'s alone against the corner rule's
`[-14, 28]`; the vector's length is (leaves + non-constant multiplications), both static properties of the
term. `toolchain = nightly-2026-05-28, rustc 1.98.0-nightly (57d06900f 2026-05-27), edition 2021, feature
gates = none, no dyn, no TypeId, threads any, target features any`. `p8b_output.txt`, `p8c_output.txt`.
The naive spelling, with the coefficient as a const argument, is refused for `generic_const_exprs` and
that refusal is committed at `p8_output.txt`.

**F112-21. A root-only range check is unsound and the per-node check is load-bearing.** Over unsigned
saturating `W = 4` with `x` in `[8, 10]`, `y` in `[8, 10]`, `z` pinned at 15, the root propagates to
`[1, 5]` and fits, the intermediate to `[16, 20]` and does not, and the two arms disagree on 9 of 9 tuples
inside the declaration. `W = 4, F = 0, signedness = unsigned, overflow policy = sat, rounding = trunc,
radix = 2, term = (x + y) - z, arity = 2, extents = two-endpoint, threads = 1, target features any`.
`p7c_output.txt`.

**F112-22. The per-node check is expressible because a composed grade is a composed type, and the licensed
arm erases.** `AllOk` recurses over the grade's own structure with no feature gate; the licensed arm
aliases an ungated `(a + b) - c` and the refused arm aliases an ungated saturating chain, so the const
gate is absent in both directions. `toolchain = nightly-2026-05-28, edition 2021, feature gates = none,
target = aarch64-apple-darwin, target features = host default, opt level = 3, container = u8, declared
extents in {0..=6, 0..=14}, term = (x + y) - y, threads any`. `p8c_output.txt`, `p8c_asm.s`.

**F112-23. Every declared extent in every sweep of this file is one-sided, and that makes one instrument
check vacuous.** With every lower bound pinned at zero, the root of an addition chain over a non-negative
domain is the widest node, so a root-only check and a per-node check agree by construction and `p7b`'s
mutation measured 0 of 4096. `p7b_output.txt` carries the defect, `p7c_output.txt` carries the witness a
two-endpoint extent produces.

**F112-15. The suite is 123 across 13 and all of it passes, and `wide-rung-shared` takes 4.38s of test
time.** `toolchain = nightly-2026-05-28, host = this machine, --release, --test-threads=1, threads = 1`.
`p0_test_gate_run.txt`. The serial flag is load-bearing and I took `110` F14's workaround rather than
reproducing its hang.

**F112-16. The bench variant package names carry a `bench-` prefix the directory names do not.**
`cargo test -p satfold-shared` matches no package; `-p bench-satfold-shared` runs 11 tests. Same
predicate as F112-15. `p0_test_gate_run.txt`.

**Unpriced.** Everything about cost. No bench ran, no timing decides anything here, and no claim in this
file depends on a magnitude.

---

## 11. Options: what this fits, fits badly, kills, and adds

**Fits well, and settles an open item.** `OPTIONS.md` Q52's last open question, whether the refinement is
a new coordinate or a member of `106` section 1's first component. Sections 3.1 to 3.4 answer **neither**,
with a measurement against each half, and place it as the carrier for the licence `108` section 3.1 built
and could not house. That is one expert's answer and it wants a second read.

**Fits well.** Q51's converged statement survives intact. The one clause I would repair is component
one's justification, which states an entailment its own document elsewhere denies (`108:442`), and the
repair is one word: the criterion is the agreement requirement and the un-recoverability is the reason
for it rather than the test.

**Fits well, with a correction.** `110` F9, the rule that an axis `R` does not read must not be a
parameter. It is right on the axes it was measured over and it misclassifies a refinement, which `R` also
does not read. Section 4's count is the same rule with three verdicts instead of two.

**Fits badly, at a nameable cost.** Any option carrying a refinement with only a magnitude part. It
survives, and it pays F112-4's price: half of `R` is outside its reach, so a rounding substitution that a
grid declaration would license is never available, and the design discovers this after the parameter list
is chosen, which `110` F8 establishes is the one decision that cannot be revisited.

**Fits badly, at a nameable cost.** Any option that lifts a refinement through a composite using the
base's own transformer. It survives only where every construction is componentwise, and it pays F112-13's
price, which is an unsound arm rather than a lost optimisation.

**Killed, with the diagnostic.** Any option that makes a declared refinement a coordinate of the
primitive. F112-1: at `W = 4` under `{add, sub, mul}` no non-trivial extent carries an algebra, so there
is no primitive there for it to be a coordinate of.

**Killed, with the diagnostic.** Any option that decides membership in the declared semantics by asking
whether a property is recoverable from the bits. F112-3 and F112-5: a declared refinement is not
recoverable from the bits and consumers holding different ones agree completely on a discharged extent,
so the test admits something the class is defined to exclude.

**Not killed but corrected.** `110` F8's "no repair". The claim is right about type equality and about the
storage boundary and wrong about the function boundary, where parametric abstraction is the repair,
compiled in `p4`.

**Not killed but bounded.** `111` F111-9 and F111-10's zero-conservative. Reproduced on the shape they
were swept over and conservative by up to 88.2% of available licences on a correlated-leaf term.
`OPTIONS.md` Q52 currently carries "exactly, zero unsound and zero conservative" without the shape; the
shape belongs in the sentence.

**Added.** A construction carries a grade transformer as well as a base predicate, and neither is
inherited. F112-13 and F112-14.

**Added.** A refinement discharges a construction's base predicate, which is what makes the composite
layer and the arm layer one mechanism rather than two. F112-12.

**Added.** The cost of a missed merge is a three-armed fact about where the spellings meet, and its one
unrepairable arm is the storage boundary that I17 protects. F112-9, F112-10.

**Added.** Two grade rules that neither dominates, disjoining into one that reaches an enumerating
oracle on every term shape swept, both const-computable and both compiled. F112-17, F112-18, F112-20,
F112-22. This is the arm structure I13 describes, arriving inside a mechanism rather than across one.

**Added, as a hazard rather than a win.** A root-only discharge check is unsound and I wrote one myself
after building the witness that refutes it. F112-21.

---

## 12. Alternatives I considered and did not take

For whoever attacks from a different angle, so the list is the starting point rather than nothing.

**A. Making the refinement's discharge a runtime check at an untrusted boundary.** Refused on principle
rather than on cost: I15 says never a runtime check, and `88` section 3 says ingest is the consumer's,
with arvo permitted to ship casting helpers and forbidden from using them on the consumer's behalf. So
the only two admissible ends of an undischargeable declaration are a compile-time refusal and an
unchecked assumption, and section 12's alternative B applies to the second.

**B. The unchecked assumption as a speed-first arm.** I5 licenses sacrificing soundness for a proven
meaningful gain, and an unchecked declaration is precisely that trade. I did not build it, because
nothing in this file is priced and I5's condition is a **proven** gain, which is what section 9's closing paragraph on pricing says nothing here supplies. The route is open and the next
step is a harness run rather than an argument. Worth attacking: whether the gain is even measurable once
`p4` shows the coercion compiling to nothing.

**C. A refinement with more than two parts.** Section 5 gives one part per region of `R`, and `R` has two
regions in `110`'s reading. If a design adds a third region, the refinement grows a third part by the same
construction. I did not look for one. `110` section 2 warns that a design carrying overflow and rounding
as two axes "stops saying it when somebody adds a third region", which is the same worry one level down.

**D. Refining the realisation map rather than the carrier.** `111` section 12 alternative C names it and
says it is the more general form. My section 5 is evidence for that: what the grade actually does is bound
which region of `R` can fire, and the carrier bound is one way to say so. Someone should build it
directly, because the grid part of my grade is already closer to a region restriction than to a carrier
restriction.

**E. A correlation-tracking propagation rule. Taken rather than left**, and section 6b is the result: an
affine grade recovers all 136 licences, is expressible with no feature gate, and composes with the corner
rule into a union that reaches the oracle on every shape swept. What is left of the alternative is
narrower and still open. A symbolic residual rather than an affine form would recover the multiplication
cases the affine form loses, and I did not try one. Nor did I try re-centring an affine form on a
non-negative domain, which is the obvious repair for its multiplication weakness and which I would attack
first.

**F. Whether the refinement needs to be in the type at all.** `111` alternative A raises it and rejects it
on the ground that a per-site bound cannot survive being stored. My section 3.4 is the same argument from
the licence side: `108`'s chain-scan predicate is per-site and does not survive a column. Both point the
same way and neither is a measurement. Someone should measure it, by asking what fraction of the arms
`108`'s predicate licenses are still licensable after a value round-trips through storage.

**G. Attacking `109`'s section 8 chain result.** Still untouched by anyone, and `109` names its own most
obvious attack: it did not test round-to-nearest, where per-step errors partly cancel. I did not run it.
That is now two members in a row declining the same target.

---

## 13. What is genuinely op's, which on my reading is nothing

The checkpoint after this file goes to him, so this section is minimal on purpose.

**Nothing in this file is a question for op**, and I want to say why rather than just asserting it,
because two of the last three questions put to him should not have been.

The placement question, whether the refinement is a coordinate or a member of component one, is exactly
the shape `104` section 3 returned to the panel: it asks which component of a decomposition a concept
attaches to, where the decomposition is the panel's invention. His test, from that file: **if both
answers leave the intent intact and differ only in what the panel calls things, it is not his.** Both
answers here leave every intent intact. So it is the panel's, and I have answered it, and it wants a
second read from an expert rather than a ruling from him.

The discharge question, what happens to a declaration that cannot be discharged, is a category-wide policy
fork and `88` section 4 rejects that shape by name: "Take the win where it applies, gate it out from
where it does not. No single one-fits-all solutions, it's impossible." The answer is two arms with two
predicates, one refusing at compile time and one assuming under I5's proven-gain condition, and building
both is the work rather than a question.

The consumer question, whether anyone would write such declarations, is answered in the same file: "the
arm is built because a region exists where it is optimal, which is what an arm is."

**Two things are reports rather than questions, and they are for the consolidation rather than for him.**
F112-9's storage-boundary result is what I17 already decides, given a measurement; and F112-13's unsound
lift is a hazard the design should know about before it chooses a composite surface. Neither needs a call.

---

## 14. Coverage, bounded

**Read in full:** `INTENTS.md`, `RULES.md`, `111` including its probe index, op's `88`, `95`, `104`,
`105`, `OPTIONS.md` Q51 and Q52.

**Read in part:** `109` sections 5 to 11 and its phase-two 16.3; `110` sections 1 to 6 and its findings
list; `106` sections 1, 2, 17 and 18; `108` sections 3.1 and 7 and its sections 8 and 9;
`satfold-shared`, `warm-clamp-shared` and `quantiser-radix-shared` test bodies and the two const census
functions.

**Not read:** `DROPLIST.md`, `PERSONA_CALLS.md`, `PRIOR_CALLS.md`, the `SEED_*` files, the archive, and
files `01` through `107` other than the sections named above. Where I cite one of those I am citing
`106`'s or `108`'s or `111`'s account of it and I say so at the citation. In particular **`40`'s
definition of observability, `97`'s congruence criterion, `82`'s F6 and `90` R3 reach me only through
`108` and `111`.**

**Verified at source rather than through an account:** `110`'s two contradicting sentences, both opened;
`110` F9's rule, opened, and its line number corrected from the one my own check first carried;
`110` F11 and F12, opened; `108` section 7's clause and `108:442`'s admission that the agreement property
can fail, both opened; `108` section 3.1's predicate table, opened; `106` section 1's first component,
opened; `109:376` and `109:424`, opened; `111:626`, `111:728`, `111:951` and `111:993`, opened;
`satfold-shared`'s two const census functions, opened; the test count, re-run.

**Not verified:** every number I quote from `109`, `110`, `106`, `108` or `111` that I did not reproduce.
Where I reproduced one I say so, and the reproductions are `111` F111-8's zero-closure result (in `p1`),
`111` F111-9's zero-unsound result (in `p2b`), `111` F111-12's `E0080` (in `p4c`), and `110` F12's shape
though not its numbers (in `p5`).

**Citations checked by opening them.** `112_probes/p6_check_my_own_citations.py` opens every `file:line`
this file leans on and tests the substring the claim depends on against the cited line and its two
neighbours. **34 checked, 0 failing, after three of them failed on the first run and were corrected.**
The three were a line number 60 lines short, a citation to a sentence that turned out to be wrapped
across two lines so the substring never appeared on either, and a table row I cited from memory. Because
the run came out green it is mutation-tested rather than trusted: a wrong line number, a wrong substring
and a wrong file each produce exactly one failure.

**Two probes of mine were defective and both are committed with their defects named rather than
overwritten.** `p2`'s conservative counter could not fire, because the measured arm fell back to the
general arm at any refusing node, which is the same dead-branch class `111` found in `110` F3's third
bullet and which I wrote a condition-can-fire warning about in that probe's own header before failing it.
`p3`'s instrument check chose `x - x` as its witness, which happens to have conservative 0, so it printed
"the counter is live: False" and read as a failed check when the counter was live at 120 on a different
term in the same run. Repairs are `p2b` and `p3b`.

**Three of my own probes were defective beyond the two named above**, and all three are committed with
the defect written into their own output rather than overwritten: `p7b`'s mutation could not fire
because every extent I sweep is one-sided; `p8` half-applied the refused-bound reflex and landed on the
forbidden feature; and `p8b` wired a root-only gate to a rule whose measured numbers are per-node, after
`p7c` had already built the witness showing root-only is unsound. Repairs are `p7c`, `p8b` and `p8c`.

**The largest thing I did not do.** I did not attack `109` section 8's chain result, which is now
untouched by two consecutive members, and I did not attack `110`'s P8 congruence result, only its P7
composite closure. I also did not price anything, so every magnitude in this unit remains unpriced.

---

## 15. Probe index

All under `112_probes/`, each committed as it ran, before this file was written.

- `p0_test_gate_run.txt`. Every test-bearing variant crate, per crate, serially. 123 pass. Both the
  failed first run and the working second one, so the package-prefix trap is on the record.
- `p1_how_many_directions_admit_a_denotation_preserving_map.py`, `p1_output.txt`. The direction count on
  eight single-axis moves and sixteen extents, with two mutations.
- `p2_the_grade_switches_off_a_region_of_R_not_an_axis.py`,
  `p2_output_first_version_conservative_is_a_dead_branch.txt`. **My dead branch**, kept, with the defect
  written into its own output.
- `p2b_the_grade_repaired_and_the_two_regions_separated.py`, `p2b_output.txt`. The repair: conservatism
  measured against the fully cheap arm, and the grid part given a coarser declared step so it can
  discharge.
- `p3_where_the_exactness_stops_correlated_leaves.py`, `p3_output.txt`. The exactness boundary, with two
  badly chosen witnesses named in its own output.
- `p3b_where_the_exactness_stops_repaired.py`, `p3b_output.txt`. The same with the witnesses fixed, plus
  the reachable-set oracle.
- `p4_where_a_missed_merge_actually_costs_something.rs`, `p4_output.txt`, `p4_asm.s`,
  `p4_asm_bodies.txt`. Three axes carried three ways in one file, no feature gate, with the aliases and
  the two arm bodies extracted.
- `p4b_the_storage_boundary_is_the_wall.rs`, `p4b_expected_failure.txt`. Expected failure. `E0308` for a
  spurious parameter and for a refinement parameter alike.
- `p4c_a_tightening_is_refused_before_the_program_exists.rs`, `p4c_expected_failure.txt`. Expected
  failure. `E0080` naming `Widen::<Lit<200>, Lit<100>>::CHECK`.
- `p5_a_declared_extent_discharges_a_constructions_predicate.py`, `p5_output.txt`. `110` F12's shape
  reproduced, the extent discharging it over a wrapping base, and the unsound componentwise lift.
- `p5b_the_lifting_rule_is_per_construction.py`, `p5b_output.txt`. The repair: the smallest sound rule
  per construction, and the unsigned complex case where none of the three fires.
- `p7_an_affine_grade_recovers_the_lost_licences.py`, `p7_output.txt`. The affine grade against the
  corner grade and the oracle, six term shapes, with the correlation-breaking mutation.
- `p7b_the_two_rules_compose_and_the_union_is_the_answer.py`, `p7b_output.txt`. The union over ten
  shapes, and **my mutation that could not fire**, with the reason written into its own output.
- `p7c_the_per_node_check_is_load_bearing.py`, `p7c_output.txt`. The two-endpoint witness that mutation
  could not reach.
- `p8_the_affine_grade_compiles_as_a_type_level_list.rs`, `p8_output.txt`. **Expected failure, kept**:
  the refused-bound reflex half-applied, landing on `generic_const_exprs`.
- `p8b_the_affine_grade_with_no_arithmetic_in_type_position.rs`, `p8b_output.txt`. The repair, compiled,
  zero feature gates, **with its own root-only gate named as a defect**.
- `p8c_the_per_node_discharge_check.rs`, `p8c_output.txt`, `p8c_asm.s`. The per-node check compiled, the
  affine advantage surviving it, and both arms aliasing their ungated controls.
- `p6_check_my_own_citations.py`, `p6_output.txt`. Thirty-four citations opened and their content tested,
  three of them wrong on the first run, with three mutations confirming the instrument fails when it
  should.
