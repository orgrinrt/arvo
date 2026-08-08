# 22. The bench that was missing

**Date:** 2026-08-08. **Author:** Xu persona. **Status:** complete. Nothing here settles anything.

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

## What is here

Five harness sections and one spike. Sections two, five, six and eight are the numbers; section four is
the instruction count the claim is actually stated in; section seven is a defect in the instrument that
was found by trying to demonstrate the fidelity check, and it refutes part of this file's own section
zero. Sections nine through twelve are what it settles, what it does not, what I had to decide, and what
is op's.

**Section zero contains a claim I later disproved.** It is left standing with a pointer rather than edited
away, because a file that quietly repairs its own errors is a file whose other claims cannot be weighed.
See section seven.

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

**And the paragraph above is wrong, which section seven establishes by demonstration.** The validation
module exists and is documented; nothing calls it. `20`'s sentence was right and this correction of it was
not. The paragraph stays as written so the error is visible rather than laundered.

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
| 129 | 17 | 24 | 32 | 4213.1 | 3958.3 | 3973.6 | 4015.2 | 4025.8 | 0.943 | 1.05% |
| 160 | 20 | 24 | 32 | 4138.9 | 3971.4 | 3968.1 | 3929.4 | 3989.6 | 0.959 | 0.98% |
| 192 | 24 | 24 | 32 | 3866.9 | 3774.6 | 3720.4 | 3789.6 | 3743.1 | 0.962 | 1.86% |
| 200 | 25 | 32 | 32 | 5902.1 | 5813.1 | 5793.1 | 5714.6 | 5719.1 | 0.982 | 1.36% |
| 232 | 29 | 32 | 32 | 5900.8 | 5447.1 | 5460.2 | 5461.2 | 5461.1 | 0.925 | 0.02% |
| 256 | 32 | 32 | 32 | 5245.6 | 5243.3 | 5243.1 | 5249.8 | 5250.8 | 1.000 | 0.13% |

**At the ratified numeral, cache-resident, the word-rounded arm is 1.8 percent faster than ragged and the
control gap on that same row is 1.36 percent.** The difference is barely outside the instrument's own
noise and it is in the direction opposite to the one the extra bytes would predict.

Across the whole sweep the ratio never exceeds 1.000: word-rounded is between zero and 7.5 percent
faster than ragged at every swept width, while touching between zero and 41 percent more bytes. At
`W = 256`, where the two shapes are the same size, the ratio is 1.000 to three decimals with a control
gap of 0.13 percent, which is the free control firing exactly where the shape arithmetic says it must.

The operation-count sweep at the ratified numeral says the same thing more directly:

| D | ragged | rag-over | wordround | CONTROL | align16 | wr/rag | control gap |
|---|---|---|---|---|---|---|---|
| 1 | 4960.6 | 4951.4 | 4683.1 | 4650.0 | 4827.1 | 0.944 | 0.71% |
| 2 | 4163.6 | 4180.2 | 4115.6 | 4134.6 | 4161.5 | 0.988 | 0.46% |
| 3 | 5902.1 | 5813.1 | 5793.1 | 5714.6 | 5719.1 | 0.982 | 1.36% |
| 4 | 8499.2 | 8400.9 | 8393.5 | 8426.9 | 8481.2 | 0.988 | 0.40% |
| 8 | 11967.3 | 11912.5 | 11856.5 | 11834.5 | 11884.6 | 0.991 | 0.18% |

