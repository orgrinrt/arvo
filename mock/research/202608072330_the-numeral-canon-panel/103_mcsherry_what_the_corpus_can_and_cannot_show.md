# 103. What the corpus can and cannot show

**Position:** eighth and last expert of the strategy-axis unit, before the consolidation. **Author:**
the `mcsherry` persona. **Standing:** nothing here settles anything; op decides.

**Assignment:** independently verify or refute `102`'s claim that every committed region in
`mock/benches/` is answer-equivalent, building my own instrument rather than reusing its one. Then, being
last, say what the unit established and what it did not.

## 0. The two gates

### 0.1 The canon gate: passed

Checked against `INTENTS.md` and `RULES.md`, both read in full before any probe.

The assigned work is a verification of an agent-authored measurement against committed artifacts. It
proposes nothing, builds nothing into the design, and reaches no intent. The one thing it could have
collided with is I13's rejection of a universal solution, since a corpus-wide claim of the form "every
region has property P" looks like exactly the universal this workspace rejects by premise. It is not: a
census over a finite committed set is an enumeration, not a generalisation, and it comes out predicated
on that set and nothing else. Every finding in section 9 carries its region.

I also checked the shape of my own dispatch against `never-ask-which-single-rule-governs.md`, because my
brief hands me three questions and one of them ("is a cost-only corpus a defect or the correct scope")
reads as a category-wide policy fork. My answer in section 6.3 refuses that framing and gives two regions
instead, which is what the rule demands.

### 0.2 The test gate: passed, with two observations, one correction of my own ordering, and no refusal

**I ran it late and that is a defect in my execution, not in the suite.** The brief puts both gates before
the assigned work. I read `INTENTS.md` and `RULES.md` first, which is the canon gate, then went into the
corpus, and ran the suite after p4. Nothing I found depends on the ordering, since the suite is green and
its bodies are real, but the ordering was wrong and I would rather say so than quietly present it as
having happened first.

`mock/crates/` is empty by design, so there is no arvo suite. The suite that exists is the thirteen bench
shared crates. Run individually, since `cargo test --workspace` from `mock/benches` reaches only the
driver crate and reports `0 passed`, which is a real trap for anyone who runs the obvious command and
reads the obvious answer:

```
bitpack-carrier-shared            9      bitpack-write-contend-shared    15
bitpack-contend-shared           12      quantiser-fadd-shared            1
bitpack-footprint-shared          6      quantiser-radix-shared           3
bitpack-plan-shared               5      satfold-shared                  11
bitpack-shared                    3      warm-clamp-shared                7
bitpack-wide-shared               6      warm-container-shared           15
                                         wide-rung-shared                30
                                         -------------------------------------
                                         123 across 13 crates, 0 failed
```

**123 across 13, which reproduces `98`, `100` and `102` exactly**, from a fourth count. `96`'s note holds:
`bitpack-write-contend-shared` hangs without `--test-threads=1`, and I ran the last seven that way.

I read the bodies rather than the names, for the surface I touch, which is the answer-discipline
machinery: every `validate_output`, every cross-arm assertion, and both quantiser families' tests in full.
Two observations, neither disqualifying:

**One redundant assertion.** `quantiser-radix-shared/src/lib.rs`, in
`an_odd_radix_has_no_representable_tie`:

```rust
assert_eq!(p % 2, 1, "3^{s} should be odd");
// 2 * lost == p has no integer solution for odd p.
assert!(p % 2 == 1);
```

The second assertion is the first one restated. It cannot fail if the first passed. Under the test gate's
redundancy clause one of them is doing the work; delete the second. One line, and I am naming it because
the gate says to name it, not because it inflates anything: no count anywhere cites this test.

**One name that outruns its body, defensibly.** The same test is named for the claim that an odd radix has
no representable tie. What it asserts is that `3^s` is odd and `10^s` is even. The step from "p is odd" to
"no exact tie exists" is a one-line arithmetic fact stated in the comment and never asserted. That is a
legitimate way to write it, since the premise is the only part a test can check and the inference is
trivial, but the name promises the conclusion and the body delivers the premise. Worth a reader knowing.

**And the thing the gate is actually for did not fire.** No tautological test, no setup that helps, no
sampled law standing in for a matrix, no assertion-free smoke test. The strongest tests in this repository
are the two in `quantiser-radix-shared`, and section 5 is about why.

