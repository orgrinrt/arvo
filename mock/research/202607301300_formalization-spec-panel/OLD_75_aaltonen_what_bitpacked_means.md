Sebastian Aaltonen, file 75. I wrote files 32 and 57. File 32 modelled the byte-aligned reading as
the identity contract's own bitpacked-column shape and measured it against a hand-rolled shift-and-mask
baseline; that model is one of the two readings this dispatch has to choose between, and I treat it as
evidence to test, not as a settled answer, exactly as the brief instructs.

**What I read.** `68_consolidation_seven.md` in full. `73_arntzen_the_byte_image.md` in full, the file
that found the ambiguity. `74b_op_checkpoint_eighteen.md` in full, the checkpoint that sets this
dispatch. `74_lattner_the_taxonomy_rechecked.md`, grepped for the "option 1/2/3" framing op's checkpoint
references; it is not there in that shape, so the framing in `74b` is op's own compression of a
conversation this corpus does not carry verbatim, and I do not treat "option 3" as a citable design
artifact, only op's own words in `74b` are. `70_wronski_the_presets_re_derived.md` in full, load-bearing:
its fixed-point preset table is what actually answers this dispatch's question, and I read it closely
rather than trusting file 73's own citation of two lines from it. `70b_op_checkpoint_seventeen.md` in
full, confirming which parts of file 70 are ratified rather than proposed. `59_fog_the_lowering_door.md`
skimmed for the one line the brief calls void (`Lowering`'s member list; superseded by `70:217`, itself
matching `68:569-575` on `Encoding`/`StoredWidth`/`Layout`/`Door`). My own files 32 and 57, reread in
full rather than from memory. `ls` of the panel directory before starting: files `00` through `74b` plus
checkpoints and probe directories, nothing after `74b` except what I created in this dispatch.

**The method constraint, applied.** Nothing below cites shipped source as a statement of what the design
means. The one place I touch the shipped tree is the canon gate (a `grep` for absence) and the pinned
toolchain check, both evidence-not-meaning uses the brief itself licenses. Every artifact this file
relies on is freshly built: a single-file disassembly probe (`75_probes/codegen.rs`) and a real bench
under the harness (`mock/benches/variants/bitpack-*`), not a probe with a timer.

**Gates.** Canon gate, reproduced fresh from the repo root: `grep -rln "Adjustment\|Bias\|Numeral"
mock/crates/ --include="*.rs"` and the same with `FullRange\|UTerm\|AddWidth`, both exit 1, empty.
Test gate: `cargo test --offline --workspace` from `mock/` reports **661 passed, 0 failed, 9 ignored**,
the standing 658 plus the 3 new tests `bench-bitpack-shared` adds (`Column<256>`/`Column<4096>`/
`Column<16384>` round-trip and permutation-bijection checks); no shipped crate touched. Toolchain,
confirmed two ways in this session: `rustc --version` from inside the repo tree resolves to
`1.98.0-nightly (57d06900f 2026-05-27)`, `aarch64-apple-darwin`, matching `rust-toolchain.toml`; the
identical command from `/tmp`, outside the repo tree, resolves to `1.94.0 (4a4ef493e 2026-03-02)`
**stable**, confirming the dispatch's own warning. Host: Apple M1, `aarch64-apple-darwin`,
`Darwin 25.5.0`.

**What is compiled, what is measured, what is reasoned.** The instruction-shape claims in section 2 are
compiled: a single-file probe, disassembled, cited by symbol and address. The throughput numbers in
section 3 are measured: a real bench in `mock/benches/`, built as cdylib variants under
`mockspace-bench-core`/`mockspace-bench-macro`, run through the orchestrator, CSVs committed alongside
this file's own citations. Section 1 (what the axis already says) and section 4 (what to build) are
reasoning built on the compiled and measured facts, marked as such throughout.

## 0. The verdict, stated first

