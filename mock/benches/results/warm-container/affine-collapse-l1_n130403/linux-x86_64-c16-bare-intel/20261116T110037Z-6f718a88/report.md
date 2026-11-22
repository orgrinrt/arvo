# Wrapping reduction whose steps are all affine: what the interior projection prevents the optimiser from doing (8192 elements, 3 ops/element)

6 variants, 40 samples per variant.
Baseline: **warm-container-headroom**

## Highlights

Baseline for all deltas below: **warm-container-headroom**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-container-plusone is an outlier: 2.4x slower than the field

warm-container-plusone (4.35 us) is 2.4x the fastest (1.81 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {warm-container-lanes-deferred, warm-container-minimum, warm-container-kernel, warm-container-native, warm-container-headroom} vs {warm-container-plusone} (65% apart)

The field splits into a fast tier {warm-container-lanes-deferred, warm-container-minimum, warm-container-kernel, warm-container-native, warm-container-headroom} and a slow tier {warm-container-plusone} with a 65% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### warm-container-minimum is inconsistent: worst-20% is 4.5x its best-20%

warm-container-minimum's best 20% of batches run at 1.45 us but its worst 20% at 6.54 us (4.5x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: warm-container-lanes-deferred** at 1806.9 ns median (-31.4% vs baseline)
- Spread: 2.41x (fastest 1806.9 ns, slowest 4354.2 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Wrapping reduction whose steps are all affine: what the interior projection prevents the optimiser from doing (8192 elements, 3 ops/element) |
| Passes | 4 |
| Runs per pass | 1000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x123456789abcdef0 |
| Counter frequency | 2112700793 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-container-headroom | 3556ns | 2729ns | 1519ns | 3317ns | 6311ns | base |
| warm-container-kernel | 2162ns | 2079ns | 1746ns | 2126ns | 2684ns | -39.22% |
| warm-container-lanes-deferred | 2032ns | 2034ns | 1783ns | 2005ns | 2364ns | -42.85% |
| warm-container-minimum | 3433ns | 1900ns | 1507ns | 2947ns | 6819ns | -3.46% |
| warm-container-native | 2373ns | 2302ns | 1636ns | 2325ns | 3257ns | -33.26% |
| warm-container-plusone | 3776ns | 4508ns | 1638ns | 3602ns | 6437ns | +6.19% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-container-headroom | 3421ns | 1467ns | 6031ns | base | 9.579 |
| warm-container-kernel | 1909ns | 1552ns | 2338ns | -44.18% | 17.161 |
| warm-container-lanes-deferred | 1793ns | 1583ns | 2054ns | -47.58% | 18.274 |
| warm-container-minimum | 3303ns | 1455ns | 6536ns | -3.45% | 9.921 |
| warm-container-native | 2090ns | 1454ns | 2850ns | -38.89% | 15.675 |
| warm-container-plusone | 3635ns | 1582ns | 6176ns | +6.26% | 9.015 |

## Performance model

- Peak throughput: **22.533 Gops/s** (warm-container-native; best 20% batches)
- Ops per call: 32768

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-container-headroom | 12.436 | 55.2% |
| warm-container-kernel | 17.848 | 79.2% |
| warm-container-lanes-deferred | 18.135 | 80.5% |
| warm-container-minimum | 17.860 | 79.3% |
| warm-container-native | 16.155 | 71.7% |
| warm-container-plusone | 7.526 | 33.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-container-headroom | 3556ns | 3556ns | base |
| warm-container-kernel | 2162ns | 2162ns | -39.22% |
| warm-container-lanes-deferred | 2032ns | 2032ns | -42.85% |
| warm-container-minimum | 3433ns | 3433ns | -3.46% |
| warm-container-native | 2373ns | 2373ns | -33.26% |
| warm-container-plusone | 3776ns | 3776ns | +6.19% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-container-headroom | 2635ns | base | --- | [1469, 5008] | --- | --- | --- | --- |
| warm-container-kernel | 1836ns | no significant difference | [-3114, +40]ns | [1808, 2010] | no | 0.6358 | 0.6358 | 0 |
| warm-container-lanes-deferred | 1807ns | no significant difference | [-3067, +58]ns | [1693, 1811] | no | 0.3352 | 0.2682 | 0 |
| warm-container-minimum | 1835ns | no significant difference | [-42, +70]ns | [1475, 4377] | no | 0.3352 | 0.2682 | 0 |
| warm-container-native | 2028ns | no significant difference | [-2904, +367]ns | [1872, 2252] | no | 0.3352 | 0.2682 | 0 |
| warm-container-plusone | 4354ns | no significant difference | [-111, +324]ns | [1793, 4788] | no | 0.3352 | 0.1539 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-container-headroom | warm-container-kernel | warm-container-lanes-deferred | warm-container-minimum | warm-container-native | warm-container-plusone |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 1469ns | +53.7% | +12.2% | +0.1% | +77.3% | +205.1% |
| 1 | 2 | 2313ns | -21.7% | -21.8% | -37.1% | -2.6% | -22.2% |
| 1 | 3 | 3424ns | -53.1% | -43.6% | -28.5% | -43.4% | -47.5% |
| 1 | 4 | 4759ns | -59.9% | -62.1% | -10.1% | -68.5% | -5.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-container-headroom | 0.247 | moderate+ |
| warm-container-kernel | -0.045 | ok |
| warm-container-lanes-deferred | 0.013 | ok |
| warm-container-minimum | 0.177 | ok |
| warm-container-native | 0.224 | moderate+ |
| warm-container-plusone | -0.250 | moderate- |

**Consistency summary:**

- **warm-container-kernel**: won 3/4, lost 1/4
- **warm-container-lanes-deferred**: won 3/4, lost 1/4
- **warm-container-minimum**: won 3/4, lost 0/4
- **warm-container-native**: won 3/4, lost 1/4
- **warm-container-plusone**: won 3/4, lost 1/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-container-headroom | 46.7ns | 3420.8ns | 1.4% |  |
| warm-container-kernel | 91.3ns | 1909.5ns | 4.8% |  |
| warm-container-lanes-deferred | 86.0ns | 1793.2ns | 4.8% |  |
| warm-container-minimum | 45.1ns | 3302.8ns | 1.4% |  |
| warm-container-native | 100.4ns | 2090.5ns | 4.8% |  |
| warm-container-plusone | 49.8ns | 3635.0ns | 1.4% |  |

## Distribution (algo ns)

```
warm-container-headroom (n=40, range 1467.3-6031.0 ns)
   1467.3 |########################################
   1695.5 |##########
   1923.7 |###
   2151.9 |
   2380.0 |
   2608.2 |
   2836.4 |
   3064.6 |###
   3292.8 |
   3521.0 |###
   3749.1 |
   3977.3 |
   4205.5 |
   4433.7 |###
   4661.9 |#######
   4890.1 |#######
   5118.2 |#######
   5346.4 |##################
   5574.6 |###
   5802.8 |#######
  (5 below, 3 above range)

warm-container-kernel (n=40, range 1552.3-2338.3 ns)
   1552.3 |####
   1591.6 |
   1630.9 |#################
   1670.2 |####
   1709.5 |####
   1748.8 |
   1788.1 |########################################
   1827.4 |####
   1866.7 |####
   1906.0 |
   1945.3 |####
   1984.6 |##########################
   2023.9 |########
   2063.2 |########
   2102.5 |
   2141.8 |
   2181.1 |
   2220.4 |########
   2259.7 |########
   2299.0 |####
  (4 below, 2 above range)

warm-container-lanes-deferred (n=40, range 1583.3-2053.9 ns)
   1583.3 |
   1606.8 |#######
   1630.3 |#####################
   1653.9 |
   1677.4 |##########
   1700.9 |
   1724.5 |
   1748.0 |
   1771.5 |###
   1795.1 |########################################
   1818.6 |###
   1842.1 |#######
   1865.6 |###
   1889.2 |###
   1912.7 |
   1936.2 |###
   1959.8 |
   1983.3 |###
   2006.8 |##########
   2030.3 |
  (4 below, 3 above range)

warm-container-minimum (n=40, range 1454.5-6535.8 ns)
   1454.5 |########################################
   1708.6 |#########################
   1962.7 |
   2216.7 |
   2470.8 |
   2724.8 |
   2978.9 |###
   3233.0 |
   3487.0 |
   3741.1 |
   3995.2 |###
   4249.2 |##########
   4503.3 |###
   4757.3 |#######
   5011.4 |###
   5265.5 |
   5519.5 |###
   5773.6 |###
   6027.7 |###
   6281.7 |##########
  (4 below, 3 above range)

warm-container-native (n=40, range 1454.2-2849.9 ns)
   1454.2 |########################################
   1524.0 |########
   1593.8 |########
   1663.6 |
   1733.4 |################
   1803.2 |################
   1872.9 |################
   1942.7 |################################
   2012.5 |########################
   2082.3 |########
   2152.1 |
   2221.9 |################################
   2291.7 |################
   2361.4 |
   2431.2 |########
   2501.0 |########
   2570.8 |################
   2640.6 |################
   2710.4 |
   2780.1 |
  (3 below, 4 above range)

warm-container-plusone (n=40, range 1582.0-6175.9 ns)
   1582.0 |########################################
   1811.7 |
   2041.4 |
   2271.1 |
   2500.8 |
   2730.5 |
   2960.2 |###
   3189.9 |
   3419.6 |
   3649.3 |
   3878.9 |
   4108.6 |###
   4338.3 |######
   4568.0 |############
   4797.7 |#########
   5027.4 |###
   5257.1 |#########
   5486.8 |###
   5716.5 |
   5946.2 |###
  (5 below, 5 above range)

```

## Diagnostics

- **warm-container-headroom**: CV=57.7% (high variance, measurements may be unstable)
- **warm-container-headroom**: worst_20/best_20 = 4.1x (possible bimodal distribution)
- **warm-container-minimum**: CV=62.6% (high variance, measurements may be unstable)
- **warm-container-minimum**: worst_20/best_20 = 4.5x (possible bimodal distribution)
- **warm-container-native**: CV=23.6% (high variance, measurements may be unstable)
- **warm-container-plusone**: CV=51.5% (high variance, measurements may be unstable)
- **warm-container-plusone**: worst_20/best_20 = 3.9x (possible bimodal distribution)