**One correction to `102`'s test-gate observation, in its favour and against my own first reading.** `102`
reports that `bitpack-shared` has no cross-arm agreement assertion of either kind, and that its
correctness therefore rests on the harness mechanism `96` found broken. That is right about the tests and
it understates `bitpack-shared`'s position: its `Routine::validate_output`
(`bitpack-shared/src/lib.rs`, `fn validate_output`) compares `output.value` against a sum it computes
itself from `input.logical`, so every arm that passes it computes the ground-truth value. The assertion
`102` looked for is missing; the oracle is not. p5 classifies it ANSWER-PINNING on that ground.

## 1. The answer, before the working

**The claim is false, and its conclusion survives anyway for a reason `102` did not give.**

`102` section 3.1 states it in bold: "**Every arm set in the committed corpus is answer-equivalent. Every
number in it compares cost at a fixed answer.**"

Measured against the committed artifacts, over the right unit:

| | regions |
|---|---|
| arms pinned to one value by an exact-value oracle | 234 of 254 |
| arms pinned only to a property, so free to differ | 10 |
| arms with no oracle at all, and consenting to differ | 4 |
| single-arm regions where the question does not arise | 6 |

So **twenty committed regions are not answer-pinned**, and for **eight of them** the arms are measured to
differ or are two different algorithms by construction. `102`'s "every" is wrong by twenty regions, and
the four it is most wrong about are the ones its conclusion turns on.

Three separate measurements, each of which alone refutes the claim:

- **p2.** The two arms of `decimal-quantiser-radix-sweep` emit different denoted values on **97.12%,
  99.22%, 99.76% and 99.95%** of lanes at the four committed sizes, run through the family's own
  `build_input`, compared in exact rational arithmetic.
- **p3.** Controlling the input so both arms see the identical pair of exact integers, they still answer
  differently on **53.72%** of 200000 trials, with `binary32` strictly closer on 106167 and `decimal32`
  strictly closer on **zero**. That is an accuracy difference, not an input artefact.
- **p4.** The harness's own byte-exact cross-variant gate, run on the harness's own 100 validation seeds,
  **REFUSES that family at all four committed sizes, on 100 of 100 seeds**, while **ACCEPTING** the
  `quantiser-fadd` control at all six of its sizes on all 100. The instrument works and the committed CSVs
  record a run the stated acceptance criterion would not have permitted.

And the mechanical cause of the error, which is the useful part:

- **p6.** `102`'s census unit is `variants/*-shared/`
  (`102_probes/p1_the_corpus_compares_cost_at_a_fixed_answer.py:41`). Four committed regions belong to no
  shared crate: `fnv1a-vs-xxhash3` at four sizes, whose routine is `ByteRoutine<N, 8, true>` declared
  inline at `mock/benches/src/main.rs:229-232`. **`MAY_DIFFER = true`.** The corpus contains a region that
  explicitly declares its arms may differ, and `102` never sees it, because its enumeration cannot.
  Its file never mentions the hash bench at all: `grep -n 'fnv1a\|xxhash\|ByteRoutine' 102_*.md` returns
  nothing.
- **p5.** `102` treats the presence of `validate_output` as establishing answer-equivalence
  (`102_probes/p1...py:55`, `has_validate = "yes" if re.search(r"fn\s+validate_output", src)`). Eleven of
  the thirteen validators compare a value against a reference. **Two check a property**:
  `quantiser-fadd-shared` bounds `|s[i]|` and refuses NaN, `quantiser-radix-shared` refuses
  `mag[i] >= 2^24` and calls itself "radix-neutral" in the comment above the check. A property validator
  admits a family of different correct answers, which is exactly what it is for.
- **p8.** And the mechanism `102` leans on for the other twelve crates, "the harness default, which is
  byte-exact cross-variant comparison", **was not running for 175 of the 254 committed regions.** The
  driver gained its `harness::validate` call on 2026-08-08 in `9db33f8c`, and 175 regions were produced
  before it. The driver's own comment beside that call says so in its own words. All 20 non-pinned regions
  sit in that pre-wiring set, produced at one commit, `25f736b`.

**What survives, and it is most of what matters.** `102`'s conclusion is that the corpus cannot exhibit
I5, I7 and I9, because those intents range over arms that disagree. That conclusion is **correct**, and
the refutation above does not touch it, because the corpus's answer-differing regions still cannot exhibit
those intents. They record no accuracy coordinate, so the difference between their arms is not measured
anywhere. The corpus does not lack answer-differing arms. **It lacks a coordinate that ranks them**, which
is `98`'s finding and `101`'s, stated one rung lower than `102` put it.

