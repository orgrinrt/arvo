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

- Bench built, 30 tests green, five cdylibs, control verified by disassembly, l1 and density sections run.

## One: what was built

`mock/benches/variants/wide-rung-shared/` plus five leaf cdylibs, registered in `bench.toml` as three
sections and in `benches/src/main.rs` as sixteen routine bridges. Committed at `441c0b3` **before** the
first run, which matters and is section 3.

Five arms over **one** transform. No arm carries its own copy of the arithmetic; they differ in exactly
one thing, the loader, and `load.rs` is the whole of that difference.

| arm | payload | how a limb is read |
|---|---|---|
| `ragged` | `ceil(W/8)` bytes | whole unaligned limbs, then the 1 to 7 byte tail from the largest power-of-two loads that fit |
| `ragged-overread` | `ceil(W/8)` bytes | every limb one unaligned 64-bit load, the last reaching into the neighbour, top limb masked |
| `wordround` | `8*ceil(W/64)` bytes | aligned 64-bit loads |
| `wordround-alias` | identical stride, different expression | identical, and this is the **noise floor** |
| `align16` | `ceil(W/8)` rounded to 16 | aligned 64-bit loads |

Widths 129, 160, 192, 200, 232, 256. All above the 128-bit native rung. `W = 200` is the numeral the
ratified claim was counted at: `rag_bytes(200) = 25` and `wr_bytes(200) = 32`, which is its "twenty five
bytes against thirty two", asserted by `w200_is_the_numeral_the_ratified_claim_was_counted_at`.

### The control is a control, established by disassembly rather than by argument

`wordround-alias` computes the same stride from `ceil(ceil(W/8)/8)*8` instead of `ceil(W/64)*8`. Equal at
every width, asserted over 1 to 512 bits. The two arms should therefore be the same program in two
cdylibs, and that is checkable rather than assumable:

```
wordround       symbol: ...4arms9wordround       -> 2767 instructions
wordround_alias symbol: ...4arms15wordround_alias -> 2767 instructions
mnemonic sequences identical: True
differing instruction indices: [2748, 2751, 2759, 2762]
first difference at index 2748 of 2767, i.e. 99.3% into the function
  [2748] 'add x9, x9, #0xX'  vs  'add x9, x9, #0xX'
```

Same instruction count, same mnemonic sequence, and the only four differences are immediate fields in
`add xN, xN, #imm` pairs at 99.3% into the function, which are the string-table addresses of the panic
messages. The two arms run identical code, so the spread between them on a row is that row's own
run-to-run variation.

### Why the answer cannot be deleted

`20` section 2.1's mechanism is a saturating fold reaching an absorbing value. Every operation in this
bench's cycle is a **bijection** on the `W`-bit residues, so nothing is absorbed:

- addition of a constant modulo `2^W`, inverse is subtraction,
- exclusive or with a constant, its own inverse,
- multiplication by three modulo `2^W`, invertible because three is odd.

The third is the one that needed proving rather than asserting, and proving it found a bug in my own
first attempt at the proof. That test used `3^-1 mod 2^64` against values reduced modulo `2^W` and failed
at `W = 129`, correctly: `3 * (3^-1 mod 2^64) mod 2^129` is `36893488147419103233`, not one, because the
inverse at 129 bits is a different number. The repair constructs the inverse at the declared width by
Newton iteration and checks both that it is one and that it round-trips.

`the_answer_moves_when_any_single_element_moves` is the consequence, asserted at **every** declared key,
at three positions including the last element, with no `black_box` anywhere. `20` found the previous
family's equivalent diagnostic passing because `black_box` on the operand hid the constant that made the
fixpoint provable, so it proved the loop ran in a configuration the bench never used.

Thirty tests, all green: `cargo test --release -p bench-wide-rung-shared`.

## Two: the numbers, cache-resident

Medians of `algo_ns` in warm mode, computed from the committed CSVs rather than read out of the findings
prose. 2048 elements, 3 operations per element, every arm reading its own contiguous region.

| W | ragged B | wr B | a16 B | ragged | rag-over | wordround | CONTROL | align16 | wr/rag | control gap |
|---|---|---|---|---|---|---|---|---|---|---|
| 129 | 17 | 24 | 32 | 4133.9 | 4037.5 | 4026.1 | 3890.8 | 3939.4 | 0.974 | 3.36% |
| 160 | 20 | 24 | 32 | 4134.6 | 3988.8 | 4124.2 | 4034.2 | 4021.4 | 0.997 | 2.18% |
| 192 | 24 | 24 | 32 | 3910.6 | 3652.7 | 3651.0 | 3669.8 | 3727.5 | 0.934 | 0.51% |
| 200 | 25 | 32 | 32 | 5794.8 | 5607.5 | 5719.8 | 5603.8 | 5603.1 | 0.987 | 2.03% |
| 232 | 29 | 32 | 32 | 5806.6 | 5325.2 | 5414.2 | 5497.5 | 5458.3 | 0.932 | 1.54% |
| 256 | 32 | 32 | 32 | 5151.0 | 5310.0 | 5324.4 | 5164.6 | 5324.0 | 1.034 | 3.00% |

