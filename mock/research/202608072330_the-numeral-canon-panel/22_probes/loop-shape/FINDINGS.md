# Loop shape: the ratified instruction count, in its own units

**Ad-hoc quick spike, not a bench.** It emits no timing and can price nothing. It exists because the
ratified rule's claim is stated in instructions, and the harness cannot check that on this host: the
`instructions` and `cycles` columns read zero in all 55,280 committed rows on aarch64 macOS. An
instruction count is a structural fact, which is the one kind of claim a spike can carry.

Toolchain `rustc 1.98.0-nightly (57d06900f 2026-05-27)`, `opt-level = 3`, fat LTO, one codegen unit.
Each exported function is exactly one arm at one width and one operation count, so a count is not a
mixture of thirty inlined monomorphisations the way the bench's own arm functions are.

Sources: `src/lib.rs` (regenerated per sweep). Raw output: `raw_output.txt`, `tail_sweep_output.txt`.

## The claim

`seed/SETTLED_container.md:345-350`, quoting `137b:47-53`: ragged is "fourteen instructions and twenty
five bytes, word-rounded is eleven and thirty two. Three instructions per operation against seven bytes
per value."

## What the emitted code says

Hot loop, `D = 3`, at the six swept widths:

| W | ragged bytes | wr bytes | ragged | rag-over | wordround | ragged minus wordround |
|---|---|---|---|---|---|---|
| 129 | 17 | 24 | 26 | 26 | 26 | 0 |
| 160 | 20 | 24 | 27 | 27 | 27 | 0 |
| 192 | 24 | 24 | 27 | 27 | 27 | 0 |
| 200 | 25 | 32 | 38 | 38 | 38 | 0 |
| 232 | 29 | 32 | 45 | 42 | 42 | +3 |
| 256 | 32 | 32 | 41 | 41 | 41 | 0 |

At `W = 200`, the numeral the claim was counted at, the two loops differ in **one** instruction and the
count is equal: `ldrb w19, [x12, #imm]` where word-rounded has `ldr w19, [x12, #imm]`. Everything else
in the 38-instruction body is identical.

## It is per element, not per operation

The claim says "per operation". If it were, the gap would grow with the operation count, because the
loop body grows with it and the load does not.

| W | D=1 | D=3 | D=8 |
|---|---|---|---|
| 129 | 0 | 0 | 0 |
| 160 | 0 | 0 | 0 |
| 192 | 0 | 0 | 0 |
| 200 | 0 | 0 | 0 |
| 232 | +4 | +3 | +3 |
| 256 | 0 | 0 | 0 |

Flat, while the loop itself goes from 16 to 56 instructions at `W = 129` and 31 to 89 at `W = 232`. The
cost is per **element**, paid once in the load, and it does not scale with the arithmetic.

## The mechanism, as a closed form

The whole residue class, limb count held at four so only the tail moves:

| W | ragged bytes | tail | popcount | ragged | rag-over | wordround | ragged minus wr |
|---|---|---|---|---|---|---|---|
| 192 | 24 | 0 | 0 | 27 | 27 | 27 | 0 |
| 200 | 25 | 1 | 1 | 38 | 38 | 38 | 0 |
| 208 | 26 | 2 | 1 | 38 | 38 | 38 | 0 |
| 216 | 27 | 3 | 2 | 41 | 38 | 38 | +3 |
| 224 | 28 | 4 | 1 | 37 | 37 | 37 | 0 |
| 232 | 29 | 5 | 2 | 45 | 42 | 42 | +3 |
| 240 | 30 | 6 | 2 | 45 | 42 | 42 | +3 |
| 248 | 31 | 7 | 3 | 48 | 42 | 42 | +6 |
| 256 | 32 | 0 | 0 | 41 | 41 | 41 | 0 |

The extra instruction count is exactly

```
extra(W) = 3 * max(0, popcount(rag_bytes(W) mod 8) - 1)
```

which holds at all eight residues. The tail is assembled from the largest power-of-two loads that fit,
so a tail of 1, 2 or 4 bytes is one load and costs nothing over the word-rounded form's one load; a tail
of 3, 5 or 6 is two loads plus a combine, and 7 is three loads plus two combines.

## The over-reading loader pays nothing, anywhere

`ragged-overread` is level with `wordround` at **every** width in both sweeps, including all four
non-power-of-two tails. It reads whole 64-bit limbs and lets the last one reach into the neighbour, and
the top-limb projection that removes the stray bytes is required by the width anyway.

So the instruction half of the ratified trade is an artifact of one way of writing the load.

## What the linker merged

Identical machine code, folded to one address, which is the strongest available statement that two arms
are the same program:

- `align16 == wordround` at `W` in 200, 232, 256, where both strides are 32 bytes.
- `ragged-overread == ragged` at `W` in 192, 256, where the ragged payload is a whole number of limbs
  and the two loaders coincide.

Both are exactly what the shape arithmetic predicts, and neither was arranged.

## What this does not establish

Instruction counts are not throughput. Whether an instruction costs anything depends on whether the loop
is issue-bound or bandwidth-bound, and this spike says nothing about that. The harness sections
`wide-rung-width-l1`, `wide-rung-width-l2` and `wide-rung-density-w200` are where that question is
answered.

It is also one target. `aarch64` on an Apple M1 tolerates unaligned loads cheaply and has `ldp`; a target
without either could order these arms differently.