And `102`'s own p5 constraint, which I was asked to test, turns out to be **sound and currently vacuous**,
for a reason nobody has stated. Section 7.

## 2. What my instrument did differently, since that is the whole point of a second read

`RULES.md` is explicit that agreement inherited by reading is not corroboration, and that the second read
has to derive before it reads. So the order I worked in, stated so it can be checked against the commit
log: I read `INTENTS.md`, `RULES.md`, `99` and my brief, then went to `mock/benches/` and built p1 through
p4 **before opening `102` or its probes**. p1 through p4 are committed at
`9d35fa13`, `100c62aa`, `5e48163c`, `e0191571`, and the commit that first reads `102`'s script is after
all four. That is the independence claim and it is checkable rather than asserted.

Four differences, and each of them is where a finding came from.

### 2.1 I asked the committed data first, and it answers

`102`'s instrument reads variant **source**. Mine started with the **CSV corpus**, because the claim is
about the corpus and because the schema has a column for exactly this question. `p1` is one pass over all
254 files and 104080 rows, reporting per-column cardinality.

```
digest         CARRIES NOTHING (single value '0' in all 104080 rows)
score          CARRIES NOTHING (single value '' in all 104080 rows)
input_tag      CARRIES NOTHING (single value '' in all 104080 rows)
instructions   distinct=     1 CONSTANT  top=[('0', 104080)]
cycles         distinct=     1 CONSTANT  top=[('0', 104080)]
setup_ns       distinct=     1 CONSTANT  top=[('0.0', 104080)]
first_ns       distinct=     1 CONSTANT  top=[('0.0', 104080)]
```

**The committed corpus records no answer.** There is a `digest` slot and it is zero in every row, so
answer-equivalence is not a property the data attests in either direction. That is not a refutation on its
own and I did not treat it as one; what it establishes is that any claim about the corpus's answers is a
claim about the **code that produced it**, and must be argued there. Both `102` and I then argue it there,
which is correct, and it is worth having the reason stated rather than assumed.

This is a re-derivation rather than a discovery, and I want that on the record: `mock/benches/src/main.rs`
already says it, in the comment beside the validation call, "The `digest` and `score` columns are zero for
every plain `timed!` variant, so they catch nothing either". `101`'s Q48 entry reports the same eight dead
columns from a third direction. Three independent instances of one fact, which is the bar `RULES.md` asks
for, and I found mine before reading either of the others.

### 2.2 I classified what each validator asserts, not whether one exists

The load-bearing difference. `102`'s classifier is a presence test:

```python
has_validate = "yes" if re.search(r"fn\s+validate_output", src) else "no"
```

Presence is not pinning. p5 opens every `validate_output` body in the corpus and separates two shapes: a
comparison of an output field against a reference the validator computed from the input, versus a bound or
a shape test on the output alone. The evidence fragment for each classification is printed beside it so a
reader checks the classifier rather than trusting it:

```
bitpack-carrier-shared         ANSWER-PINNING           'output.value != e'
satfold-shared                 ANSWER-PINNING           'if rep.value != output.value'
wide-rung-shared               ANSWER-PINNING           'd) != output.limbs'
quantiser-fadd-shared          PROPERTY-PINNING ONLY    'output.s[i].is_nan()'
quantiser-radix-shared         PROPERTY-PINNING ONLY    'output.mag[i] >='
```

Eleven pin a value. Two pin a property. I cross-checked all thirteen against the hand dump of every
validator body before trusting the regex, and they agree; the probe is a spike and I treated it as one.

### 2.3 I used the region as the unit, because that is what the claim quantifies over

The claim is "every committed **region**". A region is one committed CSV: one bench name at one size, with
the arms that ran against each other. `102` enumerates shared crates, which is a different set and not a
covering one.

p6 joins the CSVs, `bench.toml` and the driver's own `routine_for_n` table, and asks per region which
bridge it uses and what that bridge pins. That is how `fnv1a-vs-xxhash3` appears: it has no shared crate,
its bridge is declared inline, and its `MAY_DIFFER` const is `true`.

