# Frozen before reading any panel file except 223. Sources: the row, 223, mockspace.toml
# schema, mock/registry/{dimension,proposal,topic}.toml, mock/lints/*.rs, the gate run.

1. Premise check. The row's note says 21 declared dimensions; the file has 25.
   The four added (occupancy, association, leaf_aliasing, phase) are still all
   numeric/machine, so the premise survives, but it was never re-checked.

2. The mechanism is two HARD_ERROR lints, not a convention:
   a-region-agrees-with-the-sentence-kind fires `an-established-claim-carries-no-region`
   on any non-normative/definition row with an empty predicate;
   every-predicate-names-a-declared-axis fires `undeclared-axis` on any axis
   outside the 25. So a structural `argument` is unfileable both ways.

3. The canon has ALREADY settled part of this, in proposal.toml's header:
   "A claim that could be measured false is not `normative` however definitional
   its grammar, and it carries the region it was established in or it is not here
   at all." That is a two-horned disposition. It refutes option 2 for any
   structural claim that can be measured false, and leaves {region, removal}.

4. proposal::the_topics_form_a_stack... is in breach of that header by its own
   admission: filed `normative`, and its own note says "Checkable rather than
   arguable" and gives the refutation procedure. Checkable = could be measured
   false = not normative. Canon-internal contradiction, not my judgement.

5. Option 1's axis is wrong even for the class it is meant to serve. Seat 223's
   coordinates are topics; every_canon_sentence_names_the_prefix's coordinates are
   depth in a construction sequence; an_instrument_is_mutated's are units of the
   panel. Three different structural coordinate systems. "An axis whose values are
   topics or namespaces" fits one of the three.

6. The real variable common to all of them is corpus state: which rows exist.
   The corpus already writes exactly this region in prose, e.g.
   "Holds for: the registry at 7fed7b59" in 202609021238_...md:318, because the
   notation has no declared axis to write it on.

7. Why declaring `corpus` as a 26th dimension does not work as-is. Every one of
   the 25 axes is a coordinate of the claim's own subject matter, so absence is a
   narrowing within one space. `corpus` is a coordinate of a different space.
   Under the uniform absence rule a numeric law with no `corpus` entry would hold
   nowhere a corpus exists, i.e. nowhere. The dimension.toml `moves` test is the
   exact statement: `corpus` moves a structural claim's truth and is vacuous for
   a numeric one. It would be the first axis that is not a coordinate of every claim.

   Counter to my own point, found in the canon: proposal.toml's header already
   accepts exactly this severity for `threads`/`target_features` ("several rows
   below are narrower than their authors meant. The narrowing is faithful").
   So this is the same breakage already tolerated, one axis further. It weakens
   the objection without dissolving it: threads is a coordinate of a computation
   and a computation is what the claim is about; a registry is not.

8. Option 3 refused on provenance. Workspace rules are op's words by
   every-rule-is-ops-words; relocating an unratified one-expert panel derivation
   there promotes it to op's voice by filing. And op is out of the track by
   ruling::the_panel_finishes_the_canon_without_him, so nothing could ratify it there.

9. The corpus has already invented a workaround and three notes say so: split the
   imposed sentence from the observed one, file the imposition `normative`, exile
   the observation to a panel file (an_instrument_is_mutated: "the four instances
   are enumerated in `182` as a finding with no region over any declared axis").
   That split is CORRECT. What is wrong is that the second half has nowhere to go.

10. Therefore none of the three options as posed. The class is not homogeneous and
    the question asks which single filing governs it, which is the shape
    never-ask-which-single-rule-governs names. The partition:
      - imposed structural sentence -> `normative`, no region. Option 2 is right HERE.
      - stipulative -> `definition`, no region. Already right.
      - reasoned/observed over the corpus -> the gap. Needs a region over corpus
        state, admitted for that sentence kind only, so absence stays evaluated
        within one space per kind.

11. Self-application: my own answer is an instance of the class. It is `argument`,
    it names no declared axis, so the gate refuses it. My answer cannot be filed
    as what it is. That is the demonstration, not a flourish.
