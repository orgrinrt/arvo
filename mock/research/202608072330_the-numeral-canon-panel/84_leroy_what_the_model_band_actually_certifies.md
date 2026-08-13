# 84. What the model band actually certifies, and what replaces the transfer

**Author lens:** Leroy. Semantic preservation, trusted bases, what a verification claim quantifies over.
A validator is sound because it validates the artifact that ships; the exact statement of the theorem is
everything; and the value of a mechanism is measured by how small and how explicit the list of trusted
things is after it has run.

**Position:** sixth expert in the derived-algebraic-laws unit, after `76`, `77`, `79`, `80`, `81`, `82`
and op's `83`. My assigned question is the attack `80` asked for and `82` deferred: whether the model-band
cross-check of `80` section 4.3 certifies what it is claimed to certify, what sits in the trusted base
after it has run, and what would have to be true of a model band for the band-to-shipped-width transfer to
be sound rather than merely unrefuted. I wrote `17` and `68` earlier in this panel under this persona and
have no memory of either; I re-read `68` as one more unratified file, and note that this persona agreeing
with itself across sessions is one instance wearing two hats, not two.

**Probes:** seven committed sources in `84_probes/` with transcripts beside them, committed as they ran and
before this file, including one rerun of the attacked instrument. Every compiled construction below is
gate-free on the pinned `nightly-2026-05-28`; the runtime instruments use spike scaffolding (`Vec`,
boxed closures) that is instrument plumbing and not design shape, per the panel's own probe discipline.

**The headline, stated before the apparatus because it is checkable in one command.** The mechanism is
defeated, by construction, on the fragment it was demonstrated in. `84_probes/p2_defeat_the_cross_check.rs`
is `80_probes/p2c_closed_form_checked_on_a_model.rs` with the law exchanged and nothing else: every check
the mechanism runs is green, the agreement over the model band holds at compile time, the perturbation
control still refuses, the arm is licensed at width 64, **and the licensed law is false at width 64**, with
the refutation costing sixteen const multiplies under `--cfg audit`. `84_probes/p2b_no_band_can_catch_it.rs`
is the stronger member: a law true at every width 1 through 63 and false exactly at 64, so **no model band
whatsoever**, at any guard setting, on any host, could have disagreed with the closed form. The transfer is
not a residue. For this fragment it is, in general, false.

## 0. Gates, and coverage

**Canon gate: passes, situation two.** No canon exists. `mock/canon/` is absent, `mock/crates/` is empty by
the declared mutation order, and this panel is writing the first canon. `INTENTS.md` holds one RATIFIED
entry, I13 (`INTENTS.md:177-198`), ratified narrowly on op's instruction that it means no more than he
said; the other eleven are STATED under the standing instruction that nothing about them is absolute
(`INTENTS.md:40-41`). Checked against: `INTENTS.md` and `RULES.md` in full, and op's `83` at the source.
Nothing below settles anything.

**Test gate: no suite exists; the mock workspace has no members.** The substitute is the probe discipline,
applied to my own instruments first. Two of my instruments failed their first run in ways that would have
produced wrong or hollow sentences, and both are handled on the record rather than silently: `p4`'s first
battery asserted its own coverage bar and failed it, because random odd coefficients pin a law's threshold
at zero, so the thresholds the battery claimed to exercise were mostly degenerate; the fix enriches the
battery (random extra powers of two in the coefficients) rather than lowering the bar, and the committed
source carries the fix with the reason in a comment. And every probe was rebuilt from its committed,
formatter-touched source and re-run after the last commit: all seven reproduce, transcripts byte-identical
where diffed (`p1`, `p3`, `p4`) and outcome-identical for the compile-refusal variants.

**Read end to end:** `INTENTS.md`, `RULES.md`, `83`, `80`, `82`, `79`, `81`, `68`, `76` phase two, `77`
phase two, `OPTIONS.md` Q38 through Q40 (`OPTIONS.md:1870-1953`), `DROPLIST.md:220-260`,
`80_probes/p2c_closed_form_checked_on_a_model.rs` in full, `82_probes/p12_shrinking_the_transfer_residue.rs`
in full. **Read at the source in named ranges:** `76` and `77` phase one section headers only;
`OPTIONS.md` outside Q38 through Q40 only where cited by `80` or `82`. **Not read:** files `01` through
`67` and `69` through `75` except `68`, every probe directory other than `80_probes/` and `82_probes/`,
`PRIOR_CALLS.md`, `PERSONA_CALLS.md`, the seed files. Everything I say about `35`, `42`, `55b`, `63`, `74`
is routed through `80`, `82` or the register, named as such, and inherits their errors.