p6 also caught **my own** first classification being wrong, in `102`'s favour, and I am recording that
rather than quietly fixing it. My first version let the consent flag override the oracle, which labelled
all 28 `satfold` regions as consenting-to-differ and produced a headline of 48 non-pinned regions. That is
wrong: `satfold-shared` both declares `outputs_may_differ = true` **and** pins every arm to an independent
`u64` oracle, so it is answer-equivalent for the second reason regardless of the first, exactly as `102`
says and exactly as its own doc comment claims. The two questions are independent and folding one into the
other is what produced the error. The committed probe reports them in separate columns and says so in its
header. The corrected number is 20, not 48.

### 2.4 I ran the arms, and then I ran the harness's gate over them

`102`'s p1 is a static reading throughout. Mine executes. p2 and p3 call the two committed radix kernels
directly and diff their outputs in exact rational arithmetic. p4 goes further and reproduces the harness's
own acceptance criterion, on the harness's own seed sequence, and applies it to a committed region.

That last one is the difference between "I think these arms differ" and "the mechanism cited as
guaranteeing they agree refuses them". A control arm is what makes it a measurement rather than an
assertion, and p4 carries one: `quantiser-fadd`, which takes the identical two bridge defaults and passes
at every size on every seed.

## 3. The refutation, in the order I found it

### 3.1 The corpus has an arithmetic family whose two arms are two different number formats

`quantiser-radix-shared`'s own header states the position plainly, and it is worth quoting because it
means the family is not hiding anything:

> The confound that remains, stated rather than hidden: the two real formats have different precisions
> (binary32's twenty-four binary digits against decimal32's seven decimal digits) because that is what the
> standards say, so the ratio is "decimal32 against binary32", not "radix ten against radix two at fixed
> precision".

Two formats with different precisions cannot round one exact sum to one answer except by coincidence. The
`validate_output` for the family reflects that: it is radix-neutral by design, and its comment says so.

**p2**, on the family's own committed input at all four committed sizes, eight seeds each, 2048 lanes per
size:

```
SPREAD = 0    denoted value DIFFERENT : 1989 of 2048  (97.12%)
SPREAD = 2    denoted value DIFFERENT : 2032 of 2048  (99.22%)
SPREAD = 8    denoted value DIFFERENT : 2043 of 2048  (99.76%)
SPREAD = 20   denoted value DIFFERENT : 2047 of 2048  (99.95%)
undecidable (overflow) : 0 at every size
```

I report representation identity separately and discount it, because two radices spelling one quantity
with different digits proves nothing. The number above is the **denoted value**, `mag * R^exp` compared as
an exact rational.

### 3.2 The deflationary reading, and why it does not survive

There is an honest objection to p2 and I went looking for it before anyone else could. The family draws
its exponent as a **grid step**, not an absolute magnitude, which its header also states: "a pair `SPREAD`
apart is `SPREAD` grid steps apart in whichever radix reads it". So the same triple denotes a different
real under each radix, and the arms are not being handed the same value. On that reading they disagree
because their inputs disagree, which is a weaker and less interesting fact.

The sample rows make the objection visible rather than hiding it: at lane 0 the two arms return
`49545344` and `3870730000000`, which is a factor of 78000 and obviously not a rounding difference.

**p3 removes the confound entirely.** Feed both arms operands at `exp = 0`, with `mag` drawn from the
family's own `[10^6, 10^7)` band. At `exp = 0` the triple denotes the integer `mag` under **either**
radix, so both arms receive the identical pair of exact integers and the exact sum they must round is the
same integer. Everything that differs after that is rounding.

```
trials                        : 200000
answers IDENTICAL             : 92556  (46.2780%)
answers DIFFERENT             : 107444 (53.7220%)

  binary32 exact              : 193747 (96.8735%)
  decimal32 exact             : 91329  (45.6645%)
  binary32 strictly closer    : 106167
  decimal32 strictly closer   : 0
  equal error                 : 93833
```

with worked rows, so the reader can check one by hand:

```
5315940 + 8535628 = 13851568 exactly;  binary32 -> (13851568, 0),  decimal32 -> (1385157, 1)
```

`13851568` against `13851570`. One arm is exact and the other is two off, on an input both formats
represent exactly. **That is an accuracy difference between two committed arms of one committed region**,
and it is the shape I5 and I7 are about.

The predicate on p3 is narrower than the predicate on p2 and I am not going to blur them. p2 holds on the
family's **committed** input distribution. p3 holds on a **controlled** band that the committed
distribution contains as its `exp = 0` slice but is not equal to. Both are real; only p2 is a statement
about what the corpus ran.

