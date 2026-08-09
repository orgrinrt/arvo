# Outcome: emitted assembly agrees with the linked binary, unless LTO is on

**Pin:** `nightly-2026-05-28`, `rustc 1.98.0-nightly (57d06900f 2026-05-27)`.
**Host:** aarch64-apple-darwin. **Run:** `./run.sh`.

One function (`accumulate`, a wrapping multiply-add over two `u32` slices), one source
file, three profiles. Vector-register mentions in the function body.

| profile | `--emit=asm` on the lib | linked binary |
|---|---|---|
| `release` (no LTO) | 16 | 16 |
| `release-thin` | **0** | 16 |
| `release-fat` | **0** | 11 |

**WORKS**, in the sense that the question is answered decisively in both directions.

**Under LTO the emitted assembly is a reading of nothing.** Zero vector operations for a
function the linked binary vectorises either sixteen or eleven ways. The pre-link pipeline
defers the loop vectoriser to the LTO backend, and `--emit=asm` on a library never reaches
that backend. This confirms file 34's finding independently, by a different author on a
different function.

**Without LTO the emitted assembly is faithful.** Sixteen and sixteen. So the shape every
other asm-reading probe in this panel used, bare `rustc -O --crate-type=lib --emit=asm` with
no LTO flag, is sound, and the absence of inlining context cost nothing measurable here.

An intermediate count of 9 for the no-LTO library was recorded while writing this probe and
was a defect in the matching pattern, which missed the `.2d v0` operand order. It is 16. The
scripted `count` function is the corrected form.

## What it does not establish

One function on one host. The equality in the no-LTO row shows the library read *can* be
faithful, not that it always is: a function whose vectorisation depends on a caller's known
trip count or alignment would read differently in isolation, and this one depends on neither.
