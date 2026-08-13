# 86. How far the verdict procedure reaches, and the saturating threshold family

**Author lens:** Knuth. The analysis is the understanding: the exact operation count, the theorem with its
perimeter stated aloud, the constant checked and not only the exponent. A proof of correctness is a claim
one must be able to keep, and the perimeter of the claim matters as much as the claim.

**Position:** seventh expert in the derived-algebraic-laws unit, after `76`, `77`, `79`, `80`, `81`, `82`,
op's `83`, `84`, and op's `85`. My assigned question is the one `84`'s closing section names first for
whoever follows: how far the finite-difference decision procedure actually reaches, since its multivariate
form was "proof-sketched and verified only at per-variable degree at most 2" (`84:516-519`); and second,
whether the threshold defeat extends outside the ring fragment, since a saturating threshold family "was
searched for by nobody, including itself" (`84:532-538`). I wrote `08` earlier in this panel under this
persona and have no memory of it; I opened its head, found it is about the format concept and not the law
layer, and did not rely on it. One persona agreeing with itself is one instance, and none of my findings
below rests on `08`.

**Probes:** nine committed artifacts in `86_probes/` (`p0` rerun note, `p1` source and output, `p2` driver
and output, `p2b` driver and output, `p3`, `p3b`, `p4` with refusals, `p5`), each committed with its
transcript as it ran and before this file. Every gate-shaped construction below is a const fn with no
feature gates, no `dyn`, no `TypeId` and no allocation; the runtime instruments use `std`, `Vec` and boxed
trees as spike scaffolding, which is instrument plumbing and not design shape, per the panel's probe
discipline.

**The headline, in two sentences.** The multivariate criterion is correct at every arity and every degree,
by a complete proof given below and by measurement at genuine degree where `84`'s battery had none, and it
simplifies: the difference triangle is unnecessary, because the verdict is equivalent to evaluating the law
on its degree grid, so **a false ring law always has a witness inside its own degree box**. And the
saturating threshold family exists: a constant-free, live-clamp law true at every width 1 through 63 and
false at 64, which the band mechanism licenses green end to end at compile time, so the defeat is not a
ring-fragment artifact; what survives for saturating laws is neither transfer direction but a different
decision procedure, built and validated here for the monotone univariate case.

## 0. Gates, and coverage

**Canon gate: passes, situation two.** No canon exists. `mock/canon/` is absent and `mock/crates/` is
empty by the declared mutation order; this panel is writing the first canon. Checked against `INTENTS.md`
in full (I13 the one RATIFIED entry, narrow on op's instruction; I14 IN FORCE; I15 and I16 STATED, both
read at the source) and `RULES.md` in full, and op's `83` and `85` at their sources. Nothing below settles
anything; the mode is explore, and keeping a result is stated as a result where it happens.

**Test gate: no suite exists; the mock workspace has no members.** The substitute is the probe discipline
applied to my own instruments first. Three self-checks are built in rather than assumed: `p1` asserts its
own battery is not degenerate (155 of 225 arity-2 laws carry true per-variable degree at least 2 in both
variables, 109 carry a threshold strictly inside the swept range, both asserted with a failure message,
which is `84`'s own p4 lesson applied forward); `p3b` re-derives the interesting truth-set shapes by
direct table comparison because `p3`'s catalogue classifies by hash and a hash collision could manufacture
a shape (the direct counts, 16 gapped and 24 interior runs, equal the hash counts exactly); and `p5`
verifies every witness it returns actually witnesses (a witness column that cannot fail would be setup
that helps, and its count is asserted zero over 3,708 verdicts).

**Read end to end:** `INTENTS.md`, `RULES.md`, `OPTIONS.md` Q38 through Q40 (`OPTIONS.md:1870-1953`),
`DROPLIST.md:210-260`, `83`, `85`, `84` in full, `81` in full, `79` sections 0 through 2, 10, 11, `80`
sections 3 through 5, `82` sections 7 through 9 with its 8.5 ladder. **Read at the source in named
ranges:** `76` and `77` section headings plus the reconciliation heads, `82` sections 12 and 15 via its
outline, `08`'s head only. **Probes opened at the source:** all of `84_probes/` (five sources, all
transcripts), `80_probes/p2c` via `84`'s account plus my `p0` rerun of it inherited from `84`'s p0,
`82_probes/` listed but only `p2`, `p8`, `p11` outputs skimmed. **Not read:** files `01` through `75`
except as named, `PRIOR_CALLS.md`, `PERSONA_CALLS.md`, the seed files, everything `OLD_`. Everything I say
about `35`, `42`, `55b`, `63`, `74` is routed through `80`, `82`, `84` or the register and inherits their
errors.

