# Op's twentieth checkpoint: the bitpack price corrected, three gaps opened

**Date:** 2026-08-04
**Position:** after `82_pesce_the_stretch_assembled.md`. **Required reading** with the consolidation.

## The bitpack price is corrected, and the fourth rule produced the correction

**Ratified.** The storage-minimising preset costs about **1.50x** dense native on a column sum and
**1.29x** when the consumer does per-element work, not the 4.6x and 2.4x ratified at `74b`. Every
parameter of a bitpacked decode is a function of the logical width alone (period, group stride, window
offsets, per-lane shifts, mask, load width), so all of it belongs at compile time. The measured decoder
computed byte offset and shift from the running index at runtime, and LLVM unrolled by four against a
period of eight, so the shifts never folded. Moved into associated consts the loop strides by a literal,
extracts with a two-literal instruction, and vectorises unprompted.

This is the fourth design rule doing exactly what it was named for, one file after it was named.

**Ratified with it:** which decode is optimal is a joint property of the layout **and** the consumer's
operation, not of the layout alone. A layout does not fix a decode, and one decoder is not enough.

Three corrections to the ratified measurement ride along: the host's L1 is 128 KB rather than the 32 KB
the sweep assumed, so every benched size was L1-resident four times over and **the bench shape cannot
price a footprint at any size**, which is what the preset exists to buy; the instruction table described a
standalone probe rather than the benched program, with the ratio matching while both absolutes were wrong
three to five times in the same direction; and the shared decoder is silently wrong above 25 bits, now
pinned by an assertion. The likely route to the original cache error is named: the un-suffixed cache query
reports the efficiency core's 64 KB.

## Three items go to the next stretch rather than being called

**The lowering charter has two width levels and needs three.** The gap is where padding bits live: a
value's fields have an extent, a container has a width, and bitpacked storage has neither in the usual
sense. Three files each saw one side. Whether stored width means the fields' extent or the container's
decides whether a crossing statement has content on `Hot` and `Cold` at all. **First dispatch of the next
stretch**, derived with the two independent reads any design call requires.

**Quantise's failure is not a range event, and three of four presets have no refusal.** So an operation
that can fail for a non-range reason has no way to express that failure under the ratified tables, and one
file's closing argument assumes machinery those tables deny. A genuine gap between two separately-ratified
pieces rather than a defect in either. The honest resolutions differ enough (a fourth resolution instance,
a separate fallibility axis, or quantise sitting outside the preset surface) that it is derived rather
than picked.

**The exact fold width does not exist at binary256.** Two ceilings coincide at 128 by arithmetic accident,
rustc's recursion limit and the carrier's integer width, and the prior file's dismissal of that as
pathological precision was wrong on both halves and forbidden by name in the toolbox rule. A replacement
with structural recursion is built, verified on 1,225,601 cells offline and 60 in the type system, gate
free at binary256. It needs its second read.

## The owed list gets a closing artifact, and the last seven get audited

**Adopted, both halves.** Of ten items on the review's owed list, **zero name a closing artifact**, so
nine were correct only by luck and one had already been performed two stretches before it was marked
owed. From here an item is closed by a named file, probe or measurement, and the consolidation checks the
artifact exists, which is the same discipline the provenance grounds already apply to claims.

And because this stretch closed seven at once, which is when the risk is highest, **one pass confirms each
of those seven closures actually has the artifact it claims.**

## Standing

Only op's calls are final. The panel produces canon, not source; `mock/research/` and `mock/benches/` are
its ground and `mock/crates` is out of bounds until the canon is complete and earmarked as arvo's first
full canon. The verification mandate at `79b` binds the implementation phase and is not a panel topic.