### 3.3 And then the harness's own gate refuses the family

The strongest form available, because it takes nothing on my judgement. From the pinned harness at
`bce17f6`, which is what `mock/benches/Cargo.lock` pins:

```rust
pub(crate) fn validation_plan(outputs_may_differ: bool, approx_eps: Option<f64>) -> ValidationPlan {
    ValidationPlan {
        per_variant:   true,
        cross_variant: if outputs_may_differ {
            None
        } else if let Some(eps) = approx_eps {
            Some(CrossVariant::Approx(eps))
        } else {
            Some(CrossVariant::ByteExact)
        },
    }
}
```

`quantiser-radix-shared` declares neither flag, so it takes both defaults and its plan is byte-exact
cross-variant comparison. `CrossVariant::ByteExact` compares each variant's raw output buffer against a
baseline's and refuses on any mismatch. Its two arms are recorded together, as the two arms of one run, in
all four committed CSVs.

**p4** runs that comparison, on `Rng::new(0xCAFE_BABE_DEAD_BEEF)` iterated 100 times, which is
`VALIDATION_ROOT_SEED` and `DEFAULT_VALIDATION_SEEDS`:

```
SUBJECT: quantiser-radix, arms quantiser-radix2 and quantiser-radix10
  SPREAD=0   seeds=100  byte-mismatched seeds=100   gate: REFUSE
  SPREAD=2   seeds=100  byte-mismatched seeds=100   gate: REFUSE
  SPREAD=8   seeds=100  byte-mismatched seeds=100   gate: REFUSE
  SPREAD=20  seeds=100  byte-mismatched seeds=100   gate: REFUSE

CONTROL: quantiser-fadd, arms quantiser-fadd-hardware and quantiser-fadd-software
  PCT=0..100 seeds=100  byte-mismatched seeds=0     gate: ACCEPT   (all six sizes)
```

400 of 400 seed-size pairs refused on the subject; 600 of 600 accepted on the control. There is no reading
of this on which the committed CSVs for that region passed the criterion `102` cites.

### 3.4 The second refuting region, which belongs to no shared crate and says so out loud

`fnv1a-vs-xxhash3`, four committed CSVs, each holding both arms:

```
fnv1a-vs-xxhash3_n64.csv   arms: fnv1a xxhash3
fnv1a-vs-xxhash3_n256.csv  arms: fnv1a xxhash3
fnv1a-vs-xxhash3_n1024.csv arms: fnv1a xxhash3
fnv1a-vs-xxhash3_n4096.csv arms: fnv1a xxhash3
```

Its routine is registered at `mock/benches/src/main.rs:229-232` as `ByteRoutine<N, 8, true>`. The third
const parameter is `MAY_DIFFER`, and it is `true`. So the region **declares that its arms may differ**,
the harness skips cross-variant comparison for it entirely, and `ByteRoutine` has no `validate_output` of
its own. Nothing anywhere in this repository requires `fnv1a` and `xxhash3` to compute the same value, and
nothing checks whether they do.

I did not run these two arms and I am not going to claim I did. Both variant crates import `arvo::Hot`,
`arvo::strategy::Unsigned` and `arvo_hash::{ConstHash, Fnv1a}` from the deleted crate tree, so they do not
build, and the algorithms are recoverable from git only by reattaching a tier the mutation order requires
to stay detached. What the region establishes without running anything is the weaker and sufficient claim:
it is a committed region whose arms are two different hash functions, over which the corpus asserts
nothing. FNV-1a and XXH3 agreeing on every input would be the extraordinary claim, not the ordinary one.

`102` never mentions this region. That is not carelessness; it is the census unit. A sweep over
`variants/*-shared/` cannot reach a routine declared in the driver.

### 3.5 And the mechanism was not running for most of the corpus

This is the finding I did not expect and it is the deepest one, because it does not depend on any family
being unusual.

`102`'s argument for the twelve non-`satfold` crates is one sentence: "Twelve crates take the harness
default, which is byte-exact cross-variant comparison." That is a claim about a mechanism running.
`mock/benches/src/main.rs` states, in the comment beside the call it added, that it does not run by
itself:

