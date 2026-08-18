# 167. The chain, derived cold

**Member:** Tiark Rompf persona. **Unit:** the chain topic, opened by `166`.

**Phase one: written blind.** Premises only, per the cold-derivation protocol in `RULES.md`. No numbered
panel file, no register, no other member's probe, no commit log, no commit subject was read before this
file was committed. What I did read is listed in section 0.3 and includes one item the blind list did not
anticipate and which leaks panel conclusions; I name it there rather than let it pass.

---

## 0. The two gates, and the coverage bound

### 0.1 The canon gate: PASSED

Checked against `INTENTS.md` read in full, including its "How to read an entry" section, and against
`RULES.md` read in full.

The question is licensed and the licence is direct. **I7** is op's, it is STATED, and its wording ranges
over compositions rather than over single operations:

> Precise on other hand is the one that sacrifices as much performance and efficiency as makes sense, to
> be the most precise possible answer, throwing out all cold or hot axis optimisations to be *accurate*
> and *precise*, especially within chains and ops, not only alone.

`mock/canon/` does not exist, nothing is ratified except I13, and I13 is about predicated arms rather
than about compositions. So there is no ratified text this unit could misalign with, and the unit is not
building on a ratified state that would have to be re-derived.

**A second intent bears on this question and the brief did not name it.** I11:

> our main selling point are the algo crates that hilavitkutin, vehje, pretty much every single repo and
> project I have, downstream, use. As well as the contracts for things that compose to bigger units than
> just numerals alone.

"The contracts for things that compose to bigger units than just numerals alone" is a statement about
composition contracts being the point of the library, and it is op's own. It is at least as load-bearing
for this unit as I7 is, and reading the unit as an I7-only unit understates what op has said about it.
I take both as premises below.

### 0.2 The test gate: PASSED, and it reconciles two figures the record disagreed on

Run crate by crate at `--release` per the brief. Commands and raw output in `167_probes/gate/`.

| crate | tests | result |
|---|---|---|
| bitpack-carrier-shared | 9 | ok |
| bitpack-contend-shared | 12 | ok |
| bitpack-footprint-shared | 6 | ok |
| bitpack-plan-shared | 5 | ok |
| bitpack-shared | 3 | ok |
| bitpack-wide-shared | 6 | ok |
| quantiser-fadd-shared | 1 | ok |
| quantiser-radix-shared | 3 | ok |
| satfold-shared | 11 | ok |
| warm-clamp-shared | 7 | ok |
| warm-container-shared | 15 | ok |
| wide-rung-shared | 30 | ok |
| **subtotal, twelve crates** | **108** | ok |
| bitpack-write-contend-shared, `--test-threads=1` | 15 | ok, 2.25s |
| **total, thirteen crates** | **123** | ok |

`holds for: profile = release, threads = 1 for bitpack-write-contend-shared and default for the other
twelve, host = this machine, toolchain = the committed pin`

**This reconciles two counts that have been treated as competing.** 108 across twelve and 123 across
thirteen are both correct and they are not the same measurement: 108 is the twelve crates that run
unserialised, and 123 is all thirteen with the write-contention crate given `--test-threads=1`. The
thirteenth **does terminate** when serialised, in 2.25 seconds at `--release`, so a record saying it does
not is a record of an unserialised run rather than a property of the crate. I did not touch that crate.

Four other variant crates are reported to fail to build on a pre-existing cause. That is outside the
thirteen and I did not investigate it.

**Read rather than counted.** `satfold-shared`'s eleven bodies in full;
`bitpack-shared`'s three and their `check_size` helper; `wide-rung-shared`'s `per_width!` macro. I
scanned every `#[test]` in all thirteen crates mechanically for the tautology shapes: eighteen bodies
contain no `assert` or `panic` token, and every one of them delegates to a helper or a macro that does
assert; I opened four of the eighteen and confirmed this rather than inferring it.

**The suite is not decorative, and `satfold-shared`'s is the strongest I have read in this workspace.**
It carries four deliberately-wrong kernels as negative controls (`WrongOp`, `DropsALane`,
`DropsTheRemainder`, `DropsOneElement`), it asserts each defect exactly where that defect is
*expressible* and skips it where asserting would assert something false, it pins the instrument's own
sensitivity boundary as a two-sided assertion rather than deleting the case that failed, and it checks
the workload is non-degenerate with a range that can fail. `satfold-shared/src/lib.rs`'s
`saturating_addition_is_associative_at_eight_bits` closes the law over its whole domain,
`assert_eq!(total, 1 << 24)`, and its companion proves the false gate is genuinely false. That pair is
directly load-bearing for this unit and I use it in section 5.

### 0.3 Coverage bound, and one leak the blind list did not anticipate

**Read in full:** `INTENTS.md`, `RULES.md`, `mock/Cargo.toml`, `rust-toolchain.toml`, the repository's
`.claude/` rules and the workspace rules that load automatically, `satfold-shared/src/lib.rs`.

**Grepped or skimmed:** `mock/benches/bench.toml`, the variant directory listing, the thirteen shared
crates' test bodies.

