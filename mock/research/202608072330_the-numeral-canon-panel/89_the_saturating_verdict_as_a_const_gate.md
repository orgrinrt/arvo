# 89. The saturating verdict as a const gate, and the test set that turns out to be the same one

**Author lens:** Chlipala. Proof automation as infrastructure rather than polish, correct by construction
in preference to verified after the fact, and the trusted computing base named out loud. A decision
procedure whose correctness argument is a theorem beats a table somebody checked, because the theorem
survives the next law and the table does not. And a proof script that only works on the paper's running
example is not a proof script.

**Position:** eighth and last expert in the derived-algebraic-laws unit, after `76`, `77`, `79`, `80`,
`81`, `82`, op's `83`, `84`, op's `85`, `86`, and op's `87` and `88`. My assigned question is the one
`86` names in its section 10 item 6 as not built: whether the piecewise saturating procedure can be a
const gate, under the pinned toolchain, with no forbidden feature, cheap enough to carry per law, and
usable to select an arm that erases. Then, in order, whether it extends to signed and to more than one
variable.

**Probes:** ten committed artifacts in `89_probes/`, each committed with its transcript as it ran and
before this file. Every gate-shaped construction is `#![no_std]`, const fn, flat const arrays, no
`alloc`, no `Vec`, no `Box`, no `dyn`, no `TypeId`, no feature gates. The analysis instruments use
`std`, `Vec` and boxed trees as spike scaffolding, which is instrument plumbing and not design shape,
per the panel's probe discipline.

**The headline, in three sentences.** Yes, and it is far cheaper than `86` priced it, because the
piecewise machinery is unnecessary: for the monotone saturating fragment a term is exactly its integer
polynomial seen through one clamp, so the verdict at any width is agreement on the **degree box**, which
is the same test set `86` section 3 found for the ring fragment. That criterion is a const gate on the
pin, decided at width 64 in under a second with a rung-0 implementation check beside it, and it extends
to **every arity** by a proof and a 770,006-pair falsification search, which removes `86`'s univariate
perimeter. And it reaches into signed exactly as far as a declared operand window is sign-uniform, which
is not a coincidence: sign uniformity **is** the criterion's own hypothesis, so `82`'s width-64 claim,
which `84` section 6 called "the only load-bearing unmechanised thing in the construction" (`84:347`),
is now decided by a const gate rather than carried by an argument.

## 0. Gates, and coverage

**Canon gate: passes, situation two.** No canon exists. `mock/canon/` is absent and `mock/crates/` is
empty by the declared mutation order; this panel is writing the first canon. Checked against
`INTENTS.md` in full, with I13 the one RATIFIED entry and narrow on op's instruction
(`INTENTS.md:200-252`), I14 IN FORCE (`INTENTS.md:254-283`), I15, I16 and I17 STATED and read at the
source (`INTENTS.md:285-340`); and `RULES.md` in full. Op's `83`, `85`, `87` and `88` read at the source
in full. Nothing below settles anything. Where a prior answer survives my derivation I say so, which
`RULES.md:99-101` counts as a result.

**Test gate: no suite exists; the mock workspace has no members.** The substitute is the probe
discipline applied to my own instruments before anyone else's, and it is applied to `86`'s first
(section 1). Four self-checks are built in rather than assumed. `p1` asserts both proof branches are
exercised (1,439 and 8,064 cases) and runs the direct falsification test that would refute its theorem.
`p4` asserts its battery contains clamped box points (1,142 and 575 cases) and that a shrunken box
produces mismatches. `p5` asserts the inadmissible-window control still breaks the criterion, so the
control cannot silently stop controlling. `p6` puts that same assertion **inside the const gate**, so a
compile is evidence the control is live.

**Read end to end:** `INTENTS.md`, `RULES.md`, `83`, `85`, `87`, `88`, `86` in full, `84` in full,
`OPTIONS.md` Q38 through Q40 (`OPTIONS.md:1880-1953`), `DROPLIST.md:205-260`. **Read at the source in
named ranges:** `82` sections 7, 8.5, 9 heads (`82:269-380`, `82:455-480`), `79` section 2
(`79:41-102`), `80`, `79`, `81` and `82` section headings in full. **Probes opened and re-run:**
`86_probes/p5_sat_piecewise_procedure.rs` in full, rebuilt and re-run on the pin, reproducing its
transcript exactly. **Not read:** files `01` through `78`, `82`'s p1 through p12 sources line by line,
`84_probes` and `80_probes` sources, `PRIOR_CALLS.md`, `PERSONA_CALLS.md`, the seed files, everything
`OLD_`. Everything I say about `35`, `42`, `55b`, `63`, `74` and about `82`'s constructions is routed
through `82`, `84` or `86` and inherits their errors.

**One toolchain correction against my own work, recorded because it nearly shipped.** My first runs of
`p0` and `p1` executed under `1.94.0`, not the pin, because rustup resolves the toolchain from the
current directory and a probe run in a scratch directory outside the repository picks up a different
one. Both were re-run under `rustup run nightly-2026-05-28` and reproduce identically, and every probe
driver committed here pins the toolchain explicitly in the command rather than relying on the working
directory. A frontier result taken under the wrong toolchain would have been void, and nothing in the
transcript would have said so.

**Nothing here is priced.** No bench ran. Every second below is an ad-hoc quick spike with no substance
for any how-much question; the accept and refuse outcomes are the results.

## 1. The attack that had to come first: `86`'s battery cannot see the machinery it validates

