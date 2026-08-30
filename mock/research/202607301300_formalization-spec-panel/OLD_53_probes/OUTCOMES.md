# 53_probes outcomes

Toolchain for every build: `rustc 1.98.0-nightly (57d06900f 2026-05-27)`, host
`aarch64-apple-darwin`, resolved from the repo pin. Verified from this directory before any
run: `rustc --version` and `rustc +nightly-2026-05-28 --version` print the identical line
(the file-52 gotcha, a bare `rustc` resolving to stable, applies only outside the repo tree;
this directory is inside it).

## Gates, reproduced fresh for this dispatch

- Test gate: `cargo test --workspace` from `mock/`, summed per binary with
  `grep "^test result" | awk`: 654 passed, 0 failed, 122 result lines. Matches every file
  since 41.
- Canon gate: `grep -rln "Adjustment\|Bias\|Numeral" mock/crates/ --include="*.rs"` and the
  same with `FullRange\|UTerm\|AddWidth`, both from the repo root, both exit 1 (empty). The
  design surface has no shipped source.
- The `49:701` claim ("the shipped `IFixed` already computes width from precision-shaped
  parameters at its declaration site") reproduces:
  `mock/crates/arvo/src/ifixed.rs:8` ("`repr(transparent)` newtype over
  `Bits<{1+I+F}, S, Signed>`"), declaration at `ifixed.rs:43`.

## The sweep

`price/gen.py` + `price/sweep.sh` + `price/results.csv`. Tower: the sealed
`vu_bias_sealed.rs`/`vu_nat_sealed.rs` copied unmodified from `42_probes/` (the same copies
files 46, 50 and 52 carried). Build shape identical to the 36/41/42 sweeps:
`rustc --edition 2021 --crate-type lib --emit=metadata`, trait-solve-only, no codegen.
Min-of-2 per point. Every instantiation forced by a `const` assertion against a
Python-computed reduced `Fraction`, so nothing is elided and correctness is checked at the
same time as cost.

Results (`price/results.csv`, ms_min column):

| kind | count | ms | marginal ms/item |
|---|---|---|---|
| dyadic | 0 | 69 | (baseline) |
| dyadic | 25 | 137 | 2.7 |
| dyadic | 50 | 244 | 3.5 |
| dyadic | 100 | 302 | 2.3 |
| dyadic | 200 | 482 | **2.1** |
| distinct16 | 5 | 722 | 131 |
| distinct16 | 50 | 6,922 | 137 |
| distinct16 | 100 | 14,335 | **143** |
| repeat16 (100 sites / 5 distinct) | 100 | 3,361 | 28/site past the first five |
| headline (44100, 48000, 4096) | 3 | 84 | **~5 for all three** |
| chained (1/32768 x 48000/44100) | 1 | 97 | ~30 |

Cross-checks against prior committed CSVs, read not re-run: `41_probes/price/results.csv`
(bias_full, 16-bit, unsealed tower) gives 63,665 ms at 400 compositions, slope ~159
ms/composition, linear across 25/50/100/200/400. `42_probes/price/results.csv` (8-bit)
gives ~15.5 ms/composition, linear to 400, with `alias` (6,474 ms at 400) and
`alias_sealed` (6,431 ms) indistinguishable: the seal's compile cost is zero at scale, not
only at the single-composition grain file 46 measured.

## What each number establishes

- **Linearity holds at every width probed.** No superlinear term, no cliff, out to 400
  compositions (41's CSV) and 200 (this sweep). The aggregate is a multiplication, and the
  multiplication is honest.
- **The dyadic slope at scale is ~2.1 ms/composition on the sealed tower** (49:327-335
  quotes 1.55 for the magnitude-comparable case; same band, and this figure is the full
  sign-plus-magnitude composition, sealed, min-of-2).
- **The 16-bit random slope is ~143 ms/composition sealed** (159 unsealed per 41's CSV;
  difference is noise-or-better, not a seal debit).
- **Repeated instantiation of the same composition is not free: ~28 ms/site at 16-bit.**
  The solver's caching pays roughly 5x, not infinity. An aggregate model needs both terms:
  per-distinct-composition and per-additional-site.
- **The design's own headline constants are in the cheap band, not the expensive one.**
  `div_exact` by 44100/48000/4096 over a Q0.15 quantum has product numerator 1, the gcd is
  trivial, and all three compositions together cost ~5 ms. The expensive band (~143
  ms/composition) is random rational-times-rational at 16-bit magnitudes, which no named
  consumer shape produces.
- **One chained sample-rate conversion (both magnitudes large, sharing factors, a real
  Stein gcd) costs ~30 ms, once per distinct conversion.**