**Not opened:** any numbered panel file, any register, any other member's probes, the git log, the
committed CSV rows, `mock/crates`.

**The leak.** The brief permits reading `mock/benches/` including the variant crates. `satfold-shared`'s
module documentation **cites panel files by number and quotes their conclusions**: it names `80` section
5.3 and `82` section 9, reproduces their instructions-per-element figures, and says which arm lost and
why. So a member told to read the bench crates and not the panel has been handed a panel conclusion
anyway. I read it before I understood what it was, and I cannot unread it.

What it contaminated, named precisely so the discount is applied to the right thing: my section 5 uses
the *existence* of a licensed reassociation and its per-operator asymmetry, and `satfold-shared` told me
that a prior file had studied it. It did not tell me the definition of a chain, the observation-boundary
argument, the residual argument, or anything in sections 1 through 4, all of which I derived before
opening that file. **The honest handling is to treat section 5's framing as contaminated and to hold the
rest at full cold rung**, and to say that the blind list needs a line about bench crate documentation,
because this is a general channel rather than one file's accident.

**Which sections move if something I leaned on is wrong.** Sections 1 through 4 rest on op's own words
and on my own probes and would move only if I misread I7 or I11. Section 5 rests additionally on
`satfold-shared`'s committed law tests, which I re-derived independently in `167_probes/assoc/` rather
than citing. Section 7's fork rests on I15's "never any runtime checks, ever" and would collapse if that
sentence admits a reading I have not found.


---

## 1. The answer, stated once before it is argued

**A chain is not a syntactic object and "chain" is not the right unit.** The right unit is the
**unobserved region**: a maximal stretch of a computation in which no intermediate is named by anyone
outside it. Its boundary is the act of observation rather than the operator, everything inside it is
arvo's to choose, and everything at its edge is the consumer's contract. A chain of three multiplies
whose middle value gets stored in a column is two regions, not one, and two multiplies separated by a
`let` binding nobody reads afterwards are one region, not two.

**What a composition owes that a single operation does not is three obligations, and none of them is the
composition of the step obligations.**

1. **An endpoint contract that its steps cannot add up to.** Strengthening every step to "correctly
   rounded" does not make the chain correctly rounded, and probe D shows there is no intermediate width
   short of exact at which it does.
2. **A choice of association**, which a single operation does not have, whose licence is per operator and
   per width, and which probe E derives exhaustively as a four-against-eight partition.
3. **A budget**, which is a global resource. The bits a chain needs are not the sum of the bits its
   operations need, and probe C counts the gap at 50.9% to 72.7% on ordinary chains.

**And one asset, which a per-operation surface structurally destroys: the residual.** Probe A shows that
carrying the bits an operation's output type has no room for turns an error that grows linearly in the
chain length into one bounded below a single LSB, and that the carried form is exactly equal to
accumulating in the wide type, which is a theorem rather than a coincidence.

**The consequence for locus.** Each of those four is a fact about a region and none of them can be
attached to a value, because a value does not know what will be done to it next. So a design whose only
compositional surface is one operation at a time cannot hold I7's guarantee, however good its operations
are. That is the finding, and it is a locus finding rather than a mechanism one.

---

## 2. Where the binding-time boundary is, and where it has been drawn

The first question about any computation is which part is known now and which part is deferred to the
program that runs. For arvo the answer is unusually clean, and it is clean because of I14 and I15 rather
than by accident.

**Known at compile time:** every operator in the expression, the shape of the DAG, every declared width,
every strategy marker, every association as written, the length of any fold whose count is const, and
therefore every derived quantity over those. I14 puts sizes at const and makes monomorphisation the
dispatch; I15 says everything reaches one lowered path and there is never a runtime check. Between them
the entire structure of a computation is a compile-time object.

**Deferred:** the values, and nothing else.

That is a two-stage program in the strict sense. The structure is stage one and the values are stage two,
and the job of stage one is to emit the code that stage two runs.

**The boundary a per-operation surface draws is in the wrong place**, and it is worth being exact about
where. It draws it at the operation: each operation is separately a small stage-one object that emits its
own lowering. Everything *between* operations then falls out of stage one entirely, because no stage-one
object spans two operations. The consequence is not that the chain is slow. It is that the chain **is not
represented at all**, so every fact about it is recomputed, conservatively, at each step, from the only
thing a step can see, which is its operands' declared types.

That is a binding-time error in the exact sense: information available earlier is being reconstructed
later and worse. And it has the signature such errors always have, which is that the reconstruction is
sound and lossy at the same time, so nothing ever fails and the cost is invisible.

**Two clarifications, because both are ways this argument gets misread.**

It is not an argument that arvo should be lazy or that anything should be deferred to run time. The
opposite: the whole point is that the chain is available *earlier* than the design currently uses it.

