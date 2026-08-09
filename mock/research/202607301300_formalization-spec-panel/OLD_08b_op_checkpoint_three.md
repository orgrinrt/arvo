# Op's third checkpoint, after eight

**Date:** 2026-07-30
**Position in the panel:** written after `08_fog_the_union_and_what_it_costs.md`, before the ninth
member. **Required reading for every subsequent member**, with the numbered expert files, `04b` and
`06b`.

The panel continues. Op will say when it is ready for synthesis. No member writes one unless
dispatched to.

## Two attack jobs, and the first may dissolve the second finding entirely

Panellist 08 measured two things that land on decisions taken earlier, and op's response to both is to
have the next lens attack rather than to accept.

**On the three-contract split.** 08 compiled a counterexample showing that separating `Policy` from
`Lowering` does not prevent a correctness law from being conditioned on a cost axis
(`08_probes/c_split_does_not_bind.rs`), and measured that the split costs the diagnostic surface 06
had improved. Panellist 02 had called the separation "a typing fact rather than a review note", and 04
and 05 both endorsed it on that basis.

Op was offered four responses and took the fourth, with a note that changes the shape of the work:

> Option 4 but also option 2 might just make the whole "find" moot

Option 4 was to put it to the next lens. Option 2 was to keep the split and add real enforcement. So
the job is not to arbitrate whether the split is worth keeping without enforcement. **The job is to
find out whether enforcement exists.** If a mechanism can actually prevent a law from reading a
`Lowering` member, 08's finding stops being a finding and the split delivers what it was adopted for.
If no such mechanism exists under the permitted feature set, that is a much stronger statement than
the counterexample alone, and it is what the panel needs before anyone reasons further about the
contract boundary.

Nobody has yet tried. Sealing, module privacy, a crate boundary, coherence structure, a witness on the
law itself, a marker that only a `Policy` member can produce: none of these has been probed, and the
crate split D72 introduces may or may not help.

**On thread C.** 08 found that 07's witness cannot express `Refuse`, since the map is declared total
and refusing is the absence of a returned value, and proposed making the map partial, which also
reproduces 01's five-row law table mechanically including both `Refuse` rows.

Op's call: **have the next lens attack it before it is carried.** The reasoning is on the record in
the panel's own history. Thread C has now had three successive proposed shapes, from 03, then 07, then
08, and each of the first two had a hole the next member found by compiling. There is no reason to
assume the third is different, and finding its hole is worth more than adopting it.

## What is not being asked

The next member is not asked to rule on whether the split is worth its cost, nor to choose between
fused and split. Those are op's, and they are downstream of whether enforcement is possible. Report
what is mechanically true and let the call follow.

## Standing, unchanged

Only op's calls are final, and even those go stale. Four of the panel's own findings have now been
overturned by a later member compiling something rather than reading. Assume that includes anything in
this file.
