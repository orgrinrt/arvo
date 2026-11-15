# Container fork, declared-width sweep, cache-resident (8192 elements, 3 ops/element, wrapping)

6 variants, 40 samples per variant.
Baseline: **warm-container-headroom**

## Highlights

Baseline for all deltas below: **warm-container-headroom**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-container-native dominates: 89% faster than the next best (warm-container-kernel)

warm-container-native (1.99 us) leads warm-container-kernel (3.76 us) by 89%, a clear separation rather than a photo finish. CV 110.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### warm-container-native beats baseline by 43% (significant)

warm-container-native is -1.77 us (43%) faster than baseline warm-container-headroom, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-container-lanes-deferred is an outlier: 2.3x slower than the field

warm-container-lanes-deferred (4.66 us) is 2.3x the fastest (1.99 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### warm-container-native is fastest but the noisiest (CV 110.1%)

warm-container-native wins on median (1.99 us) yet has the highest variance (CV 110.1%), while warm-container-kernel is the steadiest (CV 35.0%, 3.76 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Two tiers: {warm-container-native} vs {warm-container-kernel, warm-container-plusone, warm-container-headroom, warm-container-minimum, warm-container-lanes-deferred} (89% apart)

The field splits into a fast tier {warm-container-native} and a slow tier {warm-container-kernel, warm-container-plusone, warm-container-headroom, warm-container-minimum, warm-container-lanes-deferred} with a 89% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### warm-container-native is inconsistent: worst-20% is 3.9x its best-20%

warm-container-native's best 20% of batches run at 1.84 us but its worst 20% at 7.25 us (3.9x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: warm-container-native** at 1986.4 ns median (-51.5% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 2.35x (fastest 1986.4 ns, slowest 4661.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-container-headroom | 4801ns | 4183ns | 3826ns | 4171ns | 7668ns | base |
| warm-container-kernel | 3837ns | 3956ns | 1637ns | 3949ns | 5700ns | -20.09% |
| warm-container-lanes-deferred | 4028ns | 4901ns | 1645ns | 4125ns | 6118ns | -16.11% |
| warm-container-minimum | 4207ns | 4674ns | 1854ns | 4007ns | 7162ns | -12.38% |
| warm-container-native | 3741ns | 2071ns | 1920ns | 3055ns | 7619ns | -22.09% |
| warm-container-plusone | 5085ns | 4183ns | 3422ns | 3934ns | 10202ns | +5.91% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-container-headroom | 4691ns | 3742ns | 7452ns | base | 6.985 |
| warm-container-kernel | 3633ns | 1553ns | 5389ns | -22.56% | 9.020 |
| warm-container-lanes-deferred | 3818ns | 1564ns | 5783ns | -18.61% | 8.582 |
| warm-container-minimum | 4025ns | 1778ns | 6828ns | -14.21% | 8.142 |
| warm-container-native | 3576ns | 1842ns | 7246ns | -23.76% | 9.162 |
| warm-container-plusone | 4974ns | 3354ns | 9953ns | +6.03% | 6.587 |

## Performance model

- Peak throughput: **21.103 Gops/s** (warm-container-kernel; best 20% batches)
- Ops per call: 32768

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-container-headroom | 7.993 | 37.9% |
| warm-container-kernel | 8.726 | 41.3% |
| warm-container-lanes-deferred | 7.029 | 33.3% |
| warm-container-minimum | 7.313 | 34.7% |
| warm-container-native | 16.496 | 78.2% |
| warm-container-plusone | 7.994 | 37.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-container-headroom | 4801ns | 4801ns | base |
| warm-container-kernel | 3837ns | 3837ns | -20.09% |
| warm-container-lanes-deferred | 4028ns | 4028ns | -16.11% |
| warm-container-minimum | 4207ns | 4207ns | -12.38% |
| warm-container-native | 3741ns | 3741ns | -22.09% |
| warm-container-plusone | 5085ns | 5085ns | +5.91% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-container-headroom | 4100ns | base | --- | [4098, 4106] | --- | --- | --- | --- |
| warm-container-kernel | 3755ns | -592.0ns (-14.4%) | [-1101, -289]ns | [3496, 4096] | YES | 0.0034 | 0.0007 | 0 |
| warm-container-lanes-deferred | 4662ns | no significant difference | [-2349, +782]ns | [2100, 5118] | no | 0.8746 | 0.8746 | 0 |
| warm-container-minimum | 4481ns | no significant difference | [-2121, +607]ns | [1985, 4891] | no | 0.7948 | 0.6358 | 0 |
| warm-container-native | 1986ns | -1770.4ns (-43.2%) | [-2117, -1647]ns | [1983, 4469] | YES | 0.0161 | 0.0064 | 0 |
| warm-container-plusone | 4099ns | no significant difference | [-378, +2]ns | [3364, 4104] | no | 0.2564 | 0.1539 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-container-headroom | warm-container-kernel | warm-container-lanes-deferred | warm-container-minimum | warm-container-native | warm-container-plusone |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 4106ns | -8.7% | -6.5% | -28.0% | +15.9% | -0.0% |
| 1 | 2 | 4101ns | -11.0% | -4.9% | +11.7% | -51.4% | +0.1% |
| 1 | 3 | 3741ns | -13.5% | +31.6% | +23.3% | -47.0% | -10.2% |
| 1 | 4 | 4107ns | -8.6% | +9.4% | -51.7% | -51.7% | -18.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-container-headroom | -0.416 | moderate- |
| warm-container-kernel | -0.375 | moderate- |
| warm-container-lanes-deferred | 0.072 | ok |
| warm-container-minimum | -0.229 | moderate- |
| warm-container-native | -0.080 | ok |
| warm-container-plusone | 0.252 | moderate+ |

**Consistency summary:**

- **warm-container-kernel**: won 4/4, lost 0/4
- **warm-container-lanes-deferred**: won 2/4, lost 2/4
- **warm-container-minimum**: won 2/4, lost 2/4
- **warm-container-native**: won 3/4, lost 1/4
- **warm-container-plusone**: won 2/4, lost 0/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-container-headroom | 29.8ns | 4691.3ns | 0.6% |  |
| warm-container-kernel | 58.9ns | 3632.9ns | 1.6% |  |
| warm-container-lanes-deferred | 62.2ns | 3818.1ns | 1.6% |  |
| warm-container-minimum | 51.2ns | 4024.7ns | 1.3% |  |
| warm-container-native | 45.5ns | 3576.5ns | 1.3% |  |
| warm-container-plusone | 31.0ns | 4974.4ns | 0.6% |  |

## Distribution (algo ns)

```
warm-container-headroom (n=40, range 3741.7-7452.1 ns)
   3741.7 |#####
   3927.3 |########################################
   4112.8 |#######
   4298.3 |
   4483.8 |
   4669.3 |#
   4854.8 |
   5040.4 |
   5225.9 |
   5411.4 |
   5596.9 |
   5782.4 |
   5968.0 |
   6153.5 |
   6339.0 |
   6524.5 |
   6710.0 |
   6895.5 |
   7081.1 |
   7266.6 |
  (6 below, 4 above range)

warm-container-kernel (n=40, range 1552.7-5389.2 ns)
   1552.7 |##########
   1744.6 |#####
   1936.4 |
   2128.2 |
   2320.0 |
   2511.9 |#####
   2703.7 |
   2895.5 |
   3087.3 |
   3279.2 |#####
   3471.0 |########################################
   3662.8 |###############
   3854.6 |###############
   4046.5 |###############
   4238.3 |#####
   4430.1 |#####
   4621.9 |##########
   4813.8 |
   5005.6 |###############
   5197.4 |#####
  (6 below, 4 above range)

warm-container-lanes-deferred (n=40, range 1563.5-5783.3 ns)
   1563.5 |########################################
   1774.5 |
   1985.5 |
   2196.5 |
   2407.5 |####
   2618.5 |
   2829.5 |####
   3040.5 |
   3251.4 |
   3462.4 |
   3673.4 |
   3884.4 |####
   4095.4 |####
   4306.4 |####
   4517.4 |####
   4728.4 |#############
   4939.4 |#################
   5150.4 |########
   5361.4 |########
   5572.3 |###############################
  (5 below, 2 above range)

warm-container-minimum (n=40, range 1777.8-6828.1 ns)
   1777.8 |########################################
   2030.3 |
   2282.8 |
   2535.3 |
   2787.9 |
   3040.4 |
   3292.9 |
   3545.4 |
   3797.9 |######
   4050.4 |
   4302.9 |#############
   4555.4 |#############
   4808.0 |###
   5060.5 |##########
   5313.0 |##########
   5565.5 |
   5818.0 |
   6070.5 |###
   6323.0 |
   6575.5 |###
  (4 below, 5 above range)

warm-container-native (n=40, range 1841.6-7246.1 ns)
   1841.6 |########################################
   2111.8 |
   2382.1 |##
   2652.3 |
   2922.5 |
   3192.7 |
   3463.0 |##
   3733.2 |
   4003.4 |##
   4273.6 |
   4543.9 |####
   4814.1 |####
   5084.3 |##
   5354.5 |####
   5624.8 |
   5895.0 |
   6165.2 |
   6435.4 |##
   6705.7 |##
   6975.9 |##
  (3 below, 4 above range)

warm-container-plusone (n=40, range 3354.3-9952.6 ns)
   3354.3 |###########################
   3684.2 |
   4014.2 |########################################
   4344.1 |
   4674.0 |
   5003.9 |
   5333.8 |
   5663.7 |
   5993.6 |
   6323.6 |
   6653.5 |
   6983.4 |
   7313.3 |##
   7643.2 |##
   7973.1 |
   8303.0 |##
   8633.0 |
   8962.9 |
   9292.8 |
   9622.7 |##
  (5 below, 4 above range)

```

## Diagnostics

- **warm-container-headroom**: CV=43.0% (high variance, measurements may be unstable)
- **warm-container-kernel**: CV=36.2% (high variance, measurements may be unstable)
- **warm-container-kernel**: worst_20/best_20 = 3.5x (possible bimodal distribution)
- **warm-container-lanes-deferred**: CV=46.6% (high variance, measurements may be unstable)
- **warm-container-lanes-deferred**: worst_20/best_20 = 3.7x (possible bimodal distribution)
- **warm-container-minimum**: CV=48.5% (high variance, measurements may be unstable)
- **warm-container-minimum**: worst_20/best_20 = 3.8x (possible bimodal distribution)
- **warm-container-native**: CV=61.2% (high variance, measurements may be unstable)
- **warm-container-native**: worst_20/best_20 = 3.9x (possible bimodal distribution)
- **warm-container-plusone**: CV=53.0% (high variance, measurements may be unstable)
