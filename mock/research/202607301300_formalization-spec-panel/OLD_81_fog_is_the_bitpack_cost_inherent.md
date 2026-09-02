Agner Fog, file 81. I wrote files 08 (the union and what it costs), 50 (the float model) and 59 (the
lowering door), and I carry file 59's strategy-door table's voiding rather than defending it: that
table read shipped doc comments as design, op named it a regression at `68b`, and file 70 replaced it.
Nothing below leans on it.

**What I read.** `78_consolidation_eight.md` in full, the standing base. `75_aaltonen_what_bitpacked_
means.md` in full, twice: once as a document and once against its own committed artifacts.
`77b_op_checkpoint_nineteen.md` in full, which sets this dispatch in op's words. `79_dolan_what_
capacity_is.md` and `80_leroy_the_verification_bundle.md` skimmed for what has landed since, plus
`79b_op_the_verification_mandate.md`, which is recorded and not dispatched and which nothing here
designs against. One `ls` of the panel directory, current through `80_probes`. The shipped tree I
touched for exactly two things: the standing canon-gate greps, and reading
`benches/variants/bitpack-shared/src/lib.rs` and the six `libbench_bitpack_*.dylib` binaries file 75
built, both of which are `mock/benches` rather than `mock/crates` and are the artifacts this dispatch
exists to re-examine. No claim below rests on what any source comment says the design means.

**Gates.** Canon gate, fresh from the repo root: `grep -rln "Adjustment\|Bias\|Numeral" mock/crates/
--include="*.rs"` and the same with `FullRange\|UTerm\|AddWidth`, both exit 1, empty. Test gate:
`cargo test --offline --workspace` from `mock/`, summed per binary from every `test result:` line,
**666 passed, 0 failed, 9 ignored**: the standing 661 the consolidation carries, plus the five tests
the crate I added brings. I read the body of every test in the surface I touch (`mock/benches`, twelve
`#[test]` functions across four shared crates). None is tautological: the two quantiser crates check
against independent oracles (silicon `fadd`, and exact integer arithmetic on both decimal neighbours)
and assert the checked count so an empty loop cannot pass; file 75's three check every index at every
size against the logical ground truth and verify the permutation is a bijection. The one disqualifying
test already on record, `arvo-tensor/tests/capacity.rs:14-18`, is outside my perimeter and stands as
the consolidation carries it (`78:874-876`). Toolchain `rustc 1.98.0-nightly (57d06900f 2026-05-27)`,
`aarch64-apple-darwin`, confirmed inside the tree; outside it the same command gives stable `1.94.0`.

**What is compiled, what is measured, what is reasoned.** Every instruction count in sections 2 and 3
is read out of a binary with `objdump -d`, and where it is a bench binary it is the binary the harness
loaded, not a standalone stand-in. Every ns figure is from the bench harness, warm mode, 80 samples per
variant per size, CSV and findings committed alongside this file
(`mock/benches/bitpack-decoder-shape_*`, `mock/benches/bitpack-kernel-amortisation_*`). Sections 5 and
6 reason from those. Where I predicted something and the measurement refused it, the refusal is in the
text rather than the prediction.

**One side effect I caused and undid, stated because the record is the point.** The bench orchestrator
has no per-section filter: running it re-ran and overwrote every committed CSV, meta and findings file
in `mock/benches`, including file 75's. Before restoring them I compared: file 75's own sections
re-measure within +10% / -4% of their committed medians on this host, systematically in one direction
(thermal), and every ratio it reported reproduces. Its numbers replicate. I then restored every
artifact I did not author with `git checkout`, so only the two new sections are added to the record.
Anyone re-running this pays the same tax and should restore the same way.

## 0. The verdict, stated first

**The multiple is not inherent. Most of it is the loop, and the loop was written without the facts the
type already holds.**

Measured on this host at logical width 13, against the same dense native baseline, over four column
sizes from 16384 to 262144 elements:

| what the consumer does | file 75's decoder | plan-driven decode | byte-gather decode |
|---|---:|---:|---:|
| sum the column (decode is the whole cost) | **4.55x to 4.72x** | **1.49x to 1.51x** | 1.88x |
| sum a per-element function of it | **2.42x to 2.48x** | 2.04x to 2.05x | **1.29x** |

