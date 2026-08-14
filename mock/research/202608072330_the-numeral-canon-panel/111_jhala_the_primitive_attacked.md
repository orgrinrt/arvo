# 111. The primitive attacked

Third member of topic five, and the first to attack. Two cold derivations landed before me, `109` and
`110`, dispatched blind and in parallel from the premises alone, each with a phase-two reconciliation
appended. My job is to break them and to say what a primitive is.

I read the panel before deriving, which is the attacker's position and is the opposite of theirs, so
nothing in this file earns the TWO EXPERTS rung by independence. Where I agree with a predecessor I say
whether I derived it first or read it first, because those are different results and only one of them
is worth anything as corroboration.

One thing about my own provenance goes at the top rather than in a coverage note, because it bears on
how several sections below should be weighed. **`18` and `82` in this panel are the same persona as
this file.** `82` is where the declared operand window entered the panel, and section 9 below builds
directly on it. An agreement between this file and `82` is not two experts, it is one approach applied
twice, and I mark it as such wherever it arises.

## 0. The two gates

### 0.1 Canon gate: passed

Checked against `INTENTS.md` read with its own "How to read an entry" section as normative, and against
`RULES.md`. Nothing in I1 through I18 forbids the question, presumes an answer to it, or is contradicted
by anything I build. I1 is demoted to open on op's word so the strategy set is mine to challenge; I13
governs every finding's shape; I14 fixes the operating constraints and every probe of mine stays inside
them, with **zero `#![feature(...)]` gates across all nine files** (`grep -c '^#!\[feature'` returns 0
on each Rust probe, recorded in `p1b_output.txt` and `p8_output.txt`).

Two intents do real work in what follows and I flag them rather than smuggling them in. **I3**, as `104`
settles it, is about the experience of using the type, which is what makes a literal a non-negotiable
part of arvo's operation set in section 6. **I15**, never a runtime check, is what makes the compile-time
refusal in section 9 the only admissible form of the obligation my answer introduces.

### 0.2 Test gate: passed, at 123 across 13, and it is the seventh count

I ran every test-bearing bench variant crate, per crate, serially, rather than trusting a summary.
Transcript at `111_probes/p0_test_gate_run.txt`, with the toolchain line in it.

```
grep -rnE '^[[:space:]]*#\[test\][[:space:]]*$' --include='*.rs' variants/ | grep -v '/target/' | wc -l
123
```

Per crate: `wide-rung-shared` 30, `bitpack-write-contend-shared` 15, `warm-container-shared` 15,
`bitpack-contend-shared` 12, `satfold-shared` 11, `bitpack-carrier-shared` 9, `warm-clamp-shared` 7,
`bitpack-wide-shared` 6, `bitpack-footprint-shared` 6, `bitpack-plan-shared` 5, `bitpack-shared` 3,
`quantiser-radix-shared` 3, `quantiser-fadd-shared` 1. Sum 123, all passing, no failures, no ignores.

**`110` was right and `109` was wrong on the count, and `110` corrected itself.** The 124 both phase-one
files reported is a `#[test]` inside a doc comment at `bitpack-write-contend-shared/src/stress.rs:68`.
`109` section 16.6 asks whoever holds the register to re-run rather than pick a side; it is re-run here
and the attribute-only pattern returns 123. Nothing is added and nothing was added.

**`wide-rung-shared` runs its 30 tests in 4.25 seconds on this host**, against the 4.05 seconds `110`
measured and the 107 seconds a previous brief attributed to it. That is a third measurement and the
107 figure should be dropped rather than carried as contested.

**On the surface I touch, which is the two quantiser crates and `satfold-shared`, the suite is real.**
`satfold-shared` runs four deliberately broken kernels against the same oracle its real arms are checked
against, and
`satfold-shared/src/lib.rs:1116`'s `the_one_element_defect_is_caught_up_to_1024_and_not_above_it`
asserts the instrument's own sensitivity boundary in both directions. That is a test asserting a
limitation of its own suite, which is the opposite of the failure this gate exists to catch.

**One nit, and it is `109`'s, confirmed.** `quantiser-radix-shared/src/lib.rs:370` asserts
`assert_eq!(p % 2, 1, ...)` and line 372 asserts `assert!(p % 2 == 1)`. Redundant rather than
tautological, since it can fail; one of the two is doing the work. Not a gate failure.

**And one thing I did not do**: I did not audit the ten crates outside my surface beyond running them.
Anyone reading my gate result as a census is reading a sample as one.

---

## 1. The answer, stated first

