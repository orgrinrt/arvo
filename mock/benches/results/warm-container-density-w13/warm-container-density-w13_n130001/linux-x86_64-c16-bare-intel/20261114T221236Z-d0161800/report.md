# Container fork, operation-density sweep at 13 bits (8192 elements, wrapping)

6 variants, 40 samples per variant.
Baseline: **warm-container-headroom**

## Highlights

Baseline for all deltas below: **warm-container-headroom**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (warm-container-headroom) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline warm-container-headroom has the worst median (4.91 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest warm-container-native at 1.09 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### warm-container-native dominates: 136% faster than the next best (warm-container-kernel)

warm-container-native (1.09 us) leads warm-container-kernel (2.58 us) by 136%, a clear separation rather than a photo finish. CV 7.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### warm-container-native beats baseline by 81% (significant)

warm-container-native is -3.96 us (81%) faster than baseline warm-container-headroom, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-container-headroom is an outlier: 4.5x slower than the field

warm-container-headroom (4.91 us) is 4.5x the fastest (1.09 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {warm-container-native} vs {warm-container-kernel, warm-container-lanes-deferred, warm-container-plusone, warm-container-minimum, warm-container-headroom} (136% apart)

The field splits into a fast tier {warm-container-native} and a slow tier {warm-container-kernel, warm-container-lanes-deferred, warm-container-plusone, warm-container-minimum, warm-container-headroom} with a 136% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 4.5x the fastest

Fastest warm-container-native (1.09 us) to slowest warm-container-headroom (4.91 us): 4.5x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### warm-container-plusone is inconsistent: worst-20% is 4.4x its best-20%

warm-container-plusone's best 20% of batches run at 1.57 us but its worst 20% at 6.90 us (4.4x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

### warm-container-plusone's edge over baseline is significant but tiny (-40 ns, 0.82%)

warm-container-plusone differs from baseline warm-container-headroom by -40 ns (0.82%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: warm-container-native** at 1092.8 ns median (-77.8% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 4.50x (fastest 1092.8 ns, slowest 4914.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-container-headroom | 4423ns | 5081ns | 1897ns | 4285ns | 7361ns | base |
| warm-container-kernel | 2907ns | 2805ns | 2094ns | 2826ns | 3963ns | -34.26% |
| warm-container-lanes-deferred | 3123ns | 3099ns | 2745ns | 3125ns | 3492ns | -29.39% |
| warm-container-minimum | 4252ns | 4814ns | 1884ns | 4143ns | 6946ns | -3.87% |
| warm-container-native | 1308ns | 1322ns | 1169ns | 1297ns | 1482ns | -70.42% |
| warm-container-plusone | 4171ns | 4240ns | 1626ns | 4008ns | 7207ns | -5.68% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-container-headroom | 4258ns | 1835ns | 7074ns | base | 3.848 |
| warm-container-kernel | 2696ns | 1953ns | 3670ns | -36.68% | 6.077 |
| warm-container-lanes-deferred | 2900ns | 2559ns | 3209ns | -31.89% | 5.650 |
| warm-container-minimum | 4098ns | 1821ns | 6675ns | -3.75% | 3.998 |
| warm-container-native | 1074ns | 964ns | 1194ns | -74.77% | 15.252 |
| warm-container-plusone | 4017ns | 1572ns | 6900ns | -5.67% | 4.079 |

## Performance model

- Peak throughput: **16.991 Gops/s** (warm-container-native; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-container-headroom | 3.334 | 19.6% |
| warm-container-kernel | 6.342 | 37.3% |
| warm-container-lanes-deferred | 5.705 | 33.6% |
| warm-container-minimum | 3.519 | 20.7% |
| warm-container-native | 14.993 | 88.2% |
| warm-container-plusone | 3.997 | 23.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-container-headroom | 4423ns | 4423ns | base |
| warm-container-kernel | 2907ns | 2907ns | -34.26% |
| warm-container-lanes-deferred | 3123ns | 3123ns | -29.39% |
| warm-container-minimum | 4252ns | 4252ns | -3.87% |
| warm-container-native | 1308ns | 1308ns | -70.42% |
| warm-container-plusone | 4171ns | 4171ns | -5.68% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-container-headroom | 4914ns | base | --- | [1848, 5487] | --- | --- | --- | --- |
| warm-container-kernel | 2584ns | no significant difference | [-2227, +5]ns | [2415, 2857] | no | 0.1345 | 0.0807 | 0 |
| warm-container-lanes-deferred | 2872ns | no significant difference | [-2526, +654]ns | [2830, 3080] | no | 0.2682 | 0.2682 | 0 |
| warm-container-minimum | 4656ns | no significant difference | [-46, +287]ns | [1833, 5594] | no | 0.1923 | 0.1539 | 0 |
| warm-container-native | 1093ns | -3963.8ns (-80.7%) | [-4488, -805]ns | [1054, 1098] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-plusone | 4099ns | -40.1ns (-0.8%) | [-346, -14]ns | [1833, 4999] | YES | 0.0161 | 0.0064 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-container-headroom | warm-container-kernel | warm-container-lanes-deferred | warm-container-minimum | warm-container-native | warm-container-plusone |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 1844ns | +25.6% | +64.3% | +170.3% | -40.6% | -18.8% |
| 1 | 2 | 1841ns | +28.8% | +50.1% | +17.8% | -40.3% | +155.3% |
| 1 | 3 | 5487ns | -37.4% | -48.3% | -41.4% | -81.2% | -20.4% |
| 1 | 4 | 5098ns | -43.5% | -39.6% | -19.6% | -80.5% | -14.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-container-headroom | 0.217 | moderate+ |
| warm-container-kernel | -0.007 | ok |
| warm-container-lanes-deferred | -0.229 | moderate- |
| warm-container-minimum | -0.365 | moderate- |
| warm-container-native | 0.279 | moderate+ |
| warm-container-plusone | -0.169 | ok |

**Consistency summary:**

- **warm-container-kernel**: won 2/4, lost 2/4
- **warm-container-lanes-deferred**: won 2/4, lost 2/4
- **warm-container-minimum**: won 2/4, lost 2/4
- **warm-container-native**: won 4/4, lost 0/4
- **warm-container-plusone**: won 3/4, lost 1/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-container-headroom | 59.0ns | 4258.1ns | 1.4% |  |
| warm-container-kernel | 74.7ns | 2696.2ns | 2.8% |  |
| warm-container-lanes-deferred | 79.9ns | 2900.0ns | 2.8% |  |
| warm-container-minimum | 54.3ns | 4098.3ns | 1.3% |  |
| warm-container-native | 85.5ns | 1074.2ns | 8.0% | HIGH |
| warm-container-plusone | 53.4ns | 4016.6ns | 1.3% |  |

## Distribution (algo ns)

```
warm-container-headroom (n=40, range 1834.8-7073.8 ns)
   1834.8 |########################################
   2096.8 |
   2358.7 |
   2620.7 |
   2882.6 |
   3144.6 |
   3406.5 |
   3668.5 |
   3930.4 |###
   4192.4 |
   4454.3 |###
   4716.3 |#######
   4978.2 |##################
   5240.1 |###
   5502.1 |##########
   5764.0 |#######
   6026.0 |#######
   6287.9 |###
   6549.9 |##############
   6811.8 |
  (5 below, 2 above range)

warm-container-kernel (n=40, range 1952.8-3669.6 ns)
   1952.8 |
   2038.7 |######
   2124.5 |#############
   2210.3 |########################################
   2296.2 |#############
   2382.0 |########################################
   2467.9 |######
   2553.7 |##########################
   2639.6 |######
   2725.4 |
   2811.2 |#############
   2897.1 |######
   2982.9 |
   3068.8 |####################
   3154.6 |####################
   3240.4 |
   3326.3 |######
   3412.1 |######
   3498.0 |
   3583.8 |
  (2 below, 4 above range)

warm-container-lanes-deferred (n=40, range 2558.6-3209.0 ns)
   2558.6 |
   2591.1 |##############################
   2623.6 |
   2656.1 |#####
   2688.7 |##########
   2721.2 |
   2753.7 |
   2786.2 |
   2818.7 |###############
   2851.3 |#########################
   2883.8 |###############
   2916.3 |#####
   2948.8 |#####
   2981.4 |
   3013.9 |#####
   3046.4 |
   3078.9 |
   3111.4 |########################################
   3144.0 |#####
   3176.5 |###############
  (3 below, 2 above range)

warm-container-minimum (n=40, range 1821.1-6675.1 ns)
   1821.1 |########################################
   2063.8 |
   2306.5 |###
   2549.2 |###
   2791.9 |
   3034.6 |
   3277.3 |
   3520.0 |
   3762.7 |
   4005.4 |###
   4248.1 |
   4490.8 |#######
   4733.5 |###
   4976.2 |###
   5218.9 |###
   5461.6 |##########
   5704.3 |
   5947.0 |##########
   6189.7 |#######
   6432.4 |#########################
  (5 below, 1 above range)

warm-container-native (n=40, range 964.3-1194.2 ns)
    964.3 |
    975.8 |
    987.3 |####################
    998.8 |########
   1010.3 |
   1021.8 |
   1033.3 |
   1044.8 |##
   1056.3 |##
   1067.8 |##
   1079.3 |#####
   1090.8 |########################################
   1102.3 |##
   1113.8 |
   1125.3 |##
   1136.8 |
   1148.2 |##
   1159.7 |
   1171.2 |##
   1182.7 |
  (3 below, 4 above range)

warm-container-plusone (n=40, range 1572.2-6899.9 ns)
   1572.2 |########################################
   1838.6 |
   2104.9 |
   2371.3 |
   2637.7 |
   2904.1 |####
   3170.5 |
   3436.9 |
   3703.3 |#############
   3969.6 |########
   4236.0 |########
   4502.4 |########
   4768.8 |####
   5035.2 |####
   5301.6 |####
   5567.9 |####
   5834.3 |#############
   6100.7 |
   6367.1 |#################
   6633.5 |########
  (6 below, 2 above range)

```

## Diagnostics

- **warm-container-headroom**: CV=51.1% (high variance, measurements may be unstable)
- **warm-container-headroom**: worst_20/best_20 = 3.9x (possible bimodal distribution)
- **warm-container-kernel**: CV=24.4% (high variance, measurements may be unstable)
- **warm-container-minimum**: CV=50.5% (high variance, measurements may be unstable)
- **warm-container-minimum**: worst_20/best_20 = 3.7x (possible bimodal distribution)
- **warm-container-plusone**: CV=51.4% (high variance, measurements may be unstable)
- **warm-container-plusone**: worst_20/best_20 = 4.4x (possible bimodal distribution)
