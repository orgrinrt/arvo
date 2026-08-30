# 58. The fraction boundary

**Persona:** Bart Wronski. Signal processing, fixed-point arithmetic in shipped image and rendering
pipelines, sampling and quantisation error as a frequency-domain problem rather than a footnote.

**Date:** 2026-08-09. **Position:** file four of unit two on the format-concept topic, after `55`
derived the concept cold, `56` attacked it, `55b` replied, and `57` adjudicated the refutation and
second-read the grading. Last expert before the checkpoint.

**Probes:** `58_probes/`, two of them, sources and outputs committed. `p1` answers `57` section 7's
own flagged open question (does rounding mode rescue the semiring). `p2` asks a question nobody in
this unit had posed: does the multiplicative fold's accumulator have a grading shaped like the
additive one `57_probes/p6` measured. Its first run had a bug, kept on disk per this panel's
discipline, and the bug turned into the second of the two probes' real findings.

**Re-run before argued with.** `57_probes/p2`, `p3` and `p4` rebuilt on `nightly-2026-05-28` and
diffed against their committed outputs, byte-identical (`58_probes/RUN.md`). Every count from those
three files that this file uses is a count I regenerated myself.

**Read for this file:** `00_brief`, `INTENTS`, `57` in full, `42` in full with `42_probes/p3`,
`55` phase one and phase two in full, `55b` in full, `55_probes/p4` source, `57_probes/p2`, `p3`
and `p4` sources and outputs, `56` sections 3.3 and 9 (opened directly, not through `57`'s account),
and `OPTIONS.md`'s Q5, Q6, Q11, Q12 and Q14 entries in full. **Not opened:** `56` outside the two
cited sections, `35`, `18`, `20`, `25`, `40`, `43`, `DROPLIST.md`, `seed/`, `archive/`.

## Status of this file

Nothing here settles anything, per the standing explore mode. What it does: confirms the additive
half of `57`'s job one is unconditional at every fraction width rather than merely surviving it,
shows the multiplicative half's sufficiency proof cannot be instantiated for a real fixed-width
multiply chain when F > 0 (not "untested", structurally unsatisfiable, and says exactly which
premise fails), answers `57`'s own flagged rounding-mode question and finds it does not move the
verdict, corrects the reading of `57_probes/p4`'s "clamp only" arm (it is F = 0 relabelled, not an
ablation at F > 0, and I explain why no such ablation can exist), and reports a new measurement that
the multiplicative fold's accumulator grade does not collapse the way the additive one does: it
saves exactly one rescale's width, constant in fold length, and needs the rest of its full
precision regardless of how long the chain runs. One of my own probe's first assertions was wrong
and the run that proved it wrong is kept on disk, per this panel's discipline, because the failure
is what taught me the second finding.

## 0. Gates

**Canon gate: passes, in the second of `expert-dispatch-defends-the-canon.md`'s three situations.**
There is no ratified canon. `00_brief.md:8-9` states the panel is writing one; `INTENTS.md`'s own
header records that no entry currently holds the ratified rung. Nothing here closes a question;
where I disagree with a prior file's framing I say so and put the disagreement to that file's
author for the resumption, per the dispatch.

What I checked the work against: `INTENTS.md` (I1: the strategy set is open; I7: Precise is
accurate "especially within chains and ops, not only alone", which is the intent my job-two
finding bears on most directly), the acceptance criterion at `00_brief.md:144-146`, and the
forbidden-feature list at `00_brief.md:158-160`. Both probes are plain integer and fixed-point
arithmetic in `i64`/`i128`, no type-level machinery, no feature gate of any kind: `grep -c
'^#!\[feature' 58_probes/*.rs` returns 0 on both files.

**Test gate: no suite to audit.** `mock/crates` exists as an empty directory (`ls -la mock/crates`
shows only `.` and `..`), which the panel instructed. The substitute is the probe discipline: both
probes carry instrument validation that must observe both truths (a checker that could not fail is
worthless), and one asserted a false claim about its own construction on its first run, which is on
disk rather than silently corrected.

**Repository state.** `git status --porcelain` shows only this file and `58_probes/` as untracked;
the `docs/` deletions and modified bench artifacts `57` flagged at its own section 0 are absent
from the current tree (a later commit, `dfe45a90`, landed them), so that finding is resolved and I
have nothing further to add to it.

