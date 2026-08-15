# 141. The strategy set, attacked

Two members derived this topic cold and blind to each other. My job is to attack what they produced, and
the dispatch names three targets in order: the thing they converged on blind, the firewall `139`
nominated against itself, and the two counting claims.

I will report the outcome up front, because the shape of it matters more than any single number. **The
blind convergence is real and its scope is narrower than either file states.** Both measured where a
value is put; the concern they were measuring is minimisation, which does not stop there, and one step
further in it has policy content in exactly one cell. **The firewall's diagnosis is right and its repair
is wrong**, and the repair's only positive case turns out to be an artifact of an unstated rounding
choice rather than a fact about fusion. **The two counting claims are one claim**, stated over the wrong
object by both, and one of them is false as written.

I reproduced every number I attack on an instrument I wrote before saying anything about it, and three of
my own instruments refuted me first. That sequence is most of this file, because the dead ends are where
the mechanisms were found.

---

## Gates

### Canon gate: passed

Checked against `INTENTS.md` entry by entry. The assigned work is licensed rather than in conflict.

`INTENTS.md:51-61` demotes I1 to OPEN on op's own word, so deriving the strategy set is exactly what the
catalogue calls for. `INTENTS.md:363-383` (I17) says the count "is besides the point of the intent",
so a derivation that declines to fix a count answers the question rather than dodging it. Nothing in
this file argues for dropping or downgrading the storage-minimising concern, which is what I17 forbids;
my section 2.2 argues the opposite, that the concern reaches further than either file measured.

The one entry my central argument leans on is **I13, `INTENTS.md:214-235`, which is the single RATIFIED
entry in the catalogue.** My replacement for the firewall is I13's mechanism applied to fusion, and the
reason I am confident about it is that it is not mine: op ratified it, in the words recorded there.
`INTENTS.md:317-331` (I16) also bears on it, and it cuts against a mechanism `139` proposes rather than
against my reading, which I state at the end of section 4 rather than folding it in.

### Test gate: run, and it passes with one undocumented requirement

Counts produced by a command, per `RULES.md:124`.

```
$ cd mock/benches && grep -rl '#\[test\]' variants/ | sed 's|/src/.*||;s|/tests/.*||' | sort -u | wc -l
13
$ cd mock/benches && grep -rn '#\[test\]' --include='*.rs' variants/ | wc -l
124
```

Thirteen crates, 124 grep hits, 123 tests. The extra hit is `variants/bitpack-write-contend-shared/src/stress.rs:68`,
a doc comment whose prose contains the token. The brief warned about this and the brief is right.

Twelve crates run green under `cargo test --manifest-path <crate>/Cargo.toml`, 108 tests, zero failed,
zero ignored. `wide-rung-shared` takes 286.53s on this host, which is why a workspace-wide invocation
reads as a hang.

The thirteenth needs `-- --test-threads=1`:

```
$ cargo test --manifest-path variants/bitpack-write-contend-shared/Cargo.toml -- --test-threads=1
test result: ok. 15 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 46.65s
```

**So all 123 pass, and `139`'s livelock diagnosis reproduces independently on my machine.** Its
explanation is at `139:703-717`: the worker pool at `pool.rs:87-160` is process-global with a single
coordinator, `cargo test` runs the three stress tests concurrently in one process, and a second
coordinator's `done.store(0)` resets a counter the first is spinning on. I did not re-derive that; I
confirmed the flag fixes it and I carry the diagnosis forward as `139`'s.

My time was 46.65s against `139`'s 7.97s, which is machine load rather than a disagreement, and I state
both rather than the one that flatters my run.

**One instrument failure of my own, in the gate itself.** My first attempt to bound the run used
`timeout 300 cargo test ...`, which produced no output and which I initially read as the crate hanging
again. `timeout` does not exist on this host, the shell reported `command not found` and exit 127, and
my grep for `^test result` found nothing and returned quietly. A wrapper that fails to start looks
exactly like the thing it was wrapping failing to finish. I mention it because the same shape has cost
this panel real time twice now, and because I nearly wrote up a hang that was my own missing binary.

**Test quality.** I read bodies in `satfold-shared`, `warm-container-shared` and
`bitpack-write-contend-shared`. `satfold-shared/src/lib.rs:1116-1201` runs four mutation controls that
assert the harness catches a defect rather than that a kernel agrees with itself, and `:1271` asserts
that its own false const gate is genuinely false. `warm-container-shared/src/lib.rs:1425` declares itself
"an ad-hoc quick spike with no substance" in its own doc comment and records that its first version
reported zero nanoseconds because the sink was provably zero. I found no tautology, no assertion of a
value against itself, and no arm compared against itself. Both cold derivations reached the same verdict
independently and I make that three; I have nothing to insult here and I say so plainly, because the
gate exists to catch the opposite.

---

## 1. The dispatch brief, tested first

`RULES.md` and `panels-argue-the-intent-not-the-wording.md` both put breaking the brief before the
assigned work, so I start there. One of its claims does not survive.

The brief says of the blind convergence:

> Note it also agrees with op's own sentence at `INTENTS.md:131-133` that Cold can use the same paths Hot
> uses, which is corroboration from a third direction and raises rather than lowers the stakes if it is
> wrong.

Op's sentence, in full, at `INTENTS.md:129-133`:

> Cold does not *have to* drop efficiency wins elsewhere. It can use the same paths Hot uses, not because
> it needs to by intent, but nothing in its intent would fight it. But if the path fights the intent,
> then it's not for Cold.

**That is a statement about paths, and the convergence is a statement about answers.** The two are the
same only if every path Hot may take is answer-preserving, and `INTENTS.md:110-117` (I5) says the
opposite in op's own words: "Hot *can* sacrifice soundness, that is its explicit purpose". A path that
sacrifices soundness changes answers. So op's sentence says the storage-minimising concern may adopt the
speed-first concern's cost choices, some of which are answer-changing by op's own statement of them, and
it does not say that minimisation is answer-invisible.

The two readings are compatible and they are not the same claim. Reading the sentence as corroboration
takes a claim about which lowerings a concern may reach for and converts it into a claim about whether a
concern has semantic content, which is the second act `quote-verbatim-then-name-the-intent-separately.md`
requires and which was skipped.