Nothing about the packing changed between those columns. The buffer is the same buffer, byte for byte,
built by the same packer, checked against the same ground truth at every index. What changed is where
the decode's own parameters live: file 75's decoder computes the byte offset and the bit shift from the
running index at runtime, and the plan-driven decoders read them from associated consts, because
**every one of them is a function of the logical width alone and the logical width is a type
parameter**.

So the answer to op's question is that the 4.6x is an artifact of the access pattern *and* of the
decoder, and the decoder is the larger half. The residue that survives is between 1.29x and 1.50x
depending on what the consumer does with the value, and that residue is a real price this design should
record. There is also one genuinely structural consequence, and it is not a cost: a bitpacked column
has no per-value byte image, so it has no independently writable element, and its partition granule is
therefore the same period the decode plan keys on (section 4.4).

The fourth design rule is what makes this a finding rather than a micro-optimisation. Every quantity
the naive decoder computes at runtime has a compile-time alternative that exists under the permitted
feature set: the period, the group stride, the window offsets, the per-lane shifts, the mask, the load
width, the read headroom, and the refusal for widths a window cannot hold. Per `78:163-166`, a runtime
cost with a const-time alternative that exists is the violation, not the constraint.

## 1. What the ratified measurement isolated, and what it did not

`78:552-567` is ratified and I am not reopening what it settled. `Layout::Bitpacked` has one meaning,
zero inter-value padding; the byte-aligned reading is `Layout::Dense` at a narrow `StoredWidth`; file
32's measurement was relabelled. All of that stands and this file depends on it. What op sent back for
a second look is the number, and the number has three problems that are about method rather than
about arithmetic.

### 1.1 The sweep never left the cache it claims to have left

`78:561-562` and `75:227-229` both say the three sizes span "cache-resident to past-L1", naming a
"typical 32KB L1" and 16384 elements at ~32KB as the far end. The host is stated at `75:34-35` as an
Apple M1. Read on this machine this session:

```
hw.perflevel0.l1dcachesize: 131072
hw.perflevel0.l2cachesize: 12582912
```

The performance cores have a **128 KB** L1 data cache. Every size in that sweep is L1-resident, by a
factor of four at the far end, for both layouts. The measurement is a pure compute measurement across
its whole range, which is fine, but it cannot support the inference built on it: `75:229-232` reads the
stability across sizes as evidence that the cost is "a real, compute-bound cost rather than a
cache-bandwidth artefact", and reasons that if it were bandwidth-bound the packed form's smaller
footprint would close the gap at larger N. Everything in that range is in L1, so the stability is
evidence of nothing except that the sizes were too close together to distinguish the two hypotheses.

I extended the sweep past the boundary, which is the cheap version of the experiment that would have
settled it. Dense per-element cost, four sizes, 16384 through 262144 elements (32 KB through 512 KB,
crossing L1 at 65536 and sitting in L2 above it): **0.1105, 0.1112, 0.1113, 0.1114 ns**. Flat to four
significant figures across a factor of sixteen in footprint and a cache level. At 0.111 ns per two-byte
element the dense stream is moving about 18 GB/s, which is nowhere near this machine's memory
bandwidth, so nothing in this shape is bandwidth-bound at any size the harness can hold. The packed
form's 18.75% smaller footprint therefore cannot show up here, at any N, and the correct statement is
that **this bench shape is structurally incapable of pricing the thing `Cold` exists to buy.** That is
not a criticism of a number, it is a statement about which question the artifact answers.

### 1.2 The instruction table describes a different program from the benched one

`75:178-182` gives per-element instruction counts under the heading "The instruction shape, compiled",
and `75:236-249` uses them to explain the bench's timings. They come from `75_probes/codegen.rs`, a
standalone file. Counted instead from the binaries the harness actually loaded, taking each
`_bench_entry`'s innermost backward-branching loop and dividing by the elements it retires:

| file 75 variant | loop body | elements per iteration | measured per element | `75:180-182` |
|---|---:|---:|---:|---:|
| `native_seq` | 33 | 32 | **1.03** | 5 |
| `aligned_seq` | 59 | 16 | **3.69** | 11 |
| `zeropad_seq` | 30 | 4 | **7.50** | 24 |
| `native_rand` | 9 | 1 | **9.00** | 9 |
| `zeropad_rand` | 22 | 1 | **22.00** | 27 |