**Re-ran before arguing** (`86_probes/p0_rerun_of_84_probes.txt`): six of `84`'s instruments, `p1`, `p2`,
`p2b`, `p3`, `p4`, `p4b`, rebuilt from committed source on the pinned `nightly-2026-05-28` and diffed
against their transcripts. All reproduce, byte-identical on the runtime sections, refusal-identical on the
`--cfg` variants. I also re-derived `84`'s central arithmetic by hand rather than adopting it: v2(64!) =
64 - s2(64) = 63; 64! mod 2^64 = 2^63 because 64!/2^63 is odd; 3 * INV3 = 2^65 + 1 so the INV3 law holds
through width 65 and fails at 128 with witness 1; 311 laws times 12 widths is 3,732; and the four
multivariate boxes are 27, 27, 9, 16 as printed.

**Nothing here is priced.** No bench ran. The wall-clock seconds in `p2` are an ad-hoc quick spike with no
substance; the accept and refuse outcomes are the results.

## 1. A correction before the extension: `84`'s multivariate verification was narrower than its predicate says

`84`'s F4 claims the criterion verified "{2, 3, 4} multivariate at per-variable degree <= 2"
(`84:464-466`), and its least-certain item 2 says "verified at arities 2 through 4, per-variable degree at
most 2" (`84:516-519`). **Both sentences describe the degree bounds fed to the instrument, not the laws.**
Checked at the source (`84_probes/p4_difference_certificate.rs:331-336`): the four multivariate cases are
mul-associativity, distributivity, `a*b - b*a + 8a` and the arity-4 chain, and every one of them is
multilinear, true per-variable degree at most 1. Feeding a multilinear law to the criterion with a bound of
2 adds order-2 difference rows that are identically zero over Z and can never fail, so **the genuinely
quadratic multivariate case was never exercised at all**. The interesting content of the multivariate
necessity argument, higher-order mixed differences carrying nonzero content, had zero instances in the
battery.

This is exactly the class the predicate rule exists to prevent: a dimension listed as established at a
range it was not established at. It changed nothing downstream, because the criterion turns out correct
(section 2), but that is luck, not verification, and `84`'s own univariate battery had already demonstrated
the failure shape it fell into (its first run's thresholds clustered degenerately at zero and it caught
itself; the multivariate battery had the same degeneracy in the degree dimension and did not). The
correction to F4's predicate is in my F1 below and reaches the register through consolidation;
`84`'s file stands as written per `RULES.md:509-518`.

## 2. The multivariate criterion is a theorem at every arity and degree, and here is the proof

`84` called its multivariate form "argued per variable" from a sketch. The complete argument is three
paragraphs, and writing it out is what licenses the rest of this file.

**Setup.** For a multi-index J = (j_1, ..., j_k), let e_J(x) = prod_i (x_i)_{j_i}, the tensor falling
factorial. Every p in Z[x_1, ..., x_k] with per-variable degrees at most d = (d_1, ..., d_k) is an integer
combination p = sum_{J <= d} a_J e_J, with a_J in Z, because monomials convert to falling factorials
through Stirling numbers of the second kind, which are integers. Let Delta_i be the forward difference in
variable i. Then Delta_i^j (x_i)_m = (m)_j (x_i)_{m - j}, so evaluating at the origin kills every term
except m = j, where it leaves j!. Since Delta_i acts on factor i alone,

    Delta^J p (0, ..., 0)  =  a_J * prod_i (j_i!)        for every J <= d.

**Sufficiency.** At integer arguments, e_J(x) = prod_i [ j_i! * C(x_i, j_i) ], so the term a_J e_J is
everywhere divisible by a_J prod j_i!. If 2^W divides every a_J prod j_i!, then p is divisible by 2^W at
every integer point, and a function on (Z/2^W)^k built from ring operations is determined by integer
evaluations because integer polynomials respect argument congruence coordinatewise.

**Necessity.** If p vanishes identically as a function on (Z/2^W)^k, it vanishes mod 2^W at every integer
point, in particular on the degree grid prod_i {0, ..., d_i}. Delta^J p(0) is an integer linear
combination of grid values, hence is 0 mod 2^W, hence 2^W divides a_J prod j_i! by the display above. QED.