**So the convergence stands at two instances rather than three**, and the third direction the brief
offers is not a direction on that axis. This matters for exactly the reason the brief gives: it raises
the stakes, and an inflated rung is how a two-instance result gets defended as though it were three.

I state this as a finding against the brief rather than against either member. Neither file makes the
claim; `140:263-279` cites the same sentence and uses it correctly, for the narrower point that the
concerns are not mutually exclusive, which is what the sentence actually says.

---

## 2. The blind convergence, reproduced and then narrowed

The convergence is that the storage-minimising concern is a weighting with zero policy content. `139`'s
`p6` reports zero disagreements between packed and padded across sixty configurations
(`139:230-233`); `140`'s `p3` reports ninety configurations collapsing to twenty-four answer functions
with a lossy control doubling the count (`140:564-575`). Neither read the other.

### 2.1 I reproduce it, and I am the third instance for the column

`141_probes/p4_the_concern_reaches_the_accumulator.rs`, output at `p4_out.txt`. Written in Rust over
raw integers, sharing no code with either.

The instrument holds the contested dimension fixed at the conservative reading, so nothing here depends
on it: the overflow limit is read at the **declared** width always, which is what `140`'s phase-two
section C argues the design must require (`140:862-868`). Under that reading:

| shape | lossless storage x3, accumulator pinned | assignment alone | lossy control |
|---|---|---|---|
| W=4 F=0 unsigned | 12 configs, 2 classes | 2 | 4, control fires |
| W=4 F=0 signed | 12 configs, 2 classes | 2 | 4, control fires |
| W=4 F=1 signed | 12 configs, 4 classes | 4 | 8, control fires |
| W=4 F=2 signed | 12 configs, 4 classes | 4 | 8, control fires |

**T1 confirmed at every shape: the three lossless containers add zero classes.** The lossy control fires
at every shape, so the sweep can see a container. A duplicate configuration reached by a different
construction merges at every shape, so the comparator can merge.

**That makes three independent instances of the column result**, arrived at from three models: `139`'s
signed round trip through a bitstream at arbitrary offsets, `140`'s unsigned partition over rungs, and
mine. The three also span the signedness dimension between them, which no single one of them does.

### 2.2 And then the scope, which is where it stops

Both files measured **where a value is put**. Neither measured **how wide the arithmetic is while the
value is being computed.**

I6 is the concern's statement and it is `INTENTS.md:119-127`. Op's words are that the concern
"aggressively minimises and bitpacks" and that it "should remain small for memory or disk storage". An
accumulator in a column store is an array too, and nothing in that sentence stops at the storage
boundary. So the question the two files did not ask is whether the concern is still answer-invisible one
step in.

Same instrument, adding an accumulator dimension over `{declared width, declared+2, wide}`:

| shape | storage only | storage and accumulator | verdict |
|---|---|---|---|
| W=4 F=0 unsigned | 2 classes | 36 configs, 2 classes | accumulator invisible |
| W=4 F=0 signed | 2 classes | 36 configs, **4 classes** | accumulator visible |
| W=4 F=1 unsigned | 2 classes | 36 configs, 2 classes | accumulator invisible |
| W=4 F=1 signed | 4 classes | 36 configs, **8 classes** | accumulator visible |
| W=4 F=2 unsigned | 2 classes | 36 configs, 2 classes | accumulator invisible |
| W=4 F=2 signed | 4 classes | 36 configs, **6 classes** | accumulator visible |

**T3 confirmed for signed and refuted for unsigned**, which is half of what I predicted. Splitting by
overflow position gives the exact predicate:

| shape | wrapping, acc varied vs pinned | saturating, acc varied vs pinned |
|---|---|---|
| W=4 F=0 signed | 1 vs 1, invisible | **3 vs 1, visible** |
| W=4 F=1 signed | 2 vs 2, invisible | **6 vs 2, visible** |
| W=4 F=2 signed | 2 vs 2, invisible | **4 vs 2, visible** |
| every unsigned shape | 1 vs 1, invisible | 1 vs 1, invisible |

**T4 was half wrong and the refutation is the useful half.** I predicted the accumulator would be
invisible under wrapping and visible under saturating. Wrapping is confirmed and unsigned saturating
refutes me: it is invisible there too. The mechanism is the one `139` found for a different question,
that a one-sided clamp of a monotone operation is a congruence and a two-sided one is not.

So the finding is a single cell:

**The minimising concern is answer-invisible at the column, and answer-invisible at the accumulator
except at `signedness = signed, overflow = saturating`, where narrowing the accumulator is a policy
change.**

T5 confirmed as well: `add` alone cannot see the accumulator at any shape, so a witness set of `{add}`
reports the whole effect as zero. That is `140`'s witness-set dependence arriving from a second
direction, on a second instrument, and I count it as an independent instance of that claim.

### 2.3 What this does to the convergence, and what it does not

It does not refute it. `139:244-249` says packing is answer-invisible and that the concern is therefore
a weighting; `140:673-677` says a lossless container contributes zero classes. Both are true of what they
measured and I have now measured the same thing a third time.

What it refutes is the **scope** both stated. `139:244` generalises from packing to "the
storage-minimising concern"; `140:577-579` says the concern "composes with every assignment rather than
competing with them". Neither is true at signed saturating once the concern reaches an accumulator, and
neither predicate carries a dimension that would have narrowed it, because neither model had one.

This is what the dispatch asked me to distinguish, and the answer is neither of the two it offered. It
is not a real result and it is not one wrong model held twice. It is **one correctly measured result
whose object is narrower than the concern it is being stated about**, and the two models are identically
scoped, which is why agreeing did not catch it.

---

## 3. The firewall

`139` nominated its section 4 for attack and said why (`139:661-664`): the measurements around it are
solid, the claim that the pair must satisfy it is an argument, "and it is the kind that sounds obviously
right and might be wrong in a way I cannot see from inside it."

The firewall itself, `139:266-268`:

> The policy component determines the answer. The weighting selects only among computations that conform
> to the policy. Every difference in an answer traces to the policy, and nothing else may move one.

**I agree with the firewall and I attack the repair.** The proposition that a cost model must not be
able to move an answer is the same proposition as I15's compile-time-only stance one layer up, and I
found nothing against it. What I attack is everything after `139:338`, where the firewall is loosened to
buy back a win, and the loosening turns out to buy nothing while giving up the property.

