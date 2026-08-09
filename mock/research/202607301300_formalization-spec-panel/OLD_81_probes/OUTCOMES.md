# 81_probes: outcomes

Host: Apple M1, `aarch64-apple-darwin`, Darwin 25.5.0. Performance-core cache
sizes read this session: `sysctl hw.perflevel0.l1dcachesize` = **131072**,
`hw.perflevel0.l2cachesize` = **12582912**, `hw.cachelinesize` = 128. The
top-level `hw.l1dcachesize` (65536) reports the efficiency core and is not the
figure that governs a benchmark pinned to no core in particular; both are
recorded because the difference is what makes one of the findings below.

Toolchain: `rustc 1.98.0-nightly (57d06900f 2026-05-27)`, resolved from
`rust-toolchain.toml` inside the repository tree. The identical command run
outside the tree resolves to stable `1.94.0`; every command below was run from
inside.

Build flags, stated because a flag has twice in this review turned a result into
an artifact of its own methodology:

- probes: `rustc -O -C opt-level=3 -C codegen-units=1 --edition 2024`, plus
  `-C panic=abort --crate-type=cdylib` for the disassembled builds.
- bench variants: the workspace's ordinary `cargo build --release`, which is
  `opt-level=3`, `lto=false`, `codegen-units=16`, `panic=unwind`. No
  `-C target-cpu` anywhere, so the NEON baseline is the aarch64 default, not
  a host-tuned build.

## Files

| File | What it is |
|---|---|
| `decoders.rs` | Four decoder shapes over one packed buffer at eight widths, plus the dense baseline. Its own `main` is the correctness harness. |
| `decoders2.rs` | The same period-unrolled decode with the plan moved from `const fn` calls into associated consts. |
| `decoders3.rs` | The windowed decode: window offsets and per-lane (window, shift) pairs derived in const position. |
| `width58_refusal.rs` | Compile-fail companion: the plan refuses a width a 64-bit window cannot hold. |
| `*.objdump.txt` | Disassembly of each, `objdump -d`. |

## N1. A hardwired load width is silently wrong above 25 bits

`decoders.rs` first run, before it was patched. File 75's `extract_zeropad`
(`benches/variants/bitpack-shared/src/lib.rs:168-181`) reads a fixed 32-bit
window regardless of the field width. A field may start at any of eight bit
offsets, so a 32-bit window covers a field only while `W + 7 <= 32`, that is
`W <= 25`. At `W = 27` the probe's own reference check fired:

```
assertion `left == right` failed: naive W=27
  left: 55892321781
 right: 67703481845
```

No compiler diagnostic, no panic, a wrong sum. The patched `decoders.rs`
carries this as an assertion rather than a description: `sum_naive_u32` is
required to agree with the reference **exactly when** `W + 7 <= 32`, at all
eight probed widths, and that assertion passes.

The correct load width is a function of the field width alone, and
`load_bytes(w)` in `decoders.rs` computes it: 1, 2, 4, 8 or 16 bytes for
`w + 7` up to 8, 16, 32, 64, 128 bits respectively, and no single load above
121 bits. Nothing about the packing forces a second load below 57 bits with a
64-bit window; the second load appears only when the window width is chosen
without consulting the field width.

## C1. A `const fn` in value position is not const-evaluated

`decoders.rs`'s `sum_period` derives its unroll factor from `period(W)`, a
`const fn` of a const generic parameter, called in an ordinary value position.
rustc guarantees const evaluation only in a const position, so this is left to
LLVM, and LLVM did not fold the recursive gcd. `_period_w13` in
`decoders.objdump.txt` opens with a **runtime division loop**:

```
    35a8: 1ac90948     	udiv	w8, w10, w9
    35ac: 1b09a908     	msub	w8, w8, w9, w10
    35b0: 35ffff88     	cbnz	w8, 0x35a0 <_period_w13+0x8>
```

and the group loop then carries live guards against the runtime period
(`cmp x9, #0x5`, `cmp x9, #0x2`). Half the shifts folded anyway, through jump
threading (`ubfx x4, x4, #5, #13` and friends appear), which is what makes this
worth recording: the emitted code looks like the constant-folded version until
counted.

`decoders2.rs` puts the identical quantities on a trait as associated consts.
`_direct_w13` then unrolls by the period against a literal trip count, advances
by a literal 13 bytes per group, and every extraction is a `ubfx` with two
literal operands. Same source arithmetic, same optimiser, different binding
time.

## C2. Eight overlapping wide loads become thirteen byte loads

`decoders2.rs`'s plan issues one 8-byte read per lane. LLVM proved only two or
three bytes of each are live and narrowed them back to `ldurb`, recombining
with `orr`: 42 instructions per group of eight fields, **5.25 per element**.