**At the ratified numeral, cache-resident, the word-rounded arm is 1.3 percent faster than ragged and the
control gap on that same row is 2.0 percent.** The difference is inside the instrument's own noise.

And the ordering does not hold: at `W = 256` ragged is the fastest arm and word-rounded the slowest, at
`W = 192` the reverse. Across the whole sweep every arm sits within about seven percent of every other,
with a noise floor reaching 3.4 percent.

The operation-count sweep at the ratified numeral says the same thing more directly:

| D | ragged | rag-over | wordround | CONTROL | align16 | wr/rag | control gap |
|---|---|---|---|---|---|---|---|
| 1 | 4954.6 | 5031.1 | 4676.9 | 4646.0 | 4696.9 | 0.944 | 0.66% |
| 2 | 4033.1 | 4185.4 | 4005.8 | 4005.0 | 4203.4 | 0.993 | 0.02% |
| 3 | 5794.8 | 5607.5 | 5719.8 | 5603.8 | 5603.1 | 0.987 | 2.03% |
| 4 | 8203.3 | 8218.8 | 8255.8 | 8224.0 | 8213.4 | 1.006 | 0.39% |
| 8 | 11774.5 | 11626.0 | 11530.7 | 11526.0 | 11577.5 | 0.979 | 0.04% |

The ratified claim is stated **per operation**: "three instructions per operation". If that were showing
up as throughput, the gap between ragged and word-rounded would grow with the operation count, because
the instruction count grows with it and the byte count does not. It does not grow. From one operation to
eight the ratio moves 0.944, 0.993, 0.987, 1.006, 0.979, with no trend and with the sign changing twice.

## Three: provenance, and a correction to `20` section 2.6

`20` reports 146 of 147 committed runs taken from a dirty tree and reads it as the commit discipline
having failed. I committed at `441c0b3` with `git status --porcelain` verified **empty**, ran, and got
this:

| file | git_commit |
|---|---|
| `wide-rung-width-l1_n129003.meta.json` | `441c0b3` |
| `wide-rung-width-l1_n160003.meta.json` | `441c0b3-dirty` |
| every later file | `441c0b3-dirty` |

The first size row is clean and every subsequent one is dirty. Nothing in the source changed:
`git diff --name-only HEAD` returns zero files, and all thirty dirty entries are `??`, untracked, and are
the harness's own CSV, meta and findings output.

`bench-harness/src/env.rs:55-62` computes the flag as `git status --porcelain` being non-empty. The
harness writes its artifacts into the tree it then hashes, so **the second size row of any run is dirty
because the first size row wrote a file.** The flag is self-inflicted and unavoidable for every run after
the first artifact lands.

So `20`'s count is right and its reading is not. This is not a discipline failure, it is the harness
reporting on itself, and the useful remedy is upstream: collect the environment once at run start, or
exclude the output directory from the dirtiness check. Until then a `-dirty` suffix on a bench artifact
carries no information about whether the source moved, which weakens it exactly where `20` leaned on it
in section 2.5 to argue that two spliced runs were at different commits. That splice argument still
stands on the differing **hashes** (`75710b6` against `350953f`), which is a real difference; the
`-dirty` half of it is noise.

## Four: the instruction half of the ratified claim, in its own units

The harness cannot check an instruction count on this host: `instructions` and `cycles` read zero in all
55,280 committed rows, so aarch64 macOS gives the harness no counters. The claim is stated in
instructions, so it is answered by reading the emitted code.

`22_probes/loop-shape/` is an **ad-hoc quick spike**, named that, with no timer, exporting one arm at one
width and one operation count so a count is not a mixture of thirty inlined monomorphisations. Its full
output and generator are committed beside it. Findings in `22_probes/loop-shape/FINDINGS.md`.

Hot loop, `D = 3`:

| W | ragged B | wr B | ragged | rag-over | wordround | ragged minus wordround |
|---|---|---|---|---|---|---|
| 129 | 17 | 24 | 26 | 26 | 26 | 0 |
| 160 | 20 | 24 | 27 | 27 | 27 | 0 |
| 192 | 24 | 24 | 27 | 27 | 27 | 0 |
| 200 | 25 | 32 | 38 | 38 | 38 | **0** |
| 232 | 29 | 32 | 45 | 42 | 42 | +3 |
| 256 | 32 | 32 | 41 | 41 | 41 | 0 |

At the ratified numeral the two loops are the same length and differ in exactly **one** instruction:
`ldrb w19, [x12, #imm]` where the word-rounded arm has `ldr w19, [x12, #imm]`. The claimed three
instructions are zero.

**And it is per element, not per operation.** The claim says "three instructions per operation", so the
gap should grow with the operation count. It does not:

| W | D=1 | D=3 | D=8 |
|---|---|---|---|
| 129 | 0 | 0 | 0 |
| 160 | 0 | 0 | 0 |
| 192 | 0 | 0 | 0 |
| 200 | 0 | 0 | 0 |
| 232 | +4 | +3 | +3 |
| 256 | 0 | 0 | 0 |

Flat, while the loop body itself grows from 16 to 56 instructions at `W = 129` and 31 to 89 at
`W = 232`. A per-operation cost could not hide in that.

### The mechanism, and it is a closed form

Sweeping the whole tail residue class with the limb count held at four, so only the tail moves:

| W | ragged B | tail | popcount | ragged | rag-over | wordround | delta |
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

```
extra(W) = 3 * max(0, popcount(rag_bytes(W) mod 8) - 1)
```

Exact at all eight residues. The tail is assembled from the largest power-of-two loads that fit, so a
tail of one, two or four bytes is one load and costs nothing against the word-rounded form's one load,
while three, five and six are two loads plus a combine and seven is three plus two.

**The over-reading loader is level with word-rounded at every width in the class, including all four
non-power-of-two tails.** So the instruction half of the ratified trade is a property of one way of
writing the load, not of the payload shape.

Two confirmations nobody arranged, from symbols the linker folded because the machine code was identical:
`align16 == wordround` at `W` in 200, 232, 256 where both strides are 32 bytes, and
`ragged-overread == ragged` at `W` in 192 and 256 where the ragged payload is a whole number of limbs.
Both are what the shape arithmetic predicts.

## Five: the footprint half, past L2, and why it does not pay either

458752 elements, 3 operations per element. Sizes chosen so the shapes straddle this host's 12 MiB L2 at
the ratified numeral: ragged is 11.5 MB and word-rounded is 14.7 MB.

| W | rag MB | wr MB | a16 MB | ragged | rag-over | wordround | CONTROL | align16 | wr/rag | control gap |
|---|---|---|---|---|---|---|---|---|---|---|
| 129 | 7.8 | 11.0 | 14.7 | 953155 | 896843 | 910037 | 911418 | 915636 | 0.955 | 0.15% |
| 160 | 9.2 | 11.0 | 14.7 | 947957 | 906838 | 916572 | 915804 | 922069 | 0.967 | 0.08% |
| 192 | 11.0 | 11.0 | 14.7 | 866121 | 869084 | 871333 | 866735 | 870352 | 1.006 | 0.53% |
| 200 | 11.5 | 14.7 | 14.7 | 1349341 | 1317446 | 1320593 | 1323701 | 1322162 | 0.979 | 0.24% |
| 232 | 13.3 | 14.7 | 14.7 | 1345130 | 1259853 | 1253558 | 1253812 | 1255216 | 0.932 | 0.02% |
| 256 | 14.7 | 14.7 | 14.7 | 1213080 | 1210327 | 1213671 | 1210952 | 1212711 | 1.000 | 0.22% |

The control gap collapses to between 0.02 and 0.53 percent at this size, so unlike the cache-resident
sweep the instrument here is tight enough to resolve a few percent.

**And the word-rounded arm is faster at four of the six widths, while touching up to 41 percent more
bytes.** At `W = 129` it reads 11.0 MB against ragged's 7.8 and wins by 4.5 percent. At the ratified
numeral it reads 14.7 MB against 11.5, does not fit in L2 where ragged does, and still wins by 2.1
percent. The two collision widths, 192 and 256, come out at 1.006 and 1.000, which is the free control
firing exactly where the shape arithmetic says it must.

### Why, established rather than asserted

| W | arm | MB read | ms/call | GB/s | limb-ops | Glimb-ops/s |
|---|---|---|---|---|---|---|
| 129 | ragged | 7.8 | 0.953 | 8.18 | 5.5M | 5.78 |
| 129 | wordround | 11.0 | 0.910 | 12.10 | 5.5M | 6.05 |
| 200 | ragged | 11.5 | 1.349 | 8.50 | 7.3M | 5.44 |
| 200 | wordround | 14.7 | 1.321 | 11.12 | 7.3M | 5.56 |
| 232 | ragged | 13.3 | 1.345 | 9.89 | 7.3M | 5.46 |
| 232 | wordround | 14.7 | 1.254 | 11.71 | 7.3M | 5.86 |
| 256 | ragged | 14.7 | 1.213 | 12.10 | 7.3M | 6.05 |
| 256 | wordround | 14.7 | 1.214 | 12.10 | 7.3M | 6.05 |

Achieved bandwidth is 8 to 12 GB/s against roughly 60 available on an M1. **The walk is nowhere near
bandwidth-bound.** And the limb-operation rate is 5.44 to 6.05 across every arm and every width, a spread
of eleven percent over a set of arms whose byte counts differ by forty-one percent.

So the loop is **issue-bound on the limb arithmetic**. Every arm issues the same limb arithmetic, so
every arm runs at the same rate, and the bytes a shape saves buy nothing because the bytes were never the
constraint. Saving 41 percent of a resource that is 85 percent idle is not a saving.

That is a mechanism rather than a result, and a mechanism is something to attack. Section six is the
attack.

