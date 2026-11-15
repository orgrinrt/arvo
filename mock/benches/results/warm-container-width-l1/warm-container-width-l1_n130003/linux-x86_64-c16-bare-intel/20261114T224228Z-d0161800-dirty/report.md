# Container fork, declared-width sweep, cache-resident (8192 elements, 3 ops/element, wrapping)

6 variants, 40 samples per variant.
Baseline: **warm-container-headroom**

## Highlights

Baseline for all deltas below: **warm-container-headroom**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-container-plusone is an outlier: 2.6x slower than the field

warm-container-plusone (4.61 us) is 2.6x the fastest (1.80 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### warm-container-headroom is fastest but the noisiest (CV 119.9%)

warm-container-headroom wins on median (1.80 us) yet has the highest variance (CV 119.9%), while warm-container-native is the steadiest (CV 11.0%, 1.95 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### warm-container-kernel shows alternating (throttle bounce) (autocorr -0.56)

warm-container-kernel's per-pass series has lag-1 autocorrelation -0.56, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (warm-container-headroom)

The baseline warm-container-headroom is the fastest (1.80 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Speed leader warm-container-headroom vs stability leader warm-container-native (+8% speed for 10.9x steadier)

warm-container-headroom is fastest (1.80 us, CV 119.9%); warm-container-native gives up 8.0% median for 10.9x lower variance (CV 11.0%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### warm-container-headroom is inconsistent: worst-20% is 4.2x its best-20%

warm-container-headroom's best 20% of batches run at 1.55 us but its worst 20% at 6.56 us (4.2x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (warm-container-headroom) is the fastest** at 1804.5 ns median
- Spread: 2.55x (fastest 1804.5 ns, slowest 4609.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-container-headroom | 3786ns | 1869ns | 1604ns | 3489ns | 6860ns | base |
| warm-container-kernel | 3045ns | 3046ns | 1993ns | 3115ns | 3886ns | -19.58% |
| warm-container-lanes-deferred | 3163ns | 3233ns | 1985ns | 3243ns | 4102ns | -16.46% |
| warm-container-minimum | 3602ns | 2086ns | 1842ns | 2980ns | 7229ns | -4.87% |
| warm-container-native | 2238ns | 2241ns | 1931ns | 2191ns | 2688ns | -40.88% |
| warm-container-plusone | 4111ns | 4773ns | 1674ns | 4032ns | 6785ns | +8.58% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-container-headroom | 3644ns | 1549ns | 6563ns | base | 8.992 |
| warm-container-kernel | 2834ns | 1858ns | 3549ns | -22.23% | 11.562 |
| warm-container-lanes-deferred | 2963ns | 1867ns | 3836ns | -18.71% | 11.061 |
| warm-container-minimum | 3451ns | 1779ns | 6897ns | -5.31% | 9.496 |
| warm-container-native | 1952ns | 1713ns | 2297ns | -46.43% | 16.786 |
| warm-container-plusone | 3958ns | 1617ns | 6487ns | +8.60% | 8.280 |

## Performance model

- Peak throughput: **21.154 Gops/s** (warm-container-headroom; best 20% batches)
- Ops per call: 32768

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-container-headroom | 18.160 | 85.8% |
| warm-container-kernel | 11.412 | 53.9% |
| warm-container-lanes-deferred | 10.829 | 51.2% |
| warm-container-minimum | 16.551 | 78.2% |
| warm-container-native | 16.817 | 79.5% |
| warm-container-plusone | 7.108 | 33.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-container-headroom | 3786ns | 3786ns | base |
| warm-container-kernel | 3045ns | 3045ns | -19.58% |
| warm-container-lanes-deferred | 3163ns | 3163ns | -16.46% |
| warm-container-minimum | 3602ns | 3602ns | -4.87% |
| warm-container-native | 2238ns | 2238ns | -40.88% |
| warm-container-plusone | 4111ns | 4111ns | +8.58% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-container-headroom | 1804ns | base | --- | [1793, 5735] | --- | --- | --- | --- |
| warm-container-kernel | 2871ns | no significant difference | [-2111, +802]ns | [2793, 3070] | no | 0.5370 | 0.4296 | 0 |
| warm-container-lanes-deferred | 3026ns | no significant difference | [-1834, +832]ns | [2832, 3238] | no | 0.8746 | 0.8746 | 0 |
| warm-container-minimum | 1980ns | no significant difference | [-14, +4]ns | [1791, 3862] | no | 0.2564 | 0.0807 | 0 |
| warm-container-native | 1949ns | no significant difference | [-3517, +2]ns | [1806, 1999] | no | 0.2564 | 0.1539 | 0 |
| warm-container-plusone | 4610ns | no significant difference | [-15, +30]ns | [1783, 5550] | no | 0.2564 | 0.1539 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-container-headroom | warm-container-kernel | warm-container-lanes-deferred | warm-container-minimum | warm-container-native | warm-container-plusone |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 1472ns | +120.9% | +139.6% | +35.0% | +41.5% | +152.8% |
| 1 | 2 | 1799ns | +55.3% | +67.2% | -0.4% | +11.0% | +187.1% |
| 1 | 3 | 4685ns | -34.5% | -41.0% | -24.1% | -62.3% | +12.5% |
| 1 | 4 | 2803ns | -0.4% | +16.1% | -36.0% | -32.4% | -36.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-container-headroom | -0.074 | ok |
| warm-container-kernel | -0.557 | HIGH- (thermal bounce) |
| warm-container-lanes-deferred | -0.142 | ok |
| warm-container-minimum | -0.503 | HIGH- (thermal bounce) |
| warm-container-native | 0.099 | ok |
| warm-container-plusone | -0.204 | moderate- |

**Consistency summary:**

- **warm-container-kernel**: won 2/4, lost 2/4
- **warm-container-lanes-deferred**: won 1/4, lost 3/4
- **warm-container-minimum**: won 3/4, lost 1/4
- **warm-container-native**: won 2/4, lost 2/4
- **warm-container-plusone**: won 1/4, lost 3/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-container-headroom | 49.9ns | 3644.2ns | 1.4% |  |
| warm-container-kernel | 74.2ns | 2834.0ns | 2.6% |  |
| warm-container-lanes-deferred | 71.1ns | 2962.6ns | 2.4% |  |
| warm-container-minimum | 51.4ns | 3450.8ns | 1.5% |  |
| warm-container-native | 103.7ns | 1952.1ns | 5.3% | HIGH |
| warm-container-plusone | 54.0ns | 3957.5ns | 1.4% |  |

## Distribution (algo ns)

```
warm-container-headroom (n=40, range 1549.0-6563.4 ns)
   1549.0 |########################################
   1799.8 |##########
   2050.5 |
   2301.2 |
   2551.9 |
   2802.6 |
   3053.4 |
   3304.1 |###
   3554.8 |######
   3805.5 |
   4056.2 |
   4306.9 |
   4557.7 |
   4808.4 |###
   5059.1 |
   5309.8 |
   5560.5 |##########
   5811.3 |######
   6062.0 |###
   6312.7 |####################
  (6 below, 3 above range)

warm-container-kernel (n=40, range 1857.7-3549.3 ns)
   1857.7 |
   1942.3 |
   2026.9 |
   2111.4 |
   2196.0 |
   2280.6 |
   2365.2 |
   2449.8 |####
   2534.3 |#############
   2618.9 |#############
   2703.5 |
   2788.1 |########################################
   2872.6 |########
   2957.2 |#############
   3041.8 |#################
   3126.4 |####
   3211.0 |########
   3295.5 |#############
   3380.1 |####
   3464.7 |
  (4 below, 4 above range)

warm-container-lanes-deferred (n=40, range 1866.8-3836.2 ns)
   1866.8 |
   1965.3 |
   2063.8 |
   2162.2 |
   2260.7 |##########
   2359.2 |##########
   2457.6 |##########
   2556.1 |####################
   2654.6 |##############################
   2753.1 |########################################
   2851.5 |##############################
   2950.0 |########################################
   3048.5 |##############################
   3146.9 |##########
   3245.4 |##############################
   3343.9 |####################
   3442.4 |##########
   3540.8 |########################################
   3639.3 |##########
   3737.8 |
  (3 below, 3 above range)

warm-container-minimum (n=40, range 1779.3-6897.3 ns)
   1779.3 |########################################
   2035.2 |##
   2291.1 |##
   2547.0 |
   2802.9 |
   3058.8 |
   3314.7 |
   3570.6 |
   3826.5 |
   4082.4 |
   4338.3 |
   4594.2 |
   4850.1 |
   5106.0 |##
   5361.9 |####
   5617.8 |
   5873.7 |######
   6129.6 |
   6385.5 |############
   6641.4 |
  (5 below, 2 above range)

warm-container-native (n=40, range 1713.2-2296.5 ns)
   1713.2 |
   1742.4 |#############
   1771.5 |########################################
   1800.7 |####################
   1829.9 |####################
   1859.0 |######
   1888.2 |
   1917.4 |######
   1946.5 |####################
   1975.7 |#################################
   2004.9 |######
   2034.0 |######
   2063.2 |######
   2092.4 |######
   2121.5 |
   2150.7 |######
   2179.9 |
   2209.0 |######
   2238.2 |####################
   2267.4 |######
  (4 below, 2 above range)

warm-container-plusone (n=40, range 1616.9-6487.3 ns)
   1616.9 |########################################
   1860.4 |###
   2103.9 |
   2347.4 |
   2590.9 |
   2834.5 |
   3078.0 |
   3321.5 |
   3565.0 |
   3808.6 |###
   4052.1 |###
   4295.6 |
   4539.1 |
   4782.7 |###
   5026.2 |###
   5269.7 |############
   5513.2 |###
   5756.8 |#########
   6000.3 |######
   6243.8 |############
  (4 below, 4 above range)

```

## Diagnostics

- **warm-container-headroom**: CV=59.4% (high variance, measurements may be unstable)
- **warm-container-headroom**: worst_20/best_20 = 4.2x (possible bimodal distribution)
- **warm-container-kernel**: CV=23.0% (high variance, measurements may be unstable)
- **warm-container-lanes-deferred**: CV=25.6% (high variance, measurements may be unstable)
- **warm-container-minimum**: CV=64.2% (high variance, measurements may be unstable)
- **warm-container-minimum**: worst_20/best_20 = 3.9x (possible bimodal distribution)
- **warm-container-plusone**: CV=52.9% (high variance, measurements may be unstable)
- **warm-container-plusone**: worst_20/best_20 = 4.0x (possible bimodal distribution)