`decoders3.rs` states the windows instead of the lanes. A 64-bit window placed
at a byte offset serves every lane whose bit offset leaves `W` bits inside it;
greedily assigning lanes gives two windows per eight fields at `W = 13`, at byte
offsets 0 and 6. The emitted loop (`_win_w13`) is then **vectorised by LLVM
unprompted**: `ld1.d {v}[1]` gathers, `ushr.2d` by literal amounts, `and.16b`,
`add.2d`, 119 instructions per 64 elements, **1.86 per element**.

Window plans the const constructor produced, checked against the packer at
every probed width:

| W | period | group bytes | windows | lanes per window |
|---:|---:|---:|---|---:|
| 3 | 8 | 3 | `[0]` | 8 |
| 11 | 8 | 11 | `[0, 6]` | 5, 3 |
| 12 | 2 | 3 | `[0]` | 2 |
| 13 | 8 | 13 | `[0, 6]` | 4, 4 |
| 16 | 1 | 2 | `[0]` | 1 |
| 20 | 2 | 5 | `[0]` | 2 |
| 27 | 8 | 27 | `[0, 6, 13, 20]` | 2, 2, 2, 2 |
| 57 | 8 | 57 | `[0, 7, 14, 21, 28, 35, 42, 49]` | 1 each |

## C3. The width refusal is a monomorphisation-time diagnostic

`width58_refusal.rs`. A 64-bit window cannot hold a 58-bit field. The plan's
`WINDOW_FITS` associated const asserts `W + 7 <= 64` and is touched by the
decode, so instantiating at 58 fails to compile:

```
error[E0080]: evaluation panicked: a 64-bit window cannot hold a field wider
              than 57 bits
  --> width58_refusal.rs:21:29
   |
21 |       const WINDOW_FITS: () = assert!(
   |  _____________________________^
   | |_____^ evaluation of `<Pack<58> as Packing>::WINDOW_FITS` failed here
note: erroneous constant encountered
  --> width58_refusal.rs:28:14
   |
28 |     let () = K::WINDOW_FITS;
```

`Pack<57>` in the same file compiles and runs. The refusal is at the width, not
at a read.

## I1. Per-element instruction counts, from the shipped bench binaries

Counted from `_bench_entry` in each `target/release/libbench_bitpack_*.dylib`,
taking the innermost backward-branching loop and dividing by the elements it
retires (from its own pointer stride or loop counter decrement). These are the
binaries the harness loaded, not a standalone probe.

| variant | loop body | elements per iteration | instructions per element |
|---|---:|---:|---:|
| `plan_native` (dense, sum) | 32 | 32 | 1.00 |
| `plan_naive` (index-driven, sum) | 30 | 4 | 7.50 |
| `plan_windowed` (plan-driven, sum) | 119 | 64 | 1.86 |
| `plan_simd` (byte gather, sum) | 16 | 8 | 2.00 |
| `mac_native` (dense, kernel) | 64 | 32 | 2.00 |
| `mac_windowed` (plan-driven, kernel) | 263 | 64 | 4.11 |
| `mac_narrow` (plan-driven, narrowed, kernel) | 133 | 32 | 4.16 |
| `mac_simd` (byte gather, kernel) | 24 | 8 | 3.00 |

The same count run against the binaries file 75's own bench loaded:

| file 75 variant | loop body | elements per iteration | instructions per element | file 75's table (`75:180-182`) |
|---|---:|---:|---:|---:|
| `native_seq` | 33 | 32 | **1.03** | 5 |
| `aligned_seq` | 59 | 16 | **3.69** | 11 |
| `zeropad_seq` | 30 | 4 | **7.50** | 24 |
| `native_rand` | 9 | 1 | **9.00** | 9 |
| `zeropad_rand` | 22 | 1 | **22.00** | 27 |

The dense sequential loop is fully vectorised (`ldp q4, q5` / `bic.8h` /
`ushll` / `uaddw`, 32 `u16` per iteration). The packed sequential loop is
unrolled by four against a period of eight, so its shifts stay in registers
(`lsr w0, w4, w0`) instead of folding into `ubfx`. The random-access rows agree
closely with file 75's standalone probe and the sequential rows do not, for the
reason the table makes visible: a scalar bounds-checked probe resembles the real
random-access loop and does not resemble the real sequential one.

## Reproduction

```
cd mock/research/202607301300_formalization-spec-panel/81_probes
rustc -O -C opt-level=3 -C codegen-units=1 --edition 2024 decoders.rs  -o decoders  && ./decoders
rustc -O -C opt-level=3 -C codegen-units=1 --edition 2024 decoders2.rs -o decoders2 && ./decoders2
rustc -O -C opt-level=3 -C codegen-units=1 --edition 2024 decoders3.rs -o decoders3 && ./decoders3
rustc -O --edition 2024 width58_refusal.rs -o width58_refusal   # expected to fail
rustc -O -C opt-level=3 -C codegen-units=1 -C panic=abort --edition 2024 \
      --crate-type=cdylib decoders3.rs -o libdecoders3.dylib
objdump -d libdecoders3.dylib | less
```