### 3.1 First, reproduce the table

`141_probes/p1_fusion_reproduced.rs`, output at `p1_out.txt`. I wrote the model from `139`'s prose
description of the two arms without opening `139_probes/p2_firewall.rs`, and I swept both spellings of
"truncate", because Rust spells two different things that way and they differ exactly on the negative
half where the signed rows live.

My table, under either spelling:

| cell | F=0 | F=1 | F=2 | F=3 | F=4 | F=5 |
|---|---|---|---|---|---|---|
| unsigned, wrapping | 0.00% | 0.00% | 0.00% | 0.00% | 0.00% | 0.00% |
| unsigned, saturating | 0.00% | 0.00% | 0.00% | 0.00% | 0.00% | 0.00% |
| signed, wrapping | 0.00% | 0.00% | 0.00% | 0.00% | 0.00% | 0.00% |
| signed, saturating (floor) | 42.14% | 37.50% | 30.22% | 19.77% | 7.53% | 0.01% |

`139`'s, at `139:293-302`:

| cell | F=0 | F=1 | F=2 | F=3 | F=4 | F=5 |
|---|---|---|---|---|---|---|
| signed, wrapping | 0.00% | 1.64% | 5.54% | 12.34% | 22.22% | 33.40% |
| signed, saturating | 42.14% | 39.10% | 35.67% | 31.96% | 29.52% | 33.41% |

**We agree exactly at F=0 and disagree at every F above it.** The unsigned rows agree everywhere. That is
a model disagreement in a specific place, and finding out where is the rest of this section.

The reach controls hold: at signed saturating F=0 the product leaves the declared range at 224448 of
262144 triples and the wrong-clamp control is caught at 169380, so the cell is not vacuous. At F=5 it
degenerates to 64 out-of-range triples and 32 differences, which is why my 0.01% there is nearly
toothless and I say so rather than letting a uniform-looking row imply uniform strength.

### 3.2 First hypothesis, refuted by its own control

`141_probes/p1b_which_model_produces_139s_table.rs`. A fixed-point multiply has to form a wide product,
discard F fraction bits, and bring the result into range, and the order of the last two is stated
nowhere. I hypothesised `139` reduces before shifting where I reduce after.

Model B gives, for signed wrapping: 0.00 / 52.54 / 74.80 / 82.71 / 85.55 / 85.62. Nothing like 1.64.
Worse, it breaks the unsigned rows, taking them to 46.68% and above, where both files agree on zero.

**R1, R2 and R3 all refuted.** The R4 control fires, meaning models A and B really are distinguishable,
so the probe was capable of confirming the hypothesis and did not. Dead route, closed by measurement,
and the record of it is in `p1b_out.txt`.

### 3.3 The theorem that says the reduction cannot be the cause

`141_probes/p2_the_slack_buys_nothing.rs`. Rather than keep guessing at another model, prove what fusion
alone can do.

**Absorption.** Let `R` be reduction modulo `2^W`, in either the unsigned or the two's-complement signed
reading. For all integers `x` and `c`, `R(R(x) + c) = R(x + c)`. Proof: `R(x) = x - k*2^W` for some
integer `k`, so `R(x) + c` is congruent to `x + c` modulo `2^W`, and `R` is a function of the residue
class alone.

**Corollary.** If a fused and a stepwise arm feed the same value to the final add, and the overflow
policy is wrapping, they agree at every F, every width and both signednesses. Fusion under wrapping is
answer-preserving by construction, and the required slack is zero.

Checked exhaustively at W in {4,5,6}, both signednesses, zero mismatches. The control substitutes
saturation for wrapping and fires everywhere, 848 to 16128 mismatches, so the check can detect a
non-homomorphism.

**Consequently a nonzero fusion-difference rate under wrapping is evidence that the two arms differ in
something other than where the reduction sits.** That is model-independent: it does not require me to
know `139`'s model, only to observe that its signed wrapping row is nonzero.

### 3.4 The minimum slack, recomputed, and what the declaration declares

Same probe, part (b). Minimum slack admitting fusion, W=6, both truncation spellings:

| cell | F=0 | F=1 | F=2 | F=3 | F=4 | F=5 |
|---|---|---|---|---|---|---|
| unsigned, wrapping | 0 | 0 | 0 | 0 | 0 | 0 |
| unsigned, saturating | 0 | 0 | 0 | 0 | 0 | 0 |
| signed, wrapping | 0 | 0 | 0 | 0 | 0 | 0 |
| signed, saturating | 32 | 32 | 32 | 32 | 32 | 1 |

`139`'s p3, at `139:347-352`, gives signed wrapping as `0 1 1 1 1 1` and signed saturating as
`32 32 32 32 32 1`. **The saturating row is identical, digit for digit, from two models that disagree
elsewhere.** The zeros in every unsigned cell are flagged by my C3 control as trivial, because fusion
never differs there at all, and I report them as trivial rather than as a mechanism win.

So the entire disagreement between the two models is one unit, in one cell family, at F >= 1.

Part (c) is the attack on the repair. `139:796` says the set formulation "keeps the property my firewall
exists for, which is that nothing outside the declaration can move an answer". A declaration that permits
`k` answers for a single input determines the answer only when `k = 1`, so I measured `k`:

| cell | required slack | conforming answers per input, mean / min / max | as a fraction of the 64-value range |
|---|---|---|---|
| signed saturating, F=0 | 32 | 41.74 / 33 / 64 | 65.2% |
| signed saturating, F=3 | 32 | 44.38 / 33 / 64 | 69.3% |
| signed saturating, F=5 | 1 | 2.86 / 2 / 3 | 4.5% |

The S4 control holds: at slack 0 the conforming set has cardinality exactly 1 at every input, min and
max alike.

**So the repair does not keep the property. It replaces determinism with a bound, and in the cell that
motivated it the bound admits two thirds of the representable values.** `139` says as much about the
number at `139:356-357`, calling it "a bound that has declared nothing", and then states at `139:796`
that the composition keeps the firewall's property. Those two sentences are in the same file and they
disagree.