**Replace the working "two instances" reading, not confirm it.** `Layout::Bitpacked` needs exactly one
meaning, the zero-inter-value-padding reading, and the axis stays the two-member sealed set it already
is (`Dense`, `Bitpacked`). The byte-aligned-slot reading is not a second instance of `Bitpacked` at all;
it is what `Layout::Dense` already does at a narrow `StoredWidth`, a case the design's own ratified
preset table already assigns to `Hot` (`70:137-143`, ratified in full at `70b`). The two readings this
dispatch was asked to choose between are not two strategies and not one strategy described twice; they
are two different `Layout` members that the corpus had already named separately (`Dense`, `Bitpacked`)
before this ambiguity was noticed, and the ambiguity was never about how many instances `Bitpacked`
needs. It is about whether `Bitpacked` denotes the same mechanism as `Dense`-at-a-narrow-width (in which
case it is redundant and should not exist as a separate member) or a genuinely different mechanism (in
which case it earns its keep). It is the second. Measured: at logical width 13, on this target, real
`Layout::Dense` (a native `[u16; _]` array) costs 5 instructions per element in a sequential column-sum
and 9 under random access; the zero-padding reading costs 24 and 27 respectively, roughly **4.6x to
5.5x slower sequentially and 2.2x slower under random access**. That is a real, distinct, non-trivial
mechanism, not a restatement of Dense wearing a different name.

Op's own second sentence in `74b` is the one this settles: "the description on option 1 is a bit
ambiguous so it might already achieve this implicitly from unnamed context." It does. The unnamed
context is `StoredWidth`'s own existing generality (already a member of `Lowering`, `68:569-575`,
`70:217`, not a new axis), combined with `Layout::Dense`'s own definition, which file 73 already stated
correctly but did not draw the full consequence from: "`materialise` [is] a pure relabelling" for "every
`Layout::Dense` numeral, at any `StoredWidth`" (`73:74-76`, quoted verbatim). A `StoredWidth` of 13 bits
under `Dense` is not a hypothetical; it is exactly what `Hot`'s own ratified row already names
(`minimum` `StoredWidth`, `dense` `Layout`, `70:141-142`, ratified `70b:6-8`), and its per-value
materialised footprint rounding up to the next byte is a structural consequence of what "each value owns
its own carrier" means in a byte-addressed machine, not a separate design choice needing its own name.

The working "two instances" resolution and this file's resolution converge on the outcome op actually
asked for (the cost and complexity of the packed mechanism lands on `Cold` alone), but by a cheaper
route: one new mechanism, not two, because one of the two candidate mechanisms was never new.

## 1. What the ratified preset table already says, read closely