> `harness::run` does NOT do this: `run_orchestrator` never calls `validation::validate`, so without this
> call a variant computing a different answer from its peers is timed and reported like any other.
> Demonstrated: a one-character off-by-one in a loader's tail assembly produced 400 rows of
> ordinary-looking numbers and exit 0. The `digest` and `score` columns are zero for every plain `timed!`
> variant, so they catch nothing either, which leaves the variant crate's own unit tests as the only
> fidelity check in the system.

So the gate exists only where the driver calls it, and the driver gained that call on **2026-08-08** in
`9db33f8c`, "bench: make the driver validate its arms, and re-run every wide-rung section". Every
`*.meta.json` records the `git_commit` that produced it, so this is countable with no inference at all.

**p8**, verifying rather than assuming that the call is present at that commit and absent at its parent:

```
  present at that commit      : True
  present at its parent       : False

REGIONS BY WHETHER THE GATE EXISTED WHEN THEY RAN
  produced BEFORE the wiring :  175 of 254
  produced AFTER  the wiring :   79 of 254

SPLIT BY WHETHER THE REGION'S OWN ROUTINE PINS AN ANSWER
  before wiring, answer-pinning routine     :  155
  before wiring, NOT answer-pinning         :   20
  after  wiring, answer-pinning routine     :   79
  after  wiring, NOT answer-pinning         :    0
```

**175 of 254 committed regions were produced by a driver that never called cross-variant validation.** For
those 175, the mechanism `102` cites is not evidence in either direction. And the split is not random:
every one of the 20 non-pinned regions is in the pre-wiring set, all produced at one commit, `25f736b`;
and every region produced after the wiring has an answer-pinning routine, which is what you would expect
if the gate has been doing its job since it was wired.

**This does not make the 155 wrong.** They belong to families whose own tests assert cross-arm agreement
against an independent oracle, and all 123 of those tests pass. The point is narrower and it is about
provenance rather than truth: **the artifact the belief rests on is the family's unit tests, not the
harness mechanism.** The driver's comment says exactly that, and it is the more honest citation for
anything the consolidation writes about answer-equivalence.

## 4. What survives of `102`, stated as support rather than as concession

`RULES.md` says support counts as much as attack and that independent derivation before reading beats
agreement after. Four things of `102`'s I derived or confirmed independently, and I want them on the
record at that strength.

**Its conclusion is right.** The corpus cannot exhibit I5, I7 or I9. I reached that from the other
direction: the corpus **does** contain answer-differing arms, it contains one region with a strict
accuracy ordering between them, and it records **nothing about that ordering anywhere**. p1 finds `score`
empty in all 104080 rows; p7 has to compute the error coordinate from scratch because no committed column
carries it. So the conclusion holds and its reason changes: the barrier is not the arm sets, it is the
absent coordinate. That relocates the finding one rung down, onto `98`'s and `101`'s, rather than upstream
of them.

**`satfold` is answer-equivalent and its doc comment is right about why.** I labelled it wrong first and
the correction went my way, not `102`'s. Consenting to differ and pinning to an oracle are independent,
and pinning is the stronger of the two.

**The `max_relative_error` observation is right and I confirmed the count.** `102` reports that the
harness has a third cross-variant regime, `CrossVariant::Approx(eps)`, that no arvo variant sets. p5 reads
every shared crate's `fn max_relative_error` and finds `None (default)` in thirteen of thirteen. That
regime is exactly the shape an accuracy-differing family needs and it has never been used. Section 6.2 is
about what I would do with it.

**And the measured-versus-computed split is a real distinction and worth keeping**, independently of
whether its hazard fires. Section 7.

## 5. The corpus already contains the mechanism `102` says it lacks, in one family, working

This is the constructive half and it is the part I would most want a consolidation to carry.

`102`'s pair proposal needs a way to hold arms that produce different answers while still knowing each one
is correct. It treats that as something to be designed. **One committed family already does it**, and the
shape is worth naming because it was arrived at by someone solving the concrete problem rather than by
anyone theorising about it.

`quantiser-radix-shared` has **two oracles, one per arm**, and no oracle across arms:

- `radix_two_instantiation_matches_the_silicon` checks the radix-two arm against native `f32`
  bit-for-bit, over the family's own input distribution at all four committed sizes, 32 seeds each:
  `4 * 32 * 256 = 32768` checks, asserted as `assert_eq!(checked, 4 * 32 * N as u64)` so a silently
  shortened sweep fails rather than passes.
