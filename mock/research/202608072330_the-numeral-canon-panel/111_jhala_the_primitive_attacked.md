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

---

## 10. Findings, each with its predicate

Per I13 and `RULES.md`. Two conventions stated once rather than repeated.

**Threads.** Everything enumerative below ran on one thread and is predicated `threads = 1`. Two
findings carry `threads any`, and both are compile-time structural results where the argument is
stateable rather than assumed: what rustc accepts, and which symbols it emits for a given source, are
not functions of a runtime thread count.

**Target features.** The model sweeps are exact rational arithmetic whose results no instruction
selection can move, so they carry `target features any` with that as the argument. The two assembly
findings carry the host default explicitly, because there the feature set is the thing being read.

**Model width.** Every enumerative finding is at `W <= 6`. I have made **no transfer argument** to any
real width, and `unstable-features.md` is explicit that a model-width check needs one and that the
enumeration of routes by which behaviour can vary per instantiation is not exhaustive. Every predicate
below therefore lists its width as a fixed set.

**F111-1. A law computed at const time needs the operation statically resolved, not reified as a type.**
Three carriers agree on all three censuses: a const generic value with a match, macro expansion, and
(committed, shipped) two hand-duplicated const fns. `toolchain = nightly-2026-05-28, rustc 1.98.0-nightly
(57d06900f 2026-05-27), edition 2021, feature gates = none, W = 4, I = 4, F = 0, signedness = signed,
overflow policy in {saturate-both, wrap, saturate-top-only}, operation = add, arity = 2, chain length =
3, threads any, target features any`. `p1a_output.txt`, `p1b_output.txt`, `p1c_output.txt`, and
`mock/benches/variants/satfold-shared/src/lib.rs:519` and `:547`.

**F111-2. The carrier choice does not repair a nominal split.** A completion carried as a type produces
the same `E0308` as `110` F8's const generic value. Same toolchain predicate as F111-1, `threads any`,
`target features any`. `p1d_output.txt`.

**F111-3. `110` F3's third bullet is a dead branch, and a mutation does not move its verdict.** Zero key
collisions, zero law-set comparisons performed, and a freely declared law set over the same 48
configurations still reports DETERMINED. `sweep = the 48 configurations of 110_probes/p2 lines 190-194,
key = that file's lines 51-52, threads = 1, target features any`. `p2_output.txt`.

**F111-4. Constant injection collapses definitional and reachability degeneracy, and grid-restricted
constants separate them.** Cell by cell, 144 agree and 0 disagree with rational constants; 111 agree and
33 disagree with grid-restricted constants. `W in {2,3,4}, F = 0, signedness any, overflow policy in
{sat,wrap}, rounding in {near,trunc,floor}, radix in {2,3,5}, signature in {grid-closed, +grid
constants, +rational constants}, threads = 1, target features any`. `p4_output.txt`.

**F111-5. Rounding at `F = 0` is observable under any signature containing a rational literal, and
unobservable under one restricted to grid constants.** 33 of 36 pairs, with the three exceptions named.
Same predicate as F111-4. `p4_output.txt`.

**F111-6. Observability of the rounding mode is a joint fact with the signedness and the overflow
policy, not a per-axis one.** Truncate and floor stay unobservable at unsigned saturating and separate
at unsigned wrapping. `W in {2,3,4}, F = 0, signedness = unsigned, overflow policy in {sat,wrap},
rounding in {trunc,floor}, radix = 2, signature = +rational constants, threads = 1, target features
any`. `p4_output.txt`.

**F111-7. The identity relation saturates at the literal.** `{literal}` and `{literal, add, sub, mul,
neg, half, recip, fma}` induce the identical partition, 165 classes, against 148 for the richest
operation-only signature swept. `W in {2,3,4}, F in 0..=2 with F <= W, signedness any, overflow policy
in {sat,wrap}, rounding in {near,trunc,floor}, radix in {2,3}, threads = 1, target features any`.
`p3_output.txt`. The bound half is structural rather than enumerative: every term's argument to `R` is a
rational, so no signature separates more than `R` differing on Q, which is an argument and not a sweep.

**F111-8. The extent on which two completions merge and the extent closed under the operations are
almost disjoint.** At `W = 4` under `{add}`, every bound from 1 to 7 merges and none is closed; the only
closed extents are `{0}` and the whole carrier, on which nothing merges. `W = 4, F = 0, signedness =
unsigned, overflow policy in {sat,wrap}, operation in {add}, {add,mul}, arity = 2, radix = 2, threads =
1, target features any`. `p5_output.txt`. This is a **falsification of my own hypothesis** and it stands
as recorded.

**F111-9. The propagated bound predicts the completion merge boundary exactly, in both directions.**
Zero unsound and zero conservative predictions over every cell of three sweeps. `W in {4,5}, F = 0,
signedness = unsigned, overflow policy in {sat,wrap}, operation in {add},{mul}, arity in {2,3,4},
operand bound in 0..=2^W-1, association = left-nested, threads = 1, target features any`.
`p6_output.txt`.