File 73 raised the ambiguity by finding that "bitpacked storage" had been carrying two meanings across
the whole corpus, "unnoticed for seventy-two files" (`74b:51-54`, restating `73`'s own framing). What
none of the files that noticed this reopened is the table that already answers it, because it settled
one stretch earlier and nobody was asking this question when it did.

```
| | Hot | Cold | Warm | Precise |
|---|---|---|---|---|
| StoredWidth | minimum | minimum | doubled | doubled |
| Layout | dense | bitpacked | dense | dense |
```

(`70:137-142`, the fixed-point preset table; ratified in full at `70b:6-8`, "Fixed-point, D71's
construction with the two dead rows dropped: `Hot` truncates toward negative infinity, reduce-modulo
out of range, minimum width, dense. `Cold` and `Warm` round nearest ties-to-even and clamp, differing in
stored width and layout.")

Three things this table already settles, none of them a new finding of mine, all three load-bearing for
the answer:

**There is exactly one row using `Layout::Bitpacked` in the whole table.** `Hot`, `Warm`, and `Precise`
are all `dense`. Only `Cold` is `bitpacked`. If the byte-aligned-slot reading were a genuine second
`Bitpacked` instance, it would need a home in this table somewhere, and there is no second `bitpacked`
cell for it to occupy; `Hot`'s own "minimum `StoredWidth`, dense `Layout`" row is exactly where a
byte-aligned, minimal-native-width, cheap-access mechanism belongs, and it is already spoken for by
`Dense`.

**`Hot` and `Cold` share the word "minimum" for `StoredWidth` while differing only in `Layout`.** That
is the tell that "minimum" names the carrier's *logical* width (the exact bit count a numeral needs,
13 here, identical for both rows), not a rounded, native-register width that would differ between a
speed-first preset and a storage-first one. What differs between `Hot` and `Cold` is not how many bits
the carrier logically holds; it is what `Layout` does with those bits when materialising a column of
them: `Dense` gives each carrier its own independent, necessarily byte-rounded, byte image (file 73's
own "pure relabelling", `73:74-76`); `Bitpacked` does not, because giving every carrier an independent
byte image is exactly the thing zero-inter-value-padding refuses to do.

**Cold's own stated intent uses a word that forecloses the byte-aligned reading for `Bitpacked`
specifically.** "`Cold`'s minimum `StoredWidth` and bitpacked `Layout` are 'stores as small as possible',
**literally**" (`70:174-175`, quoted verbatim, the emphasis in the source). Byte-aligned-slot packing is
not literally as small as possible; it wastes up to 7 of every 8 stored bits whenever the logical width
is not a multiple of eight (13 bits rounds to 16, a 23% overhead at this width and far worse at
narrower ones, 3 bits rounding to 8 is a 167% overhead). "Literally" is the word that rules the
byte-aligned reading out of `Bitpacked`'s own definition, not merely out of favour.

File 73 already found this citation and already leaned on it: "given `Cold`'s own re-derived intent this
stretch, 'stores as small as possible' literally (`70:174-178`), the dense reading is the one that
reading points toward, but I state it as a lean and not a ruling" (`73:225-228`, "the dense reading"
there is file 73's own name for the zero-padding reading, matching this file's vocabulary). File 73
reached the right pointer and correctly declined to rule from a citation alone, since ruling on a
type-level design question from a design-round member is not this review's shape; what file 73 did not
have was the compute-side evidence to say what the two candidates actually cost each other, which is
what this dispatch supplies, and the missing half of the ruling: that the *other* reading was never a
competing `Bitpacked` candidate in the first place, only ever `Dense`.

*Grounded on: ratified (`70:137-142`, `70b:6-8`), settled shapes (`73:74-76`, `73:225-228`, `68:569-575`,
`70:217`), reasoned (the "minimum means logical width, not native width" reading; the consequence for
which mechanism `Bitpacked` denotes).*

## 2. The instruction shape, compiled

`75_probes/codegen.rs`, one standalone file, `#[unsafe(no_mangle)]` on every probed symbol so it
survives disassembly, `#[inline(always)]` on the two extraction transforms (matching the real bench
source) so both a standalone-call view and an inlined-into-a-loop view exist from the same body.
Compiled `-C opt-level=3 -C lto=fat -C codegen-units=1 -C panic=abort`, disassembled with `objdump -d`.
Full reproduction, every symbol's disassembly, and the `nm` check confirming `objdump`'s `<ltmp0>`
mislabel is a real exported symbol (`_extract_aligned_standalone`, address 0) are in
`75_probes/OUTCOMES.md`.

**Address computation.** The byte-aligned reading's address is `i * 2`, one shift (`lsl x0,x2,#1`). The
zero-padding reading's address is `(i * 13) >> 3`, and whether that costs a real multiply depends on the
access pattern, not on the reading in isolation: in a sequential loop LLVM strength-reduces it into a
running accumulator (`add x13,x13,#0xd` each iteration, `75_probes/codegen.objdump.txt:sum_zeropad`), so
the multiply this dispatch's own framing predicted for "the zero-padding address" does not appear there.
Under a data-dependent (random) index the strength reduction cannot fire and the multiply reappears as a
real instruction, `umull x15,w9,w14` (`sum_zeropad_rand`). This is the one instruction class whose cost
is access-pattern-dependent rather than reading-dependent, and it is exactly why the random-access ratio
below is worse than the sequential one for the zero-padding reading specifically.

**Load count.** Neither reading compiles to the single wide load I assumed going in. The byte-aligned
reading (`extract_aligned_standalone`) does two separate byte loads plus a bit-field-insert (`bfi`) that
folds the mask into the combine, because Rust's bounds-checked slice indexing on `buf[off]`, `buf[off+1]`
did not fuse into one 16-bit unaligned load on this compiler. The zero-padding reading
(`extract_zeropad_standalone`) does three byte loads, not the four my own source asked for: LLVM proved
the fourth byte the source reads is never used, given the mask and the worst-case bit shift, and dropped
that load entirely. Both are honest findings against my own stated expectation in the source comments,
and I report them as such rather than silently rewriting the comments to match.