The objection `139` raises against an unconstrained weighting, at `139:260-263`, is that "two builds of
one program produce different results with no predicate anywhere naming the difference". Under its own
repair two builds do produce different results, and the predicate naming the difference is a bound that
permits 41.74 of 64 answers. The disease is bounded, not cured, and at the dose that matters the bound
is the whole patient.

### 3.5 The residual unit, and two more of my own controls failing

`141_probes/p3_the_missing_ulp_is_a_rounding_move.rs`. One unit at F >= 1 on signed values is the
signature of a rounding relocation, not a reduction relocation. So: an arm that rounds once over the
whole multiply-add, computing `shift(a*b + c*2^F)`, against a stepwise arm that rounds the product and
then adds.

Under toward-zero truncation, that arm reproduces `139`'s **entire table**:

| cell | F=0 | F=1 | F=2 | F=3 | F=4 | F=5 |
|---|---|---|---|---|---|---|
| unsigned, wrapping | 0.00% | 0.00% | 0.00% | 0.00% | 0.00% | 0.00% |
| unsigned, saturating | 0.00% | 0.00% | 0.00% | 0.00% | 0.00% | 0.00% |
| signed, wrapping | 0.00% | **1.64%** | **5.54%** | **12.34%** | **22.22%** | **33.40%** |
| signed, saturating | **42.14%** | **39.10%** | **35.67%** | **31.96%** | **29.52%** | **33.41%** |

and its slack row for signed wrapping, `0 1 1 1 1 1`. Every cell, to two decimal places.

**R8, my mechanism control, failed.** I had predicted that every differing input has a negative quantity
entering the shift, and tested the sign of the sum. Thousands of differing inputs have it non-negative.
So the numbers matched and the reason did not, and a two-decimal coincidence is not a reproduction.

`p3b` tested a second predicate, the sign of the product, and **R9 failed too**: 2208 of 4296 differing
inputs at F=1 have a non-negative product. What R9 did confirm is that the difference is **always exactly
one unit** and **never occurs when the shift is exact**, and that under floor there are zero differing
inputs at any F.

`p3c` derives the mechanism instead of guessing at it. Writing `x = (a*b)/2^F` as an exact rational, the
two arms are `round(x) + c` and `round(x + c)`, and they agree for all `x` and integer `c` exactly when
`round` is **translation equivariant**. Floor is. Truncation toward zero is not, and its failure is
characterised: `trunc(y)` is `floor(y)` for `y >= 0` and `floor(y) + 1` for negative non-integral `y`, so
the two differ exactly when `x` and `x + c` lie on **opposite sides of zero**. Both my earlier controls
tested one side, which is why each found roughly half the witnesses on the wrong side.

Checked as a biconditional, in both directions:

| F | differing | predicted | predicted but agreed | differed but unpredicted |
|---|---|---|---|---|
| 0 | 0 | 0 | 0 | 0 |
| 1 | 4296 | 4296 | 0 | 0 |
| 2 | 14520 | 14520 | 0 | 0 |
| 3 | 32344 | 32344 | 0 | 0 |
| 4 | 58246 | 58246 | 0 | 0 |
| 5 | 87552 | 87552 | 0 | 0 |

**R13 confirmed exactly.** The mechanism is earned.

### 3.6 So the slack mechanism buys nothing, in any cell

Putting the four probes together, cell by cell, for the mechanism `139` proposes:

- **Unsigned, both overflow positions, every F.** Fusion is already answer-preserving. Both files agree.
  Slack buys nothing.
- **Signed wrapping, every F.** Fusion is answer-preserving by the absorption theorem, exhaustively
  confirmed. The 1 ulp `139` prices is the failure of toward-zero truncation to commute with integer
  translation, which is a **rounding** difference, and rounding is an observable policy axis in every
  file in this panel. Under floor it is zero. Slack buys nothing.
- **Signed saturating, every F.** Slack of 32 of 63 raw units, admitting 65.2% of the range per input.
  `139` calls this a declaration that has declared nothing and I agree. Slack buys nothing usable.

**There is no cell in which the mechanism delivers a win.** Its cost, by `139`'s own accounting at
`139:604-607`, is "a slack field on every policy and a conformance obligation on every arm".

And there is a further problem with the one positive case even if the rounding spelling were fixed the
other way. If the 1 ulp comes from a rounding relocation, then a cost model permitted to spend it is a
cost model permitted to change the rounding mode. Rounding is component one by every membership test in
this panel, including `139`'s own at `139:220-222`. So the mechanism would license the weighting to move
an axis of the policy, which is precisely what the firewall forbids, in the one cell the firewall was
loosened to accommodate.

### 3.7 And the capability it was buying already exists

`141_probes/p6_fusion_is_an_axis_position_already.rs`. `140`'s p1 partitions over an **intermediate**
axis whose two positions are "round and reduce at each step" and "hold the intermediate and reduce once
at the end", and its phase-two `p6` finds that axis carries 10 to 12 of its 24 classes (`140:819-823`).

"Reduce once at the end" is what fusing a multiply-add is. So I checked whether that is an identity or a
resemblance:

| cell | X1 mismatches (all F) | X2 stepwise-vs-exact differences | rate |
|---|---|---|---|
| unsigned, wrapping | 0 | 0 | 0.00% |
| unsigned, saturating | 0 | 0 | 0.00% |
| signed, wrapping | 0 | 0 | 0.00% |
| signed, saturating F=0 | 0 | 110476 | 42.14% |
| signed, saturating F=3 | 0 | 51820 | 19.77% |

**X1 confirmed: zero mismatches everywhere.** The fused lowering of the stepwise assignment is
bit-identical to the plain lowering of the exact-intermediate assignment, at every input, in every cell.
X2 confirmed: the two intermediate positions are genuinely two positions, differing exactly where fusion
differs. X3 confirmed: the rate column reproduces my own p1 floor table cell for cell, which is what
makes this a measured identification rather than the same expression written twice.

**So the thing `139` wants a slack mechanism to buy is an axis position the design already has.** A
consumer who wants the fused answer on a signed saturating type selects the exact-intermediate position,
declares it in the type, keeps full determinism, and gets the fast arm. Nothing needs loosening.

---

## 4. Replacements owed to `139`

An attack that proposes nothing has done half a dispatch. Five, in the order I would try them.

