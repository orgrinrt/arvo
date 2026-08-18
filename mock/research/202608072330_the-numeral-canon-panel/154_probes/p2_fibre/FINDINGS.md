# P2 findings: the format does not determine the representation, and at one end of the range a primitive is not a type

Toolchain `nightly-2026-05-28`, host aarch64-apple-darwin. `fibre.rs` compiles (`rc=0`);
`fibre_refuted.rs` does not, deliberately, and `fibre_refuted.err` is its output. `sizes.out` is
`sizes.rs`'s output. All committed beside this file.

**Ad-hoc quick spike with no substance as a measurement.** The only numbers here are `size_of` and
byte arithmetic, which are exact rather than timed. Nothing is priced.

## The negative control, stated before the run

The dense instance had to satisfy the element-level signature and produce the right answer, so that
a failure at the packed instance is a fact about packing rather than about my trait. **Passed:**
`control_dense_works` (`fibre.rs:97-101`) compiles and `impl Numeral for Dense13` (`fibre.rs:75-80`)
is accepted.

## F5. One logical format, two shipped representations, in this repository

`bitpack-footprint-shared/src/lib.rs:92` declares `LOGICAL_BITS = 13`. The same buffer holds that
column twice: a dense region at `MAX_N * 2` bytes (`:109`, `PACKED_OFFSET = MAX_N * 2`) and a packed
region at `(MAX_N * 13) / 8 + 16` bytes (`:105`).

From `sizes.out`, at `MAX_N = 33_554_432` (`:101`): dense 67,108,864 bytes, packed 54,525,952 bytes,
**ratio 1.2308x**. And `bitpack-footprint-headtohead_n1048576_findings.md` measures the arms over
that pair on the harness, four variants, 40 samples each, reporting a 6.69x spread from
`bitpack-footprint-dense-alt` at 88,956.9 ns to `bitpack-footprint-packed-naive` at 594,734.2 ns.

**So the representation is a function of the pair, not of the format.** The product reading, on
which two strategies at one format share a representation, is refuted by a shipped, measured case in
the repository that the reading says cannot exist.

`holds for:` W = 13, unsigned, N = 33,554,432 for the footprint arithmetic and N in {16384, 65536,
1048576, 4194304} for the harness run, dense and bitpacked layouts, threads = 1, host
aarch64-apple-darwin, F = 0.

## F6. At the packed end, a primitive has no standalone size, and therefore is not a type

`fibre_refuted.err`:

```
error[E0080]: evaluation panicked: a packed 13-bit element does not occupy 13 bits as a standalone value
```

The most generous expressible encoding of "thirteen bits" as a Rust value, `[bool; 13]`, is **104
bits** (`sizes.out`), **8x** the logical width. A `u16` newtype is 16 bits, 1.23x, and is the dense
representation rather than the packed one. There is no third option: Rust's smallest addressable
value is a byte, and 13 is not a multiple of 8.

**The packed value exists only as a position in a column.** `Packed13Col::get` (`fibre.rs:53-66`)
is the only way to reach one, and it takes the column and an index. That is a lens, not a value.

This is the structural half of F5 and it is stronger than F5. F5 says the representation varies. F6
says that at one end of the variation **the thing being represented is not a value of any type**,
so an account of "primitive" as "a type whose values are numerals" does not cover the range arvo
declares it covers. I17 (`INTENTS.md:374-376`) forbids trading the packed end away:

> The intent is that the storage-minimising, aggressively bitpacked path is not deprioritised

so this is not an edge that can be dropped to make the account uniform.

`holds for:` W = 13, unsigned, standalone element encodings in {u16 newtype, [bool; 13]}, Rust
`nightly-2026-05-28`, target aarch64-apple-darwin, threads = 1, F = 0.

## F7. The signature that accepts both instances is over the column, not over the element

`fibre.rs` carries two signatures. `Numeral` (`:70-73`) is element-level: `Copy`, hence `Sized`,
hence unsatisfiable by a packed element per F6. `NumeralColumn` (`:120-124`) is column-level, and
**both instances implement it** (`:126-134` packed, `:136-144` dense), with one algorithm
`algo_sum_col` (`:148-157`) accepting both without naming either.

So the tier above (I11's "contracts for things that compose to bigger units than just numerals
alone") is reachable, and the arity at which it is reachable is the column. An element-level
contract is expressible and is **narrower than the range of representations arvo declares**.

This does not say arvo should have no element-level contract. It says an element-level contract is a
statement about a sub-range, and under the predicate discipline it has to say so.

`holds for:` W = 13, unsigned, dense and bitpacked layouts, one operation (sum), arity 1 over a
column, `nightly-2026-05-28`, target aarch64-apple-darwin, threads = 1, F = 0.

## What would refute each

F5: a demonstration that the dense and packed regions are not two representations of one format,
e.g. that they differ in value rather than in layout. `bitpack-footprint-shared/src/lib.rs:240-274`
(`check_size`) asserts they decode to the same logical values, so this is already closed against.
F6: a standalone Rust value occupying exactly 13 bits. Impossible on a byte-addressed target; I did
not test a target where it is not.
F7: an element-level signature satisfied by a packed element. That needs F6 overturned first.