The ratified claim is stated **per operation**: "three instructions per operation". If that were showing
up as throughput, the gap between ragged and word-rounded would grow with the operation count, because
the instruction count grows with it and the byte count does not. It does not grow. From one operation to
eight the ratio moves 0.944, 0.988, 0.982, 0.988, 0.991, drifting **toward** parity as the operation
count rises, which is the opposite of a per-operation cost and exactly what a fixed per-element cost
amortised over more arithmetic looks like.

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
| 129 | 7.8 | 11.0 | 14.7 | 961538 | 907050 | 921099 | 925180 | 933531 | 0.958 | 0.44% |
| 160 | 9.2 | 11.0 | 14.7 | 958219 | 911590 | 918790 | 912369 | 917954 | 0.959 | 0.70% |
| 192 | 11.0 | 11.0 | 14.7 | 871810 | 871082 | 872304 | 876164 | 882022 | 1.001 | 0.44% |
| 200 | 11.5 | 14.7 | 14.7 | 1352074 | 1323812 | 1328056 | 1325396 | 1326119 | 0.982 | 0.20% |
| 232 | 13.3 | 14.7 | 14.7 | 1349232 | 1261802 | 1254780 | 1254229 | 1253845 | 0.930 | 0.04% |
| 256 | 14.7 | 14.7 | 14.7 | 1216489 | 1212281 | 1213799 | 1215679 | 1220266 | 0.998 | 0.15% |

The control gap is between 0.04 and 0.70 percent at this size, so the instrument here resolves a few
percent comfortably.

**And the word-rounded arm is faster at every width where the shapes differ, while touching up to 41
percent more bytes.** At `W = 129` it reads 11.0 MB against ragged's 7.8 and wins by 4.2 percent. At the
ratified numeral it reads 14.7 MB against 11.5, does not fit in this host's 12 MiB L2 where ragged does,
and still wins by 1.8 percent. The two collision widths, 192 and 256, come out at 1.001 and 0.998, which
is the free control firing exactly where the shape arithmetic says it must.

### Why, established rather than asserted

| W | arm | MB read | ms/call | GB/s | limb-ops | Glimb-ops/s |
|---|---|---|---|---|---|---|
| 129 | ragged | 7.8 | 0.962 | 8.11 | 5.5M | 5.73 |
| 129 | wordround | 11.0 | 0.921 | 11.95 | 5.5M | 5.98 |
| 160 | ragged | 9.2 | 0.958 | 9.58 | 5.5M | 5.75 |
| 160 | wordround | 11.0 | 0.919 | 11.98 | 5.5M | 5.99 |
| 192 | ragged | 11.0 | 0.872 | 12.63 | 5.5M | 6.31 |
| 192 | wordround | 11.0 | 0.872 | 12.62 | 5.5M | 6.31 |
| 200 | ragged | 11.5 | 1.352 | 8.48 | 7.3M | 5.43 |
| 200 | wordround | 14.7 | 1.328 | 11.05 | 7.3M | 5.53 |
| 232 | ragged | 13.3 | 1.349 | 9.86 | 7.3M | 5.44 |
| 232 | wordround | 14.7 | 1.255 | 11.70 | 7.3M | 5.85 |
| 256 | ragged | 14.7 | 1.216 | 12.07 | 7.3M | 6.03 |
| 256 | wordround | 14.7 | 1.214 | 12.09 | 7.3M | 6.05 |

Achieved bandwidth is 8.1 to 12.6 GB/s against roughly 60 available on an M1. **The walk is nowhere near
bandwidth-bound.** And the limb-operation rate is 5.43 to 6.31 across every arm and every width, a spread
of sixteen percent over a set of arms whose byte counts differ by forty-one percent.

So the loop is **issue-bound on the limb arithmetic**. Every arm issues the same limb arithmetic, so
every arm runs at the same rate, and the bytes a shape saves buy nothing because the bytes were never the
constraint. Saving 41 percent of a resource that is 85 percent idle is not a saving.

That is a mechanism rather than a result, and a mechanism is something to attack. Section six is the
attack.

## Six: attacking the mechanism, and where it led

Issue-bound is a mechanism, and a mechanism is something to attack rather than report. The attack is to
raise the byte-to-work ratio: `wide-rung-walk-l2` is the same six widths and the same column at
**one** wide operation per element, the highest ratio the transform reaches. That required adding `D = 0`
to the dispatch table, and the thirty tests still pass with the new keys in `ALL_KEYS`, including
`the_answer_moves_when_any_single_element_moves`.

