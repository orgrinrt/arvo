# 86_probes outcomes

Toolchain `rustc 1.98.0-nightly (57d06900f 2026-05-27)`, `aarch64-apple-darwin`, resolved from
`rust-toolchain.toml`, every command run inside the tree from this directory.

## probe_1_niche_vs_statement0.rs

```
rustc --edition 2021 --crate-type=lib --emit=metadata probe_1_niche_vs_statement0.rs
```

**COMPILES.** Every assertion is in const position, so the compile is the result. Establishes,
against file 84's biased-niche construction (probe_6's own model shape): the layout dividend
(`Option<NonZeroU16>` is 2 bytes against 4 for `Option<u16>`); no field width `w` in 0..=16 has
`2^w == 65535`, checked at every width, so the 65535-member domain is inexpressible by field
shrinking; `NonZeroU16::new(0)` is `None` (the exclusion is the type's declared validity range,
not a module invariant); the round trip is exact over all 65,535 domain members; and the additive
bias is monotone over the whole domain, so raw carrier order agrees with datum order.

## probe_2_rtie_is_pinned_quantise_plus_escape.rs

```
rustc --edition 2021 -O -o probe_2_bin probe_2_rtie_is_pinned_quantise_plus_escape.rs
./probe_2_bin
```

**RUNS, all assertions pass.** Output:

```
cells=4000 agree=3100 refusals=900 escapes=900
OK: rTIE = pinned quantise + exponent escape, cell for cell
```

At the r = 10, p = 3, e in -2..=1 model (file 80's shape, 4000 datums, exact i64 arithmetic):
roundToIntegralExact's value function equals `quantise::<0>`'s value function on every cell where
the pinned form delivers (3100 cells); the pinned form's 900 refusals are exactly, cell for cell,
the datums where rTIE's result integer requires a positive exponent to fit in p digits; rTIE is
total on the model (asserted per cell).

## probe_3_at_exponent_is_ordinary_vocab.rs

```
rustc --edition 2021 --crate-type=lib --emit=metadata probe_3_at_exponent_is_ordinary_vocab.rs
```

**COMPILES.** A model tower with the sealed `Pos` grammar, the sealed signed-exponent vocabulary
(`EZero | EPos<P> | ENeg<P>`), and an `ExponentForm` carrying a ranged member and a fixed member.
`At<M, Q>` reads `M`'s radix and precision through and picks `Fixed<Q>`; `At<Dec3, EZero>` and
`At<Dec3, ENeg<H>>` both satisfy the ordinary `Numeral` bound, asserted in const position, and the
typed `quantise<M, Q>` signature typechecks against them. Closes file 85's open item 4 at model
level: the fixed exponent is a literal instance of the sealed vocabulary, minting nothing.