**F111-10. The same holds one axis over, with the propagated quantity being the fraction width rather
than the magnitude.** Zero unsound and zero conservative. `W = 6, F = 2, signedness = unsigned, overflow
policy = sat, rounding in {trunc,near}, operation = mul, arity in {2,3}, operand grid = multiples of
2^-c for c in {0,1,2}, radix = 2, threads = 1, target features any`. `p6_output.txt`.

**F111-11. Where the bound discharges, two completions compile to one body; where it does not, they
compile to three.** `_proved_wrap = _proved_sat`, `_unproved_sat = _ungated_sat`, `_unproved_wrap =
_ungated_wrap`. `toolchain = nightly-2026-05-28, edition 2021, feature gates = none, target =
aarch64-apple-darwin, target features = host default, opt level = 3, container = u8, declared logical
range = 0..=200, operand bounds in {100, 200}, operation = add, arity = 2, threads any`.
`p7b_output.txt`, `p7b_asm.s`.

**F111-12. A widening of a carried bound is the identity in the emitted code, and a tightening is a
build failure naming the instantiation.** Three widenings alias `_plain_identity`; the tightening is
`E0080` at `widen::<Lit<200>, Lit<100>>`. Same toolchain and target predicate as F111-11, `threads any`.
`p8_output.txt`.

**F111-13. The suite is 123 across 13 and all of it passes, and `wide-rung-shared` takes 4.25s.**
`toolchain = nightly-2026-05-28, host = this machine, --release, --test-threads=1, threads = 1`.
`p0_test_gate_run.txt`. The serial flag is load-bearing: `110` F14's hang under the default runner is
not re-measured here and I took its workaround rather than reproducing its defect.

**Unpriced.** Everything about cost. I ran no bench, took no timing that decides anything, and made no
claim depending on a magnitude. The symbol counts in F111-11 and F111-12 are structural observations
about which bodies exist, not measurements of anything, and two instructions against six decides
nothing.

---

## 11. Options: what this fits, fits badly, kills, and adds

**Fits well.** `OPTIONS.md` Q16's third way out, that the two senses of composition are one concept at
two scales, gains support it did not have: under section 9.6 a refined primitive and a composite are
both carriers with an interpretation, so `110`'s fourth option (sense one is configuration, only sense
two is composition, and the concept is closed under it) survives my derivation and I would keep it.

**Fits well.** Q51's converged strategy statement, where it says an axis belongs to the observable
assignment "if there is **any** reachable chain on which moving it is observable; where a particular
chain cannot observe it, that is a licence the resolver may take under a predicate over the chain, not a
reclassification of the axis". That is section 6's conclusion arriving from the strategy unit, and `108`
got there first. **My section 6 is a second instance of that shape on a different object**, and it is
not independent, because I read `108` section 7 before writing.

**Fits badly, at a nameable cost.** Any option that carries the completion as a type parameter and
nothing else. It survives, and it pays F111-8's price: the merge that a declared range licenses is
invisible to it, so every consumer whose operands are bounded pays for a clamp that cannot fire. The
cost is a lost optimisation rather than a wrong answer, so this is fitting badly and not being killed.

**Killed, with the diagnostic.** Any option under which "identity relative to a declared extent" is
established by finding an extent closed under the operations. F111-8: the merge region and the closed
region are almost disjoint, and the largest sound closed bound at `W = 4` under `{add}` is 0.

**Killed, with the diagnostic.** Any option that treats "the operation must be a type for a law to be
computable" as a design constraint. F111-1, three carriers, one of them already shipped in this
repository.

**Not killed but withdrawn as a measurement.** `110` F3's third bullet. The conclusion it supports
stands on `110`'s TEST 2 and on `109` P2 and `90` R3; the count should not be cited.

**Added.** A refinement is a coordinate of a primitive, and it is the coordinate the four-part
assumption omits while including its mirror image. `82` F6 is the measurement, this file is the
connection, and the two are the same persona so this is one instance and not two.

**Added.** The adequacy condition of section 8: the type owes the denotation soundness and completeness,
and `110` F9 is what discharges both by construction.

---

## 12. Alternatives I considered and did not take

For the next expert attacking from a different angle, so the list is the starting point rather than
nothing.

**A. Making the refinement a value rather than a type.** `109`'s section on the request-and-resolution
pair, and `83`'s const-availability widening, both point at a refinement supplied as a const expression
from outside the typestate. That would make the bound per-call-site rather than per-value and would
avoid every extra type. I did not take it because a per-site bound cannot survive being stored: a value
written into a column loses the declaration, and the next read has to re-establish it. Worth attacking:
whether the storage boundary is the only place the type form is needed, in which case both forms ship
with a predicate each.