**A. Keep the firewall exactly as first written, and gate the fused arm on a const predicate.** This is
I13, `INTENTS.md:214-235`, which is ratified and which says in op's words that we collect "answers to
specific regions where a predicate holds" and that a nameable predicate for the sometimes "allows
choosing the more optimal lowering and arm for that specific case". `139`'s own p2 table is that
predicate already; it drew the other conclusion from it. Under floor the fused arm is answer-preserving
in 18 of 24 cells and the predicate is `signedness = unsigned, overflow any` together with
`signedness = signed, overflow = wrap`. Zero mechanism, zero slack field, full determinism, and the win
is kept wherever it is real.

**B. Spell the fractional shift as an arithmetic shift right rather than an integer division.** That
converts the signed wrapping cells from "1 ulp of declared slack" to free (`p3` R6 and R12, zero
differing inputs at every F). It is the instruction the hardware has, it is translation equivariant so
the relocation question does not arise, and it has no semantic content at all on non-negative values.
This is the cheapest replacement on the list and it removes the mechanism's only positive case by making
the case unnecessary rather than by refusing it.

**C. For signed saturating, do not buy fusion. Select the exact-intermediate position.** `p6` X1: they
are the same function, bit for bit. The consumer declares what they want, the type says what it means,
and the arm that is fast is also the arm that is correct for that declaration. This is strictly better
than the slack mechanism on every axis: it is more precise, it costs no new field, and it preserves the
property the firewall exists for.

**D. If a tolerance is genuinely wanted for some reason none of the above covers, make it an axis
position, not a modifier on a policy.** Then two tolerances are two types, the type still determines a
specification, and a consumer reading a signature can see what they will get. As a modifier it makes the
type determine a set and leaves the choice inside that set to a cost model, which is `139`'s own stated
objection turned on its own repair.

**E. Compose the firewall with Q51's denotation repair rather than with a slack field.** `OPTIONS.md`
Q51 records that component one fixes the **denoted** answer and component two ranges over realisations.
The firewall then reads: every arm realises the denotation exactly, and the weighting ranges over
realisations that are extensionally equal. That is `139`'s property, stated in the vocabulary the unit
already converged on, with no new mechanism. `139`'s phase two at `139:788-800` proposes composing the
two by making the bound a floor and fidelity a coordinate inside it; my objection is only to the bound,
and E is that composition with the bound set to zero.

**And one caution that cuts against my own replacements.** I16, `INTENTS.md:317-331`, says the canon does
not police what shape a law takes, and that a law "is defined as makes sense and is applicable in each
situation on a case by case basis". Replacements A through C name a shape. They are offered as what the
measurements support in these cells, not as a rule about how a permission must be constructed, and if
they read as the latter they have overreached and should be cut back to the predicate alone.

---

## 5. The counting claims

The dispatch asks whether `139`'s and `140`'s counting claims are one finding or two, by construction
rather than by comparing prose.

`139` varies the **shape** with the operation set fixed and reports 2, 3, 8 and 12 classes from the same
twelve labels (`139:166-171`), concluding at `139:497-500` that "the count is not a property of the
design at all". `140` varies the **operation set** with the shape fixed and reports 15 to 24 by adding
one operation, concluding in F2 (`140:667-671`) that the count is "a strictly increasing function of the
witness set".

### 5.1 They are one claim, and it is a theorem

`141_probes/p5_the_two_counting_claims_are_one.py`, written in Python over exact integers so it shares no
code with my four Rust probes or with either member.

Both are quotients of one assignment set by observational equality, and the thing being quotiented by is
the **observation set**. An observation is a (shape, operation, input) triple, so fixing a shape and
fixing an operation set are two ways of restricting the same set. That makes the underlying statement a
theorem rather than a measurement: if `O1` is a subset of `O2`, then equality on `O2` implies equality on
`O1`, so the `O2` partition refines the `O1` partition and cannot have fewer classes.

Checked over every subset pair at every shape: **2532 pairs, zero monotonicity violations.** A violation
would have meant my instrument was broken, not that the theorem was false, and I say so rather than
presenting a theorem as a discovery.

### 5.2 `140`'s F2 is false as written

The theorem gives monotone non-decreasing. Strictness is a fact about the operation added, not about the
witness set, and the counterexamples are not rare:

**714 (shape, subset, operation) triples add exactly zero classes.** At `('u', 3, 0)`, `{add}` plus
`sub`, plus `mul`, or plus `mac` each stays at 2 classes.

So "a strictly increasing function of the witness set" is refuted, and the repair is one word. This
matters more than a wording nit because F2 is the finding `140` says it "would most want attacked"
(`140:878-882`) and the one it stakes the most on (`140:921-923`). Its content survives the correction
intact; its form does not.

### 5.3 `139`'s conclusion is too strong in the other direction

Under the full operation set, at a fixed shape, the count is determined:

| shape | classes |
|---|---|
| unsigned, W in {3,4}, F in {0,1,2} | 3 at every combination |
| signed, W in {3,4}, F = 0 | 3 |
| signed, W in {3,4}, F in {1,2} | 6 |

That is a well-defined function from shape to count. **It is not a single number and it is not "not a
property of the design at all"**; it is a property with an argument, and the argument is the shape. The
useful canon sentence is the function, not its non-existence.

The C-null control holds: a constant operation changed the count in 0 cases. C-live: the maximum count
is 6, so the partition is non-trivial. C-dup: a duplicate assignment merges.

### 5.4 My own new claim, refuted

I predicted (U4) that two shapes would reverse their order by class count between two witness sets, which
would have shown neither variation is prior to the other. **Zero reversing instances.** The shape ordering
is witness-set stable throughout the swept region, so U4 is refuted and the symmetry I was reaching for
is not there. I leave it stated because a later expert should know the route is closed and why.

### 5.5 The integer width does nothing, and establishing that took two tries

The U3 table above shows the count moving with signedness and with F and not with W. `139`'s own table
already contains the same pattern without naming it: every pair in it that differs only in W agrees
(`139:166-171`: W=6 F=3 unsigned is 8 and W=8 F=4 unsigned is 8; W=6 F=3 signed is 12 and W=8 F=4 signed
is 12; W=6 F=0 signed is 3 and W=8 F=0 signed is 3).

