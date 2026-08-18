# Probe C: how much a forward-only width rule over-provisions

```sh
rustc -O -o widths widths.rs && ./widths > widths.out
```

An exact static bit count over five chains. **No timing is taken and no magnitude of speed or size is
claimed.** The backward rule uses exactly the operator partition probe B established empirically.

| chain | forward bits | forward+backward | saved |
|---|---|---|---|
| MAC x4, 16-bit inputs, 16-bit sink | 228 | 112 | 116 (50.9%) |
| the same chain, consumer keeps everything (NC10) | 228 | 228 | **0** |
| Horner degree 4, 12-bit, 12-bit sink | 352 | 96 | 256 (72.7%) |
| Horner degree 4 with one right shift in the middle | 365 | 158 | 207 (56.7%) |
| all-blocking chain, div and min, 8-bit sink | 72 | 56 | 16 (22.2%) |

NC10 clean: with the consumer keeping every bit the forward rule produced, the saving is exactly zero.
NC11 clean: one chain saves zero and four save more.

**Two sub-findings the table makes visible.** Inserting a single right shift into the Horner chain drops
the saving from 72.7% to 56.7%, because the shift blocks the demand from propagating past it: one
non-congruence operator anywhere in a chain truncates the region the backward rule reaches. And in the
all-blocking chain **the entire 16-bit saving is the sink node alone** (24 down to 8) with nothing
propagating: the sink always narrows, and what the operator partition governs is whether anything above
it does.

`holds for: the five chain shapes listed, the width rules stated in the source, sink widths as listed,
threads = 1`