**B. Making the refinement an interval rather than an upper bound.** Everything I built carries one
endpoint. `82`'s window carries two and its sign-uniformity predicate needs both. Two endpoints is
strictly more expressive and strictly more to establish at a construction site, and I did not measure
the difference. `109` section 14 flags the same gap from the range side, that its product rule takes the
corners and is correct only for non-negative ranges.

**C. Refining the realisation map rather than the carrier.** Instead of "which values may this hold",
"which region of `R` may fire". These are interderivable in the cases I measured and they are not the
same statement in general, because a policy can be unreachable for reasons other than the operand range,
which is what section 6.1's saturation-hides-rounding cell is. I did not develop it and I think it is
the more general form.

**D. Dropping the completion from the type entirely and making every primitive total by refinement.**
The most aggressive reading of I15 available, and `109` section 13's option C names it too. Under it a
primitive has no completion at all and an operation whose range is not proved simply does not typecheck.
It is coherent, it is what a refinement type system normally does, and I did not take it because it puts
an obligation at every construction site including the ones fed by a C ABI, which `88` section 3 says
is the consumer's boundary rather than arvo's. Whether the obligation is bearable is a question about
hilavitkutin and vehje, not about arvo, which is exactly what `109` says about the same option.

**E. Asking whether the refinement is a strategy.** `106` section 1's first component is "an assignment
on the axes a consumer can observe, supplied and never derived", which is what a declared range is. If
the refinement is one of those axes, then the strategy pair already contains it and section 9's
coordinate is not new, it is a member of a component the previous unit named. I could not settle it and
it is the sharpest attack on section 9 available. What would decide it: whether a declared range is
recoverable from the bits. It is not, which is the pair's own criterion for component one, so **I lean
toward it being one of those axes** and say so rather than claiming a new coordinate. Stated as a
located uncertainty rather than resolved.

**F. Whether the law-as-demand surface is a solver.** `110`'s TEST 4 runs a demand as a query and gets a
subspace back. That is one step from a consumer writing `where Self: DistributesOverAdd` and the design
solving for a configuration, which is the refinement idea applied to the configuration space rather than
to the carrier. I did not build it. The obstacle I expect is that the solution set is not a single point,
so the design would have to pick, which is policy rather than toolbox and `arvo-toolbox-not-policer.md`
would bite.

---

## 13. Coverage, bounded

**Read in full:** `INTENTS.md`, `RULES.md`, `109` and `110` both phases, op's `88`, `95`, `104`, `105`.
**Read in part:** `106` sections 1, 11, 16, 17, 18; `108` section 7 and sections 8 to 10; `OPTIONS.md`
Q16 and Q51; `18` section 3.1 and its surrounds; `82` F6, F7 and section "what this fits";
`110_probes/p2` and `p5` in the source; `satfold-shared`, `quantiser-radix-shared` and
`quantiser-fadd-shared` test bodies.

**Not read:** `OPTIONS.md` outside two entries, `DROPLIST.md`, `PERSONA_CALLS.md`, `PRIOR_CALLS.md`, the
`SEED_*` files, the archive, `63`, `74`, `90`, `93`, `94`, `97`, `98`, `100`, `101`, `102`, `103`, `107`,
and the ninety-odd files I have not named. Where I cite one of those, I am citing `106`'s or `109`'s or
`110`'s account of it and I say so at the citation. In particular **`90` R3 and `63` C1 reach me only
through `109` and `110`**, and `97` F-H reaches me only through `106` section 16.

**Verified at source rather than through an account:** `110_probes/p2`'s sweep and key, quoted with line
numbers; `110_probes/p5`'s three signatures, quoted with line numbers; `110`'s two contradicting
sentences, opened; `82` F6 and `82:897-900`, opened; `18:363`, opened; `satfold-shared`'s two const
census functions, opened; the test count, re-run.

**Not verified:** every number I quote from `109`, `110`, `106`, `108` or `82` that I did not reproduce.
Where I reproduced one I say so: `109`'s 952, 448 and 36 (reproduced in `p1b`), `109`'s 32640 (reproduced
incidentally in `p7`), `110`'s 0-of-108 radix result (cross-checked in `p3` at 0 of 72 pairs, agreeing).

**The largest thing I did not do.** I did not attack `109`'s section 8 chain result or `110`'s P7 and
P8 composite results at all. `106` section 11 already carries the chain material at three instances and
`110`'s composite work is untouched by anything I built. If one dispatch follows this one, the composite
side is where nobody has pushed back.

**And I did not resolve section 12's alternative E**, which is whether the refinement is a new
coordinate or a member of the strategy pair's first component. I lean toward the second and could not
settle it, and it is the attack I would most want run against section 9.

---

## 14. Probe index

All under `111_probes/`, each committed as it ran, before this file was written.

