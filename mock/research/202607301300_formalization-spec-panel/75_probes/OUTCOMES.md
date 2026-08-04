# 75_probes outcomes

One artifact, `codegen.rs`, a single standalone file (no `[workspace]` crate needed; a probe this
size does not warrant one). Compiled and run fresh this session, from inside the repo tree so the
directory-scoped toolchain override applies. Toolchain confirmed immediately before every compile:
`rustc --version` from inside `75_probes/` resolves to `1.98.0-nightly (57d06900f 2026-05-27)`,
`aarch64-apple-darwin`, matching `rust-toolchain.toml`. The identical command from `/tmp`, outside the
repo tree, resolves to `1.94.0 (4a4ef493e 2026-03-02)` **stable**, confirming the dispatch's own
warning and file 73's own recorded check of the same fact. No `#![feature(...)]` line anywhere in the
file.

## Reproduction

Correctness (`cargo test`-shaped, run as a bare `rustc --test`):

```
rustc --edition 2024 --test codegen.rs -o /tmp/codegen_test
/tmp/codegen_test
```

`1 passed; 0 failed`. The single test cross-checks `sum_aligned`, `sum_zeropad`, both `*_standalone`
extraction entry points, and both `*_rand` entry points against a from-scratch reference decode over
4096 pseudo-random 13-bit values, packed by hand inside the test (not via `bench-bitpack-shared`, so
this probe stays reproducible as one file with no path dependency).

Disassembly:

```
rustc --edition 2024 -C opt-level=3 -C lto=fat -C codegen-units=1 -C panic=abort \
    --crate-type lib --emit=obj,asm codegen.rs -o codegen.o
objdump -d --no-show-raw-insn codegen.o > codegen.objdump.txt
nm codegen.o
```

