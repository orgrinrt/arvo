# 134. Cosign, with one addendum on what the dither measurement does not establish

Resumed as `126`/`129`'s author, reviewing `132` as one of the three it was written for. Read since the
last commit: `132` in full, `131` in full with `131_probes/v1`, `v2`, `v3` and their outputs, `128` in
full, `130` in full. Nothing else new.

**Canon gate: passed, inherited.** `132` and `131` both checked against `INTENTS.md`; nothing in this
file proposes a design decision or disputes their gate readings. I re-checked the I14 reading myself
(no `std::time`, `std::fs`, `std::net`, `std::thread`, which are the only entropy sources a library can
reach) against `arvo/.claude/CLAUDE.md`'s own constraint list, which states the same prohibition
independently as a live rule rather than an intent citation, so the two sources agree.

**Test gate: inherited with attribution, unchanged.** `125` section 10's eleventh run, 123 across 13 by
`--manifest-path`, is what `131` and `132` both cite. I ran nothing crate-side. I ran one arithmetic
check outside the harness (below, the golden-ratio constant), which is a one-line sanity computation on
a literal already in `131_probes/v3`, not a new measurement feeding a design claim, so it is reported in
prose rather than built as a probe.

## Verdict: cosign, with one addendum

I checked every clause that cites my own work, claim by claim, and found no misattribution, no dropped
dimension, and no widened predicate I did not license. **I cosign `132` in full.** One addendum follows,
on the entropy-free member, where the candidate's claim is accurate as measured and I want to state,
from the dithering background the dispatch asked me to bring, what the measurement does not by itself
establish. It is not a dissent: nothing in `132` or `131` is wrong. It is a caveat worth carrying into
whoever writes design text from this candidate.

## Checking my own claims, one by one

**Finding 1 (vacuity).** Cited at `132` section 1.1 (B1, both `128` and `129`) and 1.4/5.5's widening.
Matches what I derived: rounding never fires for `+`/`-` at any `F` and for `x` at `F = 0`, independently
of `125` F4. Correctly credited as a genuine blind convergence rather than a citation.

**Finding 3/6 (monotonicity, saturate composition).** Cited at 1.1 as `128`-only convergences (B2, B5)
between `125` and `128`'s own phase one, not mine; `132` does not attribute these to me and I did not
claim them as mine. Correct as written.

**Finding 4 (the deterministic/stochastic fork).** Retired at 1.7: "`126` Finding 4's framing of that
boundary as 'the genuine either/or fork'. Same fact, and its own open item 3 already said the exclusion
was exhibited rather than proved." That is exactly what my own phase one said (open item 3: "I exhibited
the trade; I did not prove the exclusion"), and the retirement is fair: the framing fell, the hedge I
attached to it was accurate about what I had and had not shown.

**F129-1 (decorrelation) and the corrected reach of `127`'s sentence.** Both cited accurately. Section
1.7: "`127`'s sentence that the correlated construction 'dominates the independent one on every property
either file measured'. True as written; `129` corrected its reach, because the property an arm actually
wants is decorrelation and the shared threshold delivers zero of it." That is precisely what my reply
argued and no more. Section 5.7's predicate names "input shape = one constant value at forty positions
in one cell," which is my probe's exact construction (`129_probes/x1`, Part B).

**The double-rounding hazard.** Section 6.1 credits me with naming the hazard and states plainly that I
slightly misplaced which equality it threatens: "`122` 4.6's equality is not threatened... What is at
risk is staged-versus-direct narrowing, and no clause in either topic states it... that is the hazard
`126` correctly identified and slightly misplaced." I went back to my own text to check this is fair
rather than take it on the candidate's word: my phase two said "a carrier that changes fraction width
partway through a derivation is a second place the grid part's 'must always apply' rule needs a
qualifier," which is a claim about 4.6 specifically. `131` F131-4 shows 4.6's own equality survives
double rounding intact (both its arms round at every node, so double rounding cancels out of the
comparison), and the actual casualty is the composition clause (staged narrowing equals direct
narrowing, which nothing in the topic before `131` had stated as its own clause). So the correction is
real and the candidate states it exactly, crediting the discovery to me and the precise location to
`131`, which is the right split.

**The bias measurement.** Section 1.5 lists it correctly: "`126` Finding 5's tie-bias magnitude, which
its own author calls adversarial." My own phase one said the same in those words. No overreach.

