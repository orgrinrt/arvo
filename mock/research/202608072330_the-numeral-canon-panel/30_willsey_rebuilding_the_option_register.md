# 30. Rebuilding the option register from the member files

**Date:** 2026-08-08. **Position:** after `29`. Dispatched to rebuild `OPTIONS.md` from the member
files `02` through `27` rather than from `MORNING.md`, per the standing finding that `MORNING.md` is a
contamination source (`19` and `23` checkpoints; `21`'s citation audit: 297 line-level anchors in the
sources against zero in the compression, 295 numeric tokens carried with none checkable). `OPTIONS.md`
is the deliverable; this file is the record of the rebuild and the findings along the way.

## Coverage, stated honestly

Read in full, and cited by opening the source line rather than trusting a summary of it: `RULES.md`,
`00_brief.md`, `01_op_answers.md`, `04_op_no_settlements_tonight.md`, `28_op_answers_two.md`,
`02_carried_what_replaces_the_two_refutations.md`, `03` through `27` in numeric order, plus their
respective `NN_probes/` output files where a specific claim was checked directly (a partial sample,
not exhaustive: I opened the ones cited above; I did not re-run any probe myself). `MORNING.md` was
opened and read for the sole purpose of locating which member files bore on which questions before
reading those files themselves; nothing in `OPTIONS.md` or here is sourced or cited from it, per the
brief's instruction.

**Not read at all:** `CANON_CANDIDATE.md`, `DROPLIST.md`, `SETTLED.md` beyond the passages member
files quote directly and that I then followed to the quote, `seed/`, `PERSONA_CALLS.md`, and the
closed predecessor panel's own tree (320 files, explicitly out of scope per every member file's own
reading discipline and per the brief). Where a member file's claim rests on one of those, I report
what the member file says and do not independently confirm it.

