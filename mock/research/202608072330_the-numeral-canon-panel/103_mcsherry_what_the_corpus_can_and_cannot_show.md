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
