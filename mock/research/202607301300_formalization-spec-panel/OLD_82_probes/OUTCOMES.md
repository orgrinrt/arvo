# Outcomes, 82_probes

Toolchain `rustc 1.98.0-nightly (57d06900f 2026-05-27)`, host `aarch64-apple-darwin`,
resolved from the repo's `rust-toolchain.toml` and confirmed by `rustc --version` run
inside the tree this session. Compile-only probes built
`rustc --edition 2021 --crate-type=lib --emit=metadata`; the codegen probe built
`--crate-type=lib -O --emit=obj` and read with `objdump -d`. Nothing under `mock/crates/`
was touched. Every timing is wall clock from `/usr/bin/time -p` on this host.

Host facts read this session with `sysctl`, because one of them corrects a published
figure: `hw.perflevel0.l1dcachesize = 131072` (128 KB, performance cores),
`hw.perflevel1.l1dcachesize = 65536` (64 KB, efficiency cores),
`hw.l1dcachesize = 65536` (the un-suffixed key reports the efficiency-core figure),
`hw.perflevel0.l2cachesize = 12582912` (12 MB), `hw.memsize = 8589934592`, Apple M1.

## probe_1_allones_at_real_precisions.rs. REFUSES at binary256, by design of the test

File 80's `foldexact` machinery, verbatim, instantiated at the four IEEE binary
interchange precisions. The grammar literals are asserted against their decimal values at
compile time first (`P113::VAL == 113`, `P237::VAL == 237`, and so on), so a refusal
cannot be a mis-typed literal.

| precision | p | outcome |
|---|---:|---|
| binary16 | 11 | compiles, `AllOnes` value asserted equal to `2^11 - 1` |
| binary32 | 24 | compiles, asserted |
| binary64 | 53 | compiles, asserted |
| binary128 | 113 | compiles, asserted, 0.052 s |
| binary256 | 237 | **refuses**, `error[E0275]: overflow evaluating the requirement`, rustc's own help naming `#![recursion_limit = "256"]`, `note: 126 redundant requirements hidden` |

**The boundary, located exactly**, by sweeping P from 100 to 139 with the projection
forced to normalise (a `PhantomData<OnesX>` in a function signature):

- P <= 128: compiles.
- P = 129: `error[E0320]: overflow while adding drop-check rules for PhantomData<I<I<...`
- P >= 130: `error[E0275]`.

**The second ceiling, independent of the first**, found by re-running with
`#![recursion_limit = "1024"]` and forcing `Nat::VAL` to evaluate:

- P <= 128: compiles.
- P >= 129: `error[E0080]: attempt to compute 2_u128 * u128::MAX, which would overflow`.

So there are two ceilings and they coincide at 128 by arithmetic accident, not by design:
rustc's default `recursion_limit` is 128, and `AllOnes<P>` denotes `2^P - 1`, which is
exactly `u128::MAX` at P = 128 and unrepresentable above it. Raising the recursion limit
clears binary256 for trait resolution (`P237` normalises at limit 1024) and then the
`Nat::VAL` ceiling fires instead. Neither ceiling is stated anywhere in the corpus.

File 80's dismissal, "a pathological precision in the thousands would meet the default
recursion limit; real precisions are two orders of magnitude below it" (80:192-194), is
wrong on both halves: the limit is 128 rather than thousands, and binary128's p = 113 is
15 below it rather than two orders of magnitude.

## probe_2_foldexact_without_allones.rs. COMPILES CLEAN, binary256 included

The same closed form, `foldexact(P, A) = bitlen(A * (2^P - 1))`, built without ever
materialising `2^P - 1`:

```
L = bitlen(A),  R = A - 2^(L-1)
foldexact(P, A) = P + L - 1 + bit
    bit = 0                   if R = 0        (A a power of two)
    bit = 1                   if R >= 1 and P >= L
    bit = [ (R << P) >= A ]   if R >= 1 and P <  L
```

Every recursion is structural (logarithmic in the value), the shift branch is entered only
when P < L so its depth is bounded by `bitlen(A)` and never by P, and the largest
intermediate is below `A^2`. P appears in the answer only as a summand.