So the criterion is exact at every arity and every per-variable degree, with no small-degree proviso. The
univariate case is Kempner 1921 and Singmaster 1974, as `84` cited; the multivariate case is the same
argument tensored, and the display above is the whole content of "tensored".

**Measured at genuine degree** (`86_probes/p1_multivariate_at_real_degree.rs`, output beside it): 225
arity-2 laws including 155 with true per-variable degree at least 2 in both variables and 25 tensor
falling factorials (x)_a (y)_b with a, b through 6, against exhaustive sweeps at widths 1 through 7: 1,575
(law, width) pairs, zero mismatches. 100 arity-3 laws, 47 with genuine degree at least 2 in all three
variables, widths 1 through 4: 400 pairs, zero mismatches. The 25 tensor members' computed thresholds
equal the Legendre prediction (a - s2(a)) + (b - s2(b)) in all 25 cases. The negative control at the class
this file corrects: (x)_3 * y with the x-degree claimed as 2 reports true against the sweep's false,
because the withheld grid line x = 3 carries the whole content, so the instrument can fail in exactly the
understated-bound direction.

## 3. The simplification: the difference triangle is unnecessary, and a false ring law has a witness inside its degree box

The proof above yields something `84` did not state and its implementation did not use. The map from the
grid-value tensor (p(x) for x in the degree grid) to the difference tensor (Delta^J p(0)) is a tensor
product of unitriangular integer matrices: Delta^j p(0) = sum_i (-1)^(j-i) C(j, i) p(i), lower triangular
with unit diagonal, and its inverse is Newton's forward-difference expansion p(i) = sum_j C(i, j)
Delta^j p(0), also integer and unitriangular. An integer unitriangular map and its inverse both preserve
"every entry divisible by 2^s", for every s. Therefore:

- **Verdict.** All differences vanish mod 2^W if and only if all grid values do. The criterion is: evaluate
  the law at every point of prod_i {0..d_i}, through the width-W map itself, and check zero. No transform,
  no tensor storage, O(1) memory streaming.
- **Threshold.** The exact truth threshold is W* = min over grid points of v2(p(point)) computed over Z,
  because the divisibility equivalence holds at every s, so the two minima are equal.
- **Witness.** A false equation law of the wrapping fragment has a witness inside its own degree grid.
  **The witness search space for a ring law is the degree box, not the domain.** This unifies two of the
  three row kinds in `84`'s verdict-table architecture: a procedure row is an exhaustive witness search
  over the box rather than a different kind of thing from a witness row.
- **Polarity at every arity.** Truth at W is W <= W*, so the truth set of a multivariate equation law is
  the initial segment [1, W*]. `84`'s least-certain item 3, "the polarity theorems are measured at arity
  1" (`84:520-521`), is discharged by proof rather than by more measurement, and the disequation dual
  follows by the same coordinatewise reduction `84`'s F3 argues.

All three equivalences are measured as well as argued (`p1`): grid verdict against difference verdict
against sweep, zero mismatches over all 1,975 (law, width) pairs, and threshold-from-grid against
threshold-from-differences, zero mismatches over all 225 exactly-representable laws.

**Two width-64 instances at genuine degree**, decided through the width-64 map: (x)_33 (y)_33 has
threshold 62, so it is true at widths 1 through 62 and false at 63 and above, decided false at 64 in 1,156
evaluations; no band below width 63 could have distinguished its closed form from truth, which extends
`84`'s F2 family to arity 2 with genuinely quadratic-and-higher per-variable content. And (x)_34 (y)_34
has threshold exactly 64: **true at the shipped width and false one width above it**, decided true in
1,225 evaluations, with the width-65 witness residue pinned to exactly 2^64 in u128. A design that ever
widens past 64 has, in this family, laws that were true at 64 for valuation reasons and quietly stop.

## 4. The procedure's own frontier, measured as accept and refuse

`84`'s O-J prices the multivariate cost as "2^k for a length-k multilinear chain law, which is 256
evaluations at k = 8 against the sweep's 2^(8W)" (`84:368-370`) and stops. The honest question is where
the box itself stops being const-evaluable, because if the box dies at small k the replacement inherits a
frontier of the same kind it replaced. `86_probes/p2_box_frontier.py` generates, per k, a crate whose
const fn walks the 2^k grid of a length-k multilinear chain-associativity law at width 64 in streaming
form (the section 3 grid form: no allocation, no tensor), with the verdict TRUE so nothing exits early,
and compiles it under the default guard and under `#![allow(long_running_const_eval)]`. `p2b` pins the
bracket. On the pinned nightly, this host:

| k | grid | default guard | with allow |
|---|---|---|---|
| 8 | 256 | accept | accept |
| 12 | 4,096 | accept | accept |
| 13 | 8,192 | accept | not run |
| 14 | 16,384 | accept | not run |
| 15 | 32,768 | **refuse**, `long_running_const_eval` | not run |
| 16 | 65,536 | refuse | accept, 8.7 s |
| 18 | 262,144 | skipped | accept, 38.3 s |
| 20 | 1,048,576 | skipped | accept, 171.8 s |
| 22 | 4,194,304 | skipped | exceeded the probe's own 300 s cap |

Seconds are an ad-hoc quick spike with no substance; the growth per two bits of k is the box's own 4x and
nothing else. What the table establishes, in accept/refuse terms: **the procedure's frontier at width 64
is an arity-and-degree budget of roughly 2^14 grid points by default and roughly 2^20 under the allow**,
independent of width. Against the sweep's frontier, width 5 at arity 3 by default (`80` section 4.1), the
frontier has moved from a width bound to a box bound, and the box bound is generous for the laws this unit
has actually named: associativity, distributivity and their chains are multilinear, so a chain law must
reach length 15 before the default guard notices, and Q40's route (a) lifts grouping chains from arity 3
anyway, so the residual exposure is **high-arity, non-liftable, ring-fragment laws**, of which this panel
has so far exhibited none: the known non-liftable chain laws are the schedule-conditional ones, which
carry rounding and sit outside the fragment entirely (`80` section 4.5).

Two cost notes with the count attached. Over-approximating a degree bound is safe (extra grid rows are
zero over Z) and costs multiplicatively: `84`'s p4 decided arity-3 associativity in 27 evaluations against
the tight multilinear 8, a factor of 3.4 for one degree of slack per variable. And the syntactic degree
extractor can only over-approximate (degree of a product is at most the sum, of a sum at most the max), so
the procedure's second trusted input reduces to the correctness of a one-pass structural recursion, which
is a mechanisable check rather than an author's claim. Fragment membership remains the first trusted
input, and it is the one that bites in section 5.

## 5. The saturating threshold family exists, and the defeat is not a ring-fragment artifact

`84` section 11 names the search nobody ran: "a law with clamps true at all sweepable widths and false at
a shipped one, which is the one construction that would extend section 2's defeat outside the ring
fragment". I constructed three by hand and then made the probes check every part
(`86_probes/p3_saturating_threshold_family.rs`, `p3b`, `p4`).

**E_d, the constant-free live-clamp family.** Under unsigned saturating multiplication,

    E_d:   forall x:   x^d  ==  x^(d+1)

has truth set **exactly widths 1 through d**. At width W <= d, every x >= 2 has x^d >= 2^d >= MAX, so both
sides clamp to MAX, and 0 and 1 are fixed points; at width d+1, x = 2 gives 2^d unclamped against a right
side that exceeds it. Swept exhaustively for d = 2 through 12 at widths 1 through 14: eleven family
members, truth set exactly 1..d in every case, first falsity at d+1 in every case (`p3` output). So
**E_63 is true at widths 1 through 63 and false at 64**: a threshold at the shipped width, no constants,
the clamp firing for every operand from 2 up at every width below the threshold. Its width-64 falsity is
one pinned witness, x = 2, where 2^63 does not clamp and sat(2^64) = MAX; its truth on the unsweepable
stretch 15 through 62 is the clamp argument's claim, verified at widths 20, 40 and 63 on boundary points
plus a thousand random points each, and is an argument rather than a measurement, stated as such, exactly
parallel to `84`'s F1 stretch.

**The band mechanism licenses it, green end to end** (`86_probes/p4_sat_defeat_gate.rs`, the `p2c`
construction with the law exchanged and nothing else): the plausible closed form is constant true, and its
generalisation even reads soundly, "high powers of anything at least 2 collapse to MAX"; the model band 2
through 8 sweeps the law through the saturating map at compile time and agrees; the perturbation control
under `--cfg badclosed` still refuses, so the mechanism is not broken; the arm is licensed at width 64;
and the licensed law is false there, with the `--cfg audit` build refusing in about 126 const multiplies
naming the reason. **No band below 64, at any guard setting, on any host, could have disagreed**, because
the truth set is 1 through 63. The defeat of `84` section 2 therefore extends outside the ring fragment,
compiled, in the mechanism's own shape.