`86`'s F7 (`86:404-410`) rests on 3,708 verdicts from `86_probes/p5`. Before building anything on it I
mutated the procedure and re-ran its own battery, seed and all
(`89_probes/p0_mutate_86_p5.rs`, `p0_output.txt`):

| mutant | mismatches against the sweep |
|---|---|
| M0, the procedure as `86` wrote it | 0 |
| M1, **no breakpoints at all**, one piece, D+1 samples from zero | **0** |
| M2, one sample per piece | 66 |
| M3, D samples per piece instead of D+1 | 14 |
| M4, breakpoints by linear scan capped at 64 | 0 |

3,601 of the 3,708 cases carry at least one interior breakpoint, so the battery is not degenerate in the
obvious way. It is degenerate in a way that is invisible without the mutant: **deleting the entire
breakpoint apparatus changes not one verdict**, on the battery and on the E_d family at width 64.

That is the failure class this panel keeps recording. A battery that a mutant passes has measured the
part of the procedure that could not have been wrong. `84`'s p4 caught itself in this shape and said so
(`84:41-48`); `86` guarded its `p2` against it explicitly and asserted the guard; `86`'s `p5` did not,
and F7's evidence is therefore weaker than its count suggests. F7's **verdict** is right, which section
2 establishes independently, and `86`'s file stands as written per `RULES.md:509-518`; what is wrong is
that the number 3,708 was doing work it cannot do.

The useful part is what M1 passing means. It is not that `86`'s procedure is wrong. It is that a much
simpler one gives the same answers, and the next section proves it always does.

## 2. The min-form lemma, and the criterion that falls out of it

`86` section 6 opens with the right instinct, that saturating arithmetic "is not modular arithmetic. It
is exact integer arithmetic composed with clamps" (`86:269-270`), and then builds a piecewise procedure
around locating where the clamps fire. The clamps do not need locating.

**Theorem A, the min-form lemma.** Over the monotone unsigned fragment, terms in one or more variables
built from the variables, nonnegative constants clamp-embedded, saturating add and saturating multiply,
saturating evaluation at width W equals

    eval_sat(t, x, W)  =  min( P_t(x), MAX_W )

with `P_t` the exact integer polynomial of `t` with its constants embedded. Clamping early and clamping
late coincide, including the case a clamped subterm is multiplied by zero, where both give zero. Every
`P_t` has nonnegative integer coefficients and is therefore nondecreasing in each variable on the
nonnegative domain.

**Theorem B, the degree criterion, univariate.** Let `D` be the syntactic degree bound, which for this
fragment is exact because there is no cancellation. Then

    for all x in [0, MAX]:  A(x) == B(x)    <=>    A(x) == B(x) for x in 0..=min(D, MAX)

*Proof.* One direction is trivial. Suppose agreement on `0..=D`. If neither side clamps at `D` then by
monotonicity neither clamps anywhere below it, so `P_A = P_B` at `D+1` points and both have degree at
most `D`, so `P_A` and `P_B` are the same polynomial and the two sides agree everywhere. If a side does
clamp at `D`, its value there is `MAX`, so the other side's value is `MAX` too, so both polynomials are
at least `MAX` at `D`; by monotonicity both are at least `MAX` above `D`, so both sides equal `MAX`
there, and everything at or below `D` was checked directly. QED.

**Measured rather than trusted** (`89_probes/p1_min_form_and_the_degree_criterion.rs`,
`p1_output.txt`). A battery of 731 term pairs, deeper and higher in degree than `86`'s (depth 4, degree
cap 28, large constants, near-miss pairs sharing structure, the E_d family, a constant-embedding family
and eight adversarial pairs), at widths 1 through 13:

- Theorem A compared at **every point of every domain**: 23,950,484 evaluations, **zero** where
  saturating evaluation differs from `min(exact, MAX)`.
- Theorem B against exhaustive sweeps: 9,503 verdicts, 3,887 true and 5,616 false, **zero mismatches**,
  zero returned witnesses that fail to witness.
- Both proof branches exercised: 1,439 cases with no clamp at `D`, 8,064 with one.
- **The direct falsification test.** A false law whose first witness sits strictly above `D` would refute
  Theorem B outright. Over 5,616 false cases: **zero**, with 507 cases whose first witness is exactly
  `D`, so the bound is attained and is not slack.
- Mutation controls, so the battery can fail: sampling `0..D` gives 533 mismatches, starting at one
  gives 1,647, `D+1` points spread over the domain gives 685, half the degree gives 377.

At width 64 the criterion decides `E_63` false with witness 2 and `E_64` true, the same verdicts `86`
reports, in `2*(D+1)` term evaluations: about 8.2 thousand node operations against the 516,033 and
616,999 evaluation steps `86`'s instrument printed. The gain is not the point; the disappearance of the
binary search, the piece bookkeeping and the clamp-indicator monotonicity induction is. `86`'s own
second-least-certain item is that induction (`86:449-452`), and it is not weakened, it is **unneeded**.

**And the shape is the interesting part.** `86` section 3 found that for ring laws the verdict is
evaluation on the degree grid. Theorem B says the same thing for a fragment with no ring structure at
all. Two fragments, arrived at by different arguments, one test set.

## 3. The assigned question, answered: it is a const gate, with four controls

`89_probes/p2_const_gate.rs`, driver `p2_run.sh`, transcript `p2_transcript.txt`. A term is a const
array of postfix nodes; evaluation and degree extraction are const fn stack machines over const-sized
arrays; nothing allocates, nothing dispatches, no feature gate appears. On
`nightly-2026-05-28`, `aarch64-apple-darwin`:

| build | outcome | what it demonstrates |
|---|---|---|
| default, law `E_64` | **ACCEPT** in 0.91 s | the arm is licensed by a verdict computed at width 64 |
| `--cfg use_e63` | REFUSE, `E0080` | `E_63` is false at width 64 and the gate says so, at width 64 |
| `--cfg perturb` | REFUSE, `E0080` | the rung-0 implementation check catches a criterion sampling one point short |
| `--cfg nonfragment` | REFUSE, `E0080` | the fragment-membership check catches a term carrying `sat_sub` |
| `--cfg unchecked_nonfragment` | **ACCEPT** | with that check removed the criterion licenses a law the same file proves false at width 64 |

The last row is the finding rather than a pass. The non-fragment law is
`sat_add(sat_sub(100, x), x) == 100`, which is 100 for `x <= 100` and `min(x, MAX)` above, so it agrees
with the constant on the whole degree box, which is `{0, 1}`, and diverges at `x = 101`. The file
asserts the criterion's TRUE and the law's falsity in the same crate, so a clean compile is a compiled
demonstration that **fragment membership is load-bearing rather than decorative**: violating it does not
degrade the verdict, it inverts it.

Three things the construction gets from the panel rather than inventing:

**The band is demoted to the one job `84` section 4 licenses it for.** `IMPL_CHECK` is a crate-level
const, rung 0 on `82`'s ladder (`82:455-473`), unskippable, and it validates the **checker's
implementation** against exhaustive const sweeps at widths 1 through 8 over fifteen laws. It transfers
nothing. `TRUTH_SET_SHAPE` is the band's second licensed job, checking a claimed shape rather than
exporting it.

**The licence sits at rung 3, per instantiation, and is forced.** `84` section 7 argues that split is
the correct design rather than a defect, and I agree with it having derived nothing new: a library claim
belongs where it cannot be skipped and an arm's claim quantifies over reached instantiations. The file
forces the instantiation through a top-level const so the transcript cannot be read as covering an arm
it never evaluated.

**Fragment membership is a const fn, not a comment.** `86`'s least-certain item 2 says the monotonicity
premise "is an induction I have stated but not mechanised" (`86:449-452`). Under Theorem A the induction
collapses to a syntactic property: every admitted operation is nondecreasing in both arguments on the
nonnegative domain, so every subterm is, so the exact polynomial has nonnegative coefficients. A const fn
walking the node array and rejecting anything that is not a variable, a nonnegative constant, a
saturating add or a saturating multiply is the whole of it, and `--cfg nonfragment` is what happens when
it fires.

## 4. The frontier, in accept and refuse, and it splits in two

`86`'s F3 measured the ring grid procedure's frontier as a box budget rather than a width bound. The
saturating criterion's frontier has the same shape and **separates into two independent costs**, which
matters more than either number.

**The verdict cost** is `2*(D+1)` evaluations of a term with about `D` nodes, so roughly `2*D^2` node
steps, and it does not depend on the width at all
(`89_probes/p3_frontier.py`, `p3_output.txt`, law `E_D` at width 64 with a TRUE verdict asserted so
nothing exits early):

| D | node steps | default guard | with `allow(long_running_const_eval)` |
|---|---|---|---|
| 64 | 16,640 | accept 0.7 s | not needed |
| 128 | 66,048 | accept 0.3 s | not needed |
| 256 | 263,168 | accept 1.1 s | not needed |
| 512 | 1,050,624 | accept 4.4 s | not needed |
| 1024 | 4,198,400 | **refuse**, `long_running_const_eval` | accept |
| 2048 | 16,785,408 | refuse | accept |
| 4096 | 67,125,248 | refuse | accept |

**The checker cost** is the rung-0 implementation check, and it is exponential in the band width because
it sweeps. The number that matters is that it is paid **once for a library rather than once per law**:
it validates the checker, and the checker is one piece of code
(`89_probes/p3b_checker_frontier.py`, `p3b_output.txt`):

| laws | band width | sweep points | default guard | with allow |
|---|---|---|---|---|
| 4 | 8 | 1,024 | accept 0.9 s | not needed |
| 4 | 10 | 4,096 | accept 1.8 s | not needed |
| 4 | 12 | 16,384 | accept 7.9 s | not needed |
| 4 | 14 | 65,536 | **refuse** | accept 36.3 s |
| 4 | 15 | 131,072 | refuse | accept 75.5 s |
| 4 | 16 | 262,144 | refuse | accept 160.5 s |
| 4 | 17 | 524,288 | refuse | accept 356.0 s |
| 8 | 14 | 131,072 | refuse | accept 89.3 s |
| 16 | 12 | 65,536 | refuse | accept 45.4 s |
| 1 | 18 | 262,144 | refuse | accept 186.5 s |

**That table's first version measured nothing, and the defect is committed rather than deleted**
(`89_probes/NOTE_p3_checker_half_first_run.md`). The original swept `E_d` for small `d` at widths above
`d`, where the law is false, so both the verdict and the sweep exited at the first witness and a row
claiming 65,536 sweep points accepted in 0.4 s. The corrected instrument asserts inside the generated
crate that every swept law is true at every width in the band, so nothing can exit early. Same shape as
`84`'s p4 and `86`'s p2; third instance in the unit; it costs one line to guard and it is invisible in a
transcript that does not.

**The arity cost** is the box, `2^n` for a multilinear law, measured on the signed window gate of section
6 (`89_probes/p7_arity_frontier.py`, `p7_output.txt`):

