# 93_probes outcomes

Toolchain: `rustc 1.98.0-nightly (57d06900f 2026-05-27)`, `aarch64-apple-darwin`, resolved from
`rust-toolchain.toml` inside the tree. C probes: Homebrew clang (`/opt/homebrew/opt/llvm/bin/clang`),
`-O2`, aarch64 native and `-target x86_64-apple-darwin -isysroot $(xcrun --show-sdk-path)`; the x86
binary executes under Rosetta 2, so its silicon-level result is read, not cited. No timing claim
anywhere; every count is read from `objdump -d` or the emitted `.s`. The bench harness was not run,
per the standing outage (`91:1104-1108`).

## probe_1_what_the_isa_actually_defines.rs

```
rustc --edition 2021 -O probe_1_what_the_isa_actually_defines.rs --out-dir out
./out/probe_1_what_the_isa_actually_defines
```

WORKS. CLAIM A: aarch64 `sdiv`/`udiv` with a zero divisor return 0 for every dividend tried,
including `0/0` and `MIN/0`; no trap. CLAIM B: `sdiv MIN/-1` returns `MIN`, which is the value
layer's own `ReduceModulo` answer for that cell. Note for the deliverable: the ISA's answer at `0/0`
is the same 0 as at `x/0`, so the instruction does not observe the design's ratified
`divideByZero`/`invalid` distinction.

## probe_2_x86_does_not_define_a_value.c

```
clang -target x86_64-apple-darwin -isysroot $(xcrun --show-sdk-path) -O2 \
  probe_2_x86_does_not_define_a_value.c -o out/probe_2_x86
./out/probe_2_x86 5 2   -> prints "5 / 2 = 2", exit 0
./out/probe_2_x86 5 0   -> killed by SIGFPE, shell status 136, no output
```

WORKS (as a refusal). x86-64 `idiv` with a zero divisor raises #DE; the process dies. The target
defines no value at the cell, on silicon executed on this host under Rosetta 2.

## probe_3_the_toolchain_takes_it_back.c

```
clang -O2 -S probe_3_the_toolchain_takes_it_back.c -o out/probe_3_aarch64.s
clang -target x86_64-apple-darwin -isysroot $(xcrun --show-sdk-path) -O2 -S \
  probe_3_the_toolchain_takes_it_back.c -o out/probe_3_x86_64.s
```

WORKS. `f` (divide first, check the divisor after) compiles to `sdiv w0, w0, w1; ret` on aarch64 and
`movl; cltd; idivl; retq` on x86-64: the `d == 0` arm is deleted on both targets, because LLVM
`sdiv` is UB on a zero divisor and the optimizer assumes the divisor nonzero. Control `g` (check
first) keeps its `cbz` / `testl; je`. So the IR the design lowers through defines the cell as UB on
every target, including the one whose silicon defines it as 0.

## probe_4_the_cell_priced.rs

```
rustc --edition 2021 -O probe_4_the_cell_priced.rs --out-dir out
./out/probe_4_the_cell_priced
objdump -d out/probe_4_the_cell_priced
```

WORKS. CLAIM A: 64 elements, 14 zero-divisor cells scattered, negative divisors included; the
consumer-stated form (`if d == 0 { 0 } else { x.wrapping_div(d) }`) and the raw-asm `sdiv` form
deliver identical outputs element for element. The two forms compute one function on this target by
construction (fallback 0 chosen to coincide with the ISA constant; `wrapping_div` states the
`MIN/-1` cell `sdiv` also delivers).

CLAIM B, steady-state loop bodies from `objdump -d`, neither loop unrolled:

| body | instructions per element (common path) | data-dependent branches per element |
|---|---:|---:|
| consumer-stated (`cbz` + `cmn`/`ccmp` guards + `sdiv`) | 11 | 2 |
| raw asm `sdiv` | 7 | 0 |

Two of the consumer form's guard instructions (`cmn x10, #1; ccmp x11, x9`) are LLVM's own inserted
`MIN/-1` guard on `wrapping_div`, not the consumer's zero test: the toolchain guards even the cell
the value layer fully defines, because LLVM `sdiv` is UB there too. The raw-asm body is therefore
reachable as a Kind-1 cfg-gated *implementation* of the consumer-stated function on aarch64 (the
stated values and the ISA constants coincide cell for cell), at which point the same 7-instruction
body carries spec-defined semantics. Identical machine code, opposite provenance.