| W | rag MB | wr MB | ragged | rag-over | wordround | CONTROL | align16 | wr/rag | control gap |
|---|---|---|---|---|---|---|---|---|---|
| 129 | 7.8 | 11.0 | 452642 | 594007 | 612552 | 614618 | 616400 | 1.353 | 0.34% |
| 160 | 9.2 | 11.0 | 426676 | 422030 | 433851 | 431797 | 430054 | 1.017 | 0.47% |
| 192 | 11.0 | 11.0 | 299581 | 299886 | 302325 | 304580 | 325507 | 1.009 | 0.75% |
| 200 | 11.5 | 14.7 | 493391 | 494037 | 490335 | 492026 | 487811 | 0.994 | 0.34% |
| 232 | 13.3 | 14.7 | 499505 | 496094 | 503088 | 506343 | 485483 | 1.007 | 0.65% |
| 256 | 14.7 | 14.7 | 511070 | 500955 | 503465 | 506423 | 504500 | 0.985 | 0.59% |

A 35 percent win for ragged at `W = 129`, and nothing anywhere else: every other width is inside 1.7
percent with control gaps under 0.8. That looked like the footprint effect finally appearing, since 129 is
the width where the strides differ most (17 bytes against 24, a ratio of 1.412 against a measured
1.353).

**It is not, and the check that shows it is not is the most useful thing in this file.**

### Ruling out the confound first

Three rows in the 11.0 MB group varied by 2x while the three in the 14.7 MB group were flat within one
percent, and the variation tracked the order the rows ran in. That is a thermal or warm-up confound if it
is anything, so the section was run a second time, unchanged, and compared against the first:

| W | order | ragged r1 | ragged r2 | ragged delta | wordround r1 | wordround r2 | wordround delta |
|---|---|---|---|---|---|---|---|
| 129 | 1 | 446842 | 450311 | +0.8% | 611601 | 611592 | -0.0% |
| 160 | 2 | 424634 | 424299 | -0.1% | 431240 | 429519 | -0.4% |
| 192 | 3 | 289570 | 289887 | +0.1% | 299113 | 289688 | -3.2% |
| 200 | 4 | 495629 | 496440 | +0.2% | 492482 | 489784 | -0.5% |
| 232 | 5 | 496389 | 497430 | +0.2% | 487011 | 486145 | -0.2% |
| 256 | 6 | 496060 | 490560 | -1.1% | 496934 | 493685 | -0.7% |

Everything within 3.2 percent and most within one. The pattern follows the **width**, not the order.

Both runs in this table predate the fidelity fix in section seven, so their numbers are the pre-fix ones
and differ slightly from the table above, which is the final validated run. That does not weaken the
comparison: it is a paired experiment between two runs of identical code, and both members of the pair are
committed, run one at `22_probes/run-order/` and run two in the history at `96d86fd`. The conclusion it
supports, that the pattern is reproducible and follows the width, is confirmed again by the final run,
where `W = 129` comes out at 1.353 against the 1.369 and 1.358 recorded here.

### The discriminator, and it kills the footprint reading

`wide-rung-walk-l1` is the identical transform at 2048 elements, where the whole column is 34 to 64 KB
and sits in L1. If the pattern is about memory it must collapse there. In nanoseconds per element, so the
two sizes are directly comparable:

| W | L1 ragged | L1 wordround | L1 wr/rag | L2 ragged | L2 wordround | L2 wr/rag |
|---|---|---|---|---|---|---|
| 129 | 1.0049 | 1.3598 | 1.353 | 0.9867 | 1.3353 | 1.353 |
| 160 | 0.9545 | 0.9244 | 0.968 | 0.9301 | 0.9457 | 1.017 |
| 192 | 0.5992 | 0.6240 | 1.041 | 0.6530 | 0.6590 | 1.009 |
| 200 | 1.0506 | 1.0099 | 0.961 | 1.0755 | 1.0688 | 0.994 |
| 232 | 1.0149 | 1.0495 | 1.034 | 1.0888 | 1.0966 | 1.007 |
| 256 | 1.0086 | 1.1003 | 1.091 | 1.1140 | 1.0975 | 0.985 |