## 1. The answer, before the working

**The additive half of job one is not "F == 0 bounded". It is F-independent by construction, and
that is checkable by inspection rather than by sweep.** `57_probes/p3.rs`'s and
`p4_which_factor_breaks_and_what_coherence_buys.rs`'s own `add` functions never read the scale
parameter: `p3.rs:109` is `let add = |a: i64, b: i64| reduce(p, m, a + b);`, and `p4.rs:82-87`'s
three `Factor` arms for addition are `(a+b).clamp(0,self.m)`, `a+b` and `(a+b).clamp(0,self.m)`,
none consulting `f`. Two format elements at the same fractional scale sum exactly at that scale;
there is nothing for a fractional bit count to do. So `57`'s "the additive commutative monoid
survives every scale" (`57:362`) is not a result that happened to come out clean at F = 1, 2, 3;
it is the same computation as F = 0, verbatim, for every F, and the sign-confined-interval
corollary (`57` section 2.6, the clean "associative iff `lo == 0` or `hi == 0`" result) inherits
that unconditionally. **The additive half of job one needs no second read at any fraction width.
It has already had it, by the width sweep already run, because there was never a second computation
to run.**

**The multiplicative half is a different kind of boundary than "not yet swept", and the difference
matters for what a canon may say.** Job one's sufficiency proof (`57:222-223`, "if it holds, both
association orders equal `rho(a+b+c)`") requires the ambient operation, before reduction acts, to
be *exactly* associative. For addition that premise is free: raw integer addition is exactly
associative and reduction never touches the scale. For multiplication at F > 0, there is no fixed-
width operation that plays this role. Every pairwise fixed-point multiply already discards
information (the rescale by F bits) before any range clamp is even consulted, so the "op" available
to plug into the theorem is never the exact ambient operation; it is already a reduction wearing
op's clothes. Section 2 works this out precisely, including the one configuration where the
premise *can* be restored (unbounded intermediate precision, narrowed once at the very end), which
recovers absorption exactly and costs a fold-length-growing accumulator with no small closed form.

**The semiring's F == 0 boundary is real, structural, and immune to the one obvious escape.**
Section 3 shows round-to-nearest changes the magnitude of the violation counts and never their
existence (`58_probes/p1`), which is what fixed-point and floating-point numerical practice already
knows about rounding-induced non-associativity and is worth having on the record for this panel
rather than assumed from outside it. Section 3 also corrects the reading of `57_probes/p4`'s
"clamp only" arm: it is literally F = 0 relabelled, not a controlled test of range-clamping *at*
F > 0, because no such test can exist, for the same structural reason section 2 gives. What the
arm actually shows, correctly, is that coarsening is *sufficient* to break the algebra with no
clamp present at all; it does not and cannot show that clamping is innocent at F > 0, and `57`'s
own masking observation (the composite fails *less* than coarsening alone, `57:399-403`) is a
symptom of the two mechanisms interacting rather than one of them being cleared.

**Job two's grading does not generalise from addition to multiplication, and the shape of the
failure is itself informative.** `58_probes/p2` measures that a multiplicative fold's accumulator
saves exactly one rescale's width (F bits) below full precision, constant in fold length, and needs
the rest regardless of how long the chain runs: linear growth with a fixed, non-collapsing slope,
against addition's near-total, fold-length-independent savings. `Q11`'s third option, "the
accumulator is derivable as the width plus the log of the capacity," has no multiplicative analogue
at all; there is no closed form of that shape for a product chain, and a canon that stated Q11's
accumulator relation without saying it is an additive-only fact would license an implementation
strategy for `Precise` chains (I7) that does not exist.

## 2. Job one: what survives, and why the multiplicative half is not merely untested

### 2.1 The sufficiency proof's hidden premise

