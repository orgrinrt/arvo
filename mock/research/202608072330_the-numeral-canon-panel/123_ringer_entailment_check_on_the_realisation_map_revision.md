# 123. Entailment check: the realisation-map candidate as revised against two partial signatures

**Role.** Independent check on `122_leroy_the_candidate_revised_against_two_partial_signatures.md`, working
from `114` through `121` forward rather than from `122` backward, per the brief and per `RULES.md`'s
instruction that the author of a compression cannot be the one who checks it. I took no part in this topic
and wrote none of `114` through `122`.

**Gates.** Canon gate: passes, situation two, identical to every member's own finding in this topic. No
canon exists; `mock/canon/` is absent and `mock/crates/` is empty by the declared mutation order; this
panel is writing the first canon. This dispatch is exactly the entailment-check mechanism `RULES.md:277`
and `119` section 6's own method call for, so it is licensed rather than merely permitted. Test gate: no
suite exists in the mockspace sense; the substitute is the probe discipline, and I checked whether the
revision's own self-corrections leave anything uncorrected downstream, which is this check's version of
"was the instrument shown able to fail."

## Coverage, stated first because it bounds everything below

**Read end to end:** `114`, `115`, `116`, `117`, `118`, `119`, `120`, `121`, `122`, all in full.
`INTENTS.md` and the relevant sections of `RULES.md` from the prior check in this same directory (`91`),
re-confirmed rather than re-read line by line since neither has changed in a way that bears on this topic.

**Not read:** any probe source under `114_probes/` through `122_probes/`. Every numeric claim I checked
below was checked by re-reading the citing file's own prose and, where cheap, by hand arithmetic or by
opening the actual shipped source the claim names. I did not rebuild or re-run a single `.py` or `.rs`
instrument; the question this check answers is whether `122` entails its sources, not whether the sources'
own measurements are correct.

**What I built:** the anchor diff below, run by shell command and reproducible. Two independent runs of the
finding-id extraction, one matching `122`'s own reported count exactly. I did not build a compiled probe;
nothing in this check required one.