**Bounds checks.** Both byte-buffer readings carry one bounds-check branch pair per byte touched (2 for
aligned, 3 to 4 for zeropad depending on entry point), predicted not-taken and cheap per-instance but not
free, and this is a real cost of the probe's own byte-slice-indexed model, not a fact about either
`Layout` mechanism in the abstract. A genuinely native `[u16; _]` array (the `extract_native`/`sum_native`
probes added once this became relevant, section 3) gets exactly one bounds check, hoisted entirely
outside a sequential loop by LLVM's own bounds-check-elimination pass, because a plain slice indexed by
a linear counter is precisely the shape that pass targets. The byte-buffer model cannot get this
hoisting because each byte access is bounds-checked independently; this is the reason section 3's native
baseline is dramatically cheaper than the byte-aligned model, and it is a fact about how I built the
probe to keep both packed readings on comparable byte-addressed infrastructure, not a fact that
`Layout::Dense` itself is expensive.

| function shape | aligned (byte-buffer) | zeropad | native `[u16;_]` |
|---|---:|---:|---:|
| single extraction, isolated | 4 real instructions + 2 bounds-check pairs | 10 real instructions + 4 bounds-check pairs | 2 real instructions + 1 bounds check |
| sequential loop, per element | 11 instructions | 24 instructions | 5 instructions |
| random-access loop, per element | 17 instructions | 27 instructions | 9 instructions |

*Grounded on: compiled (`75_probes/codegen.rs`, `75_probes/codegen.objdump.txt`, `75_probes/OUTCOMES.md`),
correctness checked (`75_probes/codegen.rs`'s own `#[test] sums_match_reference`, cross-checking every
extraction and loop entry point against a from-scratch reference decode, `1 passed`).*

## 3. The throughput, measured

Built under the harness, per `bench-and-sketch-discipline.md`: `mock/benches/variants/bitpack-shared/`
(the shared `Column<const N: usize>` `Routine`, both pack/extract transforms, the permutation builder)
plus five cdylib variants (`bitpack-native-seq`, `bitpack-native-rand`, `bitpack-aligned-seq`,
`bitpack-aligned-rand`, `bitpack-zeropad-seq`, `bitpack-zeropad-rand`, six total). Two bench sections,
`bitpack-sequential-sum` and `bitpack-random-sum`, three sizes each (256, 4096, 16384 elements), all
three variants per section run together so the harness's own findings report compares them directly.
Correctness: `cargo test --offline -p bench-bitpack-shared --release`, `3 passed, 0 failed`, cross-checks
both extraction paths against the logical ground truth at every index, every size, 8 seeds, and confirms
the permutation is a genuine bijection on `0..N`. `validate_output` re-runs the identical check on every
harness call. CSVs, meta, and findings committed alongside this file:
`mock/benches/bitpack-sequential-sum_n{256,4096,16384}.csv` and `bitpack-random-sum_n{256,4096,16384}.csv`.

Field width 13 throughout, the non-power-of-two shape file 32 and file 73's `probe_3` both used
(`32:207-230`, `73` section 5). Medians below are the harness's own `algo_ns` column (function-under-test
time, bridge overhead subtracted), warm mode, 40 samples per variant per size.

**Sequential column sum, ns per element, ratio against native `Layout::Dense`:**

| N | native | aligned (byte-buffer) | zeropad | aligned/native | zeropad/native |
|---:|---:|---:|---:|---:|---:|
| 256 | 0.086 | 0.353 | 0.473 | 4.13x | 5.53x |
| 4096 | 0.107 | 0.370 | 0.512 | 3.45x | 4.79x |
| 16384 | 0.111 | 0.371 | 0.509 | 3.35x | 4.60x |

**Random-access column sum, ns per element, ratio against native `Layout::Dense`:**