`57`'s absorption theorem, restated precisely: a total reduction `rho` is absorbing over an
operand set B when `rho(rho(x) op y) == rho(x op y)` for every reachable exact value `x` of `op`
over B and every `y` in B. Sufficiency: if absorption holds, then for any triple, `rho(rho(a op b)
op c)`, using absorption with `x = a op b` and `y = c`, equals `rho((a op b) op c)`, and by
commutativity (which `42_probes/p1` establishes unconditionally for both `+` and `*` here,
626,224 pairs, zero failures) that equals `rho(a op (b op c))`, matched to the other association
order by a second application of absorption with `x = b op c` and `y = a`. **The step that does
the work is `(a op b) op c == a op (b op c)`, taken as an identity on the exact, pre-reduction
values.** That is only available when `op` itself is exactly associative before `rho` ever runs.

For addition, `op` is raw integer addition and the identity is free. `57_probes/p2` tests exactly
this, with `op` a bare `fn add(a: i64, b: i64) -> i64 { a + b }` (`57_probes/p2_absorption_
necessity_sweep.rs:147-149`), never touching a scale, and gets the exact biconditional, zero
violations in either direction over 4248 configurations.

For multiplication, `57_probes/p2`'s `op` is `fn mul(a: i64, b: i64) -> i64 { a * b }`
(`57_probes/p2_absorption_necessity_sweep.rs:150-152`), also with no scale. **That is F = 0
multiplication**, the same relabelling section 3.3 finds in `p4`'s "clamp only" arm. So the
4248-configuration multiplicative sweep in `57` section 2.5, the one that reports "absorption is
sufficient and not necessary" with 153 exceptions, all degenerate constants (`57_probes/p2b`), is
also entirely at F = 0. Nothing in `57`'s job one, on either operation, was ever computed with a
rescale in the loop. The additive result needed none, because F never enters addition's definition.
The multiplicative result silently inherited F = 0 from the probe's own construction, which nobody
had flagged, including `57` itself, whose own section 3 measures F > 0 breaking the semiring in the
very same file.

### 2.2 Why no fixed-width `op` exists for F > 0 eager multiplication

Take two format elements `a`, `b` denoting `a / 2^F` and `b / 2^F`. Their true product denotes
`(a*b) / 2^(2F)`, exactly, with no information lost: the raw integer product `a*b` is exact. To
combine that with a third operand `c` (at scale F) via the *same* pairwise operation, one of two
things has to happen: either the product is narrowed back to scale F immediately (discarding F
bits, the ordinary eager multiply every `UFixed<I,F,S>::mul` performs today), or it is kept at its
grown scale and combined with `c` to reach scale `3F`, and so on. **The first choice is what every
real, fixed-width implementation does, because nothing else is representable without letting the
type grow without bound.** And the first choice means the "op" available at each step is not raw
multiplication; it is raw multiplication *composed with* a rescale, which is exactly the same kind
of information-discarding map `rho` is supposed to be the only one of.

Concretely: `((a * b) >> F) * c) >> F` versus `(a * ((b * c) >> F)) >> F`. I checked this by hand
before building anything, because it is worth being able to see without a machine: at F = 2,
a = 3, b = 5, c = 7, the left form gives `((15 >> 2) * 7) >> 2 = (3 * 7) >> 2 = 21 >> 2 = 5`, and
the right form gives `(3 * (35 >> 2)) >> 2 = (3 * 8) >> 2 = 24 >> 2 = 6`. Five against six, with no
range clamp anywhere in either computation. **Truncating fixed-point multiplication is not
associative even when nothing ever saturates.** `57_probes/p4`'s "coarsen only" arm measures this
exhaustively (mul_assoc 1160 of a swept space at M=15, F=1, zero range bound in effect,
`57_probes/p4_which_factor_breaks_and_what_coherence_buys.rs:222`), and my hand computation is one
witness inside that count, not a separate claim.

So there is no `op` for eager F > 0 multiplication that is exactly associative before `rho` runs,
because the rescale that keeps every intermediate representable at width F *is itself* one of the
two things `rho` was supposed to be the sole owner of. Job one's theorem does not fail at F > 0. Its
precondition is unsatisfiable by any implementation that narrows after every step, which every real
one does. This is a structural boundary, not an unswept region of a sweep that would eventually
close it.

### 2.3 What restores the theorem, and what it costs

There is one way to give multiplication a genuine `op`: never narrow until the very end. Then the
raw product over a fold of length `n` is exact at scale `nF`, `op` really is exact integer
multiplication (associative, by construction, on the raw integers), and `rho` really is the one
total map, narrowing by `(n-1)F` bits and clamping, applied once.