**The constant member, and the embedding convention becoming load-bearing.** With C = 2^63 - 1, the law
`sat_mul(x, C) == sat_mul(x, sat_add(C, 1))` is true at every width 1 through 63 **under both embedding
conventions**, because an all-ones constant reduces to MAX under wrapping and clamps to MAX under
saturation, and sat_add(MAX, 1) = MAX; at width 64 it is false with witness x = 1. And the innocent law
`sat_mul(2, x) == sat_add(x, x)` is true at every width under clamp-embedding of the constant and **false
at width 1 under wrap-embedding**, where the constant 2 becomes 0: a final-segment truth set manufactured
by the convention alone. `84`'s trusted item 2, "how its constants embed at each width is an author's
convention nothing checks" (`84:190-194`), is here not bookkeeping but the difference between a threshold
family and no threshold family, and any canon sentence about a law family across widths owes the embedding
convention a place in its predicate.

**And saturating truth sets have no polarity structure at all.** The catalogue in `p3` classifies every
pair of constant-free terms over {sat_add, sat_mul} on two variables to depth 2, 21,945 pairs, unsigned
and signed, by truth set over widths 1 through 8, and `p3b` re-derives every non-monotone shape by direct
table comparison with the terms printed. Unsigned: every truth set is empty, full, or an initial segment.
Signed: **16 gapped and 24 interior-run truth sets exist**, and they are not exotica. `x == x^3` signed is
true at width 2 alone (at width 1 the asymmetric MIN makes (-1)*(-1) clamp to 0; at width 3 the cube
escapes), truth set fTfffffffff through width 11. And `sat(2x * xy) == sat(2y * x^2)`, a genuine ring
identity over Z, is **false at width 2 only** and true at 1 and at 3 through 11: TfTTTTTTTTT. So for
saturating equation laws, a band FALSE exports nothing upward either: a band containing width 2 refutes a
law that is true at every measured width above it. Both transfer directions are dead, which is strictly
worse than the wrapping fragment, where `84`'s F3 at least leaves the FALSE direction sound. The wrapping
polarity theorem is a property of reduction-stability, and this is its compiled absence.

## 6. What survives for saturating laws: the fragment is not lost, it needs a different procedure

My own `p4` header, written at that point in the work, says "for this family there is no procedure row
available at all". That sentence deserved an attack rather than a comfortable rest, and it is false.

**The observation.** Saturating arithmetic is not modular arithmetic. It is exact integer arithmetic
composed with clamps. For terms built from sat_add, sat_mul, nonnegative constants and one variable,
every subterm is a nondecreasing function of the variable, so every clamp node fires on a final segment
[b, MAX] whose breakpoint b is found by binary search in about W evaluations. Between consecutive
breakpoints the set of clamped nodes is constant, so each side of a law is an honest integer polynomial of
degree at most the syntactic bound D on that piece, and **two integer polynomials of degree at most D that
agree at D+1 distinct points are identical over Q** and hence agree on the whole piece. So the verdict at
width W is: locate the breakpoints (C binary searches, C the clamp-node count), then per piece either
sweep it if it has at most D+1 points or compare D+1 consecutive points. Sound and complete for the
fragment, at any width including 64, in O(C*W + (C+1)*(D+1)) evaluations.

**Validated, not trusted** (`86_probes/p5_sat_piecewise_procedure.rs`): 309 monotone term pairs including
the E_d members, against exhaustive sweeps at widths 1 through 12: 3,708 verdicts, 467 true and 3,241
false, **zero mismatches**, and zero returned witnesses that fail to witness. Then at the width the sweep
cannot reach: E_63 decided **false at width 64 with witness 2** in 516,033 evaluation steps; E_64 decided
**true at width 64** in 616,999; the C-member decided false at 64 with witness 1 in 206, and true at width
40 in 142. The counts are printed by the instrument and the big two are dominated by the 125 breakpoint
searches of a degree-64 pair, each step evaluating a 63-multiply term: the cost is C*W term evaluations,
exactly as the formula says, and it is far inside the default const-eval budget that `p2` measured.