| n | box points | default guard | with allow |
|---|---|---|---|
| 8 | 256 | accept 0.4 s | not needed |
| 12 | 4,096 | accept 1.3 s | not needed |
| 14 | 16,384 | accept 4.8 s | not needed |
| 15 | 32,768 | **refuse** | accept 11.6 s |
| 16 | 65,536 | refuse | accept 24.8 s |
| 18 | 262,144 | refuse | accept 103.7 s |

**That is `86`'s F3 boundary to the point**, on a different fragment, a different law shape and a
cheaper per-point cost: default accepts through 16,384 box points and refuses at 32,768, and the allow
buys the same order it bought there. `86`'s least-certain item 5 says its box budget is "one law shape
on one host" and that a cheaper law buys a bigger box (`86:459-461`). A cheaper law bought the **same**
box, which sharpens item 5 in the direction of the budget being a step count rather than a per-point
cost, and is one more instance of the observation the droplist already carries about the nine-bit wall
(`DROPLIST.md:234-235`).

## 5. Theorem C: the criterion holds at every arity, and `86`'s univariate perimeter comes off

`86`'s F7 perimeter reads "univariate", with the reason that "multivariate pieces are regions rather
than intervals" (`86:290-292`). That is true of the piecewise procedure and false of the fragment.

**Theorem C.** For terms over `k` variables in the monotone fragment, with per-variable degree bounds
`d_i`, over any domain `[LO, HI]^k` on which the fragment's hypotheses hold, agreement on the box
`prod_i {LO..min(LO+d_i, HI)}` implies agreement on the whole domain.

*Proof.* By Theorem A each side is `min(P, MAX)` with `P` nondecreasing in each variable. Let
`Delta = P_A - P_B` and let `K` be the set of box points at which **both** sides clamp. `K` is an up-set
in the box, because clamping survives increasing any coordinate. Agreement on the box makes `Delta`
vanish on `box \ K`, which is a down-set. Expand `Delta` in the tensor falling-factorial basis
`e_J = prod_i (x_i)_{j_i}`, a basis for polynomials of per-variable degree at most `d_i`; evaluation in
that basis is triangular with respect to the coordinatewise order, since `e_J(J') = 0` unless
`J' >= J`. Processing `J` in increasing order, every `J` outside `K` has all its predecessors outside
`K` too, so its coefficient is zero. Hence `Delta = sum_{J in K} lambda_J e_J`. Now take any `z` with
`Delta(z) != 0`. Some `J in K` has `prod_i (z_i)_{j_i} != 0`, which for nonnegative integers forces
`z_i >= j_i` for every `i`, that is `z >= J`. Both sides clamp at `J`, so by monotonicity both clamp at
`z`, so both equal `MAX` and agree there anyway. QED.

Theorem B is `k = 1` of this, and the falling-factorial triangularity is the same instrument `86`
section 2 uses for the ring fragment. **The two fragments do not merely share a test set, they share the
argument for it**, with divisibility by `2^W` in one and clamping at `MAX` in the other playing the same
role: a condition preserved along the coordinatewise order.

**Measured** (`89_probes/p4_multivariate_box.rs`, `p4_output.txt`):

- `k = 2`, widths 1 through 5, exhaustive over the full square: 2,030 verdicts, **zero mismatches**,
  1,142 cases with a clamped box point so the interesting branch is entered, and the mutation control
  that shrinks the box by one per coordinate produces 691 mismatches.
- `k = 3`, widths 1 through 4, exhaustive over the full cube: 976 verdicts, **zero mismatches**, 575
  clamped cases, control 345 mismatches.
- **The direct falsification search.** Every pair of terms built to depth 2 over two variables with
  small constants, at widths 2 and 3, deduped and filtered by degree: **770,006 pairs checked, zero true
  on the box and false in the domain.** A counterexample to Theorem C is exactly that shape, and there
  is none.

The structured members include the pair my hand analysis suggested was the dangerous shape, `x*y + y`
against `x*x*y + y` at width 2, where the difference is supported precisely on the corner's up-set. It
holds, and the proof says why it must.

## 6. Signed: the reach is the window, and sign uniformity is the criterion's own hypothesis

Theorem C's hypotheses are properties of the **domain**, not of the type's signedness. On a declared
operand window `[LO, HI]` with `LO >= 0`, substituting `y_i = x_i - LO` keeps every coefficient
nonnegative, only the ceiling is reachable, so the clamped set is an up-set and the theorem applies. On
a window with `HI <= 0`, negating gives the same fragment with the floor's magnitude as the ceiling, for
additive terms; a product of two negatives leaves the window, so the non-positive half is stated for
`sat_add` only. On a **straddling** window both clamps are reachable, the floor-clamped set is a
down-set, the triangular argument loses its direction, and the criterion has no hypothesis.

So `82`'s measured predicate, a declared operand interval with `LO >= 0` or `HI <= 0` (`82:281-285`), is
exactly the condition under which Theorem C's hypotheses hold. That is a proof-shaped account of a
predicate `82` established by exhaustive search over every interval and every subset at widths 2 through
6 and could not carry past them except by a structural argument.

**Measured** (`89_probes/p5_signed_windows.rs`, `p5_output.txt`), over every declared window of the
representable set at widths 2 through 5, `k = 2`, 123 term pairs:

| | checks | mismatches against brute force |
|---|---|---|
| admissible (sign-uniform) windows | 25,120 | **0** |
| inadmissible (straddling) windows | 62,210 | **3,808** |