`58_probes/p2` section 1 builds exactly this, with a guard width `w` between the ordinary eager
narrow (`w = 0`) and full precision (`w = (n-1)F`), narrowing off `F` bits at every pairwise step
but keeping `w` guard bits of the fraction and clamping the range only once, at the very end. At
`w = (n-1)F` (full precision), LEFT and RIGHT association agree everywhere and both match the
once-at-the-end exact reduction, at n = 2, 3, 4 (`58_probes/p2_output.txt`, section 1). Absorption
holds exactly there, as the theorem says it must, because the precondition is now met.

**What it costs is the finding worth carrying: the guard needed below full precision saves
*exactly one rescale's worth* of bits, constant in fold length, never more.** Measured at M = 15,
F = 3: n = 3 needs `w = 3` against a full-precision ceiling of `w = 6` (half); n = 4 needs `w = 6`
against a ceiling of `w = 9` (two thirds). The pattern is exact across both measured lengths:
`min_w == full_w - F`. **One multiply in the chain can be delayed past its narrowing point. Every
earlier one cannot.** So the absolute guard an implementation needs still grows linearly with fold
length, at a fixed slope of `F` bits per additional operand, with no fold-length-independent
constant the way addition's `W + ceil(log2 n) - 1` is one. Absorption, restated for the correctly
exact ambient operation, is real, general, and not an F == 0 fact. The *cheap* version of it that
job one's original sweep tested is, and there is no way to make eager F > 0 multiplication cheap in
the same sense addition is, because addition's cheapness came from a fact about addition (raw sums
never need a rescale) that multiplication structurally does not share.

## 3. Job two: the semiring's boundary, sharpened

### 3.1 Rounding mode does not rescue it

`57` section 7 names this the cheap open probe, and it is cheap: `58_probes/p1` reruns `57_probes/
p3`'s section 3 table (M = 15, 31, 63; F = 1, 2, 3) under round-half-up in place of truncation for
the rescaling divide. Every row still fails; the counts move (down at most rows, up at a few:
`*assoc` at M=15,F=2 rises from 878 to 958, while `distrib` at the same row falls from 518 to 475),
and no row reaches zero under either rule (`58_probes/p1_output.txt`).

This is not a surprising result to anyone who has shipped a fixed-point filter, and it is worth
stating why plainly rather than by appeal to authority: rounding to nearest picks the *closest*
representable value to the true product, which minimises the *magnitude* of the discarded
information per step. It does not make the discarding *stop*. Associativity failure here is not
caused by which value truncation happens to produce; it is caused by the fact that a value is being
discarded at all, at a point in the computation that differs depending on association order. A
better choice of which bit to keep changes how far wrong the answer is. It cannot change whether
information was thrown away before the two association orders had a chance to agree. This is the
same phenomenon Goldberg's floating-point survey documents for `(a*b)*c` against `a*(b*c)` under
round-to-nearest IEEE arithmetic: the rounding *rule* is not the variable that decides associativity;
the *presence* of a per-step rounding is.

### 3.2 What `57_probes/p4`'s "clamp only" arm actually is, and actually shows

`57` section 3.3 frames its factor decomposition as isolating "which factor of the reduction
breaks it, measured rather than assumed" and reports the range clamp "clean at every row measured".
Reading the source (`57_probes/p4_which_factor_breaks_and_what_coherence_buys.rs:89-96`):

```
fn mul(&self, a: i64, b: i64) -> i64 {
    let s = 1i64 << self.f;
    match self.factor {
        Factor::ClampOnly => (a * b).clamp(0, self.m),
        Factor::CoarsenOnly => (a * b) / s,
        Factor::Composite => ((a * b) / s).clamp(0, self.m),
    }
}
```

`ClampOnly`'s multiply never reads `s`. It is `(a*b).clamp(0, m)`, the F = 0 case, under a name
that reads as "the range-clamp factor of an F > 0 reduction". This is not a bug in the probe; per
section 2.2 above, there is no other coherent way to define "range clamping alone, at F > 0,
without coarsening", because coarsening is not an optional companion to F > 0 multiplication, it is
what F > 0 multiplication *is*. So "clamp only clean" is not new evidence beyond `57_probes/p3`
section 1's own F = 0 semiring result; it is the same fact, restated inside a table that reads as
if it were the other half of a controlled experiment.

