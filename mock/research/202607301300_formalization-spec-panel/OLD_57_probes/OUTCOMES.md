# Probe outcomes, file 57

Everything below was built fresh for this dispatch, on `rustc 1.98.0-nightly (57d06900f
2026-05-27)`, host `aarch64-apple-darwin`, resolved via `rustc +nightly-2026-05-28`, run from
inside the repo tree so `rust-toolchain.toml`'s pin resolves for a bare `rustc` too (checked both
ways, see `flag_audit/` below). The bench artifacts under `mock/benches/` are separate and are
built and run via `cargo build --offline --release` / the `arvo-benches` orchestrator binary from
`mock/`, not `rustc` directly; their commands are in the main file's section 1.

## Codegen-flag audit (`flag_audit/`)

Two of file 25's uncited instruction-sequence claims, reproduced fresh and swept across
`-C codegen-units=1` against the rustc default (`codegen-units=16`, unstated by the flag), both
identical asm both ways:

```
rustc +nightly-2026-05-28 --edition 2021 -C opt-level=3 [-C codegen-units=1] --crate-type=cdylib -C panic=abort --emit asm -o out.s flag_audit/mul_full.rs
rustc +nightly-2026-05-28 --edition 2021 -C opt-level=3 [-C codegen-units=1] --crate-type=cdylib -C panic=abort --emit asm -o out.s flag_audit/mac_fold.rs
```

`probe_mul_full_2_2`, both flag settings, byte-identical `diff`:

```
_probe_mul_full_2_2:
    umulh	x8, x2, x0
    madd	x8, x2, x1, x8
    madd	x1, x3, x0, x8
    mul	x0, x2, x0
    ret
```

Matches `25_xu_building_the_exact_product.md:395` verbatim ("the standard `umulh`/`madd`/`madd`/`mul`
sequence").

`probe_mac_256`, both flag settings, byte-identical `diff`: four `adds`/`adc` pairs (`x8,x1,x8` /
`x9,x0,x9` / `x10,x3,x10` / `x11,x2,x11` / `x12,x5,x12` / `x13,x4,x13` / `x14,x7,x14` / `x15,x6,x15`,
folded pairwise down to `x0`/`x1`). Matches `25:273` ("four lanes of `adds`/`adc` pairs").

Both claims **survive** the flag sweep and are now grounded on `pin + host + flags` (both flag
values checked), not only `pin + host`.

`flag_audit/union_reconstruction/`: an attempt to reproduce `08_probes/e_codegen.rs`'s five-shape
instruction table (`08_fog_the_union_and_what_it_costs.md:225-233`). `e_codegen.rs` reads
`use union::*;`; the only committed candidate for that module, `08_probes/a_union.rs`, itself
declares `pub mod spare;` and `pub mod fusion;` at lines 720-721, and neither `spare.rs` nor
`fusion.rs` exists anywhere in `08_probes/` or elsewhere in the panel directory (`find . -iname
"spare.rs" -o -iname "fusion.rs"` from the panel root, empty). Compiling `a_union.rs` as a
`--crate-name=union` lib fails with `E0583` on both missing modules:

```
error[E0583]: file not found for module `spare`
error[E0583]: file not found for module `fusion`
```

This table is **not reproducible from the committed audit trail**. Recorded in the main file
section 2 rather than left silent; it is not a claim that the table is wrong, only that nobody
including this dispatch can currently rebuild the artifact it was measured from.

## Licence-leak regression pair (`codegen/licence_leak.rs` + `codegen_regression_licence_leak.rs`)

```
rustc +nightly-2026-05-28 --edition 2021 --crate-type lib -C opt-level=3 [-C codegen-units=1] -C panic=abort --emit asm -o out.s codegen/licence_leak.rs
```

Byte-identical `diff` between `-C codegen-units=1` and the rustc default, both functions:

```
_fold_interior_safe:
    ldp	q1, q0, [x0]
    fadd.4s	v0, v1, v0
    faddp.4s	v0, v0, v0
    faddp.2s	s0, v0
    ret
_fold_compensated_step:
    fadd	s2, s0, s1
    fsub	s0, s2, s0
    fsub	s0, s0, s1
    ret
```

`fold_interior_safe` vectorises (NEON `.4s`/`.2s`), reproducing `51_probes/probe_4`'s
`sum_algebraic` finding on an independent 8-element fixture. `fold_compensated_step` stays scalar,
unfused, exactly one `fadd` and two `fsub`, reproducing `51_probes/probe_6`'s `kahan_step_strict`
finding. The pair is flag-insensitive on this pin (unlike file 52's own test 4, the
assert-equal-length idiom, which is `codegen-units`-sensitive).

The harness (`codegen_regression_licence_leak.rs`) was compiled and run standalone with `rustc
--test` and passed (`1 passed; 0 failed`), confirmed both with `RUSTC` resolving the pin's absolute
binary path explicitly and with bare `rustc` from inside the repo tree (both resolve to the same
`1.98.0-nightly (57d06900f 2026-05-27)`, because `rust-toolchain.toml` lives at the repo root and
rustup walks up from cwd to find it). A separate trap surfaced on the first attempt and is worth
naming since it is a NEW one, not file 52's (`52_probes/OUTCOMES.md` records bare `rustc` resolving
to stable *outside* the repo tree; this is different): `RUSTC="rustc +nightly-2026-05-28"` fails,
because `Command::new` treats the whole value as one literal binary name with a space in it and
never finds it (`spawn rustc: Os { code: 2, kind: NotFound }`). `RUSTC` has to name a bare
executable path (`rustup which rustc --toolchain nightly-2026-05-28`), not a `rustc`-plus-flag
invocation string; the fix here was the same class of mistake file 52's own toolchain-resolution
note warns about (an implicit assumption about how a shelled-out command resolves), just a
different instance of it.

## Bench correctness cross-check

`mock/benches/variants/quantiser-fadd-shared/src/lib.rs`, `#[cfg(test)] mod tests`: exhaustive
byte-for-byte comparison of `software_add` against native `+`, over the exact input distribution
`AddSweep<PCT>::build_input` generates for all six swept PCT values and 64 seeds each,
`6 * 64 * 256 = 98,304` operations, 0 mismatches. `cargo test --offline -p
bench-quantiser-fadd-shared --release`: `1 passed; 0 failed`.