| N | native | aligned (byte-buffer) | zeropad | aligned/native | zeropad/native |
|---:|---:|---:|---:|---:|---:|
| 256 | 0.669 | 0.869 | 1.445 | 1.30x | 2.16x |
| 4096 | 0.662 | 0.854 | 1.434 | 1.29x | 2.17x |
| 16384 | 0.661 | 0.853 | 1.446 | 1.29x | 2.19x |

**The comparison that answers the design question is zeropad against native, not zeropad against
aligned.** `aligned` (the byte-buffer model) is not a design mechanism; it exists in this bench only to
give the zero-padding reading a fair, byte-addressed opponent built from identical infrastructure. Once
section 1's reading of the preset table is accepted, the real fork is `Layout::Dense` (native, `Hot`'s
own row) against `Layout::Bitpacked` (zero-padding, `Cold`'s own row), and that fork costs **4.6x to
5.5x sequentially and roughly 2.2x under random access**, stable across three sizes spanning
cache-resident (256 elements, ~4KB) through past a typical 32KB L1 (16384 elements, ~32KB aligned /
~26KB zeropad). Stability across sizes that different is itself evidence this is a real, compute-bound
cost rather than a cache-bandwidth artefact of any one size; if it were bandwidth-bound, the smaller
zeropad footprint (13/16 of aligned's, and both far smaller than any of these sizes' worth of DRAM
bandwidth pressure at this scale) would show `zeropad` closing the gap at larger N, and it does not.

**Random access costs both readings relative to their own sequential number, and the zero-padding
reading pays more for it.** Native: 0.086 to 0.111 ns/op sequential rises to 0.661 to 0.669 random,
roughly 6x. Zeropad: 0.473 to 0.512 sequential rises to 1.43 to 1.45 random, roughly 2.9x. This is
consistent with section 2's address-computation finding: native's address is a shift that folds into the
load instruction's own addressing mode for free even under random indices (no extra instruction at all,
per `sum_native_rand`'s disassembly), so its random-access penalty is pure cache/prefetch loss with no
added compute; zeropad's address is a real multiply under random access, adding compute cost on top of
the same cache/prefetch loss, which shows up in the raw ns/op numbers but in the opposite direction from what the raw multiply
count alone would predict. The resolution: zeropad's own random-versus-sequential ratio (roughly 2.9x,
0.47 to 1.44 ns/op) is smaller than native's own (roughly 6x, 0.09 to 0.66 ns/op), so the
*relative-to-native* gap narrows under random access rather than widening; native has the larger
sequential-only advantage to lose. Sequential access is where native's advantage is largest, because
LLVM's hoisted bounds check and the byte-buffer model's per-byte checks both matter most there; under
random access, per-element overhead dominates for every reading and native's structural advantages
shrink to "no multiply, still one fewer bounds check", which is still real and still favours native,
just by a smaller multiple than the sequential case shows.

*Grounded on: measured (`mock/benches/bitpack-sequential-sum_n{256,4096,16384}.csv`,
`mock/benches/bitpack-random-sum_n{256,4096,16384}.csv`, harness `algo_ns`, warm mode, 40 samples/variant,
cross-checked against the harness's own auto-generated findings reports, which independently compute the
same ratios, e.g. `bitpack-sequential-sum_n256_findings.md`: "bitpack-zeropad-seq (121 ns) is 5.5x the
fastest (22 ns)", matching this section's own 5.53x), compiled (section 2, for the mechanism each number
traces to), reasoned (the bandwidth-vs-compute-bound argument from size-stability; the
sequential-vs-random asymmetry explanation).*

## 4. What this hands forward, in the consolidation's own form