The second row is the point. The criterion is **wrong** on straddling windows, at a rate no reader would
call an edge case, which is what makes the window predicate load-bearing rather than a stylistic caveat.

**And `82`'s law is decided at the shipped width, in the compiler**
(`89_probes/p6_signed_window_gate.rs`, driver `p6_run.sh`, transcript `p6_transcript.txt`):

| build | outcome |
|---|---|
| default, window `[0, MAX]`, `n = 8`, left fold against right fold and against the balanced tree | **ACCEPT** in 0.50 s |
| `--cfg straddle`, window `[MIN, MAX]` | REFUSE, `E0080`, at the hypothesis check |
| `--cfg unchecked_straddle`, hypothesis check removed | **ACCEPT** |

The last row licenses fold reassociation on a straddling window while the same file asserts, at compile
time, that the law is false at width 64 with witness `(MAX, MAX, MIN)`. That is `82`'s straddling
divergence reproduced as a compiled consequence of dropping the hypothesis rather than as a table of
percentages. The rung-0 check additionally asserts that at widths 2 through 4 brute force says false on
a straddling window while the criterion says true, so the control cannot quietly stop controlling.

**What this does to the unit's standing account of `82`.** `84` section 6 relocated `82`'s width-64
soundness from the band to "the three-line integer argument", named it a trusted-base item until
mechanised, and called it the only load-bearing unmechanised thing in the construction (`84:344-347`).
`86` agreed and, on its own perimeter, placed the law outside both procedures and left it "on the
named-argument route" (`86:292-293`). That placement was correct for the piecewise procedure and is
wrong for the fragment: the law is multilinear over a sign-uniform window, so the box is `2^n`, and the
verdict is computed at width 64. **The named-argument row for this law can be replaced by a procedure
row**, which is the first time in this unit a law has moved in that direction.

I want to be exact about what has and has not moved. `82`'s **necessity** direction, that straddling
windows are not associative, remains `82`'s measurement and is not re-derived here; my straddling
control reproduces it at three widths and at width 64 on one witness, which is corroboration and not a
proof. What is new is that the **sufficiency** direction is a decision rather than an argument.

## 7. The wall: general signed, with the mechanism named and two routes closed

Everything in sections 2, 5 and 6 rests on one structural fact, that a term is its polynomial seen
through one clamp. `86` measured that signed saturating truth sets are gapped and interior-run
(`86:396-402`) and reported it without a mechanism. The mechanism is the failure of that fact, and it is
one line of arithmetic (`89_probes/p8_why_signed_is_gapped.rs`, `p8_output.txt`). At width 4, where
`MAX = 7`:

    sat_add(sat_add(7, 7), -7)      clamping at every node   = 0
    clamp(7 + 7 - 7)                clamping once at the root = 7

The ceiling clamp discards magnitude that a later negative operand would have restored. Once that
happens the term is not a clamped polynomial, the interpolation argument has nothing to interpolate, and
the truth set in width can do anything.

Measured over the constant-free two-variable depth-2 term space, deduped to 69 terms:

| domain | terms where clamping early equals clamping late | terms where it fails |
|---|---|---|
| full signed `[MIN, MAX]` | 37 | 32 |
| non-negative `[0, MAX]` | **69** | **0** |
| non-positive `[MIN, 0]` | 50 | 19 |

identical at widths 3, 4 and 5. And the pathology and the fragment boundary coincide rather than merely
coexist: of 2,346 term pairs, 8 have a non-monotone truth set over widths 1 through 8, and **zero** of
those 8 have both sides inside the min-form class at every width. The instrument asserts that count is
zero, so a single counterexample would have failed the probe.

Two routes out of the general signed case were attacked and closed with counterexamples
(`89_probes/p9_two_routes_closed.rs`, `p9_output.txt`), so neither is left as a hunch:

**Route A, does the criterion survive a fraction width.** Every finding in this unit carries `F = 0`,
which in this panel's notation means nothing was measured above it. It is stronger than that here: a
fixed-point saturating multiply carries a right shift, a shift is nondecreasing so the **monotonicity**
half of the hypothesis survives intact, and the **polynomial** half does not. `(x >> 2) * 4` against
`(x >> 1) * 2` both have degree bound 1, so the box is `{0, 1}`, they agree there, and they differ at
`x = 2` at every width from 3 to 8. Over a shift-carrying term space at widths 4 through 6, **82,002 of
417,384 pairs are true on the box and false in the domain**. `F = 0` is a hard boundary of this
criterion, with a two-point witness, not an untested caution.

**Route B, a saturation radius for the general signed case.** A term of degree `D` saturates once `|x|`
exceeds about `MAX^(1/D)`, so a sweep of `[-R, R]` plus a constant-tail argument would decide the law in
`2R+1` evaluations, inside the const budget as soon as `D >= 4`. The tails are not constant:
`sat_add(sat_mul(x,x), x)` on the negative tail has the square clamp to `MAX` and the surviving linear
term keep moving, giving `-1, 0, 1, 2, 3, 4` at `MIN..MIN+5` at widths 8 and 10. What survives after the
top-degree nodes clamp is a lower-degree remainder, which is the piecewise structure again. The cheap
form of the route is dead. **The expensive form is open and unbuilt**: exact coefficient tracking with
root isolation to locate clamp boundaries, by derivative recursion into monotone intervals, which is
implementable in const fn with fixed-size arrays and needs multi-limb coefficients past degree 3 because
the exact value at `x ~ 2^63` leaves `i128`. I did not build it and I am not confident it is worth
building before a design names a signed high-degree law it needs.