**The vocabulary contest (C1).** Section 1.6 states my position accurately: I kept the older spelling in
phase one and my reply does not address it, both true, and the candidate does not attribute a stronger
position to me than I took.

## The crux check: B6

The dispatch is right that this is the one entry only I can confirm, so I checked it against my own file
rather than against the candidate's account of my file.

`132`'s table: B6 is "the answer's shape: neither a copy nor a modifier of the overflow axis, but an
independent axis with its own selected property," credited to `125` section 8 and to my own
reconciliation section, named as a blind convergence by `129` alone.

My own text, `129`'s coverage section, unedited since it was written and quoted here rather than
paraphrased: "the general shape of the answer to the brief's question (neither a copy nor a modifier of
the overflow axis, an independent axis with its own selected property), which `125` section 8 and my own
reconciliation section both state, arrived at from different argument structures (a
divisibility-obstruction theorem in `125`'s case, a domain-and-operation-set sweep plus reading `122`'s
own grid-part rule in mine). I did not know either of these before this reading pass; both are genuine
independent convergences and I record them as such rather than as citations."

That is a word-for-word match to what the candidate reports. **I confirm B6 is real, is mine to confirm,
and is stated at the correct strength.** It is the answer to the question the topic was convened under
(`126`'s brief, and `125`'s brief independently, both ask whether rounding is a copy, a modifier, or
neither), and neither derivation had seen the other's when it landed on the same three-way answer. The
six-not-five-not-seven count is also right: `128`'s five (B1 through B5, all named in its own section 7)
and my two (B1 and B6, named in my own coverage section) overlap on B1 alone, so the union is six.

## The rungs, and the half of blindness the commit ordering cannot show

`132` section 1.1 is precise about what the commit ordering establishes and what it does not: "That
establishes the within-file half: `125`'s predictions were committed before its own probes ran... It
does not establish the between-file half on its own... So blindness rests on the ordering and on each
file's own coverage statement."

My own coverage statement carries the other half, and I want to point at the exact sentence rather than
let the candidate's summary stand for it. Phase one of `126` opens: "No other panel file, no register, no
probes belonging to anyone else, and no commit log, has been read before this paragraph. Everything below
is my own derivation, with my own probes, committed as they ran." That is an explicit claim of
between-file blindness, made before phase one's content, and it is what the candidate is entitled to
lean on for B1 and B6 alike. **Confirmed: nothing of mine is recorded as blind that I in fact got by
reading**, and the one place a blind claim of mine appears (B6), my own file's opening paragraph is the
second half of the evidence the commit ordering alone could not supply.

## The addendum: what the entropy-free member's measurement does not, by itself, establish

`131` F131-7 and `132` section 5.8/R6 are accurate as far as they measure. I checked the compiled
construction at `131_probes/v3_which_stochastic_members_survive_the_operating_constraints.rs` against
what I meant by "position-keyed golden-ratio ordered dither" in `129`, and it is the same technique: a
threshold computed as `position * golden_ratio mod 2^32` in fixed-point integer arithmetic, no float, no
entropy, `const fn` throughout, checked by `const _: () = assert!(...)` items so a wrong result is a
build failure. This is real dithering, not a strawman built to pass a check. The golden ratio (or more
generally an irrational rotation) is the standard one-dimensional low-discrepancy construction, and using
it this way is a real technique with real prior use (it is close kin to what practitioners call an "R2
sequence" or "golden ratio sequence" dither, used in real-time rendering for exactly this reason: no
runtime state, no lookup table, cheap, deterministic, reproducible across recomputation).

One trivial correction with no consequence for anything above it: `v3`'s constant `2_654_435_769` is
`floor(2^32 / phi)` exactly (checked directly: `int(2**32 / ((1+5**0.5)/2)) == 2654435769`). My own
`129_probes/x1` used `2654435761`, eight low, an arithmetic slip on my part rather than a different
technique. It did not change my probe's qualitative result (decorrelation, monotonicity cost), since the
result does not depend on the constant being the exact optimal rotation, only on it being close to
irrational-rotation shaped. `131`'s constant is the correct one and I note the difference for the record.

**What the measurement establishes, and what it does not.** `v3`'s `DISTINCT_AT_40 == 2` (and at 256)
proves the construction is not degenerate: a repeated tie does not receive the identical decision at
every position, which is the property that separates it from the shared-threshold failure `129` and `130`
both measured at zero. That is a real and necessary property. **It is not the property dithering
literature actually optimises for, and the gap between the two is not visible in a count of two versus
one.**