**The per-element cost barely moves between a 64 KB working set and a 15 MB one.** Every width is within
about seven percent of itself across a working set that grew by more than two hundred times and crossed
both cache levels.

So the walk is core-bound at every size measured, and the `W = 129` advantage is **1.353 in L1 and 1.353
past L2, identical to three decimal places**, in a working set where footprint cannot matter at all
against one where it is the whole point. **It is not a footprint effect.** Whatever produces it is a
property of the two loaders in the core at that width, and the bytes are not doing the work.

That is the negative result the whole exercise was for. On this host and this workload the seven bytes per
value the ratified rule trades away are a **memory-footprint quantity and not a throughput quantity**. A
consumer with millions of wide numerals still saves the bytes, and that saving is real and is what
`arvo-toolbox-not-policer.md` describes; it simply does not arrive as speed.

### What I could not close

The width-dependent baseline itself. At three limbs the per-element cost is 0.63 ns at `W = 192` and
1.33 ns at `W = 129`, a factor of 2.1 for the same stride, the same byte count, the same element count,
and loops that differ by one `and` instruction. Ruled out, each with the evidence that ruled it out:

- **Instruction count.** The two loops are 7 and 8 instructions
  (`22_probes/loop-shape/walk_loop_output.txt`). One instruction cannot be 2.1x.
- **Run order and thermal state.** Reproduced to within 3.2 percent on a second run of the same section.
- **Loaded-byte contiguity.** The hypothesis was that the word-rounded arm leaves a four-byte hole per
  element at 129, 160 and 200 and reads contiguously at 192, 232 and 256. It separates the three-limb
  group and fails on the four-limb group, where 200 is non-contiguous and 232 and 256 are contiguous and
  all three land within 1.5 percent of each other (`22_probes/loop-shape/contiguity_output.txt`).
- **The memory system.** Killed by the L1 discriminator above.

I have not found the mechanism and I am not going to invent one. It is a core-side, width-dependent
effect in multi-limb wrapping addition, it is worth two point one times on this host, and it is a better
lead for the next expert than anything I could speculate here. What would move it is a hardware
performance counter, which this host does not give the harness: `instructions` and `cycles` are zero in
every committed row.

## Seven: the fidelity check, and I was wrong in section zero

The brief required a fidelity check that actually runs, and that I show it firing. Showing it firing is
what found the largest defect in this file, and it also refutes my own section zero. I am correcting that
in place rather than editing it away.

### What I claimed, and why it was wrong

Section zero says `20`'s sentence "the only check on what a variant computed is the variant crate's own
unit tests" is loose, because `bench-harness/src/validation.rs` documents a hundred-seed validation pass
with a per-variant validator and a byte-exact cross-variant comparison.

The module exists. **Nothing calls it.** `bench-harness/src/lib.rs:141-147` is the whole of `harness::run`,
and it forwards to `run_orchestrator`, which never mentions `validate`. `grep -rn "validate(" bench-harness/src/`
outside `validation.rs` itself returns nothing but a doc comment. The consumer driver
`mock/benches/src/main.rs` did not call it either.

**`20` was right and my correction was wrong.** The validation pass is dead code in every committed run in
this directory.

### Demonstrated, not argued

A one-character change to the ragged loader's tail assembly, `rem >= 4` to `rem > 4`, which drops one byte
of the top limb at a tail of exactly four bytes and therefore corrupts `W = 160` and nothing else:

```
EXIT=0
  CSV: wide-rung-width-l1_n129003.csv (400 rows)
  CSV: wide-rung-width-l1_n160003.csv (400 rows)
  ... all six sizes, exit 0
```

