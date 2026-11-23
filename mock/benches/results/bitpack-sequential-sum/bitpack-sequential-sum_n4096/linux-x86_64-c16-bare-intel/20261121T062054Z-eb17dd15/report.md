# Layout::Bitpacked reading, sequential column sum: byte-aligned slot vs zero-inter-value-padding

3 variants, 40 samples per variant.
Baseline: **bitpack-aligned-seq**

## Highlights

Baseline for all deltas below: **bitpack-aligned-seq**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (bitpack-aligned-seq) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline bitpack-aligned-seq has the worst median (2.75 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest bitpack-zeropad-seq at 2.15 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### bitpack-zeropad-seq dominates: 17% faster than the next best (bitpack-native-seq)

bitpack-zeropad-seq (2.15 us) leads bitpack-native-seq (2.52 us) by 17%, a clear separation rather than a photo finish. CV 130.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### bitpack-zeropad-seq is fastest but the noisiest (CV 130.7%)

bitpack-zeropad-seq wins on median (2.15 us) yet has the highest variance (CV 130.7%), while bitpack-native-seq is the steadiest (CV 22.5%, 2.52 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (604 ns) is smaller than the fastest variant's own run-to-run std-dev (2.81 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### bitpack-zeropad-seq is inconsistent: worst-20% is 4.1x its best-20%

bitpack-zeropad-seq's best 20% of batches run at 2.14 us but its worst 20% at 8.80 us (4.1x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: bitpack-zeropad-seq** at 2149.9 ns median (-21.9% vs baseline)
- 1 variant significantly slower than baseline
- Spread: 1.28x (fastest 2149.9 ns, slowest 2754.4 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Layout::Bitpacked reading, sequential column sum: byte-aligned slot vs zero-inter-value-padding |
| Passes | 4 |
| Runs per pass | 1000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x123456789abcdef0 |
| Counter frequency | 2112768523 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bitpack-aligned-seq | 4273ns | 2830ns | 1924ns | 3865ns | 7847ns | base |
| bitpack-native-seq | 2818ns | 2728ns | 2071ns | 2774ns | 3695ns | -34.06% |
| bitpack-zeropad-seq | 4164ns | 2202ns | 2195ns | 3188ns | 9060ns | -2.57% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-aligned-seq | 4150ns | 1873ns | 7603ns | base | 0.987 |
| bitpack-native-seq | 2596ns | 1916ns | 3384ns | -37.45% | 1.578 |
| bitpack-zeropad-seq | 4056ns | 2144ns | 8802ns | -2.26% | 1.010 |

## Performance model

- Peak throughput: **2.187 Gops/s** (bitpack-aligned-seq; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-aligned-seq | 1.487 | 68.0% |
| bitpack-native-seq | 1.623 | 74.2% |
| bitpack-zeropad-seq | 1.905 | 87.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-aligned-seq | 4273ns | 4273ns | base |
| bitpack-native-seq | 2818ns | 2818ns | -34.06% |
| bitpack-zeropad-seq | 4164ns | 4164ns | -2.57% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-aligned-seq | 2754ns | base | --- | [1873, 5886] | --- | --- | --- | --- |
| bitpack-native-seq | 2524ns | no significant difference | [-3491, +278]ns | [2323, 2797] | no | 0.6358 | 0.6358 | 0 |
| bitpack-zeropad-seq | 2150ns | +271.5ns (+9.9%) | [+271, +280]ns | [2144, 2809] | YES | 0.0044 | 0.0022 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bitpack-aligned-seq | bitpack-native-seq | bitpack-zeropad-seq |
|---|---|---|---|---|
| 1 | 1 | 1878ns | +57.4% | +14.4% |
| 1 | 2 | 3234ns | -4.9% | -33.6% |
| 1 | 3 | 3964ns | -45.6% | -45.8% |
| 1 | 4 | 5205ns | -55.3% | -46.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-aligned-seq | 0.186 | ok |
| bitpack-native-seq | 0.124 | ok |
| bitpack-zeropad-seq | -0.082 | ok |

**Consistency summary:**

- **bitpack-native-seq**: won 3/4, lost 1/4
- **bitpack-zeropad-seq**: won 3/4, lost 1/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-aligned-seq | 41.6ns | 4150.3ns | 1.0% |  |
| bitpack-native-seq | 78.7ns | 2595.9ns | 3.0% |  |
| bitpack-zeropad-seq | 36.0ns | 4056.5ns | 0.9% |  |

## Distribution (algo ns)

```
bitpack-aligned-seq (n=40, range 1872.5-7603.0 ns)
   1872.5 |########################################
   2159.1 |###
   2445.6 |
   2732.1 |
   3018.6 |###
   3305.2 |
   3591.7 |
   3878.2 |
   4164.7 |###
   4451.3 |
   4737.8 |###
   5024.3 |
   5310.8 |###
   5597.4 |######
   5883.9 |#########
   6170.4 |######
   6456.9 |
   6743.5 |###
   7030.0 |#########
   7316.5 |######
  (6 below, 3 above range)

bitpack-native-seq (n=40, range 1915.9-3383.8 ns)
   1915.9 |#####
   1989.3 |#####
   2062.7 |######################
   2136.1 |###########
   2209.5 |###########
   2282.9 |########################################
   2356.3 |#####
   2429.6 |
   2503.0 |#################
   2576.4 |#####
   2649.8 |
   2723.2 |#################
   2796.6 |#################
   2870.0 |
   2943.4 |
   3016.8 |######################
   3090.2 |#####
   3163.6 |#####
   3237.0 |
   3310.4 |
  (1 below, 5 above range)

bitpack-zeropad-seq (n=40, range 2143.6-8801.8 ns)
   2143.6 |########################################
   2476.5 |
   2809.4 |
   3142.3 |#
   3475.2 |
   3808.1 |
   4141.0 |#
   4473.9 |
   4806.8 |
   5139.8 |
   5472.7 |
   5805.6 |
   6138.5 |
   6471.4 |#
   6804.3 |###
   7137.2 |#
   7470.1 |
   7803.0 |#####
   8135.9 |
   8468.9 |#
  (4 below, 4 above range)

```

## Diagnostics

- **bitpack-aligned-seq**: CV=58.3% (high variance, measurements may be unstable)
- **bitpack-aligned-seq**: worst_20/best_20 = 4.1x (possible bimodal distribution)
- **bitpack-native-seq**: CV=21.8% (high variance, measurements may be unstable)
- **bitpack-zeropad-seq**: CV=69.2% (high variance, measurements may be unstable)
- **bitpack-zeropad-seq**: worst_20/best_20 = 4.1x (possible bimodal distribution)
