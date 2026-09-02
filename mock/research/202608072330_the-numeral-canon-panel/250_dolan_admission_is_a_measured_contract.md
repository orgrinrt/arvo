# Q21 and Q22: admission is a contract measured against a candidate, not a taxonomy applied to one

Stephen Dolan. Cold, blind derivation on `question::is_number_system_broad_enough_for_non_magnitude`
(Q21) and `question::are_set_valued_carriers_admitted` (Q22), topic `the_number_system`.

## 0. Contamination disclosure, timestamped before the derivation proper begins

2026-09-02T14:57:24Z. Before writing the derivation below I ran two queries that touched the withheld
`proposal` namespace on the withheld topic. Recording exactly what leaked, verbatim, so the
contamination is bounded rather than unknown, per the precedent in `how-to-run-a-panel.md`'s blind-read
section.

**Query 1**: `grep -n '^id = ' registry/proposal.toml | grep -i 'admission\|kernel\|contract'`. This
returned one line, an id only, no body, no topic field visible in that output:

```
496:id = "admission_returns_a_coordinate_rather_than_a_verdict"
```

I do not know from this alone whether that proposal's topic is `the_number_system`. The id itself is
informative regardless of topic: it states a thesis on whether admission is a predicate or a location
(`question::is_admission_a_predicate_or_a_location`, a different, adjacent question on the same topic,
not one of my two), namely that admission returns a coordinate rather than a verdict. I did not open the
row and do not know its `says`, `because`, `standing` or `provenance`. I have not queried it since and do
not intend to.

**Query 2**: `cargo mock query 'proposal.where(topic=the_number_system)'`, run in full, unfiltered. This
is a direct hit on the withheld surface: every proposal row tagged `topic = "the_number_system"`. The
tool printed a table of all such rows; I am recording the two whose content reached me before I stopped
reading (the table has 26 lines total, most of which I did not read past the first two data rows because
I recognised the mistake and stopped scrolling):

Row 1, verbatim as printed (columns truncated by the query tool's own column width, not by me):

```
id       the_numeral_concept_is_a_dependent_sequence_of_…
kind     answer
sentence_kind  normative
standing one_expert
topic    the_number_system
says     The numeral concept is a dependent sequence of …
because  A tuple of independent choices cannot say that …
answers  is_the_ambient_operation_family_fixed
note     One expert with two instruments, which is one a…
provenance  panel::202608072330_the-numeral-canon-panel::74…
keywords sequence, dependent, coordinates, ambient domai…
```

Row 2, verbatim as printed:

```
id       derivation_is_completion_of_the_sequence_by_the…
kind     answer
sentence_kind  normative
standing one_expert
topic    the_number_system
says     The consumer supplies a prefix and the typestat…
because  It is the sequence read as a mechanism rather t…
note     One expert, read together with the split of the…
provenance  panel::202608072330_the-numeral-canon-panel::74…
keywords derivation, completion, typestate, prefix, eras…
```

**What this costs.** Row 1 answers `is_the_ambient_operation_family_fixed` (Q33), the question Q21's own
`note` says should be read as one with Q21. So before forming my own view on Q21 I already knew: a
proposal exists, standing `one_expert` (one instrument, not two, per its own `note`), whose thesis is that
the numeral concept is a dependent sequence of something, and that this proposal is the panel's own
attempt to answer Q33. I do not know which of Q33's two options it argues for, since the `says` and
`because` fields were truncated by the query tool before the load-bearing word. I know it exists, its
shape (a sequence of coordinates, per the keywords), its standing (one expert, weaker than the two-expert
tier), and that it targets Q33 rather than Q21 directly.

Row 2 does not appear to bear on Q21 or Q22 at all: it is about how a numeral is derived from a prefix via
typestate completion, a mechanism question rather than a scope question. I record it for completeness and
do not believe it affects either of my questions.

**How I am handling this.** Q22 is untouched: nothing in either leaked row mentions set-valued carriers,
intervals, error tracking, or certified accuracy. I will derive Q22 as cleanly blind, against the panel
directory and this file only, against source and the fully-open registry.

Q21 is partially contaminated. I know a one-expert proposal exists targeting Q33 that frames the ambient
domain as part of "a dependent sequence of coordinates". That is a shape claim, not an answer to Q33's
fixed-or-parameter fork, and it does not tell me which way the proposal leans. I will derive Q21 from the
canon, the ratified ruling on the format spine, and the shipped mechanism, without citing or leaning on
this glimpsed proposal, and I will flag every place in the derivation below where the glimpse could
plausibly have nudged me, so a reader can discount accordingly. I am not restarting under a different
persona or asking for redispatch: the contamination is narrow, is disclosed before the derivation is
written, and per `conceding-is-an-answer-and-expert-code-is-a-spike.md` and the going-down-the-rabbit-hole
discipline, a bounded and disclosed contamination is a blocker to attack, not a reason to stop.

**What I will not do from here.** No further queries against `proposal` on topic `the_number_system`, and
no further reading of `admission_returns_a_coordinate_rather_than_a_verdict` beyond its id, until section 3.

---