**Four hundred rows of ordinary-looking numbers per size, and exit zero, from an arm computing a
provably wrong answer.** No column of the output marks it. The `digest` and `score` columns are zero in my
11,200 new rows exactly as they are in the 55,280 existing ones, because every variant here is a plain
`timed!` variant like every sibling.

The same bug is caught immediately by the crate's own tests, which name the width:

```
assertion `left == right` failed: arm `ragged` disagrees with the 128-bit-radix oracle at key 160003
assertion `left == right` failed: arm `ragged-overread` disagrees with `ragged` at key 160003 (W=160, n=2048, D=3)
test result: FAILED. 26 passed; 4 failed
```

So the fidelity check exists and runs, and it is the variant crate, at build time, which is exactly what
`20` said and exactly what section 2.1's six void cells needed.

### Fixed, in the consumer, and demonstrated firing

Two changes to `mock/benches/src/main.rs`, both small, both necessary, and the first alone is worse than
nothing.

**Call the validation pass before timing.** With only this, every variant is skipped and the harness
prints a false green:

```
  Validating 5 variants × 100 seeds...
  SKIPPING wide-rung-ragged: validation worker returned 0 of 100 outputs
  ... all five skipped
  Validation OK: all 0 variants produce identical output
  5 variants excluded (0 safe)
```

`Validation OK: all 0 variants produce identical output`, followed by timing all five anyway. That is a
vacuous pass over an empty set and it is worth naming: had I shipped the call on its own, the artifacts
would have carried a validation line that established nothing.

**Dispatch the `--mode validate` worker.** The pass spawns workers with `--mode validate` and a
comma-separated `--seeds` list and expects `VOUT` lines back. The driver routed every mode to
`harness::run_worker`, which answers a different protocol and emits nothing. The worker that answers it is
`mockspace_bench_harness::harness::run_worker_validate`, reachable because `harness` is a `pub mod`, and
**not re-exported at the crate root**: `bench-harness/src/lib.rs:88` exports `run_orchestrator`,
`run_worker` and `write_csv` and stops there. A consumer following the crate root alone cannot reach it,
which is the most likely reason no consumer ever wired this up.

With both, on the same injected bug:

```
  Validating 5 variants × 100 seeds...
  Validation OK: all 5 variants produce identical output      <- W=129, where a four-byte tail does not occur
  ...
  Validating 5 variants × 100 seeds...
  MISMATCH seed=17010672633609114990 (#0):
    wide-rung-ragged vs wide-rung-ragged-overread
    first diff at byte 19: 120 vs 85
EXIT=1
```

Width-precise, through the shipped cdylibs, and the run aborts. The bug was then reverted, the thirty
tests re-run green, and **every section in this file was re-run with validation live**: sixteen
`Validation OK` lines, zero mismatches. The regenerated cache-resident numbers agree with the ones
reported above to within 3.8 percent, inside that size's control gap, so nothing in sections two or four
moves.

### What this costs the existing directory

Not one of the 147 committed runs was validated, because the call did not exist. That does not make any of
them wrong, and section 2.1's six void cells are the demonstration that it matters: those arms agreed with
each other while computing an input-independent constant, and a validation pass would not have caught
that either, since agreement is exactly what it checks. What it does catch is an arm that computes
something **different**, which is the other half, and which nothing in this repository has ever checked
except by unit test.

The upstream fix worth making, and it is not mine to make inside a panel dispatch because
`mockspace-bench-harness` is a dependency of every consumer repo: re-export `run_worker_validate` from
the crate root, and have `run_orchestrator` call `validate` itself rather than leaving it to each
consumer to discover. `20` declined the equivalent change for the same reason at its section 2.2.

## Eight: the final validated numbers

Every section was re-run after the fidelity fix. Twenty-eight size rows, twenty-eight `Validation OK`
lines, zero mismatches, on a tree whose only uncommitted files are the three foreign `warm-clamp-arity-l2`
artifacts `20` flagged in its addendum, restored untouched.

At the ratified numeral, in nanoseconds per element so the two element counts compare directly:

| section | ragged | wordround | wr/rag | control gap |
|---|---|---|---|---|
| width-l1, 2048 el, 3 ops | 2.8819 | 2.8287 | 0.982 | 1.36% |
| width-l2, 458752 el, 3 ops | 2.9473 | 2.8949 | 0.982 | 0.20% |
| walk-l1, 2048 el, 1 op | 1.0506 | 1.0099 | 0.961 | 4.06% |
| walk-l2, 458752 el, 1 op | 1.0755 | 1.0688 | 0.994 | 0.34% |

At `W = 129`, the one width where the shapes differ enough to produce a large effect:

| section | ragged | wordround | wr/rag | control gap |
|---|---|---|---|---|
| walk-l1, in L1 (34 KB against 49 KB) | 1.0049 | 1.3598 | **1.353** | 6.60% |
| walk-l2, past L2 (7.8 MB against 11.0 MB) | 0.9867 | 1.3353 | **1.353** | 0.34% |

The ratio is the same to three decimal places across a working set that grew by more than two hundred
times and crossed both cache levels. Whatever produces the 1.353 is not the bytes.

## Nine: what this settles, and what it does not

Strictly. A bench answers the question it was pointed at and no other.

**Settled, on this host, for this workload.**

The ratified figure "three instructions per operation" does not describe the emitted code. It is zero at
five of six swept widths and at four of nine in the tail residue class; where it is non-zero it is
`3 * max(0, popcount(rag_bytes(W) mod 8) - 1)`, exactly; and it is per **element**, flat across operation
counts from one to eight, not per operation. Written with an over-reading load it is zero everywhere.

The ratified figure "seven bytes per value" is correct as arithmetic and is **not a throughput quantity**
here. Across four sections, six widths, two element counts spanning 34 KB to 15 MB, and two operation
densities, the byte-count advantage never converts into speed. The one large effect that exists, 1.353 at
`W = 129`, is identical in cache and out of it, which is what excludes footprint as its cause.

The strategy axis therefore is not carrying the trade the rule says it carries, on this evidence. It is
carrying a footprint difference that is real for a consumer counting bytes and invisible to a consumer
counting time.

**Not settled, and not touchable by this bench.**

Whether the rule should change. This measures one host, one target, one workload family, and a workload
family I chose. `arvo-toolbox-not-policer.md` is explicit that footprint at the million-element scale is
the reason `Cold` exists, and nothing here contradicts that: it says the bytes do not buy speed, not that
the bytes do not matter.

Any other target. `aarch64` on an M1 has cheap unaligned loads and `ldp`. A target with expensive
unaligned access, or without a paired load, could order these arms differently, and the over-reading arm
in particular leans on unaligned loads being free.

Signedness. Every arm is unsigned. `20` section 4 names this gap for the native rungs and it is equally
open here: sign turns the top-limb projection from a mask into a sign extension.

Multiplication and division at the wide rung. The step cycle is add, exclusive or, and multiply by a small
constant. A full multi-limb multiply is quadratic in the limb count and might weight the shapes
differently.

Whether a wide numeral should be walked this way at all. Every arm loads a whole numeral per element. A
consumer doing limb-parallel work across a column would have a different access pattern entirely.

## Ten: what I had to decide that the design does not specify

Named because the numbers turn on some of them.

**What an operation is at the wide rung.** Wrapping add, exclusive or, and multiply by three. The exclusive
or is not optional: without a non-affine step the chain composes to one multiply-add and
`bench-warm-container-shared` records discarding a whole sweep to exactly that. The choice of three for the
multiply is arbitrary and only needs to be odd, which is what makes the step invertible.

**That the semantics wrap.** Nothing here saturates, deliberately: `20` section 1.4 hands op the open
question of whether `Warm` wraps or clamps, and section 2.1 shows saturation is what produced the six void
cells. Wrapping is the semantics under which the answer provably cannot be absorbed. If the wide rung is
supposed to clamp, this bench measures the wrong semantics and would need the `warm-clamp` family's shape.
**The numbers turn on this.**

