# 22. The bench that was missing

**Date:** 2026-08-08. **Author:** Xu persona. **Status:** IN PROGRESS, written to disk early and extended in
place. Nothing here settles anything.

Twenty-one files argued. This one measures. The target is the hole `20` named in its section 3.4 and 4: the
wide rung above 64 bits, where the design leaves native containers behind, is governed by a **ratified** rule
that rests on an instruction count at a single numeral, and no committed harness run touches it at all.

The ratified text, `seed/SETTLED_container.md:345-350`, quoting `137b:47-53`:

> **Adopted.** Above the native rungs a wide payload is **ragged** for `Cold` and `Precise`, sized to the
> exact bits, and **word-rounded** to whole 64-bit limbs for `Hot` and `Warm`. Measured at one numeral:
> ragged is fourteen instructions and twenty-five bytes, word-rounded is eleven and thirty-two. Three
> instructions per operation against seven bytes per value is exactly the trade the strategy axis exists to
> carry, so nothing new is invented and the axis absorbs it.

Three instructions against seven bytes, at one numeral, counted rather than run. That is the claim this file
puts on the harness.

## Status log

This section is written as work proceeds so that a dispatch that dies mid-flight still leaves a record.

- Gates run, panel and predecessor read, harness structure under study.

- Harness studied. Five facts established before writing a line, each with the command that produced it.

## Zero: the brief's claims, and one of them is wrong

The dispatch says to assume it is wrong and check the cheap claims. Four were checkable.

**"No harness run touches the wide rung."** True. Every `n` key in `bench.toml` decodes to a declared width
of 64 bits or less, and the widest carrier any arm instantiates is `u128`.

**"20 found the fidelity columns are zero in all 55,280 rows."** True, and I reproduced it. But the
conclusion the brief draws from it, that no committed bench has ever verified its arms compute the same
thing, is **wrong**, and the error is worth stating because it changed what I had to build.

`bench-harness/src/validation.rs:1-23` documents a validation pass that runs **before any timing**, over
100 seeds by default (`DEFAULT_VALIDATION_SEEDS`), in three modes: the routine's own `validate_output`
per variant, an approximate cross-variant comparison, and a byte-exact cross-variant comparison as the
default. `validation.rs:373` picks a baseline variant and compares the rest against it.

So there are **two** fidelity mechanisms and they are not the same thing. The `digest` and `score` CSV
columns are a reps-invariant witness computed inside the timed loop, and those are indeed all zero for
plain `timed!` variants. The validation pass is separate, it runs, and it is what refused nothing in the
six void cells because the arms all returned the same input-independent constant. `20`'s substance is
exactly right and its sentence "the only check on what a variant computed is the variant crate's own unit
tests" is loose. The check ran. It was vacuous.

That distinction is the whole design of this bench's fidelity story, so it had to be settled first.

**"`mock/crates` must not be written to."** Consistent with the phase gate, which I read rather than
assumed. `mockspace/src/render_agent/builtins.rs:363-382` phase-gates exactly two things under the mock
directory: paths matching `^crates/` and the root `mock/Cargo.toml`. Everything else, including
`mock/benches/`, falls through to `allow` at line 391. The round is in **TOPIC** phase (`design_rounds/`
holds three flat topic files and no changelist), so a `mock/Cargo.toml` edit is gated and bench source is
not. That is a blocker with a workaround and it is section 3.

**The pin.** `rustc --version` inside the tree returns `rustc 1.98.0-nightly (57d06900f 2026-05-27)`,
matching `rust-toolchain.toml`'s `nightly-2026-05-28`. Correct.