> **A primitive is a decidable approximation of a denotation.**
>
> The denotation is a value set together with one realisation map, over a declared operation set.
> The type is whatever must be const-available to decide validity or select a lowering.
> **The obligation nobody has stated is that the second is a decision procedure for the first, and it
> owes both soundness and completeness**: it must never call two different denotations the same, and it
> must never call one denotation two things.
>
> And the four-part working assumption has two of its elements exactly backwards. It lists a **law
> set**, which cannot be varied with the other coordinates held fixed, and it omits the **refinement**,
> which can. The coordinate it is missing is the one that already passed the freedom test the one it
> lists already failed.

The rest of this file is that claim taken apart. Sections 2 to 8 are the attack; section 9 is what
replaces what I break; sections 10 onward are the bookkeeping.

Three things I keep from my predecessors, because keeping is a result and I went looking for reasons to
break each:

- **`110`'s realisation map.** Overflow and rounding are two regions of one map, and I built two models
  on that shape without either needing them separated. Third instance after `63` C1 and `110`; not
  independent, since I read it.
- **`109`'s three sameness relations.** Nominal, representational, denotational, each licensing a
  different operation. Section 8 gives the lattice a fourth property it did not have.
- **`109`'s const-availability criterion for membership.** It is a different question from `110`'s
  identity criterion and both survive. Section 8 says what sits between them.

---

## 2. Are the two convergences genuine? The law set: yes as a conclusion, no as a measurement

The dispatch asks whether `109` and `110` agreeing that the law set is not a component is two instances
or one shared premise. It is neither, and the answer has three parts.

### 2.1 They are not the same claim, so the agreement is real

`109` section 4 asks an **authoring** question: what happens if a primitive is allowed to declare its
law set. It answers with a compiled false declaration and a rewrite gated on it that changes **952 of
4096 answers**, witness `(-8, -8, 1)`. That is a claim about what a design surface permits, and its
failure mode is live: rustc could have refused the declaration, and the rewrite could have changed
nothing.

`110` F3 asks a **state-space** question: is the law set a free coordinate. It answers with three
bullets, of which the second, that the projection is lossy at 40 algebras to 7 law sets, is the real
content.

Different instruments, different failure modes, same conclusion. So the conclusion is at three
instances counting `90` R3, and the three are genuinely independent as instruments. `110`'s own section
16.1 says this and it is right.

### 2.2 But the bullet the brief names as the measurement cannot fail

The dispatch singles out "**0 of 48 configurations can vary the law set with the others held fixed**".
That is `110` F3's third bullet and it is a dead branch.

`110_probes/p2_laws_are_a_projection_not_a_coordinate.py` builds its sweep at lines 190 to 194 as

```python
for W, F, signed, policy, rounding in product(
    [3, 4], [0, 1, 2], [False, True], ["sat", "wrap"], ["near", "trunc"]
):
```

and its key at lines 51 to 52 returns exactly those five components plus a constant radix. **Every key
in the sweep is distinct by construction**, so `if k in seen` is never true and the statement that
increments `free` never executes.

`111_probes/p2_the_law_set_freedom_test_is_a_dead_branch.py` establishes that mechanically rather than
by reading, and then mutation-tests it:

```
arm 1: law set computed from the configuration (110's shape)
  times `k in seen` was true   : 0
  law-set comparisons performed: 0
  verdict printed by TEST 3    : DETERMINED (a projection)

arm 2: MUTATION, law set declared freely per configuration
  distinct declared law sets   : 3
  times `k in seen` was true   : 0
  law-set comparisons performed: 0
  verdict printed by TEST 3    : DETERMINED (a projection)
```

**The mutation is the exact condition the test exists to detect**, a law set that is genuinely free and
reads nothing about the algebra, and the verdict does not move. A test that reports the same answer when
the thing it tests for is present is not measuring that thing.

Arm 3 shows what a live version looks like: put an axis in the sweep that is absent from the key, and
the branch is reachable, 48 hits and 32 free variations, which are exactly the `F > 0` cells and are
`110`'s own F5 arriving from the other side.

**This is not a small point about a probe.** `RULES.md` says counts are measurements and that a count
is produced with a command. This number was produced with a command and the command could not have
returned anything else. Cited as a measurement it inflates a conclusion that has better support
elsewhere in the same file.

### 2.3 And the conclusion it supports is analytic anyway, which is the more useful correction

Even repaired, no sweep can establish that a law set is not a coordinate, because `law_set(p)` is
defined at `110_probes/p2:172` as a tuple of predicates over `p`'s operation tables, and none of the ten
law predicates reads `p.F`, `p.policy`, `p.rounding` or `p.signed` (checked: `grep -n 'p\.F\|p\.policy\|
p\.rounding\|p\.signed'` returns one hit, at line 269, in a labelling helper). A function of the tables
cannot vary when the tables are held fixed. That is a fact about the definition of "law", not about
primitives.

