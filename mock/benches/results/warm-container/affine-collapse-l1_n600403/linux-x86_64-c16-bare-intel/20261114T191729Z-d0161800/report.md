# Wrapping reduction whose steps are all affine: what the interior projection prevents the optimiser from doing (8192 elements, 3 ops/element)

6 variants, 40 samples per variant.
Baseline: **warm-container-headroom**

## Highlights

Baseline for all deltas below: **warm-container-headroom**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-container-kernel dominates: 10% faster than the next best (warm-container-native)

warm-container-kernel (2.01 us) leads warm-container-native (2.22 us) by 10%, a clear separation rather than a photo finish. CV 112.0%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### warm-container-minimum beats baseline by 35% (significant)

warm-container-minimum is -986 ns (35%) faster than baseline warm-container-headroom, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-container-lanes-deferred is an outlier: 2.7x slower than the field

warm-container-lanes-deferred (5.45 us) is 2.7x the fastest (2.01 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### warm-container-kernel is fastest but the noisiest (CV 112.0%)

warm-container-kernel wins on median (2.01 us) yet has the highest variance (CV 112.0%), while warm-container-lanes-deferred is the steadiest (CV 38.1%, 5.45 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### warm-container-minimum shows alternating (throttle bounce) (autocorr -0.75)

warm-container-minimum's per-pass series has lag-1 autocorrelation -0.75, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### warm-container-minimum is inconsistent: worst-20% is 4.0x its best-20%

warm-container-minimum's best 20% of batches run at 1.61 us but its worst 20% at 6.42 us (4.0x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: warm-container-kernel** at 2013.9 ns median (-28.6% vs baseline)
- 5 variants significantly faster than baseline
- Spread: 2.70x (fastest 2013.9 ns, slowest 5445.4 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-container-headroom | 4674ns | 2891ns | 2878ns | 3594ns | 9708ns | base |
| warm-container-kernel | 4124ns | 2098ns | 2065ns | 3680ns | 7518ns | -11.76% |
| warm-container-lanes-deferred | 4573ns | 5672ns | 2045ns | 4539ns | 7201ns | -2.17% |
| warm-container-minimum | 4081ns | 4338ns | 1683ns | 3987ns | 6764ns | -12.68% |
| warm-container-native | 4135ns | 2317ns | 2053ns | 3810ns | 7192ns | -11.53% |
| warm-container-plusone | 3986ns | 2410ns | 1906ns | 3646ns | 7085ns | -14.72% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-container-headroom | 4552ns | 2810ns | 9430ns | base | 7.198 |
| warm-container-kernel | 3942ns | 1981ns | 7140ns | -13.42% | 8.313 |
| warm-container-lanes-deferred | 4381ns | 1963ns | 6869ns | -3.76% | 7.479 |
| warm-container-minimum | 3895ns | 1611ns | 6417ns | -14.44% | 8.413 |
| warm-container-native | 3954ns | 1968ns | 6839ns | -13.14% | 8.287 |
| warm-container-plusone | 3797ns | 1822ns | 6706ns | -16.59% | 8.630 |

## Performance model

- Peak throughput: **20.346 Gops/s** (warm-container-minimum; best 20% batches)
- Ops per call: 32768

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-container-headroom | 11.616 | 57.1% |
| warm-container-kernel | 16.271 | 80.0% |
| warm-container-lanes-deferred | 6.018 | 29.6% |
| warm-container-minimum | 7.885 | 38.8% |
| warm-container-native | 14.743 | 72.5% |
| warm-container-plusone | 14.205 | 69.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-container-headroom | 4674ns | 4674ns | base |
| warm-container-kernel | 4124ns | 4124ns | -11.76% |
| warm-container-lanes-deferred | 4573ns | 4573ns | -2.17% |
| warm-container-minimum | 4081ns | 4081ns | -12.68% |
| warm-container-native | 4135ns | 4135ns | -11.53% |
| warm-container-plusone | 3986ns | 3986ns | -14.72% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-container-headroom | 2821ns | base | --- | [2811, 3441] | --- | --- | --- | --- |
| warm-container-kernel | 2014ns | -828.8ns (-29.4%) | [-838, -821]ns | [1984, 5764] | YES | 0.0009 | 0.0002 | 0 |
| warm-container-lanes-deferred | 5445ns | -841.7ns (-29.8%) | [-859, -99]ns | [2013, 5910] | YES (adj: no) | 0.0807 | 0.0807 | 0 |
| warm-container-minimum | 4156ns | -986.3ns (-35.0%) | [-1284, -901]ns | [1885, 5506] | YES | 0.0207 | 0.0166 | 0 |
| warm-container-native | 2223ns | -847.0ns (-30.0%) | [-1193, -834]ns | [1976, 5666] | YES | 0.0011 | 0.0007 | 0 |
| warm-container-plusone | 2307ns | -969.2ns (-34.4%) | [-989, -947]ns | [1859, 5523] | YES | 0.0011 | 0.0007 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-container-headroom | warm-container-kernel | warm-container-lanes-deferred | warm-container-minimum | warm-container-native | warm-container-plusone |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 3447ns | +37.0% | +54.4% | +47.4% | -42.5% | +50.1% |
| 1 | 2 | 2820ns | -29.3% | +94.6% | -45.2% | -21.2% | -34.4% |
| 1 | 3 | 2811ns | -28.6% | +107.1% | +69.3% | -29.6% | +51.3% |
| 1 | 4 | 2811ns | +24.8% | -28.7% | +10.0% | +97.7% | -33.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-container-headroom | -0.072 | ok |
| warm-container-kernel | -0.216 | moderate- |
| warm-container-lanes-deferred | -0.165 | ok |
| warm-container-minimum | -0.748 | HIGH- (thermal bounce) |
| warm-container-native | -0.125 | ok |
| warm-container-plusone | -0.639 | HIGH- (thermal bounce) |

**Consistency summary:**

- **warm-container-kernel**: won 2/4, lost 2/4
- **warm-container-lanes-deferred**: won 1/4, lost 3/4
- **warm-container-minimum**: won 1/4, lost 3/4
- **warm-container-native**: won 3/4, lost 1/4
- **warm-container-plusone**: won 2/4, lost 2/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-container-headroom | 32.0ns | 4552.4ns | 0.7% |  |
| warm-container-kernel | 48.5ns | 3941.6ns | 1.2% |  |
| warm-container-lanes-deferred | 54.3ns | 4381.2ns | 1.2% |  |
| warm-container-minimum | 51.7ns | 3894.9ns | 1.3% |  |
| warm-container-native | 48.7ns | 3954.2ns | 1.2% |  |
| warm-container-plusone | 50.9ns | 3797.2ns | 1.3% |  |

## Distribution (algo ns)

```
warm-container-headroom (n=40, range 2810.4-9429.7 ns)
   2810.4 |########################################
   3141.4 |##############
   3472.3 |
   3803.3 |
   4134.2 |
   4465.2 |
   4796.2 |
   5127.1 |##
   5458.1 |
   5789.1 |
   6120.0 |
   6451.0 |
   6781.9 |
   7112.9 |##
   7443.9 |
   7774.8 |####
   8105.8 |
   8436.8 |##
   8767.7 |
   9098.7 |####
  (2 below, 4 above range)

warm-container-kernel (n=40, range 1981.2-7139.6 ns)
   1981.2 |########################################
   2239.1 |
   2497.0 |
   2754.9 |
   3012.8 |
   3270.8 |
   3528.7 |
   3786.6 |##
   4044.5 |
   4302.5 |##
   4560.4 |
   4818.3 |##
   5076.2 |
   5334.1 |##
   5592.1 |
   5850.0 |####
   6107.9 |##
   6365.8 |######
   6623.8 |##
   6881.7 |##
  (2 below, 6 above range)

warm-container-lanes-deferred (n=40, range 1963.5-6868.9 ns)
   1963.5 |########################################
   2208.7 |
   2454.0 |
   2699.3 |
   2944.6 |######
   3189.8 |###
   3435.1 |
   3680.4 |
   3925.6 |
   4170.9 |
   4416.2 |
   4661.5 |
   4906.7 |
   5152.0 |###
   5397.3 |#############
   5642.6 |######
   5887.8 |#############
   6133.1 |######
   6378.4 |##########
   6623.7 |##########
  (3 below, 3 above range)

warm-container-minimum (n=40, range 1610.5-6417.1 ns)
   1610.5 |###############
   1850.9 |########################################
   2091.2 |#####
   2331.5 |
   2571.8 |
   2812.2 |
   3052.5 |
   3292.8 |
   3533.2 |
   3773.5 |#####
   4013.8 |##########
   4254.2 |
   4494.5 |
   4734.8 |
   4975.1 |###############
   5215.5 |##########
   5455.8 |###############
   5696.1 |##########
   5936.5 |###############
   6176.8 |##########
  (6 below, 4 above range)

warm-container-native (n=40, range 1968.2-6838.6 ns)
   1968.2 |########################################
   2211.7 |
   2455.3 |##
   2698.8 |
   2942.3 |##
   3185.8 |
   3429.3 |
   3672.8 |
   3916.4 |
   4159.9 |
   4403.4 |
   4646.9 |
   4890.4 |
   5133.9 |
   5377.5 |#########
   5621.0 |##
   5864.5 |#######
   6108.0 |
   6351.5 |###########
   6595.0 |####
  (3 below, 3 above range)

warm-container-plusone (n=40, range 1822.0-6706.1 ns)
   1822.0 |########################################
   2066.2 |
   2310.4 |
   2554.6 |##
   2798.8 |##
   3043.0 |
   3287.2 |
   3531.4 |
   3775.6 |##
   4019.8 |
   4264.0 |
   4508.2 |
   4752.5 |
   4996.7 |##
   5240.9 |####
   5485.1 |##
   5729.3 |##
   5973.5 |######
   6217.7 |##
   6461.9 |#############
  (2 below, 2 above range)

```

## Diagnostics

- **warm-container-headroom**: CV=59.1% (high variance, measurements may be unstable)
- **warm-container-headroom**: worst_20/best_20 = 3.4x (possible bimodal distribution)
- **warm-container-kernel**: CV=57.2% (high variance, measurements may be unstable)
- **warm-container-kernel**: worst_20/best_20 = 3.6x (possible bimodal distribution)
- **warm-container-lanes-deferred**: CV=47.4% (high variance, measurements may be unstable)
- **warm-container-lanes-deferred**: worst_20/best_20 = 3.5x (possible bimodal distribution)
- **warm-container-minimum**: CV=51.7% (high variance, measurements may be unstable)
- **warm-container-minimum**: worst_20/best_20 = 4.0x (possible bimodal distribution)
- **warm-container-native**: CV=54.5% (high variance, measurements may be unstable)
- **warm-container-native**: worst_20/best_20 = 3.5x (possible bimodal distribution)
- **warm-container-plusone**: CV=56.2% (high variance, measurements may be unstable)
- **warm-container-plusone**: worst_20/best_20 = 3.7x (possible bimodal distribution)