- `p0_test_gate_run.txt`. Every test-bearing variant crate, per crate, serially. 123 pass.
- `p1a_fn_pointer_in_const_fn.rs`, `p1a_output.txt`. `109`'s wall reproduced verbatim.
- `p1b_const_generic_tag_and_match.rs`, `p1b_output.txt`. The operation as a const generic value, no
  feature gate, reproducing `109`'s 952, 448 and 36.
- `p1c_macro_carried_operation.rs`, `p1c_output.txt`. The operation as macro syntax, agreeing with p1b
  inside the probe.
- `p1d_the_carrier_choice_does_not_repair_the_split.rs`, `p1d_output.txt`. Expected failure. A type
  carrier splits the same way a value carrier does.
- `p2_the_law_set_freedom_test_is_a_dead_branch.py`, `p2_output.txt`. Zero comparisons performed, and a
  mutation that should move the verdict does not.
- `p3_the_signature_saturates_at_the_literal.py`, `p3_output.txt`. Nine signatures, the saturation
  result, the monotonicity instrument check, and the cross-check against `110` F5.
- `p4_constant_injection_collapses_the_two_degeneracies.py`, `p4_output.txt`. The coincidence cell by
  cell, and the three named exceptions.
- `p5_identity_is_relative_to_a_refinement.py`, `p5_output.txt`. **My falsified hypothesis**, kept.
- `p6_the_refinement_propagates_and_that_is_the_congruence.py`, `p6_output.txt`. The repair, exact in
  both directions on four sweeps.
- `p7_the_merge_is_visible_in_the_emitted_symbols.rs`, `p7_output.txt`, `p7_asm.s`. The under-controlled
  first version, with its defect named in its own output.
- `p7b_a_declared_range_inside_the_container.rs`, `p7b_output.txt`, `p7b_asm.s`. The repaired control,
  four predictions recorded before running and all four confirmed.
- `p8_a_refinement_parameter_has_a_repair_a_spurious_one_does_not.rs`, `p8_output.txt`, `p8_asm.s`, with
  `p8b_a_tightening_is_refused_at_build_time.rs`. Weakening is the identity; tightening is `E0080`.

---

## 15. Two additions found after the file was drafted, one of which earns a rung

### 15.1 F111-9 is `97`'s congruence criterion arriving from the identity side, and I did not read `97` first

`97:706-707` states, before running its own probe:

> a law holds in the representable set **iff** it is an identity of exact arithmetic **and** `pi`
> respects every ordered nesting of operations the law contains.

My F111-9 says two completions agree on a term exactly when the propagated bound discharges at every
node. Those are the same condition. Where `pi` never fires, `pi` is the identity on that node's result,
so it trivially respects that nesting; and the propagated bound discharging at every node is precisely
"`pi` respects every ordered nesting". **One criterion, two questions**: `97` asks which laws hold, I ask
which primitives are the same, and the answer is the same predicate over the same object.

**And this one is independent, which almost nothing else in this file is.** p6 was built and committed
at `c2c8382b` before I opened `97`; the ordering is checkable in `git log`, since the grep that surfaced
`97` to me runs several commits later. I did not have the criterion and derived its shape from a
falsified hypothesis of my own. So this is a second instance in the sense the middle rung wants, on a
claim `97` states and `arvo-always-optimal-internals.md` already carries.

The extension is small and I state it as an extension rather than as a finding: `97`'s criterion is
about a **law** holding, and the same predicate decides an **identity** merging. A canon that states it
once covers both, and stating it twice would be two statements of one fact.

### 15.2 The narrower form of section 5.4's negative claim

Section 5.4 says no other panel file counts primitives up to identity. Two greps back it, and a negative
claim about evidence is a claim about a place, so both are named:

```
grep -rn 'distinct primitive\|distinct numeral\|how many primitives\|number of primitives' \
  --include='*.md' . | grep -v '^./archive'
grep -rn 'equivalence class\|up to isomorphism\|distinct algebra\|quotient' \
  --include='*.md' . | grep -v '^./archive'
```

The first returns `109`, `110` and nothing else live. The second returns quotient material in `97`,
`07`, `65` and `DROPLIST.md`, all of it about laws and orders rather than about counting classes.
**Neither grep reaches `.claude/` or `.github/`**, per `109` section 12.3's finding that the shimmed
`grep` skips dot directories from a bare `.`; I did not name those trees because no panel count lives in
them, which is an assumption rather than a check.

### 15.3 Citations, opened

`111_probes/p9_check_my_own_citations.py` opens every `file:line` this file leans on and tests the
substring the claim depends on, against the cited line and its two neighbours. **18 checked, 0 failing.**

Because it came out green, it is mutation-tested rather than trusted: a wrong line number, a wrong
substring and a wrong file each produce exactly one failure and a non-zero exit, recorded in
`p9_output.txt`. What it does not cover is every number I quote from another file without reproducing,
which section 13 lists separately.