So the honest form of the finding is:

> **A law is by definition a predicate over the algebra, so asking whether it is a free coordinate is
> asking whether a function can disagree with itself. What measurement establishes is that the
> projection is lossy, 40 algebras to 7 law sets, and therefore cannot reconstruct the primitive.**

TEST 1 and TEST 2 in `110`'s probe carry that. TEST 3 does not, and TEST 1 is best read as a self-check
on the probe's own laws rather than as a finding.

### 2.4 The shared premise is real and it is the word "composition"

Both files were handed "a primitive is a named composition of a format, a number system, a law set and a
strategy", and both read "component" as "coordinate of a configuration". That reading is invited by the
word, and under it the law set's exclusion is analytic. **The reading under which the answer differs is
the one where a law set is a demand rather than a field**, and to `110`'s credit its own TEST 4 runs it:
demanding `distrib_add` selects 12 of 48 configurations, all at `F = 0`.

That is the residue, and it is not a leftover. Section 9 puts it in the answer.

---

## 3. The "number system" convergence is not one, and counting it as two instances inflates a rung

The dispatch names this as the second blind convergence. It is not a convergence.

`109` section "The question, and the shape of the answer" says number system "is not a component. It is
a name for a coherent package of choices, which is to say it is itself a composition, sitting one level
up". That is a category claim about the whole element.

`110` section 6 says "it is two things under one name", with the radix identity-bearing at `F > 0` and
every pure code assignment being presentation, four kinds of it quotiented out automatically by its P1.
That is a cut through the element with a different verdict on each half.

A category claim and a cut are not the same claim. They agree only on the negative, that the assumption
mishandles the bullet, and two files agreeing that a thing is wrong is not two instances of a finding
about what is right.

**And `109` concedes to `110` on it in its own phase two**, section 16.3: "Conceded: `110`'s treatment of
'number system' is sharper than mine. I said it is a name for a package, one level up. `110` splits it
... Take theirs." A concession after reading is agreement inherited, which `RULES.md` is explicit is not
the middle rung.

So: **one instance, `110`'s, at ONE EXPERT, with `109` withdrawing rather than corroborating.** Whoever
holds the register should not record it as two.

---

## 4. `109`'s blocker is real and its conclusion is too strong, and the repo already contains the counterexample

`109` section 4 records rustc refusing four times:

```
error: function pointer calls are not allowed in constant functions
```

and concludes: "**a law cannot be computed at const time about an operation supplied as a value; the
operation has to be a type.** That is not a detail of my probe, it is a constraint on any design that
wants derived rather than declared laws."

**The wall is real. The conclusion does not follow, and the counterexample is in this repository.**

### 4.1 Reproduced, then routed around three ways

`111_probes/p1a_fn_pointer_in_const_fn.rs` reproduces the refusal verbatim rather than taking it on
report, output in `p1a_output.txt`. Then:

**Route two, the operation as a const generic value.**
`111_probes/p1b_const_generic_tag_and_match.rs` carries the completion as a `const OP: u8` dispatched by
a `match` inside the const fn. It compiles with **no feature gate at all**, and the census it computes
reproduces `109`'s own numbers exactly:

```
op              assoc-failures  escapes-from-set
sat-both                   952                 0
wrap                         0                 0
sat-top-only               448                36
```

952 and 448 and 36 are `109` section 4's three figures, arrived at with the operation as a **value**,
which is the thing its conclusion says cannot be done.

**Route three, the operation as macro syntax.** `111_probes/p1c_macro_carried_operation.rs` expands the
operation into the const fn's body. Neither a type nor a value. Same three numbers, asserted against
route two inside the probe so the agreement is checked rather than eyeballed.

**Route four, already shipped, and this is the one that matters.**
`mock/benches/variants/satfold-shared/src/lib.rs:519` and `:547` are two const fns computing exactly
this census over two different operations, with the operation written inline and the two versions
written out twice. No type, no value, no feature gate, and they have been in the tree gating a real bench
arm the whole time. `109` derived a constraint on the design while the design's own committed code was
violating it.

### 4.2 The right statement, and why it changes nothing about the design

> **A law computed at const time needs the operation to be statically resolved. It does not need the
> operation to be a type.** At least four carriers work: a const trait, a const generic value with a
> match, macro expansion, and duplication.

And then the question that actually bears on the design, which `109` did not ask: does the carrier
choice buy anything? **No.** `111_probes/p1d_the_carrier_choice_does_not_repair_the_split.rs` carries a
completion as a type and gets the same `E0308` `110` F8 gets from a const generic value:

```
error[E0308]: mismatched types
   = note: expected `Fx<SatBoth>`
              found `Fx<Wrap>`
```

So the two carriers are equivalent on the hazard `110` prices, and the wall `109` hit is a fact about
one spelling rather than a constraint on the design. **What decides whether an axis may appear in the
type at all is `110`'s read-test, not the form the axis takes**, and section 9 is where that lands.

I would drop `109`'s sentence rather than qualify it, per `no-legacy-shims-pre-1.0.md`'s posture toward
a shape that has been replaced. What survives is the observation that the operation must be resolved
before the census runs, which is true, cheap and uncontroversial.

---

## 5. The signature claim, tested: right, monotone by construction, and it saturates at the literal

`110` F4 counts 84 primitives under `{add}` and 186 under `{add, mul}` over the same 288 configurations
and concludes that "how many primitives are there" is not well posed until somebody fixes the signature,
and that a canon enumerating axes without fixing one has not defined the thing it is enumerating axes
of. The dispatch asks me to test it. Three results, and the third is the one that changes what the
design owes.

### 5.1 The direction is by construction; only the magnitude is measured

Adding an operation refines the partition, because a term that separated two primitives is still a term
after the signature grows. So the count is non-decreasing in the signature whatever the arithmetic does,
and "the count more than doubles" measures the magnitude and never the direction.
`111_probes/p3_the_signature_saturates_at_the_literal.py` checks monotonicity holds in its own data as
an instrument check rather than reporting it as a finding: `60 <= 60 <= 126 <= 142 <= 148`.

Independent reconstruction over my own 216-configuration sweep, different widths, different rounding
set, radix in `{2, 3}`:

```
signature                                  primitives
{add}                                              60
{add,sub}                                          60
{add,mul}                                         126
{add,sub,mul,neg}                                 126
{add,sub,mul,neg,half}                            142
{add,sub,mul,neg,half,recip,fma}                  148
```

The shape reproduces: `{add}` and `{add,sub}` agree, `mul` roughly doubles, `half` and the rest add a
little. **So F4's phenomenon is at two instances, and mine is not independent, because I read `110`
before building it.**

### 5.2 The bound, which nobody has looked for

Two primitives with the same value set are separated by some term exactly when their realisation maps
differ somewhere a term can reach. Every term's argument to `R` is a rational. So **no signature can
separate more than "R differs somewhere on Q"**, and a signature containing a constant injection over Q
reaches that bound at depth one.

Measured, with the prediction recorded in the probe header before running:

```
{literal}                                       165
{literal,add}                                   165
{literal, everything}                           165
adding every operation to the literal splits nothing: True
and the partitions are identical, not merely equinumerous: True
finest signature without a literal reaches               148
the literal alone reaches                                165
```

**The identity relation saturates at the literal.** One nullary operation is strictly finer than the
richest operation-only signature swept, and adding `add`, `sub`, `mul`, `neg`, `half`, `recip` and `fma`
to it splits nothing, with the partitions identical rather than merely the same size.

### 5.3 What that does to the design obligation

F4's practical bite, as `110` states it, is that a canon must fix the signature or identity is a moving
target, and that P4's collapse classes break "the day somebody adds `half`". Under 5.2 that concern has a
much smaller domain than it looks:

> **A design that can write a literal is already at the finest identity its realisation map supports, so
> it does not have to declare a closed operation set to have stable identity. It has to have a
> constructor, which it cannot avoid having.**

And arvo cannot avoid it. I3, as `104` settles it, is about the experience of using the type; a type a
consumer cannot write a literal into is not unsurprising, it is unusable. `35:63-65` records that the
old tree's algorithm crates bounded their scalars on `FromConstant`, and I do not count that, for the
same reason `35` did not: it is one dead artifact. The argument does not need it. An algebra with no
nullary operation has no closed terms at all, so a signature without a constructor is a signature in
which **nothing** is reachable and every axis is vacuously reachability-degenerate.

### 5.4 And the brief's worry has no victims

The dispatch says "every count in this panel that ranged over primitives was taken under an unstated
signature". I checked:

```
grep -rn 'distinct primitive\|distinct numeral\|how many primitives\|number of primitives' \
  --include='*.md' . | grep -v '^./archive'
```

returns hits in `110` and in `109`'s account of `110`, and nothing else in the live panel. Everything
else this panel counts is **configurations**, not equivalence classes: `63`'s cube, `79`'s coordinate
list, `82`'s windows, `98`'s sections. A count of configurations is not signature-relative, because it
does not quotient. So the signature-relativity affects exactly one file's counts and that file is the
one that discovered it. Nothing needs re-doing.

