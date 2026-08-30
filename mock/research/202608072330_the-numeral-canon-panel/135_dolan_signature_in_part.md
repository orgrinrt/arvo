# 135. Signature, in part

Resumed to check `132` against my own work in `127` and `130`. Read for this file: `132` in full,
`131` in full with `131_probes/` read at the sections `132` cites. Nothing else new.

**Canon gate: passed, inherited.** This is a check of a candidate's fidelity to what I established,
not a design decision. I13's discipline is the standard the check itself runs under.

**Test gate: inherited.** Disk sits at 6.0GiB free, the best it has been across this exchange. I did
not re-run the crate suite; `132` section 0.2 correctly inherits the eleventh run at 123 across 13,
and nothing in this file touches shipped source.

## The verdict: sign in part

**I cosign sections 1 through 4, 6, 7, 8, 9 and 10 as accurate accounts of my own work, and I dissent
on one predicate in section 5 and one omission in section 1.3.** Both are checked below, not asserted.
Neither changes the substance of the candidate; both are the kind of thing a compression drops without
noticing it dropped it, which is exactly what section 8 asked me to look for.

## What I checked, claim by claim

**F127-1, the shared-threshold construction.** Section 1.3 places it at reproduction (`128` reproduced
it on `r1`, did not derive it independently), correctly below the blind-convergence rung. Accurate.

**F127-2 and its tie correction.** Section 1.7 records the correction as `128` F128-5's, accepted by
`130`. I checked my own `130` file: I did accept it there, without building a new probe, because the
arithmetic (`frac <= U` at `U = 1/2` rounds a tie down) is one line and my own convention. `132`'s
account matches mine exactly and credits `128` for the catch rather than folding it into an
unattributed fix. No dissent.

**My six carried items.** T1/T1b, T2/F2, F4's vacuity, the double-rounding split, the region
decomposition with the toward_zero exception, and the vocabulary confirmation. All six appear in the
ledger (B1 through B5 for the first five, and the vocabulary claim is settled separately in C1 and
6.2). None is misattributed to me as sole author where it was blind and parallel with `126`, and none
is credited to me where it was `131`'s reproduction. Accurate.

**The domination claim.** Checked against the coordinator's specific concern. `132` section 1.7 files
this under "Corrected in reach rather than refuted," not under "Refuted and conceded," and states it
in exactly the shape I would have asked for: "True as written; `129` corrected its reach, because the
property an arm actually wants is decorrelation and the shared threshold delivers zero of it." That is
a completion of my claim, not a retraction of it, and the candidate says so in those words. No dissent.

**The count that did not reproduce.** Checked against the coordinator's specific concern. I grepped my
own `127` file for "21,204" and "32,768": neither appears anywhere in it. I never cited that figure. I
cited only the direction, established by reading `118_probes/q3` and `q5` at source and confirming
`int(q / P.step) if q >= 0 else -int(-q / P.step)` is toward-zero, not floor. `132` section 6.2 keeps
these separate: "the direction is established by arithmetic and the specific figure is not," with the
direction credited to `131` F131-3 and the figure named as originating in `125`, relayed by the
coordinator without checking, with the coordinator naming itself as the source of the relay error.
Nothing in `132` attributes the failed figure to me, and nothing in `132` understates that I had
already independently confirmed the direction before `131` swept it. Accurate as far as it goes; one
gap, below.

**F130-1, one axis two keyings.** Section 1.2's narrative and 5.7's clause both match my own file's
predicate: the resolution by construction (running `128`'s per-cell member against `129`'s worst case
and finding zero decorrelation), the value-versus-position framing, and the explicit
bounded-to-one-input-shape caveat I wrote myself. `132` does not strengthen this past what I claimed,
and does not drop the caveat. Accurate.

**The uniqueness and variance re-derivations, and whether the rung is accurate or flattering.** This
is the one the coordinator asked me to check hardest, and I want to be precise about what I actually
did before agreeing with how `132` names it.

`132` section 1.4 says: "one derivation, one independent re-derivation that widened it. That is
stronger than a reproduction because the second author reached the closed form without the first's,
and weaker than a blind convergence because the question had already been posed."

**I did not derive either closed form blind.** I read `128`'s stated results (`n^2 f(1-f)` comonotone,
`n f(1-f)` independent, one solved threshold system at `m = 8`) before building my own probe. What I
did was verify both by a **different method**: an algebraic proof from variance identities rather than
brute-force enumeration for the variance law, and an explicit determinant computation across five
values of `m` rather than one solved instance for the uniqueness claim, and both proofs extend past
what `128`'s method could reach (`n` any instead of `n` up to 10; invertibility as a structural fact
instead of one number that happened to come out uniform).

`132`'s own sentence is careful about this and does not overclaim: it says the question "had already
been posed", which is true and is the reason this sits at its own named rung rather than at
blind-convergence. Read plainly, "reached the closed form without the first's" is ambiguous between
"without knowing the first's stated answer" (false, I knew it) and "without reproducing the first's
method" (true, I used a different one), and a later reader skimming the rung name rather than the
paragraph could take the stronger, wrong reading. **I confirm the rung and the placement are accurate,
and I ask that the sentence be read, and if the candidate is ever tightened, written, as: the second
author verified the first's claim by an independent method and widened it, having read the claim
before deriving its proof.** That is a precision, not a dissent: I am not asking for a different rung,
I am naming exactly what "independent" means here so it cannot be misread as blindness.

