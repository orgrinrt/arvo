# Container fork under saturating semantics, elementwise, declared-width sweep (8192 elements, 4 ops/element)

5 variants, 40 samples per variant.
Baseline: **warm-container-headroom**

## Highlights

Baseline for all deltas below: **warm-container-headroom**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-container-native beats baseline by 69% (significant)

warm-container-native is -1.09 us (69%) faster than baseline warm-container-headroom, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-container-plusone is an outlier: 3.6x slower than the field

warm-container-plusone (2.05 us) is 3.6x the fastest (566 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {warm-container-native, warm-container-kernel, warm-container-minimum} vs {warm-container-headroom, warm-container-plusone} (141% apart)

The field splits into a fast tier {warm-container-native, warm-container-kernel, warm-container-minimum} and a slow tier {warm-container-headroom, warm-container-plusone} with a 141% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 3.6x the fastest

Fastest warm-container-native (566 ns) to slowest warm-container-plusone (2.05 us): 3.6x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### warm-container-minimum is inconsistent: worst-20% is 1.5x its best-20%

warm-container-minimum's best 20% of batches run at 591 ns but its worst 20% at 889 ns (1.5x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: warm-container-native** at 565.6 ns median (-64.3% vs baseline)
- 3 variants significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 3.62x (fastest 565.6 ns, slowest 2049.4 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Container fork under saturating semantics, elementwise, declared-width sweep (8192 elements, 4 ops/element) |
| Passes | 4 |
| Runs per pass | 1000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x123456789abcdef0 |
| Counter frequency | 2112749630 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-container-headroom | 1814ns | 1773ns | 1457ns | 1814ns | 2169ns | base |
| warm-container-kernel | 835ns | 821ns | 733ns | 829ns | 954ns | -53.98% |
| warm-container-minimum | 966ns | 908ns | 817ns | 923ns | 1244ns | -46.75% |
| warm-container-native | 765ns | 814ns | 628ns | 766ns | 898ns | -57.83% |
| warm-container-plusone | 2292ns | 2307ns | 2079ns | 2249ns | 2636ns | +26.38% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-container-headroom | 1607ns | 1300ns | 1907ns | base | 25.483 |
| warm-container-kernel | 597ns | 531ns | 673ns | -62.83% | 68.558 |
| warm-container-minimum | 691ns | 591ns | 889ns | -56.98% | 59.234 |
| warm-container-native | 546ns | 455ns | 625ns | -66.02% | 74.987 |
| warm-container-plusone | 2025ns | 1853ns | 2306ns | +25.98% | 20.227 |

## Performance model

- Peak throughput: **89.953 Gops/s** (warm-container-native; best 20% batches)
- Ops per call: 40960

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-container-headroom | 25.846 | 28.7% |
| warm-container-kernel | 68.991 | 76.7% |
| warm-container-minimum | 62.316 | 69.3% |
| warm-container-native | 72.412 | 80.5% |
| warm-container-plusone | 19.986 | 22.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-container-headroom | 1814ns | 1814ns | base |
| warm-container-kernel | 835ns | 835ns | -53.98% |
| warm-container-minimum | 966ns | 966ns | -46.75% |
| warm-container-native | 765ns | 765ns | -57.83% |
| warm-container-plusone | 2292ns | 2292ns | +26.38% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-container-headroom | 1585ns | base | --- | [1527, 1694] | --- | --- | --- | --- |
| warm-container-kernel | 594ns | -973.7ns (-61.4%) | [-1076, -883]ns | [593, 594] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-minimum | 657ns | -916.0ns (-57.8%) | [-1004, -859]ns | [656, 659] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-native | 566ns | -1089.8ns (-68.8%) | [-1230, -901]ns | [506, 592] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-plusone | 2049ns | +431.1ns (+27.2%) | [+354, +516]ns | [1906, 2060] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-container-headroom | warm-container-kernel | warm-container-minimum | warm-container-native | warm-container-plusone |
|---|---|---|---|---|---|---|
| 1 | 1 | 1325ns | -59.3% | -55.4% | -60.1% | +41.8% |
| 1 | 2 | 1684ns | -63.8% | -61.0% | -73.0% | +22.3% |
| 1 | 3 | 1543ns | -57.5% | -59.0% | -61.6% | +28.7% |
| 1 | 4 | 1853ns | -68.0% | -54.4% | -68.0% | +11.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-container-headroom | -0.283 | moderate- |
| warm-container-kernel | -0.061 | ok |
| warm-container-minimum | -0.123 | ok |
| warm-container-native | -0.049 | ok |
| warm-container-plusone | -0.405 | moderate- |

**Consistency summary:**

- **warm-container-kernel**: won 4/4, lost 0/4
- **warm-container-minimum**: won 4/4, lost 0/4
- **warm-container-native**: won 4/4, lost 0/4
- **warm-container-plusone**: won 0/4, lost 4/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-container-headroom | 74.8ns | 1607.4ns | 4.7% |  |
| warm-container-kernel | 86.9ns | 597.4ns | 14.5% | HIGH |
| warm-container-minimum | 103.7ns | 691.5ns | 15.0% | HIGH |
| warm-container-native | 84.6ns | 546.2ns | 15.5% | HIGH |
| warm-container-plusone | 94.8ns | 2025.0ns | 4.7% |  |

## Distribution (algo ns)

```
warm-container-headroom (n=40, range 1299.5-1906.5 ns)
   1299.5 |######################
   1329.9 |
   1360.2 |###########
   1390.6 |
   1420.9 |######################
   1451.3 |
   1481.6 |#####
   1512.0 |
   1542.3 |############################
   1572.7 |###########
   1603.0 |
   1633.4 |###########
   1663.7 |#################
   1694.1 |#####
   1724.4 |
   1754.8 |###########
   1785.1 |
   1815.5 |
   1845.8 |########################################
   1876.2 |
  (3 below, 4 above range)

warm-container-kernel (n=40, range 531.0-672.7 ns)
    531.0 |
    538.1 |##########################
    545.2 |
    552.2 |
    559.3 |
    566.4 |
    573.5 |
    580.6 |
    587.7 |########################################
    594.7 |
    601.8 |
    608.9 |
    616.0 |##
    623.1 |##
    630.2 |
    637.2 |
    644.3 |#####
    651.4 |
    658.5 |#####################
    665.6 |
  (2 below, 1 above range)

warm-container-minimum (n=40, range 590.9-889.3 ns)
    590.9 |################
    605.8 |#####
    620.7 |
    635.7 |##
    650.6 |########################################
    665.5 |
    680.4 |
    695.3 |##
    710.3 |
    725.2 |########
    740.1 |
    755.0 |
    769.9 |##
    784.9 |
    799.8 |
    814.7 |
    829.6 |#####
    844.5 |########
    859.4 |
    874.4 |
  (4 below, 2 above range)

warm-container-native (n=40, range 455.3-625.5 ns)
    455.3 |#############
    463.9 |
    472.4 |
    480.9 |
    489.4 |##########
    497.9 |
    506.4 |
    514.9 |##
    523.4 |
    531.9 |#############
    540.4 |
    548.9 |
    557.4 |
    565.9 |
    574.4 |
    582.9 |
    591.5 |########################################
    600.0 |
    608.5 |
    617.0 |##
  (5 below, 4 above range)

warm-container-plusone (n=40, range 1853.3-2306.0 ns)
   1853.3 |####################################
   1875.9 |
   1898.5 |##############
   1921.2 |
   1943.8 |
   1966.4 |###
   1989.1 |###
   2011.7 |
   2034.4 |###
   2057.0 |########################################
   2079.6 |
   2102.3 |
   2124.9 |###
   2147.5 |###
   2170.2 |
   2192.8 |###
   2215.4 |
   2238.1 |
   2260.7 |
   2283.3 |###
  (3 below, 5 above range)

```

## Diagnostics

- **warm-container-kernel**: bridge=14.7% of algo (FFI overhead may distort results)
- **warm-container-minimum**: bridge=14.7% of algo (FFI overhead may distort results)
- **warm-container-native**: bridge=15.3% of algo (FFI overhead may distort results)