## 8. What is in the trusted base after the gate has run

The list, itemised in the shape `84` section 3 established, because a mechanism's value is measured by
how short and how explicit this list is:

1. **Fragment membership.** Checked by a const fn over the term's own node array, not claimed. Shown
   live: `--cfg nonfragment` refuses, and `--cfg unchecked_nonfragment` licenses a false law. This is a
   syntactic property of the law's spelling.
2. **The window hypothesis**, for the signed case. Checked by a const fn over the declared bounds.
   Shown live the same way in `p6`.
3. **The degree bound.** For this fragment it is exact rather than an over-approximation, and an
   over-approximation would be safe anyway (extra box rows agree by construction). An **under**-stated
   bound is unsafe and is shown live by `--cfg perturb`.
4. **The encoding of the law into the term array.** Whether the postfix nodes say what the design meant.
   Unavoidable, shared with every approach in this unit including the swept one, and the only item on
   this list with no mechanical check behind it.
5. **rustc's const evaluator, the pin, and the host**, plus the guard's time budget, which makes the
   band width in item 6 host-relative.
6. **The checker's implementation**, validated at rung 0 against const sweeps over a band. This is the
   band doing the only job `84` section 4 licenses, and it is paid once per library rather than per law.

What is **not** on the list: the transfer. There is none. The verdict is computed at the width that
ships, and the band exports nothing. That is the property `84` section 5 was after and it now holds for
two fragments rather than one.

`F = 0` is not on the list either; it is a **precondition**, refuted outside by section 7's route A, and
a design that wants a verdict at `F > 0` needs a different theorem rather than a wider band.

## 9. Findings, in the required predicate notation

Absence of a dimension is the strongest negative statement in the notation and is meant wherever it
appears. Threads is 1 throughout. Features is `any` where stated, on `80`'s ground that these are pure
value functions with language-specified semantics; the const-gate findings carry toolchain, host and
encoding because they are compile-time frontier results.

**F1. Over the monotone unsigned saturating fragment, saturating evaluation equals the exact integer
polynomial clamped once.** `policy = saturate (unsigned), F = 0, ops = {sat add, sat mul}, constants
nonnegative clamp-embedded, arity = 1, degree = 0..=28, widths = 1..=13 exhaustive at every point,
threads = 1, features any`. 23,950,484 evaluations compared, zero disagreements.

**F2. The degree criterion decides the monotone unsigned saturating fragment at any width, univariate,
in 2*(D+1) evaluations.** Same predicate as F1, plus `point verdicts at width 64`. 9,503 verdicts
against exhaustive sweeps, zero mismatches, zero failed witnesses, both proof branches exercised at
1,439 and 8,064 cases, 507 cases whose first witness is exactly `D` so the bound is attained. Direct
falsification test: zero false cases with a first witness above `D`. Four mutation controls each produce
mismatches. Width-64 verdicts: `E_63` false with witness 2, `E_64` true, the constant member false with
witness 1 and true at width 40. **This corrects `86`'s F7 in its own deliverable per
`RULES.md:509-518`**: the breakpoint search, the piece bookkeeping and the clamp-indicator monotonicity
induction are unnecessary, and `86`'s battery cannot distinguish the procedure that needs them from the
one that does not (89 section 1).

**F3. The criterion holds at every arity, over any admissible domain, with the degree box as the test
set.** `policy = saturate (unsigned), F = 0, ops = {sat add, sat mul}, constants nonnegative
clamp-embedded, arity = 2 and 3 measured (per-variable degree 1..=4), widths = 1..=5 (arity 2) and
1..=4 (arity 3) exhaustive, threads = 1, features any`. 3,006 verdicts, zero mismatches; box-shrinking
mutation controls produce 691 and 345 mismatches; 1,142 and 575 cases exercise the clamped-set branch.
Direct falsification search over 770,006 term pairs at widths 2 and 3: zero true on the box and false in
the domain. The arity-any extent is the theorem's claim, an argument rather than a measurement, stated
as such, with the proof in section 5. **This removes the univariate perimeter of `86`'s F7.**

**F4. The saturating verdict is a const gate on the pinned toolchain, with fragment membership checked
rather than claimed, and violating it inverts the verdict rather than degrading it.** `toolchain =
nightly-2026-05-28, host = aarch64-apple-darwin, encoding = postfix const array with a const fn stack
machine over u128 node values, policy = saturate (unsigned), F = 0, arity = 1, gated width = 64, band =
widths 1..=8 at rung 0, threads = 1`. Default accepts; `use_e63` refuses at the licence; `perturb`
refuses at the rung-0 implementation check; `nonfragment` refuses at the fragment check;
`unchecked_nonfragment` accepts, licensing a law the same crate proves false at width 64 with witness
101. No `alloc`, no `dyn`, no `TypeId`, no feature gates.

**F5. The gate's frontier splits into a per-law verdict cost independent of width and a per-library
checker cost exponential in the band.** `toolchain = nightly-2026-05-28, host = aarch64-apple-darwin,
encoding as F4, gated width = 64, threads = 1`. Verdict: default accepts through degree 512 (about 1.05
million node steps), refuses at degree 1024 under `long_running_const_eval`, accepts through degree 4096
with the allow. Checker: default accepts 16,384 sweep points and refuses 65,536; with the allow, 524,288
accepted. Seconds are an ad-hoc quick spike with no substance.