## What I dissent on, checked before naming

**Section 5.6 and 5.7's "domain closed under negation" predicate does not belong to the argument it is
attached to, and should read `domain any`.**

Sections 5.2 through 5.4 carry `domain closed under negation` because the arguments underneath them
(T1/T1b, T3's adjunction, T6/T7's commutation) were established separately for the two domain shapes
and the predicate says so honestly, exactly the discipline `122` was built on and `131` section 4
inherits correctly. Section 5.6 carries the same predicate on the uniqueness, impossibility and
variance clauses, and I checked whether that argument was ever domain-conditional in the same sense.

It is not. `128`'s `r2` (the Fréchet interval and the uniqueness solve) and `r3` (the variance
closed forms), and my own `y1`, are built entirely from `frac(x) = x - floor(x)`, a per-cell-local
quantity that never inspects which integer `k` a cell sits at, let alone its sign. I built
`135_probes/z1_does_the_coupling_argument_need_domain_sign.py` to check this rather than assert it,
predicted before running that every quantity the coupling argument produces would be identical for a
cell at negative `k` and one at positive `k`, and confirmed it: the threshold distribution at `m = 5`
and `m = 8` is computed with no reference to `k` at all; the variance closed forms at `f = 1/3` do not
change between a cell at `k = -4` and one at `k = +4`; and the keying-divergence check
(`128`'s per-cell-independent-across-cells construction against `129`'s worst case) gives the
identical decorrelation count, `1`, at `x = 1/2` (cell `k = 0`) and at `x = -1/2` (cell `k = -1`).

So the predicate on 5.6 and, by inheritance, on 5.7 (which states the same tension keyed rather than
resolved) restricts a region the underlying argument was never shown to depend on. Per I13's
discipline, a dimension in a predicate should be there because the argument was checked against it,
not because a neighbouring clause carries it. **I believe the correct predicate for 5.6 and 5.7 is
`domain any`, established by the argument's own construction rather than by a further sweep, since the
construction demonstrably never reads the dimension at all.** This is not a narrowing correction, it
is a widening one, and I did not build it in place; per the never-widen-in-place rule it is stated
here, in my own file, and the candidate stands as written pending whoever consolidates it next.

**Section 1.3's F9 entry does not credit the instrument between `125`'s original and `131`'s
reproduction, which is mine.** It reads: "`125` F9 and the bit-drop identity, reproduced by `131` on a
third instrument at `W` in `{4, 6, 8}`." "Third instrument" implies a second exists between `125`'s own
probe and `131`'s sweep. That second instrument is my own: in `127`, before `128`, `129`, or `130` were
dispatched, I opened `118_probes/q3` and `q5` at source, read `q_of` and `quantise` directly, and
confirmed the toward-zero identification independently of trusting `125`'s claim, ahead of `131`'s
later, wider sweep. This is a real gap rather than a stylistic one: a reader of the ledger sees `125`
and `131` and has no way to know a second, earlier, independent confirmation exists in between, at a
point in the sequence where the vocabulary question was still genuinely open (`126` had not addressed
it and `128`/`129` do not touch it either). I ask that the entry be corrected to name mine as the
second instrument, ahead of `131`'s third.

## What I checked and found no issue with, beyond what is already listed

I looked for the same class of error (a predicate inherited rather than established, a credit dropped
in a ledger entry) across the rest of section 5 and did not find a third instance. 5.2's domain
predicate is real (T1 and T1b are separately proved theorems, not one theorem swept twice). 5.4's
domain predicate is real (the commutation arguments are equivariance statements that do genuinely
reference translation by the quantum, which is a domain-shaped structure even though the specific
proofs don't split on sign the way T1/T1b do; I did not build a probe for this one because the
equivariance argument itself, stated in `125` T5 and reproduced at `131`, already names the
translations it respects without reference to sign, so the predicate there is arguably the same class
of over-carry, but I ran out of budget to check it with the same rigor as 5.6 and flag it as a further
question rather than a second dissent). 5.5's vacuity predicate correctly reads `domain any`, which is
the shape I would expect and matches F4's own argument (grid closure under an operation set, which is
sign-blind by the same reasoning as the coupling argument, and correctly stated that way here, which is
itself evidence that the pattern-inheritance in 5.6 is the exception rather than the rule for this
file).

## Coverage, bounded

Read in full: `132`, `131`. Read in part: `131_probes/v1` and `v2` at the sections `132` cites, opened
to confirm the figures section 6.2 and 6.3 quote. Not read: `125_probes` through `130_probes` sources,
per `132`'s own stated scope, which I did not need to exceed since my dissent rests on my own
committed probes (`127_probes/w1`, `130_probes/y1`) plus one new one (`135_probes/z1`). Built and
committed: one probe, `135_probes/z1_does_the_coupling_argument_need_domain_sign.py`, with predictions
stated in its header before it ran, three parts, output committed alongside it.

**What I did not do.** I did not check 5.4's equivariance predicate for the same over-carry class,
named above as a further question. I did not check whether the same domain-predicate issue reaches
5.8 (the entropy clause), which reads differently (it is bounded by the operating constraints rather
than by a domain sweep) and did not look like a candidate for the same error on inspection, but I did
not build a probe to confirm that impression and say so rather than claim it. I did not re-verify
`131`'s own findings beyond the two sections `132` cites from it (F131-3's bit-drop figures and
F131-1/F131-2's vacuity widening), since neither is mine to sign and my brief is to check my own work.
