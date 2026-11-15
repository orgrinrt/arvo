# Layout::Bitpacked reading, sequential column sum: byte-aligned slot vs zero-inter-value-padding

3 variants, 40 samples per variant.
Baseline: **bitpack-aligned-seq**

## Highlights

Baseline for all deltas below: **bitpack-aligned-seq**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### bitpack-native-seq dominates: 132% faster than the next best (bitpack-aligned-seq)

bitpack-native-seq (74 ns) leads bitpack-aligned-seq (172 ns) by 132%, a clear separation rather than a photo finish. CV 54.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### bitpack-native-seq beats baseline by 59% (significant)

bitpack-native-seq is -101 ns (59%) faster than baseline bitpack-aligned-seq, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### bitpack-zeropad-seq is an outlier: 2.7x slower than the field

bitpack-zeropad-seq (203 ns) is 2.7x the fastest (74 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### bitpack-zeropad-seq is inconsistent: worst-20% is 2.9x its best-20%

bitpack-zeropad-seq's best 20% of batches run at 160 ns but its worst 20% at 466 ns (2.9x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: bitpack-native-seq** at 74.2 ns median (-56.8% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 2.74x (fastest 74.2 ns, slowest 203.1 ns)

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
| Counter frequency | 2100219680 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bitpack-aligned-seq | 279ns | 274ns | 221ns | 272ns | 357ns | base |
| bitpack-native-seq | 184ns | 179ns | 141ns | 176ns | 252ns | -33.96% |
| bitpack-zeropad-seq | 381ns | 312ns | 242ns | 339ns | 647ns | +36.65% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-aligned-seq | 170ns | 139ns | 203ns | base | 1.504 |
| bitpack-native-seq | 77ns | 59ns | 107ns | -54.75% | 3.324 |
| bitpack-zeropad-seq | 256ns | 160ns | 466ns | +50.25% | 1.001 |

## Performance model

- Peak throughput: **4.329 Gops/s** (bitpack-native-seq; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-aligned-seq | 1.490 | 34.4% |
| bitpack-native-seq | 3.450 | 79.7% |
| bitpack-zeropad-seq | 1.260 | 29.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-aligned-seq | 279ns | 279ns | base |
| bitpack-native-seq | 184ns | 184ns | -33.96% |
| bitpack-zeropad-seq | 381ns | 381ns | +36.65% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-aligned-seq | 172ns | base | --- | [172, 172] | --- | --- | --- | --- |
| bitpack-native-seq | 74ns | -100.5ns (-58.5%) | [-104, -96]ns | [73, 75] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-zeropad-seq | 203ns | +31.3ns (+18.2%) | [+30, +80]ns | [202, 251] | YES | 0.0064 | 0.0064 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bitpack-aligned-seq | bitpack-native-seq | bitpack-zeropad-seq |
|---|---|---|---|---|
| 1 | 1 | 139ns | -45.8% | +115.1% |
| 1 | 2 | 172ns | -56.1% | +18.2% |
| 1 | 3 | 172ns | -65.6% | +18.2% |
| 1 | 4 | 177ns | -58.7% | -9.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-aligned-seq | -0.052 | ok |
| bitpack-native-seq | -0.329 | moderate- |
| bitpack-zeropad-seq | -0.017 | ok |

**Consistency summary:**

- **bitpack-native-seq**: won 4/4, lost 0/4
- **bitpack-zeropad-seq**: won 1/4, lost 3/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-aligned-seq | 39.4ns | 170.2ns | 23.2% | HIGH |
| bitpack-native-seq | 41.1ns | 77.0ns | 53.4% | HIGH |
| bitpack-zeropad-seq | 44.8ns | 255.8ns | 17.5% | HIGH |

## Distribution (algo ns)

```
bitpack-aligned-seq (n=40, range 139.0-203.4 ns)
    139.0 |##########
    142.3 |
    145.5 |
    148.7 |
    151.9 |
    155.1 |
    158.3 |
    161.6 |
    164.8 |
    168.0 |
    171.2 |########################################
    174.4 |##################
    177.6 |
    180.8 |
    184.1 |
    187.3 |
    190.5 |
    193.7 |
    196.9 |
    200.1 |
  (5 below, 1 above range)

bitpack-native-seq (n=40, range 59.1-107.0 ns)
     59.1 |###########
     61.5 |
     63.9 |
     66.3 |
     68.7 |
     71.1 |#######################
     73.5 |########################################
     75.9 |####
     78.3 |
     80.7 |
     83.0 |
     85.4 |
     87.8 |
     90.2 |
     92.6 |
     95.0 |
     97.4 |
     99.8 |
    102.2 |
    104.6 |
  (5 below, 1 above range)

bitpack-zeropad-seq (n=40, range 159.5-466.2 ns)
    159.5 |#######################
    174.9 |###
    190.2 |########################################
    205.5 |###
    220.9 |###
    236.2 |
    251.5 |
    266.9 |#############
    282.2 |###
    297.5 |####################
    312.9 |###
    328.2 |
    343.5 |
    358.9 |
    374.2 |
    389.5 |
    404.8 |
    420.2 |
    435.5 |
    450.8 |
  (4 below, 2 above range)

```

## Diagnostics

- **bitpack-aligned-seq**: CV=22.3% (high variance, measurements may be unstable)
- **bitpack-aligned-seq**: bridge=23.4% of algo (FFI overhead may distort results)
- **bitpack-native-seq**: CV=52.6% (high variance, measurements may be unstable)
- **bitpack-native-seq**: bridge=55.6% of algo (FFI overhead may distort results)
- **bitpack-zeropad-seq**: CV=70.8% (high variance, measurements may be unstable)
- **bitpack-zeropad-seq**: bridge=20.3% of algo (FFI overhead may distort results)