**How a ragged payload is loaded.** Three ways exist and I built two. The numbers turn on this more than on
anything else: the safe form and the over-reading form differ by up to six instructions per element and the
over-reading form is level with word-rounded everywhere.

**The element counts.** 2048 and 458752, picked so the shapes straddle a 128 KB L1 and a 12 MiB L2 at the
ratified numeral. Section six shows the choice did not matter, which is itself the result.

**The widths.** Six, with two chosen because the shapes collide there and give free controls. Not a sample
of a law: where a law was claimed, in the tail residue class, the whole class of eight residues was swept.

**That the arms read separate regions.** A single region read at three pitches would measure three walks
over one working set and the footprint half would vanish by construction.

## Eleven: what is op's, and what is not

**Op's, and it is the same sentence `20` already handed him.** Does the wide rung wrap or clamp? Section
ten says this bench's numbers turn on it, and `20` section 5 says two committed bench families implement
the two readings of `Warm` and disagree in direction. One answer decides whether this file measures the
wide rung or measures half of it.

**Not op's, and not to be brought to him.** Whether the ragged-against-word-rounded rule stands. The bench
exists now, and by his own standing position at `01:96-98` a contested magnitude is answered by someone
being confident enough to build the arm rather than by a ruling. The arm is built. If the answer is still
contested, the thing is still settling, and the next move is the signed arm and a second target, not an
escalation.

**Not op's, and mine to state plainly.** The ratified provenance of item 14 is an instruction count at one
numeral, and it does not survive the harness. Under `RULES.md`'s own ladder that row is RATIFIED and
governs until op moves it; what this file establishes is that the evidence recorded beneath it is wrong in
both halves, which is a fact about the evidence and not a licence to change the row. The row's status is
op's; the state of its evidence is measurement, and this is the measurement.

**Not op's, and it belongs upstream.** `mockspace-bench-harness` never calls its own validation pass and
does not re-export the worker that would let a consumer call it. That is a defect in a crate every
consumer repo depends on, and per `20` section 2.2's reasoning it is not a panel dispatch's change to
make.

## Twelve: coverage, and what I did not do

**Established on the harness, with committed artifacts:** every number in sections two, five, six and
eight. Twenty-eight size rows, 11,200 rows of CSV, five sections, five arms, all validated.

**Established with an ad-hoc quick spike, which is all it can support:** every instruction count in
section four. `22_probes/loop-shape/` is named as a spike in its own manifest and findings, has no timer,
and prices nothing.

**Established with counts, each from a command in this file:** the six declared widths in `bench.toml`
being `[8, 13, 16, 32, 60, 64]`; 147 findings files and 55,280 rows with digest zero, reproducing `20`
exactly; 11,200 new rows also with digest zero; one of ten meta files clean and the rest `-dirty`.

**Attacked and closed:** the issue-bound mechanism, by building the bare walk; the run-order confound, by
re-running the section; the footprint reading of the `W = 129` win, by the L1 discriminator; the harness's
missing fidelity check, by fixing the driver and demonstrating it refusing an injected bug.

**Attacked and not closed:** the width-dependent baseline, 2.1x between `W = 129` and `W = 192` at three
limbs. Four mechanisms ruled out with evidence, none found. It is a lead, not a result.

**Not covered.** No signed arm. No multiply or divide at the wide rung. No target other than aarch64 on an
M1. No saturating semantics. No width above 256 bits and none between 65 and 128, so the crossover out of
`u128` is untouched. I did not re-run or re-read any pre-existing bench family beyond the counts above, and
I did not verify `20`'s sections 1 through 3 other than the two counts I reproduced and the section 2.3
claim I got wrong and corrected in section seven. The three foreign `warm-clamp-arity-l2` artifacts remain
uncommitted exactly as `20` left them; I parked them for the clean-tree run and restored them, and I did
not commit them, for the reason `20` gave.