---

## 16. Probe index, addition

- `p9_check_my_own_citations.py`, `p9_output.txt`. Eighteen citations opened and their content tested,
  with three mutations confirming the instrument fails when it should.

---

# Reply to `112`

Appended after the phase and its sections were committed, under `113`'s new shape: a refutation is the
middle of the work, the attacker owes alternatives to the party it refuted, and the refuted party is
brought back to answer rather than replaced. `112` did owe them and did supply them, so this is a reply
with something in hand rather than a defence.

**Nothing above this line is edited.** Two of my findings are corrected below and one of my probes is
defective; both stay as written, because a file that quietly absorbs its corrections destroys the record
of having been wrong.

Two new probes, `r1` and `r2`, committed as they ran before this section was written.

## 17. What I concede, and it is more than `112` claimed

**F111-9's zero-conservative result is a property of the term shape. Conceded in full.** Every sweep in
my p6 is a left-nested chain of one operation over independent leaves, and `112` section 6 is right that
reporting the zero without the shape reads as a property of the mechanism. On `(x + y) - y` the same rule
licenses 16 of the 136 extents on which the arms in fact agree, losing 88.2%.

The predicate-discipline defence is available to me and I am not taking it. My F111-9 lists
`association = left-nested` and one operation per sweep, and under `RULES.md`'s absence rule a dimension
nobody names is claimed nowhere, so leaf-sharing being absent means the finding never claimed anything
about repeated leaves. That is technically true and it is worthless, because **a reader cannot act on an
absence they do not know to look for**, and the proof is that `OPTIONS.md` Q52 picked the result up as
"exactly, zero unsound and zero conservative" with no shape attached, within one file of my writing it.
The notation protected me and did not protect the reader. Naming the dimension is the fix and section
18.3 names it.

**And `112` found a defect in my p6 without knowing it was mine.** Its `p7c` establishes that a root-only
discharge check is unsound, and my p6's `propagated_bound(B, k, op)` returns `B * k` and `B ** k`, which
is **the root and nothing else**. It is sound in my sweep for exactly the reason `112` gives for its own
p7b mutation failing to fire: every extent is one-sided from zero and every chain is monotone increasing,
so the root is the widest node and root-only coincides with per-node by construction. So F111-9's rule as
implemented is the unsound form, evaluated in the one region where it cannot go wrong. Both probes below
use the per-node check.

**My lean on alternative E is wrong.** `112` is right that the refinement is not a member of the declared
semantics. The measurement is in section 18.1 and the route to it is neither of ours.

## 18. Where both of us misread `108`, and the criterion is the sentence after the one we each used

### 18.1 The clause has three sentences and the criterion is the third

`108:822-827`, opened:

> The **declared semantics** is an assignment on the axes a consumer can observe: those where moving the
> assignment changes what the program denotes, or whether it denotes at all. It is supplied and never
> derived, because a consumer of a value cannot recover it from the bits, so every consumer of that value
> must agree about it. **An axis belongs here if there is any reachable chain on which moving it is
> observable**; where a particular chain cannot observe it, that is a licence the resolver may take under
> a predicate over the chain, not a reclassification of the axis.

Three sentences. The first is the definition. The second is a property with its reason and its
consequence. **The third, at `108:825`, is the membership criterion, and it says so in the words "an axis
belongs here if".**

I used the **reason** from the middle sentence. `112` section 3.2 corrected me to the **consequence**
from the same middle sentence, and called my error a premise used as a criterion. It is the right
diagnosis of my error and it is the same error: `112` reached into the middle sentence too, one clause
further along. Neither of us used the criterion, which was sitting in the next sentence, unqualified, in
imperative form.

### 18.2 Applying it, which nobody had done

The criterion asks whether moving the axis is observable on any reachable chain. Nobody moved a
refinement. `111_probes/r1_moving_only_the_refinement.py` does: primitive fixed, every axis fixed, only
the declared bound moved, every weakening pair, exhaustive over the values the tighter declaration admits.

```
primitive held fixed at uW3/sat, declarations one-sided [0, b]

  term              decl pairs  arm changed  pairs disagreeing  value disagreements
  x + y                   1296          756                  0                    0
  x * y                   1296          711                  0                    0
  (x + y) - y             1296          252                  0                    0
  (x + y) + z               36           15                  0                    0
  (x + y) * z               36           12                  0                    0
  (x + y) - z               36            7                  0                    0

  declaration pairs where the SELECTED ARM changed : 1753
  chains where the ANSWER changed                  : 0
```

with the control moving an observable axis over the same terms and the same declarations:

```
  x + y            overflow policy             1296             210
  (x + y) + z      overflow policy            46656           19236
  (x + y) + z      signedness                 46656           39153
```

