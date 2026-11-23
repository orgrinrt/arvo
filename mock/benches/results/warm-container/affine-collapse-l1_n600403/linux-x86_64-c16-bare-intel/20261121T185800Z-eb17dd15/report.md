# Wrapping reduction whose steps are all affine: what the interior projection prevents the optimiser from doing (8192 elements, 3 ops/element)

6 variants, 40 samples per variant.
Baseline: **warm-container-headroom**

## Highlights

Baseline for all deltas below: **warm-container-headroom**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-container-kernel beats baseline by 45% (significant)

warm-container-kernel is -1.56 us (45%) faster than baseline warm-container-headroom, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-container-minimum is an outlier: 2.4x slower than the field

warm-container-minimum (4.23 us) is 2.4x the fastest (1.75 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### warm-container-kernel is fastest but the noisiest (CV 116.6%)

warm-container-kernel wins on median (1.75 us) yet has the highest variance (CV 116.6%), while warm-container-minimum is the steadiest (CV 42.8%, 4.23 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Two tiers: {warm-container-kernel, warm-container-lanes-deferred, warm-container-plusone} vs {warm-container-headroom, warm-container-native, warm-container-minimum} (83% apart)

The field splits into a fast tier {warm-container-kernel, warm-container-lanes-deferred, warm-container-plusone} and a slow tier {warm-container-headroom, warm-container-native, warm-container-minimum} with a 83% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### warm-container-lanes-deferred is inconsistent: worst-20% is 4.6x its best-20%

warm-container-lanes-deferred's best 20% of batches run at 1.42 us but its worst 20% at 6.52 us (4.6x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: warm-container-kernel** at 1747.4 ns median (-49.1% vs baseline)
- 4 variants significantly faster than baseline
- Spread: 2.42x (fastest 1747.4 ns, slowest 4234.9 ns)

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
| Counter frequency | 2112787121 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-container-headroom | 4697ns | 3518ns | 2877ns | 3666ns | 9608ns | base |
| warm-container-kernel | 3533ns | 1829ns | 1482ns | 3231ns | 6491ns | -24.77% |
| warm-container-lanes-deferred | 3535ns | 1941ns | 1483ns | 3109ns | 6862ns | -24.74% |
| warm-container-minimum | 3776ns | 4423ns | 1564ns | 3655ns | 6352ns | -19.60% |
| warm-container-native | 3895ns | 4227ns | 1431ns | 3868ns | 6440ns | -17.07% |
| warm-container-plusone | 3578ns | 1957ns | 1579ns | 3132ns | 6917ns | -23.82% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-container-headroom | 4576ns | 2810ns | 9331ns | base | 7.160 |
| warm-container-kernel | 3364ns | 1415ns | 6151ns | -26.49% | 9.741 |
| warm-container-lanes-deferred | 3366ns | 1416ns | 6515ns | -26.45% | 9.735 |
| warm-container-minimum | 3608ns | 1497ns | 6048ns | -21.17% | 9.083 |
| warm-container-native | 3679ns | 1352ns | 6070ns | -19.61% | 8.907 |
| warm-container-plusone | 3415ns | 1512ns | 6579ns | -25.37% | 9.594 |

## Performance model

- Peak throughput: **24.238 Gops/s** (warm-container-native; best 20% batches)
- Ops per call: 32768

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-container-headroom | 9.539 | 39.4% |
| warm-container-kernel | 18.752 | 77.4% |
| warm-container-lanes-deferred | 17.688 | 73.0% |
| warm-container-minimum | 7.738 | 31.9% |
| warm-container-native | 8.176 | 33.7% |
| warm-container-plusone | 17.473 | 72.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-container-headroom | 4697ns | 4697ns | base |
| warm-container-kernel | 3533ns | 3533ns | -24.77% |
| warm-container-lanes-deferred | 3535ns | 3535ns | -24.74% |
| warm-container-minimum | 3776ns | 3776ns | -19.60% |
| warm-container-native | 3895ns | 3895ns | -17.07% |
| warm-container-plusone | 3578ns | 3578ns | -23.82% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-container-headroom | 3435ns | base | --- | [3435, 3448] | --- | --- | --- | --- |
| warm-container-kernel | 1747ns | -1560.8ns (-45.4%) | [-1864, -1245]ns | [1729, 5077] | YES | 0.0017 | 0.0007 | 0 |
| warm-container-lanes-deferred | 1853ns | -1402.8ns (-40.8%) | [-1706, -1085]ns | [1730, 4257] | YES | 0.0037 | 0.0022 | 0 |
| warm-container-minimum | 4235ns | -1318.8ns (-38.4%) | [-1604, -575]ns | [1861, 4678] | YES | 0.0207 | 0.0166 | 0 |
| warm-container-native | 4008ns | no significant difference | [-1773, +651]ns | [2574, 4744] | no | 0.2682 | 0.2682 | 0 |
| warm-container-plusone | 1875ns | -1293.9ns (-37.7%) | [-1572, -954]ns | [1864, 4774] | YES | 0.0017 | 0.0007 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-container-headroom | warm-container-kernel | warm-container-lanes-deferred | warm-container-minimum | warm-container-native | warm-container-plusone |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 2816ns | -13.4% | -49.6% | +70.7% | -8.0% | -45.4% |
| 1 | 2 | 3965ns | -56.2% | -56.2% | +9.8% | -3.4% | -52.8% |
| 1 | 3 | 3441ns | -58.7% | +21.0% | -45.4% | +22.8% | +25.1% |
| 1 | 4 | 3435ns | -6.8% | -49.3% | +26.4% | +38.1% | -19.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-container-headroom | -0.474 | moderate- |
| warm-container-kernel | -0.282 | moderate- |
| warm-container-lanes-deferred | -0.318 | moderate- |
| warm-container-minimum | -0.281 | moderate- |
| warm-container-native | 0.139 | ok |
| warm-container-plusone | -0.048 | ok |

**Consistency summary:**

- **warm-container-kernel**: won 4/4, lost 0/4
- **warm-container-lanes-deferred**: won 3/4, lost 1/4
- **warm-container-minimum**: won 1/4, lost 3/4
- **warm-container-native**: won 2/4, lost 2/4
- **warm-container-plusone**: won 3/4, lost 1/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-container-headroom | 31.5ns | 4576.3ns | 0.7% |  |
| warm-container-kernel | 48.0ns | 3364.1ns | 1.4% |  |
| warm-container-lanes-deferred | 47.4ns | 3365.9ns | 1.4% |  |
| warm-container-minimum | 47.3ns | 3607.5ns | 1.3% |  |
| warm-container-native | 61.1ns | 3678.7ns | 1.7% |  |
| warm-container-plusone | 44.5ns | 3415.4ns | 1.3% |  |

## Distribution (algo ns)

```
warm-container-headroom (n=40, range 2810.4-9331.0 ns)
   2810.4 |#####################
   3136.4 |########################################
   3462.5 |
   3788.5 |
   4114.5 |##
   4440.6 |
   4766.6 |
   5092.6 |##
   5418.6 |
   5744.7 |
   6070.7 |
   6396.7 |
   6722.8 |##
   7048.8 |
   7374.8 |##
   7700.9 |##
   8026.9 |##
   8352.9 |##
   8679.0 |
   9005.0 |##
  (3 below, 3 above range)

warm-container-kernel (n=40, range 1414.7-6150.7 ns)
   1414.7 |#########################
   1651.5 |########################################
   1888.3 |
   2125.1 |
   2361.9 |
   2598.7 |
   2835.5 |
   3072.3 |
   3309.1 |###
   3545.9 |
   3782.7 |
   4019.5 |
   4256.3 |
   4493.1 |#######
   4729.9 |###
   4966.7 |#######
   5203.5 |
   5440.3 |##########
   5677.1 |##########
   5913.9 |
  (4 below, 6 above range)

warm-container-lanes-deferred (n=40, range 1416.3-6515.1 ns)
   1416.3 |######
   1671.3 |########################################
   1926.2 |######
   2181.1 |
   2436.1 |
   2691.0 |
   2945.9 |
   3200.9 |
   3455.8 |
   3710.8 |######
   3965.7 |######
   4220.6 |######
   4475.6 |###
   4730.5 |###
   4985.5 |
   5240.4 |
   5495.3 |###
   5750.3 |
   6005.2 |##########
   6260.2 |##########
  (6 below, 3 above range)

warm-container-minimum (n=40, range 1497.4-6047.7 ns)
   1497.4 |########################################
   1724.9 |##############################
   1952.5 |
   2180.0 |#####
   2407.5 |
   2635.0 |
   2862.5 |
   3090.0 |#####
   3317.5 |
   3545.0 |
   3772.6 |#####
   4000.1 |#####
   4227.6 |##########
   4455.1 |####################
   4682.6 |###############
   4910.1 |##########
   5137.6 |###############
   5365.1 |#####
   5592.7 |
   5820.2 |
  (2 below, 5 above range)

warm-container-native (n=40, range 1352.0-6069.9 ns)
   1352.0 |########################################
   1587.8 |########
   1823.7 |
   2059.6 |
   2295.5 |
   2531.4 |
   2767.3 |
   3003.2 |
   3239.1 |
   3475.0 |############
   3710.9 |########
   3946.8 |########
   4182.7 |############
   4418.6 |####
   4654.5 |########
   4890.4 |####################
   5126.3 |####
   5362.2 |
   5598.1 |################
   5834.0 |
  (2 below, 3 above range)

warm-container-plusone (n=40, range 1511.9-6578.7 ns)
   1511.9 |####################
   1765.3 |########################################
   2018.6 |###
   2272.0 |
   2525.3 |
   2778.6 |
   3032.0 |
   3285.3 |
   3538.7 |###
   3792.0 |###
   4045.3 |
   4298.7 |
   4552.0 |######
   4805.4 |
   5058.7 |######
   5312.0 |###
   5565.4 |###
   5818.7 |######
   6072.1 |###
   6325.4 |###
  (4 below, 5 above range)

```

## Diagnostics

- **warm-container-headroom**: CV=56.1% (high variance, measurements may be unstable)
- **warm-container-headroom**: worst_20/best_20 = 3.3x (possible bimodal distribution)
- **warm-container-kernel**: CV=60.6% (high variance, measurements may be unstable)
- **warm-container-kernel**: worst_20/best_20 = 4.3x (possible bimodal distribution)
- **warm-container-lanes-deferred**: CV=60.9% (high variance, measurements may be unstable)
- **warm-container-lanes-deferred**: worst_20/best_20 = 4.6x (possible bimodal distribution)
- **warm-container-minimum**: CV=50.2% (high variance, measurements may be unstable)
- **warm-container-minimum**: worst_20/best_20 = 4.0x (possible bimodal distribution)
- **warm-container-native**: CV=49.4% (high variance, measurements may be unstable)
- **warm-container-native**: worst_20/best_20 = 4.5x (possible bimodal distribution)
- **warm-container-plusone**: CV=60.9% (high variance, measurements may be unstable)
- **warm-container-plusone**: worst_20/best_20 = 4.4x (possible bimodal distribution)