**Verified at source, not merely cited.** `warm-clamp-shared/src/lib.rs:1105` (the
`clamping_is_a_retraction_on_non_negative_addition_at_every_swept_width` test, `121`'s central refutation
of `119` 4.4). `warm-clamp-shared/src/lib.rs:158-160` (`accumulator_bits_needed`) and `:289,:291`
(`fold_chunked`'s guard). All resolve exactly as cited, including the specific assertion text and the
exact line numbers `115` gives (`114`'s own `:288` for the function signature is one line off the actual
`:289`, immaterial, and `115` independently re-cited the same site at the correct line).

## The anchor diff

Same two citation shapes as before, `file:line[-line]` and `NN_probes/name`, plus the panel's `F<NNN>-<N>`
finding-id shape, extracted from `114`, `115`, `116`, `117`, `118`, `120`, `121` (the seven files `122`
names as what it revises across, `119` held separately as the document under revision):

```
union of the seven:      163 anchors
carried in 122's body
  (section 8 excluded):   61 anchors
dropped:                 116 anchors
```

That looks alarming next to the 79.3%-carried figure from the prior consolidation in this same panel, and
it is not the same kind of number. `122` is declared, in its own opening section, to be a **revision**
rather than a fresh compression: "`119` stays as landed and is not edited," and everything not named in
`122`'s delta sections "stands, unchanged and not restated except by reference." A revision that does not
restate what it did not touch is not dropping those anchors, it is pointing at the document that still
carries them, and that document is committed and citable.

**So the number that actually answers the brief's question is `122` against `119`, on finding ids, which
is the comparison `122` itself reports and which I re-ran independently:**

```
119's finding ids (body, section 6 excluded):  63   (122 reports 65; extraction-pattern noise, see below)
122's finding ids (body, section 8 excluded):  45   (122 reports 45; exact match)
new in 122, not in 119:                        12   (F121-1, F121-2, F121-4, F121-5, F122-1 through F122-8)
in 119, not in 122:                            30
```

My 45 matches `122`'s self-reported 45 exactly, on an independent extraction. My 63 against `122`'s
reported 65 is within normal extraction-pattern variance (the same variance `122` itself names against
`119`'s figure of 68 there); I did not chase the two-id gap because it does not change which ids are
missing, only the total.

**I read all 30 dropped-from-`119` ids at their point of use in `119`.** Every one sits inside `119`
section 1 (the agreement ledger, A1 through A18), and every ledger entry not named in `122` section 3
("Ledger deltas") is explicitly declared to stand by reference. `122`'s amended clauses in section 4 cite
the *surviving* findings that carry the same load (`118` F118-11 for the locality rule, `114` F114-6 for
the root arm, `118` F118-1 for the certificate), and where an amended clause's underlying mechanism traces
back further, the chain runs through those surviving citations into `119`'s unedited ledger rather than
around it. I did not find a single instance of an amended `122` clause resting on a dropped id with no
path back to it. The traceability requirement the brief states, "a drop is a defect when the clause resting
on it can no longer be traced," is met: every clause in `122` either restates its evidence directly or
points at a `119` clause marked `[STANDS]`, and `119` itself is committed and unedited.

**One drop worth naming even though it is not a defect.** `F112-2` and `F112-3`, which `119`'s gate section
uses to establish that the overflow behaviour, operation and fraction width all sit in the declared
semantics "by measurement rather than by assertion," are not re-cited anywhere in `122`. `122`'s own gate
section is shorter and does not re-ground that meta-point. Nobody in this exchange contests it, so it costs
nothing, but a canon writer reading only `122` would take the const-availability of these axes on trust
rather than on the citation `119` supplied for it.

## The self-corrections, checked against what depends on them

### F118-5, withdrawn

`122` withdraws its own predecessor's F118-5 (the claim that multiplication alone admits a
homomorphism-and-monotone counterexample, "addition is load-bearing") after `121` showed the witness came
from a window confounded on domain sign rather than isolated on operation set. I traced every citation of
`F118-5` across `122` (`122:158,193,311,312,607,654,659`) and every one is inside the withdrawal narrative
itself (section 1.6, the ledger delta at A3/A4, section 7's withdrawal list). Nothing outside that
narrative cites F118-5 as support for a live conclusion. `F118-6`, which `122` correctly separates from
F118-5 as "the same fact at a third size" and marks "corrected rather than withdrawn," is not affected by
the withdrawal because its content (a window narrower than the value set is also not closed under
negation) never attributed anything to a specific operation the way F118-5 did; I checked this distinction
against `121` F118-6's original text and it holds. Clean.

### `118_probes/q3`'s ambient-range bug, checked as far as I could without the source

`121` diagnoses the bug precisely: `118_probes/q3`'s ambient range starts at `klo - span`, negative even
for an unsigned primitive, so the probe's saturating-homomorphism column measured a straddling domain
while its predicate claimed a general result. I checked every citation of `118_probes/q3` in `122` and in
`119`'s ledger for whether it inherits the corrupted reading.

**Confirmed safe.** `119` A7 (the ledger entry citing `118` F118-7 as a reproduction of `116` F116-7) is,
as `119` states it, entirely about *wrapping*'s homomorphism behaviour at nonzero fraction width, which
`122` F122-4 independently confirms is domain-independent. The bug affected only the reading of
saturation's per-operation homomorphism status, and `119`'s own 4.4 clause (not A7) is where that reading
was used, and it is exactly the clause `122` replaces.

**Not fully ruled out, and I am saying so rather than asserting either way.** `118` F118-8 (arm W0 splitting
along the operation boundary) also cites `q3_output.txt` and its own predicate lists `overflow policy in
{wrap, sat}`, so it makes a claim at saturation from the same probe run that produced the corrupted
homomorphism table. F118-8's stated declarations are `one-sided [0, b]`, which is not the straddling range
the bug describes, and the two measurements (arm-W0 differencing versus homomorphism testing) read as
separate functions inside one probe file rather than one shared computation. That separation is plausible
from the prose and I could not confirm it without opening `118_probes/q3`'s source, which is outside what I
read for this check. `122` neither flags nor clears F118-8 on this point. I would want whoever next touches
this topic to open the source and confirm F118-8's declarations were genuinely isolated from the ambient-
range function before citing it further; until then it is unconfirmed rather than corrupted.

## What is marked as not done

`122` explicitly says, in body text a later reader will actually land on rather than only in a probe
report, that 4.8's and 4.9's predicates were swept for the domain dimension and found wanting, and that the
gap is deliberate rather than fixed: section 5 ("That is a gap and it is deliberate: the honest state is
that their predicates do not name the domain and their measurements were taken on one, and a later pass
should sweep them rather than have this one assert them") and section 7 repeats it in the open-items list.
I checked this against `119`'s original text for 4.8 and 4.9 and confirmed neither ever named a domain
dimension either, so `122` is not introducing a gap, it is naming one that predates it and choosing not to
paper over it with an unmeasured widening. That is the right call under I13 and it is stated where a reader
will see it, not buried in a probe transcript.

## Absence claims

The one the brief names, `115` F115-4's "a route `114` did not try," is corrected at `118` (which found the
`selection-assoc` variant already in `114_probes/p9:244-256`) and the correction survives cleanly into
`120`'s acceptance and `122`'s citation (`F118-16`, `122` section 1's item list). I scanned the rest of the
corpus for the same shape ("untried," "nothing has," "nobody had," nineteen further hits across `114`
through `122`) and did not find a second instance of a false one. Most are honest trackers of genuinely
unattempted work (`109` section 8's chain result, the order-preserving family's unenumerated membership,
the tree-versus-DAG question two files depend on) and each is checkable by grepping the panel for whether
anyone since has addressed it; none has. I did not individually re-verify all nineteen against source, and
say so as a coverage bound rather than a clean bill.

## Rungs

I checked the rung most likely to be wrong beyond the one `121` already caught. `121`'s own correction to
A13 (crediting `116` with an "independent arrival" at arm S2 that its own coverage section contradicts,
since `116` read `114` sections 1 through 6.3, which contains arm S2, before writing) is precisely
targeted and I verified both halves of it directly: `114`'s arm S2 is indeed under its section 4 (confirmed
by re-reading `114`'s own text), and `116`'s coverage section does say "Read in full: `113`, `114` sections
1 to 6.3 and its findings list..." exactly as `121` quotes it. `122` accepts the correction and restates
A13 correctly. I spot-checked two more entries the same way, against coverage statements rather than the
ledger's summary: A2's "saturating half independently and first, because `112` was written before `114`
existed" is true by the panel's own sequential file numbering, and A9's "each half independently prior"
matches `118`'s own account of who ran which half of the (a)-alone measurement before either read the
other. Neither shows the A13 pattern. I did not check every remaining entry (A1 through A18 minus the ones
already covered above) against its author's coverage section individually; that is a bound on this check
rather than a finding.

## Verified before reporting

Per the brief's own instruction and the precedent it names: before writing any of the above as a finding I
traced it to its stated source rather than reporting a first impression. The F118-8/q3 item above is
reported as unconfirmed rather than as a defect precisely because I could not close it without the probe
source, and I would rather hand back an open question than a severe finding I have not verified.

## Verdict

This is a materially cleaner document than the one this same check found two real defects in earlier in
this panel. The anchor trail is intact by the standard a canon-candidate revision actually owes (finding
ids traceable through `[STANDS]` references into an unedited prior document, not restated). The one
withdrawn finding has no orphaned dependents. The two dissents that drove the revision, `120`'s recursion-
depth mechanism and `121`'s domain-confounding, are each reproduced on an independent instrument before
being accepted, and the resulting new claims (the saturating deferral arm, the "two mechanisms not one"
correction to `119` 4.5) are checked against a shipped, passing test rather than only against a model
sweep, and that test resolves exactly as claimed at `warm-clamp-shared/src/lib.rs:1105`.

The one item I would not let stand uninvestigated is F118-8's isolation from `q3`'s ambient-range bug,
named above and left open rather than asserted either way. Everything else checked came back clean,
including the part of this job most likely to fail silently.