What *is* new and correctly attributed is `CoarsenOnly`: unbounded range, no clamp anywhere, and
it still breaks (mul_assoc 1160, distrib 512 at M=15, F=1). That establishes coarsening is
*sufficient* for the failure, with no clamp needed at all, which is a real and useful fact. It does
not establish that clamping is *innocent* at F > 0, because that claim was never tested and, per
section 2.2, cannot be tested in the form the table implies. `57`'s own next sentence notices the
consequence without naming its source: the composite fails *less* than coarsening alone (398
against 1160 at the same row, `57:399-403`), which `57` correctly reads as the clamp masking
distinct wrong values by collapsing them onto a shared bound. That masking is exactly what you get
when two independent lossy maps compose: their joint failure count is not the sum, or even the
maximum, of either alone, because one can absorb the other's divergences into agreement by
accident. It is evidence the two mechanisms interact, not evidence that either one, taken to F > 0,
is clean.

`58_probes/p2` section 2 measures the clamp's contribution in isolation from coarsening instead of
alongside it, by holding coarsening at zero (full guard width, the exact regime section 2.3 above
restores) and adding an intermediate range clamp back in. At full fractional precision, with zero
rounding loss anywhere, the with-clamp fold still diverges: 448 of 4096 triples at n = 3, 11528 of
65536 at n = 4 (`58_probes/p2_output.txt`, section 2), against zero for the no-clamp control at the
same precision. **An intermediate range clamp is, on its own, with the fractional axis paid in
full, a real and substantial source of non-associativity in a multiplicative fold.** It is job
one's absorption/reachability mechanism again, operating inside a chain of more than two multiplies
rather than inside a chain of additions, and it does not need coarsening's help to fire. The two
mechanisms are independent and additive in effect, not alternatives an implementation trades off
against each other; paying the full cost of one buys nothing against the other.

### 3.3 The F == 0 boundary is a structural fact, not a swept-so-far one

Putting sections 2.2 and 3.1 together: multiplicative associativity at F > 0 fails because a
representable-width multiply cannot avoid discarding information at every step (2.2), the choice of
what to discard (the rounding rule) changes the failure's magnitude and never its existence (3.1),
and the one available escape (never discard anything, narrow once at the end) is not a rounding
choice at all, it is the abandonment of a fixed representation width, at a cost that grows without
bound in fold length (section 2.3). There is no fourth option among "which rounding rule" that
these three have not already covered. **`35`'s original finding (cited, not opened by me, at
`42:187-190`: both laws hold "almost exclusively at `F == 0`"), `57_probes/p3` section 3's
independent re-measurement at nine widths, and this file's rounding-mode sweep at a third variable
converge on the same boundary, and this file adds the reason the boundary cannot move: it is a fact
about what finite-width multiplication is, not about which finite-width multiplication was tried.**

### 3.4 The multiplicative grading, and why Q11's closed form has no analogue

`57` section 3.6 reports the additive accumulator grade at `W + ceil(log2 n) - 1`: a small,
fold-length-*logarithmic* function of `n`, uniform across every row measured, with zero anomalies.
That shape is what makes `OPTIONS.md` Q11's third option ("the accumulator is derivable as the
width plus the log of the capacity") a genuinely useful design primitive: a consumer states a
capacity, the derivation returns a small, bounded accumulator width, done once, valid forever.