`p5b` checked it and **its sensitivity control came out toothless.** I predicted W=2 would break the
invariance and it did not, 0 of 31 subsets. A check that reports "the count does not move with W" and
cannot be made to move with W is indistinguishable from a check that is blind to W, so `p5b`'s zero was
not earned and I did not use it.

`p5c` adds an axis position that is width-sensitive by construction, an overflow position clamping the
high side at the literal constant 6, present as a control rather than as a design proposal.

- **V4 confirmed, control fires**: 62 triples move between W=2 and W=3 with it present.
- **V5 confirmed**: with the base axes, zero of 31 subsets at six (sign, F) combinations move across
  W in {3,4,5,6}.
- **V6 refuted**: I predicted the invariance would resume above the reachability threshold and it does
  not, 93 triples still move. My error was assuming the threshold is uniform across signedness; the
  constant 6 is outside the signed range until W=4 and inside the unsigned range from W=3.

So the earned statement is narrower than "the count is W-invariant" and more useful:

**For the axes both files actually swept, the class count does not move with the integer width, and
W-invariance is a property of the axis set rather than of the design.** An axis position whose
reachability depends on the width breaks it, and one exists as soon as anybody writes a bound with a
constant in it.

---

## 6. Replacements owed to `140`

**F. Restate F2 as monotone non-decreasing in the observation set**, with strictness a property of the
operation added. 714 counterexamples to the current wording, and the theorem is one line.

**G. State it once, as a quotient.** The class count is the cardinality of the assignment set quotiented
by observational equality over an observation set, and the observation set has shape, operation and input
as components. That single statement subsumes `139`'s shape variation and `140`'s witness variation, and
it removes the appearance of two findings competing for the same slot in a canon.

**H. State the count as a function rather than as an absence.** `shape -> count` is a design property.
Saying the count is not a design property invites a reader to conclude nothing can be said, when the
function can be tabulated exhaustively at model widths.

**I. Carry the W result with its own predicate.** The shape dependence both files measured is entirely in
signedness and fraction width, for their axis sets, and it is not safe to promote that to a general claim
about shape because `p5c` V4 exhibits an axis that breaks it.

---

## 7. Findings, with predicates

Per I13 and `RULES.md:486-541`. A dimension listed with a range or `any` is established across it, a
dimension listed with a fixed value is established there only, and an absent dimension claims nothing
anywhere that dimension is present. Every probe here runs on one thread, so every predicate says so, and
under the notation that means none of these findings holds anywhere threads exist.

**F1. A lossless storage container contributes zero distinguishable classes.** Third independent
instance; `139`'s p6 and `140`'s p3 are the other two.

```
holds for: numeral fixed-point, W = 4, F in {0, 1, 2},
           signedness in {unsigned, signed},
           rounding in {toward zero, floor}, overflow in {wrap, saturating},
           storage container in {packed at W, minimum rung, double rung},
           accumulator width = W,
           overflow limit read at the declared width,
           operations {add, subtract, multiply, multiply-add, two-term dot},
           arity in {2, 3, 4}, chain length in {1, 2},
           threads = 1,
           target features = host (aarch64-apple-darwin, rustc 1.98.0-nightly 57d06900f)
```

**F2. The accumulator width is answer-visible exactly at signed saturating.** New; neither cold
derivation has an accumulator dimension.

```
holds for: numeral fixed-point, W = 4, F in {0, 1, 2},
           signedness in {unsigned, signed},
           rounding in {toward zero, floor}, overflow in {wrap, saturating},
           storage container in {packed at W, minimum rung, double rung},
           accumulator width in {W, W + 2, 4W},
           overflow limit read at the declared width,
           operations {add, subtract, multiply, multiply-add, two-term dot},
           arity in {2, 3, 4}, chain length in {1, 2},
           threads = 1,
           target features = host (aarch64-apple-darwin, rustc 1.98.0-nightly 57d06900f)
```

**F3. Reduction modulo a power of two absorbs a prior reduction, so relocating the reduction in a
multiply-add cannot change an answer under wrapping.** Proved, then checked exhaustively with a control
that fires when saturation is substituted.

```
holds for: reduction = wrapping, W in {4, 5, 6},
           signedness in {unsigned, signed},
           x swept over four times the declared range on both sides,
           c swept over the whole declared range,
           operation = one addition after one reduction, arity = 2,
           threads = 1,
           target features = host (aarch64-apple-darwin, rustc 1.98.0-nightly 57d06900f)
```

**F4. The minimum slack admitting a fused multiply-add is zero in every cell except signed saturating,
where it is 32 of 63 raw units at F <= 4 and 1 at F = 5.** Second independent instance for the signed
saturating row, which matches `139`'s p3 digit for digit; the signed wrapping row disagrees with `139`
and F5 explains why.

```
holds for: numeral fixed-point, W = 6, F in {0, 1, 2, 3, 4, 5},
           signedness in {unsigned, signed}, overflow in {wrap, saturating},
           rounding in {truncate toward zero, floor},
           the two arms applying the same fractional shift,
           operation = multiply-add, arity = 3, chain length = 2,
           threads = 1,
           target features = host (aarch64-apple-darwin, rustc 1.98.0-nightly 57d06900f)
```

**F5. Relocating the rounding rather than the reduction differs exactly when the product quotient and the
fused quotient lie on opposite sides of zero and the shift is inexact, always by exactly one unit, and
never under floor.** Checked as a biconditional in both directions.

```
holds for: numeral fixed-point signed, W = 6, F in {0, 1, 2, 3, 4, 5},
           overflow = wrap, rounding in {truncate toward zero, floor},
           operation = multiply-add, arity = 3, chain length = 2,
           threads = 1,
           target features = host (aarch64-apple-darwin, rustc 1.98.0-nightly 57d06900f)
```

**F6. A policy declaring the slack that admits fusion at signed saturating permits a mean of 41.74
answers out of 64 for a single input, and exactly one at slack zero.**

```
holds for: numeral fixed-point signed, W = 6, F in {0, 3, 5},
           overflow = saturating, rounding = floor,
           slack in {0, the minimum admitting fusion},
           operation = multiply-add, arity = 3, chain length = 2,
           threads = 1,
           target features = host (aarch64-apple-darwin, rustc 1.98.0-nightly 57d06900f)
```

