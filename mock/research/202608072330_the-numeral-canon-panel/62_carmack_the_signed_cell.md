# 62. The signed cell

**Persona:** John Carmack. Systems engineering; the discipline of measuring the case the design
will actually ship rather than the case that was convenient to measure.

**Date:** 2026-08-09. **Position:** file seven of unit two on the format-concept topic, dispatched
from `59`'s P4 (`59:344-356`): the signed cell at nonzero fraction is empty, and it is the cell the
default strategy would occupy. `57:624-626` said it plainly ("the signed case is where the algebra
is worst and it is the case Warm would actually be") and until this file nobody had followed.

**Probes:** `62_probes/`, five of them plus a rerun record, committed as made. `62_probes/RUN.md`
is the build line. Two probes had a first run kept on disk per this panel's discipline: p3's first
mutant was not a mutant (it compared congruent representatives, which is the very transport fact
the probe measures) and the run correctly printed FAILS; it is kept as
`p3_output.v1_bad_mutant.txt` and the corrected mutant fires at 176.

**Re-run before argued with.** `57_probes/p3`, `61_probes/q2` and `35_probes/p3` rebuilt on
`nightly-2026-05-28` and diffed against their committed outputs: all three byte-identical
(`62_probes/rerun/`). Every count this file takes from any of them is a count regenerated on this
machine. `58_probes/p2` was read at source (its narrows, its fold shape) and re-derived rather than
re-run, since p4 rebuilds its construction independently in the signed domain; the n = 3 and 4
unsigned truncation savings my construction implies agree with `58`'s committed pattern.

**Read for this file:** `INTENTS.md`, `00_brief.md`, `RULES.md`. Then `58` and `61` in full with
their probe sources and outputs, then `57`, `55b`, `56`, `59`, `60` (both phases) in full.
`35_probes/p3_reduction_order.rs` and `p3.out` opened directly, since the 70.1 percent figure this
dispatch cites lives there and the discipline is to check the probe you inherit. `OPTIONS.md` last:
Q3, Q4's strategy-set note, Q5, Q6, Q11, Q12, Q13, Q14, Q17. **Not opened:** `55` itself, `08`,
`35`'s prose, `42`, `18`, `20`, `25`, `40`, `43`, `50`, `DROPLIST.md`, `seed/`, `archive/`. Where
anything below touches those, it is sourced to a file that read them and marked so.

## Status of this file

Nothing here settles anything, per the standing explore mode. What it does: fills every empty cell
of the sign-by-operation-by-policy-by-scale cube `59` section 2c drew, and the filled cube says the
unit's uniform picture ("at nonzero fraction, multiplication is broken for both policies by one
mechanism, coarsening, while addition survives for both") is **unsigned-scoped on both halves**.
In the signed domain the additive half is policy-dependent (wrap keeps a full group at every F,
saturation is broken at every F), and the multiplicative half is broken **before coarsening ever
enters**: signed two's-complement saturating multiplication fails associativity on the integer
grid, at every width measured, by a mechanism this unit had not named, the asymmetry of the
representable range under negation. Removing the asymmetry (a symmetric clamp) restores exact
multiplicative associativity at F = 0 and buys nothing at F > 0, where coarsening takes over and
owns, at deep enough fraction, the entire failure set of both policies, verbatim. Two further
results fell out on the way down: the signed rescale is two different maps (truncation toward zero
against arithmetic shift), observable in every count and in the accumulator grade, and the wrap
representative section, a relabelling at F = 0, becomes arithmetic at F > 0.

## 0. Gates

**Canon gate: passes, in the second of `expert-dispatch-defends-the-canon.md`'s three situations.**
There is no ratified canon; the panel is writing one (`00_brief.md:8-9`), and `INTENTS.md`'s own
header records that no entry holds the ratified rung. Nothing below closes a question or proposes a
mechanism. What the work was checked against: `INTENTS.md` (I1: the strategy set is open, and
nothing below presumes a count; I3 and I7, which the consequences bear on and which are quoted
where used), the acceptance criterion at `00_brief.md:144-146`, and the forbidden-feature list at
`00_brief.md:158-160`. Every probe is plain integer and `i128` arithmetic with no type-level
machinery: `grep -c '^#!\[feature' 62_probes/*.rs` returns 0 on all five files.

**Test gate: no suite exists to run.** `ls -la mock/crates` shows only `.` and `..`, the nuked
tree the brief describes, confirmed directly. The substitute is the probe discipline: every probe
carries instrument validation that must observe both truths, and one of them (p3) failed its own
first run on exactly that check, which is the discipline working.

**Repository state.** `git status --porcelain` was empty at dispatch start apart from this file's
own artifacts as they were made. The `docs/` deletions and modified bench artifacts `57` flagged
at its section 0 are absent from this tree, as `58` and `61` already recorded.

## 1. The answer, before the working

**The signed cell is worse than the unit's worst-case framing of it, and it is worse at F = 0,
where the unsigned story was at its best.** Signed two's-complement saturating multiplication
fails associativity on the integer grid: 28, 160, 780 and 3516 triples at w = 3, 4, 5, 6,
exhaustively over Q cubed, with distributivity failing at 120 through 119,422 alongside
(`62_probes/p1_output.txt` section 2). The witness is three values long and worth carrying:
`(7 * 7) * -1 = -7` against `7 * (7 * -1) = -8` at Q = [-8, 7]. There is no coarsening anywhere in
that computation. The unsigned semiring's multiplicative half, which `57` proved structural via
the congruence argument, has **no signed analogue under the two's-complement range**, so the
sign-by-scale story is not "the good F = 0 structure, lost at F > 0" as it is for unsigned; the
signed clamp never had the F = 0 structure to lose.

**The mechanism is the range's asymmetry under negation, and it is exactly a coherence failure
that the unit's own machinery diagnoses.** At F = 0 the ambient multiplication is exactly
associative, so `57`'s sufficiency theorem applies with its precondition genuinely met, and the
question reduces to whether the clamp is multiplicatively coherent. It is not: `sat(-1 * 8) = -8`
while `sat(-1 * sat(8)) = sat(-7) = -7` (first witness in `62_probes/p1_output.txt` section 4;
26, 50 and 98 window violations at w = 3, 4, 5). Negation maps the ceiling's absorbed class into
the interior on one side (`-hi` is representable) and past the floor on the other (`-(hi+k)` for
k >= 2), so "both beyond the same bound" is not preserved by multiplication by a negative, and no
congruence argument can be stated. **Make the range symmetric and the argument comes back:** under
[-h, h] the same sweep measures multiplicative associativity at exactly zero failures at every
width, and multiplicative coherence at exactly zero violations, while distributivity and additive
associativity stay broken (they route through the additive pullback, which range symmetry does not
touch). One code point, the most negative value, is the entire difference between "signed
saturating multiplication is a commutative monoid at F = 0" and "it is not even a semigroup".

**At F > 0 the signed cell dies for both policies, both rescale spellings, every configuration**,
54 of 54 measured rows failing multiplicative associativity and distributivity
(`62_probes/p2_output.txt` section 2), and the coarsening attribution transfers with a correction.
Coarsening alone, no reduction map anywhere, breaks signed association under both spellings
(9,280 to 14,880 triples over [-15, 15] cubed, `p2` section 3), so `58`'s structural argument is
sign-free, as it claimed. But the unsigned factor story ("the clamp factor is clean, the blame
lands on coarsening", `57:383-397`) does **not** transfer: in the signed domain the clamp factor
is independently broken at F = 0 (the previous paragraph is precisely the clamp-only measurement,
per `58` section 3.2's correction that clamp-only IS F = 0), so signed multiplication at F > 0
has **two independent sufficient mechanisms** where unsigned had one. And at deep fraction the
policies stop mattering entirely: at w = 4, F = 3, truncation, saturation and wrap fail on the
**identical set** of 380 triples, sat-only and wrap-only counts both exactly zero, with the
policy-specific contribution shrinking monotonically toward it (118 + 874 at F = 1, 34 + 390 at
F = 2, 0 + 0 at F = 3; `62_probes/p2b_output.txt`). Coarsening does not merely dominate; at
F = w - 1 it owns the failure set outright.

**The additive half of the signed cell is policy-dependent in a way the unsigned cell never was.**
Signed wrap keeps the full abelian group at every F (zero failures on every additive axiom, all
widths, `p1` section 5 and `p2` section 2, seconding `35_probes/p3.out`'s zero divergence row from
a third instrument), while signed saturation's additive associativity fails at counts that are
**exactly F-invariant**: 952 at w = 4 for every F in 0 through 3, 784 for the symmetric variant,
matching the F = 0 row to the unit because the add closure never reads the scale (`p2` section 2's
scale-blind check, the same argument `58` made and `61` re-made). The 952 also reproduces
`55b`'s `p5` count from an independent instrument. So "addition survives" is true per policy, not
per operation: in the unsigned domain both policies keep it, in the signed domain only wrap does.

**The rescale is two maps in the signed domain, and the difference is observable everywhere.**
Truncating division and arithmetic shift coincide on nonnegative products, which is why no
unsigned probe in this unit could distinguish them; on signed operands they differ (witness at
F = 1, (a, b, c) = (-3, 5, 7): truncation gives -24 against -25 for the two associations, floor
gives -28 against -26). The spelling moves every failure count (for example signed wrap w = 4,
F = 2: 1064 truncation against 1772 floor) and zeroes none, which is `58` section 3.1's
rounding-mode result arriving on a new axis: the spelling is a rounding rule for negatives, and
rounding rules move magnitudes, never existence.

**The multiplicative accumulator grade transfers to the signed domain, with the spellings pulling
apart.** Mirroring `58_probes/p2` section 1 in signed Q = [-8, 7] at F = 3 (`62_probes/p4`): full
guard width is clean for both spellings (exact integer multiplication associates, LEFT = RIGHT =
exact-once, all n), the guard needed still grows linearly, and the saving below full precision is
**exactly F at every measured length for the floor spelling** (3, 3, 3 at n = 3, 4, 5) while
truncation gives an irregular 4, 3, 5. Floor is the translation-covariant spelling with uniform
rounding bins; truncation's double-width bin at zero is a second absorber that produces the
excess. `60`'s probe D found the unsigned truncating saving at 3, 3, 4 over the same lengths, so
the signed truncating slack is not the unsigned one shifted; it is its own domain-dependent
number, and the clean constant-F fusion story belongs to the floor spelling in both domains
measured so far.

**And one result that is not about saturation at all: the wrap representative section becomes
arithmetic at F > 0.** At F = 0 the unsigned section [0, 2^w - 1] and the signed section
[-2^(w-1), 2^(w-1) - 1] induce the identical ring, zero disagreements over every class pair at
three widths, both spellings, which is the transport fact `56` section 4's deflation rests on and
which p1 section 5 reconfirms. At F > 0 the two sections disagree on 96 of 256 class pairs at
w = 4, F = 1, rising to 168 of 256 at F = 3 (65.6 percent), at every width and both spellings
(`62_probes/p3_output.txt`). Witness: classes 9 and 3 at w = 4, F = 1, floor: unsigned section
computes class 13, signed section computes class 5. The rescale acts on representatives, not on
residue classes, so the moment F > 0 the declaration-time choice of section is a choice of
arithmetic. `61`'s q2 measured the unsigned section; my p2's signed-section wrap rows are a
different magma with different counts at the same modulus and spelling (1460 against `61`'s 1712
at w = 4, F = 1), which is the same fact showing up at the law level.

## 2. The cube, filled in

`59` section 2c drew the cube with one cell empty and several inferred. As of this file every
cell is measured, and each row names its instrument:

| sign | op | policy | F = 0 | F > 0 | evidence |
|---|---|---|---|---|---|
| unsigned | add | sat | monoid | monoid, every F | `57_probes/p3`; scale-blind argument |
| unsigned | add | wrap | group | group, every F | `61_probes/q2` section 4 |
| unsigned | mul | sat | semiring half, congruence | dead, 9 of 9 | `57_probes/p3` |
| unsigned | mul | wrap | ring half | dead, 9 of 9 | `61_probes/q2` |
| signed | add | sat | **dead** (952 at w = 4) | dead, counts F-invariant | `55b` `p5`; `62_probes/p1`, `p2` |
| signed | add | wrap | group | group, every F | `35_probes/p3`; `62_probes/p1`, `p2` |
| signed | mul | sat, 2c range | **dead** (160 at w = 4) | dead, both spellings | `62_probes/p1`, `p2` |
| signed | mul | sat, symmetric range | **assoc exact, monoid** | dead, both spellings | `62_probes/p1`, `p2` |
| signed | mul | wrap | ring | dead, both spellings, section-dependent | `62_probes/p1`, `p2`, `p3` |

Three structural statements the cube supports, none of which any single row could:

**The F = 0 induced structure is a function of the policy, the scale, the sign domain, and the
range's symmetry under negation.** The register's Q11 material already carries the first two
(per `57` section 5 and `61` section 6); the sign domain and the symmetry parameter are new, and
the symmetry parameter is the surprising one because it is a property of Q alone, not of the
policy: the identical clamp code induces a multiplicative monoid on [-7, 7] and a non-semigroup
on [-8, 7].

**The two multiplicative failure mechanisms are independent and separately sufficient, and the
signed domain is where both live at once.** Asymmetric absorption kills the F = 0 structure with
no coarsening present (p1); coarsening kills the F > 0 structure with no reduction map present
(p2 section 3); at F = w - 1 the coarsening owns the entire failure set of both policies (p2b).
The unsigned domain was the special case with one mechanism, because a one-sided clamp on a
sign-confined domain cannot be asymmetric under an operation that never leaves the half-line.

**Nothing multiplicative survives the fraction axis anywhere in the cube.** Every F > 0
multiplicative cell is dead for every policy, every sign domain, every range choice and both
rescale spellings measured. The additive column and the F = 0 signed-wrap and symmetric-clamp
cells are the only structure the law layer has available to state, and the additive column's
signed half belongs to wrap alone.

## 3. What this does to the unit's picture, stated against the dispatch's claims

The dispatch handed four claims to test rather than accept. Verdicts:

**"Unsigned addition survives at every fraction width, structurally."** Holds, and extends: the
same scale-blindness makes the signed additive counts F-invariant too, broken ones included. The
correct general statement is that the additive column's verdict, whatever it is, is F-independent;
survival itself is a fact about the policy and the sign domain, not about addition.

**"Unsigned multiplication at nonzero fraction is dead, attributed to grid coarsening rather than
the clamp."** Holds as stated for the unsigned domain, and the attribution's converse ("the clamp
is clean") is unsigned-only. In the signed domain the clamp factor is broken at F = 0 on its own,
so the honest attribution at F > 0 is two mechanisms, not one, converging to pure coarsening as F
approaches the width (p2b's monotone partition).

**"Wrapping is the same story: ring only at F = 0, additive group at every F."** Holds, measured
here for the signed section as `61` measured it for the unsigned, with the addition that the two
sections are the same ring at F = 0 and different magmas at F > 0, so "wrap's induced structure at
F > 0" is not one thing per width; it is one thing per width **per section**.

**"Signed addition diverges at 70.1 percent under saturation, zero for the other three."**
Verified at the source (`35_probes/p3.out`, regenerated byte-identically; the probe is additive,
same-scale, exhaustive at w = 3, n = 8), and the fold-level figure now has the triple-level and
axiom-level counterparts from an independent instrument (952 of 4096 triples at w = 4, every F).
The answer to the dispatch's question is yes: additive survival is policy-dependent in the signed
domain and policy-independent in the unsigned one, and that asymmetry is invisible to any sweep
that stays sign-confined.

## 4. What I put to the others, for the resumption

**To `57`.** Two things. Your congruence argument for the unsigned semiring generalises with a
condition your file could not have needed: the two-sided relation "equal, or both beyond the same
bound" is preserved by integer multiplication exactly when the bounds are mirror images, which is
p1 section 4's measurement (coherence violations 26/50/98 asymmetric, 0/0/0 symmetric) and is
checkable as a one-paragraph argument in your congruence style. Would you fold a range-symmetry
condition into the induced-structure sentence you proposed for Q11, beside the scale condition you
already added? And your Q6 note (`57:624-626`) said the signed case carries a real accumulator
grade and the worst algebra; the multiplicative half is now measured and it is worse than your
sentence implies: broken at F = 0, where your own unsigned result was a theorem. Does that change
what you would want Q6's entry to say about clamping's cost for a signed default?

**To `58`.** Your section 3.3 boundary statement ("the semiring is a fact about the integer grid")
is correct and is unsigned-scoped in a way the file does not flag: in the signed two's-complement
domain there is no F = 0 multiplicative structure for the fraction axis to destroy. Would you
restate the boundary as nested, sign-and-symmetry deciding what exists at F = 0 and coarsening
deciding that nothing multiplicative survives past it? Second: your droplist candidate's reopen
condition asks for a total translation-covariant rounding rule that loses nothing; the
translation-covariant spelling (floor) is measured here as the better-behaved accumulator arm
(exactly F saving, three lengths) and it still zeroes no failure count anywhere, which is one more
instance for your argument rather than against it.

**To `61`.** Your q2 drove `57`'s wrap machinery at the unsigned section, and your section 3.3
counts (1712 at M = 15, F = 1, truncation) sit beside my signed-section 1460 at the same modulus
and spelling: same policy, same width, different section, different failure count, which is p3's
observability result at the law level. Should your Q17 wrap row be marked per-section, and does
your "shared code makes the transfer a fact about the code" argument want the rider that the
shared code is shared only after a section is fixed?

**To `55b`.** Your induced-algebra table (`55b:96-101`) classifies signed saturation as "not a
semigroup" on the additive evidence. The multiplicative column of that row is now measured: also
not a semigroup under the two's-complement range (160 at w = 4), and a commutative monoid under
the symmetric range. Does the ladder want the range-symmetry parameter, given that it moves a cell
two rungs without touching the policy?

**To `56`.** Your deflation (section 4) survives everything here; what it needs is a scope
sentence. "The section is chosen at declaration and no cast consults a policy" is exact at F = 0
and the declaration becomes arithmetic-bearing at F > 0 (p3): two numerals wrapping the same
modulus with different representative ranges compute different products. One sentence in the
wrapping entry saying the relabelling claim is an F = 0 claim would close it.

**To `60`.** Your probe D's two-mechanism reading (rule-conditional fusion plus a growing
absorption slack) fits the signed data with one refinement: the floor spelling shows pure fusion
(exactly F, no excess, n = 3 through 5) while truncation shows excess already at n = 3 in the
signed domain (4, 3, 5). The candidate mechanism for the excess is truncation's double-width
rounding bin at zero, which is a property of the rule, not of the final adaptation; that would
predict excess appearing wherever sign-alternating products cross zero and never under floor. I
did not isolate it; it is one hypothesis for whoever extends your probe D.

## 5. What the register should gain

Reported; I have edited neither `OPTIONS.md` nor `INTENTS.md`.

**Q17 gains the signed rows.** The entry currently records the fraction boundary as splitting
unsigned results. It should carry: the signed saturating multiplicative cell is dead at F = 0
under the two's-complement range (p1's counts) and alive-associative under a symmetric range; the
signed additive column is policy-split (wrap group, saturation dead) with counts F-invariant; at
F approaching the width the two policies' multiplicative failure sets coincide exactly (p2b); and
the accumulator saving is spelling-dependent (floor exactly F at three lengths, truncation
irregular), extending `60`'s rounding-conditional finding to a second rounding axis.

**The wrapping entry gains the section scope.** The deflation and the "declared at type time"
sentence are F = 0 facts; at F > 0 the section is observable in the induced arithmetic (p3), so
unsigned-wrap and signed-wrap at one width are one ring at F = 0 and two magmas at F > 0.

**Q11's structure-naming option gains two parameters.** The induced structure is a function of the
policy, the scale (already proposed by `57` and `61`), the sign domain, and the range's symmetry
under negation. A structure-naming mechanism that cannot express "this Q's multiplication is a
monoid and that Q's is not, same policy, same width, one code point apart" will misstate exactly
the case a signed default lives in.

**Q6 gains the measured half of `57`'s warning.** Whichever policy `Warm` takes, if `Warm` is
signed (I3 points it at Rust's primitives, which are signed by default in every integer literal
context), then under clamping its multiplication is broken at every scale including F = 0, and
under wrapping its multiplication is exact at F = 0 and broken at every F > 0. The one lever that
buys anything multiplicative back for a signed clamping numeral at F = 0 is the symmetric range,
at the price of one code point and of departing from two's-complement value range, which is a
choice with prior art in shipped DSP saturation modes and is currently expressible nowhere in the
register.

**Q5 gains an instance for the rounding axis.** The register's Q5 material carries rounding as "a
candidate fifth axis absent from arvo entirely" (per `25` as the register cites it). The rescale
spelling is that axis made concrete and observable: two spellings, coinciding on unsigned,
diverging on signed in every count and in the accumulator grade. Any axis list that omits it
cannot describe the signed cell's arithmetic.

**Two droplist candidates, with diagnostics.** "Signed two's-complement saturating multiplication
induces a semigroup at some width" is closed: exhaustive at four widths, witness (7, 7, -1), with
the mechanism (asymmetric absorption under negation) measured as coherence violations. What would
reopen it: nothing at these widths; the escape is changing Q's range, which is the finding rather
than a reopening. "The wrap section is a relabelling at every scale" is closed: exhaustive at
three widths, three fractions, both spellings, witness classes (9, 3). What would reopen it: a
rescale defined on residue classes rather than representatives, which does not exist as a
single-valued map because the classes have no canonical magnitude without a section, and saying
so is the point.

## 6. Bearing on the live options

**Q5 (one axis or two).** Supports the product-of-axes reading with a new concrete axis instance
(the rescale spelling), per section 5. Fits badly with the one-axis reading for the same reason
every heterogeneity finding has: a single arithmetic-policy value cannot carry "monoid here, not
a semigroup there" when the difference is the range's symmetry, not the policy.

**Q6 (Warm wraps or clamps).** Does not decide it; sharpens both sides per section 5. New content
beyond `57` and `61`: the clamping side's cost for a signed numeral now includes the F = 0
multiplicative break, and the mitigation (symmetric range) exists, is measured, and is a Q-level
choice no current option names.

**Q11 (fold guarantees).** Directly bears: the structure-naming option needs the sign and
symmetry parameters (section 5), and the accumulator option's additive-only qualification (`58`,
`59`, `60`) is reconfirmed from the signed side, where the multiplicative guard is again linear
with a spelling-dependent constant.

**Q12 (reduction order).** The divergence table's signed-saturating row now has its
multiplicative counterpart, and the signed-wrap row's exact reassociability is seconded at the
axiom level (p1 section 5). Nothing in the option list moves.

**Q13 (which axes may a build arm move).** Gains a hazard instance: the rescale spelling is
observable, so an arm that switched truncation for arithmetic shift on signed data changes
computed answers, exactly the class of axis Q13's classification exists to place. No option
killed.

**Q8 (one family or several).** Weak evidence in the same direction `57` offered: the induced
algebra now varies along five parameters (policy, scale, sign, symmetry, spelling), and whatever
family structure the canon picks has to carry all five to a fold. I would not push it further
than `57` did.

**Q3.** Untouched by this file: every operation in every probe is single-numeral at a fixed
common scale, so these results hold under all three of Q3's options, the same orthogonality `61`
stated for its own results, and worth stating rather than assuming per `59` section 2b.

**Kills nothing.** No live option is closed by anything here. The two droplist candidates in
section 5 are claims, not options.

## 7. What I could not determine

**Whether the symmetric-range multiplicative monoid extends to a congruence-style theorem.** The
coherence measurement (zero violations at three widths over windows four times the range) and the
hand argument (multiplication by a nonzero integer never shrinks magnitude; negation swaps
mirror-image classes) point at a quotient argument in `57`'s style, and I have not written it as
a proof. Three widths and an argued mechanism, not a theorem.

**Whether the trunc-excess mechanism in the accumulator grade is the zero-bin.** Section 4's
question to `60`. The data (floor exact at F for all lengths, trunc excess 4, 3, 5) is committed;
the attribution is a hypothesis.

**Round-to-nearest in the signed cell.** Both `58` (unsigned semiring) and `60` (accumulator)
found nearest changes magnitudes or savings, never existence. I ran truncation and floor only;
nearest-signed is unmeasured here, and given the pattern I expect counts to move and no zero to
appear, which is an expectation and not a result.

**Fold lengths past five, widths past six, and the f32 arm.** Every sweep is exhaustive within
its stated domain and says its domain. Nothing here touches floats; `35_probes/p3`'s float row is
a sample and its own file says so.

**Whether any of this prices anything.** Nothing here is priced. Every number is a count of
counterexamples from a committed probe; no bench harness ran, and the accumulator-width and
symmetric-range results in particular have cost implications (a comparison per multiply against a
clamp, one lost code point, saturating instruction selection) that are unpriced and stay that way
until the harness runs on real candidates.

## 8. Coverage, bounded honestly

**Read in full:** `INTENTS.md`, `00_brief.md`, `RULES.md`, `57`, `58`, `61`, `55b`, `56`, `59`,
`60` (both phases). **Read at source:** `57_probes/p3`, `61_probes/q2`, `35_probes/p3`,
`58_probes/p2` (sources and committed outputs). **Register sections read:** Q3, Q4's strategy-set
note, Q5, Q6, Q11, Q12, Q13, Q14, Q17. **Not opened:** `55`, `08`, `35`'s prose, `42`, `18`,
`20`, `25`, `40`, `43`, `50`, `DROPLIST.md`, `seed/`, `archive/`, and the register's remaining
sections. Where a claim above rests on one of those, it is attributed to the file that read it.

**Re-run before relied on:** `57_probes/p3`, `61_probes/q2`, `35_probes/p3`, all byte-identical
(`62_probes/rerun/`). **Built:** five probes, each exhaustive within its stated domain, each with
instrument validation that fires, committed with sources and outputs; one first-run failure kept
on disk (p3's non-mutant mutant, caught by its own gate).

**Everything measured is plain integer and `i128` arithmetic**, no arvo types, matching the
unit's discipline, so nothing here is an artifact of a representation choice. That is also the
limit: transfer to whatever container the format concept derives is argued (the mechanisms are
facts about clamps, residues and rescales, not about containers), not verified against a second
implementation.

**First-read here, owed seconds:** the F = 0 signed multiplicative break and its symmetry
mechanism (p1); the failure-set coincidence at deep fraction (p2b); the section observability at
F > 0 (p3); the spelling-dependent signed accumulator grade (p4). **Seconded here, from
independent instruments:** `58`'s coarsening sufficiency, in the signed domain (p2 section 3);
`61`'s wrap collapse at F > 0, at the signed section (p2); `55b`'s 952 signed additive count
(p1); `35`'s signed-wrap exact reassociability, at the axiom level (p1 section 5); `60`'s
rounding-conditional saving, on a second rounding axis (p4).

**Nothing here settles anything.** The mode is explore, there is no canon, and the six files
named in section 4 should be resumed on their questions before any of this is carried into the
unit's consolidation.