---

## 6. The two degeneracies are one notion at two extents, and constant injection collapses them

`110` F5 and F6 distinguish an axis that has left the definition of `R` from one the current signature
merely fails to reach, and conclude that only the first may be canonicalised away. `110` section 16.4
says a grep suggests the distinction is new, and my own grep for `nullary`, `ground term` and
`constant injection` across the live panel agrees that nothing else carries it.

**I think the conclusion is right and the distinction as drawn is an artifact of the signatures it was
measured over.**

`110_probes/p5_definitional_versus_reachability_degeneracy.py:99-106` declares its three signatures:

```python
SIG_CLOSED = ["add", "sub", "mul", "neg"]
SIG_OPEN   = ["add", "sub", "mul", "neg", "half"]
SIG_WIDE   = ["add", "sub", "mul", "neg", "half", "recip", "fma"]
```

**None of them contains a nullary operation.** So "reachable" there means reachable from carrier
elements taken as free variables, which is a legitimate reading and is not arvo's, per section 5.3.

`111_probes/p4_constant_injection_collapses_the_two_degeneracies.py` reruns the question with the
constants put back, at `F = 0`:

```
axis varied: radix   (108 configuration pairs)
  grid-closed {add,sub,mul,neg}                   0/108
  + constants restricted to the grid              0/108
  + constants over a dense rational sample        0/108
  R differs somewhere on the rational line        0/108

axis varied: rounding (36 configuration pairs)
  grid-closed {add,sub,mul,neg}                   0/36
  + constants restricted to the grid              0/36
  + constants over a dense rational sample       33/36
  R differs somewhere on the rational line       33/36
```

and then the coincidence, cell by cell rather than as two totals, because two totals agreeing is weaker
than every cell agreeing:

```
reachable under {ops + rational constants} vs R differs on the rational line:
  agree 144, disagree 0
under GRID-RESTRICTED constants:
  agree 111, disagree 33
```

So:

> **Definitional degeneracy is reachability degeneracy evaluated at the largest signature the design
> will ever admit.** The two are one notion at two extents, and they coincide exactly when the
> constants cover the ambient domain. They come apart only where a design restricts its own literals to
> the grid.

That is a better reason for `110`'s soundness conclusion than `110` gives, and it is cheaper. "Only
canonicalise a definitional degeneracy" is not a rule about two kinds of thing that need distinguishing
by a special test. It is the ordinary conservative choice of quantifying over every signature instead of
the current one, and the test for it is the one `110` already built: probe `R` over the whole line.

**Which also means that in arvo, F6's example is not an example.** Rounding at `F = 0` is observable the
moment a consumer writes a literal that is not a grid point, and a consumer can always write one. The
practical residue of the distinction is `110`'s radix case and, on this sweep, nothing else.

### 6.1 And the residue is not a per-axis fact, which is a defect in the unit of analysis

Three of the 36 rounding pairs stay unobservable even with rational constants, and they are named in the
probe output rather than left inside a ratio:

```
W=2 unsigned sat: trunc vs floor
W=3 unsigned sat: trunc vs floor
W=4 unsigned sat: trunc vs floor
```

The overflow policy erases the disagreement: over an unsigned set every negative argument clamps to the
same endpoint under saturation, and truncation and floor differ only on negatives. Under wrapping they
separate.

**So "is this axis degenerate" has no answer.** It is a joint fact about the axis, the signedness and
the overflow policy at least, and a per-axis verdict is the wrong unit. This is I13's shape appearing
inside the identity question: the verdict is a region, and the region is over more than one coordinate.

---

## 7. `110` contradicts itself about what a split costs, and its own F9 is the resolution

Two sentences, in one file, about the same act.

`110:281-282`:

> A canonicalisation that **splits** where it could have merged costs names and nothing else.

`110:357` and `110:370`:

> Two names for one thing is not a convenience, it is a missed merge, and in a nominally typed language
> a missed merge is a wall rather than a slow path. [...] **And there is no repair.**

A canonicalisation that splits where it could have merged **is** a missed merge. So the file prices the
same act at zero in the section arguing for the conservative rule and at an unrepairable compile error
in the section arguing for the parameterisation. `110:540` half-notices, saying the "costs names and
nothing else" line is a soundness statement rather than a cost statement and that the compile-time cost
is unmeasured, but that repair does not reach far enough: F8's cost is not an unmeasured compile-time
figure, it is a hard consumer-visible refusal with no in-language fix, in the same file, compiled.