**Re-ran before arguing:** `80`'s p2c, both variants, on the pin. The honest build accepts in 4.0 seconds
and licenses; the `badclosed` build refuses with the named diagnostic
(`84_probes/p0_rerun_80_p2c.txt`). The instrument I attack is the instrument `80` committed, reproduced
to the digit. I did not re-run `82`'s p12; where I lean on it, section 6 says so.

**Nothing here is priced.** No bench ran. Every compile time below is an ad-hoc quick spike with no
substance for any how-much question; the accept and refuse outcomes are the results.

## 1. What the mechanism actually checks, restated so the attack is on the real thing

`80_probes/p2c_closed_form_checked_on_a_model.rs:108-126` computes one boolean: that a hand-written
closed-form verdict function and an exhaustive swept verdict function return the same answers at widths 2
through 5, and asserts it in a crate-level const. `p2c:134-143` then gates an arm on the closed form
evaluated at width 64. `80:297-299` states the claim: "What is unchecked afterwards is then exactly one
named thing, the transfer of the agreement from widths 2 through 5 to width 64, rather than the whole
verdict." The register carries the claim at `OPTIONS.md:1896-1901` as Q38(c)'s buy line: "the transfer
proviso survives as a single named residue... the declaration is checked against the maps everywhere it
can be." Checkpoint `81:110-117` elevates it to "the escape".

Two readings of that claim have to be separated, because one is true and one is not.

**Reading one, about the band: true.** At widths 2 through 5, the closed form is checked through the maps,
in `68:145-148`'s sense. A closed form wrong anywhere in the band is refused. My rerun confirms it and my
own probes reuse it.

**Reading two, about width 64: false, and it is the reading the arm rests on.** "The unchecked part is one
named thing" implies the band did most of the certifying work and a small, nameable remainder is left. The
construction below shows the remainder is, in general, the entire verdict: there are laws, in the same
fragment, at the same arity or cheaper, for which the band's agreement is exactly vacuous with respect to
the gated width, because the closed form and the truth agree everywhere the band can look and disagree
exactly where the arm fires.

## 2. The defeat, and why it is a family rather than a curiosity

### 2.1 The threshold family

Take the wrapping fragment: expressions built from wrapping multiply, wrapping subtract and integer
constants, which is the fragment `p2c`'s own wrap policy lives in. Define, at arity 1,

    L_k(W):  for all x in Z/2^W:   x(x-1)(x-2)...(x-(k-1))  =  0   (mod 2^W)

"Every product of k consecutive integers vanishes at this width." The product of k consecutive integers is
divisible by k!, and v2(k!) = k - s2(k) by Legendre's formula, with the minimum attained at x = k, where
the product is k! itself. So L_k is **true at every width W <= k - s2(k) and false at every width above**,
with minimal counterexample x = k.

`84_probes/p1_threshold_family.rs` verifies every part of that against exhaustive sweeps rather than
trusting the argument: for k in {2,3,4,5,6,7,8,12,16}, the swept truth set is exactly the initial segment
1..=(k - s2(k)), the first counterexample is exactly x = k, the counterexample embeds upward to the sweep
cap, the two spellings of v2(k!) agree for every k to 128, and a deliberately wrong threshold formula is
caught by the sweep, so the instrument can fail. Signed and unsigned wrapping add, subtract and multiply
are checked bit-identical over every width-8 pair, so the sign axis collapses for this fragment: two's
complement is Z/2^W and there is one ring.

The family's parameter places the threshold anywhere. k = 16 gives truth through width 15. k = 64 gives
truth through width 63, and the wrapping product at the witness is pinned exactly: 64! mod 2^64 is
`0x8000000000000000`, computed and asserted (`p1_output.txt`).

### 2.2 The mechanism, green end to end, licensing a false law

`84_probes/p2_defeat_the_cross_check.rs` is `p2c`'s construction with L_16 as the law, the same model band
2..=5, and the plausible closed form: constant true. The generalisation even has the texture of a real
argument, sixteen consecutive integers contain eight evens, four multiples of four, and so on, so the
product "is divisible by a large power of two". The reasoning is right; its conclusion is wrong by exactly
the amount the band cannot see: the power is 15, not "large enough for any width".

The transcript (`p2_output.txt`, `p2_refusals.txt`):

    agreement over the band, evaluated at compile time: true
    closed_verdict(w = 64) = true   (constant time, no enumeration)
    consumer_that_rewrites_through_the_law::<Numeral64>() = licensed
    ...
    w = 15: true
    w = 16: FALSE, witness x = 16

The `--cfg badclosed` perturbation is refused with the named diagnostic, so the mechanism's own control
still bites; the mechanism is not broken, it is checking the wrong thing. The `--cfg audit` build adds the
one check the mechanism does not contain, the law itself at the gated width on the known witness, and it
refuses in sixteen const multiplies:

    error[E0080]: evaluation panicked: the licensed law is FALSE at the gated width: the
    product of the sixteen consecutive integers ending at x = 16 is 16! = 2^15 * odd,
    nonzero mod 2^64