**Not verified:** the arithmetic inside any probe. I read probe *outputs* and the prose describing
them where a specific number is load-bearing for an `OPTIONS.md` entry; I did not recompile or re-run
anything. Where a member file itself flags a number as later corrected or reconciled (the 461-versus-
476 product-overshoot count, resolved by `15`; the `12` C4-versus-ceiling framing corrected between
`12`, `13` and `14`; `27`'s section 10 `u16` row superseded by its own section 15), I have carried the
corrected/reconciled version into `OPTIONS.md` and named both where the discrepancy is itself
informative.

**Time-boxed against the full text of `23`'s 1207-line inventory and `24`'s 993-line derivation**: both
were read in full because they are themselves inventories/consolidations of the other member files and
missing them would have meant re-deriving their content by hand from twenty other files. Everything
`23` and `24` claim about a third file's content, I have where practical traced back to that file's own
line rather than trusting `23` or `24`'s citation of it; where I did not, I have said so inline in
`OPTIONS.md` by attributing the claim to the file that reports it rather than asserting it as
independently checked.

## What I found

### The instrument itself: does a live-option register belong on this side of the panel's artifact
boundaries, and is its current structure right

**Existence: yes, and this is now well-evidenced rather than merely asserted.** Op's own words
(`00_brief.md:62-67`, quoted from his direction) name the mechanism directly: keep options written
down and open, reflect each new question over every one of them, say which fit and which do not, and
let the path emerge. `RULES.md:143-165` treats the file as required panel infrastructure ("every option
is carried in `OPTIONS.md`, written out in full"). Nothing in the material I read challenges that this
kind of artifact should exist; every member file from `03` onward treats it as the working surface it
is supposed to be, several explicitly compare their own findings against it, and `14` and `23` both
audit its accuracy as a going concern rather than questioning its presence. I find no basis to
challenge its existence and I looked for one, per the dispatch's explicit invitation to.

**Locus: right, with one caveat.** The register sits in the panel's own working directory, alongside
`SETTLED.md` and `DROPLIST.md`, which is the correct side of the design's artifact boundaries: it is
panel infrastructure (a live working document), not canon (which states intent) and not design (which
would spell an implementation). Nothing in it should ever, and as rebuilt does not, carry the concrete
spelling of an implementation; where a member file's finding is a compiled construction, I have
described what it establishes rather than reproducing its code.

**The caveat, worth naming plainly.** A register built by one dispatching agent reading twenty-six
other files is itself a compression, of exactly the shape `a-compression-is-checked-by-someone-else`
warns about, and the prior version of this exact file demonstrated the failure mode concretely (built
from `MORNING.md`, which every subsequent checkpoint found wrong in a new way each time it was
checked). **This rebuild is not exempt from that risk merely by having read the sources instead of a
summary of them.** I have tried to mitigate it the way the workspace rule prescribes: every load-bearing
claim in `OPTIONS.md` carries a `file:line`-or-section citation into a member file rather than into this
report or into the prior `OPTIONS.md`, so a reader (or the next member) can check the register against
its sources without going through me. But I am one reader on one pass, and per this workspace's own
"one instance of evidence is never enough" standard, this rebuild is one instance of the entailment
check `OPTIONS.md` needs, not three. It should be checked again, independently, by someone who does not
start from this file.

**On the register's structure.** The existing grouping by op's eight `28` questions plus a "questions
not yet asked" tail held up well as I read the material and I have kept it, with one addition (Q9, the
container-derivation/width-surface topic, entirely absent from the prior pass) and one restructuring
(a dedicated "the derivation's outputs" section, because that thread's material is substantial, spans
four member files, and did not fit cleanly under any single one of the eight numbered questions without
either duplicating it or losing its coherence as one thread). I considered instead folding it entirely
into the "questions not yet asked" tail as one more bullet, the way the prior pass had a thin pointer to
it, and rejected that: the thread has four independent contributions (`15`, `16`, `17`, plus `23`'s
audit of it), a real TWO-EXPERTS-rung finding, and a currently-open blocker (`Precise`'s semantics)
that a one-line pointer would have hidden. Keeping something structural is itself a finding worth
stating, per this panel's own discipline that everything being open is not an instruction to replace
everything: the eight-question grouping held, and I kept it.

### What moved between "live" and "closed", and why

**Newly added as live, previously absent from the register entirely:**

- Q8: readings D (tie-break) and E (step-set-seam), both explicitly surviving `03`'s own scrutiny
  (`03` sections 7.1, 7.2) and never carried into the prior pass despite `03` itself being one of the
  panel's very first files. The prior pass's Q8 section had four entries (one family / several
  families / not-load-bearing / a route that survives all three, unfound); `03` alone produced five
  further readings beyond op's original three (D, E, F, G, H), and the brief's own framing anticipated
  finding "the five options beyond the three it was given" as a known gap. Confirmed and now carried.
- Q8: the `08`-derived fork on whether the canonical exponent is a design member or its two named
  values, which decides the family question by fiat under one answer and dissolves `03`'s cross-kind
  antichain under the other. This is a materially different kind of fork from A through E (it is about
  what counts as "in the design" at all, not about how to close a join), and I have kept it as its own
  paragraph within Q8 rather than merging it into any single reading.
- An entire new topic, Q9, covering the const-to-type crossing at the width surface: seven distinct
  compiled arrangements (C0 through C4/A, B, D) across four member files (`10`, `11`, `12`, `13`, with
  `15` extending the thread), three routes explicitly closed with citations (a bare byte-count carrier,
  a macro-generated table, and the const-surface design itself as it stands, reclassified from
  "refused but expressible" to "structurally insufficient" by `11`), and one cross-cutting open
  question (does any arrangement need to represent negative integer width, resolved for one coordinate
  keying and unresolved for another).
- A dedicated section on the container derivation's two outputs, a TWO-EXPERTS-rung finding (on the
  identity of the outputs and what the second is keyed on; ONE-EXPERT on the exact count) that the
  prior register represented with one compressed paragraph.

**Confirmed and kept, strengthened with citations the prior pass lacked:** Q1, Q3, Q6, Q7 all held up
well against the sources; I found no material the prior pass had wrong in substance for these, only
under-cited. Q5 is now substantially stronger than the prior pass had it: `25`'s definition-of-strategy
work independently corroborates the "two axes" reading four separate ways (the preset table's own
2x2 decomposition, four industrial systems from outside arvo, and two committed bench families), which
the prior pass's Q5 entry did not have.

**Refined rather than replaced:** Q2 gained a fourth reading (`24`'s grid-and-reach definition) that
resolves what had read, in the prior pass and across `23`'s own inventory, as an unresolved collision
between two vocabularies of "numeral". I want to flag this one specifically as a place where I made a
judgement call rather than a mechanical transcription: `24` argues its grid-and-reach framing
*dissolves* the apparent Q2/Q9 (S9-versus-S17 in `23`'s numbering) collision rather than being a
competing option, and I have represented it that way (as context that reframes the existing three
readings, plus a live open question about whether the design admits numerals its width-pair coordinates
cannot name) rather than as a fourth co-equal choice among "which two numbers does a consumer write".
A different rebuilder might reasonably have kept it as a straight fourth bullet; I judged that would
have obscured what `24` itself argues is the more important point (permanence: floats exist in the
design today and a width-pair-only canon sentence is already false about them), so I gave it the
fuller, argued treatment instead.

Q4 gained the soundness-versus-bestness fork from `07` (a genuinely separate axis from the four
existing denotation readings, composing with all of them rather than replacing any) and a correction
to the "absorbing top" reading's stated condition: the prior pass's phrasing ("sound exactly while the
computation stays at the endpoint") states a necessary-and-sufficient condition that `18`'s own probe
table refutes in the necessary direction (an operation set that decreases, multiply-by-zero, stays
sound at 0 of 512). I corrected this in place rather than silently, since `21` and `19` both
independently caught the same "exactly" overclaim in `MORNING.md` and it would have been a poor result
for the rebuild to reintroduce the identical defect from a different source.

**Closed, moved conceptually to droplist territory (not physically, since `DROPLIST.md` maintenance
was out of scope for this dispatch):** Q8 reading G (ordering by something other than inclusion,
closed by antisymmetry/injectivity arguments in `03` section 7.5); three Q9 routes (bare byte-count
carrier, macro-generated table, and the const-surface design's structural insufficiency once the
ceiling argument is separated from the ergonomics-refusal argument that originally closed it). I have
flagged all four in `OPTIONS.md` as closed-with-citation rather than removing them silently, since a
member reading the register should be able to see what was tried and ruled out, not just what remains.

### A challenge to the register's content that I am not resolving

`OPTIONS.md`'s "the derivation's outputs" section and its Q9 both depend, at more than one point, on
what `Precise` means, and **no member file in the panel has built `Precise` as anything other than
`Warm` under a different name.** `15` says so of itself explicitly (`15` section 9); `16` names the
consequence precisely (whether the two-output shape is forced by arithmetic, or only by the type
system, is undecided until `Precise`'s semantics is). This is not a finding I am adding; it is a gap I
am refusing to paper over by picking a reading. I flagged it as a blocker in `OPTIONS.md` rather than
resolving it, because resolving it would be exactly the kind of premature settlement the standing mode
forbids, and because I have no basis (measurement, op quote, or compiled construction) to prefer either
reading.

### What I kept, and said held

Per the panel's own discipline that everything being open is a licence to question every element, not
an instruction to replace one: the eight-question top-level structure held and I kept it. The "how a
member uses it" and "what this file is, and what it is not" framing sections held essentially unchanged;
I re-derived them from the same source material (`RULES.md`, `00_brief.md`, `04`) rather than copying
the prior text verbatim, and they came out substantively the same, which I read as the prior pass
having gotten that framing right even though its content underneath was built the wrong way. Q1, Q3,
Q6 and Q7's core structure and options held without needing new readings added; I extended their
citations and, for Q6 and Q7, folded in material from `20`, `22`, `25`, `27` that the prior pass either
lacked (because those files postdate it) or under-cited.

## What I did not do, and would flag for a next pass

I did not attempt to re-derive or independently check any of the panel's numeric claims from first
principles; every number in `OPTIONS.md` is attributed to the member file and (where the member file
itself names one) the probe that produced it, and stops there. I did not open every probe directory in
full; I opened the specific probe output files a member file's own prose pointed me at when a claim
mattered enough to want the primary artifact rather than the prose description of it (this happened
perhaps a dozen times across the whole read, not for every citation).

I did not audit `SETTLED.md` or `DROPLIST.md` for consistency with the rebuilt register, which was out
of scope for this dispatch (the brief asked for `OPTIONS.md`); a next dispatch reconciling those three
documents against each other would find real work, since at minimum the Q9 route reclassifications and
the newly-added Q8 readings D/E have no corresponding entries in `DROPLIST.md` yet, and several
`SETTLED.md`-candidate sentences from `23`'s inventory (the strategy definition, the two-output
derivation, "cross once at literals in one direction") are referenced from `OPTIONS.md` but live only
in `23`'s prose, not in any settled-index document.

I did not second-read any member file's own self-declared ONE-EXPERT status; where a file says its
finding is one expert's and wants a second read, I have carried that flag into `OPTIONS.md` rather than
either promoting or downgrading it myself.

## Relevant files

`/Users/orgrinrt/Dev/clause-dev/arvo/mock/research/202608072330_the-numeral-canon-panel/OPTIONS.md`
(the rebuilt deliverable), and every member file `02` through `27` in the same directory, each cited
directly from within it.