The random-access rows agree closely. The sequential rows are wrong by five times and three times
respectively, and the reason is visible in the disassembly: the real dense sequential loop is fully
vectorised (`ldp q4, q5` / `bic.8h` / `ushll` / `uaddw`, 32 `u16` per iteration,
`libbench_bitpack_native_seq.dylib` at `0x8d8`), and the standalone probe's is not, because the probe
indexed a byte buffer through bounds-checked slice arithmetic. A scalar bounds-checked probe resembles
the real random-access loop and does not resemble the real sequential one.

**What made this invisible is worth more than the correction.** The probe's own ratio, 24 over 5, is
4.8. The bench's ratio is 4.6. Two absolute numbers wrong by 3x and 5x in the same direction produced a
ratio that matched the thing it was offered as an explanation of, and the agreement between the two
read as corroboration. It is the same failure as agreement between two unratified documents: the
concordance was structural, not evidential.

Also, in passing and without consequence for any ratio: the per-element figures in `75:210-220` do not
reproduce from file 75's own committed CSVs under the statistic it names. It says "medians ... of the
harness's own `algo_ns` column"; the medians of that column in `bitpack-sequential-sum_n16384.csv` give
0.1018 and 0.4700 ns where the table says 0.111 and 0.509. Consistent 8% offset, ratio preserved
(4.62 against 4.60), so nothing downstream moves. It looks like the findings report's statistic rather
than the CSV's median.

### 1.3 The decoder under measurement is silently wrong above 25 bits

