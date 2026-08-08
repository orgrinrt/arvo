# 45's probes: what each one checks, and how to rerun it

All Rust probes: `nightly-2026-05-28`, `rustc 1.98.0-nightly (57d06900f 2026-05-27)`, zero
feature gates (`grep -c '^#!\[feature' *.rs` returns 0 on every file). All Python probes:
Python 3.14.6, standard library only (`fractions` in `p4`).

## p1_wide_rung_collision.rs

Does the pair `(declared width, stride)` determine the carrier, once the wide rung
(`W > 128`) is included, with zero dependence on `Precise` or on sign? Sweeps `W =
129..=768`, finds 40 of 640 widths where Hot's align-16 padding is a no-op (Warm's natural
byte count is already a multiple of 16), and const-checks one witness (`W = 256`) where
Hot's and Warm's carriers have identical size and stride but different alignment, hence
different types.

```
rustc +nightly-2026-05-28 --edition 2021 -O p1_wide_rung_collision.rs -o bin/p1 && ./bin/p1
```

## p2_p5_style_instrument_is_blind.rs

Reproduces `16_probes/p5_recovery_direction.rs`'s own carrier representation (a bare `u32`
bit count, matching its `native`/`carrier_bits`/`storage_bits` functions exactly) over the
SAME wide-rung domain p1 swept, and shows it reports zero collisions where p1's type-aware
version found 40. Establishes that p5's "0 of 251, extent -> carrier is a function absent
Precise" claim is an artifact of its own carrier representation, not a fact about the
domain.

```
rustc +nightly-2026-05-28 --edition 2021 -O p2_p5_style_instrument_is_blind.rs -o bin/p2 && ./bin/p2
```

## p3_search_pigeonhole_witness.py

Exhaustive search, F = 3 through 6, for a chain `x -> x*a -> (x*a)*b` where two distinct
inputs `x1 != x2` round to the SAME intermediate value under F-bit rounding after the first
multiply, but whose true once-truncated exact chain answers (`x1*a*b` and `x2*a*b`, each
rounded once at the very end) differ. Two independent rounding rules (round-half-up,
round-half-to-even) both find such witnesses at every F tested, at counts growing with F
(61, 732, 7354, 73461).

```
python3 p3_search_pigeonhole_witness.py
```

## p4_fraction_crosscheck_and_widening_recovers.py

A second, independently-coded instrument for the same question, using `fractions.Fraction`
throughout rather than integer-division tricks. Finds the SAME disagreement counts as p3's
round-half-up rule at every F (61, 732, 7354, 73461, an exact cross-check), and additionally
checks that the widened computation (never rounding the intermediate, rounding only once at
the very end) matches the once-truncated exact reference on every single disagreement found
across all four F values (thousands of cases, zero exceptions).

```
python3 p4_fraction_crosscheck_and_widening_recovers.py
```

## p5_third_output_is_mechanically_free.rs

Extends `16_probes/p6_trait_form_recovers_both.rs`'s trait-based derivation with a third
associated const, a compute carrier distinct from the storage carrier. Compiles gate-free
under BOTH readings of Precise (does not widen: compute carrier equals storage carrier for
every strategy; widens: only Precise's impl block changes). Checks that the type-system side
of the Precise question is not what is undetermined: the mechanism to express three outputs,
with the same arity across all four strategies, is available either way, at the cost of one
impl block.

```
rustc +nightly-2026-05-28 --edition 2021 -O p5_third_output_is_mechanically_free.rs -o bin/p5_notwiden && ./bin/p5_notwiden
rustc +nightly-2026-05-28 --edition 2021 --cfg precise_widens -O p5_third_output_is_mechanically_free.rs -o bin/p5_widens && ./bin/p5_widens
```
