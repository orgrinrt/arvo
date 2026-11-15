# Layout::Bitpacked reading, sequential column sum: byte-aligned slot vs zero-inter-value-padding

3 variants, 40 samples per variant.
Baseline: **bitpack-aligned-seq**

## Highlights

Baseline for all deltas below: **bitpack-aligned-seq**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (bitpack-aligned-seq) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline bitpack-aligned-seq has the worst median (662 ns). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest bitpack-native-seq at 260 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### bitpack-native-seq dominates: 142% faster than the next best (bitpack-zeropad-seq)

bitpack-native-seq (260 ns) leads bitpack-zeropad-seq (630 ns) by 142%, a clear separation rather than a photo finish. CV 14.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### bitpack-native-seq beats baseline by 66% (significant)

bitpack-native-seq is -436 ns (66%) faster than baseline bitpack-aligned-seq, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### bitpack-aligned-seq is an outlier: 2.5x slower than the field

bitpack-aligned-seq (662 ns) is 2.5x the fastest (260 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### bitpack-aligned-seq is inconsistent: worst-20% is 1.6x its best-20%

bitpack-aligned-seq's best 20% of batches run at 506 ns but its worst 20% at 831 ns (1.6x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

### bitpack-zeropad-seq's edge over baseline is significant but tiny (12 ns, 1.87%)

bitpack-zeropad-seq differs from baseline bitpack-aligned-seq by 12 ns (1.87%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: bitpack-native-seq** at 260.5 ns median (-60.6% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 2.54x (fastest 260.5 ns, slowest 661.7 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bitpack-aligned-seq | 947ns | 962ns | 718ns | 941ns | 1195ns | base |
| bitpack-native-seq | 580ns | 544ns | 395ns | 531ns | 911ns | -38.80% |
| bitpack-zeropad-seq | 915ns | 856ns | 775ns | 887ns | 1142ns | -3.36% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-aligned-seq | 659ns | 506ns | 831ns | base | 0.389 |
| bitpack-native-seq | 245ns | 190ns | 296ns | -62.78% | 1.044 |
| bitpack-zeropad-seq | 664ns | 569ns | 836ns | +0.80% | 0.386 |

## Performance model

- Peak throughput: **1.346 Gops/s** (bitpack-native-seq; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-aligned-seq | 0.387 | 28.7% |
| bitpack-native-seq | 0.983 | 73.0% |
| bitpack-zeropad-seq | 0.406 | 30.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-aligned-seq | 947ns | 947ns | base |
| bitpack-native-seq | 580ns | 580ns | -38.80% |
| bitpack-zeropad-seq | 915ns | 915ns | -3.36% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-aligned-seq | 662ns | base | --- | [604, 681] | --- | --- | --- | --- |
| bitpack-native-seq | 260ns | -436.3ns (-65.9%) | [-489, -362]ns | [233, 262] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-zeropad-seq | 630ns | +12.4ns (+1.9%) | [+2, +29]ns | [587, 639] | YES (adj: no) | 0.0807 | 0.0807 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bitpack-aligned-seq | bitpack-native-seq | bitpack-zeropad-seq |
|---|---|---|---|---|
| 1 | 1 | 604ns | -61.4% | +3.7% |
| 1 | 2 | 495ns | -46.7% | +15.8% |
| 1 | 3 | 777ns | -66.2% | +1.2% |
| 1 | 4 | 680ns | -72.0% | -7.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-aligned-seq | -0.215 | moderate- |
| bitpack-native-seq | -0.178 | ok |
| bitpack-zeropad-seq | -0.471 | moderate- |

**Consistency summary:**

- **bitpack-native-seq**: won 4/4, lost 0/4
- **bitpack-zeropad-seq**: won 1/4, lost 3/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-aligned-seq | 132.0ns | 658.8ns | 20.0% | HIGH |
| bitpack-native-seq | 104.5ns | 245.2ns | 42.6% | HIGH |
| bitpack-zeropad-seq | 85.6ns | 664.1ns | 12.9% | HIGH |

## Distribution (algo ns)

```
bitpack-aligned-seq (n=40, range 506.4-830.8 ns)
    506.4 |
    522.6 |
    538.8 |####################
    555.1 |#####
    571.3 |
    587.5 |
    603.7 |########################################
    620.0 |
    636.2 |#####
    652.4 |
    668.6 |########################################
    684.8 |
    701.1 |
    717.3 |
    733.5 |#####
    749.7 |
    766.0 |###################################
    782.2 |
    798.4 |
    814.6 |
  (6 below, 4 above range)

bitpack-native-seq (n=40, range 190.2-295.9 ns)
    190.2 |#####################
    195.4 |
    200.7 |
    206.0 |
    211.3 |
    216.6 |
    221.9 |
    227.2 |###
    232.5 |##################
    237.7 |
    243.0 |
    248.3 |######
    253.6 |
    258.9 |########################################
    264.2 |
    269.5 |
    274.8 |###
    280.0 |
    285.3 |
    290.6 |
  (3 below, 7 above range)

bitpack-zeropad-seq (n=40, range 569.1-835.5 ns)
    569.1 |########################################
    582.4 |
    595.7 |###
    609.1 |
    622.4 |####################################
    635.7 |#######
    649.0 |###
    662.4 |
    675.7 |
    689.0 |
    702.3 |###
    715.6 |
    729.0 |
    742.3 |
    755.6 |
    768.9 |###
    782.3 |#########################
    795.6 |
    808.9 |
    822.2 |
  (3 below, 3 above range)

```

## Diagnostics

- **bitpack-aligned-seq**: bridge=14.6% of algo (FFI overhead may distort results)
- **bitpack-native-seq**: bridge=39.7% of algo (FFI overhead may distort results)
- **bitpack-zeropad-seq**: bridge=13.1% of algo (FFI overhead may distort results)