Everything `80` built works as designed, and the arm computes wrong answers at width 64.

### 2.3 Widening the band cannot repair it

The defender's reply is that the band was too narrow, and `84_probes/p2b_no_band_can_catch_it.rs` is the
answer: L_64, true through width 63, false exactly at 64 and above. The const band there is 2..=8; the
runtime sweep confirms truth through width 22; the valuation argument, verified in every consequence a
sweep can reach, carries truth through 63; and the width-64 witness residue is asserted to be exactly
`1 << 63`. For this law the constant-true closed form is **right at every width except the gated one**.
There is no band below the shipped width whose agreement check could have failed, not at any guard
setting, not on any host, not with any encoding. Widening the band buys widths the family has already
paid for.

This is the strongest form of the statement `unstable-features.md:54-81` already carries for the
model-width argument in general: a check at a model width establishes something about the model width. The
droplist holds a compiled counterexample to the transfer through const-tag dispatch
(`DROPLIST.md:230-232`); the family here is stronger in one respect that matters to this unit: **it needs
no dispatch, no gate, no width-keyed path, and no policy trick.** One parametric body, ring operations and
constants below 64, and the property moves with width because the quantifier's domain moves with width.
The transfer proviso `68:213-219` states ("no law-relevant path dispatches on width") is therefore not
sufficient either: L_64 satisfies it and still flips at 64.

### 2.4 The natural member, so this is not filed as exotic

The family is not a pathology built to embarrass a probe. Its mechanism, congruence facts that hold mod
2^64 reduce correctly to every narrower width and say nothing about wider ones, is exactly the shape of
every magic constant in systems code. `p1` runs the sibling: INV3 = `0xAAAAAAAAAAAAAAAB`, the inverse of 3
mod 2^64. The law `(x * 3) * INV3 == x` is true at every width 1..=18 swept, is true at 64 by the defining
congruence, and is **false at width 128**, because 3 * INV3 is 2^65 + 1, computed and asserted in u128
with the witness printed. A design that validates its strength-reduction constants on a model band and
ships at the constant's own width is safe by accident; the day the shipped width widens, every band still
agrees and the law is false. Multiplicative inverses, Montgomery constants, magic division constants and
reciprocal approximations all have this downward-correct, upward-false shape, and the band's blind
direction is exactly upward.

## 3. So what does the cross-check certify, and what is trusted after it runs

**Certified:** that the closed-form function and the swept function agree at the band widths, through the
maps, on that toolchain and host. That is a real theorem about the spelling of the two functions, it is
worth having, and it is all there is.

**Trusted after the run, itemised**, extending `68:221-253`'s list where the law layer lands on it:

1. **The transfer itself, per law, and it is not a proviso, it is the verdict.** For any law whose
   threshold exceeds the band, the band contributes nothing to the width-64 claim; sections 2.2 and 2.3
   exhibit the class. Calling it "a single named residue" (`OPTIONS.md:1901`) mistakes the name for a
   bound on the size.
2. **The width-indexing of the law family.** A "law" checked at several widths is a family of statements,
   one per width, and how its constants embed at each width is an author's convention nothing checks. The
   family makes this concrete: at widths where 2^W <= k, every x has a zero factor and L_k's band members
   are true **vacuously**, a structurally different reason than the valuation that carries the wide
   members. The band cannot distinguish "true for the closed form's reason" from "true degenerately".
3. **The sweep's own encoding.** `82`'s F17 established the frontier reads the encoding; `p2c:30-33`
   hard-codes `MODEL_MAX_W = 5` as "the widest width at which the swept form is evaluable", citing
   `p2_frontier.py`, whose per-tuple encoding is not `p2c`'s. The constant is a claim about a different
   instrument. Nothing checks it, and a slower host or heavier encoding turns the band's top width into a
   refusal, which `80:301-303` itself flags.
4. **The reachability rung of the gate.** `p2c`'s agreement is a crate-level const, rung 0 of `82`'s F8
   ladder, unskippable, correctly placed. Its per-instantiation licence is an associated const in a
   generic impl, rung 3, evaluated only where reached. Section 7 argues that split is right, not wrong,
   but it is a trusted convention until stated.
5. **rustc's const evaluator, the pin, and the host**, as always, plus the guard's time budget, which
   makes the band's very existence host-relative.

The one-sentence form, which is this persona's home ground: **translation validation is sound because the
validator runs on the artifact that ships. The model-band cross-check runs the validator on every artifact
except the one that ships.** The width-64 instance is precisely the member of the family the band never
evaluates, and agreement on the other members is data about them.

