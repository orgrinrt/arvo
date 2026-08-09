# Op's ninth checkpoint: three ratified, the encoding held

**Date:** 2026-08-03
**Position:** after `39_knuth_does_it_still_represent_them.md`, and immediately before consolidation
three. **Required reading** with the consolidation that follows it.

Op's first move at this checkpoint was to refuse the ratification and ask a question instead, and the
question is worth recording because it is the one the four files had not asked themselves.

## The question that gated the ratification

Op, on being told `Widening` and `Growth` would leave the table:

> Now, if we lose widening and growth, do we still retain the behavior therein, so the strategies make
> sense? Or are there no real use for it somehow anymore (though I fail to see how fixed point accuracy
> vs. efficiency survives dropping them altogether).

The answer, from `35:147-167`, is that the trade never lived in those two axes. Hot, Warm and Cold
narrow immediately (`mul_full`, then `quantize` under that preset's resolution), and Precise does not
call `quantize` in a fold interior at all because its accumulator numeral *is* the product or sum
numeral, sized by the accumulator-sufficiency check built earlier for an unrelated reason
(`26:278-286`). So accuracy against efficiency is carried by `Quantisation` plus accumulator
sufficiency, both of which stay. `Widening` was a name for a combination of which primitive is called
and what that primitive returns, and the old shape needed a derivation rule that held for three
presets with a separate undocumented case for the fourth. The new shape needs no rule because there
is nothing left to derive.

## The intent check, which op asked for and which found something

Op's second concern, stated while unable to read any file directly:

> I want to ensure this round of experts didn't lose sight of the intent in their effort to compile and
> get a valid shape. It seems likely not; but as I am now relying on your short summaries, I can not
> know for sure.

A grep established that files 35 and 37, the two that changed the ratified table, mention MATLAB, IEEE
754 and SystemC exactly zero times between them. File 39 was dispatched to close that gap rather than
to reassure.

Both passed, and both came out stronger than "still representable". The `Widening` removal is
*required* by the standards rather than tolerated by them, because MATLAB's `SpecifyPrecision`
quantises into a third consumer-chosen numeral that the removed instance set could not name. Every
nontrivial level of file 37's view lattice carries a shipping standard's observable. And all three
standards state growth per operation, which is external corroboration for the one tick left open.

The defect file 39 found is in file 36, which *had* checked: `Bias = Int` fails MATLAB, witnessed by
slope one and bias one half, compiled exhaustively.

The lesson is worth keeping separate from the finding. The check was worth running even though the
files it examined passed, and the value came from running it rather than from what it found.

## The calls

**Ratified, three:**

1. `Widening` leaves `Lowering`.
2. `Growth` leaves the law key.
3. The finest-view mechanism replaces the three-relation fork.

**Held, one.** The value-unique encoding replacing the width chain stays a recommendation until the
rational-bias repair is compiled. It is the piece carrying the defect, and a known-wrong member inside
a settled contract is the worst thing to freeze.

**Open, unchanged.** `Growth` leaving `Policy` (tick 3) stays open as op called it at the eighth
checkpoint, now with the three standards' corroboration recorded against it and still no compiled
check.

**Order of work:** consolidate first, repair after. Consolidation three records the bias defect as a
known open item rather than waiting on it.

## Standing

The convergence directive from `30b` and the novelty posture from `34b` both hold unchanged. The
intent outranks every instruction, is vague on purpose, and is inferred rather than read literally.
Only op's calls are final, and even those go stale.