**F7. The fused lowering of the stepwise-intermediate assignment is bit-identical to the plain lowering
of the exact-intermediate assignment.**

```
holds for: numeral fixed-point, W = 6, F in {0, 1, 2, 3, 4, 5},
           signedness in {unsigned, signed}, overflow in {wrap, saturating},
           rounding = floor,
           intermediate in {round and reduce each step, hold and reduce once},
           operation = multiply-add, arity = 3, chain length = 2,
           threads = 1,
           target features = host (aarch64-apple-darwin, rustc 1.98.0-nightly 57d06900f)
```

**F8. The class count is monotone non-decreasing in the observation set and not strictly increasing.**

```
holds for: assignments = rounding {toward zero, floor} x overflow {wrap, saturating}
             x intermediate {stepwise, exact},
           W in {3, 4}, F in {0, 1, 2}, signedness in {unsigned, signed},
           witness sets = all 31 non-empty subsets of
             {add, subtract, multiply, multiply-add, multiply-subtract},
           threads = 1,
           target features = host (aarch64-apple-darwin, rustc 1.98.0-nightly 57d06900f)
```

**F9. For those axes the class count does not move with the integer width, and an axis position whose
reachability depends on the width makes it move.**

```
holds for: assignments = rounding {toward zero, floor} x overflow {wrap, saturating}
             x intermediate {stepwise, exact},
           W in {3, 4, 5, 6}, F in {0, 1, 2}, signedness in {unsigned, signed},
           witness sets = all 31 non-empty subsets as in F8,
           and separately with a width-sensitive overflow position added,
           threads = 1,
           target features = host (aarch64-apple-darwin, rustc 1.98.0-nightly 57d06900f)
```

Every one of these additionally requires **container width = declared width**, which is the narrowing
`139` reported against itself in its phase two (`139:750-767`) and which applies to my instruments for
the same reason: nothing in my models has a container wider than the declared width except the
accumulator dimension in F2, which is declared there explicitly.

---

## 8. What I carry forward unchanged, with a count

**Eleven positions, from two members, kept because my own derivation did not unseat them.** Four of them
I derived or measured independently, and those are marked, because an independently reached agreement is
the only thing that earns the two-expert rung and it is the contribution least likely to be reported.

From `139`:

1. **The two-component pair.** Six probes of mine, none of which produced a candidate for a third
   component or a reason to merge the two.
2. **Packing is answer-invisible at the column.** *Independently measured*, `p4` T1, third instance.
3. **The axes are discovered and the presets are chosen** (`139:504-514`). I have nothing against the
   membership procedure and my `p5` monotonicity result is what makes it well defined: the procedure
   quotients by an observation set, and enlarging the set can only refine.
4. **The count is not a single shape-free number.** Kept, with the correction in section 5.3 that it is
   a function rather than an absence.
5. **The observability procedure belongs to the chain rather than to the axis**, which `139` conceded
   against itself in phase two (`139:769-776`). *Independently supported*: my `p4` T5 shows `add` alone
   cannot see the accumulator at any shape.
6. **The livelock diagnosis of `bitpack-write-contend-shared`.** *Independently reproduced*, at 46.65s
   under the flag on my host. The diagnosis of the mechanism is `139`'s and I did not re-derive it.
7. **That fusion under signed saturation is a genuine answer change at 42.14% at F=0.** *Independently
   measured*, `p1`, exact agreement.

From `140`:

8. **The shared-baseline obligation** (`140:620-624`): every strategy's cost claim stated against one
   named arm rather than each against its own naive version. I attacked nothing here and found nothing
   against it. It is the piece `140` says is most in need of a second read and I am not that read,
   because I did not build an instrument for it; I carry it forward as still standing at one expert.
9. **The closure asymmetry** (`140:160-169`): the assignment space is closed and enumerable, the
   weighting space is open and consumer-supplied. My `p5` count function is a fact about the first and
   says nothing about the second, which is consistent with the asymmetry rather than evidence for it.
10. **The declared-width companion rule** (`140:862-868`): the overflow limit is read at the declared
    width and a container is never allowed to move it. I built every instrument on that reading and it
    is what makes F1 and F2 separable at all.
11. **That the count moves with the witness set.** *Independently measured*, `p5`, with the wording
    correction in section 5.2.

**Zero positions carried from any other panel file**, because my reading was `139`, `140`, `INTENTS.md`,
`RULES.md` and `OPTIONS.md` Q51 only.

---

## 9. Options, each with what would close it

**O-141-A. The minimising concern's assignment is free at the column and constrained at the accumulator.**
The concern composes with every assignment for stored values, and at `signedness = signed, overflow =
saturating` a narrowed accumulator is a declared policy choice rather than a cost choice.
*Closes on*: whether any consumer wants a narrowed accumulator at signed saturating. If none does, the
constraint is theoretical and the concern is free in practice. `mock/benches/variants/warm-clamp-*` and
`satfold-*` are where an accumulator-width arm already exists, so this is a computation over committed
artifacts rather than a new bench.

**O-141-B. The fractional shift is an arithmetic shift right, not an integer division.**
*Closes on*: whether any consumer semantics needs round-toward-zero on negatives specifically. If one
does, the two spellings are two rounding positions and both ship, which is I13's shape anyway. If none
does, the choice is free and it removes a whole family of relocation questions.

**O-141-C. A policy pins one answer, and every win that looks like it needs slack is an axis position.**
The strong form of my section 3. *Closes on*: finding one optimisation that changes an answer, is worth
having, and is not expressible as a position on an existing axis. `p6` shows fusion is not such a case.
If none is found across the whole optimisation catalogue, the slack mechanism can be dropped from the
option register entirely rather than carried.

**O-141-D. The canon states `shape -> count` as a table at model widths, or states only the axes.**
*Closes on*: whether a consumer ever needs to know how many distinguishable strategies a shape admits. If
the answer is only ever "declare the axes you want", the count is diagnostic rather than canonical and
belongs in the audit trail. If a design decision anywhere reads the count, it is canon and needs the
table.

**O-141-E. W-invariance is stated as a property of the axis set rather than of the design.**
*Closes on*: enumerating the axes the design actually ships and checking each for a width-dependent
position. `p5c` shows one exists as soon as a bound carries a constant, so the honest default is that
the invariance is not general.

---