**1753 declaration pairs change which arm is selected and none of them changes an answer**, and the
control shows the instrument reporting differences in the tens of thousands when an axis that does change
the denotation is moved instead. So the zero is a result rather than a dead branch, which is the check my
own section 2 demanded of `110` and which I owe my own probes.

Under `108`'s stated criterion, a refinement is not a member of the declared semantics. **`112`'s verdict
stands and I withdraw my lean.** What I would change is the argument: it does not need the
premise-and-conclusion analysis at all, because the criterion is stated and applying it settles the
question in one sweep.

### 18.3 `112`'s own supporting measurement is about something else

Said plainly because `112` would want it said. Section 3.2 argues that the agreement property "fails for
a refinement, because two consumers holding different declarations of the same value both compute
correctly whenever one weakens to the other" (`112:110`), and cites `p2b`'s Q2-Q6 table.

That table does not move the refinement. Reading
`112_probes/p2b_the_grade_repaired_and_the_two_regions_separated.py` at its `row(...)` calls, every row
moves the **overflow policy**, the **total width** or the **signedness**, with the grade held fixed at
`g3`, `g7` or `g15`. What it measures is that a discharged grade licenses substituting across an
observable axis, which is a real and useful result and is `112` F112-3. It is not a measurement of what
happens when the refinement moves.

**This does not touch `112`'s conclusion**, which R1 confirms by the route the document itself specifies.
It corrects what the conclusion rests on, and it is the same class as the defect `112` found in my p6:
a claim measured in the one configuration where the instrument could not disagree with it.

## 19. F112-6's boundary is refuted by `112`'s own probe output, and the replacement is a predicate

### 19.1 Three counterexamples, all in `p3b_output.txt`

`112` F112-6 (`112:1029`): the corner bound is "exact **only** on left-nested chains of one operation over
independent leaves". "Only" makes it necessary. From `112_probes/p3b_output.txt`:

```
(x + y) - z, SIGNED sat W=4     conservative   0     mixed operations, exact
(x + y) - x, SIGNED sat W=4     conservative   0     repeated leaf, exact
x - x,       SIGNED sat W=4     conservative   0     repeated leaf, exact
(x + y) * z, unsigned sat W=4   conservative 120     independent leaves, NOT exact
```

So one operation is not necessary, independent leaves are not sufficient, and the shape list is not the
boundary. `112` sees half of it at F112-8 ("a repeated leaf is not sufficient", `112:1042`) and does not
carry the correction back into F112-6, which is the finding a consolidation would pick up.

### 19.2 The replacement, which is structural and computable from what a type already carries

`111_probes/r2_a_structural_predicate_for_where_the_corner_rule_is_exact.py`. Two conditions, one per
source of conservatism, which is `112` section 6's own decomposition turned into a test:

> **(a)** every leaf occurs at most once, so at every node the two children have disjoint leaf sets and
> the corner rule is exact on the **range** by induction.
> **(b)** no internal node has an ancestor multiplication whose other child's interval contains zero, so
> no node's overflow can be **annihilated** downstream. The root is excluded, because a root has nothing
> downstream to be masked by, which is exactly why `x * y` is exact and `(x + y) * z` is not.

Measured over twelve rows, including two neither of us had swept:

```
  term           primitive     unsnd  consv  exact  pred fires  pred wrong
  x + y          uW4/sat           0      0    256         256           0
  x * y          uW4/sat           0      0    256         256           0
  (x + y) + z    uW4/sat           0      0   4096        4096           0
  (x + y) - z    iW4/sat           0      0    512         512           0
  (x + y) * z    uW4/sat           0    120   3976           0           0
  (x * y) + z    uW4/sat           0      0   4096        4096           0
  (x + y) - y    uW4/sat           0    120    136           0           0
  (x + y) - x    iW4/sat           0      0     64           0           0
  x - x          iW4/sat           0      0      8           0           0
  x * (y - y)    iW4/sat           0     33     31           0           0
  x + y          uW4/wrap          0      0    256         256           0
  (x + y) - y    uW4/wrap          0    240     16           0           0

  predicate violations across every row: 0
```

**Zero violations**: wherever the predicate fires the conservative count is zero. It is **sufficient and
not necessary**, and it is over-conservative on exactly the two rows `112` flagged, `x - x` and
`(x + y) - x`, where a repeated leaf's over-approximation never crosses a container edge. `(x * y) + z`
is a row neither file had and it is exact, which is the case that shows condition (b) is about the
ancestor rather than about the presence of a multiplication.

**Expressibility rests on `112` rather than on me.** Its `p8c` compiles a per-node check by recursing
over the grade's own type structure, on the observation that a composed grade is a composed type. Leaf
multiplicity and a sibling's interval are properties of that same structure, so the predicate is
computable where the discharge check already is. I did not compile it and I am not claiming I did.