Every probed function is `#[unsafe(no_mangle)]` (edition 2024's spelling of `#[no_mangle]`) so it
survives to its own symbol; the two `extract_*` transforms are `#[inline(always)]` free functions
(matching the real source in `bench-bitpack-shared`), each with a `#[inline(never)]` one-call
wrapper (`extract_aligned_standalone`, `extract_zeropad_standalone`) whose disassembly shows the
transform's body inlined into its own symbol. `objdump` labels the first function in the object
(`extract_aligned_standalone`) as `<ltmp0>` rather than by its real name, a Mach-O local-symbol
display quirk on the raw `.o`; `nm codegen.o` confirms it is a real global symbol
(`0000000000000000 T _extract_aligned_standalone`), checked before any instruction inside it was
attributed to that function by name below.

## What each probe answers

| Probe (symbol) | Question | Outcome |
|---|---|---|
| `extract_aligned_standalone` | What does one byte-aligned-slot extraction (address `i*2`, a shift) compile to, on its own? | **Compiled.** Fast path: `lsl x0,x2,#1` (address), `ldrb`×2 (Rust's per-byte bounds-checked slice indexing did not fuse into one 16-bit unaligned load), `bfi w0,w8,#8,#5` (combines the two bytes into the 13-bit result directly, folding the mask into the bit-field-insert rather than emitting a separate `and`). 4 real instructions plus 2 predicted-not-taken bounds-check branch pairs. |
| `extract_zeropad_standalone` | What does one zero-inter-value-padding extraction (address `(i*13)>>3`, a multiply) compile to, on its own? | **Compiled.** Fast path: `mov w8,#13; mul x12,x2,x8` (address, a real multiply), `lsr` (byte offset), 3× `ldrb` (LLVM proved the 4th byte the source reads is never used, given the mask and worst-case shift, and dropped that load entirely: the source asked for 4 bytes, the compiled function reads 3), `and`+`orr`×2+`lsr`+`and` (combine and mask). 10 real instructions plus 4 predicted-not-taken bounds-check branch pairs. |
| `sum_aligned` | What does the sequential column-sum loop compile to for the byte-aligned reading? | **Compiled.** Hot loop body 11 instructions/element (2× `ldrb`, 1 `ubfiz`, 2 `add`, plus loop bookkeeping and 1 bounds-check pair per iteration; LLVM did not hoist the per-iteration bounds check the way it did for `sum_native` below). No multiply anywhere in the loop. |
| `sum_zeropad` | What does the sequential column-sum loop compile to for the zero-padding reading? | **Compiled.** Hot loop body 24 instructions/element (3× `ldrb`, `and`+`orr`×2+`lsr`+`and`, 1 `add` accumulate, plus loop bookkeeping and 4 bounds-check pairs per iteration). **No multiply**: LLVM strength-reduced the address computation into a running accumulator (`add x13,x13,#0xd` each iteration) since the loop counter and the field-width stride are both compile-time-linear; the multiply this dispatch's own prose predicted for "the zero-padding address" does not appear here, because the sequential loop's own linearity removes the need for it. |
| `sum_aligned_rand` | Does a data-dependent (permuted) index reintroduce any cost the sequential loop's strength reduction removed, for the aligned reading? | **Compiled.** Hot loop body 17 instructions/element. Address computation is still `lsl x9,x9,#1`, a shift: aligned's addressing needs no multiply at any access pattern, because doubling is a shift regardless of whether the index is sequential or random. |
| `sum_zeropad_rand` | Same question, for the zero-padding reading. | **Compiled.** Hot loop body 27 instructions/element, and now carries a real `umull x15,w9,w14` (32×32→64 unsigned multiply, `idx*13`) that the sequential loop's strength reduction had removed. This is the one instruction class that is genuinely access-pattern-dependent rather than reading-dependent: it costs nothing under sequential access and a real multiply under random access. |
| `extract_native_standalone` / `sum_native` / `sum_native_rand` | What does `Layout::Dense` actually cost once the carrier is a genuinely native-typed `[u16; _]` array, rather than the byte-buffer model `extract_aligned` uses to keep both readings on comparable byte-addressed infrastructure? | **Compiled, and the byte-buffer model understates Dense's real cheapness.** `extract_native_standalone`: `ldrh w8,[x0,x2,lsl#1]` (one aligned 16-bit load, the shift folded into the addressing mode at zero extra cost) + `and` = 2 real instructions, 1 bounds check. `sum_native`'s hot loop: 5 instructions/element (`ldrh` with post-increment, `and`, `add`, `subs`, `b.ne`), and the bounds check is hoisted **once, before the loop**, not carried per iteration, because a plain `[u16]` slice indexed by a linear counter is exactly the shape LLVM's bounds-check-elimination pass is built for. `sum_native_rand`'s hot loop: 9 instructions/element, one bounds check per iteration (data-dependent indices defeat the hoist), still no multiply (the shift folds into the load's addressing mode for random indices too). |

## Instruction-count summary (fast-path / hot-loop-body, this compile only)

| Function shape | aligned (byte-buffer) | zeropad | native (`[u16;_]`) |
|---|---:|---:|---:|
| single extraction, isolated | 4 | 10 | 2 |
| sequential loop, per element | 11 | 24 | 5 |
| random-access loop, per element | 17 | 27 | 9 |

These are static instruction counts from one compile on one target, not a runtime claim; the runtime
throughput numbers this dispatch relies on are in `mock/benches/bitpack-sequential-sum_n*.csv` and
`mock/benches/bitpack-random-sum_n*.csv`, produced by the harness, not by this probe. The two do not
move together in lockstep (the sequential static-instruction ratio zeropad/aligned is 24/11 = 2.18x
while the measured wall-clock ratio is roughly 1.3x-1.4x; branch prediction and superscalar issue
absorb most of the extra predicted-not-taken bounds-check instructions that dominate the static
count), which is itself the reason the bench, not this probe, is the number the design decision
rests on. This probe's job is explaining the *shape* of the cost (which instruction classes appear,
which are access-pattern-dependent), not replacing the measurement of its *size*.