**Verified in exact integer arithmetic** over P in 1..=299 by A in 1..=4099, **1,225,601
cells, zero mismatches** against `bitlen(A * (2^P - 1))` computed directly in Python's
arbitrary-precision integers.

**Built at the type level over the same sealed grammar**, zero feature gates, `#![no_std]`.
New machinery beyond file 80's: `CmpP` (three-way structural comparison with the
`DemoteToLt` / `PromoteToGt` refinements at the mixed constructor pairs), `ClearTop`
(returning a closed two-member kind `TopZero | TopSome<R>` rather than a `Nat`, because
`Z` versus `P: Pos` is not a distinction rustc's coherence can draw without negative
reasoning), `ShlP`, and a three-level dispatch chain `CorrOnTop` -> `CorrOnCmp` ->
`ShiftVerdict`.

**Checked at compile time**: 56 cells over the same precision and arity matrix file 80's
probe_1 used, each asserted equal to an independent u128 ground truth, including both
cells where `foldnum` is loose by one (p=8/A=257 and p=11/A=2049) and the
(p=2, A=3) tight-non-power case. Then binary128 (p = 113) against ground truth, and
**binary256 (p = 237) against literals computed offline in exact integer arithmetic**:
`foldexact(237, 3) = 239`, `foldexact(237, 256) = 245`, `foldexact(237, 257) = 246`,
`foldexact(237, 4096) = 249`. The width is placed in type position at binary256
(`pub type Binary256Fold = FoldAcc2<P237, A257>`), so the spine rule's own requirement is
met at the precision where file 80's construction refuses.

**Negative control**: appending
`const _: () = assert!(<<P8 as FoldExact2<A257>>::Out as Nat>::VAL == 17);` fails the build
with `error[E0080]: evaluation panicked`, so the assertions are live and the exact width at
that cell is genuinely 16.

**Priced**: 0.20 / 0.12 / 0.09 s wall over three runs against an empty `#![no_std]` lib
baseline of 0.05 / 0.04 / 0.04 s. Comparable to file 80's 0.145 s while covering strictly
more of the matrix, including the precision where the other construction does not exist.

## probe_3_where_const_position_bites.rs. COMPILES, and locates the boundary

File 81's rule is stated as a blanket: "An associated const on the layout type is [a const
position]; a `const fn` called from the decode is not" (81:236-238). Taken literally that
condemns file 79's `last_index` and `in_bounds`, whose bodies are
`<C as Dec>::Out::VAL` and `i < C::SIZE` (`79_probes/probe_2:139-144`,
`79_probes/probe_1:117-119`). Three shapes compiled at `-O` and disassembled:

| shape | body | emitted |
|---|---|---:|
| A, `const fn` reading an associated const | `N::VAL` | **3 instructions**: `cmp x0, #0xd` / `cset` / `ret` |
| B, `const fn` computing by recursion | `8 / gcd(N::VAL, 8)` | **10 instructions**, including a `udiv`/`msub`/`cbnz` back-branching loop and a second `udiv` |
| C, associated const | `const PERIOD: usize` | **2 instructions**: `mov w0, #0x8` / `ret` |

Shape B reproduces file 81's own disassembly (`81:224-227`) independently, in a separate
file, on the same toolchain.

**And the in-loop case, which is where the rule's strength is decided.** Two identical
decode loops differing only in where the period, group stride, width and mask are written:

- `loop_from_assoc_const`: 236 instructions, **0** division instructions.
- `loop_from_const_fn`: 238 instructions, **3** division instructions surviving at `-O`.

So the loop body folded in both cases and the const-fn version still carries division
residue the associated-const version does not. The honest statement is not that a `const
fn` in value position always fails to fold; it is that whether it folds is an optimiser
heuristic rather than a language guarantee, and on this target and toolchain the heuristic
leaves work behind in both the standalone and the in-loop shape.

**The boundary**: a `const fn` whose body is a *projection* (a read of a value the trait
solver has already produced) folds reliably, because the body is a load of a constant. A
`const fn` whose body *computes* does not. File 79's constructions are on the safe side of
that line; file 81's original decoder was on the other.
