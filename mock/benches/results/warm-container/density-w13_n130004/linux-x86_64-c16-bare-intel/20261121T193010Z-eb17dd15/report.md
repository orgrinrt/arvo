# Container fork, operation-density sweep at 13 bits (8192 elements, wrapping)

6 variants, 40 samples per variant.
Baseline: **warm-container-headroom**

## Highlights

Baseline for all deltas below: **warm-container-headroom**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-container-native dominates: 105% faster than the next best (warm-container-minimum)

warm-container-native (1.61 us) leads warm-container-minimum (3.29 us) by 105%, a clear separation rather than a photo finish. CV 15.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### warm-container-native beats baseline by 58% (significant)

warm-container-native is -2.45 us (58%) faster than baseline warm-container-headroom, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-container-lanes-deferred is an outlier: 2.9x slower than the field

warm-container-lanes-deferred (4.60 us) is 2.9x the fastest (1.61 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### warm-container-lanes-deferred shows alternating (throttle bounce) (autocorr -0.64)

warm-container-lanes-deferred's per-pass series has lag-1 autocorrelation -0.64, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {warm-container-native} vs {warm-container-minimum, warm-container-headroom, warm-container-plusone, warm-container-kernel, warm-container-lanes-deferred} (105% apart)

The field splits into a fast tier {warm-container-native} and a slow tier {warm-container-minimum, warm-container-headroom, warm-container-plusone, warm-container-kernel, warm-container-lanes-deferred} with a 105% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### warm-container-kernel is inconsistent: worst-20% is 4.2x its best-20%

warm-container-kernel's best 20% of batches run at 1.40 us but its worst 20% at 5.92 us (4.2x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: warm-container-native** at 1607.3 ns median (-62.1% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 2.86x (fastest 1607.3 ns, slowest 4600.7 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Container fork, operation-density sweep at 13 bits (8192 elements, wrapping) |
| Passes | 4 |
| Runs per pass | 1000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x123456789abcdef0 |
| Counter frequency | 2112823892 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-container-headroom | 3853ns | 4379ns | 1585ns | 3919ns | 5923ns | base |
| warm-container-kernel | 3841ns | 4569ns | 1451ns | 3855ns | 6186ns | -0.32% |
| warm-container-lanes-deferred | 3839ns | 4770ns | 1450ns | 3954ns | 5885ns | -0.35% |
| warm-container-minimum | 3887ns | 3411ns | 1731ns | 3632ns | 6810ns | +0.90% |
| warm-container-native | 1898ns | 1818ns | 1569ns | 1837ns | 2410ns | -50.74% |
| warm-container-plusone | 4009ns | 4495ns | 1695ns | 3895ns | 6665ns | +4.05% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-container-headroom | 3718ns | 1533ns | 5691ns | base | 11.017 |
| warm-container-kernel | 3695ns | 1399ns | 5920ns | -0.62% | 11.085 |
| warm-container-lanes-deferred | 3695ns | 1398ns | 5632ns | -0.63% | 11.087 |
| warm-container-minimum | 3737ns | 1671ns | 6522ns | +0.52% | 10.960 |
| warm-container-native | 1661ns | 1392ns | 2065ns | -55.33% | 24.661 |
| warm-container-plusone | 3860ns | 1636ns | 6393ns | +3.82% | 10.612 |

## Performance model

- Peak throughput: **29.422 Gops/s** (warm-container-native; best 20% batches)
- Ops per call: 40960

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-container-headroom | 9.667 | 32.9% |
| warm-container-kernel | 9.295 | 31.6% |
| warm-container-lanes-deferred | 8.903 | 30.3% |
| warm-container-minimum | 12.440 | 42.3% |
| warm-container-native | 25.483 | 86.6% |
| warm-container-plusone | 9.434 | 32.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-container-headroom | 3853ns | 3853ns | base |
| warm-container-kernel | 3841ns | 3841ns | -0.32% |
| warm-container-lanes-deferred | 3839ns | 3839ns | -0.35% |
| warm-container-minimum | 3887ns | 3887ns | +0.90% |
| warm-container-native | 1898ns | 1898ns | -50.74% |
| warm-container-plusone | 4009ns | 4009ns | +4.05% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-container-headroom | 4237ns | base | --- | [3289, 4536] | --- | --- | --- | --- |
| warm-container-kernel | 4407ns | no significant difference | [-148, +386]ns | [1709, 5015] | no | 0.8746 | 0.8746 | 0 |
| warm-container-lanes-deferred | 4601ns | no significant difference | [-148, +279]ns | [2441, 4879] | no | 0.7948 | 0.6358 | 0 |
| warm-container-minimum | 3293ns | no significant difference | [-479, +220]ns | [1753, 4812] | no | 0.7948 | 0.6358 | 0 |
| warm-container-native | 1607ns | -2450.2ns (-57.8%) | [-2770, -1686]ns | [1474, 1768] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-plusone | 4342ns | no significant difference | [-78, +882]ns | [1886, 4963] | no | 0.3846 | 0.1539 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-container-headroom | warm-container-kernel | warm-container-lanes-deferred | warm-container-minimum | warm-container-native | warm-container-plusone |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 4633ns | +3.4% | -3.3% | -9.0% | -68.2% | -14.8% |
| 1 | 2 | 1537ns | +195.6% | +134.0% | +147.4% | -6.7% | +173.7% |
| 1 | 3 | 4083ns | -1.1% | +15.3% | -56.9% | -56.7% | +21.3% |
| 1 | 4 | 4480ns | -7.0% | -4.8% | -53.6% | -56.1% | -22.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-container-headroom | -0.409 | moderate- |
| warm-container-kernel | 0.237 | moderate+ |
| warm-container-lanes-deferred | -0.639 | HIGH- (thermal bounce) |
| warm-container-minimum | 0.245 | moderate+ |
| warm-container-native | 0.266 | moderate+ |
| warm-container-plusone | -0.441 | moderate- |

**Consistency summary:**

- **warm-container-kernel**: won 2/4, lost 2/4
- **warm-container-lanes-deferred**: won 2/4, lost 2/4
- **warm-container-minimum**: won 3/4, lost 1/4
- **warm-container-native**: won 4/4, lost 0/4
- **warm-container-plusone**: won 2/4, lost 2/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-container-headroom | 49.2ns | 3717.9ns | 1.3% |  |
| warm-container-kernel | 51.4ns | 3694.9ns | 1.4% |  |
| warm-container-lanes-deferred | 51.0ns | 3694.5ns | 1.4% |  |
| warm-container-minimum | 51.6ns | 3737.4ns | 1.4% |  |
| warm-container-native | 82.4ns | 1660.9ns | 5.0% |  |
| warm-container-plusone | 53.7ns | 3859.8ns | 1.4% |  |

## Distribution (algo ns)

```
warm-container-headroom (n=40, range 1532.9-5691.0 ns)
   1532.9 |########################################
   1740.9 |
   1948.8 |
   2156.7 |################
   2364.6 |
   2572.5 |########
   2780.4 |
   2988.3 |
   3196.2 |
   3404.1 |
   3612.0 |########
   3819.9 |########################
   4027.8 |################
   4235.7 |########################
   4443.6 |########################################
   4651.5 |########################
   4859.4 |########
   5067.3 |################################
   5275.2 |
   5483.1 |########
  (6 below, 3 above range)

warm-container-kernel (n=40, range 1398.9-5920.3 ns)
   1398.9 |##########
   1625.0 |########################################
   1851.0 |
   2077.1 |
   2303.2 |
   2529.2 |
   2755.3 |
   2981.4 |
   3207.4 |
   3433.5 |#####
   3659.6 |#####
   3885.7 |#####
   4111.7 |#####
   4337.8 |
   4563.9 |####################
   4789.9 |##########
   5016.0 |####################
   5242.1 |#####
   5468.1 |####################
   5694.2 |#####
  (6 below, 4 above range)

warm-container-lanes-deferred (n=40, range 1398.4-5632.2 ns)
   1398.4 |########################################
   1610.1 |################################
   1821.8 |
   2033.5 |
   2245.2 |########
   2456.9 |########
   2668.5 |########
   2880.2 |
   3091.9 |
   3303.6 |########
   3515.3 |
   3727.0 |
   3938.7 |
   4150.4 |########
   4362.0 |################
   4573.7 |################################
   4785.4 |################################
   4997.1 |################################
   5208.8 |################
   5420.5 |########################
  (4 below, 3 above range)

warm-container-minimum (n=40, range 1670.7-6522.4 ns)
   1670.7 |########################################
   1913.3 |
   2155.8 |##
   2398.4 |##
   2641.0 |
   2883.6 |
   3126.2 |
   3368.8 |
   3611.3 |
   3853.9 |##
   4096.5 |#####
   4339.1 |#####
   4581.7 |##
   4824.3 |##
   5066.9 |
   5309.4 |
   5552.0 |##
   5794.6 |##
   6037.2 |##
   6279.8 |####################
  (2 below, 2 above range)

warm-container-native (n=40, range 1392.1-2065.1 ns)
   1392.1 |##########
   1425.8 |#####
   1459.4 |########################################
   1493.1 |#####
   1526.7 |##########
   1560.4 |
   1594.0 |#########################
   1627.7 |#####
   1661.3 |
   1695.0 |
   1728.6 |
   1762.2 |###################################
   1795.9 |
   1829.5 |#####
   1863.2 |
   1896.8 |#####
   1930.5 |
   1964.1 |###############
   1997.8 |
   2031.4 |#####
  (4 below, 3 above range)

warm-container-plusone (n=40, range 1636.5-6393.1 ns)
   1636.5 |########################################
   1874.3 |#######
   2112.1 |###
   2350.0 |
   2587.8 |
   2825.6 |
   3063.5 |
   3301.3 |
   3539.1 |
   3776.9 |###
   4014.8 |#######
   4252.6 |###
   4490.4 |###
   4728.3 |##############
   4966.1 |###
   5203.9 |#######
   5441.8 |#######
   5679.6 |##########
   5917.4 |
   6155.2 |##########
  (3 below, 3 above range)

```

## Diagnostics

- **warm-container-headroom**: CV=42.9% (high variance, measurements may be unstable)
- **warm-container-headroom**: worst_20/best_20 = 3.7x (possible bimodal distribution)
- **warm-container-kernel**: CV=50.0% (high variance, measurements may be unstable)
- **warm-container-kernel**: worst_20/best_20 = 4.2x (possible bimodal distribution)
- **warm-container-lanes-deferred**: CV=46.4% (high variance, measurements may be unstable)
- **warm-container-lanes-deferred**: worst_20/best_20 = 4.0x (possible bimodal distribution)
- **warm-container-minimum**: CV=55.4% (high variance, measurements may be unstable)
- **warm-container-minimum**: worst_20/best_20 = 3.9x (possible bimodal distribution)
- **warm-container-plusone**: CV=49.9% (high variance, measurements may be unstable)
- **warm-container-plusone**: worst_20/best_20 = 3.9x (possible bimodal distribution)
