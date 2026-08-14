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
