# 142 probes: code inspection for the clamping-container fork

Carries **no timings**. Every number in file 142 comes from the harness sections
under `mock/benches/`; this directory exists so the mechanism behind those
numbers can be read rather than inferred.

`src/lib.rs` exports one symbol per case and **imports the transforms from the
bench's own shared crate** (`bench-warm-clamp-shared`), so nothing here is a
second copy of the kernel.

`p142_asm_default.s` is the emitted assembly at **cargo's default release
profile**, which is what the bench cdylibs are built with. Vector-operation
counts per symbol at that profile:

| symbol | lines | vector ops |
|---|---|---|
| `c_min_w16_a256` | 26 | 0 |
| `c_fit_w16_a256` | 43 | 16 |
| `c_lanes_w16_a256` | 93 | 31 |
| `c_head_w16_a256` | 115 | 64 |
| `c_min_w16_a4` | 47 | 28 |
| `c_fit_w16_a4` | 76 | 59 |
| `c_min_w64_a16` | 55 | 0 |
| `c_lanes_w64_a16` | 54 | 0 |

## The trap this directory hit, recorded because it nearly cost a wrong conclusion

The first cut of this probe carried its own `[profile.release]` with
`lto = "fat"`, `codegen-units = 1`, `panic = "abort"`, on the reasoning that a
more aggressive profile is a better one to read. Every symbol came out with
**zero vector operations**, which disagreed with the harness by 13x and would
have supported a confident and wrong claim that the interior-safe arm does not
vectorise either.

Isolating the three settings one at a time showed `codegen-units` and `panic`
were innocent and that **both `lto = "fat"` and `lto = "thin"` took every symbol
to zero**. That reads as a compiler finding and is not one.

`--emit=asm` on a **library** crate under LTO emits the code produced before the
link step, and under LTO the optimisation that matters happens at the link step,
so the file shows a program that never runs. Building `src/bin/emit.rs` and
disassembling the **linked** binary settles it: 991 vector operations at the
default profile and 568 under `lto = "fat"`, both nonzero.

So: read emitted assembly from a linked artifact, or from a library built
without LTO. A library plus `--emit=asm` plus LTO is a reading of nothing.

## Reproducing

```
cargo +nightly-2026-05-28 rustc --release --lib -- --emit=asm
cargo +nightly-2026-05-28 build --release --bin emit
objdump -d --no-show-raw-insn target/release/emit | grep -cE '\.8h|\.4s|\.16b|\.2d'
```

## What the harness already had, and what reading it late cost

The bench harness ships `bench-harness/src/disasm.rs`, and this directory should
have been built on it rather than beside it. What it has and what it does not:

**It has extraction from the timed artifact.** `extract_bench_entry` runs
`objdump -d --disassemble-symbols=bench_entry` against each **variant dylib**,
with an `otool -tv` fallback, and normalises addresses away
(`disasm.rs:13-59`). That is the right input, and it is the input this directory
did not use. The probe crate here is a separate compilation with its own
exports, which is exactly the gap the LTO trap above lived in: had the counts
come from the dylib in the first place, the 13x disagreement would never have
appeared, because there would have been one program rather than two.

**It does not have analysis.** The only public entry point is
`check_duplicates` (`disasm.rs:84`), which compares normalised text for exact
equality across variants and warns on a match. There is no opcode counting, no
vector-against-scalar classification, and no extraction of any symbol other than
`bench_entry`. So the counting in this directory duplicates nothing that exists.

**Two defects found by using it, both worth sending upstream.**

`extract_bench_entry` passes `bench_entry` to objdump. On Mach-O the symbol is
`_bench_entry`, so on macOS objdump answers `failed to disassemble missing
symbol bench_entry` and the function silently falls through to the `otool`
branch. The objdump path is dead on this platform and nothing says so.

More consequentially, under `#[bench_variant]` the `bench_entry` symbol is a
generated dispatcher and the arm bodies are separate static functions.
Disassembled from all six arms of this bench it is **592 instructions in every
one of them, with zero vector operations and zero saturating adds in all six**,
while the whole-dylib counts differ substantially (`min_lanes` 2246 vector
operations and 460 `uqadd`; `accfit` 1658 and 0). So `check_duplicates` is
comparing a shim that cannot reflect what the arms compute, and its silence is
not evidence that two arms differ.

Whole-dylib counts, which are aggregate over 46 keys and therefore cannot decide
a cell, but which do read the artifact that was timed:

| arm | instructions | vector ops | `uqadd` | `csel` |
|---|---|---|---|---|
| `head` | 52926 | 1534 | 0 | 744 |
| `minimum` | 58147 | 1761 | 300 | 719 |
| `min-lanes` | 60874 | 2246 | 460 | 896 |
| `acc64` | 52792 | 2053 | 16 | 729 |
| `accfit` | 52726 | 1658 | 0 | 732 |
| `accfit-dyn` | 51884 | 1227 | 0 | 726 |

Reproduce with the corrected symbol:

```
objdump -d --disassemble-symbols=_bench_entry --no-show-raw-insn \
  mock/target/release/libbench_warm_clamp_accfit.dylib
```