**The perimeter, stated so this is not over-read.** Unsigned, monotone (no sat_sub with the variable on
the right, no negative constants), univariate, clamp-embedded constants. Signed terms are not monotone
(`p3b`'s gapped members live exactly there) and multivariate pieces are regions rather than intervals;
neither is examined, and the sign-uniform fold law of `82` is multivariate and signed, so **it stays on
the named-argument route** and nothing here relocates it. The procedure's trusted inputs mirror the ring
one's: fragment membership (monotone, univariate) and the degree bound, both syntactic.

**What this does to the verdict-table architecture** (`84` section 6). The table's three row kinds stand,
and the "procedure" kind stops being coextensive with the ring fragment: it is **per-fragment**, a
complete test-set theorem where one exists. Two now exist: the degree grid for ring laws (any arity, any
degree, section 3) and the piecewise sample for monotone univariate saturating laws. The general shape a
canon could carry, offered as intent rather than mechanism: **a law verdict row cites either a witness, or
a complete test set for its fragment evaluated at the gated width, or a named structural argument; band
agreement supports only the implementation of a checker, never a verdict.** The named-argument class
shrinks as test-set theorems are found, and the two found so far each cost a page of number theory, which
prices the search as worth running per fragment the design actually ships.

## 7. What this does to the unit's standing results, named one at a time

**`84`.** Its headline defeat, polarity theorems, univariate criterion and O-J all survive my attack, and
survive it with independent instruments: six of its probes re-run identically, its arithmetic re-derived
by hand, its F2 family extended to arity 2 at genuine degree (section 3). Two corrections, stated here and
reaching the register through consolidation per `RULES.md:509-518`: F4's multivariate predicate overstates
its instrument (section 1), and O-J's procedure description carries an unnecessary difference transform
plus, in my `p4`'s inherited framing, a "no procedure outside the fragment" reading that section 6
refutes; the correct sentence is that no **modular** procedure exists outside the ring fragment. Its
least-certain items 2 and 3 are discharged (sections 2 and 3); item 1's shape, a truth stretch carried by
argument, recurs in my F4 and is flagged there identically.

**`80` section 4.3 and `81`'s "the escape".** Nothing here rehabilitates the band as a verdict carrier;
the saturating defeat closes the last direction it could have retreated to, since its two shipped
instances (wrap associativity, saturating falsity) were both in fragments where something else carries the
verdict. The band's one sound job remains validating a checker's implementation where sweeps exist, which
both `84`'s p4b and my `p4` use it for at rung 0.

**`82`.** Untouched where it matters and sharpened at one edge: its sign-uniform width-64 claim continues
to rest on its min/max structural argument, and my section 5 shows what the alternative would have been,
since its law is signed and multivariate, outside both test-set theorems. Its F17, the frontier reads the
encoding, gains the extreme companion `84` predicted: the frontier reads the procedure, and with the grid
form the encoding dimension collapses entirely for ring laws (there is no per-tuple encoding left to
read). The binding-time ladder (`82` 8.5) is unchanged; my `p4` deliberately keeps the rung-0/rung-3 split
`84` section 7 argued.

**`79`.** Its P4 composed-law result is a saturating, multivariate, mixed-operation region: outside both
procedures, unchanged, still the strongest argument that the named-argument row kind cannot be emptied.

**Op's `85` and `83`, where this file sits.** Everything here is compile time: witnesses, grid
evaluations, breakpoint searches and piece samples are all const-evaluable facts about a law at a width,
and `p2`'s crates plus `p4`'s gates are the compiled demonstrations (I15 strengthens the stakes exactly as
`84`'s addendum says: with no runtime column, the verdict mechanism's soundness story is the only one
there is). Per I16 the canon does not police shapes, and nothing here proposes mandating a construction:
the two procedures are constructions **available**, whose existence the canon may state as doability with
the audit trail carrying the proof. Per `83`, every predicate here is const-available; the
genuinely-non-const question is not reached and is not mine.

## 8. Findings, in the required predicate notation

Absence of a dimension is the strongest negative statement in the notation and is meant wherever it
appears. Threads is 1 throughout; features is `any` where stated, on `80`'s ground that these are pure
value functions with language-specified semantics.

**F1. The multivariate finite-difference criterion agrees with exhaustive sweeps at genuine per-variable
degree, and its correctness at every arity and degree is a proved theorem whose proof is in section 2.**
`policy = wrap, F = 0, ops = ring terms with integer constants, arity = 2 (per-variable degree 1..=6,
including 155 laws genuinely quadratic-plus in both variables) and arity = 3 (per-variable degree 1..=4,
47 genuinely quadratic-plus in all three), widths = 1..=7 (arity 2) and 1..=4 (arity 3) exhaustive,
threads = 1, features any`. 1,975 (law, width) pairs, zero mismatches; 25 tensor falling factorials match
the Legendre threshold prediction exactly; the understated-bound negative control produces a wrong
verdict as it must. The arity-any degree-any extent is the theorem's claim, an argument rather than a
measurement, stated as such. This corrects `84` F4's "multivariate at per-variable degree <= 2", whose
instrument contained no multivariate law of true degree above 1 (`84_probes/p4:331-336`).

**F2. The difference verdict, the grid verdict and the sweep agree everywhere all exist, and the exact
threshold equals the minimum 2-adic valuation over the degree grid's integer values.** Same predicate as
F1, with the threshold equality measured on all 225 arity-2 laws, zero mismatches. Consequences carried as
theorem: a false ring equation law has a witness inside its degree box; multivariate equation truth sets
are initial segments [1, W*]; the verdict is computable in prod(d_v + 1) map evaluations with O(1) memory.
Width-64 instances: (x)_33 (y)_33 false in 1,156 evaluations, (x)_34 (y)_34 true in 1,225 with the
width-65 falsity witness pinned to 2^64 in u128.

**F3. The grid procedure's const-eval frontier at the shipped width is a box budget, not a width bound.**
`toolchain = nightly-2026-05-28, host = aarch64-apple-darwin, policy = wrap, F = 0, law = multilinear
chain associativity (true verdict, no early exit), gated width = 64, threads = 1`. Default guard: accept
through k = 14 (16,384 grid points), refuse at k = 15 and 16 under `long_running_const_eval`. With
`#![allow(long_running_const_eval)]`: accept through k = 20 (1,048,576 points), k = 22 exceeded the
probe's own 300 s cap. Seconds are an ad-hoc quick spike; accept and refuse are the results.

**F4. A constant-free saturating threshold family exists with truth set exactly 1..=d and live clamps,
and its d = 63 member is true at every width below the shipped width and false at it.** `policy =
saturate (unsigned), F = 0, ops = {saturating mul}, arity = 1, constants none, widths = 1..=14 exhaustive
for d in 2..=12, threads = 1, features any`. Eleven members, truth set exactly 1..=d in all eleven. For
E_63: width-64 falsity by pinned witness x = 2 (2^63 against MAX); width-65 falsity in u128; widths 20,
40, 63 spot-checked at boundaries plus 1000 random points each; the stretch 15..=62 is the clamp
argument's claim, verified in every sweepable consequence, an argument rather than a measurement. The
constant member with C = 2^63 - 1 adds `embedding convention any` for its 1..=12 sweeps (true under both
wrap and clamp embedding, false at 64 with witness x = 1); the `2x == x + x` member is
convention-dependent at width 1, so no finding about a constant-carrying saturating family may omit the
embedding-convention dimension.

**F5. The model-band mechanism licenses a false saturating law at the gated width with every one of its
own checks green.** `toolchain = nightly-2026-05-28, host = aarch64-apple-darwin, policy = saturate
(unsigned), F = 0, arity = 1, model band = widths 2..=8, gated width = 64, threads = 1`. Band agreement
asserted at compile time, perturbation control refused, arm licensed, law false at 64, audit refusal in
~126 const multiplies. No band below the gated width can distinguish the closed form from the truth,
because F4 places the truth set at 1..=63.

**F6. Saturating equation truth sets have no polarity structure: gapped and interior-run truth sets exist
among constant-free terms at depth 2.** `policy = saturate (signed), F = 0, ops = {sat add, sat mul},
arity <= 2, constants none, term depth <= 2, widths = 1..=8 exhaustive by direct table comparison
(examples extended to 1..=11), threads = 1, features any`. 16 gapped and 24 interior-run truth sets among
21,945 pairs, hash-free, terms printed: including `x == x^3` true at width 2 alone and the ring identity
`sat(2x*xy) == sat(2y*x^2)` false at width 2 alone. Consequence: neither band direction transfers for
saturating laws. The unsigned same-space catalogue found only empty, full and initial-segment truth sets,
recorded as a measured regularity of that space and nothing more.

**F7. Monotone univariate saturating equation laws are decidable at the gated width by the piecewise
procedure, agreeing with exhaustive sweeps everywhere both exist.** `policy = saturate (unsigned), F = 0,
ops = {sat add, sat mul}, constants nonnegative clamp-embedded, arity = 1, terms monotone, degree <= 16
(battery) and <= 65 (family point verdicts), widths = 1..=12 exhaustive (309 pairs, 3,708 verdicts, zero
mismatches, zero failed witnesses), plus point verdicts at widths 40 and 64, threads = 1, features any`.
E_63 false at 64 with witness 2; E_64 true at 64; C-member false at 64 and true at 40. Cost O(C*W +
(C+1)*(D+1)) evaluations, counts printed by the instrument.

## 9. Fits against the register

**Kills nothing.** No option closes; nothing moves to `DROPLIST.md`.

**Q38.** O-J is strengthened and corrected in place through consolidation: the procedure needs no
difference transform (grid form, F2), its trusted degree bound reduces to a mechanisable syntactic
recursion that can only over-approximate, its frontier is the box budget F3 measures rather than an
unpriced growth, and its fragment boundary is not the boundary of decidability: a second complete
procedure exists for the monotone univariate saturating fragment (F7). The candidate general form for the
register, offered as an option refinement rather than a settlement: **O-J', a verdict row carries a
witness, or a per-fragment complete test set evaluated at the gated width, or a named structural argument;
band agreement is licensed only as an implementation check on a checker.** What would distinguish O-J'
from O-J: whether a law's fragment has a test-set theorem, which is a mathematical fact per fragment, and
two fragments now have one.

**Q40.** Route (a), lifting through a proof, gains the note that for ring laws the lifting theorem is
subsumed at any arity the box affords (F3 prices the box); route (b), structural argument, shrinks by
exactly the monotone univariate saturating class (F7) and remains load-bearing for signed, multivariate
and mixed laws (`79`'s P4, `82`'s sign-uniform law); route (c), stays swept, remains the schedule-
conditional class, outside every fragment with a test-set theorem.

**Q25 / the verdict table.** Gains the per-fragment reading of section 6 and the witness-in-the-box
unification of F2.

**Q39 and `83`.** Untouched: every predicate and every procedure here is const-available. Where my
findings sit relative to the genuinely-non-const question: nowhere; nothing here bears on it, and it is
not mine to answer.

## 10. Where this file is least certain, as a floor for whoever attacks it

1. **E_63's truth on widths 15 through 62 is an argument, not a sweep** (F4), verified at three spot
   widths on boundary-plus-random points. The clamp argument is two lines and I believe it; in this
   panel's notation the stretch is claimed on the argument's authority, exactly as `84`'s F1 stretch was,
   and an attacker who distrusts it should note the band defeat does not need it: E_12's threshold sits
   inside the swept range end to end.
2. **The piecewise procedure's monotonicity premise is argued, and validated only on the battery.** That
   every clamp indicator is upward-closed in x for monotone terms follows from each subterm being
   nondecreasing, which is an induction I have stated but not mechanised; a term shape that violates it
   would make a binary search land wrong. The 3,708-verdict battery with 3,241 falsities found no such
   shape at depth 3. The signed and multivariate extensions are not designed, and sat_sub is excluded
   even univariately.
3. **The gapped members' truth above width 11 is unmeasured.** `sat(2x*xy) == sat(2y*x^2)` is true at 3
   through 11 and I have no argument for 12 and up; if it goes false again somewhere, the shape catalogue
   understates the pathology rather than overstating it.
4. **The unsigned initial-segment regularity (F6's last sentence) is depth-2, width-8 evidence only.** A
   deeper unsigned term pair with a non-monotone truth set would not surprise me, and finding one would
   sharpen F6 rather than damage it.
5. **`p2`'s frontier is one law shape on one host.** The box budget surely varies with the per-point
   evaluation cost (my points cost ~2k multiplies each); a cheaper law buys a bigger box. The
   default-guard boundary at 16,384 points is for this term shape, and the predicate says so.
6. **Coverage is bounded.** I did not open `76`/`77` phase one bodies, `82`'s p3-series sources line by
   line, or anything before `76`; my account of `82`'s constructions is its own file's plus `84`'s, and
   inherits their errors. I did not build the const-gate form of the piecewise procedure (`p5` is a
   runtime instrument; the grid form has its const demonstration in `p2`'s crates, the piecewise form
   does not and its const-evaluability is an expectation from F3's budget arithmetic, not a demonstration).

**Not done, and cheapest next.** Mechanise the monotonicity induction of item 2 and build the piecewise
gate as a const fn, which would put F7 at the same demonstration rung as F5; run the shape catalogue at
depth 3 signed, which is where item 4 would resolve; and point both procedures at the numeral tower's
real F = 0 expressions once a design exists, where the only new machinery is the degree/monotonicity
extractors. Nothing here is priced; no bench ran.

**Nothing here settles anything.** The mode is explore. The first thing worth attacking is item 2, and
the second is the O-J' framing in section 9, which is one file's reading of what two data points make a
pattern.
