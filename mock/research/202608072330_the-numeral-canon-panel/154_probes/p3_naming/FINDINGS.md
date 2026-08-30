# P3 findings: a name costs what its body computes, and nothing for the name

`python3 gen.py`, toolchain `nightly-2026-05-28`, host aarch64-apple-darwin. Generator, generated
source, emitted assembly and output all committed beside this file.

**Ad-hoc quick spike with no substance as a measurement.** Symbol and body counts are exact; the
object-file byte size is reported for the record and prices nothing. `mock/benches/` was not used and
no magnitude claim is made.

## The controls, and the one that failed first

Two families, 20 named primitives each. `d_*` distinct by construction (a different mask per width),
`s_*` identical by construction (every one computes `x & mask(13)` under its own name).

**The first version of the identical-family control was wrong and refused to print.** It expected
that family to keep one body of its own. It keeps **zero**: all 20 fold onto `_d_wrap_13`, a member
of the *other* family, because that function already computes `x & mask(13)`. The corrected control
requires 0 bodies, 20 aliases, and exactly one distinct alias target, and that is stronger than what
it replaced. Both controls now pass:

```
CONTROL distinct-family : 20 bodies + 0 aliases (want 20 bodies, 0 aliases)
CONTROL identical-family: 0 bodies + 20 aliases (want 0 bodies, 20 aliases, 1 distinct target); targets = ['_d_wrap_13']
controls: ok
```

`grid.s:150-188` is the alias block; every line is `_s_alias_N = _d_wrap_13`.

## F8. Forty named primitives produce twenty machine bodies

The count is the grid's semantic content, not its naming. Ten widths times two policies is twenty
distinct behaviours and twenty bodies; twenty further names for behaviour that already exists cost
nothing but a symbol-table entry.

This generalises F4 from a coincidence between two functions to a rule over a grid, and it is the
second independent instance of the same mechanism (F4 was two functions in `p1_saturation`; this is
forty across two families, counted by a different instrument).

`holds for:` W in {3, 7, 11, 13, 14, 23, 27, 31, 47, 61}, policies in {wrap, clamp}, container = u64,
arity 1, `nightly-2026-05-28`, target aarch64-apple-darwin, opt-level = 3, threads = 1, F = 0.

## F9. What the 19 MB rlib is therefore about, and what it is not about

`mock/Cargo.toml:44-52` records the shipped cost, in the repository's own words:

> arvo's cost is in monomorphised output, not in source it could reuse: the
> per-width, per-strategy, per-sign impl macros turn a small file into an
> enormous object, and `libarvo_strategy.rlib` alone was 19 MB.

F8 says that cost cannot be attributed to the *naming* scheme. Names that name nothing new are free.
So a 19 MB object means the grid genuinely contained that many **distinct behaviours**, or that the
bodies were emitted before anything could fold them, which is a build-configuration question rather
than a design one.

**This is a claim about attribution and not about magnitude.** I did not build arvo, I cannot, and
the 19 MB figure is quoted from the manifest rather than reproduced. The useful consequence for the
canon: an argument of the form "fewer named primitives, because monomorphisation is expensive" does
not follow from these findings, and an argument of the form "fewer *distinct behaviours*" does.

`holds for:` this attribution reasoning holds where identical-code folding is enabled, which is
`opt-level = 3` with the default linker on aarch64-apple-darwin as measured here, threads = 1, F = 0.

## F10. The cost side of naming is not where the interesting cost is

Combining F8 with P1/F4: at runtime, a name buys nothing observable, because the machine merges
primitives that agree, and it costs nothing either, for the same reason. **The entire economics of
naming is compile-time.** That is not a small conclusion, because it means the case for naming a
primitive has to be made entirely on what a name lets you *state*, and the case against it cannot be
made on emitted size unless the names correspond to distinct behaviours, in which case the behaviours
are the cost and not the names.

`holds for:` as F8.

## What would refute each

F8: a grid where naming alone multiplies bodies. That needs folding disabled; I did not sweep linker
configurations and claim nothing outside the one measured.
F9: a build of arvo showing the 19 MB is dominated by foldable duplicates. That would refute the
attribution and is exactly the check to run when a crate exists again.
F10: any observable at runtime that distinguishes two merged primitives. By F4 and F8 they are one
address, so this needs the merge not to happen.
