# 240 probes: the build lines, the exit codes, and the controls

Toolchain `nightly-2026-05-28`, `rustc 1.98.0-nightly (57d06900f 2026-05-27)`, the
repository's own pin. Every command below was run from the mock workspace root,
`mock/`, unless the line says otherwise. Sources and outputs are committed beside
this file.

Two of the five link the shipped crate and read its real associated items. Three
are models of the coordinate space, because the sweeps vary things that are const
parameters and associated types over in the crate and cannot be varied from
outside it. The bridge between the two halves is `q1`, whose enumerator is the
same formula the models use and which agreed with the crate's own predicate on all
ten of its control rows.

## The shipped crate, first

```
cargo build -p arvo-format
ls target/debug/deps/libarvo_format-*.rlib
  target/debug/deps/libarvo_format-383d063a65822acd.rlib
```

## q1, the denotation against the shipped laws

Links the crate.

```
rustc --edition 2024 -O \
  --extern arvo_format=target/debug/deps/libarvo_format-383d063a65822acd.rlib \
  -L target/debug/deps \
  research/202608072330_the-numeral-canon-panel/240_probes/q1_the_denotation_against_the_shipped_laws.rs \
  -o /tmp/q1
/tmp/q1            # exit 1, two findings
```

One warning at build, an unused import of `Signed`, left rather than swept so the
build line reproduces exactly.

Controls: ten rows whose phase is zero or odd over two must agree, and all ten do.
Section 3 additionally requires the set comparison to call one pair different, and
it does. Output `q1_output.txt`.

## q2, the two repairs, swept

Model.

```
cd research/202608072330_the-numeral-canon-panel/240_probes
rustc --edition 2024 -O q2_the_two_repairs_swept.rs -o /tmp/q2
/tmp/q2            # exit 0, both repairs hold
```

Controls: the oracle must answer both ways over the pool, the shipped predicate
must be caught wrong somewhere, the equality oracle must call some pairs equal and
some unequal, and the offset-dropping mutant must be caught. All four fire: 8175
against 15625, 7375 caught, 1206 against 141678, 19698 caught. Output
`q2_output.txt`.

## q3, which value sets a format can denote

Model.

```
rustc --edition 2024 -O q3_what_sets_a_format_can_denote.rs -o /tmp/q3
/tmp/q3            # exit 1, two findings
```

Controls: section 1 must call something equal and something unequal, and does;
section 3's float control must read as one ladder of ratio two and its hand-built
taper must not, and both fire; section 4's float target must be reached by the
search, and it is, at `(radix 2, slope 1, magnitudes 3, slots [-3, 3])`.

**Two earlier runs are kept rather than discarded**, because the second refuted a
claim I had written into the first.

- `q3_output.v1_integer_ratios_only.txt`. The first instrument took ratios between
  consecutive gaps and reported a sentinel wherever the ratio was below one, which
  is half of them, so its profiles were unreadable while its verdict happened to
  be the same.
- `q3_output.v2_ladder_claim_refuted.txt`. The second replaced that with the
  distinct-gap ladder and **refuted my own reach claim**: 93 of 240 tuples denote
  a set that is not one geometric ladder. Section 4 was written because of that
  refutation, and it is what establishes the narrower true statement.

Current output `q3_output.txt`.

## q4, the obligation the format does not carry

Model.

```
rustc --edition 2024 -O q4_the_obligation_the_format_does_not_carry.rs -o /tmp/q4
/tmp/q4            # exit 0, the obligation holds
```

Controls: both verdicts must be present in the sweep, and are, 7352 against 9376.
A mutant admitting everything must be caught, and is, on 9376 tuples. A mutant
checking only the top join must be caught, and is, on 1264 tuples, with the first
such tuple printed. Output `q4_output.txt`.

## q5, what the adaptation is a map onto

Links the crate.

```
rustc --edition 2024 -O \
  --extern arvo_format=target/debug/deps/libarvo_format-383d063a65822acd.rlib \
  -L target/debug/deps \
  research/202608072330_the-numeral-canon-panel/240_probes/q5_the_adaptations_codomain.rs \
  -o /tmp/q5
/tmp/q5            # exit 1, one finding
```

Controls: every one-magnitude format must come out with a slot ambiguity of
exactly one, and all four do; at least one format must come out above one, and
three do, at 3, 5 and 8. Output `q5_output.txt`.

The claim that `apply.rs` never reads the magnitude is a grep rather than an
assertion inside the probe:

```
grep -c 'magnitude' crates/arvo-format/src/apply.rs      # 2
grep -c 'MAGNITUDES' crates/arvo-format/src/apply.rs     # 0
```

Both occurrences of the word are in doc comments, at `apply.rs:15` and
`apply.rs:26`.

## The suite, before any of this

`cargo mock test` over the whole tree: **red, and honestly so.** One of nine trees
fails, the bench tree, on four variants whose manifests inherit an `arvo`
dependency from a workspace root that no longer has one.

```
FAIL  variants/fnv1a/Cargo.toml
FAIL  variants/spectral-bisection/Cargo.toml
FAIL  variants/structural-decomposition/Cargo.toml
FAIL  variants/xxhash3/Cargo.toml
mock test: 1 of 9 tree(s) failed
```

The workspace members are green: `arvo-format` 51 passed plus 2 compile-fail,
`arvo-placement` 18 passed and 1 ignored, `arvo-strategy` 10 passed. The lint pack
is 603 passed and 13 ignored.