**The union-find conclusion is what has to give, and `110` supplies its own replacement.** "I would ship
the conservative one" is only free where the two rules are both available and one is safer. Here the
conservative rule ships F8's wall to every consumer who later wants one function over both spellings.
F9 is the answer: parameterise by what `R` reads, and the second spelling never exists, so the choice
between a sound rule and an exact one never has to be made.

That is not a small edit. It moves the design decision from **which canonicalisation to run** to **which
parameters to have**, which is decided once and cannot be revisited, and it is the point at which the
adequacy condition in the next section becomes the load-bearing obligation.

---

## 8. "Compile-time half" and "finite algebra over a signature" answer two different questions, and the third question between them is the one that decides the design

The dispatch asks whether the two answers are rivals, one answer in two vocabularies, or answers to two
different questions. **Two different questions**, and both files say so in phase two:
`109` section 16.3 and `110` section 16.3 independently reach the same split, that `109`'s criterion
decides **membership** (does a property belong in the primitive at all) and `110`'s decides **identity**
(are these two the same primitive).

That is right and it is not the end of it, because the split leaves a gap between them that neither
file names and that everything else in this unit rests on.

### 8.1 One is a semantics and the other is a decision procedure, and nothing states what they owe each other

`110`'s identity criterion is a denotation-preserving isomorphism over a declared signature. **It is
undecidable at real widths**, and `110` says so in its own words: exhaustive at `W <= 5`, "an exhaustive
check at a model width transfers to a real width only with an argument. I have not made one." Nothing
in a compiler will ever compute it for `W = 64`.

`109`'s criterion is about what must be const-available. **That one is decidable and is what the
compiler actually runs**: type constructor applied to type arguments, structural, trivial.

So a design has a semantic identity it cannot compute and a syntactic identity it computes for free, and
the whole question of whether the design is right is whether the second is a good decision procedure for
the first. That is a two-sided obligation and both sides have a cost:

> **Soundness.** Syntactic equality never merges two different denotations. Violating this substitutes a
> wrong answer along a name.
> **Completeness.** Syntactic inequality never splits one denotation into two names. Violating this is
> `110` F8's compile error with no repair.

`110` has both halves and never puts them together: P4 prices the unsound direction at 6 of 42 collapse
classes breaking, and F8 prices the incomplete direction at `E0308`. Its union-find paragraph then
weighs them as if only one had a cost, which is section 7's contradiction.

**And the adequacy condition is what F9 achieves.** "Parameterise by what the value set and the
realisation map read" is not a tidiness rule. It is the statement that makes syntactic identity **sound
and complete by construction** for denotational identity, which is the only way to have both when one
side is undecidable. That is the sentence I would want in a canon, and `110` writes it as a rule of
thumb about type parameters.

### 8.2 So the three-layer statement, which is what I would hand a canon writer

- **Denotation.** A value set and one realisation map, over a declared operation set. `110`, `63` C1,
  and my section 5 for the operation set's bound.
- **Type.** What must be const-available to decide validity or select a lowering. `109`.
- **Adequacy.** The type is a sound and complete decision procedure for denotational identity. Nobody's
  yet, and it is the obligation the other two are pointless without.

Adequacy is checkable the way `110` checks a congruence: at model widths, exhaustively, with the
transfer argument named rather than assumed. It is the thing a canon can demand and a design can be
audited against, and it is stated once instead of relitigated per axis.

### 8.3 And it gives `109`'s three-relation lattice the property it was missing

`109` section 10 gives three sameness relations, nominal implying representational implying
denotational, each licensing a different operation. `110` section 16.2 concedes to that framing and adds
that the denotational one is the relation closed under composition, with 131 and 17 failures for the two
weaker ones.

The adequacy condition adds the fourth fact, and it is about the top of the lattice rather than the
bottom: **nominal sameness is the only one a compiler decides, so a design is exactly as good as the gap
between nominal and denotational.** The lattice is not three coequal options to pick from per question.
It is one decidable relation at the top, one true relation at the bottom, and a design obligation to
close the distance between them.

---

## 9. What a primitive is: the refinement is the missing coordinate, and the law set is its mirror image

Now the constructive half. `95` asks a unit to end in agreement with at least something, and this is
what I believe `109`, `110`, `82` and `18` jointly support, with the corrections above applied.

### 9.1 The assumption has the law set and the refinement exactly backwards

The freedom test the panel applied to the law set has already been applied, in this panel, to another
candidate, and it came back the other way.

`82` F6, `82:768-774`: for signed saturating addition, a sign-uniform declared operand window matches
associativity on that window's generated closure exactly, zero sufficiency and zero necessity violations
over every interval at every width in `W in {2..6}`. And `82:897-900` states the structural consequence
directly:

> F6 adds one more that none of those covers: the **declared operand window**, which is a restriction on
> the inputs rather than a fact about the type's own representable set. [...] F6 separates two verdicts
> with all six of `79`'s coordinates fixed, including the representable set, and only the declared
> window moved.

**That is the freedom test, passed.** A coordinate that separates verdicts with everything else held
fixed is a coordinate, which is precisely the property `110` F3 looked for in the law set and did not
find. So:

> The working assumption lists the **law set**, which is a lossy projection of the algebra and cannot be
> varied with the others fixed, and omits the **refinement**, which is a restriction on the inputs and
> separates verdicts with everything else fixed. The one it names is a reading; the one it omits is a
> coordinate.

**This is not an independent instance and I will not have it recorded as one.** `82` is this persona.
What is new here is the connection, that the thing the assumption omits is the thing that passes the
test the thing it includes fails, and the connection is mine rather than `82`'s.

### 9.2 The refinement is also what makes the three degeneracies one thing

Section 6 shows definitional and reachability degeneracy are one notion at two extents. There is a third
extent and it is the consumer's:

- **trivial extent**, the whole carrier and every rational argument to `R`: `110`'s definitional.
- **the term algebra's image**: `110`'s reachability.
- **a declared extent**: `109` P5's carried range, `82`'s operand window.

`111_probes/p5_identity_is_relative_to_a_refinement.py` was built to establish that and **it broke.**
The obvious form of the claim, that identity relative to a declared extent is a congruence, needs the
extent to be closed under the operations, and the measurement says the merge region and the closed
region are almost disjoint:

```
completion axis: saturate against wrap, W = 4, F = 0, signature {add}
  operands <= 0   extent 1   disagreements 0    closed? yes   <== merged, and a congruence
  operands <= 7   extent 8   disagreements 0    closed? no    <== merged, but NOT closed
  operands <= 15  extent 16  disagreements 120  closed? yes
  largest sound declared bound: 0
```

Every bound from 1 to 7 merges and none of them is closed; the only closed extents are the trivial one
and the whole carrier, on which nothing merges. So the naive form is dead and its own table killed it.

### 9.3 The repair, and it is the result rather than the repair

The diagnosis is that a closed extent is what you need when the operation's result type is its operand
type. That is the endomorphism assumption `109` section 8 already refuted from the chain side, one level
up: `mul : P x P -> P` is what forces the quantisation, and the moment the result may be a different
primitive the problem changes shape.

The same move fixes the extent. An operation does not preserve a refinement, it **transforms** one:

```
add : {v <= a} x {v <= b} -> {v <= a + b}
```

which is `109` P5's `RSum` stated as a typing rather than as a bound. Then there is no invariant extent
to look for. Each node of a derivation carries its own extent and the merge is checked against that
node's, and composition works because the extent propagates rather than because it is preserved.

`111_probes/p6_the_refinement_propagates_and_that_is_the_congruence.py` measures whether the propagated
bound predicts the merge boundary, exhaustively, in **both** directions, because a rule that only
over-approximates is sound and useless:

```
W = 4, F = 0, unsigned, signature {add}, saturate against wrap
  unsound predictions (rule says merge, answers differ): 0
  conservative        (rule refuses, answers agree)    : 0
W = 4, F = 0, unsigned, signature {mul}
  unsound: 0, conservative: 0
W = 5, F = 0, unsigned, signature {add}
  unsound: 0, conservative: 0

rounding axis, W = 6, F = 2, signature {mul}, truncate against nearest
  unsound: 0, conservative: 0
```

**Exact in every cell of three completion sweeps and one rounding sweep**, at arities 2, 3 and 4. And
the propagated quantity is different per axis, magnitude for the completion and fraction width for the
rounding, so this is two arms with two predicates rather than one mechanism wearing two names.

### 9.4 It reaches the compiler, and the merge is visible in the emitted symbols

A model result about merging is worth little if the merge is invisible to the thing that lowers the
code. `109` P5 established the neighbouring half, that a carried range removes the completion from the
emitted code. What was missing is the merge itself: do two functions differing only in which completion
they name become **one body**.

`111_probes/p7_the_merge_is_visible_in_the_emitted_symbols.rs` did it with an under-controlled arm and I
kept that version rather than fixing it in place, because wrapping at the container width is the bare
add, so three of its four symbols aliased for a reason having nothing to do with the bound. The defect
is recorded in `p7_output.txt`.