## 10. Coverage, bounds, and what I did not do

**My predictions that fell, which is the part worth reading.** Ten, of which four changed a conclusion
rather than a detail:

| # | prediction | verdict |
|---|---|---|
| Q1 | unsigned wrapping fusion is 0.00% at every F, with the reason I gave | confirmed, and **my stated reason was incomplete**, which I flagged in the probe before running it |
| Q6 | at least one cell disagrees with `139` under at least one truncation mode | confirmed, and it turned out to be every signed cell at F > 0 |
| R1, R2, R3 | model B (reduce before shift) reproduces `139`'s table | **refuted**, and it broke the unsigned rows both files agree on |
| R8 | every differing input has a negative quantity entering the shift | **refuted** by my own control, 2088 counterexamples at F=1 |
| R9 | every differing input has a negative product | **refuted** by my own control, 2208 counterexamples at F=1 |
| R13 | the difference is exactly the failure of translation equivariance | confirmed, biconditionally, zero exceptions |
| T3 | the accumulator is answer-visible | confirmed for signed, **refuted for unsigned** |
| T4 | visible under saturating, invisible under wrapping | **half refuted**: unsigned saturating is invisible too |
| U4 | two shapes reverse their order between two witness sets | **refuted**, zero instances |
| V2 | a degenerate width breaks W-invariance | **control came out toothless**, so V1 was unestablished and had to be redone |
| V6 | invariance resumes above the reachability threshold | **refuted**, the threshold is not uniform across signedness |

The two that mattered most are R8 and R9, because between them they stopped me shipping a two-decimal
reproduction of `139`'s table with the wrong reason attached, which would have read as a full explanation
and been a coincidence.

**Everything here is `threads = 1`.** No probe touches concurrency, so under the panel's notation none of
these findings holds anywhere threads exist. That is the strong reading and it is intended.

**Every measurement is at model widths.** W in {3,4,5,6}. I have no transfer argument to 64 bits and I am
not offering one, and `unstable-features.md` is explicit that a model-width result needs its own transfer
argument rather than inheriting one.

**Container width equals declared width in every instrument except p4's accumulator dimension.** That is
the same narrowing `139` reported against itself, and it applies to me for the same reason. Q51 reports
0% against 89.081% depending on which width the limit is read at, so this is not a technicality, and
every finding above is conditional on the declared-width reading.

**I priced nothing.** No claim here is a bench result and none is called one. Whether the fused arm is
actually faster than the stepwise arm, and by how much, is **unpriced**, and `mock/benches/` is where
that would be answered. My whole argument is that the fast arm is available without the mechanism, not
that it is fast; if it turned out that fusion buys nothing measurable, section 3 would still stand and
would simply matter less.

**I did not attack `140`'s baseline argument.** It is the piece `140` most wants a second read on
(`140:702-706`, `140:926-927`) and I did not give it one, because my instruments were all pointed at
semantics and a baseline argument is about how cost claims are stated. It remains at one expert and it
should be somebody's next dispatch.

**I did not touch the weighting side at all.** `139`'s `p4` weight-cell geometry, the 44.3% mapping
difference across targets, and the Pareto-optimal arm no linear weighting can select are all untested by
me. I have no view and I am not manufacturing one.

**I read five documents**: `139` in full, `140` in full, `INTENTS.md` in full, `RULES.md` in full, and
`OPTIONS.md` entry Q51 only. I did not read `40`, `93`, `102`, `106`, `107` or `108`, which Q51 cites,
so my section 3.7 and replacement E reason against Q51's account of `108` section 7 rather than against
`108` itself. If Q51's compression of it is wrong, replacement E moves and nothing else in this file
does.

**Where I would want a second pair of eyes first.** Section 2.2's accumulator result, because it is the
only genuinely new claim in this file, it rests on one instrument, and it is the kind of finding that is
easy to produce by modelling an accumulator in a way no implementation would. The specific thing to check
is whether `acc_width` in `p4` is a shape any consumer would actually build, and whether the operations I
routed through it are routed the way a real kernel would route them. If a second reader builds a
different accumulator model and the signed-saturating cell still separates, the finding is real; if it
does not, F2 is an artifact of my own construction and the convergence stands unqualified.

---

## Appendix: the probes

Eight, committed before this file, each proving one thing once.

1. `p1_fusion_reproduced.rs`: `139`'s fusion table on an independent model, both truncation spellings,
   with reach controls that flag a vacuous cell. Agrees exactly at F=0 and at every unsigned cell,
   disagrees at every signed cell above F=0.
2. `p1b_which_model_produces_139s_table.rs`: the reduce-before-shift hypothesis, refuted by its own
   controls, kept as the record of a closed route.
3. `p2_the_slack_buys_nothing.rs`: the absorption theorem checked exhaustively with a control that fires
   under saturation; the minimum slack recomputed; the conforming-set cardinality measured at 41.74 of
   64 with a slack-zero control at exactly 1.
4. `p3_the_missing_ulp_is_a_rounding_move.rs`: the one-rounding arm reproducing `139`'s whole table to two
   decimals, and my mechanism control failing.
5. `p3b_the_mechanism_control_corrected.rs`: the second mechanism guess, also refuted, and the two facts
   that survived it (always one unit, never under floor).
6. `p3c_translation_equivariance_is_the_mechanism.rs`: the mechanism derived and checked as a
   biconditional, zero exceptions in either direction.
7. `p4_the_concern_reaches_the_accumulator.rs`: the column result reproduced, the accumulator result
   found, the predicate localised to one cell, with a lossy control, a witness-set control and a merge
   control.
8. `p5_the_two_counting_claims_are_one.py`, `p5b_the_shape_dependence_is_not_in_w.py`,
   `p5c_w_invariance_with_a_control_that_fires.py`: the quotient statement, monotonicity over 2532 subset
   pairs, 714 counterexamples to strictness, the shape-to-count function, and the W result with the
   control that `p5b` lacked.
9. `p6_fusion_is_an_axis_position_already.rs`: the fused lowering and the exact-intermediate policy shown
   bit-identical, with a non-vacuity control and a cell-for-cell agreement control.

Three of them exist only because an earlier one refuted me. That is the sequence, and it is the reason
the mechanism in section 3.5 is a finding rather than a coincidence.