## 20. The annihilation case dissolves, and it was an artifact of the extents we both swept

This is the part `112` unlocked for me, and it is the thing I could not have reached before reading it.

`112` F112-7 (`112:1039`) reports that on `(x + y) * z` the corner rule and an enumerating oracle both
license 385 of 4096 while the arms agree on 120 more, "which no node-wise rule reaches", because the fact
is about the term's result not depending on the node. Separately, `112` F112-23 (`112:1142`) records that
**every declared extent in every sweep of that file is one-sided**, with the lower bound pinned at zero.

Those two are the same fact and neither file connects them. A one-sided extent from zero **always
contains the annihilator of multiplication**. So the annihilation case is not a property of node-wise
rules; it is a property of declarations that cannot exclude zero. Raise the lower bound:

```
  z declared             extents   consv   exact  pred fires
  [0, zhi]                  4096     120    3976           0
  [1, zhi]                  3840       0    3840        3840
  [2, zhi]                  3584       0    3584        3584
  [3, zhi]                  3328       0    3328        3328
```

**The conservatism goes to zero the moment the multiplier is declared away from zero**, and the
structural predicate fires exactly where it does, on all 3840 of the `[1, zhi]` cells.

Three consequences.

**`112`'s "no node-wise rule can" is right about one-sided declarations and wrong in general.** A
node-wise rule reaches it, and the thing that reaches it is the declaration's lower bound rather than a
cleverer propagation.

**It is a second argument for two-endpoint declarations**, alongside `82` F6's sign-uniform window which
needs both endpoints and `112`'s own F112-21 witness which had to use one. My section 12 alternative B
listed two-endpoint windows as untested and strictly more expressive; this prices one thing they buy.

**And it redirects `112`'s alternative E.** That alternative sends the next dispatch at the annihilation
case on the ground that its shape is a masked or predicated lane, which is ordinary in the consumers I11
names. The measurement says the annihilation case is reachable already, by a declaration a consumer can
write, so the dispatch that would have gone after a smarter rule should instead go after the two-endpoint
declaration and what it costs to establish at a construction site.

## 21. Is `112`'s propagation rule better, or better on the shapes it chose? Both, and the answer is a composition

`112` section 6b.3's one-sided form with a corner cross-term reaches an enumerating oracle on all
thirteen rows it swept and is beaten on none. I have no reason to doubt it and I did not re-run it. It is
a better propagation rule than mine on every shape either of us has measured.

**What domination on thirteen rows does not answer is what the rule costs to carry**, and that is where
the composition lives. The corner rule carries two numbers per node whatever the term is. An affine form
carries one coefficient per leaf plus one per non-constant multiplication, which is `112`'s own statement
of it at F112-20 and is a property of the term:

```
  term                        corner state  affine state  leaf repeats
  x + y                                  2             2         False
  (x + y) - y                            2             2          True
  (x + y) * z                            2             4         False
  fold of 8 adds                         2             8         False
  fold of 16 adds                        2            16         False
  fold of 64 adds                        2            64         False
```

**On a fold no leaf repeats, the predicate of section 19.2 fires, the corner rule is exact, and the
affine form buys nothing while costing one coefficient per element.** The fold is the shape
`satfold-shared` and `warm-clamp-shared` are both built around, and `112` says so itself in section 6's
closing line. At a 64-element fold that is a 64-long type-level list propagated at compile time against
two integers, for zero additional licences.

So the honest deliverable is not which rule wins. It is:

> **Where the structural predicate fires, the corner rule is exact and is the cheaper carrier. Where a
> leaf repeats, the corner rule loses licences and the one-signed affine form recovers them. The
> predicate is const-checkable and decides between them statically.**

Two arms, one predicate, which is I13 rather than a ranking, and it is `never-ask-which-single-rule-governs.md`
applied to a question I could otherwise have asked as "whose propagation rule".

**This sharpens `112` section 9's clause rather than contradicting it.** That clause says sound rules
disjoin and a design carries as many as it can afford. Disjoining is the right default and needs no
analysis; what the predicate adds is that a design **can** afford to carry one, because the choice is
decidable from the term. Where the affine state is O(n) in the fold length, that difference is the whole
question.

## 22. What I hold, with the reason

**F111-9 holds, with the dimension named.** Not withdrawn. The rule is exact on the region section 19.2's
predicate describes, and `112`'s own measurement of `(x + y) - z` and mine of `(x * y) + z` both sit
inside it and outside `112`'s stated boundary. A finding that holds on a region and fails outside it is
two arms, which is the posture I13 ratifies, and withdrawing it would discard a real region because its
first statement of the region was too narrow to be read.

**And the correction to it is not the one `112` made.** `112` narrows my region to left-nested chains of
one operation over independent leaves. The measured region is wider on both axes: mixed operations are
fine, and independent leaves are neither necessary nor sufficient.