**The design text.** *`Layout` stays the two-member sealed set `{Dense, Bitpacked}`; no third member,
no per-preset sub-variant. `Dense`'s `materialise` is a pure relabelling at any `StoredWidth`, including
a `StoredWidth` narrower than a native register, and its per-value byte image rounds up to the next byte
boundary as a structural consequence of giving every value an independent carrier, not as a separate
design choice. `Bitpacked` denotes zero-inter-value-padding exclusively: fields share bytes with their
neighbours, no per-value byte image exists (file 73's own finding, unchanged and confirmed rather than
revised), and `materialise` is a column-level obligation rather than a per-value crossing. The ratified
preset table already assigns `Dense` to `Hot`, `Warm`, and `Precise`, and `Bitpacked` to `Cold` alone
(`70:137-142`, `70b:6-8`); this file adds no new assignment, it closes the question of what the one
`Bitpacked` cell in that table means. Measured on this target at logical width 13: `Bitpacked` against
`Dense` costs roughly 4.6x to 5.5x per element under sequential column access and roughly 2.2x under
random access, both ratios stable across three sizes spanning cache-resident to past-L1. This is the
real magnitude of "seldom computed... can take more cost than warm" (`70:120-122`, file 70 quoting
op's own intent statement for `Cold` at `68b:69-73`) that the design was previously stating in prose
without a number attached.*
(Grounded: ratified `70:137-142`, `70b:6-8`; settled shapes `73:74-76`, `73:225-227`, `68:569-575`;
compiled `75_probes/`; measured `mock/benches/bitpack-*`.)

**What this does not decide, stated as owed.** The exact `StoredWidth` a `Dense` carrier settles on for
a given logical width (whether it always rounds to the next byte, or to the next native register width,
which differ for widths between 9 and 15 bits and again above 33) is a `Lowering`/container-dispatch
question this file did not need to resolve to answer the `Bitpacked`-vs-`Dense` question, and I flag it
rather than assume an answer: my own `extract_aligned` byte-buffer model rounded to the next byte (16
bits for 13), while arvo's own shipped container dispatch (a tree-fact cited only as evidence the
question is live, not as design meaning, per this dispatch's own method constraint) maps `N <= 128` to
whatever native primitive fits, which for 13 bits is also `u16`, so the two happen to agree at this
width; whether they agree at every width is unchecked. Whether a real `arvo-bitmask`/container-dispatch
`Dense` implementation, once built, reaches the near-free `sum_native` numbers this file measured or
something closer to the byte-buffer `aligned` numbers (the difference being entirely about whether the
carrier is a genuinely native-typed array or a hand-indexed byte buffer) is an implementation question
for whoever builds it, not a design question this file needed to settle; I flag it because the gap
between the two (roughly 3x to 4x sequentially) is large enough that a naive byte-buffer implementation
of `Dense` would silently give away most of the advantage this file is crediting it with. Column-level
capacity for a `Bitpacked` buffer (file 73's own owed item, `73:445-448`) remains owed; nothing here
builds it.

**A correction to my own file 32, stated plainly.** File 32 section 4 modelled "the datum-versus-value
split costs a packed column" using a byte-aligned, no-word-boundary-crossing packing (`Q(13,3)`, 16-bit
slots, four per 64-bit word) and found the `Encoding` trait indirection costs nothing over a hand-rolled
shift-and-mask baseline. That finding stands unchanged; what does not survive contact with this file's
work is treating that model as a measurement of `Layout::Bitpacked`. It was always a measurement of
`Layout::Dense` at a narrow `StoredWidth`, correctly built, mislabelled. File 73 noticed the same gap
from the opposite direction (`73` section 5, "file 32's own model has padding... every field's byte
image is independently addressable") without yet having the preset-table citation to say what that
implies for the axis; this file supplies that citation and the label correction together.

## 5. Standing

Nothing here overturns a ratified call. `70:137-142` and `70b:6-8` are treated as governing throughout,
per the provenance ladder this workspace's own rules state: a ratified table is defended, not weighed
against a candidate reading, and this file's job was to determine which candidate reading the ratified
table's single `Bitpacked` cell already picks out, not to relitigate whether it should. Op's working
"two instances" resolution in `74b` is superseded by this file's finding, not confirmed: the intent it
was reaching for (confine cost and complexity to `Cold`) is honoured, more cheaply, by recognising that
one of the two candidate readings was never a `Bitpacked` instance at all. Only op's calls are final, and
this file's own replacement of the working reading is exactly the kind of correction the review's
standing method expects a member to bring rather than to defer.