## 4. When a band verdict does transfer, exactly, and the direction flips with polarity

The workspace correction (`unstable-features.md:54-81`) says a model-width check needs its own named
transfer argument, the honest default being none, and names three shapes: a symmetry, a saturation point,
an induction on the parameter. The unit has been treating that as prose. For the wrapping fragment it can
be made exact, and the exact form has a structure nobody in this unit has stated: **which direction
transfers is decided by the polarity of the law.**

Both halves follow from one fact, verified and then used: reduction mod 2^(W-1) commutes with the wrapping
ring operations, and integer polynomials respect argument congruence. `84_probes/p3_transfer_polarity.rs`
checks both consequences exhaustively over widths 1..=16 on the threshold family, on mixed-coefficient
polynomial laws, and on a disequation family, with zero defects:

**Equation laws** (`forall x: p(x) == q(x)`): truth is downward closed, so the truth set in width is an
initial segment, and a counterexample at a model width embeds upward literally, same witness, every wider
width. **A band FALSE is therefore sound at every shipped width. A band TRUE is evidence of nothing
above the band**, and the family realises the failure at every threshold.

**Disequation laws** (`forall x: p(x) != q(x)`): truth is upward closed. A width-W solution of p == q
reduces to a width-(W-1) solution, so no-solution transfers upward: `x + 32 != x` is false at widths 1
through 5, where 32 reduces to zero, and true from width 6 up, a final segment. **For this polarity a band
TRUE at the top of the band is sound at every wider width**, a genuine positive band-transfer theorem, and
a band FALSE is the direction that proves nothing.

Two consequences for the unit's standing results. First, `80` section 4.2's cost asymmetry (false cheap,
true refused) composes with the transfer asymmetry into something sharper than either: for equation laws
in this fragment, the verdict that is cheap at the band is also the only one the band can export, and the
verdict that is expensive is also the one the band cannot export at any price. The mechanism is doubly
inverted for licensing arms. Second, the polarity split is a property of the **fragment**: outside ring
terms, neither closure holds and neither transfer direction exists. `p2c`'s own negative half, the
saturating verdict declared false at 64 on band falsity, rests on a transfer with no theorem behind it,
because saturation is not reduction-stable; it happens to be right, and the honest instrument for it is a
shipped witness evaluated at the gated width, one map evaluation, which is what my `--cfg audit` consts
are. **Falsity never needs a band: it needs a witness, checkable at the shipped width for any law of any
shape in constant work.** The band's negative half should be replaced by witnesses; its positive half, for
this fragment, is replaced by section 5.

What would have to be true of a model band for the positive transfer to be sound, answered exactly: the
law's truth-in-width function would have to be determined by its values on the band, which for equation
laws in this fragment means a stated, computable bound on the law's threshold with the band reaching it.
Section 5 shows that whenever such a bound is computable the exact threshold is computable too, so **the
band is never the load-bearing instrument: where a transfer argument exists the band is redundant, and
where none exists the band is unfounded.** Its sound job, in every case examined here, is validating the
implementation of a checker, which is reading one of `68`'s list items back at the right size.

## 5. The constructive replacement: the verdict is decidable at the gated width, for this fragment

This section is the rabbit hole rather than the report. Having shown the band cannot carry the positive
verdict, the obligation is to find what can, and for the whole fragment the defeat lives in, something
can: **the positive verdict at the shipped width is computable at the shipped width, in constant work per
degree, with no sweep, no band and no transfer.**

The classical fact (Kempner 1921, Singmaster 1974, restated for m = 2^W): write an integer polynomial in
the falling-factorial basis, p = sum a_j (x)_j. Then p vanishes identically on Z/2^W exactly when 2^W
divides a_j * j! for every j; sufficiency because (x)_j takes values j! * C(x, j), necessity by
unitriangular evaluation at x = 0, 1, 2, in order. And a_j * j! is the j-th forward difference of p at 0.
So:

    "forall x mod 2^W: p(x) == 0"
        <=>   all forward differences of p at 0, orders 0..=deg, vanish mod 2^W

The right side costs deg+1 evaluations **through the wrapping map itself, at the target width**, plus a
difference triangle. Wrapped evaluation points are harmless because the map respects argument congruence.
The multivariate form takes mixed differences over a box of size prod(deg_v + 1), by the same argument per
variable. The exact truth threshold is the same computation over the integers: W* is the minimum 2-adic
valuation of the nonzero differences.

