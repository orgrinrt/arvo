# Review pass 1, arvo-bits-tier

Date: 2026-11-20
Reviewed: `work/arvo-bits-tier` at `14d964bd` against `origin/dev` at `d15dd473`, by `pr-reviewer-senior`. The findings are taken up by the round that follows this file on the branch.

## Verdict

MERGE AFTER FIXES. The operations are correct. The test gate fails (findings 3 to 5), the generated `docs/STRUCTURE*` files are stale (finding 1), and the refusal has a hole (finding 2). Nothing lands until 1 to 5 are fixed.

Gates. The canon gate passed. I checked against `ruling::every_family_of_the_old_arvo_returns_and_the_shape_is_open` (ratified by op; the shape and the taxonomy are left open), `obligation::set_operations_over_a_fixed_size_bit_set`, `ruling::never_a_runtime_check_and_one_lowered_path` (ratified) and `proposal::the_old_bit_contracts_crate_is_gone` (one expert, binds nothing). No `open`-rung row or open question reserves where the bit contracts live. I am the first read on the placement question, so a second independent read is owed.

## Findings

1. `docs/STRUCTURE.md:42-50`, `docs/STRUCTURE.GRAPH.{dot,svg,png}`, public surface, blocker. Not regenerated: they still put `arvo-bits` at depth 0 with no dependencies. `cargo mock` on a clean clone at `14d964bd` moves it to depth 1 and adds the edge `arvo_format -> arvo_bits`. Fix: regenerate and commit. Separately, the generator's "Public API" lists only `Bits`, not `Bytes` (already missing on dev) or the four traits. That is a mockspace ticket, not this branch.

2. `src/tier.rs:79`, correctness, high. `const WIDTH: Width = Width::bits(N);` never forces `Bits::<N>::ADMITTED`. `<Bits<0> as BitWidth>::WIDTH` and `<Bits<65> as BitWidth>::WIDTH` both pass `cargo check` and `cargo build` and run with exit 0. That contradicts DESIGN:211, "`N` outside `1..=64` is refused". Fix: `{ let () = Self::ADMITTED; Width::bits(N) }`, with compile_fail cases for `WIDTH` at 0 and at 65.

3. `tests/compile_fail.rs` and every `tests/ui/*.rs`, tests, high. trybuild runs `cargo check` when a project has no pass case (`trybuild-1.0.120/src/cargo.rs:97,130,153`), so every refusal is pinned only inside a `const` item, which check refuses. The path DESIGN:133 claims, a refusal at `cargo build` for a call at run time, is never pinned in the crate. Probe: `Bits::<8>::masked(x).shift_down::<8>()` in `main`, and `rotate_up::<8>` through `fn f<B: BitSequence>`, both pass `cargo check` (exit 0) and fail `cargo build` (exit 101). `shift_down` and `rotate_up` also have no refusal case at all: with their `let () = Within::<N, K>::ADMITTED;` deleted, `shift_down::<8>` on `Bits<8>` returns 0 and `rotate_up::<8>` returns the value unchanged, and nothing fails. Fix: one `TestCases` holding the refusals and a `t.pass(..)` case, so trybuild builds; cases for a call at run time, inherent and generic; every one of the six methods that take an index or an amount.

4. `tests/common/mod.rs:70`, `the_bit_access.rs`, `the_bit_sequence.rs`, tests, high. They sweep 13 of the 64 admitted widths, a sampled law, where DESIGN:222-224 says "at the widths above". `the_bit_logic.rs:93-95` already covers 1 to 64 with a macro, so this was a choice and not a limit. Fix: every width from 1 to 64 at every index or amount, by a macro or a generated test file.

5. `tests/the_bit_tier_in_a_constant.rs:44`, tests, medium. `width_of::<Bits<N>>() == Width::bits(N)` restates the implementation's one line and passes at `N = 0`. Delete it, and put an independent law in its place, `Bits::<N>::masked(u64::MAX).raw().count_ones() == WIDTH.count()`, with the compile_fail cases of finding 2.

6. `tests/the_bit_access.rs:42`, tests, medium. The oracle for "every other bit as it was" is the implementation's own formula (`src/ops.rs:47-48`). Fix: compare bit by bit, `j == I ? to : bit j of v`, the way `common::rotated_up` already does for rotation.

7. `src/tier.rs:27-30,59-64`, DESIGN:67-68,100-101, drift, medium. The traits promise "refused at build" for every implementor, nothing in a trait enforces it, and the design expects implementors outside the crate (an array of `Bits<64>`). The argument for keeping `ADMITTED` private, that there is no outside implementor, no longer holds. Fix: state it as an obligation the implementor owes, as `Slots::ADMITTED` does. `BitSequence` also speaks of "the width" without `BitWidth` as a supertrait; add it or say why not.

8. `src/ops.rs:24-27,46,78,95,120`, `Cargo.toml:120-123`, spec, low. The five mutant flags are in the source and not in the design. `arvo-numeral` and `arvo-capacity` name theirs in their designs. Fix: a sentence under the laws.

9. The mutant sketch's `outputs/*.txt`, tests, low. `run.sh:18` filters out the `panicked at` lines, so the record cannot show which assertion each mutant failed. Fix: keep those lines.

10. Drift, low.
    - `mock/benches/bit-set/arms/bit-set-tier/src/lib.rs:5` and the lowering note's line 11 say all three operations go through a generic function. Only `meet` does; `union` and `is_empty` are fully qualified calls on `Bits<64>`.
    - `mock/DESIGN.md.tmpl` says "on `arvo-format` alone", and notko is also a dependency.
    - The changelists name the round `202611200155`; its `.meta` says `202611200157`.
    - The crate README is 32 lines against the rule's 3 to 10 (it was 26 on dev).

## Verified

- All eleven operations are correct at `N = 1` and `N = 64`, at index or amount 0 and `N - 1`. The rotation below 64 bits never shifts by 64.
- The bench evidence is what it says: the six disassembly files show `test`/`setne`, `mov`/`or` and `test`/`sete`, the same in both arms. The note says no timing was run. Its predicate (`N = 64`, x86-64, release profile, across a C boundary) is right and leaves out the widths below 64. The host `u64` arm is the right competitor.
- The mutant outputs match the findings file: four red runs at exit 101, and the index cases exit 101 as shipped and 0 under the flag.
- `cargo mock test -p arvo-bits --full --no-cache` at `14d964bd`: 32 of 32 passed.
- Commit messages, read one at a time: no bylines, adverts or workflow leakage. The lockfile and `Cargo.toml` changes are consistent.
- No em-dashes and no bold or italic in the public templates or rustdoc. No new bare primitive in a public position beyond const generic parameters and the existing `raw` door.
- Placement: it conflicts with neither the canon, `a-crate-holds-only-its-own-thing` nor `extract-the-shared-logic-never-mirror-it`. `arvo-bridge-home-rule` covers const mirrors of std traits, not these, and still names the dead `arvo-storage`.

## Notes

The topic cites the proposal's `instead` ("Bit access on `arvo-bits`") and not its `because`, which says `BitLogic` and `BitSequence` "return with the masks over `arvo-bits`". It also leans on stack-coord's call, which is a coordinator's call and not a ratification. The placement stands on the ratified shape-open ruling alone, and the topic should say so. The reproductions in findings 2 and 3 are one line each and belong in the compile_fail cases.
