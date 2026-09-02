# Op's twenty-ninth checkpoint: full erasure is the gate, and the caveat is not addressed

**Date:** 2026-08-07
**Position:** after `135_lamport_the_law_list.md`, while `136` is in flight. **Required reading with the
standing base, and it sets an acceptance criterion the canon is measured against.**

Op is back and every call here is his. This checkpoint exists because he asked for it to be written
immediately, and because what it records is a gate rather than a preference.

## The gate, stated first because everything else in this file is subordinate to it

> There *is* a way to express usage through bits and bytes *and* have the typestate derive the matching
> container and numeral representations, then validate, and erase on lowering to be exactly what you
> describe before that caveat.
>
> Anything less than that, no caveats left, is unacceptable for this design and canon.

So the acceptance criterion has four parts and all four must hold at once:

1. **The consumer expresses usage in bits and bytes.** Widths are what a consumer writes and thinks in.
2. **The typestate derives the matching container and numeral representation.** Not the consumer, not a
   later layer. Derived, per `110:3251` and `130b:39-48`.
3. **It validates.** The analysis, the laws and the refusals are real and they run.
4. **It erases on lowering.** What reaches the machine is the container and nothing else.

**No caveats left.** A design that satisfies three of four, or satisfies all four with a condition attached,
does not meet the bar. This is the standard the canon is judged against before promotion, and it is not
negotiable.

## What was verified, and it is the good half

The proposed shape was compiled rather than argued, on the pin `rustc 1.98.0-nightly (57d06900f)`, at `-O`,
with the numeral as `#[repr(transparent)]` carrying one real field and ZST markers for everything else.
Probe at `/private/tmp/claude-501/-Users-orgrinrt-Dev-clause-dev/transp/`.

Three compile-time assertions hold: `size_of` and `align_of` equal the native container's at sixteen bits and
`size_of` equals it at sixty-four.

The codegen result is stronger than equivalence. **LLVM's identical-code-folding collapsed the native
functions into the arvo ones and emitted symbol aliases**, so the whole object file defines three symbols:

```
_native16   = _arvo16          _arvo16:   add w0, w1, w0 ; ret
_native64   = _arvo64          _arvo64:   add x0, x1, x0 ; ret
_native_vec = _arvo_vec        _arvo_vec: 4x add.8h, 4x ldp, 2x stp
```

`native16`, `native64` and `native_vec` have no bodies, because there was nothing to distinguish them from.
The vectorised loop autovectorises identically at eight lanes. Parts 3 and 4 of the gate are met on this
shape, and part 2 is met by the container being derived rather than written.

## The caveat, which is what this checkpoint is actually about

The erasure above holds **because the operation body names a machine type**. File `132` measured the
alternative: a body honestly generic over a byte count, a ripple-carry loop over `[u8; B]`, which is the only
body a design with no rung ladder could write. **Sixty-six instructions against two** at eight bytes
(`132:308-318`), and the vector form deinterleaves with `xtn.8b` and `shrn.8b`, does byte-lane adds with an
explicit carry chain, and reinterleaves, where the native form does one `add.8h`. That is not a missed
optimisation. It is the program the source wrote, and LLVM cannot recover the wide add because the carry
chain is a real data dependence.

I reported this to op as a bounded caveat, on the reading that the rung ladder is what buys the erasure and
that every route on the table keeps it, so nothing currently proposed costs transparency.

**Op's answer is that a bounded caveat is still a caveat, and it is not addressed.** His position: the
condition should not exist. The typestate has, or can be given, everything it needs to derive the container
and the representation from the widths the consumer wrote, validate against them, and erase, without the
operation body having to name a machine type as a precondition for the erasure to happen.

That reframes it. The panel has been treating "the body must name a machine type" as a fact about Rust to be
designed around. Op is treating it as a statement about where the derivation currently stops, and therefore
as something the design must close rather than route around. Given his record this session, that reading
deserves to be tested before it is doubted: he has overturned five converged panel conclusions, every one by
asking why an assumption was there rather than whether the argument was sound.

## What this changes about the work in flight

**The step A / step B seam is not the end of the analysis.** `132` drew it and four files have carried it as
settled: step B, rung to machine type, is gate-free with native-identical codegen; step A, widths to a rung,
is the whole cost. That framing is correct about the *gates*. It is silent about the *condition*, because
step B is exactly the thing that names the machine type, so the seam explains why the erasure works without
asking whether it could work without a ladder at all. `133:72-73` flagged the seam as premise-dependent in
one sentence nobody carried, and `134b` listed it among its seven pushbacks. This is the second time it has
come up and the first time it has come up from op.

**The container fork is not affected in its shape**, since every route keeps step B and therefore keeps the
erasure. It is affected in what counts as done: a route that meets the gate only through the ladder's naming
of a machine type has a caveat attached, and op has just said a caveat attached is not acceptable.

**A dispatch is owed on this specifically**, and it is not the one running. `136` is writing the view
homomorphism, identity and order laws and should finish. The next dispatch after it takes the gate: whether
the derivation can be closed so that the erasure holds unconditionally, what the typestate would have to
carry, and what remains if it cannot.

## What is op's, and what is not

**His, and now stated:** the gate itself. Four parts, all at once, no caveats.

**Not his to be talked out of:** whether a bounded caveat is acceptable. He has ruled it is not, and the
panel's job is to close it rather than to price it.

**Open, and the subject of the next dispatch:** whether the condition can be removed, and how. Op asserts a
way exists. That assertion is a direction to search, not a result, and the search has to be compiled like
everything else in this panel. If it cannot be closed, that is a finding op needs, stated with what it is
quantified over, rather than a caveat quietly carried forward.

## Standing

Only op's calls are final, and by his own principle (`108b:11-20`) they go stale when their evidence moves.
The panel produces canon, not source. `mock/research/` and `mock/benches/` are its ground and `mock/crates`
is out of bounds until the canon is complete and earmarked as arvo's first full canon. Experts are dispatched
one at a time, never in parallel, each reading the ones before it, and each writes its file incrementally
because four dispatches in this panel have died mid-flight and only the one that had saved a partial lost
nothing.

The consolidation is promoted to canon whole and supersedes everything before it, so comprehensiveness is a
requirement rather than a virtue. This gate is part of what it is measured against.