`84_probes/p4_difference_certificate.rs` refuses to trust that and measures it. A battery of 311
univariate laws (falling factorials, scaled monomials, the two hand polynomials from p3, and 300 random
polynomials with coefficients carrying random extra powers of two so 267 of the thresholds land strictly
inside the swept range) is checked against exhaustive sweeps at every width 1 through 12: **3,732
(law, width) pairs, zero mismatches for the exact certificate and zero for the through-the-map modular
form.** The multivariate form is checked against exhaustive sweeps for arity-3 wrapping-mul associativity,
arity-3 distributivity, a false trivariate law with threshold 3, and an arity-4 chain identity: zero
mismatches. Then it is used at the width the band could never reach: at width 64, L_16 is decided false in
17 evaluations, L_64 false in 65, L_128 **true** in 129, associativity and distributivity true in 27 each,
the arity-4 chain in 16.

The procedure's trusted inputs are two, and both are shown live by negative controls that produce wrong
verdicts on demand: **fragment membership** (a saturating law fed to the test yields true against a
sweep's false, because a clamp is not a ring term and the clamp never fires at the difference points) and
**the degree bound** (claiming degree 2 for (x)_4 yields true against false, because the withheld points
carry the whole content). Both are syntactic facts about the expression tree, checkable by inspection of
the law's spelling rather than by any sweep.

`84_probes/p4b_certificate_gate.rs` is the design-shaped consequence: `p2c`'s construction with the gate
replaced. The arm's licence evaluates **the law itself at width 64** through the difference criterion; a
rung-0 crate-level const validates the criterion's implementation against the sweep at widths 2..=8, which
is the model band demoted to the one job section 3 licenses it for. The default build licenses L_128 and
wrap associativity at width 64, in under a second of compile time against `p2c`'s 4.0 seconds; `--cfg
use_l16` and `--cfg use_l64` are refused with `E0080` naming the falsity, decided at width 64, which are
exactly the two laws the band mechanism licensed in sections 2.2 and 2.3. There is no declared verdict
anywhere in the construction: the closed form, the table entry, the thing `68:145-148` warned is paper, has
been replaced by a decision procedure whose correctness argument is a stated theorem with a three-line
proof, whose implementation is validated through the maps where sweeps exist, and whose evaluation happens
at the width that ships.

**The frontier consequence, and it is the extreme instance of `82`'s F17.** `80`'s (width, arity) frontier
table, `82`'s one-bit sign-uniform dent (F16) and one-bit encoding dent (F17) are all facts about the
sweep procedure. For this fragment, changing the procedure moves the arity-3 positive-verdict frontier
from width 5 to every width at once. `OPTIONS.md:1888-1889`'s clause that at a shipped width the compiler
"produces only NEGATIVE verdicts" is a statement about Q38(a)'s sweep encoding, not about the verdict:
inside the ring fragment, positive verdicts at width 64 cost a degree-bounded box of map evaluations. The
`2^(W*k)` wall is real and remains the price of **semantic ignorance**, paid exactly when nothing about
the law's structure is known; it is not the price of a positive verdict.

**The boundary, stated so this is not over-read.** The procedure covers ring terms only: wrapping
add/sub/mul/neg and integer constants, F = 0. Saturation, rounding, shifts and comparisons are outside,
and `84_probes/p4`'s first negative control is the compiled demonstration of what happens if that is
forgotten. The unit's schedule-conditional chain laws (`80` section 4.5) involve rounding and stay out of
reach, exactly as `80`'s O-H(c) placed them. `82`'s sign-uniform saturating result is likewise outside;
its closed form rests on its own structural argument (`82` section 15 item 1, the min/max identity checked
to width 16 with a three-line unmechanized residue), and that argument, not `82`'s band, is what carries
its width-64 claim. Naming that is a relocation, not a demotion: `82`'s construction survives because it
has an O-H route; what does not survive is the idea that the band supplied it.

## 6. What this does to the unit's standing results, named one at a time

**`80` section 4.3 and its p2c.** The instrument is honest, reproduces, and its wrap instance is correct
at width 64, because "Z/2^W is a group" is a real structural argument (O-H(b)), independently derivable by
the difference criterion (p4b licenses it with no hand-written verdict at all). What is defeated is the
mechanism's general claim, `80:297-299`, and its register form. The sentence "the unchecked part is one
sentence rather than the whole verdict" (`80:688-689`) is false as a general statement about the
mechanism: for band-undetermined laws the unchecked part **is** the whole verdict, and section 2
constructs them at every threshold, at arity 1, in the mechanism's own fragment. `80` named this the piece
it most wanted broken (`80:807-809`) and stated the dependency plainly; the flag was correct and the
mechanism as a transfer-certifier is now broken on the record.

**`82`'s p3a, p3d, p7.** Three constructions carrying the cross-check, extended rather than tested
(`82:1004-1005`, its own words). They stand, for the reason above: their law's width-64 claim rests on the
p12 identity argument, not on the band. But the description each inherits from `80`, agreement checked on
a band with "the transfer proviso" as the residue, misstates where their soundness lives, and `82`'s least
certain item 1 (`82:930-938`) already sensed this: it pushed the identity as wide as sweeps go and named
the last step as prose. The honest restatement for all three: the band validates the closed form's
implementation; the three-line integer argument carries the verdict; that argument is a trusted-base item
until mechanised, and it is the only load-bearing unmechanised thing in the construction.

**`81`'s framing.** The checkpoint calls the mechanism "the escape" and "the only route to a shipped
width" (`81:110-117`, `81:248-250`). After section 5 there are two routes, and for the wrapping fragment
the second is strictly stronger: it produces the positive verdict at the shipped width itself. The
checkpoint's sentence was right about the evidence that existed when it was written.

**Q38, Q39, Q40.** Q38(c)'s buy line (`OPTIONS.md:1896-1901`) is corrected per section 3: the maps-check
is band-local and the residue is unbounded, not single. Q38(a)'s negative-only clause
(`OPTIONS.md:1888-1889`) is procedure-relative per section 5. Q40 gains the route below. Q39 is untouched:
everything here is const-available and typestate-or-const in `83`'s sense, and nothing bears on
value-gated arms.

**The option this file adds, written out in full so it is not lost.**

**O-J. Where a law verdict's truth is established, fourth answer: decided at the gated width by a
width-generic decision procedure.** For laws whose two sides are ring terms over the wrapping fragment,
the verdict at width W is computed through the width-W map itself by the finite-difference criterion, in a
box of prod(deg_v + 1) evaluations, sound and complete, const-evaluable, with the exact truth threshold
available as the same computation over the integers. Cost: fragment membership and the syntactic degree
bound become trusted inputs, both checkable from the law's spelling, both shown to produce wrong verdicts
when violated (p4's negative controls); multivariate cost grows with the degree box, 2^k for a
length-k multilinear chain law, which is 256 evaluations at k = 8 against the sweep's 2^(8W). Buys: the
positive verdict at the shipped width, which (a) cannot produce and (b) and (c) only assert; no model
band; no transfer in the trusted base at all. **What would distinguish it from (c) for a given law:**
whether the law's operations are all ring operations. Where they are, (c) is strictly dominated: its
closed form becomes a cached answer the procedure recomputes at will. Where they are not, (c) survives
with its residue restated per section 3: the band validates the spelling, a per-law structural argument
carries the verdict, and the argument is named in the trusted base. **And for negative verdicts of any
law, fragment or not: a shipped witness evaluated at the gated width, one evaluation, no band.**

**The verdict-table architecture this implies**, offered as the constructive shape of the whole answer:
a law inventory needs no row whose content is "believed, checked at model widths". Every row is one of
three things, each carrying its own instrument: a **witness** (falsity, checkable at any width in one
evaluation), a **procedure verdict** (ring-fragment truth, computed at the gated width), or a **named
argument** (non-fragment truth: a structural theorem like the group or absorbing-clamp arguments, cited
into the audit trail, with the band validating its implementation where sweeps reach). What has no place
in the table is the fourth kind the unit almost canonised: a truth claim whose entire support is band
agreement. Section 2 is what that kind is worth.

## 7. The binding-time ladder, and which rung the mechanism's parts belong on

The brief's second question, taken as far as my findings reach and no further.

`82`'s F8 measured four rungs and named rung 3, the const assert in a generic's associated const, the
weakest: a wrong declaration in unreached code compiles clean. `p2c` splits across the ladder, and the
split is the correct design rather than a defect: the **verdict table and its validation** (the agreement
const) sit at rung 0, attached to no function, unskippable, which is where a claim about the library's
stated laws belongs; the **licence** sits at rung 3, per instantiation, which is where a claim about an
arm belongs, because its quantifier is "every instantiation that is reached", and an unreached
instantiation has discharged nothing and claimed nothing. That is the observation-perimeter shape: the
guarantee holds over the surface through which the thing can be observed, and a generic arm is observed by
instantiating it. p4b keeps the same split deliberately: implementation check at rung 0, per-law licence
at rung 3, with the file's reached instantiations forced through top-level consts so the transcript cannot
be read as covering arms it never evaluated.

What turns rung 3 from a property into a defect is exactly one move: reading "the crate compiled" as
"every declaration in it was checked". `82`'s p3b measured that misreading; the specification that
prevents it is one sentence, **a rung-3 permission quantifies over reached instantiations and a library
claim must sit at rung 0**, and both mechanisms in this unit already conform to it once it is said. So my
answer to "defect or property": property, and it is one sentence of specification away from being safe,
which is cheaper than any repair that tries to force library-wide evaluation of generic gates.

## 8. What else in the unit's evidence is a fact about a procedure rather than a law

The brief's third question. `82`'s F17 showed the frontier reads the encoding, one bit. Section 5 shows it
reads the decision procedure, all the bits. Sorted into bins, for the consolidation to carry:

**Facts about laws** (survive any procedure change): every exhaustive verdict at a reached width, the
threshold family's truth sets, the polarity closure theorems, `82`'s F6 equivalence, the group and
absorbing-clamp arguments.

**Facts about the sweep procedure** (true, and cited as if about laws): the (width, arity) frontier table
(`80` section 4.1, corrected by `82` F17), the negative-only-at-shipped-width asymmetry (`80` section
4.2, `OPTIONS.md:1888-1889`), the guard-buys-three-bits measurement (`80` section 4.4), the sign-uniform
one-bit purchase (`82` F16). Each is a real property of "verdict by enumeration" and none survives a
change of procedure; anything the canon carries from them owes the procedure name in the predicate, the
same way F17 established the encoding is owed.

**Facts about the mechanism's bookkeeping**: `p2c:30-33`'s hard-coded band ceiling, measured by a
different instrument's encoding; host-dependence of the band via the guard's time budget.

## 9. Findings, in the required predicate notation

Absence of a dimension is the strongest negative statement in the notation and is meant wherever it
appears. Threads is 1 throughout because every instrument ran on one thread and nothing about concurrency
was checked; features is `any` where stated on `80`'s own ground, that the computations are pure value
functions with language-specified semantics.

**F1. The wrapping fragment contains equation laws with truth threshold at every attainable value, and
the threshold, first witness and upward embedding follow the 2-adic valuation exactly.** `policy = wrap,
F = 0, ops = {wrapping mul, wrapping sub, integer constants}, arity = 1, sign any (signed and unsigned
wrapping are bit-identical, measured over all width-8 pairs for add, sub, mul), widths = 1..=18 exhaustive
per member with embedding checked to 22, threads = 1, features any`. For k in {2..8, 12, 16}: truth set
exactly 1..=(k - s2(k)), first witness exactly x = k, witness embeds to the cap. For k = 64: truth
measured at 1..=22, residue at the witness pinned to exactly 2^63 at width 64; the stretch 23..=63 is the
valuation theorem's claim, verified in every sweepable consequence, and is an argument rather than a
measurement, stated as such.

**F2. `80`'s model-band cross-check mechanism licenses a false law at the gated width with every one of
its own checks green.** `toolchain = nightly-2026-05-28, host = aarch64-apple-darwin, policy = wrap,
F = 0, arity = 1, model band = widths 2..=5 (p2) and 2..=8 (p2b), gated width = 64, threads = 1`.
Agreement asserted at compile time, perturbation refused, arm licensed, law false at 64, refutation by
one witness in 16 (respectively 64) const multiplies under `--cfg audit`. For the p2b member the truth
set is 1..=63, so no band below the gated width can distinguish the closed form from the truth.

**F3. Verdict transfer direction in the wrapping fragment is decided by law polarity.** `policy = wrap,
F = 0, ops = ring terms with integer constants, arity = 1 measured, widths = 1..=16 exhaustive, threads =
1, features any`. Equation laws: truth sets are initial segments and every first counterexample embeds
upward unchanged, 6 of 6 laws, zero defects; so a band FALSE is sound upward and a band TRUE exports
nothing. Disequation laws: truth sets are final segments, 5 of 5 laws; so a band TRUE at its top width is
sound upward and a band FALSE exports nothing. The closure arguments are width-general; the measurement is
the stated range. No claim for any operation set beyond ring terms: saturating, order-predicated and mixed
forms are absent from this predicate deliberately.

**F4. The finite-difference criterion decides equation laws of the wrapping fragment, agreeing with
exhaustive sweeps everywhere both exist, and computes the verdict at width 64 in a degree-bounded number
of map evaluations.** `policy = wrap, F = 0, ops = ring terms with integer constants, arity = 1 at
degrees 0..=16 and {2, 3, 4} multivariate at per-variable degree <= 2, widths = 1..=12 univariate and
1..=6 (arity 3), 1..=8 (arity 2), 1..=5 (arity 4) multivariate, plus point verdicts at width 64, threads
= 1, features any`. 3,732 univariate (law, width) pairs, zero mismatches for both the exact-threshold
certificate and the through-the-map modular form; zero multivariate mismatches; width-64 verdicts: L_16
false, L_64 false, L_128 true, mul-associativity true, distributivity true, the 8a-perturbed law false,
the arity-4 chain true. Trusted inputs are fragment membership and the degree bound; violating either
produces a wrong verdict, measured by two negative controls.

**F5. A const gate on the criterion refuses at width 64 the two laws the band mechanism licensed, and
licenses two true ones, with the band demoted to a rung-0 implementation check.** `toolchain =
nightly-2026-05-28, host = aarch64-apple-darwin, policy = wrap, F = 0, gated width = 64, threads = 1`.
Default build licenses L_128 and arity-3 wrap associativity; `--cfg use_l16` and `--cfg use_l64` refuse
with E0080 naming falsity decided at width 64; the implementation check validates the criterion against
sweeps at widths 2..=8 for three family members.

**F6. The INV3 constant law is true at every width up to its constant's defining width and false
above.** `policy = wrap, F = 0, ops = {wrapping mul, the 64-bit constant INV3 reduced per width}, arity =
1, widths = 1..=18 exhaustive plus the width-128 witness in u128, threads = 1, features any`. 3 * INV3 =
2^65 + 1 exactly, so the law holds through width 65 and fails at 128 with witness x = 1, computed.

## 10. Fits against the register

**Kills nothing.** No option closes; nothing moves to `DROPLIST.md`. The corrections below are stated
here, per `RULES.md:509-518`, and reach the register through consolidation; `80`'s and `82`'s files stand
as written.

Q38 gains O-J, written out in section 6, and two corrections: (c)'s buy line is band-local and its residue
unbounded (section 3); (a)'s negative-only clause is a fact about the sweep procedure (section 5). Q40's
route (a) is subsumed for polynomial grouping laws by O-J's procedure, which needs no per-law lifting
theorem; route (b) survives as the non-fragment route and gains the requirement that the argument, not
the band, be named as the trusted item; route (c) is confirmed as the residual class, now characterisable:
laws that are not ring terms and have no structural argument. Q25 gains the verdict-table architecture of
section 6 as a candidate shape: witness rows, procedure rows, argument rows, and no band-supported rows.
The `81` checkpoint's "still owed" list is discharged on its second item: the attack on section 4.3 has
now been run, and the mechanism's transfer claim did not survive it while both shipped instances of the
mechanism did, for reasons the mechanism itself does not state.

**Where the findings sit relative to the questions not mine to answer.** Q-A: every construction here is
compile-time, per type; nothing bears on runtime validation of a datum, and the witness rows of section
6's table are compile-time constants, not ingest checks. The genuinely-non-const question `83` leaves open
is untouched: every predicate in this file is const-available. Q-B: nothing above depends on `no_std`,
`dyn` or `TypeId` in either direction; the probes' runtime scaffolding uses spike plumbing, and the
const-gate constructions use none of the banned features.

## 11. Where this file is least certain, as a floor for whoever attacks it

1. **The k = 64 truth stretch from width 23 to 63 is an argument, not a sweep.** The valuation theorem is
   verified in every consequence a sweep reaches and in the pinned residue at 64, and it is three lines of
   number theory, but in this panel's notation the stretch is claimed on the theorem's authority and I say
   so. An attacker who distrusts it should note the defeat does not need it: L_16's threshold sits inside
   the swept range end to end (F1, F2).
2. **The multivariate difference criterion is verified at small degree boxes and stated from a proof
   sketch.** The univariate necessity argument is classical; the multivariate tensor version I argued per
   variable and verified at arities 2 through 4, per-variable degree at most 2, zero mismatches. A wrong
   generalisation would surface first at higher per-variable degree, which nothing here reaches.
3. **The polarity theorems are measured at arity 1.** The closure arguments are arity-general (reduction
   commutes coordinatewise), but no multivariate polarity measurement exists in this file.
4. **The fragment boundary is sharp in the probes and blunt in prose.** "Ring terms only" is decidable
   from a spelling, but I have not enumerated where the numeral tower's real operations sit relative to
   it; F = 0 wrapping arithmetic is inside, everything with a clamp, shift-with-rounding or comparison is
   outside, and the mixed cases (a ring expression guarded by a comparison, a shift by a constant) are
   unexamined.
5. **Coverage is bounded.** I did not open `35`, `42`, `55b`, `63`, `74`, or any probe directory beyond
   `80_probes/` and `82_probes/`; my account of the sign-uniform construction's actual dependence on p12's
   identity is my reading of `82` sections 8, 12 and 15 plus p12's source, not of its p3 sources line by
   line.

**Not done, and what it leaves.** The cheapest next instances: mechanise the three-line valuation argument
and `82`'s min/max argument as const checks, which would empty the last prose slots in both files'
trusted bases; point the difference criterion at the numeral tower's real F = 0 expressions once a design
exists, since the degree-bound extraction is the only new machinery a design would need; and search for a
saturating threshold family, a law with clamps true at all sweepable widths and false at a shipped one,
which is the one construction that would extend section 2's defeat outside the ring fragment and against
which `82`'s structural argument is the only defence standing. Nothing here is priced; no bench ran.

**Nothing here settles anything.** The mode is explore. This file goes to whoever attacks next, and the
first thing worth attacking is item 2 of section 11.