**F6. The arity frontier of the box gate is 16,384 box points by default and at least 65,536 with the
allow, on a fragment and law shape different from `86`'s F3 and with a cheaper per-point cost.**
`toolchain = nightly-2026-05-28, host = aarch64-apple-darwin, encoding as F4, policy = saturate
(signed), F = 0, ops = {sat add}, window = [0, MAX] declared, law = fold reassociation, gated width =
64, threads = 1`. Accept at n = 14, refuse-guard at n = 15, accept with the allow at n = 16. Matching
`86`'s F3 boundary point for point is evidence the budget is a step count rather than a per-point cost.

**F7. Sign uniformity of a declared operand window is the box criterion's own hypothesis, and the
criterion is wrong outside it.** `policy = saturate (signed), F = 0, ops = {sat add, sat mul} for
non-negative windows and {sat add} for non-positive windows, arity = 2 measured, every declared window
of the representable set, widths = 2..=5 exhaustive, threads = 1, features any`. 25,120 checks on
admissible windows with zero mismatches; 62,210 checks on straddling windows with 3,808 mismatches.

**F8. `82`'s sign-uniform fold reassociation is decided at width 64 by a const gate, with no band and no
transfer.** `toolchain = nightly-2026-05-28, host = aarch64-apple-darwin, encoding = postfix const array
with a const fn stack machine over i128 node values, policy = saturate (signed), F = 0, ops = {sat add},
window = [0, MAX] declared, arity = 8, laws = left fold against right fold and against the balanced tree,
gated width = 64, band = widths 2..=4 at rung 0, threads = 1`. Default accepts in 0.50 s; `straddle`
refuses at the hypothesis check; `unchecked_straddle` accepts and licenses a law the same crate proves
false at width 64 with witness `(MAX, MAX, MIN)`.

**F9. The mechanism behind gapped signed truth sets is the failure of the min-form property, and the
pathology coincides with the fragment boundary rather than merely coexisting with it.** `policy =
saturate (signed), F = 0, ops = {sat add, sat mul}, constants none, arity = 2, term depth <= 2, widths =
3..=5 for the min-form counts and 1..=8 for the shape catalogue, exhaustive, threads = 1, features any`.
Minimal failure at width 4 with a three-node constant term. 69 deduped terms: 37 min-form on the full
signed domain, 69 on the non-negative domain, 50 on the non-positive domain. 2,346 pairs, 8 with a
non-monotone truth set, zero of which have both sides min-form at every width.

**F10. The box criterion is false at F > 0, with a two-point witness, because a shift makes the value
non-polynomial while leaving it monotone.** `policy = saturate (unsigned), F > 0 realised as a right
shift by a constant, ops = {sat add, sat mul, shr by a constant}, constants nonnegative, arity = 1,
widths = 3..=8 exhaustive, threads = 1, features any`. `(x >> 2) * 4 == (x >> 1) * 2` is true on its
degree box `{0, 1}` and false at `x = 2` at every width measured. Over a shift-carrying term space at
widths 4..=6, 82,002 of 417,384 pairs are true on the box and false in the domain.

**F11. A saturation-radius procedure for the general signed case does not work in its cheap form.**
`policy = saturate (signed), F = 0, ops = {sat add, sat mul}, arity = 1, degree = 2, widths = 8 and 10,
threads = 1, features any`. `sat_mul(x,x) + x` is not constant on the negative tail; its values at
`MIN..MIN+5` are `-1, 0, 1, 2, 3, 4`.

## 10. Fits against the register

**Kills nothing outright.** Nothing moves to `DROPLIST.md` as an option. Two routes internal to this
file are closed with their diagnostics in section 7 and belong there if the consolidation wants them:
the saturation-radius shortcut for general signed laws, closed by a non-constant tail; and any reading
of the box criterion as reaching `F > 0`, closed by a two-point witness at 19.6 percent of a term space.

**Q38** (`OPTIONS.md:1880-1906`). O-J is strengthened and its boundary moves. `84`'s O-J is stated for
"laws whose two sides are ring terms over the wrapping fragment" (`84:362-370`); the same procedure
shape, evaluation on the degree box, now covers a second fragment with no ring structure, at every
arity, with the same falling-factorial argument. The candidate general form is not a new option but a
correction to O-J's own scope sentence.

**On `86`'s O-J' framing, and I flag it as the thing I most want attacked.** `86` section 9 offers
"a verdict row carries a witness, or a per-fragment complete test set evaluated at the gated width, or a
named structural argument" (`86:421-423`), with the honest note that it is "one file's reading of what
two data points make a pattern" (`86:475-476`). Within one dispatch the second data point turned out to
be the first one wearing different clothes, and a third region moved from the argument kind to the
procedure kind (section 6). A closed enumeration of evidence kinds is the shape op has rejected three
times in this sitting (`88:118-123`), and the corrective there applies here: the useful sentence is that
a verdict names its evidence and where it was computed, not that the kinds are three.

**Q39** (`OPTIONS.md:1907-1932`) and `83`. Untouched. Every predicate in this file is const-available:
fragment membership, window bounds, degree, and the verdict itself are all const expressions over the
typestate and over const data. Where my findings sit relative to the genuinely-non-const question `83`
leaves open: **nowhere**. Nothing here bears on it and it is not mine to answer.

**Q40** (`OPTIONS.md:1933-1953`). Route (b), the structural argument, **shrinks by one named law**, and
it is the law `84` and `86` both identified as the residual argument in the unit. `82`'s sign-uniform
fold moves from (b) to the procedure kind. Route (a), lifting through a proof, is unaffected. Route (c),
stays swept, is unaffected: the schedule-conditional class carries rounding, which is section 7's route
A boundary, so it is outside every test-set theorem this unit has.