`p7b_a_declared_range_inside_the_container.rs` repairs the control by declaring a logical range strictly
inside the container, saturating at 200 and wrapping modulo 201, so neither completion is free. Four
predictions were written into the probe header before running and all four hold:

```
_proved_wrap  = _proved_sat        one body, `add w0, w1, w0`, bound discharged
_unproved_sat = _ungated_sat       a second body, the clamp
_unproved_wrap = _ungated_wrap     a third body, the modulus
```

with behaviour checked alongside, 10201 of 10201 pairs agreeing inside the proved bound and 20100 of
40401 differing outside it. No feature gate. **Three distinct bodies where the semantics says three,
one where it says one.**

### 9.5 The obvious objection to my own answer, tested, and it does not land

A refinement in the type means more types. A design worried about extra names has just acquired a lot of
them, and `110` F8 prices extra names at a compile error with no repair. So does my answer import the
hazard it was supposed to help with?

**No, and the reason has a name.** A **spurious** parameter is one `R` does not read: two types
differing only in it denote the same thing, so what is wanted is equality, and Rust gives no way to make
two type constructors applied to different arguments equal. A **refinement** parameter is read, by the
arm selection rather than by `R`: two types differing only in it denote different sets, one contained in
the other, so what is wanted is not equality but **weakening**, and weakening is a total function that
is the identity on the representation.

`111_probes/p8_a_refinement_parameter_has_a_repair_a_spurious_one_does_not.rs`, three predictions
recorded before running:

```
widening is the identity on all 256 of 256 representations
_widen_100_to_200 = _plain_identity
_widen_7_to_15    = _plain_identity
_widen_derived    = _plain_identity
```

and the wrong direction is refused at build time rather than at runtime, which is what I15 requires:

```
error[E0080]: evaluation panicked: widening must not tighten the bound
   evaluation of `widen::<Lit<200>, Lit<100>>::{constant#0}` failed here
```

naming the exact instantiation. No feature gate on either file.

> **An axis nothing reads has no repair. An axis the arm selection reads has one, it is weakening, it is
> free at runtime, and its violation is a build failure.** So the cost `110` F8 measures is a cost of
> spurious parameters specifically, and does not transfer to refinement parameters.

### 9.6 The statement, offered

Suggestions. Op decides, and per I12 an opinion before the experts converge is an ack.

> A **primitive** is a value set, one realisation map taking an exact result back into it, and a
> **refinement** naming which values the primitive is declared to hold. Its **denotation** is the term
> algebra these induce over a declared operation set, restricted to the refinement's extent.
>
> Its **identity** is denotational sameness of that restricted algebra. The refinement is part of it:
> two primitives differing only in the realisation map's behaviour outside the refinement's extent are
> the same primitive, and that is what licenses substituting one arm for another.
>
> A **law** is read off the algebra and never declared. Read as a **demand** it is a predicate over the
> configuration space, which is a surface a consumer may use and not a field a consumer may set.
>
> An **axis is degenerate relative to an extent**, and the three extents that matter are the whole
> ambient domain, the image of the operation set, and the consumer's declaration. Only degeneracy at the
> first may be canonicalised away, because it is the only one that quantifies over every operation set
> the design will ever have.
>
> A **refinement is transformed by an operation rather than preserved by it**, so composition is a
> derivation rather than an invariant, and each node's licence is checked against its own extent.
>
> The **type** carries whatever must be const-available to decide validity or select a lowering, and it
> owes the denotation both **soundness** and **completeness**: never one name for two denotations, never
> two names for one. An axis the realisation map does not read must not be a parameter, because its
> presence breaks completeness and nothing in the language repairs it. An axis the arm selection reads
> may be a parameter, because weakening repairs it and weakening is free.

**Permanence.** Every sentence survives a rewrite in another language or decade. None names a container,
a width, a marker, a type parameter, a crate, or a count.

**Equivalence.** Three teams implementing this produce units that behave the same on what matters: a
consumer declares a range and gets the cheap arm where the range proves it, an invalid declaration is a
build failure rather than a runtime one, no law is writable by hand, and two spellings of one denotation
do not exist because the parameter list was chosen so they cannot. They differ on how the refinement is
spelled, whether weakening is a method or a coercion, and how many named primitives ship.

**Where it is weaker than I would like, stated rather than hidden.** The soundness-and-completeness
obligation is checkable at model widths and I have not made the transfer argument to real widths, and
`unstable-features.md` is explicit that one is owed. The refinement in section 9.3 is an upper bound on
magnitude and a fraction width; whether every axis of `R` has a propagable quantity is untested and I
would expect the signedness not to. And nothing here prices anything: the word for every magnitude in
this file is unpriced.