**I do not hold my section 9.6's expectation about signedness.** `112` section 5 refutes it with a
measurement and I accept it: a magnitude bound licenses the signedness substitution, because the grade
acts on `R`'s argument rather than on the axis. That is a better statement of my own mechanism than I had.

## 23. Findings, with predicates

**F111-14. Moving only the refinement changes which arm is selected and never changes an answer.** 1753
declaration pairs across six terms change the selected arm; zero of them change any value. `W = 3, F = 0,
signedness = unsigned, overflow policy = sat, rounding = trunc, radix = 2, operations in {add, sub, mul},
term shapes as enumerated in the probe, arity in {2, 3}, declarations = one-sided [0, b], every pair at
two leaves and uniform at three, discharge check = per node, threads = 1, target features any`.
`r1_output.txt`. Control: the same terms with the overflow policy or the signedness moved instead give
210 to 39153 disagreements, so the instrument fires.

**F111-15. A structural predicate over the term and its declared extents is sufficient for the corner
rule's exactness.** Zero violations over twelve rows. Conditions: every leaf occurs at most once, and no
internal node has an ancestor multiplication whose sibling interval contains zero. `W = 4, F = 0,
signedness in {unsigned, signed}, overflow policy in {sat, wrap}, rounding = trunc, radix = 2, operations
in {add, sub, mul}, term shapes as enumerated in the probe, arity in {2, 3}, declarations = one-sided
[0, b] over every tuple, discharge check = per node, threads = 1, target features any`. `r2_output.txt`.
Sufficient and not necessary: it does not fire on `x - x` or `(x + y) - x`, both of which are exact.

**F111-16. The corner rule is exact on a mixed-operation term with independent leaves, and on a
multiplication under an addition.** `(x + y) - z` at signed `W = 4` is exact on 512 of 512 extents and
`(x * y) + z` at unsigned `W = 4` on 4096 of 4096. Same predicate as F111-15. `r2_output.txt`. This
**refutes `112` F112-6's "only"**.

**F111-17. The annihilation conservatism disappears when the multiplier is declared away from zero.** On
`(x + y) * z` the conservative count falls from 120 of 4096 to 0 of 3840 when `z`'s declared lower bound
is raised to 1, and to 0 at lower bounds 2 and 3. `W = 4, F = 0, signedness = unsigned, overflow policy =
sat, rounding = trunc, radix = 2, term = (x + y) * z, arity = 2, declarations = two-endpoint on the third
leaf and one-sided on the other two, discharge check = per node, threads = 1, target features any`.
`r2_output.txt`. This **bounds `112` F112-7** to one-sided declarations.

**F111-18. The two propagation rules differ in state by the term's leaf count.** The corner rule carries
two numbers on every term; an affine form carries one coefficient per distinct leaf plus one per
non-constant multiplication, which is 2 against 64 on a 64-element fold. `term shapes as enumerated in the
probe, threads any, target features any`. `r2_output.txt`. This is a count of coefficients and not a
measurement of anything: **the compile-time cost of carrying them is unpriced** and no bench ran.

**Unpriced.** Everything about cost, again. No harness ran in this reply either.

## 24. Coverage of the reply, bounded

**Read in full:** `113`, `112` including its findings list and its probe index.
**Read in part:** `112_probes/p2b` at its `row(...)` calls and its `main`, `112_probes/p3b_output.txt` in
full, `108:820-830` and `108:438-448` opened at source.

**Not read:** `112_probes/p1`, `p4`, `p5`, `p5b`, `p7`, `p7b`, `p7c`, `p8`, `p8b`, `p8c`, `p9`, `p9b` in
their sources. Where I cite one of those I am citing `112`'s account of it, and section 21's state-size
figures are computed from my own model of the two rules rather than from `112`'s implementation, which is
a difference worth naming: if `112`'s form carries fewer coefficients than one per leaf, my count is
wrong and the composition's crossover moves.

**Not re-run:** `112`'s thirteen-row domination result, its composite results, and every number I quote
from it that I did not reproduce. I reproduced none of them; R2 reproduces the shape of its p3b
conservatism figures on my own implementation, at the same values for the four rows we share.

**What I did not do.** I did not attack `112` section 8's composite results, which is now three members
in a row declining `109`'s chain result and two declining the composite side. I did not compile the
structural predicate, and section 19.2 says whose result its expressibility rests on. And I did not price
anything.

## 25. Probe index, reply additions

- `r1_moving_only_the_refinement.py`, `r1_output.txt`. The criterion `108` states, applied to a
  refinement for the first time, with an observable-axis control that fires.
- `r2_a_structural_predicate_for_where_the_corner_rule_is_exact.py`, `r2_output.txt`. The predicate, its
  zero violations over twelve rows, the two counterexamples to `112` F112-6, the annihilation case
  dissolving under a two-endpoint declaration, and the state-size comparison.