**Q25 and the verdict table.** Gains the finding that a law can move between row kinds when a theorem is
found, which is a fact about the table's rows being provisional rather than about any law.

## 11. What the consolidation must not lose, and what is genuinely open

I am the last file before this topic's consolidation, and `RULES.md:189-210` says a consolidation drops
live options structurally, twice observed. The items below are the ones with nothing attached to grip.

**Must not be lost.**

1. **The two fragments share one test set and one argument.** Ring laws (`86` section 3) and monotone
   saturating laws (sections 2 and 5) are both decided by evaluation on the degree box, by the same
   falling-factorial triangularity, with divisibility and clamping playing the same role. That is the
   most compressible sentence this unit has produced and it is the one a canon would want, as intent:
   **a law verdict at a shipped width is computable there, on a test set determined by the law's degree,
   for fragments where the failure condition is preserved along the coordinatewise order.**
2. **The verdict transfers nothing and needs no band.** The band's only job is validating a checker's
   implementation, and that cost is per library rather than per law. `84` established the first half;
   section 4's split establishes the second.
3. **`82`'s width-64 claim is no longer an argument.** `84:344-347` named it the only load-bearing
   unmechanised item in that construction and `86:292-293` left it there. It is a const gate now, and if
   the consolidation carries the older sentence the unit will look less finished than it is.
4. **Sign uniformity is the criterion's hypothesis, not a coincidence.** A consolidation that records
   `82`'s predicate as a measured regularity loses the reason it is exactly that predicate.
5. **`F = 0` is a boundary with a witness**, not an untested edge. Section 7 route A.
6. **Fragment membership and the window hypothesis invert the verdict when violated.** Both are shown
   live by builds that accept. A consolidation that lists them as "trusted inputs" without that fact
   understates what they cost.
7. **The defect in section 1.** Not because `86` is wrong, but because it is the third instance in this
   unit of a battery that cannot fail, and the panel's own record of that pattern is what makes the next
   one catchable.

**Genuinely open, and I did not close them.**

- **General signed with both clamps reachable.** No procedure. The expensive route is named in section 7
  with its cost; nobody has built it, and nothing says a design needs it.
- **Any fraction width.** Refuted for this criterion. Whether another theorem exists for
  clamp-and-shift terms is untouched.
- **`79`'s P4** (`79:41-102`). Saturating, multivariate, mixed-operation with `sat_sub`, over a
  straddling domain. Outside every fragment in this unit, unchanged, and still the strongest evidence
  that the named-argument kind cannot be emptied.
- **Whether any of this reaches the numeral tower's real expressions.** `mock/crates/` is empty, so the
  question has no object yet. The machinery a design would need is a degree extractor and a fragment
  checker over whatever its expression form turns out to be, both syntactic.
- **Pricing.** Nothing in this unit has run on the bench harness, and `87:83-87` says so plainly. Every
  second in this file is an ad-hoc quick spike.

## 12. Where this file is least certain, as a floor for whoever attacks it

1. **Theorem C's arity-any claim is a proof, measured at arities 2 and 3.** The falsification search is
   large but it is at two widths, arity 2, depth 2. A wrong generalisation would surface first at higher
   per-variable degree with a large clamped set, which the 770,006-pair search reaches only partly. This
   is the same shape as `84`'s least-certain item 2 and `86`'s correction of it, and I flag it in the
   same terms rather than hoping the pattern breaks here.
2. **The min-form lemma's `HUGE` guard.** `p1`'s exact evaluation saturates at `u128::MAX`, sound for
   comparison against `MAX_W` because the fragment is nonnegative and monotone. That soundness is an
   argument, not a measurement, and a term whose exact value first exceeds `u128::MAX` and is then
   multiplied by zero is the shape that would test it; the fragment makes that harmless and I have not
   built the case.
3. **The non-positive window is stated for additive terms only** and is measured as part of `p5`'s
   window sweep rather than singled out. The negation argument for it is three lines and unmechanised.
4. **`p8`'s term space is depth 2 and constant-free**, reconstructed to be comparable with `86`'s
   catalogue rather than identical to it. Its 8 non-monotone pairs are not `86`'s 16 gapped plus 24
   interior-run, because the spaces differ; the claim I make from it is the coincidence of pathology and
   fragment boundary within my own space, not a reproduction of `86`'s counts.
5. **The frontier tables are one host, one encoding, one law shape per table.** F5 and F6 say so in
   their predicates. The verdict table's cost model assumes the per-point cost is the term size, which
   the D = 512 to 1024 boundary is consistent with and does not prove.
6. **Coverage is bounded.** I did not open `76`, `77`, `79` beyond section 2, `80`, `81`, or anything
   before them; I did not open `82`'s or `84`'s probe sources; my account of `82`'s constructions is
   `82`'s own file plus `84`'s and `86`'s and inherits their errors. I re-ran `86`'s `p5` and no other
   prior probe.

**Not done, and cheapest next.** Point the criterion at a real expression form once a design exists,
where the only new machinery is the degree extractor and the fragment checker. Build the arity-4 and
higher falsification search for Theorem C, which is the cheapest attack on item 1. And run the shape
catalogue at depth 3 signed, which `86`'s own least-certain item 4 also names.

**Nothing here settles anything.** The mode is explore. The first thing worth attacking is item 1, and
the second is the O-J' framing in section 10, which I think is a snapshot being written down as a law.