`benches/variants/bitpack-shared/src/lib.rs:168-181` reads a fixed 32-bit window at every width. A
field may begin at any of eight bit offsets, so a 32-bit window covers a field only while `W + 7 <= 32`.
The source comment states the consequence correctly for its own choice of load width and treats it as a
property of the design ("a design that let the field width grow past 19 bits ... would need the
two-load-and-OR shape"). It is a property of the choice. At `W = 27` my first probe run produced a
wrong sum with no diagnostic:

```
assertion `left == right` failed: naive W=27
  left: 55892321781
 right: 67703481845
```

`81_probes/decoders.rs` now carries this as an assertion rather than a description: the hardwired-32
form is required to agree with the reference **exactly when** `W + 7 <= 32`, at all eight probed
widths. The width the window must have is a const function of the field width
(`load_bytes` in the same file: 1, 2, 4, 8 or 16 bytes as `W + 7` fits 8, 16, 32, 64 or 128 bits), and
choosing it that way removes the second load entirely below 57 bits.

*Grounded on: measured (`sysctl` this session; `mock/benches/bitpack-decoder-shape_*` CSVs), compiled
(`objdump` of the six `libbench_bitpack_*.dylib` binaries and of `81_probes/decoders.rs`), settled
shapes (`78:552-567`, ratified, unchanged by any of this).*

## 2. The per-element sequence, and which parts the packing forces

File 75's sequential loop, from `libbench_bitpack_zeropad_seq.dylib`, retires four elements per
iteration in thirty instructions. Per element, in order:

1. `sub` then `lsr x, #3`: the byte offset, from the running bit offset.
2. `ldr w`: the 32-bit window.
3. `and w, #imm`: the bit shift, masked out of the running bit offset.
4. `lsr w, w, w`: the extraction, **register-operand shift**.
5. `and w, #0x1fff`: the field mask.
6. `add x`: the accumulation.

Plus one `add x15, x15, #0x34` and one branch per group of four. What the packing forces out of that
list is item 5, the mask, which is already a literal, and one load. What it does not force is items 1,
3 and 4: the byte offset and the shift amount are not data. They depend on the element's position
modulo the packing's period, and the period is `8 / gcd(W, 8)`, a function of the width alone.

At `W = 13` the period is 8 elements in exactly 13 bytes. **LLVM unrolled the loop by four.** Four is
not the period, so after four elements the bit offset has advanced by 52, which is 4 modulo 8, and the
shift pattern does not repeat. So the shifts stayed in registers and the offsets stayed live. One more
factor of two on the unroll and every one of them would have folded, and the compiler had no way to
know that, because nothing in the program said 8.

The quantities that are functions of `W`, in full, with what each decides:

| quantity | value | decides |
|---|---|---|
| period `P` | `8 / gcd(W, 8)` | the unroll factor; also the partition granule (4.4) |
| group bytes `G` | `W * P / 8`, always whole | the pointer stride |
| window offsets | greedy byte offsets covering every lane | how many loads per group |
| lane shifts | `(j * W) mod 8` relative to its window | the `ubfx` / `ushr` immediate |
| mask | `(1 << W) - 1` | already a literal in every shape |
| load width | narrowest of 1, 2, 4, 8, 16 bytes with `W + 7` bits | whether a second load exists |
| read headroom | the load width, past the last group | whether the read needs a bound check |
| well-formedness | `W + 7 <= 8 * load width` | whether this plan exists at all |

`81_probes/decoders3.rs` computes all of them in const position and prints them; they are checked
against the packer at eight widths, and the period and group laws are asserted over **every width from
1 to 57** in the bench crate's own test rather than at the one width the bench measures.

### 2.1 The failure that cost me an hour, recorded because the design depends on it

My first attempt wrote those quantities as `const fn`s and called them in ordinary value position,
which is what a reader would naturally write. rustc guarantees const evaluation only in a const
position, so this was left to LLVM, and LLVM did not fold the recursive `gcd`. The emitted function
opens with a runtime division loop:

```
    35a8: 1ac90948     	udiv	w8, w10, w9
    35ac: 1b09a908     	msub	w8, w8, w9, w10
    35b0: 35ffff88     	cbnz	w8, 0x35a0 <_period_w13+0x8>
```

and carries live guards against the runtime period through the group body. Half the shifts folded
anyway through jump threading, so the disassembly contains `ubfx x4, x4, #5, #13` and looks like the
constant-folded version until counted. Moving the identical arithmetic onto a trait as associated
consts, changing nothing else, produced a loop unrolled by a literal 8 advancing by a literal 13 bytes
with every extraction a two-literal `ubfx`.

The design consequence is a sentence, and it is stronger than a style preference: **a fact the fourth
rule requires to be settled at compile time has to be written in a const position to be settled there.**
An associated const on the layout type is; a `const fn` called from the decode is not. This is the same
binding-time distinction file 76 landed on for the capacity split (`78:754-762`), arrived at
independently from the other side.

*Grounded on: compiled (`81_probes/OUTCOMES.md` findings C1, C2, N1, with disassembly), reasoned (the
attribution of each instruction to its cause).*

## 3. What moved when the plan moved into the type

Three decoders over one buffer. `naive` is file 75's shape. `windowed` reads through window offsets and
lane shifts held as associated consts. `simd` gathers each field's bytes straight into a natural-width
lane with `tbl`, so values arrive as `u16` rather than as the low bits of a 64-bit window; its gather
indices and per-lane shift vector are the same consts, laid out for `USHL`.

Per-element instruction counts, from the bench binaries:

| decoder | sum | with a per-element kernel |
|---|---:|---:|
| dense native | 1.00 | 2.00 |
| `naive` | 7.50 | (not counted) |
| `windowed` | 1.86 | 4.11 |
| `windowed`, narrowed to `u16` lanes | | 4.16 |
| `simd` byte gather | 2.00 | 3.00 |

Runtime, `mock/benches/bitpack-decoder-shape_*`, median `algo_ns` over 80 warm samples per variant per
size, per element:

| N | footprint dense / packed | native | naive | windowed | simd |
|---:|---|---:|---:|---:|---:|
| 16384 | 32 KB / 26.6 KB | 0.1105 | 0.5216 (4.72x) | 0.1672 (1.51x) | 0.2079 (1.88x) |
| 65536 | 128 KB / 106.5 KB | 0.1112 | 0.5075 (4.56x) | 0.1658 (1.49x) | 0.2090 (1.88x) |
| 98304 | 192 KB / 156 KB | 0.1113 | 0.5210 (4.68x) | 0.1669 (1.50x) | 0.2091 (1.88x) |
| 262144 | 512 KB / 416 KB | 0.1114 | 0.5065 (4.55x) | 0.1669 (1.50x) | 0.2095 (1.88x) |

`naive` independently reproduces the ratified 4.6x, at four sizes it was never measured at, in a
separate crate with a separately written packer. The measurement was sound. `windowed` is the same
buffer at **1.50x**, flat.

Two things in that table are worth saying out loud. The windowed decode **vectorised without being
asked**: once the two window loads per group were stated rather than left as eight overlapping ones,
LLVM produced `ld1.d`, `ushr.2d` by literal amounts, `and.16b` and `add.2d`, 119 instructions per 64
elements. And the hand-written NEON gather is *slower* than the scalar plan for this particular
consumer operation, 1.88x against 1.50x, because a plain sum wants values in wide accumulator lanes and
the gather's whole purpose is to deliver them narrow. Which decode is best is not a property of the
layout; it is a property of the layout together with what the consumer does next.

## 4. How the multiple moves across access patterns

### 4.1 Sequential, decode-dominated

Section 3. 4.6x becomes 1.50x. The residue is the extra shift and the extra load-plus-recombine that
the dense form does not need, and it is close to instruction-throughput bound: 1.86 instructions per
element against 1.00 predicts 1.86x and the machine delivers 1.50x, the difference being that a wide
core absorbs some of the extra work in parallel.

One honesty note on the baseline. The dense variant masks (`bic.8h`) because the bench's carrier is a
`u16` holding a 13-bit value. A real `Layout::Dense` carrier whose invariant already guarantees the
upper bits are zero would not need that mask, so the true dense floor is at or below 1.00 instructions
per element and the multiples above are, if anything, generous to the packed side.

### 4.2 Sequential, with per-element consumer work

I predicted this would shrink the multiple, on the reasoning that a fixed decode overhead amortises
against whatever the consumer then does. **The measurement refused the prediction**, and the refusal
was more informative than the prediction would have been. Adding one 32-bit multiply, a shift and an
xor per element moved the plan-driven decoder from 1.50x to 2.05x:

| N | native | naive | windowed | narrowed | simd gather |
|---:|---:|---:|---:|---:|---:|
| 16384 | 0.1975 | 0.4906 (2.48x) | 0.4049 (2.05x) | 0.4020 (2.04x) | 0.2544 (**1.29x**) |
| 65536 | 0.1972 | 0.4811 (2.44x) | 0.4041 (2.05x) | 0.4051 (2.05x) | 0.2542 (**1.29x**) |
| 98304 | 0.1973 | 0.4886 (2.48x) | 0.4044 (2.05x) | 0.4016 (2.04x) | 0.2537 (**1.29x**) |
| 262144 | 0.1969 | 0.4773 (2.42x) | 0.4026 (2.04x) | 0.4007 (2.04x) | 0.2531 (**1.29x**) |

The cause is lane width. The windowed decode leaves each field in the low bits of a 64-bit window, so
subsequent 32-bit work runs two lanes to a vector where the dense path runs four or eight. My first fix
was to narrow each group to a `[u16; 8]` before the kernel; that is the `narrowed` column, and it
changed nothing at all (4.16 instructions per element against 4.11), because the narrowing itself costs
what it saves. The fix that works is not to narrow after decoding but to decode into narrow lanes:
gather each field's bytes into its own lane with `tbl`, shift by a constant per-lane vector, mask, and
hand the consumer values already in their natural width. That is the `simd` column, **1.29x**, and it
is the best result in this file.

So the multiple is not monotone in the amount of consumer work, and no single decoder wins both
columns. The design statement that follows is in section 6.

### 4.3 Random access

I did not re-measure this and I am not going to restate it as if I had. File 75's figure is roughly
2.2x, and its own instruction counts for the random-access loops agree closely with the binaries
(section 1.2: 9 against 22 instructions per element). Three things are worth adding from the compiled
side. The address multiply that file 75 attributes the random-access penalty to
(`75:150-155`, `umull`) is a single instruction on a three-cycle pipeline and cannot account for a
2.2x; the 13 extra instructions are mostly the byte recombination and the register-operand shift. The
per-lane plan does not help a single random element, because `i mod P` is genuinely data at that point,
so this is the one access pattern where the naive shape is close to the best available. And under a
real cache miss, which is what random access over a column larger than cache actually costs, both forms
are latency-bound and the compute difference is hidden; file 75's own numbers show exactly this, the
gap narrowing from 4.6x to 2.2x as soon as the access stops being sequential.

The honest summary for random access is that the packed form is worse by a smaller factor than the
sequential naive figure suggested, for a reason that has nothing to do with packing being good at
random access: the dense form loses more of its own advantage.

### 4.4 Writes, and the one consequence that is not a cost

I did not measure writes and I am not going to guess at a number for them. But one structural fact
follows from the ratified definition alone and belongs in the design text rather than in a bench.

`Layout::Bitpacked` means no inter-value padding, so adjacent values share bytes. A store to one value
is therefore a read-modify-write of a byte that partly belongs to its neighbour. Two consequences, and
the second is the one that reaches outside arvo:

- A single-element write costs a load, a mask, an or and a store where the dense form costs a store.
- **Two writers on adjacent elements are writing the same byte.** For a column dispatched in parallel
  slices, a slice boundary that falls inside a group is a data race that no amount of care inside
  either writer prevents.

The granule that removes it is already computed: a slice boundary is safe exactly when it is a multiple
of `P = 8 / gcd(W, 8)` elements, because that is precisely the point at which a group ends on a byte
boundary. The same const that decides the unroll decides the partition granule. This design does not
own the scheduler that slices columns, but it owns the number, and stating it is the downstream
contract this section owes: **a bitpacked column publishes its write granule, and a consumer that
partitions it must partition on that granule.**

*Grounded on: measured (`mock/benches/bitpack-decoder-shape_*`, `mock/benches/bitpack-kernel-
amortisation_*`, 80 warm samples per variant per size, correctness re-checked by the harness's own
`validate_output` on every call and by five committed tests), compiled (`81_probes/OUTCOMES.md` I1),
settled shapes (`78:552-567`, the ratified one meaning), reasoned (4.3's attribution, 4.4 in full).*

## 5. What is inherent, stated as a price rather than a suspicion

Three things survive every decoder I built, and they are the honest content of "bitpacking costs
something".

**One extra shift and one extra mask per element.** A dense carrier is already the value; a packed
carrier has to be shifted into place and masked. At width 13 with an eight-element period, seven of
every eight elements need a nonzero shift. This is irreducible.

**Values arrive at the window's width, not their own.** Everything after the decode either accepts wide
lanes or pays to narrow them. A gather-based decode moves the cost into the decode rather than removing
it: `simd` is 2.00 instructions per element against `windowed`'s 1.86 for a plain sum, and wins only
when the consumer's own work benefits from narrow lanes.

**No independently addressable element.** Section 4.4. This is not a throughput cost; it is a
constraint on how a consumer may write and partition, and it is the only item in this file that no
choice of decoder can remove.

Measured, on this host, at width 13, against dense: **1.29x to 1.50x**, depending on the consumer's
work, with the best decoder for that work. Not 4.6x. The 4.6x was the price of computing at runtime
what the type already knew.

And the thing this bench cannot price, stated so nobody reads the multiple as the whole story: at width
13 the packed column is 18.75% smaller, and at width 3 it is 62.5% smaller. Section 1.1 establishes
that no size the harness can hold makes that visible on a single-core sequential sum, because the loop
is compute-bound at about 18 GB/s throughout. `Cold`'s intent (`78:404-405`, op's own "should be something
between warm and precise... can take more cost than warm") is a footprint intent, and pricing a
footprint intent needs a workload with bandwidth contention, which is a multi-column, multi-core shape
this review has no artifact for. Naming that gap is more useful than another number in the regime that
cannot show it.

## 6. What this hands forward, in the consolidation's own form

**The design text.**

*`Layout::Bitpacked`'s cost against `Layout::Dense` is a property of the decoder, not of the layout,
and the decoder's every parameter is a function of the logical width alone. A bitpacked column of
logical width `W` publishes, as consts on its layout type and not as runtime arithmetic: the period
`P = 8 / gcd(W, 8)`, the group stride `G = W * P / 8` bytes, the window offsets and per-lane bit shifts
that address every field in a group, the field mask, the load width (the narrowest of one, two, four,
eight or sixteen bytes admitting `W + 7` bits), and the read headroom that load width requires past the
last group. A layout whose width admits no single-load window refuses at monomorphisation rather than
addressing a lane wrongly; the refusal is a const evaluation failure naming the width, compiled.
Because these are consts rather than a `const fn` called from the decode, a decode monomorphised at `W`
unrolls against a literal period, strides by a literal, and extracts with two-literal bitfield
instructions, which is the difference between 7.5 and 1.86 instructions per element at width 13,
compiled. Measured on aarch64 at width 13 over four sizes from 32 KB to 512 KB of dense footprint, and
against a fully vectorised dense baseline: a plain column sum costs 1.50x dense, and a column sum of a
per-element function of each value costs 1.29x dense with a decode that gathers into natural-width
lanes. Which decode is optimal is a joint property of the layout and the consumer's own operation, not
of the layout alone: the plan-driven decode wins when the consumer accumulates wide, the gather-based
decode wins when the consumer computes narrow, and the two differ by 1.6x on the same buffer. The
substrate ships both and picks on the operation's lane width, per the always-optimal-internals rule; it
does not ship one and call the other's regime the layout's price.*

*The same period is the column's write granule. A bitpacked column has no per-value byte image, so
adjacent values share bytes and no element is independently writable; a consumer that partitions such a
column for parallel writes must place every boundary on a multiple of `P` elements, which is where a
group ends on a byte boundary. The column publishes `P` for that purpose as well as for the decode.*

*The multiple is not the whole cost model. It prices decode against a compute-bound loop, which is the
regime where a smaller footprint buys nothing. `Cold`'s footprint intent is realised under bandwidth
contention across columns and cores, a regime no artifact in this review measures.*

(Grounded: ratified `78:552-567`, `78:163-166` (the fourth rule), `78:404-405` (`Cold`'s intent, op's
words); compiled `81_probes/` in full plus the thirteen bench binaries disassembled this session (file 75's
five plus the eight this file adds); measured
`mock/benches/bitpack-decoder-shape_*` and `mock/benches/bitpack-kernel-amortisation_*`; reasoned, the
write-granule consequence and the joint-optimality statement.)

**What this corrects rather than adds.** `78:561-562`'s "stable across three sizes spanning
cache-resident to past-L1" is wrong on the host that produced it: 128 KB L1, largest size 32 KB, never
left. The stability is real and means the loop is compute-bound; it is not evidence about whether the
multiple is inherent. `75:178-182`'s instruction table characterises a standalone probe, not the
benched program, and is off by five times on the dense sequential row. Neither correction touches the
ratified reading of what `Bitpacked` means, which is what op ratified and what this file assumes
throughout.

**What is owed, named rather than performed.**

- The bandwidth-contention shape (section 5's last paragraph): several packed and dense columns
  streamed concurrently across cores, which is the only shape that can price a footprint. It needs a
  harness input larger than the current `Routine::Input`-by-value construction admits; the input
  transits the stack in `build_input_bytes` (the harness's own `build_input_bytes`, mockspace `bench-core/src/lib.rs:148-158`, which materialises
  the whole flat input as a local before copying it), which caps a flat input
  at a few megabytes. That cap is why my sweep stops at 512 KB.
- Widths above 57 with a 128-bit window, and the two-load shape above 121. The const constructor
  refuses them today rather than serving them; the plan for them exists on paper (section 2's table)
  and is not built.
- Writes, measured. Section 4.4 states the structure and gives no number.
- Random access, re-measured against the plan-driven shape. I carried file 75's figure with a
  correction to its explanation and did not re-run it.
- Whether the decoder pair belongs to `Lowering` or to the algorithm crates that consume columns. This
  file establishes that the choice between them is keyed on the consumer's operation, which by the
  layer-keying rule puts the *selection* one layer above the layout, and I flag rather than settle it.

## 7. Standing

Nothing here overturns a ratified call. `78:552-567` is treated as governing throughout and every
measurement in this file assumes its reading of what `Bitpacked` denotes. What op sent back was the
number attached to that reading, and the number moves: from 4.6x to between 1.29x and 1.50x, measured
the same way, on the same buffer, on the same host, with a decoder that reads its own parameters from
the type instead of recomputing them per element. The corrections in section 1 are to method and to two
figures, one of them mine to notice and nobody's to have caught earlier without doing this work. Only
op's calls are final, and even those go stale.