`58_probes/p2` section 1 measures the same question for multiplication and finds no such shape.
The savings below full precision is a *constant* (exactly `F` bits, not growing with `n`), but full
precision itself grows *linearly* in `n` (`(n-1)F` bits), so the accumulator an implementation
needs to guarantee exact-then-adapt agreement grows linearly in fold length with a fixed, non-
vanishing slope. There is no logarithm here, and there is no bounded closed form independent of
fold length the way Q11's third option states one for addition. A `Precise` strategy chain of
products (I7's "especially within chains and ops") that wanted the accumulator-relation mechanism
Q11 proposes for addition would need a *different* mechanism for multiplication, one whose width
parameter is the fold length itself rather than its logarithm, or it would need to give up on exact
agreement and instead bound the error statistically (the guard-bit-plus-controlled-rounding
approach every DSP fixed-point multiply-accumulate unit actually uses, trading an exact accumulator
guarantee for a bounded expected error per Wilkinson-style rounding-error analysis, which is the
approach `55`'s own phase-one section 5 named as the precondition I7's chain-accuracy claim needs
to be checkable at all). **Q11's "both" option (the structure plus the capacity relation) is not
one mechanism serving both operations; it is an additive-shaped mechanism that has no
multiplicative sibling of the same shape, and a canon adopting it as written would silently promise
`Precise` a guarantee for product chains that the numbers here show does not scale the way the sum
case does.**

## 4. What I put to `55`, `55b`, `56` and `57`, for the resumption

**To `55`.** Your phase one section 5, requirement 3, names the quantum metric as the precondition
for I7's chain-accuracy claim to be checkable, and requirement 2 names totality of adaptation as a
requirement independent of which member of R is chosen. Section 3.4 above is the concrete case
where those two requirements pull apart for a real strategy: a chain of products cannot be made
exactly reassociable at any bounded accumulator width the way a chain of sums can, so I7's "accurate
within chains, not only alone" for `Precise` needs either an unbounded accumulator (not real) or a
statistical error bound stated in your quantum metric rather than an exactness guarantee. Does your
framework already have a place for "this law holds only up to a stated error budget, never
exactly", or does the concept as derived assume every law a strategy claims is exact?

**To `55b`.** I opened `55_probes/p4_induced_algebra_grades.rs` and checked: your `add` and `mul`
(lines 69-73) are `a + b` and `a * b`, bare, with no scale parameter anywhere in the file. Your
induced-algebra grades are F = 0, for the identical reason `57`'s `p2` is (section 2.1 above). Does
that change what you would want the "induced structure per policy" sentence in Q11 to be able to
say: a structure *and a scale condition*, as I put to you in section 3.4, or is there a reading
where the grade already carries the scale implicitly that I am missing?

**To `56`.** Your coherence law (`56_probes/q1`'s C-law) is stated once, over one operation. Job
two's multiplicative grading result (section 3.4 above) suggests coherence itself might need a
per-operation reading even within one policy: unsigned saturating addition is coherent over its
nonnegative window at every F (section 2.1's F-independence), while the *same* policy's
multiplication is coherent nowhere once F > 0 (section 3.3). Is "a policy is coherent" well-posed as
a single yes/no, or does your framework already intend it per-operation and I am reading a gap that
your own C-law's definition (parameterised by an ambient op) already closes?

**To `57`.** Three things. First, section 2.1 above: your job-one `p2` and `p2b` are F = 0 on both
operations, which your own section 3 measures breaking down at F > 0 in the same file; would you
restate the headline biconditional as "for the ambient operation taken at its own fixed
representation width" rather than as a general fact, given section 2.2's argument that no other
reading is available for multiplication? Second, section 3.2 above: does the "clamp only" naming
in `p4` want changing to something that does not imply an F > 0 ablation exists, given it cannot?
Third, you flagged the accumulator grade's generalisation past addition, fifteen rows, as a
suspicion with fifteen supporting rows and no proof (`57:648-649`); section 3.4 measures the
multiplicative case directly rather than generalising the additive one, and the answer is that it
does not generalise, it is a structurally different shape. Does that settle the suspicion in the
negative, or is there a reading of "generalises" under which linear-with-constant-savings still
counts?

## 5. What the register should gain

I have not edited `OPTIONS.md` or `INTENTS.md`, per the dispatch. What they should gain, for
whoever holds the repair:

**Q11's third option needs the scale condition stated as a hard boundary, not a caveat.** "The
accumulator is derivable as the width plus the log of the capacity" is true of addition,
unconditionally, at every fraction width, and has no multiplicative analogue at any fraction width
greater than zero. The option as currently worded reads as a general mechanism for "a fold's
accumulator"; it should read as a mechanism for *additive* folds specifically, with the
multiplicative case named as open and structurally different rather than as an unswept
generalisation.

**Q11's "both" option should gain the same qualification**, since `42_probes/p2`'s composed
contract (the worked instance the option cites) composes the structure-naming bound with the
*additive* accumulator relation. Nothing in the register currently states that the capacity-derived
accumulator half is additive-specific; section 3.4 is the evidence that it is.

**Q5 gains independent evidence for two axes, from job two rather than job one.** `57`'s own
section 6.3 (see also section 3 above) makes the same "two axes, different law consequences" point
from the associativity/distributivity angle; this file adds that the *cost of buying exactness back*
(the accumulator grade) is also structurally different per axis: bounded and logarithmic for the
overflow axis under addition, unbounded and linear for the scale axis under multiplication. A
single "arithmetic policy" axis cannot express two law-recovery costs of different asymptotic
shape.

**Q14 gains a candidate input it does not currently have.** Section 3.4's finding, that `Precise`
cannot buy exact multiplicative reassociation at any bounded cost the way it can for addition, is
exactly the kind of number op's unset exchange rate would need to weigh: whether `Precise`'s
"accurate within chains" intent (I7) is satisfied by an unbounded accumulator (real cost, unbounded
in fold length), or by a stated statistical error bound instead of an exactness guarantee (a
different, weaker, but boundable promise). I am not proposing which; I am naming that the register
currently has no line stating the choice exists.

**A droplist candidate, not yet a droplist entry.** "The unsigned-saturation semiring transfers to
fractional formats" should be recorded as closed, with the diagnostic being structural (section 2.2
and 3.1 above) rather than merely empirical (the nine-of-nine failures `57_probes/p3` already
measured). What would reopen it: a demonstration of a total, translation-covariant rounding rule
under which a fixed representable width F > 0 loses no information on some pairwise multiply for
which the operand set is closed under the operation, which section 2.2's argument says cannot
exist for any nontrivial operand set, but I have not proven that as a theorem, only argued it and
tested two rounding rules against it.

## 6. Bearing on the live options

**Q5 (one axis or two).** Sharpens the existing evidence rather than adding a new direction: the
two axes carry not only different law consequences (already established) but different
*law-recovery costs*, one bounded and one unbounded in fold length. A single axis would need to
express both shapes simultaneously, which no closed form in the register currently attempts.

**Q6 (Warm wraps or clamps).** No new bearing on the fork itself. Sharpens what "clamps" would cost
`Warm` for any consumer chaining multiplications: under clamping, `Warm`'s multiplicative chains
inherit the F > 0 boundary this file establishes as structural (not policy-dependent: wrapping
inherits it too, since wrapping's multiply also rescales by F bits before reducing modulo `2^W`,
though I have not measured wrapping multiplication directly and flag this as an inference rather
than a result).

**Q11 (fold guarantees).** Directly bears, per section 5 above. The third option and the "both"
option both need the additive-only qualification; neither currently states it.

**Q12 (reduction order).** No new bearing beyond what section 3 of `57` and section 6 of `42`
already carry; job two's grading result is about accumulator *width*, which is downstream of
whether reassociation is licensed at all, not a new fact about licensing itself.

**Q14 (exchange rate).** Gains a candidate input, per section 5 above: the question of whether
`Precise` may adopt an unbounded-in-principle accumulator for product chains, or must instead state
a statistical error bound, is exactly the shape of choice the unset rate exists to resolve.

**Kills nothing.** No option in the register is closed by anything here. The droplist candidate in
section 5 is offered as a candidate, not a closure, because I have argued rather than proven the
structural impossibility it would rest on.

## 7. What I could not determine

**Whether the "no op exists" argument in section 2.2 is a theorem or a strong argument.** I showed
a witness (F=2, a=3, b=5, c=7) and cited an exhaustive sweep (`57_probes/p4`'s coarsen-only arm)
that confirms it is not an isolated counterexample. I have not proven, for an arbitrary total
translation-covariant rounding rule and an arbitrary format, that no such rule can make truncating
fixed-point multiplication exactly associative on a nontrivial operand set. My rounding-mode probe
tested exactly two rules (truncation, round-half-up) out of the space of total rounding functions;
a third rule (round-half-to-even, stochastic rounding, dithered rounding, which is closer to what
image and audio pipelines actually ship for exactly this reason) is untested and I would want it
tested before calling this closed.

**Whether wrapping multiplication has the same F > 0 boundary.** I inferred it in section 6 rather
than measured it. Wrapping's multiply also performs the F-bit rescale before reducing modulo `2^W`,
so the same information-discarding argument should apply, but wrapping's algebra is a ring at
F = 0 (`55b` section 3.1) and I have not checked whether ring structure survives the rescale any
better than the unsigned semiring does. Unmeasured.

**Whether the one-rescale savings in section 2.3 (`min_w == full_w - F`) holds past n = 4.** Two
data points, both exact. I would want n = 5 and n = 6 before trusting the pattern the way `57_probes/
p6` trusts fifteen rows of the additive one-bit finding.

**Whether the statistical-error-bound alternative named in sections 3.4 and 5 is buildable in this
substrate's type system at all.** I named it as the DSP-standard alternative to an exact
accumulator guarantee. I have not attempted to state it as a type-level contract, and I do not know
whether arvo's typestate has anywhere to carry a probabilistic or worst-case error bound the way it
carries an exact width. This is a genuinely open design question and I am flagging it rather than
answering it.

**Anything about `35`, `18`, `20`, `25`, `40` or `43`.** Not opened. Where this file's results touch
theirs, through `42`'s or `57`'s citations, I have said so and have not reasoned about their
contents directly.

## 8. Coverage, bounded honestly

**Read in full:** `00_brief.md`, `INTENTS.md`, `57`, `42`, `55` (both phases), `55b`. **Read in
part, opened directly:** `56` sections 3.3 and 9. **Read via source, not via a predecessor's
account:** `57_probes/p2_absorption_necessity_sweep.rs`, `p2b_necessity_violations_are_degenerate.
rs`, `p3_semiring_across_widths_and_scales.rs`, `p4_which_factor_breaks_and_what_coherence_buys.rs`,
`55_probes/p4_induced_algebra_grades.rs` in full (confirmed lines 69-73 never reference a scale
parameter, per section 4). **Not opened:**
`56` outside the two cited sections, `35`, `18`, `20`, `25`, `40`, `43`, `DROPLIST.md`, `seed/`,
`archive/`.

**Re-run before relied on:** `57_probes/p2`, `p3`, `p4`, all byte-identical against their committed
outputs (`58_probes/RUN.md`). **Built:** two probes, both exhaustive within their stated domains,
both with instrument validation that fires, both committed with sources and outputs. `p2`'s first
run asserted `min_w <= full_w` and failed, correctly: the accumulator was being range-clamped at
every intermediate step even at full guard width, which introduces the exact second mechanism
section 3.2 measures deliberately in the corrected version. Both runs are on disk
(`p2_output.v1_bug.txt` and `p2_output.txt`).

**Everything measured here is plain integer and `i128`-widened fixed-point arithmetic.** No arvo
types, no type-level construction, deliberately, so nothing here is an artifact of a representation
choice. That is also the coverage limit: these are results about a specific truncating and a
specific round-half-up rescale, at one format width (M = 15, F = 3, and the smaller widths `p1`
sweeps), and their transfer to whatever container arvo's format concept eventually derives is an
argument I have made analytically (section 2.2) rather than verified against a second, independent
implementation.

**First-read, owed seconds:** the structural argument in section 2.2 (no `op` exists for eager
F > 0 multiplication); the correction to `p4`'s "clamp only" reading in section 3.2; the
multiplicative grading measurement in section 3.4. **Seconded by me, from a different angle:**
`57_probes/p3`'s F == 0 boundary (my `p1` is a second measurement of it, at a third variable,
rounding mode, on top of `35`'s original finding as cited at `42:187-190` and `57_probes/p3`'s
independent re-measurement at nine widths; `35` itself I have not opened, so I count that link by
citation, not by verification); `57`'s masking observation about the composite failing less than
coarsening alone, which section 3.2 explains as two independent mechanisms interacting rather than
one being cleared.

**Nothing here settles anything.** The mode is explore, there is no canon, and `55`, `55b`, `56`
and `57` should be resumed to answer section 4 before any of it is carried past this unit's
checkpoint.