- `radix_ten_delivers_the_nearest_grid_point_ties_to_even` checks the radix-ten arm against the
  **definition** rather than against the other arm, because no silicon exists to check it against on any
  pinned target: the delivered significand must be the nearest decimal32 grid point, both neighbours
  tested in exact integer arithmetic, with ties-to-even checked separately. Same 32768 checks, same
  assertion on the count. Its own comment says why this is an independent oracle rather than a second call
  to the same rounding code.

That is the general shape, and it generalises past this family:

> **A region whose arms may produce different answers is validated arm by arm, each against its own
> declared semantics, rather than arm against arm.** Cross-arm agreement is then a consequence where the
> semantics coincide, and its absence is not a defect where they do not.

The harness already has the switch for it (`outputs_may_differ`), already has the intermediate regime for
the bounded-disagreement case (`max_relative_error`), and one family already writes the per-arm oracles.
What is missing is not a mechanism. It is a **coordinate** recording how far each arm's answer is from its
reference, and a `Routine::score_output` hook that `101` reports zero of 94 variant crates implement.

So the honest statement of the gap is much smaller than "the corpus is structurally unable to exhibit
those intents". It is: **the corpus can hold answer-differing arms, validates them correctly where it
does, and does not write down the difference.**

## 6. The three judgements my brief asked for

### 6.1 Is answer-equivalence a property of the corpus, the harness, or the convention

**All three, in different regions, and the split is exactly what makes the claim slippery.**

- **Of the harness**, for the 79 regions produced after `9db33f8c`, where the driver calls `validate` and
  the byte-exact comparison genuinely runs. There it is enforced.
- **Of the convention**, for the 155 pre-wiring regions with answer-pinning routines. The oracle is in the
  code and the tests exercise it, so the arms agree because the people who wrote the families made them
  agree and checked it. The harness had nothing to do with it during those runs.
- **Of neither**, for the 20 remaining. Nothing required it and nothing checked it.

**What it would take to build a family that is not answer-equivalent** is the more useful half of the
question, and the answer is that almost nothing is missing, which I did not expect before p5:

1. Declare `outputs_may_differ = true`, or `max_relative_error = Some(eps)` for the bounded case. One
   method each, both already in `Routine`.
2. Give each arm its own oracle, the way `quantiser-radix-shared` already does. No new mechanism.
3. Implement `score_output`, which exists in the trait and which zero of 94 variants implement, so the
   difference between the arms lands in a committed column instead of nowhere.

Step 3 is the only one that is real work, and it is the one nobody has done. Steps 1 and 2 are a
declaration and a test.

### 6.2 Is a cost-only corpus a defect or the correct scope

**Neither, and the question as posed asks for a single verdict over a category, which is the shape
`never-ask-which-single-rule-governs.md` refuses.** There are two regions and they want different answers.

**Where the arms compute one value, a cost-only corpus is exactly the right scope and is not a defect.**
`warm-container-shared`'s own doc comment gives the reason better than I would: without the agreement
requirement the fast arm is fast because it is doing less. Pinning the answer is what makes the timing
mean anything, and 234 regions do it. Adding a fidelity column there would measure a constant.

**Where the arms compute different values, a cost-only corpus is not a defect in the corpus either. It is
an incomplete instrument.** The bench is correct about what it measures; it simply does not measure the
other axis, and the other axis is the one op's I5 and I7 are about. A committed CSV row that records
`algo_ns` for `quantiser-radix2` and `quantiser-radix10` is a true and useful comparison of the two
formats' cost, and it says nothing about the thing that separates them.

So the composed answer, which is what I13 asks for rather than a ruling:

| region | verdict |
|---|---|
| arms answer-equivalent | cost-only is correct and complete; a fidelity column would be constant |
| arms answer-differing, no fidelity column | cost-only is correct and incomplete; the missing column is `score_output` |
| arms answer-differing, no per-arm oracle | not a scope question at all; the region is unvalidated |

The third row is the only one that is a defect, and it has four instances: `fnv1a-vs-xxhash3` at four
sizes.

**And the intent-carrying comparison does not belong to a different instrument.** That was my first guess
and p5 killed it. The harness already has the consent switch, the approximate-comparison regime and the
scoring hook; `quantiser-radix-shared` already writes per-arm oracles inside the ordinary bench crate
shape. A separate instrument would be a second index over the same data, which is a rebuild cost paid
forever for a distinction the existing one already expresses.

### 6.3 `102`'s p5 constraint: sound, and currently vacuous

Section 7, because it needed its own measurement.