Dithering, in the imaging and audio practice both `129` and `130` cite it from, is chosen for **spectral
shape**, not merely for non-repetition. A dither whose error is decorrelated but structured (a low
frequency component, a periodic component, a diagonal pattern) can be visually or audibly worse than a
plain deterministic quantiser, because the eye and ear are specifically insensitive to *high-frequency,
spectrally flat* noise and specifically sensitive to *any* low-frequency or periodic structure, however
small its amplitude. This is why "blue noise" is the term of art rather than "any noise": it names a
noise spectrum with energy pushed away from low frequencies, and it is a stronger, different property
from "the output is not constant across a run of forty positions." A one-dimensional low-discrepancy
sequence like the golden-ratio construction here is good at avoiding short-period repetition (which is
what `DISTINCT_AT_40` and `DISTINCT_AT_256` test), and it is a real, credible ordered-dithering technique
along a genuinely one-dimensional axis: a sample index in an audio stream, a monotonically increasing
counter, any position where "nearby in position" means "nearby in the actual physical or temporal axis
the eye or ear integrates over."

**Where it needs a caveat is when "position" is a flattened index into something genuinely
two-dimensional**, which is the common case for the imaging half of the literature this topic keeps
citing. If position `i` addresses a pixel via row-major flattening of a 2D image, a one-dimensional
low-discrepancy sequence applied to `i` directly is not guaranteed to be low-discrepancy, let alone
spectrally flat, in the two-dimensional sense that matters for what the eye actually sees: it can produce
visible diagonal or periodic structure when the image itself has any regularity that beats against the
sequence's own period or slope, which is exactly the aliasing failure mode ordered dithering is generally
known to be more prone to than genuine blue-noise dithering (a precomputed or algorithmically-generated
2D texture specifically optimised to have no low-frequency structure in either axis, the kind actually
used in modern real-time rendering for temporal and spatial dithering together).

**None of this contradicts anything measured.** `131`'s three assertions (decorrelates, costs
monotonicity, compiles gate-free) are all true and all I would have written. What I am flagging is that
the fourth, unstated thing a reader might infer, that this construction is therefore *as good as* a
genuine dithering scheme for the visual and audible use case the topic keeps citing as the motivation,
does not follow from what was measured, and the gap is invisible in a distinct-output count of two.
Whether "position" in a real arvo consumer means a 1D sequential axis (where this construction is
well-matched and the caveat does not bite) or a flattened 2D one (where it might) is a fact about the
consumer, not about arvo, and is exactly the kind of question `130`'s own open item ("whether a
construction exists that is keyed on both axes at once") and `132`'s own "where it is weaker than I would
like" (the order-preserving licence family being one member deep in a literature nobody has enumerated)
both already gesture at without landing on this specific version of it.

I am not proposing a fix. A genuinely 2D-aware low-discrepancy or blue-noise construction is a real,
buildable thing (a precomputed table, keyed on a 2D coordinate rather than a flattened index, is the
standard answer in the rendering literature), and building and measuring it is further work nobody in
this topic has done. I flag it as the open item it is rather than guess at its shape.

## Coverage, bounded

Read in full: `132`, `131` with `131_probes/v1`, `v2`, `v3` and their outputs, `128`, `130`. Read in
part: my own `126` and `129`, reopened at the specific sentences quoted above rather than reread whole.
`arvo/.claude/CLAUDE.md`'s constraint list, checked against `132`'s I14 reading. Not read: `125`,
`127`, already covered in `129`; `125_probes` through `130_probes` sources beyond `131`'s own citations
of them; `OPTIONS.md`, `AGREEMENTS.md`, `DROPLIST.md`, `INTENTS.md`, `RULES.md`, all excluded per the
dispatch's "nothing else new."

**Checked directly rather than taken on the candidate's word.** B6's wording, against my own file. The
between-file blindness claim, against my own file's opening paragraph. The double-rounding attribution,
against my own phase two's exact sentence. The golden-ratio constant, by direct computation. The I14
reading, against `arvo/.claude/CLAUDE.md`'s own constraint list.

**Not checked.** The anchor accounting in section 9 beyond the grep already run in this dispatch (no
citation of mine appeared in the reported "not carried" set, which lists only entries from the preceding
topic). `131`'s reproductions of `125`'s T3/T5/F6 and the Fréchet uniqueness proof, none of which I
derived and none of which this cosign depends on. Any bench-harness measurement, since none exists in
this topic and none is claimed.