And it is not an argument against per-operation lowering. Some of I13's arms genuinely are per operation:
a width-specific lowering of one multiply is an arm, its predicate is over that operation's own
typestate, and nothing above it is needed. The claim is narrower and it is this: **some arms are not per
operation, and under a per-operation-only surface those arms have no site to be applied at.** An arm is a
rewrite of an expression. Where there is only ever one operation in view, there is exactly one thing to do
with it, and the whole reassociation family, the whole width-narrowing family and the whole
residual-carrying family are unreachable, not because they were rejected but because there is nothing for
them to be predicates over.

I13 is the one RATIFIED entry. I read this as I13 and I7 wanting the same mechanism, and that is the
single most load-bearing sentence in this file.

---

## 3. What is carried along a chain, and what is discarded at each step

Four things are computed at every step of a chain and thrown away by an operation whose output type has
room only for a value.

**The residual.** A Q(.F) multiply produces 2F fraction bits and returns F of them. The other F bits
existed, were correct, and are destroyed. Probe A measures what that costs over a chain and what
recovering it buys.

**The exactness bit.** Whether this particular step rounded at all. A chain in which no step rounded is
exact, and nothing per operation records that it happened.

**The achieved range.** The declared type of a result says what values it could hold. The step knows a
much tighter bound, from its operands' achieved ranges rather than their declared ones. That knowledge is
regenerated from the declared type at the next step, which is where the width over-provisioning in probe
C comes from.

**The correlation.** Two intermediates derived from a common ancestor have errors that are not
independent and can cancel exactly. `(a - b) + b` is `a`. Per operation there is no `a` to notice.

Probe A is the measurement of the first of these, and it is the one that admits a clean number.

### 3.1 What the residual is worth, measured

`167_probes/residual/`. A fixed-point multiply-accumulate at `F = 12`, error in LSBs of the Q(.12)
result, worst of 32 seeds:

| n | naive, floor per step | naive_round, nearest per step | widened | comp, residual carried |
|---|---|---|---|---|
| 16 | 10.94 | 2.87 | 0.998 | 0.998 |
| 256 | 136.2 | 8.43 | 0.995 | 0.995 |
| 4096 | 2084.7 | 37.95 | 0.989 | 0.989 |
| 65536 | 32831.3 | 231.6 | 0.984 | 0.984 |
| 1048576 | 524046.5 | 1418.2 | 0.929 | 0.929 |

Three readings, and the third is the design one.

**Per-operation truncation makes the error linear in the chain length.** 524046 at n = 1048576 is n/2,
which is the floor bias accumulating unopposed.

**The best a per-operation design can do still grows.** `naive_round` is round-to-nearest at every step,
which is the strongest per-operation accuracy contract there is, and its error grows as the square root
of the chain length. It is better by a large factor and it is the same shape of answer: an error that
depends on how long the chain is.

**The two arms whose error does not grow at all are both chain-level constructions**, and one of them
needs no wide accumulator. `comp` equals `widened` on every row because `acc * 2^F + carry_n = sum(p_i)`
exactly, so the carried residual reconstructs the wide accumulation. It is one extra F-bit register
against a doubled accumulator type.

Negative controls all clean: at `F = 0` every arm is exact (nothing to discard), on a workload where
every product is exactly representable `naive` is exact (the workload is not rigged), and `fake_comp`,
which computes the residual and discards it, equals `naive` bit for bit at all eleven sizes (the
advantage is the feeding forward and not the computing).

`holds for: F = 12, I = 3 including sign, n in {1 .. 2^20}, operands uniform in [-4,4), signedness =
signed, rounding in {floor, nearest}, threads = 1, profile = rustc -O`

### 3.2 The probe that failed first, and what the failure was worth

The companion probe was built to show a region where the one-rounding guarantee is reachable by carrying
the residual and unreachable by widening the accumulator at a fixed container width. **Its first version
found zero such geometries at any size tested**, and the failure is more informative than the fix.

It drew operands from `[-4, 4)`. On decorrelated signed data the accumulated sum is a random walk growing
as the square root of `n`, so the worst-case accumulator width `I + 2F + log2(n)` is never approached and
nothing overflows, however large the geometry. `carrier_bound_v1_FAILED.out` is that run, kept.

**The finding inside the failure: worst-case accumulator width is a fact about correlation between the
terms, not about the chain's length or its declared widths.** A chain sized for the worst case pays up to
`log2(n)/2` bits more than the realised behaviour needs, and nothing in a declared type distinguishes the
two cases. That is a chain-level fact of a third kind, and it is one arvo genuinely cannot derive: it is a
property of the data. What arvo can do is let the region be declared, and refuse to pretend it knows.

With non-cancelling operands the boundary appears exactly where the arithmetic predicts, at a 64-bit
container:

| F | n | widened bits needed | comp bits needed | outcome |
|---|---|---|---|---|
| 20 | 262144 | 61 | 41 | both exact |
| 24 | 16384 | 65 | 41 | **only the carried form is exact** |
| 26 | 262144 | 73 | 47 | **only the carried form is exact** |

Five such geometries, with the control (a geometry where both fit must agree, and does) clean.

`holds for: container = 64 bits, I = 3 including sign, F in {8,16,20,24,26}, n in {2^10, 2^14, 2^18},
operands non-cancelling, rounding = floor, signedness = signed, threads = 1`
